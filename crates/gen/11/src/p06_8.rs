// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_8) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $06DE10: muls.w d3, d1
pub fn i_06de10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE12;
    cpu.ir = 0xC3C3;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $06DE12: asr.w #$2, d1
pub fn i_06de12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE14;
    cpu.ir = 0xE441;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06DE14: move.w $36(a6), d0
pub fn i_06de14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE18;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DE18: muls.w d0, d1
pub fn i_06de18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE1A;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $06DE1A: muls.w d0, d2
pub fn i_06de1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE1C;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $06DE1C: asr.l #$8, d1
pub fn i_06de1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE1E;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $06DE1E: asr.l #$8, d2
pub fn i_06de1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE20;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $06DE20: move.w d1, $2a(a6)
pub fn i_06de20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE24;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE24: move.w d2, $28(a6)
pub fn i_06de24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE28;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE28: jsr $5e9b6.l
pub fn i_06de28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DE2E);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DE2E: cmpi.b #$0, d0
pub fn i_06de2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE32;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $06DE32: bgt.w $6de66
pub fn i_06de32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE36;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6DE66; return 10; }
    8
}
/// $06DE36: jsr $5e9b6.l
pub fn i_06de36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DE3C);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DE3C: andi.w #$3, d0
pub fn i_06de3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE40;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DE40: addq.w #$2, d0
pub fn i_06de40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE42;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06DE42: move.w $28(a6), d1
pub fn i_06de42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE46;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DE46: muls.w d1, d0
pub fn i_06de46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE48;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $06DE48: asr.w #$1, d0
pub fn i_06de48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE4A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06DE4A: move.w d0, $28(a6)
pub fn i_06de4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE4E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE4E: jsr $5e9b6.l
pub fn i_06de4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DE54);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DE54: andi.w #$3, d0
pub fn i_06de54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE58;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DE58: addq.w #$2, d0
pub fn i_06de58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE5A;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06DE5A: move.w $2a(a6), d1
pub fn i_06de5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE5E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DE5E: muls.w d1, d0
pub fn i_06de5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE60;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $06DE60: asr.w #$1, d0
pub fn i_06de60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE62;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06DE62: move.w d0, $2a(a6)
pub fn i_06de62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE66;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE66: move.w $2a(a6), d0
pub fn i_06de66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE6A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DE6A: move.w d0, $84(a6)
pub fn i_06de6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE6E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE6E: move.w #$ffd0, $2e(a6)
pub fn i_06de6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE74;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE74: lea.l $6de7a(pc), a1
pub fn i_06de74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DE7A;
    16
}
/// $06DE78: move.l a1, (a6)
pub fn i_06de78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DE7A: jsr $27d50.l
pub fn i_06de7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DE80);
    cpu.pc = 0x27D50;
    16
}
/// $06DE80: bcc.w $6deb6
pub fn i_06de80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DEB6; return 10; }
    8
}
/// $06DE84: move.w #$ffd0, $2e(a6)
pub fn i_06de84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE8A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE8A: move.w $28(a6), d0
pub fn i_06de8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE8E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DE8E: asr.w #$2, d0
pub fn i_06de8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE90;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06DE90: move.w d0, $28(a6)
pub fn i_06de90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE94;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE94: move.w $84(a6), d0
pub fn i_06de94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE98;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DE98: asr.w #$1, d0
pub fn i_06de98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE9A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06DE9A: move.w d0, $2a(a6)
pub fn i_06de9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DE9E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DE9E: move.w d0, $84(a6)
pub fn i_06de9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEA2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DEA2: subq.w #$1, $80(a6)
pub fn i_06dea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEA6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06DEA6: cmpi.w #$0, $80(a6)
pub fn i_06dea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEAC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06DEAC: bgt.w $6deb6
pub fn i_06deac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEB0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6DEB6; return 10; }
    8
}
/// $06DEB0: lea.l $6da90(pc), a1
pub fn i_06deb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06DEB4: move.l a1, (a6)
pub fn i_06deb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DEB6: move.w $82(a6), d0
pub fn i_06deb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEBA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06DEBA: cmp.w $80(a6), d0
pub fn i_06deba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEBE;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06DEBE: beq.w $6ded0
pub fn i_06debe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEC2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6DED0; return 10; }
    8
}
/// $06DEC2: addq.b #$1, $72(a6)
pub fn i_06dec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEC6;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06DEC6: btst.b #$0, $72(a6)
pub fn i_06dec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DECC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06DECC: beq.w $6dee8
pub fn i_06decc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DED0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6DEE8; return 10; }
    8
}
/// $06DED0: jsr $5e804.l
pub fn i_06ded0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DED6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DED6);
    cpu.pc = 0x5E804;
    16
}
/// $06DED6: bcc.w $6dee2
pub fn i_06ded6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DEE2; return 10; }
    8
}
/// $06DEDA: andi.b #$ee, ccr
pub fn i_06deda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEDE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06DEDE: bra.w $6dee8
pub fn i_06dede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x6DEE8;
    10
}
/// $06DEE2: jsr $28d70.l
pub fn i_06dee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DEE8);
    cpu.pc = 0x28D70;
    16
}
/// $06DEE8: movea.l #$ffffffff, a0
pub fn i_06dee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEEE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06DEEE: lea.l $2d403e.l, a0
pub fn i_06deee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D403E;
    20
}
/// $06DEF4: jsr $5dd56.l
pub fn i_06def4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DEFA);
    cpu.pc = 0x5DD56;
    16
}
/// $06DEFA: bcc.w $6df04
pub fn i_06defa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DEFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DF04; return 10; }
    8
}
// ===== SetTaskHandler_06defe  ($06DEFE, 8 B, kernel/task_handlers.c)
/// $06DEFE: lea.l $6da90(pc), a1
pub fn i_06defe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06DF02: move.l a1, (a6)
pub fn i_06df02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF04;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DF04: rts
pub fn i_06df04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FireBurst_Wide_06df06  ($06DF06, 44 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DF06: jsr $5e9b6.l
pub fn i_06df06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF0C);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DF0C: andi.w #$f00, d0
pub fn i_06df0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF10;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF10: addi.w #$800, d0
pub fn i_06df10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF14;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x800, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF14: jsr $6e20c(pc)
pub fn i_06df14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6DF18);
    cpu.pc = 0x6E20C;
    16
}
/// $06DF18: move.w d0, $28(a6)
pub fn i_06df18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF1C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DF1C: jsr $5e9b6.l
pub fn i_06df1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF22);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DF22: andi.w #$f00, d0
pub fn i_06df22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF26;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF26: addi.w #$800, d0
pub fn i_06df26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF2A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x800, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF2A: move.w d0, $2a(a6)
pub fn i_06df2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF2E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DF2E: bra.w $6df5a
pub fn i_06df2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF32;
    cpu.ir = 0x6000;
    cpu.pc = 0x6DF5A;
    10
}
// ===== FireBurst_Tmpl_06df32  ($06DF32, 182 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06DF32: jsr $5e9b6.l
pub fn i_06df32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF38);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DF38: andi.w #$700, d0
pub fn i_06df38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF3C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x700) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF3C: addi.w #$300, d0
pub fn i_06df3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF40;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x300, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF40: jsr $6e20c(pc)
pub fn i_06df40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6DF44);
    cpu.pc = 0x6E20C;
    16
}
/// $06DF44: move.w d0, $28(a6)
pub fn i_06df44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF48;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DF48: jsr $5e9b6.l
pub fn i_06df48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF4E);
    cpu.pc = 0x5E9B6;
    16
}
/// $06DF4E: andi.w #$700, d0
pub fn i_06df4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF52;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x700) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF52: addi.w #$500, d0
pub fn i_06df52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF56;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x500, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06DF56: move.w d0, $2a(a6)
pub fn i_06df56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF5A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06DF5A: bset.b #$4, $6b(a6)
pub fn i_06df5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF60;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06DF60: move.w #$108e, d0
pub fn i_06df60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF64;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DF64: jsr $2352.l
pub fn i_06df64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF6A);
    cpu.pc = 0x2352;
    16
}
/// $06DF6A: lea.l $2d4666.l, a0
pub fn i_06df6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4666;
    20
}
/// $06DF70: jsr $28cd4.l
pub fn i_06df70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF76);
    cpu.pc = 0x28CD4;
    16
}
/// $06DF76: move.l #$ffffffff, $48(a6)
pub fn i_06df76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF7E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06DF7E: move.l #$2d3e42, $4c(a6)
pub fn i_06df7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF86;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D3E42;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06DF86: move.w #$2, d1
pub fn i_06df86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF8A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DF8A: jsr $236e.l
pub fn i_06df8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF90);
    cpu.pc = 0x236E;
    16
}
/// $06DF90: clr.w $72(a6)
pub fn i_06df90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF94;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06DF94: jsr $283ca.l
pub fn i_06df94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DF9A);
    cpu.pc = 0x283CA;
    16
}
/// $06DF9A: move.w #$d000, d0
pub fn i_06df9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DF9E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DF9E: jsr $28134.l
pub fn i_06df9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DFA4);
    cpu.pc = 0x28134;
    16
}
/// $06DFA4: andi.w #$ffe3, $38(a6)
pub fn i_06dfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFAA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DFAA: ori.w #$14, $38(a6)
pub fn i_06dfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFB0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06DFB0: lea.l $6dfb6(pc), a1
pub fn i_06dfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DFB6;
    16
}
/// $06DFB4: move.l a1, (a6)
pub fn i_06dfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DFB6: jsr $27cee.l
pub fn i_06dfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DFBC);
    cpu.pc = 0x27CEE;
    16
}
/// $06DFBC: bcc.w $6dfc8
pub fn i_06dfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFC0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6DFC8; return 10; }
    8
}
/// $06DFC0: lea.l $31d26.l, a1
pub fn i_06dfc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFC6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31D26;
    20
}
/// $06DFC6: move.l a1, (a6)
pub fn i_06dfc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DFC8: jsr $28d70.l
pub fn i_06dfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DFCE);
    cpu.pc = 0x28D70;
    16
}
/// $06DFCE: jsr $6e2fe(pc)
pub fn i_06dfce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6DFD2);
    cpu.pc = 0x6E2FE;
    16
}
/// $06DFD2: btst.b #$1, $13(a6)
pub fn i_06dfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFD8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06DFD8: beq.w $6dfe4
pub fn i_06dfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFDC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6DFE4; return 10; }
    8
}
/// $06DFDC: lea.l $31d26.l, a1
pub fn i_06dfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFE2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31D26;
    20
}
/// $06DFE2: move.l a1, (a6)
pub fn i_06dfe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06DFE4: bra.w $6e15e
pub fn i_06dfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x6E15E;
    10
}
// ===== FireBurst_Low_06dfe8  ($06DFE8, 114 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06DFE8: bset.b #$4, $6b(a6)
pub fn i_06dfe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFEE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06DFEE: move.w #$1086, d0
pub fn i_06dfee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFF2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1086;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06DFF2: jsr $2352.l
pub fn i_06dff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6DFF8);
    cpu.pc = 0x2352;
    16
}
/// $06DFF8: eori.b #$1, $3a(a6)
pub fn i_06dff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6DFFE;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06DFFE: move.l #$2d3d9e, $4c(a6)
pub fn i_06dffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E006;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D3D9E;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06E006: move.w #$3, d1
pub fn i_06e006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E00A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E00A: jsr $236e.l
pub fn i_06e00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E010;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E010);
    cpu.pc = 0x236E;
    16
}
/// $06E010: addi.w #$10, $24(a6)
pub fn i_06e010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E016;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E016: clr.w $2e(a6)
pub fn i_06e016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E01A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E01A: move.w #$ff00, $2a(a6)
pub fn i_06e01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E020;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E020: jsr $27bc8.l
pub fn i_06e020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E026);
    cpu.pc = 0x27BC8;
    16
}
/// $06E026: bcc.b $6e020
pub fn i_06e026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E028;
    cpu.ir = 0x64F8;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E020; return 10; }
    8
}
/// $06E028: jsr $267e2.l
pub fn i_06e028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E02E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E02E);
    cpu.pc = 0x267E2;
    16
}
/// $06E02E: move.w #$180, d0
pub fn i_06e02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E032;
    cpu.ir = 0x303C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E032: jsr $6e20c(pc)
pub fn i_06e032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E036;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E036);
    cpu.pc = 0x6E20C;
    16
}
/// $06E036: move.w d0, $28(a6)
pub fn i_06e036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E03A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E03A: clr.w $72(a6)
pub fn i_06e03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E03E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E03E: jsr $283ca.l
pub fn i_06e03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E044;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E044);
    cpu.pc = 0x283CA;
    16
}
/// $06E044: move.w #$d000, d0
pub fn i_06e044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E048;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E048: jsr $28134.l
pub fn i_06e048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E04E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E04E);
    cpu.pc = 0x28134;
    16
}
/// $06E04E: andi.w #$ffe3, $38(a6)
pub fn i_06e04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E054;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E054: ori.w #$14, $38(a6)
pub fn i_06e054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E05A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetTaskHandler_06e05a  ($06E05A, 8 B, kernel/task_handlers.c)
/// $06E05A: lea.l $6e062(pc), a1
pub fn i_06e05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E05E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E062;
    16
}
/// $06E05E: move.l a1, (a6)
pub fn i_06e05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E060;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E060: rts
pub fn i_06e060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E062;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FireBurst_Fly_06e062  ($06E062, 56 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E062: jsr $27a92.l
pub fn i_06e062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E068;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E068);
    cpu.pc = 0x27A92;
    16
}
/// $06E068: jsr $28d70.l
pub fn i_06e068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E06E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E06E);
    cpu.pc = 0x28D70;
    16
}
/// $06E06E: jsr $6e2fe(pc)
pub fn i_06e06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E072;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E072);
    cpu.pc = 0x6E2FE;
    16
}
/// $06E072: cmpi.w #$32, $72(a6)
pub fn i_06e072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E078;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x32, Size::Word, false);
    16
}
/// $06E078: blt.w $6e084
pub fn i_06e078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E07C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6E084; return 10; }
    8
}
/// $06E07C: lea.l $31c20.l, a1
pub fn i_06e07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E082;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31C20;
    20
}
/// $06E082: move.l a1, (a6)
pub fn i_06e082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E084;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E084: btst.b #$1, $13(a6)
pub fn i_06e084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E08A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06E08A: beq.w $6e096
pub fn i_06e08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E08E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E096; return 10; }
    8
}
/// $06E08E: lea.l $31c20.l, a1
pub fn i_06e08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E094;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31C20;
    20
}
/// $06E094: move.l a1, (a6)
pub fn i_06e094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E096;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E096: bra.w $6e15e
pub fn i_06e096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E09A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6E15E;
    10
}
// ===== FireBurst_Arc_06e09a  ($06E09A, 188 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E09A: bset.b #$4, $6b(a6)
pub fn i_06e09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0A0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E0A0: move.w #$1086, d0
pub fn i_06e0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0A4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1086;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E0A4: jsr $2352.l
pub fn i_06e0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E0AA);
    cpu.pc = 0x2352;
    16
}
/// $06E0AA: move.l #$2d3cfa, $4c(a6)
pub fn i_06e0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0B2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D3CFA;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06E0B2: move.w #$58, d1
pub fn i_06e0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x58;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E0B6: jsr $236e.l
pub fn i_06e0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E0BC);
    cpu.pc = 0x236E;
    16
}
/// $06E0BC: jsr $5e9b6.l
pub fn i_06e0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E0C2);
    cpu.pc = 0x5E9B6;
    16
}
/// $06E0C2: andi.w #$3, d0
pub fn i_06e0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0C6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E0C6: move.w d0, d1
pub fn i_06e0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0C8;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E0C8: add.w d0, d0
pub fn i_06e0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0CA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E0CA: add.w d1, d0
pub fn i_06e0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0CC;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E0CC: add.w d0, d0
pub fn i_06e0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0CE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E0CE: ext.l d0
pub fn i_06e0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0D0;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $06E0D0: lea.l $2d4212.l, a0
pub fn i_06e0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4212;
    20
}
/// $06E0D6: adda.l d0, a0
pub fn i_06e0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0D8;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $06E0D8: move.w $2(a0), $2e(a6)
pub fn i_06e0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0DE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E0DE: move.w $4(a0), $2a(a6)
pub fn i_06e0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0E4;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E0E4: move.w (a0), d0
pub fn i_06e0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0E6;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $06E0E6: jsr $6e20c(pc)
pub fn i_06e0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E0EA);
    cpu.pc = 0x6E20C;
    16
}
/// $06E0EA: move.w d0, $28(a6)
pub fn i_06e0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0EE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E0EE: clr.w $72(a6)
pub fn i_06e0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0F2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E0F2: jsr $283ca.l
pub fn i_06e0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E0F8);
    cpu.pc = 0x283CA;
    16
}
/// $06E0F8: move.w #$d000, d0
pub fn i_06e0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E0FC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E0FC: jsr $28134.l
pub fn i_06e0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E102;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E102);
    cpu.pc = 0x28134;
    16
}
/// $06E102: andi.w #$ffe3, $38(a6)
pub fn i_06e102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E108;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E108: ori.w #$14, $38(a6)
pub fn i_06e108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E10E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E10E: lea.l $6e114(pc), a1
pub fn i_06e10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E112;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E114;
    16
}
/// $06E112: move.l a1, (a6)
pub fn i_06e112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E114;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E114: jsr $27d50.l
pub fn i_06e114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E11A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E11A);
    cpu.pc = 0x27D50;
    16
}
/// $06E11A: bcc.w $6e126
pub fn i_06e11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E11E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E126; return 10; }
    8
}
/// $06E11E: lea.l $31c20.l, a1
pub fn i_06e11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E124;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31C20;
    20
}
/// $06E124: move.l a1, (a6)
pub fn i_06e124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E126;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E126: jsr $28d70.l
pub fn i_06e126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E12C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E12C);
    cpu.pc = 0x28D70;
    16
}
/// $06E12C: jsr $6e2fe(pc)
pub fn i_06e12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E130;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E130);
    cpu.pc = 0x6E2FE;
    16
}
/// $06E130: btst.b #$1, $13(a6)
pub fn i_06e130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E136;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06E136: beq.w $6e140
pub fn i_06e136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E13A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E140; return 10; }
    8
}
/// $06E13A: lea.l $6da9e(pc), a1
pub fn i_06e13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E13E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA9E;
    16
}
/// $06E13E: move.l a1, (a6)
pub fn i_06e13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E140;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E140: movea.l #$ffffffff, a0
pub fn i_06e140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E146;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06E146: lea.l $2d4046.l, a0
pub fn i_06e146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E14C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4046;
    20
}
/// $06E14C: jsr $5dd56.l
pub fn i_06e14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E152;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E152);
    cpu.pc = 0x5DD56;
    16
}
/// $06E152: bcc.w $6e15c
pub fn i_06e152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E156;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E15C; return 10; }
    8
}
// ===== SetTaskHandler_06e156  ($06E156, 8 B, kernel/task_handlers.c)
/// $06E156: lea.l $6da90(pc), a1
pub fn i_06e156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E15A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06E15A: move.l a1, (a6)
pub fn i_06e15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E15C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E15C: rts
pub fn i_06e15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E15E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FireBurst_FreeIfOffWorld_06e15e  ($06E15E, 16 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E15E: movea.l #$ffffffff, a0
pub fn i_06e15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E164;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06E164: jsr $5dd56.l
pub fn i_06e164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E16A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E16A);
    cpu.pc = 0x5DD56;
    16
}
/// $06E16A: bcc.w $6e174
pub fn i_06e16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E16E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E174; return 10; }
    8
}
// ===== SetTaskHandler_06e16e  ($06E16E, 8 B, kernel/task_handlers.c)
/// $06E16E: lea.l $6da90(pc), a1
pub fn i_06e16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E172;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DA90;
    16
}
/// $06E172: move.l a1, (a6)
pub fn i_06e172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E174;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E174: rts
pub fn i_06e174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E176;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_PickBurstVel_06e176  ($06E176, 150 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E176: lea.l $2b8f74.l, a0
pub fn i_06e176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E17C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8F74;
    20
}
/// $06E17C: jsr $799de.l
pub fn i_06e17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E182;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E182);
    cpu.pc = 0x799DE;
    16
}
/// $06E182: andi.w #$3, d0
pub fn i_06e182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E186;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E186: cmpi.w #$2, $70(a6)
pub fn i_06e186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E18C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $06E18C: bne.w $6e194
pub fn i_06e18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E190;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E194; return 10; }
    8
}
/// $06E190: move.w #$4, d0
pub fn i_06e190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E194;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E194: lsl.w #$2, d0
pub fn i_06e194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E196;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06E196: lea.l $2d4186.l, a0
pub fn i_06e196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E19C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4186;
    20
}
/// $06E19C: move.l (a0, d0.w), $5c(a6)
pub fn i_06e19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1A2;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $06E1A2: lea.l $2d4036.l, a0
pub fn i_06e1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4036;
    20
}
/// $06E1A8: jsr $5e086.l
pub fn i_06e1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E1AE);
    cpu.pc = 0x5E086;
    16
}
/// $06E1AE: bcc.w $6e1b8
pub fn i_06e1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E1B8; return 10; }
    8
}
/// $06E1B2: jsr $5e0d4.l
pub fn i_06e1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E1B8);
    cpu.pc = 0x5E0D4;
    16
}
/// $06E1B8: move.w $22(a6), d0
pub fn i_06e1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1BC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06E1BC: sub.w $22(a0), d0
pub fn i_06e1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1C0;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06E1C0: cmpi.w #$60, d0
pub fn i_06e1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1C4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $06E1C4: bgt.w $6e1cc
pub fn i_06e1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1C8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E1CC; return 10; }
    8
}
/// $06E1C8: move.w #$60, d0
pub fn i_06e1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1CC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E1CC: subi.w #$60, d0
pub fn i_06e1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1D0;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x60, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E1D0: lsr.w #$4, d0
pub fn i_06e1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1D2;
    cpu.ir = 0xE848;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06E1D2: cmpi.w #$3, d0
