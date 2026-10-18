// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región bios_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $C16DCA: moveq #$7, d7
pub fn i_c16dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DCC;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16DCC: move.l (a1)+, (a0)+
pub fn i_c16dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DCE;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C16DCE: dbra d7, $c16dcc
pub fn i_c16dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DD2;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16DD2; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16DD2; return 14; }
    cpu.pc = 0xC16DCC;
    10
}
/// $C16DD2: jsr $c12a0e(pc)
pub fn i_c16dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16DD6);
    cpu.pc = 0xC12A0E;
    16
}
/// $C16DD6: addq.b #$1, $ac2(a5)
pub fn i_c16dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DDA;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16DDA: movea.l $abe(a5), a0
pub fn i_c16dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DDE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C16DDE: move.l #$c1fdaa, (a0)+
pub fn i_c16dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DE4;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1FDAA;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16DE4: move.l a0, $abe(a5)
pub fn i_c16de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DE8;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C16DE8: subq.b #$1, $ac2(a5)
pub fn i_c16de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DEC;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16DEC: move.w #$12c, $9dc(a5)
pub fn i_c16dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DF2;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16DF2: move.w #$9, $9d8(a5)
pub fn i_c16df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DF8;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16DF8: rts
pub fn i_c16df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16DFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16DFA: move.l #$eee0000, $400002.l
pub fn i_c16dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E04;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C16E04: jsr $c12a0e(pc)
pub fn i_c16e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E08);
    cpu.pc = 0xC12A0E;
    16
}
/// $C16E08: addq.b #$1, $ac2(a5)
pub fn i_c16e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E0C;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16E0C: movea.l $abe(a5), a0
pub fn i_c16e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E10;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C16E10: move.l #$c1fe52, (a0)+
pub fn i_c16e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E16;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1FE52;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C16E16: bra.b $c16de4
pub fn i_c16e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E18;
    cpu.ir = 0x60CC;
    cpu.pc = 0xC16DE4;
    10
}
/// $C16E20: cmpi.w #$9, $9d8(a5)
pub fn i_c16e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E26;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x9, Size::Word, false);
    16
}
/// $C16E26: bne.w $c16e5c
pub fn i_c16e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E2A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC16E5C; return 10; }
    8
}
/// $C16E2A: tst.b $a83(a5)
pub fn i_c16e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E2E;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xA83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C16E2E: beq.w $c16e5c
pub fn i_c16e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC16E5C; return 10; }
    8
}
/// $C16E32: clr.l $400002.l
pub fn i_c16e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E38;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x400002);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x400002, r2);
    20
}
/// $C16E38: jsr $c12a0e(pc)
pub fn i_c16e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E3C);
    cpu.pc = 0xC12A0E;
    16
}
/// $C16E3C: jsr $c10fae(pc)
pub fn i_c16e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E40);
    cpu.pc = 0xC10FAE;
    16
}
/// $C16E40: jsr $c1727e(pc)
pub fn i_c16e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E44);
    cpu.pc = 0xC1727E;
    16
}
/// $C16E44: move.b #$3, $320000.l
pub fn i_c16e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E4C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x320000, v1 as u8);
    16
}
/// $C16E4C: move.b d0, $3a001b.l
pub fn i_c16e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E52;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001B, v1 as u8);
    16
}
/// $C16E52: move.b d0, $3a0013.l
pub fn i_c16e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E58;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0013, v1 as u8);
    16
}
/// $C16E58: clr.b $b8d(a5)
pub fn i_c16e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E5C;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xB8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C16E5C: jmp $c102d2.l
pub fn i_c16e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E62;
    cpu.ir = 0x4EF9;
    cpu.pc = 0xC102D2;
    8
}
/// $C16E62: move.w $9da(a5), d1
pub fn i_c16e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E66;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C16E66: lea.l $c1f80e.l, a0
pub fn i_c16e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E6C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xC1F80E;
    20
}
/// $C16E6C: moveq #$0, d0
pub fn i_c16e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E6E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16E6E: move.b (a0, d1.w), d0
pub fn i_c16e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E72;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C16E72: jsr $c16eb8(pc)
pub fn i_c16e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16E76);
    cpu.pc = 0xC16EB8;
    16
}
/// $C16E76: addq.w #$1, $9da(a5)
pub fn i_c16e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E7A;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9DA);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C16E7A: rts
pub fn i_c16e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16E7C: lea.l $3c0002.l, a1
pub fn i_c16e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E82;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3C0002;
    20
}
/// $C16E82: move.w #$1, $2(a1)
pub fn i_c16e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E88;
    cpu.ir = 0x337C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16E88: moveq #$11, d7
pub fn i_c16e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E8A;
    cpu.ir = 0x7E11;
    cpu.d[7] = 0x11;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16E8A: move.w d0, -$2(a1)
pub fn i_c16e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E8E;
    cpu.ir = 0x3340;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16E8E: moveq #$3, d6
pub fn i_c16e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E90;
    cpu.ir = 0x7C03;
    cpu.d[6] = 0x3;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16E90: move.b (a0)+, d2
pub fn i_c16e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E92;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C16E92: move.w d2, (a1)
pub fn i_c16e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E94;
    cpu.ir = 0x3282;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16E94: nop
pub fn i_c16e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E96;
    cpu.ir = 0x4E71;
    4
}
/// $C16E96: move.w d1, (a1)
pub fn i_c16e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E98;
    cpu.ir = 0x3281;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16E98: dbra d6, $c16e90
pub fn i_c16e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E9C;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16E9C; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16E9C; return 14; }
    cpu.pc = 0xC16E90;
    10
}
/// $C16E9C: moveq #$1b, d6
pub fn i_c16e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16E9E;
    cpu.ir = 0x7C1B;
    cpu.d[6] = 0x1B;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16E9E: move.b #$ff, d2
pub fn i_c16e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EA2;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C16EA2: move.w d2, (a1)
pub fn i_c16ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EA4;
    cpu.ir = 0x3282;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16EA4: nop
pub fn i_c16ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EA6;
    cpu.ir = 0x4E71;
    4
}
/// $C16EA6: move.w #$0, (a1)
pub fn i_c16ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EAA;
    cpu.ir = 0x32BC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v1 as u16);
    8
}
/// $C16EAA: dbra d6, $c16ea2
pub fn i_c16eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EAE;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16EAE; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16EAE; return 14; }
    cpu.pc = 0xC16EA2;
    10
}
/// $C16EAE: addi.w #$40, d0
pub fn i_c16eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EB2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C16EB2: dbra d7, $c16e8a
pub fn i_c16eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EB6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16EB6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16EB6; return 14; }
    cpu.pc = 0xC16E8A;
    10
}
/// $C16EB6: rts
pub fn i_c16eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16EB8: move.w #$c, d1
pub fn i_c16eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EBC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16EBC: mulu.w d1, d0
pub fn i_c16ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EBE;
    cpu.ir = 0xC0C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C16EBE: addi.l #$c1f83e, d0
pub fn i_c16ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EC4;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC1F83E, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $C16EC4: move.l d0, $9e8(a5)
pub fn i_c16ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EC8;
    cpu.ir = 0x2B40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C16EC8: move.w #$1, $9e6(a5)
pub fn i_c16ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ECE;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E6);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16ECE: rts
pub fn i_c16ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ED0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16ED0: lea.l $3c0002.l, a0
pub fn i_c16ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16ED6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x3C0002;
    20
}
/// $C16ED6: move.w #$1, $2(a0)
pub fn i_c16ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EDC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16EDC: move.w #$7020, -$2(a0)
pub fn i_c16edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EE2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x7020;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16EE2: move.w #$ff, d0
pub fn i_c16ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EE6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16EE6: move.w #$4bf, d1
pub fn i_c16ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EEA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4BF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16EEA: jsr $c16f04(pc)
pub fn i_c16eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16EEE);
    cpu.pc = 0xC16F04;
    16
}
/// $C16EEE: move.w #$20, d0
pub fn i_c16eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EF2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16EF2: move.w #$1f, d1
pub fn i_c16ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EF6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16EF6: jsr $c16f04(pc)
pub fn i_c16ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16EFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16EFA);
    cpu.pc = 0xC16F04;
    16
}
/// $C16EFA: move.w #$7000, -$2(a0)
pub fn i_c16efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F00;
    cpu.ir = 0x317C;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16F00: move.w #$1f, d1
pub fn i_c16f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F04;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F04: move.w d0, (a0)
pub fn i_c16f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F06;
    cpu.ir = 0x3080;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C16F06: dbra d1, $c16f04
pub fn i_c16f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F0A;
    cpu.ir = 0x51C9;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC16F0A; return 12; }
    let n = (cpu.d[1] as u16).wrapping_sub(1);
    cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC16F0A; return 14; }
    cpu.pc = 0xC16F04;
    10
}
/// $C16F0A: rts
pub fn i_c16f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16F0C: lea.l $3c0002.l, a0
pub fn i_c16f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x3C0002;
    20
}
/// $C16F12: move.w #$1, $2(a0)
pub fn i_c16f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F18;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16F18: move.w #$8000, -$2(a0)
pub fn i_c16f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F1E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16F1E: move.w #$fff, d0
pub fn i_c16f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F22;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F22: move.w #$1ff, d1
pub fn i_c16f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F26;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F26: jsr $c16f04(pc)
pub fn i_c16f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16F2A);
    cpu.pc = 0xC16F04;
    16
}
/// $C16F2A: moveq #$0, d0
pub fn i_c16f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F2C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16F2C: move.w #$1ff, d1
pub fn i_c16f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F30;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F30: jsr $c16f04(pc)
pub fn i_c16f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16F34);
    cpu.pc = 0xC16F04;
    16
}
/// $C16F34: move.w #$be00, d0
pub fn i_c16f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F38;
    cpu.ir = 0x303C;
    let v1: u32 = 0xBE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F38: move.w #$1ff, d1
pub fn i_c16f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F3C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F3C: jsr $c16f04(pc)
pub fn i_c16f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16F40);
    cpu.pc = 0xC16F04;
    16
}
/// $C16F40: move.w #$0, -$2(a0)
pub fn i_c16f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F46;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16F46: move.w #$ff, d0
pub fn i_c16f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F4A: move.w #$2, $2(a0)
pub fn i_c16f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F50;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C16F50: moveq #$1f, d1
pub fn i_c16f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F52;
    cpu.ir = 0x721F;
    cpu.d[1] = 0x1F;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16F52: bra.b $c16f04
pub fn i_c16f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F54;
    cpu.ir = 0x60B0;
    cpu.pc = 0xC16F04;
    10
}
/// $C16F54: move.w d0, d1
pub fn i_c16f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F56;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C16F56: lsr.b #$4, d1
pub fn i_c16f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F58;
    cpu.ir = 0xE809;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $C16F58: jsr $c16f62(pc)
pub fn i_c16f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC16F5C);
    cpu.pc = 0xC16F62;
    16
}
/// $C16F5C: exg.l d0, d1
pub fn i_c16f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F5E;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $C16F5E: andi.b #$f, d1
pub fn i_c16f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F62;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F62: cmpi.b #$a, d1
pub fn i_c16f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F66;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xA, Size::Byte, false);
    8
}
/// $C16F66: bcc.b $c16f6e
pub fn i_c16f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F68;
    cpu.ir = 0x6406;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC16F6E; return 10; }
    8
}
/// $C16F68: addi.b #$30, d1
pub fn i_c16f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F6C;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F6C: rts
pub fn i_c16f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16F6E: addi.b #$37, d1
pub fn i_c16f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F72;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x37, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C16F72: rts
pub fn i_c16f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16F74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C16FD4: lea.l $10f300.l, a5
pub fn i_c16fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FDA;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C16FDA: lea.l $c16f74(pc), a4
pub fn i_c16fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FDE;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0xC16F74;
    16
}
/// $C16FDE: lea.l $380001.l, a2
pub fn i_c16fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FE4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x380001;
    20
}
/// $C16FE4: moveq #$0, d0
pub fn i_c16fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FE6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C16FE6: move.b d0, $a94(a5)
pub fn i_c16fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FEA;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA94);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C16FEA: move.b d0, $a9a(a5)
pub fn i_c16fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FEE;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C16FEE: move.b d0, $aa0(a5)
pub fn i_c16fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FF2;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAA0);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C16FF2: move.b d0, $aa6(a5)
pub fn i_c16ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FF6;
    cpu.ir = 0x1B40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAA6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C16FF6: move.b $300001.l, d0
pub fn i_c16ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC16FFC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x300001) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C16FFC: andi.b #$4, d0
pub fn i_c16ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17000;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17000: bne.b $c17008
pub fn i_c17000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17002;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17008; return 10; }
    8
}
/// $C17002: move.b #$3, $a94(a5)
pub fn i_c17002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17008;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA94);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17008: move.b #$0, (a2)
pub fn i_c17008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1700C;
    cpu.ir = 0x14BC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C1700C: rts
pub fn i_c1700c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1700E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1705E: bsr.w $c1706a
pub fn i_c1705e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17062;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC17062);
    cpu.pc = 0xC1706A;
    18
}
/// $C17062: bsr.w $c1706a
pub fn i_c17062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17066;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC17066);
    cpu.pc = 0xC1706A;
    18
}
/// $C17066: bsr.w $c17072
pub fn i_c17066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1706A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC1706A);
    cpu.pc = 0xC17072;
    18
}
/// $C1706A: bsr.w $c17072
pub fn i_c1706a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1706E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC1706E);
    cpu.pc = 0xC17072;
    18
}
/// $C1706E: bsr.w $c17072
pub fn i_c1706e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17072;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC17072);
    cpu.pc = 0xC17072;
    18
}
/// $C17072: nop
pub fn i_c17072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17074;
    cpu.ir = 0x4E71;
    4
}
/// $C17074: nop
pub fn i_c17074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17076;
    cpu.ir = 0x4E71;
    4
}
/// $C17076: nop
pub fn i_c17076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17078;
    cpu.ir = 0x4E71;
    4
}
/// $C17078: nop
pub fn i_c17078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1707A;
    cpu.ir = 0x4E71;
    4
}
/// $C1707A: nop
pub fn i_c1707a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1707C;
    cpu.ir = 0x4E71;
    4
}
/// $C1707C: nop
pub fn i_c1707c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1707E;
    cpu.ir = 0x4E71;
    4
}
/// $C1707E: rts
pub fn i_c1707e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17080;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17080: lea.l $10f300.l, a5
pub fn i_c17080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17086;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C17086: lea.l $a94(a5), a0
pub fn i_c17086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1708A;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA94);
    cpu.a[0] = ad1;
    16
}
/// $C1708A: lea.l $380001.l, a2
pub fn i_c1708a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17090;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x380001;
    20
}
/// $C17090: moveq #$0, d1
pub fn i_c17090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17092;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17092: move.b #$12, (a2)
pub fn i_c17092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17096;
    cpu.ir = 0x14BC;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C17096: bsr.b $c17066
