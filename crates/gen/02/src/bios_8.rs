// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región bios_8) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $C18C7A: move.b #$ff, (a0)+
pub fn i_c18c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C7E;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18C7E: move.w #$3, (a0)+
pub fn i_c18c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C82;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18C82: add.w d6, d3
pub fn i_c18c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C84;
    cpu.ir = 0xD646;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18C84: move.w d3, (a0)+
pub fn i_c18c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C86;
    cpu.ir = 0x30C3;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18C86: move.w #$108, (a0)+
pub fn i_c18c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C8A;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18C8A: move.b #$53, (a0)+
pub fn i_c18c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C8E;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x53;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18C8E: cmpi.b #$31, d1
pub fn i_c18c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C92;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x31, Size::Byte, false);
    8
}
/// $C18C92: bne.b $c18c9a
pub fn i_c18c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C94;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18C9A; return 10; }
    8
}
/// $C18C94: move.b #$20, -$1(a0)
pub fn i_c18c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C9A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C18C9A: move.b #$ff, (a0)+
pub fn i_c18c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18C9E;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18C9E: rts
pub fn i_c18c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18CA0: move.b $a97(a5), d0
pub fn i_c18ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CA4;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18CA4: andi.b #$30, d0
pub fn i_c18ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CA8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18CA8: beq.w $c18ce2
pub fn i_c18ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18CE2; return 10; }
    8
}
/// $C18CAC: move.b d0, $3a001d.l
pub fn i_c18cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CB2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C18CB2: move.b -$7fbe(a4), d0
pub fn i_c18cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CB6;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8042);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18CB6: beq.b $c18cbc
pub fn i_c18cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CB8;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18CBC; return 10; }
    8
}
/// $C18CB8: moveq #$0, d0
pub fn i_c18cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CBA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18CBA: bra.b $c18cbe
pub fn i_c18cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CBC;
    cpu.ir = 0x6002;
    cpu.pc = 0xC18CBE;
    10
}
/// $C18CBC: moveq #$1, d0
pub fn i_c18cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CBE;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18CBE: move.b d0, -$7fbe(a4)
pub fn i_c18cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CC2;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8042);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C18CC2: addq.b #$1, $ac2(a5)
pub fn i_c18cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CC6;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18CC6: movea.l $abe(a5), a0
pub fn i_c18cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CCA;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C18CCA: lea.l $c18ce4(pc), a1
pub fn i_c18cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC18CE4;
    16
}
/// $C18CCE: tst.b -$7fbe(a4)
pub fn i_c18cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CD2;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8042);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C18CD2: beq.b $c18cd8
pub fn i_c18cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CD4;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18CD8; return 10; }
    8
}
/// $C18CD4: lea.l $c18d00(pc), a1
pub fn i_c18cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC18D00;
    16
}
/// $C18CD8: move.l a1, (a0)+
pub fn i_c18cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CDA;
    cpu.ir = 0x20C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18CDA: move.l a0, $abe(a5)
pub fn i_c18cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CDE;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18CDE: subq.b #$1, $ac2(a5)
pub fn i_c18cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CE2;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18CE2: rts
pub fn i_c18ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18CE4: ori.b #$4b, d3
pub fn i_c18ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CE8;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4B) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18CE8: movep.w $4f4e(a0), d0
pub fn i_c18ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CEC;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4F4E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C18CEC: ?
pub fn i_c18cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CEE;
    cpu.ir = 0x4C59;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18CEE: movea.l (a7), a0
pub fn i_c18cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CF0;
    cpu.ir = 0x2057;
    let v2 = bus.read32(cpu.a[7]);
    cpu.a[0] = v2;
    12
}
/// $C18CF0: swap d5
pub fn i_c18cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CF2;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18CF2: ?
pub fn i_c18cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CF4;
    cpu.ir = 0x4E20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18CF4: ?
pub fn i_c18cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CF6;
    cpu.ir = 0x4352;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18CF6: ?
pub fn i_c18cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CF8;
    cpu.ir = 0x4544;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18CF8: ?
pub fn i_c18cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CFA;
    cpu.ir = 0x4954;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18CFA: ?
pub fn i_c18cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CFC;
    cpu.ir = 0x4544;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18CFC: dc.w $ffff
pub fn i_c18cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18CFE;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C18CFE: ori.b #$3, d0
pub fn i_c18cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D02;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18D02: moveq #$4b, d1
pub fn i_c18d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D04;
    cpu.ir = 0x724B;
    cpu.d[1] = 0x4B;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18D04: movep.w $414e(a0), d0
pub fn i_c18d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D08;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x414E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C18D08: subq.w #$4, (a4)
pub fn i_c18d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D0A;
    cpu.ir = 0x5954;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C18D0A: ?
pub fn i_c18d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D0C;
    cpu.ir = 0x494D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18D1C: move.b $a98(a5), d0
pub fn i_c18d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D20;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18D20: andi.b #$30, d0
pub fn i_c18d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D24;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18D24: beq.w $c18db6
pub fn i_c18d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18DB6; return 10; }
    8
}
/// $C18D28: move.b d0, $3a001d.l
pub fn i_c18d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D2E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C18D2E: moveq #$1, d1
pub fn i_c18d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D30;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18D30: andi.b #$10, d0
pub fn i_c18d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D34;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18D34: bne.b $c18d3a
pub fn i_c18d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D36;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18D3A; return 10; }
    8
}
/// $C18D36: move.b #$99, d1
pub fn i_c18d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D3A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C18D3A: move.b -$7fbc(a4), d2
pub fn i_c18d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D3E;
    cpu.ir = 0x142C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8044);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18D3E: sub.b d0, d0
pub fn i_c18d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D40;
    cpu.ir = 0x9000;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C18D40: abcd.b d1, d2
pub fn i_c18d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D42;
    cpu.ir = 0xC501;
    let s = cpu.d[1] as u8; let d = cpu.d[2] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[2] = (cpu.d[2] & !0xFF) | r as u32;
    6
}
/// $C18D42: cmpi.b #$60, d2
pub fn i_c18d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D46;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x60, Size::Byte, false);
    8
}
/// $C18D46: bcc.b $c18d3e
pub fn i_c18d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D48;
    cpu.ir = 0x64F6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC18D3E; return 10; }
    8
}
/// $C18D48: move.b d2, -$7fbc(a4)
pub fn i_c18d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D4C;
    cpu.ir = 0x1942;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8044);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C18D4C: move.b #$3b, -$7fbb(a4)
pub fn i_c18d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D52;
    cpu.ir = 0x197C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8045);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C18D52: clr.b -$7fbd(a4)
pub fn i_c18d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D56;
    cpu.ir = 0x422C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8043);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18D56: tst.b d2
pub fn i_c18d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D58;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C18D58: bne.b $c18d60
pub fn i_c18d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D5A;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18D60; return 10; }
    8
}
/// $C18D5A: move.b #$1, -$7fbd(a4)
pub fn i_c18d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D60;
    cpu.ir = 0x197C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8043);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C18D60: addq.b #$1, $ac2(a5)
pub fn i_c18d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D64;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18D64: movea.l $abe(a5), a0
pub fn i_c18d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D68;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C18D68: tst.b -$7fbd(a4)
pub fn i_c18d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D6C;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8043);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C18D6C: bne.w $c18da8
pub fn i_c18d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D70;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18DA8; return 10; }
    8
}
/// $C18D70: clr.l (a0)+
pub fn i_c18d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D72;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C18D72: move.w #$3, (a0)+
pub fn i_c18d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D76;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18D76: move.w #$738e, (a0)+
pub fn i_c18d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D7A;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x738E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18D7A: move.w #$108, (a0)+
pub fn i_c18d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D7E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18D7E: move.b -$7fbc(a4), d0
pub fn i_c18d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D82;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8044);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18D82: jsr $c16f54(pc)
pub fn i_c18d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D86;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18D86);
    cpu.pc = 0xC16F54;
    16
}
/// $C18D86: move.b d0, (a0)+
pub fn i_c18d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D88;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18D88: move.b d1, (a0)+
pub fn i_c18d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D8A;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18D8A: move.b #$20, (a0)+
pub fn i_c18d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D8E;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18D8E: move.b #$73, (a0)+
pub fn i_c18d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D92;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x73;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18D92: move.b #$65, (a0)+
pub fn i_c18d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D96;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x65;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18D96: move.b #$63, (a0)+
pub fn i_c18d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D9A;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x63;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18D9A: move.b #$20, (a0)+
pub fn i_c18d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18D9E;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18D9E: move.b #$ff, (a0)+
pub fn i_c18d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DA2;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C18DA2: clr.w (a0)+
pub fn i_c18da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DA4;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C18DA4: bra.w $c18dae
pub fn i_c18da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DA8;
    cpu.ir = 0x6000;
    cpu.pc = 0xC18DAE;
    10
}
/// $C18DA8: move.l #$c18db8, (a0)+
pub fn i_c18da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DAE;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC18DB8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18DAE: move.l a0, $abe(a5)
pub fn i_c18dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DB2;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18DB2: subq.b #$1, $ac2(a5)
pub fn i_c18db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DB6;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18DB6: rts
pub fn i_c18db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18DB8: ori.b #$8e, d3
pub fn i_c18db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DBC;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8E) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18DBC: movep.w $5749(a0), d0
pub fn i_c18dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DC0;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5749);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C18DC0: addq.w #$2, a0
pub fn i_c18dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DC2;
    cpu.ir = 0x5448;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $C18DC2: ?
pub fn i_c18dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DC4;
    cpu.ir = 0x4F55;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18DC8: move.b $a97(a5), d0
pub fn i_c18dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DCC;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18DCC: andi.b #$30, d0
pub fn i_c18dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DD0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18DD0: beq.w $c18e0a
pub fn i_c18dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18E0A; return 10; }
    8
}
/// $C18DD4: move.b d0, $3a001d.l
pub fn i_c18dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DDA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C18DDA: move.b -$7fba(a4), d0
pub fn i_c18dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DDE;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8046);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C18DDE: beq.b $c18de4
pub fn i_c18dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DE0;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18DE4; return 10; }
    8
}
/// $C18DE0: moveq #$0, d0
pub fn i_c18de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DE2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18DE2: bra.b $c18de6
pub fn i_c18de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DE4;
    cpu.ir = 0x6002;
    cpu.pc = 0xC18DE6;
    10
}
/// $C18DE4: moveq #$1, d0
pub fn i_c18de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DE6;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18DE6: move.b d0, -$7fba(a4)
pub fn i_c18de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DEA;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8046);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C18DEA: addq.b #$1, $ac2(a5)
pub fn i_c18dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DEE;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18DEE: movea.l $abe(a5), a0
pub fn i_c18dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DF2;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C18DF2: lea.l $c18e28(pc), a1
pub fn i_c18df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC18E28;
    16
}
/// $C18DF6: tst.b -$7fba(a4)
pub fn i_c18df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DFA;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8046);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C18DFA: beq.b $c18e00
pub fn i_c18dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18DFC;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18E00; return 10; }
    8
}
/// $C18DFC: lea.l $c18e0c(pc), a1
pub fn i_c18dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC18E0C;
    16
}
/// $C18E00: move.l a1, (a0)+
pub fn i_c18e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E02;
    cpu.ir = 0x20C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18E02: move.l a0, $abe(a5)
pub fn i_c18e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E06;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18E06: subq.b #$1, $ac2(a5)
pub fn i_c18e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E0A;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18E0A: rts
pub fn i_c18e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18E0C: ori.b #$31, d3
pub fn i_c18e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E10;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x31) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18E10: movep.w $5749(a0), d0
pub fn i_c18e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E14;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5749);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C18E14: addq.w #$2, a0
pub fn i_c18e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E16;
    cpu.ir = 0x5448;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $C18E16: ?
pub fn i_c18e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E18;
    cpu.ir = 0x4F55;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18E18: addq.b #$2, -(a0)
pub fn i_c18e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E1A;
    cpu.ir = 0x5420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C18E1A: move.l -(a0), d0
pub fn i_c18e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E1C;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C18E1C: move.l -(a0), d0
pub fn i_c18e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E1E;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C18E1E: move.l -(a0), d0
pub fn i_c18e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E20;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C18E20: move.l -(a0), d0
pub fn i_c18e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E22;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C18E22: move.l -(a0), d0
pub fn i_c18e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E24;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C18E24: ?
pub fn i_c18e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E26;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C18E26: ori.b #$3, d0
pub fn i_c18e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E2A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18E2A: moveq #$31, d1
pub fn i_c18e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E2C;
    cpu.ir = 0x7231;
    cpu.d[1] = 0x31;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18E2C: movep.w $5345(a0), d0
pub fn i_c18e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E30;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5345);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C18E30: addq.b #$2, -(a0)
pub fn i_c18e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E32;
    cpu.ir = 0x5420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C18E32: subq.w #$2, (a0)
pub fn i_c18e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E34;
    cpu.ir = 0x5550;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C18E34: movea.l a1, a0
pub fn i_c18e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E36;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C18E36: ?
pub fn i_c18e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E38;
    cpu.ir = 0x4E20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18E38: ?
pub fn i_c18e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E3A;
    cpu.ir = 0x4541;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18E3A: ?
pub fn i_c18e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E3C;
    cpu.ir = 0x4348;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18E3C: movea.l d7, a0
pub fn i_c18e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E3E;
    cpu.ir = 0x2047;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C18E3E: ?
pub fn i_c18e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E40;
    cpu.ir = 0x414D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18E44: move.w $9e2(a5), d0
pub fn i_c18e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E48;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C18E48: add.w d0, d0
pub fn i_c18e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E4A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18E4A: add.w d0, d0
pub fn i_c18e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E4C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18E4C: movea.l $c18e52(pc, d0.w), a0
pub fn i_c18e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E50;
    cpu.ir = 0x207B;
    let ad2 = 0xC18E52_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C18E50: jmp (a0)
pub fn i_c18e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E52;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C18E52: ?
pub fn i_c18e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E54;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18E54: or.b (a0)+, d6
pub fn i_c18e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E56;
    cpu.ir = 0x8C18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $C18E56: ?
pub fn i_c18e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E58;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18E58: divu.w d2, d6
pub fn i_c18e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E5A;
    cpu.ir = 0x8CC2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[6] / dv; let rm = cpu.d[6] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[6] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $C18E5A: ?
pub fn i_c18e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E5C;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18E5C: or.w d6, -(a0)
pub fn i_c18e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E5E;
    cpu.ir = 0x8D60;
    let ad1 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad1;
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | cpu.d[6]) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    10
}
/// $C18E5E: ?
pub fn i_c18e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E60;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18E60: divs.w $522d(a2), d6
pub fn i_c18e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E64;
    cpu.ir = 0x8DEA;
    let ad2 = cpu.a[2].wrapping_add(0x522D);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v3 as u16 as i16 as i32;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let dd = cpu.d[6] as i32; let q = dd.wrapping_div(dv); let rm = dd.wrapping_rem(dv);
    if !(-32768..=32767).contains(&q) { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 166; }
    cpu.d[6] = ((rm as u32 & 0xFFFF) << 16) | (q as u32 & 0xFFFF);
    cpu.sr.set_nz(q as u32 & 0xFFFF, Size::Word); cpu.sr.set(StatusRegister::V, false);
    166
}
/// $C18E62: addq.b #$1, $ac2(a5)
pub fn i_c18e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E66;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18E64: ?
pub fn i_c18e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E66;
    cpu.ir = 0xAC2;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18E66: movea.l $abe(a5), a0
pub fn i_c18e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E6A;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C18E6A: clr.l (a0)+
pub fn i_c18e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E6C;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C18E6C: move.w #$20ff, d1
pub fn i_c18e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E70;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18E70: jsr $c18e8e(pc)
pub fn i_c18e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18E74);
    cpu.pc = 0xC18E8E;
    16
}
/// $C18E74: move.w $9e4(a5), $9e2(a5)
pub fn i_c18e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E7A;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9E4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9E2);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C18E7A: move.w #$11ff, d1
pub fn i_c18e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E7E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18E7E: jsr $c18e8e(pc)
pub fn i_c18e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18E82);
    cpu.pc = 0xC18E8E;
    16
}
/// $C18E82: clr.w (a0)+
pub fn i_c18e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E84;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C18E84: move.l a0, $abe(a5)
pub fn i_c18e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E88;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18E88: subq.b #$1, $ac2(a5)
pub fn i_c18e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E8C;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C18E8C: rts
pub fn i_c18e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18E8E: move.w #$3, (a0)+
pub fn i_c18e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E92;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18E92: move.w $9e2(a5), d0
pub fn i_c18e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E96;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C18E96: move.w d0, d2
pub fn i_c18e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E98;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C18E98: add.w d0, d0
pub fn i_c18e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E9A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18E9A: add.w d0, d2
pub fn i_c18e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18E9C;
    cpu.ir = 0xD440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18E9C: addi.w #$7068, d2
pub fn i_c18e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EA0;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7068, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C18EA0: move.w d2, (a0)+
pub fn i_c18ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EA2;
    cpu.ir = 0x30C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18EA2: move.w #$1108, (a0)+
pub fn i_c18ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EA6;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18EA6: move.w d1, (a0)+
pub fn i_c18ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EA8;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18EA8: rts
pub fn i_c18ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C18EAA: ori.b #$45, d3
pub fn i_c18eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EAE;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x45) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18EAE: movep.w $5345(a0), d0
pub fn i_c18eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EB2;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5345);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C18EB2: addq.w #$2, (a4)
pub fn i_c18eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EB4;
    cpu.ir = 0x5454;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C18EB4: ?
pub fn i_c18eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EB6;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18EB6: dc.w $4720
pub fn i_c18eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EB8;
    cpu.ir = 0x4720;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18EB8: subq.w #$2, (a0)
pub fn i_c18eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EBA;
    cpu.ir = 0x5550;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C18EBA: movea.l (a4), a0
pub fn i_c18eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EBC;
    cpu.ir = 0x2054;
    let v2 = bus.read32(cpu.a[4]);
    cpu.a[0] = v2;
    12
}
/// $C18EBC: swap d5
pub fn i_c18ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EBE;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18EBE: movea.l d3, a0
pub fn i_c18ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EC0;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C18EC0: ?
pub fn i_c18ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EC2;
    cpu.ir = 0x4142;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18EC2: ?
pub fn i_c18ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EC4;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18EC4: ?
pub fn i_c18ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EC6;
    cpu.ir = 0x4554;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18EC6: dc.w $ffff
pub fn i_c18ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18EC8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C18EC8: ori.b #$a8, d3
pub fn i_c18ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18ECC;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xA8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18ECC: movep.w $434f(a0), d0
pub fn i_c18ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18ED0;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x434F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C18ED0: ?
pub fn i_c18ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18ED2;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18ED2: movea.l (a2), a0
pub fn i_c18ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18ED4;
    cpu.ir = 0x2052;
    let v2 = bus.read32(cpu.a[2]);
    cpu.a[0] = v2;
    12
}
/// $C18ED4: ?
pub fn i_c18ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18ED6;
    cpu.ir = 0x4154;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18ED6: dc.w $4520
pub fn i_c18ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18ED8;
    cpu.ir = 0x4520;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C18F2C: jsr $c10fae.l
pub fn i_c18f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC18F32);
    cpu.pc = 0xC10FAE;
    16
}
/// $C18F32: jsr $c1727e(pc)
pub fn i_c18f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC18F36);
    cpu.pc = 0xC1727E;
    16
}
/// $C18F36: move.b #$1, $bbf(a5)
pub fn i_c18f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F3C;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C18F3C: move.b d0, $3a001d.l
pub fn i_c18f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F42;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C18F42: move.w #$2700, sr
pub fn i_c18f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F46;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C18F46: move.w $9de(a5), d0
pub fn i_c18f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F4A;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C18F4A: subq.w #$1, d0
pub fn i_c18f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F4C;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C18F4C: mulu.w #$14, d0
pub fn i_c18f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F50;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0x14;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C18F50: movea.l a5, a0
pub fn i_c18f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F52;
    cpu.ir = 0x204D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C18F52: lea.l (a0, d0.w), a0
pub fn i_c18f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F56;
    cpu.ir = 0x41F0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $C18F56: move.l a0, $9e8(a5)
pub fn i_c18f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F5A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C18F5A: move.w (a0), d5
pub fn i_c18f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F5C;
    cpu.ir = 0x3A10;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C18F5C: move.b d5, $380021.l
pub fn i_c18f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F62;
    cpu.ir = 0x13C5;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x380021, v1 as u8);
    16
}
/// $C18F62: move.b d5, $3a001b.l
pub fn i_c18f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F68;
    cpu.ir = 0x13C5;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001B, v1 as u8);
    16
}
/// $C18F68: moveq #$0, d0
pub fn i_c18f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F6A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18F6A: move.b $c00401.l, d0
pub fn i_c18f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F70;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0xC00401) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C18F70: add.w d0, d0
pub fn i_c18f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F72;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18F72: add.w d0, d0
pub fn i_c18f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F74;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C18F74: addi.l #$116, d0
pub fn i_c18f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F7A;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x116, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $C18F7A: movea.l d0, a2
pub fn i_c18f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F7C;
    cpu.ir = 0x2440;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $C18F7C: movea.l (a2), a1
pub fn i_c18f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F7E;
    cpu.ir = 0x2252;
    let v2 = bus.read32(cpu.a[2]);
    cpu.a[1] = v2;
    12
}
/// $C18F7E: lea.l $10(a1), a1
pub fn i_c18f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F82;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x10);
    cpu.a[1] = ad1;
    16
}
/// $C18F82: lea.l $200(a5), a2
pub fn i_c18f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F86;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x200);
    cpu.a[2] = ad1;
    16
}
/// $C18F86: lea.l $900(a5), a6
pub fn i_c18f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F8A;
    cpu.ir = 0x4DED;
    let ad1 = cpu.a[5].wrapping_add(0x900);
    cpu.a[6] = ad1;
    16
}
/// $C18F8A: movea.l a2, a3
pub fn i_c18f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F8C;
    cpu.ir = 0x264A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[3] = v1;
    4
}
/// $C18F8C: moveq #$0, d7
pub fn i_c18f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F8E;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18F8E: move.l (a1)+, (a3)+
pub fn i_c18f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F90;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18F90: move.l (a1)+, (a3)+
pub fn i_c18f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F92;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18F92: move.l (a1)+, (a3)+
pub fn i_c18f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F94;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18F94: move.l (a1)+, (a3)+
pub fn i_c18f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F96;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18F96: move.w (a2)+, d0
pub fn i_c18f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F98;
    cpu.ir = 0x301A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C18F98: addq.w #$1, d0
pub fn i_c18f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F9A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C18F9A: beq.b $c18fac
pub fn i_c18f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F9C;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18FAC; return 10; }
    8
}
/// $C18F9C: move.l (a1)+, (a3)+
pub fn i_c18f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18F9E;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18F9E: move.l (a1)+, (a3)+
pub fn i_c18f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FA0;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18FA0: move.l (a1)+, (a3)+
pub fn i_c18fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FA2;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18FA2: move.w #$0, (a6)+
pub fn i_c18fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FA6;
    cpu.ir = 0x3CFC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18FA6: move.w #$0, (a6)+
pub fn i_c18fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FAA;
    cpu.ir = 0x3CFC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18FAA: addq.w #$1, d7
pub fn i_c18faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FAC;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C18FAC: move.w (a2)+, d0
pub fn i_c18fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FAE;
    cpu.ir = 0x301A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C18FAE: addq.w #$1, d0
pub fn i_c18fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FB0;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C18FB0: beq.b $c18fc2
pub fn i_c18fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FB2;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18FC2; return 10; }
    8
}
/// $C18FB2: move.l (a1)+, (a3)+
pub fn i_c18fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FB4;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18FB4: move.l (a1)+, (a3)+
pub fn i_c18fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FB6;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18FB6: move.l (a1)+, (a3)+
pub fn i_c18fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FB8;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18FB8: move.w #$1, (a6)+
pub fn i_c18fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FBC;
    cpu.ir = 0x3CFC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18FBC: move.w #$2, (a6)+
pub fn i_c18fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FC0;
    cpu.ir = 0x3CFC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18FC0: addq.w #$1, d7
pub fn i_c18fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FC2;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C18FC2: move.b (a2)+, d0
pub fn i_c18fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FC4;
    cpu.ir = 0x101A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C18FC4: addq.b #$1, d0
pub fn i_c18fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FC6;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C18FC6: beq.b $c18fd8
pub fn i_c18fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FC8;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18FD8; return 10; }
    8
}
/// $C18FC8: move.l (a1)+, (a3)+
pub fn i_c18fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FCA;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18FCA: move.l (a1)+, (a3)+
pub fn i_c18fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FCC;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18FCC: move.l (a1)+, (a3)+
pub fn i_c18fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FCE;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18FCE: move.w #$2, (a6)+
pub fn i_c18fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FD2;
    cpu.ir = 0x3CFC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18FD2: move.w #$4, (a6)+
pub fn i_c18fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FD6;
    cpu.ir = 0x3CFC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18FD6: addq.w #$1, d7
pub fn i_c18fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FD8;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C18FD8: move.b (a2)+, d0
pub fn i_c18fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FDA;
    cpu.ir = 0x101A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C18FDA: addq.b #$1, d0
pub fn i_c18fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FDC;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C18FDC: beq.b $c18fee
pub fn i_c18fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FDE;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC18FEE; return 10; }
    8
}
/// $C18FDE: move.l (a1)+, (a3)+
pub fn i_c18fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FE0;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18FE0: move.l (a1)+, (a3)+
pub fn i_c18fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FE2;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18FE2: move.l (a1)+, (a3)+
pub fn i_c18fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FE4;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18FE4: move.w #$3, (a6)+
pub fn i_c18fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FE8;
    cpu.ir = 0x3CFC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18FE8: move.w #$5, (a6)+
pub fn i_c18fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FEC;
    cpu.ir = 0x3CFC;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C18FEC: addq.w #$1, d7
pub fn i_c18fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FEE;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C18FEE: moveq #$4, d5
pub fn i_c18fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FF0;
    cpu.ir = 0x7A04;
    cpu.d[5] = 0x4;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18FF0: moveq #$6, d6
pub fn i_c18ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FF2;
    cpu.ir = 0x7C06;
    cpu.d[6] = 0x6;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C18FF2: move.b (a2)+, d0
pub fn i_c18ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FF4;
    cpu.ir = 0x101A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C18FF4: andi.b #$f, d0
pub fn i_c18ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FF8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C18FF8: beq.w $c1901c
pub fn i_c18ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FFC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1901C; return 10; }
    8
}
/// $C18FFC: move.l (a1)+, (a3)+
pub fn i_c18ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC18FFE;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C18FFE: move.l (a1)+, (a3)+
pub fn i_c18ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19000;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19000: move.l (a1)+, (a3)+
pub fn i_c19000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19002;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19002: move.l (a1)+, (a3)+
pub fn i_c19002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19004;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19004: move.l (a1)+, (a3)+
pub fn i_c19004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19006;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19006: move.l (a1)+, (a3)+
pub fn i_c19006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19008;
    cpu.ir = 0x26D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3]; cpu.a[3] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19008: subq.b #$1, d0
pub fn i_c19008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1900A;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1900A: bne.b $c19002
pub fn i_c1900a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1900C;
    cpu.ir = 0x66F6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19002; return 10; }
    8
}
/// $C1900C: move.w d5, (a6)+
pub fn i_c1900c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1900E;
    cpu.ir = 0x3CC5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1900E: move.w d6, (a6)+
pub fn i_c1900e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19010;
    cpu.ir = 0x3CC6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19010: addq.b #$1, d5
pub fn i_c19010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19012;
    cpu.ir = 0x5205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C19012: addq.b #$1, d6
pub fn i_c19012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19014;
    cpu.ir = 0x5206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C19014: addq.b #$1, d7
