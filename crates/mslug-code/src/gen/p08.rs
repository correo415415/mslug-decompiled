// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


// ===== Anim_ScriptStep_08bc74  ($08BC74, 108 B, mission/cutscene_anim_08ba0c.s)
/// $08BC74: cmpi.w #$ffff, $70(a6)
pub fn i_08bc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC7A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BC7A: beq.w $8bce0
pub fn i_08bc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BCE0; return 10; }
    8
}
/// $08BC7E: subq.w #$1, $70(a6)
pub fn i_08bc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC82;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08BC82: cmpi.w #$0, $70(a6)
pub fn i_08bc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC88;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08BC88: bgt.w $8bcd2
pub fn i_08bc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC8C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8BCD2; return 10; }
    8
}
/// $08BC8C: movea.l $7c(a6), a1
pub fn i_08bc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC90;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08BC90: adda.l $78(a6), a1
pub fn i_08bc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC94;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $08BC94: move.w (a1), $70(a6)
pub fn i_08bc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC98;
    cpu.ir = 0x3D51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad3, v2 as u16);
    16
}
/// $08BC98: cmpi.w #$ffff, $70(a6)
pub fn i_08bc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC9E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BC9E: beq.w $8bcbc
pub fn i_08bc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCA2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BCBC; return 10; }
    8
}
/// $08BCA2: move.w $2(a1), $76(a6)
pub fn i_08bca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCA8;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08BCA8: move.b $4(a1), $92(a6)
pub fn i_08bca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCAE;
    cpu.ir = 0x1D69;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08BCAE: move.b $5(a1), $93(a6)
pub fn i_08bcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCB4;
    cpu.ir = 0x1D69;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x93);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08BCB4: addq.l #$8, $78(a6)
pub fn i_08bcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCB8;
    cpu.ir = 0x50AE;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 8, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $08BCB8: bra.w $8bcce
pub fn i_08bcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BCCE;
    10
}
/// $08BCBC: move.w #$0, $76(a6)
pub fn i_08bcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCC2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BCC2: move.b #$0, $92(a6)
pub fn i_08bcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCC8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BCC8: move.b #$0, $93(a6)
pub fn i_08bcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCCE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x93);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BCCE: bra.w $8bcd8
pub fn i_08bcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCD2;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BCD8;
    10
}
/// $08BCD2: move.b #$0, $93(a6)
pub fn i_08bcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCD8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x93);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BCD8: andi.b #$ee, ccr
pub fn i_08bcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCDC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08BCDC: bra.w $8bce4
pub fn i_08bcdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BCE4;
    10
}
// ===== SetXN_08bce0  ($08BCE0, 6 B, thunks/ccr_helpers.c)
/// $08BCE0: ori.b #$11, ccr
pub fn i_08bce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCE4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08BCE4: rts
pub fn i_08bce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_ScriptStepFix_A_08bce6  ($08BCE6, 316 B, mission/cutscene_anim_08ba0c.s)
/// $08BCE6: cmpi.w #$ffff, $70(a6)
pub fn i_08bce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCEC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BCEC: beq.w $8be22
pub fn i_08bcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCF0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BE22; return 10; }
    8
}
/// $08BCF0: subq.w #$1, $70(a6)
pub fn i_08bcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCF4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08BCF4: cmpi.w #$0, $70(a6)
pub fn i_08bcf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCFA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08BCFA: bgt.w $8be1a
pub fn i_08bcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCFE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8BE1A; return 10; }
    8
}
/// $08BCFE: movea.l $7c(a6), a1
pub fn i_08bcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD02;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08BD02: adda.l $82(a6), a1
pub fn i_08bd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD06;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $08BD06: move.w (a1), $70(a6)
pub fn i_08bd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD0A;
    cpu.ir = 0x3D51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad3, v2 as u16);
    16
}
/// $08BD0A: cmpi.w #$ffff, $70(a6)
pub fn i_08bd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD10;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BD10: beq.w $8be1a
pub fn i_08bd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD14;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BE1A; return 10; }
    8
}
/// $08BD14: move.w $2(a1), $80(a6)
pub fn i_08bd14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD1A;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08BD1A: cmpi.l #$0, $4(a1)
pub fn i_08bd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD22;
    cpu.ir = 0xCA9;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x0, Size::Long, false);
    20
}
/// $08BD22: beq.w $8bd30
pub fn i_08bd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD26;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BD30; return 10; }
    8
}
/// $08BD26: movea.l $4(a1), a0
pub fn i_08bd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD2A;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08BD2A: jsr $28cd4.l
pub fn i_08bd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BD30);
    cpu.pc = 0x28CD4;
    16
}
/// $08BD30: cmpi.w #$1, $80(a6)
pub fn i_08bd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD36;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $08BD36: bne.w $8bd54
pub fn i_08bd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BD54; return 10; }
    8
}
/// $08BD3A: movea.w #$7084, a1
pub fn i_08bd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD3E;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7084;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $08BD3E: move.w #$2320, d0
pub fn i_08bd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD42;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BD42: move.w #$20, d1
pub fn i_08bd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD46;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BD46: move.w #$8, d2
pub fn i_08bd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD4A;
    cpu.ir = 0x343C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BD4A: jsr $5da9c.l
pub fn i_08bd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BD50);
    cpu.pc = 0x5DA9C;
    16
}
/// $08BD50: bra.w $8be12
pub fn i_08bd50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD54;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BD54: cmpi.w #$d, $80(a6)
pub fn i_08bd54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD5A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD, Size::Word, false);
    16
}
/// $08BD5A: bne.w $8bd6a
pub fn i_08bd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD5E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BD6A; return 10; }
    8
}
/// $08BD5E: move.l #$70a9, $86(a6)
pub fn i_08bd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD66;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x70A9;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BD66: bra.w $8be12
pub fn i_08bd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD6A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BD6A: cmpi.w #$2, $80(a6)
pub fn i_08bd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD70;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $08BD70: bne.w $8bd82
pub fn i_08bd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BD82; return 10; }
    8
}
/// $08BD74: lea.l $8c730(pc), a1
pub fn i_08bd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C730;
    16
}
/// $08BD78: jsr $4ae.l
pub fn i_08bd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BD7E);
    cpu.pc = 0x4AE;
    16
}
/// $08BD7E: bra.w $8be12
pub fn i_08bd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD82;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BD82: cmpi.w #$3, $80(a6)
pub fn i_08bd82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD88;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $08BD88: bne.w $8bdae
pub fn i_08bd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD8C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BDAE; return 10; }
    8
}
/// $08BD8C: lea.l $2ae50.l, a1
pub fn i_08bd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD92;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2AE50;
    20
}
/// $08BD92: jsr $4ae.l
pub fn i_08bd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BD98);
    cpu.pc = 0x4AE;
    16
}
/// $08BD98: move.w #$a0, $22(a0)
pub fn i_08bd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD9E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BD9E: move.w #$1ff, $24(a0)
pub fn i_08bd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDA4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BDA4: move.b #$3, $98(a0)
pub fn i_08bda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDAA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BDAA: bra.w $8be12
pub fn i_08bdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BDAE: cmpi.w #$0, $80(a6)
pub fn i_08bdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDB4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08BDB4: bne.w $8bdc0
pub fn i_08bdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDB8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BDC0; return 10; }
    8
}
/// $08BDB8: bra.w $8be12
pub fn i_08bdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BDC0: movea.l $86(a6), a1
pub fn i_08bdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDC4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08BDC4: addi.l #$40, $86(a6)
pub fn i_08bdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDCC;
    cpu.ir = 0x6AE;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Long, true);
    bus.write32(ad1, r3);
    20
}
/// $08BDCC: move.w $80(a6), d0
pub fn i_08bdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDD0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08BDD0: andi.w #$ff00, d0
pub fn i_08bdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDD4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08BDD4: move.w #$4, d1
pub fn i_08bdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDD8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BDD8: cmpi.w #$500, d0
pub fn i_08bdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDDC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x500, Size::Word, false);
    8
}
/// $08BDDC: bne.w $8bde4
pub fn i_08bddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BDE4; return 10; }
    8
}
/// $08BDE0: move.w #$9, d1
pub fn i_08bde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDE4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BDE4: cmpi.w #$600, d0
pub fn i_08bde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDE8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x600, Size::Word, false);
    8
}
/// $08BDE8: bne.w $8bdf0
pub fn i_08bde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BDF0; return 10; }
    8
}
/// $08BDEC: move.w #$9, d1
pub fn i_08bdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDF0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BDF0: cmpi.w #$700, d0
pub fn i_08bdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDF4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x700, Size::Word, false);
    8
}
/// $08BDF4: bne.w $8bdfc
pub fn i_08bdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDF8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BDFC; return 10; }
    8
}
/// $08BDF8: move.w #$9, d1
pub fn i_08bdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDFC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BDFC: cmpi.w #$d00, d0
pub fn i_08bdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE00;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xD00, Size::Word, false);
    8
}
/// $08BE00: bcs.w $8be08
pub fn i_08be00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE04;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8BE08; return 10; }
    8
}
/// $08BE04: move.w #$9, d1
pub fn i_08be04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE08;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BE08: move.w $80(a6), d0
pub fn i_08be08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE0C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08BE0C: jsr $47872.l
pub fn i_08be0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BE12);
    cpu.pc = 0x47872;
    16
}
/// $08BE12: addq.l #$8, $82(a6)
pub fn i_08be12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE16;
    cpu.ir = 0x50AE;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 8, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $08BE16: bra.w $8bcfe
pub fn i_08be16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BCFE;
    10
}
/// $08BE1A: andi.b #$ee, ccr
pub fn i_08be1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE1E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08BE1E: bra.w $8be26
pub fn i_08be1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE22;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE26;
    10
}
// ===== SetXN_08be22  ($08BE22, 6 B, thunks/ccr_helpers.c)
/// $08BE22: ori.b #$11, ccr
pub fn i_08be22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE26;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08BE26: rts
pub fn i_08be26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_ScriptStepFix_B_08be28  ($08BE28, 360 B, mission/cutscene_anim_08ba0c.s)
/// $08BE28: cmpi.w #$ffff, $70(a6)
pub fn i_08be28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE2E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BE2E: beq.w $8bf90
pub fn i_08be2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BF90; return 10; }
    8
}
/// $08BE32: subq.w #$1, $70(a6)
pub fn i_08be32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE36;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08BE36: cmpi.w #$0, $70(a6)
pub fn i_08be36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE3C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08BE3C: bgt.w $8bf88
pub fn i_08be3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE40;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8BF88; return 10; }
    8
}
/// $08BE40: movea.l $7c(a6), a1
pub fn i_08be40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE44;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08BE44: adda.l $82(a6), a1
pub fn i_08be44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE48;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $08BE48: move.w (a1), $70(a6)
pub fn i_08be48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE4C;
    cpu.ir = 0x3D51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad3, v2 as u16);
    16
}
/// $08BE4C: cmpi.w #$ffff, $70(a6)
pub fn i_08be4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE52;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BE52: beq.w $8bf88
pub fn i_08be52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE56;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BF88; return 10; }
    8
}
/// $08BE56: move.w $2(a1), $80(a6)
pub fn i_08be56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE5C;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08BE5C: cmpi.l #$0, $4(a1)
pub fn i_08be5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE64;
    cpu.ir = 0xCA9;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x0, Size::Long, false);
    20
}
/// $08BE64: beq.w $8be72
pub fn i_08be64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE68;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BE72; return 10; }
    8
}
/// $08BE68: movea.l $4(a1), a0
pub fn i_08be68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE6C;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08BE6C: jsr $28cd4.l
pub fn i_08be6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BE72);
    cpu.pc = 0x28CD4;
    16
}
/// $08BE72: cmpi.w #$1, $80(a6)
pub fn i_08be72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE78;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $08BE78: bne.w $8be96
pub fn i_08be78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE7C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BE96; return 10; }
    8
}
/// $08BE7C: movea.w #$7084, a1
pub fn i_08be7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE80;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7084;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $08BE80: move.w #$2320, d0
pub fn i_08be80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE84;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BE84: move.w #$20, d1
pub fn i_08be84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE88;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BE88: move.w #$8, d2
pub fn i_08be88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE8C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BE8C: jsr $5da9c.l
pub fn i_08be8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BE92);
    cpu.pc = 0x5DA9C;
    16
}
/// $08BE92: bra.w $8be12
pub fn i_08be92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE96;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BE96: cmpi.w #$d, $80(a6)
pub fn i_08be96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE9C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD, Size::Word, false);
    16
}
/// $08BE9C: bne.w $8bebe
pub fn i_08be9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEA0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BEBE; return 10; }
    8
}
/// $08BEA0: move.l $86(a6), d0
pub fn i_08bea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEA4;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BEA4: andi.l #$f00f, d0
pub fn i_08bea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEAA;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF00F) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $08BEAA: addi.l #$a0, d0
pub fn i_08beaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEB0;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0xA0, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $08BEB0: addi.l #$2, d0
pub fn i_08beb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEB6;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $08BEB6: move.l d0, $86(a6)
pub fn i_08beb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEBA;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BEBA: bra.w $8bf80
pub fn i_08beba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEBE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BF80;
    10
}
/// $08BEBE: cmpi.w #$2, $80(a6)
pub fn i_08bebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEC4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $08BEC4: bne.w $8bed6
pub fn i_08bec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEC8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BED6; return 10; }
    8
}
/// $08BEC8: lea.l $8c730(pc), a1
pub fn i_08bec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BECC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C730;
    16
}
/// $08BECC: jsr $4ae.l
pub fn i_08becc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BED2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BED2);
    cpu.pc = 0x4AE;
    16
}
/// $08BED2: bra.w $8be12
pub fn i_08bed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BED6;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BED6: cmpi.w #$3, $80(a6)
pub fn i_08bed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEDC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $08BEDC: bne.w $8bf02
pub fn i_08bedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BF02; return 10; }
    8
}
/// $08BEE0: lea.l $2ae50.l, a1
pub fn i_08bee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEE6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2AE50;
    20
}
/// $08BEE6: jsr $4ae.l
pub fn i_08bee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BEEC);
    cpu.pc = 0x4AE;
    16
}
/// $08BEEC: move.w #$a0, $22(a0)
pub fn i_08beec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEF2;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BEF2: move.w #$1ff, $24(a0)
pub fn i_08bef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEF8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BEF8: move.b #$3, $98(a0)
pub fn i_08bef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEFE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BEFE: bra.w $8be12
pub fn i_08befe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF02;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BF02: cmpi.w #$0, $80(a6)
pub fn i_08bf02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF08;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08BF08: bne.w $8bf14
pub fn i_08bf08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF0C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BF14; return 10; }
    8
}
/// $08BF0C: bra.w $8bf80
pub fn i_08bf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF10;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BF80;
    10
}
/// $08BF14: move.l $86(a6), d0
pub fn i_08bf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF18;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BF18: andi.l #$ffff, d0
pub fn i_08bf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF1E;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $08BF1E: addi.l #$20, $86(a6)
pub fn i_08bf1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF26;
    cpu.ir = 0x6AE;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Long, true);
    bus.write32(ad1, r3);
    20
}
/// $08BF26: move.w $80(a6), d1
pub fn i_08bf26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF2A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08BF2A: ori.w #$2300, d1
pub fn i_08bf2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF2E;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08BF2E: movem.w d0-d1, $3c0000.l
pub fn i_08bf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF36;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08BF36: cmpi.w #$e6, $80(a6)
pub fn i_08bf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF3C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xE6, Size::Word, false);
    16
}
/// $08BF3C: bne.w $8bf56
pub fn i_08bf3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BF56; return 10; }
    8
}
/// $08BF40: addi.l #$1, d0
pub fn i_08bf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF46;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $08BF46: addi.w #$10, d1
pub fn i_08bf46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF4A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08BF4A: movem.w d0-d1, $3c0000.l
pub fn i_08bf4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF52;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08BF52: bra.w $8bf80
pub fn i_08bf52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF56;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BF80;
    10
}
/// $08BF56: cmpi.w #$e7, $80(a6)
pub fn i_08bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF5C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xE7, Size::Word, false);
    16
}
/// $08BF5C: bcs.w $8bf80
pub fn i_08bf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF60;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8BF80; return 10; }
    8
}
/// $08BF60: cmpi.w #$ef, $80(a6)
pub fn i_08bf60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF66;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xEF, Size::Word, false);
    16
}
/// $08BF66: bhi.w $8bf80
pub fn i_08bf66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF6A;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8BF80; return 10; }
    8
}
/// $08BF6A: subi.l #$1, d0
pub fn i_08bf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF70;
    cpu.ir = 0x480;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $08BF70: subi.w #$10, d1
pub fn i_08bf70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF74;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08BF74: movem.w d0-d1, $3c0000.l
pub fn i_08bf74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF7C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08BF7C: bra.w $8bf80
pub fn i_08bf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF80;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BF80;
    10
}
/// $08BF80: addq.l #$8, $82(a6)
pub fn i_08bf80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF84;
    cpu.ir = 0x50AE;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 8, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $08BF84: bra.w $8be40
pub fn i_08bf84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF88;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE40;
    10
}
/// $08BF88: andi.b #$ee, ccr
pub fn i_08bf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF8C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08BF8C: bra.w $8bf94
pub fn i_08bf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF90;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BF94;
    10
}
// ===== SetXN_08bf90  ($08BF90, 6 B, thunks/ccr_helpers.c)
/// $08BF90: ori.b #$11, ccr
pub fn i_08bf90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF94;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08BF94: rts
pub fn i_08bf94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_State_F1_08C008  ($08C008, 342 B, entity/anim_state_machine_08c008.s)
/// $08C008: jsr $22c8.l
pub fn i_08c008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C00E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C00E);
    cpu.pc = 0x22C8;
    16
}
/// $08C00E: move.w #$2b, d0
pub fn i_08c00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C012;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C012: jsr $2352.l
pub fn i_08c012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C018;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C018);
    cpu.pc = 0x2352;
    16
}
/// $08C018: move.w #$12e, d1
pub fn i_08c018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C01C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C01C: jsr $236e.l
pub fn i_08c01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C022;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C022);
    cpu.pc = 0x236E;
    16
}
/// $08C022: lea.l $2f2dbe.l, a0
pub fn i_08c022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C028;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2DBE;
    20
}
/// $08C028: jsr $28cd4.l
pub fn i_08c028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C02E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C02E);
    cpu.pc = 0x28CD4;
    16
}
/// $08C02E: move.w #$a0, $22(a6)
pub fn i_08c02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C034;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C034: move.w #$1c3, $24(a6)
pub fn i_08c034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C03A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1C3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C03A: clr.w $26(a6)
pub fn i_08c03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C03E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C03E: move.w #$4000, $38(a6)
pub fn i_08c03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C044;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C044: bset.b #$6, $12(a6)
pub fn i_08c044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C04A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C04A: move.b #$9, d0
pub fn i_08c04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C04E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08C04E: jsr $43568.l
pub fn i_08c04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C054;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C054);
    cpu.pc = 0x43568;
    16
}
/// $08C054: move.w #$0, $70(a6)
pub fn i_08c054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C05A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C05A: move.w #$0, $76(a6)
pub fn i_08c05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C060;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C060: move.l #$0, $78(a6)
pub fn i_08c060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C068;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C068: lea.l $2ef8aa.l, a1
pub fn i_08c068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C06E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF8AA;
    20
}
/// $08C06E: move.l a1, $7c(a6)
pub fn i_08c06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C072;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C072: move.l #$0, $90(a6)
pub fn i_08c072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C07A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C07A: movea.l a6, a1
pub fn i_08c07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C07C;
    cpu.ir = 0x224E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $08C07C: adda.l #$90, a1
pub fn i_08c07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C082;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x90;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $08C082: move.l a1, $72(a6)
pub fn i_08c082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C086;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C086: lea.l $2ae50.l, a1
pub fn i_08c086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C08C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2AE50;
    20
}
/// $08C08C: jsr $4ae.l
pub fn i_08c08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C092;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C092);
    cpu.pc = 0x4AE;
    16
}
/// $08C092: move.w #$a0, $22(a0)
pub fn i_08c092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C098;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C098: move.w #$1ff, $24(a0)
pub fn i_08c098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C09E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C09E: move.b #$3, $98(a0)
pub fn i_08c09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0A4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C0A4: lea.l $100440.l, a0
pub fn i_08c0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $08C0AA: move.l #$335a6, (a0)
pub fn i_08c0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0B0;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x335A6;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C0B0: jsr $5fe.l
pub fn i_08c0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0B6);
    cpu.pc = 0x5FE;
    16
}
/// $08C0B6: lea.l $8c2b8(pc), a1
pub fn i_08c0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C2B8;
    16
}
/// $08C0BA: jsr $4ae.l
pub fn i_08c0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0C0);
    cpu.pc = 0x4AE;
    16
}
/// $08C0C0: lea.l $8c322(pc), a1
pub fn i_08c0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C322;
    16
}
/// $08C0C4: jsr $4ae.l
pub fn i_08c0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0CA);
    cpu.pc = 0x4AE;
    16
}
/// $08C0CA: lea.l $8c37e(pc), a1
pub fn i_08c0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C37E;
    16
}
/// $08C0CE: jsr $4ae.l
pub fn i_08c0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0D4);
    cpu.pc = 0x4AE;
    16
}
/// $08C0D4: lea.l $8c3da(pc), a1
pub fn i_08c0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C3DA;
    16
}
/// $08C0D8: jsr $4ae.l
pub fn i_08c0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0DE);
    cpu.pc = 0x4AE;
    16
}
/// $08C0DE: lea.l $8c436(pc), a1
pub fn i_08c0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C436;
    16
}
/// $08C0E2: jsr $4ae.l
pub fn i_08c0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0E8);
    cpu.pc = 0x4AE;
    16
}
/// $08C0E8: lea.l $8c5b2(pc), a1
pub fn i_08c0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C5B2;
    16
}
/// $08C0EC: jsr $4ae.l
pub fn i_08c0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0F2);
    cpu.pc = 0x4AE;
    16
}
/// $08C0F2: move.b #$0, $32(a6)
pub fn i_08c0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0F8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C0F8: move.b #$0, $33(a6)
pub fn i_08c0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C0FE: move.w #$0, $34(a6)
pub fn i_08c0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C104;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C104: lea.l $8c10a(pc), a1
pub fn i_08c104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C108;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C10A;
    16
}
/// $08C108: move.l a1, (a6)
pub fn i_08c108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C10A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C10A: lea.l $2c072c.l, a1
pub fn i_08c10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C110;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $08C110: move.w $34(a6), d0
pub fn i_08c110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C114;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08C114: lsl.w #$1, d0
pub fn i_08c114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C116;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $08C116: move.w (a1, d0.w), d1
pub fn i_08c116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C11A;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08C11A: cmpi.w #$ff, d1
pub fn i_08c11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C11E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Word, false);
    8
}
/// $08C11E: ble.w $8c126
pub fn i_08c11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C122;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8C126; return 10; }
    8
}
/// $08C122: move.w #$ff, d1
pub fn i_08c122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C126;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C126: move.b d1, $33(a6)
pub fn i_08c126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C12A;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C12A: addq.w #$8, $34(a6)
pub fn i_08c12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C12E;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C12E: cmpi.w #$3f, $34(a6)
pub fn i_08c12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C134;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3F, Size::Word, false);
    16
}
/// $08C134: ble.w $8c14a
pub fn i_08c134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C138;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8C14A; return 10; }
    8
}
/// $08C138: move.w #$0, $34(a6)
pub fn i_08c138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C13E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C13E: move.b #$ff, $33(a6)
pub fn i_08c13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C144;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C144: lea.l $8c15e(pc), a1
pub fn i_08c144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C148;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C15E;
    16
}
/// $08C148: move.l a1, (a6)
pub fn i_08c148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C14A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C14A: jsr $967c0.l
pub fn i_08c14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C150;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C150);
    cpu.pc = 0x967C0;
    16
}
/// $08C150: jsr $436de.l
pub fn i_08c150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C156;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C156);
    cpu.pc = 0x436DE;
    16
}
/// $08C156: jsr $28d70.l
pub fn i_08c156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C15C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C15C);
    cpu.pc = 0x28D70;
    16
}
/// $08C15C: rts
pub fn i_08c15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C15E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_State_F2_08C15E  ($08C15E, 78 B, entity/anim_state_machine_08c008.s)
/// $08C15E: lea.l $2c072c.l, a1
pub fn i_08c15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C164;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $08C164: move.w $34(a6), d0
pub fn i_08c164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C168;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08C168: lsl.w #$1, d0
pub fn i_08c168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C16A;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $08C16A: move.w (a1, d0.w), d1
pub fn i_08c16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C16E;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08C16E: cmpi.w #$ff, d1
pub fn i_08c16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C172;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Word, false);
    8
}
/// $08C172: ble.w $8c17a
pub fn i_08c172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C176;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8C17A; return 10; }
    8
}
/// $08C176: move.w #$ff, d1
pub fn i_08c176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C17A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C17A: move.b d1, $32(a6)
pub fn i_08c17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C17E;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C17E: addq.w #$4, $34(a6)
pub fn i_08c17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C182;
    cpu.ir = 0x586E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C182: cmpi.w #$3f, $34(a6)