pub fn i_06e1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1D6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $06E1D6: bcs.w $6e1de
pub fn i_06e1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1DA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6E1DE; return 10; }
    8
}
/// $06E1DA: move.w #$3, d0
pub fn i_06e1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E1DE: move.w d0, d1
pub fn i_06e1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1E0;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E1E0: add.w d0, d0
pub fn i_06e1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1E2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E1E2: add.w d1, d0
pub fn i_06e1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1E4;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E1E4: add.w d0, d0
pub fn i_06e1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1E6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E1E6: movea.l $5c(a6), a0
pub fn i_06e1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1EA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06E1EA: adda.l d0, a0
pub fn i_06e1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1EC;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $06E1EC: move.w (a0), $28(a6)
pub fn i_06e1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1F0;
    cpu.ir = 0x3D50;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad3, v2 as u16);
    16
}
/// $06E1F0: move.w $2(a0), $2e(a6)
pub fn i_06e1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1F6;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E1F6: move.w $4(a0), $2a(a6)
pub fn i_06e1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E1FC;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E1FC: btst.b #$0, $3a(a6)
pub fn i_06e1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E202;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06E202: beq.w $6e20a
pub fn i_06e202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E206;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E20A; return 10; }
    8
}
/// $06E206: neg.w $28(a6)
pub fn i_06e206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E20A;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E20A: rts
pub fn i_06e20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E20C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Facing_NegIfLeft_06e20c  ($06E20C, 12 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E20C: btst.b #$0, $3a(a6)
pub fn i_06e20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E212;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06E212: beq.w $6e21e
pub fn i_06e212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E216;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E21E; return 10; }
    8
}
/// $06E216: neg.w d0
pub fn i_06e216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E218;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_06e218  ($06E218, 6 B, thunks/ccr_helpers.c)
/// $06E218: ori.b #$11, ccr
pub fn i_06e218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E21C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06E21C: rts
pub fn i_06e21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E21E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06e21e  ($06E21E, 6 B, thunks/ccr_helpers.c)
/// $06E21E: andi.b #$ee, ccr
pub fn i_06e21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E222;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06E222: rts
pub fn i_06e222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E224;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SpawnAndTag_06E224  ($06E224, 38 B, entity/spawn_and_tag_06e224.s)
/// $06E224: movem.w d0, -(a7)
pub fn i_06e224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E228;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $06E228: lea.l $6df32(pc), a1
pub fn i_06e228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E22C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DF32;
    16
}
/// $06E22C: jsr $4ae.l
pub fn i_06e22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E232;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E232);
    cpu.pc = 0x4AE;
    16
}
/// $06E232: jsr $5dd02.l
pub fn i_06e232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E238;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E238);
    cpu.pc = 0x5DD02;
    16
}
/// $06E238: movem.w (a7)+, d0
pub fn i_06e238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E23C;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06E23C: andi.b #$1, d0
pub fn i_06e23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E240;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06E240: move.b d0, $3a(a0)
pub fn i_06e240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E244;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E244: jsr $6e2bc(pc)
pub fn i_06e244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E248;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E248);
    cpu.pc = 0x6E2BC;
    16
}
/// $06E248: rts
pub fn i_06e248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E24A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_FireBurstWide_06e24a  ($06E24A, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E24A: movem.w d0, -(a7)
pub fn i_06e24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E24E;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $06E24E: lea.l $6df06(pc), a1
pub fn i_06e24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E252;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DF06;
    16
}
/// $06E252: jsr $4ae.l
pub fn i_06e252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E258;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E258);
    cpu.pc = 0x4AE;
    16
}
/// $06E258: jsr $5dd02.l
pub fn i_06e258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E25E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E25E);
    cpu.pc = 0x5DD02;
    16
}
/// $06E25E: movem.w (a7)+, d0
pub fn i_06e25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E262;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06E262: andi.b #$1, d0
pub fn i_06e262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E266;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06E266: move.b d0, $3a(a0)
pub fn i_06e266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E26A;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_06e26a  ($06E26A, 6 B, thunks/jsr_pc_thunks.c)
/// $06E26A: jsr $6e2bc(pc)
pub fn i_06e26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E26E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E26E);
    cpu.pc = 0x6E2BC;
    16
}
/// $06E26E: rts
pub fn i_06e26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E270;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_FireBurstArc_06e270  ($06E270, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E270: movem.w d0, -(a7)
pub fn i_06e270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E274;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $06E274: lea.l $6e09a(pc), a1
pub fn i_06e274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E278;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E09A;
    16
}
/// $06E278: jsr $4ae.l
pub fn i_06e278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E27E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E27E);
    cpu.pc = 0x4AE;
    16
}
/// $06E27E: jsr $5dd02.l
pub fn i_06e27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E284;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E284);
    cpu.pc = 0x5DD02;
    16
}
/// $06E284: movem.w (a7)+, d0
pub fn i_06e284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E288;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06E288: andi.b #$1, d0
pub fn i_06e288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E28C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06E28C: move.b d0, $3a(a0)
pub fn i_06e28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E290;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_06e290  ($06E290, 6 B, thunks/jsr_pc_thunks.c)
/// $06E290: jsr $6e2bc(pc)
pub fn i_06e290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E294;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E294);
    cpu.pc = 0x6E2BC;
    16
}
/// $06E294: rts
pub fn i_06e294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E296;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_FireBurstLow_06e296  ($06E296, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E296: movem.w d0, -(a7)
pub fn i_06e296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E29A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $06E29A: lea.l $6dfe8(pc), a1
pub fn i_06e29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E29E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DFE8;
    16
}
/// $06E29E: jsr $4ae.l
pub fn i_06e29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E2A4);
    cpu.pc = 0x4AE;
    16
}
/// $06E2A4: jsr $5dd02.l
pub fn i_06e2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E2AA);
    cpu.pc = 0x5DD02;
    16
}
/// $06E2AA: movem.w (a7)+, d0
pub fn i_06e2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2AE;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06E2AE: andi.b #$1, d0
pub fn i_06e2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2B2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06E2B2: move.b d0, $3a(a0)
pub fn i_06e2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2B6;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_06e2b6  ($06E2B6, 6 B, thunks/jsr_pc_thunks.c)
/// $06E2B6: jsr $6e2bc(pc)
pub fn i_06e2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E2BA);
    cpu.pc = 0x6E2BC;
    16
}
/// $06E2BA: rts
pub fn i_06e2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CopyAnimFromLeader_06E2BC  ($06E2BC, 66 B, entity/copy_anim_from_leader_06e2bc.s)
/// $06E2BC: movea.l $50(a6), a1
pub fn i_06e2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2C0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06E2C0: move.w $22(a1), $22(a0)
pub fn i_06e2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2C6;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E2C6: move.w $24(a1), $24(a0)
pub fn i_06e2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2CC;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E2CC: move.w $38(a1), $38(a0)
pub fn i_06e2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2D2;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E2D2: move.w $30(a1), $30(a0)
pub fn i_06e2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2D8;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E2D8: move.b $3b(a1), $3b(a0)
pub fn i_06e2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2DE;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E2DE: move.b $46(a1), $46(a0)
pub fn i_06e2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2E4;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x46);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x46);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E2E4: move.b $47(a1), $47(a0)
pub fn i_06e2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2EA;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x47);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x47);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E2EA: move.l $3c(a1), $3c(a0)
pub fn i_06e2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2F0;
    cpu.ir = 0x2169;
    let ad2 = cpu.a[1].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $06E2F0: move.l $40(a1), $40(a0)
pub fn i_06e2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2F6;
    cpu.ir = 0x2169;
    let ad2 = cpu.a[1].wrapping_add(0x40);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0].wrapping_add(0x40);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $06E2F6: move.w $30(a1), $30(a0)
pub fn i_06e2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2FC;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E2FC: rts
pub fn i_06e2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E2FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FireBurst_TickHit_06e2fe  ($06E2FE, 20 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E2FE: addq.w #$1, $72(a6)
pub fn i_06e2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E302;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E302: cmpi.w #$4, $72(a6)
pub fn i_06e302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E308;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $06E308: ble.w $6e318
pub fn i_06e308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E30C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6E318; return 10; }
    8
}
/// $06E30C: jsr $283d8.l
pub fn i_06e30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E312;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E312);
    cpu.pc = 0x283D8;
    16
}
// ===== ClearXN_06e312  ($06E312, 6 B, thunks/ccr_helpers.c)
/// $06E312: andi.b #$ee, ccr
pub fn i_06e312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E316;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06E316: rts
pub fn i_06e316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E318;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06e318  ($06E318, 6 B, thunks/ccr_helpers.c)
/// $06E318: ori.b #$11, ccr
pub fn i_06e318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E31C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06E31C: rts
pub fn i_06e31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E31E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_PlayCry_06e31e  ($06E31E, 44 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E31E: lea.l $2d4284.l, a0
pub fn i_06e31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E324;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4284;
    20
}
/// $06E324: move.b $9b(a6), d0
pub fn i_06e324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E328;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E328: andi.w #$3, d0
pub fn i_06e328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E32C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E32C: add.w d0, d0
pub fn i_06e32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E32E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E32E: move.w (a0, d0.w), d1
pub fn i_06e32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E332;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06E332: jsr $236e.l
pub fn i_06e332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E338);
    cpu.pc = 0x236E;
    16
}
/// $06E338: move.w #$1, d1
pub fn i_06e338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E33C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E33C: jsr $236e.l
pub fn i_06e33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E342;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E342);
    cpu.pc = 0x236E;
    16
}
/// $06E342: move.w $16(a6), $14(a6)
pub fn i_06e342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E348;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E348: rts
pub fn i_06e348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E34A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CopyParentAnimTimer_06e34a  ($06E34A, 12 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E34A: movea.l $c(a6), a0
pub fn i_06e34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E34E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06E34E: move.w $14(a0), $14(a6)
pub fn i_06e34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E354;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E354: rts
pub fn i_06e354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E356;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_PlaySnd157To159_06e356  ($06E356, 38 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E356: move.w #$157, d1
pub fn i_06e356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E35A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x157;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E35A: jsr $236e.l
pub fn i_06e35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E360;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E360);
    cpu.pc = 0x236E;
    16
}
/// $06E360: move.w #$158, d1
pub fn i_06e360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E364;
    cpu.ir = 0x323C;
    let v1: u32 = 0x158;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E364: jsr $236e.l
