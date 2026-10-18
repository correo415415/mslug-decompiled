// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p02_4) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $02BE64: move.b #$f, $20(a6)
pub fn i_02be64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE6A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02BE6A: andi.w #$ff, d0
pub fn i_02be6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE6E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02BE6E: lsr.w #$2, d0
pub fn i_02be6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE70;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02BE70: add.b d0, $20(a6)
pub fn i_02be70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE74;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02BE74: lea.l $2be7a(pc), a1
pub fn i_02be74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BE7A;
    16
}
/// $02BE78: move.l a1, (a6)
pub fn i_02be78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BE7A: subq.b #$1, $91(a6)
pub fn i_02be7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE7E;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02BE7E: bne.w $2be90
pub fn i_02be7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE82;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2BE90; return 10; }
    8
}
/// $02BE82: clr.w $28(a6)
pub fn i_02be82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE86;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02BE86: clr.w $2c(a6)
pub fn i_02be86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE8A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02BE8A: lea.l $2be90(pc), a1
pub fn i_02be8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BE90;
    16
}
/// $02BE8E: move.l a1, (a6)
pub fn i_02be8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BE90: jsr $2aa24(pc)
pub fn i_02be90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BE94);
    cpu.pc = 0x2AA24;
    16
}
/// $02BE94: jsr $2a7d8(pc)
pub fn i_02be94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BE98);
    cpu.pc = 0x2A7D8;
    16
}
/// $02BE98: bcc.w $2bea2
pub fn i_02be98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BEA2; return 10; }
    8
}
/// $02BE9C: lea.l $2b4ce(pc), a1
pub fn i_02be9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CE;
    16
}
/// $02BEA0: move.l a1, (a6)
pub fn i_02bea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BEA2: jsr $2a4f0(pc)
pub fn i_02bea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEA6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BEA6);
    cpu.pc = 0x2A4F0;
    16
}
/// $02BEA6: bcc.w $2beb0
pub fn i_02bea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BEB0; return 10; }
    8
}
/// $02BEAA: lea.l $2b4ce(pc), a1
pub fn i_02beaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CE;
    16
}
/// $02BEAE: move.l a1, (a6)
pub fn i_02beae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BEB0: jsr $28d70.l
pub fn i_02beb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BEB6);
    cpu.pc = 0x28D70;
    16
}
/// $02BEB6: bcc.w $2bec0
pub fn i_02beb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BEC0; return 10; }
    8
}
/// $02BEBA: lea.l $2b38c(pc), a1
pub fn i_02beba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B38C;
    16
}
/// $02BEBE: move.l a1, (a6)
pub fn i_02bebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BEC0: jsr $283d8.l
pub fn i_02bec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BEC6);
    cpu.pc = 0x283D8;
    16
}
/// $02BEC6: jsr $2a328(pc)
pub fn i_02bec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BECA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BECA);
    cpu.pc = 0x2A328;
    16
}
/// $02BECA: bcc.w $2bed4
pub fn i_02beca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BECE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BED4; return 10; }
    8
}
/// $02BECE: lea.l $2bf64(pc), a1
pub fn i_02bece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BED2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BF64;
    16
}
/// $02BED2: move.l a1, (a6)
pub fn i_02bed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BED4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BED4: jsr $2a34e(pc)
pub fn i_02bed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BED8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BED8);
    cpu.pc = 0x2A34E;
    16
}
/// $02BED8: bcc.w $2bee2
pub fn i_02bed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BEE2; return 10; }
    8
}
/// $02BEDC: lea.l $2c07a(pc), a1
pub fn i_02bedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C07A;
    16
}
/// $02BEE0: move.l a1, (a6)
pub fn i_02bee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BEE2: jsr $2a59a(pc)
pub fn i_02bee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BEE6);
    cpu.pc = 0x2A59A;
    16
}
/// $02BEE6: bcc.w $2bf02
pub fn i_02bee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BF02; return 10; }
    8
}
/// $02BEEA: jsr $2a9a0(pc)
pub fn i_02beea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BEEE);
    cpu.pc = 0x2A9A0;
    16
}
/// $02BEEE: andi.w #$ff, d0
pub fn i_02beee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEF2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02BEF2: lea.l $2a078(pc), a0
pub fn i_02bef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEF6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A078;
    16
}
/// $02BEF6: movea.l #$ffffffff, a1
pub fn i_02bef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEFC;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02BEFC: jsr $772.l
pub fn i_02befc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BF02);
    cpu.pc = 0x772;
    16
}
/// $02BF02: jsr $2a8c0(pc)
pub fn i_02bf02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BF06);
    cpu.pc = 0x2A8C0;
    16
}
/// $02BF06: bcc.w $2bf10
pub fn i_02bf06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BF10; return 10; }
    8
}
/// $02BF0A: lea.l $2d02e(pc), a1
pub fn i_02bf0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02BF0E: move.l a1, (a6)
pub fn i_02bf0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BF10: jsr $2aaf0(pc)
pub fn i_02bf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BF14);
    cpu.pc = 0x2AAF0;
    16
}
/// $02BF14: lea.l $2c9b0(pc), a0
pub fn i_02bf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF18;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02BF18: movea.l #$ffffffff, a1
pub fn i_02bf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF1E;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02BF1E: jsr $772.l
pub fn i_02bf1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BF24);
    cpu.pc = 0x772;
    16
}
/// $02BF24: jsr $2aac0(pc)
pub fn i_02bf24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BF28);
    cpu.pc = 0x2AAC0;
    16
}
/// $02BF28: bcc.w $2bf32
pub fn i_02bf28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF2C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BF32; return 10; }
    8
}
/// $02BF2C: lea.l $2c24a(pc), a1
pub fn i_02bf2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02BF30: move.l a1, (a6)
pub fn i_02bf30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BF32: jsr $2ac80(pc)
pub fn i_02bf32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BF36);
    cpu.pc = 0x2AC80;
    16
}
/// $02BF36: bcc.w $2bf40
pub fn i_02bf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BF40; return 10; }
    8
}
/// $02BF3A: lea.l $2dd74(pc), a1
pub fn i_02bf3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02BF3E: move.l a1, (a6)
pub fn i_02bf3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BF40: jsr $2a5cc(pc)
pub fn i_02bf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BF44);
    cpu.pc = 0x2A5CC;
    16
}
/// $02BF44: bcc.w $2bf4e
pub fn i_02bf44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF48;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BF4E; return 10; }
    8
}
/// $02BF48: lea.l $2bbf2(pc), a1
pub fn i_02bf48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02BF4C: move.l a1, (a6)
pub fn i_02bf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BF4E: jsr $2a690(pc)
pub fn i_02bf4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BF52);
    cpu.pc = 0x2A690;
    16
}
/// $02BF52: lea.l $2a060(pc), a0
pub fn i_02bf52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF56;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02BF56: movea.l #$ffffffff, a1
pub fn i_02bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF5C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02bf5c  ($02BF5C, 8 B, thunks/jsr_abs_thunks.c)
/// $02BF5C: jsr $772.l
pub fn i_02bf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BF62);
    cpu.pc = 0x772;
    16
}
/// $02BF62: rts
pub fn i_02bf62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CruiseRightB_02bf64  ($02BF64, 270 B, slug/states_02ae3e.s)
/// $02BF64: move.w #$2a0, $28(a6)
pub fn i_02bf64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF6A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02BF6A: clr.w $2c(a6)
pub fn i_02bf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF6E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02BF6E: jsr $2a9a0(pc)
pub fn i_02bf6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BF72);
    cpu.pc = 0x2A9A0;
    16
}
/// $02BF72: movea.l #$2793d0, a0
pub fn i_02bf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF78;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2793D0;
    cpu.a[0] = v1;
    4
}
/// $02BF78: lsl.w #$2, d0
pub fn i_02bf78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF7A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02BF7A: movea.l (a0, d0.w), a0
pub fn i_02bf7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF7E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02BF7E: cmpa.l #$ffffffff, a0
pub fn i_02bf7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF84;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02BF84: beq.w $2bf8e
pub fn i_02bf84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF88;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2BF8E; return 10; }
    8
}
/// $02BF88: jsr $28cd4.l
pub fn i_02bf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BF8E);
    cpu.pc = 0x28CD4;
    16
}
/// $02BF8E: lea.l $29ffc(pc), a1
pub fn i_02bf8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x29FFC;
    16
}
/// $02BF92: movea.l (a1, d0.w), a0
pub fn i_02bf92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF96;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02BF96: move.l a0, $48(a6)
pub fn i_02bf96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF9A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02BF9A: move.b #$0, $20(a6)
pub fn i_02bf9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFA0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02BFA0: andi.w #$ff, d0
pub fn i_02bfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFA4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02BFA4: lsr.w #$2, d0
pub fn i_02bfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFA6;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02BFA6: add.b d0, $20(a6)
pub fn i_02bfa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFAA;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02BFAA: lea.l $2bfb0(pc), a1
pub fn i_02bfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BFB0;
    16
}
/// $02BFAE: move.l a1, (a6)
pub fn i_02bfae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BFB0: jsr $2aa24(pc)
pub fn i_02bfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BFB4);
    cpu.pc = 0x2AA24;
    16
}
/// $02BFB4: jsr $2a7d8(pc)
pub fn i_02bfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BFB8);
    cpu.pc = 0x2A7D8;
    16
}
/// $02BFB8: bcc.w $2bfc2
pub fn i_02bfb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFBC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BFC2; return 10; }
    8
}
/// $02BFBC: lea.l $2b4ca(pc), a1
pub fn i_02bfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CA;
    16
}
/// $02BFC0: move.l a1, (a6)
pub fn i_02bfc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BFC2: jsr $2a4f0(pc)
pub fn i_02bfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BFC6);
    cpu.pc = 0x2A4F0;
    16
}
/// $02BFC6: bcc.w $2bfd0
pub fn i_02bfc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFCA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BFD0; return 10; }
    8
}
/// $02BFCA: lea.l $2b4ca(pc), a1
pub fn i_02bfca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CA;
    16
}
/// $02BFCE: move.l a1, (a6)
pub fn i_02bfce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BFD0: jsr $2a328(pc)
pub fn i_02bfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BFD4);
    cpu.pc = 0x2A328;
    16
}
/// $02BFD4: bcs.w $2bfde
pub fn i_02bfd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFD8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2BFDE; return 10; }
    8
}
/// $02BFD8: lea.l $2bd00(pc), a1
pub fn i_02bfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BD00;
    16
}
/// $02BFDC: move.l a1, (a6)
pub fn i_02bfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFDE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BFDE: jsr $2a34e(pc)
pub fn i_02bfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BFE2);
    cpu.pc = 0x2A34E;
    16
}
/// $02BFE2: bcc.w $2bfec
pub fn i_02bfe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2BFEC; return 10; }
    8
}
/// $02BFE6: lea.l $2c07a(pc), a1
pub fn i_02bfe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C07A;
    16
}
/// $02BFEA: move.l a1, (a6)
pub fn i_02bfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02BFEC: jsr $28d70.l
pub fn i_02bfec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BFF2);
    cpu.pc = 0x28D70;
    16
}
/// $02BFF2: jsr $283d8.l
pub fn i_02bff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2BFF8);
    cpu.pc = 0x283D8;
    16
}
/// $02BFF8: jsr $2a59a(pc)
pub fn i_02bff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2BFFC);
    cpu.pc = 0x2A59A;
    16
}
/// $02BFFC: bcc.w $2c018
pub fn i_02bffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C000;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C018; return 10; }
    8
}
/// $02C000: jsr $2a9a0(pc)
pub fn i_02c000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C004;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C004);
    cpu.pc = 0x2A9A0;
    16
}
/// $02C004: andi.w #$ff, d0
pub fn i_02c004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C008;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C008: lea.l $2a078(pc), a0
pub fn i_02c008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C00C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A078;
    16
}
/// $02C00C: movea.l #$ffffffff, a1
pub fn i_02c00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C012;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C012: jsr $772.l
pub fn i_02c012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C018;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C018);
    cpu.pc = 0x772;
    16
}
/// $02C018: jsr $2a8c0(pc)
pub fn i_02c018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C01C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C01C);
    cpu.pc = 0x2A8C0;
    16
}
/// $02C01C: bcc.w $2c026
pub fn i_02c01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C020;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C026; return 10; }
    8
}
/// $02C020: lea.l $2d02e(pc), a1
pub fn i_02c020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C024;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02C024: move.l a1, (a6)
pub fn i_02c024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C026;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C026: jsr $2aaf0(pc)
pub fn i_02c026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C02A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C02A);
    cpu.pc = 0x2AAF0;
    16
}
/// $02C02A: lea.l $2c9b0(pc), a0
pub fn i_02c02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C02E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02C02E: movea.l #$ffffffff, a1
pub fn i_02c02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C034;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C034: jsr $772.l
pub fn i_02c034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C03A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C03A);
    cpu.pc = 0x772;
    16
}
/// $02C03A: jsr $2aac0(pc)
pub fn i_02c03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C03E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C03E);
    cpu.pc = 0x2AAC0;
    16
}
/// $02C03E: bcc.w $2c048
pub fn i_02c03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C042;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C048; return 10; }
    8
}
/// $02C042: lea.l $2c24a(pc), a1
pub fn i_02c042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C046;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02C046: move.l a1, (a6)
pub fn i_02c046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C048;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C048: jsr $2ac80(pc)
pub fn i_02c048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C04C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C04C);
    cpu.pc = 0x2AC80;
    16
}
/// $02C04C: bcc.w $2c056
pub fn i_02c04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C050;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C056; return 10; }
    8
}
/// $02C050: lea.l $2dd74(pc), a1
pub fn i_02c050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C054;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02C054: move.l a1, (a6)
pub fn i_02c054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C056;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C056: jsr $2a5cc(pc)
pub fn i_02c056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C05A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C05A);
    cpu.pc = 0x2A5CC;
    16
}
/// $02C05A: bcc.w $2c064
pub fn i_02c05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C05E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C064; return 10; }
    8
}
/// $02C05E: lea.l $2bbf2(pc), a1
pub fn i_02c05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C062;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02C062: move.l a1, (a6)
pub fn i_02c062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C064;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C064: jsr $2a690(pc)
pub fn i_02c064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C068;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C068);
    cpu.pc = 0x2A690;
    16
}
/// $02C068: lea.l $2a060(pc), a0
pub fn i_02c068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C06C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02C06C: movea.l #$ffffffff, a1
pub fn i_02c06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C072;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02c072  ($02C072, 8 B, thunks/jsr_abs_thunks.c)
/// $02C072: jsr $772.l
pub fn i_02c072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C078);
    cpu.pc = 0x772;
    16
}
/// $02C078: rts
pub fn i_02c078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C07A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CruiseLeftB_02c07a  ($02C07A, 270 B, slug/states_02ae3e.s)
/// $02C07A: move.w #$fd60, $28(a6)
pub fn i_02c07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C080;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02C080: clr.w $2c(a6)
pub fn i_02c080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C084;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02C084: jsr $2a9a0(pc)
pub fn i_02c084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C088;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C088);
    cpu.pc = 0x2A9A0;
    16
}
/// $02C088: movea.l #$279524, a0
pub fn i_02c088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C08E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279524;
    cpu.a[0] = v1;
    4
}
/// $02C08E: lsl.w #$2, d0
pub fn i_02c08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C090;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C090: movea.l (a0, d0.w), a0
pub fn i_02c090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C094;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C094: cmpa.l #$ffffffff, a0
pub fn i_02c094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C09A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02C09A: beq.w $2c0a4
pub fn i_02c09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C09E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C0A4; return 10; }
    8
}
/// $02C09E: jsr $28cd4.l
pub fn i_02c09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C0A4);
    cpu.pc = 0x28CD4;
    16
}
/// $02C0A4: lea.l $29ffc(pc), a1
pub fn i_02c0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x29FFC;
    16
}
/// $02C0A8: movea.l (a1, d0.w), a0
pub fn i_02c0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0AC;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C0AC: move.l a0, $48(a6)
pub fn i_02c0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0B0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02C0B0: move.b #$5, $20(a6)
pub fn i_02c0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0B6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02C0B6: andi.w #$ff, d0
pub fn i_02c0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0BA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C0BA: lsr.w #$2, d0
pub fn i_02c0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0BC;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C0BC: add.b d0, $20(a6)
pub fn i_02c0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0C0;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02C0C0: lea.l $2c0c6(pc), a1
pub fn i_02c0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C0C6;
    16
}
/// $02C0C4: move.l a1, (a6)
pub fn i_02c0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C0C6: jsr $2aa24(pc)
pub fn i_02c0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C0CA);
    cpu.pc = 0x2AA24;
    16
}
/// $02C0CA: jsr $2a7d8(pc)
pub fn i_02c0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C0CE);
    cpu.pc = 0x2A7D8;
    16
}
/// $02C0CE: bcc.w $2c0d8
pub fn i_02c0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C0D8; return 10; }
    8
}
/// $02C0D2: lea.l $2b4ce(pc), a1
pub fn i_02c0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CE;
    16
}
/// $02C0D6: move.l a1, (a6)
pub fn i_02c0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C0D8: jsr $2a4f0(pc)
pub fn i_02c0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C0DC);
    cpu.pc = 0x2A4F0;
    16
}
/// $02C0DC: bcc.w $2c0e6
pub fn i_02c0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C0E6; return 10; }
    8
}
/// $02C0E0: lea.l $2b4ce(pc), a1
pub fn i_02c0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CE;
    16
}
/// $02C0E4: move.l a1, (a6)
pub fn i_02c0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C0E6: jsr $2a34e(pc)
pub fn i_02c0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C0EA);
    cpu.pc = 0x2A34E;
    16
}
/// $02C0EA: bcs.w $2c0f4
pub fn i_02c0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0EE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2C0F4; return 10; }
    8
}
/// $02C0EE: lea.l $2be32(pc), a1
pub fn i_02c0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BE32;
    16
}
/// $02C0F2: move.l a1, (a6)
pub fn i_02c0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C0F4: jsr $2a328(pc)
pub fn i_02c0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C0F8);
    cpu.pc = 0x2A328;
    16
}
/// $02C0F8: bcc.w $2c102
pub fn i_02c0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C0FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C102; return 10; }
    8
}
/// $02C0FC: lea.l $2bf64(pc), a1
pub fn i_02c0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C100;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BF64;
    16
}
/// $02C100: move.l a1, (a6)
pub fn i_02c100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C102;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C102: jsr $28d70.l
pub fn i_02c102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C108;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C108);
    cpu.pc = 0x28D70;
    16
}
/// $02C108: jsr $283d8.l
pub fn i_02c108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C10E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C10E);
    cpu.pc = 0x283D8;
    16
}
/// $02C10E: jsr $2a59a(pc)
pub fn i_02c10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C112;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C112);
    cpu.pc = 0x2A59A;
    16
}
/// $02C112: bcc.w $2c12e
pub fn i_02c112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C116;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C12E; return 10; }
    8
}
/// $02C116: jsr $2a9a0(pc)
pub fn i_02c116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C11A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C11A);
    cpu.pc = 0x2A9A0;
    16
}
/// $02C11A: andi.w #$ff, d0
pub fn i_02c11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C11E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C11E: lea.l $2a078(pc), a0
pub fn i_02c11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C122;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A078;
    16
}
/// $02C122: movea.l #$ffffffff, a1
pub fn i_02c122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C128;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C128: jsr $772.l
pub fn i_02c128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C12E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C12E);
    cpu.pc = 0x772;
    16
}
/// $02C12E: jsr $2a8c0(pc)
pub fn i_02c12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C132;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C132);
    cpu.pc = 0x2A8C0;
    16
}
/// $02C132: bcc.w $2c13c
pub fn i_02c132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C136;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C13C; return 10; }
    8
}
/// $02C136: lea.l $2d02e(pc), a1
pub fn i_02c136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C13A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02C13A: move.l a1, (a6)
pub fn i_02c13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C13C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C13C: jsr $2aaf0(pc)
pub fn i_02c13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C140;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C140);
    cpu.pc = 0x2AAF0;
    16
}
/// $02C140: lea.l $2c9b0(pc), a0
pub fn i_02c140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C144;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02C144: movea.l #$ffffffff, a1
pub fn i_02c144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C14A;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C14A: jsr $772.l
pub fn i_02c14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C150;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C150);
    cpu.pc = 0x772;
    16
}
/// $02C150: jsr $2aac0(pc)
pub fn i_02c150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C154;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C154);
    cpu.pc = 0x2AAC0;
    16
}
/// $02C154: bcc.w $2c15e
pub fn i_02c154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C158;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C15E; return 10; }
    8
}
/// $02C158: lea.l $2c24a(pc), a1
pub fn i_02c158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C15C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02C15C: move.l a1, (a6)
pub fn i_02c15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C15E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C15E: jsr $2ac80(pc)
pub fn i_02c15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C162;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C162);
    cpu.pc = 0x2AC80;
    16
}
/// $02C162: bcc.w $2c16c
pub fn i_02c162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C166;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C16C; return 10; }
    8
}
/// $02C166: lea.l $2dd74(pc), a1
pub fn i_02c166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C16A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02C16A: move.l a1, (a6)
pub fn i_02c16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C16C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C16C: jsr $2a5cc(pc)
pub fn i_02c16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C170;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C170);
    cpu.pc = 0x2A5CC;
    16
}
/// $02C170: bcc.w $2c17a
pub fn i_02c170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C174;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C17A; return 10; }
    8
}
/// $02C174: lea.l $2bbf2(pc), a1
pub fn i_02c174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C178;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02C178: move.l a1, (a6)
pub fn i_02c178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C17A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C17A: jsr $2a690(pc)
pub fn i_02c17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C17E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C17E);
    cpu.pc = 0x2A690;
    16
}
/// $02C17E: lea.l $2a060(pc), a0
pub fn i_02c17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C182;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02C182: movea.l #$ffffffff, a1
pub fn i_02c182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C188;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02c188  ($02C188, 8 B, thunks/jsr_abs_thunks.c)
/// $02C188: jsr $772.l
pub fn i_02c188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C18E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C18E);
    cpu.pc = 0x772;
    16
}
/// $02C18E: rts
pub fn i_02c18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C190;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_FireIdle_02c190  ($02C190, 186 B, slug/states_02ae3e.s)
/// $02C190: bclr.b #$5, $8c(a6)
pub fn i_02c190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C196;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02C196: jsr $2a9a0(pc)
pub fn i_02c196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C19A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C19A);
    cpu.pc = 0x2A9A0;
    16
}
/// $02C19A: movea.l #$279498, a0
pub fn i_02c19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1A0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279498;
    cpu.a[0] = v1;
    4
}
/// $02C1A0: lsl.w #$2, d0
pub fn i_02c1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1A2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C1A2: movea.l (a0, d0.w), a0
pub fn i_02c1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1A6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C1A6: cmpa.l #$ffffffff, a0
pub fn i_02c1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1AC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02C1AC: beq.w $2c1b6
pub fn i_02c1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C1B6; return 10; }
    8
}
/// $02C1B0: jsr $28cd4.l
pub fn i_02c1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C1B6);
    cpu.pc = 0x28CD4;
    16
}
/// $02C1B6: lea.l $29ffc(pc), a1
pub fn i_02c1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x29FFC;
    16
}
/// $02C1BA: movea.l (a1, d0.w), a0
pub fn i_02c1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1BE;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C1BE: move.l a0, $48(a6)
pub fn i_02c1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1C2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02C1C2: move.b #$14, $20(a6)
pub fn i_02c1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1C8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02C1C8: andi.w #$ff, d0
pub fn i_02c1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1CC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C1CC: lsr.w #$2, d0
pub fn i_02c1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1CE;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C1CE: add.b d0, $20(a6)
pub fn i_02c1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1D2;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02C1D2: move.b #$4, d1
pub fn i_02c1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1D6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02C1D6: jsr $8f714.l
pub fn i_02c1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C1DC);
    cpu.pc = 0x8F714;
    16
}
/// $02C1DC: bcs.w $2c22e
pub fn i_02c1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1E0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2C22E; return 10; }
    8
}
/// $02C1E0: cmpi.b #$ff, $84(a6)
pub fn i_02c1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1E6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $02C1E6: beq.w $2c200
pub fn i_02c1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1EA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C200; return 10; }
    8
}
/// $02C1EA: lea.l $311c0.l, a1
pub fn i_02c1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1F0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x311C0;
    20
}
/// $02C1F0: jsr $6fe.l
pub fn i_02c1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C1F6);
    cpu.pc = 0x6FE;
    16
}
/// $02C1F6: jsr $5dd02.l
pub fn i_02c1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C1FC);
    cpu.pc = 0x5DD02;
    16
}
/// $02C1FC: bra.w $2c212
pub fn i_02c1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C200;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C212;
    10
}
/// $02C200: lea.l $31240.l, a1
pub fn i_02c200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C206;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31240;
    20
}
/// $02C206: jsr $6fe.l
pub fn i_02c206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C20C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C20C);
    cpu.pc = 0x6FE;
    16
}
/// $02C20C: jsr $5dd02.l
pub fn i_02c20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C212;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C212);
    cpu.pc = 0x5DD02;
    16
}
/// $02C212: jsr $517fe.l
pub fn i_02c212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C218;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C218);
    cpu.pc = 0x517FE;
    16
}
/// $02C218: lea.l $319f0.l, a1
pub fn i_02c218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C21E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x319F0;
    20
}
/// $02C21E: jsr $6fe.l
pub fn i_02c21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C224;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C224);
    cpu.pc = 0x6FE;
    16
}
/// $02C224: jsr $5dd02.l
pub fn i_02c224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C22A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C22A);
    cpu.pc = 0x5DD02;
    16
}
/// $02C22A: bra.w $2c246
pub fn i_02c22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C22E;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C246;
    10
}
/// $02C22E: lea.l $31944.l, a1
pub fn i_02c22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C234;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31944;
    20
}
/// $02C234: jsr $6fe.l
pub fn i_02c234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C23A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C23A);
    cpu.pc = 0x6FE;
    16
}
/// $02C23A: jsr $5dd02.l
pub fn i_02c23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C240);
    cpu.pc = 0x5DD02;
    16
}
/// $02C240: jsr $517fe.l
pub fn i_02c240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C246;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C246);
    cpu.pc = 0x517FE;
    16
}
/// $02C246: bra.w $2c300
pub fn i_02c246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C24A;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C300;
    10
}
// ===== Slug_FireFlat_02c24a  ($02C24A, 480 B, slug/states_02ae3e.s)
/// $02C24A: bclr.b #$5, $8c(a6)
pub fn i_02c24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C250;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02C250: jsr $2a9a0(pc)
pub fn i_02c250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C254;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C254);
    cpu.pc = 0x2A9A0;
    16
}
/// $02C254: movea.l #$279498, a0
pub fn i_02c254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C25A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279498;
    cpu.a[0] = v1;
    4
}
/// $02C25A: lsl.w #$2, d0
pub fn i_02c25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C25C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C25C: movea.l (a0, d0.w), a0
pub fn i_02c25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C260;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C260: cmpa.l #$ffffffff, a0
pub fn i_02c260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C266;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02C266: beq.w $2c270
pub fn i_02c266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C26A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C270; return 10; }
    8
}
/// $02C26A: jsr $28cd4.l
pub fn i_02c26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C270);
    cpu.pc = 0x28CD4;
    16
}
/// $02C270: lea.l $29ffc(pc), a1
pub fn i_02c270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C274;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x29FFC;
    16
}
/// $02C274: movea.l (a1, d0.w), a0
pub fn i_02c274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C278;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C278: move.l a0, $48(a6)
pub fn i_02c278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C27C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02C27C: move.b #$3c, $20(a6)
pub fn i_02c27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C282;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02C282: andi.w #$ff, d0
pub fn i_02c282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C286;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C286: lsr.w #$2, d0
pub fn i_02c286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C288;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C288: add.b d0, $20(a6)
pub fn i_02c288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C28C;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02C28C: move.b #$4, d1
pub fn i_02c28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C290;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02C290: jsr $8f714.l
pub fn i_02c290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C296;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C296);
    cpu.pc = 0x8F714;
    16
}
/// $02C296: bcs.w $2c2e8
pub fn i_02c296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C29A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2C2E8; return 10; }
    8
}
/// $02C29A: cmpi.b #$ff, $84(a6)
pub fn i_02c29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2A0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $02C2A0: beq.w $2c2ba
pub fn i_02c2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C2BA; return 10; }
    8
}
/// $02C2A4: lea.l $312b2.l, a1
pub fn i_02c2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2AA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x312B2;
    20
}
/// $02C2AA: jsr $6fe.l
pub fn i_02c2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C2B0);
    cpu.pc = 0x6FE;
    16
}
/// $02C2B0: jsr $5dd02.l
pub fn i_02c2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C2B6);
    cpu.pc = 0x5DD02;
    16
}
/// $02C2B6: bra.w $2c2cc
pub fn i_02c2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C2CC;
    10
}
/// $02C2BA: lea.l $31240.l, a1
pub fn i_02c2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2C0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31240;
    20
}
/// $02C2C0: jsr $6fe.l
pub fn i_02c2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C2C6);
    cpu.pc = 0x6FE;
    16
}
/// $02C2C6: jsr $5dd02.l
pub fn i_02c2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C2CC);
    cpu.pc = 0x5DD02;
    16
}
/// $02C2CC: jsr $517fe.l
pub fn i_02c2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C2D2);
    cpu.pc = 0x517FE;
    16
}
/// $02C2D2: lea.l $319f0.l, a1
pub fn i_02c2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2D8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x319F0;
    20
}
/// $02C2D8: jsr $6fe.l
pub fn i_02c2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C2DE);
    cpu.pc = 0x6FE;
    16
}
/// $02C2DE: jsr $5dd02.l
pub fn i_02c2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C2E4);
    cpu.pc = 0x5DD02;
    16
}
/// $02C2E4: bra.w $2c300
pub fn i_02c2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C300;
    10
}
/// $02C2E8: lea.l $31944.l, a1
pub fn i_02c2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2EE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31944;
    20
}
/// $02C2EE: jsr $6fe.l
pub fn i_02c2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C2F4);
    cpu.pc = 0x6FE;
    16
}
/// $02C2F4: jsr $5dd02.l
pub fn i_02c2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C2FA);
    cpu.pc = 0x5DD02;
    16
}
/// $02C2FA: jsr $517fe.l
pub fn i_02c2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C300;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C300);
    cpu.pc = 0x517FE;
    16
}
/// $02C300: cmpi.b #$0, $90(a6)
pub fn i_02c300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C306;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $02C306: beq.w $2c30e
pub fn i_02c306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C30A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C30E; return 10; }
    8
}
/// $02C30A: jsr $2aab0(pc)
pub fn i_02c30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C30E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C30E);
    cpu.pc = 0x2AAB0;
    16
}
/// $02C30E: move.b #$1e, $8e(a6)
pub fn i_02c30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C314;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02C314: bset.b #$0, $8d(a6)
pub fn i_02c314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C31A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02C31A: move.l #$295b4, $60(a6)
pub fn i_02c31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C322;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02C322: jsr $5cf1c.l
pub fn i_02c322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C328);
    cpu.pc = 0x5CF1C;
    16
}
/// $02C328: bcs.w $2c334
pub fn i_02c328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C32C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2C334; return 10; }
    8
}
/// $02C32C: clr.w $28(a6)
pub fn i_02c32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C330;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02C330: clr.w $2c(a6)
pub fn i_02c330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C334;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02C334: lea.l $2c33a(pc), a1
pub fn i_02c334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C338;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C33A;
    16
}
/// $02C338: move.l a1, (a6)
pub fn i_02c338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C33A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C33A: jsr $2aac0(pc)
pub fn i_02c33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C33E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C33E);
    cpu.pc = 0x2AAC0;
    16
}
/// $02C33E: bcc.w $2c348
pub fn i_02c33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C342;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C348; return 10; }
    8
}
/// $02C342: lea.l $2c24a(pc), a1
pub fn i_02c342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C346;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02C346: move.l a1, (a6)
pub fn i_02c346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C348;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C348: jsr $2abc0(pc)
pub fn i_02c348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C34C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C34C);
    cpu.pc = 0x2ABC0;
    16
}
/// $02C34C: bcs.w $2c36c
pub fn i_02c34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C350;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2C36C; return 10; }
    8
}
/// $02C350: jsr $2a328(pc)
pub fn i_02c350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C354;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C354);
    cpu.pc = 0x2A328;
    16
}
/// $02C354: bcc.w $2c35e
pub fn i_02c354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C358;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C35E; return 10; }
    8
}
/// $02C358: lea.l $2c572(pc), a1
pub fn i_02c358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C35C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C572;
    16
}
/// $02C35C: move.l a1, (a6)
pub fn i_02c35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C35E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C35E: jsr $2a34e(pc)
pub fn i_02c35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C362;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C362);
    cpu.pc = 0x2A34E;
    16
}
/// $02C362: bcc.w $2c36c
pub fn i_02c362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C366;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C36C; return 10; }
    8
}
/// $02C366: lea.l $2c648(pc), a1
pub fn i_02c366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C36A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C648;
    16
}
/// $02C36A: move.l a1, (a6)
pub fn i_02c36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C36C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C36C: jsr $2aa24(pc)
pub fn i_02c36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C370;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C370);
    cpu.pc = 0x2AA24;
    16
}
/// $02C370: move.w $28(a6), d0
pub fn i_02c370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C374;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02C374: move.w #$540, d1
pub fn i_02c374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C378;
    cpu.ir = 0x323C;
    let v1: u32 = 0x540;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02C378: jsr $267f4.l