pub fn i_08c182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C188;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3F, Size::Word, false);
    16
}
/// $08C188: ble.w $8c198
pub fn i_08c188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C18C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8C198; return 10; }
    8
}
/// $08C18C: move.b #$ff, $32(a6)
pub fn i_08c18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C192;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C192: lea.l $8c1ac(pc), a1
pub fn i_08c192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C196;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C1AC;
    16
}
/// $08C196: move.l a1, (a6)
pub fn i_08c196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C198;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C198: jsr $967c0.l
pub fn i_08c198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C19E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C19E);
    cpu.pc = 0x967C0;
    16
}
/// $08C19E: jsr $436de.l
pub fn i_08c19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C1A4);
    cpu.pc = 0x436DE;
    16
}
/// $08C1A4: jsr $28d70.l
pub fn i_08c1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C1AA);
    cpu.pc = 0x28D70;
    16
}
/// $08C1AA: rts
pub fn i_08c1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_State_F3_08C1AC  ($08C1AC, 62 B, entity/anim_state_machine_08c008.s)
/// $08C1AC: jsr $8bc74(pc)
pub fn i_08c1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8C1B0);
    cpu.pc = 0x8BC74;
    16
}
/// $08C1B0: bcc.w $8c1d6
pub fn i_08c1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8C1D6; return 10; }
    8
}
/// $08C1B4: movea.w #$7084, a1
pub fn i_08c1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1B8;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7084;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $08C1B8: move.w #$2320, d0
pub fn i_08c1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1BC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C1BC: move.w #$20, d1
pub fn i_08c1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C1C0: move.w #$19, d2
pub fn i_08c1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1C4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C1C4: jsr $5da9c.l
pub fn i_08c1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C1CA);
    cpu.pc = 0x5DA9C;
    16
}
/// $08C1CA: move.w #$0, $34(a6)
pub fn i_08c1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C1D0: lea.l $8c1ea(pc), a1
pub fn i_08c1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C1EA;
    16
}
/// $08C1D4: move.l a1, (a6)
pub fn i_08c1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C1D6: jsr $967c0.l
pub fn i_08c1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C1DC);
    cpu.pc = 0x967C0;
    16
}
/// $08C1DC: jsr $436de.l
pub fn i_08c1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C1E2);
    cpu.pc = 0x436DE;
    16
}
/// $08C1E2: jsr $28d70.l
pub fn i_08c1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C1E8);
    cpu.pc = 0x28D70;
    16
}
/// $08C1E8: rts
pub fn i_08c1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Base_08c2b8  ($08C2B8, 98 B, mission/cutscene_anim_08ba0c.s)
/// $08C2B8: move.w #$e000, $38(a6)
pub fn i_08c2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C2BE: bset.b #$6, $12(a6)
pub fn i_08c2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2C4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C2C4: move.w #$b1, d1
pub fn i_08c2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C2C8: jsr $236e.l
pub fn i_08c2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C2CE);
    cpu.pc = 0x236E;
    16
}
/// $08C2CE: lea.l $2f2e0a.l, a0
pub fn i_08c2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2E0A;
    20
}
/// $08C2D4: jsr $28cd4.l
pub fn i_08c2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C2DA);
    cpu.pc = 0x28CD4;
    16
}
/// $08C2DA: move.w #$40, $22(a6)
pub fn i_08c2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C2E0: move.w #$12b, $24(a6)
pub fn i_08c2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C2E6: clr.w $26(a6)
pub fn i_08c2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2EA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C2EA: lea.l $8c2f0(pc), a1
pub fn i_08c2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C2F0;
    16
}
/// $08C2EE: move.l a1, (a6)
pub fn i_08c2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C2F0: movea.l $c(a6), a0
pub fn i_08c2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2F4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08C2F4: move.b $92(a0), d0
pub fn i_08c2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2F8;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08C2F8: andi.l #$f, d0
pub fn i_08c2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2FE;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $08C2FE: movea.l #$2f2dca, a0
pub fn i_08c2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C304;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F2DCA;
    cpu.a[0] = v1;
    4
}
/// $08C304: lsl.w #$2, d0
pub fn i_08c304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C306;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08C306: movea.l (a0, d0.w), a0
pub fn i_08c306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C30A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08C30A: cmpa.l #$ffffffff, a0
pub fn i_08c30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C310;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08C310: beq.w $8c31a
pub fn i_08c310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C314;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C31A; return 10; }
    8
}
/// $08C314: jsr $28cd4.l
pub fn i_08c314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C31A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C31A);
    cpu.pc = 0x28CD4;
    16
}
// ===== JsrAbsThunk_08c31a  ($08C31A, 8 B, thunks/jsr_abs_thunks.c)
/// $08C31A: jsr $28d70.l
pub fn i_08c31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C320;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C320);
    cpu.pc = 0x28D70;
    16
}
/// $08C320: rts
pub fn i_08c320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C322;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Slot1_08c322  ($08C322, 84 B, mission/cutscene_anim_08ba0c.s)
/// $08C322: move.w #$e000, $38(a6)
pub fn i_08c322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C328;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C328: bset.b #$6, $12(a6)
pub fn i_08c328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C32E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C32E: move.w #$ae, d1
pub fn i_08c32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C332;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C332: jsr $236e.l
pub fn i_08c332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C338);
    cpu.pc = 0x236E;
    16
}
/// $08C338: lea.l $2f2ed6.l, a0
pub fn i_08c338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C33E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2ED6;
    20
}
/// $08C33E: jsr $28cd4.l
pub fn i_08c33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C344;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C344);
    cpu.pc = 0x28CD4;
    16
}
/// $08C344: move.w #$70, $22(a6)
pub fn i_08c344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C34A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C34A: move.w #$12b, $24(a6)
pub fn i_08c34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C350;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C350: clr.w $26(a6)
pub fn i_08c350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C354;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C354: lea.l $8c4fa(pc), a1
pub fn i_08c354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C358;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C4FA;
    16
}
/// $08C358: jsr $4ae.l
pub fn i_08c358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C35E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C35E);
    cpu.pc = 0x4AE;
    16
}
/// $08C35E: jsr $5dd02.l
pub fn i_08c35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C364;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C364);
    cpu.pc = 0x5DD02;
    16
}
/// $08C364: move.b #$10, $8a(a6)
pub fn i_08c364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C36A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C36A: move.w #$0, $76(a6)
pub fn i_08c36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C370;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C370: lea.l $8c4a0(pc), a1
pub fn i_08c370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C374;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C4A0;
    16
}
/// $08C374: move.l a1, (a6)
pub fn i_08c374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C376;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_08c376  ($08C376, 8 B, thunks/jsr_abs_thunks.c)
/// $08C376: jsr $28d70.l
pub fn i_08c376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C37C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C37C);
    cpu.pc = 0x28D70;
    16
}
/// $08C37C: rts
pub fn i_08c37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C37E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Slot2_08c37e  ($08C37E, 84 B, mission/cutscene_anim_08ba0c.s)
/// $08C37E: move.w #$e000, $38(a6)
pub fn i_08c37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C384;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C384: bset.b #$6, $12(a6)
pub fn i_08c384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C38A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C38A: move.w #$af, d1
pub fn i_08c38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C38E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C38E: jsr $236e.l
pub fn i_08c38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C394;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C394);
    cpu.pc = 0x236E;
    16
}
/// $08C394: lea.l $2f2ed6.l, a0
pub fn i_08c394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C39A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2ED6;
    20
}
/// $08C39A: jsr $28cd4.l
pub fn i_08c39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C3A0);
    cpu.pc = 0x28CD4;
    16
}
/// $08C3A0: move.w #$a0, $22(a6)
pub fn i_08c3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3A6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C3A6: move.w #$12b, $24(a6)
pub fn i_08c3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3AC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C3AC: clr.w $26(a6)
pub fn i_08c3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3B0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C3B0: lea.l $8c526(pc), a1
pub fn i_08c3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C526;
    16
}
/// $08C3B4: jsr $4ae.l
pub fn i_08c3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C3BA);
    cpu.pc = 0x4AE;
    16
}
/// $08C3BA: jsr $5dd02.l
pub fn i_08c3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C3C0);
    cpu.pc = 0x5DD02;
    16
}
/// $08C3C0: move.b #$20, $8a(a6)
pub fn i_08c3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3C6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C3C6: move.w #$0, $76(a6)
pub fn i_08c3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C3CC: lea.l $8c4a0(pc), a1
pub fn i_08c3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C4A0;
    16
}
/// $08C3D0: move.l a1, (a6)
pub fn i_08c3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_08c3d2  ($08C3D2, 8 B, thunks/jsr_abs_thunks.c)
/// $08C3D2: jsr $28d70.l
pub fn i_08c3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C3D8);
    cpu.pc = 0x28D70;
    16
}
/// $08C3D8: rts
pub fn i_08c3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Slot3_08c3da  ($08C3DA, 84 B, mission/cutscene_anim_08ba0c.s)
/// $08C3DA: move.w #$e000, $38(a6)
pub fn i_08c3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C3E0: bset.b #$6, $12(a6)
pub fn i_08c3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3E6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C3E6: move.w #$b0, d1
pub fn i_08c3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3EA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C3EA: jsr $236e.l
pub fn i_08c3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C3F0);
    cpu.pc = 0x236E;
    16
}
/// $08C3F0: lea.l $2f2ed6.l, a0
pub fn i_08c3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2ED6;
    20
}
/// $08C3F6: jsr $28cd4.l
pub fn i_08c3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C3FC);
    cpu.pc = 0x28CD4;
    16
}
/// $08C3FC: move.w #$d0, $22(a6)
pub fn i_08c3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C402;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C402: move.w #$12b, $24(a6)
pub fn i_08c402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C408;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C408: clr.w $26(a6)
pub fn i_08c408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C40C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C40C: lea.l $8c552(pc), a1
pub fn i_08c40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C410;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C552;
    16
}
/// $08C410: jsr $4ae.l
pub fn i_08c410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C416;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C416);
    cpu.pc = 0x4AE;
    16
}
/// $08C416: jsr $5dd02.l
pub fn i_08c416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C41C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C41C);
    cpu.pc = 0x5DD02;
    16
}
/// $08C41C: move.b #$40, $8a(a6)
pub fn i_08c41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C422;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C422: move.w #$0, $76(a6)
pub fn i_08c422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C428;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C428: lea.l $8c4a0(pc), a1
pub fn i_08c428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C42C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C4A0;
    16
}
/// $08C42C: move.l a1, (a6)
pub fn i_08c42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C42E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_08c42e  ($08C42E, 8 B, thunks/jsr_abs_thunks.c)
/// $08C42E: jsr $28d70.l
pub fn i_08c42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C434;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C434);
    cpu.pc = 0x28D70;
    16
}
/// $08C434: rts
pub fn i_08c434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C436;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Slot4_08c436  ($08C436, 140 B, mission/cutscene_anim_08ba0c.s)
/// $08C436: movea.w #$73d7, a1
pub fn i_08c436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C43A;
    cpu.ir = 0x327C;
    let v1: u32 = 0x73D7;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $08C43A: move.w #$2e80, d0
pub fn i_08c43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C43E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2E80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C43E: move.w #$4, d1
pub fn i_08c43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C442;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C442: move.w #$4, d2
pub fn i_08c442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C446;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C446: jsr $5da56.l
pub fn i_08c446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C44C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C44C);
    cpu.pc = 0x5DA56;
    16
}
/// $08C44C: move.w #$e000, $38(a6)
pub fn i_08c44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C452;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C452: bset.b #$6, $12(a6)
pub fn i_08c452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C458;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C458: move.w #$b2, d1
pub fn i_08c458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C45C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C45C: jsr $236e.l
pub fn i_08c45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C462;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C462);
    cpu.pc = 0x236E;
    16
}
/// $08C462: lea.l $2f2ed6.l, a0
pub fn i_08c462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C468;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2ED6;
    20
}
/// $08C468: jsr $28cd4.l
pub fn i_08c468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C46E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C46E);
    cpu.pc = 0x28CD4;
    16
}
/// $08C46E: move.w #$100, $22(a6)
pub fn i_08c46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C474;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C474: move.w #$12b, $24(a6)
pub fn i_08c474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C47A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C47A: clr.w $26(a6)
pub fn i_08c47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C47E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C47E: lea.l $8c57e(pc), a1
pub fn i_08c47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C482;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C57E;
    16
}
/// $08C482: jsr $4ae.l
pub fn i_08c482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C488;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C488);
    cpu.pc = 0x4AE;
    16
}
/// $08C488: jsr $5dd02.l
pub fn i_08c488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C48E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C48E);
    cpu.pc = 0x5DD02;
    16
}
/// $08C48E: move.b #$80, $8a(a6)
pub fn i_08c48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C494;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C494: move.w #$0, $76(a6)
pub fn i_08c494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C49A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C49A: lea.l $8c4a0(pc), a1
pub fn i_08c49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C49E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C4A0;
    16
}
/// $08C49E: move.l a1, (a6)
pub fn i_08c49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C4A0: movea.l $c(a6), a0
pub fn i_08c4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4A4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08C4A4: move.b $93(a0), d0
pub fn i_08c4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4A8;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x93);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08C4A8: and.b $8a(a6), d0
pub fn i_08c4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4AC;
    cpu.ir = 0xC02E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08C4AC: beq.w $8c4ca
pub fn i_08c4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C4CA; return 10; }
    8
}
/// $08C4B0: move.w #$1, $76(a6)
pub fn i_08c4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C4B6: lea.l $2f2ee2.l, a0
pub fn i_08c4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2EE2;
    20
}
/// $08C4BC: jsr $28cd4.l
pub fn i_08c4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C4C2);
    cpu.pc = 0x28CD4;
    16
}
// ===== JsrAbsThunk_08c4c2  ($08C4C2, 8 B, thunks/jsr_abs_thunks.c)
/// $08C4C2: jsr $28d70.l
pub fn i_08c4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C4C8);
    cpu.pc = 0x28D70;
    16
}
/// $08C4C8: rts
pub fn i_08c4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Slot_Tail_08c4ca  ($08C4CA, 40 B, mission/cutscene_anim_08ba0c.s)
/// $08C4CA: move.b $92(a0), d0
pub fn i_08c4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4CE;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08C4CE: and.b $8a(a6), d0
pub fn i_08c4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4D2;
    cpu.ir = 0xC02E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08C4D2: bne.w $8c4f2
pub fn i_08c4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4D6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C4F2; return 10; }
    8
}
/// $08C4D6: cmpi.w #$1, $76(a6)
pub fn i_08c4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4DC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $08C4DC: bne.w $8c4f2
pub fn i_08c4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C4F2; return 10; }
    8
}
/// $08C4E0: move.w #$0, $76(a6)
pub fn i_08c4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C4E6: lea.l $2f2f02.l, a0
pub fn i_08c4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2F02;
    20
}
/// $08C4EC: jsr $28cd4.l
pub fn i_08c4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C4F2);
    cpu.pc = 0x28CD4;
    16
}
// ===== JsrAbsThunk_08c4f2  ($08C4F2, 8 B, thunks/jsr_abs_thunks.c)
/// $08C4F2: jsr $28d70.l
pub fn i_08c4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C4F8);
    cpu.pc = 0x28D70;
    16
}
/// $08C4F8: rts
pub fn i_08c4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C4FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Slot1_Lit_08c4fa  ($08C4FA, 44 B, mission/cutscene_anim_08ba0c.s)
/// $08C4FA: move.w #$e000, $38(a6)
pub fn i_08c4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C500;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C500: bset.b #$6, $12(a6)
pub fn i_08c500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C506;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C506: move.w #$ae, d1
pub fn i_08c506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C50A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C50A: jsr $236e.l
pub fn i_08c50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C510);
    cpu.pc = 0x236E;
    16
}
/// $08C510: lea.l $2f2f22.l, a0
pub fn i_08c510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C516;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2F22;
    20
}
/// $08C516: jsr $28cd4.l
pub fn i_08c516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C51C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C51C);
    cpu.pc = 0x28CD4;
    16
}
/// $08C51C: lea.l $8c5aa(pc), a1
pub fn i_08c51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C520;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C5AA;
    16
}
/// $08C520: move.l a1, (a6)
pub fn i_08c520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C522;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C522: bra.w $8c5aa
pub fn i_08c522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C526;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C5AA;
    10
}
// ===== Icon_Slot2_Lit_08c526  ($08C526, 44 B, mission/cutscene_anim_08ba0c.s)
/// $08C526: move.w #$e000, $38(a6)
pub fn i_08c526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C52C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C52C: bset.b #$6, $12(a6)
pub fn i_08c52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C532;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C532: move.w #$af, d1
pub fn i_08c532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C536;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C536: jsr $236e.l
pub fn i_08c536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C53C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C53C);
    cpu.pc = 0x236E;
    16
}
/// $08C53C: lea.l $2f2f2e.l, a0
pub fn i_08c53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C542;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2F2E;
    20
}
/// $08C542: jsr $28cd4.l
pub fn i_08c542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C548;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C548);
    cpu.pc = 0x28CD4;
    16
}
/// $08C548: lea.l $8c5aa(pc), a1
pub fn i_08c548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C54C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C5AA;
    16
}
/// $08C54C: move.l a1, (a6)
pub fn i_08c54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C54E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C54E: bra.w $8c5aa
pub fn i_08c54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C552;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C5AA;
    10
}
// ===== Icon_Slot3_Lit_08c552  ($08C552, 44 B, mission/cutscene_anim_08ba0c.s)
/// $08C552: move.w #$e000, $38(a6)
pub fn i_08c552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C558;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C558: bset.b #$6, $12(a6)
pub fn i_08c558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C55E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C55E: move.w #$b0, d1
pub fn i_08c55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C562;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C562: jsr $236e.l
pub fn i_08c562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C568;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C568);
    cpu.pc = 0x236E;
    16
}
/// $08C568: lea.l $2f2f3a.l, a0
pub fn i_08c568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C56E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2F3A;
    20
}
/// $08C56E: jsr $28cd4.l
pub fn i_08c56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C574;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C574);
    cpu.pc = 0x28CD4;
    16
}
/// $08C574: lea.l $8c5aa(pc), a1
pub fn i_08c574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C578;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C5AA;
    16
}
/// $08C578: move.l a1, (a6)
pub fn i_08c578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C57A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C57A: bra.w $8c5aa
pub fn i_08c57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C57E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C5AA;
    10
}
// ===== Icon_Slot4_Lit_08c57e  ($08C57E, 44 B, mission/cutscene_anim_08ba0c.s)
/// $08C57E: move.w #$e000, $38(a6)
pub fn i_08c57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C584;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C584: bset.b #$6, $12(a6)
pub fn i_08c584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C58A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C58A: move.w #$b2, d1
pub fn i_08c58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C58E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C58E: jsr $236e.l
pub fn i_08c58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C594;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C594);
    cpu.pc = 0x236E;
    16
}
/// $08C594: lea.l $2f2f46.l, a0
pub fn i_08c594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C59A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2F46;
    20
}
/// $08C59A: jsr $28cd4.l
pub fn i_08c59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C5A0);
    cpu.pc = 0x28CD4;
    16
}
/// $08C5A0: lea.l $8c5aa(pc), a1
pub fn i_08c5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C5AA;
    16
}
/// $08C5A4: move.l a1, (a6)
pub fn i_08c5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C5A6: bra.w $8c5aa
pub fn i_08c5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C5AA;
    10
}
// ===== JsrAbsThunk_08c5aa  ($08C5AA, 8 B, thunks/jsr_abs_thunks.c)
/// $08C5AA: jsr $28d70.l
pub fn i_08c5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C5B0);
    cpu.pc = 0x28D70;
    16
}
/// $08C5B0: rts
pub fn i_08c5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Anchor_Init_08c5b2  ($08C5B2, 190 B, mission/cutscene_anim_08ba0c.s)
/// $08C5B2: move.w #$8000, $38(a6)
pub fn i_08c5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C5B8: cmpi.b #$0, $10fd83.l
pub fn i_08c5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5C0;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $08C5C0: bne.w $8c5ce
pub fn i_08c5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C5CE; return 10; }
    8
}
/// $08C5C4: move.b #$0, $8a(a6)
pub fn i_08c5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C5CA: bra.w $8c614
pub fn i_08c5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C614;
    10
}
/// $08C5CE: cmpi.b #$2, $10fd83.l
pub fn i_08c5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5D6;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $08C5D6: bne.w $8c60e
pub fn i_08c5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C60E; return 10; }
    8
}
/// $08C5DA: move.b $10fd92.l, d0
pub fn i_08c5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5E0;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FD92) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08C5E0: cmpi.b #$2, d0
pub fn i_08c5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5E4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $08C5E4: bne.w $8c5ee
pub fn i_08c5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C5EE; return 10; }
    8
}
/// $08C5E8: move.b #$3, $8a(a6)
pub fn i_08c5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C5EE: cmpi.b #$1, d0
pub fn i_08c5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5F2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $08C5F2: bne.w $8c5fc
pub fn i_08c5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C5FC; return 10; }
    8
}
/// $08C5F6: move.b #$2, $8a(a6)
pub fn i_08c5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C5FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C5FC: cmpi.b #$0, d0
pub fn i_08c5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C600;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $08C600: bne.w $8c60a
pub fn i_08c600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C604;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C60A; return 10; }
    8
}
/// $08C604: move.b #$1, $8a(a6)
pub fn i_08c604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C60A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C60A: bra.w $8c614
pub fn i_08c60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C60E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C614;
    10
}
/// $08C60E: move.b #$1, $8a(a6)
pub fn i_08c60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C614;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C614: move.w #$97, d1
pub fn i_08c614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C618;
    cpu.ir = 0x323C;
    let v1: u32 = 0x97;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C618: jsr $236e.l
pub fn i_08c618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C61E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C61E);
    cpu.pc = 0x236E;
    16
}
/// $08C61E: lea.l $2f2f52.l, a0
pub fn i_08c61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C624;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2F52;
    20
}
/// $08C624: jsr $28cd4.l
pub fn i_08c624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C62A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C62A);
    cpu.pc = 0x28CD4;
    16
}
/// $08C62A: move.w #$e0, $22(a6)
pub fn i_08c62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C630;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C630: move.w #$160, $24(a6)
pub fn i_08c630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C636;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C636: clr.w $26(a6)
pub fn i_08c636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C63A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C63A: jsr $267e2.l
pub fn i_08c63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C640;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C640);
    cpu.pc = 0x267E2;
    16
}
/// $08C640: clr.l $2c(a6)
pub fn i_08c640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C644;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $08C644: move.l #$7084, $86(a6)
pub fn i_08c644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C64C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x7084;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C64C: move.w #$0, $70(a6)
pub fn i_08c64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C652;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C652: move.w #$0, $80(a6)
pub fn i_08c652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C658;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C658: move.l #$0, $82(a6)
pub fn i_08c658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C660;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C660: lea.l $2f0056.l, a1
pub fn i_08c660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C666;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F0056;
    20
}
/// $08C666: move.l a1, $7c(a6)
pub fn i_08c666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C66A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C66A: move.w #$0, $76(a6)
pub fn i_08c66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C670;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08c670  ($08C670, 8 B, kernel/task_handlers.c)
/// $08C670: lea.l $8c678(pc), a1
pub fn i_08c670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C674;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C678;
    16
}
/// $08C674: move.l a1, (a6)
pub fn i_08c674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C676;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C676: rts
pub fn i_08c676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C678;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Anchor_Run_08c678  ($08C678, 170 B, mission/cutscene_anim_08ba0c.s)
/// $08C678: movea.l $c(a6), a0
pub fn i_08c678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C67C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08C67C: cmpi.w #$0, $76(a0)
pub fn i_08c67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C682;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08C682: beq.w $8c70c
pub fn i_08c682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C686;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C70C; return 10; }
    8
}
/// $08C686: clr.l d0
pub fn i_08c686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C688;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08C688: move.w $76(a0), d0
pub fn i_08c688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C68C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08C68C: cmp.w $76(a6), d0
pub fn i_08c68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C690;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08C690: beq.w $8c6f6
pub fn i_08c690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C694;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C6F6; return 10; }
    8
}
/// $08C694: move.w d0, $76(a6)
pub fn i_08c694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C698;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C698: cmpi.b #$0, $8a(a6)
pub fn i_08c698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C69E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08C69E: bne.w $8c6ac
pub fn i_08c69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C6AC; return 10; }
    8
}
/// $08C6A2: lea.l $2efacc.l, a1
pub fn i_08c6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6A8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EFACC;
    20
}
/// $08C6A8: bra.w $8c6dc
pub fn i_08c6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C6DC;
    10
}
/// $08C6AC: cmpi.b #$3, $8a(a6)
pub fn i_08c6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6B2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $08C6B2: bne.w $8c6bc
pub fn i_08c6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C6BC; return 10; }
    8
}
/// $08C6B6: lea.l $2efeec.l, a1
pub fn i_08c6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6BC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EFEEC;
    20
}
/// $08C6BC: cmpi.b #$2, $8a(a6)
pub fn i_08c6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6C2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $08C6C2: bne.w $8c6cc
pub fn i_08c6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6C6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C6CC; return 10; }
    8
}
/// $08C6C6: lea.l $2efd8c.l, a1
pub fn i_08c6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6CC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EFD8C;
    20
}
/// $08C6CC: cmpi.b #$1, $8a(a6)
pub fn i_08c6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6D2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08C6D2: bne.w $8c6dc
pub fn i_08c6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6D6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C6DC; return 10; }
    8
}
/// $08C6D6: lea.l $2efc2c.l, a1
pub fn i_08c6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6DC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EFC2C;
    20
}
/// $08C6DC: lsl.l #$3, d0
pub fn i_08c6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6DE;
    cpu.ir = 0xE788;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 1, true);
    cpu.d[0] = r;
    12
}
/// $08C6DE: adda.l d0, a1
pub fn i_08c6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6E0;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $08C6E0: move.l (a1)+, $7c(a6)
pub fn i_08c6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6E4;
    cpu.ir = 0x2D59;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $08C6E4: move.l (a1), $86(a6)