pub fn i_06e364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E36A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E36A);
    cpu.pc = 0x236E;
    16
}
/// $06E36A: move.w #$159, d1
pub fn i_06e36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E36E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x159;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E36E: jsr $236e.l
pub fn i_06e36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E374);
    cpu.pc = 0x236E;
    16
}
/// $06E374: move.w $16(a6), $14(a6)
pub fn i_06e374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E37A;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E37A: rts
pub fn i_06e37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E37C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_PlaySnd47Or144_06e37c  ($06E37C, 16 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E37C: move.w #$47, d1
pub fn i_06e37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E380;
    cpu.ir = 0x323C;
    let v1: u32 = 0x47;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E380: tst.b $9e(a6)
pub fn i_06e380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E384;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E384: beq.w $6e38c
pub fn i_06e384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E388;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E38C; return 10; }
    8
}
/// $06E388: move.w #$144, d1
pub fn i_06e388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E38C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x144;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_06e38c  ($06E38C, 8 B, thunks/jsr_abs_thunks.c)
/// $06E38C: jsr $236e.l
pub fn i_06e38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E392;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E392);
    cpu.pc = 0x236E;
    16
}
/// $06E392: rts
pub fn i_06e392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E394;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_ClearFlag10E39A_06e394  ($06E394, 8 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E394: tst.b $9c(a6)
pub fn i_06e394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E398;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E398: beq.w $6e3a2
pub fn i_06e398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E39C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E3A2; return 10; }
    8
}
// ===== ClrRamWord_06e39c  ($06E39C, 8 B, thunks/clr_ram_word.c)
/// $06E39C: clr.b $10e39a.l
pub fn i_06e39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3A2;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $06E3A2: rts
pub fn i_06e3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_SpawnSparkAndDebris_06e3a4  ($06E3A4, 110 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E3A4: move.w #$1065, d0
pub fn i_06e3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3A8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1065;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E3A8: jsr $2352.l
pub fn i_06e3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E3AE);
    cpu.pc = 0x2352;
    16
}
/// $06E3AE: lea.l $6dc90(pc), a1
pub fn i_06e3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DC90;
    16
}
/// $06E3B2: jsr $4ae.l
pub fn i_06e3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E3B8);
    cpu.pc = 0x4AE;
    16
}
/// $06E3B8: jsr $5dd02.l
pub fn i_06e3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E3BE);
    cpu.pc = 0x5DD02;
    16
}
/// $06E3BE: move.w #$d000, $38(a0)
pub fn i_06e3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3C4;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E3C4: move.w #$ffec, d0
pub fn i_06e3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3C8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFEC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E3C8: jsr $6e20c(pc)
pub fn i_06e3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E3CC);
    cpu.pc = 0x6E20C;
    16
}
/// $06E3CC: add.w d0, $22(a0)
pub fn i_06e3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3D0;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E3D0: addi.w #$c, $24(a0)
pub fn i_06e3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3D6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E3D6: lea.l $6dbde(pc), a1
pub fn i_06e3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DBDE;
    16
}
/// $06E3DA: jsr $4ae.l
pub fn i_06e3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E3E0);
    cpu.pc = 0x4AE;
    16
}
/// $06E3E0: jsr $5dd02.l
pub fn i_06e3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E3E6);
    cpu.pc = 0x5DD02;
    16
}
/// $06E3E6: move.b $9f(a6), $89(a0)
pub fn i_06e3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3EC;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x89);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E3EC: move.w $70(a6), $70(a0)
pub fn i_06e3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3F2;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06E3F2: move.b $9e(a6), $9e(a0)
pub fn i_06e3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3F8;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E3F8: move.w #$d000, $38(a0)
pub fn i_06e3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E3FE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E3FE: move.w #$ffd0, d0
pub fn i_06e3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E402;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E402: jsr $6e20c(pc)
pub fn i_06e402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E406;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E406);
    cpu.pc = 0x6E20C;
    16
}
/// $06E406: add.w d0, $22(a0)
pub fn i_06e406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E40A;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E40A: addi.w #$2c, $24(a0)
pub fn i_06e40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E410;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E410: rts
pub fn i_06e410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E412;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SpawnLoop16_06E412  ($06E412, 36 B, entity/spawn_loop16_06e412.s)
/// $06E412: move.w #$10, d0
pub fn i_06e412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E416;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E416: move.w d0, -(a7)
pub fn i_06e416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E418;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $06E418: lea.l $6dd5c(pc), a1
pub fn i_06e418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E41C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DD5C;
    16
}
/// $06E41C: jsr $4ae.l
pub fn i_06e41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E422);
    cpu.pc = 0x4AE;
    16
}
/// $06E422: jsr $5dd02.l
pub fn i_06e422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E428;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E428);
    cpu.pc = 0x5DD02;
    16
}
/// $06E428: move.b $9b(a6), $9b(a0)
pub fn i_06e428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E42E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E42E: move.w (a7)+, d0
pub fn i_06e42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E430;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $06E430: subq.w #$1, d0
pub fn i_06e430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E432;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06E432: bne.b $6e416
pub fn i_06e432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E434;
    cpu.ir = 0x66E2;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E416; return 10; }
    8
}
/// $06E434: rts
pub fn i_06e434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E436;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_ExplosionAbove_06e436  ($06E436, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E436: lea.l $77fd6.l, a1
pub fn i_06e436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E43C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06E43C: jsr $4ae.l
pub fn i_06e43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E442);
    cpu.pc = 0x4AE;
    16
}
/// $06E442: jsr $5dd02.l
pub fn i_06e442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E448);
    cpu.pc = 0x5DD02;
    16
}
/// $06E448: move.w #$8000, $38(a0)
pub fn i_06e448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E44E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E44E: addi.w #$10, $24(a0)
pub fn i_06e44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E454;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E454: rts
pub fn i_06e454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E456;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_Fx7801E_06e456  ($06E456, 26 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E456: lea.l $7801e.l, a1
pub fn i_06e456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E45C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7801E;
    20
}
/// $06E45C: jsr $4ae.l
pub fn i_06e45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E462;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E462);
    cpu.pc = 0x4AE;
    16
}
/// $06E462: jsr $5dd02.l
pub fn i_06e462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E468);
    cpu.pc = 0x5DD02;
    16
}
/// $06E468: move.w #$2000, $38(a0)
pub fn i_06e468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E46E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E46E: rts
pub fn i_06e46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E470;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_Fx2D40A2_06e470  ($06E470, 20 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E470: lea.l $2d40a2.l, a1
pub fn i_06e470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E476;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D40A2;
    20
}
/// $06E476: jsr $77c7e.l
pub fn i_06e476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E47C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E47C);
    cpu.pc = 0x77C7E;
    16
}
/// $06E47C: move.w #$8000, $38(a0)
pub fn i_06e47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E482;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E482: rts
pub fn i_06e482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E484;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_SetSpriteByFlags7E7F_06e484  ($06E484, 28 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E484: tst.b $7e(a6)
pub fn i_06e484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E488;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E488: beq.w $6e4a6
pub fn i_06e488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E48C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E4A6; return 10; }
    8
}
/// $06E48C: lea.l $2d422a.l, a1
pub fn i_06e48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E492;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D422A;
    20
}
/// $06E492: tst.b $7f(a6)
pub fn i_06e492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E496;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E496: beq.w $6e4a0
pub fn i_06e496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E49A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E4A0; return 10; }
    8
}
/// $06E49A: lea.l $2d424a.l, a1
pub fn i_06e49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4A0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D424A;
    20
}
// ===== JsrAbsThunk_06e4a0  ($06E4A0, 8 B, thunks/jsr_abs_thunks.c)
/// $06E4A0: jsr $43fac.l
pub fn i_06e4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E4A6);
    cpu.pc = 0x43FAC;
    16
}
/// $06E4A6: rts
pub fn i_06e4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_Spawn12Scatter_06e4a8  ($06E4A8, 36 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E4A8: move.w #$c, d0
pub fn i_06e4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4AC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E4AC: move.w d0, -(a7)
pub fn i_06e4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4AE;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $06E4AE: lea.l $6dd86(pc), a1
pub fn i_06e4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DD86;
    16
}
/// $06E4B2: jsr $4ae.l
pub fn i_06e4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E4B8);
    cpu.pc = 0x4AE;
    16
}
/// $06E4B8: jsr $5dd02.l
pub fn i_06e4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E4BE);
    cpu.pc = 0x5DD02;
    16
}
/// $06E4BE: move.b $9b(a6), $9b(a0)
pub fn i_06e4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4C4;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E4C4: move.w (a7)+, d0
pub fn i_06e4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4C6;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $06E4C6: subq.w #$1, d0
pub fn i_06e4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4C8;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06E4C8: bne.b $6e4ac
pub fn i_06e4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4CA;
    cpu.ir = 0x66E2;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E4AC; return 10; }
    8
}
/// $06E4CA: rts
pub fn i_06e4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_SpawnSmokeCycle_06e4cc  ($06E4CC, 66 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E4CC: lea.l $6dc5e(pc), a1
pub fn i_06e4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6DC5E;
    16
}
/// $06E4D0: jsr $6fe.l
pub fn i_06e4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E4D6);
    cpu.pc = 0x6FE;
    16
}
/// $06E4D6: jsr $5dd02.l
pub fn i_06e4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E4DC);
    cpu.pc = 0x5DD02;
    16
}
/// $06E4DC: move.b $9e(a6), $9e(a0)
pub fn i_06e4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4E2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E4E2: subq.w #$1, $38(a0)
pub fn i_06e4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4E6;
    cpu.ir = 0x5368;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E4E6: move.b $88(a6), d0