pub fn i_c19014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19016;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C19016: cmpi.b #$10, d6
pub fn i_c19016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1901A;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $C1901A: bne.b $c18ff2
pub fn i_c1901a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1901C;
    cpu.ir = 0x66D6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC18FF2; return 10; }
    8
}
/// $C1901C: move.b d5, $3a000b.l
pub fn i_c1901c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19022;
    cpu.ir = 0x13C5;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000B, v1 as u8);
    16
}
/// $C19022: move.w #$2000, sr
pub fn i_c19022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19026;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2000;
    write_sr(cpu, v1 as u16);
    12
}
/// $C19026: move.w d7, $9ec(a5)
pub fn i_c19026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1902A;
    cpu.ir = 0x3B47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9EC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1902A: lea.l $d08000.l, a4
pub fn i_c1902a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19030;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C19030: movea.l $9e8(a5), a0
pub fn i_c19030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19034;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C19034: move.w $2(a0), d0
pub fn i_c19034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19038;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19038: lsl.w #$4, d0
pub fn i_c19038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1903A;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C1903A: lea.l -$7de0(a4), a0
pub fn i_c1903a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1903E;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8220);
    cpu.a[0] = ad1;
    16
}
/// $C1903E: lea.l (a0, d0.w), a0
pub fn i_c1903e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19042;
    cpu.ir = 0x41F0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $C19042: lea.l $a84(a5), a1
pub fn i_c19042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19046;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xA84);
    cpu.a[1] = ad1;
    16
}
/// $C19046: move.l (a0)+, (a1)+
pub fn i_c19046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19048;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19048: move.l (a0)+, (a1)+
pub fn i_c19048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1904A;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1904A: move.l (a0)+, (a1)+
pub fn i_c1904a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1904C;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1904C: move.l (a0)+, (a1)+
pub fn i_c1904c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1904E;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1904E: clr.l $9e2(a5)
pub fn i_c1904e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19052;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C19052: clr.w $9ee(a5)
pub fn i_c19052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19056;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x9EE);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C19056: jsr $c12a0e(pc)
pub fn i_c19056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1905A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1905A);
    cpu.pc = 0xC12A0E;
    16
}
/// $C1905A: addq.b #$1, $ac2(a5)
pub fn i_c1905a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1905E;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1905E: lea.l $c00(a5), a0
pub fn i_c1905e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19062;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xC00);
    cpu.a[0] = ad1;
    16
}
/// $C19062: clr.l (a0)+
pub fn i_c19062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19064;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C19064: move.w #$3, (a0)+
pub fn i_c19064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19068;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19068: move.w #$7144, (a0)+
pub fn i_c19068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1906C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7144;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1906C: move.w #$108, (a0)+
pub fn i_c1906c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19070;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19070: movea.l $9e8(a5), a1
pub fn i_c19070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19074;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C19074: move.l $4(a1), (a0)+
pub fn i_c19074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19078;
    cpu.ir = 0x20E9;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C19078: move.l $8(a1), (a0)+
pub fn i_c19078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1907C;
    cpu.ir = 0x20E9;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C1907C: move.l $c(a1), (a0)+
pub fn i_c1907c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19080;
    cpu.ir = 0x20E9;
    let ad2 = cpu.a[1].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C19080: move.l $10(a1), (a0)+
pub fn i_c19080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19084;
    cpu.ir = 0x20E9;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C19084: move.w #$ff00, (a0)+
pub fn i_c19084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19088;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19088: move.w #$3, (a0)+
pub fn i_c19088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1908C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1908C: move.w #$70c7, (a0)+
pub fn i_c1908c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19090;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x70C7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19090: move.w $9ee(a5), d7
pub fn i_c19090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19094;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x9EE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19094: moveq #$7, d6
pub fn i_c19094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19096;
    cpu.ir = 0x7C07;
    cpu.d[6] = 0x7;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19096: cmp.w $9ec(a5), d7
pub fn i_c19096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1909A;
    cpu.ir = 0xBE6D;
    let ad2 = cpu.a[5].wrapping_add(0x9EC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C1909A: bcc.w $c19102
pub fn i_c1909a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1909E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC19102; return 10; }
    8
}
/// $C1909E: moveq #$0, d0
pub fn i_c1909e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190A0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C190A0: moveq #$0, d4
pub fn i_c190a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190A2;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C190A2: lea.l $200(a5), a2
pub fn i_c190a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190A6;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x200);
    cpu.a[2] = ad1;
    16
}
/// $C190A6: lea.l $10(a2), a3
pub fn i_c190a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190AA;
    cpu.ir = 0x47EA;
    let ad1 = cpu.a[2].wrapping_add(0x10);
    cpu.a[3] = ad1;
    16
}
/// $C190AA: cmp.b d0, d7
pub fn i_c190aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190AC;
    cpu.ir = 0xBE00;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C190AC: beq.w $c190e8
pub fn i_c190ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC190E8; return 10; }
    8
}
/// $C190B0: cmpi.b #$2, d4
pub fn i_c190b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190B4;
    cpu.ir = 0xC04;
    let v1 = cpu.d[4] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $C190B4: bcc.b $c190c2
pub fn i_c190b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190B6;
    cpu.ir = 0x640C;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC190C2; return 10; }
    8
}
/// $C190B6: move.w (a2)+, d1
pub fn i_c190b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190B8;
    cpu.ir = 0x321A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C190B8: addq.w #$1, d1
pub fn i_c190b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190BA;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C190BA: beq.b $c190e4
pub fn i_c190ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190BC;
    cpu.ir = 0x6728;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC190E4; return 10; }
    8
}
/// $C190BC: lea.l $c(a3), a3
pub fn i_c190bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190C0;
    cpu.ir = 0x47EB;
    let ad1 = cpu.a[3].wrapping_add(0xC);
    cpu.a[3] = ad1;
    16
}
/// $C190C0: bra.b $c190e2
pub fn i_c190c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190C2;
    cpu.ir = 0x6020;
    cpu.pc = 0xC190E2;
    10
}
/// $C190C2: cmpi.b #$4, d4
pub fn i_c190c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190C6;
    cpu.ir = 0xC04;
    let v1 = cpu.d[4] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $C190C6: bcc.b $c190d4
pub fn i_c190c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190C8;
    cpu.ir = 0x640C;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC190D4; return 10; }
    8
}
/// $C190C8: move.b (a2)+, d1
pub fn i_c190c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190CA;
    cpu.ir = 0x121A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C190CA: addq.b #$1, d1
pub fn i_c190ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190CC;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C190CC: beq.b $c190e4
pub fn i_c190cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190CE;
    cpu.ir = 0x6716;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC190E4; return 10; }
    8
}
/// $C190CE: lea.l $c(a3), a3
pub fn i_c190ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190D2;
    cpu.ir = 0x47EB;
    let ad1 = cpu.a[3].wrapping_add(0xC);
    cpu.a[3] = ad1;
    16
}
/// $C190D2: bra.b $c190e2
pub fn i_c190d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190D4;
    cpu.ir = 0x600E;
    cpu.pc = 0xC190E2;
    10
}
/// $C190D4: moveq #$f, d1
pub fn i_c190d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190D6;
    cpu.ir = 0x720F;
    cpu.d[1] = 0xF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C190D6: and.b (a2)+, d1
pub fn i_c190d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190D8;
    cpu.ir = 0xC21A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $C190D8: addq.b #$1, d1
pub fn i_c190d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190DA;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C190DA: mulu.w #$c, d1
pub fn i_c190da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190DE;
    cpu.ir = 0xC2FC;
    let v1: u32 = 0xC;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C190DE: lea.l (a3, d1.w), a3
pub fn i_c190de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190E2;
    cpu.ir = 0x47F3;
    let ad1 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    cpu.a[3] = ad1;
    18
}
/// $C190E2: addq.b #$1, d0
pub fn i_c190e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190E4;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C190E4: addq.b #$1, d4
pub fn i_c190e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190E6;
    cpu.ir = 0x5204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C190E6: bra.b $c190aa
pub fn i_c190e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190E8;
    cpu.ir = 0x60C2;
    cpu.pc = 0xC190AA;
    10
}
/// $C190E8: move.w #$108, (a0)+
pub fn i_c190e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190EC;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C190EC: move.l (a3)+, (a0)+
pub fn i_c190ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190EE;
    cpu.ir = 0x20DB;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C190EE: move.l (a3)+, (a0)+
pub fn i_c190ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190F0;
    cpu.ir = 0x20DB;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C190F0: move.l (a3)+, (a0)+
pub fn i_c190f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190F2;
    cpu.ir = 0x20DB;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C190F2: move.w #$ff00, (a0)+
pub fn i_c190f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190F6;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C190F6: move.l #$50002, (a0)+
pub fn i_c190f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190FC;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x50002;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C190FC: addq.b #$1, d7
pub fn i_c190fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC190FE;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C190FE: dbra d6, $c19096
pub fn i_c190fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19102;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC19102; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC19102; return 14; }
    cpu.pc = 0xC19096;
    10
}
/// $C19102: subq.l #$4, a0
pub fn i_c19102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19104;
    cpu.ir = 0x5988;
    cpu.a[0] = cpu.a[0].wrapping_sub(4);
    8
}
/// $C19104: clr.w (a0)+
pub fn i_c19104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19106;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C19106: move.l a0, $abe(a5)
pub fn i_c19106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1910A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1910A: subq.b #$1, $ac2(a5)
pub fn i_c1910a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1910E;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1910E: jsr $c17290(pc)
pub fn i_c1910e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19112;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19112);
    cpu.pc = 0xC17290;
    16
}
/// $C19112: clr.l $a00(a5)
pub fn i_c19112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19116;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0xA00);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C19116: moveq #$7, d7
pub fn i_c19116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19118;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19118: move.w $9ee(a5), $9e2(a5)
pub fn i_c19118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1911E;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9EE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9E2);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1911E: move.w $9e2(a5), d0
pub fn i_c1911e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19122;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19122: cmp.w $9ec(a5), d0
pub fn i_c19122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19126;
    cpu.ir = 0xB06D;
    let ad2 = cpu.a[5].wrapping_add(0x9EC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C19126: bcc.w $c19140
pub fn i_c19126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1912A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC19140; return 10; }
    8
}
/// $C1912A: jsr $c192e8(pc)
pub fn i_c1912a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1912E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1912E);
    cpu.pc = 0xC192E8;
    16
}
/// $C1912E: addq.w #$1, $9e2(a5)
pub fn i_c1912e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19132;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C19132: dbra d7, $c1911e
pub fn i_c19132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19136;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC19136; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC19136; return 14; }
    cpu.pc = 0xC1911E;
    10
}
/// $C19136: jsr $c17290(pc)
pub fn i_c19136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1913A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1913A);
    cpu.pc = 0xC17290;
    16
}
/// $C1913A: move.w $9e4(a5), $9e2(a5)
pub fn i_c1913a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19140;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9E4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9E2);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C19140: move.w $9e4(a5), $9e2(a5)
pub fn i_c19140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19146;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9E4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9E2);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C19146: jsr $c194f8(pc)
pub fn i_c19146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1914A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1914A);
    cpu.pc = 0xC194F8;
    16
}
/// $C1914A: jsr $c1727e(pc)
pub fn i_c1914a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1914E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1914E);
    cpu.pc = 0xC1727E;
    16
}
/// $C1914E: move.b $a97(a5), d0
pub fn i_c1914e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19152;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C19152: move.b d0, d1
pub fn i_c19152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19154;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C19154: btst.b #$6, d0
pub fn i_c19154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19158;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C19158: bne.w $c19544
pub fn i_c19158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1915C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19544; return 10; }
    8
}
/// $C1915C: andi.b #$3, d0
pub fn i_c1915c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19160;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19160: beq.w $c191ca
pub fn i_c19160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19164;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC191CA; return 10; }
    8
}
/// $C19164: moveq #$1, d1
pub fn i_c19164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19166;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19166: andi.b #$2, d0
pub fn i_c19166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1916A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1916A: bne.b $c1916e
pub fn i_c1916a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1916C;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1916E; return 10; }
    8
}
/// $C1916C: moveq #$ff, d1
pub fn i_c1916c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1916E;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1916E: add.w $9e2(a5), d1
pub fn i_c1916e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19172;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C19172: cmp.w $9ec(a5), d1
pub fn i_c19172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19176;
    cpu.ir = 0xB26D;
    let ad2 = cpu.a[5].wrapping_add(0x9EC);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C19176: bcc.b $c1914a
pub fn i_c19176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19178;
    cpu.ir = 0x64D2;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1914A; return 10; }
    8
}
/// $C19178: move.w d1, $9e4(a5)
pub fn i_c19178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1917C;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1917C: move.w $9e2(a5), d0
pub fn i_c1917c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19180;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19180: eor.w d1, d0
pub fn i_c19180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19182;
    cpu.ir = 0xB340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C19182: andi.b #$8, d0
pub fn i_c19182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19186;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19186: beq.b $c19194
pub fn i_c19186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19188;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19194; return 10; }
    8
}
/// $C19188: andi.b #$f8, d1
pub fn i_c19188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1918C;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xF8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1918C: move.w d1, $9ee(a5)
pub fn i_c1918c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19190;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9EE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C19190: bra.w $c19056
pub fn i_c19190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19194;
    cpu.ir = 0x6000;
    cpu.pc = 0xC19056;
    10
}
/// $C19194: lea.l $900(a5), a6
pub fn i_c19194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19198;
    cpu.ir = 0x4DED;
    let ad1 = cpu.a[5].wrapping_add(0x900);
    cpu.a[6] = ad1;
    16
}
/// $C19198: move.w $9e2(a5), d0
pub fn i_c19198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1919C;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1919C: add.w d0, d0
pub fn i_c1919c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1919E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1919E: add.w d0, d0
pub fn i_c1919e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191A0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C191A0: cmpi.w #$2, (a6, d0.w)
pub fn i_c191a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191A6;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    18
}
/// $C191A6: bcc.b $c191ac
pub fn i_c191a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191A8;
    cpu.ir = 0x6404;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC191AC; return 10; }
    8
}
/// $C191A8: jsr $c192e8(pc)
pub fn i_c191a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC191AC);
    cpu.pc = 0xC192E8;
    16
}
/// $C191AC: jsr $c194f8(pc)
pub fn i_c191ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC191B0);
    cpu.pc = 0xC194F8;
    16
}
/// $C191B0: lea.l $900(a5), a6
pub fn i_c191b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191B4;
    cpu.ir = 0x4DED;
    let ad1 = cpu.a[5].wrapping_add(0x900);
    cpu.a[6] = ad1;
    16
}
/// $C191B4: move.w $9e2(a5), d0
pub fn i_c191b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191B8;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C191B8: add.w d0, d0
pub fn i_c191b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191BA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C191BA: add.w d0, d0
pub fn i_c191ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191BC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C191BC: cmpi.w #$2, (a6, d0.w)
pub fn i_c191bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191C2;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    18
}
/// $C191C2: bcc.b $c191c8
pub fn i_c191c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191C4;
    cpu.ir = 0x6404;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC191C8; return 10; }
    8
}
/// $C191C4: jsr $c192e8(pc)
pub fn i_c191c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC191C8);
    cpu.pc = 0xC192E8;
    16
}
/// $C191C8: bra.b $c1914a
pub fn i_c191c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191CA;
    cpu.ir = 0x6080;
    cpu.pc = 0xC1914A;
    10
}
/// $C191CA: move.w $9e2(a5), d6
pub fn i_c191ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191CE;
    cpu.ir = 0x3C2D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C191CE: lea.l $a84(a5), a1
pub fn i_c191ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191D2;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xA84);
    cpu.a[1] = ad1;
    16
}
/// $C191D2: lea.l $900(a5), a6
pub fn i_c191d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191D6;
    cpu.ir = 0x4DED;
    let ad1 = cpu.a[5].wrapping_add(0x900);
    cpu.a[6] = ad1;
    16
}
/// $C191D6: add.w d6, d6
pub fn i_c191d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191D8;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C191D8: add.w d6, d6
pub fn i_c191d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191DA;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C191DA: move.w (a6, d6.w), d5
pub fn i_c191da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191DE;
    cpu.ir = 0x3A36;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C191DE: cmpi.w #$2, d5
pub fn i_c191de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191E2;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $C191E2: bcc.w $c191fc
pub fn i_c191e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC191FC; return 10; }
    8
}
/// $C191E6: andi.b #$c, d1
pub fn i_c191e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191EA;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C191EA: beq.w $c191fc
pub fn i_c191ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191EE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC191FC; return 10; }
    8
}
/// $C191EE: bchg.b #$0, $a01(a5)
pub fn i_c191ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191F4;
    cpu.ir = 0x86D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[5].wrapping_add(0xA01);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $C191F4: jsr $c192e8(pc)
pub fn i_c191f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC191F8);
    cpu.pc = 0xC192E8;
    16
}
/// $C191F8: bra.w $c1914a
pub fn i_c191f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC191FC;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1914A;
    10
}
/// $C191FC: move.b $a98(a5), d0
pub fn i_c191fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19200;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C19200: andi.b #$30, d0
pub fn i_c19200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19204;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19204: beq.w $c1914a
pub fn i_c19204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19208;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1914A; return 10; }
    8
}
/// $C19208: cmpi.w #$2, d5
pub fn i_c19208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1920C;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $C1920C: bcs.w $c19252
pub fn i_c1920c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19210;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC19252; return 10; }
    8
}
/// $C19210: beq.w $c192a2
pub fn i_c19210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19214;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC192A2; return 10; }
    8
}
/// $C19214: cmpi.w #$3, d5
pub fn i_c19214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19218;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $C19218: beq.w $c192c6
pub fn i_c19218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1921C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC192C6; return 10; }
    8
}
/// $C1921C: moveq #$1, d1
pub fn i_c1921c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1921E;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1921E: andi.b #$10, d0
pub fn i_c1921e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19222;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19222: bne.b $c19226
pub fn i_c19222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19224;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19226; return 10; }
    8
}
/// $C19224: moveq #$ff, d1
pub fn i_c19224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19226;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19226: move.w $2(a6, d6.w), d3
pub fn i_c19226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1922A;
    cpu.ir = 0x3636;
    let ad2 = cpu.a[6].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1922A: lea.l $200(a5), a2
pub fn i_c1922a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1922E;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x200);
    cpu.a[2] = ad1;
    16
}
/// $C1922E: move.b (a2, d3.w), d0
pub fn i_c1922e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19232;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C19232: andi.b #$f, d0
pub fn i_c19232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19236;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19236: add.b (a1, d3.w), d1
pub fn i_c19236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1923A;
    cpu.ir = 0xD231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $C1923A: bpl.b $c19240
pub fn i_c1923a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1923C;
    cpu.ir = 0x6A04;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC19240; return 10; }
    8
}
/// $C1923C: move.b d0, d1
pub fn i_c1923c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1923E;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1923E: subq.b #$1, d1
pub fn i_c1923e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19240;
    cpu.ir = 0x5301;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C19240: cmp.b d0, d1
pub fn i_c19240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19242;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C19242: bcs.b $c19246
pub fn i_c19242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19244;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC19246; return 10; }
    8
}
/// $C19244: moveq #$0, d1
pub fn i_c19244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19246;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19246: move.b d1, (a1, d3.w)
pub fn i_c19246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1924A;
    cpu.ir = 0x1381;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C1924A: jsr $c192e8(pc)
pub fn i_c1924a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1924E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1924E);
    cpu.pc = 0xC192E8;
    16
}
/// $C1924E: bra.w $c1914a
pub fn i_c1924e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19252;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1914A;
    10
}
/// $C19252: move.w $2(a6, d6.w), d6
pub fn i_c19252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19256;
    cpu.ir = 0x3C36;
    let ad2 = cpu.a[6].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C19256: moveq #$1, d1
pub fn i_c19256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19258;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19258: andi.b #$10, d0
pub fn i_c19258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1925C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1925C: bne.b $c19262
pub fn i_c1925c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1925E;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19262; return 10; }
    8
}
/// $C1925E: move.b #$99, d1
pub fn i_c1925e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19262;
    cpu.ir = 0x123C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C19262: tst.w $a00(a5)
pub fn i_c19262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19266;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $C19266: bne.w $c19280
pub fn i_c19266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1926A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19280; return 10; }
    8
}
/// $C1926A: move.b (a1, d6.w), d0
pub fn i_c1926a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1926E;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C1926E: sub.b d2, d2
pub fn i_c1926e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19270;
    cpu.ir = 0x9402;
    let v1 = cpu.d[2] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    4
}
/// $C19270: abcd.b d1, d0
pub fn i_c19270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19272;
    cpu.ir = 0xC101;
    let s = cpu.d[1] as u8; let d = cpu.d[0] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[0] = (cpu.d[0] & !0xFF) | r as u32;
    6
}
/// $C19272: cmpi.b #$30, d0
pub fn i_c19272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19276;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $C19276: bcc.b $c1926e
pub fn i_c19276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19278;
    cpu.ir = 0x64F6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1926E; return 10; }
    8
}
/// $C19278: move.b d0, (a1, d6.w)
pub fn i_c19278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1927C;
    cpu.ir = 0x1380;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C1927C: bra.w $c1929a
pub fn i_c1927c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19280;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1929A;
    10
}
/// $C19280: move.b $1(a1, d6.w), d0
pub fn i_c19280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19284;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x1).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C19284: sub.b d2, d2
pub fn i_c19284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19286;
    cpu.ir = 0x9402;
    let v1 = cpu.d[2] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    4
}
/// $C19286: abcd.b d1, d0
pub fn i_c19286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19288;
    cpu.ir = 0xC101;
    let s = cpu.d[1] as u8; let d = cpu.d[0] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[0] = (cpu.d[0] & !0xFF) | r as u32;
    6
}
/// $C19288: bne.b $c19290
pub fn i_c19288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1928A;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19290; return 10; }
    8
}
/// $C1928A: tst.b (a1, d6.w)
pub fn i_c1928a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1928E;
    cpu.ir = 0x4A31;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $C1928E: beq.b $c19284
pub fn i_c1928e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19290;
    cpu.ir = 0x67F4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19284; return 10; }
    8
}
/// $C19290: cmpi.b #$60, d0
pub fn i_c19290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19294;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x60, Size::Byte, false);
    8
}
/// $C19294: bcc.b $c19284
pub fn i_c19294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19296;
    cpu.ir = 0x64EE;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC19284; return 10; }
    8
}
/// $C19296: move.b d0, $1(a1, d6.w)
pub fn i_c19296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1929A;
    cpu.ir = 0x1380;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x1).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $C1929A: jsr $c192e8(pc)
pub fn i_c1929a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1929E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1929E);
    cpu.pc = 0xC192E8;
    16
}
/// $C1929E: bra.w $c1914a
pub fn i_c1929e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192A2;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1914A;
    10
}
/// $C192A2: moveq #$1, d1
pub fn i_c192a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192A4;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C192A4: andi.b #$10, d0
pub fn i_c192a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192A8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C192A8: bne.b $c192ac
pub fn i_c192a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192AA;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC192AC; return 10; }
    8
}
/// $C192AA: moveq #$ff, d1
pub fn i_c192aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192AC;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C192AC: move.b $4(a1), d0
pub fn i_c192ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192B0;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C192B0: add.b d1, d0
pub fn i_c192b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192B2;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C192B2: beq.b $c192b0
pub fn i_c192b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192B4;
    cpu.ir = 0x67FC;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC192B0; return 10; }
    8
}
/// $C192B4: cmpi.b #$64, d0
pub fn i_c192b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192B8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x64, Size::Byte, false);
    8
}
/// $C192B8: bcc.b $c192b0
pub fn i_c192b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192BA;
    cpu.ir = 0x64F6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC192B0; return 10; }
    8
}
/// $C192BA: move.b d0, $4(a1)
pub fn i_c192ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192BE;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C192BE: jsr $c192e8(pc)
pub fn i_c192be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC192C2);
    cpu.pc = 0xC192E8;
    16
}
/// $C192C2: bra.w $c1914a
pub fn i_c192c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192C6;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1914A;
    10
}
/// $C192C6: moveq #$1, d1
pub fn i_c192c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192C8;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C192C8: andi.b #$10, d0
pub fn i_c192c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192CC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C192CC: bne.b $c192d0
pub fn i_c192cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192CE;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC192D0; return 10; }
    8
}
/// $C192CE: moveq #$ff, d1
pub fn i_c192ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192D0;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C192D0: move.b $5(a1), d0
pub fn i_c192d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192D4;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C192D4: add.b d1, d0
pub fn i_c192d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192D6;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C192D6: cmpi.b #$65, d0
pub fn i_c192d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192DA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x65, Size::Byte, false);
    8
}
/// $C192DA: bcc.b $c192d4
pub fn i_c192da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192DC;
    cpu.ir = 0x64F8;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC192D4; return 10; }
    8
}
/// $C192DC: move.b d0, $5(a1)
pub fn i_c192dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192E0;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C192E0: jsr $c192e8(pc)
pub fn i_c192e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC192E4);
    cpu.pc = 0xC192E8;
    16
}
/// $C192E4: bra.w $c1914a
pub fn i_c192e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192E8;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1914A;
    10
}
/// $C192E8: addq.b #$1, $ac2(a5)
pub fn i_c192e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192EC;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C192EC: movea.l $abe(a5), a0
pub fn i_c192ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192F0;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C192F0: clr.l (a0)+
pub fn i_c192f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192F2;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C192F2: move.w #$3, (a0)+
pub fn i_c192f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192F6;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C192F6: move.w $9e2(a5), d6
pub fn i_c192f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192FA;
    cpu.ir = 0x3C2D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C192FA: move.w d6, d0
pub fn i_c192fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192FC;
    cpu.ir = 0x3006;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C192FC: add.w d6, d6
pub fn i_c192fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC192FE;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C192FE: add.w d6, d6
pub fn i_c192fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19300;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19300: lea.l $900(a5), a6
pub fn i_c19300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19304;
    cpu.ir = 0x4DED;
    let ad1 = cpu.a[5].wrapping_add(0x900);
    cpu.a[6] = ad1;
    16
}
/// $C19304: move.w (a6, d6.w), d5
pub fn i_c19304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19308;
    cpu.ir = 0x3A36;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C19308: move.w $2(a6, d6.w), d6
pub fn i_c19308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1930C;
    cpu.ir = 0x3C36;
    let ad2 = cpu.a[6].wrapping_add(0x2).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $C1930C: andi.w #$7, d0
pub fn i_c1930c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19310;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C19310: mulu.w #$3, d0
pub fn i_c19310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19314;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0x3;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C19314: addi.w #$7287, d0
pub fn i_c19314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19318;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7287, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C19318: move.w d0, (a0)+
pub fn i_c19318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1931A;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1931A: move.w #$308, (a0)+
pub fn i_c1931a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1931E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x308;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1931E: moveq #$0, d0
pub fn i_c1931e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19320;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19320: lea.l $200(a5), a2
pub fn i_c19320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19324;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x200);
    cpu.a[2] = ad1;
    16
}
/// $C19324: lea.l $10(a2), a3
pub fn i_c19324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19328;
    cpu.ir = 0x47EA;
    let ad1 = cpu.a[2].wrapping_add(0x10);
    cpu.a[3] = ad1;
    16
}
/// $C19328: cmp.b d0, d5
pub fn i_c19328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1932A;
    cpu.ir = 0xBA00;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $C1932A: beq.w $c19364
pub fn i_c1932a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1932E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19364; return 10; }
    8
}
/// $C1932E: cmpi.b #$2, d0
pub fn i_c1932e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19332;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $C19332: bcc.b $c19340
pub fn i_c19332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19334;
    cpu.ir = 0x640C;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC19340; return 10; }
    8
}
/// $C19334: move.w (a2)+, d1
pub fn i_c19334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19336;
    cpu.ir = 0x321A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $C19336: addq.w #$1, d1
pub fn i_c19336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19338;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C19338: beq.b $c19360
pub fn i_c19338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1933A;
    cpu.ir = 0x6726;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19360; return 10; }
    8
}
/// $C1933A: lea.l $c(a3), a3
pub fn i_c1933a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1933E;
    cpu.ir = 0x47EB;
    let ad1 = cpu.a[3].wrapping_add(0xC);
    cpu.a[3] = ad1;
    16
}
/// $C1933E: bra.b $c19360
pub fn i_c1933e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19340;
    cpu.ir = 0x6020;
    cpu.pc = 0xC19360;
    10
}
/// $C19340: cmpi.b #$4, d0
pub fn i_c19340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19344;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $C19344: bcc.b $c19352
pub fn i_c19344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19346;
    cpu.ir = 0x640C;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC19352; return 10; }
    8
}
/// $C19346: move.b (a2)+, d1
pub fn i_c19346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19348;
    cpu.ir = 0x121A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C19348: addq.b #$1, d1