pub fn i_02c378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C37E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C37E);
    cpu.pc = 0x267F4;
    16
}
/// $02C37E: move.w d0, $28(a6)
pub fn i_02c37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C382;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02C382: jsr $2abc0(pc)
pub fn i_02c382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C386;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C386);
    cpu.pc = 0x2ABC0;
    16
}
/// $02C386: bcc.w $2c392
pub fn i_02c386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C38A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C392; return 10; }
    8
}
/// $02C38A: clr.w $28(a6)
pub fn i_02c38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C38E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02C38E: clr.w $2c(a6)
pub fn i_02c38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C392;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02C392: jsr $2a85a(pc)
pub fn i_02c392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C396;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C396);
    cpu.pc = 0x2A85A;
    16
}
/// $02C396: bcc.w $2c3a0
pub fn i_02c396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C39A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C3A0; return 10; }
    8
}
/// $02C39A: lea.l $2b4d2(pc), a1
pub fn i_02c39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C39E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4D2;
    16
}
/// $02C39E: move.l a1, (a6)
pub fn i_02c39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C3A0: jsr $2a478(pc)
pub fn i_02c3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C3A4);
    cpu.pc = 0x2A478;
    16
}
/// $02C3A4: jsr $28d70.l
pub fn i_02c3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C3AA);
    cpu.pc = 0x28D70;
    16
}
/// $02C3AA: bcc.w $2c3b4
pub fn i_02c3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C3B4; return 10; }
    8
}
/// $02C3AE: lea.l $2c432(pc), a1
pub fn i_02c3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C432;
    16
}
/// $02C3B2: move.l a1, (a6)
pub fn i_02c3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C3B4: cmpi.b #$10, $8e(a6)
pub fn i_02c3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3BA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Byte, false);
    16
}
/// $02C3BA: bcc.w $2c3f2
pub fn i_02c3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C3F2; return 10; }
    8
}
/// $02C3BE: jsr $2a59a(pc)
pub fn i_02c3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C3C2);
    cpu.pc = 0x2A59A;
    16
}
/// $02C3C2: bcc.w $2c3de
pub fn i_02c3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C3DE; return 10; }
    8
}
/// $02C3C6: jsr $2a9a0(pc)
pub fn i_02c3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C3CA);
    cpu.pc = 0x2A9A0;
    16
}
/// $02C3CA: andi.w #$ff, d0
pub fn i_02c3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3CE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C3CE: lea.l $2a078(pc), a0
pub fn i_02c3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3D2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A078;
    16
}
/// $02C3D2: movea.l #$ffffffff, a1
pub fn i_02c3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3D8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C3D8: jsr $772.l
pub fn i_02c3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C3DE);
    cpu.pc = 0x772;
    16
}
/// $02C3DE: jsr $2aaf0(pc)
pub fn i_02c3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C3E2);
    cpu.pc = 0x2AAF0;
    16
}
/// $02C3E2: lea.l $2c9b0(pc), a0
pub fn i_02c3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3E6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02C3E6: movea.l #$ffffffff, a1
pub fn i_02c3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3EC;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C3EC: jsr $772.l
pub fn i_02c3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C3F2);
    cpu.pc = 0x772;
    16
}
/// $02C3F2: jsr $2a8c0(pc)
pub fn i_02c3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C3F6);
    cpu.pc = 0x2A8C0;
    16
}
/// $02C3F6: bcc.w $2c400
pub fn i_02c3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C400; return 10; }
    8
}
/// $02C3FA: lea.l $2d02e(pc), a1
pub fn i_02c3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02C3FE: move.l a1, (a6)
pub fn i_02c3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C400;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C400: jsr $2ac80(pc)
pub fn i_02c400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C404;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C404);
    cpu.pc = 0x2AC80;
    16
}
/// $02C404: bcc.w $2c40e
pub fn i_02c404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C408;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C40E; return 10; }
    8
}
/// $02C408: lea.l $2dd74(pc), a1
pub fn i_02c408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C40C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02C40C: move.l a1, (a6)
pub fn i_02c40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C40E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C40E: jsr $2a5cc(pc)
pub fn i_02c40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C412;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C412);
    cpu.pc = 0x2A5CC;
    16
}
/// $02C412: bcc.w $2c41c
pub fn i_02c412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C416;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C41C; return 10; }
    8
}
/// $02C416: lea.l $2bbf2(pc), a1
pub fn i_02c416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C41A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02C41A: move.l a1, (a6)
pub fn i_02c41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C41C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C41C: jsr $2a690(pc)
pub fn i_02c41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C420;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C420);
    cpu.pc = 0x2A690;
    16
}
/// $02C420: lea.l $2a060(pc), a0
pub fn i_02c420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C424;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02C424: movea.l #$ffffffff, a1
pub fn i_02c424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C42A;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02c42a  ($02C42A, 8 B, thunks/jsr_abs_thunks.c)
/// $02C42A: jsr $772.l
pub fn i_02c42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C430);
    cpu.pc = 0x772;
    16
}
/// $02C430: rts
pub fn i_02c430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C432;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_FireRecoil_02c432  ($02C432, 282 B, slug/states_02ae3e.s)
/// $02C432: bclr.b #$5, $8c(a6)
pub fn i_02c432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C438;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02C438: jsr $2a9a0(pc)
pub fn i_02c438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C43C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C43C);
    cpu.pc = 0x2A9A0;
    16
}
/// $02C43C: movea.l #$2794e8, a0
pub fn i_02c43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C442;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2794E8;
    cpu.a[0] = v1;
    4
}
/// $02C442: lsl.w #$2, d0
pub fn i_02c442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C444;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C444: movea.l (a0, d0.w), a0
pub fn i_02c444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C448;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C448: cmpa.l #$ffffffff, a0
pub fn i_02c448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C44E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02C44E: beq.w $2c458
pub fn i_02c44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C452;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C458; return 10; }
    8
}
/// $02C452: jsr $28cd4.l
pub fn i_02c452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C458;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C458);
    cpu.pc = 0x28CD4;
    16
}
/// $02C458: move.b #$14, $20(a6)
pub fn i_02c458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C45E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02C45E: andi.w #$ff, d0
pub fn i_02c45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C462;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C462: lsr.w #$2, d0
pub fn i_02c462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C464;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C464: add.b d0, $20(a6)
pub fn i_02c464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C468;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02C468: lea.l $2c46e(pc), a1
pub fn i_02c468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C46C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C46E;
    16
}
/// $02C46C: move.l a1, (a6)
pub fn i_02c46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C46E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C46E: clr.w $28(a6)
pub fn i_02c46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C472;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02C472: clr.w $2c(a6)
pub fn i_02c472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C476;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02C476: jsr $2aa24(pc)
pub fn i_02c476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C47A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C47A);
    cpu.pc = 0x2AA24;
    16
}
/// $02C47A: jsr $2abc0(pc)
pub fn i_02c47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C47E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C47E);
    cpu.pc = 0x2ABC0;
    16
}
/// $02C47E: bcs.w $2c49e
pub fn i_02c47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C482;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2C49E; return 10; }
    8
}
/// $02C482: jsr $2a328(pc)
pub fn i_02c482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C486;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C486);
    cpu.pc = 0x2A328;
    16
}
/// $02C486: bcc.w $2c490
pub fn i_02c486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C48A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C490; return 10; }
    8
}
/// $02C48A: lea.l $2b8de(pc), a1
pub fn i_02c48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C48E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B8DE;
    16
}
/// $02C48E: move.l a1, (a6)
pub fn i_02c48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C490;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C490: jsr $2a34e(pc)
pub fn i_02c490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C494;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C494);
    cpu.pc = 0x2A34E;
    16
}
/// $02C494: bcc.w $2c49e
pub fn i_02c494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C498;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C49E; return 10; }
    8
}
/// $02C498: lea.l $2ba44(pc), a1
pub fn i_02c498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C49C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BA44;
    16
}
/// $02C49C: move.l a1, (a6)
pub fn i_02c49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C49E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C49E: move.w $28(a6), d0
pub fn i_02c49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4A2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02C4A2: move.w #$540, d1
pub fn i_02c4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x540;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02C4A6: jsr $267f4.l
pub fn i_02c4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C4AC);
    cpu.pc = 0x267F4;
    16
}
/// $02C4AC: move.w d0, $28(a6)
pub fn i_02c4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4B0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02C4B0: jsr $2a7d8(pc)
pub fn i_02c4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C4B4);
    cpu.pc = 0x2A7D8;
    16
}
/// $02C4B4: bcc.w $2c4be
pub fn i_02c4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4B8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C4BE; return 10; }
    8
}
/// $02C4B8: lea.l $2b4d2(pc), a1
pub fn i_02c4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4D2;
    16
}
/// $02C4BC: move.l a1, (a6)
pub fn i_02c4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C4BE: jsr $2a478(pc)
pub fn i_02c4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C4C2);
    cpu.pc = 0x2A478;
    16
}
/// $02C4C2: jsr $28d70.l
pub fn i_02c4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C4C8);
    cpu.pc = 0x28D70;
    16
}
/// $02C4C8: bcc.w $2c4d2
pub fn i_02c4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C4D2; return 10; }
    8
}
/// $02C4CC: lea.l $2b38c(pc), a1
pub fn i_02c4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B38C;
    16
}
/// $02C4D0: move.l a1, (a6)
pub fn i_02c4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C4D2: jsr $2a59a(pc)
pub fn i_02c4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C4D6);
    cpu.pc = 0x2A59A;
    16
}
/// $02C4D6: bcc.w $2c4f2
pub fn i_02c4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C4F2; return 10; }
    8
}
/// $02C4DA: jsr $2a9a0(pc)
pub fn i_02c4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C4DE);
    cpu.pc = 0x2A9A0;
    16
}
/// $02C4DE: andi.w #$ff, d0
pub fn i_02c4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4E2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C4E2: lea.l $2a078(pc), a0
pub fn i_02c4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4E6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A078;
    16
}
/// $02C4E6: movea.l #$ffffffff, a1
pub fn i_02c4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4EC;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C4EC: jsr $772.l
pub fn i_02c4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C4F2);
    cpu.pc = 0x772;
    16
}
/// $02C4F2: jsr $2a8c0(pc)
pub fn i_02c4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C4F6);
    cpu.pc = 0x2A8C0;
    16
}
/// $02C4F6: bcc.w $2c500
pub fn i_02c4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C500; return 10; }
    8
}
/// $02C4FA: lea.l $2d02e(pc), a1
pub fn i_02c4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02C4FE: move.l a1, (a6)
pub fn i_02c4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C500;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C500: jsr $2aaf0(pc)
pub fn i_02c500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C504;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C504);
    cpu.pc = 0x2AAF0;
    16
}
/// $02C504: lea.l $2c9b0(pc), a0
pub fn i_02c504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C508;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02C508: movea.l #$ffffffff, a1
pub fn i_02c508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C50E;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C50E: jsr $772.l
pub fn i_02c50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C514);
    cpu.pc = 0x772;
    16
}
/// $02C514: jsr $2aac0(pc)
pub fn i_02c514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C518;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C518);
    cpu.pc = 0x2AAC0;
    16
}
/// $02C518: bcc.w $2c522
pub fn i_02c518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C51C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C522; return 10; }
    8
}
/// $02C51C: lea.l $2c24a(pc), a1
pub fn i_02c51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C520;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02C520: move.l a1, (a6)
pub fn i_02c520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C522;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C522: jsr $2ac80(pc)
pub fn i_02c522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C526;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C526);
    cpu.pc = 0x2AC80;
    16
}
/// $02C526: bcc.w $2c530
pub fn i_02c526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C52A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C530; return 10; }
    8
}
/// $02C52A: lea.l $2dd74(pc), a1
pub fn i_02c52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C52E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02C52E: move.l a1, (a6)
pub fn i_02c52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C530;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C530: jsr $2a5cc(pc)
pub fn i_02c530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C534;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C534);
    cpu.pc = 0x2A5CC;
    16
}
/// $02C534: bcc.w $2c53e
pub fn i_02c534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C538;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C53E; return 10; }
    8
}
/// $02C538: lea.l $2bbf2(pc), a1
pub fn i_02c538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C53C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02C53C: move.l a1, (a6)
pub fn i_02c53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C53E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C53E: jsr $2a690(pc)
pub fn i_02c53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C542;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C542);
    cpu.pc = 0x2A690;
    16
}
/// $02C542: lea.l $2a060(pc), a0
pub fn i_02c542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C546;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02C546: movea.l #$ffffffff, a1
pub fn i_02c546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C54C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02c54c  ($02C54C, 8 B, thunks/jsr_abs_thunks.c)
/// $02C54C: jsr $772.l
pub fn i_02c54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C552);
    cpu.pc = 0x772;
    16
}
/// $02C552: rts
pub fn i_02c552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C554;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_FireFlatResume_02c554  ($02C554, 30 B, slug/states_02ae3e.s)
/// $02C554: jsr $267e6.l
pub fn i_02c554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C55A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C55A);
    cpu.pc = 0x267E6;
    16
}
/// $02C55A: move.w #$0, $28(a6)
pub fn i_02c55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C560;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02C560: move.l #$295b4, $60(a6)
pub fn i_02c560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C568;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02C568: lea.l $2c33a(pc), a1
pub fn i_02c568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C56C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C33A;
    16
}
/// $02C56C: move.l a1, (a6)
pub fn i_02c56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C56E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C56E: jmp $2c33a(pc)
pub fn i_02c56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C572;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2C33A;
    8
}
// ===== Slug_FireMoveRight_02c572  ($02C572, 206 B, slug/states_02ae3e.s)
/// $02C572: move.w #$2a0, $28(a6)
pub fn i_02c572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C578;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02C578: clr.w $2c(a6)
pub fn i_02c578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C57C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02C57C: lea.l $2c582(pc), a1
pub fn i_02c57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C580;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C582;
    16
}
/// $02C580: move.l a1, (a6)
pub fn i_02c580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C582;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C582: jsr $2aa24(pc)
pub fn i_02c582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C586;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C586);
    cpu.pc = 0x2AA24;
    16
}
/// $02C586: jsr $2a7d8(pc)
pub fn i_02c586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C58A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C58A);
    cpu.pc = 0x2A7D8;
    16
}
/// $02C58A: bcc.w $2c594
pub fn i_02c58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C58E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C594; return 10; }
    8
}
/// $02C58E: lea.l $2b4ca(pc), a1
pub fn i_02c58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C592;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CA;
    16
}
/// $02C592: move.l a1, (a6)
pub fn i_02c592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C594;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C594: jsr $2a4f0(pc)
pub fn i_02c594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C598;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C598);
    cpu.pc = 0x2A4F0;
    16
}
/// $02C598: bcc.w $2c5a2
pub fn i_02c598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C59C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C5A2; return 10; }
    8
}
/// $02C59C: lea.l $2b4ca(pc), a1
pub fn i_02c59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CA;
    16
}
/// $02C5A0: move.l a1, (a6)
pub fn i_02c5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C5A2: jsr $2a328(pc)
pub fn i_02c5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C5A6);
    cpu.pc = 0x2A328;
    16
}
/// $02C5A6: bcs.w $2c5b0
pub fn i_02c5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5AA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2C5B0; return 10; }
    8
}
/// $02C5AA: lea.l $2c554(pc), a1
pub fn i_02c5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C554;
    16
}
/// $02C5AE: move.l a1, (a6)
pub fn i_02c5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C5B0: jsr $2a34e(pc)
pub fn i_02c5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C5B4);
    cpu.pc = 0x2A34E;
    16
}
/// $02C5B4: bcc.w $2c5be
pub fn i_02c5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5B8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C5BE; return 10; }
    8
}
/// $02C5B8: lea.l $2c648(pc), a1
pub fn i_02c5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C648;
    16
}
/// $02C5BC: move.l a1, (a6)
pub fn i_02c5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C5BE: jsr $28d70.l
pub fn i_02c5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C5C4);
    cpu.pc = 0x28D70;
    16
}
/// $02C5C4: bcc.w $2c5ce
pub fn i_02c5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C5CE; return 10; }
    8
}
/// $02C5C8: lea.l $2bf64(pc), a1
pub fn i_02c5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BF64;
    16
}
/// $02C5CC: move.l a1, (a6)
pub fn i_02c5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C5CE: jsr $283d8.l
pub fn i_02c5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C5D4);
    cpu.pc = 0x283D8;
    16
}
/// $02C5D4: jsr $2a59a(pc)
pub fn i_02c5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C5D8);
    cpu.pc = 0x2A59A;
    16
}
/// $02C5D8: bcc.w $2c5f4
pub fn i_02c5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C5F4; return 10; }
    8
}
/// $02C5DC: jsr $2a9a0(pc)
pub fn i_02c5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C5E0);
    cpu.pc = 0x2A9A0;
    16
}
/// $02C5E0: andi.w #$ff, d0
pub fn i_02c5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5E4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C5E4: lea.l $2a078(pc), a0
pub fn i_02c5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5E8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A078;
    16
}
/// $02C5E8: movea.l #$ffffffff, a1
pub fn i_02c5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5EE;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C5EE: jsr $772.l
pub fn i_02c5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C5F4);
    cpu.pc = 0x772;
    16
}
/// $02C5F4: jsr $2a8c0(pc)
pub fn i_02c5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C5F8);
    cpu.pc = 0x2A8C0;
    16
}
/// $02C5F8: bcc.w $2c602
pub fn i_02c5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C602; return 10; }
    8
}
/// $02C5FC: lea.l $2d02e(pc), a1
pub fn i_02c5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C600;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02C600: move.l a1, (a6)
pub fn i_02c600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C602;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C602: jsr $2aaf0(pc)
pub fn i_02c602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C606;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C606);
    cpu.pc = 0x2AAF0;
    16
}
/// $02C606: lea.l $2c9b0(pc), a0
pub fn i_02c606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C60A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02C60A: movea.l #$ffffffff, a1
pub fn i_02c60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C610;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C610: jsr $772.l
pub fn i_02c610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C616;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C616);
    cpu.pc = 0x772;
    16
}
/// $02C616: jsr $2ac80(pc)
pub fn i_02c616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C61A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C61A);
    cpu.pc = 0x2AC80;
    16
}
/// $02C61A: bcc.w $2c624
pub fn i_02c61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C61E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C624; return 10; }
    8
}
/// $02C61E: lea.l $2dd74(pc), a1
pub fn i_02c61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C622;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02C622: move.l a1, (a6)
pub fn i_02c622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C624;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C624: jsr $2a5cc(pc)
pub fn i_02c624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C628;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C628);
    cpu.pc = 0x2A5CC;
    16
}
/// $02C628: bcc.w $2c632
pub fn i_02c628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C62C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C632; return 10; }
    8
}
/// $02C62C: lea.l $2bbf2(pc), a1
pub fn i_02c62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C630;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02C630: move.l a1, (a6)
pub fn i_02c630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C632;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C632: jsr $2a690(pc)
pub fn i_02c632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C636;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C636);
    cpu.pc = 0x2A690;
    16
}
/// $02C636: lea.l $2a060(pc), a0
pub fn i_02c636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C63A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02C63A: movea.l #$ffffffff, a1
pub fn i_02c63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C640;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02c640  ($02C640, 8 B, thunks/jsr_abs_thunks.c)
/// $02C640: jsr $772.l
pub fn i_02c640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C646);
    cpu.pc = 0x772;
    16
}
/// $02C646: rts
pub fn i_02c646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C648;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_FireMoveLeft_02c648  ($02C648, 206 B, slug/states_02ae3e.s)
/// $02C648: move.w #$fd60, $28(a6)
pub fn i_02c648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C64E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02C64E: clr.w $2c(a6)
pub fn i_02c64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C652;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02C652: lea.l $2c658(pc), a1
pub fn i_02c652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C656;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C658;
    16
}
/// $02C656: move.l a1, (a6)
pub fn i_02c656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C658;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C658: jsr $2aa24(pc)
pub fn i_02c658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C65C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C65C);
    cpu.pc = 0x2AA24;
    16
}
/// $02C65C: jsr $2a7d8(pc)
pub fn i_02c65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C660;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C660);
    cpu.pc = 0x2A7D8;
    16
}
/// $02C660: bcc.w $2c66a
pub fn i_02c660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C664;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C66A; return 10; }
    8
}
/// $02C664: lea.l $2b4ce(pc), a1
pub fn i_02c664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C668;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CE;
    16
}
/// $02C668: move.l a1, (a6)
pub fn i_02c668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C66A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C66A: jsr $2a4f0(pc)
pub fn i_02c66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C66E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C66E);
    cpu.pc = 0x2A4F0;
    16
}
/// $02C66E: bcc.w $2c678
pub fn i_02c66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C672;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C678; return 10; }
    8
}
/// $02C672: lea.l $2b4ce(pc), a1
pub fn i_02c672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C676;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4CE;
    16
}
/// $02C676: move.l a1, (a6)
pub fn i_02c676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C678;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C678: jsr $2a34e(pc)
pub fn i_02c678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C67C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C67C);
    cpu.pc = 0x2A34E;
    16
}
/// $02C67C: bcs.w $2c686
pub fn i_02c67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C680;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2C686; return 10; }
    8
}
/// $02C680: lea.l $2c554(pc), a1
pub fn i_02c680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C684;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C554;
    16
}
/// $02C684: move.l a1, (a6)
pub fn i_02c684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C686;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C686: jsr $2a328(pc)
pub fn i_02c686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C68A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C68A);
    cpu.pc = 0x2A328;
    16
}
/// $02C68A: bcc.w $2c694
pub fn i_02c68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C68E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C694; return 10; }
    8
}
/// $02C68E: lea.l $2c572(pc), a1
pub fn i_02c68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C692;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C572;
    16
}
/// $02C692: move.l a1, (a6)
pub fn i_02c692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C694;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C694: jsr $28d70.l
pub fn i_02c694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C69A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C69A);
    cpu.pc = 0x28D70;
    16
}
/// $02C69A: bcc.w $2c6a4
pub fn i_02c69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C69E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C6A4; return 10; }
    8
}
/// $02C69E: lea.l $2c07a(pc), a1
pub fn i_02c69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C07A;
    16
}
/// $02C6A2: move.l a1, (a6)
pub fn i_02c6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C6A4: jsr $283d8.l
pub fn i_02c6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C6AA);
    cpu.pc = 0x283D8;
    16
}
/// $02C6AA: jsr $2a59a(pc)
pub fn i_02c6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C6AE);
    cpu.pc = 0x2A59A;
    16
}
/// $02C6AE: bcc.w $2c6ca
pub fn i_02c6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C6CA; return 10; }
    8
}
/// $02C6B2: jsr $2a9a0(pc)
pub fn i_02c6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C6B6);
    cpu.pc = 0x2A9A0;
    16
}
/// $02C6B6: andi.w #$ff, d0
pub fn i_02c6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6BA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C6BA: lea.l $2a078(pc), a0
pub fn i_02c6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6BE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A078;
    16
}
/// $02C6BE: movea.l #$ffffffff, a1
pub fn i_02c6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6C4;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C6C4: jsr $772.l
pub fn i_02c6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C6CA);
    cpu.pc = 0x772;
    16
}
/// $02C6CA: jsr $2a8c0(pc)
pub fn i_02c6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C6CE);
    cpu.pc = 0x2A8C0;
    16
}
/// $02C6CE: bcc.w $2c6d8
pub fn i_02c6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C6D8; return 10; }
    8
}
/// $02C6D2: lea.l $2d02e(pc), a1
pub fn i_02c6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02C6D6: move.l a1, (a6)
pub fn i_02c6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C6D8: jsr $2aaf0(pc)
pub fn i_02c6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C6DC);
    cpu.pc = 0x2AAF0;
    16
}
/// $02C6DC: lea.l $2c9b0(pc), a0
pub fn i_02c6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6E0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02C6E0: movea.l #$ffffffff, a1
pub fn i_02c6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6E6;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C6E6: jsr $772.l
pub fn i_02c6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C6EC);
    cpu.pc = 0x772;
    16
}
/// $02C6EC: jsr $2ac80(pc)
pub fn i_02c6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C6F0);
    cpu.pc = 0x2AC80;
    16
}
/// $02C6F0: bcc.w $2c6fa
pub fn i_02c6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C6FA; return 10; }
    8
}
/// $02C6F4: lea.l $2dd74(pc), a1
pub fn i_02c6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02C6F8: move.l a1, (a6)
pub fn i_02c6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C6FA: jsr $2a5cc(pc)
pub fn i_02c6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C6FE);
    cpu.pc = 0x2A5CC;
    16
}
/// $02C6FE: bcc.w $2c708
pub fn i_02c6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C702;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C708; return 10; }
    8
}
/// $02C702: lea.l $2bbf2(pc), a1
pub fn i_02c702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C706;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02C706: move.l a1, (a6)
pub fn i_02c706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C708;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C708: jsr $2a690(pc)
pub fn i_02c708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C70C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C70C);
    cpu.pc = 0x2A690;
    16
}
/// $02C70C: lea.l $2a060(pc), a0
pub fn i_02c70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C710;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02C710: movea.l #$ffffffff, a1
pub fn i_02c710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C716;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02c716  ($02C716, 8 B, thunks/jsr_abs_thunks.c)
/// $02C716: jsr $772.l
pub fn i_02c716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C71C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C71C);
    cpu.pc = 0x772;
    16
}
/// $02C71C: rts
pub fn i_02c71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C71E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_FireSlope_02c71e  ($02C71E, 474 B, slug/states_02ae3e.s)
/// $02C71E: bclr.b #$5, $8c(a6)
pub fn i_02c71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C724;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02C724: move.b #$4, d1
pub fn i_02c724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C728;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02C728: jsr $8f714.l
pub fn i_02c728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C72E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C72E);
    cpu.pc = 0x8F714;
    16
}
/// $02C72E: bcs.w $2c780
pub fn i_02c72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C732;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2C780; return 10; }
    8
}
/// $02C732: cmpi.b #$ff, $84(a6)
pub fn i_02c732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C738;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $02C738: beq.w $2c752
pub fn i_02c738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C73C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C752; return 10; }
    8
}
/// $02C73C: lea.l $312b2.l, a1
pub fn i_02c73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C742;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x312B2;
    20
}
/// $02C742: jsr $6fe.l
pub fn i_02c742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C748;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C748);
    cpu.pc = 0x6FE;
    16
}
/// $02C748: jsr $5dd02.l
pub fn i_02c748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C74E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C74E);
    cpu.pc = 0x5DD02;
    16
}
/// $02C74E: bra.w $2c764
pub fn i_02c74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C752;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C764;
    10
}
/// $02C752: lea.l $31240.l, a1
pub fn i_02c752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C758;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31240;
    20
}
/// $02C758: jsr $6fe.l
pub fn i_02c758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C75E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C75E);
    cpu.pc = 0x6FE;
    16
}
/// $02C75E: jsr $5dd02.l
pub fn i_02c75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C764;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C764);
    cpu.pc = 0x5DD02;
    16
}
/// $02C764: jsr $517fe.l
pub fn i_02c764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C76A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C76A);
    cpu.pc = 0x517FE;
    16
}
/// $02C76A: lea.l $319f0.l, a1
pub fn i_02c76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C770;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x319F0;
    20
}
/// $02C770: jsr $6fe.l
pub fn i_02c770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C776;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C776);
    cpu.pc = 0x6FE;
    16
}
/// $02C776: jsr $5dd02.l
pub fn i_02c776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C77C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C77C);
    cpu.pc = 0x5DD02;
    16
}
/// $02C77C: bra.w $2c798
pub fn i_02c77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C780;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C798;
    10
}
/// $02C780: lea.l $31922.l, a1
pub fn i_02c780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C786;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31922;
    20
}
/// $02C786: jsr $6fe.l
pub fn i_02c786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C78C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C78C);
    cpu.pc = 0x6FE;
    16
}
/// $02C78C: jsr $5dd02.l
pub fn i_02c78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C792;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C792);
    cpu.pc = 0x5DD02;
    16
}
/// $02C792: jsr $517fe.l
pub fn i_02c792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C798);
    cpu.pc = 0x517FE;
    16
}
/// $02C798: cmpi.b #$0, $90(a6)
pub fn i_02c798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C79E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $02C79E: beq.w $2c7a6
pub fn i_02c79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C7A6; return 10; }
    8
}
/// $02C7A2: jsr $2aab0(pc)
pub fn i_02c7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C7A6);
    cpu.pc = 0x2AAB0;
    16
}
/// $02C7A6: move.b #$1e, $8e(a6)
pub fn i_02c7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7AC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02C7AC: jsr $2a958(pc)
pub fn i_02c7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C7B0);
    cpu.pc = 0x2A958;
    16
}
/// $02C7B0: cmp.w $80(a6), d2
pub fn i_02c7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7B4;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02C7B4: beq.w $2c80a
pub fn i_02c7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C80A; return 10; }
    8
}
/// $02C7B8: move.w d2, $80(a6)
pub fn i_02c7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7BC;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02C7BC: move.w d2, d0
pub fn i_02c7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7BE;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02C7BE: asr.w #$1, d0
pub fn i_02c7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7C0;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02C7C0: neg.w d0
pub fn i_02c7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7C2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02C7C2: addi.w #$20, d0
pub fn i_02c7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7C6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C7C6: cmpi.w #$40, d0
pub fn i_02c7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7CA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02C7CA: bls.w $2c7d2
pub fn i_02c7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7CE;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2C7D2; return 10; }
    8
}
/// $02C7CE: move.w #$40, d0
pub fn i_02c7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7D2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02C7D2: movem.w d0, -(a7)
pub fn i_02c7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7D6;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02C7D6: jsr $295a6(pc)
pub fn i_02c7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C7DA);
    cpu.pc = 0x295A6;
    16
}
/// $02C7DA: movea.l #$2b0c30, a0
pub fn i_02c7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7E0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02C7E0: lsl.w #$2, d0
pub fn i_02c7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7E2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C7E2: movea.l (a0, d0.w), a0
pub fn i_02c7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7E6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C7E6: cmpa.l #$ffffffff, a0
pub fn i_02c7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7EC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02C7EC: beq.w $2c7f6
pub fn i_02c7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C7F6; return 10; }
    8
}
/// $02C7F0: jsr $28cd4.l
pub fn i_02c7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C7F6);
    cpu.pc = 0x28CD4;
    16
}
/// $02C7F6: movem.w (a7)+, d0
pub fn i_02c7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C7FA;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02C7FA: move.b #$0, $20(a6)
pub fn i_02c7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C800;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02C800: lsr.w #$2, d0
pub fn i_02c800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C802;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C802: andi.w #$ff, d0
pub fn i_02c802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C806;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C806: add.b d0, $20(a6)
pub fn i_02c806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C80A;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02C80A: bset.b #$0, $8d(a6)
pub fn i_02c80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C810;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02C810: move.l #$295b4, $60(a6)
pub fn i_02c810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C818;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02C818: lea.l $2c81e(pc), a1
pub fn i_02c818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C81C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C81E;
    16
}
/// $02C81C: move.l a1, (a6)
pub fn i_02c81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C81E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C81E: bset.b #$2, $8d(a6)
pub fn i_02c81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C824;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02C824: jsr $2aa0e(pc)
pub fn i_02c824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C828;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C828);
    cpu.pc = 0x2AA0E;
    16
}
/// $02C828: move.w $2a(a6), d0
pub fn i_02c828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C82C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02C82C: move.w #$400, d1
pub fn i_02c82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C830;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02C830: jsr $267f4.l
pub fn i_02c830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C836;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C836);
    cpu.pc = 0x267F4;
    16
}
/// $02C836: move.w d0, $2a(a6)
pub fn i_02c836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C83A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02C83A: jsr $2a85a(pc)
pub fn i_02c83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C83E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C83E);
    cpu.pc = 0x2A85A;
    16
}
/// $02C83E: jsr $2a478(pc)
pub fn i_02c83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C842;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C842);
    cpu.pc = 0x2A478;
    16
}
/// $02C842: jsr $28d70.l
pub fn i_02c842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C848;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C848);
    cpu.pc = 0x28D70;
    16
}
/// $02C848: jsr $2a958(pc)
pub fn i_02c848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C84C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C84C);
    cpu.pc = 0x2A958;
    16
}
/// $02C84C: cmp.w $80(a6), d2
pub fn i_02c84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C850;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02C850: beq.w $2c8a6
pub fn i_02c850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C854;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C8A6; return 10; }
    8
}
/// $02C854: move.w d2, $80(a6)
pub fn i_02c854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C858;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02C858: move.w d2, d0
pub fn i_02c858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C85A;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02C85A: asr.w #$1, d0
pub fn i_02c85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C85C;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02C85C: neg.w d0
pub fn i_02c85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C85E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02C85E: addi.w #$20, d0
pub fn i_02c85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C862;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C862: cmpi.w #$40, d0
pub fn i_02c862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C866;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02C866: bls.w $2c86e
pub fn i_02c866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C86A;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2C86E; return 10; }
    8
}
/// $02C86A: move.w #$40, d0
pub fn i_02c86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C86E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02C86E: movem.w d0, -(a7)
pub fn i_02c86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C872;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02C872: jsr $295a6(pc)
pub fn i_02c872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C876;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C876);
    cpu.pc = 0x295A6;
    16
}
/// $02C876: movea.l #$2b0c30, a0
pub fn i_02c876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C87C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02C87C: lsl.w #$2, d0
pub fn i_02c87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C87E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C87E: movea.l (a0, d0.w), a0
pub fn i_02c87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C882;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C882: cmpa.l #$ffffffff, a0
pub fn i_02c882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C888;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02C888: beq.w $2c892
pub fn i_02c888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C88C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C892; return 10; }
    8
}
/// $02C88C: jsr $28cd4.l
pub fn i_02c88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C892;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C892);
    cpu.pc = 0x28CD4;
    16
}
/// $02C892: movem.w (a7)+, d0
pub fn i_02c892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C896;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02C896: move.b #$0, $20(a6)
pub fn i_02c896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C89C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02C89C: lsr.w #$2, d0
pub fn i_02c89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C89E;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C89E: andi.w #$ff, d0
pub fn i_02c89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8A2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C8A2: add.b d0, $20(a6)
pub fn i_02c8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8A6;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02C8A6: lea.l $2b4d2(pc), a1
pub fn i_02c8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4D2;
    16
}
/// $02C8AA: move.l a1, (a6)
pub fn i_02c8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C8AC: jsr $2a8c0(pc)
pub fn i_02c8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C8B0);
    cpu.pc = 0x2A8C0;
    16
}
/// $02C8B0: bcc.w $2c8ba
pub fn i_02c8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C8BA; return 10; }
    8
}
/// $02C8B4: lea.l $2d02e(pc), a1
pub fn i_02c8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02C8B8: move.l a1, (a6)
pub fn i_02c8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C8BA: jsr $2aaf0(pc)
pub fn i_02c8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C8BE);
    cpu.pc = 0x2AAF0;
    16
}
/// $02C8BE: lea.l $2c9b0(pc), a0
pub fn i_02c8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8C2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02C8C2: movea.l #$ffffffff, a1
pub fn i_02c8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8C8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02C8C8: jsr $772.l
pub fn i_02c8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C8CE);
    cpu.pc = 0x772;
    16
}
/// $02C8CE: jsr $2aac0(pc)
pub fn i_02c8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C8D2);
    cpu.pc = 0x2AAC0;
    16
}
/// $02C8D2: bcc.w $2c8dc
pub fn i_02c8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C8DC; return 10; }
    8
}
/// $02C8D6: lea.l $2c71e(pc), a1
pub fn i_02c8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C71E;
    16
}
/// $02C8DA: move.l a1, (a6)
pub fn i_02c8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C8DC: jsr $2ac80(pc)
pub fn i_02c8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C8E0);
    cpu.pc = 0x2AC80;
    16
}
/// $02C8E0: bcc.w $2c8ea
pub fn i_02c8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C8EA; return 10; }
    8
}
/// $02C8E4: lea.l $2dd74(pc), a1
pub fn i_02c8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02C8E8: move.l a1, (a6)
pub fn i_02c8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C8EA: jsr $2a690(pc)
pub fn i_02c8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C8EE);
    cpu.pc = 0x2A690;
    16
}
/// $02C8EE: lea.l $2a060(pc), a0
pub fn i_02c8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8F2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02C8F2: movea.l #$ffffffff, a1
pub fn i_02c8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8F8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02c8f8  ($02C8F8, 8 B, thunks/jsr_abs_thunks.c)
/// $02C8F8: jsr $772.l
pub fn i_02c8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C8FE);
    cpu.pc = 0x772;
    16
}
/// $02C8FE: rts
pub fn i_02c8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C900;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_JumpCrouch_02c908  ($02C908, 84 B, slug/states_02ae3e.s)
/// $02C908: bclr.b #$5, $8c(a6)
pub fn i_02c908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C90E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02C90E: clr.w $2c(a6)
pub fn i_02c90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C912;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02C912: jsr $2a9a0(pc)
pub fn i_02c912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C916;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C916);
    cpu.pc = 0x2A9A0;
    16
}
/// $02C916: movea.l #$279308, a0
pub fn i_02c916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C91C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279308;
    cpu.a[0] = v1;
    4
}
/// $02C91C: lsl.w #$2, d0
pub fn i_02c91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C91E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C91E: movea.l (a0, d0.w), a0
pub fn i_02c91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C922;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C922: cmpa.l #$ffffffff, a0
pub fn i_02c922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C928;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02C928: beq.w $2c932
pub fn i_02c928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C92C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C932; return 10; }
    8
}
/// $02C92C: jsr $28cd4.l
pub fn i_02c92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C932);
    cpu.pc = 0x28CD4;
    16
}
/// $02C932: lea.l $2a038(pc), a1
pub fn i_02c932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C936;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2A038;
    16
}
/// $02C936: movea.l (a1, d0.w), a0
pub fn i_02c936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C93A;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02C93A: move.l a0, $48(a6)
pub fn i_02c93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C93E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02C93E: move.b #$2d, $20(a6)
pub fn i_02c93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C944;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02C944: andi.w #$ff, d0
pub fn i_02c944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C948;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02C948: lsr.w #$2, d0
pub fn i_02c948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C94A;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02C94A: add.b d0, $20(a6)
pub fn i_02c94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C94E;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02C94E: move.l #$295b4, $60(a6)
pub fn i_02c94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C956;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02C956: lea.l $2c95c(pc), a1
pub fn i_02c956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C95A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C95C;
    16
}
/// $02C95A: move.l a1, (a6)
pub fn i_02c95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C95C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Slug_JumpCrouch_Loop_02c95c  ($02C95C, 76 B, slug/states_02ae3e.s)
/// $02C95C: jsr $2aa24(pc)
pub fn i_02c95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C960;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C960);
    cpu.pc = 0x2AA24;
    16
}
/// $02C960: jsr $2a85a(pc)
pub fn i_02c960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C964;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C964);
    cpu.pc = 0x2A85A;
    16
}
/// $02C964: jsr $2a478(pc)
pub fn i_02c964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C968;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C968);
    cpu.pc = 0x2A478;
    16
}
/// $02C968: jsr $28d70.l
pub fn i_02c968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C96E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C96E);
    cpu.pc = 0x28D70;
    16
}
/// $02C96E: bcc.w $2c978
pub fn i_02c96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C972;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C978; return 10; }
    8
}
/// $02C972: lea.l $2ca0c(pc), a1
pub fn i_02c972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C976;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CA0C;
    16
}
/// $02C976: move.l a1, (a6)
pub fn i_02c976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C978;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C978: jsr $283d8.l
pub fn i_02c978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C97E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C97E);
    cpu.pc = 0x283D8;
    16
}
/// $02C97E: jsr $2aac0(pc)
pub fn i_02c97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C982;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C982);
    cpu.pc = 0x2AAC0;
    16
}
/// $02C982: bcc.w $2c98c
pub fn i_02c982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C986;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C98C; return 10; }
    8
}
/// $02C986: lea.l $2d286(pc), a1
pub fn i_02c986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C98A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D286;
    16
}
/// $02C98A: move.l a1, (a6)
pub fn i_02c98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C98C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C98C: jsr $2ac80(pc)
pub fn i_02c98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C990;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C990);
    cpu.pc = 0x2AC80;
    16
}
/// $02C990: bcc.w $2c99a
pub fn i_02c990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C994;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2C99A; return 10; }
    8
}
/// $02C994: lea.l $2dd74(pc), a1
pub fn i_02c994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C998;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02C998: move.l a1, (a6)
pub fn i_02c998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C99A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02C99A: jsr $2a690(pc)
pub fn i_02c99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C99E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2C99E);
    cpu.pc = 0x2A690;
    16
}
/// $02C99E: lea.l $2a060(pc), a0
pub fn i_02c99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9A2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02C9A2: movea.l #$ffffffff, a1
pub fn i_02c9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9A8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02c9a8  ($02C9A8, 8 B, thunks/jsr_abs_thunks.c)
/// $02C9A8: jsr $772.l
pub fn i_02c9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C9AE);
    cpu.pc = 0x772;
    16
}
/// $02C9AE: rts
pub fn i_02c9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_JumpNeutral_02c9c4  ($02C9C4, 16 B, slug/states_02ae3e.s)
/// $02C9C4: bclr.b #$5, $8c(a6)
pub fn i_02c9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9CA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02C9CA: move.w #$0, $28(a6)
pub fn i_02c9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02C9D0: bra.w $2c908
pub fn i_02c9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C908;
    10
}
// ===== Slug_JumpRight_02c9d4  ($02C9D4, 28 B, slug/states_02ae3e.s)
/// $02C9D4: cmpi.w #$3, $94(a6)
pub fn i_02c9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9DA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $02C9DA: bne.w $2c9e0
pub fn i_02c9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2C9E0; return 10; }
    8
}
/// $02C9DE: bra.b $2c9c4
pub fn i_02c9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9E0;
    cpu.ir = 0x60E4;
    cpu.pc = 0x2C9C4;
    10
}
/// $02C9E0: bclr.b #$5, $8c(a6)
pub fn i_02c9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9E6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02C9E6: move.w #$2aa, $28(a6)
pub fn i_02c9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2AA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02C9EC: bra.w $2c908
pub fn i_02c9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C908;
    10
}
// ===== Slug_JumpLeft_02c9f0  ($02C9F0, 28 B, slug/states_02ae3e.s)
/// $02C9F0: cmpi.w #$4, $94(a6)
pub fn i_02c9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9F6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $02C9F6: bne.w $2c9fc
pub fn i_02c9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9FA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2C9FC; return 10; }
    8
}
/// $02C9FA: bra.b $2c9c4
pub fn i_02c9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9FC;
    cpu.ir = 0x60C8;
    cpu.pc = 0x2C9C4;
    10
}
/// $02C9FC: bclr.b #$5, $8c(a6)
pub fn i_02c9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA02;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CA02: move.w #$fd56, $28(a6)
pub fn i_02ca02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA08;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD56;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CA08: bra.w $2c908
pub fn i_02ca08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C908;
    10
}
// ===== Slug_JumpLaunch_02ca0c  ($02CA0C, 126 B, slug/states_02ae3e.s)
/// $02CA0C: bclr.b #$3, $5b(a6)
pub fn i_02ca0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA12;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CA12: bclr.b #$5, $8c(a6)
pub fn i_02ca12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA18;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CA18: move.w #$4a4, $2a(a6)
pub fn i_02ca18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4A4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CA1E: move.w #$ff9d, $2e(a6)
pub fn i_02ca1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF9D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CA24: move.w $24(a6), d0
pub fn i_02ca24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA28;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02CA28: cmp.w $82(a6), d0
pub fn i_02ca28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA2C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02CA2C: ble.w $2ca34
pub fn i_02ca2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA30;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2CA34; return 10; }
    8
}
/// $02CA30: move.w d0, $82(a6)
pub fn i_02ca30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA34;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CA34: jsr $2a9a0(pc)
pub fn i_02ca34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA38;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CA38);
    cpu.pc = 0x2A9A0;
    16
}
/// $02CA38: movea.l #$27931c, a0
pub fn i_02ca38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA3E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x27931C;
    cpu.a[0] = v1;
    4
}
/// $02CA3E: lsl.w #$2, d0
pub fn i_02ca3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA40;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CA40: movea.l (a0, d0.w), a0
pub fn i_02ca40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA44;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02CA44: cmpa.l #$ffffffff, a0
pub fn i_02ca44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA4A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02CA4A: beq.w $2ca54
pub fn i_02ca4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA4E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CA54; return 10; }
    8
}
/// $02CA4E: jsr $28cd4.l
pub fn i_02ca4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CA54);
    cpu.pc = 0x28CD4;
    16
}
/// $02CA54: lea.l $2a04c(pc), a1
pub fn i_02ca54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2A04C;
    16
}
/// $02CA58: movea.l (a1, d0.w), a0
pub fn i_02ca58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA5C;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02CA5C: move.l a0, $48(a6)
pub fn i_02ca5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA60;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02CA60: move.b #$32, $20(a6)
pub fn i_02ca60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA66;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02CA66: andi.w #$ff, d0
pub fn i_02ca66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA6A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CA6A: lsr.w #$2, d0
pub fn i_02ca6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA6C;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CA6C: add.b d0, $20(a6)
pub fn i_02ca6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA70;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02CA70: lea.l $2ca8a(pc), a1
pub fn i_02ca70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CA8A;
    16
}
/// $02CA74: move.l a1, (a6)
pub fn i_02ca74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CA76: jsr $2a958(pc)
pub fn i_02ca76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA7A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CA7A);
    cpu.pc = 0x2A958;
    16
}
/// $02CA7A: cmp.w $80(a6), d2
pub fn i_02ca7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA7E;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02CA7E: beq.w $2ca86
pub fn i_02ca7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CA86; return 10; }
    8
}
/// $02CA82: move.w d2, $80(a6)
pub fn i_02ca82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA86;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CA86: jsr $2a478(pc)
pub fn i_02ca86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CA8A);
    cpu.pc = 0x2A478;
    16
}
// ===== Slug_JumpAir_02ca8a  ($02CA8A, 376 B, slug/states_02ae3e.s)
/// $02CA8A: jsr $2aac0(pc)
pub fn i_02ca8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA8E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CA8E);
    cpu.pc = 0x2AAC0;
    16
}
/// $02CA8E: bcc.w $2ca98
pub fn i_02ca8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CA98; return 10; }
    8
}
/// $02CA92: lea.l $2d298(pc), a1
pub fn i_02ca92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D298;
    16
}
/// $02CA96: move.l a1, (a6)
pub fn i_02ca96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CA98: jsr $5d5b6.l
pub fn i_02ca98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CA9E);
    cpu.pc = 0x5D5B6;
    16
}
/// $02CA9E: asl.w #$1, d0
pub fn i_02ca9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAA0;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02CAA0: lea.l $2c900(pc), a0
pub fn i_02caa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAA4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C900;
    16
}
/// $02CAA4: move.w (a0, d0.w), $2c(a6)
pub fn i_02caa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAAA;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $02CAAA: move.w $28(a6), d0
pub fn i_02caaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAAE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02CAAE: move.w #$2a0, d1
pub fn i_02caae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAB2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CAB2: jsr $267f4.l
pub fn i_02cab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CAB8);
    cpu.pc = 0x267F4;
    16
}
/// $02CAB8: move.w d0, $28(a6)
pub fn i_02cab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CABC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CABC: move.w $2a(a6), d0
pub fn i_02cabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAC0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02CAC0: bgt.w $2cad2
pub fn i_02cac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAC4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2CAD2; return 10; }
    8
}
/// $02CAC4: move.w #$400, d1
pub fn i_02cac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAC8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CAC8: jsr $267f4.l
pub fn i_02cac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CACE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CACE);
    cpu.pc = 0x267F4;
    16
}
/// $02CACE: move.w d0, $2a(a6)
pub fn i_02cace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAD2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CAD2: bset.b #$1, $8d(a6)
pub fn i_02cad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAD8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CAD8: jsr $2aa0e(pc)
pub fn i_02cad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CADC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CADC);
    cpu.pc = 0x2AA0E;
    16
}
/// $02CADC: jsr $2a878(pc)
pub fn i_02cadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CAE0);
    cpu.pc = 0x2A878;
    16
}
/// $02CAE0: bcc.w $2cb1a
pub fn i_02cae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAE4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CB1A; return 10; }
    8
}
/// $02CAE4: lea.l $2cc1a(pc), a1
pub fn i_02cae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CC1A;
    16
}
/// $02CAE8: move.l a1, (a6)
pub fn i_02cae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CAEA: jsr $5cef8.l
pub fn i_02caea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CAF0);
    cpu.pc = 0x5CEF8;
    16
}
/// $02CAF0: bcc.w $2cb0c
pub fn i_02caf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAF4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CB0C; return 10; }
    8
}
/// $02CAF4: jsr $2a9a0(pc)
pub fn i_02caf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CAF8);
    cpu.pc = 0x2A9A0;
    16
}
/// $02CAF8: andi.w #$ff, d0
pub fn i_02caf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAFC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CAFC: lea.l $2a08c(pc), a0
pub fn i_02cafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB00;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A08C;
    16
}
/// $02CB00: movea.l #$ffffffff, a1
pub fn i_02cb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB06;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02CB06: jsr $772.l
pub fn i_02cb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CB0C);
    cpu.pc = 0x772;
    16
}
/// $02CB0C: jsr $2aac0(pc)
pub fn i_02cb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CB10);
    cpu.pc = 0x2AAC0;
    16
}
/// $02CB10: bcc.w $2cb1a
pub fn i_02cb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CB1A; return 10; }
    8
}
/// $02CB14: lea.l $2c24a(pc), a1
pub fn i_02cb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02CB18: move.l a1, (a6)
pub fn i_02cb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CB1A: jsr $28d70.l
pub fn i_02cb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CB20);
    cpu.pc = 0x28D70;
    16
}
/// $02CB20: tst.w $2a(a6)
pub fn i_02cb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB24;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02CB24: bgt.w $2cbb0
pub fn i_02cb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB28;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2CBB0; return 10; }
    8
}
/// $02CB28: jsr $2a8c0(pc)
pub fn i_02cb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CB2C);
    cpu.pc = 0x2A8C0;
    16
}
/// $02CB2C: cmpi.b #$0, d1
pub fn i_02cb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB30;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $02CB30: ble.w $2cbb0
pub fn i_02cb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB34;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2CBB0; return 10; }
    8
}
/// $02CB34: clr.w $28(a6)
pub fn i_02cb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB38;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02CB38: clr.w $2c(a6)
pub fn i_02cb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB3C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02CB3C: jsr $2a958(pc)
pub fn i_02cb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CB40);
    cpu.pc = 0x2A958;
    16
}
/// $02CB40: cmp.w $80(a6), d2
pub fn i_02cb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB44;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02CB44: beq.w $2cbb0
pub fn i_02cb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB48;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CBB0; return 10; }
    8
}
/// $02CB48: move.w d2, $80(a6)
pub fn i_02cb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB4C;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CB4C: move.w d2, d0
pub fn i_02cb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB4E;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CB4E: asr.w #$1, d0
pub fn i_02cb4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB50;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02CB50: neg.w d0
pub fn i_02cb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB52;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02CB52: addi.w #$20, d0
pub fn i_02cb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB56;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CB56: cmpi.w #$40, d0
pub fn i_02cb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB5A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02CB5A: bls.w $2cb62
pub fn i_02cb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB5E;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2CB62; return 10; }
    8
}
/// $02CB5E: move.w #$40, d0
pub fn i_02cb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB62;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CB62: movem.w d0, -(a7)
pub fn i_02cb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB66;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02CB66: jsr $295a6(pc)
pub fn i_02cb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CB6A);
    cpu.pc = 0x295A6;
    16
}
/// $02CB6A: movea.l #$2b0c30, a0
pub fn i_02cb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB70;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02CB70: lsl.w #$2, d0
pub fn i_02cb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB72;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CB72: movea.l (a0, d0.w), a0
pub fn i_02cb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB76;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02CB76: cmpa.l #$ffffffff, a0
pub fn i_02cb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB7C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02CB7C: beq.w $2cb86
pub fn i_02cb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CB86; return 10; }
    8
}
/// $02CB80: jsr $28cd4.l
pub fn i_02cb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CB86);
    cpu.pc = 0x28CD4;
    16
}
/// $02CB86: movem.w (a7)+, d0
pub fn i_02cb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB8A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02CB8A: move.b #$0, $20(a6)
pub fn i_02cb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB90;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02CB90: lsr.w #$2, d0
pub fn i_02cb90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB92;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CB92: andi.w #$ff, d0
pub fn i_02cb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB96;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CB96: add.b d0, $20(a6)
pub fn i_02cb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB9A;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02CB9A: bset.b #$2, $8d(a6)
pub fn i_02cb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBA0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CBA0: lea.l $2ca8a(pc), a0
pub fn i_02cba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBA4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2CA8A;
    16
}
/// $02CBA4: cmpa.l (a6), a0
pub fn i_02cba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBA6;
    cpu.ir = 0xB1D6;
    let v2 = bus.read32(cpu.a[6]);
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    14
}
/// $02CBA6: bne.w $2cbb0
pub fn i_02cba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBAA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2CBB0; return 10; }
    8
}
/// $02CBAA: lea.l $2cc02(pc), a1
pub fn i_02cbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CC02;
    16
}
/// $02CBAE: move.l a1, (a6)
pub fn i_02cbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CBB0: btst.b #$1, $8d(a6)
pub fn i_02cbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBB6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02CBB6: beq.w $2cbd2
pub fn i_02cbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBBA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CBD2; return 10; }
    8
}
/// $02CBBA: jsr $2a690(pc)
pub fn i_02cbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBBE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CBBE);
    cpu.pc = 0x2A690;
    16
}
/// $02CBBE: lea.l $2a06c(pc), a0
pub fn i_02cbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBC2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A06C;
    16
}
/// $02CBC2: movea.l #$ffffffff, a1
pub fn i_02cbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBC8;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02CBC8: jsr $772.l
pub fn i_02cbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CBCE);
    cpu.pc = 0x772;
    16
}
/// $02CBCE: bra.w $2cbe6
pub fn i_02cbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBD2;
    cpu.ir = 0x6000;
    cpu.pc = 0x2CBE6;
    10
}
/// $02CBD2: jsr $2a690(pc)
pub fn i_02cbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CBD6);
    cpu.pc = 0x2A690;
    16
}
/// $02CBD6: lea.l $2a060(pc), a0
pub fn i_02cbd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBDA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02CBDA: movea.l #$ffffffff, a1
pub fn i_02cbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBE0;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02CBE0: jsr $772.l
pub fn i_02cbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CBE6);
    cpu.pc = 0x772;
    16
}
/// $02CBE6: movea.l #$ffffffff, a0
pub fn i_02cbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBEC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02CBEC: lea.l $27964e.l, a0
pub fn i_02cbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02CBF2: jsr $5dd56.l
pub fn i_02cbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CBF8);
    cpu.pc = 0x5DD56;
    16
}
/// $02CBF8: bcc.w $2cc00
pub fn i_02cbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CC00; return 10; }
    8
}
/// $02CBFC: jmp $2dcbc(pc)
pub fn i_02cbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC00;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02CC00: rts
pub fn i_02cc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_JumpAirFire_02cc02  ($02CC02, 24 B, slug/states_02ae3e.s)
/// $02CC02: bset.b #$2, $8d(a6)
pub fn i_02cc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC08;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CC08: jsr $2aac0(pc)
pub fn i_02cc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CC0C);
    cpu.pc = 0x2AAC0;
    16
}
/// $02CC0C: bcc.w $2cc16
pub fn i_02cc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC10;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CC16; return 10; }
    8
}
/// $02CC10: lea.l $2d380(pc), a1
pub fn i_02cc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D380;
    16
}
/// $02CC14: move.l a1, (a6)
pub fn i_02cc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CC16: bra.w $2ca98
pub fn i_02cc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x2CA98;
    10
}
// ===== Slug_JumpLand_02cc1a  ($02CC1A, 506 B, slug/states_02ae3e.s)
/// $02CC1A: bclr.b #$5, $8c(a6)
pub fn i_02cc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC20;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CC20: bset.b #$7, $5b(a6)
pub fn i_02cc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC26;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CC26: clr.w $2e(a6)
pub fn i_02cc26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC2A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02CC2A: clr.w $2a(a6)
pub fn i_02cc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC2E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02CC2E: tst.w $28(a6)
pub fn i_02cc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC32;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02CC32: beq.w $2cc4a
pub fn i_02cc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC36;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CC4A; return 10; }
    8
}
/// $02CC36: move.w $28(a6), d1
pub fn i_02cc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC3A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02CC3A: ext.l d1
pub fn i_02cc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC3C;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02CC3C: swap d1
pub fn i_02cc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC3E;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02CC3E: move.w #$80, d0
pub fn i_02cc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC42;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CC42: eor.w d1, d0
pub fn i_02cc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC44;
    cpu.ir = 0xB340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02CC44: sub.w d1, d0