pub fn i_08c6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6E8;
    cpu.ir = 0x2D51;
    let v2 = bus.read32(cpu.a[1]);
    let ad3 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $08C6E8: move.l #$0, $82(a6)
pub fn i_08c6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6F0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C6F0: move.w #$0, $70(a6)
pub fn i_08c6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C6F6: cmpi.b #$0, $8a(a6)
pub fn i_08c6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C6FC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08C6FC: bne.w $8c708
pub fn i_08c6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C700;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C708; return 10; }
    8
}
/// $08C700: jsr $8bce6(pc)
pub fn i_08c700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C704;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8C704);
    cpu.pc = 0x8BCE6;
    16
}
/// $08C704: bra.w $8c70c
pub fn i_08c704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C708;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C70C;
    10
}
/// $08C708: jsr $8be28(pc)
pub fn i_08c708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C70C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8C70C);
    cpu.pc = 0x8BE28;
    16
}
/// $08C70C: jsr $27cee.l
pub fn i_08c70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C712;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C712);
    cpu.pc = 0x27CEE;
    16
}
/// $08C712: jsr $28d70.l
pub fn i_08c712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C718;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C718);
    cpu.pc = 0x28D70;
    16
}
/// $08C718: cmpi.w #$150, $22(a6)
pub fn i_08c718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C71E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $08C71E: bls.w $8c72e
pub fn i_08c71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C722;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x8C72E; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08c722  ($08C722, 14 B, kernel/jsr5b6_jmp518.c)
/// $08C722: jsr $5b6.l
pub fn i_08c722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C728;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C728);
    cpu.pc = 0x5B6;
    16
}
/// $08C728: jmp $518.l
pub fn i_08c728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C72E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08C72E: rts
pub fn i_08c72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C730;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_InstallListByVariant_08c820  ($08C820, 60 B, mission/cutscene_anim_08ba0c.s)
/// $08C820: cmpi.b #$2, $10e2ee.l
pub fn i_08c820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C828;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E2EE) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $08C828: bne.w $8c838
pub fn i_08c828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C82C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C838; return 10; }
    8
}
/// $08C82C: lea.l $2f3688.l, a0
pub fn i_08c82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C832;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3688;
    20
}
/// $08C832: jmp $5db58.l
pub fn i_08c832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C838;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5DB58;
    8
}
/// $08C838: cmpi.b #$1, $10e2ee.l
pub fn i_08c838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C840;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E2EE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $08C840: bne.w $8c850
pub fn i_08c840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C844;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C850; return 10; }
    8
}
/// $08C844: lea.l $2f35fe.l, a0
pub fn i_08c844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C84A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F35FE;
    20
}
/// $08C84A: jmp $5db58.l
pub fn i_08c84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C850;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5DB58;
    8
}
/// $08C850: lea.l $2f3574.l, a0
pub fn i_08c850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C856;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F3574;
    20
}
/// $08C856: jmp $5db58.l
pub fn i_08c856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C85C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5DB58;
    8
}
// ===== ClrRamWord_08c85c  ($08C85C, 8 B, thunks/clr_ram_word.c)
/// $08C85C: clr.b $10e2ee.l
pub fn i_08c85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C862;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E2EE) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E2EE, r2 as u8);
    16
}
/// $08C862: rts
pub fn i_08c862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C864;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cut_Watcher_Init_08c864  ($08C864, 28 B, mission/cutscene_anim_08ba0c.s)
/// $08C864: move.w #$7408, $22(a6)
pub fn i_08c864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C86A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7408;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C86A: clr.b $10e2ef.l
pub fn i_08c86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C870;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E2EF) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E2EF, r2 as u8);
    16
}
/// $08C870: move.w #$ffff, $70(a6)
pub fn i_08c870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C876;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C876: clr.w $30(a6)
pub fn i_08c876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C87A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C87A: lea.l $8c880(pc), a1
pub fn i_08c87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C87E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C880;
    16
}
/// $08C87E: move.l a1, (a6)
pub fn i_08c87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C880;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Cut_Watcher_Run_08c880  ($08C880, 122 B, mission/cutscene_anim_08ba0c.s)
/// $08C880: move.w $106f5c.l, d0
pub fn i_08c880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C886;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $08C886: cmp.w $70(a6), d0
pub fn i_08c886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C88A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08C88A: beq.w $8c89c
pub fn i_08c88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C88E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C89C; return 10; }
    8
}
/// $08C88E: move.w d0, $70(a6)
pub fn i_08c88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C892;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C892: move.w #$78, $30(a6)
pub fn i_08c892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C898;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C898: bra.w $8c8e8
pub fn i_08c898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C89C;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C8E8;
    10
}
/// $08C89C: tst.w $30(a6)
pub fn i_08c89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8A0;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $08C8A0: beq.w $8c8ac
pub fn i_08c8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C8AC; return 10; }
    8
}
/// $08C8A4: subq.w #$1, $30(a6)
pub fn i_08c8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8A8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C8A8: bra.w $8c8e8
pub fn i_08c8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C8E8;
    10
}
/// $08C8AC: move.w $106e88.l, d0
pub fn i_08c8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8B2;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106E88) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $08C8B2: or.w $106e8a.l, d0
pub fn i_08c8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8B8;
    cpu.ir = 0x8079;
    let v2 = bus.read16(0x106E8A) as u32;
    let r = (v2 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $08C8B8: bne.w $8c8e8
pub fn i_08c8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8BC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C8E8; return 10; }
    8
}
/// $08C8BC: jsr $5e1aa.l
pub fn i_08c8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C8C2);
    cpu.pc = 0x5E1AA;
    16
}
/// $08C8C2: bcs.w $8c8e8
pub fn i_08c8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8C8E8; return 10; }
    8
}
/// $08C8C6: jsr $43ce2.l
pub fn i_08c8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C8CC);
    cpu.pc = 0x43CE2;
    16
}
/// $08C8CC: bcs.w $8c8e8
pub fn i_08c8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8D0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8C8E8; return 10; }
    8
}
/// $08C8D0: tst.b $106ed3.l
pub fn i_08c8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8D6;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $08C8D6: beq.w $8c8e8
pub fn i_08c8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C8E8; return 10; }
    8
}
/// $08C8DA: move.w #$78, $30(a6)
pub fn i_08c8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C8E0: move.b #$2, $10e2ef.l
pub fn i_08c8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8E8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EF, v1 as u8);
    16
}
/// $08C8E8: tst.b $10e2ef.l
pub fn i_08c8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8EE;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E2EF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $08C8EE: bne.w $8c8f4
pub fn i_08c8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8F2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C8F4; return 10; }
    8
}
/// $08C8F2: rts
pub fn i_08c8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08C8F4: lea.l $8c8fa(pc), a1
pub fn i_08c8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C8FA;
    16
}
/// $08C8F8: move.l a1, (a6)
pub fn i_08c8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Cut_Watcher_Play_08c8fa  ($08C8FA, 56 B, mission/cutscene_anim_08ba0c.s)
/// $08C8FA: bsr.w $8c820
pub fn i_08c8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C8FE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8C8FE);
    cpu.pc = 0x8C820;
    18
}
/// $08C8FE: move.w #$10d5, d0
pub fn i_08c8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C902;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10D5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C902: jsr $2352.l
pub fn i_08c902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C908;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C908);
    cpu.pc = 0x2352;
    16
}
/// $08C908: lea.l $8c90e(pc), a1
pub fn i_08c908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C90C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C90E;
    16
}
/// $08C90C: move.l a1, (a6)
pub fn i_08c90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C90E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C90E: jsr $5db6a.l
pub fn i_08c90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C914;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C914);
    cpu.pc = 0x5DB6A;
    16
}
/// $08C914: bcc.w $8c938
pub fn i_08c914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C918;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8C938; return 10; }
    8
}
/// $08C918: subq.b #$1, $10e2ef.l
pub fn i_08c918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C91E;
    cpu.ir = 0x5339;
    let v1 = bus.read8(0x10E2EF) as u32;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10E2EF, r2 as u8);
    16
}
/// $08C91E: bne.w $8c932
pub fn i_08c91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C922;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8C932; return 10; }
    8
}
/// $08C922: jsr $5dbdc.l
pub fn i_08c922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C928;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C928);
    cpu.pc = 0x5DBDC;
    16
}
/// $08C928: lea.l $8c880(pc), a1
pub fn i_08c928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C92C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C880;
    16
}
/// $08C92C: move.l a1, (a6)
pub fn i_08c92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C92E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C92E: bra.w $8c938
pub fn i_08c92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C932;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C938;
    10
}
// ===== SetTaskHandler_08c932  ($08C932, 8 B, kernel/task_handlers.c)
/// $08C932: lea.l $8c8fa(pc), a1
pub fn i_08c932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C936;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C8FA;
    16
}
/// $08C936: move.l a1, (a6)
pub fn i_08c936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C938;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C938: rts
pub fn i_08c938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C93A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pos_IntegrateXY88_Accel_08d34e  ($08D34E, 102 B, enemies/soldiers/grunts_capsule_08d184.s)
/// $08D34E: tst.w $28(a6)
pub fn i_08d34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D352;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $08D352: beq.w $8d378
pub fn i_08d352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D356;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8D378; return 10; }
    8
}
/// $08D356: move.w $22(a6), d0
pub fn i_08d356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D35A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D35A: swap d0
pub fn i_08d35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D35C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D35C: move.w $26(a6), d0
pub fn i_08d35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D360;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D360: clr.b d0
pub fn i_08d360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D362;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08D362: move.w $28(a6), d1
pub fn i_08d362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D366;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D366: ext.l d1
pub fn i_08d366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D368;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D368: asl.l #$8, d1
pub fn i_08d368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D36A;
    cpu.ir = 0xE181;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, true);
    cpu.d[1] = r;
    22
}
/// $08D36A: add.l d1, d0
pub fn i_08d36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D36C;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $08D36C: asr.w #$8, d0
pub fn i_08d36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D36E;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08D36E: move.b d0, $26(a6)
pub fn i_08d36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D372;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D372: swap d0
pub fn i_08d372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D374;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D374: move.w d0, $22(a6)
pub fn i_08d374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D378;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D378: tst.w $2a(a6)
pub fn i_08d378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D37C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $08D37C: beq.w $8d3a2
pub fn i_08d37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D380;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8D3A2; return 10; }
    8
}
/// $08D380: move.w $24(a6), d0
pub fn i_08d380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D384;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D384: swap d0
pub fn i_08d384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D386;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D386: move.b $27(a6), d0
pub fn i_08d386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D38A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08D38A: asl.w #$8, d0
pub fn i_08d38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D38C;
    cpu.ir = 0xE140;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08D38C: move.w $2a(a6), d1
pub fn i_08d38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D390;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D390: ext.l d1
pub fn i_08d390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D392;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D392: asl.l #$8, d1
pub fn i_08d392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D394;
    cpu.ir = 0xE181;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, true);
    cpu.d[1] = r;
    22
}
/// $08D394: add.l d1, d0
pub fn i_08d394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D396;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $08D396: asr.w #$8, d0
pub fn i_08d396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D398;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08D398: move.b d0, $27(a6)
pub fn i_08d398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D39C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08D39C: swap d0
pub fn i_08d39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D39E;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08D39E: move.w d0, $24(a6)
pub fn i_08d39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3A2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08D3A2: move.w $2c(a6), d0
pub fn i_08d3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3A6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D3A6: add.w d0, $28(a6)
pub fn i_08d3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3AA;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D3AA: move.w $2e(a6), d0
pub fn i_08d3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3AE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08D3AE: add.w d0, $2a(a6)
pub fn i_08d3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3B2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08D3B2: rts
pub fn i_08d3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8D3B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bobber_Tmpl182_08e4e6  ($08E4E6, 24 B, enemies/critters_rings_08e4e6.s)
/// $08E4E6: move.w #$c000, $38(a6)
pub fn i_08e4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E4EC: jsr $8f002(pc)
pub fn i_08e4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E4F0);
    cpu.pc = 0x8F002;
    16
}
/// $08E4F0: jsr $8f010(pc)
pub fn i_08e4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E4F4);
    cpu.pc = 0x8F010;
    16
}
/// $08E4F4: move.w #$12a, d1
pub fn i_08e4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4F8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E4F8: jsr $236e.l
pub fn i_08e4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E4FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E4FE);
    cpu.pc = 0x236E;
    16
}
// ===== Bobber_Descend_08e4fe  ($08E4FE, 70 B, enemies/critters_rings_08e4e6.s)
/// $08E4FE: move.w #$4, $2e(a6)
pub fn i_08e4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E504;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E504: move.w #$6, $70(a6)
pub fn i_08e504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E50A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E50A: move.w #$7, d0
pub fn i_08e50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E50E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E50E: jsr $5ea1c.l
pub fn i_08e50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E514);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E514: add.w d0, $70(a6)
pub fn i_08e514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E518;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E518: lea.l $2f4036.l, a0
pub fn i_08e518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E51E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4036;
    20
}
/// $08E51E: jsr $28cd4.l
pub fn i_08e51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E524;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E524);
    cpu.pc = 0x28CD4;
    16
}
/// $08E524: lea.l $8e52a(pc), a1
pub fn i_08e524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E528;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E52A;
    16
}
/// $08E528: move.l a1, (a6)
pub fn i_08e528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E52A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E52A: jsr $2783a.l
pub fn i_08e52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E530;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E530);
    cpu.pc = 0x2783A;
    16
}
/// $08E530: jsr $8d34e(pc)
pub fn i_08e530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E534;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E534);
    cpu.pc = 0x8D34E;
    16
}
/// $08E534: jsr $28d70.l
pub fn i_08e534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E53A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E53A);
    cpu.pc = 0x28D70;
    16
}
/// $08E53A: cmpi.w #$1d0, $24(a6)
pub fn i_08e53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E540;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1D0, Size::Word, false);
    16
}
/// $08E540: blt.w $8e54c
pub fn i_08e540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E544;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8E54C; return 10; }
    8
}
// ===== SetTaskHandler_08e544  ($08E544, 8 B, kernel/task_handlers.c)
/// $08E544: lea.l $8e566(pc), a1
pub fn i_08e544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E548;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E566;
    16
}
/// $08E548: move.l a1, (a6)
pub fn i_08e548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E54A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E54A: rts
pub fn i_08e54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E54C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bobber_DescendEnd_08e54c  ($08E54C, 20 B, enemies/critters_rings_08e4e6.s)
/// $08E54C: subq.w #$1, $70(a6)
pub fn i_08e54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E550;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E550: cmpi.w #$0, $70(a6)
pub fn i_08e550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E556;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E556: bhi.w $8e560
pub fn i_08e556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E55A;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8E560; return 10; }
    8
}
/// $08E55A: lea.l $8e566(pc), a1
pub fn i_08e55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E55E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E566;
    16
}
/// $08E55E: move.l a1, (a6)
pub fn i_08e55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E560;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08e560  ($08E560, 6 B, thunks/jsr_pc_thunks.c)
/// $08E560: jsr $8efb0(pc)
pub fn i_08e560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E564;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E564);
    cpu.pc = 0x8EFB0;
    16
}
/// $08E564: rts
pub fn i_08e564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E566;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bobber_Ascend_08e566  ($08E566, 70 B, enemies/critters_rings_08e4e6.s)
/// $08E566: move.w #$fffc, $2e(a6)
pub fn i_08e566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E56C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E56C: move.w #$6, $70(a6)
pub fn i_08e56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E572;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08E572: move.w #$7, d0
pub fn i_08e572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E576;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08E576: jsr $5ea1c.l
pub fn i_08e576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E57C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E57C);
    cpu.pc = 0x5EA1C;
    16
}
/// $08E57C: add.w d0, $70(a6)
pub fn i_08e57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E580;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E580: lea.l $2f4036.l, a0
pub fn i_08e580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E586;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4036;
    20
}
/// $08E586: jsr $28cd4.l
pub fn i_08e586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E58C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E58C);
    cpu.pc = 0x28CD4;
    16
}
/// $08E58C: lea.l $8e592(pc), a1
pub fn i_08e58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E590;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E592;
    16
}
/// $08E590: move.l a1, (a6)
pub fn i_08e590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E592;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E592: jsr $2783a.l
pub fn i_08e592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E598;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E598);
    cpu.pc = 0x2783A;
    16
}
/// $08E598: jsr $8d34e(pc)
pub fn i_08e598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E59C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E59C);
    cpu.pc = 0x8D34E;
    16
}
/// $08E59C: jsr $28d70.l
pub fn i_08e59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8E5A2);
    cpu.pc = 0x28D70;
    16
}
/// $08E5A2: cmpi.w #$160, $24(a6)
pub fn i_08e5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5A8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x160, Size::Word, false);
    16
}
/// $08E5A8: bge.w $8e5b4
pub fn i_08e5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5AC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8E5B4; return 10; }
    8
}
// ===== SetTaskHandler_08e5ac  ($08E5AC, 8 B, kernel/task_handlers.c)
/// $08E5AC: lea.l $8e4fe(pc), a1
pub fn i_08e5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4FE;
    16
}
/// $08E5B0: move.l a1, (a6)
pub fn i_08e5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08E5B2: rts
pub fn i_08e5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bobber_AscendEnd_08e5b4  ($08E5B4, 20 B, enemies/critters_rings_08e4e6.s)
/// $08E5B4: subq.w #$1, $70(a6)
pub fn i_08e5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5B8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08E5B8: cmpi.w #$0, $70(a6)
pub fn i_08e5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5BE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08E5BE: bhi.w $8e5c8
pub fn i_08e5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5C2;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8E5C8; return 10; }
    8
}
/// $08E5C2: lea.l $8e4fe(pc), a1
pub fn i_08e5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8E4FE;
    16
}
/// $08E5C6: move.l a1, (a6)
pub fn i_08e5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_08e5c8  ($08E5C8, 6 B, thunks/jsr_pc_thunks.c)
/// $08E5C8: jsr $8efb0(pc)
pub fn i_08e5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8E5CC);
    cpu.pc = 0x8EFB0;
    16
}
/// $08E5CC: rts
pub fn i_08e5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8E5CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_GroundKill_08efb0  ($08EFB0, 16 B, enemies/critters_rings_08e4e6.s)
/// $08EFB0: movea.l #$ffffffff, a0
pub fn i_08efb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFB6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08EFB6: jsr $5dd5c.l
pub fn i_08efb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EFBC);
    cpu.pc = 0x5DD5C;
    16
}
/// $08EFBC: bcc.w $8efcc
pub fn i_08efbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFC0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8EFCC; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08efc0  ($08EFC0, 14 B, kernel/jsr5b6_jmp518.c)
/// $08EFC0: jsr $5b6.l
pub fn i_08efc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8EFC6);
    cpu.pc = 0x5B6;
    16
}
/// $08EFC6: jmp $518.l
pub fn i_08efc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFCC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08EFCC: rts
pub fn i_08efcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8EFCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_FacingFromParam_08f002  ($08F002, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F002: andi.b #$1, $98(a6)
pub fn i_08f002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F008;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08F008: move.b $98(a6), $3a(a6)
pub fn i_08f008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F00E;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08F00E: rts
pub fn i_08f00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F010;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Phys_VelXFromParam_08f010  ($08F010, 28 B, enemies/critters_rings_08e4e6.s)
/// $08F010: clr.w d0
pub fn i_08f010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F012;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F012: move.b $99(a6), d0
pub fn i_08f012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F016;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F016: lsl.w #$5, d0
pub fn i_08f016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F018;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F018: move.w d0, $28(a6)
pub fn i_08f018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F01C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F01C: cmpi.b #$0, $3a(a6)
pub fn i_08f01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F022;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08F022: bne.w $8f02a
pub fn i_08f022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F026;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F02A; return 10; }
    8
}
/// $08F026: neg.w $28(a6)
pub fn i_08f026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F02A;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F02A: rts
pub fn i_08f02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F02C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ring_Reset_08f138  ($08F138, 16 B, enemies/critters_rings_08e4e6.s)
/// $08F138: clr.w (a0)
pub fn i_08f138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F13A;
    cpu.ir = 0x4250;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $08F13A: clr.w $2(a0)
pub fn i_08f13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F13E;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F13E: clr.w $4(a0)
pub fn i_08f13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F142;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F142: clr.w $6(a0)
pub fn i_08f142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F146;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F146: rts
pub fn i_08f146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F148;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ring_Compact_08f148  ($08F148, 16 B, enemies/critters_rings_08e4e6.s)
/// $08F148: move.w $4(a0), (a0)
pub fn i_08f148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F14C;
    cpu.ir = 0x30A8;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v3 as u16);
    16
}
/// $08F14C: move.w $2(a0), $4(a0)
pub fn i_08f14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F152;
    cpu.ir = 0x3168;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08F152: clr.w $6(a0)
pub fn i_08f152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F156;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F156: rts
pub fn i_08f156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F158;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rings_InitAll_08f158  ($08F158, 26 B, enemies/critters_rings_08e4e6.s)
/// $08F158: lea.l $10e2f2.l, a0
pub fn i_08f158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F15E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F15E: bsr.b $8f138
pub fn i_08f15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F160;
    cpu.ir = 0x61D8;
    cpu.push32(bus, 0x8F160);
    cpu.pc = 0x8F138;
    18
}
/// $08F160: lea.l $10e33a.l, a0
pub fn i_08f160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F166;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E33A;
    20
}
/// $08F166: bsr.b $8f138
pub fn i_08f166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F168;
    cpu.ir = 0x61D0;
    cpu.push32(bus, 0x8F168);
    cpu.pc = 0x8F138;
    18
}
/// $08F168: lea.l $10e362.l, a0
pub fn i_08f168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F16E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F16E: bsr.b $8f138
pub fn i_08f16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F170;
    cpu.ir = 0x61C8;
    cpu.push32(bus, 0x8F170);
    cpu.pc = 0x8F138;
    18
}
/// $08F170: rts
pub fn i_08f170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F172;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rings_CompactAll_08f172  ($08F172, 26 B, enemies/critters_rings_08e4e6.s)
/// $08F172: lea.l $10e2f2.l, a0
pub fn i_08f172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F178;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F178: bsr.b $8f148
pub fn i_08f178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F17A;
    cpu.ir = 0x61CE;
    cpu.push32(bus, 0x8F17A);
    cpu.pc = 0x8F148;
    18
}
/// $08F17A: lea.l $10e33a.l, a0
pub fn i_08f17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F180;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E33A;
    20
}
/// $08F180: bsr.b $8f148
pub fn i_08f180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F182;
    cpu.ir = 0x61C6;
    cpu.push32(bus, 0x8F182);
    cpu.pc = 0x8F148;
    18
}
/// $08F182: lea.l $10e362.l, a0
pub fn i_08f182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F188;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F188: bsr.b $8f148
pub fn i_08f188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F18A;
    cpu.ir = 0x61BE;
    cpu.push32(bus, 0x8F18A);
    cpu.pc = 0x8F148;
    18
}
/// $08F18A: rts
pub fn i_08f18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F18C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ZoneRing_HitTest_08f2a0  ($08F2A0, 78 B, enemies/critters_rings_08e4e6.s)
/// $08F2A0: move.w $22(a6), d0
pub fn i_08f2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2A4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F2A4: add.w $106f50.l, d0
pub fn i_08f2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2AA;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F2AA: move.w $24(a6), d1
pub fn i_08f2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2AE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F2AE: subi.w #$200, d1
pub fn i_08f2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2B2;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F2B2: neg.w d1
pub fn i_08f2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2B4;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F2B4: add.w $106f54.l, d1
pub fn i_08f2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2BA;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08F2BA: lea.l $10e2f2.l, a0
pub fn i_08f2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2F2;
    20
}
/// $08F2C0: lea.l $8(a0), a1
pub fn i_08f2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2C4;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F2C4: move.w (a0), d7
pub fn i_08f2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2C6;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F2C6: cmp.w $4(a0), d7
pub fn i_08f2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2CA;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F2CA: beq.w $8f302
pub fn i_08f2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F302; return 10; }
    8
}
/// $08F2CE: cmp.w (a1, d7.w), d0
pub fn i_08f2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2D2;
    cpu.ir = 0xB071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $08F2D2: bcs.w $8f2f4