pub fn i_c19348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1934A;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1934A: beq.b $c19360
pub fn i_c1934a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1934C;
    cpu.ir = 0x6714;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19360; return 10; }
    8
}
/// $C1934C: lea.l $c(a3), a3
pub fn i_c1934c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19350;
    cpu.ir = 0x47EB;
    let ad1 = cpu.a[3].wrapping_add(0xC);
    cpu.a[3] = ad1;
    16
}
/// $C19350: bra.b $c19360
pub fn i_c19350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19352;
    cpu.ir = 0x600E;
    cpu.pc = 0xC19360;
    10
}
/// $C19352: moveq #$f, d1
pub fn i_c19352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19354;
    cpu.ir = 0x720F;
    cpu.d[1] = 0xF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19354: and.b (a2)+, d1
pub fn i_c19354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19356;
    cpu.ir = 0xC21A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $C19356: addq.b #$1, d1
pub fn i_c19356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19358;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C19358: mulu.w #$c, d1
pub fn i_c19358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1935C;
    cpu.ir = 0xC2FC;
    let v1: u32 = 0xC;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C1935C: lea.l (a3, d1.w), a3
pub fn i_c1935c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19360;
    cpu.ir = 0x47F3;
    let ad1 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    cpu.a[3] = ad1;
    18
}
/// $C19360: addq.b #$1, d0
pub fn i_c19360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19362;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C19362: bra.b $c19328
pub fn i_c19362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19364;
    cpu.ir = 0x60C4;
    cpu.pc = 0xC19328;
    10
}
/// $C19364: lea.l $a84(a5), a2
pub fn i_c19364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19368;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0xA84);
    cpu.a[2] = ad1;
    16
}
/// $C19368: moveq #$0, d0
pub fn i_c19368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1936A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1936A: cmpi.w #$6, d6
pub fn i_c1936a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1936E;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $C1936E: bcs.w $c19394
pub fn i_c1936e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19372;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC19394; return 10; }
    8
}
/// $C19372: move.b (a2, d6.w), d0
pub fn i_c19372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19376;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C19376: mulu.w #$c, d0
pub fn i_c19376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1937A;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0xC;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C1937A: lea.l $c(a3, d0.w), a3
pub fn i_c1937a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1937E;
    cpu.ir = 0x47F3;
    let ad1 = cpu.a[3].wrapping_add(0xC).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[3] = ad1;
    18
}
/// $C1937E: move.l (a3)+, (a0)+
pub fn i_c1937e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19380;
    cpu.ir = 0x20DB;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19380: move.l (a3)+, (a0)+
pub fn i_c19380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19382;
    cpu.ir = 0x20DB;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19382: move.l (a3)+, (a0)+
pub fn i_c19382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19384;
    cpu.ir = 0x20DB;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19384: move.w #$ff00, (a0)+
pub fn i_c19384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19388;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19388: clr.w (a0)+
pub fn i_c19388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1938A;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1938A: move.l a0, $abe(a5)
pub fn i_c1938a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1938E;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1938E: subq.b #$1, $ac2(a5)
pub fn i_c1938e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19392;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19392: rts
pub fn i_c19392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19394;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19394: cmpi.w #$4, d6
pub fn i_c19394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19398;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x4, Size::Word, false);
    8
}
/// $C19398: bcs.w $c1947a
pub fn i_c19398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1939C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1947A; return 10; }
    8
}
/// $C1939C: beq.w $c19452
pub fn i_c1939c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19452; return 10; }
    8
}
/// $C193A0: move.b (a2, d6.w), d0
pub fn i_c193a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193A4;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C193A4: beq.w $c19406
pub fn i_c193a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19406; return 10; }
    8
}
/// $C193A8: cmpi.b #$64, d0
pub fn i_c193a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193AC;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x64, Size::Byte, false);
    8
}
/// $C193AC: beq.w $c1942c
pub fn i_c193ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1942C; return 10; }
    8
}
/// $C193B0: move.b d0, d2
pub fn i_c193b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193B2;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C193B2: divu.w #$a, d0
pub fn i_c193b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193B6;
    cpu.ir = 0x80FC;
    let v1: u32 = 0xA;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $C193B6: move.l d0, d1
pub fn i_c193b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193B8;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C193B8: addi.b #$30, d0
pub fn i_c193b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193BC;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C193BC: cmpi.b #$30, d0
pub fn i_c193bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193C0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $C193C0: bne.b $c193c6
pub fn i_c193c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193C2;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC193C6; return 10; }
    8
}
/// $C193C2: move.b #$20, d0
pub fn i_c193c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193C6;
    cpu.ir = 0x103C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C193C6: move.b d0, (a0)+
pub fn i_c193c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193C8;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C193C8: swap d1
pub fn i_c193c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193CA;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C193CA: addi.b #$30, d1
pub fn i_c193ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193CE;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C193CE: move.b d1, (a0)+
pub fn i_c193ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193D0;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C193D0: move.b #$20, (a0)+
pub fn i_c193d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193D4;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C193D4: move.b #$74, (a0)+
pub fn i_c193d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193D8;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C193D8: move.b #$69, (a0)+
pub fn i_c193d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193DC;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x69;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C193DC: move.b #$6d, (a0)+
pub fn i_c193dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193E0;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x6D;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C193E0: move.b #$65, (a0)+
pub fn i_c193e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193E4;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x65;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C193E4: move.b #$73, (a0)+
pub fn i_c193e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193E8;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x73;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C193E8: subq.b #$1, d2
pub fn i_c193e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193EA;
    cpu.ir = 0x5302;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C193EA: bne.b $c193f2
pub fn i_c193ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193EC;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC193F2; return 10; }
    8
}
/// $C193EC: move.b #$20, -$1(a0)
pub fn i_c193ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193F2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C193F2: move.b #$20, (a0)+
pub fn i_c193f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193F6;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C193F6: move.b #$ff, (a0)+
pub fn i_c193f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193FA;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C193FA: clr.w (a0)+
pub fn i_c193fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC193FC;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C193FC: move.l a0, $abe(a5)
pub fn i_c193fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19400;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19400: subq.b #$1, $ac2(a5)
pub fn i_c19400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19404;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19404: rts
pub fn i_c19404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19406;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19406: move.b #$57, (a0)+
pub fn i_c19406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1940A;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x57;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1940A: move.b #$49, (a0)+
pub fn i_c1940a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1940E;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x49;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1940E: move.b #$54, (a0)+
pub fn i_c1940e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19412;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x54;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19412: move.b #$48, (a0)+
pub fn i_c19412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19416;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19416: move.b #$4f, (a0)+
pub fn i_c19416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1941A;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x4F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1941A: move.b #$55, (a0)+
pub fn i_c1941a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1941E;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x55;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1941E: move.b #$54, (a0)+
pub fn i_c1941e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19422;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x54;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19422: move.b #$20, (a0)+
pub fn i_c19422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19426;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19426: move.b #$20, (a0)+
pub fn i_c19426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1942A;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1942A: bra.b $c193f6
pub fn i_c1942a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1942C;
    cpu.ir = 0x60CA;
    cpu.pc = 0xC193F6;
    10
}
/// $C1942C: move.b #$49, (a0)+
pub fn i_c1942c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19430;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x49;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19430: move.b #$4e, (a0)+
pub fn i_c19430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19434;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x4E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19434: move.b #$46, (a0)+
pub fn i_c19434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19438;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x46;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19438: move.b #$49, (a0)+
pub fn i_c19438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1943C;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x49;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1943C: move.b #$4e, (a0)+
pub fn i_c1943c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19440;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x4E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19440: move.b #$49, (a0)+
pub fn i_c19440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19444;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x49;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19444: move.b #$54, (a0)+
pub fn i_c19444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19448;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x54;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19448: move.b #$45, (a0)+
pub fn i_c19448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1944C;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x45;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1944C: move.b #$20, (a0)+
pub fn i_c1944c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19450;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19450: bra.b $c193f6
pub fn i_c19450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19452;
    cpu.ir = 0x60A4;
    cpu.pc = 0xC193F6;
    10
}
/// $C19452: move.b (a2, d6.w), d0
pub fn i_c19452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19456;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C19456: divu.w #$a, d0
pub fn i_c19456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1945A;
    cpu.ir = 0x80FC;
    let v1: u32 = 0xA;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $C1945A: move.l d0, d1
pub fn i_c1945a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1945C;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1945C: addi.b #$30, d0
pub fn i_c1945c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19460;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19460: cmpi.b #$30, d0
pub fn i_c19460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19464;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $C19464: bne.b $c1946a
pub fn i_c19464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19466;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1946A; return 10; }
    8
}
/// $C19466: move.b #$20, d0
pub fn i_c19466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1946A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1946A: move.b d0, (a0)+
pub fn i_c1946a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1946C;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1946C: swap d1
pub fn i_c1946c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1946E;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1946E: addi.b #$30, d1
pub fn i_c1946e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19472;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19472: move.b d1, (a0)+
pub fn i_c19472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19474;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19474: move.w #$ff00, (a0)+
pub fn i_c19474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19478;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19478: bra.b $c193fa
pub fn i_c19478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1947A;
    cpu.ir = 0x6080;
    cpu.pc = 0xC193FA;
    10
}
/// $C1947A: move.b (a2, d6.w), d0
pub fn i_c1947a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1947E;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C1947E: moveq #$0, d2
pub fn i_c1947e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19480;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19480: jsr $c194d4(pc)
pub fn i_c19480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19484;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19484);
    cpu.pc = 0xC194D4;
    16
}
/// $C19484: move.l #$5003f, (a0)+
pub fn i_c19484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1948A;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x5003F;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1948A: move.w #$108, (a0)+
pub fn i_c1948a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1948E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1948E: move.b #$6d, (a0)+
pub fn i_c1948e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19492;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x6D;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19492: move.b #$69, (a0)+
pub fn i_c19492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19496;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x69;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19496: move.b #$6e, (a0)+
pub fn i_c19496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1949A;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x6E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1949A: move.b #$20, (a0)+
pub fn i_c1949a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1949E;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1949E: move.w #$ff00, (a0)+
pub fn i_c1949e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194A2;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C194A2: move.l #$5007f, (a0)+
pub fn i_c194a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194A8;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x5007F;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C194A8: move.w #$108, (a0)+
pub fn i_c194a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194AC;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C194AC: move.b $1(a2, d6.w), d0
pub fn i_c194ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194B0;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x1).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C194B0: moveq #$1, d2
pub fn i_c194b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194B2;
    cpu.ir = 0x7401;
    cpu.d[2] = 0x1;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C194B2: jsr $c194d4(pc)
pub fn i_c194b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC194B6);
    cpu.pc = 0xC194D4;
    16
}
/// $C194B6: move.l #$5003f, (a0)+
pub fn i_c194b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194BC;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x5003F;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C194BC: move.w #$108, (a0)+
pub fn i_c194bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194C0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C194C0: move.b #$73, (a0)+
pub fn i_c194c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194C4;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x73;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C194C4: move.b #$65, (a0)+
pub fn i_c194c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194C8;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x65;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C194C8: move.b #$63, (a0)+
pub fn i_c194c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194CC;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x63;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C194CC: move.b #$ff, (a0)+
pub fn i_c194cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194D0;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C194D0: bra.w $c193fa
pub fn i_c194d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194D4;
    cpu.ir = 0x6000;
    cpu.pc = 0xC193FA;
    10
}
/// $C194D4: move.w $9e2(a5), d1
pub fn i_c194d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194D8;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C194D8: cmp.w $9e4(a5), d1
pub fn i_c194d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194DC;
    cpu.ir = 0xB26D;
    let ad2 = cpu.a[5].wrapping_add(0x9E4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C194DC: bne.b $c194ea
pub fn i_c194dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194DE;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC194EA; return 10; }
    8
}
/// $C194DE: cmp.w $a00(a5), d2
pub fn i_c194de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194E2;
    cpu.ir = 0xB46D;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C194E2: bne.b $c194ea
pub fn i_c194e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194E4;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC194EA; return 10; }
    8
}
/// $C194E4: move.w #$1108, -$2(a0)
pub fn i_c194e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194EA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C194EA: jsr $c16f54(pc)
pub fn i_c194ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC194EE);
    cpu.pc = 0xC16F54;
    16
}
/// $C194EE: move.b d0, (a0)+
pub fn i_c194ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194F0;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C194F0: move.b d1, (a0)+
pub fn i_c194f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194F2;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C194F2: move.w #$ff00, (a0)+
pub fn i_c194f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194F6;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C194F6: rts
pub fn i_c194f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C194F8: addq.b #$1, $ac2(a5)
pub fn i_c194f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC194FC;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C194FC: movea.l $abe(a5), a0
pub fn i_c194fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19500;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C19500: clr.l (a0)+
pub fn i_c19500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19502;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C19502: move.w #$20ff, d1
pub fn i_c19502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19506;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19506: jsr $c19524(pc)
pub fn i_c19506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1950A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1950A);
    cpu.pc = 0xC19524;
    16
}
/// $C1950A: move.w $9e4(a5), $9e2(a5)
pub fn i_c1950a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19510;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9E4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9E2);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C19510: move.w #$11ff, d1
pub fn i_c19510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19514;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19514: jsr $c19524(pc)
pub fn i_c19514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19518;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19518);
    cpu.pc = 0xC19524;
    16
}
/// $C19518: clr.w (a0)+
pub fn i_c19518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1951A;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1951A: move.l a0, $abe(a5)
pub fn i_c1951a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1951E;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1951E: subq.b #$1, $ac2(a5)
pub fn i_c1951e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19522;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19522: rts
pub fn i_c19522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19524;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19524: move.w #$3, (a0)+
pub fn i_c19524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19528;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19528: move.w $9e2(a5), d0
pub fn i_c19528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1952C;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1952C: andi.b #$7, d0
pub fn i_c1952c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19530;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19530: move.w d0, d2
pub fn i_c19530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19532;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19532: add.w d0, d0
pub fn i_c19532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19534;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19534: add.w d0, d2
pub fn i_c19534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19536;
    cpu.ir = 0xD440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19536: addi.w #$7067, d2
pub fn i_c19536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1953A;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7067, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1953A: move.w d2, (a0)+
pub fn i_c1953a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1953C;
    cpu.ir = 0x30C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1953C: move.w #$1108, (a0)+
pub fn i_c1953c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19540;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19540: move.w d1, (a0)+
pub fn i_c19540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19542;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19542: rts
pub fn i_c19542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19544;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19544: move.b d0, $3a001d.l
pub fn i_c19544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1954A;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C1954A: lea.l $d08000.l, a4
pub fn i_c1954a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19550;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C19550: movea.l $9e8(a5), a0
pub fn i_c19550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19554;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C19554: move.w $2(a0), d0
pub fn i_c19554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19558;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19558: lsl.w #$4, d0
pub fn i_c19558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1955A;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C1955A: lea.l -$7de0(a4), a0
pub fn i_c1955a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1955E;
    cpu.ir = 0x41EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8220);
    cpu.a[0] = ad1;
    16
}
/// $C1955E: lea.l (a0, d0.w), a0
pub fn i_c1955e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19562;
    cpu.ir = 0x41F0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $C19562: lea.l $a84(a5), a1
pub fn i_c19562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19566;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xA84);
    cpu.a[1] = ad1;
    16
}
/// $C19566: move.l (a1)+, (a0)+
pub fn i_c19566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19568;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19568: move.l (a1)+, (a0)+
pub fn i_c19568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1956A;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1956A: move.l (a1)+, (a0)+
pub fn i_c1956a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1956C;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1956C: move.l (a1)+, (a0)+
pub fn i_c1956c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1956E;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C1956E: rts
pub fn i_c1956e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19570;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19570: clr.w $9de(a5)
pub fn i_c19570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19574;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x9DE);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C19574: move.b #$1, $bbf(a5)
pub fn i_c19574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1957A;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBBF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1957A: moveq #$0, d0
pub fn i_c1957a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1957C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1957C: move.w #$27, d7
pub fn i_c1957c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19580;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x27;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19580: movea.l a5, a0
pub fn i_c19580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19582;
    cpu.ir = 0x204D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C19582: move.l d0, (a0)+
pub fn i_c19582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19584;
    cpu.ir = 0x20C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C19584: dbra d7, $c19582
pub fn i_c19584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19588;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC19588; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC19588; return 14; }
    cpu.pc = 0xC19582;
    10
}
/// $C19588: lea.l $d08000.l, a4
pub fn i_c19588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1958E;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C1958E: moveq #$0, d6
pub fn i_c1958e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19590;
    cpu.ir = 0x7C00;
    cpu.d[6] = 0x0;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19590: move.w d6, d5
pub fn i_c19590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19592;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19592: lea.l -$7edc(a4), a2
pub fn i_c19592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19596;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8124);
    cpu.a[2] = ad1;
    16
}
/// $C19596: lea.l -$7d60(a4), a1
pub fn i_c19596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1959A;
    cpu.ir = 0x43EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF82A0);
    cpu.a[1] = ad1;
    16
}
/// $C1959A: movea.l a5, a0
pub fn i_c1959a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1959C;
    cpu.ir = 0x204D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1959C: tst.w (a2)
pub fn i_c1959c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1959E;
    cpu.ir = 0x4A52;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    8
}
/// $C1959E: beq.b $c195bc
pub fn i_c1959e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195A0;
    cpu.ir = 0x671C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC195BC; return 10; }
    8
}
/// $C195A0: addq.b #$1, d5
pub fn i_c195a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195A2;
    cpu.ir = 0x5205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C195A2: move.w d6, (a0)+
pub fn i_c195a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195A4;
    cpu.ir = 0x30C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C195A4: move.w $2(a2), d0
pub fn i_c195a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195A8;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C195A8: move.w d0, (a0)+
pub fn i_c195a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195AA;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C195AA: lsl.w #$4, d0
pub fn i_c195aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195AC;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C195AC: move.l (a1, d0.w), (a0)+
pub fn i_c195ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195B0;
    cpu.ir = 0x20F1;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C195B0: move.l $4(a1, d0.w), (a0)+
pub fn i_c195b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195B4;
    cpu.ir = 0x20F1;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C195B4: move.l $8(a1, d0.w), (a0)+
pub fn i_c195b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195B8;
    cpu.ir = 0x20F1;
    let ad2 = cpu.a[1].wrapping_add(0x8).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C195B8: move.l $c(a1, d0.w), (a0)+
pub fn i_c195b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195BC;
    cpu.ir = 0x20F1;
    let ad2 = cpu.a[1].wrapping_add(0xC).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C195BC: addq.l #$4, a2
pub fn i_c195bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195BE;
    cpu.ir = 0x588A;
    cpu.a[2] = cpu.a[2].wrapping_add(4);
    8
}
/// $C195BE: addq.b #$1, d6
pub fn i_c195be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195C0;
    cpu.ir = 0x5206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C195C0: cmpi.b #$8, d6
pub fn i_c195c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195C4;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $C195C4: bcs.b $c1959c
pub fn i_c195c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195C6;
    cpu.ir = 0x65D6;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1959C; return 10; }
    8
}
/// $C195C6: move.b d5, $9e6(a5)
pub fn i_c195c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195CA;
    cpu.ir = 0x1B45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E6);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C195CA: addq.b #$2, d5
pub fn i_c195ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195CC;
    cpu.ir = 0x5405;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C195CC: move.b d5, $9e7(a5)
pub fn i_c195cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195D0;
    cpu.ir = 0x1B45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E7);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C195D0: jsr $c12a0e(pc)
pub fn i_c195d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC195D4);
    cpu.pc = 0xC12A0E;
    16
}
/// $C195D4: move.b $9e6(a5), d5
pub fn i_c195d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195D8;
    cpu.ir = 0x1A2D;
    let ad2 = cpu.a[5].wrapping_add(0x9E6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C195D8: addq.b #$1, $ac2(a5)
pub fn i_c195d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195DC;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C195DC: movea.l $abe(a5), a0
pub fn i_c195dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195E0;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C195E0: move.l #$c17e20, (a0)+
pub fn i_c195e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195E6;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC17E20;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C195E6: jsr $c17eaa(pc)
pub fn i_c195e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC195EA);
    cpu.pc = 0xC17EAA;
    16
}
/// $C195EA: move.l #$c196ce, (a0)+
pub fn i_c195ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195F0;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC196CE;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C195F0: clr.l (a0)+
pub fn i_c195f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195F2;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C195F2: movea.l a5, a1
pub fn i_c195f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195F4;
    cpu.ir = 0x224D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C195F4: move.l #$50001, (a0)+
pub fn i_c195f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195FA;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x50001;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C195FA: move.w #$108, (a0)+
pub fn i_c195fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC195FE;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C195FE: move.b #$53, (a0)+
pub fn i_c195fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19602;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x53;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19602: move.b #$4c, (a0)+
pub fn i_c19602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19606;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x4C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19606: move.b #$4f, (a0)+
pub fn i_c19606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1960A;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x4F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1960A: move.b #$54, (a0)+
pub fn i_c1960a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1960E;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x54;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1960E: move.b $1(a1), d0
pub fn i_c1960e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19612;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C19612: addi.b #$31, d0
pub fn i_c19612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19616;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x31, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19616: move.b d0, (a0)+
pub fn i_c19616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19618;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19618: move.b #$20, (a0)+
pub fn i_c19618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1961C;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1961C: addq.l #$4, a1
pub fn i_c1961c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1961E;
    cpu.ir = 0x5889;
    cpu.a[1] = cpu.a[1].wrapping_add(4);
    8
}
/// $C1961E: move.l (a1)+, (a0)+
pub fn i_c1961e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19620;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19620: move.l (a1)+, (a0)+
pub fn i_c19620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19622;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19622: move.l (a1)+, (a0)+
pub fn i_c19622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19624;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19624: move.l (a1)+, (a0)+
pub fn i_c19624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19626;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19626: move.w #$ff00, (a0)+
pub fn i_c19626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1962A;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1962A: subq.b #$1, d5
pub fn i_c1962a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1962C;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C1962C: bne.b $c195f4
pub fn i_c1962c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1962E;
    cpu.ir = 0x66C6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC195F4; return 10; }
    8
}
/// $C1962E: clr.w (a0)+
pub fn i_c1962e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19630;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C19630: move.l a0, $abe(a5)
pub fn i_c19630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19634;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19634: subq.b #$1, $ac2(a5)
pub fn i_c19634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19638;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19638: jsr $c17290(pc)
pub fn i_c19638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1963C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1963C);
    cpu.pc = 0xC17290;
    16
}
/// $C1963C: move.w $9de(a5), $9e0(a5)
pub fn i_c1963c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19642;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9E0);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C19642: jsr $c1968a(pc)
pub fn i_c19642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19646;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19646);
    cpu.pc = 0xC1968A;
    16
}
/// $C19646: clr.w $ae4(a5)
pub fn i_c19646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1964A;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xAE4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1964A: jsr $c1727e(pc)
pub fn i_c1964a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1964E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1964E);
    cpu.pc = 0xC1727E;
    16
}
/// $C1964E: jsr $c17eb2(pc)
pub fn i_c1964e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19652;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19652);
    cpu.pc = 0xC17EB2;
    16
}
/// $C19652: move.b $a97(a5), d0
pub fn i_c19652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19656;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C19656: btst.b #$6, d0
pub fn i_c19656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1965A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1965A: beq.b $c19662
pub fn i_c1965a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1965C;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19662; return 10; }
    8
}
/// $C1965C: clr.b $bbf(a5)
pub fn i_c1965c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19660;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xBBF);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19660: rts
pub fn i_c19660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19662;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19662: btst.b #$4, d0
pub fn i_c19662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19666;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C19666: bne.w $c196f8
pub fn i_c19666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1966A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC196F8; return 10; }
    8
}
/// $C1966A: andi.b #$3, d0
pub fn i_c1966a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1966E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1966E: beq.b $c1964a
pub fn i_c1966e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19670;
    cpu.ir = 0x67DA;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1964A; return 10; }
    8
}
/// $C19670: moveq #$1, d1
pub fn i_c19670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19672;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19672: andi.b #$2, d0
pub fn i_c19672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19676;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19676: bne.b $c1967a
pub fn i_c19676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19678;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1967A; return 10; }
    8
}
/// $C19678: moveq #$ff, d1
pub fn i_c19678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1967A;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1967A: add.w $9de(a5), d1
pub fn i_c1967a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1967E;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C1967E: cmp.b $9e7(a5), d1
pub fn i_c1967e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19682;
    cpu.ir = 0xB22D;
    let ad2 = cpu.a[5].wrapping_add(0x9E7);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C19682: bcc.b $c1964a
pub fn i_c19682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19684;
    cpu.ir = 0x64C6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1964A; return 10; }
    8
}
/// $C19684: move.w d1, $9e0(a5)
pub fn i_c19684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19688;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C19688: bra.b $c19642
pub fn i_c19688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1968A;
    cpu.ir = 0x60B8;
    cpu.pc = 0xC19642;
    10
}
/// $C1968A: addq.b #$1, $ac2(a5)
pub fn i_c1968a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1968E;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1968E: movea.l $abe(a5), a0
pub fn i_c1968e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19692;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C19692: clr.l (a0)+
pub fn i_c19692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19694;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C19694: move.w #$20ff, d1
pub fn i_c19694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19698;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19698: jsr $c196b6(pc)
pub fn i_c19698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1969C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1969C);
    cpu.pc = 0xC196B6;
    16
}
/// $C1969C: move.w $9e0(a5), $9de(a5)
pub fn i_c1969c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196A2;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9E0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9DE);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C196A2: move.w #$11ff, d1
pub fn i_c196a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C196A6: jsr $c196b6(pc)
pub fn i_c196a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC196AA);
    cpu.pc = 0xC196B6;
    16
}
/// $C196AA: clr.w (a0)+
pub fn i_c196aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196AC;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C196AC: move.l a0, $abe(a5)
pub fn i_c196ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196B0;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C196B0: subq.b #$1, $ac2(a5)
pub fn i_c196b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196B4;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C196B4: rts
pub fn i_c196b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C196B6: move.w #$3, (a0)+
pub fn i_c196b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196BA;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C196BA: move.w $9de(a5), d0
pub fn i_c196ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196BE;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C196BE: add.w d0, d0
pub fn i_c196be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196C0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C196C0: addi.w #$7085, d0
pub fn i_c196c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196C4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x7085, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C196C4: move.w d0, (a0)+
pub fn i_c196c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196C6;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C196C6: move.w #$1108, (a0)+
pub fn i_c196c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196CA;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C196CA: move.w d1, (a0)+
pub fn i_c196ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196CC;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C196CC: rts
pub fn i_c196cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C196CE: ori.b #$c5, d3
pub fn i_c196ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196D2;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xC5) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C196D2: movep.w $4341(a0), d0
pub fn i_c196d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196D6;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4341);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C196D6: ?
pub fn i_c196d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196D8;
    cpu.ir = 0x4249;
    let v1 = cpu.a[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.a[1] = r2 as i16 as i32 as u32;
    4
}
/// $C196D8: trap #$5
pub fn i_c196d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196DA;
    cpu.ir = 0x4E45;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x94);
    34
}
/// $C196DA: addq.b #$2, $434f(a7)
pub fn i_c196da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196DE;
    cpu.ir = 0x542F;
    let ad1 = cpu.a[7].wrapping_add(0x434F);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C196DE: ?