pub fn i_02cc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC46;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02CC46: move.w d0, $28(a6)
pub fn i_02cc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC4A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CC4A: jsr $2a958(pc)
pub fn i_02cc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CC4E);
    cpu.pc = 0x2A958;
    16
}
/// $02CC4E: move.w d2, $80(a6)
pub fn i_02cc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC52;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CC52: jsr $2a4ec(pc)
pub fn i_02cc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CC56);
    cpu.pc = 0x2A4EC;
    16
}
/// $02CC56: bcs.w $2cc5e
pub fn i_02cc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC5A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2CC5E; return 10; }
    8
}
/// $02CC5A: bra.w $2cc68
pub fn i_02cc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x2CC68;
    10
}
/// $02CC5E: lea.l $2ce1c(pc), a1
pub fn i_02cc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CE1C;
    16
}
/// $02CC62: move.l a1, (a6)
pub fn i_02cc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CC64: bra.w $2ce1c
pub fn i_02cc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC68;
    cpu.ir = 0x6000;
    cpu.pc = 0x2CE1C;
    10
}
/// $02CC68: move.b #$11, $91(a6)
pub fn i_02cc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC6E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02CC6E: jsr $2a9a0(pc)
pub fn i_02cc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CC72);
    cpu.pc = 0x2A9A0;
    16
}
/// $02CC72: movea.l #$279330, a0
pub fn i_02cc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC78;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279330;
    cpu.a[0] = v1;
    4
}
/// $02CC78: lsl.w #$2, d0
pub fn i_02cc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC7A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CC7A: movea.l (a0, d0.w), a0
pub fn i_02cc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC7E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02CC7E: cmpa.l #$ffffffff, a0
pub fn i_02cc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC84;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02CC84: beq.w $2cc8e
pub fn i_02cc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC88;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CC8E; return 10; }
    8
}
/// $02CC88: jsr $28cd4.l
pub fn i_02cc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CC8E);
    cpu.pc = 0x28CD4;
    16
}
/// $02CC8E: lea.l $29ffc(pc), a1
pub fn i_02cc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x29FFC;
    16
}
/// $02CC92: movea.l (a1, d0.w), a0
pub fn i_02cc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC96;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02CC96: move.l a0, $48(a6)
pub fn i_02cc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC9A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02CC9A: move.b #$37, $20(a6)
pub fn i_02cc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCA0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x37;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02CCA0: andi.w #$ff, d0
pub fn i_02cca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCA4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CCA4: lsr.w #$2, d0
pub fn i_02cca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCA6;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CCA6: add.b d0, $20(a6)
pub fn i_02cca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCAA;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02CCAA: lea.l $2ccb0(pc), a1
pub fn i_02ccaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CCB0;
    16
}
/// $02CCAE: move.l a1, (a6)
pub fn i_02ccae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CCB0: jsr $2aa24(pc)
pub fn i_02ccb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CCB4);
    cpu.pc = 0x2AA24;
    16
}
/// $02CCB4: jsr $5cef8.l
pub fn i_02ccb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CCBA);
    cpu.pc = 0x5CEF8;
    16
}
/// $02CCBA: bcc.w $2ccd6
pub fn i_02ccba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCBE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CCD6; return 10; }
    8
}
/// $02CCBE: jsr $2a9a0(pc)
pub fn i_02ccbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CCC2);
    cpu.pc = 0x2A9A0;
    16
}
/// $02CCC2: andi.w #$ff, d0
pub fn i_02ccc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCC6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CCC6: lea.l $2a08c(pc), a0
pub fn i_02ccc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCCA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A08C;
    16
}
/// $02CCCA: movea.l #$ffffffff, a1
pub fn i_02ccca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCD0;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02CCD0: jsr $772.l
pub fn i_02ccd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CCD6);
    cpu.pc = 0x772;
    16
}
/// $02CCD6: jsr $5d5b6.l
pub fn i_02ccd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CCDC);
    cpu.pc = 0x5D5B6;
    16
}
/// $02CCDC: asl.w #$1, d0
pub fn i_02ccdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCDE;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02CCDE: lea.l $2c900(pc), a0
pub fn i_02ccde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCE2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C900;
    16
}
/// $02CCE2: move.w (a0, d0.w), $2c(a6)
pub fn i_02cce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCE8;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $02CCE8: tst.w $2c(a6)
pub fn i_02cce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCEC;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02CCEC: bne.w $2ccfc
pub fn i_02ccec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCF0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2CCFC; return 10; }
    8
}
/// $02CCF0: move.w $28(a6), d0
pub fn i_02ccf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCF4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02CCF4: neg.w d0
pub fn i_02ccf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCF6;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02CCF6: asr.w #$3, d0
pub fn i_02ccf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCF8;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02CCF8: move.w d0, $2c(a6)
pub fn i_02ccf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCFC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CCFC: move.w $28(a6), d0
pub fn i_02ccfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD00;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02CD00: move.w #$2a0, d1
pub fn i_02cd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD04;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CD04: jsr $267f4.l
pub fn i_02cd04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CD0A);
    cpu.pc = 0x267F4;
    16
}
/// $02CD0A: move.w d0, $28(a6)
pub fn i_02cd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD0E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CD0E: jsr $2a7d8(pc)
pub fn i_02cd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CD12);
    cpu.pc = 0x2A7D8;
    16
}
/// $02CD12: jsr $2a478(pc)
pub fn i_02cd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CD16);
    cpu.pc = 0x2A478;
    16
}
/// $02CD16: subi.b #$1, $91(a6)
pub fn i_02cd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD1C;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CD1C: jsr $28d70.l
pub fn i_02cd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CD22);
    cpu.pc = 0x28D70;
    16
}
/// $02CD22: bcc.w $2cd34
pub fn i_02cd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CD34; return 10; }
    8
}
/// $02CD26: clr.w $28(a6)
pub fn i_02cd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD2A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02CD2A: clr.w $2c(a6)
pub fn i_02cd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD2E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02CD2E: lea.l $2b4d2(pc), a1
pub fn i_02cd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD32;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4D2;
    16
}
/// $02CD32: move.l a1, (a6)
pub fn i_02cd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD34;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CD34: tst.b $91(a6)
pub fn i_02cd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD38;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02CD38: bne.w $2cd4a
pub fn i_02cd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD3C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2CD4A; return 10; }
    8
}
/// $02CD3C: clr.w $28(a6)
pub fn i_02cd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD40;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02CD40: clr.w $2c(a6)
pub fn i_02cd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD44;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02CD44: lea.l $2b4d2(pc), a1
pub fn i_02cd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4D2;
    16
}
/// $02CD48: move.l a1, (a6)
pub fn i_02cd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CD4A: jsr $2a958(pc)
pub fn i_02cd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CD4E);
    cpu.pc = 0x2A958;
    16
}
/// $02CD4E: cmp.w $80(a6), d2
pub fn i_02cd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD52;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02CD52: beq.w $2cdb4
pub fn i_02cd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD56;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CDB4; return 10; }
    8
}
/// $02CD56: move.w d2, $80(a6)
pub fn i_02cd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD5A;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CD5A: move.w d2, d0
pub fn i_02cd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD5C;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CD5C: asr.w #$1, d0
pub fn i_02cd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD5E;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02CD5E: neg.w d0
pub fn i_02cd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD60;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02CD60: addi.w #$20, d0
pub fn i_02cd60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD64;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CD64: cmpi.w #$40, d0
pub fn i_02cd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD68;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02CD68: bls.w $2cd70
pub fn i_02cd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD6C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2CD70; return 10; }
    8
}
/// $02CD6C: move.w #$40, d0
pub fn i_02cd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CD70: movem.w d0, -(a7)
pub fn i_02cd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD74;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02CD74: jsr $295a6(pc)
pub fn i_02cd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CD78);
    cpu.pc = 0x295A6;
    16
}
/// $02CD78: movea.l #$2b0c30, a0
pub fn i_02cd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD7E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02CD7E: lsl.w #$2, d0
pub fn i_02cd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD80;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CD80: movea.l (a0, d0.w), a0
pub fn i_02cd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD84;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02CD84: cmpa.l #$ffffffff, a0
pub fn i_02cd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD8A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02CD8A: beq.w $2cd94
pub fn i_02cd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD8E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CD94; return 10; }
    8
}
/// $02CD8E: jsr $28cd4.l
pub fn i_02cd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CD94);
    cpu.pc = 0x28CD4;
    16
}
/// $02CD94: movem.w (a7)+, d0
pub fn i_02cd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD98;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02CD98: move.b #$0, $20(a6)
pub fn i_02cd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD9E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02CD9E: lsr.w #$2, d0
pub fn i_02cd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDA0;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CDA0: andi.w #$ff, d0
pub fn i_02cda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDA4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CDA4: add.b d0, $20(a6)
pub fn i_02cda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDA8;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02CDA8: bset.b #$2, $8d(a6)
pub fn i_02cda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDAE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CDAE: lea.l $2ce7e(pc), a1
pub fn i_02cdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CE7E;
    16
}
/// $02CDB2: move.l a1, (a6)
pub fn i_02cdb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CDB4: jsr $283d8.l
pub fn i_02cdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CDBA);
    cpu.pc = 0x283D8;
    16
}
/// $02CDBA: jsr $2a8c0(pc)
pub fn i_02cdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDBE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CDBE);
    cpu.pc = 0x2A8C0;
    16
}
/// $02CDBE: bcc.w $2cdc8
pub fn i_02cdbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CDC8; return 10; }
    8
}
/// $02CDC2: lea.l $2d02e(pc), a1
pub fn i_02cdc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02CDC6: move.l a1, (a6)
pub fn i_02cdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CDC8: jsr $2ab3c(pc)
pub fn i_02cdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDCC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CDCC);
    cpu.pc = 0x2AB3C;
    16
}
/// $02CDCC: lea.l $2c9b0(pc), a0
pub fn i_02cdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDD0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02CDD0: movea.l #$ffffffff, a1
pub fn i_02cdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDD6;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02CDD6: jsr $772.l
pub fn i_02cdd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CDDC);
    cpu.pc = 0x772;
    16
}
/// $02CDDC: jsr $2aac0(pc)
pub fn i_02cddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CDE0);
    cpu.pc = 0x2AAC0;
    16
}
/// $02CDE0: bcc.w $2cdea
pub fn i_02cde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDE4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CDEA; return 10; }
    8
}
/// $02CDE4: lea.l $2c24a(pc), a1
pub fn i_02cde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02CDE8: move.l a1, (a6)
pub fn i_02cde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CDEA: jsr $2ac80(pc)
pub fn i_02cdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CDEE);
    cpu.pc = 0x2AC80;
    16
}
/// $02CDEE: bcc.w $2cdf8
pub fn i_02cdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CDF8; return 10; }
    8
}
/// $02CDF2: lea.l $2dd74(pc), a1
pub fn i_02cdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02CDF6: move.l a1, (a6)
pub fn i_02cdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CDF8: jsr $2a5cc(pc)
pub fn i_02cdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CDFC);
    cpu.pc = 0x2A5CC;
    16
}
/// $02CDFC: bcc.w $2ce06
pub fn i_02cdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CE06; return 10; }
    8
}
/// $02CE00: lea.l $2bbf2(pc), a1
pub fn i_02ce00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02CE04: move.l a1, (a6)
pub fn i_02ce04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CE06: jsr $2a690(pc)
pub fn i_02ce06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE0A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CE0A);
    cpu.pc = 0x2A690;
    16
}
/// $02CE0A: lea.l $2a060(pc), a0
pub fn i_02ce0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE0E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02CE0E: movea.l #$ffffffff, a1
pub fn i_02ce0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE14;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02ce14  ($02CE14, 8 B, thunks/jsr_abs_thunks.c)
/// $02CE14: jsr $772.l
pub fn i_02ce14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CE1A);
    cpu.pc = 0x772;
    16
}
/// $02CE1A: rts
pub fn i_02ce1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_JumpLandSlope_02ce1c  ($02CE1C, 470 B, slug/states_02ae3e.s)
/// $02CE1C: move.b #$11, $91(a6)
pub fn i_02ce1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE22;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02CE22: jsr $2a958(pc)
pub fn i_02ce22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CE26);
    cpu.pc = 0x2A958;
    16
}
/// $02CE26: move.w d2, $80(a6)
pub fn i_02ce26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE2A;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CE2A: move.w d2, d0
pub fn i_02ce2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE2C;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CE2C: asr.w #$1, d0
pub fn i_02ce2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE2E;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02CE2E: neg.w d0
pub fn i_02ce2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE30;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02CE30: addi.w #$20, d0
pub fn i_02ce30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE34;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CE34: cmpi.w #$40, d0
pub fn i_02ce34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE38;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02CE38: bls.w $2ce40
pub fn i_02ce38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE3C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2CE40; return 10; }
    8
}
/// $02CE3C: move.w #$40, d0
pub fn i_02ce3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE40;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CE40: movem.w d0, -(a7)
pub fn i_02ce40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE44;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02CE44: jsr $295a6(pc)
pub fn i_02ce44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CE48);
    cpu.pc = 0x295A6;
    16
}
/// $02CE48: movea.l #$2b0c30, a0
pub fn i_02ce48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE4E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02CE4E: lsl.w #$2, d0
pub fn i_02ce4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE50;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CE50: movea.l (a0, d0.w), a0
pub fn i_02ce50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE54;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02CE54: cmpa.l #$ffffffff, a0
pub fn i_02ce54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE5A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02CE5A: beq.w $2ce64
pub fn i_02ce5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE5E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CE64; return 10; }
    8
}
/// $02CE5E: jsr $28cd4.l
pub fn i_02ce5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CE64);
    cpu.pc = 0x28CD4;
    16
}
/// $02CE64: movem.w (a7)+, d0
pub fn i_02ce64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE68;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02CE68: move.b #$0, $20(a6)
pub fn i_02ce68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE6E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02CE6E: lsr.w #$2, d0
pub fn i_02ce6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE70;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CE70: andi.w #$ff, d0
pub fn i_02ce70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE74;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CE74: add.b d0, $20(a6)
pub fn i_02ce74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE78;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02CE78: lea.l $2ce7e(pc), a1
pub fn i_02ce78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CE7E;
    16
}
/// $02CE7C: move.l a1, (a6)
pub fn i_02ce7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CE7E: bset.b #$2, $8d(a6)
pub fn i_02ce7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE84;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CE84: subi.b #$1, $91(a6)
pub fn i_02ce84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE8A;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $02CE8A: jsr $2aa24(pc)
pub fn i_02ce8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE8E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CE8E);
    cpu.pc = 0x2AA24;
    16
}
/// $02CE8E: jsr $2a59a(pc)
pub fn i_02ce8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CE92);
    cpu.pc = 0x2A59A;
    16
}
/// $02CE92: bcc.w $2ce9c
pub fn i_02ce92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CE9C; return 10; }
    8
}
/// $02CE96: lea.l $2b6f4(pc), a1
pub fn i_02ce96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B6F4;
    16
}
/// $02CE9A: move.l a1, (a6)
pub fn i_02ce9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CE9C: jsr $5d5b6.l
pub fn i_02ce9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CEA2);
    cpu.pc = 0x5D5B6;
    16
}
/// $02CEA2: asl.w #$1, d0
pub fn i_02cea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEA4;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02CEA4: lea.l $2c900(pc), a0
pub fn i_02cea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEA8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C900;
    16
}
/// $02CEA8: move.w (a0, d0.w), $2c(a6)
pub fn i_02cea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEAE;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $02CEAE: tst.w $2c(a6)
pub fn i_02ceae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEB2;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02CEB2: bne.w $2cec2
pub fn i_02ceb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEB6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2CEC2; return 10; }
    8
}
/// $02CEB6: move.w $28(a6), d0
pub fn i_02ceb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEBA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02CEBA: neg.w d0
pub fn i_02ceba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEBC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02CEBC: asr.w #$3, d0
pub fn i_02cebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEBE;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02CEBE: move.w d0, $2c(a6)
pub fn i_02cebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEC2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CEC2: move.w $28(a6), d0
pub fn i_02cec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEC6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02CEC6: move.w #$2a0, d1
pub fn i_02cec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CECA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CECA: jsr $267f4.l
pub fn i_02ceca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CED0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CED0);
    cpu.pc = 0x267F4;
    16
}
/// $02CED0: move.w d0, $28(a6)
pub fn i_02ced0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CED4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CED4: jsr $2a7d8(pc)
pub fn i_02ced4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CED8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CED8);
    cpu.pc = 0x2A7D8;
    16
}
/// $02CED8: jsr $2a478(pc)
pub fn i_02ced8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CEDC);
    cpu.pc = 0x2A478;
    16
}
/// $02CEDC: jsr $28d70.l
pub fn i_02cedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CEE2);
    cpu.pc = 0x28D70;
    16
}
/// $02CEE2: jsr $2a8c0(pc)
pub fn i_02cee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CEE6);
    cpu.pc = 0x2A8C0;
    16
}
/// $02CEE6: tst.b $91(a6)
pub fn i_02cee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEEA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02CEEA: beq.w $2cefa
pub fn i_02ceea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CEFA; return 10; }
    8
}
/// $02CEEE: cmpi.b #$3, d1
pub fn i_02ceee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEF2;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $02CEF2: bge.w $2cefa
pub fn i_02cef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEF6;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2CEFA; return 10; }
    8
}
/// $02CEF6: bra.w $2cf08
pub fn i_02cef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x2CF08;
    10
}
/// $02CEFA: clr.w $28(a6)
pub fn i_02cefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEFE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02CEFE: clr.w $2c(a6)
pub fn i_02cefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF02;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02CF02: lea.l $2b4d2(pc), a1
pub fn i_02cf02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4D2;
    16
}
/// $02CF06: move.l a1, (a6)
pub fn i_02cf06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF08;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CF08: jsr $2a958(pc)
pub fn i_02cf08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CF0C);
    cpu.pc = 0x2A958;
    16
}
/// $02CF0C: cmp.w $80(a6), d2
pub fn i_02cf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF10;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02CF10: beq.w $2cf66
pub fn i_02cf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF14;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CF66; return 10; }
    8
}
/// $02CF14: move.w d2, $80(a6)
pub fn i_02cf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF18;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02CF18: move.w d2, d0
pub fn i_02cf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF1A;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CF1A: asr.w #$1, d0
pub fn i_02cf1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF1C;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02CF1C: neg.w d0
pub fn i_02cf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF1E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02CF1E: addi.w #$20, d0
pub fn i_02cf1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF22;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CF22: cmpi.w #$40, d0
pub fn i_02cf22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF26;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02CF26: bls.w $2cf2e
pub fn i_02cf26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF2A;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2CF2E; return 10; }
    8
}
/// $02CF2A: move.w #$40, d0
pub fn i_02cf2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF2E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02CF2E: movem.w d0, -(a7)
pub fn i_02cf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF32;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02CF32: jsr $295a6(pc)
pub fn i_02cf32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CF36);
    cpu.pc = 0x295A6;
    16
}
/// $02CF36: movea.l #$2b0c30, a0
pub fn i_02cf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF3C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02CF3C: lsl.w #$2, d0
pub fn i_02cf3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF3E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CF3E: movea.l (a0, d0.w), a0
pub fn i_02cf3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF42;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02CF42: cmpa.l #$ffffffff, a0
pub fn i_02cf42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF48;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02CF48: beq.w $2cf52
pub fn i_02cf48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CF52; return 10; }
    8
}
/// $02CF4C: jsr $28cd4.l
pub fn i_02cf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CF52);
    cpu.pc = 0x28CD4;
    16
}
/// $02CF52: movem.w (a7)+, d0
pub fn i_02cf52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF56;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02CF56: move.b #$0, $20(a6)
pub fn i_02cf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02CF5C: lsr.w #$2, d0
pub fn i_02cf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF5E;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02CF5E: andi.w #$ff, d0
pub fn i_02cf5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF62;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02CF62: add.b d0, $20(a6)
pub fn i_02cf62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF66;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02CF66: jsr $283d8.l
pub fn i_02cf66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CF6C);
    cpu.pc = 0x283D8;
    16
}
/// $02CF6C: jsr $2a8c0(pc)
pub fn i_02cf6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CF70);
    cpu.pc = 0x2A8C0;
    16
}
/// $02CF70: bcc.w $2cf7a
pub fn i_02cf70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF74;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CF7A; return 10; }
    8
}
/// $02CF74: lea.l $2d02e(pc), a1
pub fn i_02cf74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02CF78: move.l a1, (a6)
pub fn i_02cf78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CF7A: cmpi.b #$3, $106f2a.l
pub fn i_02cf7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF82;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $02CF82: bne.w $2cf9a
pub fn i_02cf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF86;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2CF9A; return 10; }
    8
}
/// $02CF86: jsr $2ab3c(pc)
pub fn i_02cf86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CF8A);
    cpu.pc = 0x2AB3C;
    16
}
/// $02CF8A: lea.l $2c9b0(pc), a0
pub fn i_02cf8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF8E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02CF8E: movea.l #$ffffffff, a1
pub fn i_02cf8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF94;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02CF94: jsr $772.l
pub fn i_02cf94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CF9A);
    cpu.pc = 0x772;
    16
}
/// $02CF9A: cmpi.b #$2, $106f2a.l
pub fn i_02cf9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFA2;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $02CFA2: bne.w $2cfba
pub fn i_02cfa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2CFBA; return 10; }
    8
}
/// $02CFA6: jsr $2ab3c(pc)
pub fn i_02cfa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFAA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CFAA);
    cpu.pc = 0x2AB3C;
    16
}
/// $02CFAA: lea.l $2c9b0(pc), a0
pub fn i_02cfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFAE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02CFAE: movea.l #$ffffffff, a1
pub fn i_02cfae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFB4;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02CFB4: jsr $772.l
pub fn i_02cfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CFBA);
    cpu.pc = 0x772;
    16
}
/// $02CFBA: jsr $2aac0(pc)
pub fn i_02cfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFBE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CFBE);
    cpu.pc = 0x2AAC0;
    16
}
/// $02CFBE: bcc.w $2cfc8
pub fn i_02cfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CFC8; return 10; }
    8
}
/// $02CFC2: lea.l $2c24a(pc), a1
pub fn i_02cfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02CFC6: move.l a1, (a6)
pub fn i_02cfc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CFC8: jsr $2ac80(pc)
pub fn i_02cfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFCC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CFCC);
    cpu.pc = 0x2AC80;
    16
}
/// $02CFCC: bcc.w $2cfd6
pub fn i_02cfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFD0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CFD6; return 10; }
    8
}
/// $02CFD0: lea.l $2dd74(pc), a1
pub fn i_02cfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02CFD4: move.l a1, (a6)
pub fn i_02cfd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFD6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CFD6: jsr $2a5cc(pc)
pub fn i_02cfd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFDA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CFDA);
    cpu.pc = 0x2A5CC;
    16
}
/// $02CFDA: bcc.w $2cfe4
pub fn i_02cfda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFDE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2CFE4; return 10; }
    8
}
/// $02CFDE: lea.l $2fc70(pc), a1
pub fn i_02cfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FC70;
    16
}
/// $02CFE2: move.l a1, (a6)
pub fn i_02cfe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02CFE4: jsr $2a690(pc)
pub fn i_02cfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2CFE8);
    cpu.pc = 0x2A690;
    16
}
/// $02CFE8: lea.l $2a060(pc), a0
pub fn i_02cfe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFEC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02CFEC: movea.l #$ffffffff, a1
pub fn i_02cfec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFF2;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02cff2  ($02CFF2, 8 B, thunks/jsr_abs_thunks.c)
/// $02CFF2: jsr $772.l
pub fn i_02cff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2CFF8);
    cpu.pc = 0x772;
    16
}
/// $02CFF8: rts
pub fn i_02cff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_FallStart_02cffa  ($02CFFA, 52 B, slug/states_02ae3e.s)
/// $02CFFA: bclr.b #$5, $8c(a6)
pub fn i_02cffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D000;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D000: move.w #$ffe0, $2e(a6)
pub fn i_02d000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D006;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D006: clr.w $2c(a6)
pub fn i_02d006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D00A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02D00A: addq.w #$4, $82(a6)
pub fn i_02d00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D00E;
    cpu.ir = 0x586E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $02D00E: move.w $24(a6), d0