pub fn i_08f2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2D6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F2F4; return 10; }
    8
}
/// $08F2D6: cmp.w $4(a1, d7.w), d0
pub fn i_08f2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2DA;
    cpu.ir = 0xB071;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $08F2DA: bhi.w $8f2f4
pub fn i_08f2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2DE;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8F2F4; return 10; }
    8
}
/// $08F2DE: cmp.w $2(a1, d7.w), d1
pub fn i_08f2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2E2;
    cpu.ir = 0xB271;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $08F2E2: bcs.w $8f2f4
pub fn i_08f2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2E6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F2F4; return 10; }
    8
}
/// $08F2E6: cmp.w $6(a1, d7.w), d1
pub fn i_08f2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2EA;
    cpu.ir = 0xB271;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $08F2EA: bhi.w $8f2f4
pub fn i_08f2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2EE;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8F2F4; return 10; }
    8
}
// ===== SetC_08f2ee  ($08F2EE, 6 B, thunks/ccr_helpers.c)
/// $08F2EE: ori.b #$1, ccr
pub fn i_08f2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2F2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F2F2: rts
pub fn i_08f2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ZoneRing_HitTest_Next_08f2f4  ($08F2F4, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F2F4: addq.w #$8, d7
pub fn i_08f2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2F6;
    cpu.ir = 0x5047;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F2F6: cmpi.w #$40, d7
pub fn i_08f2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2FA;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $08F2FA: bcs.w $8f300
pub fn i_08f2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F2FE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F300; return 10; }
    8
}
/// $08F2FE: clr.w d7
pub fn i_08f2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F300;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F300: bra.b $8f2c6
pub fn i_08f300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F302;
    cpu.ir = 0x60C4;
    cpu.pc = 0x8F2C6;
    10
}
// ===== ClearC_08f302  ($08F302, 6 B, thunks/ccr_helpers.c)
/// $08F302: andi.b #$fe, ccr
pub fn i_08f302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F306;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F306: rts
pub fn i_08f306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F308;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PosRing_PushCapped_08F308  ($08F308, 60 B, kernel/pos_ring_push_capped_08f308.s)
/// $08F308: lea.l $10e33a.l, a0
pub fn i_08f308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F30E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E33A;
    20
}
/// $08F30E: lea.l $8(a0), a1
pub fn i_08f30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F312;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F312: adda.w $2(a0), a1
pub fn i_08f312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F316;
    cpu.ir = 0xD2E8;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $08F316: move.w $22(a6), (a1)
pub fn i_08f316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F31A;
    cpu.ir = 0x32AE;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v3 as u16);
    16
}
/// $08F31A: move.w $24(a6), $2(a1)
pub fn i_08f31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F320;
    cpu.ir = 0x336E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08F320: cmpi.w #$4, $6(a0)
pub fn i_08f320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F326;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F326: bcc.w $8f342
pub fn i_08f326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F32A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F342; return 10; }
    8
}
/// $08F32A: move.w $2(a0), d7
pub fn i_08f32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F32E;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F32E: addq.w #$4, d7
pub fn i_08f32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F330;
    cpu.ir = 0x5847;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F330: cmpi.w #$20, d7
pub fn i_08f330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F334;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $08F334: bcs.w $8f33a
pub fn i_08f334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F338;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F33A; return 10; }
    8
}
/// $08F338: clr.w d7
pub fn i_08f338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F33A;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F33A: move.w d7, $2(a0)
pub fn i_08f33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F33E;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F33E: addq.w #$1, $6(a0)
pub fn i_08f33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F342;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08F342: rts
pub fn i_08f342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F344;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PosRing_FindNear_08f344  ($08F344, 72 B, enemies/critters_rings_08e4e6.s)
/// $08F344: lea.l $10e33a.l, a0
pub fn i_08f344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F34A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E33A;
    20
}
/// $08F34A: lea.l $8(a0), a1
pub fn i_08f34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F34E;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F34E: move.w (a0), d7
pub fn i_08f34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F350;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F350: cmp.w $4(a0), d7
pub fn i_08f350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F354;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F354: beq.w $8f3a0
pub fn i_08f354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F358;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F3A0; return 10; }
    8
}
/// $08F358: move.w (a1, d7.w), d0
pub fn i_08f358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F35C;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08F35C: move.w $2(a1, d7.w), d1
pub fn i_08f35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F360;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08F360: sub.w $22(a6), d0
pub fn i_08f360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F364;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F364: smi.b d2
pub fn i_08f364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F366;
    cpu.ir = 0x5BC2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $08F366: sub.w $24(a6), d1
pub fn i_08f366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F36A;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F36A: addi.w #$40, d0
pub fn i_08f36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F36E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F36E: addi.w #$30, d1
pub fn i_08f36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F372;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F372: cmpi.w #$80, d0
pub fn i_08f372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F376;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $08F376: bcc.w $8f392
pub fn i_08f376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F37A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F392; return 10; }
    8
}
/// $08F37A: cmpi.w #$60, d1
pub fn i_08f37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F37E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $08F37E: bcc.w $8f392
pub fn i_08f37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F382;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F392; return 10; }
    8
}
/// $08F382: btst.b #$0, $3a(a6)
pub fn i_08f382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F388;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08F388: seq.b d0
pub fn i_08f388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F38A;
    cpu.ir = 0x57C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $08F38A: eor.b d2, d0
pub fn i_08f38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F38C;
    cpu.ir = 0xB500;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
// ===== SetC_08f38c  ($08F38C, 6 B, thunks/ccr_helpers.c)
/// $08F38C: ori.b #$1, ccr
pub fn i_08f38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F390;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F390: rts
pub fn i_08f390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F392;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PosRing_FindNear_Next_08f392  ($08F392, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F392: addq.w #$4, d7
pub fn i_08f392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F394;
    cpu.ir = 0x5847;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F394: cmpi.w #$20, d7
pub fn i_08f394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F398;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $08F398: bcs.w $8f39e
pub fn i_08f398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F39C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F39E; return 10; }
    8
}
/// $08F39C: clr.w d7
pub fn i_08f39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F39E;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F39E: bra.b $8f350
pub fn i_08f39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3A0;
    cpu.ir = 0x60B0;
    cpu.pc = 0x8F350;
    10
}
// ===== ClearC_08f3a0  ($08F3A0, 6 B, thunks/ccr_helpers.c)
/// $08F3A0: andi.b #$fe, ccr
pub fn i_08f3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3A4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F3A4: rts
pub fn i_08f3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F3A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_FindPending_08f470  ($08F470, 108 B, enemies/critters_rings_08e4e6.s)
/// $08F470: lea.l $10e362.l, a0
pub fn i_08f470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F476;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F476: lea.l $8(a0), a1
pub fn i_08f476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F47A;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F47A: move.w (a0), d7
pub fn i_08f47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F47C;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F47C: cmpa.l #$100440, a6
pub fn i_08f47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F482;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F482: beq.w $8f4bc
pub fn i_08f482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F486;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4BC; return 10; }
    8
}
/// $08F486: cmpa.l #$1004e0, a6
pub fn i_08f486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F48C;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F48C: beq.w $8f4bc
pub fn i_08f48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F490;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4BC; return 10; }
    8
}
/// $08F490: cmp.w $4(a0), d7
pub fn i_08f490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F494;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F494: beq.w $8f4b8
pub fn i_08f494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F498;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4B8; return 10; }
    8
}
/// $08F498: tst.b $4(a1, d7.w)
pub fn i_08f498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F49C;
    cpu.ir = 0x4A31;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $08F49C: bpl.w $8f4aa
pub fn i_08f49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4A0;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8F4AA; return 10; }
    8
}
/// $08F4A0: cmpi.b #$fe, $5(a1, d7.w)
pub fn i_08f4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4A6;
    cpu.ir = 0xC31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    18
}
/// $08F4A6: beq.w $8f4e2
pub fn i_08f4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4E2; return 10; }
    8
}
/// $08F4AA: addq.w #$6, d7
pub fn i_08f4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4AC;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F4AC: cmpi.w #$30, d7
pub fn i_08f4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4B0;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F4B0: bcs.w $8f4b6
pub fn i_08f4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4B4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F4B6; return 10; }
    8
}
/// $08F4B4: clr.w d7
pub fn i_08f4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4B6;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F4B6: bra.b $8f490
pub fn i_08f4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4B8;
    cpu.ir = 0x60D8;
    cpu.pc = 0x8F490;
    10
}
/// $08F4B8: bra.w $8f4dc
pub fn i_08f4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8F4DC;
    10
}
/// $08F4BC: cmp.w $4(a0), d7
pub fn i_08f4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4C0;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F4C0: beq.w $8f4dc
pub fn i_08f4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4DC; return 10; }
    8
}
/// $08F4C4: cmpi.b #$fe, $5(a1, d7.w)
pub fn i_08f4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4CA;
    cpu.ir = 0xC31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    18
}
/// $08F4CA: beq.w $8f4e2
pub fn i_08f4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F4E2; return 10; }
    8
}
/// $08F4CE: addq.w #$6, d7
pub fn i_08f4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4D0;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F4D0: cmpi.w #$30, d7
pub fn i_08f4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4D4;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F4D4: bcs.w $8f4da
pub fn i_08f4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4D8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F4DA; return 10; }
    8
}
/// $08F4D8: clr.w d7
pub fn i_08f4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4DA;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F4DA: bra.b $8f4bc
pub fn i_08f4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4DC;
    cpu.ir = 0x60E0;
    cpu.pc = 0x8F4BC;
    10
}
// ===== ClearC_08f4dc  ($08F4DC, 6 B, thunks/ccr_helpers.c)
/// $08F4DC: andi.b #$fe, ccr
pub fn i_08f4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4E0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F4E0: rts
pub fn i_08f4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_InRange_08f4e2  ($08F4E2, 56 B, enemies/critters_rings_08e4e6.s)
/// $08F4E2: lea.l (a1, d7.w), a2
pub fn i_08f4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4E6;
    cpu.ir = 0x45F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[2] = ad1;
    18
}
/// $08F4E6: move.w (a2), d0
pub fn i_08f4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4E8;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F4E8: move.w $2(a2), d1
pub fn i_08f4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4EC;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F4EC: moveq #$ff, d2
pub fn i_08f4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4EE;
    cpu.ir = 0x74FF;
    cpu.d[2] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F4EE: move.b $4(a2), d2
pub fn i_08f4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4F2;
    cpu.ir = 0x142A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F4F2: move.w $22(a6), d4
pub fn i_08f4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4F6;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F4F6: sub.w d0, d4
pub fn i_08f4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4F8;
    cpu.ir = 0x9840;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F4F8: addi.w #$18, d4
pub fn i_08f4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F4FC;
    cpu.ir = 0x644;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F4FC: cmpi.w #$30, d4
pub fn i_08f4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F500;
    cpu.ir = 0xC44;
    let v1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F500: bcc.w $8f51a
pub fn i_08f500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F504;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F51A; return 10; }
    8
}
/// $08F504: move.w $24(a6), d4
pub fn i_08f504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F508;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F508: sub.w d1, d4
pub fn i_08f508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F50A;
    cpu.ir = 0x9841;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F50A: addi.w #$20, d4
pub fn i_08f50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F50E;
    cpu.ir = 0x644;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F50E: cmpi.w #$40, d4
pub fn i_08f50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F512;
    cpu.ir = 0xC44;
    let v1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $08F512: bcc.w $8f51a
pub fn i_08f512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F516;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F51A; return 10; }
    8
}
/// $08F516: andi.w #$ff, d2
pub fn i_08f516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F51A;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
// ===== SetC_08f51a  ($08F51A, 6 B, thunks/ccr_helpers.c)
/// $08F51A: ori.b #$1, ccr
pub fn i_08f51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F51E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F51E: rts
pub fn i_08f51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F520;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimById_08f520  ($08F520, 84 B, enemies/critters_rings_08e4e6.s)
/// $08F520: lea.l $10e362.l, a0
pub fn i_08f520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F526;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E362;
    20
}
/// $08F526: lea.l $8(a0), a1
pub fn i_08f526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F52A;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    cpu.a[1] = ad1;
    16
}
/// $08F52A: move.w (a0), d7
pub fn i_08f52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F52C;
    cpu.ir = 0x3E10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F52C: move.b #$0, d3
pub fn i_08f52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F530;
    cpu.ir = 0x163C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F530: cmpa.l #$100440, a6
pub fn i_08f530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F536;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F536: beq.w $8f54c
pub fn i_08f536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F53A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F54C; return 10; }
    8
}
/// $08F53A: move.b #$1, d3
pub fn i_08f53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F53E;
    cpu.ir = 0x163C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F53E: cmpa.l #$1004e0, a6
pub fn i_08f53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F544;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08F544: beq.w $8f54c
pub fn i_08f544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F548;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F54C; return 10; }
    8
}
/// $08F548: move.b #$2, d3
pub fn i_08f548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F54C;
    cpu.ir = 0x163C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F54C: cmp.w $4(a0), d7
pub fn i_08f54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F550;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F550: beq.w $8f574
pub fn i_08f550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F554;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F574; return 10; }
    8
}
/// $08F554: cmp.b $4(a1, d7.w), d2
pub fn i_08f554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F558;
    cpu.ir = 0xB431;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F558: bne.w $8f566
pub fn i_08f558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F55C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F566; return 10; }
    8
}
/// $08F55C: cmpi.b #$fe, $5(a1, d7.w)
pub fn i_08f55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F562;
    cpu.ir = 0xC31;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    18
}
/// $08F562: beq.w $8f57a
pub fn i_08f562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F566;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F57A; return 10; }
    8
}
/// $08F566: addq.w #$6, d7
pub fn i_08f566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F568;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F568: cmpi.w #$30, d7
pub fn i_08f568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F56C;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F56C: bcs.w $8f572
pub fn i_08f56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F570;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F572; return 10; }
    8
}
/// $08F570: clr.w d7
pub fn i_08f570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F572;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F572: bra.b $8f54c
pub fn i_08f572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F574;
    cpu.ir = 0x60D8;
    cpu.pc = 0x8F54C;
    10
}
// ===== ClearC_08f574  ($08F574, 6 B, thunks/ccr_helpers.c)
/// $08F574: andi.b #$fe, ccr
pub fn i_08f574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F578;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F578: rts
pub fn i_08f578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F57A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimById_Mark_08f57a  ($08F57A, 30 B, enemies/critters_rings_08e4e6.s)
/// $08F57A: move.b #$ff, $5(a1, d7.w)
pub fn i_08f57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F580;
    cpu.ir = 0x13BC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F580: move.w $4(a0), d7
pub fn i_08f580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F584;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F584: cmp.w $2(a0), d7
pub fn i_08f584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F588;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $08F588: beq.w $8f5ac
pub fn i_08f588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F58C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F5AC; return 10; }
    8
}
/// $08F58C: cmp.b $4(a1, d7.w), d2
pub fn i_08f58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F590;
    cpu.ir = 0xB431;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $08F590: bne.w $8f59e
pub fn i_08f590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F594;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F59E; return 10; }
    8
}
/// $08F594: move.b d3, $5(a1, d7.w)
pub fn i_08f594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F598;
    cpu.ir = 0x1383;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
// ===== SetC_08f598  ($08F598, 6 B, thunks/ccr_helpers.c)
/// $08F598: ori.b #$1, ccr
pub fn i_08f598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F59C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F59C: rts
pub fn i_08f59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F59E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TargetRing_ClaimById_Next_08f59e  ($08F59E, 14 B, enemies/critters_rings_08e4e6.s)
/// $08F59E: addq.w #$6, d7
pub fn i_08f59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5A0;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F5A0: cmpi.w #$30, d7
pub fn i_08f5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5A4;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F5A4: bcs.w $8f5aa
pub fn i_08f5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5A8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F5AA; return 10; }
    8
}
/// $08F5A8: clr.w d7
pub fn i_08f5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5AA;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F5AA: bra.b $8f584
pub fn i_08f5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5AC;
    cpu.ir = 0x60D8;
    cpu.pc = 0x8F584;
    10
}
// ===== TargetRing_ClaimById_Append_08f5ac  ($08F5AC, 42 B, enemies/critters_rings_08e4e6.s)
/// $08F5AC: move.b d2, $4(a1, d7.w)
pub fn i_08f5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5B0;
    cpu.ir = 0x1382;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F5B0: move.b d3, $5(a1, d7.w)
pub fn i_08f5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5B4;
    cpu.ir = 0x1383;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F5B4: cmpi.w #$4, $6(a0)
pub fn i_08f5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5BA;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $08F5BA: bcc.w $8f5d6
pub fn i_08f5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F5D6; return 10; }
    8
}
/// $08F5BE: move.w $2(a0), d7
pub fn i_08f5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5C2;
    cpu.ir = 0x3E28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F5C2: addq.w #$6, d7
pub fn i_08f5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5C4;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F5C4: cmpi.w #$30, d7
pub fn i_08f5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5C8;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $08F5C8: bcs.w $8f5ce
pub fn i_08f5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5CC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F5CE; return 10; }
    8
}
/// $08F5CC: clr.w d7
pub fn i_08f5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5CE;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08F5CE: move.w d7, $2(a0)
pub fn i_08f5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5D2;
    cpu.ir = 0x3147;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F5D2: addq.w #$1, $6(a0)
pub fn i_08f5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5D6;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x6);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetC_08f5d6  ($08F5D6, 6 B, thunks/ccr_helpers.c)
/// $08F5D6: ori.b #$1, ccr
pub fn i_08f5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5DA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F5DA: rts
pub fn i_08f5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F5DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_MaskF0F0_08f6d2  ($08F6D2, 8 B, mission/gameover_continue_08f6d2.s)
/// $08F6D2: move.w #$f0f0, $96(a6)
pub fn i_08f6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0F0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F6D8: rts
pub fn i_08f6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_MaskCurF0_08f6da  ($08F6DA, 24 B, mission/gameover_continue_08f6d2.s)
/// $08F6DA: move.b $106f28.l, d0
pub fn i_08f6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6E0;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F6E0: not.b d0
pub fn i_08f6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6E2;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F6E2: andi.w #$1, d0
pub fn i_08f6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6E6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F6E6: addi.w #$96, d0
pub fn i_08f6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6EA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F6EA: move.b #$f0, (a6, d0.w)
pub fn i_08f6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6F0;
    cpu.ir = 0x1DBC;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $08F6F0: rts
pub fn i_08f6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F6F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_TestMaskCur_08f714  ($08F714, 24 B, mission/gameover_continue_08f6d2.s)
/// $08F714: move.b $106f28.l, d0
pub fn i_08f714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F71A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F71A: andi.w #$1, d0
pub fn i_08f71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F71E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F71E: addi.w #$96, d0
pub fn i_08f71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F722;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F722: move.b (a6, d0.w), d0
pub fn i_08f722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F726;
    cpu.ir = 0x1036;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $08F726: and.b d0, d1
pub fn i_08f726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F728;
    cpu.ir = 0xC200;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $08F728: beq.w $8f732
pub fn i_08f728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F72C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F732; return 10; }
    8
}
// ===== SetXN_08f72c  ($08F72C, 6 B, thunks/ccr_helpers.c)
/// $08F72C: ori.b #$11, ccr
pub fn i_08f72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F730;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08F730: rts
pub fn i_08f730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F732;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_08f732  ($08F732, 6 B, thunks/ccr_helpers.c)
/// $08F732: andi.b #$ee, ccr
pub fn i_08f732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F736;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08F736: rts
pub fn i_08f736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F738;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anchor_GetWorldPos_08f738  ($08F738, 54 B, mission/gameover_continue_08f6d2.s)
/// $08F738: move.l $70(a0), d1
pub fn i_08f738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F73C;
    cpu.ir = 0x2228;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08F73C: bmi.w $8f774
pub fn i_08f73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F740;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x8F774; return 10; }
    8
}
/// $08F740: movea.l d1, a1
pub fn i_08f740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F742;
    cpu.ir = 0x2241;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $08F742: movea.l (a1), a2
pub fn i_08f742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F744;
    cpu.ir = 0x2451;
    let v2 = bus.read32(cpu.a[1]);
    cpu.a[2] = v2;
    12
}
/// $08F744: andi.w #$ff, d0
pub fn i_08f744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F748;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F748: add.w d0, d0
pub fn i_08f748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F74A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F74A: add.w d0, d0
pub fn i_08f74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F74C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08F74C: lea.l (a2, d0.w), a2
pub fn i_08f74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F750;
    cpu.ir = 0x45F2;
    let ad1 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[2] = ad1;
    18
}
/// $08F750: move.w (a2), d0
pub fn i_08f750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F752;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $08F752: move.w $2(a2), d1
pub fn i_08f752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F756;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08F756: move.w d0, d2
pub fn i_08f756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F758;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F758: or.w d1, d2
pub fn i_08f758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F75A;
    cpu.ir = 0x8441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 | cpu.d[2]) & 0xFFFF;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $08F75A: beq.w $8f774
pub fn i_08f75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F75E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F774; return 10; }
    8
}
/// $08F75E: add.w $22(a0), d0
pub fn i_08f75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F762;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F762: add.w $24(a0), d1
pub fn i_08f762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F766;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F766: add.w $4(a1), d0
pub fn i_08f766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F76A;
    cpu.ir = 0xD069;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08F76A: add.w $6(a1), d1
pub fn i_08f76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F76E;
    cpu.ir = 0xD269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
// ===== SetC_08f76e  ($08F76E, 6 B, thunks/ccr_helpers.c)
/// $08F76E: ori.b #$1, ccr
pub fn i_08f76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F772;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $08F772: rts
pub fn i_08f772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F774;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_08f774  ($08F774, 6 B, thunks/ccr_helpers.c)
/// $08F774: andi.b #$fe, ccr
pub fn i_08f774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F778;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $08F778: rts
pub fn i_08f778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F77A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_FoldMask_08f79c  ($08F79C, 16 B, mission/gameover_continue_08f6d2.s)
/// $08F79C: move.b $96(a0), d0
pub fn i_08f79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7A0;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x96);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08F7A0: or.b $97(a0), d0
pub fn i_08f7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7A4;
    cpu.ir = 0x8028;
    let ad2 = cpu.a[0].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08F7A4: move.b d0, d1