pub fn i_c196de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196E0;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C196E0: dc.w $ffff
pub fn i_c196e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196E2;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C196E2: ori.b #$1, d5
pub fn i_c196e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196E6;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C196E6: movep.w $4341(a0), d0
pub fn i_c196e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196EA;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4341);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C196EA: ?
pub fn i_c196ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196EC;
    cpu.ir = 0x4249;
    let v1 = cpu.a[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.a[1] = r2 as i16 as i32 as u32;
    4
}
/// $C196EC: trap #$5
pub fn i_c196ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196EE;
    cpu.ir = 0x4E45;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x94);
    34
}
/// $C196EE: addq.b #$2, $504c(a7)
pub fn i_c196ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196F2;
    cpu.ir = 0x542F;
    let ad1 = cpu.a[7].wrapping_add(0x504C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C196F2: ?
pub fn i_c196f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196F4;
    cpu.ir = 0x4159;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C196F4: dc.w $ffff
pub fn i_c196f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196F6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C196F6: ori.b #$7a, d0
pub fn i_c196f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196FA;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x7A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C196F8: pea.l $c19574(pc)
pub fn i_c196f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196FC;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC19574);
    24
}
/// $C196FA: dc.w $fe7a
pub fn i_c196fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC196FC;
    cpu.ir = 0xFE7A;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C196FC: move.w $9de(a5), d0
pub fn i_c196fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19700;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19700: beq.w $c19710
pub fn i_c19700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19704;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19710; return 10; }
    8
}
/// $C19704: cmpi.w #$1, d0
pub fn i_c19704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19708;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $C19708: beq.w $c19838
pub fn i_c19708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1970C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19838; return 10; }
    8
}
/// $C1970C: bra.w $c1999e
pub fn i_c1970c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19710;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1999E;
    10
}
/// $C19710: lea.l $d08000.l, a4
pub fn i_c19710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19716;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C19716: lea.l $200(a5), a1
pub fn i_c19716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1971A;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x200);
    cpu.a[1] = ad1;
    16
}
/// $C1971A: lea.l $1ba0(a4), a2
pub fn i_c1971a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1971E;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0x1BA0);
    cpu.a[2] = ad1;
    16
}
/// $C1971E: move.b -$7ede(a4), d0
pub fn i_c1971e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19722;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8122);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C19722: jsr $c19c88(pc)
pub fn i_c19722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19726;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19726);
    cpu.pc = 0xC19C88;
    16
}
/// $C19726: lea.l $2020(a4), a2
pub fn i_c19726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1972A;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0x2020);
    cpu.a[2] = ad1;
    16
}
/// $C1972A: jsr $c19cb0(pc)
pub fn i_c1972a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1972E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1972E);
    cpu.pc = 0xC19CB0;
    16
}
/// $C1972E: clr.l $9e2(a5)
pub fn i_c1972e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19732;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C19732: jsr $c12a0e(pc)
pub fn i_c19732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19736;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19736);
    cpu.pc = 0xC12A0E;
    16
}
/// $C19736: addq.b #$1, $ac2(a5)
pub fn i_c19736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1973A;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1973A: movea.l $abe(a5), a0
pub fn i_c1973a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1973E;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1973E: move.l #$c19d5c, (a0)+
pub fn i_c1973e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19744;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC19D5C;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C19744: move.l #$c19ce4, (a0)+
pub fn i_c19744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1974A;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC19CE4;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1974A: move.l a0, $abe(a5)
pub fn i_c1974a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1974E;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1974E: subq.b #$1, $ac2(a5)
pub fn i_c1974e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19752;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19752: jsr $c17290(pc)
pub fn i_c19752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19756;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19756);
    cpu.pc = 0xC17290;
    16
}
/// $C19756: lea.l $200(a5), a2
pub fn i_c19756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1975A;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x200);
    cpu.a[2] = ad1;
    16
}
/// $C1975A: move.w $9e2(a5), d0
pub fn i_c1975a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1975E;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1975E: bne.b $c19768
pub fn i_c1975e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19760;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19768; return 10; }
    8
}
/// $C19760: jsr $c197a6(pc)
pub fn i_c19760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19764;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19764);
    cpu.pc = 0xC197A6;
    16
}
/// $C19764: bra.w $c1977c
pub fn i_c19764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19768;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1977C;
    10
}
/// $C19768: lea.l $70(a2), a2
pub fn i_c19768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1976C;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0x70);
    cpu.a[2] = ad1;
    16
}
/// $C1976C: moveq #$1, d7
pub fn i_c1976c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1976E;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1976E: subq.w #$1, d0
pub fn i_c1976e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19770;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C19770: beq.b $c19778
pub fn i_c19770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19772;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19778; return 10; }
    8
}
/// $C19772: lea.l $60(a2), a2
pub fn i_c19772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19776;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0x60);
    cpu.a[2] = ad1;
    16
}
/// $C19776: moveq #$7, d7
pub fn i_c19776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19778;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19778: jsr $c197f4(pc)
pub fn i_c19778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1977C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1977C);
    cpu.pc = 0xC197F4;
    16
}
/// $C1977C: jsr $c1727e(pc)
pub fn i_c1977c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19780;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19780);
    cpu.pc = 0xC1727E;
    16
}
/// $C19780: move.b $a97(a5), d0
pub fn i_c19780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19784;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C19784: btst.b #$6, d0
pub fn i_c19784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19788;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C19788: beq.b $c1978c
pub fn i_c19788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1978A;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1978C; return 10; }
    8
}
/// $C1978A: rts
pub fn i_c1978a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1978C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1978C: btst.b #$4, d0
pub fn i_c1978c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19790;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C19790: beq.b $c1977c
pub fn i_c19790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19792;
    cpu.ir = 0x67EA;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1977C; return 10; }
    8
}
/// $C19792: move.w $9e2(a5), d0
pub fn i_c19792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19796;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19796: addq.w #$1, d0
pub fn i_c19796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19798;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C19798: cmpi.w #$3, d0
pub fn i_c19798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1979C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $C1979C: bne.b $c197a0
pub fn i_c1979c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1979E;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC197A0; return 10; }
    8
}
/// $C1979E: moveq #$0, d0
pub fn i_c1979e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197A0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C197A0: move.w d0, $9e2(a5)
pub fn i_c197a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197A4;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C197A4: bra.b $c19732
pub fn i_c197a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197A6;
    cpu.ir = 0x608C;
    cpu.pc = 0xC19732;
    10
}
/// $C197A6: lea.l $200(a5), a2
pub fn i_c197a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197AA;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x200);
    cpu.a[2] = ad1;
    16
}
/// $C197AA: move.w #$7009, d5
pub fn i_c197aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197AE;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x7009;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C197AE: moveq #$6, d6
pub fn i_c197ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197B0;
    cpu.ir = 0x7C06;
    cpu.d[6] = 0x6;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C197B0: tst.b $1(a2)
pub fn i_c197b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197B4;
    cpu.ir = 0x4A2A;
    let ad2 = cpu.a[2].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C197B4: beq.w $c197f2
pub fn i_c197b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC197F2; return 10; }
    8
}
/// $C197B8: addq.b #$1, $ac2(a5)
pub fn i_c197b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197BC;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C197BC: movea.l $abe(a5), a0
pub fn i_c197bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197C0;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C197C0: clr.l (a0)+
pub fn i_c197c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197C2;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C197C2: jsr $c19aa0(pc)
pub fn i_c197c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC197C6);
    cpu.pc = 0xC19AA0;
    16
}
/// $C197C6: jsr $c19b62(pc)
pub fn i_c197c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC197CA);
    cpu.pc = 0xC19B62;
    16
}
/// $C197CA: jsr $c19b6e(pc)
pub fn i_c197ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC197CE);
    cpu.pc = 0xC19B6E;
    16
}
/// $C197CE: jsr $c19b7a(pc)
pub fn i_c197ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC197D2);
    cpu.pc = 0xC19B7A;
    16
}
/// $C197D2: clr.w (a0)+
pub fn i_c197d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197D4;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C197D4: move.l a0, $abe(a5)
pub fn i_c197d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197D8;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C197D8: subq.b #$1, $ac2(a5)
pub fn i_c197d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197DC;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C197DC: movem.l d0-d7/a0-a6, -(a7)
pub fn i_c197dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197E0;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C197E0: jsr $c17290(pc)
pub fn i_c197e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC197E4);
    cpu.pc = 0xC17290;
    16
}
/// $C197E4: movem.l (a7)+, d0-d7/a0-a6
pub fn i_c197e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197E8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C197E8: lea.l $10(a2), a2
pub fn i_c197e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197EC;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0x10);
    cpu.a[2] = ad1;
    16
}
/// $C197EC: addq.w #$3, d5
pub fn i_c197ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197EE;
    cpu.ir = 0x5645;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C197EE: dbra d6, $c197b0
pub fn i_c197ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197F2;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC197F2; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC197F2; return 14; }
    cpu.pc = 0xC197B0;
    10
}
/// $C197F2: rts
pub fn i_c197f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C197F4: move.w #$7009, d5
pub fn i_c197f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197F8;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x7009;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C197F8: moveq #$5, d6
pub fn i_c197f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197FA;
    cpu.ir = 0x7C05;
    cpu.d[6] = 0x5;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C197FA: addq.b #$1, $ac2(a5)
pub fn i_c197fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC197FE;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C197FE: movea.l $abe(a5), a0
pub fn i_c197fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19802;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C19802: clr.l (a0)+
pub fn i_c19802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19804;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C19804: jsr $c19adc(pc)
pub fn i_c19804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19808;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19808);
    cpu.pc = 0xC19ADC;
    16
}
/// $C19808: jsr $c19b62(pc)
pub fn i_c19808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1980C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1980C);
    cpu.pc = 0xC19B62;
    16
}
/// $C1980C: jsr $c19b6e(pc)
pub fn i_c1980c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19810;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19810);
    cpu.pc = 0xC19B6E;
    16
}
/// $C19810: jsr $c19b7a(pc)
pub fn i_c19810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19814;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19814);
    cpu.pc = 0xC19B7A;
    16
}
/// $C19814: clr.w (a0)+
pub fn i_c19814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19816;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C19816: move.l a0, $abe(a5)
pub fn i_c19816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1981A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1981A: subq.b #$1, $ac2(a5)
pub fn i_c1981a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1981E;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1981E: movem.l d0-d7/a0-a6, -(a7)
pub fn i_c1981e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19822;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C19822: jsr $c17290(pc)
pub fn i_c19822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19826;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19826);
    cpu.pc = 0xC17290;
    16
}
/// $C19826: movem.l (a7)+, d0-d7/a0-a6
pub fn i_c19826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1982A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C1982A: lea.l $10(a2), a2
pub fn i_c1982a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1982E;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0x10);
    cpu.a[2] = ad1;
    16
}
/// $C1982E: addq.b #$1, d7
pub fn i_c1982e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19830;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C19830: addq.w #$3, d5
pub fn i_c19830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19832;
    cpu.ir = 0x5645;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C19832: dbra d6, $c197fa
pub fn i_c19832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19836;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC19836; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC19836; return 14; }
    cpu.pc = 0xC197FA;
    10
}
/// $C19836: rts
pub fn i_c19836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19838;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19838: lea.l $d08000.l, a4
pub fn i_c19838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1983E;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C1983E: lea.l $200(a5), a1
pub fn i_c1983e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19842;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x200);
    cpu.a[1] = ad1;
    16
}
/// $C19842: lea.l $1de0(a4), a2
pub fn i_c19842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19846;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0x1DE0);
    cpu.a[2] = ad1;
    16
}
/// $C19846: move.b -$7ede(a4), d0
pub fn i_c19846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1984A;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8122);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1984A: jsr $c19c88(pc)
pub fn i_c1984a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1984E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1984E);
    cpu.pc = 0xC19C88;
    16
}
/// $C1984E: lea.l $20e0(a4), a2
pub fn i_c1984e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19852;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0x20E0);
    cpu.a[2] = ad1;
    16
}
/// $C19852: jsr $c19cb0(pc)
pub fn i_c19852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19856;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19856);
    cpu.pc = 0xC19CB0;
    16
}
/// $C19856: clr.l $9e2(a5)
pub fn i_c19856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1985A;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C1985A: jsr $c12a0e(pc)
pub fn i_c1985a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1985E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1985E);
    cpu.pc = 0xC12A0E;
    16
}
/// $C1985E: addq.b #$1, $ac2(a5)
pub fn i_c1985e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19862;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19862: movea.l $abe(a5), a0
pub fn i_c19862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19866;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C19866: move.l #$c19d5c, (a0)+
pub fn i_c19866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1986C;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC19D5C;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1986C: move.l #$c19d20, (a0)+
pub fn i_c1986c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19872;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC19D20;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C19872: move.l a0, $abe(a5)
pub fn i_c19872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19876;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19876: subq.b #$1, $ac2(a5)
pub fn i_c19876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1987A;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1987A: jsr $c17290(pc)
pub fn i_c1987a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1987E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1987E);
    cpu.pc = 0xC17290;
    16
}
/// $C1987E: lea.l $200(a5), a2
pub fn i_c1987e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19882;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x200);
    cpu.a[2] = ad1;
    16
}
/// $C19882: move.w $9e2(a5), d0
pub fn i_c19882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19886;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19886: bne.b $c19890
pub fn i_c19886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19888;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19890; return 10; }
    8
}
/// $C19888: jsr $c198ce(pc)
pub fn i_c19888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1988C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1988C);
    cpu.pc = 0xC198CE;
    16
}
/// $C1988C: bra.w $c198a4
pub fn i_c1988c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19890;
    cpu.ir = 0x6000;
    cpu.pc = 0xC198A4;
    10
}
/// $C19890: lea.l $70(a2), a2
pub fn i_c19890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19894;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0x70);
    cpu.a[2] = ad1;
    16
}
/// $C19894: moveq #$1, d7
pub fn i_c19894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19896;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19896: subq.w #$1, d0
pub fn i_c19896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19898;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C19898: beq.b $c198a0
pub fn i_c19898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1989A;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC198A0; return 10; }
    8
}
/// $C1989A: lea.l $60(a2), a2
pub fn i_c1989a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1989E;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0x60);
    cpu.a[2] = ad1;
    16
}
/// $C1989E: moveq #$7, d7
pub fn i_c1989e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198A0;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C198A0: jsr $c1995a(pc)
pub fn i_c198a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC198A4);
    cpu.pc = 0xC1995A;
    16
}
/// $C198A4: jsr $c1727e(pc)
pub fn i_c198a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC198A8);
    cpu.pc = 0xC1727E;
    16
}
/// $C198A8: move.b $a97(a5), d0
pub fn i_c198a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198AC;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C198AC: btst.b #$6, d0
pub fn i_c198ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198B0;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C198B0: beq.b $c198b4
pub fn i_c198b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198B2;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC198B4; return 10; }
    8
}
/// $C198B2: rts
pub fn i_c198b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C198B4: btst.b #$4, d0
pub fn i_c198b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198B8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C198B8: beq.b $c198a4
pub fn i_c198b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198BA;
    cpu.ir = 0x67EA;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC198A4; return 10; }
    8
}
/// $C198BA: move.w $9e2(a5), d0
pub fn i_c198ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198BE;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C198BE: addq.w #$1, d0
pub fn i_c198be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198C0;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C198C0: cmpi.w #$3, d0
pub fn i_c198c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198C4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $C198C4: bne.b $c198c8
pub fn i_c198c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198C6;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC198C8; return 10; }
    8
}
/// $C198C6: moveq #$0, d0
pub fn i_c198c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198C8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C198C8: move.w d0, $9e2(a5)
pub fn i_c198c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198CC;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C198CC: bra.b $c1985a
pub fn i_c198cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198CE;
    cpu.ir = 0x608C;
    cpu.pc = 0xC1985A;
    10
}
/// $C198CE: lea.l $200(a5), a2
pub fn i_c198ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198D2;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x200);
    cpu.a[2] = ad1;
    16
}
/// $C198D2: move.w #$7009, d5
pub fn i_c198d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198D6;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x7009;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C198D6: moveq #$6, d6
pub fn i_c198d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198D8;
    cpu.ir = 0x7C06;
    cpu.d[6] = 0x6;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C198D8: tst.b $1(a2)
pub fn i_c198d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198DC;
    cpu.ir = 0x4A2A;
    let ad2 = cpu.a[2].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C198DC: beq.w $c1991a
pub fn i_c198dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1991A; return 10; }
    8
}
/// $C198E0: addq.b #$1, $ac2(a5)
pub fn i_c198e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198E4;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C198E4: movea.l $abe(a5), a0
pub fn i_c198e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198E8;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C198E8: clr.l (a0)+
pub fn i_c198e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198EA;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C198EA: jsr $c19aa0(pc)
pub fn i_c198ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC198EE);
    cpu.pc = 0xC19AA0;
    16
}
/// $C198EE: jsr $c19b62(pc)
pub fn i_c198ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC198F2);
    cpu.pc = 0xC19B62;
    16
}
/// $C198F2: jsr $c19b6e(pc)
pub fn i_c198f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC198F6);
    cpu.pc = 0xC19B6E;
    16
}
/// $C198F6: jsr $c19bf8(pc)
pub fn i_c198f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC198FA);
    cpu.pc = 0xC19BF8;
    16
}
/// $C198FA: clr.w (a0)+
pub fn i_c198fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC198FC;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C198FC: move.l a0, $abe(a5)
pub fn i_c198fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19900;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19900: subq.b #$1, $ac2(a5)
pub fn i_c19900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19904;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19904: movem.l d0-d7/a0-a6, -(a7)
pub fn i_c19904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19908;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C19908: jsr $c17290(pc)
pub fn i_c19908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1990C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1990C);
    cpu.pc = 0xC17290;
    16
}
/// $C1990C: movem.l (a7)+, d0-d7/a0-a6
pub fn i_c1990c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19910;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C19910: lea.l $10(a2), a2
pub fn i_c19910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19914;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0x10);
    cpu.a[2] = ad1;
    16
}
/// $C19914: addq.w #$3, d5
pub fn i_c19914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19916;
    cpu.ir = 0x5645;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C19916: dbra d6, $c198d8
pub fn i_c19916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1991A;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1991A; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1991A; return 14; }
    cpu.pc = 0xC198D8;
    10
}
/// $C1991A: rts
pub fn i_c1991a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1991C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1991C: jsr $c1995a(pc)
pub fn i_c1991c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19920;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19920);
    cpu.pc = 0xC1995A;
    16
}
/// $C19920: cmpi.w #$d, d7
pub fn i_c19920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19924;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xD, Size::Word, false);
    8
}
/// $C19924: bne.w $c19958
pub fn i_c19924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19928;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19958; return 10; }
    8
}
/// $C19928: addq.b #$1, $ac2(a5)
pub fn i_c19928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1992C;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1992C: movea.l $abe(a5), a0
pub fn i_c1992c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19930;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C19930: clr.l (a0)+
pub fn i_c19930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19932;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C19932: jsr $c19b42(pc)
pub fn i_c19932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19936;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19936);
    cpu.pc = 0xC19B42;
    16
}
/// $C19936: jsr $c19b62(pc)
pub fn i_c19936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1993A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1993A);
    cpu.pc = 0xC19B62;
    16
}
/// $C1993A: jsr $c19b6e(pc)
pub fn i_c1993a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1993E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1993E);
    cpu.pc = 0xC19B6E;
    16
}
/// $C1993E: jsr $c19bf8(pc)
pub fn i_c1993e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19942;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19942);
    cpu.pc = 0xC19BF8;
    16
}
/// $C19942: clr.w (a0)+
pub fn i_c19942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19944;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C19944: move.l a0, $abe(a5)
pub fn i_c19944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19948;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19948: subq.b #$1, $ac2(a5)
pub fn i_c19948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1994C;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1994C: movem.l d0-d7/a0-a6, -(a7)
pub fn i_c1994c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19950;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C19950: jsr $c17290(pc)
pub fn i_c19950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19954;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19954);
    cpu.pc = 0xC17290;
    16
}
/// $C19954: movem.l (a7)+, d0-d7/a0-a6
pub fn i_c19954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19958;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C19958: rts
pub fn i_c19958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1995A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1995A: move.w #$7009, d5
pub fn i_c1995a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1995E;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x7009;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1995E: moveq #$5, d6
pub fn i_c1995e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19960;
    cpu.ir = 0x7C05;
    cpu.d[6] = 0x5;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19960: addq.b #$1, $ac2(a5)
pub fn i_c19960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19964;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19964: movea.l $abe(a5), a0
pub fn i_c19964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19968;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C19968: clr.l (a0)+
pub fn i_c19968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1996A;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C1996A: jsr $c19adc(pc)
pub fn i_c1996a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1996E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1996E);
    cpu.pc = 0xC19ADC;
    16
}
/// $C1996E: jsr $c19b62(pc)
pub fn i_c1996e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19972;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19972);
    cpu.pc = 0xC19B62;
    16
}
/// $C19972: jsr $c19b6e(pc)
pub fn i_c19972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19976;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19976);
    cpu.pc = 0xC19B6E;
    16
}
/// $C19976: jsr $c19bf8(pc)
pub fn i_c19976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1997A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1997A);
    cpu.pc = 0xC19BF8;
    16
}
/// $C1997A: clr.w (a0)+
pub fn i_c1997a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1997C;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1997C: move.l a0, $abe(a5)
pub fn i_c1997c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19980;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19980: subq.b #$1, $ac2(a5)
pub fn i_c19980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19984;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19984: movem.l d0-d7/a0-a6, -(a7)
pub fn i_c19984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19988;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C19988: jsr $c17290(pc)
pub fn i_c19988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1998C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1998C);
    cpu.pc = 0xC17290;
    16
}
/// $C1998C: movem.l (a7)+, d0-d7/a0-a6
pub fn i_c1998c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19990;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C19990: lea.l $10(a2), a2
pub fn i_c19990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19994;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0x10);
    cpu.a[2] = ad1;
    16
}
/// $C19994: addq.b #$1, d7
pub fn i_c19994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19996;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $C19996: addq.w #$3, d5
pub fn i_c19996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19998;
    cpu.ir = 0x5645;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C19998: dbra d6, $c19960
pub fn i_c19998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1999C;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1999C; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1999C; return 14; }
    cpu.pc = 0xC19960;
    10
}
/// $C1999C: rts
pub fn i_c1999c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1999E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1999E: lea.l $d08000.l, a4
pub fn i_c1999e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199A4;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C199A4: movea.l a5, a1
pub fn i_c199a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199A6;
    cpu.ir = 0x224D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C199A6: move.w $9de(a5), d0
pub fn i_c199a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199AA;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C199AA: subq.w #$2, d0
pub fn i_c199aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199AC;
    cpu.ir = 0x5540;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C199AC: mulu.w #$14, d0
pub fn i_c199ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199B0;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0x14;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C199B0: lea.l (a1, d0.w), a1
pub fn i_c199b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199B4;
    cpu.ir = 0x43F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $C199B4: moveq #$0, d0
pub fn i_c199b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199B6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C199B6: move.b $3(a1), d0
pub fn i_c199b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199BA;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C199BA: move.w d0, d6
pub fn i_c199ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199BC;
    cpu.ir = 0x3C00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C199BC: addq.l #$4, a1
pub fn i_c199bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199BE;
    cpu.ir = 0x5889;
    cpu.a[1] = cpu.a[1].wrapping_add(4);
    8
}
/// $C199BE: move.l a1, $9e8(a5)
pub fn i_c199be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199C2;
    cpu.ir = 0x2B49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C199C2: moveq #$0, d1
pub fn i_c199c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199C4;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C199C4: move.w d0, d1
pub fn i_c199c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199C6;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C199C6: mulu.w #$240, d0
pub fn i_c199c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199CA;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0x240;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C199CA: addi.l #$d08320, d0
pub fn i_c199ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199D0;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0xD08320, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $C199D0: movea.l d0, a2
pub fn i_c199d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199D2;
    cpu.ir = 0x2440;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $C199D2: addi.l #$d0011a, d1
pub fn i_c199d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199D8;
    cpu.ir = 0x681;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0xD0011A, Size::Long, true);
    cpu.d[1] = r2;
    8
}
/// $C199D8: movea.l d1, a3
pub fn i_c199d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199DA;
    cpu.ir = 0x2641;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[3] = v1;
    4
}
/// $C199DA: move.b (a3), d0
pub fn i_c199da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199DC;
    cpu.ir = 0x1013;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C199DC: lea.l $200(a5), a1
pub fn i_c199dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199E0;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x200);
    cpu.a[1] = ad1;
    16
}
/// $C199E0: jsr $c19c88(pc)
pub fn i_c199e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC199E4);
    cpu.pc = 0xC19C88;
    16
}
/// $C199E4: move.w d6, d1
pub fn i_c199e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199E6;
    cpu.ir = 0x3206;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C199E6: mulu.w #$c0, d6
pub fn i_c199e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199EA;
    cpu.ir = 0xCCFC;
    let v1: u32 = 0xC0;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[6] & 0xFFFF);
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C199EA: lea.l $1520(a4), a2
pub fn i_c199ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199EE;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0x1520);
    cpu.a[2] = ad1;
    16
}
/// $C199EE: adda.l d6, a2
pub fn i_c199ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199F0;
    cpu.ir = 0xD5C6;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $C199F0: jsr $c19cb0(pc)
pub fn i_c199f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC199F4);
    cpu.pc = 0xC19CB0;
    16
}
/// $C199F4: lsl.w #$4, d1
pub fn i_c199f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199F6;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C199F6: lea.l $1b20(a4), a2
pub fn i_c199f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199FA;
    cpu.ir = 0x45EC;
    let ad1 = cpu.a[4].wrapping_add(0x1B20);
    cpu.a[2] = ad1;
    16
}
/// $C199FA: lea.l (a2, d1.w), a2
pub fn i_c199fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC199FE;
    cpu.ir = 0x45F2;
    let ad1 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    cpu.a[2] = ad1;
    18
}
/// $C199FE: move.l (a2)+, (a1)+
pub fn i_c199fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A00;
    cpu.ir = 0x22DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19A00: move.l (a2)+, (a1)+
pub fn i_c19a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A02;
    cpu.ir = 0x22DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19A02: move.l (a2)+, (a1)+
pub fn i_c19a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A04;
    cpu.ir = 0x22DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19A04: move.l (a2)+, (a1)+
pub fn i_c19a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A06;
    cpu.ir = 0x22DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19A06: clr.w $9e2(a5)