pub fn i_02d00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D012;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02D012: cmp.w $82(a6), d0
pub fn i_02d012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D016;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02D016: ble.w $2d01e
pub fn i_02d016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D01A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2D01E; return 10; }
    8
}
/// $02D01A: move.w d0, $82(a6)
pub fn i_02d01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D01E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D01E: lea.l $2d024(pc), a1
pub fn i_02d01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D022;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D024;
    16
}
/// $02D022: move.l a1, (a6)
pub fn i_02d022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D024;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D024: bset.b #$2, $8d(a6)
pub fn i_02d024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D02A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D02A: jmp $2d0c4(pc)
pub fn i_02d02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D02E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2D0C4;
    8
}
// ===== Slug_Fall_02d02e  ($02D02E, 150 B, slug/states_02ae3e.s)
/// $02D02E: bclr.b #$5, $8c(a6)
pub fn i_02d02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D034;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D034: move.w #$ffe0, $2e(a6)
pub fn i_02d034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D03A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D03A: clr.w $2c(a6)
pub fn i_02d03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D03E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02D03E: addq.w #$4, $82(a6)
pub fn i_02d03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D042;
    cpu.ir = 0x586E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $02D042: move.w $24(a6), d0
pub fn i_02d042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D046;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02D046: cmp.w $82(a6), d0
pub fn i_02d046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D04A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02D04A: ble.w $2d052
pub fn i_02d04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D04E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2D052; return 10; }
    8
}
/// $02D04E: move.w d0, $82(a6)
pub fn i_02d04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D052;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D052: jsr $2a958(pc)
pub fn i_02d052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D056;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D056);
    cpu.pc = 0x2A958;
    16
}
/// $02D056: move.w d2, $80(a6)
pub fn i_02d056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D05A;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D05A: move.w d2, d0
pub fn i_02d05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D05C;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D05C: asr.w #$1, d0
pub fn i_02d05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D05E;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02D05E: neg.w d0
pub fn i_02d05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D060;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02D060: addi.w #$20, d0
pub fn i_02d060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D064;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D064: cmpi.w #$40, d0
pub fn i_02d064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D068;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02D068: bls.w $2d070
pub fn i_02d068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D06C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2D070; return 10; }
    8
}
/// $02D06C: move.w #$40, d0
pub fn i_02d06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D070;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D070: movem.w d0, -(a7)
pub fn i_02d070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D074;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02D074: jsr $295a6(pc)
pub fn i_02d074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D078;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D078);
    cpu.pc = 0x295A6;
    16
}
/// $02D078: movea.l #$2b0c30, a0
pub fn i_02d078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D07E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02D07E: lsl.w #$2, d0
pub fn i_02d07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D080;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D080: movea.l (a0, d0.w), a0
pub fn i_02d080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D084;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02D084: cmpa.l #$ffffffff, a0
pub fn i_02d084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D08A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02D08A: beq.w $2d094
pub fn i_02d08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D08E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D094; return 10; }
    8
}
/// $02D08E: jsr $28cd4.l
pub fn i_02d08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D094;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D094);
    cpu.pc = 0x28CD4;
    16
}
/// $02D094: movem.w (a7)+, d0
pub fn i_02d094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D098;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02D098: move.b #$0, $20(a6)
pub fn i_02d098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D09E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D09E: lsr.w #$2, d0
pub fn i_02d09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0A0;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D0A0: andi.w #$ff, d0
pub fn i_02d0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0A4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D0A4: add.b d0, $20(a6)
pub fn i_02d0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0A8;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02D0A8: bset.b #$2, $8d(a6)
pub fn i_02d0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0AE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D0AE: lea.l $2d0c4(pc), a0
pub fn i_02d0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0B2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2D0C4;
    16
}
/// $02D0B2: cmpa.l (a6), a0
pub fn i_02d0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0B4;
    cpu.ir = 0xB1D6;
    let v2 = bus.read32(cpu.a[6]);
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    14
}
/// $02D0B4: bne.w $2d0be
pub fn i_02d0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D0BE; return 10; }
    8
}
/// $02D0B8: lea.l $2d26e(pc), a1
pub fn i_02d0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D26E;
    16
}
/// $02D0BC: move.l a1, (a6)
pub fn i_02d0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D0BE: lea.l $2d0c4(pc), a1
pub fn i_02d0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D0C4;
    16
}
/// $02D0C2: move.l a1, (a6)
pub fn i_02d0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Slug_Fall_Loop_02d0c4  ($02D0C4, 426 B, slug/states_02ae3e.s)
/// $02D0C4: jsr $5d5b6.l
pub fn i_02d0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D0CA);
    cpu.pc = 0x5D5B6;
    16
}
/// $02D0CA: asl.w #$1, d0
pub fn i_02d0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0CC;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02D0CC: lea.l $2c900(pc), a0
pub fn i_02d0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0D0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C900;
    16
}
/// $02D0D0: move.w (a0, d0.w), $2c(a6)
pub fn i_02d0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0D6;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $02D0D6: move.w $28(a6), d0
pub fn i_02d0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0DA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02D0DA: move.w #$2a0, d1
pub fn i_02d0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0DE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D0DE: jsr $267f4.l
pub fn i_02d0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D0E4);
    cpu.pc = 0x267F4;
    16
}
/// $02D0E4: move.w d0, $28(a6)
pub fn i_02d0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0E8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D0E8: move.w $2a(a6), d0
pub fn i_02d0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0EC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02D0EC: move.w #$400, d1
pub fn i_02d0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D0F0: jsr $267f4.l
pub fn i_02d0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D0F6);
    cpu.pc = 0x267F4;
    16
}
/// $02D0F6: move.w d0, $2a(a6)
pub fn i_02d0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0FA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D0FA: bset.b #$1, $8d(a6)
pub fn i_02d0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D100;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D100: jsr $2aa0e(pc)
pub fn i_02d100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D104;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D104);
    cpu.pc = 0x2AA0E;
    16
}
/// $02D104: jsr $2aac0(pc)
pub fn i_02d104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D108;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D108);
    cpu.pc = 0x2AAC0;
    16
}
/// $02D108: bcc.w $2d112
pub fn i_02d108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D10C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D112; return 10; }
    8
}
/// $02D10C: lea.l $2d394(pc), a1
pub fn i_02d10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D110;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D394;
    16
}
/// $02D110: move.l a1, (a6)
pub fn i_02d110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D112;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D112: jsr $2a878(pc)
pub fn i_02d112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D116;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D116);
    cpu.pc = 0x2A878;
    16
}
/// $02D116: bcc.w $2d182
pub fn i_02d116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D11A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D182; return 10; }
    8
}
/// $02D11A: lea.l $2cc1a(pc), a1
pub fn i_02d11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D11E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CC1A;
    16
}
/// $02D11E: move.l a1, (a6)
pub fn i_02d11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D120;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D120: jsr $5cef8.l
pub fn i_02d120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D126;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D126);
    cpu.pc = 0x5CEF8;
    16
}
/// $02D126: bcc.w $2d142
pub fn i_02d126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D12A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D142; return 10; }
    8
}
/// $02D12A: jsr $2a9a0(pc)
pub fn i_02d12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D12E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D12E);
    cpu.pc = 0x2A9A0;
    16
}
/// $02D12E: andi.w #$ff, d0
pub fn i_02d12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D132;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D132: lea.l $2a08c(pc), a0
pub fn i_02d132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D136;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A08C;
    16
}
/// $02D136: movea.l #$ffffffff, a1
pub fn i_02d136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D13C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02D13C: jsr $772.l
pub fn i_02d13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D142;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D142);
    cpu.pc = 0x772;
    16
}
/// $02D142: cmpi.b #$3, $106f2a.l
pub fn i_02d142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D14A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $02D14A: bne.w $2d162
pub fn i_02d14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D14E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D162; return 10; }
    8
}
/// $02D14E: jsr $2ab3c(pc)
pub fn i_02d14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D152;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D152);
    cpu.pc = 0x2AB3C;
    16
}
/// $02D152: lea.l $2c9b0(pc), a0
pub fn i_02d152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D156;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02D156: movea.l #$ffffffff, a1
pub fn i_02d156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D15C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02D15C: jsr $772.l
pub fn i_02d15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D162;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D162);
    cpu.pc = 0x772;
    16
}
/// $02D162: cmpi.b #$2, $106f2a.l
pub fn i_02d162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D16A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $02D16A: bne.w $2d182
pub fn i_02d16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D16E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D182; return 10; }
    8
}
/// $02D16E: jsr $2ab3c(pc)
pub fn i_02d16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D172;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D172);
    cpu.pc = 0x2AB3C;
    16
}
/// $02D172: lea.l $2c9b0(pc), a0
pub fn i_02d172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D176;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02D176: movea.l #$ffffffff, a1
pub fn i_02d176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D17C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02D17C: jsr $772.l
pub fn i_02d17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D182;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D182);
    cpu.pc = 0x772;
    16
}
/// $02D182: jsr $2a478(pc)
pub fn i_02d182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D186;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D186);
    cpu.pc = 0x2A478;
    16
}
/// $02D186: jsr $28d70.l
pub fn i_02d186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D18C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D18C);
    cpu.pc = 0x28D70;
    16
}
/// $02D18C: tst.w $2a(a6)
pub fn i_02d18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D190;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02D190: bgt.w $2d21c
pub fn i_02d190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D194;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2D21C; return 10; }
    8
}
/// $02D194: jsr $2a8c0(pc)
pub fn i_02d194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D198;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D198);
    cpu.pc = 0x2A8C0;
    16
}
/// $02D198: cmpi.b #$0, d1
pub fn i_02d198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D19C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $02D19C: ble.w $2d1a8
pub fn i_02d19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1A0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2D1A8; return 10; }
    8
}
/// $02D1A0: clr.w $28(a6)
pub fn i_02d1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1A4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02D1A4: clr.w $2c(a6)
pub fn i_02d1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1A8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02D1A8: jsr $2a958(pc)
pub fn i_02d1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D1AC);
    cpu.pc = 0x2A958;
    16
}
/// $02D1AC: cmp.w $80(a6), d2
pub fn i_02d1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1B0;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02D1B0: beq.w $2d21c
pub fn i_02d1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D21C; return 10; }
    8
}
/// $02D1B4: move.w d2, $80(a6)
pub fn i_02d1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1B8;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D1B8: move.w d2, d0
pub fn i_02d1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1BA;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D1BA: asr.w #$1, d0
pub fn i_02d1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1BC;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02D1BC: neg.w d0
pub fn i_02d1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1BE;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02D1BE: addi.w #$20, d0
pub fn i_02d1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1C2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D1C2: cmpi.w #$40, d0
pub fn i_02d1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1C6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02D1C6: bls.w $2d1ce
pub fn i_02d1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1CA;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2D1CE; return 10; }
    8
}
/// $02D1CA: move.w #$40, d0
pub fn i_02d1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1CE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D1CE: movem.w d0, -(a7)
pub fn i_02d1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1D2;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02D1D2: jsr $295a6(pc)
pub fn i_02d1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D1D6);
    cpu.pc = 0x295A6;
    16
}
/// $02D1D6: movea.l #$2b0c30, a0
pub fn i_02d1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1DC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02D1DC: lsl.w #$2, d0
pub fn i_02d1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1DE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D1DE: movea.l (a0, d0.w), a0
pub fn i_02d1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1E2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02D1E2: cmpa.l #$ffffffff, a0
pub fn i_02d1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1E8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02D1E8: beq.w $2d1f2
pub fn i_02d1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D1F2; return 10; }
    8
}
/// $02D1EC: jsr $28cd4.l
pub fn i_02d1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D1F2);
    cpu.pc = 0x28CD4;
    16
}
/// $02D1F2: movem.w (a7)+, d0
pub fn i_02d1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1F6;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02D1F6: move.b #$0, $20(a6)
pub fn i_02d1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D1FC: lsr.w #$2, d0
pub fn i_02d1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1FE;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D1FE: andi.w #$ff, d0
pub fn i_02d1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D202;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D202: add.b d0, $20(a6)
pub fn i_02d202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D206;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02D206: bset.b #$2, $8d(a6)
pub fn i_02d206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D20C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D20C: lea.l $2d0c4(pc), a0
pub fn i_02d20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D210;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2D0C4;
    16
}
/// $02D210: cmpa.l (a6), a0
pub fn i_02d210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D212;
    cpu.ir = 0xB1D6;
    let v2 = bus.read32(cpu.a[6]);
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    14
}
/// $02D212: bne.w $2d21c
pub fn i_02d212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D216;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D21C; return 10; }
    8
}
/// $02D216: lea.l $2d26e(pc), a1
pub fn i_02d216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D21A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D26E;
    16
}
/// $02D21A: move.l a1, (a6)
pub fn i_02d21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D21C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D21C: btst.b #$1, $8d(a6)
pub fn i_02d21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D222;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02D222: beq.w $2d23e
pub fn i_02d222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D226;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D23E; return 10; }
    8
}
/// $02D226: jsr $2a690(pc)
pub fn i_02d226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D22A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D22A);
    cpu.pc = 0x2A690;
    16
}
/// $02D22A: lea.l $2a06c(pc), a0
pub fn i_02d22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D22E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A06C;
    16
}
/// $02D22E: movea.l #$ffffffff, a1
pub fn i_02d22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D234;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02D234: jsr $772.l
pub fn i_02d234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D23A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D23A);
    cpu.pc = 0x772;
    16
}
/// $02D23A: bra.w $2d252
pub fn i_02d23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D23E;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D252;
    10
}
/// $02D23E: jsr $2a690(pc)
pub fn i_02d23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D242;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D242);
    cpu.pc = 0x2A690;
    16
}
/// $02D242: lea.l $2a060(pc), a0
pub fn i_02d242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D246;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02D246: movea.l #$ffffffff, a1
pub fn i_02d246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D24C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02D24C: jsr $772.l
pub fn i_02d24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D252;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D252);
    cpu.pc = 0x772;
    16
}
/// $02D252: movea.l #$ffffffff, a0
pub fn i_02d252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D258;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02D258: lea.l $27964e.l, a0
pub fn i_02d258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D25E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02D25E: jsr $5dd56.l
pub fn i_02d25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D264;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D264);
    cpu.pc = 0x5DD56;
    16
}
/// $02D264: bcc.w $2d26c
pub fn i_02d264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D268;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D26C; return 10; }
    8
}
/// $02D268: jmp $2dcbc(pc)
pub fn i_02d268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D26C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02D26C: rts
pub fn i_02d26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D26E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_FallFire_02d26e  ($02D26E, 24 B, slug/states_02ae3e.s)
/// $02D26E: bset.b #$2, $8d(a6)
pub fn i_02d26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D274;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D274: jsr $2aac0(pc)
pub fn i_02d274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D278;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D278);
    cpu.pc = 0x2AAC0;
    16
}
/// $02D278: bcc.w $2d282
pub fn i_02d278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D27C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D282; return 10; }
    8
}
/// $02D27C: lea.l $2d380(pc), a1
pub fn i_02d27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D280;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D380;
    16
}
/// $02D280: move.l a1, (a6)
pub fn i_02d280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D282;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D282: bra.w $2d0c4
pub fn i_02d282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D286;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D0C4;
    10
}
// ===== Slug_FireJumpCrouch_02d286  ($02D286, 250 B, slug/states_02ae3e.s)
/// $02D286: bclr.b #$3, $5b(a6)
pub fn i_02d286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D28C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D28C: move.w #$4a4, $2a(a6)
pub fn i_02d28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D292;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4A4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D292: move.w #$ff9d, $2e(a6)
pub fn i_02d292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D298;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF9D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D298: bclr.b #$5, $8c(a6)
pub fn i_02d298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D29E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D29E: jsr $2a9a0(pc)
pub fn i_02d29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D2A2);
    cpu.pc = 0x2A9A0;
    16
}
/// $02D2A2: movea.l #$27931c, a0
pub fn i_02d2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2A8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x27931C;
    cpu.a[0] = v1;
    4
}
/// $02D2A8: lsl.w #$2, d0
pub fn i_02d2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2AA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D2AA: movea.l (a0, d0.w), a0
pub fn i_02d2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2AE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02D2AE: cmpa.l #$ffffffff, a0
pub fn i_02d2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2B4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02D2B4: beq.w $2d2be
pub fn i_02d2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D2BE; return 10; }
    8
}
/// $02D2B8: jsr $28cd4.l
pub fn i_02d2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D2BE);
    cpu.pc = 0x28CD4;
    16
}
/// $02D2BE: move.b #$2d, $20(a6)
pub fn i_02d2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D2C4: andi.w #$ff, d0
pub fn i_02d2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2C8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D2C8: lsr.w #$2, d0
pub fn i_02d2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2CA;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D2CA: add.b d0, $20(a6)
pub fn i_02d2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2CE;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02D2CE: bclr.b #$5, $8c(a6)
pub fn i_02d2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2D4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D2D4: move.b #$4, d1
pub fn i_02d2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2D8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02D2D8: jsr $8f714.l
pub fn i_02d2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D2DE);
    cpu.pc = 0x8F714;
    16
}
/// $02D2DE: bcs.w $2d330
pub fn i_02d2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2E2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2D330; return 10; }
    8
}
/// $02D2E2: cmpi.b #$ff, $84(a6)
pub fn i_02d2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2E8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $02D2E8: beq.w $2d302
pub fn i_02d2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D302; return 10; }
    8
}
/// $02D2EC: lea.l $312b2.l, a1
pub fn i_02d2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2F2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x312B2;
    20
}
/// $02D2F2: jsr $6fe.l
pub fn i_02d2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D2F8);
    cpu.pc = 0x6FE;
    16
}
/// $02D2F8: jsr $5dd02.l
pub fn i_02d2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D2FE);
    cpu.pc = 0x5DD02;
    16
}
/// $02D2FE: bra.w $2d314
pub fn i_02d2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D302;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D314;
    10
}
/// $02D302: lea.l $31240.l, a1
pub fn i_02d302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D308;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31240;
    20
}
/// $02D308: jsr $6fe.l
pub fn i_02d308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D30E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D30E);
    cpu.pc = 0x6FE;
    16
}
/// $02D30E: jsr $5dd02.l
pub fn i_02d30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D314;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D314);
    cpu.pc = 0x5DD02;
    16
}
/// $02D314: jsr $517fe.l
pub fn i_02d314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D31A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D31A);
    cpu.pc = 0x517FE;
    16
}
/// $02D31A: lea.l $31af6.l, a1
pub fn i_02d31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D320;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31AF6;
    20
}
/// $02D320: jsr $6fe.l
pub fn i_02d320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D326;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D326);
    cpu.pc = 0x6FE;
    16
}
/// $02D326: jsr $5dd02.l
pub fn i_02d326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D32C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D32C);
    cpu.pc = 0x5DD02;
    16
}
/// $02D32C: bra.w $2d348
pub fn i_02d32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D330;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D348;
    10
}
/// $02D330: lea.l $31922.l, a1
pub fn i_02d330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D336;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31922;
    20
}
/// $02D336: jsr $6fe.l
pub fn i_02d336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D33C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D33C);
    cpu.pc = 0x6FE;
    16
}
/// $02D33C: jsr $5dd02.l
pub fn i_02d33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D342;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D342);
    cpu.pc = 0x5DD02;
    16
}
/// $02D342: jsr $517fe.l
pub fn i_02d342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D348;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D348);
    cpu.pc = 0x517FE;
    16
}
/// $02D348: cmpi.b #$0, $90(a6)
pub fn i_02d348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D34E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $02D34E: beq.w $2d356
pub fn i_02d34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D352;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D356; return 10; }
    8
}
/// $02D352: jsr $2aab0(pc)
pub fn i_02d352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D356;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D356);
    cpu.pc = 0x2AAB0;
    16
}
/// $02D356: move.b #$1e, $8e(a6)
pub fn i_02d356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D35C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D35C: bset.b #$0, $8d(a6)
pub fn i_02d35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D362;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D362: btst.b #$2, $8d(a6)
pub fn i_02d362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D368;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02D368: bne.w $2d376
pub fn i_02d368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D36C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D376; return 10; }
    8
}
/// $02D36C: lea.l $2d49e(pc), a1
pub fn i_02d36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D370;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D49E;
    16
}
/// $02D370: move.l a1, (a6)
pub fn i_02d370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D372;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D372: bra.w $2d37c
pub fn i_02d372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D376;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D37C;
    10
}
/// $02D376: lea.l $2d38a(pc), a1
pub fn i_02d376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D37A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D38A;
    16
}
/// $02D37A: move.l a1, (a6)
pub fn i_02d37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D37C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D37C: bra.w $2d49e
pub fn i_02d37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D380;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D49E;
    10
}
// ===== Slug_FireAirB_02d380  ($02D380, 10 B, slug/states_02ae3e.s)
/// $02D380: bset.b #$2, $8d(a6)
pub fn i_02d380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D386;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D386: bra.w $2d394
pub fn i_02d386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D38A;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D394;
    10
}
// ===== Slug_FireAirResume_02d38a  ($02D38A, 10 B, slug/states_02ae3e.s)
/// $02D38A: bset.b #$2, $8d(a6)
pub fn i_02d38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D390;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D390: bra.w $2d49e
pub fn i_02d390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D394;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D49E;
    10
}
// ===== Slug_FireAir_02d394  ($02D394, 266 B, slug/states_02ae3e.s)
/// $02D394: bclr.b #$5, $8c(a6)
pub fn i_02d394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D39A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D39A: move.b #$4, d1
pub fn i_02d39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D39E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02D39E: jsr $8f714.l
pub fn i_02d39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D3A4);
    cpu.pc = 0x8F714;
    16
}
/// $02D3A4: bcs.w $2d3f6
pub fn i_02d3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3A8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2D3F6; return 10; }
    8
}
/// $02D3A8: cmpi.b #$ff, $84(a6)
pub fn i_02d3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3AE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $02D3AE: beq.w $2d3c8
pub fn i_02d3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D3C8; return 10; }
    8
}
/// $02D3B2: lea.l $312b2.l, a1
pub fn i_02d3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3B8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x312B2;
    20
}
/// $02D3B8: jsr $6fe.l
pub fn i_02d3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D3BE);
    cpu.pc = 0x6FE;
    16
}
/// $02D3BE: jsr $5dd02.l
pub fn i_02d3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D3C4);
    cpu.pc = 0x5DD02;
    16
}
/// $02D3C4: bra.w $2d3da
pub fn i_02d3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D3DA;
    10
}
/// $02D3C8: lea.l $31240.l, a1
pub fn i_02d3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3CE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31240;
    20
}
/// $02D3CE: jsr $6fe.l
pub fn i_02d3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D3D4);
    cpu.pc = 0x6FE;
    16
}
/// $02D3D4: jsr $5dd02.l
pub fn i_02d3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D3DA);
    cpu.pc = 0x5DD02;
    16
}
/// $02D3DA: jsr $517fe.l
pub fn i_02d3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D3E0);
    cpu.pc = 0x517FE;
    16
}
/// $02D3E0: lea.l $31af6.l, a1
pub fn i_02d3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31AF6;
    20
}
/// $02D3E6: jsr $6fe.l
pub fn i_02d3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D3EC);
    cpu.pc = 0x6FE;
    16
}
/// $02D3EC: jsr $5dd02.l
pub fn i_02d3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D3F2);
    cpu.pc = 0x5DD02;
    16
}
/// $02D3F2: bra.w $2d40e
pub fn i_02d3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D40E;
    10
}
/// $02D3F6: lea.l $31922.l, a1
pub fn i_02d3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3FC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31922;
    20
}
/// $02D3FC: jsr $6fe.l
pub fn i_02d3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D402;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D402);
    cpu.pc = 0x6FE;
    16
}
/// $02D402: jsr $5dd02.l
pub fn i_02d402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D408);
    cpu.pc = 0x5DD02;
    16
}
/// $02D408: jsr $517fe.l
pub fn i_02d408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D40E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D40E);
    cpu.pc = 0x517FE;
    16
}
/// $02D40E: cmpi.b #$0, $90(a6)
pub fn i_02d40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D414;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $02D414: beq.w $2d41c
pub fn i_02d414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D418;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D41C; return 10; }
    8
}
/// $02D418: jsr $2aab0(pc)
pub fn i_02d418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D41C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D41C);
    cpu.pc = 0x2AAB0;
    16
}
/// $02D41C: move.b #$1e, $8e(a6)
pub fn i_02d41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D422;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D422: jsr $2a958(pc)
pub fn i_02d422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D426;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D426);
    cpu.pc = 0x2A958;
    16
}
/// $02D426: move.w d2, $80(a6)
pub fn i_02d426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D42A;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D42A: move.w d2, d0
pub fn i_02d42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D42C;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D42C: asr.w #$1, d0
pub fn i_02d42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D42E;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02D42E: neg.w d0
pub fn i_02d42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D430;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02D430: addi.w #$20, d0
pub fn i_02d430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D434;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D434: cmpi.w #$40, d0
pub fn i_02d434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D438;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02D438: bls.w $2d440
pub fn i_02d438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D43C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2D440; return 10; }
    8
}
/// $02D43C: move.w #$40, d0
pub fn i_02d43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D440;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D440: movem.w d0, -(a7)
pub fn i_02d440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D444;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02D444: jsr $295a6(pc)
pub fn i_02d444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D448;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D448);
    cpu.pc = 0x295A6;
    16
}
/// $02D448: movea.l #$2b0c30, a0
pub fn i_02d448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D44E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02D44E: lsl.w #$2, d0
pub fn i_02d44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D450;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D450: movea.l (a0, d0.w), a0
pub fn i_02d450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D454;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02D454: cmpa.l #$ffffffff, a0
pub fn i_02d454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D45A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02D45A: beq.w $2d464
pub fn i_02d45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D45E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D464; return 10; }
    8
}
/// $02D45E: jsr $28cd4.l
pub fn i_02d45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D464;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D464);
    cpu.pc = 0x28CD4;
    16
}
/// $02D464: movem.w (a7)+, d0
pub fn i_02d464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D468;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02D468: move.b #$0, $20(a6)
pub fn i_02d468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D46E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D46E: lsr.w #$2, d0
pub fn i_02d46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D470;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D470: andi.w #$ff, d0
pub fn i_02d470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D474;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D474: add.b d0, $20(a6)
pub fn i_02d474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D478;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02D478: bset.b #$2, $8d(a6)
pub fn i_02d478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D47E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D47E: bset.b #$0, $8d(a6)
pub fn i_02d47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D484;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D484: btst.b #$2, $8d(a6)
pub fn i_02d484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D48A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02D48A: bne.w $2d498
pub fn i_02d48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D48E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D498; return 10; }
    8
}
/// $02D48E: lea.l $2d49e(pc), a1
pub fn i_02d48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D492;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D49E;
    16
}
/// $02D492: move.l a1, (a6)
pub fn i_02d492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D494;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D494: bra.w $2d49e
pub fn i_02d494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D498;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D49E;
    10
}
/// $02D498: lea.l $2d38a(pc), a1
pub fn i_02d498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D49C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D38A;
    16
}
/// $02D49C: move.l a1, (a6)
pub fn i_02d49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D49E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Slug_FireAir_Loop_02d49e  ($02D49E, 416 B, slug/states_02ae3e.s)
/// $02D49E: jsr $5d5b6.l
pub fn i_02d49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D4A4);
    cpu.pc = 0x5D5B6;
    16
}
/// $02D4A4: asl.w #$1, d0
pub fn i_02d4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4A6;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02D4A6: lea.l $2c900(pc), a0
pub fn i_02d4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4AA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C900;
    16
}
/// $02D4AA: move.w (a0, d0.w), $2c(a6)
pub fn i_02d4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4B0;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $02D4B0: move.w $28(a6), d0
pub fn i_02d4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4B4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02D4B4: move.w #$2a0, d1
pub fn i_02d4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4B8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D4B8: jsr $267f4.l
pub fn i_02d4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D4BE);
    cpu.pc = 0x267F4;
    16
}
/// $02D4BE: move.w d0, $28(a6)
pub fn i_02d4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4C2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D4C2: move.w $2a(a6), d0
pub fn i_02d4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4C6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02D4C6: bgt.w $2d4d8
pub fn i_02d4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4CA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2D4D8; return 10; }
    8
}
/// $02D4CA: move.w #$400, d1
pub fn i_02d4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4CE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D4CE: jsr $267f4.l
pub fn i_02d4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D4D4);
    cpu.pc = 0x267F4;
    16
}
/// $02D4D4: move.w d0, $2a(a6)
pub fn i_02d4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4D8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D4D8: bset.b #$1, $8d(a6)
pub fn i_02d4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4DE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D4DE: jsr $2aa0e(pc)
pub fn i_02d4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D4E2);
    cpu.pc = 0x2AA0E;
    16
}
/// $02D4E2: jsr $2a878(pc)
pub fn i_02d4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D4E6);
    cpu.pc = 0x2A878;
    16
}
/// $02D4E6: bcc.w $2d552
pub fn i_02d4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D552; return 10; }
    8
}
/// $02D4EA: lea.l $2cc1a(pc), a1
pub fn i_02d4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2CC1A;
    16
}
/// $02D4EE: move.l a1, (a6)
pub fn i_02d4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D4F0: jsr $5cef8.l
pub fn i_02d4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D4F6);
    cpu.pc = 0x5CEF8;
    16
}
/// $02D4F6: bcc.w $2d512
pub fn i_02d4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D512; return 10; }
    8
}
/// $02D4FA: jsr $2a9a0(pc)
pub fn i_02d4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D4FE);
    cpu.pc = 0x2A9A0;
    16
}
/// $02D4FE: andi.w #$ff, d0
pub fn i_02d4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D502;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D502: lea.l $2a08c(pc), a0
pub fn i_02d502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D506;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A08C;
    16
}
/// $02D506: movea.l #$ffffffff, a1
pub fn i_02d506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D50C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02D50C: jsr $772.l
pub fn i_02d50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D512);
    cpu.pc = 0x772;
    16
}
/// $02D512: cmpi.b #$3, $106f2a.l
pub fn i_02d512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D51A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $02D51A: bne.w $2d532
pub fn i_02d51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D51E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D532; return 10; }
    8
}
/// $02D51E: jsr $2ab3c(pc)
pub fn i_02d51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D522;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D522);
    cpu.pc = 0x2AB3C;
    16
}
/// $02D522: lea.l $2c9b0(pc), a0
pub fn i_02d522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D526;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02D526: movea.l #$ffffffff, a1
pub fn i_02d526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D52C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02D52C: jsr $772.l
pub fn i_02d52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D532);
    cpu.pc = 0x772;
    16
}
/// $02D532: cmpi.b #$2, $106f2a.l
pub fn i_02d532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D53A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $02D53A: bne.w $2d552
pub fn i_02d53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D53E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D552; return 10; }
    8
}
/// $02D53E: jsr $2ab3c(pc)
pub fn i_02d53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D542;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D542);
    cpu.pc = 0x2AB3C;
    16
}
/// $02D542: lea.l $2c9b0(pc), a0
pub fn i_02d542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D546;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2C9B0;
    16
}
/// $02D546: movea.l #$ffffffff, a1
pub fn i_02d546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D54C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02D54C: jsr $772.l
pub fn i_02d54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D552);
    cpu.pc = 0x772;
    16
}
/// $02D552: jsr $2a478(pc)
pub fn i_02d552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D556;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D556);
    cpu.pc = 0x2A478;
    16
}
/// $02D556: jsr $28d70.l
pub fn i_02d556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D55C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D55C);
    cpu.pc = 0x28D70;
    16
}
/// $02D55C: tst.w $2a(a6)
pub fn i_02d55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D560;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $02D560: bgt.w $2d5ec
pub fn i_02d560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D564;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2D5EC; return 10; }
    8
}
/// $02D564: jsr $2a8c0(pc)
pub fn i_02d564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D568;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D568);
    cpu.pc = 0x2A8C0;
    16
}
/// $02D568: cmpi.b #$0, d1
pub fn i_02d568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D56C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $02D56C: ble.w $2d5ec
pub fn i_02d56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D570;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2D5EC; return 10; }
    8
}
/// $02D570: clr.w $28(a6)
pub fn i_02d570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D574;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02D574: clr.w $2c(a6)
pub fn i_02d574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D578;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02D578: jsr $2a958(pc)
pub fn i_02d578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D57C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D57C);
    cpu.pc = 0x2A958;
    16
}
/// $02D57C: cmp.w $80(a6), d2
pub fn i_02d57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D580;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02D580: beq.w $2d5ec
pub fn i_02d580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D584;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D5EC; return 10; }
    8
}
/// $02D584: move.w d2, $80(a6)
pub fn i_02d584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D588;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D588: move.w d2, d0
pub fn i_02d588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D58A;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D58A: asr.w #$1, d0
pub fn i_02d58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D58C;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02D58C: neg.w d0
pub fn i_02d58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D58E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02D58E: addi.w #$20, d0
pub fn i_02d58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D592;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D592: cmpi.w #$40, d0
pub fn i_02d592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D596;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02D596: bls.w $2d59e
pub fn i_02d596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D59A;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2D59E; return 10; }
    8
}
/// $02D59A: move.w #$40, d0
pub fn i_02d59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D59E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D59E: movem.w d0, -(a7)
pub fn i_02d59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5A2;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $02D5A2: jsr $295a6(pc)
pub fn i_02d5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D5A6);
    cpu.pc = 0x295A6;
    16
}
/// $02D5A6: movea.l #$2b0c30, a0
pub fn i_02d5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5AC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2B0C30;
    cpu.a[0] = v1;
    4
}
/// $02D5AC: lsl.w #$2, d0
pub fn i_02d5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5AE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D5AE: movea.l (a0, d0.w), a0
pub fn i_02d5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5B2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02D5B2: cmpa.l #$ffffffff, a0
pub fn i_02d5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5B8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02D5B8: beq.w $2d5c2
pub fn i_02d5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D5C2; return 10; }
    8
}
/// $02D5BC: jsr $28cd4.l
pub fn i_02d5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D5C2);
    cpu.pc = 0x28CD4;
    16
}
/// $02D5C2: movem.w (a7)+, d0
pub fn i_02d5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5C6;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $02D5C6: move.b #$0, $20(a6)
pub fn i_02d5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D5CC: lsr.w #$2, d0
pub fn i_02d5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5CE;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D5CE: andi.w #$ff, d0
pub fn i_02d5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5D2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D5D2: add.b d0, $20(a6)
pub fn i_02d5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5D6;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02D5D6: bset.b #$2, $8d(a6)
pub fn i_02d5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5DC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D5DC: lea.l $2d49e(pc), a0
pub fn i_02d5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5E0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2D49E;
    16
}
/// $02D5E0: cmpa.l (a6), a0
pub fn i_02d5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5E2;
    cpu.ir = 0xB1D6;
    let v2 = bus.read32(cpu.a[6]);
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    14
}
/// $02D5E2: bne.w $2d5ec
pub fn i_02d5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D5EC; return 10; }
    8
}
/// $02D5E6: lea.l $2d38a(pc), a1
pub fn i_02d5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D38A;
    16
}
/// $02D5EA: move.l a1, (a6)
pub fn i_02d5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D5EC: btst.b #$1, $8d(a6)
pub fn i_02d5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5F2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02D5F2: beq.w $2d60e
pub fn i_02d5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D60E; return 10; }
    8
}
/// $02D5F6: jsr $2a690(pc)
pub fn i_02d5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D5FA);
    cpu.pc = 0x2A690;
    16
}
/// $02D5FA: lea.l $2a06c(pc), a0
pub fn i_02d5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5FE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A06C;
    16
}
/// $02D5FE: movea.l #$ffffffff, a1
pub fn i_02d5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D604;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02D604: jsr $772.l
pub fn i_02d604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D60A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D60A);
    cpu.pc = 0x772;
    16
}
/// $02D60A: bra.w $2d622
pub fn i_02d60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D60E;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D622;
    10
}
/// $02D60E: jsr $2a690(pc)
pub fn i_02d60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D612;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D612);
    cpu.pc = 0x2A690;
    16
}
/// $02D612: lea.l $2a060(pc), a0
pub fn i_02d612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D616;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02D616: movea.l #$ffffffff, a1
pub fn i_02d616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D61C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02D61C: jsr $772.l
pub fn i_02d61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D622;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D622);
    cpu.pc = 0x772;
    16
}
/// $02D622: movea.l #$ffffffff, a0
pub fn i_02d622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D628;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02D628: lea.l $27964e.l, a0
pub fn i_02d628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D62E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02D62E: jsr $5dd56.l
pub fn i_02d62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D634;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D634);
    cpu.pc = 0x5DD56;
    16
}
/// $02D634: bcc.w $2d63c
pub fn i_02d634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D638;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D63C; return 10; }
    8
}
/// $02D638: jmp $2dcbc(pc)
pub fn i_02d638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D63C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02D63C: rts
pub fn i_02d63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D63E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_HitReact_02d63e  ($02D63E, 240 B, slug/states_02ae3e.s)
/// $02D63E: bclr.b #$5, $8c(a6)
pub fn i_02d63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D644;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D644: jsr $2a9a0(pc)
pub fn i_02d644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D648;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D648);
    cpu.pc = 0x2A9A0;
    16
}
/// $02D648: movea.l #$27954c, a0
pub fn i_02d648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D64E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x27954C;
    cpu.a[0] = v1;
    4
}
/// $02D64E: lsl.w #$2, d0
pub fn i_02d64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D650;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D650: movea.l (a0, d0.w), a0
pub fn i_02d650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D654;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02D654: cmpa.l #$ffffffff, a0
pub fn i_02d654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D65A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02D65A: beq.w $2d664
pub fn i_02d65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D65E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D664; return 10; }
    8
}
/// $02D65E: jsr $28cd4.l
pub fn i_02d65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D664;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D664);
    cpu.pc = 0x28CD4;
    16
}
/// $02D664: move.w #$0, d0
pub fn i_02d664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D668;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D668: move.b #$96, $20(a6)
pub fn i_02d668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D66E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D66E: andi.w #$ff, d0
pub fn i_02d66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D672;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D672: lsr.w #$2, d0
pub fn i_02d672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D674;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D674: add.b d0, $20(a6)
pub fn i_02d674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D678;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02D678: bra.w $2d6ba
pub fn i_02d678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D67C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D6BA;
    10
}
/// $02D6BA: jsr $267e2.l
pub fn i_02d6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D6C0);
    cpu.pc = 0x267E2;
    16
}
/// $02D6C0: move.l #$295b4, $60(a6)
pub fn i_02d6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6C8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02D6C8: lea.l $2d6ce(pc), a1
pub fn i_02d6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D6CE;
    16
}
/// $02D6CC: move.l a1, (a6)
pub fn i_02d6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D6CE: move.b #$28, $45(a6)
pub fn i_02d6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6D4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D6D4: bclr.b #$3, $13(a6)
pub fn i_02d6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6DA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D6DA: jsr $2aa24(pc)
pub fn i_02d6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D6DE);
    cpu.pc = 0x2AA24;
    16
}
/// $02D6DE: jsr $2a7d8(pc)
pub fn i_02d6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D6E2);
    cpu.pc = 0x2A7D8;
    16
}
/// $02D6E2: jsr $28d70.l
pub fn i_02d6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D6E8);
    cpu.pc = 0x28D70;
    16
}
/// $02D6E8: bcc.w $2d70a
pub fn i_02d6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D70A; return 10; }
    8
}
/// $02D6EC: lea.l $2b38c(pc), a1
pub fn i_02d6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B38C;
    16
}
/// $02D6F0: move.l a1, (a6)
pub fn i_02d6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D6F2: move.b #$28, $45(a6)
pub fn i_02d6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6F8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D6F8: move.b #$28, d0
pub fn i_02d6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6FC;
    cpu.ir = 0x103C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02D6FC: jsr $5e722.l