pub fn i_08f7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7A6;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F7A6: lsr.b #$4, d1
pub fn i_08f7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7A8;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $08F7A8: or.b d1, d0
pub fn i_08f7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7AA;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $08F7AA: rts
pub fn i_08f7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_Resolve_08f7ac  ($08F7AC, 34 B, mission/gameover_continue_08f6d2.s)
/// $08F7AC: move.l a0, d0
pub fn i_08f7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7AE;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F7AE: beq.w $8f7cc
pub fn i_08f7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F7CC; return 10; }
    8
}
/// $08F7B2: jsr $2ac0e.l
pub fn i_08f7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F7B8);
    cpu.pc = 0x2AC0E;
    16
}
/// $08F7B8: bcc.w $8f7c6
pub fn i_08f7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8F7C6; return 10; }
    8
}
/// $08F7BC: lea.l $100580.l, a0
pub fn i_08f7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $08F7C2: bra.w $8f7cc
pub fn i_08f7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x8F7CC;
    10
}
/// $08F7C6: movea.l #$0, a0
pub fn i_08f7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7CC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x0;
    cpu.a[0] = v1;
    4
}
/// $08F7CC: rts
pub fn i_08f7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_FindFreeAnchor_08f7ce  ($08F7CE, 88 B, mission/gameover_continue_08f6d2.s)
/// $08F7CE: bsr.b $8f7ac
pub fn i_08f7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7D0;
    cpu.ir = 0x61DC;
    cpu.push32(bus, 0x8F7D0);
    cpu.pc = 0x8F7AC;
    18
}
/// $08F7D0: move.l a0, d0
pub fn i_08f7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7D2;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F7D2: beq.w $8f826
pub fn i_08f7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8F826; return 10; }
    8
}
/// $08F7D6: bsr.b $8f79c
pub fn i_08f7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7D8;
    cpu.ir = 0x61C4;
    cpu.push32(bus, 0x8F7D8);
    cpu.pc = 0x8F79C;
    18
}
/// $08F7D8: move.w d0, d7
pub fn i_08f7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7DA;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08F7DA: btst.b #$0, d7
pub fn i_08f7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7DE;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08F7DE: bne.w $8f7f2
pub fn i_08f7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F7F2; return 10; }
    8
}
/// $08F7E2: move.b #$0, d0
pub fn i_08f7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7E6;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F7E6: bsr.w $8f738
pub fn i_08f7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7EA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8F7EA);
    cpu.pc = 0x8F738;
    18
}
/// $08F7EA: bcs.w $8f7f2
pub fn i_08f7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7EE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F7F2; return 10; }
    8
}
/// $08F7EE: bset.b #$0, d7
pub fn i_08f7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7F2;
    cpu.ir = 0x8C7;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[7] = r2;
    8
}
/// $08F7F2: btst.b #$1, d7
pub fn i_08f7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7F6;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08F7F6: bne.w $8f80a
pub fn i_08f7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7FA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F80A; return 10; }
    8
}
/// $08F7FA: move.b #$1, d0
pub fn i_08f7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F7FE;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F7FE: bsr.w $8f738
pub fn i_08f7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F802;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8F802);
    cpu.pc = 0x8F738;
    18
}
/// $08F802: bcs.w $8f80a
pub fn i_08f802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F806;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F80A; return 10; }
    8
}
/// $08F806: bset.b #$1, d7
pub fn i_08f806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F80A;
    cpu.ir = 0x8C7;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[7] = r2;
    8
}
/// $08F80A: btst.b #$2, d7
pub fn i_08f80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F80E;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08F80E: bne.w $8f822
pub fn i_08f80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F812;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8F822; return 10; }
    8
}
/// $08F812: move.b #$2, d0
pub fn i_08f812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F816;
    cpu.ir = 0x103C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F816: bsr.w $8f738
pub fn i_08f816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F81A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x8F81A);
    cpu.pc = 0x8F738;
    18
}
/// $08F81A: bcs.w $8f822
pub fn i_08f81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F81E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8F822; return 10; }
    8
}
/// $08F81E: bset.b #$2, d7
pub fn i_08f81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F822;
    cpu.ir = 0x8C7;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[7] = r2;
    8
}
/// $08F822: move.b d7, d0
pub fn i_08f822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F824;
    cpu.ir = 0x1007;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F824: rts
pub fn i_08f824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F826;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RetMinus1_0008F826  ($08F826, 4 B, thunks/trivial_returns.c)
/// $08F826: moveq #$ff, d0
pub fn i_08f826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F828;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F828: rts
pub fn i_08f828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F82A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerSlot_ClearP3Cur_08f8e2  ($08F8E2, 28 B, mission/gameover_continue_08f6d2.s)
/// $08F8E2: move.b $106f28.l, d0
pub fn i_08f8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8E8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08F8E8: not.b d0
pub fn i_08f8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8EA;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08F8EA: andi.w #$1, d0
pub fn i_08f8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8EE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F8EE: addi.w #$96, d0
pub fn i_08f8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8F2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x96, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08F8F2: lea.l $100580.l, a0
pub fn i_08f8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8F8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $08F8F8: clr.b (a0, d0.w)
pub fn i_08f8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8FC;
    cpu.ir = 0x4230;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    14
}
/// $08F8FC: rts
pub fn i_08f8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F8FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_Boot_08f91a  ($08F91A, 72 B, mission/gameover_continue_08f6d2.s)
/// $08F91A: move.b #$a, d0
pub fn i_08f91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F91E;
    cpu.ir = 0x103C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08F91E: jsr $43568.l
pub fn i_08f91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F924;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F924);
    cpu.pc = 0x43568;
    16
}
/// $08F924: lea.l $2f4a32.l, a0
pub fn i_08f924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F92A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4A32;
    20
}
/// $08F92A: jsr $2b58.l
pub fn i_08f92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F930);
    cpu.pc = 0x2B58;
    16
}
/// $08F930: move.w #$90, $22(a6)
pub fn i_08f930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F936;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F936: move.w #$1c0, $24(a6)
pub fn i_08f936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F93C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1C0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F93C: move.w #$ff00, $2a(a6)
pub fn i_08f93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F942;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F942: moveq #$0, d0
pub fn i_08f942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F944;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F944: move.b d0, $20(a6)
pub fn i_08f944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F948;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08F948: move.b d0, $21(a6)
pub fn i_08f948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F94C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08F94C: move.w #$e, $82(a6)
pub fn i_08f94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F952;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F952: move.w d0, $86(a6)
pub fn i_08f952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F956;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F956: move.w d0, $84(a6)
pub fn i_08f956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F95A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F95A: moveq #$2, d0
pub fn i_08f95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F95C;
    cpu.ir = 0x7002;
    cpu.d[0] = 0x2;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08F95C: jsr $523b2.l
pub fn i_08f95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F962;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F962);
    cpu.pc = 0x523B2;
    16
}
// ===== SetTaskHandler_08f962  ($08F962, 8 B, kernel/task_handlers.c)
/// $08F962: lea.l $8f96a(pc), a1
pub fn i_08f962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F966;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8F96A;
    16
}
/// $08F966: move.l a1, (a6)
pub fn i_08f966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F968;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08F968: rts
pub fn i_08f968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F96A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_Spawn_08f96a  ($08F96A, 824 B, mission/gameover_continue_08f6d2.s)
/// $08F96A: lea.l $91514(pc), a1
pub fn i_08f96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F96E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91514;
    16
}
/// $08F96E: jsr $4ae.l
pub fn i_08f96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F974;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F974);
    cpu.pc = 0x4AE;
    16
}
/// $08F974: lea.l $90db0(pc), a1
pub fn i_08f974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F978;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90DB0;
    16
}
/// $08F978: jsr $4ae.l
pub fn i_08f978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F97E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F97E);
    cpu.pc = 0x4AE;
    16
}
/// $08F97E: jsr $5dd02.l
pub fn i_08f97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F984;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F984);
    cpu.pc = 0x5DD02;
    16
}
/// $08F984: lea.l $90de2(pc), a1
pub fn i_08f984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F988;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90DE2;
    16
}
/// $08F988: jsr $4ae.l
pub fn i_08f988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F98E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F98E);
    cpu.pc = 0x4AE;
    16
}
/// $08F98E: jsr $5dd02.l
pub fn i_08f98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F994;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F994);
    cpu.pc = 0x5DD02;
    16
}
/// $08F994: lea.l $90f00(pc), a1
pub fn i_08f994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F998;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90F00;
    16
}
/// $08F998: jsr $4ae.l
pub fn i_08f998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F99E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F99E);
    cpu.pc = 0x4AE;
    16
}
/// $08F99E: jsr $5dd02.l
pub fn i_08f99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9A4);
    cpu.pc = 0x5DD02;
    16
}
/// $08F9A4: move.w #$0, $80(a0)
pub fn i_08f9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9AA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F9AA: lea.l $90f68(pc), a1
pub fn i_08f9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90F68;
    16
}
/// $08F9AE: jsr $4ae.l
pub fn i_08f9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9B4);
    cpu.pc = 0x4AE;
    16
}
/// $08F9B4: jsr $5dd02.l
pub fn i_08f9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9BA);
    cpu.pc = 0x5DD02;
    16
}
/// $08F9BA: move.w #$0, $80(a0)
pub fn i_08f9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F9C0: lea.l $90f00(pc), a1
pub fn i_08f9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90F00;
    16
}
/// $08F9C4: jsr $4ae.l
pub fn i_08f9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9CA);
    cpu.pc = 0x4AE;
    16
}
/// $08F9CA: jsr $5dd02.l
pub fn i_08f9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9D0);
    cpu.pc = 0x5DD02;
    16
}
/// $08F9D0: move.w #$1, $80(a0)
pub fn i_08f9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9D6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F9D6: lea.l $90e86(pc), a1
pub fn i_08f9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08F9DA: jsr $4ae.l
pub fn i_08f9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9E0);
    cpu.pc = 0x4AE;
    16
}
/// $08F9E0: jsr $5dd02.l
pub fn i_08f9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9E6);
    cpu.pc = 0x5DD02;
    16
}
/// $08F9E6: move.w #$0, $80(a0)
pub fn i_08f9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9EC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08F9EC: lea.l $90e86(pc), a1
pub fn i_08f9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08F9F0: jsr $4ae.l
pub fn i_08f9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9F6);
    cpu.pc = 0x4AE;
    16
}
/// $08F9F6: jsr $5dd02.l
pub fn i_08f9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8F9FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8F9FC);
    cpu.pc = 0x5DD02;
    16
}
/// $08F9FC: move.w #$1, $80(a0)
pub fn i_08f9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA02;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA02: lea.l $90e86(pc), a1
pub fn i_08fa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA06: jsr $4ae.l
pub fn i_08fa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA0C);
    cpu.pc = 0x4AE;
    16
}
/// $08FA0C: jsr $5dd02.l
pub fn i_08fa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA12);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA12: move.w #$2, $80(a0)
pub fn i_08fa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA18;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA18: lea.l $90e86(pc), a1
pub fn i_08fa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA1C: jsr $4ae.l
pub fn i_08fa1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA22);
    cpu.pc = 0x4AE;
    16
}
/// $08FA22: jsr $5dd02.l
pub fn i_08fa22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA28);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA28: move.w #$3, $80(a0)
pub fn i_08fa28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA2E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA2E: lea.l $90e86(pc), a1
pub fn i_08fa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA32;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA32: jsr $4ae.l
pub fn i_08fa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA38);
    cpu.pc = 0x4AE;
    16
}
/// $08FA38: jsr $5dd02.l
pub fn i_08fa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA3E);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA3E: move.w #$4, $80(a0)
pub fn i_08fa3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA44;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA44: lea.l $90e86(pc), a1
pub fn i_08fa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA48: jsr $4ae.l
pub fn i_08fa48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA4E);
    cpu.pc = 0x4AE;
    16
}
/// $08FA4E: jsr $5dd02.l
pub fn i_08fa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA54);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA54: move.w #$5, $80(a0)
pub fn i_08fa54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA5A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA5A: lea.l $90e86(pc), a1
pub fn i_08fa5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA5E: jsr $4ae.l
pub fn i_08fa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA64);
    cpu.pc = 0x4AE;
    16
}
/// $08FA64: jsr $5dd02.l
pub fn i_08fa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA6A);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA6A: move.w #$6, $80(a0)
pub fn i_08fa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA70;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA70: lea.l $90e86(pc), a1
pub fn i_08fa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA74: jsr $4ae.l
pub fn i_08fa74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA7A);
    cpu.pc = 0x4AE;
    16
}
/// $08FA7A: jsr $5dd02.l
pub fn i_08fa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA80);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA80: move.w #$7, $80(a0)
pub fn i_08fa80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA86;
    cpu.ir = 0x317C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA86: lea.l $90e86(pc), a1
pub fn i_08fa86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FA8A: jsr $4ae.l
pub fn i_08fa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA90);
    cpu.pc = 0x4AE;
    16
}
/// $08FA90: jsr $5dd02.l
pub fn i_08fa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FA96);
    cpu.pc = 0x5DD02;
    16
}
/// $08FA96: move.w #$8, $80(a0)
pub fn i_08fa96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FA9C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FA9C: lea.l $90e86(pc), a1
pub fn i_08fa9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90E86;
    16
}
/// $08FAA0: jsr $4ae.l
pub fn i_08faa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAA6);
    cpu.pc = 0x4AE;
    16
}
/// $08FAA6: jsr $5dd02.l
pub fn i_08faa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAAC);
    cpu.pc = 0x5DD02;
    16
}
/// $08FAAC: move.w #$9, $80(a0)
pub fn i_08faac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAB2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FAB2: lea.l $9076e(pc), a1
pub fn i_08fab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9076E;
    16
}
/// $08FAB6: jsr $4ae.l
pub fn i_08fab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FABC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FABC);
    cpu.pc = 0x4AE;
    16
}
/// $08FABC: jsr $5dd02.l
pub fn i_08fabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAC2);
    cpu.pc = 0x5DD02;
    16
}
/// $08FAC2: lea.l $90728(pc), a1
pub fn i_08fac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90728;
    16
}
/// $08FAC6: jsr $4ae.l
pub fn i_08fac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FACC);
    cpu.pc = 0x4AE;
    16
}
/// $08FACC: jsr $5dd02.l
pub fn i_08facc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAD2);
    cpu.pc = 0x5DD02;
    16
}
/// $08FAD2: lea.l $907b4(pc), a1
pub fn i_08fad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x907B4;
    16
}
/// $08FAD6: jsr $4ae.l
pub fn i_08fad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FADC);
    cpu.pc = 0x4AE;
    16
}
/// $08FADC: jsr $5dd02.l
pub fn i_08fadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAE2);
    cpu.pc = 0x5DD02;
    16
}
/// $08FAE2: lea.l $907fa(pc), a1
pub fn i_08fae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x907FA;
    16
}
/// $08FAE6: jsr $4ae.l
pub fn i_08fae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAEC);
    cpu.pc = 0x4AE;
    16
}
/// $08FAEC: jsr $5dd02.l
pub fn i_08faec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAF2);
    cpu.pc = 0x5DD02;
    16
}
/// $08FAF2: lea.l $90840(pc), a1
pub fn i_08faf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90840;
    16
}
/// $08FAF6: jsr $4ae.l
pub fn i_08faf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FAFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FAFC);
    cpu.pc = 0x4AE;
    16
}
/// $08FAFC: jsr $5dd02.l
pub fn i_08fafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB02);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB02: lea.l $9088e(pc), a1
pub fn i_08fb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9088E;
    16
}
/// $08FB06: jsr $4ae.l
pub fn i_08fb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB0C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB0C: jsr $5dd02.l
pub fn i_08fb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB12);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB12: lea.l $908d4(pc), a1
pub fn i_08fb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x908D4;
    16
}
/// $08FB16: jsr $4ae.l
pub fn i_08fb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB1C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB1C: jsr $5dd02.l
pub fn i_08fb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB22);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB22: lea.l $9091a(pc), a1
pub fn i_08fb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9091A;
    16
}
/// $08FB26: jsr $4ae.l
pub fn i_08fb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB2C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB2C: jsr $5dd02.l
pub fn i_08fb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB32);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB32: lea.l $90968(pc), a1
pub fn i_08fb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90968;
    16
}
/// $08FB36: jsr $4ae.l
pub fn i_08fb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB3C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB3C: jsr $5dd02.l
pub fn i_08fb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB42);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB42: lea.l $909ae(pc), a1
pub fn i_08fb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x909AE;
    16
}
/// $08FB46: jsr $4ae.l
pub fn i_08fb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB4C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB4C: jsr $5dd02.l
pub fn i_08fb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB52);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB52: lea.l $909f4(pc), a1
pub fn i_08fb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x909F4;
    16
}
/// $08FB56: jsr $4ae.l
pub fn i_08fb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB5C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB5C: jsr $5dd02.l
pub fn i_08fb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB62);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB62: lea.l $90a3a(pc), a1
pub fn i_08fb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90A3A;
    16
}
/// $08FB66: jsr $4ae.l
pub fn i_08fb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB6C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB6C: jsr $5dd02.l
pub fn i_08fb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB72);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB72: lea.l $90a80(pc), a1
pub fn i_08fb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90A80;
    16
}
/// $08FB76: jsr $4ae.l
pub fn i_08fb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB7C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB7C: jsr $5dd02.l
pub fn i_08fb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB82);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB82: lea.l $90ac6(pc), a1
pub fn i_08fb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90AC6;
    16
}
/// $08FB86: jsr $4ae.l
pub fn i_08fb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB8C);
    cpu.pc = 0x4AE;
    16
}
/// $08FB8C: jsr $5dd02.l
pub fn i_08fb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB92);
    cpu.pc = 0x5DD02;
    16
}
/// $08FB92: move.w #$10df, d0
pub fn i_08fb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB96;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FB96: jsr $2352.l
pub fn i_08fb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FB9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FB9C);
    cpu.pc = 0x2352;
    16
}
/// $08FB9C: lea.l $8fba2(pc), a1
pub fn i_08fb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FBA2;
    16
}
/// $08FBA0: move.l a1, (a6)
pub fn i_08fba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FBA2: addq.w #$1, $86(a6)
pub fn i_08fba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBA6;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FBA6: move.w $86(a6), d0
pub fn i_08fba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBAA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FBAA: andi.w #$1, d0
pub fn i_08fbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBAE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08FBAE: beq.w $8fc8c
pub fn i_08fbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBB2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8FC8C; return 10; }
    8
}
/// $08FBB2: cmpi.w #$178, $24(a6)
pub fn i_08fbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBB8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x178, Size::Word, false);
    16
}
/// $08FBB8: ble.w $8fc8c
pub fn i_08fbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBBC;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8FC8C; return 10; }
    8
}
/// $08FBBC: jsr $8fcaa(pc)
pub fn i_08fbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8FBC0);
    cpu.pc = 0x8FCAA;
    16
}
/// $08FBC0: cmpi.w #$1a8, $24(a6)
pub fn i_08fbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBC6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1A8, Size::Word, false);
    16
}
/// $08FBC6: bne.w $8fc26
pub fn i_08fbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBCA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FC26; return 10; }
    8
}
/// $08FBCA: cmpi.b #$2, $21(a6)
pub fn i_08fbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBD0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $08FBD0: beq.w $8fc26
pub fn i_08fbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8FC26; return 10; }
    8
}
/// $08FBD4: move.b #$2, $21(a6)
pub fn i_08fbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FBDA: move.w #$10df, d0
pub fn i_08fbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBDE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FBDE: jsr $2222.l
pub fn i_08fbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FBE4);
    cpu.pc = 0x2222;
    16
}
/// $08FBE4: lea.l $905c2(pc), a1
pub fn i_08fbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x905C2;
    16
}
/// $08FBE8: jsr $4ae.l
pub fn i_08fbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FBEE);
    cpu.pc = 0x4AE;
    16
}
/// $08FBEE: jsr $5dd02.l
pub fn i_08fbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FBF4);
    cpu.pc = 0x5DD02;
    16
}
/// $08FBF4: move.w #$20, $82(a0)
pub fn i_08fbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBFA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FBFA: lea.l $9068c(pc), a1
pub fn i_08fbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FBFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9068C;
    16
}
/// $08FBFE: jsr $4ae.l
pub fn i_08fbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC04);
    cpu.pc = 0x4AE;
    16
}
/// $08FC04: jsr $5dd02.l
pub fn i_08fc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC0A);
    cpu.pc = 0x5DD02;
    16
}
/// $08FC0A: move.w #$20, $82(a0)
pub fn i_08fc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC10;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FC10: lea.l $9034e(pc), a1
pub fn i_08fc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9034E;
    16
}
/// $08FC14: jsr $4ae.l
pub fn i_08fc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC1A);
    cpu.pc = 0x4AE;
    16
}
/// $08FC1A: jsr $5dd02.l
pub fn i_08fc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC20);
    cpu.pc = 0x5DD02;
    16
}
/// $08FC20: move.w #$20, $82(a0)
pub fn i_08fc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC26;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FC26: cmpi.w #$190, $24(a6)
pub fn i_08fc26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC2C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x190, Size::Word, false);
    16
}
/// $08FC2C: bne.w $8fc8c
pub fn i_08fc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC30;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FC8C; return 10; }
    8
}
/// $08FC30: cmpi.b #$2, $21(a6)
pub fn i_08fc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC36;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $08FC36: beq.w $8fc8c
pub fn i_08fc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC3A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8FC8C; return 10; }
    8
}
/// $08FC3A: move.b #$2, $21(a6)
pub fn i_08fc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC40;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FC40: move.w #$10df, d0
pub fn i_08fc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC44;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FC44: jsr $2222.l
pub fn i_08fc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC4A);
    cpu.pc = 0x2222;
    16
}
/// $08FC4A: lea.l $905c2(pc), a1
pub fn i_08fc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x905C2;
    16
}
/// $08FC4E: jsr $4ae.l
pub fn i_08fc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC54);
    cpu.pc = 0x4AE;
    16
}
/// $08FC54: jsr $5dd02.l
pub fn i_08fc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC5A);
    cpu.pc = 0x5DD02;
    16
}
/// $08FC5A: move.w #$1e, $82(a0)
pub fn i_08fc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC60;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FC60: lea.l $906e8(pc), a1
pub fn i_08fc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x906E8;
    16
}
/// $08FC64: jsr $4ae.l
pub fn i_08fc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC6A);
    cpu.pc = 0x4AE;
    16
}
/// $08FC6A: jsr $5dd02.l
pub fn i_08fc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC70);
    cpu.pc = 0x5DD02;
    16
}
/// $08FC70: move.w #$1e, $82(a0)
pub fn i_08fc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC76;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FC76: lea.l $904e4(pc), a1
pub fn i_08fc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC7A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x904E4;
    16
}
/// $08FC7A: jsr $4ae.l
pub fn i_08fc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC80);
    cpu.pc = 0x4AE;
    16
}
/// $08FC80: jsr $5dd02.l
pub fn i_08fc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FC86);
    cpu.pc = 0x5DD02;
    16
}
/// $08FC86: move.w #$1e, $82(a0)
pub fn i_08fc86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC8C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FC8C: cmpi.w #$178, $24(a6)
pub fn i_08fc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC92;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x178, Size::Word, false);
    16
}
/// $08FC92: bgt.w $8fca8
pub fn i_08fc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC96;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8FCA8; return 10; }
    8
}
/// $08FC96: move.b #$1, $21(a6)
pub fn i_08fc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FC9C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FC9C: move.w #$10, $82(a6)
pub fn i_08fc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCA2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08fca2  ($08FCA2, 8 B, kernel/task_handlers.c)
/// $08FCA2: lea.l $8fcca(pc), a1
pub fn i_08fca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FCCA;
    16
}
/// $08FCA6: move.l a1, (a6)
pub fn i_08fca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FCA8: rts
pub fn i_08fca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_IntegrateY_08fcaa  ($08FCAA, 32 B, mission/gameover_continue_08f6d2.s)
/// $08FCAA: cmpi.w #$140, $24(a6)
pub fn i_08fcaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCB0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $08FCB0: ble.w $8fcc8
pub fn i_08fcb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCB4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8FCC8; return 10; }
    8
}
/// $08FCB4: move.w $2a(a6), d0
pub fn i_08fcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCB8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FCB8: move.w d0, d1
pub fn i_08fcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCBA;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FCBA: asr.w #$8, d1
pub fn i_08fcba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCBC;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08FCBC: add.b d0, $27(a6)
pub fn i_08fcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCC0;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08FCC0: moveq #$0, d0
pub fn i_08fcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCC2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08FCC2: addx.w d1, d0
pub fn i_08fcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCC4;
    cpu.ir = 0xD141;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let h2 = cpu.d[1] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08FCC4: add.w d0, $24(a6)