pub fn i_c19a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A0A;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C19A0A: jsr $c12a0e(pc)
pub fn i_c19a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19A0E);
    cpu.pc = 0xC12A0E;
    16
}
/// $C19A0E: addq.b #$1, $ac2(a5)
pub fn i_c19a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A12;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19A12: movea.l $abe(a5), a0
pub fn i_c19a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A16;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C19A16: move.l #$c19d5c, (a0)+
pub fn i_c19a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A1C;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC19D5C;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C19A1C: move.l #$c19d34, (a0)+
pub fn i_c19a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A22;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC19D34;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C19A22: clr.l (a0)+
pub fn i_c19a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A24;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C19A24: move.w #$3, (a0)+
pub fn i_c19a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A28;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19A28: move.w #$7143, (a0)+
pub fn i_c19a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A2C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7143;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19A2C: move.w #$108, (a0)+
pub fn i_c19a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A30;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19A30: movea.l $9e8(a5), a1
pub fn i_c19a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A34;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C19A34: move.l (a1)+, (a0)+
pub fn i_c19a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A36;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19A36: move.l (a1)+, (a0)+
pub fn i_c19a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A38;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19A38: move.l (a1)+, (a0)+
pub fn i_c19a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A3A;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19A3A: move.l (a1)+, (a0)+
pub fn i_c19a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A3C;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19A3C: move.w #$ff00, (a0)+
pub fn i_c19a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A40;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19A40: clr.w (a0)+
pub fn i_c19a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A42;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C19A42: move.l a0, $abe(a5)
pub fn i_c19a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A46;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19A46: subq.b #$1, $ac2(a5)
pub fn i_c19a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A4A;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19A4A: jsr $c17290(pc)
pub fn i_c19a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19A4E);
    cpu.pc = 0xC17290;
    16
}
/// $C19A4E: lea.l $200(a5), a2
pub fn i_c19a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A52;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x200);
    cpu.a[2] = ad1;
    16
}
/// $C19A52: move.w $9e2(a5), d0
pub fn i_c19a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A56;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19A56: bne.b $c19a60
pub fn i_c19a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A58;
    cpu.ir = 0x6608;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19A60; return 10; }
    8
}
/// $C19A58: jsr $c198ce(pc)
pub fn i_c19a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19A5C);
    cpu.pc = 0xC198CE;
    16
}
/// $C19A5C: bra.w $c19a74
pub fn i_c19a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A60;
    cpu.ir = 0x6000;
    cpu.pc = 0xC19A74;
    10
}
/// $C19A60: lea.l $70(a2), a2
pub fn i_c19a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A64;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0x70);
    cpu.a[2] = ad1;
    16
}
/// $C19A64: moveq #$1, d7
pub fn i_c19a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A66;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19A66: subq.w #$1, d0
pub fn i_c19a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A68;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C19A68: beq.b $c19a70
pub fn i_c19a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A6A;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19A70; return 10; }
    8
}
/// $C19A6A: lea.l $60(a2), a2
pub fn i_c19a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A6E;
    cpu.ir = 0x45EA;
    let ad1 = cpu.a[2].wrapping_add(0x60);
    cpu.a[2] = ad1;
    16
}
/// $C19A6E: moveq #$7, d7
pub fn i_c19a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A70;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19A70: jsr $c1991c(pc)
pub fn i_c19a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19A74);
    cpu.pc = 0xC1991C;
    16
}
/// $C19A74: jsr $c1727e(pc)
pub fn i_c19a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19A78);
    cpu.pc = 0xC1727E;
    16
}
/// $C19A78: move.b $a97(a5), d0
pub fn i_c19a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A7C;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C19A7C: btst.b #$6, d0
pub fn i_c19a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A80;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C19A80: beq.b $c19a84
pub fn i_c19a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A82;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19A84; return 10; }
    8
}
/// $C19A82: rts
pub fn i_c19a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19A84: btst.b #$4, d0
pub fn i_c19a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A88;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C19A88: beq.b $c19a74
pub fn i_c19a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A8A;
    cpu.ir = 0x67EA;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19A74; return 10; }
    8
}
/// $C19A8A: move.w $9e2(a5), d0
pub fn i_c19a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A8E;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19A8E: addq.w #$1, d0
pub fn i_c19a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A90;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C19A90: cmpi.w #$3, d0
pub fn i_c19a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A94;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $C19A94: bne.b $c19a98
pub fn i_c19a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A96;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19A98; return 10; }
    8
}
/// $C19A96: moveq #$0, d0
pub fn i_c19a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A98;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19A98: move.w d0, $9e2(a5)
pub fn i_c19a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19A9C;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C19A9C: bra.w $c19a0a
pub fn i_c19a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AA0;
    cpu.ir = 0x6000;
    cpu.pc = 0xC19A0A;
    10
}
/// $C19AA0: move.w #$3, (a0)+
pub fn i_c19aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AA4;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19AA4: move.w #$80, d0
pub fn i_c19aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AA8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19AA8: add.w d5, d0
pub fn i_c19aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AAA;
    cpu.ir = 0xD045;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19AAA: move.w d0, (a0)+
pub fn i_c19aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AAC;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19AAC: move.w #$108, (a0)+
pub fn i_c19aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AB0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19AB0: tst.b $3(a2)
pub fn i_c19ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AB4;
    cpu.ir = 0x4A2A;
    let ad2 = cpu.a[2].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C19AB4: bne.b $c19abc
pub fn i_c19ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AB6;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19ABC; return 10; }
    8
}
/// $C19AB6: move.w #$1108, -$2(a0)
pub fn i_c19ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19ABC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C19ABC: move.b $1(a2), d0
pub fn i_c19abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AC0;
    cpu.ir = 0x102A;
    let ad2 = cpu.a[2].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C19AC0: jsr $c1217c(pc)
pub fn i_c19ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19AC4);
    cpu.pc = 0xC1217C;
    16
}
/// $C19AC4: jsr $c19af0(pc)
pub fn i_c19ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19AC8);
    cpu.pc = 0xC19AF0;
    16
}
/// $C19AC8: move.b #$2f, -$1(a0)
pub fn i_c19ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19ACE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C19ACE: move.b $2(a2), d0
pub fn i_c19ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AD2;
    cpu.ir = 0x102A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C19AD2: jsr $c19b2e(pc)
pub fn i_c19ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19AD6);
    cpu.pc = 0xC19B2E;
    16
}
/// $C19AD6: move.w #$ff00, (a0)+
pub fn i_c19ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19ADA;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19ADA: rts
pub fn i_c19ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19ADC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19ADC: move.w #$3, (a0)+
pub fn i_c19adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AE0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19AE0: move.w #$c0, d0
pub fn i_c19ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AE4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19AE4: add.w d5, d0
pub fn i_c19ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AE6;
    cpu.ir = 0xD045;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19AE6: move.w d0, (a0)+
pub fn i_c19ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AE8;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19AE8: move.w #$108, (a0)+
pub fn i_c19ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AEC;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19AEC: moveq #$0, d0
pub fn i_c19aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AEE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19AEE: move.b d7, d0
pub fn i_c19aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AF0;
    cpu.ir = 0x1007;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C19AF0: add.w d0, d0
pub fn i_c19af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AF2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19AF2: add.w d0, d0
pub fn i_c19af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AF4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19AF4: move.l $c19afa(pc, d0.w), (a0)+
pub fn i_c19af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AF8;
    cpu.ir = 0x20FB;
    let ad2 = 0xC19AFA_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C19AF8: rts
pub fn i_c19af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19AFA: tst.w d1
pub fn i_c19afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AFC;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C19AFC: ?
pub fn i_c19afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19AFE;
    cpu.ir = 0x4EFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19AFE: tst.w d1
pub fn i_c19afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B00;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C19B00: ?
pub fn i_c19b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B02;
    cpu.ir = 0x4EFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19B02: not.w d5
pub fn i_c19b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B04;
    cpu.ir = 0x4645;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C19B04: ?
pub fn i_c19b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B06;
    cpu.ir = 0x42FF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19B06: ?
pub fn i_c19b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B08;
    cpu.ir = 0x4D41;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19B2E: jsr $c16f54(pc)
pub fn i_c19b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19B32);
    cpu.pc = 0xC16F54;
    16
}
/// $C19B32: cmpi.b #$30, d0
pub fn i_c19b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B36;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $C19B36: bne.b $c19b3c
pub fn i_c19b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B38;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19B3C; return 10; }
    8
}
/// $C19B38: move.b #$20, d0
pub fn i_c19b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B3C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C19B3C: move.b d0, (a0)+
pub fn i_c19b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B3E;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19B3E: move.b d1, (a0)+
pub fn i_c19b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B40;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19B40: rts
pub fn i_c19b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19B42: move.w #$3, (a0)+
pub fn i_c19b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B46;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19B46: move.w #$c0, d0
pub fn i_c19b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19B4A: add.w d5, d0
pub fn i_c19b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B4C;
    cpu.ir = 0xD045;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19B4C: move.w d0, (a0)+
pub fn i_c19b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B4E;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19B4E: move.w #$108, (a0)+
pub fn i_c19b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B52;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19B52: move.l #$544f5441, (a0)+
pub fn i_c19b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B58;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x544F5441;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C19B58: move.b #$4c, (a0)+
pub fn i_c19b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B5C;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x4C;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19B5C: move.b #$ff, (a0)+
pub fn i_c19b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B60;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19B60: rts
pub fn i_c19b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19B62: move.w #$180, d0
pub fn i_c19b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B66;
    cpu.ir = 0x303C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19B66: move.l $4(a2), d4
pub fn i_c19b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B6A;
    cpu.ir = 0x282A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19B6A: bra.w $c19b82
pub fn i_c19b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B6E;
    cpu.ir = 0x6000;
    cpu.pc = 0xC19B82;
    10
}
/// $C19B6E: move.w #$280, d0
pub fn i_c19b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B72;
    cpu.ir = 0x303C;
    let v1: u32 = 0x280;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19B72: move.l $8(a2), d4
pub fn i_c19b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B76;
    cpu.ir = 0x282A;
    let ad2 = cpu.a[2].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19B76: bra.w $c19b82
pub fn i_c19b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B7A;
    cpu.ir = 0x6000;
    cpu.pc = 0xC19B82;
    10
}
/// $C19B7A: move.w #$380, d0
pub fn i_c19b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B7E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x380;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19B7E: move.l $c(a2), d4
pub fn i_c19b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B82;
    cpu.ir = 0x282A;
    let ad2 = cpu.a[2].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19B82: move.w #$3, (a0)+
pub fn i_c19b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B86;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19B86: add.w d5, d0
pub fn i_c19b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B88;
    cpu.ir = 0xD045;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19B88: move.w d0, (a0)+
pub fn i_c19b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B8A;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19B8A: move.w #$108, (a0)+
pub fn i_c19b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B8E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19B8E: cmpi.l #$98967f, d4
pub fn i_c19b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B94;
    cpu.ir = 0xC84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x98967F, Size::Long, false);
    8
}
/// $C19B94: bcs.b $c19b9c
pub fn i_c19b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B96;
    cpu.ir = 0x6506;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC19B9C; return 10; }
    8
}
/// $C19B96: move.l #$98967f, d4
pub fn i_c19b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19B9C;
    cpu.ir = 0x283C;
    let v1: u32 = 0x98967F;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19B9C: move.b #$20, d1
pub fn i_c19b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BA0;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C19BA0: move.l #$f4240, d3
pub fn i_c19ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BA6;
    cpu.ir = 0x263C;
    let v1: u32 = 0xF4240;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19BA6: jsr $c19cba(pc)
pub fn i_c19ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BAA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19BAA);
    cpu.pc = 0xC19CBA;
    16
}
/// $C19BAA: jsr $c19cd6(pc)
pub fn i_c19baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19BAE);
    cpu.pc = 0xC19CD6;
    16
}
/// $C19BAE: move.l #$186a0, d3
pub fn i_c19bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BB4;
    cpu.ir = 0x263C;
    let v1: u32 = 0x186A0;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19BB4: jsr $c19cba(pc)
pub fn i_c19bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19BB8);
    cpu.pc = 0xC19CBA;
    16
}
/// $C19BB8: jsr $c19cd6(pc)
pub fn i_c19bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19BBC);
    cpu.pc = 0xC19CD6;
    16
}
/// $C19BBC: move.l #$2710, d3
pub fn i_c19bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BC2;
    cpu.ir = 0x263C;
    let v1: u32 = 0x2710;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19BC2: jsr $c19cba(pc)
pub fn i_c19bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19BC6);
    cpu.pc = 0xC19CBA;
    16
}
/// $C19BC6: jsr $c19cd6(pc)
pub fn i_c19bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19BCA);
    cpu.pc = 0xC19CD6;
    16
}
/// $C19BCA: move.l #$3e8, d3
pub fn i_c19bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BD0;
    cpu.ir = 0x263C;
    let v1: u32 = 0x3E8;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19BD0: jsr $c19cba(pc)
pub fn i_c19bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19BD4);
    cpu.pc = 0xC19CBA;
    16
}
/// $C19BD4: jsr $c19cd6(pc)
pub fn i_c19bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19BD8);
    cpu.pc = 0xC19CD6;
    16
}
/// $C19BD8: moveq #$64, d3
pub fn i_c19bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BDA;
    cpu.ir = 0x7664;
    cpu.d[3] = 0x64;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19BDA: jsr $c19cba(pc)
pub fn i_c19bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19BDE);
    cpu.pc = 0xC19CBA;
    16
}
/// $C19BDE: jsr $c19cd6(pc)
pub fn i_c19bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19BE2);
    cpu.pc = 0xC19CD6;
    16
}
/// $C19BE2: moveq #$a, d3
pub fn i_c19be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BE4;
    cpu.ir = 0x760A;
    cpu.d[3] = 0xA;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19BE4: jsr $c19cba(pc)
pub fn i_c19be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19BE8);
    cpu.pc = 0xC19CBA;
    16
}
/// $C19BE8: jsr $c19cd6(pc)
pub fn i_c19be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19BEC);
    cpu.pc = 0xC19CD6;
    16
}
/// $C19BEC: ori.b #$30, d4
pub fn i_c19bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BF0;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19BF0: move.b d4, (a0)+
pub fn i_c19bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BF2;
    cpu.ir = 0x10C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19BF2: move.b #$ff, (a0)+
pub fn i_c19bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BF6;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19BF6: rts
pub fn i_c19bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19BF8: move.w #$380, d0
pub fn i_c19bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19BFC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x380;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19BFC: move.l $c(a2), d4
pub fn i_c19bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C00;
    cpu.ir = 0x282A;
    let ad2 = cpu.a[2].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19C00: move.w #$3, (a0)+
pub fn i_c19c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C04;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19C04: add.w d5, d0
pub fn i_c19c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C06;
    cpu.ir = 0xD045;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19C06: move.w d0, (a0)+
pub fn i_c19c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C08;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19C08: move.w #$108, (a0)+
pub fn i_c19c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C0C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19C0C: move.l $4(a2), d3
pub fn i_c19c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C10;
    cpu.ir = 0x262A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.d[3] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19C10: bne.b $c19c18
pub fn i_c19c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C12;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19C18; return 10; }
    8
}
/// $C19C12: moveq #$0, d0
pub fn i_c19c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C14;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19C14: move.l d0, -(a7)
pub fn i_c19c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C16;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C19C16: bra.b $c19c34
pub fn i_c19c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C18;
    cpu.ir = 0x601C;
    cpu.pc = 0xC19C34;
    10
}
/// $C19C18: jsr $c19cba(pc)
pub fn i_c19c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19C1C);
    cpu.pc = 0xC19CBA;
    16
}
/// $C19C1C: cmpi.l #$ea24, d0
pub fn i_c19c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C22;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xEA24, Size::Long, false);
    8
}
/// $C19C22: bcs.b $c19c2a
pub fn i_c19c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C24;
    cpu.ir = 0x6506;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC19C2A; return 10; }
    8
}
/// $C19C24: move.l #$ea24, d0
pub fn i_c19c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C2A;
    cpu.ir = 0x203C;
    let v1: u32 = 0xEA24;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19C2A: move.l d0, d4
pub fn i_c19c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C2C;
    cpu.ir = 0x2800;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19C2C: moveq #$3c, d3
pub fn i_c19c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C2E;
    cpu.ir = 0x763C;
    cpu.d[3] = 0x3C;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19C2E: jsr $c19cba(pc)
pub fn i_c19c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19C32);
    cpu.pc = 0xC19CBA;
    16
}
/// $C19C32: move.l d4, -(a7)
pub fn i_c19c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C34;
    cpu.ir = 0x2F04;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C19C34: move.b #$20, d1
pub fn i_c19c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C38;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C19C38: move.l d0, d4
pub fn i_c19c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C3A;
    cpu.ir = 0x2800;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19C3A: moveq #$64, d3
pub fn i_c19c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C3C;
    cpu.ir = 0x7664;
    cpu.d[3] = 0x64;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19C3C: jsr $c19cba(pc)
pub fn i_c19c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19C40);
    cpu.pc = 0xC19CBA;
    16
}
/// $C19C40: jsr $c19cd6(pc)
pub fn i_c19c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19C44);
    cpu.pc = 0xC19CD6;
    16
}
/// $C19C44: moveq #$a, d3
pub fn i_c19c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C46;
    cpu.ir = 0x760A;
    cpu.d[3] = 0xA;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19C46: jsr $c19cba(pc)
pub fn i_c19c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19C4A);
    cpu.pc = 0xC19CBA;
    16
}
/// $C19C4A: jsr $c19cd6(pc)
pub fn i_c19c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19C4E);
    cpu.pc = 0xC19CD6;
    16
}
/// $C19C4E: ori.b #$30, d4
pub fn i_c19c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C52;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19C52: move.b d4, (a0)+
pub fn i_c19c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C54;
    cpu.ir = 0x10C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19C54: move.b #$ff, (a0)+
pub fn i_c19c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C58;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19C58: move.w #$3e0, d0
pub fn i_c19c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C5C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3E0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19C5C: add.w d5, d0
pub fn i_c19c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C5E;
    cpu.ir = 0xD045;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19C5E: move.w #$3, (a0)+
pub fn i_c19c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C62;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19C62: move.w d0, (a0)+
pub fn i_c19c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C64;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19C64: move.w #$108, (a0)+
pub fn i_c19c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C68;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19C68: move.b #$3a, (a0)+
pub fn i_c19c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C6C;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x3A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19C6C: move.l (a7)+, d4
pub fn i_c19c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C6E;
    cpu.ir = 0x281F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[4] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C19C6E: moveq #$a, d3
pub fn i_c19c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C70;
    cpu.ir = 0x760A;
    cpu.d[3] = 0xA;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19C70: jsr $c19cba(pc)
pub fn i_c19c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19C74);
    cpu.pc = 0xC19CBA;
    16
}
/// $C19C74: nop
pub fn i_c19c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C76;
    cpu.ir = 0x4E71;
    4
}
/// $C19C76: ori.b #$30, d0
pub fn i_c19c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C7A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19C7A: move.b d0, (a0)+
pub fn i_c19c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C7C;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19C7C: ori.b #$30, d4
pub fn i_c19c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C80;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19C80: move.b d4, (a0)+
pub fn i_c19c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C82;
    cpu.ir = 0x10C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19C82: move.b #$ff, (a0)+
pub fn i_c19c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C86;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19C86: rts
pub fn i_c19c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19C88: andi.w #$ff, d0
pub fn i_c19c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C8C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C19C8C: lsl.w #$4, d0
pub fn i_c19c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C8E;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $C19C8E: moveq #$6, d7
pub fn i_c19c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C90;
    cpu.ir = 0x7E06;
    cpu.d[7] = 0x6;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19C90: move.l (a2, d0.w), (a1)+
pub fn i_c19c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C94;
    cpu.ir = 0x22F2;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C19C94: move.l $4(a2, d0.w), (a1)+
pub fn i_c19c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C98;
    cpu.ir = 0x22F2;
    let ad2 = cpu.a[2].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C19C98: move.l $8(a2, d0.w), (a1)+
pub fn i_c19c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19C9C;
    cpu.ir = 0x22F2;
    let ad2 = cpu.a[2].wrapping_add(0x8).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C19C9C: move.l $c(a2, d0.w), (a1)+
pub fn i_c19c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CA0;
    cpu.ir = 0x22F2;
    let ad2 = cpu.a[2].wrapping_add(0xC).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C19CA0: subi.w #$10, d0
pub fn i_c19ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CA4;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C19CA4: bpl.b $c19caa
pub fn i_c19ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CA6;
    cpu.ir = 0x6A04;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC19CAA; return 10; }
    8
}
/// $C19CA6: move.w #$230, d0
pub fn i_c19ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CAA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x230;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19CAA: dbra d7, $c19c90
pub fn i_c19caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CAE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC19CAE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC19CAE; return 14; }
    cpu.pc = 0xC19C90;
    10
}
/// $C19CAE: rts
pub fn i_c19cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19CB0: moveq #$2f, d7
pub fn i_c19cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CB2;
    cpu.ir = 0x7E2F;
    cpu.d[7] = 0x2F;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19CB2: move.l (a2)+, (a1)+
pub fn i_c19cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CB4;
    cpu.ir = 0x22DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C19CB4: dbra d7, $c19cb2
pub fn i_c19cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CB8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC19CB8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC19CB8; return 14; }
    cpu.pc = 0xC19CB2;
    10
}
/// $C19CB8: rts
pub fn i_c19cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19CBA: move.l d7, -(a7)
pub fn i_c19cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CBC;
    cpu.ir = 0x2F07;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $C19CBC: moveq #$0, d0
pub fn i_c19cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CBE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19CBE: moveq #$1f, d7
pub fn i_c19cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CC0;
    cpu.ir = 0x7E1F;
    cpu.d[7] = 0x1F;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19CC0: add.l d4, d4
pub fn i_c19cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CC2;
    cpu.ir = 0xD884;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[4] = r;
    4
}
/// $C19CC2: addx.l d0, d0
pub fn i_c19cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CC4;
    cpu.ir = 0xD180;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let h2 = cpu.d[0] & 0xFFFFFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Long);
    cpu.d[0] = r;
    8
}
/// $C19CC4: cmp.l d3, d0
pub fn i_c19cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CC6;
    cpu.ir = 0xB083;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $C19CC6: bcs.b $c19ccc
pub fn i_c19cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CC8;
    cpu.ir = 0x6504;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC19CCC; return 10; }
    8
}
/// $C19CC8: sub.l d3, d0
pub fn i_c19cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CCA;
    cpu.ir = 0x9083;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $C19CCA: addq.l #$1, d4
pub fn i_c19cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CCC;
    cpu.ir = 0x5284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[4] = r2;
    4
}
/// $C19CCC: dbra d7, $c19cc0
pub fn i_c19ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CD0;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC19CD0; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC19CD0; return 14; }
    cpu.pc = 0xC19CC0;
    10
}
/// $C19CD0: exg.l d0, d4
pub fn i_c19cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CD2;
    cpu.ir = 0xC144;
    cpu.d.swap(0, 4);
    6
}
/// $C19CD2: move.l (a7)+, d7
pub fn i_c19cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CD4;
    cpu.ir = 0x2E1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[7] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C19CD4: rts
pub fn i_c19cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19CD6: tst.b d0
pub fn i_c19cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CD8;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C19CD8: beq.b $c19cde
pub fn i_c19cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CDA;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19CDE; return 10; }
    8
}
/// $C19CDA: move.b #$30, d1
pub fn i_c19cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CDE;
    cpu.ir = 0x123C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C19CDE: or.b d1, d0
pub fn i_c19cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CE0;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $C19CE0: move.b d0, (a0)+
pub fn i_c19ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CE2;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C19CE2: rts
pub fn i_c19ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19CE4: ori.b #$43, d3
pub fn i_c19ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CE8;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x43) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19CE8: movep.w $4341(a0), d0
pub fn i_c19ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CEC;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4341);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C19CEC: ?
pub fn i_c19cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CEE;
    cpu.ir = 0x4249;
    let v1 = cpu.a[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.a[1] = r2 as i16 as i32 as u32;
    4
}
/// $C19CEE: trap #$5
pub fn i_c19cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CF0;
    cpu.ir = 0x4E45;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x94);
    34
}
/// $C19CF0: addq.b #$2, $434f(a7)
pub fn i_c19cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CF4;
    cpu.ir = 0x542F;
    let ad1 = cpu.a[7].wrapping_add(0x434F);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19CF4: ?
pub fn i_c19cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CF6;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19CF6: dc.w $ffff
pub fn i_c19cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CF8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C19CF8: ori.b #$a6, d3
pub fn i_c19cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19CFC;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xA6) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19CFC: movep.w $434f(a0), d0
pub fn i_c19cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D00;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x434F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C19D00: ?
pub fn i_c19d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D02;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D02: ?
pub fn i_c19d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D06;
    cpu.ir = 0x31FF;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3, v1 as u16);
    12
}
/// $C19D06: moveq #$a6, d1
pub fn i_c19d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D08;
    cpu.ir = 0x72A6;
    cpu.d[1] = 0xFFFFFFA6;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19D08: movep.w $434f(a0), d0
pub fn i_c19d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D0C;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x434F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C19D0C: ?
pub fn i_c19d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D0E;
    cpu.ir = 0x494E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D0E: ?
pub fn i_c19d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D10;
    cpu.ir = 0x32FF;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C19D10: ori.b #$86, d3
pub fn i_c19d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D14;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x86) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19D14: movep.w $5345(a0), d0
pub fn i_c19d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D18;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5345);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C19D18: addq.w #$1, (a6)
pub fn i_c19d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D1A;
    cpu.ir = 0x5256;
    let ad1 = cpu.a[6];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C19D1A: ?
pub fn i_c19d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D1C;
    cpu.ir = 0x4943;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D20: ori.b #$43, d3
pub fn i_c19d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D24;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x43) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19D24: movep.w $4341(a0), d0
pub fn i_c19d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D28;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4341);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C19D28: ?
pub fn i_c19d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D2A;
    cpu.ir = 0x4249;
    let v1 = cpu.a[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.a[1] = r2 as i16 as i32 as u32;
    4
}
/// $C19D2A: trap #$5
pub fn i_c19d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D2C;
    cpu.ir = 0x4E45;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0x94);
    34
}
/// $C19D2C: addq.b #$2, $504c(a7)
pub fn i_c19d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D30;
    cpu.ir = 0x542F;
    let ad1 = cpu.a[7].wrapping_add(0x504C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19D30: ?
pub fn i_c19d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D32;
    cpu.ir = 0x4159;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D32: dc.w $ff00
pub fn i_c19d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D34;
    cpu.ir = 0xFF00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C19D34: ori.b #$a6, d3
pub fn i_c19d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D38;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xA6) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19D38: movep.w $504c(a0), d0
pub fn i_c19d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D3C;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x504C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C19D3C: ?
pub fn i_c19d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D3E;
    cpu.ir = 0x4159;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D3E: dc.w $ffff
pub fn i_c19d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D40;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C19D40: ori.b #$86, d3
pub fn i_c19d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D44;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x86) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19D44: movep.w $434f(a0), d0
pub fn i_c19d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D48;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x434F);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C19D48: link.w a4, #$2eff
pub fn i_c19d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D4C;
    cpu.ir = 0x4E54;
    cpu.push32(bus, cpu.a[4]); cpu.a[4] = cpu.a[7]; cpu.a[7] = cpu.a[7].wrapping_add(0x2EFF);
    16
}
/// $C19D4C: ori.b #$86, d3
pub fn i_c19d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D50;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x86) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19D50: movep.w $4156(a0), d0
pub fn i_c19d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D54;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4156);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C19D54: movea.l (a4), a7
pub fn i_c19d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D56;
    cpu.ir = 0x2E54;
    let v2 = bus.read32(cpu.a[4]);
    cpu.a[7] = v2;
    12
}
/// $C19D56: ?
pub fn i_c19d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D58;
    cpu.ir = 0x494D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D5C: btst.l d0, d1
pub fn i_c19d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D5E;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C19D5E: btst.l d2, -(a1)
pub fn i_c19d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D60;
    cpu.ir = 0x521;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 8);
    let ad2 = cpu.a[1].wrapping_sub(1); cpu.a[1] = ad2;
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    14
}
/// $C19D60: move.l d2, d0
pub fn i_c19d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D62;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19D62: ori.b #$65, d3
pub fn i_c19d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D66;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x65) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19D66: ori.b #$4f, d7
pub fn i_c19d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D6A;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19D6A: ?
pub fn i_c19d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D6C;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D6C: ?
pub fn i_c19d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D6E;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D6E: ?
pub fn i_c19d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D70;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D70: addq.w #$8, a7
pub fn i_c19d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D72;
    cpu.ir = 0x504F;
    cpu.a[7] = cpu.a[7].wrapping_add(8);
    8
}
/// $C19D72: ?
pub fn i_c19d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D74;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D74: ?
pub fn i_c19d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D76;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D76: ?
pub fn i_c19d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D78;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D78: addq.w #$8, a7
pub fn i_c19d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D7A;
    cpu.ir = 0x504F;
    cpu.a[7] = cpu.a[7].wrapping_add(8);
    8
}
/// $C19D7A: ?
pub fn i_c19d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D7C;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D7C: ?
pub fn i_c19d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D7E;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19D7E: ?
pub fn i_c19d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19D80;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19E9A: addq.l #$1, d0
pub fn i_c19e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19E9C;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $C19E9C: or.l d0, d0
pub fn i_c19e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19E9E;
    cpu.ir = 0x8080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19E9E: or.l d0, d0
pub fn i_c19e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EA0;
    cpu.ir = 0x8080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19EA0: or.l d0, d0
pub fn i_c19ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EA2;
    cpu.ir = 0x8080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19EA2: addq.l #$1, d0
pub fn i_c19ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EA4;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $C19EA4: or.l d0, d0
pub fn i_c19ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EA6;
    cpu.ir = 0x8080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19EA6: or.l d0, d0
pub fn i_c19ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EA8;
    cpu.ir = 0x8080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19EA8: or.l d0, d0
pub fn i_c19ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EAA;
    cpu.ir = 0x8080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19EAA: addq.l #$1, d0
pub fn i_c19eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EAC;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $C19EAC: or.l d0, d0
pub fn i_c19eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EAE;
    cpu.ir = 0x8080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19EAE: or.l d0, d0
pub fn i_c19eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EB0;
    cpu.ir = 0x8080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19EB0: or.l d0, d0