pub fn i_02d6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D702);
    cpu.pc = 0x5E722;
    16
}
/// $02D702: lea.l $29a14(pc), a0
pub fn i_02d702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D706;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29A14;
    16
}
/// $02D706: move.l a0, $48(a6)
pub fn i_02d706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D70A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02D70A: jsr $2a8c0(pc)
pub fn i_02d70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D70E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D70E);
    cpu.pc = 0x2A8C0;
    16
}
/// $02D70E: bcc.w $2d718
pub fn i_02d70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D712;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D718; return 10; }
    8
}
/// $02D712: lea.l $2d02e(pc), a1
pub fn i_02d712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D716;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02D716: move.l a1, (a6)
pub fn i_02d716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D718;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D718: jsr $2ac80(pc)
pub fn i_02d718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D71C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D71C);
    cpu.pc = 0x2AC80;
    16
}
/// $02D71C: bcc.w $2d726
pub fn i_02d71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D720;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D726; return 10; }
    8
}
/// $02D720: lea.l $2dd74(pc), a1
pub fn i_02d720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D724;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02D724: move.l a1, (a6)
pub fn i_02d724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D726;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D726: jsr $2a664(pc)
pub fn i_02d726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D72A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D72A);
    cpu.pc = 0x2A664;
    16
}
/// $02D72A: bcc.w $2d734
pub fn i_02d72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D72E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2D734; return 10; }
    8
}
// ===== SetTaskHandler_02d72e  ($02D72E, 8 B, kernel/task_handlers.c)
/// $02D72E: lea.l $2da38(pc), a1
pub fn i_02d72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D732;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DA38;
    16
}
/// $02D732: move.l a1, (a6)
pub fn i_02d732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D734;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D734: rts
pub fn i_02d734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D736;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_HitLaunchA_02d736  ($02D736, 204 B, slug/states_02ae3e.s)
/// $02D736: bclr.b #$5, $8c(a6)
pub fn i_02d736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D73C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D73C: jsr $2a9a0(pc)
pub fn i_02d73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D740;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D740);
    cpu.pc = 0x2A9A0;
    16
}
/// $02D740: movea.l #$279574, a0
pub fn i_02d740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D746;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279574;
    cpu.a[0] = v1;
    4
}
/// $02D746: lsl.w #$2, d0
pub fn i_02d746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D748;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D748: movea.l (a0, d0.w), a0
pub fn i_02d748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D74C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02D74C: cmpa.l #$ffffffff, a0
pub fn i_02d74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D752;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02D752: beq.w $2d75c
pub fn i_02d752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D756;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D75C; return 10; }
    8
}
/// $02D756: jsr $28cd4.l
pub fn i_02d756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D75C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D75C);
    cpu.pc = 0x28CD4;
    16
}
/// $02D75C: move.w #$0, d0
pub fn i_02d75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D760;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02D760: move.b #$96, $20(a6)
pub fn i_02d760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D766;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D766: andi.w #$ff, d0
pub fn i_02d766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D76A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02D76A: lsr.w #$2, d0
pub fn i_02d76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D76C;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02D76C: add.b d0, $20(a6)
pub fn i_02d76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D770;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02D770: move.w #$3fc, $2a(a6)
pub fn i_02d770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D776;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3FC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D776: move.w #$ffab, $2e(a6)
pub fn i_02d776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D77C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFAB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02D77C: bclr.b #$5, $8c(a6)
pub fn i_02d77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D782;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D782: clr.w $2c(a6)
pub fn i_02d782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D786;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02D786: lea.l $2d78c(pc), a1
pub fn i_02d786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D78A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D78C;
    16
}
/// $02D78A: move.l a1, (a6)
pub fn i_02d78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D78C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02D78C: bset.b #$1, $8d(a6)
pub fn i_02d78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D792;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D792: move.b #$28, $45(a6)
pub fn i_02d792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D798;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02D798: move.b #$28, d0
pub fn i_02d798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D79C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $02D79C: jsr $5e722.l
pub fn i_02d79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D7A2);
    cpu.pc = 0x5E722;
    16
}
/// $02D7A2: bclr.b #$3, $13(a6)
pub fn i_02d7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7A8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02D7A8: jsr $2aa0e(pc)
pub fn i_02d7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2D7AC);
    cpu.pc = 0x2AA0E;
    16
}
/// $02D7AC: move.w $2a(a6), d0
pub fn i_02d7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7B0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02D7B0: move.w #$400, d1
pub fn i_02d7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x02BE64, i_02be64),
    (0x02BE6A, i_02be6a),
    (0x02BE6E, i_02be6e),
    (0x02BE70, i_02be70),
    (0x02BE74, i_02be74),
    (0x02BE78, i_02be78),
    (0x02BE7A, i_02be7a),
    (0x02BE7E, i_02be7e),
    (0x02BE82, i_02be82),
    (0x02BE86, i_02be86),
    (0x02BE8A, i_02be8a),
    (0x02BE8E, i_02be8e),
    (0x02BE90, i_02be90),
    (0x02BE94, i_02be94),
    (0x02BE98, i_02be98),
    (0x02BE9C, i_02be9c),
    (0x02BEA0, i_02bea0),
    (0x02BEA2, i_02bea2),
    (0x02BEA6, i_02bea6),
    (0x02BEAA, i_02beaa),
    (0x02BEAE, i_02beae),
    (0x02BEB0, i_02beb0),
    (0x02BEB6, i_02beb6),
    (0x02BEBA, i_02beba),
    (0x02BEBE, i_02bebe),
    (0x02BEC0, i_02bec0),
    (0x02BEC6, i_02bec6),
    (0x02BECA, i_02beca),
    (0x02BECE, i_02bece),
    (0x02BED2, i_02bed2),
    (0x02BED4, i_02bed4),
    (0x02BED8, i_02bed8),
    (0x02BEDC, i_02bedc),
    (0x02BEE0, i_02bee0),
    (0x02BEE2, i_02bee2),
    (0x02BEE6, i_02bee6),
    (0x02BEEA, i_02beea),
    (0x02BEEE, i_02beee),
    (0x02BEF2, i_02bef2),
    (0x02BEF6, i_02bef6),
    (0x02BEFC, i_02befc),
    (0x02BF02, i_02bf02),
    (0x02BF06, i_02bf06),
    (0x02BF0A, i_02bf0a),
    (0x02BF0E, i_02bf0e),
    (0x02BF10, i_02bf10),
    (0x02BF14, i_02bf14),
    (0x02BF18, i_02bf18),
    (0x02BF1E, i_02bf1e),
    (0x02BF24, i_02bf24),
    (0x02BF28, i_02bf28),
    (0x02BF2C, i_02bf2c),
    (0x02BF30, i_02bf30),
    (0x02BF32, i_02bf32),
    (0x02BF36, i_02bf36),
    (0x02BF3A, i_02bf3a),
    (0x02BF3E, i_02bf3e),
    (0x02BF40, i_02bf40),
    (0x02BF44, i_02bf44),
    (0x02BF48, i_02bf48),
    (0x02BF4C, i_02bf4c),
    (0x02BF4E, i_02bf4e),
    (0x02BF52, i_02bf52),
    (0x02BF56, i_02bf56),
    (0x02BF5C, i_02bf5c),
    (0x02BF62, i_02bf62),
    (0x02BF64, i_02bf64),
    (0x02BF6A, i_02bf6a),
    (0x02BF6E, i_02bf6e),
    (0x02BF72, i_02bf72),
    (0x02BF78, i_02bf78),
    (0x02BF7A, i_02bf7a),
    (0x02BF7E, i_02bf7e),
    (0x02BF84, i_02bf84),
    (0x02BF88, i_02bf88),
    (0x02BF8E, i_02bf8e),
    (0x02BF92, i_02bf92),
    (0x02BF96, i_02bf96),
    (0x02BF9A, i_02bf9a),
    (0x02BFA0, i_02bfa0),
    (0x02BFA4, i_02bfa4),
    (0x02BFA6, i_02bfa6),
    (0x02BFAA, i_02bfaa),
    (0x02BFAE, i_02bfae),
    (0x02BFB0, i_02bfb0),
    (0x02BFB4, i_02bfb4),
    (0x02BFB8, i_02bfb8),
    (0x02BFBC, i_02bfbc),
    (0x02BFC0, i_02bfc0),
    (0x02BFC2, i_02bfc2),
    (0x02BFC6, i_02bfc6),
    (0x02BFCA, i_02bfca),
    (0x02BFCE, i_02bfce),
    (0x02BFD0, i_02bfd0),
    (0x02BFD4, i_02bfd4),
    (0x02BFD8, i_02bfd8),
    (0x02BFDC, i_02bfdc),
    (0x02BFDE, i_02bfde),
    (0x02BFE2, i_02bfe2),
    (0x02BFE6, i_02bfe6),
    (0x02BFEA, i_02bfea),
    (0x02BFEC, i_02bfec),
    (0x02BFF2, i_02bff2),
    (0x02BFF8, i_02bff8),
    (0x02BFFC, i_02bffc),
    (0x02C000, i_02c000),
    (0x02C004, i_02c004),
    (0x02C008, i_02c008),
    (0x02C00C, i_02c00c),
    (0x02C012, i_02c012),
    (0x02C018, i_02c018),
    (0x02C01C, i_02c01c),
    (0x02C020, i_02c020),
    (0x02C024, i_02c024),
    (0x02C026, i_02c026),
    (0x02C02A, i_02c02a),
    (0x02C02E, i_02c02e),
    (0x02C034, i_02c034),
    (0x02C03A, i_02c03a),
    (0x02C03E, i_02c03e),
    (0x02C042, i_02c042),
    (0x02C046, i_02c046),
    (0x02C048, i_02c048),
    (0x02C04C, i_02c04c),
    (0x02C050, i_02c050),
    (0x02C054, i_02c054),
    (0x02C056, i_02c056),
    (0x02C05A, i_02c05a),
    (0x02C05E, i_02c05e),
    (0x02C062, i_02c062),
    (0x02C064, i_02c064),
    (0x02C068, i_02c068),
    (0x02C06C, i_02c06c),
    (0x02C072, i_02c072),
    (0x02C078, i_02c078),
    (0x02C07A, i_02c07a),
    (0x02C080, i_02c080),
    (0x02C084, i_02c084),
    (0x02C088, i_02c088),
    (0x02C08E, i_02c08e),
    (0x02C090, i_02c090),
    (0x02C094, i_02c094),
    (0x02C09A, i_02c09a),
    (0x02C09E, i_02c09e),
    (0x02C0A4, i_02c0a4),
    (0x02C0A8, i_02c0a8),
    (0x02C0AC, i_02c0ac),
    (0x02C0B0, i_02c0b0),
    (0x02C0B6, i_02c0b6),
    (0x02C0BA, i_02c0ba),
    (0x02C0BC, i_02c0bc),
    (0x02C0C0, i_02c0c0),
    (0x02C0C4, i_02c0c4),
    (0x02C0C6, i_02c0c6),
    (0x02C0CA, i_02c0ca),
    (0x02C0CE, i_02c0ce),
    (0x02C0D2, i_02c0d2),
    (0x02C0D6, i_02c0d6),
    (0x02C0D8, i_02c0d8),
    (0x02C0DC, i_02c0dc),
    (0x02C0E0, i_02c0e0),
    (0x02C0E4, i_02c0e4),
    (0x02C0E6, i_02c0e6),
    (0x02C0EA, i_02c0ea),
    (0x02C0EE, i_02c0ee),
    (0x02C0F2, i_02c0f2),
    (0x02C0F4, i_02c0f4),
    (0x02C0F8, i_02c0f8),
    (0x02C0FC, i_02c0fc),
    (0x02C100, i_02c100),
    (0x02C102, i_02c102),
    (0x02C108, i_02c108),
    (0x02C10E, i_02c10e),
    (0x02C112, i_02c112),
    (0x02C116, i_02c116),
    (0x02C11A, i_02c11a),
    (0x02C11E, i_02c11e),
    (0x02C122, i_02c122),
    (0x02C128, i_02c128),
    (0x02C12E, i_02c12e),
    (0x02C132, i_02c132),
    (0x02C136, i_02c136),
    (0x02C13A, i_02c13a),
    (0x02C13C, i_02c13c),
    (0x02C140, i_02c140),
    (0x02C144, i_02c144),
    (0x02C14A, i_02c14a),
    (0x02C150, i_02c150),
    (0x02C154, i_02c154),
    (0x02C158, i_02c158),
    (0x02C15C, i_02c15c),
    (0x02C15E, i_02c15e),
    (0x02C162, i_02c162),
    (0x02C166, i_02c166),
    (0x02C16A, i_02c16a),
    (0x02C16C, i_02c16c),
    (0x02C170, i_02c170),
    (0x02C174, i_02c174),
    (0x02C178, i_02c178),
    (0x02C17A, i_02c17a),
    (0x02C17E, i_02c17e),
    (0x02C182, i_02c182),
    (0x02C188, i_02c188),
    (0x02C18E, i_02c18e),
    (0x02C190, i_02c190),
    (0x02C196, i_02c196),
    (0x02C19A, i_02c19a),
    (0x02C1A0, i_02c1a0),
    (0x02C1A2, i_02c1a2),
    (0x02C1A6, i_02c1a6),
    (0x02C1AC, i_02c1ac),
    (0x02C1B0, i_02c1b0),
    (0x02C1B6, i_02c1b6),
    (0x02C1BA, i_02c1ba),
    (0x02C1BE, i_02c1be),
    (0x02C1C2, i_02c1c2),
    (0x02C1C8, i_02c1c8),
    (0x02C1CC, i_02c1cc),
    (0x02C1CE, i_02c1ce),
    (0x02C1D2, i_02c1d2),
    (0x02C1D6, i_02c1d6),
    (0x02C1DC, i_02c1dc),
    (0x02C1E0, i_02c1e0),
    (0x02C1E6, i_02c1e6),
    (0x02C1EA, i_02c1ea),
    (0x02C1F0, i_02c1f0),
    (0x02C1F6, i_02c1f6),
    (0x02C1FC, i_02c1fc),
    (0x02C200, i_02c200),
    (0x02C206, i_02c206),
    (0x02C20C, i_02c20c),
    (0x02C212, i_02c212),
    (0x02C218, i_02c218),
    (0x02C21E, i_02c21e),
    (0x02C224, i_02c224),
    (0x02C22A, i_02c22a),
    (0x02C22E, i_02c22e),
    (0x02C234, i_02c234),
    (0x02C23A, i_02c23a),
    (0x02C240, i_02c240),
    (0x02C246, i_02c246),
    (0x02C24A, i_02c24a),
    (0x02C250, i_02c250),
    (0x02C254, i_02c254),
    (0x02C25A, i_02c25a),
    (0x02C25C, i_02c25c),
    (0x02C260, i_02c260),
    (0x02C266, i_02c266),
    (0x02C26A, i_02c26a),
    (0x02C270, i_02c270),
    (0x02C274, i_02c274),
    (0x02C278, i_02c278),
    (0x02C27C, i_02c27c),
    (0x02C282, i_02c282),
    (0x02C286, i_02c286),
    (0x02C288, i_02c288),
    (0x02C28C, i_02c28c),
    (0x02C290, i_02c290),
    (0x02C296, i_02c296),
    (0x02C29A, i_02c29a),
    (0x02C2A0, i_02c2a0),
    (0x02C2A4, i_02c2a4),
    (0x02C2AA, i_02c2aa),
    (0x02C2B0, i_02c2b0),
    (0x02C2B6, i_02c2b6),
    (0x02C2BA, i_02c2ba),
    (0x02C2C0, i_02c2c0),
    (0x02C2C6, i_02c2c6),
    (0x02C2CC, i_02c2cc),
    (0x02C2D2, i_02c2d2),
    (0x02C2D8, i_02c2d8),
    (0x02C2DE, i_02c2de),
    (0x02C2E4, i_02c2e4),
    (0x02C2E8, i_02c2e8),
    (0x02C2EE, i_02c2ee),
    (0x02C2F4, i_02c2f4),
    (0x02C2FA, i_02c2fa),
    (0x02C300, i_02c300),
    (0x02C306, i_02c306),
    (0x02C30A, i_02c30a),
    (0x02C30E, i_02c30e),
    (0x02C314, i_02c314),
    (0x02C31A, i_02c31a),
    (0x02C322, i_02c322),
    (0x02C328, i_02c328),
    (0x02C32C, i_02c32c),
    (0x02C330, i_02c330),
    (0x02C334, i_02c334),
    (0x02C338, i_02c338),
    (0x02C33A, i_02c33a),
    (0x02C33E, i_02c33e),
    (0x02C342, i_02c342),
    (0x02C346, i_02c346),
    (0x02C348, i_02c348),
    (0x02C34C, i_02c34c),
    (0x02C350, i_02c350),
    (0x02C354, i_02c354),
    (0x02C358, i_02c358),
    (0x02C35C, i_02c35c),
    (0x02C35E, i_02c35e),
    (0x02C362, i_02c362),
    (0x02C366, i_02c366),
    (0x02C36A, i_02c36a),
    (0x02C36C, i_02c36c),
    (0x02C370, i_02c370),
    (0x02C374, i_02c374),
    (0x02C378, i_02c378),
    (0x02C37E, i_02c37e),
    (0x02C382, i_02c382),
    (0x02C386, i_02c386),
    (0x02C38A, i_02c38a),
    (0x02C38E, i_02c38e),
    (0x02C392, i_02c392),
    (0x02C396, i_02c396),
    (0x02C39A, i_02c39a),
    (0x02C39E, i_02c39e),
    (0x02C3A0, i_02c3a0),
    (0x02C3A4, i_02c3a4),
    (0x02C3AA, i_02c3aa),
    (0x02C3AE, i_02c3ae),
    (0x02C3B2, i_02c3b2),
    (0x02C3B4, i_02c3b4),
    (0x02C3BA, i_02c3ba),
    (0x02C3BE, i_02c3be),
    (0x02C3C2, i_02c3c2),
    (0x02C3C6, i_02c3c6),
    (0x02C3CA, i_02c3ca),
    (0x02C3CE, i_02c3ce),
    (0x02C3D2, i_02c3d2),
    (0x02C3D8, i_02c3d8),
    (0x02C3DE, i_02c3de),
    (0x02C3E2, i_02c3e2),
    (0x02C3E6, i_02c3e6),
    (0x02C3EC, i_02c3ec),
    (0x02C3F2, i_02c3f2),
    (0x02C3F6, i_02c3f6),
    (0x02C3FA, i_02c3fa),
    (0x02C3FE, i_02c3fe),
    (0x02C400, i_02c400),
    (0x02C404, i_02c404),
    (0x02C408, i_02c408),
    (0x02C40C, i_02c40c),
    (0x02C40E, i_02c40e),
    (0x02C412, i_02c412),
    (0x02C416, i_02c416),
    (0x02C41A, i_02c41a),
    (0x02C41C, i_02c41c),
    (0x02C420, i_02c420),
    (0x02C424, i_02c424),
    (0x02C42A, i_02c42a),
    (0x02C430, i_02c430),
    (0x02C432, i_02c432),
    (0x02C438, i_02c438),
    (0x02C43C, i_02c43c),
    (0x02C442, i_02c442),
    (0x02C444, i_02c444),
    (0x02C448, i_02c448),
    (0x02C44E, i_02c44e),
    (0x02C452, i_02c452),
    (0x02C458, i_02c458),
    (0x02C45E, i_02c45e),
    (0x02C462, i_02c462),
    (0x02C464, i_02c464),
    (0x02C468, i_02c468),
    (0x02C46C, i_02c46c),
    (0x02C46E, i_02c46e),
    (0x02C472, i_02c472),
    (0x02C476, i_02c476),
    (0x02C47A, i_02c47a),
    (0x02C47E, i_02c47e),
    (0x02C482, i_02c482),
    (0x02C486, i_02c486),
    (0x02C48A, i_02c48a),
    (0x02C48E, i_02c48e),
    (0x02C490, i_02c490),
    (0x02C494, i_02c494),
    (0x02C498, i_02c498),
    (0x02C49C, i_02c49c),
    (0x02C49E, i_02c49e),
    (0x02C4A2, i_02c4a2),
    (0x02C4A6, i_02c4a6),
    (0x02C4AC, i_02c4ac),
    (0x02C4B0, i_02c4b0),
    (0x02C4B4, i_02c4b4),
    (0x02C4B8, i_02c4b8),
    (0x02C4BC, i_02c4bc),
    (0x02C4BE, i_02c4be),
    (0x02C4C2, i_02c4c2),
    (0x02C4C8, i_02c4c8),
    (0x02C4CC, i_02c4cc),
    (0x02C4D0, i_02c4d0),
    (0x02C4D2, i_02c4d2),
    (0x02C4D6, i_02c4d6),
    (0x02C4DA, i_02c4da),
    (0x02C4DE, i_02c4de),
    (0x02C4E2, i_02c4e2),
    (0x02C4E6, i_02c4e6),
    (0x02C4EC, i_02c4ec),
    (0x02C4F2, i_02c4f2),
    (0x02C4F6, i_02c4f6),
    (0x02C4FA, i_02c4fa),
    (0x02C4FE, i_02c4fe),
    (0x02C500, i_02c500),
    (0x02C504, i_02c504),
    (0x02C508, i_02c508),
    (0x02C50E, i_02c50e),
    (0x02C514, i_02c514),
    (0x02C518, i_02c518),
    (0x02C51C, i_02c51c),
    (0x02C520, i_02c520),
    (0x02C522, i_02c522),
    (0x02C526, i_02c526),
    (0x02C52A, i_02c52a),
    (0x02C52E, i_02c52e),
    (0x02C530, i_02c530),
    (0x02C534, i_02c534),
    (0x02C538, i_02c538),
    (0x02C53C, i_02c53c),
    (0x02C53E, i_02c53e),
    (0x02C542, i_02c542),
    (0x02C546, i_02c546),
    (0x02C54C, i_02c54c),
    (0x02C552, i_02c552),
    (0x02C554, i_02c554),
    (0x02C55A, i_02c55a),
    (0x02C560, i_02c560),
    (0x02C568, i_02c568),
    (0x02C56C, i_02c56c),
    (0x02C56E, i_02c56e),
    (0x02C572, i_02c572),
    (0x02C578, i_02c578),
    (0x02C57C, i_02c57c),
    (0x02C580, i_02c580),
    (0x02C582, i_02c582),
    (0x02C586, i_02c586),
    (0x02C58A, i_02c58a),
    (0x02C58E, i_02c58e),
    (0x02C592, i_02c592),
    (0x02C594, i_02c594),
    (0x02C598, i_02c598),
    (0x02C59C, i_02c59c),
    (0x02C5A0, i_02c5a0),
    (0x02C5A2, i_02c5a2),
    (0x02C5A6, i_02c5a6),
    (0x02C5AA, i_02c5aa),
    (0x02C5AE, i_02c5ae),
    (0x02C5B0, i_02c5b0),
    (0x02C5B4, i_02c5b4),
    (0x02C5B8, i_02c5b8),
    (0x02C5BC, i_02c5bc),
    (0x02C5BE, i_02c5be),
    (0x02C5C4, i_02c5c4),
    (0x02C5C8, i_02c5c8),
    (0x02C5CC, i_02c5cc),
    (0x02C5CE, i_02c5ce),
    (0x02C5D4, i_02c5d4),
    (0x02C5D8, i_02c5d8),
    (0x02C5DC, i_02c5dc),
    (0x02C5E0, i_02c5e0),
    (0x02C5E4, i_02c5e4),
    (0x02C5E8, i_02c5e8),
    (0x02C5EE, i_02c5ee),
    (0x02C5F4, i_02c5f4),
    (0x02C5F8, i_02c5f8),
    (0x02C5FC, i_02c5fc),
    (0x02C600, i_02c600),
    (0x02C602, i_02c602),
    (0x02C606, i_02c606),
    (0x02C60A, i_02c60a),
    (0x02C610, i_02c610),
    (0x02C616, i_02c616),
    (0x02C61A, i_02c61a),
    (0x02C61E, i_02c61e),
    (0x02C622, i_02c622),
    (0x02C624, i_02c624),
    (0x02C628, i_02c628),
    (0x02C62C, i_02c62c),
    (0x02C630, i_02c630),
    (0x02C632, i_02c632),
    (0x02C636, i_02c636),
    (0x02C63A, i_02c63a),
    (0x02C640, i_02c640),
    (0x02C646, i_02c646),
    (0x02C648, i_02c648),
    (0x02C64E, i_02c64e),
    (0x02C652, i_02c652),
    (0x02C656, i_02c656),
    (0x02C658, i_02c658),
    (0x02C65C, i_02c65c),
    (0x02C660, i_02c660),
    (0x02C664, i_02c664),
    (0x02C668, i_02c668),
    (0x02C66A, i_02c66a),
    (0x02C66E, i_02c66e),
    (0x02C672, i_02c672),
    (0x02C676, i_02c676),
    (0x02C678, i_02c678),
    (0x02C67C, i_02c67c),
    (0x02C680, i_02c680),
    (0x02C684, i_02c684),
    (0x02C686, i_02c686),
    (0x02C68A, i_02c68a),
    (0x02C68E, i_02c68e),
    (0x02C692, i_02c692),
    (0x02C694, i_02c694),
    (0x02C69A, i_02c69a),
    (0x02C69E, i_02c69e),
    (0x02C6A2, i_02c6a2),
    (0x02C6A4, i_02c6a4),
    (0x02C6AA, i_02c6aa),
    (0x02C6AE, i_02c6ae),
    (0x02C6B2, i_02c6b2),
    (0x02C6B6, i_02c6b6),
    (0x02C6BA, i_02c6ba),
    (0x02C6BE, i_02c6be),
    (0x02C6C4, i_02c6c4),
    (0x02C6CA, i_02c6ca),
    (0x02C6CE, i_02c6ce),
    (0x02C6D2, i_02c6d2),
    (0x02C6D6, i_02c6d6),
    (0x02C6D8, i_02c6d8),
    (0x02C6DC, i_02c6dc),
    (0x02C6E0, i_02c6e0),
    (0x02C6E6, i_02c6e6),
    (0x02C6EC, i_02c6ec),
    (0x02C6F0, i_02c6f0),
    (0x02C6F4, i_02c6f4),
    (0x02C6F8, i_02c6f8),
    (0x02C6FA, i_02c6fa),
    (0x02C6FE, i_02c6fe),
    (0x02C702, i_02c702),
    (0x02C706, i_02c706),
    (0x02C708, i_02c708),
    (0x02C70C, i_02c70c),
    (0x02C710, i_02c710),
    (0x02C716, i_02c716),
    (0x02C71C, i_02c71c),
    (0x02C71E, i_02c71e),
    (0x02C724, i_02c724),
    (0x02C728, i_02c728),
    (0x02C72E, i_02c72e),
    (0x02C732, i_02c732),
    (0x02C738, i_02c738),
    (0x02C73C, i_02c73c),
    (0x02C742, i_02c742),
    (0x02C748, i_02c748),
    (0x02C74E, i_02c74e),
    (0x02C752, i_02c752),
    (0x02C758, i_02c758),
    (0x02C75E, i_02c75e),
    (0x02C764, i_02c764),
    (0x02C76A, i_02c76a),
    (0x02C770, i_02c770),
    (0x02C776, i_02c776),
    (0x02C77C, i_02c77c),
    (0x02C780, i_02c780),
    (0x02C786, i_02c786),
    (0x02C78C, i_02c78c),
    (0x02C792, i_02c792),
    (0x02C798, i_02c798),
    (0x02C79E, i_02c79e),
    (0x02C7A2, i_02c7a2),
    (0x02C7A6, i_02c7a6),
    (0x02C7AC, i_02c7ac),
    (0x02C7B0, i_02c7b0),
    (0x02C7B4, i_02c7b4),
    (0x02C7B8, i_02c7b8),
    (0x02C7BC, i_02c7bc),
    (0x02C7BE, i_02c7be),
    (0x02C7C0, i_02c7c0),
    (0x02C7C2, i_02c7c2),
    (0x02C7C6, i_02c7c6),
    (0x02C7CA, i_02c7ca),
    (0x02C7CE, i_02c7ce),
    (0x02C7D2, i_02c7d2),
    (0x02C7D6, i_02c7d6),
    (0x02C7DA, i_02c7da),
    (0x02C7E0, i_02c7e0),
    (0x02C7E2, i_02c7e2),
    (0x02C7E6, i_02c7e6),
    (0x02C7EC, i_02c7ec),
    (0x02C7F0, i_02c7f0),
    (0x02C7F6, i_02c7f6),
    (0x02C7FA, i_02c7fa),
    (0x02C800, i_02c800),
    (0x02C802, i_02c802),
    (0x02C806, i_02c806),
    (0x02C80A, i_02c80a),
    (0x02C810, i_02c810),
    (0x02C818, i_02c818),
    (0x02C81C, i_02c81c),
    (0x02C81E, i_02c81e),
    (0x02C824, i_02c824),
    (0x02C828, i_02c828),
    (0x02C82C, i_02c82c),
    (0x02C830, i_02c830),
    (0x02C836, i_02c836),
    (0x02C83A, i_02c83a),
    (0x02C83E, i_02c83e),
    (0x02C842, i_02c842),
    (0x02C848, i_02c848),
    (0x02C84C, i_02c84c),
    (0x02C850, i_02c850),
    (0x02C854, i_02c854),
    (0x02C858, i_02c858),
    (0x02C85A, i_02c85a),
    (0x02C85C, i_02c85c),
    (0x02C85E, i_02c85e),
    (0x02C862, i_02c862),
    (0x02C866, i_02c866),
    (0x02C86A, i_02c86a),
    (0x02C86E, i_02c86e),
    (0x02C872, i_02c872),
    (0x02C876, i_02c876),
    (0x02C87C, i_02c87c),
    (0x02C87E, i_02c87e),
    (0x02C882, i_02c882),
    (0x02C888, i_02c888),
    (0x02C88C, i_02c88c),
    (0x02C892, i_02c892),
    (0x02C896, i_02c896),
    (0x02C89C, i_02c89c),
    (0x02C89E, i_02c89e),
    (0x02C8A2, i_02c8a2),
    (0x02C8A6, i_02c8a6),
    (0x02C8AA, i_02c8aa),
    (0x02C8AC, i_02c8ac),
    (0x02C8B0, i_02c8b0),
    (0x02C8B4, i_02c8b4),
    (0x02C8B8, i_02c8b8),
    (0x02C8BA, i_02c8ba),
    (0x02C8BE, i_02c8be),
    (0x02C8C2, i_02c8c2),
    (0x02C8C8, i_02c8c8),
    (0x02C8CE, i_02c8ce),
    (0x02C8D2, i_02c8d2),
    (0x02C8D6, i_02c8d6),
    (0x02C8DA, i_02c8da),
    (0x02C8DC, i_02c8dc),
    (0x02C8E0, i_02c8e0),
    (0x02C8E4, i_02c8e4),
    (0x02C8E8, i_02c8e8),
    (0x02C8EA, i_02c8ea),
    (0x02C8EE, i_02c8ee),
    (0x02C8F2, i_02c8f2),
    (0x02C8F8, i_02c8f8),
    (0x02C8FE, i_02c8fe),
    (0x02C908, i_02c908),
    (0x02C90E, i_02c90e),
    (0x02C912, i_02c912),
    (0x02C916, i_02c916),
    (0x02C91C, i_02c91c),
    (0x02C91E, i_02c91e),
    (0x02C922, i_02c922),
    (0x02C928, i_02c928),
    (0x02C92C, i_02c92c),
    (0x02C932, i_02c932),
    (0x02C936, i_02c936),
    (0x02C93A, i_02c93a),
    (0x02C93E, i_02c93e),
    (0x02C944, i_02c944),
    (0x02C948, i_02c948),
    (0x02C94A, i_02c94a),
    (0x02C94E, i_02c94e),
    (0x02C956, i_02c956),
    (0x02C95A, i_02c95a),
    (0x02C95C, i_02c95c),
    (0x02C960, i_02c960),
    (0x02C964, i_02c964),
    (0x02C968, i_02c968),
    (0x02C96E, i_02c96e),
    (0x02C972, i_02c972),
    (0x02C976, i_02c976),
    (0x02C978, i_02c978),
    (0x02C97E, i_02c97e),
    (0x02C982, i_02c982),
    (0x02C986, i_02c986),
    (0x02C98A, i_02c98a),
    (0x02C98C, i_02c98c),
    (0x02C990, i_02c990),
    (0x02C994, i_02c994),
    (0x02C998, i_02c998),
    (0x02C99A, i_02c99a),
    (0x02C99E, i_02c99e),
    (0x02C9A2, i_02c9a2),
    (0x02C9A8, i_02c9a8),
    (0x02C9AE, i_02c9ae),
    (0x02C9C4, i_02c9c4),
    (0x02C9CA, i_02c9ca),
    (0x02C9D0, i_02c9d0),
    (0x02C9D4, i_02c9d4),
    (0x02C9DA, i_02c9da),
    (0x02C9DE, i_02c9de),
    (0x02C9E0, i_02c9e0),
    (0x02C9E6, i_02c9e6),
    (0x02C9EC, i_02c9ec),
    (0x02C9F0, i_02c9f0),
    (0x02C9F6, i_02c9f6),
    (0x02C9FA, i_02c9fa),
    (0x02C9FC, i_02c9fc),
    (0x02CA02, i_02ca02),
    (0x02CA08, i_02ca08),
    (0x02CA0C, i_02ca0c),
    (0x02CA12, i_02ca12),
    (0x02CA18, i_02ca18),
    (0x02CA1E, i_02ca1e),
    (0x02CA24, i_02ca24),
    (0x02CA28, i_02ca28),
    (0x02CA2C, i_02ca2c),
    (0x02CA30, i_02ca30),
    (0x02CA34, i_02ca34),
    (0x02CA38, i_02ca38),
    (0x02CA3E, i_02ca3e),
    (0x02CA40, i_02ca40),
    (0x02CA44, i_02ca44),
    (0x02CA4A, i_02ca4a),
    (0x02CA4E, i_02ca4e),
    (0x02CA54, i_02ca54),
    (0x02CA58, i_02ca58),
    (0x02CA5C, i_02ca5c),
    (0x02CA60, i_02ca60),
    (0x02CA66, i_02ca66),
    (0x02CA6A, i_02ca6a),
    (0x02CA6C, i_02ca6c),
    (0x02CA70, i_02ca70),
    (0x02CA74, i_02ca74),
    (0x02CA76, i_02ca76),
    (0x02CA7A, i_02ca7a),
    (0x02CA7E, i_02ca7e),
    (0x02CA82, i_02ca82),
    (0x02CA86, i_02ca86),
    (0x02CA8A, i_02ca8a),
    (0x02CA8E, i_02ca8e),
    (0x02CA92, i_02ca92),
    (0x02CA96, i_02ca96),
    (0x02CA98, i_02ca98),
    (0x02CA9E, i_02ca9e),
    (0x02CAA0, i_02caa0),
    (0x02CAA4, i_02caa4),
    (0x02CAAA, i_02caaa),
    (0x02CAAE, i_02caae),
    (0x02CAB2, i_02cab2),
    (0x02CAB8, i_02cab8),
    (0x02CABC, i_02cabc),
    (0x02CAC0, i_02cac0),
    (0x02CAC4, i_02cac4),
    (0x02CAC8, i_02cac8),
    (0x02CACE, i_02cace),
    (0x02CAD2, i_02cad2),
    (0x02CAD8, i_02cad8),
    (0x02CADC, i_02cadc),
    (0x02CAE0, i_02cae0),
    (0x02CAE4, i_02cae4),
    (0x02CAE8, i_02cae8),
    (0x02CAEA, i_02caea),
    (0x02CAF0, i_02caf0),
    (0x02CAF4, i_02caf4),
    (0x02CAF8, i_02caf8),
    (0x02CAFC, i_02cafc),
    (0x02CB00, i_02cb00),
    (0x02CB06, i_02cb06),
    (0x02CB0C, i_02cb0c),
    (0x02CB10, i_02cb10),
    (0x02CB14, i_02cb14),
    (0x02CB18, i_02cb18),
    (0x02CB1A, i_02cb1a),
    (0x02CB20, i_02cb20),
    (0x02CB24, i_02cb24),
    (0x02CB28, i_02cb28),
    (0x02CB2C, i_02cb2c),
    (0x02CB30, i_02cb30),
    (0x02CB34, i_02cb34),
    (0x02CB38, i_02cb38),
    (0x02CB3C, i_02cb3c),
    (0x02CB40, i_02cb40),
    (0x02CB44, i_02cb44),
    (0x02CB48, i_02cb48),
    (0x02CB4C, i_02cb4c),
    (0x02CB4E, i_02cb4e),
    (0x02CB50, i_02cb50),
    (0x02CB52, i_02cb52),
    (0x02CB56, i_02cb56),
    (0x02CB5A, i_02cb5a),
    (0x02CB5E, i_02cb5e),
    (0x02CB62, i_02cb62),
    (0x02CB66, i_02cb66),
    (0x02CB6A, i_02cb6a),
    (0x02CB70, i_02cb70),
    (0x02CB72, i_02cb72),
    (0x02CB76, i_02cb76),
    (0x02CB7C, i_02cb7c),
    (0x02CB80, i_02cb80),
    (0x02CB86, i_02cb86),
    (0x02CB8A, i_02cb8a),
    (0x02CB90, i_02cb90),
    (0x02CB92, i_02cb92),
    (0x02CB96, i_02cb96),
    (0x02CB9A, i_02cb9a),
    (0x02CBA0, i_02cba0),
    (0x02CBA4, i_02cba4),
    (0x02CBA6, i_02cba6),
    (0x02CBAA, i_02cbaa),
    (0x02CBAE, i_02cbae),
    (0x02CBB0, i_02cbb0),
    (0x02CBB6, i_02cbb6),
    (0x02CBBA, i_02cbba),
    (0x02CBBE, i_02cbbe),
    (0x02CBC2, i_02cbc2),
    (0x02CBC8, i_02cbc8),
    (0x02CBCE, i_02cbce),
    (0x02CBD2, i_02cbd2),
    (0x02CBD6, i_02cbd6),
    (0x02CBDA, i_02cbda),
    (0x02CBE0, i_02cbe0),
    (0x02CBE6, i_02cbe6),
    (0x02CBEC, i_02cbec),
    (0x02CBF2, i_02cbf2),
    (0x02CBF8, i_02cbf8),
    (0x02CBFC, i_02cbfc),
    (0x02CC00, i_02cc00),
    (0x02CC02, i_02cc02),
    (0x02CC08, i_02cc08),
    (0x02CC0C, i_02cc0c),
    (0x02CC10, i_02cc10),
    (0x02CC14, i_02cc14),
    (0x02CC16, i_02cc16),
    (0x02CC1A, i_02cc1a),
    (0x02CC20, i_02cc20),
    (0x02CC26, i_02cc26),
    (0x02CC2A, i_02cc2a),
    (0x02CC2E, i_02cc2e),
    (0x02CC32, i_02cc32),
    (0x02CC36, i_02cc36),
    (0x02CC3A, i_02cc3a),
    (0x02CC3C, i_02cc3c),
    (0x02CC3E, i_02cc3e),
    (0x02CC42, i_02cc42),
    (0x02CC44, i_02cc44),
    (0x02CC46, i_02cc46),
    (0x02CC4A, i_02cc4a),
    (0x02CC4E, i_02cc4e),
    (0x02CC52, i_02cc52),
    (0x02CC56, i_02cc56),
    (0x02CC5A, i_02cc5a),
    (0x02CC5E, i_02cc5e),
    (0x02CC62, i_02cc62),
    (0x02CC64, i_02cc64),
    (0x02CC68, i_02cc68),
    (0x02CC6E, i_02cc6e),
    (0x02CC72, i_02cc72),
    (0x02CC78, i_02cc78),
    (0x02CC7A, i_02cc7a),
    (0x02CC7E, i_02cc7e),
    (0x02CC84, i_02cc84),
    (0x02CC88, i_02cc88),
    (0x02CC8E, i_02cc8e),
    (0x02CC92, i_02cc92),
    (0x02CC96, i_02cc96),
    (0x02CC9A, i_02cc9a),
    (0x02CCA0, i_02cca0),
    (0x02CCA4, i_02cca4),
    (0x02CCA6, i_02cca6),
    (0x02CCAA, i_02ccaa),
    (0x02CCAE, i_02ccae),
    (0x02CCB0, i_02ccb0),
    (0x02CCB4, i_02ccb4),
    (0x02CCBA, i_02ccba),
    (0x02CCBE, i_02ccbe),
    (0x02CCC2, i_02ccc2),
    (0x02CCC6, i_02ccc6),
    (0x02CCCA, i_02ccca),
    (0x02CCD0, i_02ccd0),
    (0x02CCD6, i_02ccd6),
    (0x02CCDC, i_02ccdc),
    (0x02CCDE, i_02ccde),
    (0x02CCE2, i_02cce2),
    (0x02CCE8, i_02cce8),
    (0x02CCEC, i_02ccec),
    (0x02CCF0, i_02ccf0),
    (0x02CCF4, i_02ccf4),
    (0x02CCF6, i_02ccf6),
    (0x02CCF8, i_02ccf8),
    (0x02CCFC, i_02ccfc),
    (0x02CD00, i_02cd00),
    (0x02CD04, i_02cd04),
    (0x02CD0A, i_02cd0a),
    (0x02CD0E, i_02cd0e),
    (0x02CD12, i_02cd12),
    (0x02CD16, i_02cd16),
    (0x02CD1C, i_02cd1c),
    (0x02CD22, i_02cd22),
    (0x02CD26, i_02cd26),
    (0x02CD2A, i_02cd2a),
    (0x02CD2E, i_02cd2e),
    (0x02CD32, i_02cd32),
    (0x02CD34, i_02cd34),
    (0x02CD38, i_02cd38),
    (0x02CD3C, i_02cd3c),
    (0x02CD40, i_02cd40),
    (0x02CD44, i_02cd44),
    (0x02CD48, i_02cd48),
    (0x02CD4A, i_02cd4a),
    (0x02CD4E, i_02cd4e),
    (0x02CD52, i_02cd52),
    (0x02CD56, i_02cd56),
    (0x02CD5A, i_02cd5a),
    (0x02CD5C, i_02cd5c),
    (0x02CD5E, i_02cd5e),
    (0x02CD60, i_02cd60),
    (0x02CD64, i_02cd64),
    (0x02CD68, i_02cd68),
    (0x02CD6C, i_02cd6c),
    (0x02CD70, i_02cd70),
    (0x02CD74, i_02cd74),
    (0x02CD78, i_02cd78),
    (0x02CD7E, i_02cd7e),
    (0x02CD80, i_02cd80),
    (0x02CD84, i_02cd84),
    (0x02CD8A, i_02cd8a),
    (0x02CD8E, i_02cd8e),
    (0x02CD94, i_02cd94),
    (0x02CD98, i_02cd98),
    (0x02CD9E, i_02cd9e),
    (0x02CDA0, i_02cda0),
    (0x02CDA4, i_02cda4),
    (0x02CDA8, i_02cda8),
    (0x02CDAE, i_02cdae),
    (0x02CDB2, i_02cdb2),
    (0x02CDB4, i_02cdb4),
    (0x02CDBA, i_02cdba),
    (0x02CDBE, i_02cdbe),
    (0x02CDC2, i_02cdc2),
    (0x02CDC6, i_02cdc6),
    (0x02CDC8, i_02cdc8),
    (0x02CDCC, i_02cdcc),
    (0x02CDD0, i_02cdd0),
    (0x02CDD6, i_02cdd6),
    (0x02CDDC, i_02cddc),
    (0x02CDE0, i_02cde0),
    (0x02CDE4, i_02cde4),
    (0x02CDE8, i_02cde8),
    (0x02CDEA, i_02cdea),
    (0x02CDEE, i_02cdee),
    (0x02CDF2, i_02cdf2),
    (0x02CDF6, i_02cdf6),
    (0x02CDF8, i_02cdf8),
    (0x02CDFC, i_02cdfc),
    (0x02CE00, i_02ce00),
    (0x02CE04, i_02ce04),
    (0x02CE06, i_02ce06),
    (0x02CE0A, i_02ce0a),
    (0x02CE0E, i_02ce0e),
    (0x02CE14, i_02ce14),
    (0x02CE1A, i_02ce1a),
    (0x02CE1C, i_02ce1c),
    (0x02CE22, i_02ce22),
    (0x02CE26, i_02ce26),
    (0x02CE2A, i_02ce2a),
    (0x02CE2C, i_02ce2c),
    (0x02CE2E, i_02ce2e),
    (0x02CE30, i_02ce30),
    (0x02CE34, i_02ce34),
    (0x02CE38, i_02ce38),
    (0x02CE3C, i_02ce3c),
    (0x02CE40, i_02ce40),
    (0x02CE44, i_02ce44),
    (0x02CE48, i_02ce48),
    (0x02CE4E, i_02ce4e),
    (0x02CE50, i_02ce50),
    (0x02CE54, i_02ce54),
    (0x02CE5A, i_02ce5a),
    (0x02CE5E, i_02ce5e),
    (0x02CE64, i_02ce64),
    (0x02CE68, i_02ce68),
    (0x02CE6E, i_02ce6e),
    (0x02CE70, i_02ce70),
    (0x02CE74, i_02ce74),
    (0x02CE78, i_02ce78),
    (0x02CE7C, i_02ce7c),
    (0x02CE7E, i_02ce7e),
    (0x02CE84, i_02ce84),
    (0x02CE8A, i_02ce8a),
    (0x02CE8E, i_02ce8e),
    (0x02CE92, i_02ce92),
    (0x02CE96, i_02ce96),
    (0x02CE9A, i_02ce9a),
    (0x02CE9C, i_02ce9c),
    (0x02CEA2, i_02cea2),
    (0x02CEA4, i_02cea4),
    (0x02CEA8, i_02cea8),
    (0x02CEAE, i_02ceae),
    (0x02CEB2, i_02ceb2),
    (0x02CEB6, i_02ceb6),
    (0x02CEBA, i_02ceba),
    (0x02CEBC, i_02cebc),
    (0x02CEBE, i_02cebe),
    (0x02CEC2, i_02cec2),
    (0x02CEC6, i_02cec6),
    (0x02CECA, i_02ceca),
    (0x02CED0, i_02ced0),
    (0x02CED4, i_02ced4),
    (0x02CED8, i_02ced8),
    (0x02CEDC, i_02cedc),
    (0x02CEE2, i_02cee2),
    (0x02CEE6, i_02cee6),
    (0x02CEEA, i_02ceea),
    (0x02CEEE, i_02ceee),
    (0x02CEF2, i_02cef2),
    (0x02CEF6, i_02cef6),
    (0x02CEFA, i_02cefa),
    (0x02CEFE, i_02cefe),
    (0x02CF02, i_02cf02),
    (0x02CF06, i_02cf06),
    (0x02CF08, i_02cf08),
    (0x02CF0C, i_02cf0c),
    (0x02CF10, i_02cf10),
    (0x02CF14, i_02cf14),
    (0x02CF18, i_02cf18),
    (0x02CF1A, i_02cf1a),
    (0x02CF1C, i_02cf1c),
    (0x02CF1E, i_02cf1e),
    (0x02CF22, i_02cf22),
    (0x02CF26, i_02cf26),
    (0x02CF2A, i_02cf2a),
    (0x02CF2E, i_02cf2e),
    (0x02CF32, i_02cf32),
    (0x02CF36, i_02cf36),
    (0x02CF3C, i_02cf3c),
    (0x02CF3E, i_02cf3e),
    (0x02CF42, i_02cf42),
    (0x02CF48, i_02cf48),
    (0x02CF4C, i_02cf4c),
    (0x02CF52, i_02cf52),
    (0x02CF56, i_02cf56),
    (0x02CF5C, i_02cf5c),
    (0x02CF5E, i_02cf5e),
    (0x02CF62, i_02cf62),
    (0x02CF66, i_02cf66),
    (0x02CF6C, i_02cf6c),
    (0x02CF70, i_02cf70),
    (0x02CF74, i_02cf74),
    (0x02CF78, i_02cf78),
    (0x02CF7A, i_02cf7a),
    (0x02CF82, i_02cf82),
    (0x02CF86, i_02cf86),
    (0x02CF8A, i_02cf8a),
    (0x02CF8E, i_02cf8e),
    (0x02CF94, i_02cf94),
    (0x02CF9A, i_02cf9a),
    (0x02CFA2, i_02cfa2),
    (0x02CFA6, i_02cfa6),
    (0x02CFAA, i_02cfaa),
    (0x02CFAE, i_02cfae),
    (0x02CFB4, i_02cfb4),
    (0x02CFBA, i_02cfba),
    (0x02CFBE, i_02cfbe),
    (0x02CFC2, i_02cfc2),
    (0x02CFC6, i_02cfc6),
    (0x02CFC8, i_02cfc8),
    (0x02CFCC, i_02cfcc),
    (0x02CFD0, i_02cfd0),
    (0x02CFD4, i_02cfd4),
    (0x02CFD6, i_02cfd6),
    (0x02CFDA, i_02cfda),
    (0x02CFDE, i_02cfde),
    (0x02CFE2, i_02cfe2),
    (0x02CFE4, i_02cfe4),
    (0x02CFE8, i_02cfe8),
    (0x02CFEC, i_02cfec),
    (0x02CFF2, i_02cff2),
    (0x02CFF8, i_02cff8),
    (0x02CFFA, i_02cffa),
    (0x02D000, i_02d000),
    (0x02D006, i_02d006),
    (0x02D00A, i_02d00a),
    (0x02D00E, i_02d00e),
    (0x02D012, i_02d012),
    (0x02D016, i_02d016),
    (0x02D01A, i_02d01a),
    (0x02D01E, i_02d01e),
    (0x02D022, i_02d022),
    (0x02D024, i_02d024),
    (0x02D02A, i_02d02a),
    (0x02D02E, i_02d02e),
    (0x02D034, i_02d034),
    (0x02D03A, i_02d03a),
    (0x02D03E, i_02d03e),
    (0x02D042, i_02d042),
    (0x02D046, i_02d046),
    (0x02D04A, i_02d04a),
    (0x02D04E, i_02d04e),
    (0x02D052, i_02d052),
    (0x02D056, i_02d056),
    (0x02D05A, i_02d05a),
    (0x02D05C, i_02d05c),
    (0x02D05E, i_02d05e),
    (0x02D060, i_02d060),
    (0x02D064, i_02d064),
    (0x02D068, i_02d068),
    (0x02D06C, i_02d06c),
    (0x02D070, i_02d070),
    (0x02D074, i_02d074),
    (0x02D078, i_02d078),
    (0x02D07E, i_02d07e),
    (0x02D080, i_02d080),
    (0x02D084, i_02d084),
    (0x02D08A, i_02d08a),
    (0x02D08E, i_02d08e),
    (0x02D094, i_02d094),
    (0x02D098, i_02d098),
    (0x02D09E, i_02d09e),
    (0x02D0A0, i_02d0a0),
    (0x02D0A4, i_02d0a4),
    (0x02D0A8, i_02d0a8),
    (0x02D0AE, i_02d0ae),
    (0x02D0B2, i_02d0b2),
    (0x02D0B4, i_02d0b4),
    (0x02D0B8, i_02d0b8),
    (0x02D0BC, i_02d0bc),
    (0x02D0BE, i_02d0be),
    (0x02D0C2, i_02d0c2),
    (0x02D0C4, i_02d0c4),
    (0x02D0CA, i_02d0ca),
    (0x02D0CC, i_02d0cc),
    (0x02D0D0, i_02d0d0),
    (0x02D0D6, i_02d0d6),
    (0x02D0DA, i_02d0da),
    (0x02D0DE, i_02d0de),
    (0x02D0E4, i_02d0e4),
    (0x02D0E8, i_02d0e8),
    (0x02D0EC, i_02d0ec),
    (0x02D0F0, i_02d0f0),
    (0x02D0F6, i_02d0f6),
    (0x02D0FA, i_02d0fa),
    (0x02D100, i_02d100),
    (0x02D104, i_02d104),
    (0x02D108, i_02d108),
    (0x02D10C, i_02d10c),
    (0x02D110, i_02d110),
    (0x02D112, i_02d112),
    (0x02D116, i_02d116),
    (0x02D11A, i_02d11a),
    (0x02D11E, i_02d11e),
    (0x02D120, i_02d120),
    (0x02D126, i_02d126),
    (0x02D12A, i_02d12a),
    (0x02D12E, i_02d12e),
    (0x02D132, i_02d132),
    (0x02D136, i_02d136),
    (0x02D13C, i_02d13c),
    (0x02D142, i_02d142),
    (0x02D14A, i_02d14a),
    (0x02D14E, i_02d14e),
    (0x02D152, i_02d152),
    (0x02D156, i_02d156),
    (0x02D15C, i_02d15c),
    (0x02D162, i_02d162),
    (0x02D16A, i_02d16a),
    (0x02D16E, i_02d16e),
    (0x02D172, i_02d172),
    (0x02D176, i_02d176),
    (0x02D17C, i_02d17c),
    (0x02D182, i_02d182),
    (0x02D186, i_02d186),
    (0x02D18C, i_02d18c),
    (0x02D190, i_02d190),
    (0x02D194, i_02d194),
    (0x02D198, i_02d198),
    (0x02D19C, i_02d19c),
    (0x02D1A0, i_02d1a0),
    (0x02D1A4, i_02d1a4),
    (0x02D1A8, i_02d1a8),
    (0x02D1AC, i_02d1ac),
    (0x02D1B0, i_02d1b0),
    (0x02D1B4, i_02d1b4),
    (0x02D1B8, i_02d1b8),
    (0x02D1BA, i_02d1ba),
    (0x02D1BC, i_02d1bc),
    (0x02D1BE, i_02d1be),
    (0x02D1C2, i_02d1c2),
    (0x02D1C6, i_02d1c6),
    (0x02D1CA, i_02d1ca),
    (0x02D1CE, i_02d1ce),
    (0x02D1D2, i_02d1d2),
    (0x02D1D6, i_02d1d6),
    (0x02D1DC, i_02d1dc),
    (0x02D1DE, i_02d1de),
    (0x02D1E2, i_02d1e2),
    (0x02D1E8, i_02d1e8),
    (0x02D1EC, i_02d1ec),
    (0x02D1F2, i_02d1f2),
    (0x02D1F6, i_02d1f6),
    (0x02D1FC, i_02d1fc),
    (0x02D1FE, i_02d1fe),
    (0x02D202, i_02d202),
    (0x02D206, i_02d206),
    (0x02D20C, i_02d20c),
    (0x02D210, i_02d210),
    (0x02D212, i_02d212),
    (0x02D216, i_02d216),
    (0x02D21A, i_02d21a),
    (0x02D21C, i_02d21c),
    (0x02D222, i_02d222),
    (0x02D226, i_02d226),
    (0x02D22A, i_02d22a),
    (0x02D22E, i_02d22e),
    (0x02D234, i_02d234),
    (0x02D23A, i_02d23a),
    (0x02D23E, i_02d23e),
    (0x02D242, i_02d242),
    (0x02D246, i_02d246),
    (0x02D24C, i_02d24c),
    (0x02D252, i_02d252),
    (0x02D258, i_02d258),
    (0x02D25E, i_02d25e),
    (0x02D264, i_02d264),
    (0x02D268, i_02d268),
    (0x02D26C, i_02d26c),
    (0x02D26E, i_02d26e),
    (0x02D274, i_02d274),
    (0x02D278, i_02d278),
    (0x02D27C, i_02d27c),
    (0x02D280, i_02d280),
    (0x02D282, i_02d282),
    (0x02D286, i_02d286),
    (0x02D28C, i_02d28c),
    (0x02D292, i_02d292),
    (0x02D298, i_02d298),
    (0x02D29E, i_02d29e),
    (0x02D2A2, i_02d2a2),
    (0x02D2A8, i_02d2a8),
    (0x02D2AA, i_02d2aa),
    (0x02D2AE, i_02d2ae),
    (0x02D2B4, i_02d2b4),
    (0x02D2B8, i_02d2b8),
    (0x02D2BE, i_02d2be),
    (0x02D2C4, i_02d2c4),
    (0x02D2C8, i_02d2c8),
    (0x02D2CA, i_02d2ca),
    (0x02D2CE, i_02d2ce),
    (0x02D2D4, i_02d2d4),
    (0x02D2D8, i_02d2d8),
    (0x02D2DE, i_02d2de),
    (0x02D2E2, i_02d2e2),
    (0x02D2E8, i_02d2e8),
    (0x02D2EC, i_02d2ec),
    (0x02D2F2, i_02d2f2),
    (0x02D2F8, i_02d2f8),
    (0x02D2FE, i_02d2fe),
    (0x02D302, i_02d302),
    (0x02D308, i_02d308),
    (0x02D30E, i_02d30e),
    (0x02D314, i_02d314),
    (0x02D31A, i_02d31a),
    (0x02D320, i_02d320),
    (0x02D326, i_02d326),
    (0x02D32C, i_02d32c),
    (0x02D330, i_02d330),
    (0x02D336, i_02d336),
    (0x02D33C, i_02d33c),
    (0x02D342, i_02d342),
    (0x02D348, i_02d348),
    (0x02D34E, i_02d34e),
    (0x02D352, i_02d352),
    (0x02D356, i_02d356),
    (0x02D35C, i_02d35c),
    (0x02D362, i_02d362),
    (0x02D368, i_02d368),
    (0x02D36C, i_02d36c),
    (0x02D370, i_02d370),
    (0x02D372, i_02d372),
    (0x02D376, i_02d376),
    (0x02D37A, i_02d37a),
    (0x02D37C, i_02d37c),
    (0x02D380, i_02d380),
    (0x02D386, i_02d386),
    (0x02D38A, i_02d38a),
    (0x02D390, i_02d390),
    (0x02D394, i_02d394),
    (0x02D39A, i_02d39a),
    (0x02D39E, i_02d39e),
    (0x02D3A4, i_02d3a4),
    (0x02D3A8, i_02d3a8),
    (0x02D3AE, i_02d3ae),
    (0x02D3B2, i_02d3b2),
    (0x02D3B8, i_02d3b8),
    (0x02D3BE, i_02d3be),
    (0x02D3C4, i_02d3c4),
    (0x02D3C8, i_02d3c8),
    (0x02D3CE, i_02d3ce),
    (0x02D3D4, i_02d3d4),
    (0x02D3DA, i_02d3da),
    (0x02D3E0, i_02d3e0),
    (0x02D3E6, i_02d3e6),
    (0x02D3EC, i_02d3ec),
    (0x02D3F2, i_02d3f2),
    (0x02D3F6, i_02d3f6),
    (0x02D3FC, i_02d3fc),
    (0x02D402, i_02d402),
    (0x02D408, i_02d408),
    (0x02D40E, i_02d40e),
    (0x02D414, i_02d414),
    (0x02D418, i_02d418),
    (0x02D41C, i_02d41c),
    (0x02D422, i_02d422),
    (0x02D426, i_02d426),
    (0x02D42A, i_02d42a),
    (0x02D42C, i_02d42c),
    (0x02D42E, i_02d42e),
    (0x02D430, i_02d430),
    (0x02D434, i_02d434),
    (0x02D438, i_02d438),
    (0x02D43C, i_02d43c),
    (0x02D440, i_02d440),
    (0x02D444, i_02d444),
    (0x02D448, i_02d448),
    (0x02D44E, i_02d44e),
    (0x02D450, i_02d450),
    (0x02D454, i_02d454),
    (0x02D45A, i_02d45a),
    (0x02D45E, i_02d45e),
    (0x02D464, i_02d464),
    (0x02D468, i_02d468),
    (0x02D46E, i_02d46e),
    (0x02D470, i_02d470),
    (0x02D474, i_02d474),
    (0x02D478, i_02d478),
    (0x02D47E, i_02d47e),
    (0x02D484, i_02d484),
    (0x02D48A, i_02d48a),
    (0x02D48E, i_02d48e),
    (0x02D492, i_02d492),
    (0x02D494, i_02d494),
    (0x02D498, i_02d498),
    (0x02D49C, i_02d49c),
    (0x02D49E, i_02d49e),
    (0x02D4A4, i_02d4a4),
    (0x02D4A6, i_02d4a6),
    (0x02D4AA, i_02d4aa),
    (0x02D4B0, i_02d4b0),
    (0x02D4B4, i_02d4b4),
    (0x02D4B8, i_02d4b8),
    (0x02D4BE, i_02d4be),
    (0x02D4C2, i_02d4c2),
    (0x02D4C6, i_02d4c6),
    (0x02D4CA, i_02d4ca),
    (0x02D4CE, i_02d4ce),
    (0x02D4D4, i_02d4d4),
    (0x02D4D8, i_02d4d8),
    (0x02D4DE, i_02d4de),
    (0x02D4E2, i_02d4e2),
    (0x02D4E6, i_02d4e6),
    (0x02D4EA, i_02d4ea),
    (0x02D4EE, i_02d4ee),
    (0x02D4F0, i_02d4f0),
    (0x02D4F6, i_02d4f6),
    (0x02D4FA, i_02d4fa),
    (0x02D4FE, i_02d4fe),
    (0x02D502, i_02d502),
    (0x02D506, i_02d506),
    (0x02D50C, i_02d50c),
    (0x02D512, i_02d512),
    (0x02D51A, i_02d51a),
    (0x02D51E, i_02d51e),
    (0x02D522, i_02d522),
    (0x02D526, i_02d526),
    (0x02D52C, i_02d52c),
    (0x02D532, i_02d532),
    (0x02D53A, i_02d53a),
    (0x02D53E, i_02d53e),
    (0x02D542, i_02d542),
    (0x02D546, i_02d546),
    (0x02D54C, i_02d54c),
    (0x02D552, i_02d552),
    (0x02D556, i_02d556),
    (0x02D55C, i_02d55c),
    (0x02D560, i_02d560),
    (0x02D564, i_02d564),
    (0x02D568, i_02d568),
    (0x02D56C, i_02d56c),
    (0x02D570, i_02d570),
    (0x02D574, i_02d574),
    (0x02D578, i_02d578),
    (0x02D57C, i_02d57c),
    (0x02D580, i_02d580),
    (0x02D584, i_02d584),
    (0x02D588, i_02d588),
    (0x02D58A, i_02d58a),
    (0x02D58C, i_02d58c),
    (0x02D58E, i_02d58e),
    (0x02D592, i_02d592),
    (0x02D596, i_02d596),
    (0x02D59A, i_02d59a),
    (0x02D59E, i_02d59e),
    (0x02D5A2, i_02d5a2),
    (0x02D5A6, i_02d5a6),
    (0x02D5AC, i_02d5ac),
    (0x02D5AE, i_02d5ae),
    (0x02D5B2, i_02d5b2),
    (0x02D5B8, i_02d5b8),
    (0x02D5BC, i_02d5bc),
    (0x02D5C2, i_02d5c2),
    (0x02D5C6, i_02d5c6),
    (0x02D5CC, i_02d5cc),
    (0x02D5CE, i_02d5ce),
    (0x02D5D2, i_02d5d2),
    (0x02D5D6, i_02d5d6),
    (0x02D5DC, i_02d5dc),
    (0x02D5E0, i_02d5e0),
    (0x02D5E2, i_02d5e2),
    (0x02D5E6, i_02d5e6),
    (0x02D5EA, i_02d5ea),
    (0x02D5EC, i_02d5ec),
    (0x02D5F2, i_02d5f2),
    (0x02D5F6, i_02d5f6),
    (0x02D5FA, i_02d5fa),
    (0x02D5FE, i_02d5fe),
    (0x02D604, i_02d604),
    (0x02D60A, i_02d60a),
    (0x02D60E, i_02d60e),
    (0x02D612, i_02d612),
    (0x02D616, i_02d616),
    (0x02D61C, i_02d61c),
    (0x02D622, i_02d622),
    (0x02D628, i_02d628),
    (0x02D62E, i_02d62e),
    (0x02D634, i_02d634),
    (0x02D638, i_02d638),
    (0x02D63C, i_02d63c),
    (0x02D63E, i_02d63e),
    (0x02D644, i_02d644),
    (0x02D648, i_02d648),
    (0x02D64E, i_02d64e),
    (0x02D650, i_02d650),
    (0x02D654, i_02d654),
    (0x02D65A, i_02d65a),
    (0x02D65E, i_02d65e),
    (0x02D664, i_02d664),
    (0x02D668, i_02d668),
    (0x02D66E, i_02d66e),
    (0x02D672, i_02d672),
    (0x02D674, i_02d674),
    (0x02D678, i_02d678),
    (0x02D6BA, i_02d6ba),
    (0x02D6C0, i_02d6c0),
    (0x02D6C8, i_02d6c8),
    (0x02D6CC, i_02d6cc),
    (0x02D6CE, i_02d6ce),
    (0x02D6D4, i_02d6d4),
    (0x02D6DA, i_02d6da),
    (0x02D6DE, i_02d6de),
    (0x02D6E2, i_02d6e2),
    (0x02D6E8, i_02d6e8),
    (0x02D6EC, i_02d6ec),
    (0x02D6F0, i_02d6f0),
    (0x02D6F2, i_02d6f2),
    (0x02D6F8, i_02d6f8),
    (0x02D6FC, i_02d6fc),
    (0x02D702, i_02d702),
    (0x02D706, i_02d706),
    (0x02D70A, i_02d70a),
    (0x02D70E, i_02d70e),
    (0x02D712, i_02d712),
    (0x02D716, i_02d716),
    (0x02D718, i_02d718),
    (0x02D71C, i_02d71c),
    (0x02D720, i_02d720),
    (0x02D724, i_02d724),
    (0x02D726, i_02d726),
    (0x02D72A, i_02d72a),
    (0x02D72E, i_02d72e),
    (0x02D732, i_02d732),
    (0x02D734, i_02d734),
    (0x02D736, i_02d736),
    (0x02D73C, i_02d73c),
    (0x02D740, i_02d740),
    (0x02D746, i_02d746),
    (0x02D748, i_02d748),
    (0x02D74C, i_02d74c),
    (0x02D752, i_02d752),
    (0x02D756, i_02d756),
    (0x02D75C, i_02d75c),
    (0x02D760, i_02d760),
    (0x02D766, i_02d766),
    (0x02D76A, i_02d76a),
    (0x02D76C, i_02d76c),
    (0x02D770, i_02d770),
    (0x02D776, i_02d776),
    (0x02D77C, i_02d77c),
    (0x02D782, i_02d782),
    (0x02D786, i_02d786),
    (0x02D78A, i_02d78a),
    (0x02D78C, i_02d78c),
    (0x02D792, i_02d792),
    (0x02D798, i_02d798),
    (0x02D79C, i_02d79c),
    (0x02D7A2, i_02d7a2),
    (0x02D7A8, i_02d7a8),
    (0x02D7AC, i_02d7ac),
    (0x02D7B0, i_02d7b0),
];