pub fn i_06e4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4EA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E4EA: addq.b #$1, d0
pub fn i_06e4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4EC;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $06E4EC: andi.w #$3, d0
pub fn i_06e4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4F0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E4F0: cmpi.w #$3, d0
pub fn i_06e4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4F4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $06E4F4: blt.w $6e4fa
pub fn i_06e4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4F8;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6E4FA; return 10; }
    8
}
/// $06E4F8: clr.w d0
pub fn i_06e4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4FA;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06E4FA: move.w d0, d1
pub fn i_06e4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E4FC;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E4FC: move.b d0, $88(a6)
pub fn i_06e4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E500;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E500: add.w d1, d1
pub fn i_06e500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E502;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E502: addi.w #$16, d1
pub fn i_06e502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E506;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x16, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E506: move.w (a6, d1.w), $14(a6)
pub fn i_06e506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E50C;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $06E50C: rts
pub fn i_06e50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E50E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Walker_CmpSlotWithParent_06e50e  ($06E50E, 16 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E50E: movea.l $8(a6), a1
pub fn i_06e50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E512;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06E512: move.b $10(a6), d0
pub fn i_06e512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E516;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E516: cmp.b $10(a1), d0
pub fn i_06e516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E51A;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06E51A: bcs.w $6e524
pub fn i_06e51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E51E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6E524; return 10; }
    8
}
// ===== ClearXN_06e51e  ($06E51E, 6 B, thunks/ccr_helpers.c)
/// $06E51E: andi.b #$ee, ccr
pub fn i_06e51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E522;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06E522: rts
pub fn i_06e522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E524;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06e524  ($06E524, 6 B, thunks/ccr_helpers.c)
/// $06E524: ori.b #$11, ccr
pub fn i_06e524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E528;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06E528: rts
pub fn i_06e528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E52A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Tmpl4B_06e52a  ($06E52A, 314 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E52A: move.w #$3b, d1
pub fn i_06e52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E52E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E52E: jsr $236e.l
pub fn i_06e52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E534;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E534);
    cpu.pc = 0x236E;
    16
}
/// $06E534: move.w #$9, $1c(a6)
pub fn i_06e534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E53A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E53A: jsr $138fe.l
pub fn i_06e53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E540;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E540);
    cpu.pc = 0x138FE;
    16
}
/// $06E540: lea.l $2d4a4e.l, a0
pub fn i_06e540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E546;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4A4E;
    20
}
/// $06E546: move.b $9a(a6), d0
pub fn i_06e546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E54A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E54A: andi.w #$3, d0
pub fn i_06e54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E54E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E54E: lsl.w #$2, d0
pub fn i_06e54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E550;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06E550: movea.l (a0, d0.w), a0
pub fn i_06e550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E554;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06E554: jsr $799de.l
pub fn i_06e554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E55A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E55A);
    cpu.pc = 0x799DE;
    16
}
/// $06E55A: move.w d0, $72(a6)
pub fn i_06e55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E55E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E55E: lea.l $2b91fe.l, a0
pub fn i_06e55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E564;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B91FE;
    20
}
/// $06E564: jsr $799de.l
pub fn i_06e564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E56A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E56A);
    cpu.pc = 0x799DE;
    16
}
/// $06E56A: move.w d0, $66(a6)
pub fn i_06e56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E56E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E56E: move.w #$c000, d0
pub fn i_06e56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E572;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E572: jsr $28134.l
pub fn i_06e572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E578;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E578);
    cpu.pc = 0x28134;
    16
}
/// $06E578: andi.w #$ffe3, $38(a6)
pub fn i_06e578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E57E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E57E: ori.w #$14, $38(a6)
pub fn i_06e57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E584;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E584: move.l #$2d46c6, $48(a6)
pub fn i_06e584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E58C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D46C6;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06E58C: clr.b $20(a6)
pub fn i_06e58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E590;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06E590: jsr $6eeae(pc)
pub fn i_06e590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E594;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E594);
    cpu.pc = 0x6EEAE;
    16
}
/// $06E594: lea.l $723da.l, a1
pub fn i_06e594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E59A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x723DA;
    20
}
/// $06E59A: jsr $4ae.l
pub fn i_06e59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E5A0);
    cpu.pc = 0x4AE;
    16
}
/// $06E5A0: jsr $5dd02.l
pub fn i_06e5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E5A6);
    cpu.pc = 0x5DD02;
    16
}
/// $06E5A6: move.w #$ffe0, $98(a0)
pub fn i_06e5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5AC;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E5AC: lea.l $6e966(pc), a1
pub fn i_06e5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E966;
    16
}
/// $06E5B0: jsr $4ae.l
pub fn i_06e5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E5B6);
    cpu.pc = 0x4AE;
    16
}
/// $06E5B6: jsr $5dd02.l
pub fn i_06e5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E5BC);
    cpu.pc = 0x5DD02;
    16
}
/// $06E5BC: lea.l $6e9c0(pc), a1
pub fn i_06e5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E9C0;
    16
}
/// $06E5C0: jsr $4ae.l
pub fn i_06e5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E5C6);
    cpu.pc = 0x4AE;
    16
}
/// $06E5C6: jsr $5dd02.l
pub fn i_06e5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E5CC);
    cpu.pc = 0x5DD02;
    16
}
/// $06E5CC: move.b $99(a6), $99(a0)
pub fn i_06e5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5D2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E5D2: move.b $9b(a6), $9b(a0)
pub fn i_06e5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5D8;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E5D8: move.b $9d(a6), $9d(a0)
pub fn i_06e5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5DE;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9D);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E5DE: move.b $9e(a6), $9e(a0)
pub fn i_06e5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5E4;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06E5E4: lea.l $6e5ea(pc), a1
pub fn i_06e5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E5EA;
    16
}
/// $06E5E8: move.l a1, (a6)
pub fn i_06e5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E5EA: tst.b $9d(a6)
pub fn i_06e5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5EE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E5EE: beq.w $6e5f8
pub fn i_06e5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E5F8; return 10; }
    8
}
/// $06E5F2: bset.b #$6, $13(a6)
pub fn i_06e5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5F8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E5F8: jsr $6f094(pc)
pub fn i_06e5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E5FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E5FC);
    cpu.pc = 0x6F094;
    16
}
/// $06E5FC: jsr $78f8a.l
pub fn i_06e5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E602);
    cpu.pc = 0x78F8A;
    16
}
/// $06E602: bcc.w $6e60c
pub fn i_06e602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E606;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E60C; return 10; }
    8
}
/// $06E606: lea.l $6e770(pc), a1
pub fn i_06e606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E60A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E770;
    16
}
/// $06E60A: move.l a1, (a6)
pub fn i_06e60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E60C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E60C: jsr $6ef0e(pc)
pub fn i_06e60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E610;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E610);
    cpu.pc = 0x6EF0E;
    16
}
/// $06E610: jsr $28d70.l
pub fn i_06e610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E616;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E616);
    cpu.pc = 0x28D70;
    16
}
/// $06E616: jsr $2870a.l
pub fn i_06e616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E61C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E61C);
    cpu.pc = 0x2870A;
    16
}
/// $06E61C: bcc.w $6e63e
pub fn i_06e61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E620;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E63E; return 10; }
    8
}
/// $06E620: bclr.b #$3, $13(a6)
pub fn i_06e620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E626;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E626: lea.l $2d4d20.l, a0
pub fn i_06e626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E62C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4D20;
    20
}
/// $06E62C: jsr $28cd4.l
pub fn i_06e62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E632;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E632);
    cpu.pc = 0x28CD4;
    16
}
/// $06E632: lea.l $5e766.l, a0
pub fn i_06e632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E638;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06E638: jsr $5e770.l
pub fn i_06e638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E63E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E63E);
    cpu.pc = 0x5E770;
    16
}
/// $06E63E: jsr $28758.l
pub fn i_06e63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E644;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E644);
    cpu.pc = 0x28758;
    16
}
/// $06E644: bcc.w $6e64e
pub fn i_06e644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E648;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E64E; return 10; }
    8
}
/// $06E648: lea.l $6e66c(pc), a1
pub fn i_06e648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E64C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E66C;
    16
}
/// $06E64C: move.l a1, (a6)
pub fn i_06e64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E64E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E64E: movea.l #$ffffffff, a0
pub fn i_06e64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E654;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06E654: lea.l $2d49b6.l, a0
pub fn i_06e654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E65A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D49B6;
    20
}
/// $06E65A: jsr $5dd5c.l
pub fn i_06e65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E660;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E660);
    cpu.pc = 0x5DD5C;
    16
}
/// $06E660: bcc.w $6e66a
pub fn i_06e660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E664;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E66A; return 10; }
    8
}
// ===== SetTaskHandler_06e664  ($06E664, 8 B, kernel/task_handlers.c)
/// $06E664: lea.l $6e80a(pc), a1
pub fn i_06e664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E668;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E80A;
    16
}
/// $06E668: move.l a1, (a6)
pub fn i_06e668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E66A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E66A: rts
pub fn i_06e66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E66C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Attack_06e66c  ($06E66C, 222 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E66C: move.b $9c(a6), d0
pub fn i_06e66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E670;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E670: cmpi.b #$2, d0
pub fn i_06e670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E674;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $06E674: bne.w $6e682
pub fn i_06e674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E678;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E682; return 10; }
    8
}
/// $06E678: jsr $5e9b6.l
pub fn i_06e678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E67E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E67E);
    cpu.pc = 0x5E9B6;
    16
}
/// $06E67E: andi.b #$1, d0
pub fn i_06e67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E682;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06E682: move.b d0, $9c(a6)
pub fn i_06e682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E686;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E686: tst.b d0
pub fn i_06e686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E688;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $06E688: bne.w $6e770
pub fn i_06e688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E68C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E770; return 10; }
    8
}
/// $06E68C: move.w #$10b8, d0
pub fn i_06e68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E690;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E690: jsr $2352.l
pub fn i_06e690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E696;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E696);
    cpu.pc = 0x2352;
    16
}
/// $06E696: move.w #$c000, d0
pub fn i_06e696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E69A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E69A: jsr $28134.l
pub fn i_06e69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E6A0);
    cpu.pc = 0x28134;
    16
}
/// $06E6A0: andi.w #$ffe3, $38(a6)
pub fn i_06e6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6A6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E6A6: ori.w #$1c, $38(a6)
pub fn i_06e6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6AC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E6AC: move.l #$2d4816, $4c(a6)
pub fn i_06e6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6B4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D4816;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06E6B4: jsr $283ca.l
pub fn i_06e6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E6BA);
    cpu.pc = 0x283CA;
    16
}
/// $06E6BA: clr.w $2a(a6)
pub fn i_06e6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6BE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E6BE: move.w #$ffe0, $2e(a6)
pub fn i_06e6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E6C4: move.w $36(a6), d0
pub fn i_06e6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6C8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06E6C8: cmpi.w #$600, d0
pub fn i_06e6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6CC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x600, Size::Word, false);
    8
}
/// $06E6CC: ble.w $6e6d4
pub fn i_06e6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6D0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6E6D4; return 10; }
    8
}
/// $06E6D0: move.w #$600, d0
pub fn i_06e6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6D4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E6D4: btst.b #$0, $3a(a6)
pub fn i_06e6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6DA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06E6DA: bne.w $6e6e0
pub fn i_06e6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E6E0; return 10; }
    8
}
/// $06E6DE: neg.w d0
pub fn i_06e6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6E0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06E6E0: move.w d0, $28(a6)
pub fn i_06e6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6E4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E6E4: lea.l $2d4c9c.l, a0
pub fn i_06e6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4C9C;
    20
}
/// $06E6EA: jsr $28cd4.l
pub fn i_06e6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E6F0);
    cpu.pc = 0x28CD4;
    16
}
/// $06E6F0: lea.l $6e6f6(pc), a1
pub fn i_06e6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E6F6;
    16
}
/// $06E6F4: move.l a1, (a6)
pub fn i_06e6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E6F6: jsr $27d50.l
pub fn i_06e6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E6FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E6FC);
    cpu.pc = 0x27D50;
    16
}
/// $06E6FC: bcc.w $6e706
pub fn i_06e6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E700;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E706; return 10; }
    8
}
/// $06E700: lea.l $6e770(pc), a1
pub fn i_06e700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E704;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E770;
    16
}
/// $06E704: move.l a1, (a6)
pub fn i_06e704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E706;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E706: jsr $6ef0e(pc)
pub fn i_06e706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E70A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6E70A);
    cpu.pc = 0x6EF0E;
    16
}
/// $06E70A: jsr $28d70.l
pub fn i_06e70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E710);
    cpu.pc = 0x28D70;
    16
}
/// $06E710: jsr $283ca.l
pub fn i_06e710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E716;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E716);
    cpu.pc = 0x283CA;
    16
}
/// $06E716: jsr $283d8.l
pub fn i_06e716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E71C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E71C);
    cpu.pc = 0x283D8;
    16
}
/// $06E71C: tst.b $9b(a6)
pub fn i_06e71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E720;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E720: beq.w $6e734
pub fn i_06e720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E724;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E734; return 10; }
    8
}
/// $06E724: cmpi.w #$120, $24(a6)
pub fn i_06e724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E72A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $06E72A: bgt.w $6e734
pub fn i_06e72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E72E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E734; return 10; }
    8
}
/// $06E72E: lea.l $6e752(pc), a1
pub fn i_06e72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E732;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E752;
    16
}
/// $06E732: move.l a1, (a6)
pub fn i_06e732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E734;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E734: movea.l #$ffffffff, a0
pub fn i_06e734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E73A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06E73A: lea.l $2d49b6.l, a0
pub fn i_06e73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E740;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D49B6;
    20
}
/// $06E740: jsr $5dd5c.l
pub fn i_06e740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E746;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E746);
    cpu.pc = 0x5DD5C;
    16
}
/// $06E746: bcc.w $6e750
pub fn i_06e746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E74A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6E750; return 10; }
    8
}
// ===== SetTaskHandler_06e74a  ($06E74A, 8 B, kernel/task_handlers.c)
/// $06E74A: lea.l $6e80a(pc), a1
pub fn i_06e74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E74E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E80A;
    16
}
/// $06E74E: move.l a1, (a6)
pub fn i_06e74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E750;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E750: rts
pub fn i_06e750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E752;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Die_06e752  ($06E752, 184 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E752: move.w #$120, $24(a6)
pub fn i_06e752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E758;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E758: move.w #$ffff, $38(a6)
pub fn i_06e758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E75E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E75E: lea.l $78890.l, a1
pub fn i_06e75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E764;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78890;
    20
}
/// $06E764: jsr $4ae.l
pub fn i_06e764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E76A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E76A);
    cpu.pc = 0x4AE;
    16
}
/// $06E76A: jsr $5dd02.l
pub fn i_06e76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E770;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E770);
    cpu.pc = 0x5DD02;
    16
}
/// $06E770: tst.b $9c(a6)
pub fn i_06e770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E774;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E774: bne.w $6e786
pub fn i_06e774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E778;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E786; return 10; }
    8
}
/// $06E778: tst.b $9b(a6)
pub fn i_06e778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E77C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06E77C: bne.w $6e786
pub fn i_06e77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E780;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E786; return 10; }
    8
}
/// $06E780: jsr $434dc.l
pub fn i_06e780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E786;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E786);
    cpu.pc = 0x434DC;
    16
}
/// $06E786: move.w #$4000, d0
pub fn i_06e786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E78A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E78A: jsr $28134.l
pub fn i_06e78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E790);
    cpu.pc = 0x28134;
    16
}
/// $06E790: andi.w #$ffe3, $38(a6)
pub fn i_06e790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E796;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E796: ori.w #$0, $38(a6)
pub fn i_06e796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E79C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E79C: move.w #$1023, d0
pub fn i_06e79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E7A0: jsr $2352.l
pub fn i_06e7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7A6);
    cpu.pc = 0x2352;
    16
}
/// $06E7A6: lea.l $77fd6.l, a1
pub fn i_06e7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7AC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06E7AC: jsr $4ae.l
pub fn i_06e7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7B2);
    cpu.pc = 0x4AE;
    16
}
/// $06E7B2: jsr $5dd02.l
pub fn i_06e7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7B8);
    cpu.pc = 0x5DD02;
    16
}
/// $06E7B8: lea.l $2d49d0.l, a1
pub fn i_06e7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7BE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D49D0;
    20
}
/// $06E7BE: jsr $77c7e.l
pub fn i_06e7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7C4);
    cpu.pc = 0x77C7E;
    16
}
/// $06E7C4: lea.l $6edfc(pc), a1
pub fn i_06e7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EDFC;
    16
}
/// $06E7C8: jsr $4ae.l
pub fn i_06e7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7CE);
    cpu.pc = 0x4AE;
    16
}
/// $06E7CE: jsr $5dd02.l
pub fn i_06e7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7D4);
    cpu.pc = 0x5DD02;
    16
}
/// $06E7D4: lea.l $6edfc(pc), a1
pub fn i_06e7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EDFC;
    16
}
/// $06E7D8: jsr $4ae.l
pub fn i_06e7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7DE);
    cpu.pc = 0x4AE;
    16
}
/// $06E7DE: jsr $5dd02.l
pub fn i_06e7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7E4);
    cpu.pc = 0x5DD02;
    16
}
/// $06E7E4: eori.b #$1, $3a(a0)
pub fn i_06e7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7EA;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E7EA: lea.l $6ee0c(pc), a1
pub fn i_06e7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE0C;
    16
}
/// $06E7EE: jsr $4ae.l
pub fn i_06e7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7F4);
    cpu.pc = 0x4AE;
    16
}
/// $06E7F4: jsr $5dd02.l
pub fn i_06e7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E7FA);
    cpu.pc = 0x5DD02;
    16
}
/// $06E7FA: lea.l $6ee1c(pc), a1
pub fn i_06e7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E7FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE1C;
    16
}
/// $06E7FE: jsr $4ae.l
pub fn i_06e7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E804);
    cpu.pc = 0x4AE;
    16
}
/// $06E804: jsr $5dd02.l
pub fn i_06e804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E80A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E80A);
    cpu.pc = 0x5DD02;
    16
}
// ===== Gunship_Free_06e80a  ($06E80A, 12 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E80A: move.b #$1, $20(a6)
pub fn i_06e80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E810;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E810: jmp $518.l
pub fn i_06e810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E816;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Gunship_Nop_06e816  ($06E816, 2 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E816: rts
pub fn i_06e816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E818;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_06e818  ($06E818, 8 B, kernel/jmp_to_scheduler.c)
/// $06E818: jmp $518.l
pub fn i_06e818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E81E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Gunship_FlashCount5_06e820  ($06E820, 100 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E820: move.w #$3b, d1
pub fn i_06e820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E824;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E824: jsr $236e.l
pub fn i_06e824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E82A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E82A);
    cpu.pc = 0x236E;
    16
}
/// $06E82A: clr.b $20(a6)
pub fn i_06e82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E82E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06E82E: move.w #$10b8, d0
pub fn i_06e82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E832;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E832: jsr $2352.l
pub fn i_06e832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E838);
    cpu.pc = 0x2352;
    16
}
/// $06E838: move.w #$d000, d0
pub fn i_06e838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E83C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E83C: jsr $28134.l
pub fn i_06e83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E842;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E842);
    cpu.pc = 0x28134;
    16
}
/// $06E842: andi.w #$ffe3, $38(a6)
pub fn i_06e842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E848;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E848: ori.w #$18, $38(a6)
pub fn i_06e848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E84E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E84E: bset.b #$6, $12(a6)
pub fn i_06e84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E854;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E854: move.w #$d000, $34(a6)
pub fn i_06e854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E85A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E85A: lea.l $2d4c42.l, a0
pub fn i_06e85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E860;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4C42;
    20
}
/// $06E860: jsr $28cd4.l
pub fn i_06e860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E866;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E866);
    cpu.pc = 0x28CD4;
    16
}
/// $06E866: move.w #$5, $70(a6)
pub fn i_06e866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E86C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E86C: clr.b $7e(a6)
pub fn i_06e86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E870;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06E870: lea.l $6e876(pc), a1
pub fn i_06e870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E874;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E876;
    16
}
/// $06E874: move.l a1, (a6)
pub fn i_06e874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E876;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E876: subq.w #$1, $70(a6)
pub fn i_06e876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E87A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06E87A: cmpi.w #$0, $70(a6)
pub fn i_06e87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E880;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06E880: bgt.w $6e88a
pub fn i_06e880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E884;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E88A; return 10; }
    8
}
// ===== SetTaskHandler_06e884  ($06E884, 8 B, kernel/task_handlers.c)
/// $06E884: lea.l $6e88c(pc), a1
pub fn i_06e884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E888;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E88C;
    16
}
/// $06E888: move.l a1, (a6)
pub fn i_06e888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E88A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E88A: rts
pub fn i_06e88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E88C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_SpawnCrewArc_06e88c  ($06E88C, 158 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E88C: lea.l $6e966(pc), a1
pub fn i_06e88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E890;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E966;
    16
}
/// $06E890: jsr $4ae.l
pub fn i_06e890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E896;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E896);
    cpu.pc = 0x4AE;
    16
}
/// $06E896: jsr $5dd02.l
pub fn i_06e896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E89C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E89C);
    cpu.pc = 0x5DD02;
    16
}
/// $06E89C: bset.b #$6, $12(a0)
pub fn i_06e89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8A2;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E8A2: lea.l $6e8a8(pc), a1
pub fn i_06e8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E8A8;
    16
}
/// $06E8A6: move.l a1, (a6)
pub fn i_06e8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E8A8: movea.l $c(a6), a0
pub fn i_06e8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8AC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06E8AC: lea.l $2c07ac.l, a1
pub fn i_06e8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8B2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $06E8B2: lea.l $2c072c.l, a2
pub fn i_06e8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8B8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C072C;
    20
}
/// $06E8B8: move.b $34(a6), d0
pub fn i_06e8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8BC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E8BC: andi.w #$ff, d0
pub fn i_06e8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8C0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E8C0: lsl.w #$1, d0
pub fn i_06e8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8C2;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06E8C2: move.w (a1, d0.w), d1
pub fn i_06e8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8C6;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06E8C6: move.w (a2, d0.w), d2
pub fn i_06e8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8CA;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06E8CA: move.w $22(a0), d0
pub fn i_06e8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8CE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06E8CE: add.w d0, d1
pub fn i_06e8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8D0;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06E8D0: move.w d1, $22(a6)
pub fn i_06e8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8D4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E8D4: addi.w #$100, d2
pub fn i_06e8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8D8;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E8D8: add.w $24(a0), d2
pub fn i_06e8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8DC;
    cpu.ir = 0xD468;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06E8DC: move.w d2, $24(a6)