pub fn i_c17096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17098;
    cpu.ir = 0x61CE;
    cpu.push32(bus, 0xC17098);
    cpu.pc = 0xC17066;
    18
}
/// $C17098: move.b $380000.l, d1
pub fn i_c17098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1709E;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1709E: move.b $340000.l, d5
pub fn i_c1709e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170A4;
    cpu.ir = 0x1A39;
    let v2 = bus.read8(0x340000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C170A4: swap d5
pub fn i_c170a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170A6;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C170A6: move.b $300000.l, d5
pub fn i_c170a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170AC;
    cpu.ir = 0x1A39;
    let v2 = bus.read8(0x300000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C170AC: lsl.b #$4, d1
pub fn i_c170ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170AE;
    cpu.ir = 0xE909;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $C170AE: move.b #$1b, (a2)
pub fn i_c170ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170B2;
    cpu.ir = 0x14BC;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C170B2: bsr.b $c17066
pub fn i_c170b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170B4;
    cpu.ir = 0x61B2;
    cpu.push32(bus, 0xC170B4);
    cpu.pc = 0xC17066;
    18
}
/// $C170B4: move.b $380000.l, d2
pub fn i_c170b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170BA;
    cpu.ir = 0x1439;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C170BA: move.b $340000.l, d4
pub fn i_c170ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170C0;
    cpu.ir = 0x1839;
    let v2 = bus.read8(0x340000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C170C0: swap d4
pub fn i_c170c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170C2;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C170C2: move.b $300000.l, d4
pub fn i_c170c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170C8;
    cpu.ir = 0x1839;
    let v2 = bus.read8(0x300000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C170C8: andi.b #$f, d2
pub fn i_c170c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170CC;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C170CC: or.b d1, d2
pub fn i_c170cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170CE;
    cpu.ir = 0x8401;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C170CE: not.b d2
pub fn i_c170ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170D0;
    cpu.ir = 0x4602;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C170D0: move.b $10fedc.l, d3
pub fn i_c170d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170D6;
    cpu.ir = 0x1639;
    let v2 = bus.read8(0x10FEDC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C170D6: eor.b d2, d3
pub fn i_c170d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170D8;
    cpu.ir = 0xB503;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C170D8: and.b d2, d3
pub fn i_c170d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170DA;
    cpu.ir = 0xC602;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C170DA: cmpi.b #$3, $a94(a5)
pub fn i_c170da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170E0;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xA94);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $C170E0: beq.b $c170ea
pub fn i_c170e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170E2;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC170EA; return 10; }
    8
}
/// $C170E2: cmpi.b #$3, $a9a(a5)
pub fn i_c170e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170E8;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xA9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $C170E8: bne.b $c170ee
pub fn i_c170e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170EA;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC170EE; return 10; }
    8
}
/// $C170EA: jsr $c1724c(pc)
pub fn i_c170ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC170EE);
    cpu.pc = 0xC1724C;
    16
}
/// $C170EE: move.b #$cf, d7
pub fn i_c170ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170F2;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0xCF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C170F2: jsr $c171c8(pc)
pub fn i_c170f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC170F6);
    cpu.pc = 0xC171C8;
    16
}
/// $C170F6: swap d4
pub fn i_c170f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170F8;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C170F8: swap d5
pub fn i_c170f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170FA;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C170FA: swap d6
pub fn i_c170fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC170FC;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C170FC: lea.l $a9a(a5), a0
pub fn i_c170fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17100;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA9A);
    cpu.a[0] = ad1;
    16
}
/// $C17100: move.b #$3f, d7
pub fn i_c17100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17104;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0x3F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C17104: jsr $c171c8(pc)
pub fn i_c17104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17108;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17108);
    cpu.pc = 0xC171C8;
    16
}
/// $C17108: lea.l $a94(a5), a0
pub fn i_c17108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1710C;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA94);
    cpu.a[0] = ad1;
    16
}
/// $C1710C: moveq #$1, d0
pub fn i_c1710c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1710E;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1710E: jsr $c171a0(pc)
pub fn i_c1710e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17112;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17112);
    cpu.pc = 0xC171A0;
    16
}
/// $C17112: lea.l $a9a(a5), a0
pub fn i_c17112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17116;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA9A);
    cpu.a[0] = ad1;
    16
}
/// $C17116: moveq #$3, d0
pub fn i_c17116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17118;
    cpu.ir = 0x7003;
    cpu.d[0] = 0x3;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17118: jsr $c171a0(pc)
pub fn i_c17118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1711C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1711C);
    cpu.pc = 0xC171A0;
    16
}
/// $C1711C: move.b d2, $bdc(a5)
pub fn i_c1711c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17120;
    cpu.ir = 0x1B42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBDC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17120: move.b d3, $bdd(a5)
pub fn i_c17120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17124;
    cpu.ir = 0x1B43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBDD);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17124: andi.b #$55, d2
pub fn i_c17124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17128;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x55) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17128: andi.b #$55, d3
pub fn i_c17128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1712C;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x55) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1712C: move.b d2, $aac(a5)
pub fn i_c1712c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17130;
    cpu.ir = 0x1B42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAAC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17130: move.b d3, $aad(a5)
pub fn i_c17130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17134;
    cpu.ir = 0x1B43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAAD);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17134: move.b #$0, $380001.l
pub fn i_c17134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1713C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380001, v1 as u8);
    16
}
/// $C1713C: lea.l $a94(a5), a0
pub fn i_c1713c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17140;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA94);
    cpu.a[0] = ad1;
    16
}
/// $C17140: lea.l $be8(a5), a1
pub fn i_c17140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17144;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xBE8);
    cpu.a[1] = ad1;
    16
}
/// $C17144: jsr $c17150(pc)
pub fn i_c17144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17148;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17148);
    cpu.pc = 0xC17150;
    16
}
/// $C17148: lea.l $a9a(a5), a0
pub fn i_c17148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1714C;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xA9A);
    cpu.a[0] = ad1;
    16
}
/// $C1714C: lea.l $bee(a5), a1
pub fn i_c1714c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17150;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xBEE);
    cpu.a[1] = ad1;
    16
}
/// $C17150: cmpi.b #$3, (a0)
pub fn i_c17150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17154;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    12
}
/// $C17154: beq.b $c17160
pub fn i_c17154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17156;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17160; return 10; }
    8
}
/// $C17156: move.l (a0), (a1)
pub fn i_c17156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17158;
    cpu.ir = 0x2290;
    let v2 = bus.read32(cpu.a[0]);
    bus.write32(cpu.a[1], v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C17158: move.w $4(a0), $4(a1)
pub fn i_c17158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1715E;
    cpu.ir = 0x3368;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x4);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1715E: rts
pub fn i_c1715e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17160;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17160: move.b #$1, (a1)
pub fn i_c17160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17164;
    cpu.ir = 0x12BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $C17164: move.b $1(a0), d0
pub fn i_c17164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17168;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17168: move.b $2(a0), d1
pub fn i_c17168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1716C;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1716C: jsr $c1718e(pc)
pub fn i_c1716c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17170;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17170);
    cpu.pc = 0xC1718E;
    16
}
/// $C17170: move.b d0, $1(a1)
pub fn i_c17170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17174;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17174: move.b d0, $2(a1)
pub fn i_c17174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17178;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17178: move.b $d(a0), d0
pub fn i_c17178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1717C;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0xD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1717C: move.b $e(a0), d1
pub fn i_c1717c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17180;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17180: jsr $c1718e(pc)
pub fn i_c17180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17184;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17184);
    cpu.pc = 0xC1718E;
    16
}
/// $C17184: move.b d0, $3(a1)
pub fn i_c17184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17188;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17188: move.b d0, $4(a1)
pub fn i_c17188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1718C;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1718C: rts
pub fn i_c1718c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1718E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1718E: bclr.b #$7, d0
pub fn i_c1718e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17192;
    cpu.ir = 0x880;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[0] = r2;
    8
}
/// $C17192: btst.b #$0, d1
pub fn i_c17192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17196;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C17196: beq.b $c1719c
pub fn i_c17196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17198;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1719C; return 10; }
    8
}
/// $C17198: bset.b #$7, d0
pub fn i_c17198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1719C;
    cpu.ir = 0x8C0;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $C1719C: ror.b #$4, d0
pub fn i_c1719c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1719E;
    cpu.ir = 0xE818;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 3, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C1719E: rts
pub fn i_c1719e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C171A0: cmpi.b #$3, (a0)
pub fn i_c171a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171A4;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    12
}
/// $C171A4: bne.b $c171c6
pub fn i_c171a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171A6;
    cpu.ir = 0x6620;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC171C6; return 10; }
    8
}
/// $C171A6: btst.l d0, d2
pub fn i_c171a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171A8;
    cpu.ir = 0x102;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C171A8: beq.b $c171c6
pub fn i_c171a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171AA;
    cpu.ir = 0x671C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC171C6; return 10; }
    8
}
/// $C171AA: move.b $3(a0), d5
pub fn i_c171aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171AE;
    cpu.ir = 0x1A28;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C171AE: andi.b #$1f, d5
pub fn i_c171ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171B2;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x1F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C171B2: move.b $1(a0), d4
pub fn i_c171b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171B6;
    cpu.ir = 0x1828;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C171B6: and.b d5, d4
pub fn i_c171b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171B8;
    cpu.ir = 0xC805;
    let v1 = cpu.d[5] & 0xFF;
    let r = (v1 & cpu.d[4]) & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C171B8: bne.b $c171c2
pub fn i_c171b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171BA;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC171C2; return 10; }
    8
}
/// $C171BA: move.b $2(a0), d4
pub fn i_c171ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171BE;
    cpu.ir = 0x1828;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C171BE: and.b d5, d4
pub fn i_c171be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171C0;
    cpu.ir = 0xC805;
    let v1 = cpu.d[5] & 0xFF;
    let r = (v1 & cpu.d[4]) & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C171C0: beq.b $c171c6
pub fn i_c171c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171C2;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC171C6; return 10; }
    8
}
/// $C171C2: bclr.b d0, d2
pub fn i_c171c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171C4;
    cpu.ir = 0x182;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[2] = r2;
    8
}
/// $C171C4: bclr.b d0, d3
pub fn i_c171c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171C6;
    cpu.ir = 0x183;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[3] = r2;
    8
}
/// $C171C6: rts
pub fn i_c171c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C171C8: cmpi.b #$3, (a0)
pub fn i_c171c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171CC;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    12
}
/// $C171CC: beq.b $c17226
pub fn i_c171cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171CE;
    cpu.ir = 0x6758;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17226; return 10; }
    8
}
/// $C171CE: move.b d4, d0
pub fn i_c171ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171D0;
    cpu.ir = 0x1004;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C171D0: jsr $c171de(pc)
pub fn i_c171d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC171D4);
    cpu.pc = 0xC171DE;
    16
}
/// $C171D4: lea.l $c(a0), a0
pub fn i_c171d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171D8;
    cpu.ir = 0x41E8;
    let ad1 = cpu.a[0].wrapping_add(0xC);
    cpu.a[0] = ad1;
    16
}
/// $C171D8: tst.b (a0)
pub fn i_c171d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171DA;
    cpu.ir = 0x4A10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $C171DA: beq.b $c17218
pub fn i_c171da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171DC;
    cpu.ir = 0x673C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17218; return 10; }
    8
}
/// $C171DC: move.b d5, d0
pub fn i_c171dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171DE;
    cpu.ir = 0x1005;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C171DE: not.b d0
pub fn i_c171de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171E0;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C171E0: move.b $2(a0), d1
pub fn i_c171e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171E4;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C171E4: cmp.b d0, d1
pub fn i_c171e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171E6;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C171E6: bne.b $c171f6
pub fn i_c171e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171E8;
    cpu.ir = 0x660E;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC171F6; return 10; }
    8
}
/// $C171E8: subq.b #$1, $5(a0)
pub fn i_c171e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171EC;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x5);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C171EC: bpl.b $c17202
pub fn i_c171ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171EE;
    cpu.ir = 0x6A14;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC17202; return 10; }
    8
}
/// $C171EE: move.b #$8, $5(a0)
pub fn i_c171ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171F4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C171F4: bra.b $c171fc
pub fn i_c171f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171F6;
    cpu.ir = 0x6006;
    cpu.pc = 0xC171FC;
    10
}
/// $C171F6: move.b #$10, $5(a0)
pub fn i_c171f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC171FC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C171FC: move.b d0, $4(a0)
pub fn i_c171fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17200;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17200: bra.b $c17206
pub fn i_c17200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17202;
    cpu.ir = 0x6004;
    cpu.pc = 0xC17206;
    10
}
/// $C17202: clr.b $4(a0)
pub fn i_c17202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17206;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17206: move.b d1, $1(a0)
pub fn i_c17206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1720A;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1720A: move.b d0, $2(a0)
pub fn i_c1720a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1720E;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1720E: eor.b d0, d1
pub fn i_c1720e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17210;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C17210: and.b d0, d1
pub fn i_c17210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17212;
    cpu.ir = 0xC200;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17212: move.b d1, $3(a0)
pub fn i_c17212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17216;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17216: rts
pub fn i_c17216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17218;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17218: clr.b $1(a0)
pub fn i_c17218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1721C;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x1);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1721C: clr.l $2(a0)
pub fn i_c1721c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17220;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x2);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C17220: and.b d7, d2
pub fn i_c17220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17222;
    cpu.ir = 0xC407;
    let v1 = cpu.d[7] & 0xFF;
    let r = (v1 & cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17222: and.b d7, d3
pub fn i_c17222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17224;
    cpu.ir = 0xC607;
    let v1 = cpu.d[7] & 0xFF;
    let r = (v1 & cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17224: rts
pub fn i_c17224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17226;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17226: and.b d7, d2
pub fn i_c17226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17228;
    cpu.ir = 0xC407;
    let v1 = cpu.d[7] & 0xFF;
    let r = (v1 & cpu.d[2]) & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17228: and.b d7, d3
pub fn i_c17228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1722A;
    cpu.ir = 0xC607;
    let v1 = cpu.d[7] & 0xFF;
    let r = (v1 & cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1722A: move.b d6, d0
pub fn i_c1722a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1722C;
    cpu.ir = 0x1006;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1722C: jsr $c1723a(pc)
pub fn i_c1722c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17230;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17230);
    cpu.pc = 0xC1723A;
    16
}
/// $C17230: move.b d5, d0
pub fn i_c17230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17232;
    cpu.ir = 0x1005;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C17232: jsr $c1723a(pc)
pub fn i_c17232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17236;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17236);
    cpu.pc = 0xC1723A;
    16
}
/// $C17236: lsr.w #$8, d6
pub fn i_c17236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17238;
    cpu.ir = 0xE04E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C17238: move.b d6, d0