pub fn i_c19eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EB2;
    cpu.ir = 0x8080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19EB2: addq.l #$1, d0
pub fn i_c19eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EB4;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $C19EB4: or.l d0, d0
pub fn i_c19eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EB6;
    cpu.ir = 0x8080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19EB6: or.l d0, d0
pub fn i_c19eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EB8;
    cpu.ir = 0x8080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19EB8: or.l d0, d0
pub fn i_c19eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EBA;
    cpu.ir = 0x8080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFFFFFF;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19EBA: addq.w #$1, (a3)
pub fn i_c19eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EBC;
    cpu.ir = 0x5253;
    let ad1 = cpu.a[3];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C19EBC: ?
pub fn i_c19ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EBE;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19EBE: ?
pub fn i_c19ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EC0;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19EC0: ?
pub fn i_c19ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EC2;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19EC2: ?
pub fn i_c19ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EC4;
    cpu.ir = 0x4F55;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19EC4: ?
pub fn i_c19ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EC6;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19EC6: ?
pub fn i_c19ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EC8;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19EC8: ?
pub fn i_c19ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19ECA;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19ECA: ?
pub fn i_c19eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19ECC;
    cpu.ir = 0x4F55;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19ECC: ?
pub fn i_c19ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19ECE;
    cpu.ir = 0x4F4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19EDC: jsr $c12a0e(pc)
pub fn i_c19edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19EE0);
    cpu.pc = 0xC12A0E;
    16
}
/// $C19EE0: addq.b #$1, $ac2(a5)
pub fn i_c19ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EE4;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19EE4: movea.l $abe(a5), a0
pub fn i_c19ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EE8;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C19EE8: move.l #$c1a288, (a0)+
pub fn i_c19ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EEE;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC1A288;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C19EEE: move.l a0, $abe(a5)
pub fn i_c19eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EF2;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C19EF2: subq.b #$1, $ac2(a5)
pub fn i_c19ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EF6;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19EF6: clr.l $9e2(a5)
pub fn i_c19ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19EFA;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C19EFA: move.b #$2, $9e8(a5)
pub fn i_c19efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F00;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C19F00: jsr $c1a21e(pc)
pub fn i_c19f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19F04);
    cpu.pc = 0xC1A21E;
    16
}
/// $C19F04: jsr $c17290(pc)
pub fn i_c19f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19F08);
    cpu.pc = 0xC17290;
    16
}
/// $C19F08: lea.l $d08000.l, a4
pub fn i_c19f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F0E;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0xD08000;
    20
}
/// $C19F0E: addq.w #$1, $9e2(a5)
pub fn i_c19f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F12;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C19F12: cmpi.w #$5, $9e2(a5)
pub fn i_c19f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F18;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Word, false);
    16
}
/// $C19F18: bne.b $c19f00
pub fn i_c19f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F1A;
    cpu.ir = 0x66E6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19F00; return 10; }
    8
}
/// $C19F1A: clr.l $9e2(a5)
pub fn i_c19f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F1E;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0x9E2);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C19F1E: clr.b $9e8(a5)
pub fn i_c19f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F22;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0x9E8);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C19F22: jsr $c1a240(pc)
pub fn i_c19f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19F26);
    cpu.pc = 0xC1A240;
    16
}
/// $C19F26: jsr $c19ff2(pc)
pub fn i_c19f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19F2A);
    cpu.pc = 0xC19FF2;
    16
}
/// $C19F2A: jsr $c19fe6(pc)
pub fn i_c19f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F2E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19F2E);
    cpu.pc = 0xC19FE6;
    16
}
/// $C19F2E: jsr $c1727e(pc)
pub fn i_c19f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC19F32);
    cpu.pc = 0xC1727E;
    16
}
/// $C19F32: move.b $a97(a5), d0
pub fn i_c19f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F36;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C19F36: btst.b #$6, d0
pub fn i_c19f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F3A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C19F3A: beq.b $c19f3e
pub fn i_c19f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F3C;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19F3E; return 10; }
    8
}
/// $C19F3C: rts
pub fn i_c19f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F3E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C19F3E: andi.b #$3, d0
pub fn i_c19f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F42;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19F42: beq.w $c19f60
pub fn i_c19f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F46;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19F60; return 10; }
    8
}
/// $C19F46: moveq #$1, d1
pub fn i_c19f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F48;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19F48: andi.b #$2, d0
pub fn i_c19f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F4C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19F4C: bne.b $c19f50
pub fn i_c19f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F4E;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19F50; return 10; }
    8
}
/// $C19F4E: moveq #$ff, d1
pub fn i_c19f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F50;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19F50: add.w $9e2(a5), d1
pub fn i_c19f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F54;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C19F54: cmpi.w #$5, d1
pub fn i_c19f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F58;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $C19F58: bcc.b $c19f2e
pub fn i_c19f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F5A;
    cpu.ir = 0x64D4;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC19F2E; return 10; }
    8
}
/// $C19F5A: move.w d1, $9e4(a5)
pub fn i_c19f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F5E;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x9E4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C19F5E: bra.b $c19f22
pub fn i_c19f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F60;
    cpu.ir = 0x60C2;
    cpu.pc = 0xC19F22;
    10
}
/// $C19F60: pea.l $c19f2e(pc)
pub fn i_c19f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F64;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC19F2E);
    24
}
/// $C19F64: move.w $9e2(a5), d0
pub fn i_c19f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F68;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19F68: add.w d0, d0
pub fn i_c19f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F6A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19F6A: add.w d0, d0
pub fn i_c19f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F6C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19F6C: movea.l $c19f72(pc, d0.w), a0
pub fn i_c19f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F70;
    cpu.ir = 0x207B;
    let ad2 = 0xC19F72_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C19F70: jmp (a0)
pub fn i_c19f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F72;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C19F72: ?
pub fn i_c19f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F74;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19F74: subx.l d6, d7
pub fn i_c19f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F76;
    cpu.ir = 0x9F86;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let h2 = cpu.d[6] & 0xFFFFFFFF;
    let r = subx_with_flags(cpu, h1, h2, x, Size::Long);
    cpu.d[7] = r;
    8
}
/// $C19F76: ?
pub fn i_c19f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F78;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19F78: subx.l d6, d7
pub fn i_c19f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F7A;
    cpu.ir = 0x9F86;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let h2 = cpu.d[6] & 0xFFFFFFFF;
    let r = subx_with_flags(cpu, h1, h2, x, Size::Long);
    cpu.d[7] = r;
    8
}
/// $C19F7A: ?
pub fn i_c19f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F7C;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19F7C: dc.w $a07a
pub fn i_c19f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F7E;
    cpu.ir = 0xA07A;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C19F7E: ?
pub fn i_c19f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F80;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19F80: dc.w $a0f6
pub fn i_c19f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F82;
    cpu.ir = 0xA0F6;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C19F82: ?
pub fn i_c19f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F84;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C19F84: dc.w $a1a2
pub fn i_c19f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F86;
    cpu.ir = 0xA1A2;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineA);
    return 34;
}
/// $C19F86: move.b $a98(a5), d0
pub fn i_c19f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F8A;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C19F8A: andi.b #$30, d0
pub fn i_c19f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F8E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19F8E: bne.w $c19fa8
pub fn i_c19f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19FA8; return 10; }
    8
}
/// $C19F92: move.b $a97(a5), d0
pub fn i_c19f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F96;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C19F96: andi.b #$c, d0
pub fn i_c19f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F9A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19F9A: beq.w $c1a02c
pub fn i_c19f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19F9E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A02C; return 10; }
    8
}
/// $C19F9E: bchg.b #$0, $9e8(a5)
pub fn i_c19f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FA4;
    cpu.ir = 0x86D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[5].wrapping_add(0x9E8);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $C19FA4: bra.w $c19fe0
pub fn i_c19fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FA8;
    cpu.ir = 0x6000;
    cpu.pc = 0xC19FE0;
    10
}
/// $C19FA8: move.b d0, $3a001d.l
pub fn i_c19fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FAE;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C19FAE: moveq #$1, d1
pub fn i_c19fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FB0;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19FB0: andi.b #$10, d0
pub fn i_c19fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FB4;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C19FB4: bne.b $c19fb8
pub fn i_c19fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FB6;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19FB8; return 10; }
    8
}
/// $C19FB6: moveq #$ff, d1
pub fn i_c19fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FB8;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19FB8: lea.l -$7fc6(a4), a3
pub fn i_c19fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FBC;
    cpu.ir = 0x47EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF803A);
    cpu.a[3] = ad1;
    16
}
/// $C19FBC: move.w $9e2(a5), d0
pub fn i_c19fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FC0;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19FC0: add.w d0, d0
pub fn i_c19fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FC2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C19FC2: adda.l d0, a3
pub fn i_c19fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FC4;
    cpu.ir = 0xD7C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $C19FC4: btst.b #$0, $9e8(a5)
pub fn i_c19fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FCA;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C19FCA: beq.b $c19fce
pub fn i_c19fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FCC;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19FCE; return 10; }
    8
}
/// $C19FCC: addq.l #$1, a3
pub fn i_c19fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FCE;
    cpu.ir = 0x528B;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    8
}
/// $C19FCE: move.b (a3), d0
pub fn i_c19fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FD0;
    cpu.ir = 0x1013;
    let v2 = bus.read8(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $C19FD0: add.b d1, d0
pub fn i_c19fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FD2;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C19FD2: bne.b $c19fd6
pub fn i_c19fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FD4;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC19FD6; return 10; }
    8
}
/// $C19FD4: moveq #$9, d0
pub fn i_c19fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FD6;
    cpu.ir = 0x7009;
    cpu.d[0] = 0x9;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19FD6: cmpi.b #$a, d0
pub fn i_c19fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FDA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xA, Size::Byte, false);
    8
}
/// $C19FDA: bcs.b $c19fde
pub fn i_c19fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FDC;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC19FDE; return 10; }
    8
}
/// $C19FDC: moveq #$1, d0
pub fn i_c19fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FDE;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19FDE: move.b d0, (a3)
pub fn i_c19fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FE0;
    cpu.ir = 0x1680;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[3], v1 as u8);
    8
}
/// $C19FE0: move.w $9e2(a5), d0
pub fn i_c19fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FE4;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C19FE4: beq.b $c19ff2
pub fn i_c19fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FE6;
    cpu.ir = 0x670C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC19FF2; return 10; }
    8
}
/// $C19FE6: lea.l -$7fc4(a4), a3
pub fn i_c19fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FEA;
    cpu.ir = 0x47EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF803C);
    cpu.a[3] = ad1;
    16
}
/// $C19FEA: moveq #$1, d2
pub fn i_c19fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FEC;
    cpu.ir = 0x7401;
    cpu.d[2] = 0x1;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19FEC: move.w #$700b, d6
pub fn i_c19fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FF0;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x700B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19FF0: bra.b $c19ffc
pub fn i_c19ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FF2;
    cpu.ir = 0x600A;
    cpu.pc = 0xC19FFC;
    10
}
/// $C19FF2: lea.l -$7fc6(a4), a3
pub fn i_c19ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FF6;
    cpu.ir = 0x47EC;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF803A);
    cpu.a[3] = ad1;
    16
}
/// $C19FF6: move.w #$7008, d6
pub fn i_c19ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FFA;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x7008;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C19FFA: moveq #$0, d2
pub fn i_c19ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC19FFC;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C19FFC: addq.b #$1, $ac2(a5)
pub fn i_c19ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A000;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A000: movea.l $abe(a5), a0
pub fn i_c1a000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A004;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A004: clr.l (a0)+
pub fn i_c1a004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A006;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C1A006: move.w #$1e0, d4
pub fn i_c1a006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A00A;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1E0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A00A: move.w #$280, d3
pub fn i_c1a00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A00E;
    cpu.ir = 0x363C;
    let v1: u32 = 0x280;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A00E: moveq #$0, d7
pub fn i_c1a00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A010;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A010: jsr $c1a02e(pc)
pub fn i_c1a010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A014;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A014);
    cpu.pc = 0xC1A02E;
    16
}
/// $C1A014: move.w #$2c0, d4
pub fn i_c1a014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A018;
    cpu.ir = 0x383C;
    let v1: u32 = 0x2C0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A018: move.w #$3a0, d3
pub fn i_c1a018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A01C;
    cpu.ir = 0x363C;
    let v1: u32 = 0x3A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1A01C: moveq #$1, d7
pub fn i_c1a01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A01E;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A01E: jsr $c1a02e(pc)
pub fn i_c1a01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A022;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1A022);
    cpu.pc = 0xC1A02E;
    16
}
/// $C1A022: clr.w (a0)+
pub fn i_c1a022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A024;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1A024: move.l a0, $abe(a5)
pub fn i_c1a024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A028;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A028: subq.b #$1, $ac2(a5)
pub fn i_c1a028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A02C;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A02C: rts
pub fn i_c1a02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A02E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A02E: move.w #$3, (a0)+
pub fn i_c1a02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A032;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A032: add.w d6, d4
pub fn i_c1a032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A034;
    cpu.ir = 0xD846;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1A034: move.w d4, (a0)+
pub fn i_c1a034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A036;
    cpu.ir = 0x30C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A036: move.w #$108, (a0)+
pub fn i_c1a036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A03A;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A03A: cmp.b $9e8(a5), d7
pub fn i_c1a03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A03E;
    cpu.ir = 0xBE2D;
    let ad2 = cpu.a[5].wrapping_add(0x9E8);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $C1A03E: bne.b $c1a04c
pub fn i_c1a03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A040;
    cpu.ir = 0x660C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A04C; return 10; }
    8
}
/// $C1A040: cmp.w $9e2(a5), d2
pub fn i_c1a040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A044;
    cpu.ir = 0xB46D;
    let ad2 = cpu.a[5].wrapping_add(0x9E2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C1A044: bne.b $c1a04c
pub fn i_c1a044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A046;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A04C; return 10; }
    8
}
/// $C1A046: move.w #$1108, -$2(a0)
pub fn i_c1a046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A04C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1A04C: move.b (a3)+, d1
pub fn i_c1a04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A04E;
    cpu.ir = 0x121B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C1A04E: addi.b #$30, d1
pub fn i_c1a04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A052;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A052: move.b d1, (a0)+
pub fn i_c1a052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A054;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A054: move.b #$ff, (a0)+
pub fn i_c1a054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A058;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A058: move.w #$3, (a0)+
pub fn i_c1a058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A05C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A05C: add.w d6, d3
pub fn i_c1a05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A05E;
    cpu.ir = 0xD646;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1A05E: move.w d3, (a0)+
pub fn i_c1a05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A060;
    cpu.ir = 0x30C3;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A060: move.w #$108, (a0)+
pub fn i_c1a060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A064;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1A064: move.b #$53, (a0)+
pub fn i_c1a064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A068;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x53;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A068: cmpi.b #$31, d1
pub fn i_c1a068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A06C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x31, Size::Byte, false);
    8
}
/// $C1A06C: bne.b $c1a074
pub fn i_c1a06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A06E;
    cpu.ir = 0x6606;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A074; return 10; }
    8
}
/// $C1A06E: move.b #$20, -$1(a0)
pub fn i_c1a06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A074;
    cpu.ir = 0x117C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1A074: move.b #$ff, (a0)+
pub fn i_c1a074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A078;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1A078: rts
pub fn i_c1a078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A07A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A07A: move.b $a97(a5), d0
pub fn i_c1a07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A07E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A07E: andi.b #$30, d0
pub fn i_c1a07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A082;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A082: beq.w $c1a0bc
pub fn i_c1a082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A086;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A0BC; return 10; }
    8
}
/// $C1A086: move.b d0, $3a001d.l
pub fn i_c1a086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A08C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C1A08C: move.b -$7fbe(a4), d0
pub fn i_c1a08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A090;
    cpu.ir = 0x102C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8042);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A090: beq.b $c1a096
pub fn i_c1a090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A092;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A096; return 10; }
    8
}
/// $C1A092: moveq #$0, d0
pub fn i_c1a092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A094;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A094: bra.b $c1a098
pub fn i_c1a094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A096;
    cpu.ir = 0x6002;
    cpu.pc = 0xC1A098;
    10
}
/// $C1A096: moveq #$1, d0
pub fn i_c1a096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A098;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A098: move.b d0, -$7fbe(a4)
pub fn i_c1a098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A09C;
    cpu.ir = 0x1940;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8042);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1A09C: addq.b #$1, $ac2(a5)
pub fn i_c1a09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0A0;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A0A0: movea.l $abe(a5), a0
pub fn i_c1a0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0A4;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1A0A4: lea.l $c1a0be(pc), a1
pub fn i_c1a0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC1A0BE;
    16
}
/// $C1A0A8: tst.b -$7fbe(a4)
pub fn i_c1a0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0AC;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8042);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1A0AC: beq.b $c1a0b2
pub fn i_c1a0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0AE;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A0B2; return 10; }
    8
}
/// $C1A0AE: lea.l $c1a0da(pc), a1
pub fn i_c1a0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC1A0DA;
    16
}
/// $C1A0B2: move.l a1, (a0)+
pub fn i_c1a0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0B4;
    cpu.ir = 0x20C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1A0B4: move.l a0, $abe(a5)
pub fn i_c1a0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0B8;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1A0B8: subq.b #$1, $ac2(a5)
pub fn i_c1a0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0BC;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A0BC: rts
pub fn i_c1a0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1A0BE: ori.b #$4e, d3
pub fn i_c1a0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0C2;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x4E) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A0C2: movep.w $4f4e(a0), d0
pub fn i_c1a0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0C6;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4F4E);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1A0C6: ?
pub fn i_c1a0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0C8;
    cpu.ir = 0x4C59;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A0C8: movea.l (a7), a0
pub fn i_c1a0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0CA;
    cpu.ir = 0x2057;
    let v2 = bus.read32(cpu.a[7]);
    cpu.a[0] = v2;
    12
}
/// $C1A0CA: swap d5
pub fn i_c1a0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0CC;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A0CC: ?
pub fn i_c1a0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0CE;
    cpu.ir = 0x4E20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A0CE: ?
pub fn i_c1a0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0D0;
    cpu.ir = 0x4352;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A0D0: ?
pub fn i_c1a0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0D2;
    cpu.ir = 0x4544;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A0D2: ?
pub fn i_c1a0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0D4;
    cpu.ir = 0x4954;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A0D4: ?
pub fn i_c1a0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0D6;
    cpu.ir = 0x4544;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A0D6: dc.w $ffff
pub fn i_c1a0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0D8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C1A0D8: ori.b #$3, d0
pub fn i_c1a0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0DC;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A0DC: moveq #$4e, d1
pub fn i_c1a0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0DE;
    cpu.ir = 0x724E;
    cpu.d[1] = 0x4E;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A0DE: movep.w $4652(a0), d0
pub fn i_c1a0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0E2;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4652);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1A0E2: ?
pub fn i_c1a0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0E4;
    cpu.ir = 0x4545;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1A0F6: move.b $a98(a5), d0
pub fn i_c1a0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0FA;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A0FA: andi.b #$30, d0
pub fn i_c1a0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A0FE;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A0FE: beq.w $c1a190
pub fn i_c1a0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A102;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1A190; return 10; }
    8
}
/// $C1A102: move.b d0, $3a001d.l
pub fn i_c1a102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A108;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001D, v1 as u8);
    16
}
/// $C1A108: moveq #$1, d1
pub fn i_c1a108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A10A;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1A10A: andi.b #$10, d0
pub fn i_c1a10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A10E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1A10E: bne.b $c1a114
pub fn i_c1a10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A110;
    cpu.ir = 0x6604;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1A114; return 10; }
    8
}
/// $C1A110: move.b #$99, d1
pub fn i_c1a110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A114;
    cpu.ir = 0x123C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1A114: move.b -$7fbc(a4), d2
pub fn i_c1a114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A118;
    cpu.ir = 0x142C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8044);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1A118: sub.b d0, d0
pub fn i_c1a118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A11A;
    cpu.ir = 0x9000;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C1A11A: abcd.b d1, d2
pub fn i_c1a11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A11C;
    cpu.ir = 0xC501;
    let s = cpu.d[1] as u8; let d = cpu.d[2] as u8;
    let x = cpu.sr.get(StatusRegister::X) as u8;
    let (r, carry, vf) = abcd_calc(d, s, x);
    if r != 0 { cpu.sr.set(StatusRegister::Z, false); }
    cpu.sr.set(StatusRegister::N, r & 0x80 != 0); cpu.sr.set(StatusRegister::V, vf);
    cpu.sr.set(StatusRegister::C, carry); cpu.sr.set(StatusRegister::X, carry);
    cpu.d[2] = (cpu.d[2] & !0xFF) | r as u32;
    6
}
/// $C1A11C: cmpi.b #$60, d2
pub fn i_c1a11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A120;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x60, Size::Byte, false);
    8
}
/// $C1A120: bcc.b $c1a118
pub fn i_c1a120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A122;
    cpu.ir = 0x64F6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1A118; return 10; }
    8
}
/// $C1A122: move.b d2, -$7fbc(a4)
pub fn i_c1a122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A126;
    cpu.ir = 0x1942;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8044);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1A126: move.b #$3b, -$7fbb(a4)
pub fn i_c1a126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A12C;
    cpu.ir = 0x197C;
    let v1: u32 = 0x3B;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0xFFFF8045);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1A12C: clr.b -$7fbd(a4)
pub fn i_c1a12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A130;
    cpu.ir = 0x422C;
    let ad1 = cpu.a[4].wrapping_add(0xFFFF8043);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1A130: tst.b d2