pub fn i_06e8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8E0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E8E0: cmpi.w #$140, $22(a6)
pub fn i_06e8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8E6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $06E8E6: bgt.w $6e8f0
pub fn i_06e8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8EA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E8F0; return 10; }
    8
}
/// $06E8EA: jsr $28d70.l
pub fn i_06e8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E8F0);
    cpu.pc = 0x28D70;
    16
}
/// $06E8F0: subi.w #$80, $34(a6)
pub fn i_06e8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8F6;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06E8F6: cmpi.b #$c0, $34(a6)
pub fn i_06e8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E8FC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Byte, false);
    16
}
/// $06E8FC: bne.w $6e90a
pub fn i_06e8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E900;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E90A; return 10; }
    8
}
/// $06E900: movea.l $c(a6), a0
pub fn i_06e900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E904;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06E904: move.b #$1, $21(a0)
pub fn i_06e904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E90A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E90A: move.b $34(a6), d0
pub fn i_06e90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E90E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E90E: andi.w #$ff, d0
pub fn i_06e90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E912;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06E912: cmpi.w #$80, d0
pub fn i_06e912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E916;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $06E916: bgt.w $6e920
pub fn i_06e916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E91A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E920; return 10; }
    8
}
/// $06E91A: lea.l $6e932(pc), a1
pub fn i_06e91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E91E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E932;
    16
}
/// $06E91E: move.l a1, (a6)
pub fn i_06e91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E920;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E920: cmpi.w #$ffe0, $22(a6)
pub fn i_06e920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E926;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFE0, Size::Word, false);
    16
}
/// $06E926: bgt.w $6e930
pub fn i_06e926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E92A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E930; return 10; }
    8
}
// ===== SetTaskHandler_06e92a  ($06E92A, 8 B, kernel/task_handlers.c)
/// $06E92A: lea.l $6e932(pc), a1
pub fn i_06e92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E92E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E932;
    16
}
/// $06E92E: move.l a1, (a6)
pub fn i_06e92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E930;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E930: rts
pub fn i_06e930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E932;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_FallToGround_06e932  ($06E932, 44 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E932: movea.l $c(a6), a0
pub fn i_06e932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E936;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06E936: move.b #$2, $21(a0)
pub fn i_06e936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E93C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E93C: move.w #$fe00, $28(a6)
pub fn i_06e93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E942;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E942: lea.l $6e948(pc), a1
pub fn i_06e942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E946;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E948;
    16
}
/// $06E946: move.l a1, (a6)
pub fn i_06e946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E948;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E948: jsr $27cee.l
pub fn i_06e948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E94E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E94E);
    cpu.pc = 0x27CEE;
    16
}
/// $06E94E: jsr $28d70.l
pub fn i_06e94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E954;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E954);
    cpu.pc = 0x28D70;
    16
}
/// $06E954: cmpi.w #$ffe0, $22(a6)
pub fn i_06e954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E95A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFE0, Size::Word, false);
    16
}
/// $06E95A: bgt.w $6e964
pub fn i_06e95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E95E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6E964; return 10; }
    8
}
// ===== SetTaskHandler_06e95e  ($06E95E, 8 B, kernel/task_handlers.c)
/// $06E95E: lea.l $6e818(pc), a1
pub fn i_06e95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E962;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06E962: move.l a1, (a6)
pub fn i_06e962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E964;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E964: rts
pub fn i_06e964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E966;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Crew_06e966  ($06E966, 82 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E966: move.w #$3b, d1
pub fn i_06e966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E96A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E96A: jsr $236e.l
pub fn i_06e96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E970);
    cpu.pc = 0x236E;
    16
}
/// $06E970: move.w #$9, $1c(a6)
pub fn i_06e970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E976;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E976: jsr $138fe.l
pub fn i_06e976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E97C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E97C);
    cpu.pc = 0x138FE;
    16
}
/// $06E97C: lea.l $2d4d6e.l, a0
pub fn i_06e97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E982;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4D6E;
    20
}
/// $06E982: jsr $28cd4.l
pub fn i_06e982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E988;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E988);
    cpu.pc = 0x28CD4;
    16
}
/// $06E988: lea.l $6e98e(pc), a1
pub fn i_06e988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E98C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E98E;
    16
}
/// $06E98C: move.l a1, (a6)
pub fn i_06e98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E98E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06E98E: jsr $5e506.l
pub fn i_06e98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E994;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E994);
    cpu.pc = 0x5E506;
    16
}
/// $06E994: btst.b #$0, $5a(a0)
pub fn i_06e994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E99A;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06E99A: beq.w $6e9a4
pub fn i_06e99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E99E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6E9A4; return 10; }
    8
}
/// $06E99E: bset.b #$0, $5a(a6)
pub fn i_06e99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9A4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E9A4: movea.l $c(a6), a0
pub fn i_06e9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9A8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06E9A8: cmpi.b #$1, $20(a0)
pub fn i_06e9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9AE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06E9AE: bne.w $6e9b8
pub fn i_06e9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E9B8; return 10; }
    8
}
/// $06E9B2: lea.l $6e818(pc), a1
pub fn i_06e9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06E9B6: move.l a1, (a6)
pub fn i_06e9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_06e9b8  ($06E9B8, 8 B, thunks/jsr_abs_thunks.c)
/// $06E9B8: jsr $28d70.l
pub fn i_06e9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E9BE);
    cpu.pc = 0x28D70;
    16
}
/// $06E9BE: rts
pub fn i_06e9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Gunner_06e9c0  ($06E9C0, 206 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06E9C0: bset.b #$4, $6b(a6)
pub fn i_06e9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9C6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06E9C6: move.b $99(a6), d0
pub fn i_06e9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9CA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06E9CA: cmpi.b #$2, d0
pub fn i_06e9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9CE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $06E9CE: bne.w $6e9d8
pub fn i_06e9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6E9D8; return 10; }
    8
}
/// $06E9D2: jsr $5e9b6.l
pub fn i_06e9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E9D8);
    cpu.pc = 0x5E9B6;
    16
}
/// $06E9D8: andi.b #$1, d0
pub fn i_06e9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9DC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06E9DC: move.b d0, $99(a6)
pub fn i_06e9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9E0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06E9E0: move.w #$3c, d1
pub fn i_06e9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9E4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06E9E4: jsr $236e.l
pub fn i_06e9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E9EA);
    cpu.pc = 0x236E;
    16
}
/// $06E9EA: move.w #$6, $1c(a6)
pub fn i_06e9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06E9F0: jsr $138fe.l
pub fn i_06e9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6E9F6);
    cpu.pc = 0x138FE;
    16
}
/// $06E9F6: lea.l $2b950a.l, a0
pub fn i_06e9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6E9FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B950A;
    20
}
/// $06E9FC: jsr $799de.l
pub fn i_06e9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA02);
    cpu.pc = 0x799DE;
    16
}
/// $06EA02: move.w d0, $36(a6)
pub fn i_06ea02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA06;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06EA06: move.w #$d000, d0
pub fn i_06ea06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA0A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EA0A: jsr $28134.l
pub fn i_06ea0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA10);
    cpu.pc = 0x28134;
    16
}
/// $06EA10: andi.w #$ffe3, $38(a6)
pub fn i_06ea10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA16;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EA16: ori.w #$1c, $38(a6)
pub fn i_06ea16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA1C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EA1C: move.l #$2d471a, $48(a6)
pub fn i_06ea1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA24;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D471A;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06EA24: lea.l $2b9488.l, a0
pub fn i_06ea24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA2A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9488;
    20
}
/// $06EA2A: jsr $799de.l
pub fn i_06ea2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA30);
    cpu.pc = 0x799DE;
    16
}
/// $06EA30: move.w d0, $66(a6)
pub fn i_06ea30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA34;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06EA34: lea.l $2d4e64.l, a0
pub fn i_06ea34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4E64;
    20
}
/// $06EA3A: jsr $28cd4.l
pub fn i_06ea3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA40);
    cpu.pc = 0x28CD4;
    16
}
/// $06EA40: lea.l $6ea46(pc), a1
pub fn i_06ea40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EA46;
    16
}
/// $06EA44: move.l a1, (a6)
pub fn i_06ea44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EA46: jsr $5e506.l
pub fn i_06ea46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA4C);
    cpu.pc = 0x5E506;
    16
}
/// $06EA4C: jsr $6ef2c(pc)
pub fn i_06ea4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EA50);
    cpu.pc = 0x6EF2C;
    16
}
/// $06EA50: jsr $28d70.l
pub fn i_06ea50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA56);
    cpu.pc = 0x28D70;
    16
}
/// $06EA56: jsr $6ef66(pc)
pub fn i_06ea56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EA5A);
    cpu.pc = 0x6EF66;
    16
}
/// $06EA5A: bcc.w $6ea64
pub fn i_06ea5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EA64; return 10; }
    8
}
/// $06EA5E: lea.l $6ea96(pc), a1
pub fn i_06ea5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EA96;
    16
}
/// $06EA62: move.l a1, (a6)
pub fn i_06ea62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EA64: movea.l $c(a6), a0
pub fn i_06ea64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA68;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06EA68: cmpi.b #$1, $20(a0)
pub fn i_06ea68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA6E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06EA6E: bne.w $6ea78
pub fn i_06ea6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6EA78; return 10; }
    8
}
/// $06EA72: lea.l $6e818(pc), a1
pub fn i_06ea72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06EA76: move.l a1, (a6)
pub fn i_06ea76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EA78: movea.l #$ffffffff, a0
pub fn i_06ea78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA7E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06EA7E: lea.l $2d49b6.l, a0
pub fn i_06ea7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA84;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D49B6;
    20
}
/// $06EA84: jsr $5dd5c.l
pub fn i_06ea84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EA8A);
    cpu.pc = 0x5DD5C;
    16
}
/// $06EA8A: bcc.w $6ea94
pub fn i_06ea8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EA94; return 10; }
    8
}
// ===== SetTaskHandler_06ea8e  ($06EA8E, 8 B, kernel/task_handlers.c)
/// $06EA8E: lea.l $6e818(pc), a1
pub fn i_06ea8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06EA92: move.l a1, (a6)
pub fn i_06ea92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EA94: rts
pub fn i_06ea94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_GunnerAim_06ea96  ($06EA96, 60 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EA96: lea.l $2d4e70.l, a0
pub fn i_06ea96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EA9C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4E70;
    20
}
/// $06EA9C: jsr $28cd4.l
pub fn i_06ea9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EAA2);
    cpu.pc = 0x28CD4;
    16
}
/// $06EAA2: lea.l $6eaa8(pc), a1
pub fn i_06eaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EAA8;
    16
}
/// $06EAA6: move.l a1, (a6)
pub fn i_06eaa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EAA8: jsr $5e506.l
pub fn i_06eaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EAAE);
    cpu.pc = 0x5E506;
    16
}
/// $06EAAE: jsr $6ef2c(pc)
pub fn i_06eaae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EAB2);
    cpu.pc = 0x6EF2C;
    16
}
/// $06EAB2: jsr $28d70.l
pub fn i_06eab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EAB8);
    cpu.pc = 0x28D70;
    16
}
/// $06EAB8: bcc.w $6ead0
pub fn i_06eab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EABC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EAD0; return 10; }
    8
}
/// $06EABC: lea.l $6ead2(pc), a1
pub fn i_06eabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EAD2;
    16
}
/// $06EAC0: move.l a1, (a6)
pub fn i_06eac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EAC2: tst.b $9e(a6)
pub fn i_06eac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAC6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06EAC6: beq.w $6ead0
pub fn i_06eac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EACA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EAD0; return 10; }
    8
}
/// $06EACA: lea.l $6ec54(pc), a1
pub fn i_06eaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EACE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EC54;
    16
}
/// $06EACE: move.l a1, (a6)
pub fn i_06eace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EAD0: bra.b $6ea64
pub fn i_06ead0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAD2;
    cpu.ir = 0x6092;
    cpu.pc = 0x6EA64;
    10
}
// ===== Gunship_GunnerFire_06ead2  ($06EAD2, 170 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EAD2: move.w #$d000, d0
pub fn i_06ead2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAD6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EAD6: jsr $28134.l
pub fn i_06ead6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EADC);
    cpu.pc = 0x28134;
    16
}
/// $06EADC: andi.w #$ffe3, $38(a6)
pub fn i_06eadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAE2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EAE2: ori.w #$1c, $38(a6)
pub fn i_06eae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAE8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EAE8: move.w #$ff80, $2e(a6)
pub fn i_06eae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06EAEE: lea.l $2d4e9a.l, a0
pub fn i_06eaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4E9A;
    20
}
/// $06EAF4: jsr $28cd4.l
pub fn i_06eaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EAFA);
    cpu.pc = 0x28CD4;
    16
}
/// $06EAFA: lea.l $6eb00(pc), a1
pub fn i_06eafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EAFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EB00;
    16
}
/// $06EAFE: move.l a1, (a6)
pub fn i_06eafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EB00: jsr $27d50.l
pub fn i_06eb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB06);
    cpu.pc = 0x27D50;
    16
}
/// $06EB06: jsr $6eff0(pc)
pub fn i_06eb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB0A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EB0A);
    cpu.pc = 0x6EFF0;
    16
}
/// $06EB0A: bcc.w $6eb14
pub fn i_06eb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB0E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EB14; return 10; }
    8
}
/// $06EB0E: lea.l $6ebd0(pc), a1
pub fn i_06eb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EBD0;
    16
}
/// $06EB12: move.l a1, (a6)
pub fn i_06eb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EB14: jsr $28d70.l
pub fn i_06eb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB1A);
    cpu.pc = 0x28D70;
    16
}
/// $06EB1A: jsr $283d8.l
pub fn i_06eb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB20);
    cpu.pc = 0x283D8;
    16
}
/// $06EB20: btst.b #$1, $13(a6)
pub fn i_06eb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB26;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06EB26: beq.w $6eb30
pub fn i_06eb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EB30; return 10; }
    8
}
/// $06EB2A: lea.l $6ee8a(pc), a1
pub fn i_06eb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE8A;
    16
}
/// $06EB2E: move.l a1, (a6)
pub fn i_06eb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EB30: jsr $2870a.l
pub fn i_06eb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB36);
    cpu.pc = 0x2870A;
    16
}
/// $06EB36: bcc.w $6eb4c
pub fn i_06eb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EB4C; return 10; }
    8
}
/// $06EB3A: bclr.b #$3, $13(a6)
pub fn i_06eb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB40;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06EB40: lea.l $5e766.l, a0
pub fn i_06eb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06EB46: jsr $5e770.l
pub fn i_06eb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB4C);
    cpu.pc = 0x5E770;
    16
}
/// $06EB4C: jsr $28758.l
pub fn i_06eb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB52);
    cpu.pc = 0x28758;
    16
}
/// $06EB52: bcc.w $6eb66
pub fn i_06eb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EB66; return 10; }
    8
}
/// $06EB56: move.w #$1022, d0
pub fn i_06eb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB5A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EB5A: jsr $2352.l
pub fn i_06eb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB60);
    cpu.pc = 0x2352;
    16
}
/// $06EB60: lea.l $6ee8a(pc), a1
pub fn i_06eb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE8A;
    16
}
/// $06EB64: move.l a1, (a6)
pub fn i_06eb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EB66: movea.l #$ffffffff, a0
pub fn i_06eb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB6C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06EB6C: lea.l $2d49b6.l, a0
pub fn i_06eb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB72;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D49B6;
    20
}
/// $06EB72: jsr $5dd56.l
pub fn i_06eb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EB78);
    cpu.pc = 0x5DD56;
    16
}
/// $06EB78: bcc.w $6eb82
pub fn i_06eb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB7C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EB82; return 10; }
    8
}
// ===== SetTaskHandler_06eb7c  ($06EB7C, 8 B, kernel/task_handlers.c)
/// $06EB7C: lea.l $6e818(pc), a1
pub fn i_06eb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06EB80: move.l a1, (a6)
pub fn i_06eb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EB82: rts
pub fn i_06eb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Hitbox_06eb84  ($06EB84, 168 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EB84: or.b d2, d0
pub fn i_06eb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB86;
    cpu.ir = 0x8002;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $06EB86: ori.b #$0, d0
pub fn i_06eb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB8A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06EB8A: dc.w $fff0
pub fn i_06eb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EB8C;
    cpu.ir = 0xFFF0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Gunship_GunnerIdle_06ec2c  ($06EC2C, 40 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EC2C: clr.w $2a(a6)
pub fn i_06ec2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC30;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EC30: clr.w $2e(a6)
pub fn i_06ec30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC34;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EC34: lea.l $6ec3a(pc), a1
pub fn i_06ec34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EC3A;
    16
}
/// $06EC38: move.l a1, (a6)
pub fn i_06ec38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EC3A: jsr $27cee.l
pub fn i_06ec3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC40);
    cpu.pc = 0x27CEE;
    16
}
/// $06EC40: jsr $28998.l
pub fn i_06ec40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC46);
    cpu.pc = 0x28998;
    16
}
/// $06EC46: jsr $6f050(pc)
pub fn i_06ec46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EC4A);
    cpu.pc = 0x6F050;
    16
}
/// $06EC4A: jsr $28d70.l
pub fn i_06ec4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC50);
    cpu.pc = 0x28D70;
    16
}
/// $06EC50: bra.w $6eb1a
pub fn i_06ec50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC54;
    cpu.ir = 0x6000;
    cpu.pc = 0x6EB1A;
    10
}
// ===== Gunship_GunnerFireB_06ec54  ($06EC54, 230 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EC54: move.w #$d000, d0
pub fn i_06ec54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC58;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EC58: jsr $28134.l
pub fn i_06ec58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC5E);
    cpu.pc = 0x28134;
    16
}
/// $06EC5E: andi.w #$ffe3, $38(a6)
pub fn i_06ec5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC64;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EC64: ori.w #$1c, $38(a6)
pub fn i_06ec64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC6A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EC6A: move.w #$ff80, $2e(a6)
pub fn i_06ec6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06EC70: move.l #$2d4962, $4c(a6)
pub fn i_06ec70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC78;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D4962;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06EC78: move.l #$2d47c2, $48(a6)
pub fn i_06ec78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC80;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D47C2;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06EC80: move.w #$28, d0
pub fn i_06ec80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC84;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EC84: move.w d0, $8c(a6)
pub fn i_06ec84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC88;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06EC88: sub.w d0, $24(a6)
pub fn i_06ec88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC8C;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EC8C: lea.l $2d4ea0.l, a0
pub fn i_06ec8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC92;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4EA0;
    20
}
/// $06EC92: jsr $28cd4.l
pub fn i_06ec92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EC98);
    cpu.pc = 0x28CD4;
    16
}
/// $06EC98: lea.l $6ec9e(pc), a1
pub fn i_06ec98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EC9E;
    16
}
/// $06EC9C: move.l a1, (a6)
pub fn i_06ec9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EC9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EC9E: move.w $2a(a6), $8a(a6)
pub fn i_06ec9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECA4;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06ECA4: move.w $24(a6), $86(a6)
pub fn i_06eca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECAA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06ECAA: move.b $27(a6), $89(a6)
pub fn i_06ecaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECB0;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06ECB0: jsr $27bc8.l
pub fn i_06ecb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ECB6);
    cpu.pc = 0x27BC8;
    16
}
/// $06ECB6: bcc.w $6ecde
pub fn i_06ecb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ECDE; return 10; }
    8
}
/// $06ECBA: move.w #$ff80, $2e(a6)
pub fn i_06ecba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06ECC0: move.w $8a(a6), $2a(a6)
pub fn i_06ecc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECC6;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06ECC6: move.w $86(a6), $24(a6)
pub fn i_06ecc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECCC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06ECCC: move.b $89(a6), $27(a6)
pub fn i_06eccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECD2;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x27);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06ECD2: jsr $27cee.l
pub fn i_06ecd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ECD8);
    cpu.pc = 0x27CEE;
    16
}
/// $06ECD8: lea.l $6ed42(pc), a1
pub fn i_06ecd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ED42;
    16
}
/// $06ECDC: move.l a1, (a6)
pub fn i_06ecdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECDE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ECDE: move.w $8c(a6), d0
pub fn i_06ecde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECE2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06ECE2: add.w d0, $24(a6)
pub fn i_06ece2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECE6;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06ECE6: jsr $28d70.l
pub fn i_06ece6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ECEC);
    cpu.pc = 0x28D70;
    16
}
/// $06ECEC: move.w $8c(a6), d0
pub fn i_06ecec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECF0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06ECF0: sub.w d0, $24(a6)
pub fn i_06ecf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECF4;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06ECF4: jsr $283d8.l
pub fn i_06ecf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ECFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ECFA);
    cpu.pc = 0x283D8;
    16
}
/// $06ECFA: btst.b #$1, $13(a6)
pub fn i_06ecfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED00;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06ED00: beq.w $6ed0a
pub fn i_06ed00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6ED0A; return 10; }
    8
}
/// $06ED04: lea.l $6ee82(pc), a1
pub fn i_06ed04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE82;
    16
}
/// $06ED08: move.l a1, (a6)
pub fn i_06ed08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ED0A: jsr $2870a.l
pub fn i_06ed0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ED10);
    cpu.pc = 0x2870A;
    16
}
/// $06ED10: bcc.w $6ed24
pub fn i_06ed10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ED24; return 10; }
    8
}
/// $06ED14: move.w #$1022, d0
pub fn i_06ed14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED18;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06ED18: jsr $2352.l
pub fn i_06ed18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ED1E);
    cpu.pc = 0x2352;
    16
}
/// $06ED1E: lea.l $6ee82(pc), a1
pub fn i_06ed1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE82;
    16
}
/// $06ED22: move.l a1, (a6)
pub fn i_06ed22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ED24: movea.l #$ffffffff, a0
pub fn i_06ed24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED2A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06ED2A: lea.l $2d49b6.l, a0
pub fn i_06ed2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D49B6;
    20
}
/// $06ED30: jsr $5dd56.l
pub fn i_06ed30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ED36);
    cpu.pc = 0x5DD56;
    16
}
/// $06ED36: bcc.w $6ed40
pub fn i_06ed36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ED40; return 10; }
    8
}
// ===== SetTaskHandler_06ed3a  ($06ED3A, 8 B, kernel/task_handlers.c)
/// $06ED3A: lea.l $6e818(pc), a1
pub fn i_06ed3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06ED3E: move.l a1, (a6)
pub fn i_06ed3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ED40: rts
pub fn i_06ed40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_GunnerReload_06ed42  ($06ED42, 122 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06ED42: move.w $8c(a6), d0
pub fn i_06ed42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED46;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06ED46: add.w d0, $24(a6)
pub fn i_06ed46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED4A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06ED4A: move.l #$2d490e, $4c(a6)
pub fn i_06ed4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED52;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D490E;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06ED52: move.l #$2d476e, $48(a6)
pub fn i_06ed52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED5A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D476E;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06ED5A: move.w #$8, d0
pub fn i_06ed5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED5E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06ED5E: move.w d0, $8c(a6)
pub fn i_06ed5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED62;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06ED62: sub.w d0, $24(a6)
pub fn i_06ed62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED66;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06ED66: move.w #$ff80, d1
pub fn i_06ed66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED6A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06ED6A: move.w d1, $2e(a6)
pub fn i_06ed6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED6E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06ED6E: lea.l $2d4ee4.l, a0
pub fn i_06ed6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4EE4;
    20
}
/// $06ED74: jsr $28cd4.l
pub fn i_06ed74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ED7A);
    cpu.pc = 0x28CD4;
    16
}
/// $06ED7A: lea.l $6ed80(pc), a1
pub fn i_06ed7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6ED80;
    16
}
/// $06ED7E: move.l a1, (a6)
pub fn i_06ed7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ED80: tst.b $9d(a6)
pub fn i_06ed80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED84;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06ED84: beq.w $6ed8e
pub fn i_06ed84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED88;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6ED8E; return 10; }
    8
}
/// $06ED88: bset.b #$6, $13(a6)
pub fn i_06ed88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED8E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06ED8E: jsr $27bc8.l
pub fn i_06ed8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6ED94);
    cpu.pc = 0x27BC8;
    16
}
/// $06ED94: bcc.w $6ed9e
pub fn i_06ed94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED98;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6ED9E; return 10; }
    8
}
/// $06ED98: lea.l $6edbc(pc), a1
pub fn i_06ed98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EDBC;
    16
}
/// $06ED9C: move.l a1, (a6)
pub fn i_06ed9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6ED9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06ED9E: jsr $6f050(pc)
pub fn i_06ed9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EDA2);
    cpu.pc = 0x6F050;
    16
}
/// $06EDA2: move.w $8c(a6), d0
pub fn i_06eda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDA6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EDA6: add.w d0, $24(a6)
pub fn i_06eda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDAA;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EDAA: jsr $28d70.l
pub fn i_06edaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EDB0);
    cpu.pc = 0x28D70;
    16
}
/// $06EDB0: move.w $8c(a6), d0
pub fn i_06edb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDB4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EDB4: sub.w d0, $24(a6)
pub fn i_06edb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDB8;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EDB8: bra.w $6ecf4
pub fn i_06edb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x6ECF4;
    10
}
// ===== Gunship_GunnerWait_06edbc  ($06EDBC, 64 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EDBC: clr.w $2e(a6)
pub fn i_06edbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDC0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EDC0: clr.w $2a(a6)
pub fn i_06edc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDC4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EDC4: lea.l $6edca(pc), a1
pub fn i_06edc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EDCA;
    16
}
/// $06EDC8: move.l a1, (a6)
pub fn i_06edc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EDCA: tst.b $9d(a6)
pub fn i_06edca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDCE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06EDCE: beq.w $6edd8
pub fn i_06edce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDD2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EDD8; return 10; }
    8
}
/// $06EDD2: bset.b #$6, $13(a6)
pub fn i_06edd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDD8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06EDD8: jsr $27a92.l
pub fn i_06edd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EDDE);
    cpu.pc = 0x27A92;
    16
}
/// $06EDDE: jsr $6f050(pc)
pub fn i_06edde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6EDE2);
    cpu.pc = 0x6F050;
    16
}
/// $06EDE2: move.w $8c(a6), d0
pub fn i_06ede2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDE6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EDE6: add.w d0, $24(a6)
pub fn i_06ede6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDEA;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EDEA: jsr $28d70.l
pub fn i_06edea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EDF0);
    cpu.pc = 0x28D70;
    16
}
/// $06EDF0: move.w $8c(a6), d0
pub fn i_06edf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDF4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EDF4: sub.w d0, $24(a6)
pub fn i_06edf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDF8;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EDF8: bra.w $6ecf4
pub fn i_06edf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EDFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x6ECF4;
    10
}
// ===== Gunship_WreckPiece_06edfc  ($06EDFC, 82 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EDFC: lea.l $2d4fc4.l, a0
pub fn i_06edfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4FC4;
    20
}
/// $06EE02: jsr $28cd4.l
pub fn i_06ee02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE08);
    cpu.pc = 0x28CD4;
    16
}
/// $06EE08: bra.w $6ee28
pub fn i_06ee08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6EE28;
    10
}
/// $06EE28: move.w #$8000, d0
pub fn i_06ee28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE2C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EE2C: jsr $28134.l
pub fn i_06ee2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE32);
    cpu.pc = 0x28134;
    16
}
/// $06EE32: andi.w #$ffe3, $38(a6)
pub fn i_06ee32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE38;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EE38: ori.w #$0, $38(a6)
pub fn i_06ee38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE3E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EE3E: move.w #$3b, d1
pub fn i_06ee3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE42;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EE42: jsr $236e.l
pub fn i_06ee42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE48);
    cpu.pc = 0x236E;
    16
}
/// $06EE48: jmp $6dce0.l
pub fn i_06ee48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE4E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x6DCE0;
    8
}
// ===== Gunship_Shell_06ee4e  ($06EE4E, 44 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EE4E: move.w #$4, d1
pub fn i_06ee4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE52;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EE52: jsr $236e.l
pub fn i_06ee52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE58);
    cpu.pc = 0x236E;
    16
}
/// $06EE58: lea.l $2dd8b6.l, a0
pub fn i_06ee58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD8B6;
    20
}
/// $06EE5E: jsr $28cd4.l
pub fn i_06ee5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE64);
    cpu.pc = 0x28CD4;
    16
}
/// $06EE64: lea.l $6ee6a(pc), a1
pub fn i_06ee64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE6A;
    16
}
/// $06EE68: move.l a1, (a6)
pub fn i_06ee68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EE6A: jsr $2783a.l
pub fn i_06ee6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE70);
    cpu.pc = 0x2783A;
    16
}
/// $06EE70: jsr $28d70.l
pub fn i_06ee70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE76);
    cpu.pc = 0x28D70;
    16
}
/// $06EE76: bcc.w $6ee80
pub fn i_06ee76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6EE80; return 10; }
    8
}
// ===== SetTaskHandler_06ee7a  ($06EE7A, 8 B, kernel/task_handlers.c)
/// $06EE7A: lea.l $6e818(pc), a1
pub fn i_06ee7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6E818;
    16
}
/// $06EE7E: move.l a1, (a6)
pub fn i_06ee7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06EE80: rts
pub fn i_06ee80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_Explode_06ee82  ($06EE82, 44 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EE82: move.w $8c(a6), d0
pub fn i_06ee82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE86;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EE86: add.w d0, $24(a6)
pub fn i_06ee86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE8A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EE8A: move.w #$4000, d0
pub fn i_06ee8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE8E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EE8E: jsr $28134.l
pub fn i_06ee8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EE94);
    cpu.pc = 0x28134;
    16
}
/// $06EE94: andi.w #$ffe3, $38(a6)
pub fn i_06ee94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EE9A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EE9A: ori.w #$1c, $38(a6)
pub fn i_06ee9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEA0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06EEA0: move.l #$ffffffff, $48(a6)
pub fn i_06eea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEA8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06EEA8: jmp $77f6a.l
pub fn i_06eea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEAE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Gunship_SetPathByParam98_06eeae  ($06EEAE, 32 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EEAE: move.b $98(a6), d0
pub fn i_06eeae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEB2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06EEB2: andi.w #$7, d0
pub fn i_06eeb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEB6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06EEB6: lsl.w #$2, d0
pub fn i_06eeb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEB8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06EEB8: lea.l $2d4a2e.l, a0
pub fn i_06eeb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEBE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4A2E;
    20
}
/// $06EEBE: move.l (a0, d0.w), $90(a6)
pub fn i_06eebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEC4;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $06EEC4: clr.w $94(a6)
pub fn i_06eec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEC8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EEC8: clr.b $96(a6)
pub fn i_06eec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EECC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x96);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06EECC: rts
pub fn i_06eecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EECE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_HeightClass_06eece  ($06EECE, 56 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EECE: move.w #$2, d0
pub fn i_06eece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EED2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EED2: move.w $24(a6), d1
pub fn i_06eed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EED6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EED6: cmpi.w #$1a0, d1
pub fn i_06eed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEDA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1A0, Size::Word, false);
    8
}
/// $06EEDA: bgt.w $6eeec
pub fn i_06eeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEDE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6EEEC; return 10; }
    8
}
/// $06EEDE: move.w #$1, d0
pub fn i_06eede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEE2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EEE2: cmpi.w #$160, d1
pub fn i_06eee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEE6;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x160, Size::Word, false);
    8
}
/// $06EEE6: bgt.w $6eeec
pub fn i_06eee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEEA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6EEEC; return 10; }
    8
}
/// $06EEEA: clr.w d0
pub fn i_06eeea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEEC;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06EEEC: btst.b #$0, $3a(a6)
pub fn i_06eeec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEF2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06EEF2: beq.w $6eefa
pub fn i_06eef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EEFA; return 10; }
    8
}
/// $06EEF6: addi.w #$3, d0
pub fn i_06eef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEFA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06EEFA: add.w d0, d0
pub fn i_06eefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EEFC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06EEFC: lea.l $2d4a22.l, a0
pub fn i_06eefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4A22;
    20
}
/// $06EF02: move.w (a0, d0.w), d0
pub fn i_06ef02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF06;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
// ===== JsrAbsThunk_06ef06  ($06EF06, 8 B, thunks/jsr_abs_thunks.c)
/// $06EF06: jsr $2352.l
pub fn i_06ef06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EF0C);
    cpu.pc = 0x2352;
    16
}
/// $06EF0C: rts
pub fn i_06ef0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0006EF0E  ($06EF0E, 2 B, thunks/trivial_rts.c)
/// $06EF0E: rts
pub fn i_06ef0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_AddDepthOffset_06ef10  ($06EF10, 28 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EF10: move.b $106f28.l, d0
pub fn i_06ef10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF16;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $06EF16: andi.w #$3, d0
pub fn i_06ef16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF1A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06EF1A: add.w d0, d0
pub fn i_06ef1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF1C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06EF1C: lea.l $2d4a9e.l, a0
pub fn i_06ef1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4A9E;
    20
}
/// $06EF22: move.w (a0, d0.w), d1
pub fn i_06ef22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF26;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06EF26: add.w d1, $24(a6)
pub fn i_06ef26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF2A;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EF2A: rts
pub fn i_06ef2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_CrewOffsetByParent7D_06ef2c  ($06EF2C, 58 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EF2C: movea.l $c(a6), a0
pub fn i_06ef2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF30;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06EF30: move.b $7d(a0), d1
pub fn i_06ef30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF34;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06EF34: andi.w #$7, d1
pub fn i_06ef34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF38;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06EF38: lsl.w #$3, d1
pub fn i_06ef38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF3A;
    cpu.ir = 0xE749;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06EF3A: lea.l $2d4a5e.l, a1
pub fn i_06ef3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF40;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D4A5E;
    20
}
/// $06EF40: move.w $4(a1, d1.w), d0
pub fn i_06ef40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF44;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06EF44: add.w d0, $38(a6)
pub fn i_06ef44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF48;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EF48: move.w $2(a1, d1.w), d0
pub fn i_06ef48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF4C;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06EF4C: add.w d0, $24(a6)
pub fn i_06ef4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF50;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EF50: move.w (a1, d1.w), d0
pub fn i_06ef50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF54;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06EF54: btst.b #$0, $3a(a6)
pub fn i_06ef54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF5A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06EF5A: beq.w $6ef60
pub fn i_06ef5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF5E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EF60; return 10; }
    8
}
/// $06EF5E: neg.w d0
pub fn i_06ef5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF60;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06EF60: add.w d0, $22(a6)
pub fn i_06ef60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF64;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06EF64: rts
pub fn i_06ef64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_ParentFlag7F_06ef66  ($06EF66, 14 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EF66: movea.l $c(a6), a0
pub fn i_06ef66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF6A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06EF6A: cmpi.b #$0, $7f(a0)
pub fn i_06ef6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF70;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06EF70: beq.w $6ef7a
pub fn i_06ef70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF74;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EF7A; return 10; }
    8
}
// ===== ClearXN_06ef74  ($06EF74, 6 B, thunks/ccr_helpers.c)
/// $06EF74: andi.b #$ee, ccr
pub fn i_06ef74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF78;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06EF78: rts
pub fn i_06ef78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06ef7a  ($06EF7A, 6 B, thunks/ccr_helpers.c)
/// $06EF7A: ori.b #$11, ccr
pub fn i_06ef7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF7E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06EF7E: rts
pub fn i_06ef7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_InScreen_06ef80  ($06EF80, 68 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EF80: move.w $24(a6), d2
pub fn i_06ef80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF84;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EF84: addq.w #$8, d2
pub fn i_06ef84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF86;
    cpu.ir = 0x5042;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
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

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x06DE10, i_06de10),
    (0x06DE12, i_06de12),
    (0x06DE14, i_06de14),
    (0x06DE18, i_06de18),
    (0x06DE1A, i_06de1a),
    (0x06DE1C, i_06de1c),
    (0x06DE1E, i_06de1e),
    (0x06DE20, i_06de20),
    (0x06DE24, i_06de24),
    (0x06DE28, i_06de28),
    (0x06DE2E, i_06de2e),
    (0x06DE32, i_06de32),
    (0x06DE36, i_06de36),
    (0x06DE3C, i_06de3c),
    (0x06DE40, i_06de40),
    (0x06DE42, i_06de42),
    (0x06DE46, i_06de46),
    (0x06DE48, i_06de48),
    (0x06DE4A, i_06de4a),
    (0x06DE4E, i_06de4e),
    (0x06DE54, i_06de54),
    (0x06DE58, i_06de58),
    (0x06DE5A, i_06de5a),
    (0x06DE5E, i_06de5e),
    (0x06DE60, i_06de60),
    (0x06DE62, i_06de62),
    (0x06DE66, i_06de66),
    (0x06DE6A, i_06de6a),
    (0x06DE6E, i_06de6e),
    (0x06DE74, i_06de74),
    (0x06DE78, i_06de78),
    (0x06DE7A, i_06de7a),
    (0x06DE80, i_06de80),
    (0x06DE84, i_06de84),
    (0x06DE8A, i_06de8a),
    (0x06DE8E, i_06de8e),
    (0x06DE90, i_06de90),
    (0x06DE94, i_06de94),
    (0x06DE98, i_06de98),
    (0x06DE9A, i_06de9a),
    (0x06DE9E, i_06de9e),
    (0x06DEA2, i_06dea2),
    (0x06DEA6, i_06dea6),
    (0x06DEAC, i_06deac),
    (0x06DEB0, i_06deb0),
    (0x06DEB4, i_06deb4),
    (0x06DEB6, i_06deb6),
    (0x06DEBA, i_06deba),
    (0x06DEBE, i_06debe),
    (0x06DEC2, i_06dec2),
    (0x06DEC6, i_06dec6),
    (0x06DECC, i_06decc),
    (0x06DED0, i_06ded0),
    (0x06DED6, i_06ded6),
    (0x06DEDA, i_06deda),
    (0x06DEDE, i_06dede),
    (0x06DEE2, i_06dee2),
    (0x06DEE8, i_06dee8),
    (0x06DEEE, i_06deee),
    (0x06DEF4, i_06def4),
    (0x06DEFA, i_06defa),
    (0x06DEFE, i_06defe),
    (0x06DF02, i_06df02),
    (0x06DF04, i_06df04),
    (0x06DF06, i_06df06),
    (0x06DF0C, i_06df0c),
    (0x06DF10, i_06df10),
    (0x06DF14, i_06df14),
    (0x06DF18, i_06df18),
    (0x06DF1C, i_06df1c),
    (0x06DF22, i_06df22),
    (0x06DF26, i_06df26),
    (0x06DF2A, i_06df2a),
    (0x06DF2E, i_06df2e),
    (0x06DF32, i_06df32),
    (0x06DF38, i_06df38),
    (0x06DF3C, i_06df3c),
    (0x06DF40, i_06df40),
    (0x06DF44, i_06df44),
    (0x06DF48, i_06df48),
    (0x06DF4E, i_06df4e),
    (0x06DF52, i_06df52),
    (0x06DF56, i_06df56),
    (0x06DF5A, i_06df5a),
    (0x06DF60, i_06df60),
    (0x06DF64, i_06df64),
    (0x06DF6A, i_06df6a),
    (0x06DF70, i_06df70),
    (0x06DF76, i_06df76),
    (0x06DF7E, i_06df7e),
    (0x06DF86, i_06df86),
    (0x06DF8A, i_06df8a),
    (0x06DF90, i_06df90),
    (0x06DF94, i_06df94),
    (0x06DF9A, i_06df9a),
    (0x06DF9E, i_06df9e),
    (0x06DFA4, i_06dfa4),
    (0x06DFAA, i_06dfaa),
    (0x06DFB0, i_06dfb0),
    (0x06DFB4, i_06dfb4),
    (0x06DFB6, i_06dfb6),
    (0x06DFBC, i_06dfbc),
    (0x06DFC0, i_06dfc0),
    (0x06DFC6, i_06dfc6),
    (0x06DFC8, i_06dfc8),
    (0x06DFCE, i_06dfce),
    (0x06DFD2, i_06dfd2),
    (0x06DFD8, i_06dfd8),
    (0x06DFDC, i_06dfdc),
    (0x06DFE2, i_06dfe2),
    (0x06DFE4, i_06dfe4),
    (0x06DFE8, i_06dfe8),
    (0x06DFEE, i_06dfee),
    (0x06DFF2, i_06dff2),
    (0x06DFF8, i_06dff8),
    (0x06DFFE, i_06dffe),
    (0x06E006, i_06e006),
    (0x06E00A, i_06e00a),
    (0x06E010, i_06e010),
    (0x06E016, i_06e016),
    (0x06E01A, i_06e01a),
    (0x06E020, i_06e020),
    (0x06E026, i_06e026),
    (0x06E028, i_06e028),
    (0x06E02E, i_06e02e),
    (0x06E032, i_06e032),
    (0x06E036, i_06e036),
    (0x06E03A, i_06e03a),
    (0x06E03E, i_06e03e),
    (0x06E044, i_06e044),
    (0x06E048, i_06e048),
    (0x06E04E, i_06e04e),
    (0x06E054, i_06e054),
    (0x06E05A, i_06e05a),
    (0x06E05E, i_06e05e),
    (0x06E060, i_06e060),
    (0x06E062, i_06e062),
    (0x06E068, i_06e068),
    (0x06E06E, i_06e06e),
    (0x06E072, i_06e072),
    (0x06E078, i_06e078),
    (0x06E07C, i_06e07c),
    (0x06E082, i_06e082),
    (0x06E084, i_06e084),
    (0x06E08A, i_06e08a),
    (0x06E08E, i_06e08e),
    (0x06E094, i_06e094),
    (0x06E096, i_06e096),
    (0x06E09A, i_06e09a),
    (0x06E0A0, i_06e0a0),
    (0x06E0A4, i_06e0a4),
    (0x06E0AA, i_06e0aa),
    (0x06E0B2, i_06e0b2),
    (0x06E0B6, i_06e0b6),
    (0x06E0BC, i_06e0bc),
    (0x06E0C2, i_06e0c2),
    (0x06E0C6, i_06e0c6),
    (0x06E0C8, i_06e0c8),
    (0x06E0CA, i_06e0ca),
    (0x06E0CC, i_06e0cc),
    (0x06E0CE, i_06e0ce),
    (0x06E0D0, i_06e0d0),
    (0x06E0D6, i_06e0d6),
    (0x06E0D8, i_06e0d8),
    (0x06E0DE, i_06e0de),
    (0x06E0E4, i_06e0e4),
    (0x06E0E6, i_06e0e6),
    (0x06E0EA, i_06e0ea),
    (0x06E0EE, i_06e0ee),
    (0x06E0F2, i_06e0f2),
    (0x06E0F8, i_06e0f8),
    (0x06E0FC, i_06e0fc),
    (0x06E102, i_06e102),
    (0x06E108, i_06e108),
    (0x06E10E, i_06e10e),
    (0x06E112, i_06e112),
    (0x06E114, i_06e114),
    (0x06E11A, i_06e11a),
    (0x06E11E, i_06e11e),
    (0x06E124, i_06e124),
    (0x06E126, i_06e126),
    (0x06E12C, i_06e12c),
    (0x06E130, i_06e130),
    (0x06E136, i_06e136),
    (0x06E13A, i_06e13a),
    (0x06E13E, i_06e13e),
    (0x06E140, i_06e140),
    (0x06E146, i_06e146),
    (0x06E14C, i_06e14c),
    (0x06E152, i_06e152),
    (0x06E156, i_06e156),
    (0x06E15A, i_06e15a),
    (0x06E15C, i_06e15c),
    (0x06E15E, i_06e15e),
    (0x06E164, i_06e164),
    (0x06E16A, i_06e16a),
    (0x06E16E, i_06e16e),
    (0x06E172, i_06e172),
    (0x06E174, i_06e174),
    (0x06E176, i_06e176),
    (0x06E17C, i_06e17c),
    (0x06E182, i_06e182),
    (0x06E186, i_06e186),
    (0x06E18C, i_06e18c),
    (0x06E190, i_06e190),
    (0x06E194, i_06e194),
    (0x06E196, i_06e196),
    (0x06E19C, i_06e19c),
    (0x06E1A2, i_06e1a2),
    (0x06E1A8, i_06e1a8),
    (0x06E1AE, i_06e1ae),
    (0x06E1B2, i_06e1b2),
    (0x06E1B8, i_06e1b8),
    (0x06E1BC, i_06e1bc),
    (0x06E1C0, i_06e1c0),
    (0x06E1C4, i_06e1c4),
    (0x06E1C8, i_06e1c8),
    (0x06E1CC, i_06e1cc),
    (0x06E1D0, i_06e1d0),
    (0x06E1D2, i_06e1d2),
    (0x06E1D6, i_06e1d6),
    (0x06E1DA, i_06e1da),
    (0x06E1DE, i_06e1de),
    (0x06E1E0, i_06e1e0),
    (0x06E1E2, i_06e1e2),
    (0x06E1E4, i_06e1e4),
    (0x06E1E6, i_06e1e6),
    (0x06E1EA, i_06e1ea),
    (0x06E1EC, i_06e1ec),
    (0x06E1F0, i_06e1f0),
    (0x06E1F6, i_06e1f6),
    (0x06E1FC, i_06e1fc),
    (0x06E202, i_06e202),
    (0x06E206, i_06e206),
    (0x06E20A, i_06e20a),
    (0x06E20C, i_06e20c),
    (0x06E212, i_06e212),
    (0x06E216, i_06e216),
    (0x06E218, i_06e218),
    (0x06E21C, i_06e21c),
    (0x06E21E, i_06e21e),
    (0x06E222, i_06e222),
    (0x06E224, i_06e224),
    (0x06E228, i_06e228),
    (0x06E22C, i_06e22c),
    (0x06E232, i_06e232),
    (0x06E238, i_06e238),
    (0x06E23C, i_06e23c),
    (0x06E240, i_06e240),
    (0x06E244, i_06e244),
    (0x06E248, i_06e248),
    (0x06E24A, i_06e24a),
    (0x06E24E, i_06e24e),
    (0x06E252, i_06e252),
    (0x06E258, i_06e258),
    (0x06E25E, i_06e25e),
    (0x06E262, i_06e262),
    (0x06E266, i_06e266),
    (0x06E26A, i_06e26a),
    (0x06E26E, i_06e26e),
    (0x06E270, i_06e270),
    (0x06E274, i_06e274),
    (0x06E278, i_06e278),
    (0x06E27E, i_06e27e),
    (0x06E284, i_06e284),
    (0x06E288, i_06e288),
    (0x06E28C, i_06e28c),
    (0x06E290, i_06e290),
    (0x06E294, i_06e294),
    (0x06E296, i_06e296),
    (0x06E29A, i_06e29a),
    (0x06E29E, i_06e29e),
    (0x06E2A4, i_06e2a4),
    (0x06E2AA, i_06e2aa),
    (0x06E2AE, i_06e2ae),
    (0x06E2B2, i_06e2b2),
    (0x06E2B6, i_06e2b6),
    (0x06E2BA, i_06e2ba),
    (0x06E2BC, i_06e2bc),
    (0x06E2C0, i_06e2c0),
    (0x06E2C6, i_06e2c6),
    (0x06E2CC, i_06e2cc),
    (0x06E2D2, i_06e2d2),
    (0x06E2D8, i_06e2d8),
    (0x06E2DE, i_06e2de),
    (0x06E2E4, i_06e2e4),
    (0x06E2EA, i_06e2ea),
    (0x06E2F0, i_06e2f0),
    (0x06E2F6, i_06e2f6),
    (0x06E2FC, i_06e2fc),
    (0x06E2FE, i_06e2fe),
    (0x06E302, i_06e302),
    (0x06E308, i_06e308),
    (0x06E30C, i_06e30c),
    (0x06E312, i_06e312),
    (0x06E316, i_06e316),
    (0x06E318, i_06e318),
    (0x06E31C, i_06e31c),
    (0x06E31E, i_06e31e),
    (0x06E324, i_06e324),
    (0x06E328, i_06e328),
    (0x06E32C, i_06e32c),
    (0x06E32E, i_06e32e),
    (0x06E332, i_06e332),
    (0x06E338, i_06e338),
    (0x06E33C, i_06e33c),
    (0x06E342, i_06e342),
    (0x06E348, i_06e348),
    (0x06E34A, i_06e34a),
    (0x06E34E, i_06e34e),
    (0x06E354, i_06e354),
    (0x06E356, i_06e356),
    (0x06E35A, i_06e35a),
    (0x06E360, i_06e360),
    (0x06E364, i_06e364),
    (0x06E36A, i_06e36a),
    (0x06E36E, i_06e36e),
    (0x06E374, i_06e374),
    (0x06E37A, i_06e37a),
    (0x06E37C, i_06e37c),
    (0x06E380, i_06e380),
    (0x06E384, i_06e384),
    (0x06E388, i_06e388),
    (0x06E38C, i_06e38c),
    (0x06E392, i_06e392),
    (0x06E394, i_06e394),
    (0x06E398, i_06e398),
    (0x06E39C, i_06e39c),
    (0x06E3A2, i_06e3a2),
    (0x06E3A4, i_06e3a4),
    (0x06E3A8, i_06e3a8),
    (0x06E3AE, i_06e3ae),
    (0x06E3B2, i_06e3b2),
    (0x06E3B8, i_06e3b8),
    (0x06E3BE, i_06e3be),
    (0x06E3C4, i_06e3c4),
    (0x06E3C8, i_06e3c8),
    (0x06E3CC, i_06e3cc),
    (0x06E3D0, i_06e3d0),
    (0x06E3D6, i_06e3d6),
    (0x06E3DA, i_06e3da),
    (0x06E3E0, i_06e3e0),
    (0x06E3E6, i_06e3e6),
    (0x06E3EC, i_06e3ec),
    (0x06E3F2, i_06e3f2),
    (0x06E3F8, i_06e3f8),
    (0x06E3FE, i_06e3fe),
    (0x06E402, i_06e402),
    (0x06E406, i_06e406),
    (0x06E40A, i_06e40a),
    (0x06E410, i_06e410),
    (0x06E412, i_06e412),
    (0x06E416, i_06e416),
    (0x06E418, i_06e418),
    (0x06E41C, i_06e41c),
    (0x06E422, i_06e422),
    (0x06E428, i_06e428),
    (0x06E42E, i_06e42e),
    (0x06E430, i_06e430),
    (0x06E432, i_06e432),
    (0x06E434, i_06e434),
    (0x06E436, i_06e436),
    (0x06E43C, i_06e43c),
    (0x06E442, i_06e442),
    (0x06E448, i_06e448),
    (0x06E44E, i_06e44e),
    (0x06E454, i_06e454),
    (0x06E456, i_06e456),
    (0x06E45C, i_06e45c),
    (0x06E462, i_06e462),
    (0x06E468, i_06e468),
    (0x06E46E, i_06e46e),
    (0x06E470, i_06e470),
    (0x06E476, i_06e476),
    (0x06E47C, i_06e47c),
    (0x06E482, i_06e482),
    (0x06E484, i_06e484),
    (0x06E488, i_06e488),
    (0x06E48C, i_06e48c),
    (0x06E492, i_06e492),
    (0x06E496, i_06e496),
    (0x06E49A, i_06e49a),
    (0x06E4A0, i_06e4a0),
    (0x06E4A6, i_06e4a6),
    (0x06E4A8, i_06e4a8),
    (0x06E4AC, i_06e4ac),
    (0x06E4AE, i_06e4ae),
    (0x06E4B2, i_06e4b2),
    (0x06E4B8, i_06e4b8),
    (0x06E4BE, i_06e4be),
    (0x06E4C4, i_06e4c4),
    (0x06E4C6, i_06e4c6),
    (0x06E4C8, i_06e4c8),
    (0x06E4CA, i_06e4ca),
    (0x06E4CC, i_06e4cc),
    (0x06E4D0, i_06e4d0),
    (0x06E4D6, i_06e4d6),
    (0x06E4DC, i_06e4dc),
    (0x06E4E2, i_06e4e2),
    (0x06E4E6, i_06e4e6),
    (0x06E4EA, i_06e4ea),
    (0x06E4EC, i_06e4ec),
    (0x06E4F0, i_06e4f0),
    (0x06E4F4, i_06e4f4),
    (0x06E4F8, i_06e4f8),
    (0x06E4FA, i_06e4fa),
    (0x06E4FC, i_06e4fc),
    (0x06E500, i_06e500),
    (0x06E502, i_06e502),
    (0x06E506, i_06e506),
    (0x06E50C, i_06e50c),
    (0x06E50E, i_06e50e),
    (0x06E512, i_06e512),
    (0x06E516, i_06e516),
    (0x06E51A, i_06e51a),
    (0x06E51E, i_06e51e),
    (0x06E522, i_06e522),
    (0x06E524, i_06e524),
    (0x06E528, i_06e528),
    (0x06E52A, i_06e52a),
    (0x06E52E, i_06e52e),
    (0x06E534, i_06e534),
    (0x06E53A, i_06e53a),
    (0x06E540, i_06e540),
    (0x06E546, i_06e546),
    (0x06E54A, i_06e54a),
    (0x06E54E, i_06e54e),
    (0x06E550, i_06e550),
    (0x06E554, i_06e554),
    (0x06E55A, i_06e55a),
    (0x06E55E, i_06e55e),
    (0x06E564, i_06e564),
    (0x06E56A, i_06e56a),
    (0x06E56E, i_06e56e),
    (0x06E572, i_06e572),
    (0x06E578, i_06e578),
    (0x06E57E, i_06e57e),
    (0x06E584, i_06e584),
    (0x06E58C, i_06e58c),
    (0x06E590, i_06e590),
    (0x06E594, i_06e594),
    (0x06E59A, i_06e59a),
    (0x06E5A0, i_06e5a0),
    (0x06E5A6, i_06e5a6),
    (0x06E5AC, i_06e5ac),
    (0x06E5B0, i_06e5b0),
    (0x06E5B6, i_06e5b6),
    (0x06E5BC, i_06e5bc),
    (0x06E5C0, i_06e5c0),
    (0x06E5C6, i_06e5c6),
    (0x06E5CC, i_06e5cc),
    (0x06E5D2, i_06e5d2),
    (0x06E5D8, i_06e5d8),
    (0x06E5DE, i_06e5de),
    (0x06E5E4, i_06e5e4),
    (0x06E5E8, i_06e5e8),
    (0x06E5EA, i_06e5ea),
    (0x06E5EE, i_06e5ee),
    (0x06E5F2, i_06e5f2),
    (0x06E5F8, i_06e5f8),
    (0x06E5FC, i_06e5fc),
    (0x06E602, i_06e602),
    (0x06E606, i_06e606),
    (0x06E60A, i_06e60a),
    (0x06E60C, i_06e60c),
    (0x06E610, i_06e610),
    (0x06E616, i_06e616),
    (0x06E61C, i_06e61c),
    (0x06E620, i_06e620),
    (0x06E626, i_06e626),
    (0x06E62C, i_06e62c),
    (0x06E632, i_06e632),
    (0x06E638, i_06e638),
    (0x06E63E, i_06e63e),
    (0x06E644, i_06e644),
    (0x06E648, i_06e648),
    (0x06E64C, i_06e64c),
    (0x06E64E, i_06e64e),
    (0x06E654, i_06e654),
    (0x06E65A, i_06e65a),
    (0x06E660, i_06e660),
    (0x06E664, i_06e664),
    (0x06E668, i_06e668),
    (0x06E66A, i_06e66a),
    (0x06E66C, i_06e66c),
    (0x06E670, i_06e670),
    (0x06E674, i_06e674),
    (0x06E678, i_06e678),
    (0x06E67E, i_06e67e),
    (0x06E682, i_06e682),
    (0x06E686, i_06e686),
    (0x06E688, i_06e688),
    (0x06E68C, i_06e68c),
    (0x06E690, i_06e690),
    (0x06E696, i_06e696),
    (0x06E69A, i_06e69a),
    (0x06E6A0, i_06e6a0),
    (0x06E6A6, i_06e6a6),
    (0x06E6AC, i_06e6ac),
    (0x06E6B4, i_06e6b4),
    (0x06E6BA, i_06e6ba),
    (0x06E6BE, i_06e6be),
    (0x06E6C4, i_06e6c4),
    (0x06E6C8, i_06e6c8),
    (0x06E6CC, i_06e6cc),
    (0x06E6D0, i_06e6d0),
    (0x06E6D4, i_06e6d4),
    (0x06E6DA, i_06e6da),
    (0x06E6DE, i_06e6de),
    (0x06E6E0, i_06e6e0),
    (0x06E6E4, i_06e6e4),
    (0x06E6EA, i_06e6ea),
    (0x06E6F0, i_06e6f0),
    (0x06E6F4, i_06e6f4),
    (0x06E6F6, i_06e6f6),
    (0x06E6FC, i_06e6fc),
    (0x06E700, i_06e700),
    (0x06E704, i_06e704),
    (0x06E706, i_06e706),
    (0x06E70A, i_06e70a),
    (0x06E710, i_06e710),
    (0x06E716, i_06e716),
    (0x06E71C, i_06e71c),
    (0x06E720, i_06e720),
    (0x06E724, i_06e724),
    (0x06E72A, i_06e72a),
    (0x06E72E, i_06e72e),
    (0x06E732, i_06e732),
    (0x06E734, i_06e734),
    (0x06E73A, i_06e73a),
    (0x06E740, i_06e740),
    (0x06E746, i_06e746),
    (0x06E74A, i_06e74a),
    (0x06E74E, i_06e74e),
    (0x06E750, i_06e750),
    (0x06E752, i_06e752),
    (0x06E758, i_06e758),
    (0x06E75E, i_06e75e),
    (0x06E764, i_06e764),
    (0x06E76A, i_06e76a),
    (0x06E770, i_06e770),
    (0x06E774, i_06e774),
    (0x06E778, i_06e778),
    (0x06E77C, i_06e77c),
    (0x06E780, i_06e780),
    (0x06E786, i_06e786),
    (0x06E78A, i_06e78a),
    (0x06E790, i_06e790),
    (0x06E796, i_06e796),
    (0x06E79C, i_06e79c),
    (0x06E7A0, i_06e7a0),
    (0x06E7A6, i_06e7a6),
    (0x06E7AC, i_06e7ac),
    (0x06E7B2, i_06e7b2),
    (0x06E7B8, i_06e7b8),
    (0x06E7BE, i_06e7be),
    (0x06E7C4, i_06e7c4),
    (0x06E7C8, i_06e7c8),
    (0x06E7CE, i_06e7ce),
    (0x06E7D4, i_06e7d4),
    (0x06E7D8, i_06e7d8),
    (0x06E7DE, i_06e7de),
    (0x06E7E4, i_06e7e4),
    (0x06E7EA, i_06e7ea),
    (0x06E7EE, i_06e7ee),
    (0x06E7F4, i_06e7f4),
    (0x06E7FA, i_06e7fa),
    (0x06E7FE, i_06e7fe),
    (0x06E804, i_06e804),
    (0x06E80A, i_06e80a),
    (0x06E810, i_06e810),
    (0x06E816, i_06e816),
    (0x06E818, i_06e818),
    (0x06E820, i_06e820),
    (0x06E824, i_06e824),
    (0x06E82A, i_06e82a),
    (0x06E82E, i_06e82e),
    (0x06E832, i_06e832),
    (0x06E838, i_06e838),
    (0x06E83C, i_06e83c),
    (0x06E842, i_06e842),
    (0x06E848, i_06e848),
    (0x06E84E, i_06e84e),
    (0x06E854, i_06e854),
    (0x06E85A, i_06e85a),
    (0x06E860, i_06e860),
    (0x06E866, i_06e866),
    (0x06E86C, i_06e86c),
    (0x06E870, i_06e870),
    (0x06E874, i_06e874),
    (0x06E876, i_06e876),
    (0x06E87A, i_06e87a),
    (0x06E880, i_06e880),
    (0x06E884, i_06e884),
    (0x06E888, i_06e888),
    (0x06E88A, i_06e88a),
    (0x06E88C, i_06e88c),
    (0x06E890, i_06e890),
    (0x06E896, i_06e896),
    (0x06E89C, i_06e89c),
    (0x06E8A2, i_06e8a2),
    (0x06E8A6, i_06e8a6),
    (0x06E8A8, i_06e8a8),
    (0x06E8AC, i_06e8ac),
    (0x06E8B2, i_06e8b2),
    (0x06E8B8, i_06e8b8),
    (0x06E8BC, i_06e8bc),
    (0x06E8C0, i_06e8c0),
    (0x06E8C2, i_06e8c2),
    (0x06E8C6, i_06e8c6),
    (0x06E8CA, i_06e8ca),
    (0x06E8CE, i_06e8ce),
    (0x06E8D0, i_06e8d0),
    (0x06E8D4, i_06e8d4),
    (0x06E8D8, i_06e8d8),
    (0x06E8DC, i_06e8dc),
    (0x06E8E0, i_06e8e0),
    (0x06E8E6, i_06e8e6),
    (0x06E8EA, i_06e8ea),
    (0x06E8F0, i_06e8f0),
    (0x06E8F6, i_06e8f6),
    (0x06E8FC, i_06e8fc),
    (0x06E900, i_06e900),
    (0x06E904, i_06e904),
    (0x06E90A, i_06e90a),
    (0x06E90E, i_06e90e),
    (0x06E912, i_06e912),
    (0x06E916, i_06e916),
    (0x06E91A, i_06e91a),
    (0x06E91E, i_06e91e),
    (0x06E920, i_06e920),
    (0x06E926, i_06e926),
    (0x06E92A, i_06e92a),
    (0x06E92E, i_06e92e),
    (0x06E930, i_06e930),
    (0x06E932, i_06e932),
    (0x06E936, i_06e936),
    (0x06E93C, i_06e93c),
    (0x06E942, i_06e942),
    (0x06E946, i_06e946),
    (0x06E948, i_06e948),
    (0x06E94E, i_06e94e),
    (0x06E954, i_06e954),
    (0x06E95A, i_06e95a),
    (0x06E95E, i_06e95e),
    (0x06E962, i_06e962),
    (0x06E964, i_06e964),
    (0x06E966, i_06e966),
    (0x06E96A, i_06e96a),
    (0x06E970, i_06e970),
    (0x06E976, i_06e976),
    (0x06E97C, i_06e97c),
    (0x06E982, i_06e982),
    (0x06E988, i_06e988),
    (0x06E98C, i_06e98c),
    (0x06E98E, i_06e98e),
    (0x06E994, i_06e994),
    (0x06E99A, i_06e99a),
    (0x06E99E, i_06e99e),
    (0x06E9A4, i_06e9a4),
    (0x06E9A8, i_06e9a8),
    (0x06E9AE, i_06e9ae),
    (0x06E9B2, i_06e9b2),
    (0x06E9B6, i_06e9b6),
    (0x06E9B8, i_06e9b8),
    (0x06E9BE, i_06e9be),
    (0x06E9C0, i_06e9c0),
    (0x06E9C6, i_06e9c6),
    (0x06E9CA, i_06e9ca),
    (0x06E9CE, i_06e9ce),
    (0x06E9D2, i_06e9d2),
    (0x06E9D8, i_06e9d8),
    (0x06E9DC, i_06e9dc),
    (0x06E9E0, i_06e9e0),
    (0x06E9E4, i_06e9e4),
    (0x06E9EA, i_06e9ea),
    (0x06E9F0, i_06e9f0),
    (0x06E9F6, i_06e9f6),
    (0x06E9FC, i_06e9fc),
    (0x06EA02, i_06ea02),
    (0x06EA06, i_06ea06),
    (0x06EA0A, i_06ea0a),
    (0x06EA10, i_06ea10),
    (0x06EA16, i_06ea16),
    (0x06EA1C, i_06ea1c),
    (0x06EA24, i_06ea24),
    (0x06EA2A, i_06ea2a),
    (0x06EA30, i_06ea30),
    (0x06EA34, i_06ea34),
    (0x06EA3A, i_06ea3a),
    (0x06EA40, i_06ea40),
    (0x06EA44, i_06ea44),
    (0x06EA46, i_06ea46),
    (0x06EA4C, i_06ea4c),
    (0x06EA50, i_06ea50),
    (0x06EA56, i_06ea56),
    (0x06EA5A, i_06ea5a),
    (0x06EA5E, i_06ea5e),
    (0x06EA62, i_06ea62),
    (0x06EA64, i_06ea64),
    (0x06EA68, i_06ea68),
    (0x06EA6E, i_06ea6e),
    (0x06EA72, i_06ea72),
    (0x06EA76, i_06ea76),
    (0x06EA78, i_06ea78),
    (0x06EA7E, i_06ea7e),
    (0x06EA84, i_06ea84),
    (0x06EA8A, i_06ea8a),
    (0x06EA8E, i_06ea8e),
    (0x06EA92, i_06ea92),
    (0x06EA94, i_06ea94),
    (0x06EA96, i_06ea96),
    (0x06EA9C, i_06ea9c),
    (0x06EAA2, i_06eaa2),
    (0x06EAA6, i_06eaa6),
    (0x06EAA8, i_06eaa8),
    (0x06EAAE, i_06eaae),
    (0x06EAB2, i_06eab2),
    (0x06EAB8, i_06eab8),
    (0x06EABC, i_06eabc),
    (0x06EAC0, i_06eac0),
    (0x06EAC2, i_06eac2),
    (0x06EAC6, i_06eac6),
    (0x06EACA, i_06eaca),
    (0x06EACE, i_06eace),
    (0x06EAD0, i_06ead0),
    (0x06EAD2, i_06ead2),
    (0x06EAD6, i_06ead6),
    (0x06EADC, i_06eadc),
    (0x06EAE2, i_06eae2),
    (0x06EAE8, i_06eae8),
    (0x06EAEE, i_06eaee),
    (0x06EAF4, i_06eaf4),
    (0x06EAFA, i_06eafa),
    (0x06EAFE, i_06eafe),
    (0x06EB00, i_06eb00),
    (0x06EB06, i_06eb06),
    (0x06EB0A, i_06eb0a),
    (0x06EB0E, i_06eb0e),
    (0x06EB12, i_06eb12),
    (0x06EB14, i_06eb14),
    (0x06EB1A, i_06eb1a),
    (0x06EB20, i_06eb20),
    (0x06EB26, i_06eb26),
    (0x06EB2A, i_06eb2a),
    (0x06EB2E, i_06eb2e),
    (0x06EB30, i_06eb30),
    (0x06EB36, i_06eb36),
    (0x06EB3A, i_06eb3a),
    (0x06EB40, i_06eb40),
    (0x06EB46, i_06eb46),
    (0x06EB4C, i_06eb4c),
    (0x06EB52, i_06eb52),
    (0x06EB56, i_06eb56),
    (0x06EB5A, i_06eb5a),
    (0x06EB60, i_06eb60),
    (0x06EB64, i_06eb64),
    (0x06EB66, i_06eb66),
    (0x06EB6C, i_06eb6c),
    (0x06EB72, i_06eb72),
    (0x06EB78, i_06eb78),
    (0x06EB7C, i_06eb7c),
    (0x06EB80, i_06eb80),
    (0x06EB82, i_06eb82),
    (0x06EB84, i_06eb84),
    (0x06EB86, i_06eb86),
    (0x06EB8A, i_06eb8a),
    (0x06EC2C, i_06ec2c),
    (0x06EC30, i_06ec30),
    (0x06EC34, i_06ec34),
    (0x06EC38, i_06ec38),
    (0x06EC3A, i_06ec3a),
    (0x06EC40, i_06ec40),
    (0x06EC46, i_06ec46),
    (0x06EC4A, i_06ec4a),
    (0x06EC50, i_06ec50),
    (0x06EC54, i_06ec54),
    (0x06EC58, i_06ec58),
    (0x06EC5E, i_06ec5e),
    (0x06EC64, i_06ec64),
    (0x06EC6A, i_06ec6a),
    (0x06EC70, i_06ec70),
    (0x06EC78, i_06ec78),
    (0x06EC80, i_06ec80),
    (0x06EC84, i_06ec84),
    (0x06EC88, i_06ec88),
    (0x06EC8C, i_06ec8c),
    (0x06EC92, i_06ec92),
    (0x06EC98, i_06ec98),
    (0x06EC9C, i_06ec9c),
    (0x06EC9E, i_06ec9e),
    (0x06ECA4, i_06eca4),
    (0x06ECAA, i_06ecaa),
    (0x06ECB0, i_06ecb0),
    (0x06ECB6, i_06ecb6),
    (0x06ECBA, i_06ecba),
    (0x06ECC0, i_06ecc0),
    (0x06ECC6, i_06ecc6),
    (0x06ECCC, i_06eccc),
    (0x06ECD2, i_06ecd2),
    (0x06ECD8, i_06ecd8),
    (0x06ECDC, i_06ecdc),
    (0x06ECDE, i_06ecde),
    (0x06ECE2, i_06ece2),
    (0x06ECE6, i_06ece6),
    (0x06ECEC, i_06ecec),
    (0x06ECF0, i_06ecf0),
    (0x06ECF4, i_06ecf4),
    (0x06ECFA, i_06ecfa),
    (0x06ED00, i_06ed00),
    (0x06ED04, i_06ed04),
    (0x06ED08, i_06ed08),
    (0x06ED0A, i_06ed0a),
    (0x06ED10, i_06ed10),
    (0x06ED14, i_06ed14),
    (0x06ED18, i_06ed18),
    (0x06ED1E, i_06ed1e),
    (0x06ED22, i_06ed22),
    (0x06ED24, i_06ed24),
    (0x06ED2A, i_06ed2a),
    (0x06ED30, i_06ed30),
    (0x06ED36, i_06ed36),
    (0x06ED3A, i_06ed3a),
    (0x06ED3E, i_06ed3e),
    (0x06ED40, i_06ed40),
    (0x06ED42, i_06ed42),
    (0x06ED46, i_06ed46),
    (0x06ED4A, i_06ed4a),
    (0x06ED52, i_06ed52),
    (0x06ED5A, i_06ed5a),
    (0x06ED5E, i_06ed5e),
    (0x06ED62, i_06ed62),
    (0x06ED66, i_06ed66),
    (0x06ED6A, i_06ed6a),
    (0x06ED6E, i_06ed6e),
    (0x06ED74, i_06ed74),
    (0x06ED7A, i_06ed7a),
    (0x06ED7E, i_06ed7e),
    (0x06ED80, i_06ed80),
    (0x06ED84, i_06ed84),
    (0x06ED88, i_06ed88),
    (0x06ED8E, i_06ed8e),
    (0x06ED94, i_06ed94),
    (0x06ED98, i_06ed98),
    (0x06ED9C, i_06ed9c),
    (0x06ED9E, i_06ed9e),
    (0x06EDA2, i_06eda2),
    (0x06EDA6, i_06eda6),
    (0x06EDAA, i_06edaa),
    (0x06EDB0, i_06edb0),
    (0x06EDB4, i_06edb4),
    (0x06EDB8, i_06edb8),
    (0x06EDBC, i_06edbc),
    (0x06EDC0, i_06edc0),
    (0x06EDC4, i_06edc4),
    (0x06EDC8, i_06edc8),
    (0x06EDCA, i_06edca),
    (0x06EDCE, i_06edce),
    (0x06EDD2, i_06edd2),
    (0x06EDD8, i_06edd8),
    (0x06EDDE, i_06edde),
    (0x06EDE2, i_06ede2),
    (0x06EDE6, i_06ede6),
    (0x06EDEA, i_06edea),
    (0x06EDF0, i_06edf0),
    (0x06EDF4, i_06edf4),
    (0x06EDF8, i_06edf8),
    (0x06EDFC, i_06edfc),
    (0x06EE02, i_06ee02),
    (0x06EE08, i_06ee08),
    (0x06EE28, i_06ee28),
    (0x06EE2C, i_06ee2c),
    (0x06EE32, i_06ee32),
    (0x06EE38, i_06ee38),
    (0x06EE3E, i_06ee3e),
    (0x06EE42, i_06ee42),
    (0x06EE48, i_06ee48),
    (0x06EE4E, i_06ee4e),
    (0x06EE52, i_06ee52),
    (0x06EE58, i_06ee58),
    (0x06EE5E, i_06ee5e),
    (0x06EE64, i_06ee64),
    (0x06EE68, i_06ee68),
    (0x06EE6A, i_06ee6a),
    (0x06EE70, i_06ee70),
    (0x06EE76, i_06ee76),
    (0x06EE7A, i_06ee7a),
    (0x06EE7E, i_06ee7e),
    (0x06EE80, i_06ee80),
    (0x06EE82, i_06ee82),
    (0x06EE86, i_06ee86),
    (0x06EE8A, i_06ee8a),
    (0x06EE8E, i_06ee8e),
    (0x06EE94, i_06ee94),
    (0x06EE9A, i_06ee9a),
    (0x06EEA0, i_06eea0),
    (0x06EEA8, i_06eea8),
    (0x06EEAE, i_06eeae),
    (0x06EEB2, i_06eeb2),
    (0x06EEB6, i_06eeb6),
    (0x06EEB8, i_06eeb8),
    (0x06EEBE, i_06eebe),
    (0x06EEC4, i_06eec4),
    (0x06EEC8, i_06eec8),
    (0x06EECC, i_06eecc),
    (0x06EECE, i_06eece),
    (0x06EED2, i_06eed2),
    (0x06EED6, i_06eed6),
    (0x06EEDA, i_06eeda),
    (0x06EEDE, i_06eede),
    (0x06EEE2, i_06eee2),
    (0x06EEE6, i_06eee6),
    (0x06EEEA, i_06eeea),
    (0x06EEEC, i_06eeec),
    (0x06EEF2, i_06eef2),
    (0x06EEF6, i_06eef6),
    (0x06EEFA, i_06eefa),
    (0x06EEFC, i_06eefc),
    (0x06EF02, i_06ef02),
    (0x06EF06, i_06ef06),
    (0x06EF0C, i_06ef0c),
    (0x06EF0E, i_06ef0e),
    (0x06EF10, i_06ef10),
    (0x06EF16, i_06ef16),
    (0x06EF1A, i_06ef1a),
    (0x06EF1C, i_06ef1c),
    (0x06EF22, i_06ef22),
    (0x06EF26, i_06ef26),
    (0x06EF2A, i_06ef2a),
    (0x06EF2C, i_06ef2c),
    (0x06EF30, i_06ef30),
    (0x06EF34, i_06ef34),
    (0x06EF38, i_06ef38),
    (0x06EF3A, i_06ef3a),
    (0x06EF40, i_06ef40),
    (0x06EF44, i_06ef44),
    (0x06EF48, i_06ef48),
    (0x06EF4C, i_06ef4c),
    (0x06EF50, i_06ef50),
    (0x06EF54, i_06ef54),
    (0x06EF5A, i_06ef5a),
    (0x06EF5E, i_06ef5e),
    (0x06EF60, i_06ef60),
    (0x06EF64, i_06ef64),
    (0x06EF66, i_06ef66),
    (0x06EF6A, i_06ef6a),
    (0x06EF70, i_06ef70),
    (0x06EF74, i_06ef74),
    (0x06EF78, i_06ef78),
    (0x06EF7A, i_06ef7a),
    (0x06EF7E, i_06ef7e),
    (0x06EF80, i_06ef80),
    (0x06EF84, i_06ef84),
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
    (0x06F0BE, i_06f0be),
    (0x06F0C4, i_06f0c4),
    (0x06F0CC, i_06f0cc),
    (0x06F0D2, i_06f0d2),
    (0x06F0DA, i_06f0da),
    (0x06F0E0, i_06f0e0),
    (0x06F0E8, i_06f0e8),
    (0x06F0EE, i_06f0ee),
    (0x06F0F6, i_06f0f6),
    (0x06F0FC, i_06f0fc),
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
];