pub fn i_c17238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1723A;
    cpu.ir = 0x1006;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1723A: addq.l #$1, a0
pub fn i_c1723a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1723C;
    cpu.ir = 0x5288;
    cpu.a[0] = cpu.a[0].wrapping_add(1);
    8
}
/// $C1723C: not.b d0
pub fn i_c1723c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1723E;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1723E: move.b (a0), d1
pub fn i_c1723e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17240;
    cpu.ir = 0x1210;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C17240: move.b d0, (a0)
pub fn i_c17240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17242;
    cpu.ir = 0x1080;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[0], v1 as u8);
    8
}
/// $C17242: eor.b d0, d1
pub fn i_c17242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17244;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C17244: and.b d0, d1
pub fn i_c17244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17246;
    cpu.ir = 0xC200;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17246: move.b d1, $c(a0)
pub fn i_c17246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1724A;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1724A: rts
pub fn i_c1724a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1724C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1724C: move.b #$24, (a2)
pub fn i_c1724c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17250;
    cpu.ir = 0x14BC;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C17250: bsr.w $c17066
pub fn i_c17250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17254;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC17254);
    cpu.pc = 0xC17066;
    18
}
/// $C17254: move.b $340000.l, d6
pub fn i_c17254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1725A;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x340000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1725A: swap d6
pub fn i_c1725a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1725C;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1725C: move.b $300000.l, d6
pub fn i_c1725c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17262;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x300000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C17262: lsl.l #$8, d6
pub fn i_c17262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17264;
    cpu.ir = 0xE18E;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[6] = r;
    22
}
/// $C17264: move.b #$9, (a2)
pub fn i_c17264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17268;
    cpu.ir = 0x14BC;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[2], v1 as u8);
    8
}
/// $C17268: bsr.w $c17066
pub fn i_c17268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1726C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC1726C);
    cpu.pc = 0xC17066;
    18
}
/// $C1726C: move.b $300000.l, d6
pub fn i_c1726c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17272;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x300000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C17272: swap d6
pub fn i_c17272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17274;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17274: move.b $340000.l, d6
pub fn i_c17274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1727A;
    cpu.ir = 0x1C39;
    let v2 = bus.read8(0x340000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1727A: swap d6
pub fn i_c1727a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1727C;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1727C: rts
pub fn i_c1727c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1727E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1727E: move.b #$1, $10fe8c.l
pub fn i_c1727e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17286;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FE8C, v1 as u8);
    16
}
/// $C17286: tst.b $10fe8c.l
pub fn i_c17286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1728C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FE8C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C1728C: bne.b $c17286
pub fn i_c1728c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1728E;
    cpu.ir = 0x66F8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17286; return 10; }
    8
}
/// $C1728E: rts
pub fn i_c1728e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17290;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17290: tst.b $10fdc2.l
pub fn i_c17290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17296;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FDC2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C17296: bne.b $c172d0
pub fn i_c17296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17298;
    cpu.ir = 0x6638;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC172D0; return 10; }
    8
}
/// $C17298: addq.b #$1, $10fdc2.l
pub fn i_c17298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1729E;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x10FDC2) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10FDC2, r2 as u8);
    16
}
/// $C1729E: move.l #$10feca, $10fec6.l
pub fn i_c1729e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172A8;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x10FECA;
    bus.write32(0x10FEC6, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C172A8: lea.l $10ff00.l, a0
pub fn i_c172a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FF00;
    20
}
/// $C172AE: lea.l $3c0000.l, a6
pub fn i_c172ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172B4;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x3C0000;
    20
}
/// $C172B4: lea.l $2(a6), a4
pub fn i_c172b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172B8;
    cpu.ir = 0x49EE;
    let ad1 = cpu.a[6].wrapping_add(0x2);
    cpu.a[4] = ad1;
    16
}
/// $C172B8: cmpa.l $10fdbe.l, a0
pub fn i_c172b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172BE;
    cpu.ir = 0xB1F9;
    let v2 = bus.read32(0x10FDBE);
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    22
}
/// $C172BE: bcs.b $c172d2
pub fn i_c172be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172C0;
    cpu.ir = 0x6512;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC172D2; return 10; }
    8
}
/// $C172C0: move.l #$10ff00, $10fdbe.l
pub fn i_c172c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172CA;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x10FF00;
    bus.write32(0x10FDBE, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C172CA: clr.b $10fdc2.l
pub fn i_c172ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172D0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10FDC2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10FDC2, r2 as u8);
    16
}
/// $C172D0: rts
pub fn i_c172d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C172D2: movea.l (a0)+, a1
pub fn i_c172d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172D4;
    cpu.ir = 0x2258;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $C172D4: move.l a1, d4
pub fn i_c172d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172D6;
    cpu.ir = 0x2809;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C172D6: bne.b $c172da
pub fn i_c172d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172D8;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC172DA; return 10; }
    8
}
/// $C172D8: movea.l a0, a1
pub fn i_c172d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172DA;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C172DA: move.b (a1)+, d1
pub fn i_c172da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172DC;
    cpu.ir = 0x1219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C172DC: moveq #$0, d0
pub fn i_c172dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172DE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C172DE: move.b (a1)+, d0
pub fn i_c172de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172E0;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C172E0: cmpi.b #$e, d0
pub fn i_c172e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172E4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $C172E4: bcc.b $c172c0
pub fn i_c172e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172E6;
    cpu.ir = 0x64DA;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC172C0; return 10; }
    8
}
/// $C172E6: lsl.w #$2, d0
pub fn i_c172e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172E8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C172E8: movea.l $c172f0(pc, d0.w), a3
pub fn i_c172e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172EC;
    cpu.ir = 0x267B;
    let ad2 = 0xC172F0_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    18
}
/// $C172EC: jsr (a3)
pub fn i_c172ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172EE;
    cpu.ir = 0x4E93;
    cpu.push32(bus, 0xC172EE);
    cpu.jump_to(cpu.a[3]);
    16
}
/// $C172EE: bra.b $c172da
pub fn i_c172ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC172F0;
    cpu.ir = 0x60EA;
    cpu.pc = 0xC172DA;
    10
}
/// $C17328: or.l d4, d4
pub fn i_c17328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1732A;
    cpu.ir = 0x8884;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[4]) & 0xFFFFFFFF;
    cpu.d[4] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1732A: bne.b $c1732e
pub fn i_c1732a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1732C;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1732E; return 10; }
    8
}
/// $C1732C: movea.l a1, a0
pub fn i_c1732c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1732E;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1732E: move.l #$c172b8, (a7)
pub fn i_c1732e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17334;
    cpu.ir = 0x2EBC;
    let v1: u32 = 0xC172B8;
    bus.write32(cpu.a[7], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17334: rts
pub fn i_c17334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17336;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17336: swap d1
pub fn i_c17336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17338;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17338: move.l d1, d5
pub fn i_c17338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1733A;
    cpu.ir = 0x2A01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1733A: move.w (a1)+, d5
pub fn i_c1733a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1733C;
    cpu.ir = 0x3A19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C1733C: rts
pub fn i_c1733c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1733E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1733E: ext.w d1
pub fn i_c1733e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17340;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C17340: move.w d1, $2(a4)
pub fn i_c17340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17344;
    cpu.ir = 0x3941;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C17344: rts
pub fn i_c17344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17346;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17346: move.w (a1)+, d6
pub fn i_c17346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17348;
    cpu.ir = 0x3C19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C17348: move.w d6, (a6)
pub fn i_c17348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1734A;
    cpu.ir = 0x3C86;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[6], v1 as u16);
    8
}
/// $C1734A: rts
pub fn i_c1734a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1734C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1738C: movea.l (a1)+, a2
pub fn i_c1738c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1738E;
    cpu.ir = 0x2459;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    12
}
/// $C1738E: moveq #$1, d3
pub fn i_c1738e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17390;
    cpu.ir = 0x7601;
    cpu.d[3] = 0x1;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17390: bsr.w $c173aa
pub fn i_c17390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17394;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0xC17394);
    cpu.pc = 0xC173AA;
    18
}
/// $C17394: or.b d3, d3
pub fn i_c17394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17396;
    cpu.ir = 0x8603;
    let v1 = cpu.d[3] & 0xFF;
    let r = (v1 | cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C17396: bne.b $c173a2
pub fn i_c17396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17398;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC173A2; return 10; }
    8
}
/// $C17398: move.l a2, d0
pub fn i_c17398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1739A;
    cpu.ir = 0x200A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1739A: addq.l #$1, d0
pub fn i_c1739a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1739C;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $C1739C: andi.w #$fffe, d0
pub fn i_c1739c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173A0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFFE) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C173A0: movea.l d0, a1
pub fn i_c173a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173A2;
    cpu.ir = 0x2240;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C173A2: rts
pub fn i_c173a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C173A4: add.w (a1)+, d6
pub fn i_c173a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173A6;
    cpu.ir = 0xDC59;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $C173A6: move.w d6, (a6)
pub fn i_c173a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173A8;
    cpu.ir = 0x3C86;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[6], v1 as u16);
    8
}
/// $C173A8: rts
pub fn i_c173a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C173AA: btst.b #$11, d5
pub fn i_c173aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173AE;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((17) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C173AE: bne.b $c173ce
pub fn i_c173ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173B0;
    cpu.ir = 0x661E;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC173CE; return 10; }
    8
}
/// $C173B0: move.w d5, d2
pub fn i_c173b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173B2;
    cpu.ir = 0x3405;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C173B2: move.b d5, d7
pub fn i_c173b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173B4;
    cpu.ir = 0x1E05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C173B4: btst.b #$10, d5
pub fn i_c173b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173B8;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((16) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C173B8: bne.b $c173c4
pub fn i_c173b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173BA;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC173C4; return 10; }
    8
}
/// $C173BA: move.b (a2)+, d2
pub fn i_c173ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173BC;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C173BC: cmp.b d2, d7
pub fn i_c173bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173BE;
    cpu.ir = 0xBE02;
    let v1 = cpu.d[2] & 0xFF;
    let h1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C173BE: beq.b $c173cc
pub fn i_c173be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173C0;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC173CC; return 10; }
    8
}
/// $C173C0: move.w d2, (a4)
pub fn i_c173c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173C2;
    cpu.ir = 0x3882;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C173C2: bra.b $c173ba
pub fn i_c173c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173C4;
    cpu.ir = 0x60F6;
    cpu.pc = 0xC173BA;
    10
}
/// $C173C4: move.b (a2)+, d2
pub fn i_c173c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173C6;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C173C6: move.w d2, (a4)
pub fn i_c173c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173C8;
    cpu.ir = 0x3882;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C173C8: subq.b #$1, d7
pub fn i_c173c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173CA;
    cpu.ir = 0x5307;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C173CA: bne.b $c173c4
pub fn i_c173ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173CC;
    cpu.ir = 0x66F8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC173C4; return 10; }
    8
}
/// $C173CC: rts
pub fn i_c173cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C173CE: move.w d5, d7
pub fn i_c173ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173D0;
    cpu.ir = 0x3E05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C173D0: btst.b #$10, d5
pub fn i_c173d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173D4;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((16) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C173D4: bne.b $c173e0
pub fn i_c173d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173D6;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC173E0; return 10; }
    8
}
/// $C173D6: move.w (a2)+, d2
pub fn i_c173d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173D8;
    cpu.ir = 0x341A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C173D8: cmp.w d2, d5
pub fn i_c173d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173DA;
    cpu.ir = 0xBA42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $C173DA: beq.b $c173e6
pub fn i_c173da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173DC;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC173E6; return 10; }
    8
}
/// $C173DC: move.w d2, (a4)
pub fn i_c173dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173DE;
    cpu.ir = 0x3882;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C173DE: bra.b $c173d6
pub fn i_c173de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173E0;
    cpu.ir = 0x60F6;
    cpu.pc = 0xC173D6;
    10
}
/// $C173E0: move.w (a2)+, (a4)
pub fn i_c173e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173E2;
    cpu.ir = 0x389A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v3 as u16);
    12
}
/// $C173E2: subq.w #$1, d7
pub fn i_c173e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173E4;
    cpu.ir = 0x5347;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C173E4: bne.b $c173e0
pub fn i_c173e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173E6;
    cpu.ir = 0x66FA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC173E0; return 10; }
    8
}
/// $C173E6: rts
pub fn i_c173e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC173E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1744E: move.w #$20, $2(a4)
pub fn i_c1744e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17454;
    cpu.ir = 0x397C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C17454: move.b d1, d2
pub fn i_c17454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17456;
    cpu.ir = 0x1401;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C17456: lsl.w #$8, d2
pub fn i_c17456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17458;
    cpu.ir = 0xE14A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C17458: movea.l a1, a2
pub fn i_c17458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1745A;
    cpu.ir = 0x2449;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $C1745A: jsr $c17478(pc)
pub fn i_c1745a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1745E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1745E);
    cpu.pc = 0xC17478;
    16
}
/// $C1745E: addq.w #$1, d6
pub fn i_c1745e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17460;
    cpu.ir = 0x5246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C17460: move.w d6, (a6)
pub fn i_c17460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17462;
    cpu.ir = 0x3C86;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[6], v1 as u16);
    8
}
/// $C17462: movea.l a1, a2
pub fn i_c17462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17464;
    cpu.ir = 0x2449;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $C17464: addi.w #$100, d2
pub fn i_c17464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17468;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C17468: jsr $c17478(pc)
pub fn i_c17468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1746C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1746C);
    cpu.pc = 0xC17478;
    16
}
/// $C1746C: move.l a2, d0
pub fn i_c1746c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1746E;
    cpu.ir = 0x200A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1746E: addq.l #$1, d0
pub fn i_c1746e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17470;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $C17470: andi.w #$fffe, d0
pub fn i_c17470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17474;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFFE) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C17474: movea.l d0, a1
pub fn i_c17474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17476;
    cpu.ir = 0x2240;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C17476: rts
pub fn i_c17476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17478;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17478: move.b (a2)+, d2
pub fn i_c17478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1747A;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1747A: cmpi.b #$ff, d2
pub fn i_c1747a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1747E;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $C1747E: beq.b $c17484
pub fn i_c1747e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17480;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17484; return 10; }
    8
}
/// $C17480: move.w d2, (a4)
pub fn i_c17480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17482;
    cpu.ir = 0x3882;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $C17482: bra.b $c17478