pub fn i_08fcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCC8;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FCC8: rts
pub fn i_08fcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_Wait_08fcca  ($08FCCA, 92 B, mission/gameover_continue_08f6d2.s)
/// $08FCCA: jsr $8fcaa(pc)
pub fn i_08fcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCCE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8FCCE);
    cpu.pc = 0x8FCAA;
    16
}
/// $08FCCE: subq.w #$1, $82(a6)
pub fn i_08fcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCD2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FCD2: bne.w $8fd2c
pub fn i_08fcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCD6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FD2C; return 10; }
    8
}
/// $08FCD6: move.w #$10df, d0
pub fn i_08fcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCDA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FCDA: jsr $2222.l
pub fn i_08fcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FCE0);
    cpu.pc = 0x2222;
    16
}
/// $08FCE0: move.b #$2, $21(a6)
pub fn i_08fce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FCE6: moveq #$0, d0
pub fn i_08fce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCE8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08FCE8: moveq #$0, d1
pub fn i_08fce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCEA;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08FCEA: jsr $437da.l
pub fn i_08fcea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FCF0);
    cpu.pc = 0x437DA;
    16
}
/// $08FCF0: move.w #$18, $82(a6)
pub fn i_08fcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCF6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FCF6: lea.l $90630(pc), a1
pub fn i_08fcf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FCFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90630;
    16
}
/// $08FCFA: jsr $4ae.l
pub fn i_08fcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD00);
    cpu.pc = 0x4AE;
    16
}
/// $08FD00: jsr $5dd02.l
pub fn i_08fd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD06);
    cpu.pc = 0x5DD02;
    16
}
/// $08FD06: lea.l $900e4(pc), a1
pub fn i_08fd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x900E4;
    16
}
/// $08FD0A: jsr $4ae.l
pub fn i_08fd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD10);
    cpu.pc = 0x4AE;
    16
}
/// $08FD10: jsr $5dd02.l
pub fn i_08fd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD16);
    cpu.pc = 0x5DD02;
    16
}
/// $08FD16: lea.l $9016c(pc), a1
pub fn i_08fd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9016C;
    16
}
/// $08FD1A: jsr $4ae.l
pub fn i_08fd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD20);
    cpu.pc = 0x4AE;
    16
}
/// $08FD20: jsr $5dd02.l
pub fn i_08fd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD26);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_08fd26  ($08FD26, 8 B, kernel/task_handlers.c)
/// $08FD26: lea.l $8fd2e(pc), a1
pub fn i_08fd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FD2E;
    16
}
/// $08FD2A: move.l a1, (a6)
pub fn i_08fd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FD2C: rts
pub fn i_08fd2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_Wait2_08fd2e  ($08FD2E, 50 B, mission/gameover_continue_08f6d2.s)
/// $08FD2E: jsr $8fcaa(pc)
pub fn i_08fd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8FD32);
    cpu.pc = 0x8FCAA;
    16
}
/// $08FD32: subq.w #$1, $82(a6)
pub fn i_08fd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD36;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FD36: bne.w $8fd66
pub fn i_08fd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FD66; return 10; }
    8
}
/// $08FD3A: move.w #$10df, d0
pub fn i_08fd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD3E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FD3E: jsr $2352.l
pub fn i_08fd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD44);
    cpu.pc = 0x2352;
    16
}
/// $08FD44: move.b #$3, $20(a6)
pub fn i_08fd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD4A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FD4A: clr.b $21(a6)
pub fn i_08fd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD4E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08FD4E: move.w #$1b, $82(a6)
pub fn i_08fd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD54;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FD54: lea.l $106f6c.l, a0
pub fn i_08fd54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD5A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $08FD5A: jsr $51ed6.l
pub fn i_08fd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD60);
    cpu.pc = 0x51ED6;
    16
}
// ===== SetTaskHandler_08fd60  ($08FD60, 8 B, kernel/task_handlers.c)
/// $08FD60: lea.l $8fd68(pc), a1
pub fn i_08fd60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FD68;
    16
}
/// $08FD64: move.l a1, (a6)
pub fn i_08fd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FD66: rts
pub fn i_08fd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_Final_08fd68  ($08FD68, 58 B, mission/gameover_continue_08f6d2.s)
/// $08FD68: jsr $8fcaa(pc)
pub fn i_08fd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8FD6C);
    cpu.pc = 0x8FCAA;
    16
}
/// $08FD6C: cmpi.w #$3, $82(a6)
pub fn i_08fd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD72;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $08FD72: bne.w $8fd84
pub fn i_08fd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FD84; return 10; }
    8
}
/// $08FD76: move.b #$1, $10a2d1.l
pub fn i_08fd76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD7E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $08FD7E: move.b #$4, $21(a6)
pub fn i_08fd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD84;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FD84: subq.w #$1, $82(a6)
pub fn i_08fd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD88;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FD88: bne.w $8fda8
pub fn i_08fd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD8C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FDA8; return 10; }
    8
}
/// $08FD8C: move.w #$10de, d0
pub fn i_08fd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD90;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10DE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FD90: jsr $2352.l
pub fn i_08fd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FD96);
    cpu.pc = 0x2352;
    16
}
/// $08FD96: move.b #$3, $21(a6)
pub fn i_08fd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FD9C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08FD9C: move.w #$10, $82(a6)
pub fn i_08fd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDA2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08fda2  ($08FDA2, 8 B, kernel/task_handlers.c)
/// $08FDA2: lea.l $8fdaa(pc), a1
pub fn i_08fda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FDAA;
    16
}
/// $08FDA6: move.l a1, (a6)
pub fn i_08fda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FDA8: rts
pub fn i_08fda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_WaitCredit_08fdaa  ($08FDAA, 70 B, mission/gameover_continue_08f6d2.s)
/// $08FDAA: tst.b $10a2cf.l
pub fn i_08fdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDB0;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10A2CF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $08FDB0: beq.w $8fdf6
pub fn i_08fdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDB4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8FDF6; return 10; }
    8
}
/// $08FDB4: subq.w #$1, $82(a6)
pub fn i_08fdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDB8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FDB8: bne.w $8fdf6
pub fn i_08fdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDBC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FDF6; return 10; }
    8
}
/// $08FDBC: jsr $5b6.l
pub fn i_08fdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FDC2);
    cpu.pc = 0x5B6;
    16
}
/// $08FDC2: clr.b $20(a6)
pub fn i_08fdc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDC6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08FDC6: lea.l $91514(pc), a1
pub fn i_08fdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x91514;
    16
}
/// $08FDCA: jsr $4ae.l
pub fn i_08fdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FDD0);
    cpu.pc = 0x4AE;
    16
}
/// $08FDD0: lea.l $9004c(pc), a1
pub fn i_08fdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9004C;
    16
}
/// $08FDD4: jsr $4ae.l
pub fn i_08fdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FDDA);
    cpu.pc = 0x4AE;
    16
}
/// $08FDDA: jsr $5dd02.l
pub fn i_08fdda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FDE0);
    cpu.pc = 0x5DD02;
    16
}
/// $08FDE0: lea.l $90cba(pc), a1
pub fn i_08fde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x90CBA;
    16
}
/// $08FDE4: jsr $4ae.l
pub fn i_08fde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FDEA);
    cpu.pc = 0x4AE;
    16
}
/// $08FDEA: jsr $5dd02.l
pub fn i_08fdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FDF0);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_08fdf0  ($08FDF0, 8 B, kernel/task_handlers.c)
/// $08FDF0: lea.l $8fdf8(pc), a1
pub fn i_08fdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FDF8;
    16
}
/// $08FDF4: move.l a1, (a6)
pub fn i_08fdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FDF6: rts
pub fn i_08fdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_Continue_08fdf8  ($08FDF8, 50 B, mission/gameover_continue_08f6d2.s)
/// $08FDF8: cmpi.b #$1, $20(a6)
pub fn i_08fdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FDFE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08FDFE: bne.w $8fe30
pub fn i_08fdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE02;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FE30; return 10; }
    8
}
/// $08FE02: lea.l $8fff6(pc), a1
pub fn i_08fe02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FFF6;
    16
}
/// $08FE06: jsr $4ae.l
pub fn i_08fe06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FE0C);
    cpu.pc = 0x4AE;
    16
}
/// $08FE0C: jsr $5dd02.l
pub fn i_08fe0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FE12);
    cpu.pc = 0x5DD02;
    16
}
/// $08FE12: move.w $22(a6), $88(a6)
pub fn i_08fe12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE18;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08FE18: move.w $24(a6), $8a(a6)
pub fn i_08fe18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE1E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08FE1E: move.w #$11, $82(a6)
pub fn i_08fe1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FE24: move.w #$0, $86(a6)
pub fn i_08fe24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE2A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_08fe2a  ($08FE2A, 8 B, kernel/task_handlers.c)
/// $08FE2A: lea.l $8fe32(pc), a1
pub fn i_08fe2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FE32;
    16
}
/// $08FE2E: move.l a1, (a6)
pub fn i_08fe2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FE30: rts
pub fn i_08fe30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_ContinuePath_08fe32  ($08FE32, 98 B, mission/gameover_continue_08f6d2.s)
/// $08FE32: cmpi.w #$14, $86(a6)
pub fn i_08fe32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE38;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Word, false);
    16
}
/// $08FE38: bge.w $8fe66
pub fn i_08fe38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE3C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8FE66; return 10; }
    8
}
/// $08FE3C: lea.l $2f4bf8.l, a0
pub fn i_08fe3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE42;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4BF8;
    20
}
/// $08FE42: move.w $86(a6), d0
pub fn i_08fe42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE46;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FE46: add.w d0, d0
pub fn i_08fe46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE48;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08FE48: add.w d0, d0
pub fn i_08fe48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE4A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08FE4A: move.w (a0, d0.w), d1
pub fn i_08fe4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE4E;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08FE4E: add.w $88(a6), d1
pub fn i_08fe4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE52;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08FE52: move.w d1, $22(a6)
pub fn i_08fe52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE56;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FE56: move.w $2(a0, d0.w), d1
pub fn i_08fe56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE5A;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08FE5A: add.w $8a(a6), d1
pub fn i_08fe5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE5E;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08FE5E: move.w d1, $24(a6)
pub fn i_08fe5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE62;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FE62: addq.w #$1, $86(a6)
pub fn i_08fe62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE66;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FE66: subq.w #$1, $82(a6)
pub fn i_08fe66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE6A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FE6A: bne.w $8fe9a
pub fn i_08fe6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE6E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FE9A; return 10; }
    8
}
/// $08FE6E: move.w #$18, $82(a6)
pub fn i_08fe6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE74;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FE74: lea.l $8fecc(pc), a1
pub fn i_08fe74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FECC;
    16
}
/// $08FE78: jsr $4ae.l
pub fn i_08fe78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FE7E);
    cpu.pc = 0x4AE;
    16
}
/// $08FE7E: jsr $5dd02.l
pub fn i_08fe7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FE84);
    cpu.pc = 0x5DD02;
    16
}
/// $08FE84: lea.l $8ff2c(pc), a1
pub fn i_08fe84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FF2C;
    16
}
/// $08FE88: jsr $4ae.l
pub fn i_08fe88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FE8E);
    cpu.pc = 0x4AE;
    16
}
/// $08FE8E: jsr $5dd02.l
pub fn i_08fe8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FE94);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_08fe94  ($08FE94, 8 B, kernel/task_handlers.c)
/// $08FE94: lea.l $8fe9c(pc), a1
pub fn i_08fe94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FE9C;
    16
}
/// $08FE98: move.l a1, (a6)
pub fn i_08fe98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FE9A: rts
pub fn i_08fe9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FE9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_ContinueHold_08fe9c  ($08FE9C, 18 B, mission/gameover_continue_08f6d2.s)
/// $08FE9C: subq.w #$1, $82(a6)
pub fn i_08fe9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEA0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FEA0: bne.w $8feb4
pub fn i_08fea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FEB4; return 10; }
    8
}
/// $08FEA4: move.w #$96, $82(a6)
pub fn i_08fea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEAA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FEAA: jsr $91618(pc)
pub fn i_08feaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8FEAE);
    cpu.pc = 0x91618;
    16
}
// ===== SetTaskHandler_08feae  ($08FEAE, 8 B, kernel/task_handlers.c)
/// $08FEAE: lea.l $8feb6(pc), a1
pub fn i_08feae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FEB6;
    16
}
/// $08FEB2: move.l a1, (a6)
pub fn i_08feb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FEB4: rts
pub fn i_08feb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GameOver_ContinueEnd_08feb6  ($08FEB6, 14 B, mission/gameover_continue_08f6d2.s)
/// $08FEB6: subq.w #$1, $82(a6)
pub fn i_08feb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEBA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FEBA: bne.w $8feca
pub fn i_08feba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEBE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FECA; return 10; }
    8
}
/// $08FEBE: clr.b $106ed2.l
pub fn i_08febe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEC4;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
// ===== SetTaskHandler_08fec4  ($08FEC4, 8 B, kernel/task_handlers.c)
/// $08FEC4: lea.l $8feca(pc), a1
pub fn i_08fec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FECA;
    16
}
/// $08FEC8: move.l a1, (a6)
pub fn i_08fec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FECA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FECA: rts
pub fn i_08feca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FECC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Sprite_Right_08fecc  ($08FECC, 96 B, mission/gameover_continue_08f6d2.s)
/// $08FECC: move.w #$d9, d1
pub fn i_08fecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FED0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FED0: jsr $236e.l
pub fn i_08fed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FED6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FED6);
    cpu.pc = 0x236E;
    16
}
/// $08FED6: move.w #$130, $22(a6)
pub fn i_08fed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEDC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x130;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FEDC: move.w #$180, $24(a6)
pub fn i_08fedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEE2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FEE2: move.w $22(a6), $88(a6)
pub fn i_08fee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEE8;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08FEE8: move.w #$d8, $8c(a6)
pub fn i_08fee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FEEE: move.w #$ffe8, $28(a6)
pub fn i_08feee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEF4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FEF4: move.w #$0, $80(a6)
pub fn i_08fef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FEFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FEFA: move.w #$0, $86(a6)
pub fn i_08fefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF00;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF00: move.w #$d000, d0
pub fn i_08ff00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF04;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FF04: jsr $28134.l
pub fn i_08ff04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FF0A);
    cpu.pc = 0x28134;
    16
}
/// $08FF0A: andi.w #$ffe3, $38(a6)
pub fn i_08ff0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF10;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08FF10: ori.w #$0, $38(a6)
pub fn i_08ff10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF16;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08FF16: lea.l $2f503e.l, a0
pub fn i_08ff16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF1C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F503E;
    20
}
/// $08FF1C: jsr $28cd4.l
pub fn i_08ff1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FF22);
    cpu.pc = 0x28CD4;
    16
}
/// $08FF22: lea.l $8ff88(pc), a1
pub fn i_08ff22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FF88;
    16
}
/// $08FF26: move.l a1, (a6)
pub fn i_08ff26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08FF28: bra.w $8ff88
pub fn i_08ff28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF2C;
    cpu.ir = 0x6000;
    cpu.pc = 0x8FF88;
    10
}
// ===== GO_Sprite_Left_08ff2c  ($08FF2C, 92 B, mission/gameover_continue_08f6d2.s)
/// $08FF2C: move.w #$da, d1
pub fn i_08ff2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF30;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FF30: jsr $236e.l
pub fn i_08ff30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FF36);
    cpu.pc = 0x236E;
    16
}
/// $08FF36: move.w #$20, $22(a6)
pub fn i_08ff36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF3C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF3C: move.w #$138, $24(a6)
pub fn i_08ff3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF42;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x138;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF42: move.w $22(a6), $88(a6)
pub fn i_08ff42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF48;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08FF48: move.w #$ff28, $8c(a6)
pub fn i_08ff48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF4E: move.w #$18, $28(a6)
pub fn i_08ff4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF54;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF54: move.w #$2, $80(a6)
pub fn i_08ff54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF5A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF5A: move.w #$0, $86(a6)
pub fn i_08ff5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF60;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF60: move.w #$e000, d0
pub fn i_08ff60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF64;
    cpu.ir = 0x303C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FF64: jsr $28134.l