pub fn i_c1a130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1A132;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0xC18C7A, i_c18c7a),
    (0xC18C7E, i_c18c7e),
    (0xC18C82, i_c18c82),
    (0xC18C84, i_c18c84),
    (0xC18C86, i_c18c86),
    (0xC18C8A, i_c18c8a),
    (0xC18C8E, i_c18c8e),
    (0xC18C92, i_c18c92),
    (0xC18C94, i_c18c94),
    (0xC18C9A, i_c18c9a),
    (0xC18C9E, i_c18c9e),
    (0xC18CA0, i_c18ca0),
    (0xC18CA4, i_c18ca4),
    (0xC18CA8, i_c18ca8),
    (0xC18CAC, i_c18cac),
    (0xC18CB2, i_c18cb2),
    (0xC18CB6, i_c18cb6),
    (0xC18CB8, i_c18cb8),
    (0xC18CBA, i_c18cba),
    (0xC18CBC, i_c18cbc),
    (0xC18CBE, i_c18cbe),
    (0xC18CC2, i_c18cc2),
    (0xC18CC6, i_c18cc6),
    (0xC18CCA, i_c18cca),
    (0xC18CCE, i_c18cce),
    (0xC18CD2, i_c18cd2),
    (0xC18CD4, i_c18cd4),
    (0xC18CD8, i_c18cd8),
    (0xC18CDA, i_c18cda),
    (0xC18CDE, i_c18cde),
    (0xC18CE2, i_c18ce2),
    (0xC18CE4, i_c18ce4),
    (0xC18CE8, i_c18ce8),
    (0xC18CEC, i_c18cec),
    (0xC18CEE, i_c18cee),
    (0xC18CF0, i_c18cf0),
    (0xC18CF2, i_c18cf2),
    (0xC18CF4, i_c18cf4),
    (0xC18CF6, i_c18cf6),
    (0xC18CF8, i_c18cf8),
    (0xC18CFA, i_c18cfa),
    (0xC18CFC, i_c18cfc),
    (0xC18CFE, i_c18cfe),
    (0xC18D02, i_c18d02),
    (0xC18D04, i_c18d04),
    (0xC18D08, i_c18d08),
    (0xC18D0A, i_c18d0a),
    (0xC18D1C, i_c18d1c),
    (0xC18D20, i_c18d20),
    (0xC18D24, i_c18d24),
    (0xC18D28, i_c18d28),
    (0xC18D2E, i_c18d2e),
    (0xC18D30, i_c18d30),
    (0xC18D34, i_c18d34),
    (0xC18D36, i_c18d36),
    (0xC18D3A, i_c18d3a),
    (0xC18D3E, i_c18d3e),
    (0xC18D40, i_c18d40),
    (0xC18D42, i_c18d42),
    (0xC18D46, i_c18d46),
    (0xC18D48, i_c18d48),
    (0xC18D4C, i_c18d4c),
    (0xC18D52, i_c18d52),
    (0xC18D56, i_c18d56),
    (0xC18D58, i_c18d58),
    (0xC18D5A, i_c18d5a),
    (0xC18D60, i_c18d60),
    (0xC18D64, i_c18d64),
    (0xC18D68, i_c18d68),
    (0xC18D6C, i_c18d6c),
    (0xC18D70, i_c18d70),
    (0xC18D72, i_c18d72),
    (0xC18D76, i_c18d76),
    (0xC18D7A, i_c18d7a),
    (0xC18D7E, i_c18d7e),
    (0xC18D82, i_c18d82),
    (0xC18D86, i_c18d86),
    (0xC18D88, i_c18d88),
    (0xC18D8A, i_c18d8a),
    (0xC18D8E, i_c18d8e),
    (0xC18D92, i_c18d92),
    (0xC18D96, i_c18d96),
    (0xC18D9A, i_c18d9a),
    (0xC18D9E, i_c18d9e),
    (0xC18DA2, i_c18da2),
    (0xC18DA4, i_c18da4),
    (0xC18DA8, i_c18da8),
    (0xC18DAE, i_c18dae),
    (0xC18DB2, i_c18db2),
    (0xC18DB6, i_c18db6),
    (0xC18DB8, i_c18db8),
    (0xC18DBC, i_c18dbc),
    (0xC18DC0, i_c18dc0),
    (0xC18DC2, i_c18dc2),
    (0xC18DC8, i_c18dc8),
    (0xC18DCC, i_c18dcc),
    (0xC18DD0, i_c18dd0),
    (0xC18DD4, i_c18dd4),
    (0xC18DDA, i_c18dda),
    (0xC18DDE, i_c18dde),
    (0xC18DE0, i_c18de0),
    (0xC18DE2, i_c18de2),
    (0xC18DE4, i_c18de4),
    (0xC18DE6, i_c18de6),
    (0xC18DEA, i_c18dea),
    (0xC18DEE, i_c18dee),
    (0xC18DF2, i_c18df2),
    (0xC18DF6, i_c18df6),
    (0xC18DFA, i_c18dfa),
    (0xC18DFC, i_c18dfc),
    (0xC18E00, i_c18e00),
    (0xC18E02, i_c18e02),
    (0xC18E06, i_c18e06),
    (0xC18E0A, i_c18e0a),
    (0xC18E0C, i_c18e0c),
    (0xC18E10, i_c18e10),
    (0xC18E14, i_c18e14),
    (0xC18E16, i_c18e16),
    (0xC18E18, i_c18e18),
    (0xC18E1A, i_c18e1a),
    (0xC18E1C, i_c18e1c),
    (0xC18E1E, i_c18e1e),
    (0xC18E20, i_c18e20),
    (0xC18E22, i_c18e22),
    (0xC18E24, i_c18e24),
    (0xC18E26, i_c18e26),
    (0xC18E2A, i_c18e2a),
    (0xC18E2C, i_c18e2c),
    (0xC18E30, i_c18e30),
    (0xC18E32, i_c18e32),
    (0xC18E34, i_c18e34),
    (0xC18E36, i_c18e36),
    (0xC18E38, i_c18e38),
    (0xC18E3A, i_c18e3a),
    (0xC18E3C, i_c18e3c),
    (0xC18E3E, i_c18e3e),
    (0xC18E44, i_c18e44),
    (0xC18E48, i_c18e48),
    (0xC18E4A, i_c18e4a),
    (0xC18E4C, i_c18e4c),
    (0xC18E50, i_c18e50),
    (0xC18E52, i_c18e52),
    (0xC18E54, i_c18e54),
    (0xC18E56, i_c18e56),
    (0xC18E58, i_c18e58),
    (0xC18E5A, i_c18e5a),
    (0xC18E5C, i_c18e5c),
    (0xC18E5E, i_c18e5e),
    (0xC18E60, i_c18e60),
    (0xC18E62, i_c18e62),
    (0xC18E64, i_c18e64),
    (0xC18E66, i_c18e66),
    (0xC18E6A, i_c18e6a),
    (0xC18E6C, i_c18e6c),
    (0xC18E70, i_c18e70),
    (0xC18E74, i_c18e74),
    (0xC18E7A, i_c18e7a),
    (0xC18E7E, i_c18e7e),
    (0xC18E82, i_c18e82),
    (0xC18E84, i_c18e84),
    (0xC18E88, i_c18e88),
    (0xC18E8C, i_c18e8c),
    (0xC18E8E, i_c18e8e),
    (0xC18E92, i_c18e92),
    (0xC18E96, i_c18e96),
    (0xC18E98, i_c18e98),
    (0xC18E9A, i_c18e9a),
    (0xC18E9C, i_c18e9c),
    (0xC18EA0, i_c18ea0),
    (0xC18EA2, i_c18ea2),
    (0xC18EA6, i_c18ea6),
    (0xC18EA8, i_c18ea8),
    (0xC18EAA, i_c18eaa),
    (0xC18EAE, i_c18eae),
    (0xC18EB2, i_c18eb2),
    (0xC18EB4, i_c18eb4),
    (0xC18EB6, i_c18eb6),
    (0xC18EB8, i_c18eb8),
    (0xC18EBA, i_c18eba),
    (0xC18EBC, i_c18ebc),
    (0xC18EBE, i_c18ebe),
    (0xC18EC0, i_c18ec0),
    (0xC18EC2, i_c18ec2),
    (0xC18EC4, i_c18ec4),
    (0xC18EC6, i_c18ec6),
    (0xC18EC8, i_c18ec8),
    (0xC18ECC, i_c18ecc),
    (0xC18ED0, i_c18ed0),
    (0xC18ED2, i_c18ed2),
    (0xC18ED4, i_c18ed4),
    (0xC18ED6, i_c18ed6),
    (0xC18F2C, i_c18f2c),
    (0xC18F32, i_c18f32),
    (0xC18F36, i_c18f36),
    (0xC18F3C, i_c18f3c),
    (0xC18F42, i_c18f42),
    (0xC18F46, i_c18f46),
    (0xC18F4A, i_c18f4a),
    (0xC18F4C, i_c18f4c),
    (0xC18F50, i_c18f50),
    (0xC18F52, i_c18f52),
    (0xC18F56, i_c18f56),
    (0xC18F5A, i_c18f5a),
    (0xC18F5C, i_c18f5c),
    (0xC18F62, i_c18f62),
    (0xC18F68, i_c18f68),
    (0xC18F6A, i_c18f6a),
    (0xC18F70, i_c18f70),
    (0xC18F72, i_c18f72),
    (0xC18F74, i_c18f74),
    (0xC18F7A, i_c18f7a),
    (0xC18F7C, i_c18f7c),
    (0xC18F7E, i_c18f7e),
    (0xC18F82, i_c18f82),
    (0xC18F86, i_c18f86),
    (0xC18F8A, i_c18f8a),
    (0xC18F8C, i_c18f8c),
    (0xC18F8E, i_c18f8e),
    (0xC18F90, i_c18f90),
    (0xC18F92, i_c18f92),
    (0xC18F94, i_c18f94),
    (0xC18F96, i_c18f96),
    (0xC18F98, i_c18f98),
    (0xC18F9A, i_c18f9a),
    (0xC18F9C, i_c18f9c),
    (0xC18F9E, i_c18f9e),
    (0xC18FA0, i_c18fa0),
    (0xC18FA2, i_c18fa2),
    (0xC18FA6, i_c18fa6),
    (0xC18FAA, i_c18faa),
    (0xC18FAC, i_c18fac),
    (0xC18FAE, i_c18fae),
    (0xC18FB0, i_c18fb0),
    (0xC18FB2, i_c18fb2),
    (0xC18FB4, i_c18fb4),
    (0xC18FB6, i_c18fb6),
    (0xC18FB8, i_c18fb8),
    (0xC18FBC, i_c18fbc),
    (0xC18FC0, i_c18fc0),
    (0xC18FC2, i_c18fc2),
    (0xC18FC4, i_c18fc4),
    (0xC18FC6, i_c18fc6),
    (0xC18FC8, i_c18fc8),
    (0xC18FCA, i_c18fca),
    (0xC18FCC, i_c18fcc),
    (0xC18FCE, i_c18fce),
    (0xC18FD2, i_c18fd2),
    (0xC18FD6, i_c18fd6),
    (0xC18FD8, i_c18fd8),
    (0xC18FDA, i_c18fda),
    (0xC18FDC, i_c18fdc),
    (0xC18FDE, i_c18fde),
    (0xC18FE0, i_c18fe0),
    (0xC18FE2, i_c18fe2),
    (0xC18FE4, i_c18fe4),
    (0xC18FE8, i_c18fe8),
    (0xC18FEC, i_c18fec),
    (0xC18FEE, i_c18fee),
    (0xC18FF0, i_c18ff0),
    (0xC18FF2, i_c18ff2),
    (0xC18FF4, i_c18ff4),
    (0xC18FF8, i_c18ff8),
    (0xC18FFC, i_c18ffc),
    (0xC18FFE, i_c18ffe),
    (0xC19000, i_c19000),
    (0xC19002, i_c19002),
    (0xC19004, i_c19004),
    (0xC19006, i_c19006),
    (0xC19008, i_c19008),
    (0xC1900A, i_c1900a),
    (0xC1900C, i_c1900c),
    (0xC1900E, i_c1900e),
    (0xC19010, i_c19010),
    (0xC19012, i_c19012),
    (0xC19014, i_c19014),
    (0xC19016, i_c19016),
    (0xC1901A, i_c1901a),
    (0xC1901C, i_c1901c),
    (0xC19022, i_c19022),
    (0xC19026, i_c19026),
    (0xC1902A, i_c1902a),
    (0xC19030, i_c19030),
    (0xC19034, i_c19034),
    (0xC19038, i_c19038),
    (0xC1903A, i_c1903a),
    (0xC1903E, i_c1903e),
    (0xC19042, i_c19042),
    (0xC19046, i_c19046),
    (0xC19048, i_c19048),
    (0xC1904A, i_c1904a),
    (0xC1904C, i_c1904c),
    (0xC1904E, i_c1904e),
    (0xC19052, i_c19052),
    (0xC19056, i_c19056),
    (0xC1905A, i_c1905a),
    (0xC1905E, i_c1905e),
    (0xC19062, i_c19062),
    (0xC19064, i_c19064),
    (0xC19068, i_c19068),
    (0xC1906C, i_c1906c),
    (0xC19070, i_c19070),
    (0xC19074, i_c19074),
    (0xC19078, i_c19078),
    (0xC1907C, i_c1907c),
    (0xC19080, i_c19080),
    (0xC19084, i_c19084),
    (0xC19088, i_c19088),
    (0xC1908C, i_c1908c),
    (0xC19090, i_c19090),
    (0xC19094, i_c19094),
    (0xC19096, i_c19096),
    (0xC1909A, i_c1909a),
    (0xC1909E, i_c1909e),
    (0xC190A0, i_c190a0),
    (0xC190A2, i_c190a2),
    (0xC190A6, i_c190a6),
    (0xC190AA, i_c190aa),
    (0xC190AC, i_c190ac),
    (0xC190B0, i_c190b0),
    (0xC190B4, i_c190b4),
    (0xC190B6, i_c190b6),
    (0xC190B8, i_c190b8),
    (0xC190BA, i_c190ba),
    (0xC190BC, i_c190bc),
    (0xC190C0, i_c190c0),
    (0xC190C2, i_c190c2),
    (0xC190C6, i_c190c6),
    (0xC190C8, i_c190c8),
    (0xC190CA, i_c190ca),
    (0xC190CC, i_c190cc),
    (0xC190CE, i_c190ce),
    (0xC190D2, i_c190d2),
    (0xC190D4, i_c190d4),
    (0xC190D6, i_c190d6),
    (0xC190D8, i_c190d8),
    (0xC190DA, i_c190da),
    (0xC190DE, i_c190de),
    (0xC190E2, i_c190e2),
    (0xC190E4, i_c190e4),
    (0xC190E6, i_c190e6),
    (0xC190E8, i_c190e8),
    (0xC190EC, i_c190ec),
    (0xC190EE, i_c190ee),
    (0xC190F0, i_c190f0),
    (0xC190F2, i_c190f2),
    (0xC190F6, i_c190f6),
    (0xC190FC, i_c190fc),
    (0xC190FE, i_c190fe),
    (0xC19102, i_c19102),
    (0xC19104, i_c19104),
    (0xC19106, i_c19106),
    (0xC1910A, i_c1910a),
    (0xC1910E, i_c1910e),
    (0xC19112, i_c19112),
    (0xC19116, i_c19116),
    (0xC19118, i_c19118),
    (0xC1911E, i_c1911e),
    (0xC19122, i_c19122),
    (0xC19126, i_c19126),
    (0xC1912A, i_c1912a),
    (0xC1912E, i_c1912e),
    (0xC19132, i_c19132),
    (0xC19136, i_c19136),
    (0xC1913A, i_c1913a),
    (0xC19140, i_c19140),
    (0xC19146, i_c19146),
    (0xC1914A, i_c1914a),
    (0xC1914E, i_c1914e),
    (0xC19152, i_c19152),
    (0xC19154, i_c19154),
    (0xC19158, i_c19158),
    (0xC1915C, i_c1915c),
    (0xC19160, i_c19160),
    (0xC19164, i_c19164),
    (0xC19166, i_c19166),
    (0xC1916A, i_c1916a),
    (0xC1916C, i_c1916c),
    (0xC1916E, i_c1916e),
    (0xC19172, i_c19172),
    (0xC19176, i_c19176),
    (0xC19178, i_c19178),
    (0xC1917C, i_c1917c),
    (0xC19180, i_c19180),
    (0xC19182, i_c19182),
    (0xC19186, i_c19186),
    (0xC19188, i_c19188),
    (0xC1918C, i_c1918c),
    (0xC19190, i_c19190),
    (0xC19194, i_c19194),
    (0xC19198, i_c19198),
    (0xC1919C, i_c1919c),
    (0xC1919E, i_c1919e),
    (0xC191A0, i_c191a0),
    (0xC191A6, i_c191a6),
    (0xC191A8, i_c191a8),
    (0xC191AC, i_c191ac),
    (0xC191B0, i_c191b0),
    (0xC191B4, i_c191b4),
    (0xC191B8, i_c191b8),
    (0xC191BA, i_c191ba),
    (0xC191BC, i_c191bc),
    (0xC191C2, i_c191c2),
    (0xC191C4, i_c191c4),
    (0xC191C8, i_c191c8),
    (0xC191CA, i_c191ca),
    (0xC191CE, i_c191ce),
    (0xC191D2, i_c191d2),
    (0xC191D6, i_c191d6),
    (0xC191D8, i_c191d8),
    (0xC191DA, i_c191da),
    (0xC191DE, i_c191de),
    (0xC191E2, i_c191e2),
    (0xC191E6, i_c191e6),
    (0xC191EA, i_c191ea),
    (0xC191EE, i_c191ee),
    (0xC191F4, i_c191f4),
    (0xC191F8, i_c191f8),
    (0xC191FC, i_c191fc),
    (0xC19200, i_c19200),
    (0xC19204, i_c19204),
    (0xC19208, i_c19208),
    (0xC1920C, i_c1920c),
    (0xC19210, i_c19210),
    (0xC19214, i_c19214),
    (0xC19218, i_c19218),
    (0xC1921C, i_c1921c),
    (0xC1921E, i_c1921e),
    (0xC19222, i_c19222),
    (0xC19224, i_c19224),
    (0xC19226, i_c19226),
    (0xC1922A, i_c1922a),
    (0xC1922E, i_c1922e),
    (0xC19232, i_c19232),
    (0xC19236, i_c19236),
    (0xC1923A, i_c1923a),
    (0xC1923C, i_c1923c),
    (0xC1923E, i_c1923e),
    (0xC19240, i_c19240),
    (0xC19242, i_c19242),
    (0xC19244, i_c19244),
    (0xC19246, i_c19246),
    (0xC1924A, i_c1924a),
    (0xC1924E, i_c1924e),
    (0xC19252, i_c19252),
    (0xC19256, i_c19256),
    (0xC19258, i_c19258),
    (0xC1925C, i_c1925c),
    (0xC1925E, i_c1925e),
    (0xC19262, i_c19262),
    (0xC19266, i_c19266),
    (0xC1926A, i_c1926a),
    (0xC1926E, i_c1926e),
    (0xC19270, i_c19270),
    (0xC19272, i_c19272),
    (0xC19276, i_c19276),
    (0xC19278, i_c19278),
    (0xC1927C, i_c1927c),
    (0xC19280, i_c19280),
    (0xC19284, i_c19284),
    (0xC19286, i_c19286),
    (0xC19288, i_c19288),
    (0xC1928A, i_c1928a),
    (0xC1928E, i_c1928e),
    (0xC19290, i_c19290),
    (0xC19294, i_c19294),
    (0xC19296, i_c19296),
    (0xC1929A, i_c1929a),
    (0xC1929E, i_c1929e),
    (0xC192A2, i_c192a2),
    (0xC192A4, i_c192a4),
    (0xC192A8, i_c192a8),
    (0xC192AA, i_c192aa),
    (0xC192AC, i_c192ac),
    (0xC192B0, i_c192b0),
    (0xC192B2, i_c192b2),
    (0xC192B4, i_c192b4),
    (0xC192B8, i_c192b8),
    (0xC192BA, i_c192ba),
    (0xC192BE, i_c192be),
    (0xC192C2, i_c192c2),
    (0xC192C6, i_c192c6),
    (0xC192C8, i_c192c8),
    (0xC192CC, i_c192cc),
    (0xC192CE, i_c192ce),
    (0xC192D0, i_c192d0),
    (0xC192D4, i_c192d4),
    (0xC192D6, i_c192d6),
    (0xC192DA, i_c192da),
    (0xC192DC, i_c192dc),
    (0xC192E0, i_c192e0),
    (0xC192E4, i_c192e4),
    (0xC192E8, i_c192e8),
    (0xC192EC, i_c192ec),
    (0xC192F0, i_c192f0),
    (0xC192F2, i_c192f2),
    (0xC192F6, i_c192f6),
    (0xC192FA, i_c192fa),
    (0xC192FC, i_c192fc),
    (0xC192FE, i_c192fe),
    (0xC19300, i_c19300),
    (0xC19304, i_c19304),
    (0xC19308, i_c19308),
    (0xC1930C, i_c1930c),
    (0xC19310, i_c19310),
    (0xC19314, i_c19314),
    (0xC19318, i_c19318),
    (0xC1931A, i_c1931a),
    (0xC1931E, i_c1931e),
    (0xC19320, i_c19320),
    (0xC19324, i_c19324),
    (0xC19328, i_c19328),
    (0xC1932A, i_c1932a),
    (0xC1932E, i_c1932e),
    (0xC19332, i_c19332),
    (0xC19334, i_c19334),
    (0xC19336, i_c19336),
    (0xC19338, i_c19338),
    (0xC1933A, i_c1933a),
    (0xC1933E, i_c1933e),
    (0xC19340, i_c19340),
    (0xC19344, i_c19344),
    (0xC19346, i_c19346),
    (0xC19348, i_c19348),
    (0xC1934A, i_c1934a),
    (0xC1934C, i_c1934c),
    (0xC19350, i_c19350),
    (0xC19352, i_c19352),
    (0xC19354, i_c19354),
    (0xC19356, i_c19356),
    (0xC19358, i_c19358),
    (0xC1935C, i_c1935c),
    (0xC19360, i_c19360),
    (0xC19362, i_c19362),
    (0xC19364, i_c19364),
    (0xC19368, i_c19368),
    (0xC1936A, i_c1936a),
    (0xC1936E, i_c1936e),
    (0xC19372, i_c19372),
    (0xC19376, i_c19376),
    (0xC1937A, i_c1937a),
    (0xC1937E, i_c1937e),
    (0xC19380, i_c19380),
    (0xC19382, i_c19382),
    (0xC19384, i_c19384),
    (0xC19388, i_c19388),
    (0xC1938A, i_c1938a),
    (0xC1938E, i_c1938e),
    (0xC19392, i_c19392),
    (0xC19394, i_c19394),
    (0xC19398, i_c19398),
    (0xC1939C, i_c1939c),
    (0xC193A0, i_c193a0),
    (0xC193A4, i_c193a4),
    (0xC193A8, i_c193a8),
    (0xC193AC, i_c193ac),
    (0xC193B0, i_c193b0),
    (0xC193B2, i_c193b2),
    (0xC193B6, i_c193b6),
    (0xC193B8, i_c193b8),
    (0xC193BC, i_c193bc),
    (0xC193C0, i_c193c0),
    (0xC193C2, i_c193c2),
    (0xC193C6, i_c193c6),
    (0xC193C8, i_c193c8),
    (0xC193CA, i_c193ca),
    (0xC193CE, i_c193ce),
    (0xC193D0, i_c193d0),
    (0xC193D4, i_c193d4),
    (0xC193D8, i_c193d8),
    (0xC193DC, i_c193dc),
    (0xC193E0, i_c193e0),
    (0xC193E4, i_c193e4),
    (0xC193E8, i_c193e8),
    (0xC193EA, i_c193ea),
    (0xC193EC, i_c193ec),
    (0xC193F2, i_c193f2),
    (0xC193F6, i_c193f6),
    (0xC193FA, i_c193fa),
    (0xC193FC, i_c193fc),
    (0xC19400, i_c19400),
    (0xC19404, i_c19404),
    (0xC19406, i_c19406),
    (0xC1940A, i_c1940a),
    (0xC1940E, i_c1940e),
    (0xC19412, i_c19412),
    (0xC19416, i_c19416),
    (0xC1941A, i_c1941a),
    (0xC1941E, i_c1941e),
    (0xC19422, i_c19422),
    (0xC19426, i_c19426),
    (0xC1942A, i_c1942a),
    (0xC1942C, i_c1942c),
    (0xC19430, i_c19430),
    (0xC19434, i_c19434),
    (0xC19438, i_c19438),
    (0xC1943C, i_c1943c),
    (0xC19440, i_c19440),
    (0xC19444, i_c19444),
    (0xC19448, i_c19448),
    (0xC1944C, i_c1944c),
    (0xC19450, i_c19450),
    (0xC19452, i_c19452),
    (0xC19456, i_c19456),
    (0xC1945A, i_c1945a),
    (0xC1945C, i_c1945c),
    (0xC19460, i_c19460),
    (0xC19464, i_c19464),
    (0xC19466, i_c19466),
    (0xC1946A, i_c1946a),
    (0xC1946C, i_c1946c),
    (0xC1946E, i_c1946e),
    (0xC19472, i_c19472),
    (0xC19474, i_c19474),
    (0xC19478, i_c19478),
    (0xC1947A, i_c1947a),
    (0xC1947E, i_c1947e),
    (0xC19480, i_c19480),
    (0xC19484, i_c19484),
    (0xC1948A, i_c1948a),
    (0xC1948E, i_c1948e),
    (0xC19492, i_c19492),
    (0xC19496, i_c19496),
    (0xC1949A, i_c1949a),
    (0xC1949E, i_c1949e),
    (0xC194A2, i_c194a2),
    (0xC194A8, i_c194a8),
    (0xC194AC, i_c194ac),
    (0xC194B0, i_c194b0),
    (0xC194B2, i_c194b2),
    (0xC194B6, i_c194b6),
    (0xC194BC, i_c194bc),
    (0xC194C0, i_c194c0),
    (0xC194C4, i_c194c4),
    (0xC194C8, i_c194c8),
    (0xC194CC, i_c194cc),
    (0xC194D0, i_c194d0),
    (0xC194D4, i_c194d4),
    (0xC194D8, i_c194d8),
    (0xC194DC, i_c194dc),
    (0xC194DE, i_c194de),
    (0xC194E2, i_c194e2),
    (0xC194E4, i_c194e4),
    (0xC194EA, i_c194ea),
    (0xC194EE, i_c194ee),
    (0xC194F0, i_c194f0),
    (0xC194F2, i_c194f2),
    (0xC194F6, i_c194f6),
    (0xC194F8, i_c194f8),
    (0xC194FC, i_c194fc),
    (0xC19500, i_c19500),
    (0xC19502, i_c19502),
    (0xC19506, i_c19506),
    (0xC1950A, i_c1950a),
    (0xC19510, i_c19510),
    (0xC19514, i_c19514),
    (0xC19518, i_c19518),
    (0xC1951A, i_c1951a),
    (0xC1951E, i_c1951e),
    (0xC19522, i_c19522),
    (0xC19524, i_c19524),
    (0xC19528, i_c19528),
    (0xC1952C, i_c1952c),
    (0xC19530, i_c19530),
    (0xC19532, i_c19532),
    (0xC19534, i_c19534),
    (0xC19536, i_c19536),
    (0xC1953A, i_c1953a),
    (0xC1953C, i_c1953c),
    (0xC19540, i_c19540),
    (0xC19542, i_c19542),
    (0xC19544, i_c19544),
    (0xC1954A, i_c1954a),
    (0xC19550, i_c19550),
    (0xC19554, i_c19554),
    (0xC19558, i_c19558),
    (0xC1955A, i_c1955a),
    (0xC1955E, i_c1955e),
    (0xC19562, i_c19562),
    (0xC19566, i_c19566),
    (0xC19568, i_c19568),
    (0xC1956A, i_c1956a),
    (0xC1956C, i_c1956c),
    (0xC1956E, i_c1956e),
    (0xC19570, i_c19570),
    (0xC19574, i_c19574),
    (0xC1957A, i_c1957a),
    (0xC1957C, i_c1957c),
    (0xC19580, i_c19580),
    (0xC19582, i_c19582),
    (0xC19584, i_c19584),
    (0xC19588, i_c19588),
    (0xC1958E, i_c1958e),
    (0xC19590, i_c19590),
    (0xC19592, i_c19592),
    (0xC19596, i_c19596),
    (0xC1959A, i_c1959a),
    (0xC1959C, i_c1959c),
    (0xC1959E, i_c1959e),
    (0xC195A0, i_c195a0),
    (0xC195A2, i_c195a2),
    (0xC195A4, i_c195a4),
    (0xC195A8, i_c195a8),
    (0xC195AA, i_c195aa),
    (0xC195AC, i_c195ac),
    (0xC195B0, i_c195b0),
    (0xC195B4, i_c195b4),
    (0xC195B8, i_c195b8),
    (0xC195BC, i_c195bc),
    (0xC195BE, i_c195be),
    (0xC195C0, i_c195c0),
    (0xC195C4, i_c195c4),
    (0xC195C6, i_c195c6),
    (0xC195CA, i_c195ca),
    (0xC195CC, i_c195cc),
    (0xC195D0, i_c195d0),
    (0xC195D4, i_c195d4),
    (0xC195D8, i_c195d8),
    (0xC195DC, i_c195dc),
    (0xC195E0, i_c195e0),
    (0xC195E6, i_c195e6),
    (0xC195EA, i_c195ea),
    (0xC195F0, i_c195f0),
    (0xC195F2, i_c195f2),
    (0xC195F4, i_c195f4),
    (0xC195FA, i_c195fa),
    (0xC195FE, i_c195fe),
    (0xC19602, i_c19602),
    (0xC19606, i_c19606),
    (0xC1960A, i_c1960a),
    (0xC1960E, i_c1960e),
    (0xC19612, i_c19612),
    (0xC19616, i_c19616),
    (0xC19618, i_c19618),
    (0xC1961C, i_c1961c),
    (0xC1961E, i_c1961e),
    (0xC19620, i_c19620),
    (0xC19622, i_c19622),
    (0xC19624, i_c19624),
    (0xC19626, i_c19626),
    (0xC1962A, i_c1962a),
    (0xC1962C, i_c1962c),
    (0xC1962E, i_c1962e),
    (0xC19630, i_c19630),
    (0xC19634, i_c19634),
    (0xC19638, i_c19638),
    (0xC1963C, i_c1963c),
    (0xC19642, i_c19642),
    (0xC19646, i_c19646),
    (0xC1964A, i_c1964a),
    (0xC1964E, i_c1964e),
    (0xC19652, i_c19652),
    (0xC19656, i_c19656),
    (0xC1965A, i_c1965a),
    (0xC1965C, i_c1965c),
    (0xC19660, i_c19660),
    (0xC19662, i_c19662),
    (0xC19666, i_c19666),
    (0xC1966A, i_c1966a),
    (0xC1966E, i_c1966e),
    (0xC19670, i_c19670),
    (0xC19672, i_c19672),
    (0xC19676, i_c19676),
    (0xC19678, i_c19678),
    (0xC1967A, i_c1967a),
    (0xC1967E, i_c1967e),
    (0xC19682, i_c19682),
    (0xC19684, i_c19684),
    (0xC19688, i_c19688),
    (0xC1968A, i_c1968a),
    (0xC1968E, i_c1968e),
    (0xC19692, i_c19692),
    (0xC19694, i_c19694),
    (0xC19698, i_c19698),
    (0xC1969C, i_c1969c),
    (0xC196A2, i_c196a2),
    (0xC196A6, i_c196a6),
    (0xC196AA, i_c196aa),
    (0xC196AC, i_c196ac),
    (0xC196B0, i_c196b0),
    (0xC196B4, i_c196b4),
    (0xC196B6, i_c196b6),
    (0xC196BA, i_c196ba),
    (0xC196BE, i_c196be),
    (0xC196C0, i_c196c0),
    (0xC196C4, i_c196c4),
    (0xC196C6, i_c196c6),
    (0xC196CA, i_c196ca),
    (0xC196CC, i_c196cc),
    (0xC196CE, i_c196ce),
    (0xC196D2, i_c196d2),
    (0xC196D6, i_c196d6),
    (0xC196D8, i_c196d8),
    (0xC196DA, i_c196da),
    (0xC196DE, i_c196de),
    (0xC196E0, i_c196e0),
    (0xC196E2, i_c196e2),
    (0xC196E6, i_c196e6),
    (0xC196EA, i_c196ea),
    (0xC196EC, i_c196ec),
    (0xC196EE, i_c196ee),
    (0xC196F2, i_c196f2),
    (0xC196F4, i_c196f4),
    (0xC196F6, i_c196f6),
    (0xC196F8, i_c196f8),
    (0xC196FA, i_c196fa),
    (0xC196FC, i_c196fc),
    (0xC19700, i_c19700),
    (0xC19704, i_c19704),
    (0xC19708, i_c19708),
    (0xC1970C, i_c1970c),
    (0xC19710, i_c19710),
    (0xC19716, i_c19716),
    (0xC1971A, i_c1971a),
    (0xC1971E, i_c1971e),
    (0xC19722, i_c19722),
    (0xC19726, i_c19726),
    (0xC1972A, i_c1972a),
    (0xC1972E, i_c1972e),
    (0xC19732, i_c19732),
    (0xC19736, i_c19736),
    (0xC1973A, i_c1973a),
    (0xC1973E, i_c1973e),
    (0xC19744, i_c19744),
    (0xC1974A, i_c1974a),
    (0xC1974E, i_c1974e),
    (0xC19752, i_c19752),
    (0xC19756, i_c19756),
    (0xC1975A, i_c1975a),
    (0xC1975E, i_c1975e),
    (0xC19760, i_c19760),
    (0xC19764, i_c19764),
    (0xC19768, i_c19768),
    (0xC1976C, i_c1976c),
    (0xC1976E, i_c1976e),
    (0xC19770, i_c19770),
    (0xC19772, i_c19772),
    (0xC19776, i_c19776),
    (0xC19778, i_c19778),
    (0xC1977C, i_c1977c),
    (0xC19780, i_c19780),
    (0xC19784, i_c19784),
    (0xC19788, i_c19788),
    (0xC1978A, i_c1978a),
    (0xC1978C, i_c1978c),
    (0xC19790, i_c19790),
    (0xC19792, i_c19792),
    (0xC19796, i_c19796),
    (0xC19798, i_c19798),
    (0xC1979C, i_c1979c),
    (0xC1979E, i_c1979e),
    (0xC197A0, i_c197a0),
    (0xC197A4, i_c197a4),
    (0xC197A6, i_c197a6),
    (0xC197AA, i_c197aa),
    (0xC197AE, i_c197ae),
    (0xC197B0, i_c197b0),
    (0xC197B4, i_c197b4),
    (0xC197B8, i_c197b8),
    (0xC197BC, i_c197bc),
    (0xC197C0, i_c197c0),
    (0xC197C2, i_c197c2),
    (0xC197C6, i_c197c6),
    (0xC197CA, i_c197ca),
    (0xC197CE, i_c197ce),
    (0xC197D2, i_c197d2),
    (0xC197D4, i_c197d4),
    (0xC197D8, i_c197d8),
    (0xC197DC, i_c197dc),
    (0xC197E0, i_c197e0),
    (0xC197E4, i_c197e4),
    (0xC197E8, i_c197e8),
    (0xC197EC, i_c197ec),
    (0xC197EE, i_c197ee),
    (0xC197F2, i_c197f2),
    (0xC197F4, i_c197f4),
    (0xC197F8, i_c197f8),
    (0xC197FA, i_c197fa),
    (0xC197FE, i_c197fe),
    (0xC19802, i_c19802),
    (0xC19804, i_c19804),
    (0xC19808, i_c19808),
    (0xC1980C, i_c1980c),
    (0xC19810, i_c19810),
    (0xC19814, i_c19814),
    (0xC19816, i_c19816),
    (0xC1981A, i_c1981a),
    (0xC1981E, i_c1981e),
    (0xC19822, i_c19822),
    (0xC19826, i_c19826),
    (0xC1982A, i_c1982a),
    (0xC1982E, i_c1982e),
    (0xC19830, i_c19830),
    (0xC19832, i_c19832),
    (0xC19836, i_c19836),
    (0xC19838, i_c19838),
    (0xC1983E, i_c1983e),
    (0xC19842, i_c19842),
    (0xC19846, i_c19846),
    (0xC1984A, i_c1984a),
    (0xC1984E, i_c1984e),
    (0xC19852, i_c19852),
    (0xC19856, i_c19856),
    (0xC1985A, i_c1985a),
    (0xC1985E, i_c1985e),
    (0xC19862, i_c19862),
    (0xC19866, i_c19866),
    (0xC1986C, i_c1986c),
    (0xC19872, i_c19872),
    (0xC19876, i_c19876),
    (0xC1987A, i_c1987a),
    (0xC1987E, i_c1987e),
    (0xC19882, i_c19882),
    (0xC19886, i_c19886),
    (0xC19888, i_c19888),
    (0xC1988C, i_c1988c),
    (0xC19890, i_c19890),
    (0xC19894, i_c19894),
    (0xC19896, i_c19896),
    (0xC19898, i_c19898),
    (0xC1989A, i_c1989a),
    (0xC1989E, i_c1989e),
    (0xC198A0, i_c198a0),
    (0xC198A4, i_c198a4),
    (0xC198A8, i_c198a8),
    (0xC198AC, i_c198ac),
    (0xC198B0, i_c198b0),
    (0xC198B2, i_c198b2),
    (0xC198B4, i_c198b4),
    (0xC198B8, i_c198b8),
    (0xC198BA, i_c198ba),
    (0xC198BE, i_c198be),
    (0xC198C0, i_c198c0),
    (0xC198C4, i_c198c4),
    (0xC198C6, i_c198c6),
    (0xC198C8, i_c198c8),
    (0xC198CC, i_c198cc),
    (0xC198CE, i_c198ce),
    (0xC198D2, i_c198d2),
    (0xC198D6, i_c198d6),
    (0xC198D8, i_c198d8),
    (0xC198DC, i_c198dc),
    (0xC198E0, i_c198e0),
    (0xC198E4, i_c198e4),
    (0xC198E8, i_c198e8),
    (0xC198EA, i_c198ea),
    (0xC198EE, i_c198ee),
    (0xC198F2, i_c198f2),
    (0xC198F6, i_c198f6),
    (0xC198FA, i_c198fa),
    (0xC198FC, i_c198fc),
    (0xC19900, i_c19900),
    (0xC19904, i_c19904),
    (0xC19908, i_c19908),
    (0xC1990C, i_c1990c),
    (0xC19910, i_c19910),
    (0xC19914, i_c19914),
    (0xC19916, i_c19916),
    (0xC1991A, i_c1991a),
    (0xC1991C, i_c1991c),
    (0xC19920, i_c19920),
    (0xC19924, i_c19924),
    (0xC19928, i_c19928),
    (0xC1992C, i_c1992c),
    (0xC19930, i_c19930),
    (0xC19932, i_c19932),
    (0xC19936, i_c19936),
    (0xC1993A, i_c1993a),
    (0xC1993E, i_c1993e),
    (0xC19942, i_c19942),
    (0xC19944, i_c19944),
    (0xC19948, i_c19948),
    (0xC1994C, i_c1994c),
    (0xC19950, i_c19950),
    (0xC19954, i_c19954),
    (0xC19958, i_c19958),
    (0xC1995A, i_c1995a),
    (0xC1995E, i_c1995e),
    (0xC19960, i_c19960),
    (0xC19964, i_c19964),
    (0xC19968, i_c19968),
    (0xC1996A, i_c1996a),
    (0xC1996E, i_c1996e),
    (0xC19972, i_c19972),
    (0xC19976, i_c19976),
    (0xC1997A, i_c1997a),
    (0xC1997C, i_c1997c),
    (0xC19980, i_c19980),
    (0xC19984, i_c19984),
    (0xC19988, i_c19988),
    (0xC1998C, i_c1998c),
    (0xC19990, i_c19990),
    (0xC19994, i_c19994),
    (0xC19996, i_c19996),
    (0xC19998, i_c19998),
    (0xC1999C, i_c1999c),
    (0xC1999E, i_c1999e),
    (0xC199A4, i_c199a4),
    (0xC199A6, i_c199a6),
    (0xC199AA, i_c199aa),
    (0xC199AC, i_c199ac),
    (0xC199B0, i_c199b0),
    (0xC199B4, i_c199b4),
    (0xC199B6, i_c199b6),
    (0xC199BA, i_c199ba),
    (0xC199BC, i_c199bc),
    (0xC199BE, i_c199be),
    (0xC199C2, i_c199c2),
    (0xC199C4, i_c199c4),
    (0xC199C6, i_c199c6),
    (0xC199CA, i_c199ca),
    (0xC199D0, i_c199d0),
    (0xC199D2, i_c199d2),
    (0xC199D8, i_c199d8),
    (0xC199DA, i_c199da),
    (0xC199DC, i_c199dc),
    (0xC199E0, i_c199e0),
    (0xC199E4, i_c199e4),
    (0xC199E6, i_c199e6),
    (0xC199EA, i_c199ea),
    (0xC199EE, i_c199ee),
    (0xC199F0, i_c199f0),
    (0xC199F4, i_c199f4),
    (0xC199F6, i_c199f6),
    (0xC199FA, i_c199fa),
    (0xC199FE, i_c199fe),
    (0xC19A00, i_c19a00),
    (0xC19A02, i_c19a02),
    (0xC19A04, i_c19a04),
    (0xC19A06, i_c19a06),
    (0xC19A0A, i_c19a0a),
    (0xC19A0E, i_c19a0e),
    (0xC19A12, i_c19a12),
    (0xC19A16, i_c19a16),
    (0xC19A1C, i_c19a1c),
    (0xC19A22, i_c19a22),
    (0xC19A24, i_c19a24),
    (0xC19A28, i_c19a28),
    (0xC19A2C, i_c19a2c),
    (0xC19A30, i_c19a30),
    (0xC19A34, i_c19a34),
    (0xC19A36, i_c19a36),
    (0xC19A38, i_c19a38),
    (0xC19A3A, i_c19a3a),
    (0xC19A3C, i_c19a3c),
    (0xC19A40, i_c19a40),
    (0xC19A42, i_c19a42),
    (0xC19A46, i_c19a46),
    (0xC19A4A, i_c19a4a),
    (0xC19A4E, i_c19a4e),
    (0xC19A52, i_c19a52),
    (0xC19A56, i_c19a56),
    (0xC19A58, i_c19a58),
    (0xC19A5C, i_c19a5c),
    (0xC19A60, i_c19a60),
    (0xC19A64, i_c19a64),
    (0xC19A66, i_c19a66),
    (0xC19A68, i_c19a68),
    (0xC19A6A, i_c19a6a),
    (0xC19A6E, i_c19a6e),
    (0xC19A70, i_c19a70),
    (0xC19A74, i_c19a74),
    (0xC19A78, i_c19a78),
    (0xC19A7C, i_c19a7c),
    (0xC19A80, i_c19a80),
    (0xC19A82, i_c19a82),
    (0xC19A84, i_c19a84),
    (0xC19A88, i_c19a88),
    (0xC19A8A, i_c19a8a),
    (0xC19A8E, i_c19a8e),
    (0xC19A90, i_c19a90),
    (0xC19A94, i_c19a94),
    (0xC19A96, i_c19a96),
    (0xC19A98, i_c19a98),
    (0xC19A9C, i_c19a9c),
    (0xC19AA0, i_c19aa0),
    (0xC19AA4, i_c19aa4),
    (0xC19AA8, i_c19aa8),
    (0xC19AAA, i_c19aaa),
    (0xC19AAC, i_c19aac),
    (0xC19AB0, i_c19ab0),
    (0xC19AB4, i_c19ab4),
    (0xC19AB6, i_c19ab6),
    (0xC19ABC, i_c19abc),
    (0xC19AC0, i_c19ac0),
    (0xC19AC4, i_c19ac4),
    (0xC19AC8, i_c19ac8),
    (0xC19ACE, i_c19ace),
    (0xC19AD2, i_c19ad2),
    (0xC19AD6, i_c19ad6),
    (0xC19ADA, i_c19ada),
    (0xC19ADC, i_c19adc),
    (0xC19AE0, i_c19ae0),
    (0xC19AE4, i_c19ae4),
    (0xC19AE6, i_c19ae6),
    (0xC19AE8, i_c19ae8),
    (0xC19AEC, i_c19aec),
    (0xC19AEE, i_c19aee),
    (0xC19AF0, i_c19af0),
    (0xC19AF2, i_c19af2),
    (0xC19AF4, i_c19af4),
    (0xC19AF8, i_c19af8),
    (0xC19AFA, i_c19afa),
    (0xC19AFC, i_c19afc),
    (0xC19AFE, i_c19afe),
    (0xC19B00, i_c19b00),
    (0xC19B02, i_c19b02),
    (0xC19B04, i_c19b04),
    (0xC19B06, i_c19b06),
    (0xC19B2E, i_c19b2e),
    (0xC19B32, i_c19b32),
    (0xC19B36, i_c19b36),
    (0xC19B38, i_c19b38),
    (0xC19B3C, i_c19b3c),
    (0xC19B3E, i_c19b3e),
    (0xC19B40, i_c19b40),
    (0xC19B42, i_c19b42),
    (0xC19B46, i_c19b46),
    (0xC19B4A, i_c19b4a),
    (0xC19B4C, i_c19b4c),
    (0xC19B4E, i_c19b4e),
    (0xC19B52, i_c19b52),
    (0xC19B58, i_c19b58),
    (0xC19B5C, i_c19b5c),
    (0xC19B60, i_c19b60),
    (0xC19B62, i_c19b62),
    (0xC19B66, i_c19b66),
    (0xC19B6A, i_c19b6a),
    (0xC19B6E, i_c19b6e),
    (0xC19B72, i_c19b72),
    (0xC19B76, i_c19b76),
    (0xC19B7A, i_c19b7a),
    (0xC19B7E, i_c19b7e),
    (0xC19B82, i_c19b82),
    (0xC19B86, i_c19b86),
    (0xC19B88, i_c19b88),
    (0xC19B8A, i_c19b8a),
    (0xC19B8E, i_c19b8e),
    (0xC19B94, i_c19b94),
    (0xC19B96, i_c19b96),
    (0xC19B9C, i_c19b9c),
    (0xC19BA0, i_c19ba0),
    (0xC19BA6, i_c19ba6),
    (0xC19BAA, i_c19baa),
    (0xC19BAE, i_c19bae),
    (0xC19BB4, i_c19bb4),
    (0xC19BB8, i_c19bb8),
    (0xC19BBC, i_c19bbc),
    (0xC19BC2, i_c19bc2),
    (0xC19BC6, i_c19bc6),
    (0xC19BCA, i_c19bca),
    (0xC19BD0, i_c19bd0),
    (0xC19BD4, i_c19bd4),
    (0xC19BD8, i_c19bd8),
    (0xC19BDA, i_c19bda),
    (0xC19BDE, i_c19bde),
    (0xC19BE2, i_c19be2),
    (0xC19BE4, i_c19be4),
    (0xC19BE8, i_c19be8),
    (0xC19BEC, i_c19bec),
    (0xC19BF0, i_c19bf0),
    (0xC19BF2, i_c19bf2),
    (0xC19BF6, i_c19bf6),
    (0xC19BF8, i_c19bf8),
    (0xC19BFC, i_c19bfc),
    (0xC19C00, i_c19c00),
    (0xC19C04, i_c19c04),
    (0xC19C06, i_c19c06),
    (0xC19C08, i_c19c08),
    (0xC19C0C, i_c19c0c),
    (0xC19C10, i_c19c10),
    (0xC19C12, i_c19c12),
    (0xC19C14, i_c19c14),
    (0xC19C16, i_c19c16),
    (0xC19C18, i_c19c18),
    (0xC19C1C, i_c19c1c),
    (0xC19C22, i_c19c22),
    (0xC19C24, i_c19c24),
    (0xC19C2A, i_c19c2a),
    (0xC19C2C, i_c19c2c),
    (0xC19C2E, i_c19c2e),
    (0xC19C32, i_c19c32),
    (0xC19C34, i_c19c34),
    (0xC19C38, i_c19c38),
    (0xC19C3A, i_c19c3a),
    (0xC19C3C, i_c19c3c),
    (0xC19C40, i_c19c40),
    (0xC19C44, i_c19c44),
    (0xC19C46, i_c19c46),
    (0xC19C4A, i_c19c4a),
    (0xC19C4E, i_c19c4e),
    (0xC19C52, i_c19c52),
    (0xC19C54, i_c19c54),
    (0xC19C58, i_c19c58),
    (0xC19C5C, i_c19c5c),
    (0xC19C5E, i_c19c5e),
    (0xC19C62, i_c19c62),
    (0xC19C64, i_c19c64),
    (0xC19C68, i_c19c68),
    (0xC19C6C, i_c19c6c),
    (0xC19C6E, i_c19c6e),
    (0xC19C70, i_c19c70),
    (0xC19C74, i_c19c74),
    (0xC19C76, i_c19c76),
    (0xC19C7A, i_c19c7a),
    (0xC19C7C, i_c19c7c),
    (0xC19C80, i_c19c80),
    (0xC19C82, i_c19c82),
    (0xC19C86, i_c19c86),
    (0xC19C88, i_c19c88),
    (0xC19C8C, i_c19c8c),
    (0xC19C8E, i_c19c8e),
    (0xC19C90, i_c19c90),
    (0xC19C94, i_c19c94),
    (0xC19C98, i_c19c98),
    (0xC19C9C, i_c19c9c),
    (0xC19CA0, i_c19ca0),
    (0xC19CA4, i_c19ca4),
    (0xC19CA6, i_c19ca6),
    (0xC19CAA, i_c19caa),
    (0xC19CAE, i_c19cae),
    (0xC19CB0, i_c19cb0),
    (0xC19CB2, i_c19cb2),
    (0xC19CB4, i_c19cb4),
    (0xC19CB8, i_c19cb8),
    (0xC19CBA, i_c19cba),
    (0xC19CBC, i_c19cbc),
    (0xC19CBE, i_c19cbe),
    (0xC19CC0, i_c19cc0),
    (0xC19CC2, i_c19cc2),
    (0xC19CC4, i_c19cc4),
    (0xC19CC6, i_c19cc6),
    (0xC19CC8, i_c19cc8),
    (0xC19CCA, i_c19cca),
    (0xC19CCC, i_c19ccc),
    (0xC19CD0, i_c19cd0),
    (0xC19CD2, i_c19cd2),
    (0xC19CD4, i_c19cd4),
    (0xC19CD6, i_c19cd6),
    (0xC19CD8, i_c19cd8),
    (0xC19CDA, i_c19cda),
    (0xC19CDE, i_c19cde),
    (0xC19CE0, i_c19ce0),
    (0xC19CE2, i_c19ce2),
    (0xC19CE4, i_c19ce4),
    (0xC19CE8, i_c19ce8),
    (0xC19CEC, i_c19cec),
    (0xC19CEE, i_c19cee),
    (0xC19CF0, i_c19cf0),
    (0xC19CF4, i_c19cf4),
    (0xC19CF6, i_c19cf6),
    (0xC19CF8, i_c19cf8),
    (0xC19CFC, i_c19cfc),
    (0xC19D00, i_c19d00),
    (0xC19D02, i_c19d02),
    (0xC19D06, i_c19d06),
    (0xC19D08, i_c19d08),
    (0xC19D0C, i_c19d0c),
    (0xC19D0E, i_c19d0e),
    (0xC19D10, i_c19d10),
    (0xC19D14, i_c19d14),
    (0xC19D18, i_c19d18),
    (0xC19D1A, i_c19d1a),
    (0xC19D20, i_c19d20),
    (0xC19D24, i_c19d24),
    (0xC19D28, i_c19d28),
    (0xC19D2A, i_c19d2a),
    (0xC19D2C, i_c19d2c),
    (0xC19D30, i_c19d30),
    (0xC19D32, i_c19d32),
    (0xC19D34, i_c19d34),
    (0xC19D38, i_c19d38),
    (0xC19D3C, i_c19d3c),
    (0xC19D3E, i_c19d3e),
    (0xC19D40, i_c19d40),
    (0xC19D44, i_c19d44),
    (0xC19D48, i_c19d48),
    (0xC19D4C, i_c19d4c),
    (0xC19D50, i_c19d50),
    (0xC19D54, i_c19d54),
    (0xC19D56, i_c19d56),
    (0xC19D5C, i_c19d5c),
    (0xC19D5E, i_c19d5e),
    (0xC19D60, i_c19d60),
    (0xC19D62, i_c19d62),
    (0xC19D66, i_c19d66),
    (0xC19D6A, i_c19d6a),
    (0xC19D6C, i_c19d6c),
    (0xC19D6E, i_c19d6e),
    (0xC19D70, i_c19d70),
    (0xC19D72, i_c19d72),
    (0xC19D74, i_c19d74),
    (0xC19D76, i_c19d76),
    (0xC19D78, i_c19d78),
    (0xC19D7A, i_c19d7a),
    (0xC19D7C, i_c19d7c),
    (0xC19D7E, i_c19d7e),
    (0xC19E9A, i_c19e9a),
    (0xC19E9C, i_c19e9c),
    (0xC19E9E, i_c19e9e),
    (0xC19EA0, i_c19ea0),
    (0xC19EA2, i_c19ea2),
    (0xC19EA4, i_c19ea4),
    (0xC19EA6, i_c19ea6),
    (0xC19EA8, i_c19ea8),
    (0xC19EAA, i_c19eaa),
    (0xC19EAC, i_c19eac),
    (0xC19EAE, i_c19eae),
    (0xC19EB0, i_c19eb0),
    (0xC19EB2, i_c19eb2),
    (0xC19EB4, i_c19eb4),
    (0xC19EB6, i_c19eb6),
    (0xC19EB8, i_c19eb8),
    (0xC19EBA, i_c19eba),
    (0xC19EBC, i_c19ebc),
    (0xC19EBE, i_c19ebe),
    (0xC19EC0, i_c19ec0),
    (0xC19EC2, i_c19ec2),
    (0xC19EC4, i_c19ec4),
    (0xC19EC6, i_c19ec6),
    (0xC19EC8, i_c19ec8),
    (0xC19ECA, i_c19eca),
    (0xC19ECC, i_c19ecc),
    (0xC19EDC, i_c19edc),
    (0xC19EE0, i_c19ee0),
    (0xC19EE4, i_c19ee4),
    (0xC19EE8, i_c19ee8),
    (0xC19EEE, i_c19eee),
    (0xC19EF2, i_c19ef2),
    (0xC19EF6, i_c19ef6),
    (0xC19EFA, i_c19efa),
    (0xC19F00, i_c19f00),
    (0xC19F04, i_c19f04),
    (0xC19F08, i_c19f08),
    (0xC19F0E, i_c19f0e),
    (0xC19F12, i_c19f12),
    (0xC19F18, i_c19f18),
    (0xC19F1A, i_c19f1a),
    (0xC19F1E, i_c19f1e),
    (0xC19F22, i_c19f22),
    (0xC19F26, i_c19f26),
    (0xC19F2A, i_c19f2a),
    (0xC19F2E, i_c19f2e),
    (0xC19F32, i_c19f32),
    (0xC19F36, i_c19f36),
    (0xC19F3A, i_c19f3a),
    (0xC19F3C, i_c19f3c),
    (0xC19F3E, i_c19f3e),
    (0xC19F42, i_c19f42),
    (0xC19F46, i_c19f46),
    (0xC19F48, i_c19f48),
    (0xC19F4C, i_c19f4c),
    (0xC19F4E, i_c19f4e),
    (0xC19F50, i_c19f50),
    (0xC19F54, i_c19f54),
    (0xC19F58, i_c19f58),
    (0xC19F5A, i_c19f5a),
    (0xC19F5E, i_c19f5e),
    (0xC19F60, i_c19f60),
    (0xC19F64, i_c19f64),
    (0xC19F68, i_c19f68),
    (0xC19F6A, i_c19f6a),
    (0xC19F6C, i_c19f6c),
    (0xC19F70, i_c19f70),
    (0xC19F72, i_c19f72),
    (0xC19F74, i_c19f74),
    (0xC19F76, i_c19f76),
    (0xC19F78, i_c19f78),
    (0xC19F7A, i_c19f7a),
    (0xC19F7C, i_c19f7c),
    (0xC19F7E, i_c19f7e),
    (0xC19F80, i_c19f80),
    (0xC19F82, i_c19f82),
    (0xC19F84, i_c19f84),
    (0xC19F86, i_c19f86),
    (0xC19F8A, i_c19f8a),
    (0xC19F8E, i_c19f8e),
    (0xC19F92, i_c19f92),
    (0xC19F96, i_c19f96),
    (0xC19F9A, i_c19f9a),
    (0xC19F9E, i_c19f9e),
    (0xC19FA4, i_c19fa4),
    (0xC19FA8, i_c19fa8),
    (0xC19FAE, i_c19fae),
    (0xC19FB0, i_c19fb0),
    (0xC19FB4, i_c19fb4),
    (0xC19FB6, i_c19fb6),
    (0xC19FB8, i_c19fb8),
    (0xC19FBC, i_c19fbc),
    (0xC19FC0, i_c19fc0),
    (0xC19FC2, i_c19fc2),
    (0xC19FC4, i_c19fc4),
    (0xC19FCA, i_c19fca),
    (0xC19FCC, i_c19fcc),
    (0xC19FCE, i_c19fce),
    (0xC19FD0, i_c19fd0),
    (0xC19FD2, i_c19fd2),
    (0xC19FD4, i_c19fd4),
    (0xC19FD6, i_c19fd6),
    (0xC19FDA, i_c19fda),
    (0xC19FDC, i_c19fdc),
    (0xC19FDE, i_c19fde),
    (0xC19FE0, i_c19fe0),
    (0xC19FE4, i_c19fe4),
    (0xC19FE6, i_c19fe6),
    (0xC19FEA, i_c19fea),
    (0xC19FEC, i_c19fec),
    (0xC19FF0, i_c19ff0),
    (0xC19FF2, i_c19ff2),
    (0xC19FF6, i_c19ff6),
    (0xC19FFA, i_c19ffa),
    (0xC19FFC, i_c19ffc),
    (0xC1A000, i_c1a000),
    (0xC1A004, i_c1a004),
    (0xC1A006, i_c1a006),
    (0xC1A00A, i_c1a00a),
    (0xC1A00E, i_c1a00e),
    (0xC1A010, i_c1a010),
    (0xC1A014, i_c1a014),
    (0xC1A018, i_c1a018),
    (0xC1A01C, i_c1a01c),
    (0xC1A01E, i_c1a01e),
    (0xC1A022, i_c1a022),
    (0xC1A024, i_c1a024),
    (0xC1A028, i_c1a028),
    (0xC1A02C, i_c1a02c),
    (0xC1A02E, i_c1a02e),
    (0xC1A032, i_c1a032),
    (0xC1A034, i_c1a034),
    (0xC1A036, i_c1a036),
    (0xC1A03A, i_c1a03a),
    (0xC1A03E, i_c1a03e),
    (0xC1A040, i_c1a040),
    (0xC1A044, i_c1a044),
    (0xC1A046, i_c1a046),
    (0xC1A04C, i_c1a04c),
    (0xC1A04E, i_c1a04e),
    (0xC1A052, i_c1a052),
    (0xC1A054, i_c1a054),
    (0xC1A058, i_c1a058),
    (0xC1A05C, i_c1a05c),
    (0xC1A05E, i_c1a05e),
    (0xC1A060, i_c1a060),
    (0xC1A064, i_c1a064),
    (0xC1A068, i_c1a068),
    (0xC1A06C, i_c1a06c),
    (0xC1A06E, i_c1a06e),
    (0xC1A074, i_c1a074),
    (0xC1A078, i_c1a078),
    (0xC1A07A, i_c1a07a),
    (0xC1A07E, i_c1a07e),
    (0xC1A082, i_c1a082),
    (0xC1A086, i_c1a086),
    (0xC1A08C, i_c1a08c),
    (0xC1A090, i_c1a090),
    (0xC1A092, i_c1a092),
    (0xC1A094, i_c1a094),
    (0xC1A096, i_c1a096),
    (0xC1A098, i_c1a098),
    (0xC1A09C, i_c1a09c),
    (0xC1A0A0, i_c1a0a0),
    (0xC1A0A4, i_c1a0a4),
    (0xC1A0A8, i_c1a0a8),
    (0xC1A0AC, i_c1a0ac),
    (0xC1A0AE, i_c1a0ae),
    (0xC1A0B2, i_c1a0b2),
    (0xC1A0B4, i_c1a0b4),
    (0xC1A0B8, i_c1a0b8),
    (0xC1A0BC, i_c1a0bc),
    (0xC1A0BE, i_c1a0be),
    (0xC1A0C2, i_c1a0c2),
    (0xC1A0C6, i_c1a0c6),
    (0xC1A0C8, i_c1a0c8),
    (0xC1A0CA, i_c1a0ca),
    (0xC1A0CC, i_c1a0cc),
    (0xC1A0CE, i_c1a0ce),
    (0xC1A0D0, i_c1a0d0),
    (0xC1A0D2, i_c1a0d2),
    (0xC1A0D4, i_c1a0d4),
    (0xC1A0D6, i_c1a0d6),
    (0xC1A0D8, i_c1a0d8),
    (0xC1A0DC, i_c1a0dc),
    (0xC1A0DE, i_c1a0de),
    (0xC1A0E2, i_c1a0e2),
    (0xC1A0F6, i_c1a0f6),
    (0xC1A0FA, i_c1a0fa),
    (0xC1A0FE, i_c1a0fe),
    (0xC1A102, i_c1a102),
    (0xC1A108, i_c1a108),
    (0xC1A10A, i_c1a10a),
    (0xC1A10E, i_c1a10e),
    (0xC1A110, i_c1a110),
    (0xC1A114, i_c1a114),
    (0xC1A118, i_c1a118),
    (0xC1A11A, i_c1a11a),
    (0xC1A11C, i_c1a11c),
    (0xC1A120, i_c1a120),
    (0xC1A122, i_c1a122),
    (0xC1A126, i_c1a126),
    (0xC1A12C, i_c1a12c),
    (0xC1A130, i_c1a130),
];