pub fn i_c17482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17484;
    cpu.ir = 0x60F4;
    cpu.pc = 0xC17478;
    10
}
/// $C17484: rts
pub fn i_c17484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17486;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17AE6: lea.l $10f300.l, a5
pub fn i_c17ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AEC;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C17AEC: move.w #$2700, sr
pub fn i_c17aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AF0;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C17AF0: clr.w $a80(a5)
pub fn i_c17af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AF4;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xA80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C17AF4: jsr $c10fae(pc)
pub fn i_c17af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17AF8);
    cpu.pc = 0xC10FAE;
    16
}
/// $C17AF8: move.b d0, $3a0003.l
pub fn i_c17af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17AFE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C17AFE: move.b d0, $3a000b.l
pub fn i_c17afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B04;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C17B04: moveq #$0, d0
pub fn i_c17b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B06;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17B06: move.l d0, d1
pub fn i_c17b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B08;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17B08: move.l d0, d2
pub fn i_c17b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B0A;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17B0A: move.l d0, d3
pub fn i_c17b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B0C;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17B0C: lea.l $110000.l, a0
pub fn i_c17b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x110000;
    20
}
/// $C17B12: move.w #$1ff, d7
pub fn i_c17b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B16;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17B16: move.b d0, $300001.l
pub fn i_c17b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B1C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C17B1C: movem.l d0-d3, -(a0)
pub fn i_c17b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B20;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B20: movem.l d0-d3, -(a0)
pub fn i_c17b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B24;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B24: movem.l d0-d3, -(a0)
pub fn i_c17b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B28;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B28: movem.l d0-d3, -(a0)
pub fn i_c17b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B2C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B2C: movem.l d0-d3, -(a0)
pub fn i_c17b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B30;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B30: movem.l d0-d3, -(a0)
pub fn i_c17b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B34;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B34: movem.l d0-d3, -(a0)
pub fn i_c17b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B38;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B38: movem.l d0-d3, -(a0)
pub fn i_c17b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B3C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B3C: dbra d7, $c17b16
pub fn i_c17b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B40;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC17B40; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC17B40; return 14; }
    cpu.pc = 0xC17B16;
    10
}
/// $C17B40: move.b d0, $3a000f.l
pub fn i_c17b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B46;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $C17B46: lea.l $402000.l, a0
pub fn i_c17b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B4C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C17B4C: move.w #$7f, d7
pub fn i_c17b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B50;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17B50: movem.l d0-d3, -(a0)
pub fn i_c17b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B54;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B54: movem.l d0-d3, -(a0)
pub fn i_c17b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B58;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B58: movem.l d0-d3, -(a0)
pub fn i_c17b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B5C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B5C: movem.l d0-d3, -(a0)
pub fn i_c17b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B60;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B60: dbra d7, $c17b50
pub fn i_c17b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B64;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC17B64; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC17B64; return 14; }
    cpu.pc = 0xC17B50;
    10
}
/// $C17B64: move.b d0, $300001.l
pub fn i_c17b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B6A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C17B6A: move.b d0, $3a001f.l
pub fn i_c17b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B70;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C17B70: lea.l $402000.l, a0
pub fn i_c17b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C17B76: move.w #$7f, d7
pub fn i_c17b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B7A;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17B7A: movem.l d0-d3, -(a0)
pub fn i_c17b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B7E;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B7E: movem.l d0-d3, -(a0)
pub fn i_c17b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B82;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B82: movem.l d0-d3, -(a0)
pub fn i_c17b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B86;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B86: movem.l d0-d3, -(a0)
pub fn i_c17b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B8A;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C17B8A: dbra d7, $c17b7a
pub fn i_c17b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B8E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC17B8E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC17B8E; return 14; }
    cpu.pc = 0xC17B7A;
    10
}
/// $C17B8E: move.l #$eee0000, $400002.l
pub fn i_c17b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17B98;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C17B98: move.l #$e000000, $400022.l
pub fn i_c17b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BA2;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xE000000;
    bus.write32(0x400022, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C17BA2: move.l #$100f0000, $4001ea.l
pub fn i_c17ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BAC;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x100F0000;
    bus.write32(0x4001EA, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C17BAC: lea.l $10f300.l, a5
pub fn i_c17bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BB2;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C17BB2: move.b #$ff, $b8d(a5)
pub fn i_c17bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BB8;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xB8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17BB8: move.b #$1, $bbc(a5)
pub fn i_c17bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BBE;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17BBE: move.b $c00401.l, $a83(a5)
pub fn i_c17bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BC6;
    cpu.ir = 0x1B79;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xA83);
    bus.write8(ad3, v2 as u8);
    24
}
/// $C17BC6: move.b #$80, $10fd82.l
pub fn i_c17bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BCE;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD82, v1 as u8);
    16
}
/// $C17BCE: move.l #$10ff00, $abe(a5)
pub fn i_c17bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BD6;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17BD6: move.b d0, $3a0001.l
pub fn i_c17bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BDC;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0001, v1 as u8);
    16
}
/// $C17BDC: move.w #$7, $3c000c.l
pub fn i_c17bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BE4;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C17BE4: move.w #$2000, sr
pub fn i_c17be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BE8;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C17BE8: move.w #$0, $9d8(a5)
pub fn i_c17be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BEE;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C17BEE: jsr $c12a0e(pc)
pub fn i_c17bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17BF2);
    cpu.pc = 0xC12A0E;
    16
}
/// $C17BF2: jsr $c11c42(pc)
pub fn i_c17bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17BF6);
    cpu.pc = 0xC11C42;
    16
}
/// $C17BF6: addq.b #$1, $ac2(a5)
pub fn i_c17bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BFA;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17BFA: movea.l $abe(a5), a0
pub fn i_c17bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17BFE;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C17BFE: move.l #$c17d44, (a0)+
pub fn i_c17bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C04;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17D44;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17C04: move.l #$c17e20, (a0)+
pub fn i_c17c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C0A;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17E20;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17C0A: jsr $c17eaa(pc)
pub fn i_c17c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C0E);
    cpu.pc = 0xC17EAA;
    16
}
/// $C17C0E: nop
pub fn i_c17c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C10;
    cpu.ir = 0x4E71;
    4
}
/// $C17C10: clr.l (a0)+
pub fn i_c17c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C12;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C17C12: move.w #$7318, d2
pub fn i_c17c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C16;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7318;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17C16: move.b $ad2(a5), d0
pub fn i_c17c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C1A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C1A: jsr $c17d12(pc)
pub fn i_c17c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C1E);
    cpu.pc = 0xC17D12;
    16
}
/// $C17C1E: move.w #$7258, d2
pub fn i_c17c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C22;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17C22: move.b $ad3(a5), d0
pub fn i_c17c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C26;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C26: jsr $c17d12(pc)
pub fn i_c17c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C2A);
    cpu.pc = 0xC17D12;
    16
}
/// $C17C2A: move.w #$72b8, d2
pub fn i_c17c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C2E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x72B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17C2E: move.b $ad4(a5), d0
pub fn i_c17c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C32;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C32: jsr $c17d12(pc)
pub fn i_c17c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C36);
    cpu.pc = 0xC17D12;
    16
}
/// $C17C36: move.w #$3, (a0)+
pub fn i_c17c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C3A;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17C3A: move.w #$7398, (a0)+
pub fn i_c17c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C3E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7398;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17C3E: jsr $c17e7a(pc)
pub fn i_c17c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C42);
    cpu.pc = 0xC17E7A;
    16
}
/// $C17C42: clr.w (a0)+
pub fn i_c17c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C44;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17C44: move.l a0, $abe(a5)
pub fn i_c17c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C48;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17C48: subq.b #$1, $ac2(a5)
pub fn i_c17c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C4C;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17C4C: move.w $9d8(a5), $9da(a5)
pub fn i_c17c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C52;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9DA);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C17C52: jsr $c17cca(pc)
pub fn i_c17c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C56);
    cpu.pc = 0xC17CCA;
    16
}
/// $C17C56: jsr $c11c42.l
pub fn i_c17c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC17C5C);
    cpu.pc = 0xC11C42;
    16
}
/// $C17C5C: addq.b #$1, $ac2(a5)
pub fn i_c17c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C60;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17C60: movea.l $abe(a5), a0
pub fn i_c17c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C64;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C17C64: clr.l (a0)+
pub fn i_c17c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C66;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C17C66: move.w #$725a, d2
pub fn i_c17c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C6A;
    cpu.ir = 0x343C;
    let v1: u32 = 0x725A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17C6A: move.b $ad6(a5), d0
pub fn i_c17c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C6E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C6E: jsr $c17d12(pc)
pub fn i_c17c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C72);
    cpu.pc = 0xC17D12;
    16
}
/// $C17C72: move.w #$72ba, d2
pub fn i_c17c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C76;
    cpu.ir = 0x343C;
    let v1: u32 = 0x72BA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17C76: move.b $ad7(a5), d0
pub fn i_c17c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C7A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C7A: jsr $c17d12(pc)
pub fn i_c17c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C7E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C7E);
    cpu.pc = 0xC17D12;
    16
}
/// $C17C7E: move.w #$731a, d2
pub fn i_c17c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C82;
    cpu.ir = 0x343C;
    let v1: u32 = 0x731A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17C82: move.b $ad8(a5), d0
pub fn i_c17c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C86;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C86: jsr $c17d12(pc)
pub fn i_c17c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C8A);
    cpu.pc = 0xC17D12;
    16
}
/// $C17C8A: clr.w (a0)+
pub fn i_c17c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C8C;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17C8C: move.l a0, $abe(a5)
pub fn i_c17c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C90;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17C90: subq.b #$1, $ac2(a5)
pub fn i_c17c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C94;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17C94: jsr $c1727e(pc)
pub fn i_c17c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17C98);
    cpu.pc = 0xC1727E;
    16
}
/// $C17C98: move.b $a97(a5), d0
pub fn i_c17c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17C9C;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17C9C: btst.b #$4, d0
pub fn i_c17c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CA0;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C17CA0: bne.w $c17fca
pub fn i_c17ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17FCA; return 10; }
    8
}
/// $C17CA4: andi.b #$3, d0
pub fn i_c17ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CA8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17CA8: beq.b $c17c56
pub fn i_c17ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CAA;
    cpu.ir = 0x67AC;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17C56; return 10; }
    8
}
/// $C17CAA: moveq #$1, d1
pub fn i_c17caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CAC;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17CAC: andi.b #$2, d0
pub fn i_c17cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CB0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17CB0: bne.b $c17cb4
pub fn i_c17cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CB2;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC17CB4; return 10; }
    8
}
/// $C17CB2: moveq #$ff, d1
pub fn i_c17cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CB4;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17CB4: add.w $9d8(a5), d1
pub fn i_c17cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CB8;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C17CB8: bmi.b $c17c56
pub fn i_c17cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CBA;
    cpu.ir = 0x6B9C;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC17C56; return 10; }
    8
}
/// $C17CBA: cmpi.w #$6, d1
pub fn i_c17cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CBE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $C17CBE: bcc.b $c17c56
pub fn i_c17cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CC0;
    cpu.ir = 0x6496;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC17C56; return 10; }
    8
}
/// $C17CC0: move.w d1, $9da(a5)
pub fn i_c17cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CC4;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C17CC4: jsr $c17cca(pc)
pub fn i_c17cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17CC8);
    cpu.pc = 0xC17CCA;
    16
}
/// $C17CC8: bra.b $c17c56
pub fn i_c17cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CCA;
    cpu.ir = 0x608C;
    cpu.pc = 0xC17C56;
    10
}
/// $C17CCA: addq.b #$1, $ac2(a5)
pub fn i_c17cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CCE;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17CCE: movea.l $abe(a5), a0
pub fn i_c17cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CD2;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C17CD2: clr.l (a0)+
pub fn i_c17cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CD4;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C17CD4: move.w #$20ff, d1
pub fn i_c17cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CD8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17CD8: jsr $c17cf6(pc)
pub fn i_c17cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17CDC);
    cpu.pc = 0xC17CF6;
    16
}
/// $C17CDC: move.w $9da(a5), $9d8(a5)
pub fn i_c17cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CE2;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9DA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C17CE2: move.w #$11ff, d1
pub fn i_c17ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CE6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17CE6: jsr $c17cf6(pc)
pub fn i_c17ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC17CEA);
    cpu.pc = 0xC17CF6;
    16
}
/// $C17CEA: clr.w (a0)+
pub fn i_c17cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CEC;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C17CEC: move.l a0, $abe(a5)
pub fn i_c17cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CF0;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17CF0: subq.b #$1, $ac2(a5)
pub fn i_c17cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CF4;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17CF4: rts
pub fn i_c17cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17CF6: move.w #$3, (a0)+
pub fn i_c17cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CFA;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17CFA: move.w $9d8(a5), d0
pub fn i_c17cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17CFE;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C17CFE: move.w d0, d2
pub fn i_c17cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D00;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C17D00: add.w d0, d0
pub fn i_c17d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D02;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C17D02: add.w d0, d2
pub fn i_c17d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D04;
    cpu.ir = 0xD440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C17D04: addi.w #$70e5, d2
pub fn i_c17d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D08;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x70E5, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C17D08: move.w d2, (a0)+
pub fn i_c17d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D0A;
    cpu.ir = 0x30C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D0A: move.w #$1108, (a0)+
pub fn i_c17d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D0E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D0E: move.w d1, (a0)+
pub fn i_c17d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D10;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D10: rts
pub fn i_c17d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17D12: move.w #$3, (a0)+
pub fn i_c17d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D16;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D16: move.w d2, (a0)+
pub fn i_c17d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D18;
    cpu.ir = 0x30C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D18: move.w #$108, (a0)+
pub fn i_c17d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D1C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D1C: move.b #$20, (a0)+
pub fn i_c17d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D20;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C17D20: move.b d0, d1
pub fn i_c17d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D22;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C17D22: lsr.b #$4, d0
pub fn i_c17d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D24;
    cpu.ir = 0xE808;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C17D24: andi.b #$f, d0
pub fn i_c17d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D28;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17D28: beq.b $c17d32
pub fn i_c17d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D2A;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC17D32; return 10; }
    8
}
/// $C17D2A: addi.b #$30, d0
pub fn i_c17d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D2E;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17D2E: move.b d0, -$1(a0)
pub fn i_c17d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D32;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17D32: andi.b #$f, d1
pub fn i_c17d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D36;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17D36: addi.b #$30, d1
pub fn i_c17d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D3A;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C17D3A: move.b d1, (a0)+
pub fn i_c17d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D3C;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C17D3C: move.w #$ff00, (a0)+
pub fn i_c17d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D40;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17D40: addq.w #$1, d6
pub fn i_c17d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D42;
    cpu.ir = 0x5246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C17D42: rts