pub fn i_08ff64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FF6A);
    cpu.pc = 0x28134;
    16
}
/// $08FF6A: andi.w #$ffe3, $38(a6)
pub fn i_08ff6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF70;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08FF70: ori.w #$0, $38(a6)
pub fn i_08ff70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF76;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08FF76: lea.l $2f504a.l, a0
pub fn i_08ff76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F504A;
    20
}
/// $08FF7C: jsr $28cd4.l
pub fn i_08ff7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FF82);
    cpu.pc = 0x28CD4;
    16
}
/// $08FF82: lea.l $8ff88(pc), a1
pub fn i_08ff82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8FF88;
    16
}
/// $08FF86: move.l a1, (a6)
pub fn i_08ff86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== GO_Sprite_Slide_08ff88  ($08FF88, 102 B, mission/gameover_continue_08f6d2.s)
/// $08FF88: move.w $88(a6), d0
pub fn i_08ff88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF8C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FF8C: add.w $8c(a6), d0
pub fn i_08ff8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF90;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08FF90: move.w d0, $22(a6)
pub fn i_08ff90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF94;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08FF94: tst.w $8c(a6)
pub fn i_08ff94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF98;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $08FF98: beq.w $8ffa8
pub fn i_08ff98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FF9C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8FFA8; return 10; }
    8
}
/// $08FF9C: move.w $28(a6), d0
pub fn i_08ff9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFA0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FFA0: add.w d0, $8c(a6)
pub fn i_08ffa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFA4;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FFA4: bra.w $8ffee
pub fn i_08ffa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x8FFEE;
    10
}
/// $08FFA8: cmpi.w #$14, $86(a6)
pub fn i_08ffa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFAE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Word, false);
    16
}
/// $08FFAE: bge.w $8ffee
pub fn i_08ffae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFB2;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8FFEE; return 10; }
    8
}
/// $08FFB2: tst.w $86(a6)
pub fn i_08ffb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFB6;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $08FFB6: bne.w $8ffcc
pub fn i_08ffb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFBA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8FFCC; return 10; }
    8
}
/// $08FFBA: move.b #$6, $10a2d1.l
pub fn i_08ffba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFC2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $08FFC2: move.w #$10e0, d0
pub fn i_08ffc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFC6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10E0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FFC6: jsr $2352.l
pub fn i_08ffc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FFCC);
    cpu.pc = 0x2352;
    16
}
/// $08FFCC: lea.l $2f4c48.l, a0
pub fn i_08ffcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFD2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F4C48;
    20
}
/// $08FFD2: moveq #$0, d0
pub fn i_08ffd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFD4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08FFD4: move.w $86(a6), d0
pub fn i_08ffd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFD8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FFD8: add.w d0, d0
pub fn i_08ffd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFDA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08FFDA: add.w d0, d0
pub fn i_08ffda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFDC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08FFDC: adda.l d0, a0
pub fn i_08ffdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFDE;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $08FFDE: move.w $80(a6), d0
pub fn i_08ffde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFE2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08FFE2: move.w (a0, d0.w), d1
pub fn i_08ffe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFE6;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08FFE6: add.w d1, $22(a6)
pub fn i_08ffe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFEA;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08FFEA: addq.w #$1, $86(a6)
pub fn i_08ffea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFEE;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== JsrAbsThunk_08ffee  ($08FFEE, 8 B, thunks/jsr_abs_thunks.c)
/// $08FFEE: jsr $28d70.l
pub fn i_08ffee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8FFF4);
    cpu.pc = 0x28D70;
    16
}
/// $08FFF4: rts
pub fn i_08fff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GO_Sprite_Mid_08fff6  ($08FFF6, 78 B, mission/gameover_continue_08f6d2.s)
/// $08FFF6: move.w #$db, d1
pub fn i_08fff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8FFFA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08FFFA: jsr $236e.l
pub fn i_08fffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x90000;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x90000);
    cpu.pc = 0x236E;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x08BC74, i_08bc74),
    (0x08BC7A, i_08bc7a),
    (0x08BC7E, i_08bc7e),
    (0x08BC82, i_08bc82),
    (0x08BC88, i_08bc88),
    (0x08BC8C, i_08bc8c),
    (0x08BC90, i_08bc90),
    (0x08BC94, i_08bc94),
    (0x08BC98, i_08bc98),
    (0x08BC9E, i_08bc9e),
    (0x08BCA2, i_08bca2),
    (0x08BCA8, i_08bca8),
    (0x08BCAE, i_08bcae),
    (0x08BCB4, i_08bcb4),
    (0x08BCB8, i_08bcb8),
    (0x08BCBC, i_08bcbc),
    (0x08BCC2, i_08bcc2),
    (0x08BCC8, i_08bcc8),
    (0x08BCCE, i_08bcce),
    (0x08BCD2, i_08bcd2),
    (0x08BCD8, i_08bcd8),
    (0x08BCDC, i_08bcdc),
    (0x08BCE0, i_08bce0),
    (0x08BCE4, i_08bce4),
    (0x08BCE6, i_08bce6),
    (0x08BCEC, i_08bcec),
    (0x08BCF0, i_08bcf0),
    (0x08BCF4, i_08bcf4),
    (0x08BCFA, i_08bcfa),
    (0x08BCFE, i_08bcfe),
    (0x08BD02, i_08bd02),
    (0x08BD06, i_08bd06),
    (0x08BD0A, i_08bd0a),
    (0x08BD10, i_08bd10),
    (0x08BD14, i_08bd14),
    (0x08BD1A, i_08bd1a),
    (0x08BD22, i_08bd22),
    (0x08BD26, i_08bd26),
    (0x08BD2A, i_08bd2a),
    (0x08BD30, i_08bd30),
    (0x08BD36, i_08bd36),
    (0x08BD3A, i_08bd3a),
    (0x08BD3E, i_08bd3e),
    (0x08BD42, i_08bd42),
    (0x08BD46, i_08bd46),
    (0x08BD4A, i_08bd4a),
    (0x08BD50, i_08bd50),
    (0x08BD54, i_08bd54),
    (0x08BD5A, i_08bd5a),
    (0x08BD5E, i_08bd5e),
    (0x08BD66, i_08bd66),
    (0x08BD6A, i_08bd6a),
    (0x08BD70, i_08bd70),
    (0x08BD74, i_08bd74),
    (0x08BD78, i_08bd78),
    (0x08BD7E, i_08bd7e),
    (0x08BD82, i_08bd82),
    (0x08BD88, i_08bd88),
    (0x08BD8C, i_08bd8c),
    (0x08BD92, i_08bd92),
    (0x08BD98, i_08bd98),
    (0x08BD9E, i_08bd9e),
    (0x08BDA4, i_08bda4),
    (0x08BDAA, i_08bdaa),
    (0x08BDAE, i_08bdae),
    (0x08BDB4, i_08bdb4),
    (0x08BDB8, i_08bdb8),
    (0x08BDC0, i_08bdc0),
    (0x08BDC4, i_08bdc4),
    (0x08BDCC, i_08bdcc),
    (0x08BDD0, i_08bdd0),
    (0x08BDD4, i_08bdd4),
    (0x08BDD8, i_08bdd8),
    (0x08BDDC, i_08bddc),
    (0x08BDE0, i_08bde0),
    (0x08BDE4, i_08bde4),
    (0x08BDE8, i_08bde8),
    (0x08BDEC, i_08bdec),
    (0x08BDF0, i_08bdf0),
    (0x08BDF4, i_08bdf4),
    (0x08BDF8, i_08bdf8),
    (0x08BDFC, i_08bdfc),
    (0x08BE00, i_08be00),
    (0x08BE04, i_08be04),
    (0x08BE08, i_08be08),
    (0x08BE0C, i_08be0c),
    (0x08BE12, i_08be12),
    (0x08BE16, i_08be16),
    (0x08BE1A, i_08be1a),
    (0x08BE1E, i_08be1e),
    (0x08BE22, i_08be22),
    (0x08BE26, i_08be26),
    (0x08BE28, i_08be28),
    (0x08BE2E, i_08be2e),
    (0x08BE32, i_08be32),
    (0x08BE36, i_08be36),
    (0x08BE3C, i_08be3c),
    (0x08BE40, i_08be40),
    (0x08BE44, i_08be44),
    (0x08BE48, i_08be48),
    (0x08BE4C, i_08be4c),
    (0x08BE52, i_08be52),
    (0x08BE56, i_08be56),
    (0x08BE5C, i_08be5c),
    (0x08BE64, i_08be64),
    (0x08BE68, i_08be68),
    (0x08BE6C, i_08be6c),
    (0x08BE72, i_08be72),
    (0x08BE78, i_08be78),
    (0x08BE7C, i_08be7c),
    (0x08BE80, i_08be80),
    (0x08BE84, i_08be84),
    (0x08BE88, i_08be88),
    (0x08BE8C, i_08be8c),
    (0x08BE92, i_08be92),
    (0x08BE96, i_08be96),
    (0x08BE9C, i_08be9c),
    (0x08BEA0, i_08bea0),
    (0x08BEA4, i_08bea4),
    (0x08BEAA, i_08beaa),
    (0x08BEB0, i_08beb0),
    (0x08BEB6, i_08beb6),
    (0x08BEBA, i_08beba),
    (0x08BEBE, i_08bebe),
    (0x08BEC4, i_08bec4),
    (0x08BEC8, i_08bec8),
    (0x08BECC, i_08becc),
    (0x08BED2, i_08bed2),
    (0x08BED6, i_08bed6),
    (0x08BEDC, i_08bedc),
    (0x08BEE0, i_08bee0),
    (0x08BEE6, i_08bee6),
    (0x08BEEC, i_08beec),
    (0x08BEF2, i_08bef2),
    (0x08BEF8, i_08bef8),
    (0x08BEFE, i_08befe),
    (0x08BF02, i_08bf02),
    (0x08BF08, i_08bf08),
    (0x08BF0C, i_08bf0c),
    (0x08BF14, i_08bf14),
    (0x08BF18, i_08bf18),
    (0x08BF1E, i_08bf1e),
    (0x08BF26, i_08bf26),
    (0x08BF2A, i_08bf2a),
    (0x08BF2E, i_08bf2e),
    (0x08BF36, i_08bf36),
    (0x08BF3C, i_08bf3c),
    (0x08BF40, i_08bf40),
    (0x08BF46, i_08bf46),
    (0x08BF4A, i_08bf4a),
    (0x08BF52, i_08bf52),
    (0x08BF56, i_08bf56),
    (0x08BF5C, i_08bf5c),
    (0x08BF60, i_08bf60),
    (0x08BF66, i_08bf66),
    (0x08BF6A, i_08bf6a),
    (0x08BF70, i_08bf70),
    (0x08BF74, i_08bf74),
    (0x08BF7C, i_08bf7c),
    (0x08BF80, i_08bf80),
    (0x08BF84, i_08bf84),
    (0x08BF88, i_08bf88),
    (0x08BF8C, i_08bf8c),
    (0x08BF90, i_08bf90),
    (0x08BF94, i_08bf94),
    (0x08C008, i_08c008),
    (0x08C00E, i_08c00e),
    (0x08C012, i_08c012),
    (0x08C018, i_08c018),
    (0x08C01C, i_08c01c),
    (0x08C022, i_08c022),
    (0x08C028, i_08c028),
    (0x08C02E, i_08c02e),
    (0x08C034, i_08c034),
    (0x08C03A, i_08c03a),
    (0x08C03E, i_08c03e),
    (0x08C044, i_08c044),
    (0x08C04A, i_08c04a),
    (0x08C04E, i_08c04e),
    (0x08C054, i_08c054),
    (0x08C05A, i_08c05a),
    (0x08C060, i_08c060),
    (0x08C068, i_08c068),
    (0x08C06E, i_08c06e),
    (0x08C072, i_08c072),
    (0x08C07A, i_08c07a),
    (0x08C07C, i_08c07c),
    (0x08C082, i_08c082),
    (0x08C086, i_08c086),
    (0x08C08C, i_08c08c),
    (0x08C092, i_08c092),
    (0x08C098, i_08c098),
    (0x08C09E, i_08c09e),
    (0x08C0A4, i_08c0a4),
    (0x08C0AA, i_08c0aa),
    (0x08C0B0, i_08c0b0),
    (0x08C0B6, i_08c0b6),
    (0x08C0BA, i_08c0ba),
    (0x08C0C0, i_08c0c0),
    (0x08C0C4, i_08c0c4),
    (0x08C0CA, i_08c0ca),
    (0x08C0CE, i_08c0ce),
    (0x08C0D4, i_08c0d4),
    (0x08C0D8, i_08c0d8),
    (0x08C0DE, i_08c0de),
    (0x08C0E2, i_08c0e2),
    (0x08C0E8, i_08c0e8),
    (0x08C0EC, i_08c0ec),
    (0x08C0F2, i_08c0f2),
    (0x08C0F8, i_08c0f8),
    (0x08C0FE, i_08c0fe),
    (0x08C104, i_08c104),
    (0x08C108, i_08c108),
    (0x08C10A, i_08c10a),
    (0x08C110, i_08c110),
    (0x08C114, i_08c114),
    (0x08C116, i_08c116),
    (0x08C11A, i_08c11a),
    (0x08C11E, i_08c11e),
    (0x08C122, i_08c122),
    (0x08C126, i_08c126),
    (0x08C12A, i_08c12a),
    (0x08C12E, i_08c12e),
    (0x08C134, i_08c134),
    (0x08C138, i_08c138),
    (0x08C13E, i_08c13e),
    (0x08C144, i_08c144),
    (0x08C148, i_08c148),
    (0x08C14A, i_08c14a),
    (0x08C150, i_08c150),
    (0x08C156, i_08c156),
    (0x08C15C, i_08c15c),
    (0x08C15E, i_08c15e),
    (0x08C164, i_08c164),
    (0x08C168, i_08c168),
    (0x08C16A, i_08c16a),
    (0x08C16E, i_08c16e),
    (0x08C172, i_08c172),
    (0x08C176, i_08c176),
    (0x08C17A, i_08c17a),
    (0x08C17E, i_08c17e),
    (0x08C182, i_08c182),
    (0x08C188, i_08c188),
    (0x08C18C, i_08c18c),
    (0x08C192, i_08c192),
    (0x08C196, i_08c196),
    (0x08C198, i_08c198),
    (0x08C19E, i_08c19e),
    (0x08C1A4, i_08c1a4),
    (0x08C1AA, i_08c1aa),
    (0x08C1AC, i_08c1ac),
    (0x08C1B0, i_08c1b0),
    (0x08C1B4, i_08c1b4),
    (0x08C1B8, i_08c1b8),
    (0x08C1BC, i_08c1bc),
    (0x08C1C0, i_08c1c0),
    (0x08C1C4, i_08c1c4),
    (0x08C1CA, i_08c1ca),
    (0x08C1D0, i_08c1d0),
    (0x08C1D4, i_08c1d4),
    (0x08C1D6, i_08c1d6),
    (0x08C1DC, i_08c1dc),
    (0x08C1E2, i_08c1e2),
    (0x08C1E8, i_08c1e8),
    (0x08C2B8, i_08c2b8),
    (0x08C2BE, i_08c2be),
    (0x08C2C4, i_08c2c4),
    (0x08C2C8, i_08c2c8),
    (0x08C2CE, i_08c2ce),
    (0x08C2D4, i_08c2d4),
    (0x08C2DA, i_08c2da),
    (0x08C2E0, i_08c2e0),
    (0x08C2E6, i_08c2e6),
    (0x08C2EA, i_08c2ea),
    (0x08C2EE, i_08c2ee),
    (0x08C2F0, i_08c2f0),
    (0x08C2F4, i_08c2f4),
    (0x08C2F8, i_08c2f8),
    (0x08C2FE, i_08c2fe),
    (0x08C304, i_08c304),
    (0x08C306, i_08c306),
    (0x08C30A, i_08c30a),
    (0x08C310, i_08c310),
    (0x08C314, i_08c314),
    (0x08C31A, i_08c31a),
    (0x08C320, i_08c320),
    (0x08C322, i_08c322),
    (0x08C328, i_08c328),
    (0x08C32E, i_08c32e),
    (0x08C332, i_08c332),
    (0x08C338, i_08c338),
    (0x08C33E, i_08c33e),
    (0x08C344, i_08c344),
    (0x08C34A, i_08c34a),
    (0x08C350, i_08c350),
    (0x08C354, i_08c354),
    (0x08C358, i_08c358),
    (0x08C35E, i_08c35e),
    (0x08C364, i_08c364),
    (0x08C36A, i_08c36a),
    (0x08C370, i_08c370),
    (0x08C374, i_08c374),
    (0x08C376, i_08c376),
    (0x08C37C, i_08c37c),
    (0x08C37E, i_08c37e),
    (0x08C384, i_08c384),
    (0x08C38A, i_08c38a),
    (0x08C38E, i_08c38e),
    (0x08C394, i_08c394),
    (0x08C39A, i_08c39a),
    (0x08C3A0, i_08c3a0),
    (0x08C3A6, i_08c3a6),
    (0x08C3AC, i_08c3ac),
    (0x08C3B0, i_08c3b0),
    (0x08C3B4, i_08c3b4),
    (0x08C3BA, i_08c3ba),
    (0x08C3C0, i_08c3c0),
    (0x08C3C6, i_08c3c6),
    (0x08C3CC, i_08c3cc),
    (0x08C3D0, i_08c3d0),
    (0x08C3D2, i_08c3d2),
    (0x08C3D8, i_08c3d8),
    (0x08C3DA, i_08c3da),
    (0x08C3E0, i_08c3e0),
    (0x08C3E6, i_08c3e6),
    (0x08C3EA, i_08c3ea),
    (0x08C3F0, i_08c3f0),
    (0x08C3F6, i_08c3f6),
    (0x08C3FC, i_08c3fc),
    (0x08C402, i_08c402),
    (0x08C408, i_08c408),
    (0x08C40C, i_08c40c),
    (0x08C410, i_08c410),
    (0x08C416, i_08c416),
    (0x08C41C, i_08c41c),
    (0x08C422, i_08c422),
    (0x08C428, i_08c428),
    (0x08C42C, i_08c42c),
    (0x08C42E, i_08c42e),
    (0x08C434, i_08c434),
    (0x08C436, i_08c436),
    (0x08C43A, i_08c43a),
    (0x08C43E, i_08c43e),
    (0x08C442, i_08c442),
    (0x08C446, i_08c446),
    (0x08C44C, i_08c44c),
    (0x08C452, i_08c452),
    (0x08C458, i_08c458),
    (0x08C45C, i_08c45c),
    (0x08C462, i_08c462),
    (0x08C468, i_08c468),
    (0x08C46E, i_08c46e),
    (0x08C474, i_08c474),
    (0x08C47A, i_08c47a),
    (0x08C47E, i_08c47e),
    (0x08C482, i_08c482),
    (0x08C488, i_08c488),
    (0x08C48E, i_08c48e),
    (0x08C494, i_08c494),
    (0x08C49A, i_08c49a),
    (0x08C49E, i_08c49e),
    (0x08C4A0, i_08c4a0),
    (0x08C4A4, i_08c4a4),
    (0x08C4A8, i_08c4a8),
    (0x08C4AC, i_08c4ac),
    (0x08C4B0, i_08c4b0),
    (0x08C4B6, i_08c4b6),
    (0x08C4BC, i_08c4bc),
    (0x08C4C2, i_08c4c2),
    (0x08C4C8, i_08c4c8),
    (0x08C4CA, i_08c4ca),
    (0x08C4CE, i_08c4ce),
    (0x08C4D2, i_08c4d2),
    (0x08C4D6, i_08c4d6),
    (0x08C4DC, i_08c4dc),
    (0x08C4E0, i_08c4e0),
    (0x08C4E6, i_08c4e6),
    (0x08C4EC, i_08c4ec),
    (0x08C4F2, i_08c4f2),
    (0x08C4F8, i_08c4f8),
    (0x08C4FA, i_08c4fa),
    (0x08C500, i_08c500),
    (0x08C506, i_08c506),
    (0x08C50A, i_08c50a),
    (0x08C510, i_08c510),
    (0x08C516, i_08c516),
    (0x08C51C, i_08c51c),
    (0x08C520, i_08c520),
    (0x08C522, i_08c522),
    (0x08C526, i_08c526),
    (0x08C52C, i_08c52c),
    (0x08C532, i_08c532),
    (0x08C536, i_08c536),
    (0x08C53C, i_08c53c),
    (0x08C542, i_08c542),
    (0x08C548, i_08c548),
    (0x08C54C, i_08c54c),
    (0x08C54E, i_08c54e),
    (0x08C552, i_08c552),
    (0x08C558, i_08c558),
    (0x08C55E, i_08c55e),
    (0x08C562, i_08c562),
    (0x08C568, i_08c568),
    (0x08C56E, i_08c56e),
    (0x08C574, i_08c574),
    (0x08C578, i_08c578),
    (0x08C57A, i_08c57a),
    (0x08C57E, i_08c57e),
    (0x08C584, i_08c584),
    (0x08C58A, i_08c58a),
    (0x08C58E, i_08c58e),
    (0x08C594, i_08c594),
    (0x08C59A, i_08c59a),
    (0x08C5A0, i_08c5a0),
    (0x08C5A4, i_08c5a4),
    (0x08C5A6, i_08c5a6),
    (0x08C5AA, i_08c5aa),
    (0x08C5B0, i_08c5b0),
    (0x08C5B2, i_08c5b2),
    (0x08C5B8, i_08c5b8),
    (0x08C5C0, i_08c5c0),
    (0x08C5C4, i_08c5c4),
    (0x08C5CA, i_08c5ca),
    (0x08C5CE, i_08c5ce),
    (0x08C5D6, i_08c5d6),
    (0x08C5DA, i_08c5da),
    (0x08C5E0, i_08c5e0),
    (0x08C5E4, i_08c5e4),
    (0x08C5E8, i_08c5e8),
    (0x08C5EE, i_08c5ee),
    (0x08C5F2, i_08c5f2),
    (0x08C5F6, i_08c5f6),
    (0x08C5FC, i_08c5fc),
    (0x08C600, i_08c600),
    (0x08C604, i_08c604),
    (0x08C60A, i_08c60a),
    (0x08C60E, i_08c60e),
    (0x08C614, i_08c614),
    (0x08C618, i_08c618),
    (0x08C61E, i_08c61e),
    (0x08C624, i_08c624),
    (0x08C62A, i_08c62a),
    (0x08C630, i_08c630),
    (0x08C636, i_08c636),
    (0x08C63A, i_08c63a),
    (0x08C640, i_08c640),
    (0x08C644, i_08c644),
    (0x08C64C, i_08c64c),
    (0x08C652, i_08c652),
    (0x08C658, i_08c658),
    (0x08C660, i_08c660),
    (0x08C666, i_08c666),
    (0x08C66A, i_08c66a),
    (0x08C670, i_08c670),
    (0x08C674, i_08c674),
    (0x08C676, i_08c676),
    (0x08C678, i_08c678),
    (0x08C67C, i_08c67c),
    (0x08C682, i_08c682),
    (0x08C686, i_08c686),
    (0x08C688, i_08c688),
    (0x08C68C, i_08c68c),
    (0x08C690, i_08c690),
    (0x08C694, i_08c694),
    (0x08C698, i_08c698),
    (0x08C69E, i_08c69e),
    (0x08C6A2, i_08c6a2),
    (0x08C6A8, i_08c6a8),
    (0x08C6AC, i_08c6ac),
    (0x08C6B2, i_08c6b2),
    (0x08C6B6, i_08c6b6),
    (0x08C6BC, i_08c6bc),
    (0x08C6C2, i_08c6c2),
    (0x08C6C6, i_08c6c6),
    (0x08C6CC, i_08c6cc),
    (0x08C6D2, i_08c6d2),
    (0x08C6D6, i_08c6d6),
    (0x08C6DC, i_08c6dc),
    (0x08C6DE, i_08c6de),
    (0x08C6E0, i_08c6e0),
    (0x08C6E4, i_08c6e4),
    (0x08C6E8, i_08c6e8),
    (0x08C6F0, i_08c6f0),
    (0x08C6F6, i_08c6f6),
    (0x08C6FC, i_08c6fc),
    (0x08C700, i_08c700),
    (0x08C704, i_08c704),
    (0x08C708, i_08c708),
    (0x08C70C, i_08c70c),
    (0x08C712, i_08c712),
    (0x08C718, i_08c718),
    (0x08C71E, i_08c71e),
    (0x08C722, i_08c722),
    (0x08C728, i_08c728),
    (0x08C72E, i_08c72e),
    (0x08C820, i_08c820),
    (0x08C828, i_08c828),
    (0x08C82C, i_08c82c),
    (0x08C832, i_08c832),
    (0x08C838, i_08c838),
    (0x08C840, i_08c840),
    (0x08C844, i_08c844),
    (0x08C84A, i_08c84a),
    (0x08C850, i_08c850),
    (0x08C856, i_08c856),
    (0x08C85C, i_08c85c),
    (0x08C862, i_08c862),
    (0x08C864, i_08c864),
    (0x08C86A, i_08c86a),
    (0x08C870, i_08c870),
    (0x08C876, i_08c876),
    (0x08C87A, i_08c87a),
    (0x08C87E, i_08c87e),
    (0x08C880, i_08c880),
    (0x08C886, i_08c886),
    (0x08C88A, i_08c88a),
    (0x08C88E, i_08c88e),
    (0x08C892, i_08c892),
    (0x08C898, i_08c898),
    (0x08C89C, i_08c89c),
    (0x08C8A0, i_08c8a0),
    (0x08C8A4, i_08c8a4),
    (0x08C8A8, i_08c8a8),
    (0x08C8AC, i_08c8ac),
    (0x08C8B2, i_08c8b2),
    (0x08C8B8, i_08c8b8),
    (0x08C8BC, i_08c8bc),
    (0x08C8C2, i_08c8c2),
    (0x08C8C6, i_08c8c6),
    (0x08C8CC, i_08c8cc),
    (0x08C8D0, i_08c8d0),
    (0x08C8D6, i_08c8d6),
    (0x08C8DA, i_08c8da),
    (0x08C8E0, i_08c8e0),
    (0x08C8E8, i_08c8e8),
    (0x08C8EE, i_08c8ee),
    (0x08C8F2, i_08c8f2),
    (0x08C8F4, i_08c8f4),
    (0x08C8F8, i_08c8f8),
    (0x08C8FA, i_08c8fa),
    (0x08C8FE, i_08c8fe),
    (0x08C902, i_08c902),
    (0x08C908, i_08c908),
    (0x08C90C, i_08c90c),
    (0x08C90E, i_08c90e),
    (0x08C914, i_08c914),
    (0x08C918, i_08c918),
    (0x08C91E, i_08c91e),
    (0x08C922, i_08c922),
    (0x08C928, i_08c928),
    (0x08C92C, i_08c92c),
    (0x08C92E, i_08c92e),
    (0x08C932, i_08c932),
    (0x08C936, i_08c936),
    (0x08C938, i_08c938),
    (0x08D34E, i_08d34e),
    (0x08D352, i_08d352),
    (0x08D356, i_08d356),
    (0x08D35A, i_08d35a),
    (0x08D35C, i_08d35c),
    (0x08D360, i_08d360),
    (0x08D362, i_08d362),
    (0x08D366, i_08d366),
    (0x08D368, i_08d368),
    (0x08D36A, i_08d36a),
    (0x08D36C, i_08d36c),
    (0x08D36E, i_08d36e),
    (0x08D372, i_08d372),
    (0x08D374, i_08d374),
    (0x08D378, i_08d378),
    (0x08D37C, i_08d37c),
    (0x08D380, i_08d380),
    (0x08D384, i_08d384),
    (0x08D386, i_08d386),
    (0x08D38A, i_08d38a),
    (0x08D38C, i_08d38c),
    (0x08D390, i_08d390),
    (0x08D392, i_08d392),
    (0x08D394, i_08d394),
    (0x08D396, i_08d396),
    (0x08D398, i_08d398),
    (0x08D39C, i_08d39c),
    (0x08D39E, i_08d39e),
    (0x08D3A2, i_08d3a2),
    (0x08D3A6, i_08d3a6),
    (0x08D3AA, i_08d3aa),
    (0x08D3AE, i_08d3ae),
    (0x08D3B2, i_08d3b2),
    (0x08E4E6, i_08e4e6),
    (0x08E4EC, i_08e4ec),
    (0x08E4F0, i_08e4f0),
    (0x08E4F4, i_08e4f4),
    (0x08E4F8, i_08e4f8),
    (0x08E4FE, i_08e4fe),
    (0x08E504, i_08e504),
    (0x08E50A, i_08e50a),
    (0x08E50E, i_08e50e),
    (0x08E514, i_08e514),
    (0x08E518, i_08e518),
    (0x08E51E, i_08e51e),
    (0x08E524, i_08e524),
    (0x08E528, i_08e528),
    (0x08E52A, i_08e52a),
    (0x08E530, i_08e530),
    (0x08E534, i_08e534),
    (0x08E53A, i_08e53a),
    (0x08E540, i_08e540),
    (0x08E544, i_08e544),
    (0x08E548, i_08e548),
    (0x08E54A, i_08e54a),
    (0x08E54C, i_08e54c),
    (0x08E550, i_08e550),
    (0x08E556, i_08e556),
    (0x08E55A, i_08e55a),
    (0x08E55E, i_08e55e),
    (0x08E560, i_08e560),
    (0x08E564, i_08e564),
    (0x08E566, i_08e566),
    (0x08E56C, i_08e56c),
    (0x08E572, i_08e572),
    (0x08E576, i_08e576),
    (0x08E57C, i_08e57c),
    (0x08E580, i_08e580),
    (0x08E586, i_08e586),
    (0x08E58C, i_08e58c),
    (0x08E590, i_08e590),
    (0x08E592, i_08e592),
    (0x08E598, i_08e598),
    (0x08E59C, i_08e59c),
    (0x08E5A2, i_08e5a2),
    (0x08E5A8, i_08e5a8),
    (0x08E5AC, i_08e5ac),
    (0x08E5B0, i_08e5b0),
    (0x08E5B2, i_08e5b2),
    (0x08E5B4, i_08e5b4),
    (0x08E5B8, i_08e5b8),
    (0x08E5BE, i_08e5be),
    (0x08E5C2, i_08e5c2),
    (0x08E5C6, i_08e5c6),
    (0x08E5C8, i_08e5c8),
    (0x08E5CC, i_08e5cc),
    (0x08EFB0, i_08efb0),
    (0x08EFB6, i_08efb6),
    (0x08EFBC, i_08efbc),
    (0x08EFC0, i_08efc0),
    (0x08EFC6, i_08efc6),
    (0x08EFCC, i_08efcc),
    (0x08F002, i_08f002),
    (0x08F008, i_08f008),
    (0x08F00E, i_08f00e),
    (0x08F010, i_08f010),
    (0x08F012, i_08f012),
    (0x08F016, i_08f016),
    (0x08F018, i_08f018),
    (0x08F01C, i_08f01c),
    (0x08F022, i_08f022),
    (0x08F026, i_08f026),
    (0x08F02A, i_08f02a),
    (0x08F138, i_08f138),
    (0x08F13A, i_08f13a),
    (0x08F13E, i_08f13e),
    (0x08F142, i_08f142),
    (0x08F146, i_08f146),
    (0x08F148, i_08f148),
    (0x08F14C, i_08f14c),
    (0x08F152, i_08f152),
    (0x08F156, i_08f156),
    (0x08F158, i_08f158),
    (0x08F15E, i_08f15e),
    (0x08F160, i_08f160),
    (0x08F166, i_08f166),
    (0x08F168, i_08f168),
    (0x08F16E, i_08f16e),
    (0x08F170, i_08f170),
    (0x08F172, i_08f172),
    (0x08F178, i_08f178),
    (0x08F17A, i_08f17a),
    (0x08F180, i_08f180),
    (0x08F182, i_08f182),
    (0x08F188, i_08f188),
    (0x08F18A, i_08f18a),
    (0x08F2A0, i_08f2a0),
    (0x08F2A4, i_08f2a4),
    (0x08F2AA, i_08f2aa),
    (0x08F2AE, i_08f2ae),
    (0x08F2B2, i_08f2b2),
    (0x08F2B4, i_08f2b4),
    (0x08F2BA, i_08f2ba),
    (0x08F2C0, i_08f2c0),
    (0x08F2C4, i_08f2c4),
    (0x08F2C6, i_08f2c6),
    (0x08F2CA, i_08f2ca),
    (0x08F2CE, i_08f2ce),
    (0x08F2D2, i_08f2d2),
    (0x08F2D6, i_08f2d6),
    (0x08F2DA, i_08f2da),
    (0x08F2DE, i_08f2de),
    (0x08F2E2, i_08f2e2),
    (0x08F2E6, i_08f2e6),
    (0x08F2EA, i_08f2ea),
    (0x08F2EE, i_08f2ee),
    (0x08F2F2, i_08f2f2),
    (0x08F2F4, i_08f2f4),
    (0x08F2F6, i_08f2f6),
    (0x08F2FA, i_08f2fa),
    (0x08F2FE, i_08f2fe),
    (0x08F300, i_08f300),
    (0x08F302, i_08f302),
    (0x08F306, i_08f306),
    (0x08F308, i_08f308),
    (0x08F30E, i_08f30e),
    (0x08F312, i_08f312),
    (0x08F316, i_08f316),
    (0x08F31A, i_08f31a),
    (0x08F320, i_08f320),
    (0x08F326, i_08f326),
    (0x08F32A, i_08f32a),
    (0x08F32E, i_08f32e),
    (0x08F330, i_08f330),
    (0x08F334, i_08f334),
    (0x08F338, i_08f338),
    (0x08F33A, i_08f33a),
    (0x08F33E, i_08f33e),
    (0x08F342, i_08f342),
    (0x08F344, i_08f344),
    (0x08F34A, i_08f34a),
    (0x08F34E, i_08f34e),
    (0x08F350, i_08f350),
    (0x08F354, i_08f354),
    (0x08F358, i_08f358),
    (0x08F35C, i_08f35c),
    (0x08F360, i_08f360),
    (0x08F364, i_08f364),
    (0x08F366, i_08f366),
    (0x08F36A, i_08f36a),
    (0x08F36E, i_08f36e),
    (0x08F372, i_08f372),
    (0x08F376, i_08f376),
    (0x08F37A, i_08f37a),
    (0x08F37E, i_08f37e),
    (0x08F382, i_08f382),
    (0x08F388, i_08f388),
    (0x08F38A, i_08f38a),
    (0x08F38C, i_08f38c),
    (0x08F390, i_08f390),
    (0x08F392, i_08f392),
    (0x08F394, i_08f394),
    (0x08F398, i_08f398),
    (0x08F39C, i_08f39c),
    (0x08F39E, i_08f39e),
    (0x08F3A0, i_08f3a0),
    (0x08F3A4, i_08f3a4),
    (0x08F470, i_08f470),
    (0x08F476, i_08f476),
    (0x08F47A, i_08f47a),
    (0x08F47C, i_08f47c),
    (0x08F482, i_08f482),
    (0x08F486, i_08f486),
    (0x08F48C, i_08f48c),
    (0x08F490, i_08f490),
    (0x08F494, i_08f494),
    (0x08F498, i_08f498),
    (0x08F49C, i_08f49c),
    (0x08F4A0, i_08f4a0),
    (0x08F4A6, i_08f4a6),
    (0x08F4AA, i_08f4aa),
    (0x08F4AC, i_08f4ac),
    (0x08F4B0, i_08f4b0),
    (0x08F4B4, i_08f4b4),
    (0x08F4B6, i_08f4b6),
    (0x08F4B8, i_08f4b8),
    (0x08F4BC, i_08f4bc),
    (0x08F4C0, i_08f4c0),
    (0x08F4C4, i_08f4c4),
    (0x08F4CA, i_08f4ca),
    (0x08F4CE, i_08f4ce),
    (0x08F4D0, i_08f4d0),
    (0x08F4D4, i_08f4d4),
    (0x08F4D8, i_08f4d8),
    (0x08F4DA, i_08f4da),
    (0x08F4DC, i_08f4dc),
    (0x08F4E0, i_08f4e0),
    (0x08F4E2, i_08f4e2),
    (0x08F4E6, i_08f4e6),
    (0x08F4E8, i_08f4e8),
    (0x08F4EC, i_08f4ec),
    (0x08F4EE, i_08f4ee),
    (0x08F4F2, i_08f4f2),
    (0x08F4F6, i_08f4f6),
    (0x08F4F8, i_08f4f8),
    (0x08F4FC, i_08f4fc),
    (0x08F500, i_08f500),
    (0x08F504, i_08f504),
    (0x08F508, i_08f508),
    (0x08F50A, i_08f50a),
    (0x08F50E, i_08f50e),
    (0x08F512, i_08f512),
    (0x08F516, i_08f516),
    (0x08F51A, i_08f51a),
    (0x08F51E, i_08f51e),
    (0x08F520, i_08f520),
    (0x08F526, i_08f526),
    (0x08F52A, i_08f52a),
    (0x08F52C, i_08f52c),
    (0x08F530, i_08f530),
    (0x08F536, i_08f536),
    (0x08F53A, i_08f53a),
    (0x08F53E, i_08f53e),
    (0x08F544, i_08f544),
    (0x08F548, i_08f548),
    (0x08F54C, i_08f54c),
    (0x08F550, i_08f550),
    (0x08F554, i_08f554),
    (0x08F558, i_08f558),
    (0x08F55C, i_08f55c),
    (0x08F562, i_08f562),
    (0x08F566, i_08f566),
    (0x08F568, i_08f568),
    (0x08F56C, i_08f56c),
    (0x08F570, i_08f570),
    (0x08F572, i_08f572),
    (0x08F574, i_08f574),
    (0x08F578, i_08f578),
    (0x08F57A, i_08f57a),
    (0x08F580, i_08f580),
    (0x08F584, i_08f584),
    (0x08F588, i_08f588),
    (0x08F58C, i_08f58c),
    (0x08F590, i_08f590),
    (0x08F594, i_08f594),
    (0x08F598, i_08f598),
    (0x08F59C, i_08f59c),
    (0x08F59E, i_08f59e),
    (0x08F5A0, i_08f5a0),
    (0x08F5A4, i_08f5a4),
    (0x08F5A8, i_08f5a8),
    (0x08F5AA, i_08f5aa),
    (0x08F5AC, i_08f5ac),
    (0x08F5B0, i_08f5b0),
    (0x08F5B4, i_08f5b4),
    (0x08F5BA, i_08f5ba),
    (0x08F5BE, i_08f5be),
    (0x08F5C2, i_08f5c2),
    (0x08F5C4, i_08f5c4),
    (0x08F5C8, i_08f5c8),
    (0x08F5CC, i_08f5cc),
    (0x08F5CE, i_08f5ce),
    (0x08F5D2, i_08f5d2),
    (0x08F5D6, i_08f5d6),
    (0x08F5DA, i_08f5da),
    (0x08F6D2, i_08f6d2),
    (0x08F6D8, i_08f6d8),
    (0x08F6DA, i_08f6da),
    (0x08F6E0, i_08f6e0),
    (0x08F6E2, i_08f6e2),
    (0x08F6E6, i_08f6e6),
    (0x08F6EA, i_08f6ea),
    (0x08F6F0, i_08f6f0),
    (0x08F714, i_08f714),
    (0x08F71A, i_08f71a),
    (0x08F71E, i_08f71e),
    (0x08F722, i_08f722),
    (0x08F726, i_08f726),
    (0x08F728, i_08f728),
    (0x08F72C, i_08f72c),
    (0x08F730, i_08f730),
    (0x08F732, i_08f732),
    (0x08F736, i_08f736),
    (0x08F738, i_08f738),
    (0x08F73C, i_08f73c),
    (0x08F740, i_08f740),
    (0x08F742, i_08f742),
    (0x08F744, i_08f744),
    (0x08F748, i_08f748),
    (0x08F74A, i_08f74a),
    (0x08F74C, i_08f74c),
    (0x08F750, i_08f750),
    (0x08F752, i_08f752),
    (0x08F756, i_08f756),
    (0x08F758, i_08f758),
    (0x08F75A, i_08f75a),
    (0x08F75E, i_08f75e),
    (0x08F762, i_08f762),
    (0x08F766, i_08f766),
    (0x08F76A, i_08f76a),
    (0x08F76E, i_08f76e),
    (0x08F772, i_08f772),
    (0x08F774, i_08f774),
    (0x08F778, i_08f778),
    (0x08F79C, i_08f79c),
    (0x08F7A0, i_08f7a0),
    (0x08F7A4, i_08f7a4),
    (0x08F7A6, i_08f7a6),
    (0x08F7A8, i_08f7a8),
    (0x08F7AA, i_08f7aa),
    (0x08F7AC, i_08f7ac),
    (0x08F7AE, i_08f7ae),
    (0x08F7B2, i_08f7b2),
    (0x08F7B8, i_08f7b8),
    (0x08F7BC, i_08f7bc),
    (0x08F7C2, i_08f7c2),
    (0x08F7C6, i_08f7c6),
    (0x08F7CC, i_08f7cc),
    (0x08F7CE, i_08f7ce),
    (0x08F7D0, i_08f7d0),
    (0x08F7D2, i_08f7d2),
    (0x08F7D6, i_08f7d6),
    (0x08F7D8, i_08f7d8),
    (0x08F7DA, i_08f7da),
    (0x08F7DE, i_08f7de),
    (0x08F7E2, i_08f7e2),
    (0x08F7E6, i_08f7e6),
    (0x08F7EA, i_08f7ea),
    (0x08F7EE, i_08f7ee),
    (0x08F7F2, i_08f7f2),
    (0x08F7F6, i_08f7f6),
    (0x08F7FA, i_08f7fa),
    (0x08F7FE, i_08f7fe),
    (0x08F802, i_08f802),
    (0x08F806, i_08f806),
    (0x08F80A, i_08f80a),
    (0x08F80E, i_08f80e),
    (0x08F812, i_08f812),
    (0x08F816, i_08f816),
    (0x08F81A, i_08f81a),
    (0x08F81E, i_08f81e),
    (0x08F822, i_08f822),
    (0x08F824, i_08f824),
    (0x08F826, i_08f826),
    (0x08F828, i_08f828),
    (0x08F8E2, i_08f8e2),
    (0x08F8E8, i_08f8e8),
    (0x08F8EA, i_08f8ea),
    (0x08F8EE, i_08f8ee),
    (0x08F8F2, i_08f8f2),
    (0x08F8F8, i_08f8f8),
    (0x08F8FC, i_08f8fc),
    (0x08F91A, i_08f91a),
    (0x08F91E, i_08f91e),
    (0x08F924, i_08f924),
    (0x08F92A, i_08f92a),
    (0x08F930, i_08f930),
    (0x08F936, i_08f936),
    (0x08F93C, i_08f93c),
    (0x08F942, i_08f942),
    (0x08F944, i_08f944),
    (0x08F948, i_08f948),
    (0x08F94C, i_08f94c),
    (0x08F952, i_08f952),
    (0x08F956, i_08f956),
    (0x08F95A, i_08f95a),
    (0x08F95C, i_08f95c),
    (0x08F962, i_08f962),
    (0x08F966, i_08f966),
    (0x08F968, i_08f968),
    (0x08F96A, i_08f96a),
    (0x08F96E, i_08f96e),
    (0x08F974, i_08f974),
    (0x08F978, i_08f978),
    (0x08F97E, i_08f97e),
    (0x08F984, i_08f984),
    (0x08F988, i_08f988),
    (0x08F98E, i_08f98e),
    (0x08F994, i_08f994),
    (0x08F998, i_08f998),
    (0x08F99E, i_08f99e),
    (0x08F9A4, i_08f9a4),
    (0x08F9AA, i_08f9aa),
    (0x08F9AE, i_08f9ae),
    (0x08F9B4, i_08f9b4),
    (0x08F9BA, i_08f9ba),
    (0x08F9C0, i_08f9c0),
    (0x08F9C4, i_08f9c4),
    (0x08F9CA, i_08f9ca),
    (0x08F9D0, i_08f9d0),
    (0x08F9D6, i_08f9d6),
    (0x08F9DA, i_08f9da),
    (0x08F9E0, i_08f9e0),
    (0x08F9E6, i_08f9e6),
    (0x08F9EC, i_08f9ec),
    (0x08F9F0, i_08f9f0),
    (0x08F9F6, i_08f9f6),
    (0x08F9FC, i_08f9fc),
    (0x08FA02, i_08fa02),
    (0x08FA06, i_08fa06),
    (0x08FA0C, i_08fa0c),
    (0x08FA12, i_08fa12),
    (0x08FA18, i_08fa18),
    (0x08FA1C, i_08fa1c),
    (0x08FA22, i_08fa22),
    (0x08FA28, i_08fa28),
    (0x08FA2E, i_08fa2e),
    (0x08FA32, i_08fa32),
    (0x08FA38, i_08fa38),
    (0x08FA3E, i_08fa3e),
    (0x08FA44, i_08fa44),
    (0x08FA48, i_08fa48),
    (0x08FA4E, i_08fa4e),
    (0x08FA54, i_08fa54),
    (0x08FA5A, i_08fa5a),
    (0x08FA5E, i_08fa5e),
    (0x08FA64, i_08fa64),
    (0x08FA6A, i_08fa6a),
    (0x08FA70, i_08fa70),
    (0x08FA74, i_08fa74),
    (0x08FA7A, i_08fa7a),
    (0x08FA80, i_08fa80),
    (0x08FA86, i_08fa86),
    (0x08FA8A, i_08fa8a),
    (0x08FA90, i_08fa90),
    (0x08FA96, i_08fa96),
    (0x08FA9C, i_08fa9c),
    (0x08FAA0, i_08faa0),
    (0x08FAA6, i_08faa6),
    (0x08FAAC, i_08faac),
    (0x08FAB2, i_08fab2),
    (0x08FAB6, i_08fab6),
    (0x08FABC, i_08fabc),
    (0x08FAC2, i_08fac2),
    (0x08FAC6, i_08fac6),
    (0x08FACC, i_08facc),
    (0x08FAD2, i_08fad2),
    (0x08FAD6, i_08fad6),
    (0x08FADC, i_08fadc),
    (0x08FAE2, i_08fae2),
    (0x08FAE6, i_08fae6),
    (0x08FAEC, i_08faec),
    (0x08FAF2, i_08faf2),
    (0x08FAF6, i_08faf6),
    (0x08FAFC, i_08fafc),
    (0x08FB02, i_08fb02),
    (0x08FB06, i_08fb06),
    (0x08FB0C, i_08fb0c),
    (0x08FB12, i_08fb12),
    (0x08FB16, i_08fb16),
    (0x08FB1C, i_08fb1c),
    (0x08FB22, i_08fb22),
    (0x08FB26, i_08fb26),
    (0x08FB2C, i_08fb2c),
    (0x08FB32, i_08fb32),
    (0x08FB36, i_08fb36),
    (0x08FB3C, i_08fb3c),
    (0x08FB42, i_08fb42),
    (0x08FB46, i_08fb46),
    (0x08FB4C, i_08fb4c),
    (0x08FB52, i_08fb52),
    (0x08FB56, i_08fb56),
    (0x08FB5C, i_08fb5c),
    (0x08FB62, i_08fb62),
    (0x08FB66, i_08fb66),
    (0x08FB6C, i_08fb6c),
    (0x08FB72, i_08fb72),
    (0x08FB76, i_08fb76),
    (0x08FB7C, i_08fb7c),
    (0x08FB82, i_08fb82),
    (0x08FB86, i_08fb86),
    (0x08FB8C, i_08fb8c),
    (0x08FB92, i_08fb92),
    (0x08FB96, i_08fb96),
    (0x08FB9C, i_08fb9c),
    (0x08FBA0, i_08fba0),
    (0x08FBA2, i_08fba2),
    (0x08FBA6, i_08fba6),
    (0x08FBAA, i_08fbaa),
    (0x08FBAE, i_08fbae),
    (0x08FBB2, i_08fbb2),
    (0x08FBB8, i_08fbb8),
    (0x08FBBC, i_08fbbc),
    (0x08FBC0, i_08fbc0),
    (0x08FBC6, i_08fbc6),
    (0x08FBCA, i_08fbca),
    (0x08FBD0, i_08fbd0),
    (0x08FBD4, i_08fbd4),
    (0x08FBDA, i_08fbda),
    (0x08FBDE, i_08fbde),
    (0x08FBE4, i_08fbe4),
    (0x08FBE8, i_08fbe8),
    (0x08FBEE, i_08fbee),
    (0x08FBF4, i_08fbf4),
    (0x08FBFA, i_08fbfa),
    (0x08FBFE, i_08fbfe),
    (0x08FC04, i_08fc04),
    (0x08FC0A, i_08fc0a),
    (0x08FC10, i_08fc10),
    (0x08FC14, i_08fc14),
    (0x08FC1A, i_08fc1a),
    (0x08FC20, i_08fc20),
    (0x08FC26, i_08fc26),
    (0x08FC2C, i_08fc2c),
    (0x08FC30, i_08fc30),
    (0x08FC36, i_08fc36),
    (0x08FC3A, i_08fc3a),
    (0x08FC40, i_08fc40),
    (0x08FC44, i_08fc44),
    (0x08FC4A, i_08fc4a),
    (0x08FC4E, i_08fc4e),
    (0x08FC54, i_08fc54),
    (0x08FC5A, i_08fc5a),
    (0x08FC60, i_08fc60),
    (0x08FC64, i_08fc64),
    (0x08FC6A, i_08fc6a),
    (0x08FC70, i_08fc70),
    (0x08FC76, i_08fc76),
    (0x08FC7A, i_08fc7a),
    (0x08FC80, i_08fc80),
    (0x08FC86, i_08fc86),
    (0x08FC8C, i_08fc8c),
    (0x08FC92, i_08fc92),
    (0x08FC96, i_08fc96),
    (0x08FC9C, i_08fc9c),
    (0x08FCA2, i_08fca2),
    (0x08FCA6, i_08fca6),
    (0x08FCA8, i_08fca8),
    (0x08FCAA, i_08fcaa),
    (0x08FCB0, i_08fcb0),
    (0x08FCB4, i_08fcb4),
    (0x08FCB8, i_08fcb8),
    (0x08FCBA, i_08fcba),
    (0x08FCBC, i_08fcbc),
    (0x08FCC0, i_08fcc0),
    (0x08FCC2, i_08fcc2),
    (0x08FCC4, i_08fcc4),
    (0x08FCC8, i_08fcc8),
    (0x08FCCA, i_08fcca),
    (0x08FCCE, i_08fcce),
    (0x08FCD2, i_08fcd2),
    (0x08FCD6, i_08fcd6),
    (0x08FCDA, i_08fcda),
    (0x08FCE0, i_08fce0),
    (0x08FCE6, i_08fce6),
    (0x08FCE8, i_08fce8),
    (0x08FCEA, i_08fcea),
    (0x08FCF0, i_08fcf0),
    (0x08FCF6, i_08fcf6),
    (0x08FCFA, i_08fcfa),
    (0x08FD00, i_08fd00),
    (0x08FD06, i_08fd06),
    (0x08FD0A, i_08fd0a),
    (0x08FD10, i_08fd10),
    (0x08FD16, i_08fd16),
    (0x08FD1A, i_08fd1a),
    (0x08FD20, i_08fd20),
    (0x08FD26, i_08fd26),
    (0x08FD2A, i_08fd2a),
    (0x08FD2C, i_08fd2c),
    (0x08FD2E, i_08fd2e),
    (0x08FD32, i_08fd32),
    (0x08FD36, i_08fd36),
    (0x08FD3A, i_08fd3a),
    (0x08FD3E, i_08fd3e),
    (0x08FD44, i_08fd44),
    (0x08FD4A, i_08fd4a),
    (0x08FD4E, i_08fd4e),
    (0x08FD54, i_08fd54),
    (0x08FD5A, i_08fd5a),
    (0x08FD60, i_08fd60),
    (0x08FD64, i_08fd64),
    (0x08FD66, i_08fd66),
    (0x08FD68, i_08fd68),
    (0x08FD6C, i_08fd6c),
    (0x08FD72, i_08fd72),
    (0x08FD76, i_08fd76),
    (0x08FD7E, i_08fd7e),
    (0x08FD84, i_08fd84),
    (0x08FD88, i_08fd88),
    (0x08FD8C, i_08fd8c),
    (0x08FD90, i_08fd90),
    (0x08FD96, i_08fd96),
    (0x08FD9C, i_08fd9c),
    (0x08FDA2, i_08fda2),
    (0x08FDA6, i_08fda6),
    (0x08FDA8, i_08fda8),
    (0x08FDAA, i_08fdaa),
    (0x08FDB0, i_08fdb0),
    (0x08FDB4, i_08fdb4),
    (0x08FDB8, i_08fdb8),
    (0x08FDBC, i_08fdbc),
    (0x08FDC2, i_08fdc2),
    (0x08FDC6, i_08fdc6),
    (0x08FDCA, i_08fdca),
    (0x08FDD0, i_08fdd0),
    (0x08FDD4, i_08fdd4),
    (0x08FDDA, i_08fdda),
    (0x08FDE0, i_08fde0),
    (0x08FDE4, i_08fde4),
    (0x08FDEA, i_08fdea),
    (0x08FDF0, i_08fdf0),
    (0x08FDF4, i_08fdf4),
    (0x08FDF6, i_08fdf6),
    (0x08FDF8, i_08fdf8),
    (0x08FDFE, i_08fdfe),
    (0x08FE02, i_08fe02),
    (0x08FE06, i_08fe06),
    (0x08FE0C, i_08fe0c),
    (0x08FE12, i_08fe12),
    (0x08FE18, i_08fe18),
    (0x08FE1E, i_08fe1e),
    (0x08FE24, i_08fe24),
    (0x08FE2A, i_08fe2a),
    (0x08FE2E, i_08fe2e),
    (0x08FE30, i_08fe30),
    (0x08FE32, i_08fe32),
    (0x08FE38, i_08fe38),
    (0x08FE3C, i_08fe3c),
    (0x08FE42, i_08fe42),
    (0x08FE46, i_08fe46),
    (0x08FE48, i_08fe48),
    (0x08FE4A, i_08fe4a),
    (0x08FE4E, i_08fe4e),
    (0x08FE52, i_08fe52),
    (0x08FE56, i_08fe56),
    (0x08FE5A, i_08fe5a),
    (0x08FE5E, i_08fe5e),
    (0x08FE62, i_08fe62),
    (0x08FE66, i_08fe66),
    (0x08FE6A, i_08fe6a),
    (0x08FE6E, i_08fe6e),
    (0x08FE74, i_08fe74),
    (0x08FE78, i_08fe78),
    (0x08FE7E, i_08fe7e),
    (0x08FE84, i_08fe84),
    (0x08FE88, i_08fe88),
    (0x08FE8E, i_08fe8e),
    (0x08FE94, i_08fe94),
    (0x08FE98, i_08fe98),
    (0x08FE9A, i_08fe9a),
    (0x08FE9C, i_08fe9c),
    (0x08FEA0, i_08fea0),
    (0x08FEA4, i_08fea4),
    (0x08FEAA, i_08feaa),
    (0x08FEAE, i_08feae),
    (0x08FEB2, i_08feb2),
    (0x08FEB4, i_08feb4),
    (0x08FEB6, i_08feb6),
    (0x08FEBA, i_08feba),
    (0x08FEBE, i_08febe),
    (0x08FEC4, i_08fec4),
    (0x08FEC8, i_08fec8),
    (0x08FECA, i_08feca),
    (0x08FECC, i_08fecc),
    (0x08FED0, i_08fed0),
    (0x08FED6, i_08fed6),
    (0x08FEDC, i_08fedc),
    (0x08FEE2, i_08fee2),
    (0x08FEE8, i_08fee8),
    (0x08FEEE, i_08feee),
    (0x08FEF4, i_08fef4),
    (0x08FEFA, i_08fefa),
    (0x08FF00, i_08ff00),
    (0x08FF04, i_08ff04),
    (0x08FF0A, i_08ff0a),
    (0x08FF10, i_08ff10),
    (0x08FF16, i_08ff16),
    (0x08FF1C, i_08ff1c),
    (0x08FF22, i_08ff22),
    (0x08FF26, i_08ff26),
    (0x08FF28, i_08ff28),
    (0x08FF2C, i_08ff2c),
    (0x08FF30, i_08ff30),
    (0x08FF36, i_08ff36),
    (0x08FF3C, i_08ff3c),
    (0x08FF42, i_08ff42),
    (0x08FF48, i_08ff48),
    (0x08FF4E, i_08ff4e),
    (0x08FF54, i_08ff54),
    (0x08FF5A, i_08ff5a),
    (0x08FF60, i_08ff60),
    (0x08FF64, i_08ff64),
    (0x08FF6A, i_08ff6a),
    (0x08FF70, i_08ff70),
    (0x08FF76, i_08ff76),
    (0x08FF7C, i_08ff7c),
    (0x08FF82, i_08ff82),
    (0x08FF86, i_08ff86),
    (0x08FF88, i_08ff88),
    (0x08FF8C, i_08ff8c),
    (0x08FF90, i_08ff90),
    (0x08FF94, i_08ff94),
    (0x08FF98, i_08ff98),
    (0x08FF9C, i_08ff9c),
    (0x08FFA0, i_08ffa0),
    (0x08FFA4, i_08ffa4),
    (0x08FFA8, i_08ffa8),
    (0x08FFAE, i_08ffae),
    (0x08FFB2, i_08ffb2),
    (0x08FFB6, i_08ffb6),
    (0x08FFBA, i_08ffba),
    (0x08FFC2, i_08ffc2),
    (0x08FFC6, i_08ffc6),
    (0x08FFCC, i_08ffcc),
    (0x08FFD2, i_08ffd2),
    (0x08FFD4, i_08ffd4),
    (0x08FFD8, i_08ffd8),
    (0x08FFDA, i_08ffda),
    (0x08FFDC, i_08ffdc),
    (0x08FFDE, i_08ffde),
    (0x08FFE2, i_08ffe2),
    (0x08FFE6, i_08ffe6),
    (0x08FFEA, i_08ffea),
    (0x08FFEE, i_08ffee),
    (0x08FFF4, i_08fff4),
    (0x08FFF6, i_08fff6),
    (0x08FFFA, i_08fffa),
];