pub fn i_c17d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17D44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17E7A: move.w #$108, (a0)+
pub fn i_c17e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E7E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C17E7E: moveq #$0, d0
pub fn i_c17e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E80;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C17E80: move.b $ad5(a5), d0
pub fn i_c17e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E84;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAD5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C17E84: add.w d0, d0
pub fn i_c17e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E86;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C17E86: add.w d0, d0
pub fn i_c17e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E88;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C17E88: move.l $c17e8e(pc, d0.w), (a0)+
pub fn i_c17e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E8C;
    cpu.ir = 0x20FB;
    let ad2 = 0xC17E8E_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C17E8C: rts
pub fn i_c17e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17E8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17EAA: move.l #$c17dec, (a0)+
pub fn i_c17eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EB0;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17DEC;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17EB0: rts
pub fn i_c17eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17EB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17F40: move.l a0, -(a7)
pub fn i_c17f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F42;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C17F42: subq.b #$1, $ae5(a5)
pub fn i_c17f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F46;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAE5);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17F46: bcc.w $c17f7e
pub fn i_c17f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC17F7E; return 10; }
    8
}
/// $C17F4A: addq.b #$1, $ac2(a5)
pub fn i_c17f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F4E;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17F4E: movea.l $abe(a5), a0
pub fn i_c17f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F52;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C17F52: not.b $ae4(a5)
pub fn i_c17f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F56;
    cpu.ir = 0x462D;
    let ad1 = cpu.a[5].wrapping_add(0xAE4);
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17F56: bpl.w $c17f6a
pub fn i_c17f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F5A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC17F6A; return 10; }
    8
}
/// $C17F5A: move.b #$32, $ae5(a5)
pub fn i_c17f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F60;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAE5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17F60: move.l #$c17f82, (a0)+
pub fn i_c17f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F66;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17F82;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17F66: bra.w $c17f76
pub fn i_c17f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F6A;
    cpu.ir = 0x6000;
    cpu.pc = 0xC17F76;
    10
}
/// $C17F6A: move.b #$a, $ae5(a5)
pub fn i_c17f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F70;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAE5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C17F70: move.l #$c17fa6, (a0)+
pub fn i_c17f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F76;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17FA6;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C17F76: move.l a0, $abe(a5)
pub fn i_c17f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F7A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C17F7A: subq.b #$1, $ac2(a5)
pub fn i_c17f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F7E;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C17F7E: movea.l (a7)+, a0
pub fn i_c17f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F80;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $C17F80: rts
pub fn i_c17f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17F82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C17FCA: pea.l $c17bee(pc)
pub fn i_c17fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FCE;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC17BEE);
    24
}
/// $C17FCE: move.w $9d8(a5), d0
pub fn i_c17fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FD2;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C17FD2: add.w d0, d0
pub fn i_c17fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FD4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C17FD4: add.w d0, d0
pub fn i_c17fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FD6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C17FD6: movea.l $c17fdc(pc, d0.w), a0
pub fn i_c17fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FDA;
    cpu.ir = 0x207B;
    let ad2 = 0xC17FDC_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C17FDA: jmp (a0)
pub fn i_c17fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC17FDC;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C1A328: move.b d0, $3a0003.l
pub fn i_c1a328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A32E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0003, v1 as u8);
    16
}
/// $C1A32E: move.b d0, $3a000b.l
pub fn i_c1a32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A334;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C1A334: moveq #$0, d0
pub fn i_c1a334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A336;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A336: move.l d0, d1
pub fn i_c1a336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A338;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A338: move.l d0, d2
pub fn i_c1a338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A33A;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A33A: move.l d0, d3
pub fn i_c1a33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A33C;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A33C: lea.l $110000.l, a0
pub fn i_c1a33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A342;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x110000;
    20
}
/// $C1A342: move.w #$1ff, d7
pub fn i_c1a342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A346;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A346: move.b d0, $300001.l
pub fn i_c1a346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A34C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1A34C: movem.l d0-d3, -(a0)
pub fn i_c1a34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A350;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A350: movem.l d0-d3, -(a0)
pub fn i_c1a350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A354;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A354: movem.l d0-d3, -(a0)
pub fn i_c1a354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A358;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A358: movem.l d0-d3, -(a0)
pub fn i_c1a358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A35C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A35C: movem.l d0-d3, -(a0)
pub fn i_c1a35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A360;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A360: movem.l d0-d3, -(a0)
pub fn i_c1a360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A364;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A364: movem.l d0-d3, -(a0)
pub fn i_c1a364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A368;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A368: movem.l d0-d3, -(a0)
pub fn i_c1a368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A36C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A36C: dbra d7, $c1a346
pub fn i_c1a36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A370;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1A370; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1A370; return 14; }
    cpu.pc = 0xC1A346;
    10
}
/// $C1A370: move.b d0, $3a000f.l
pub fn i_c1a370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A376;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $C1A376: lea.l $402000.l, a0
pub fn i_c1a376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A37C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C1A37C: move.w #$7f, d7
pub fn i_c1a37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A380;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A380: movem.l d0-d3, -(a0)
pub fn i_c1a380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A384;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A384: movem.l d0-d3, -(a0)
pub fn i_c1a384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A388;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A388: movem.l d0-d3, -(a0)
pub fn i_c1a388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A38C;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A38C: movem.l d0-d3, -(a0)
pub fn i_c1a38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A390;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A390: dbra d7, $c1a380
pub fn i_c1a390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A394;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1A394; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1A394; return 14; }
    cpu.pc = 0xC1A380;
    10
}
/// $C1A394: move.b d0, $300001.l
pub fn i_c1a394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A39A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1A39A: move.b d0, $3a001f.l
pub fn i_c1a39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3A0;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C1A3A0: lea.l $402000.l, a0
pub fn i_c1a3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x402000;
    20
}
/// $C1A3A6: move.w #$7f, d7
pub fn i_c1a3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3AA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A3AA: movem.l d0-d3, -(a0)
pub fn i_c1a3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3AE;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A3AE: movem.l d0-d3, -(a0)
pub fn i_c1a3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3B2;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A3B2: movem.l d0-d3, -(a0)
pub fn i_c1a3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3B6;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A3B6: movem.l d0-d3, -(a0)
pub fn i_c1a3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3BA;
    cpu.ir = 0x48E0;
    let mut a = cpu.a[0];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[0] = a;
    20
}
/// $C1A3BA: dbra d7, $c1a3aa
pub fn i_c1a3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3BE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1A3BE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1A3BE; return 14; }
    cpu.pc = 0xC1A3AA;
    10
}
/// $C1A3BE: move.l #$eee0000, $400002.l
pub fn i_c1a3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3C8;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x400002, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1A3C8: move.l #$e000000, $400022.l
pub fn i_c1a3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3D2;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xE000000;
    bus.write32(0x400022, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1A3D2: lea.l $10f300.l, a5
pub fn i_c1a3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3D8;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C1A3D8: jsr $c16fd4.l
pub fn i_c1a3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A3DE);
    cpu.pc = 0xC16FD4;
    16
}
/// $C1A3DE: move.b #$1, $be6(a5)
pub fn i_c1a3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3E4;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBE6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1A3E4: move.b #$ff, $b8d(a5)
pub fn i_c1a3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3EA;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xB8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1A3EA: move.b #$1, $bbc(a5)
pub fn i_c1a3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3F0;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBC);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1A3F0: move.b $c00401.l, $a83(a5)
pub fn i_c1a3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A3F8;
    cpu.ir = 0x1B79;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xA83);
    bus.write8(ad3, v2 as u8);
    24
}
/// $C1A3F8: move.b #$80, $10fd82.l
pub fn i_c1a3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A400;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD82, v1 as u8);
    16
}
/// $C1A400: move.l #$10ff00, $abe(a5)
pub fn i_c1a400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A408;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FF00;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A408: move.b d0, $3a0001.l
pub fn i_c1a408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A40E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0001, v1 as u8);
    16
}
/// $C1A40E: move.w #$7, $3c000c.l
pub fn i_c1a40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A416;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C1A416: move.w #$2000, sr
pub fn i_c1a416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A41A;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C1A41A: move.w #$0, $9d8(a5)
pub fn i_c1a41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A420;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1A420: jsr $c12a0e.l
pub fn i_c1a420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A426;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A426);
    cpu.pc = 0xC12A0E;
    16
}
/// $C1A426: addq.b #$1, $ac2(a5)
pub fn i_c1a426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A42A;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A42A: movea.l $abe(a5), a0
pub fn i_c1a42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A42E;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A42E: move.l #$c1a504, (a0)+
pub fn i_c1a42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A434;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1A504;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A434: move.l a0, $abe(a5)
pub fn i_c1a434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A438;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A438: subq.b #$1, $ac2(a5)
pub fn i_c1a438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A43C;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A43C: jsr $c1727e.l
pub fn i_c1a43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A442);
    cpu.pc = 0xC1727E;
    16
}
/// $C1A442: jsr $c1a536(pc)
pub fn i_c1a442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A446;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A446);
    cpu.pc = 0xC1A536;
    16
}
/// $C1A446: tst.b d5
pub fn i_c1a446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A448;
    cpu.ir = 0x4A05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1A448: beq.b $c1a43c
pub fn i_c1a448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A44A;
    cpu.ir = 0x67F2;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A43C; return 10; }
    8
}
/// $C1A44A: clr.w $9d8(a5)
pub fn i_c1a44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A44E;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x9D8);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1A44E: jsr $c12a0e.l
pub fn i_c1a44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A454);
    cpu.pc = 0xC12A0E;
    16
}
/// $C1A454: jsr $c1727e.l
pub fn i_c1a454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A45A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A45A);
    cpu.pc = 0xC1727E;
    16
}
/// $C1A45A: jsr $c1a4a8(pc)
pub fn i_c1a45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A45E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A45E);
    cpu.pc = 0xC1A4A8;
    16
}
/// $C1A45E: jsr $c1a536(pc)
pub fn i_c1a45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A462;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A462);
    cpu.pc = 0xC1A536;
    16
}
/// $C1A462: tst.b d5
pub fn i_c1a462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A464;
    cpu.ir = 0x4A05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C1A464: beq.b $c1a420
pub fn i_c1a464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A466;
    cpu.ir = 0x67BA;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A420; return 10; }
    8
}
/// $C1A466: move.b $bea(a5), d0
pub fn i_c1a466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A46A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xBEA);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A46A: andi.b #$f0, d0
pub fn i_c1a46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A46E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A46E: cmpi.b #$f0, d0
pub fn i_c1a46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A472;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xF0, Size::Byte, false);
    8
}
/// $C1A472: beq.w $c1a5e2
pub fn i_c1a472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A476;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A5E2; return 10; }
    8
}
/// $C1A476: move.b $beb(a5), d0
pub fn i_c1a476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A47A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xBEB);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A47A: andi.b #$3, d0
pub fn i_c1a47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A47E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A47E: beq.b $c1a454
pub fn i_c1a47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A480;
    cpu.ir = 0x67D4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A454; return 10; }
    8
}
/// $C1A480: lea.l $800000.l, a6
pub fn i_c1a480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A486;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x800000;
    20
}
/// $C1A486: move.w $9d8(a5), d2
pub fn i_c1a486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A48A;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1A48A: moveq #$1, d1
pub fn i_c1a48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A48C;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A48C: andi.b #$1, d0
pub fn i_c1a48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A490;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A490: bne.b $c1a494
pub fn i_c1a490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A492;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A494; return 10; }
    8
}
/// $C1A492: moveq #$ff, d1
pub fn i_c1a492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A494;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A494: add.b d1, d2
pub fn i_c1a494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A496;
    cpu.ir = 0xD401;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    4
}
/// $C1A496: cmp.w $74(a6), d2
pub fn i_c1a496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A49A;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C1A49A: bcc.b $c1a494
pub fn i_c1a49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A49C;
    cpu.ir = 0x64F8;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1A494; return 10; }
    8
}
/// $C1A49C: move.w d2, $9d8(a5)
pub fn i_c1a49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4A0;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1A4A0: jsr $c12a0e.l
pub fn i_c1a4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A4A6);
    cpu.pc = 0xC12A0E;
    16
}
/// $C1A4A6: bra.b $c1a454
pub fn i_c1a4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4A8;
    cpu.ir = 0x60AC;
    cpu.pc = 0xC1A454;
    10
}
/// $C1A4A8: addq.b #$1, $ac2(a5)
pub fn i_c1a4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4AC;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A4AC: movea.l $abe(a5), a0
pub fn i_c1a4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4B0;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A4B0: move.l #$c1a522, (a0)+
pub fn i_c1a4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4B6;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1A522;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A4B6: clr.l (a0)+
pub fn i_c1a4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4B8;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C1A4B8: move.w #$3, (a0)+
pub fn i_c1a4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4BC;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A4BC: move.w #$72c6, (a0)+
pub fn i_c1a4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4C0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x72C6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A4C0: move.w #$108, (a0)+
pub fn i_c1a4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4C4;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A4C4: move.b $9d9(a5), d0
pub fn i_c1a4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4C8;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0x9D9);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A4C8: ori.b #$30, d0
pub fn i_c1a4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4CC;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A4CC: move.b d0, (a0)+
pub fn i_c1a4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4CE;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A4CE: move.b #$ff, (a0)+
pub fn i_c1a4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4D2;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A4D2: clr.w (a0)+
pub fn i_c1a4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4D4;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1A4D4: move.l a0, $abe(a5)
pub fn i_c1a4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4D8;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A4D8: subq.b #$1, $ac2(a5)
pub fn i_c1a4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4DC;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A4DC: jsr $c17290.l
pub fn i_c1a4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A4E2);
    cpu.pc = 0xC17290;
    16
}
/// $C1A4E2: addq.b #$1, $ac2(a5)
pub fn i_c1a4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4E6;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A4E6: movea.l $abe(a5), a0
pub fn i_c1a4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4EA;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A4EA: clr.l (a0)+
pub fn i_c1a4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4EC;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C1A4EC: jsr $c1a57a(pc)
pub fn i_c1a4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A4F0);
    cpu.pc = 0xC1A57A;
    16
}
/// $C1A4F0: clr.w (a0)+
pub fn i_c1a4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4F2;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1A4F2: move.l a0, $abe(a5)
pub fn i_c1a4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4F6;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A4F6: subq.b #$1, $ac2(a5)
pub fn i_c1a4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4FA;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A4FA: rts
pub fn i_c1a4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A4FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A536: moveq #$0, d5
pub fn i_c1a536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A538;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A538: move.b $380000.l, d0
pub fn i_c1a538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A53E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C1A53E: andi.b #$30, d0
pub fn i_c1a53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A542;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A542: beq.b $c1a546
pub fn i_c1a542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A544;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A546; return 10; }
    8
}
/// $C1A544: rts
pub fn i_c1a544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A546;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A546: lea.l $800000.l, a6
pub fn i_c1a546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A54C;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x800000;
    20
}
/// $C1A54C: lea.l $40(a6), a0
pub fn i_c1a54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A550;
    cpu.ir = 0x41EE;
    let ad1 = cpu.a[6].wrapping_add(0x40);
    cpu.a[0] = ad1;
    16
}
/// $C1A550: lea.l $c1a4fc(pc), a1
pub fn i_c1a550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A554;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC1A4FC;
    16
}
/// $C1A554: moveq #$7, d7
pub fn i_c1a554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A556;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A556: move.w (a0), d0
pub fn i_c1a556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A558;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C1A558: cmp.b (a1)+, d0
pub fn i_c1a558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A55A;
    cpu.ir = 0xB019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    8
}
/// $C1A55A: bne.b $c1a544
pub fn i_c1a55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A55C;
    cpu.ir = 0x66E8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A544; return 10; }
    8
}
/// $C1A55C: addq.l #$4, a0
pub fn i_c1a55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A55E;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $C1A55E: dbra d7, $c1a556
pub fn i_c1a55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A562;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1A562; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1A562; return 14; }
    cpu.pc = 0xC1A556;
    10
}
/// $C1A562: move.w $10(a6), d0
pub fn i_c1a562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A566;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1A566: andi.b #$f0, d0
pub fn i_c1a566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A56A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A56A: cmpi.b #$20, d0
pub fn i_c1a56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A56E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $C1A56E: bne.b $c1a544
pub fn i_c1a56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A570;
    cpu.ir = 0x66D4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A544; return 10; }
    8
}
/// $C1A570: tst.w $74(a6)
pub fn i_c1a570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A574;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $C1A574: beq.b $c1a544
pub fn i_c1a574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A576;
    cpu.ir = 0x67CE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A544; return 10; }
    8
}
/// $C1A576: moveq #$1, d5
pub fn i_c1a576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A578;
    cpu.ir = 0x7A01;
    cpu.d[5] = 0x1;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A578: rts
pub fn i_c1a578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A57A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A57A: lea.l $800100.l, a6
pub fn i_c1a57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A580;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x800100;
    20
}
/// $C1A580: move.w $9d8(a5), d0
pub fn i_c1a580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A584;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1A584: move.w #$2800, d1
pub fn i_c1a584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A588;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A588: mulu.w d0, d1
pub fn i_c1a588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A58A;
    cpu.ir = 0xC2C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C1A58A: adda.l d1, a6
pub fn i_c1a58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A58C;
    cpu.ir = 0xDDC1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[6] = cpu.a[6].wrapping_add(v1);
    8
}
/// $C1A58C: move.l #$370ca, (a0)+
pub fn i_c1a58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A592;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x370CA;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A592: moveq #$0, d5
pub fn i_c1a592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A594;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A594: tst.w $12(a6, d5.w)
pub fn i_c1a594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A598;
    cpu.ir = 0x4A76;
    let ad2 = cpu.a[6].wrapping_add(0x12).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    14
}
/// $C1A598: beq.w $c1a5d8
pub fn i_c1a598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A59C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A5D8; return 10; }
    8
}
/// $C1A59C: move.l #$50001, (a0)+
pub fn i_c1a59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5A2;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x50001;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A5A2: move.w #$108, (a0)+
pub fn i_c1a5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5A6;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A5A6: move.l #$534c4f54, (a0)+
pub fn i_c1a5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5AC;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x534C4F54;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A5AC: move.w d5, d0
pub fn i_c1a5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5AE;
    cpu.ir = 0x3005;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A5AE: lsr.w #$2, d0
pub fn i_c1a5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5B0;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C1A5B0: addi.b #$31, d0
pub fn i_c1a5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5B4;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x31, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A5B4: move.b d0, (a0)+
pub fn i_c1a5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5B6;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A5B6: move.b #$20, (a0)+
pub fn i_c1a5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5BA;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A5BA: move.w $14(a6, d5.w), d0
pub fn i_c1a5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5BE;
    cpu.ir = 0x3036;
    let ad2 = cpu.a[6].wrapping_add(0x14).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1A5BE: andi.w #$ff, d0
pub fn i_c1a5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5C2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1A5C2: lsl.w #$4, d0
pub fn i_c1a5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5C4;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C1A5C4: addi.w #$f2, d0
pub fn i_c1a5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5C8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1A5C8: lea.l (a6, d0.w), a1
pub fn i_c1a5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5CC;
    cpu.ir = 0x43F6;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $C1A5CC: move.l (a1)+, (a0)+
pub fn i_c1a5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5CE;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1A5CE: move.l (a1)+, (a0)+
pub fn i_c1a5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5D0;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1A5D0: move.l (a1)+, (a0)+
pub fn i_c1a5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5D2;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1A5D2: move.l (a1)+, (a0)+
pub fn i_c1a5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5D4;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1A5D4: move.w #$ff00, (a0)+
pub fn i_c1a5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5D8;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A5D8: addq.b #$4, d5
pub fn i_c1a5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5DA;
    cpu.ir = 0x5805;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1A5DA: cmpi.w #$20, d5
pub fn i_c1a5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5DE;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $C1A5DE: bne.b $c1a594
pub fn i_c1a5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5E0;
    cpu.ir = 0x66B4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A594; return 10; }
    8
}
/// $C1A5E0: rts
pub fn i_c1a5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A5E2: lea.l $d08000.l, a4
pub fn i_c1a5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5E8;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C1A5E8: move.w #$2700, sr
pub fn i_c1a5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5EC;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C1A5EC: move.b d0, $3a001d.l
pub fn i_c1a5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5F2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C1A5F2: move.w $9d8(a5), d0
pub fn i_c1a5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5F6;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1A5F6: move.w #$2800, d1
pub fn i_c1a5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5FA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A5FA: mulu.w d0, d1
pub fn i_c1a5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A5FC;
    cpu.ir = 0xC2C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C1A5FC: lea.l $800100.l, a0
pub fn i_c1a5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A602;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x800100;
    20
}
/// $C1A602: adda.l d1, a0
pub fn i_c1a602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A604;
    cpu.ir = 0xD1C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $C1A604: lea.l $300001.l, a3
pub fn i_c1a604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A60A;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x300001;
    20
}
/// $C1A60A: move.l $800070.l, -$7e54(a4)
pub fn i_c1a60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A612;
    cpu.ir = 0x2979;
    let v2 = bus.read32(0x800070);
    let ad3 = cpu.a[4].wrapping_add(0xFFFF81AC);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C1A612: move.w $9d8(a5), -$7e50(a4)
pub fn i_c1a612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A618;
    cpu.ir = 0x396D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[4].wrapping_add(0xFFFF81B0);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1A618: move.l (a0)+, -$7efc(a4)
pub fn i_c1a618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A61C;
    cpu.ir = 0x2958;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[4].wrapping_add(0xFFFF8104);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C1A61C: move.l (a0)+, -$7ef8(a4)
pub fn i_c1a61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A620;
    cpu.ir = 0x2958;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[4].wrapping_add(0xFFFF8108);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C1A620: move.l #$25, d7
pub fn i_c1a620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A626;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x25;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A626: lea.l -$7ee6(a4), a1
pub fn i_c1a626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A62A;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF811A);
    cpu.a[1] = ad1;
    16
}
/// $C1A62A: jsr $c1a688(pc)
pub fn i_c1a62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A62E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A62E);
    cpu.pc = 0xC1A688;
    16
}
/// $C1A62E: move.l #$90, d7
pub fn i_c1a62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A634;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x90;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A634: lea.l -$7e00(a4), a1
pub fn i_c1a634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A638;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8200);
    cpu.a[1] = ad1;
    16
}
/// $C1A638: jsr $c1a688(pc)
pub fn i_c1a638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A63C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A63C);
    cpu.pc = 0xC1A688;
    16
}
/// $C1A63C: move.l #$10f0, d7
pub fn i_c1a63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A642;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x10F0;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A642: lea.l $320(a4), a1
pub fn i_c1a642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A646;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0x320);
    cpu.a[1] = ad1;
    16
}
/// $C1A646: jsr $c1a688(pc)
pub fn i_c1a646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A64A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A64A);
    cpu.pc = 0xC1A688;
    16
}
/// $C1A64A: lea.l -$7edc(a4), a0
pub fn i_c1a64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A64E;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[0] = ad1;
    16
}
/// $C1A64E: moveq #$0, d0
pub fn i_c1a64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A650;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A650: tst.w (a0, d0.w)
pub fn i_c1a650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A654;
    cpu.ir = 0x4A70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    14
}
/// $C1A654: beq.w $c1a65e
pub fn i_c1a654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A658;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A65E; return 10; }
    8
}
/// $C1A658: move.b #$80, $2(a0, d0.w)
pub fn i_c1a658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A65E;
    cpu.ir = 0x11BC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C1A65E: addq.w #$4, d0
pub fn i_c1a65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A660;
    cpu.ir = 0x5840;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1A660: cmpi.w #$20, d0
pub fn i_c1a660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A664;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $C1A664: bne.b $c1a650
pub fn i_c1a664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A666;
    cpu.ir = 0x66EA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A650; return 10; }
    8
}
/// $C1A666: move.b d0, $3a000d.l
pub fn i_c1a666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A66C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000D, v1 as u8);
    16
}
/// $C1A66C: movea.l $abe(a5), a0
pub fn i_c1a66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A670;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A670: move.l #$c1a692, (a0)+
pub fn i_c1a670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A676;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1A692;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A676: move.l a0, $abe(a5)
pub fn i_c1a676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A67A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A67A: jsr $c17290.l
pub fn i_c1a67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A680;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC1A680);
    cpu.pc = 0xC17290;
    16
}
/// $C1A680: move.b d0, $300001.l
pub fn i_c1a680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A686;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C1A686: bra.b $c1a680
pub fn i_c1a686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A688;
    cpu.ir = 0x60F8;
    cpu.pc = 0xC1A680;
    10
}
/// $C1A688: move.b d0, (a3)
pub fn i_c1a688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A68A;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C1A68A: move.w (a0)+, (a1)+
pub fn i_c1a68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A68C;
    cpu.ir = 0x32D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $C1A68C: subq.l #$1, d7
pub fn i_c1a68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A68E;
    cpu.ir = 0x5387;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[7] = r2;
    4
}
/// $C1A68E: bne.b $c1a688
pub fn i_c1a68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A690;
    cpu.ir = 0x66F8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A688; return 10; }
    8
}
/// $C1A690: rts
pub fn i_c1a690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A692;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0xC16DCA, i_c16dca),
    (0xC16DCC, i_c16dcc),
    (0xC16DCE, i_c16dce),
    (0xC16DD2, i_c16dd2),
    (0xC16DD6, i_c16dd6),
    (0xC16DDA, i_c16dda),
    (0xC16DDE, i_c16dde),
    (0xC16DE4, i_c16de4),
    (0xC16DE8, i_c16de8),
    (0xC16DEC, i_c16dec),
    (0xC16DF2, i_c16df2),
    (0xC16DF8, i_c16df8),
    (0xC16DFA, i_c16dfa),
    (0xC16E04, i_c16e04),
    (0xC16E08, i_c16e08),
    (0xC16E0C, i_c16e0c),
    (0xC16E10, i_c16e10),
    (0xC16E16, i_c16e16),
    (0xC16E20, i_c16e20),
    (0xC16E26, i_c16e26),
    (0xC16E2A, i_c16e2a),
    (0xC16E2E, i_c16e2e),
    (0xC16E32, i_c16e32),
    (0xC16E38, i_c16e38),
    (0xC16E3C, i_c16e3c),
    (0xC16E40, i_c16e40),
    (0xC16E44, i_c16e44),
    (0xC16E4C, i_c16e4c),
    (0xC16E52, i_c16e52),
    (0xC16E58, i_c16e58),
    (0xC16E5C, i_c16e5c),
    (0xC16E62, i_c16e62),
    (0xC16E66, i_c16e66),
    (0xC16E6C, i_c16e6c),
    (0xC16E6E, i_c16e6e),
    (0xC16E72, i_c16e72),
    (0xC16E76, i_c16e76),
    (0xC16E7A, i_c16e7a),
    (0xC16E7C, i_c16e7c),
    (0xC16E82, i_c16e82),
    (0xC16E88, i_c16e88),
    (0xC16E8A, i_c16e8a),
    (0xC16E8E, i_c16e8e),
    (0xC16E90, i_c16e90),
    (0xC16E92, i_c16e92),
    (0xC16E94, i_c16e94),
    (0xC16E96, i_c16e96),
    (0xC16E98, i_c16e98),
    (0xC16E9C, i_c16e9c),
    (0xC16E9E, i_c16e9e),
    (0xC16EA2, i_c16ea2),
    (0xC16EA4, i_c16ea4),
    (0xC16EA6, i_c16ea6),
    (0xC16EAA, i_c16eaa),
    (0xC16EAE, i_c16eae),
    (0xC16EB2, i_c16eb2),
    (0xC16EB6, i_c16eb6),
    (0xC16EB8, i_c16eb8),
    (0xC16EBC, i_c16ebc),
    (0xC16EBE, i_c16ebe),
    (0xC16EC4, i_c16ec4),
    (0xC16EC8, i_c16ec8),
    (0xC16ECE, i_c16ece),
    (0xC16ED0, i_c16ed0),
    (0xC16ED6, i_c16ed6),
    (0xC16EDC, i_c16edc),
    (0xC16EE2, i_c16ee2),
    (0xC16EE6, i_c16ee6),
    (0xC16EEA, i_c16eea),
    (0xC16EEE, i_c16eee),
    (0xC16EF2, i_c16ef2),
    (0xC16EF6, i_c16ef6),
    (0xC16EFA, i_c16efa),
    (0xC16F00, i_c16f00),
    (0xC16F04, i_c16f04),
    (0xC16F06, i_c16f06),
    (0xC16F0A, i_c16f0a),
    (0xC16F0C, i_c16f0c),
    (0xC16F12, i_c16f12),
    (0xC16F18, i_c16f18),
    (0xC16F1E, i_c16f1e),
    (0xC16F22, i_c16f22),
    (0xC16F26, i_c16f26),
    (0xC16F2A, i_c16f2a),
    (0xC16F2C, i_c16f2c),
    (0xC16F30, i_c16f30),
    (0xC16F34, i_c16f34),
    (0xC16F38, i_c16f38),
    (0xC16F3C, i_c16f3c),
    (0xC16F40, i_c16f40),
    (0xC16F46, i_c16f46),
    (0xC16F4A, i_c16f4a),
    (0xC16F50, i_c16f50),
    (0xC16F52, i_c16f52),
    (0xC16F54, i_c16f54),
    (0xC16F56, i_c16f56),
    (0xC16F58, i_c16f58),
    (0xC16F5C, i_c16f5c),
    (0xC16F5E, i_c16f5e),
    (0xC16F62, i_c16f62),
    (0xC16F66, i_c16f66),
    (0xC16F68, i_c16f68),
    (0xC16F6C, i_c16f6c),
    (0xC16F6E, i_c16f6e),
    (0xC16F72, i_c16f72),
    (0xC16FD4, i_c16fd4),
    (0xC16FDA, i_c16fda),
    (0xC16FDE, i_c16fde),
    (0xC16FE4, i_c16fe4),
    (0xC16FE6, i_c16fe6),
    (0xC16FEA, i_c16fea),
    (0xC16FEE, i_c16fee),
    (0xC16FF2, i_c16ff2),
    (0xC16FF6, i_c16ff6),
    (0xC16FFC, i_c16ffc),
    (0xC17000, i_c17000),
    (0xC17002, i_c17002),
    (0xC17008, i_c17008),
    (0xC1700C, i_c1700c),
    (0xC1705E, i_c1705e),
    (0xC17062, i_c17062),
    (0xC17066, i_c17066),
    (0xC1706A, i_c1706a),
    (0xC1706E, i_c1706e),
    (0xC17072, i_c17072),
    (0xC17074, i_c17074),
    (0xC17076, i_c17076),
    (0xC17078, i_c17078),
    (0xC1707A, i_c1707a),
    (0xC1707C, i_c1707c),
    (0xC1707E, i_c1707e),
    (0xC17080, i_c17080),
    (0xC17086, i_c17086),
    (0xC1708A, i_c1708a),
    (0xC17090, i_c17090),
    (0xC17092, i_c17092),
    (0xC17096, i_c17096),
    (0xC17098, i_c17098),
    (0xC1709E, i_c1709e),
    (0xC170A4, i_c170a4),
    (0xC170A6, i_c170a6),
    (0xC170AC, i_c170ac),
    (0xC170AE, i_c170ae),
    (0xC170B2, i_c170b2),
    (0xC170B4, i_c170b4),
    (0xC170BA, i_c170ba),
    (0xC170C0, i_c170c0),
    (0xC170C2, i_c170c2),
    (0xC170C8, i_c170c8),
    (0xC170CC, i_c170cc),
    (0xC170CE, i_c170ce),
    (0xC170D0, i_c170d0),
    (0xC170D6, i_c170d6),
    (0xC170D8, i_c170d8),
    (0xC170DA, i_c170da),
    (0xC170E0, i_c170e0),
    (0xC170E2, i_c170e2),
    (0xC170E8, i_c170e8),
    (0xC170EA, i_c170ea),
    (0xC170EE, i_c170ee),
    (0xC170F2, i_c170f2),
    (0xC170F6, i_c170f6),
    (0xC170F8, i_c170f8),
    (0xC170FA, i_c170fa),
    (0xC170FC, i_c170fc),
    (0xC17100, i_c17100),
    (0xC17104, i_c17104),
    (0xC17108, i_c17108),
    (0xC1710C, i_c1710c),
    (0xC1710E, i_c1710e),
    (0xC17112, i_c17112),
    (0xC17116, i_c17116),
    (0xC17118, i_c17118),
    (0xC1711C, i_c1711c),
    (0xC17120, i_c17120),
    (0xC17124, i_c17124),
    (0xC17128, i_c17128),
    (0xC1712C, i_c1712c),
    (0xC17130, i_c17130),
    (0xC17134, i_c17134),
    (0xC1713C, i_c1713c),
    (0xC17140, i_c17140),
    (0xC17144, i_c17144),
    (0xC17148, i_c17148),
    (0xC1714C, i_c1714c),
    (0xC17150, i_c17150),
    (0xC17154, i_c17154),
    (0xC17156, i_c17156),
    (0xC17158, i_c17158),
    (0xC1715E, i_c1715e),
    (0xC17160, i_c17160),
    (0xC17164, i_c17164),
    (0xC17168, i_c17168),
    (0xC1716C, i_c1716c),
    (0xC17170, i_c17170),
    (0xC17174, i_c17174),
    (0xC17178, i_c17178),
    (0xC1717C, i_c1717c),
    (0xC17180, i_c17180),
    (0xC17184, i_c17184),
    (0xC17188, i_c17188),
    (0xC1718C, i_c1718c),
    (0xC1718E, i_c1718e),
    (0xC17192, i_c17192),
    (0xC17196, i_c17196),
    (0xC17198, i_c17198),
    (0xC1719C, i_c1719c),
    (0xC1719E, i_c1719e),
    (0xC171A0, i_c171a0),
    (0xC171A4, i_c171a4),
    (0xC171A6, i_c171a6),
    (0xC171A8, i_c171a8),
    (0xC171AA, i_c171aa),
    (0xC171AE, i_c171ae),
    (0xC171B2, i_c171b2),
    (0xC171B6, i_c171b6),
    (0xC171B8, i_c171b8),
    (0xC171BA, i_c171ba),
    (0xC171BE, i_c171be),
    (0xC171C0, i_c171c0),
    (0xC171C2, i_c171c2),
    (0xC171C4, i_c171c4),
    (0xC171C6, i_c171c6),
    (0xC171C8, i_c171c8),
    (0xC171CC, i_c171cc),
    (0xC171CE, i_c171ce),
    (0xC171D0, i_c171d0),
    (0xC171D4, i_c171d4),
    (0xC171D8, i_c171d8),
    (0xC171DA, i_c171da),
    (0xC171DC, i_c171dc),
    (0xC171DE, i_c171de),
    (0xC171E0, i_c171e0),
    (0xC171E4, i_c171e4),
    (0xC171E6, i_c171e6),
    (0xC171E8, i_c171e8),
    (0xC171EC, i_c171ec),
    (0xC171EE, i_c171ee),
    (0xC171F4, i_c171f4),
    (0xC171F6, i_c171f6),
    (0xC171FC, i_c171fc),
    (0xC17200, i_c17200),
    (0xC17202, i_c17202),
    (0xC17206, i_c17206),
    (0xC1720A, i_c1720a),
    (0xC1720E, i_c1720e),
    (0xC17210, i_c17210),
    (0xC17212, i_c17212),
    (0xC17216, i_c17216),
    (0xC17218, i_c17218),
    (0xC1721C, i_c1721c),
    (0xC17220, i_c17220),
    (0xC17222, i_c17222),
    (0xC17224, i_c17224),
    (0xC17226, i_c17226),
    (0xC17228, i_c17228),
    (0xC1722A, i_c1722a),
    (0xC1722C, i_c1722c),
    (0xC17230, i_c17230),
    (0xC17232, i_c17232),
    (0xC17236, i_c17236),
    (0xC17238, i_c17238),
    (0xC1723A, i_c1723a),
    (0xC1723C, i_c1723c),
    (0xC1723E, i_c1723e),
    (0xC17240, i_c17240),
    (0xC17242, i_c17242),
    (0xC17244, i_c17244),
    (0xC17246, i_c17246),
    (0xC1724A, i_c1724a),
    (0xC1724C, i_c1724c),
    (0xC17250, i_c17250),
    (0xC17254, i_c17254),
    (0xC1725A, i_c1725a),
    (0xC1725C, i_c1725c),
    (0xC17262, i_c17262),
    (0xC17264, i_c17264),
    (0xC17268, i_c17268),
    (0xC1726C, i_c1726c),
    (0xC17272, i_c17272),
    (0xC17274, i_c17274),
    (0xC1727A, i_c1727a),
    (0xC1727C, i_c1727c),
    (0xC1727E, i_c1727e),
    (0xC17286, i_c17286),
    (0xC1728C, i_c1728c),
    (0xC1728E, i_c1728e),
    (0xC17290, i_c17290),
    (0xC17296, i_c17296),
    (0xC17298, i_c17298),
    (0xC1729E, i_c1729e),
    (0xC172A8, i_c172a8),
    (0xC172AE, i_c172ae),
    (0xC172B4, i_c172b4),
    (0xC172B8, i_c172b8),
    (0xC172BE, i_c172be),
    (0xC172C0, i_c172c0),
    (0xC172CA, i_c172ca),
    (0xC172D0, i_c172d0),
    (0xC172D2, i_c172d2),
    (0xC172D4, i_c172d4),
    (0xC172D6, i_c172d6),
    (0xC172D8, i_c172d8),
    (0xC172DA, i_c172da),
    (0xC172DC, i_c172dc),
    (0xC172DE, i_c172de),
    (0xC172E0, i_c172e0),
    (0xC172E4, i_c172e4),
    (0xC172E6, i_c172e6),
    (0xC172E8, i_c172e8),
    (0xC172EC, i_c172ec),
    (0xC172EE, i_c172ee),
    (0xC17328, i_c17328),
    (0xC1732A, i_c1732a),
    (0xC1732C, i_c1732c),
    (0xC1732E, i_c1732e),
    (0xC17334, i_c17334),
    (0xC17336, i_c17336),
    (0xC17338, i_c17338),
    (0xC1733A, i_c1733a),
    (0xC1733C, i_c1733c),
    (0xC1733E, i_c1733e),
    (0xC17340, i_c17340),
    (0xC17344, i_c17344),
    (0xC17346, i_c17346),
    (0xC17348, i_c17348),
    (0xC1734A, i_c1734a),
    (0xC1738C, i_c1738c),
    (0xC1738E, i_c1738e),
    (0xC17390, i_c17390),
    (0xC17394, i_c17394),
    (0xC17396, i_c17396),
    (0xC17398, i_c17398),
    (0xC1739A, i_c1739a),
    (0xC1739C, i_c1739c),
    (0xC173A0, i_c173a0),
    (0xC173A2, i_c173a2),
    (0xC173A4, i_c173a4),
    (0xC173A6, i_c173a6),
    (0xC173A8, i_c173a8),
    (0xC173AA, i_c173aa),
    (0xC173AE, i_c173ae),
    (0xC173B0, i_c173b0),
    (0xC173B2, i_c173b2),
    (0xC173B4, i_c173b4),
    (0xC173B8, i_c173b8),
    (0xC173BA, i_c173ba),
    (0xC173BC, i_c173bc),
    (0xC173BE, i_c173be),
    (0xC173C0, i_c173c0),
    (0xC173C2, i_c173c2),
    (0xC173C4, i_c173c4),
    (0xC173C6, i_c173c6),
    (0xC173C8, i_c173c8),
    (0xC173CA, i_c173ca),
    (0xC173CC, i_c173cc),
    (0xC173CE, i_c173ce),
    (0xC173D0, i_c173d0),
    (0xC173D4, i_c173d4),
    (0xC173D6, i_c173d6),
    (0xC173D8, i_c173d8),
    (0xC173DA, i_c173da),
    (0xC173DC, i_c173dc),
    (0xC173DE, i_c173de),
    (0xC173E0, i_c173e0),
    (0xC173E2, i_c173e2),
    (0xC173E4, i_c173e4),
    (0xC173E6, i_c173e6),
    (0xC1744E, i_c1744e),
    (0xC17454, i_c17454),
    (0xC17456, i_c17456),
    (0xC17458, i_c17458),
    (0xC1745A, i_c1745a),
    (0xC1745E, i_c1745e),
    (0xC17460, i_c17460),
    (0xC17462, i_c17462),
    (0xC17464, i_c17464),
    (0xC17468, i_c17468),
    (0xC1746C, i_c1746c),
    (0xC1746E, i_c1746e),
    (0xC17470, i_c17470),
    (0xC17474, i_c17474),
    (0xC17476, i_c17476),
    (0xC17478, i_c17478),
    (0xC1747A, i_c1747a),
    (0xC1747E, i_c1747e),
    (0xC17480, i_c17480),
    (0xC17482, i_c17482),
    (0xC17484, i_c17484),
    (0xC17AE6, i_c17ae6),
    (0xC17AEC, i_c17aec),
    (0xC17AF0, i_c17af0),
    (0xC17AF4, i_c17af4),
    (0xC17AF8, i_c17af8),
    (0xC17AFE, i_c17afe),
    (0xC17B04, i_c17b04),
    (0xC17B06, i_c17b06),
    (0xC17B08, i_c17b08),
    (0xC17B0A, i_c17b0a),
    (0xC17B0C, i_c17b0c),
    (0xC17B12, i_c17b12),
    (0xC17B16, i_c17b16),
    (0xC17B1C, i_c17b1c),
    (0xC17B20, i_c17b20),
    (0xC17B24, i_c17b24),
    (0xC17B28, i_c17b28),
    (0xC17B2C, i_c17b2c),
    (0xC17B30, i_c17b30),
    (0xC17B34, i_c17b34),
    (0xC17B38, i_c17b38),
    (0xC17B3C, i_c17b3c),
    (0xC17B40, i_c17b40),
    (0xC17B46, i_c17b46),
    (0xC17B4C, i_c17b4c),
    (0xC17B50, i_c17b50),
    (0xC17B54, i_c17b54),
    (0xC17B58, i_c17b58),
    (0xC17B5C, i_c17b5c),
    (0xC17B60, i_c17b60),
    (0xC17B64, i_c17b64),
    (0xC17B6A, i_c17b6a),
    (0xC17B70, i_c17b70),
    (0xC17B76, i_c17b76),
    (0xC17B7A, i_c17b7a),
    (0xC17B7E, i_c17b7e),
    (0xC17B82, i_c17b82),
    (0xC17B86, i_c17b86),
    (0xC17B8A, i_c17b8a),
    (0xC17B8E, i_c17b8e),
    (0xC17B98, i_c17b98),
    (0xC17BA2, i_c17ba2),
    (0xC17BAC, i_c17bac),
    (0xC17BB2, i_c17bb2),
    (0xC17BB8, i_c17bb8),
    (0xC17BBE, i_c17bbe),
    (0xC17BC6, i_c17bc6),
    (0xC17BCE, i_c17bce),
    (0xC17BD6, i_c17bd6),
    (0xC17BDC, i_c17bdc),
    (0xC17BE4, i_c17be4),
    (0xC17BE8, i_c17be8),
    (0xC17BEE, i_c17bee),
    (0xC17BF2, i_c17bf2),
    (0xC17BF6, i_c17bf6),
    (0xC17BFA, i_c17bfa),
    (0xC17BFE, i_c17bfe),
    (0xC17C04, i_c17c04),
    (0xC17C0A, i_c17c0a),
    (0xC17C0E, i_c17c0e),
    (0xC17C10, i_c17c10),
    (0xC17C12, i_c17c12),
    (0xC17C16, i_c17c16),
    (0xC17C1A, i_c17c1a),
    (0xC17C1E, i_c17c1e),
    (0xC17C22, i_c17c22),
    (0xC17C26, i_c17c26),
    (0xC17C2A, i_c17c2a),
    (0xC17C2E, i_c17c2e),
    (0xC17C32, i_c17c32),
    (0xC17C36, i_c17c36),
    (0xC17C3A, i_c17c3a),
    (0xC17C3E, i_c17c3e),
    (0xC17C42, i_c17c42),
    (0xC17C44, i_c17c44),
    (0xC17C48, i_c17c48),
    (0xC17C4C, i_c17c4c),
    (0xC17C52, i_c17c52),
    (0xC17C56, i_c17c56),
    (0xC17C5C, i_c17c5c),
    (0xC17C60, i_c17c60),
    (0xC17C64, i_c17c64),
    (0xC17C66, i_c17c66),
    (0xC17C6A, i_c17c6a),
    (0xC17C6E, i_c17c6e),
    (0xC17C72, i_c17c72),
    (0xC17C76, i_c17c76),
    (0xC17C7A, i_c17c7a),
    (0xC17C7E, i_c17c7e),
    (0xC17C82, i_c17c82),
    (0xC17C86, i_c17c86),
    (0xC17C8A, i_c17c8a),
    (0xC17C8C, i_c17c8c),
    (0xC17C90, i_c17c90),
    (0xC17C94, i_c17c94),
    (0xC17C98, i_c17c98),
    (0xC17C9C, i_c17c9c),
    (0xC17CA0, i_c17ca0),
    (0xC17CA4, i_c17ca4),
    (0xC17CA8, i_c17ca8),
    (0xC17CAA, i_c17caa),
    (0xC17CAC, i_c17cac),
    (0xC17CB0, i_c17cb0),
    (0xC17CB2, i_c17cb2),
    (0xC17CB4, i_c17cb4),
    (0xC17CB8, i_c17cb8),
    (0xC17CBA, i_c17cba),
    (0xC17CBE, i_c17cbe),
    (0xC17CC0, i_c17cc0),
    (0xC17CC4, i_c17cc4),
    (0xC17CC8, i_c17cc8),
    (0xC17CCA, i_c17cca),
    (0xC17CCE, i_c17cce),
    (0xC17CD2, i_c17cd2),
    (0xC17CD4, i_c17cd4),
    (0xC17CD8, i_c17cd8),
    (0xC17CDC, i_c17cdc),
    (0xC17CE2, i_c17ce2),
    (0xC17CE6, i_c17ce6),
    (0xC17CEA, i_c17cea),
    (0xC17CEC, i_c17cec),
    (0xC17CF0, i_c17cf0),
    (0xC17CF4, i_c17cf4),
    (0xC17CF6, i_c17cf6),
    (0xC17CFA, i_c17cfa),
    (0xC17CFE, i_c17cfe),
    (0xC17D00, i_c17d00),
    (0xC17D02, i_c17d02),
    (0xC17D04, i_c17d04),
    (0xC17D08, i_c17d08),
    (0xC17D0A, i_c17d0a),
    (0xC17D0E, i_c17d0e),
    (0xC17D10, i_c17d10),
    (0xC17D12, i_c17d12),
    (0xC17D16, i_c17d16),
    (0xC17D18, i_c17d18),
    (0xC17D1C, i_c17d1c),
    (0xC17D20, i_c17d20),
    (0xC17D22, i_c17d22),
    (0xC17D24, i_c17d24),
    (0xC17D28, i_c17d28),
    (0xC17D2A, i_c17d2a),
    (0xC17D2E, i_c17d2e),
    (0xC17D32, i_c17d32),
    (0xC17D36, i_c17d36),
    (0xC17D3A, i_c17d3a),
    (0xC17D3C, i_c17d3c),
    (0xC17D40, i_c17d40),
    (0xC17D42, i_c17d42),
    (0xC17E7A, i_c17e7a),
    (0xC17E7E, i_c17e7e),
    (0xC17E80, i_c17e80),
    (0xC17E84, i_c17e84),
    (0xC17E86, i_c17e86),
    (0xC17E88, i_c17e88),
    (0xC17E8C, i_c17e8c),
    (0xC17EAA, i_c17eaa),
    (0xC17EB0, i_c17eb0),
    (0xC17F40, i_c17f40),
    (0xC17F42, i_c17f42),
    (0xC17F46, i_c17f46),
    (0xC17F4A, i_c17f4a),
    (0xC17F4E, i_c17f4e),
    (0xC17F52, i_c17f52),
    (0xC17F56, i_c17f56),
    (0xC17F5A, i_c17f5a),
    (0xC17F60, i_c17f60),
    (0xC17F66, i_c17f66),
    (0xC17F6A, i_c17f6a),
    (0xC17F70, i_c17f70),
    (0xC17F76, i_c17f76),
    (0xC17F7A, i_c17f7a),
    (0xC17F7E, i_c17f7e),
    (0xC17F80, i_c17f80),
    (0xC17FCA, i_c17fca),
    (0xC17FCE, i_c17fce),
    (0xC17FD2, i_c17fd2),
    (0xC17FD4, i_c17fd4),
    (0xC17FD6, i_c17fd6),
    (0xC17FDA, i_c17fda),
    (0xC1A328, i_c1a328),
    (0xC1A32E, i_c1a32e),
    (0xC1A334, i_c1a334),
    (0xC1A336, i_c1a336),
    (0xC1A338, i_c1a338),
    (0xC1A33A, i_c1a33a),
    (0xC1A33C, i_c1a33c),
    (0xC1A342, i_c1a342),
    (0xC1A346, i_c1a346),
    (0xC1A34C, i_c1a34c),
    (0xC1A350, i_c1a350),
    (0xC1A354, i_c1a354),
    (0xC1A358, i_c1a358),
    (0xC1A35C, i_c1a35c),
    (0xC1A360, i_c1a360),
    (0xC1A364, i_c1a364),
    (0xC1A368, i_c1a368),
    (0xC1A36C, i_c1a36c),
    (0xC1A370, i_c1a370),
    (0xC1A376, i_c1a376),
    (0xC1A37C, i_c1a37c),
    (0xC1A380, i_c1a380),
    (0xC1A384, i_c1a384),
    (0xC1A388, i_c1a388),
    (0xC1A38C, i_c1a38c),
    (0xC1A390, i_c1a390),
    (0xC1A394, i_c1a394),
    (0xC1A39A, i_c1a39a),
    (0xC1A3A0, i_c1a3a0),
    (0xC1A3A6, i_c1a3a6),
    (0xC1A3AA, i_c1a3aa),
    (0xC1A3AE, i_c1a3ae),
    (0xC1A3B2, i_c1a3b2),
    (0xC1A3B6, i_c1a3b6),
    (0xC1A3BA, i_c1a3ba),
    (0xC1A3BE, i_c1a3be),
    (0xC1A3C8, i_c1a3c8),
    (0xC1A3D2, i_c1a3d2),
    (0xC1A3D8, i_c1a3d8),
    (0xC1A3DE, i_c1a3de),
    (0xC1A3E4, i_c1a3e4),
    (0xC1A3EA, i_c1a3ea),
    (0xC1A3F0, i_c1a3f0),
    (0xC1A3F8, i_c1a3f8),
    (0xC1A400, i_c1a400),
    (0xC1A408, i_c1a408),
    (0xC1A40E, i_c1a40e),
    (0xC1A416, i_c1a416),
    (0xC1A41A, i_c1a41a),
    (0xC1A420, i_c1a420),
    (0xC1A426, i_c1a426),
    (0xC1A42A, i_c1a42a),
    (0xC1A42E, i_c1a42e),
    (0xC1A434, i_c1a434),
    (0xC1A438, i_c1a438),
    (0xC1A43C, i_c1a43c),
    (0xC1A442, i_c1a442),
    (0xC1A446, i_c1a446),
    (0xC1A448, i_c1a448),
    (0xC1A44A, i_c1a44a),
    (0xC1A44E, i_c1a44e),
    (0xC1A454, i_c1a454),
    (0xC1A45A, i_c1a45a),
    (0xC1A45E, i_c1a45e),
    (0xC1A462, i_c1a462),
    (0xC1A464, i_c1a464),
    (0xC1A466, i_c1a466),
    (0xC1A46A, i_c1a46a),
    (0xC1A46E, i_c1a46e),
    (0xC1A472, i_c1a472),
    (0xC1A476, i_c1a476),
    (0xC1A47A, i_c1a47a),
    (0xC1A47E, i_c1a47e),
    (0xC1A480, i_c1a480),
    (0xC1A486, i_c1a486),
    (0xC1A48A, i_c1a48a),
    (0xC1A48C, i_c1a48c),
    (0xC1A490, i_c1a490),
    (0xC1A492, i_c1a492),
    (0xC1A494, i_c1a494),
    (0xC1A496, i_c1a496),
    (0xC1A49A, i_c1a49a),
    (0xC1A49C, i_c1a49c),
    (0xC1A4A0, i_c1a4a0),
    (0xC1A4A6, i_c1a4a6),
    (0xC1A4A8, i_c1a4a8),
    (0xC1A4AC, i_c1a4ac),
    (0xC1A4B0, i_c1a4b0),
    (0xC1A4B6, i_c1a4b6),
    (0xC1A4B8, i_c1a4b8),
    (0xC1A4BC, i_c1a4bc),
    (0xC1A4C0, i_c1a4c0),
    (0xC1A4C4, i_c1a4c4),
    (0xC1A4C8, i_c1a4c8),
    (0xC1A4CC, i_c1a4cc),
    (0xC1A4CE, i_c1a4ce),
    (0xC1A4D2, i_c1a4d2),
    (0xC1A4D4, i_c1a4d4),
    (0xC1A4D8, i_c1a4d8),
    (0xC1A4DC, i_c1a4dc),
    (0xC1A4E2, i_c1a4e2),
    (0xC1A4E6, i_c1a4e6),
    (0xC1A4EA, i_c1a4ea),
    (0xC1A4EC, i_c1a4ec),
    (0xC1A4F0, i_c1a4f0),
    (0xC1A4F2, i_c1a4f2),
    (0xC1A4F6, i_c1a4f6),
    (0xC1A4FA, i_c1a4fa),
    (0xC1A536, i_c1a536),
    (0xC1A538, i_c1a538),
    (0xC1A53E, i_c1a53e),
    (0xC1A542, i_c1a542),
    (0xC1A544, i_c1a544),
    (0xC1A546, i_c1a546),
    (0xC1A54C, i_c1a54c),
    (0xC1A550, i_c1a550),
    (0xC1A554, i_c1a554),
    (0xC1A556, i_c1a556),
    (0xC1A558, i_c1a558),
    (0xC1A55A, i_c1a55a),
    (0xC1A55C, i_c1a55c),
    (0xC1A55E, i_c1a55e),
    (0xC1A562, i_c1a562),
    (0xC1A566, i_c1a566),
    (0xC1A56A, i_c1a56a),
    (0xC1A56E, i_c1a56e),
    (0xC1A570, i_c1a570),
    (0xC1A574, i_c1a574),
    (0xC1A576, i_c1a576),
    (0xC1A578, i_c1a578),
    (0xC1A57A, i_c1a57a),
    (0xC1A580, i_c1a580),
    (0xC1A584, i_c1a584),
    (0xC1A588, i_c1a588),
    (0xC1A58A, i_c1a58a),
    (0xC1A58C, i_c1a58c),
    (0xC1A592, i_c1a592),
    (0xC1A594, i_c1a594),
    (0xC1A598, i_c1a598),
    (0xC1A59C, i_c1a59c),
    (0xC1A5A2, i_c1a5a2),
    (0xC1A5A6, i_c1a5a6),
    (0xC1A5AC, i_c1a5ac),
    (0xC1A5AE, i_c1a5ae),
    (0xC1A5B0, i_c1a5b0),
    (0xC1A5B4, i_c1a5b4),
    (0xC1A5B6, i_c1a5b6),
    (0xC1A5BA, i_c1a5ba),
    (0xC1A5BE, i_c1a5be),
    (0xC1A5C2, i_c1a5c2),
    (0xC1A5C4, i_c1a5c4),
    (0xC1A5C8, i_c1a5c8),
    (0xC1A5CC, i_c1a5cc),
    (0xC1A5CE, i_c1a5ce),
    (0xC1A5D0, i_c1a5d0),
    (0xC1A5D2, i_c1a5d2),
    (0xC1A5D4, i_c1a5d4),
    (0xC1A5D8, i_c1a5d8),
    (0xC1A5DA, i_c1a5da),
    (0xC1A5DE, i_c1a5de),
    (0xC1A5E0, i_c1a5e0),
    (0xC1A5E2, i_c1a5e2),
    (0xC1A5E8, i_c1a5e8),
    (0xC1A5EC, i_c1a5ec),
    (0xC1A5F2, i_c1a5f2),
    (0xC1A5F6, i_c1a5f6),
    (0xC1A5FA, i_c1a5fa),
    (0xC1A5FC, i_c1a5fc),
    (0xC1A602, i_c1a602),
    (0xC1A604, i_c1a604),
    (0xC1A60A, i_c1a60a),
    (0xC1A612, i_c1a612),
    (0xC1A618, i_c1a618),
    (0xC1A61C, i_c1a61c),
    (0xC1A620, i_c1a620),
    (0xC1A626, i_c1a626),
    (0xC1A62A, i_c1a62a),
    (0xC1A62E, i_c1a62e),
    (0xC1A634, i_c1a634),
    (0xC1A638, i_c1a638),
    (0xC1A63C, i_c1a63c),
    (0xC1A642, i_c1a642),
    (0xC1A646, i_c1a646),
    (0xC1A64A, i_c1a64a),
    (0xC1A64E, i_c1a64e),
    (0xC1A650, i_c1a650),
    (0xC1A654, i_c1a654),
    (0xC1A658, i_c1a658),
    (0xC1A65E, i_c1a65e),
    (0xC1A660, i_c1a660),
    (0xC1A664, i_c1a664),
    (0xC1A666, i_c1a666),
    (0xC1A66C, i_c1a66c),
    (0xC1A670, i_c1a670),
    (0xC1A676, i_c1a676),
    (0xC1A67A, i_c1a67a),
    (0xC1A680, i_c1a680),
    (0xC1A686, i_c1a686),
    (0xC1A688, i_c1a688),
    (0xC1A68A, i_c1a68a),
    (0xC1A68C, i_c1a68c),
    (0xC1A68E, i_c1a68e),
    (0xC1A690, i_c1a690),
];
