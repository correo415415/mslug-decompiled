// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_4) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $0669D0: jsr $2783a.l
pub fn i_0669d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x669D6);
    cpu.pc = 0x2783A;
    16
}
/// $0669D6: jsr $28d70.l
pub fn i_0669d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x669DC);
    cpu.pc = 0x28D70;
    16
}
/// $0669DC: bcc.w $669e6
pub fn i_0669dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x669E6; return 10; }
    8
}
/// $0669E0: lea.l $66a86(pc), a1
pub fn i_0669e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A86;
    16
}
/// $0669E4: move.l a1, (a6)
pub fn i_0669e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0669E6: bra.w $66a5c
pub fn i_0669e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x66A5C;
    10
}
// ===== Paratrooper_Die_0669ea  ($0669EA, 148 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0669EA: jsr $13600.l
pub fn i_0669ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x669F0);
    cpu.pc = 0x13600;
    16
}
/// $0669F0: move.w #$183, d1
pub fn i_0669f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669F4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x183;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0669F4: tst.b $10fd8f.l
pub fn i_0669f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669FA;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0669FA: bne.w $66a02
pub fn i_0669fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x669FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x66A02; return 10; }
    8
}
/// $0669FE: move.w #$1d7, d1
pub fn i_0669fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A02;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066A02: jsr $236e.l
pub fn i_066a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A08);
    cpu.pc = 0x236E;
    16
}
/// $066A08: move.w #$1043, d0
pub fn i_066a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A0C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1043;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066A0C: jsr $2352.l
pub fn i_066a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A12);
    cpu.pc = 0x2352;
    16
}
/// $066A12: lea.l $66f6e(pc), a0
pub fn i_066a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A16;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x66F6E;
    16
}
/// $066A16: jsr $28cd4.l
pub fn i_066a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A1C);
    cpu.pc = 0x28CD4;
    16
}
/// $066A1C: lea.l $66a22(pc), a1
pub fn i_066a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A22;
    16
}
/// $066A20: move.l a1, (a6)
pub fn i_066a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066A22: jsr $2783a.l
pub fn i_066a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A28);
    cpu.pc = 0x2783A;
    16
}
/// $066A28: jsr $28d70.l
pub fn i_066a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A2E);
    cpu.pc = 0x28D70;
    16
}
/// $066A2E: bcc.w $66a38
pub fn i_066a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66A38; return 10; }
    8
}
/// $066A32: lea.l $66a86(pc), a1
pub fn i_066a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A86;
    16
}
/// $066A36: move.l a1, (a6)
pub fn i_066a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066A38: bra.w $66a6c
pub fn i_066a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x66A6C;
    10
}
/// $066A3C: movea.l $c(a6), a0
pub fn i_066a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A40;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $066A40: tst.b $20(a0)
pub fn i_066a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A44;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $066A44: beq.w $66a4e
pub fn i_066a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A48;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x66A4E; return 10; }
    8
}
/// $066A48: lea.l $669c0(pc), a1
pub fn i_066a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x669C0;
    16
}
/// $066A4C: move.l a1, (a6)
pub fn i_066a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066A4E: tst.b $9b(a6)
pub fn i_066a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A52;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $066A52: bne.w $66a5c
pub fn i_066a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x66A5C; return 10; }
    8
}
/// $066A56: lea.l $669c0(pc), a1
pub fn i_066a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x669C0;
    16
}
/// $066A5A: move.l a1, (a6)
pub fn i_066a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066A5C: jsr $2870a.l
pub fn i_066a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A62);
    cpu.pc = 0x2870A;
    16
}
/// $066A62: bcc.w $66a6c
pub fn i_066a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66A6C; return 10; }
    8
}
/// $066A66: lea.l $669ea(pc), a1
pub fn i_066a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x669EA;
    16
}
/// $066A6A: move.l a1, (a6)
pub fn i_066a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066A6C: movea.l $c(a6), a0
pub fn i_066a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A70;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $066A70: addq.b #$1, $21(a0)
pub fn i_066a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A74;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $066A74: cmpi.w #$0, $22(a6)
pub fn i_066a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A7A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $066A7A: bgt.w $66a84
pub fn i_066a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A7E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x66A84; return 10; }
    8
}
// ===== SetTaskHandler_066a7e  ($066A7E, 8 B, kernel/task_handlers.c)
/// $066A7E: lea.l $66a86(pc), a1
pub fn i_066a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A86;
    16
}
/// $066A82: move.l a1, (a6)
pub fn i_066a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066A84: rts
pub fn i_066a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_066a86  ($066A86, 8 B, kernel/jmp_to_scheduler.c)
/// $066A86: jmp $518.l
pub fn i_066a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A8C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $066A8C: rts
pub fn i_066a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_066a8e  ($066A8E, 14 B, kernel/jsr5b6_jmp518.c)
/// $066A8E: jsr $5b6.l
pub fn i_066a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66A94);
    cpu.pc = 0x5B6;
    16
}
/// $066A94: jmp $518.l
pub fn i_066a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A9A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $066A9A: rts
pub fn i_066a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66A9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_FragExplode_066a9c  ($066A9C, 28 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066A9C: move.w #$c000, d0
pub fn i_066a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AA0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066AA0: jsr $28134.l
pub fn i_066aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66AA6);
    cpu.pc = 0x28134;
    16
}
/// $066AA6: andi.w #$ffe3, $38(a6)
pub fn i_066aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AAC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $066AAC: ori.w #$14, $38(a6)
pub fn i_066aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AB2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $066AB2: jmp $77f6a.l
pub fn i_066ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AB8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Paratrooper_Frag_066ab8  ($066AB8, 172 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066AB8: lea.l $2b8ae2.l, a0
pub fn i_066ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66ABE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8AE2;
    20
}
/// $066ABE: jsr $799de.l
pub fn i_066abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66AC4);
    cpu.pc = 0x799DE;
    16
}
/// $066AC4: move.b $76(a6), d3
pub fn i_066ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AC8;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $066AC8: andi.w #$ff, d3
pub fn i_066ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66ACC;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $066ACC: add.w d3, d3
pub fn i_066acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66ACE;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $066ACE: lea.l $2c07ac.l, a1
pub fn i_066ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AD4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $066AD4: lea.l $2c072c.l, a2
pub fn i_066ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66ADA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C072C;
    20
}
/// $066ADA: move.w (a1, d3.w), d1
pub fn i_066ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66ADE;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $066ADE: move.w (a2, d3.w), d2
pub fn i_066ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AE2;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $066AE2: muls.w d0, d1
pub fn i_066ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AE4;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $066AE4: muls.w d0, d2
pub fn i_066ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AE6;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $066AE6: asr.l #$8, d1
pub fn i_066ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AE8;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $066AE8: asr.l #$8, d2
pub fn i_066ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AEA;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $066AEA: move.w d1, $28(a6)
pub fn i_066aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AEE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066AEE: move.w d2, $2a(a6)
pub fn i_066aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AF2;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $066AF2: move.w #$d000, d0
pub fn i_066af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AF6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066AF6: jsr $28134.l
pub fn i_066af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66AFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66AFC);
    cpu.pc = 0x28134;
    16
}
/// $066AFC: andi.w #$ffe3, $38(a6)
pub fn i_066afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B02;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $066B02: ori.w #$14, $38(a6)
pub fn i_066b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B08;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $066B08: move.w #$17b, d1
pub fn i_066b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B0C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066B0C: jsr $236e.l
pub fn i_066b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B12);
    cpu.pc = 0x236E;
    16
}
/// $066B12: lea.l $66fee(pc), a0
pub fn i_066b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B16;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x66FEE;
    16
}
/// $066B16: jsr $28cd4.l
pub fn i_066b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B1C);
    cpu.pc = 0x28CD4;
    16
}
/// $066B1C: lea.l $66b22(pc), a1
pub fn i_066b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66B22;
    16
}
/// $066B20: move.l a1, (a6)
pub fn i_066b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066B22: jsr $27cee.l
pub fn i_066b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B28);
    cpu.pc = 0x27CEE;
    16
}
/// $066B28: jsr $28d70.l
pub fn i_066b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B2E);
    cpu.pc = 0x28D70;
    16
}
/// $066B2E: jsr $283d8.l
pub fn i_066b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B34);
    cpu.pc = 0x283D8;
    16
}
/// $066B34: btst.b #$1, $13(a6)
pub fn i_066b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B3A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $066B3A: beq.w $66b44
pub fn i_066b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x66B44; return 10; }
    8
}
/// $066B3E: lea.l $66a9c(pc), a1
pub fn i_066b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A9C;
    16
}
/// $066B42: move.l a1, (a6)
pub fn i_066b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066B44: cmpi.w #$1c8, $24(a6)
pub fn i_066b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B4A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1C8, Size::Word, false);
    16
}
/// $066B4A: blt.w $66b54
pub fn i_066b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B4E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x66B54; return 10; }
    8
}
/// $066B4E: lea.l $66a9c(pc), a1
pub fn i_066b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A9C;
    16
}
/// $066B52: move.l a1, (a6)
pub fn i_066b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066B54: movea.l #$ffffffff, a0
pub fn i_066b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B5A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $066B5A: jsr $5dd56.l
pub fn i_066b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B60);
    cpu.pc = 0x5DD56;
    16
}
/// $066B60: bcc.w $66b6a
pub fn i_066b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B64;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66B6A; return 10; }
    8
}
// ===== SetTaskHandler_066b64  ($066B64, 8 B, kernel/task_handlers.c)
/// $066B64: lea.l $66a8e(pc), a1
pub fn i_066b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A8E;
    16
}
/// $066B68: move.l a1, (a6)
pub fn i_066b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066B6A: rts
pub fn i_066b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_Smoke_066b6c  ($066B6C, 86 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066B6C: move.w #$c000, d0
pub fn i_066b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B70;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066B70: jsr $28134.l
pub fn i_066b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B76);
    cpu.pc = 0x28134;
    16
}
/// $066B76: andi.w #$ffe3, $38(a6)
pub fn i_066b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B7C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $066B7C: ori.w #$14, $38(a6)
pub fn i_066b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B82;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $066B82: move.w #$4, d1
pub fn i_066b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B86;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $066B86: jsr $236e.l
pub fn i_066b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B8C);
    cpu.pc = 0x236E;
    16
}
/// $066B8C: lea.l $67014(pc), a0
pub fn i_066b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B90;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x67014;
    16
}
/// $066B90: jsr $28cd4.l
pub fn i_066b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66B96);
    cpu.pc = 0x28CD4;
    16
}
/// $066B96: lea.l $66b9c(pc), a1
pub fn i_066b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66B9C;
    16
}
/// $066B9A: move.l a1, (a6)
pub fn i_066b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66B9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066B9C: jsr $2783a.l
pub fn i_066b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66BA2);
    cpu.pc = 0x2783A;
    16
}
/// $066BA2: jsr $28d70.l
pub fn i_066ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66BA8);
    cpu.pc = 0x28D70;
    16
}
/// $066BA8: bcc.w $66bb2
pub fn i_066ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66BB2; return 10; }
    8
}
/// $066BAC: lea.l $66a8e(pc), a1
pub fn i_066bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A8E;
    16
}
/// $066BB0: move.l a1, (a6)
pub fn i_066bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066BB2: movea.l #$ffffffff, a0
pub fn i_066bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BB8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $066BB8: jsr $5dd56.l
pub fn i_066bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66BBE);
    cpu.pc = 0x5DD56;
    16
}
/// $066BBE: bcc.w $66bc8
pub fn i_066bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66BC8; return 10; }
    8
}
// ===== SetTaskHandler_066bc2  ($066BC2, 8 B, kernel/task_handlers.c)
/// $066BC2: lea.l $66a8e(pc), a1
pub fn i_066bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66A8E;
    16
}
/// $066BC6: move.l a1, (a6)
pub fn i_066bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $066BC8: rts
pub fn i_066bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_Approach_066bca  ($066BCA, 54 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066BCA: jsr $5e0d4.l
pub fn i_066bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66BD0);
    cpu.pc = 0x5E0D4;
    16
}
/// $066BD0: move.w $22(a0), d0
pub fn i_066bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BD4;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $066BD4: sub.w $22(a6), d0
pub fn i_066bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BD8;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $066BD8: cmpi.w #$0, d0
pub fn i_066bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BDC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $066BDC: bgt.w $66be2
pub fn i_066bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BE0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x66BE2; return 10; }
    8
}
/// $066BE0: neg.w d0
pub fn i_066be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BE2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $066BE2: cmpi.w #$20, d0
pub fn i_066be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BE6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $066BE6: blt.w $66c00
pub fn i_066be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BEA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x66C00; return 10; }
    8
}
/// $066BEA: subi.w #$18, $22(a6)
pub fn i_066bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BF0;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $066BF0: jsr $5e5a8.l
pub fn i_066bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66BF6);
    cpu.pc = 0x5E5A8;
    16
}
/// $066BF6: bcc.w $66c06
pub fn i_066bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66BFA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x66C06; return 10; }
    8
}
/// $066BFA: addi.w #$18, $22(a6)
pub fn i_066bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C00;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetXN_066c00  ($066C00, 6 B, thunks/ccr_helpers.c)
/// $066C00: ori.b #$11, ccr
pub fn i_066c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C04;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $066C04: rts
pub fn i_066c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_ApproachFail_066c06  ($066C06, 6 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066C06: addi.w #$18, $22(a6)
pub fn i_066c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C0C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== ClearXN_066c0c  ($066C0C, 6 B, thunks/ccr_helpers.c)
/// $066C0C: andi.b #$ee, ccr
pub fn i_066c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C10;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $066C10: rts
pub fn i_066c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_TimerInBounds_066c12  ($066C12, 34 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066C12: subq.w #$1, $70(a6)
pub fn i_066c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C16;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $066C16: cmpi.w #$20, $22(a6)
pub fn i_066c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C1C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $066C1C: blt.w $66c3a
pub fn i_066c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C20;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x66C3A; return 10; }
    8
}
/// $066C20: cmpi.w #$120, $22(a6)
pub fn i_066c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C26;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $066C26: bgt.w $66c3a
pub fn i_066c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C2A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x66C3A; return 10; }
    8
}
/// $066C2A: cmpi.w #$0, $70(a6)
pub fn i_066c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C30;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $066C30: bgt.w $66c3a
pub fn i_066c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C34;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x66C3A; return 10; }
    8
}
// ===== SetXN_066c34  ($066C34, 6 B, thunks/ccr_helpers.c)
/// $066C34: ori.b #$11, ccr
pub fn i_066c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C38;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $066C38: rts
pub fn i_066c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_066c3a  ($066C3A, 6 B, thunks/ccr_helpers.c)
/// $066C3A: andi.b #$ee, ccr
pub fn i_066c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C3E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $066C3E: rts
pub fn i_066c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_SpawnPairA_066c40  ($066C40, 64 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066C40: lea.l $66b6c(pc), a1
pub fn i_066c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66B6C;
    16
}
/// $066C44: jsr $4ae.l
pub fn i_066c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66C4A);
    cpu.pc = 0x4AE;
    16
}
/// $066C4A: jsr $5dd02.l
pub fn i_066c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66C50);
    cpu.pc = 0x5DD02;
    16
}
/// $066C50: addi.w #$fff8, $22(a0)
pub fn i_066c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C56;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $066C56: addi.w #$20, $24(a0)
pub fn i_066c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C5C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $066C5C: lea.l $66ab8(pc), a1
pub fn i_066c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66AB8;
    16
}
/// $066C60: jsr $4ae.l
pub fn i_066c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66C66);
    cpu.pc = 0x4AE;
    16
}
/// $066C66: jsr $5dd02.l
pub fn i_066c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66C6C);
    cpu.pc = 0x5DD02;
    16
}
/// $066C6C: addi.w #$fff8, $22(a0)
pub fn i_066c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C72;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $066C72: addi.w #$20, $24(a0)
pub fn i_066c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C78;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $066C78: move.b #$40, $76(a0)
pub fn i_066c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C7E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $066C7E: rts
pub fn i_066c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_SpawnPairB_066c80  ($066C80, 64 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066C80: lea.l $66b6c(pc), a1
pub fn i_066c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66B6C;
    16
}
/// $066C84: jsr $4ae.l
pub fn i_066c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66C8A);
    cpu.pc = 0x4AE;
    16
}
/// $066C8A: jsr $5dd02.l
pub fn i_066c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66C90);
    cpu.pc = 0x5DD02;
    16
}
/// $066C90: addi.w #$ffec, $22(a0)
pub fn i_066c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C96;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFEC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $066C96: addi.w #$1c, $24(a0)
pub fn i_066c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66C9C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $066C9C: lea.l $66ab8(pc), a1
pub fn i_066c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66CA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x66AB8;
    16
}
/// $066CA0: jsr $4ae.l
pub fn i_066ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66CA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66CA6);
    cpu.pc = 0x4AE;
    16
}
/// $066CA6: jsr $5dd02.l
pub fn i_066ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66CAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66CAC);
    cpu.pc = 0x5DD02;
    16
}
/// $066CAC: addi.w #$ffec, $22(a0)
pub fn i_066cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66CB2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFEC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $066CB2: addi.w #$1c, $24(a0)
pub fn i_066cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66CB8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $066CB8: move.b #$50, $76(a0)
pub fn i_066cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66CBE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $066CBE: rts
pub fn i_066cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66CC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Paratrooper_SpawnChute_066cc0  ($066CC0, 24 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $066CC0: lea.l $788e4.l, a1
pub fn i_066cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66CC6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x788E4;
    20
}
/// $066CC6: jsr $4ae.l
pub fn i_066cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66CCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66CCC);
    cpu.pc = 0x4AE;
    16
}
/// $066CCC: jsr $5dd02.l
pub fn i_066ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66CD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x66CD2);
    cpu.pc = 0x5DD02;
    16
}
/// $066CD2: subq.w #$1, $38(a0)
pub fn i_066cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66CD6;
    cpu.ir = 0x5368;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $066CD6: rts
pub fn i_066cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x66CD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0670B6: movea.l $8(a6), a1
pub fn i_0670b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670BA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0670BA: move.b $10(a6), d0
pub fn i_0670ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670BE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0670BE: cmp.b $10(a1), d0
pub fn i_0670be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670C2;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0670C2: bcs.w $670cc
pub fn i_0670c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x670CC; return 10; }
    8
}
// ===== ClearXN_0670c6  ($0670C6, 6 B, thunks/ccr_helpers.c)
/// $0670C6: andi.b #$ee, ccr
pub fn i_0670c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670CA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0670CA: rts
pub fn i_0670ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0670cc  ($0670CC, 6 B, thunks/ccr_helpers.c)
/// $0670CC: ori.b #$11, ccr
pub fn i_0670cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670D0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0670D0: rts
pub fn i_0670d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_Tmpl6C_0670d2  ($0670D2, 258 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0670D2: jsr $67e7e(pc)
pub fn i_0670d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x670D6);
    cpu.pc = 0x67E7E;
    16
}
/// $0670D6: lea.l $2bf6de.l, a0
pub fn i_0670d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF6DE;
    20
}
/// $0670DC: jsr $799de.l
pub fn i_0670dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x670E2);
    cpu.pc = 0x799DE;
    16
}
/// $0670E2: move.w d0, $74(a6)
pub fn i_0670e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670E6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0670E6: cmpi.b #$0, $98(a6)
pub fn i_0670e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670EC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0670EC: beq.w $670f6
pub fn i_0670ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x670F6; return 10; }
    8
}
/// $0670F0: move.w #$36, $74(a6)
pub fn i_0670f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x36;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0670F6: tst.b $99(a6)
pub fn i_0670f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670FA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0670FA: beq.w $67254
pub fn i_0670fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x670FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x67254; return 10; }
    8
}
/// $0670FE: lea.l $2c7d1e.l, a0
pub fn i_0670fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67104;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7D1E;
    20
}
/// $067104: jsr $28cd4.l
pub fn i_067104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6710A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6710A);
    cpu.pc = 0x28CD4;
    16
}
/// $06710A: lea.l $67110(pc), a1
pub fn i_06710a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6710E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67110;
    16
}
/// $06710E: move.l a1, (a6)
pub fn i_06710e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67110;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067110: jsr $67fb8(pc)
pub fn i_067110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67114;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67114);
    cpu.pc = 0x67FB8;
    16
}
/// $067114: jsr $681ee(pc)
pub fn i_067114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67118;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67118);
    cpu.pc = 0x681EE;
    16
}
/// $067118: jsr $67f46(pc)
pub fn i_067118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6711C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6711C);
    cpu.pc = 0x67F46;
    16
}
/// $06711C: jsr $28d70.l
pub fn i_06711c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67122;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67122);
    cpu.pc = 0x28D70;
    16
}
/// $067122: tst.b $99(a6)
pub fn i_067122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67126;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $067126: bne.w $67146
pub fn i_067126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6712A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x67146; return 10; }
    8
}
/// $06712A: jsr $6815a(pc)
pub fn i_06712a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6712E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6712E);
    cpu.pc = 0x6815A;
    16
}
/// $06712E: bcc.w $67138
pub fn i_06712e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67132;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67138; return 10; }
    8
}
/// $067132: lea.l $6731e(pc), a1
pub fn i_067132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67136;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6731E;
    16
}
/// $067136: move.l a1, (a6)
pub fn i_067136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67138;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067138: jsr $6810a(pc)
pub fn i_067138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6713C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6713C);
    cpu.pc = 0x6810A;
    16
}
/// $06713C: bcc.w $67146
pub fn i_06713c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67140;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67146; return 10; }
    8
}
/// $067140: lea.l $67254(pc), a1
pub fn i_067140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67144;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67254;
    16
}
/// $067144: move.l a1, (a6)
pub fn i_067144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67146;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067146: jsr $68186(pc)
pub fn i_067146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6714A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6714A);
    cpu.pc = 0x68186;
    16
}
/// $06714A: bcc.w $67154
pub fn i_06714a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6714E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67154; return 10; }
    8
}
/// $06714E: lea.l $671dc(pc), a1
pub fn i_06714e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67152;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x671DC;
    16
}
/// $067152: move.l a1, (a6)
pub fn i_067152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67154;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067154: jsr $68020(pc)
pub fn i_067154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67158;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67158);
    cpu.pc = 0x68020;
    16
}
/// $067158: bcc.w $67162
pub fn i_067158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6715C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67162; return 10; }
    8
}
/// $06715C: lea.l $67394(pc), a1
pub fn i_06715c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67160;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67394;
    16
}
/// $067160: move.l a1, (a6)
pub fn i_067160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67162;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067162: jsr $680a4(pc)
pub fn i_067162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67166;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67166);
    cpu.pc = 0x680A4;
    16
}
/// $067166: bcc.w $67170
pub fn i_067166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6716A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67170; return 10; }
    8
}
/// $06716A: lea.l $674a2(pc), a1
pub fn i_06716a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6716E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x674A2;
    16
}
/// $06716E: move.l a1, (a6)
pub fn i_06716e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67170;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067170: jsr $681c6(pc)
pub fn i_067170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67174;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67174);
    cpu.pc = 0x681C6;
    16
}
/// $067174: bcc.w $6717e
pub fn i_067174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67178;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6717E; return 10; }
    8
}
/// $067178: lea.l $67220(pc), a1
pub fn i_067178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6717C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67220;
    16
}
/// $06717C: move.l a1, (a6)
pub fn i_06717c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6717E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06717E: subq.w #$1, $72(a6)
pub fn i_06717e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67182;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $067182: jsr $2870a.l
pub fn i_067182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67188;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67188);
    cpu.pc = 0x2870A;
    16
}
/// $067188: bcc.w $671b4
pub fn i_067188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6718C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x671B4; return 10; }
    8
}
/// $06718C: bclr.b #$3, $13(a6)
pub fn i_06718c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67192;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $067192: lea.l $5e766.l, a0
pub fn i_067192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67198;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $067198: jsr $5e770.l
pub fn i_067198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6719E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6719E);
    cpu.pc = 0x5E770;
    16
}
/// $06719E: lea.l $675bc(pc), a1
pub fn i_06719e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x675BC;
    16
}
/// $0671A2: move.l a1, (a6)
pub fn i_0671a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0671A4: cmpi.b #$1, $58(a6)
pub fn i_0671a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671AA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0671AA: beq.w $671b4
pub fn i_0671aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x671B4; return 10; }
    8
}
/// $0671AE: lea.l $675d2(pc), a1
pub fn i_0671ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x675D2;
    16
}
/// $0671B2: move.l a1, (a6)
pub fn i_0671b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0671B4: jsr $28758.l
pub fn i_0671b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x671BA);
    cpu.pc = 0x28758;
    16
}
/// $0671BA: bcs.w $67692
pub fn i_0671ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671BE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x67692; return 10; }
    8
}
/// $0671BE: movea.l #$ffffffff, a0
pub fn i_0671be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671C4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0671C4: lea.l $2c7ce6.l, a0
pub fn i_0671c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7CE6;
    20
}
/// $0671CA: jsr $5dd5c.l
pub fn i_0671ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x671D0);
    cpu.pc = 0x5DD5C;
    16
}
/// $0671D0: bcc.w $671da
pub fn i_0671d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x671DA; return 10; }
    8
}
// ===== SetTaskHandler_0671d4  ($0671D4, 8 B, kernel/task_handlers.c)
/// $0671D4: lea.l $67b7c(pc), a1
pub fn i_0671d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B7C;
    16
}
/// $0671D8: move.l a1, (a6)
pub fn i_0671d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0671DA: rts
pub fn i_0671da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_Guard_0671dc  ($0671DC, 68 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0671DC: clr.w $76(a6)
pub fn i_0671dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671E0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0671E0: lea.l $2c7ee6.l, a0
pub fn i_0671e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671E6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7EE6;
    20
}
/// $0671E6: jsr $28cd4.l
pub fn i_0671e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x671EC);
    cpu.pc = 0x28CD4;
    16
}
/// $0671EC: lea.l $671f2(pc), a1
pub fn i_0671ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x671F2;
    16
}
/// $0671F0: move.l a1, (a6)
pub fn i_0671f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0671F2: jsr $67fb8(pc)
pub fn i_0671f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x671F6);
    cpu.pc = 0x67FB8;
    16
}
/// $0671F6: jsr $681ee(pc)
pub fn i_0671f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x671FA);
    cpu.pc = 0x681EE;
    16
}
/// $0671FA: jsr $67f46(pc)
pub fn i_0671fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x671FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x671FE);
    cpu.pc = 0x67F46;
    16
}
/// $0671FE: jsr $28d70.l
pub fn i_0671fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67204);
    cpu.pc = 0x28D70;
    16
}
/// $067204: bcc.w $6720e
pub fn i_067204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67208;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6720E; return 10; }
    8
}
/// $067208: lea.l $670fe(pc), a1
pub fn i_067208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6720C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x670FE;
    16
}
/// $06720C: move.l a1, (a6)
pub fn i_06720c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6720E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06720E: jsr $681c6(pc)
pub fn i_06720e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67212;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67212);
    cpu.pc = 0x681C6;
    16
}
/// $067212: bcc.w $6721c
pub fn i_067212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67216;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6721C; return 10; }
    8
}
/// $067216: lea.l $67220(pc), a1
pub fn i_067216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6721A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67220;
    16
}
/// $06721A: move.l a1, (a6)
pub fn i_06721a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6721C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06721C: bra.w $6760e
pub fn i_06721c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67220;
    cpu.ir = 0x6000;
    cpu.pc = 0x6760E;
    10
}
// ===== ShieldSoldier_Fall_067220  ($067220, 192 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067220: lea.l $2c7f54.l, a0
pub fn i_067220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67226;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7F54;
    20
}
/// $067226: jsr $28cd4.l
pub fn i_067226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6722C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6722C);
    cpu.pc = 0x28CD4;
    16
}
/// $06722C: lea.l $67232(pc), a1
pub fn i_06722c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67230;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67232;
    16
}
/// $067230: move.l a1, (a6)
pub fn i_067230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67232;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067232: jsr $67fb8(pc)
pub fn i_067232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67236;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67236);
    cpu.pc = 0x67FB8;
    16
}
/// $067236: jsr $681ee(pc)
pub fn i_067236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6723A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6723A);
    cpu.pc = 0x681EE;
    16
}
/// $06723A: jsr $27c8c.l
pub fn i_06723a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67240);
    cpu.pc = 0x27C8C;
    16
}
/// $067240: bcc.w $6724a
pub fn i_067240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67244;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6724A; return 10; }
    8
}
/// $067244: lea.l $670fe(pc), a1
pub fn i_067244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67248;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x670FE;
    16
}
/// $067248: move.l a1, (a6)
pub fn i_067248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6724A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06724A: jsr $28d70.l
pub fn i_06724a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67250;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67250);
    cpu.pc = 0x28D70;
    16
}
/// $067250: bra.w $6760e
pub fn i_067250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67254;
    cpu.ir = 0x6000;
    cpu.pc = 0x6760E;
    10
}
/// $067254: jsr $67f86(pc)
pub fn i_067254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67258;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67258);
    cpu.pc = 0x67F86;
    16
}
/// $067258: move.l #$2c7a8a, $48(a6)
pub fn i_067258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67260;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C7A8A;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $067260: lea.l $2c7da4.l, a0
pub fn i_067260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67266;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7DA4;
    20
}
/// $067266: jsr $28cd4.l
pub fn i_067266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6726C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6726C);
    cpu.pc = 0x28CD4;
    16
}
/// $06726C: lea.l $67272(pc), a1
pub fn i_06726c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67270;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67272;
    16
}
/// $067270: move.l a1, (a6)
pub fn i_067270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67272;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067272: jsr $67fb8(pc)
pub fn i_067272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67276;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67276);
    cpu.pc = 0x67FB8;
    16
}
/// $067276: jsr $681ee(pc)
pub fn i_067276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6727A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6727A);
    cpu.pc = 0x681EE;
    16
}
/// $06727A: jsr $67f62(pc)
pub fn i_06727a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6727E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6727E);
    cpu.pc = 0x67F62;
    16
}
/// $06727E: jsr $28d70.l
pub fn i_06727e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67284;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67284);
    cpu.pc = 0x28D70;
    16
}
/// $067284: jsr $68034(pc)
pub fn i_067284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67288;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67288);
    cpu.pc = 0x68034;
    16
}
/// $067288: bcc.w $67292
pub fn i_067288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6728C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67292; return 10; }
    8
}
/// $06728C: lea.l $672e0(pc), a1
pub fn i_06728c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67290;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x672E0;
    16
}
/// $067290: move.l a1, (a6)
pub fn i_067290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67292;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067292: jsr $680b8(pc)
pub fn i_067292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67296;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67296);
    cpu.pc = 0x680B8;
    16
}
/// $067296: bcc.w $672a0
pub fn i_067296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6729A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x672A0; return 10; }
    8
}
/// $06729A: lea.l $672e0(pc), a1
pub fn i_06729a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6729E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x672E0;
    16
}
/// $06729E: move.l a1, (a6)
pub fn i_06729e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0672A0: jsr $68186(pc)
pub fn i_0672a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x672A4);
    cpu.pc = 0x68186;
    16
}
/// $0672A4: bcc.w $672ae
pub fn i_0672a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x672AE; return 10; }
    8
}
/// $0672A8: lea.l $672e0(pc), a1
pub fn i_0672a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x672E0;
    16
}
/// $0672AC: move.l a1, (a6)
pub fn i_0672ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0672AE: jsr $680e8(pc)
pub fn i_0672ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x672B2);
    cpu.pc = 0x680E8;
    16
}
/// $0672B2: bcc.w $672bc
pub fn i_0672b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x672BC; return 10; }
    8
}
/// $0672B6: lea.l $672e0(pc), a1
pub fn i_0672b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x672E0;
    16
}
/// $0672BA: move.l a1, (a6)
pub fn i_0672ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0672BC: jsr $681c6(pc)
pub fn i_0672bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x672C0);
    cpu.pc = 0x681C6;
    16
}
/// $0672C0: bcc.w $672ca
pub fn i_0672c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x672CA; return 10; }
    8
}
/// $0672C4: lea.l $67220(pc), a1
pub fn i_0672c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67220;
    16
}
/// $0672C8: move.l a1, (a6)
pub fn i_0672c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0672CA: cmpi.l #$67272, (a6)
pub fn i_0672ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672D0;
    cpu.ir = 0xC96;
    let v2 = bus.read32(cpu.a[6]);
    sub_with_flags(cpu, v2, 0x67272, Size::Long, false);
    16
}
/// $0672D0: beq.w $672dc
pub fn i_0672d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x672DC; return 10; }
    8
}
/// $0672D4: move.l #$2c79e2, $48(a6)
pub fn i_0672d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672DC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C79E2;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0672DC: bra.w $6760e
pub fn i_0672dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6760E;
    10
}
// ===== ShieldSoldier_Land_0672e0  ($0672E0, 180 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0672E0: jsr $267e2.l
pub fn i_0672e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x672E6);
    cpu.pc = 0x267E2;
    16
}
/// $0672E6: lea.l $2c7e2a.l, a0
pub fn i_0672e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7E2A;
    20
}
/// $0672EC: jsr $28cd4.l
pub fn i_0672ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x672F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0672F2: lea.l $672f8(pc), a1
pub fn i_0672f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x672F8;
    16
}
/// $0672F6: move.l a1, (a6)
pub fn i_0672f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0672F8: jsr $67f46(pc)
pub fn i_0672f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x672FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x672FC);
    cpu.pc = 0x67F46;
    16
}
/// $0672FC: jsr $28d70.l
pub fn i_0672fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67302;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67302);
    cpu.pc = 0x28D70;
    16
}
/// $067302: bcc.w $6730c
pub fn i_067302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67306;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6730C; return 10; }
    8
}
/// $067306: lea.l $670fe(pc), a1
pub fn i_067306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6730A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x670FE;
    16
}
/// $06730A: move.l a1, (a6)
pub fn i_06730a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6730C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06730C: jsr $681c6(pc)
pub fn i_06730c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67310;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67310);
    cpu.pc = 0x681C6;
    16
}
/// $067310: bcc.w $6731a
pub fn i_067310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67314;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6731A; return 10; }
    8
}
/// $067314: lea.l $67220(pc), a1
pub fn i_067314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67318;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67220;
    16
}
/// $067318: move.l a1, (a6)
pub fn i_067318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6731A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06731A: bra.w $6760e
pub fn i_06731a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6731E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6760E;
    10
}
/// $06731E: jsr $67f86(pc)
pub fn i_06731e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67322;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67322);
    cpu.pc = 0x67F86;
    16
}
/// $067322: neg.w $28(a6)
pub fn i_067322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67326;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $067326: lea.l $2c7e92.l, a0
pub fn i_067326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6732C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7E92;
    20
}
/// $06732C: jsr $28cd4.l
pub fn i_06732c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67332;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67332);
    cpu.pc = 0x28CD4;
    16
}
/// $067332: lea.l $67338(pc), a1
pub fn i_067332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67336;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67338;
    16
}
/// $067336: move.l a1, (a6)
pub fn i_067336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67338;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067338: jsr $67fb8(pc)
pub fn i_067338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6733C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6733C);
    cpu.pc = 0x67FB8;
    16
}
/// $06733C: jsr $681ee(pc)
pub fn i_06733c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67340;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67340);
    cpu.pc = 0x681EE;
    16
}
/// $067340: jsr $67f62(pc)
pub fn i_067340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67344;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67344);
    cpu.pc = 0x67F62;
    16
}
/// $067344: jsr $28d70.l
pub fn i_067344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6734A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6734A);
    cpu.pc = 0x28D70;
    16
}
/// $06734A: jsr $68130(pc)
pub fn i_06734a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6734E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6734E);
    cpu.pc = 0x68130;
    16
}
/// $06734E: bcc.w $67358
pub fn i_06734e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67352;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67358; return 10; }
    8
}
/// $067352: lea.l $670fe(pc), a1
pub fn i_067352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67356;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x670FE;
    16
}
/// $067356: move.l a1, (a6)
pub fn i_067356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67358;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067358: jsr $68020(pc)
pub fn i_067358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6735C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6735C);
    cpu.pc = 0x68020;
    16
}
/// $06735C: bcc.w $67366
pub fn i_06735c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67360;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67366; return 10; }
    8
}
/// $067360: lea.l $67394(pc), a1
pub fn i_067360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67364;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67394;
    16
}
/// $067364: move.l a1, (a6)
pub fn i_067364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67366;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067366: jsr $680a4(pc)
pub fn i_067366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6736A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6736A);
    cpu.pc = 0x680A4;
    16
}
/// $06736A: bcc.w $67374
pub fn i_06736a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6736E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67374; return 10; }
    8
}
/// $06736E: lea.l $674a2(pc), a1
pub fn i_06736e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67372;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x674A2;
    16
}
/// $067372: move.l a1, (a6)
pub fn i_067372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67374;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067374: jsr $68186(pc)
pub fn i_067374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67378;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67378);
    cpu.pc = 0x68186;
    16
}
/// $067378: bcc.w $67382
pub fn i_067378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6737C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67382; return 10; }
    8
}
/// $06737C: lea.l $671dc(pc), a1
pub fn i_06737c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67380;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x671DC;
    16
}
/// $067380: move.l a1, (a6)
pub fn i_067380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67382;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067382: jsr $681c6(pc)
pub fn i_067382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67386;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67386);
    cpu.pc = 0x681C6;
    16
}
/// $067386: bcc.w $67390
pub fn i_067386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6738A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67390; return 10; }
    8
}
/// $06738A: lea.l $67220(pc), a1
pub fn i_06738a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6738E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67220;
    16
}
/// $06738E: move.l a1, (a6)
pub fn i_06738e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67390;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067390: bra.w $6760e
pub fn i_067390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67394;
    cpu.ir = 0x6000;
    cpu.pc = 0x6760E;
    10
}
// ===== ShieldSoldier_Crouch_067394  ($067394, 80 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067394: lea.l $2bf822.l, a0
pub fn i_067394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6739A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF822;
    20
}
/// $06739A: jsr $799de.l
pub fn i_06739a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x673A0);
    cpu.pc = 0x799DE;
    16
}
/// $0673A0: move.b d0, $82(a6)
pub fn i_0673a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673A4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0673A4: lea.l $2c7fb0.l, a0
pub fn i_0673a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7FB0;
    20
}
/// $0673AA: jsr $28cd4.l
pub fn i_0673aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x673B0);
    cpu.pc = 0x28CD4;
    16
}
/// $0673B0: lea.l $673b6(pc), a1
pub fn i_0673b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x673B6;
    16
}
/// $0673B4: move.l a1, (a6)
pub fn i_0673b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0673B6: jsr $67fb8(pc)
pub fn i_0673b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x673BA);
    cpu.pc = 0x67FB8;
    16
}
/// $0673BA: jsr $681ee(pc)
pub fn i_0673ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x673BE);
    cpu.pc = 0x681EE;
    16
}
/// $0673BE: jsr $67f46(pc)
pub fn i_0673be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x673C2);
    cpu.pc = 0x67F46;
    16
}
/// $0673C2: jsr $28d70.l
pub fn i_0673c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x673C8);
    cpu.pc = 0x28D70;
    16
}
/// $0673C8: bcc.w $673d2
pub fn i_0673c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x673D2; return 10; }
    8
}
/// $0673CC: lea.l $673e4(pc), a1
pub fn i_0673cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x673E4;
    16
}
/// $0673D0: move.l a1, (a6)
pub fn i_0673d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0673D2: jsr $681c6(pc)
pub fn i_0673d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x673D6);
    cpu.pc = 0x681C6;
    16
}
/// $0673D6: bcc.w $673e0
pub fn i_0673d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x673E0; return 10; }
    8
}
/// $0673DA: lea.l $67220(pc), a1
pub fn i_0673da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67220;
    16
}
/// $0673DE: move.l a1, (a6)
pub fn i_0673de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0673E0: bra.w $6760e
pub fn i_0673e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x6760E;
    10
}
// ===== ShieldSoldier_CrouchHold_0673e4  ($0673E4, 110 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0673E4: lea.l $2bf926.l, a0
pub fn i_0673e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF926;
    20
}
/// $0673EA: jsr $799de.l
pub fn i_0673ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x673F0);
    cpu.pc = 0x799DE;
    16
}
/// $0673F0: move.w d0, $72(a6)
pub fn i_0673f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673F4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0673F4: lea.l $2c802a.l, a0
pub fn i_0673f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x673FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C802A;
    20
}
/// $0673FA: jsr $28cd4.l
pub fn i_0673fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67400;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67400);
    cpu.pc = 0x28CD4;
    16
}
/// $067400: lea.l $67406(pc), a1
pub fn i_067400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67404;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67406;
    16
}
/// $067404: move.l a1, (a6)
pub fn i_067404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67406;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067406: jsr $67fb8(pc)
pub fn i_067406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6740A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6740A);
    cpu.pc = 0x67FB8;
    16
}
/// $06740A: jsr $681ee(pc)
pub fn i_06740a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6740E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6740E);
    cpu.pc = 0x681EE;
    16
}
/// $06740E: jsr $67f46(pc)
pub fn i_06740e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67412;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67412);
    cpu.pc = 0x67F46;
    16
}
/// $067412: jsr $28d70.l
pub fn i_067412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67418;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67418);
    cpu.pc = 0x28D70;
    16
}
/// $067418: bcc.w $67440
pub fn i_067418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6741C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67440; return 10; }
    8
}
/// $06741C: cmpi.w #$0, $72(a6)
pub fn i_06741c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67422;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $067422: bgt.w $67440
pub fn i_067422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67426;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x67440; return 10; }
    8
}
/// $067426: lea.l $673e4(pc), a1
pub fn i_067426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6742A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x673E4;
    16
}
/// $06742A: move.l a1, (a6)
pub fn i_06742a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6742C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06742C: subq.b #$1, $82(a6)
pub fn i_06742c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67430;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $067430: cmpi.b #$0, $82(a6)
pub fn i_067430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67436;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $067436: bgt.w $67440
pub fn i_067436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6743A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x67440; return 10; }
    8
}
/// $06743A: lea.l $67452(pc), a1
pub fn i_06743a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6743E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67452;
    16
}
/// $06743E: move.l a1, (a6)
pub fn i_06743e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67440;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067440: jsr $681c6(pc)
pub fn i_067440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67444;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67444);
    cpu.pc = 0x681C6;
    16
}
/// $067444: bcc.w $6744e
pub fn i_067444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67448;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6744E; return 10; }
    8
}
/// $067448: lea.l $67220(pc), a1
pub fn i_067448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6744C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67220;
    16
}
/// $06744C: move.l a1, (a6)
pub fn i_06744c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6744E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06744E: bra.w $6760e
pub fn i_06744e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67452;
    cpu.ir = 0x6000;
    cpu.pc = 0x6760E;
    10
}
// ===== ShieldSoldier_Rise_067452  ($067452, 80 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067452: lea.l $2bf8a4.l, a0
pub fn i_067452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67458;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF8A4;
    20
}
/// $067458: jsr $799de.l
pub fn i_067458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6745E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6745E);
    cpu.pc = 0x799DE;
    16
}
/// $06745E: move.w d0, $72(a6)
pub fn i_06745e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67462;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067462: lea.l $2c806e.l, a0
pub fn i_067462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67468;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C806E;
    20
}
/// $067468: jsr $28cd4.l
pub fn i_067468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6746E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6746E);
    cpu.pc = 0x28CD4;
    16
}
/// $06746E: lea.l $67474(pc), a1
pub fn i_06746e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67472;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67474;
    16
}
/// $067472: move.l a1, (a6)
pub fn i_067472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67474;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067474: jsr $67fb8(pc)
pub fn i_067474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67478;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67478);
    cpu.pc = 0x67FB8;
    16
}
/// $067478: jsr $681ee(pc)
pub fn i_067478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6747C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6747C);
    cpu.pc = 0x681EE;
    16
}
/// $06747C: jsr $67f46(pc)
pub fn i_06747c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67480;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67480);
    cpu.pc = 0x67F46;
    16
}
/// $067480: jsr $28d70.l
pub fn i_067480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67486;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67486);
    cpu.pc = 0x28D70;
    16
}
/// $067486: bcc.w $67490
pub fn i_067486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6748A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67490; return 10; }
    8
}
/// $06748A: lea.l $670fe(pc), a1
pub fn i_06748a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6748E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x670FE;
    16
}
/// $06748E: move.l a1, (a6)
pub fn i_06748e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67490;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067490: jsr $681c6(pc)
pub fn i_067490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67494;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67494);
    cpu.pc = 0x681C6;
    16
}
/// $067494: bcc.w $6749e
pub fn i_067494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67498;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6749E; return 10; }
    8
}
/// $067498: lea.l $67220(pc), a1
pub fn i_067498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6749C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67220;
    16
}
/// $06749C: move.l a1, (a6)
pub fn i_06749c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6749E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06749E: bra.w $6760e
pub fn i_06749e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x6760E;
    10
}
// ===== ShieldSoldier_Aim_0674a2  ($0674A2, 80 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0674A2: lea.l $2bf9a8.l, a0
pub fn i_0674a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF9A8;
    20
}
/// $0674A8: jsr $799de.l
pub fn i_0674a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x674AE);
    cpu.pc = 0x799DE;
    16
}
/// $0674AE: move.b d0, $82(a6)
pub fn i_0674ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674B2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0674B2: lea.l $2c80e4.l, a0
pub fn i_0674b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C80E4;
    20
}
/// $0674B8: jsr $28cd4.l
pub fn i_0674b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x674BE);
    cpu.pc = 0x28CD4;
    16
}
/// $0674BE: lea.l $674c4(pc), a1
pub fn i_0674be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x674C4;
    16
}
/// $0674C2: move.l a1, (a6)
pub fn i_0674c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0674C4: jsr $67fb8(pc)
pub fn i_0674c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x674C8);
    cpu.pc = 0x67FB8;
    16
}
/// $0674C8: jsr $681ee(pc)
pub fn i_0674c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x674CC);
    cpu.pc = 0x681EE;
    16
}
/// $0674CC: jsr $67f46(pc)
pub fn i_0674cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x674D0);
    cpu.pc = 0x67F46;
    16
}
/// $0674D0: jsr $28d70.l
pub fn i_0674d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x674D6);
    cpu.pc = 0x28D70;
    16
}
/// $0674D6: bcc.w $674e0
pub fn i_0674d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x674E0; return 10; }
    8
}
/// $0674DA: lea.l $674f2(pc), a1
pub fn i_0674da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x674F2;
    16
}
/// $0674DE: move.l a1, (a6)
pub fn i_0674de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0674E0: jsr $681c6(pc)
pub fn i_0674e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x674E4);
    cpu.pc = 0x681C6;
    16
}
/// $0674E4: bcc.w $674ee
pub fn i_0674e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x674EE; return 10; }
    8
}
/// $0674E8: lea.l $67220(pc), a1
pub fn i_0674e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67220;
    16
}
/// $0674EC: move.l a1, (a6)
pub fn i_0674ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0674EE: bra.w $6760e
pub fn i_0674ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x6760E;
    10
}
// ===== ShieldSoldier_Attack_0674f2  ($0674F2, 122 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0674F2: lea.l $2bfaac.l, a0
pub fn i_0674f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674F8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFAAC;
    20
}
/// $0674F8: jsr $799de.l
pub fn i_0674f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x674FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x674FE);
    cpu.pc = 0x799DE;
    16
}
/// $0674FE: move.w d0, $72(a6)
pub fn i_0674fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67502;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067502: lea.l $2c8146.l, a0
pub fn i_067502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67508;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8146;
    20
}
/// $067508: jsr $28cd4.l
pub fn i_067508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6750E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6750E);
    cpu.pc = 0x28CD4;
    16
}
/// $06750E: lea.l $67514(pc), a1
pub fn i_06750e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67512;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67514;
    16
}
/// $067512: move.l a1, (a6)
pub fn i_067512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67514;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067514: jsr $67fb8(pc)
pub fn i_067514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67518;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67518);
    cpu.pc = 0x67FB8;
    16
}
/// $067518: jsr $681ee(pc)
pub fn i_067518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6751C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6751C);
    cpu.pc = 0x681EE;
    16
}
/// $06751C: jsr $67f46(pc)
pub fn i_06751c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67520;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67520);
    cpu.pc = 0x67F46;
    16
}
/// $067520: jsr $28d70.l
pub fn i_067520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67526;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67526);
    cpu.pc = 0x28D70;
    16
}
/// $067526: bcc.w $6754e
pub fn i_067526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6752A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6754E; return 10; }
    8
}
/// $06752A: cmpi.w #$0, $72(a6)
pub fn i_06752a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67530;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $067530: bgt.w $6754e
pub fn i_067530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67534;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6754E; return 10; }
    8
}
/// $067534: lea.l $674f2(pc), a1
pub fn i_067534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67538;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x674F2;
    16
}
/// $067538: move.l a1, (a6)
pub fn i_067538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6753A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06753A: subq.b #$1, $82(a6)
pub fn i_06753a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6753E;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06753E: cmpi.b #$0, $82(a6)
pub fn i_06753e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67544;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $067544: bgt.w $6754e
pub fn i_067544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67548;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6754E; return 10; }
    8
}
/// $067548: lea.l $6756c(pc), a1
pub fn i_067548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6754C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6756C;
    16
}
/// $06754C: move.l a1, (a6)
pub fn i_06754c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6754E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06754E: jsr $283ca.l
pub fn i_06754e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67554;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67554);
    cpu.pc = 0x283CA;
    16
}
/// $067554: jsr $283d8.l
pub fn i_067554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6755A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6755A);
    cpu.pc = 0x283D8;
    16
}
/// $06755A: jsr $681c6(pc)
pub fn i_06755a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6755E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6755E);
    cpu.pc = 0x681C6;
    16
}
/// $06755E: bcc.w $67568
pub fn i_06755e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67562;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67568; return 10; }
    8
}
/// $067562: lea.l $67220(pc), a1
pub fn i_067562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67566;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67220;
    16
}
/// $067566: move.l a1, (a6)
pub fn i_067566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67568;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067568: bra.w $6760e
pub fn i_067568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6756C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6760E;
    10
}
// ===== ShieldSoldier_Recover_06756c  ($06756C, 120 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06756C: lea.l $2bfa2a.l, a0
pub fn i_06756c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67572;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFA2A;
    20
}
/// $067572: jsr $799de.l
pub fn i_067572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67578;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67578);
    cpu.pc = 0x799DE;
    16
}
/// $067578: move.w d0, $72(a6)
pub fn i_067578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6757C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06757C: lea.l $2c82a0.l, a0
pub fn i_06757c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67582;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C82A0;
    20
}
/// $067582: jsr $28cd4.l
pub fn i_067582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67588;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67588);
    cpu.pc = 0x28CD4;
    16
}
/// $067588: lea.l $6758e(pc), a1
pub fn i_067588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6758C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6758E;
    16
}
/// $06758C: move.l a1, (a6)
pub fn i_06758c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6758E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06758E: jsr $67fb8(pc)
pub fn i_06758e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67592;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67592);
    cpu.pc = 0x67FB8;
    16
}
/// $067592: jsr $681ee(pc)
pub fn i_067592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67596;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67596);
    cpu.pc = 0x681EE;
    16
}
/// $067596: jsr $67f46(pc)
pub fn i_067596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6759A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6759A);
    cpu.pc = 0x67F46;
    16
}
/// $06759A: jsr $28d70.l
pub fn i_06759a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x675A0);
    cpu.pc = 0x28D70;
    16
}
/// $0675A0: bcc.w $675aa
pub fn i_0675a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675A4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x675AA; return 10; }
    8
}
/// $0675A4: lea.l $670fe(pc), a1
pub fn i_0675a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x670FE;
    16
}
/// $0675A8: move.l a1, (a6)
pub fn i_0675a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0675AA: jsr $681c6(pc)
pub fn i_0675aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x675AE);
    cpu.pc = 0x681C6;
    16
}
/// $0675AE: bcc.w $675b8
pub fn i_0675ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x675B8; return 10; }
    8
}
/// $0675B2: lea.l $67220(pc), a1
pub fn i_0675b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67220;
    16
}
/// $0675B6: move.l a1, (a6)
pub fn i_0675b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0675B8: bra.w $6760e
pub fn i_0675b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x6760E;
    10
}
/// $0675BC: lea.l $2c834c.l, a0
pub fn i_0675bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C834C;
    20
}
/// $0675C2: jsr $28cd4.l
pub fn i_0675c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x675C8);
    cpu.pc = 0x28CD4;
    16
}
/// $0675C8: lea.l $675e4(pc), a1
pub fn i_0675c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x675E4;
    16
}
/// $0675CC: move.l a1, (a6)
pub fn i_0675cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0675CE: bra.w $675e4
pub fn i_0675ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x675E4;
    10
}
/// $0675D2: lea.l $2c82ec.l, a0
pub fn i_0675d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C82EC;
    20
}
/// $0675D8: jsr $28cd4.l
pub fn i_0675d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x675DE);
    cpu.pc = 0x28CD4;
    16
}
/// $0675DE: lea.l $675e4(pc), a1
pub fn i_0675de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x675E4;
    16
}
/// $0675E2: move.l a1, (a6)
pub fn i_0675e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== ShieldSoldier_Tail_0675e4  ($0675E4, 106 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0675E4: jsr $67fb8(pc)
pub fn i_0675e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x675E8);
    cpu.pc = 0x67FB8;
    16
}
/// $0675E8: jsr $681ee(pc)
pub fn i_0675e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x675EC);
    cpu.pc = 0x681EE;
    16
}
/// $0675EC: jsr $67f46(pc)
pub fn i_0675ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x675F0);
    cpu.pc = 0x67F46;
    16
}
/// $0675F0: jsr $28d70.l
pub fn i_0675f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x675F6);
    cpu.pc = 0x28D70;
    16
}
/// $0675F6: bcc.w $67600
pub fn i_0675f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67600; return 10; }
    8
}
/// $0675FA: lea.l $670fe(pc), a1
pub fn i_0675fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x675FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x670FE;
    16
}
/// $0675FE: move.l a1, (a6)
pub fn i_0675fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67600;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067600: jsr $681c6(pc)
pub fn i_067600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67604;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67604);
    cpu.pc = 0x681C6;
    16
}
/// $067604: bcc.w $6760e
pub fn i_067604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67608;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6760E; return 10; }
    8
}
/// $067608: lea.l $67220(pc), a1
pub fn i_067608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6760C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67220;
    16
}
/// $06760C: move.l a1, (a6)
pub fn i_06760c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6760E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06760E: subq.w #$1, $72(a6)
pub fn i_06760e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67612;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $067612: jsr $2870a.l
pub fn i_067612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67618;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67618);
    cpu.pc = 0x2870A;
    16
}
/// $067618: bcc.w $6762e
pub fn i_067618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6761C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6762E; return 10; }
    8
}
/// $06761C: bclr.b #$3, $13(a6)
pub fn i_06761c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67622;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $067622: lea.l $5e766.l, a0
pub fn i_067622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67628;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $067628: jsr $5e770.l
pub fn i_067628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6762E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6762E);
    cpu.pc = 0x5E770;
    16
}
/// $06762E: jsr $28758.l
pub fn i_06762e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67634;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67634);
    cpu.pc = 0x28758;
    16
}
/// $067634: bcs.w $67692
pub fn i_067634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67638;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x67692; return 10; }
    8
}
/// $067638: movea.l #$ffffffff, a0
pub fn i_067638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6763E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06763E: lea.l $2c7ce6.l, a0
pub fn i_06763e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67644;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7CE6;
    20
}
/// $067644: jsr $5dd5c.l
pub fn i_067644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6764A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6764A);
    cpu.pc = 0x5DD5C;
    16
}
/// $06764A: bcc.w $67654
pub fn i_06764a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6764E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67654; return 10; }
    8
}
// ===== SetTaskHandler_06764e  ($06764E, 8 B, kernel/task_handlers.c)
/// $06764E: lea.l $67b7c(pc), a1
pub fn i_06764e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67652;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B7C;
    16
}
/// $067652: move.l a1, (a6)
pub fn i_067652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67654;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067654: rts
pub fn i_067654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67656;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_ShieldLost_067656  ($067656, 54 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067656: jsr $68286(pc)
pub fn i_067656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6765A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6765A);
    cpu.pc = 0x68286;
    16
}
/// $06765A: lea.l $29bfc4.l, a0
pub fn i_06765a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67660;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BFC4;
    20
}
/// $067660: jsr $28cd4.l
pub fn i_067660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67666;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67666);
    cpu.pc = 0x28CD4;
    16
}
/// $067666: lea.l $6766c(pc), a1
pub fn i_067666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6766A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6766C;
    16
}
/// $06766A: move.l a1, (a6)
pub fn i_06766a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6766C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06766C: jsr $67f46(pc)
pub fn i_06766c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67670;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67670);
    cpu.pc = 0x67F46;
    16
}
/// $067670: jsr $28d70.l
pub fn i_067670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67676;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67676);
    cpu.pc = 0x28D70;
    16
}
/// $067676: bcc.w $67680
pub fn i_067676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6767A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67680; return 10; }
    8
}
/// $06767A: lea.l $6768c(pc), a1
pub fn i_06767a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6767E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6768C;
    16
}
/// $06767E: move.l a1, (a6)
pub fn i_06767e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67680;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067680: move.l #$ffffffff, $48(a6)
pub fn i_067680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67688;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $067688: bra.w $6760e
pub fn i_067688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6768C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6760E;
    10
}
// ===== ShieldSoldier_ToFlee_06768c  ($06768C, 6 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06768C: jmp $58f82.l
pub fn i_06768c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67692;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x58F82;
    8
}
// ===== ShieldSoldier_Die_067692  ($067692, 32 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067692: clr.b $20(a6)
pub fn i_067692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67696;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $067696: bset.b #$3, $13(a6)
pub fn i_067696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6769C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06769C: jsr $68260(pc)
pub fn i_06769c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x676A0);
    cpu.pc = 0x68260;
    16
}
/// $0676A0: tst.b $9a(a6)
pub fn i_0676a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676A4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0676A4: beq.w $676b2
pub fn i_0676a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x676B2; return 10; }
    8
}
/// $0676A8: lea.l $4abc0.l, a1
pub fn i_0676a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676AE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4ABC0;
    20
}
/// $0676AE: move.l a1, (a6)
pub fn i_0676ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0676B0: rts
pub fn i_0676b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_0676b2  ($0676B2, 8 B, thunks/jsr_abs_thunks.c)
/// $0676B2: jsr $49fd0.l
pub fn i_0676b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x676B8);
    cpu.pc = 0x49FD0;
    16
}
/// $0676B8: rts
pub fn i_0676b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_Tmpl6D_0676ba  ($0676BA, 190 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0676BA: jsr $67e7e(pc)
pub fn i_0676ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x676BE);
    cpu.pc = 0x67E7E;
    16
}
/// $0676BE: tst.b $99(a6)
pub fn i_0676be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676C2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0676C2: bne.w $677c4
pub fn i_0676c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676C6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x677C4; return 10; }
    8
}
/// $0676C6: lea.l $2c7d1e.l, a0
pub fn i_0676c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7D1E;
    20
}
/// $0676CC: jsr $28cd4.l
pub fn i_0676cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x676D2);
    cpu.pc = 0x28CD4;
    16
}
/// $0676D2: lea.l $676d8(pc), a1
pub fn i_0676d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x676D8;
    16
}
/// $0676D6: move.l a1, (a6)
pub fn i_0676d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0676D8: jsr $67fb8(pc)
pub fn i_0676d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x676DC);
    cpu.pc = 0x67FB8;
    16
}
/// $0676DC: jsr $681ee(pc)
pub fn i_0676dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x676E0);
    cpu.pc = 0x681EE;
    16
}
/// $0676E0: jsr $67f46(pc)
pub fn i_0676e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x676E4);
    cpu.pc = 0x67F46;
    16
}
/// $0676E4: jsr $28d70.l
pub fn i_0676e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x676EA);
    cpu.pc = 0x28D70;
    16
}
/// $0676EA: jsr $68186(pc)
pub fn i_0676ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x676EE);
    cpu.pc = 0x68186;
    16
}
/// $0676EE: bcc.w $676f8
pub fn i_0676ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x676F8; return 10; }
    8
}
/// $0676F2: lea.l $67780(pc), a1
pub fn i_0676f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67780;
    16
}
/// $0676F6: move.l a1, (a6)
pub fn i_0676f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0676F8: jsr $68070(pc)
pub fn i_0676f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x676FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x676FC);
    cpu.pc = 0x68070;
    16
}
/// $0676FC: bcc.w $67706
pub fn i_0676fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67700;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67706; return 10; }
    8
}
/// $067700: lea.l $67886(pc), a1
pub fn i_067700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67704;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67886;
    16
}
/// $067704: move.l a1, (a6)
pub fn i_067704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67706;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067706: jsr $680a4(pc)
pub fn i_067706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6770A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6770A);
    cpu.pc = 0x680A4;
    16
}
/// $06770A: bcc.w $67714
pub fn i_06770a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6770E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67714; return 10; }
    8
}
/// $06770E: lea.l $67994(pc), a1
pub fn i_06770e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67712;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67994;
    16
}
/// $067712: move.l a1, (a6)
pub fn i_067712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67714;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067714: jsr $681c6(pc)
pub fn i_067714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67718;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67718);
    cpu.pc = 0x681C6;
    16
}
/// $067718: bcc.w $67722
pub fn i_067718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6771C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67722; return 10; }
    8
}
/// $06771C: lea.l $67aae(pc), a1
pub fn i_06771c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67720;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AAE;
    16
}
/// $067720: move.l a1, (a6)
pub fn i_067720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67722;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067722: subq.w #$1, $72(a6)
pub fn i_067722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67726;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $067726: jsr $2870a.l
pub fn i_067726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6772C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6772C);
    cpu.pc = 0x2870A;
    16
}
/// $06772C: bcc.w $67758
pub fn i_06772c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67730;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67758; return 10; }
    8
}
/// $067730: bclr.b #$3, $13(a6)
pub fn i_067730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67736;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $067736: lea.l $5e766.l, a0
pub fn i_067736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6773C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06773C: jsr $5e770.l
pub fn i_06773c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67742;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67742);
    cpu.pc = 0x5E770;
    16
}
/// $067742: lea.l $67ae2(pc), a1
pub fn i_067742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67746;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AE2;
    16
}
/// $067746: move.l a1, (a6)
pub fn i_067746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67748;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067748: cmpi.b #$1, $58(a6)
pub fn i_067748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6774E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06774E: beq.w $67758
pub fn i_06774e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67752;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x67758; return 10; }
    8
}
/// $067752: lea.l $67af8(pc), a1
pub fn i_067752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67756;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AF8;
    16
}
/// $067756: move.l a1, (a6)
pub fn i_067756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67758;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067758: jsr $28758.l
pub fn i_067758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6775E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6775E);
    cpu.pc = 0x28758;
    16
}
/// $06775E: bcs.w $67692
pub fn i_06775e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67762;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x67692; return 10; }
    8
}
/// $067762: movea.l #$ffffffff, a0
pub fn i_067762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67768;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $067768: lea.l $2c7ce6.l, a0
pub fn i_067768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6776E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7CE6;
    20
}
/// $06776E: jsr $5dd5c.l
pub fn i_06776e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67774;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67774);
    cpu.pc = 0x5DD5C;
    16
}
/// $067774: bcc.w $6777e
pub fn i_067774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67778;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6777E; return 10; }
    8
}
// ===== SetTaskHandler_067778  ($067778, 8 B, kernel/task_handlers.c)
/// $067778: lea.l $67b7c(pc), a1
pub fn i_067778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6777C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B7C;
    16
}
/// $06777C: move.l a1, (a6)
pub fn i_06777c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6777E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06777E: rts
pub fn i_06777e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67780;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldierB_Guard_067780  ($067780, 68 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067780: clr.w $76(a6)
pub fn i_067780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67784;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $067784: lea.l $2c7ee6.l, a0
pub fn i_067784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6778A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7EE6;
    20
}
/// $06778A: jsr $28cd4.l
pub fn i_06778a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67790);
    cpu.pc = 0x28CD4;
    16
}
/// $067790: lea.l $67796(pc), a1
pub fn i_067790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67794;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67796;
    16
}
/// $067794: move.l a1, (a6)
pub fn i_067794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67796;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067796: jsr $67fb8(pc)
pub fn i_067796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6779A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6779A);
    cpu.pc = 0x67FB8;
    16
}
/// $06779A: jsr $681ee(pc)
pub fn i_06779a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6779E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6779E);
    cpu.pc = 0x681EE;
    16
}
/// $06779E: jsr $67f46(pc)
pub fn i_06779e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x677A2);
    cpu.pc = 0x67F46;
    16
}
/// $0677A2: jsr $28d70.l
pub fn i_0677a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x677A8);
    cpu.pc = 0x28D70;
    16
}
/// $0677A8: bcc.w $677b2
pub fn i_0677a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x677B2; return 10; }
    8
}
/// $0677AC: lea.l $676c6(pc), a1
pub fn i_0677ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x676C6;
    16
}
/// $0677B0: move.l a1, (a6)
pub fn i_0677b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0677B2: jsr $681c6(pc)
pub fn i_0677b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x677B6);
    cpu.pc = 0x681C6;
    16
}
/// $0677B6: bcc.w $677c0
pub fn i_0677b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x677C0; return 10; }
    8
}
/// $0677BA: lea.l $67aae(pc), a1
pub fn i_0677ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AAE;
    16
}
/// $0677BE: move.l a1, (a6)
pub fn i_0677be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0677C0: bra.w $67b34
pub fn i_0677c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x67B34;
    10
}
// ===== ShieldSoldierB_Run_0677c4  ($0677C4, 130 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0677C4: jsr $67f86(pc)
pub fn i_0677c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x677C8);
    cpu.pc = 0x67F86;
    16
}
/// $0677C8: move.l #$2c7a8a, $48(a6)
pub fn i_0677c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677D0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C7A8A;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0677D0: lea.l $2c7da4.l, a0
pub fn i_0677d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7DA4;
    20
}
/// $0677D6: jsr $28cd4.l
pub fn i_0677d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x677DC);
    cpu.pc = 0x28CD4;
    16
}
/// $0677DC: lea.l $677e2(pc), a1
pub fn i_0677dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x677E2;
    16
}
/// $0677E0: move.l a1, (a6)
pub fn i_0677e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0677E2: jsr $67fb8(pc)
pub fn i_0677e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x677E6);
    cpu.pc = 0x67FB8;
    16
}
/// $0677E6: jsr $681ee(pc)
pub fn i_0677e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x677EA);
    cpu.pc = 0x681EE;
    16
}
/// $0677EA: jsr $67f62(pc)
pub fn i_0677ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x677EE);
    cpu.pc = 0x67F62;
    16
}
/// $0677EE: jsr $28d70.l
pub fn i_0677ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x677F4);
    cpu.pc = 0x28D70;
    16
}
/// $0677F4: move.b $99(a6), d0
pub fn i_0677f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677F8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0677F8: andi.w #$ff, d0
pub fn i_0677f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677FC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0677FC: lsl.w #$4, d0
pub fn i_0677fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x677FE;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0677FE: btst.b #$0, $3a(a6)
pub fn i_0677fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67804;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $067804: bne.w $67814
pub fn i_067804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67808;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x67814; return 10; }
    8
}
/// $067808: cmp.w $22(a6), d0
pub fn i_067808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6780C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06780C: bgt.w $6781c
pub fn i_06780c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67810;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6781C; return 10; }
    8
}
/// $067810: bra.w $67822
pub fn i_067810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67814;
    cpu.ir = 0x6000;
    cpu.pc = 0x67822;
    10
}
/// $067814: cmp.w $22(a6), d0
pub fn i_067814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67818;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $067818: bgt.w $67822
pub fn i_067818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6781C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x67822; return 10; }
    8
}
/// $06781C: lea.l $67846(pc), a1
pub fn i_06781c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67820;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67846;
    16
}
/// $067820: move.l a1, (a6)
pub fn i_067820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67822;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067822: cmpi.l #$677e2, (a6)
pub fn i_067822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67828;
    cpu.ir = 0xC96;
    let v2 = bus.read32(cpu.a[6]);
    sub_with_flags(cpu, v2, 0x677E2, Size::Long, false);
    16
}
/// $067828: beq.w $67834
pub fn i_067828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6782C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x67834; return 10; }
    8
}
/// $06782C: move.l #$2c79e2, $48(a6)
pub fn i_06782c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67834;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C79E2;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $067834: jsr $681c6(pc)
pub fn i_067834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67838;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67838);
    cpu.pc = 0x681C6;
    16
}
/// $067838: bcc.w $67842
pub fn i_067838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6783C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67842; return 10; }
    8
}
/// $06783C: lea.l $67aae(pc), a1
pub fn i_06783c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67840;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AAE;
    16
}
/// $067840: move.l a1, (a6)
pub fn i_067840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67842;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067842: bra.w $67b34
pub fn i_067842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67846;
    cpu.ir = 0x6000;
    cpu.pc = 0x67B34;
    10
}
// ===== ShieldSoldierB_Land_067846  ($067846, 144 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067846: jsr $267e2.l
pub fn i_067846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6784C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6784C);
    cpu.pc = 0x267E2;
    16
}
/// $06784C: lea.l $2c7e2a.l, a0
pub fn i_06784c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67852;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7E2A;
    20
}
/// $067852: jsr $28cd4.l
pub fn i_067852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67858;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67858);
    cpu.pc = 0x28CD4;
    16
}
/// $067858: lea.l $6785e(pc), a1
pub fn i_067858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6785C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6785E;
    16
}
/// $06785C: move.l a1, (a6)
pub fn i_06785c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6785E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06785E: jsr $2783a.l
pub fn i_06785e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67864;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67864);
    cpu.pc = 0x2783A;
    16
}
/// $067864: jsr $28d70.l
pub fn i_067864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6786A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6786A);
    cpu.pc = 0x28D70;
    16
}
/// $06786A: bcc.w $67874
pub fn i_06786a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6786E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67874; return 10; }
    8
}
/// $06786E: lea.l $676c6(pc), a1
pub fn i_06786e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67872;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x676C6;
    16
}
/// $067872: move.l a1, (a6)
pub fn i_067872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67874;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067874: jsr $681c6(pc)
pub fn i_067874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67878;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67878);
    cpu.pc = 0x681C6;
    16
}
/// $067878: bcc.w $67882
pub fn i_067878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6787C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67882; return 10; }
    8
}
/// $06787C: lea.l $67aae(pc), a1
pub fn i_06787c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67880;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AAE;
    16
}
/// $067880: move.l a1, (a6)
pub fn i_067880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67882;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067882: bra.w $67b34
pub fn i_067882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67886;
    cpu.ir = 0x6000;
    cpu.pc = 0x67B34;
    10
}
/// $067886: lea.l $2bf822.l, a0
pub fn i_067886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6788C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF822;
    20
}
/// $06788C: jsr $799de.l
pub fn i_06788c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67892;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67892);
    cpu.pc = 0x799DE;
    16
}
/// $067892: move.b d0, $82(a6)
pub fn i_067892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67896;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $067896: lea.l $2c7fb0.l, a0
pub fn i_067896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6789C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7FB0;
    20
}
/// $06789C: jsr $28cd4.l
pub fn i_06789c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x678A2);
    cpu.pc = 0x28CD4;
    16
}
/// $0678A2: lea.l $678a8(pc), a1
pub fn i_0678a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x678A8;
    16
}
/// $0678A6: move.l a1, (a6)
pub fn i_0678a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0678A8: jsr $67fb8(pc)
pub fn i_0678a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x678AC);
    cpu.pc = 0x67FB8;
    16
}
/// $0678AC: jsr $681ee(pc)
pub fn i_0678ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x678B0);
    cpu.pc = 0x681EE;
    16
}
/// $0678B0: jsr $67f46(pc)
pub fn i_0678b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x678B4);
    cpu.pc = 0x67F46;
    16
}
/// $0678B4: jsr $28d70.l
pub fn i_0678b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x678BA);
    cpu.pc = 0x28D70;
    16
}
/// $0678BA: bcc.w $678c4
pub fn i_0678ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x678C4; return 10; }
    8
}
/// $0678BE: lea.l $678d6(pc), a1
pub fn i_0678be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x678D6;
    16
}
/// $0678C2: move.l a1, (a6)
pub fn i_0678c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0678C4: jsr $681c6(pc)
pub fn i_0678c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x678C8);
    cpu.pc = 0x681C6;
    16
}
/// $0678C8: bcc.w $678d2
pub fn i_0678c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x678D2; return 10; }
    8
}
/// $0678CC: lea.l $67aae(pc), a1
pub fn i_0678cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AAE;
    16
}
/// $0678D0: move.l a1, (a6)
pub fn i_0678d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0678D2: bra.w $67b34
pub fn i_0678d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x67B34;
    10
}
// ===== ShieldSoldierB_CrouchHold_0678d6  ($0678D6, 110 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0678D6: lea.l $2bf926.l, a0
pub fn i_0678d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF926;
    20
}
/// $0678DC: jsr $799de.l
pub fn i_0678dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x678E2);
    cpu.pc = 0x799DE;
    16
}
/// $0678E2: move.w d0, $72(a6)
pub fn i_0678e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678E6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0678E6: lea.l $2c802a.l, a0
pub fn i_0678e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C802A;
    20
}
/// $0678EC: jsr $28cd4.l
pub fn i_0678ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x678F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0678F2: lea.l $678f8(pc), a1
pub fn i_0678f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x678F8;
    16
}
/// $0678F6: move.l a1, (a6)
pub fn i_0678f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0678F8: jsr $67fb8(pc)
pub fn i_0678f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x678FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x678FC);
    cpu.pc = 0x67FB8;
    16
}
/// $0678FC: jsr $681ee(pc)
pub fn i_0678fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67900;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67900);
    cpu.pc = 0x681EE;
    16
}
/// $067900: jsr $67f46(pc)
pub fn i_067900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67904;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67904);
    cpu.pc = 0x67F46;
    16
}
/// $067904: jsr $28d70.l
pub fn i_067904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6790A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6790A);
    cpu.pc = 0x28D70;
    16
}
/// $06790A: bcc.w $67932
pub fn i_06790a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6790E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67932; return 10; }
    8
}
/// $06790E: cmpi.w #$0, $72(a6)
pub fn i_06790e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67914;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $067914: bgt.w $67440
pub fn i_067914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67918;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x67440; return 10; }
    8
}
/// $067918: lea.l $678d6(pc), a1
pub fn i_067918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6791C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x678D6;
    16
}
/// $06791C: move.l a1, (a6)
pub fn i_06791c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6791E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06791E: subq.b #$1, $82(a6)
pub fn i_06791e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67922;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $067922: cmpi.b #$0, $82(a6)
pub fn i_067922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67928;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $067928: bgt.w $67932
pub fn i_067928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6792C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x67932; return 10; }
    8
}
/// $06792C: lea.l $67944(pc), a1
pub fn i_06792c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67930;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67944;
    16
}
/// $067930: move.l a1, (a6)
pub fn i_067930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67932;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067932: jsr $681c6(pc)
pub fn i_067932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67936;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67936);
    cpu.pc = 0x681C6;
    16
}
/// $067936: bcc.w $67940
pub fn i_067936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6793A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67940; return 10; }
    8
}
/// $06793A: lea.l $67aae(pc), a1
pub fn i_06793a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6793E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AAE;
    16
}
/// $06793E: move.l a1, (a6)
pub fn i_06793e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67940;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067940: bra.w $67b34
pub fn i_067940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67944;
    cpu.ir = 0x6000;
    cpu.pc = 0x67B34;
    10
}
// ===== ShieldSoldierB_Rise_067944  ($067944, 160 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067944: lea.l $2bf8a4.l, a0
pub fn i_067944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6794A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF8A4;
    20
}
/// $06794A: jsr $799de.l
pub fn i_06794a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67950;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67950);
    cpu.pc = 0x799DE;
    16
}
/// $067950: move.w d0, $72(a6)
pub fn i_067950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67954;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067954: lea.l $2c806e.l, a0
pub fn i_067954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6795A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C806E;
    20
}
/// $06795A: jsr $28cd4.l
pub fn i_06795a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67960;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67960);
    cpu.pc = 0x28CD4;
    16
}
/// $067960: lea.l $67966(pc), a1
pub fn i_067960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67964;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67966;
    16
}
/// $067964: move.l a1, (a6)
pub fn i_067964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67966;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067966: jsr $67fb8(pc)
pub fn i_067966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6796A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6796A);
    cpu.pc = 0x67FB8;
    16
}
/// $06796A: jsr $681ee(pc)
pub fn i_06796a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6796E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6796E);
    cpu.pc = 0x681EE;
    16
}
/// $06796E: jsr $67f46(pc)
pub fn i_06796e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67972;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67972);
    cpu.pc = 0x67F46;
    16
}
/// $067972: jsr $28d70.l
pub fn i_067972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67978;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67978);
    cpu.pc = 0x28D70;
    16
}
/// $067978: bcc.w $67982
pub fn i_067978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6797C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67982; return 10; }
    8
}
/// $06797C: lea.l $676c6(pc), a1
pub fn i_06797c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67980;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x676C6;
    16
}
/// $067980: move.l a1, (a6)
pub fn i_067980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67982;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067982: jsr $681c6(pc)
pub fn i_067982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67986;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67986);
    cpu.pc = 0x681C6;
    16
}
/// $067986: bcc.w $67990
pub fn i_067986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6798A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67990; return 10; }
    8
}
/// $06798A: lea.l $67aae(pc), a1
pub fn i_06798a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6798E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AAE;
    16
}
/// $06798E: move.l a1, (a6)
pub fn i_06798e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67990;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067990: bra.w $67b34
pub fn i_067990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67994;
    cpu.ir = 0x6000;
    cpu.pc = 0x67B34;
    10
}
/// $067994: lea.l $2bf9a8.l, a0
pub fn i_067994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6799A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF9A8;
    20
}
/// $06799A: jsr $799de.l
pub fn i_06799a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x679A0);
    cpu.pc = 0x799DE;
    16
}
/// $0679A0: move.b d0, $82(a6)
pub fn i_0679a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679A4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0679A4: lea.l $2c80e4.l, a0
pub fn i_0679a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C80E4;
    20
}
/// $0679AA: jsr $28cd4.l
pub fn i_0679aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x679B0);
    cpu.pc = 0x28CD4;
    16
}
/// $0679B0: lea.l $679b6(pc), a1
pub fn i_0679b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x679B6;
    16
}
/// $0679B4: move.l a1, (a6)
pub fn i_0679b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0679B6: jsr $67fb8(pc)
pub fn i_0679b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x679BA);
    cpu.pc = 0x67FB8;
    16
}
/// $0679BA: jsr $681ee(pc)
pub fn i_0679ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x679BE);
    cpu.pc = 0x681EE;
    16
}
/// $0679BE: jsr $67f46(pc)
pub fn i_0679be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x679C2);
    cpu.pc = 0x67F46;
    16
}
/// $0679C2: jsr $28d70.l
pub fn i_0679c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x679C8);
    cpu.pc = 0x28D70;
    16
}
/// $0679C8: bcc.w $679d2
pub fn i_0679c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x679D2; return 10; }
    8
}
/// $0679CC: lea.l $679e4(pc), a1
pub fn i_0679cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x679E4;
    16
}
/// $0679D0: move.l a1, (a6)
pub fn i_0679d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0679D2: jsr $681c6(pc)
pub fn i_0679d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x679D6);
    cpu.pc = 0x681C6;
    16
}
/// $0679D6: bcc.w $679e0
pub fn i_0679d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x679E0; return 10; }
    8
}
/// $0679DA: lea.l $67aae(pc), a1
pub fn i_0679da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AAE;
    16
}
/// $0679DE: move.l a1, (a6)
pub fn i_0679de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0679E0: bra.w $67b34
pub fn i_0679e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x67B34;
    10
}
// ===== ShieldSoldierB_Attack_0679e4  ($0679E4, 122 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0679E4: lea.l $2bfaac.l, a0
pub fn i_0679e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFAAC;
    20
}
/// $0679EA: jsr $799de.l
pub fn i_0679ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x679F0);
    cpu.pc = 0x799DE;
    16
}
/// $0679F0: move.w d0, $72(a6)
pub fn i_0679f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679F4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0679F4: lea.l $2c8146.l, a0
pub fn i_0679f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x679FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8146;
    20
}
/// $0679FA: jsr $28cd4.l
pub fn i_0679fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67A00);
    cpu.pc = 0x28CD4;
    16
}
/// $067A00: lea.l $67a06(pc), a1
pub fn i_067a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67A06;
    16
}
/// $067A04: move.l a1, (a6)
pub fn i_067a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067A06: jsr $67fb8(pc)
pub fn i_067a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A0A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67A0A);
    cpu.pc = 0x67FB8;
    16
}
/// $067A0A: jsr $681ee(pc)
pub fn i_067a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67A0E);
    cpu.pc = 0x681EE;
    16
}
/// $067A0E: jsr $67f46(pc)
pub fn i_067a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67A12);
    cpu.pc = 0x67F46;
    16
}
/// $067A12: jsr $28d70.l
pub fn i_067a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67A18);
    cpu.pc = 0x28D70;
    16
}
/// $067A18: bcc.w $67a40
pub fn i_067a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67A40; return 10; }
    8
}
/// $067A1C: cmpi.w #$0, $72(a6)
pub fn i_067a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A22;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $067A22: bgt.w $6754e
pub fn i_067a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A26;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6754E; return 10; }
    8
}
/// $067A26: lea.l $679e4(pc), a1
pub fn i_067a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x679E4;
    16
}
/// $067A2A: move.l a1, (a6)
pub fn i_067a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067A2C: subq.b #$1, $82(a6)
pub fn i_067a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A30;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $067A30: cmpi.b #$0, $82(a6)
pub fn i_067a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A36;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $067A36: bgt.w $67a40
pub fn i_067a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A3A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x67A40; return 10; }
    8
}
/// $067A3A: lea.l $67a5e(pc), a1
pub fn i_067a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67A5E;
    16
}
/// $067A3E: move.l a1, (a6)
pub fn i_067a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067A40: jsr $283ca.l
pub fn i_067a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67A46);
    cpu.pc = 0x283CA;
    16
}
/// $067A46: jsr $283d8.l
pub fn i_067a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67A4C);
    cpu.pc = 0x283D8;
    16
}
/// $067A4C: jsr $681c6(pc)
pub fn i_067a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67A50);
    cpu.pc = 0x681C6;
    16
}
/// $067A50: bcc.w $67a5a
pub fn i_067a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A54;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67A5A; return 10; }
    8
}
/// $067A54: lea.l $67aae(pc), a1
pub fn i_067a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AAE;
    16
}
/// $067A58: move.l a1, (a6)
pub fn i_067a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067A5A: bra.w $67b34
pub fn i_067a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x67B34;
    10
}
// ===== ShieldSoldierB_Recover_067a5e  ($067A5E, 80 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067A5E: lea.l $2bfa2a.l, a0
pub fn i_067a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFA2A;
    20
}
/// $067A64: jsr $799de.l
pub fn i_067a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67A6A);
    cpu.pc = 0x799DE;
    16
}
/// $067A6A: move.w d0, $72(a6)
pub fn i_067a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A6E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067A6E: lea.l $2c82a0.l, a0
pub fn i_067a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C82A0;
    20
}
/// $067A74: jsr $28cd4.l
pub fn i_067a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67A7A);
    cpu.pc = 0x28CD4;
    16
}
/// $067A7A: lea.l $67a80(pc), a1
pub fn i_067a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67A80;
    16
}
/// $067A7E: move.l a1, (a6)
pub fn i_067a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067A80: jsr $67fb8(pc)
pub fn i_067a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67A84);
    cpu.pc = 0x67FB8;
    16
}
/// $067A84: jsr $681ee(pc)
pub fn i_067a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A88;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67A88);
    cpu.pc = 0x681EE;
    16
}
/// $067A88: jsr $67f46(pc)
pub fn i_067a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67A8C);
    cpu.pc = 0x67F46;
    16
}
/// $067A8C: jsr $28d70.l
pub fn i_067a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67A92);
    cpu.pc = 0x28D70;
    16
}
/// $067A92: bcc.w $67a9c
pub fn i_067a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67A9C; return 10; }
    8
}
/// $067A96: lea.l $676c6(pc), a1
pub fn i_067a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x676C6;
    16
}
/// $067A9A: move.l a1, (a6)
pub fn i_067a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67A9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067A9C: jsr $681c6(pc)
pub fn i_067a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67AA0);
    cpu.pc = 0x681C6;
    16
}
/// $067AA0: bcc.w $67aaa
pub fn i_067aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AA4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67AAA; return 10; }
    8
}
/// $067AA4: lea.l $67aae(pc), a1
pub fn i_067aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AAE;
    16
}
/// $067AA8: move.l a1, (a6)
pub fn i_067aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067AAA: bra.w $67b34
pub fn i_067aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x67B34;
    10
}
// ===== ShieldSoldierB_Fall_067aae  ($067AAE, 92 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067AAE: lea.l $2c7f54.l, a0
pub fn i_067aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AB4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7F54;
    20
}
/// $067AB4: jsr $28cd4.l
pub fn i_067ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67ABA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67ABA);
    cpu.pc = 0x28CD4;
    16
}
/// $067ABA: lea.l $67ac0(pc), a1
pub fn i_067aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67ABE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AC0;
    16
}
/// $067ABE: move.l a1, (a6)
pub fn i_067abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067AC0: jsr $67fb8(pc)
pub fn i_067ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67AC4);
    cpu.pc = 0x67FB8;
    16
}
/// $067AC4: jsr $681ee(pc)
pub fn i_067ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67AC8);
    cpu.pc = 0x681EE;
    16
}
/// $067AC8: jsr $27c8c.l
pub fn i_067ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67ACE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67ACE);
    cpu.pc = 0x27C8C;
    16
}
/// $067ACE: bcc.w $67ad8
pub fn i_067ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AD2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67AD8; return 10; }
    8
}
/// $067AD2: lea.l $677c4(pc), a1
pub fn i_067ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x677C4;
    16
}
/// $067AD6: move.l a1, (a6)
pub fn i_067ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067AD8: jsr $28d70.l
pub fn i_067ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67ADE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67ADE);
    cpu.pc = 0x28D70;
    16
}
/// $067ADE: bra.w $67b34
pub fn i_067ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x67B34;
    10
}
/// $067AE2: lea.l $2c834c.l, a0
pub fn i_067ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AE8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C834C;
    20
}
/// $067AE8: jsr $28cd4.l
pub fn i_067ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67AEE);
    cpu.pc = 0x28CD4;
    16
}
/// $067AEE: lea.l $67b0a(pc), a1
pub fn i_067aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B0A;
    16
}
/// $067AF2: move.l a1, (a6)
pub fn i_067af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067AF4: bra.w $67b0a
pub fn i_067af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x67B0A;
    10
}
/// $067AF8: lea.l $2c82ec.l, a0
pub fn i_067af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67AFE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C82EC;
    20
}
/// $067AFE: jsr $28cd4.l
pub fn i_067afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67B04);
    cpu.pc = 0x28CD4;
    16
}
/// $067B04: lea.l $67b0a(pc), a1
pub fn i_067b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B0A;
    16
}
/// $067B08: move.l a1, (a6)
pub fn i_067b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== ShieldSoldierB_Tail_067b0a  ($067B0A, 106 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067B0A: jsr $67fb8(pc)
pub fn i_067b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67B0E);
    cpu.pc = 0x67FB8;
    16
}
/// $067B0E: jsr $681ee(pc)
pub fn i_067b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67B12);
    cpu.pc = 0x681EE;
    16
}
/// $067B12: jsr $67f46(pc)
pub fn i_067b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67B16);
    cpu.pc = 0x67F46;
    16
}
/// $067B16: jsr $28d70.l
pub fn i_067b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67B1C);
    cpu.pc = 0x28D70;
    16
}
/// $067B1C: bcc.w $67b26
pub fn i_067b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67B26; return 10; }
    8
}
/// $067B20: lea.l $676c6(pc), a1
pub fn i_067b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x676C6;
    16
}
/// $067B24: move.l a1, (a6)
pub fn i_067b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067B26: jsr $681c6(pc)
pub fn i_067b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x67B2A);
    cpu.pc = 0x681C6;
    16
}
/// $067B2A: bcc.w $67b34
pub fn i_067b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B2E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67B34; return 10; }
    8
}
/// $067B2E: lea.l $67aae(pc), a1
pub fn i_067b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B32;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67AAE;
    16
}
/// $067B32: move.l a1, (a6)
pub fn i_067b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B34;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067B34: subq.w #$1, $72(a6)
pub fn i_067b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B38;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $067B38: jsr $2870a.l
pub fn i_067b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67B3E);
    cpu.pc = 0x2870A;
    16
}
/// $067B3E: bcc.w $67b54
pub fn i_067b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67B54; return 10; }
    8
}
/// $067B42: bclr.b #$3, $13(a6)
pub fn i_067b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B48;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $067B48: lea.l $5e766.l, a0
pub fn i_067b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B4E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $067B4E: jsr $5e770.l
pub fn i_067b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67B54);
    cpu.pc = 0x5E770;
    16
}
/// $067B54: jsr $28758.l
pub fn i_067b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67B5A);
    cpu.pc = 0x28758;
    16
}
/// $067B5A: bcs.w $67692
pub fn i_067b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B5E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x67692; return 10; }
    8
}
/// $067B5E: movea.l #$ffffffff, a0
pub fn i_067b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B64;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $067B64: lea.l $2c7ce6.l, a0
pub fn i_067b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7CE6;
    20
}
/// $067B6A: jsr $5dd5c.l
pub fn i_067b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67B70);
    cpu.pc = 0x5DD5C;
    16
}
/// $067B70: bcc.w $67b7a
pub fn i_067b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B74;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67B7A; return 10; }
    8
}
// ===== SetTaskHandler_067b74  ($067B74, 8 B, kernel/task_handlers.c)
/// $067B74: lea.l $67b7c(pc), a1
pub fn i_067b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B7C;
    16
}
/// $067B78: move.l a1, (a6)
pub fn i_067b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067B7A: rts
pub fn i_067b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_067b7c  ($067B7C, 8 B, kernel/jmp_to_scheduler.c)
/// $067B7C: jmp $518.l
pub fn i_067b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B82;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $067B82: rts
pub fn i_067b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_067b84  ($067B84, 14 B, kernel/jsr5b6_jmp518.c)
/// $067B84: jsr $5b6.l
pub fn i_067b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67B8A);
    cpu.pc = 0x5B6;
    16
}
/// $067B8A: jmp $518.l
pub fn i_067b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B90;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $067B90: rts
pub fn i_067b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Shield_Init_067b92  ($067B92, 226 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067B92: movea.l $c(a6), a0
pub fn i_067b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B96;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $067B96: tst.b $9a(a0)
pub fn i_067b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B9A;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $067B9A: bne.w $67bb8
pub fn i_067b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67B9E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x67BB8; return 10; }
    8
}
/// $067B9E: move.w #$38, d1
pub fn i_067b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BA2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067BA2: jsr $236e.l
pub fn i_067ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67BA8);
    cpu.pc = 0x236E;
    16
}
/// $067BA8: move.w #$1a, $1c(a6)
pub fn i_067ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BAE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067BAE: jsr $138fe.l
pub fn i_067bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67BB4);
    cpu.pc = 0x138FE;
    16
}
/// $067BB4: bra.w $67bce
pub fn i_067bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BB8;
    cpu.ir = 0x6000;
    cpu.pc = 0x67BCE;
    10
}
/// $067BB8: move.w #$1c7, d1
pub fn i_067bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BBC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067BBC: jsr $236e.l
pub fn i_067bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67BC2);
    cpu.pc = 0x236E;
    16
}
/// $067BC2: move.w #$1f, $1c(a6)
pub fn i_067bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067BC8: jsr $138fe.l
pub fn i_067bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67BCE);
    cpu.pc = 0x138FE;
    16
}
/// $067BCE: lea.l $2bf59a.l, a0
pub fn i_067bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF59A;
    20
}
/// $067BD4: jsr $799de.l
pub fn i_067bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67BDA);
    cpu.pc = 0x799DE;
    16
}
/// $067BDA: move.w d0, $66(a6)
pub fn i_067bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BDE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067BDE: move.l #$2c7a36, $48(a6)
pub fn i_067bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BE6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C7A36;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $067BE6: lea.l $67bec(pc), a1
pub fn i_067be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67BEC;
    16
}
/// $067BEA: move.l a1, (a6)
pub fn i_067bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067BEC: jsr $5e506.l
pub fn i_067bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67BF2);
    cpu.pc = 0x5E506;
    16
}
/// $067BF2: move.l $94(a0), $94(a6)
pub fn i_067bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BF8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $067BF8: move.b $80(a0), d0
pub fn i_067bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67BFC;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $067BFC: cmp.b $80(a6), d0
pub fn i_067bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C00;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $067C00: beq.w $67c28
pub fn i_067c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x67C28; return 10; }
    8
}
/// $067C04: move.b d0, $80(a6)
pub fn i_067c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C08;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $067C08: andi.w #$7, d0
pub fn i_067c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C0C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $067C0C: movea.l #$2c7cfe, a0
pub fn i_067c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C12;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2C7CFE;
    cpu.a[0] = v1;
    4
}
/// $067C12: lsl.w #$2, d0
pub fn i_067c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C14;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $067C14: movea.l (a0, d0.w), a0
pub fn i_067c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C18;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $067C18: cmpa.l #$ffffffff, a0
pub fn i_067c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C1E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $067C1E: beq.w $67c28
pub fn i_067c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C22;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x67C28; return 10; }
    8
}
/// $067C22: jsr $28cd4.l
pub fn i_067c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67C28);
    cpu.pc = 0x28CD4;
    16
}
/// $067C28: jsr $28d70.l
pub fn i_067c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67C2E);
    cpu.pc = 0x28D70;
    16
}
/// $067C2E: jsr $2870a.l
pub fn i_067c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67C34);
    cpu.pc = 0x2870A;
    16
}
/// $067C34: bcc.w $67c3e
pub fn i_067c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C38;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67C3E; return 10; }
    8
}
/// $067C38: bclr.b #$3, $13(a6)
pub fn i_067c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C3E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $067C3E: jsr $28758.l
pub fn i_067c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67C44);
    cpu.pc = 0x28758;
    16
}
/// $067C44: bcc.w $67c58
pub fn i_067c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C48;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67C58; return 10; }
    8
}
/// $067C48: movea.l $c(a6), a0
pub fn i_067c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C4C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $067C4C: move.l #$67656, (a0)
pub fn i_067c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C52;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x67656;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067C52: lea.l $67c7c(pc), a1
pub fn i_067c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67C7C;
    16
}
/// $067C56: move.l a1, (a6)
pub fn i_067c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067C58: movea.l $c(a6), a0
pub fn i_067c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C5C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $067C5C: tst.b $20(a0)
pub fn i_067c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C60;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $067C60: bne.w $67c6a
pub fn i_067c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C64;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x67C6A; return 10; }
    8
}
/// $067C64: lea.l $67c7c(pc), a1
pub fn i_067c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67C7C;
    16
}
/// $067C68: move.l a1, (a6)
pub fn i_067c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067C6A: jsr $5e45a.l
pub fn i_067c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67C70);
    cpu.pc = 0x5E45A;
    16
}
/// $067C70: bcc.w $67c7a
pub fn i_067c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C74;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67C7A; return 10; }
    8
}
// ===== SetTaskHandler_067c74  ($067C74, 8 B, kernel/task_handlers.c)
/// $067C74: lea.l $67b84(pc), a1
pub fn i_067c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B84;
    16
}
/// $067C78: move.l a1, (a6)
pub fn i_067c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067C7A: rts
pub fn i_067c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Shield_FlyOff_067c7c  ($067C7C, 100 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067C7C: addi.w #$fff8, $22(a6)
pub fn i_067c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C82;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $067C82: addi.w #$12, $24(a6)
pub fn i_067c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C88;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x12, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $067C88: move.w #$cc, d0
pub fn i_067c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C8C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xCC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067C8C: jsr $5dca4.l
pub fn i_067c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67C92);
    cpu.pc = 0x5DCA4;
    16
}
/// $067C92: move.w d0, $28(a6)
pub fn i_067c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C96;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067C96: move.w #$438, $2a(a6)
pub fn i_067c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67C9C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x438;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067C9C: move.w #$ffdc, $2e(a6)
pub fn i_067c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CA2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFDC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067CA2: move.w #$0, $2c(a6)
pub fn i_067ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CA8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067CA8: lea.l $2c836c.l, a0
pub fn i_067ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CAE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C836C;
    20
}
/// $067CAE: jsr $28cd4.l
pub fn i_067cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67CB4);
    cpu.pc = 0x28CD4;
    16
}
/// $067CB4: lea.l $67cba(pc), a1
pub fn i_067cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67CBA;
    16
}
/// $067CB8: move.l a1, (a6)
pub fn i_067cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067CBA: jsr $27c8c.l
pub fn i_067cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67CC0);
    cpu.pc = 0x27C8C;
    16
}
/// $067CC0: bcc.w $67cca
pub fn i_067cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67CCA; return 10; }
    8
}
/// $067CC4: lea.l $67ce8(pc), a1
pub fn i_067cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67CE8;
    16
}
/// $067CC8: move.l a1, (a6)
pub fn i_067cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067CCA: jsr $28d70.l
pub fn i_067cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67CD0);
    cpu.pc = 0x28D70;
    16
}
/// $067CD0: movea.l #$ffffffff, a0
pub fn i_067cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CD6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $067CD6: jsr $5dd56.l
pub fn i_067cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67CDC);
    cpu.pc = 0x5DD56;
    16
}
/// $067CDC: bcc.w $67ce6
pub fn i_067cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67CE6; return 10; }
    8
}
// ===== SetTaskHandler_067ce0  ($067CE0, 8 B, kernel/task_handlers.c)
/// $067CE0: lea.l $67b84(pc), a1
pub fn i_067ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B84;
    16
}
/// $067CE4: move.l a1, (a6)
pub fn i_067ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067CE6: rts
pub fn i_067ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Shield_Bounce_067ce8  ($067CE8, 74 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067CE8: move.w #$cc, d0
pub fn i_067ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CEC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xCC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067CEC: jsr $5dca4.l
pub fn i_067cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67CF2);
    cpu.pc = 0x5DCA4;
    16
}
/// $067CF2: move.w d0, $28(a6)
pub fn i_067cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CF6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067CF6: move.w #$65e, $2a(a6)
pub fn i_067cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67CFC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x65E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067CFC: move.w #$ff5d, $2e(a6)
pub fn i_067cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D02;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF5D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067D02: move.w #$0, $2c(a6)
pub fn i_067d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D08;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067D08: lea.l $2c836c.l, a0
pub fn i_067d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D0E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C836C;
    20
}
/// $067D0E: jsr $28cd4.l
pub fn i_067d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67D14);
    cpu.pc = 0x28CD4;
    16
}
/// $067D14: lea.l $67d1a(pc), a1
pub fn i_067d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67D1A;
    16
}
/// $067D18: move.l a1, (a6)
pub fn i_067d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067D1A: jsr $27c8c.l
pub fn i_067d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67D20);
    cpu.pc = 0x27C8C;
    16
}
/// $067D20: bcc.w $67d2a
pub fn i_067d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67D2A; return 10; }
    8
}
/// $067D24: lea.l $67d32(pc), a1
pub fn i_067d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67D32;
    16
}
/// $067D28: move.l a1, (a6)
pub fn i_067d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D2A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067D2A: jsr $28d70.l
pub fn i_067d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67D30);
    cpu.pc = 0x28D70;
    16
}
/// $067D30: bra.b $67cd0
pub fn i_067d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D32;
    cpu.ir = 0x609E;
    cpu.pc = 0x67CD0;
    10
}
// ===== Shield_Blink_067d32  ($067D32, 60 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067D32: move.w #$1e, $70(a6)
pub fn i_067d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D38;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067D38: lea.l $67d3e(pc), a1
pub fn i_067d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67D3E;
    16
}
/// $067D3C: move.l a1, (a6)
pub fn i_067d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067D3E: jsr $2783a.l
pub fn i_067d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67D44);
    cpu.pc = 0x2783A;
    16
}
/// $067D44: btst.b #$0, $106f28.l
pub fn i_067d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D4C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $067D4C: beq.w $67d56
pub fn i_067d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D50;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x67D56; return 10; }
    8
}
/// $067D50: jsr $28d70.l
pub fn i_067d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67D56);
    cpu.pc = 0x28D70;
    16
}
/// $067D56: subq.w #$1, $70(a6)
pub fn i_067d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D5A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $067D5A: cmpi.w #$0, $70(a6)
pub fn i_067d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D60;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $067D60: bgt.w $67d6a
pub fn i_067d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D64;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x67D6A; return 10; }
    8
}
/// $067D64: lea.l $67b84(pc), a1
pub fn i_067d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B84;
    16
}
/// $067D68: move.l a1, (a6)
pub fn i_067d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067D6A: bra.w $67cd0
pub fn i_067d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x67CD0;
    10
}
// ===== Shield_Debris_067d6e  ($067D6E, 164 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067D6E: bset.b #$4, $6b(a6)
pub fn i_067d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D74;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $067D74: move.w #$d000, d0
pub fn i_067d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D78;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067D78: jsr $28134.l
pub fn i_067d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67D7E);
    cpu.pc = 0x28134;
    16
}
/// $067D7E: andi.w #$ffe3, $38(a6)
pub fn i_067d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D84;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $067D84: ori.w #$14, $38(a6)
pub fn i_067d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D8A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $067D8A: lea.l $2bf7a0.l, a0
pub fn i_067d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D90;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF7A0;
    20
}
/// $067D90: jsr $799de.l
pub fn i_067d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67D96);
    cpu.pc = 0x799DE;
    16
}
/// $067D96: btst.b #$0, $3a(a6)
pub fn i_067d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67D9C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $067D9C: bne.w $67da2
pub fn i_067d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DA0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x67DA2; return 10; }
    8
}
/// $067DA0: neg.w d0
pub fn i_067da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DA2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $067DA2: move.w d0, $28(a6)
pub fn i_067da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DA6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067DA6: move.w #$157, d1
pub fn i_067da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DAA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x157;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067DAA: jsr $236e.l
pub fn i_067daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67DB0);
    cpu.pc = 0x236E;
    16
}
/// $067DB0: move.w #$158, d1
pub fn i_067db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DB4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x158;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067DB4: jsr $236e.l
pub fn i_067db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67DBA);
    cpu.pc = 0x236E;
    16
}
/// $067DBA: move.w #$159, d1
pub fn i_067dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DBE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x159;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067DBE: jsr $236e.l
pub fn i_067dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67DC4);
    cpu.pc = 0x236E;
    16
}
/// $067DC4: lea.l $2c83f2.l, a0
pub fn i_067dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DCA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C83F2;
    20
}
/// $067DCA: jsr $28cd4.l
pub fn i_067dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67DD0);
    cpu.pc = 0x28CD4;
    16
}
/// $067DD0: lea.l $67dd6(pc), a1
pub fn i_067dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67DD6;
    16
}
/// $067DD4: move.l a1, (a6)
pub fn i_067dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DD6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067DD6: jsr $27cee.l
pub fn i_067dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67DDC);
    cpu.pc = 0x27CEE;
    16
}
/// $067DDC: bcc.w $67de6
pub fn i_067ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67DE6; return 10; }
    8
}
/// $067DE0: lea.l $67e7a(pc), a1
pub fn i_067de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67E7A;
    16
}
/// $067DE4: move.l a1, (a6)
pub fn i_067de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067DE6: jsr $28d70.l
pub fn i_067de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67DEC);
    cpu.pc = 0x28D70;
    16
}
/// $067DEC: jsr $283d8.l
pub fn i_067dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67DF2);
    cpu.pc = 0x283D8;
    16
}
/// $067DF2: btst.b #$1, $13(a6)
pub fn i_067df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DF8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $067DF8: beq.w $67e02
pub fn i_067df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67DFC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x67E02; return 10; }
    8
}
/// $067DFC: lea.l $67e7a(pc), a1
pub fn i_067dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67E7A;
    16
}
/// $067E00: move.l a1, (a6)
pub fn i_067e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067E02: movea.l #$ffffffff, a0
pub fn i_067e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E08;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $067E08: jsr $5dd56.l
pub fn i_067e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67E0E);
    cpu.pc = 0x5DD56;
    16
}
/// $067E0E: bcc.w $67e18
pub fn i_067e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67E18; return 10; }
    8
}
// ===== SetTaskHandler_067e12  ($067E12, 8 B, kernel/task_handlers.c)
/// $067E12: lea.l $67b84(pc), a1
pub fn i_067e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B84;
    16
}
/// $067E16: move.l a1, (a6)
pub fn i_067e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067E18: rts
pub fn i_067e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Shield_Smoke_067e1a  ($067E1A, 88 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067E1A: move.w #$2000, d0
pub fn i_067e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E1E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067E1E: jsr $28134.l
pub fn i_067e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67E24);
    cpu.pc = 0x28134;
    16
}
/// $067E24: andi.w #$ffe3, $38(a6)
pub fn i_067e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E2A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $067E2A: ori.w #$14, $38(a6)
pub fn i_067e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E30;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $067E30: move.w #$17a, d1
pub fn i_067e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E34;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067E34: jsr $236e.l
pub fn i_067e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67E3A);
    cpu.pc = 0x236E;
    16
}
/// $067E3A: lea.l $2c8428.l, a0
pub fn i_067e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E40;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8428;
    20
}
/// $067E40: jsr $28cd4.l
pub fn i_067e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67E46);
    cpu.pc = 0x28CD4;
    16
}
/// $067E46: lea.l $67e4c(pc), a1
pub fn i_067e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67E4C;
    16
}
/// $067E4A: move.l a1, (a6)
pub fn i_067e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067E4C: jsr $2783a.l
pub fn i_067e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67E52);
    cpu.pc = 0x2783A;
    16
}
/// $067E52: jsr $28d70.l
pub fn i_067e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67E58);
    cpu.pc = 0x28D70;
    16
}
/// $067E58: bcc.w $67e62
pub fn i_067e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67E62; return 10; }
    8
}
/// $067E5C: lea.l $67b84(pc), a1
pub fn i_067e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B84;
    16
}
/// $067E60: move.l a1, (a6)
pub fn i_067e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067E62: movea.l #$ffffffff, a0
pub fn i_067e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E68;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $067E68: jsr $5dd56.l
pub fn i_067e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67E6E);
    cpu.pc = 0x5DD56;
    16
}
/// $067E6E: bcc.w $67e78
pub fn i_067e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67E78; return 10; }
    8
}
// ===== SetTaskHandler_067e72  ($067E72, 8 B, kernel/task_handlers.c)
/// $067E72: lea.l $67b84(pc), a1
pub fn i_067e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B84;
    16
}
/// $067E76: move.l a1, (a6)
pub fn i_067e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $067E78: rts
pub fn i_067e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Shield_Free_067e7a  ($067E7A, 4 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067E7A: bra.w $67b84
pub fn i_067e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E7E;
    cpu.ir = 0x6000;
    cpu.pc = 0x67B84;
    10
}
// ===== ShieldSoldier_Setup_067e7e  ($067E7E, 194 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067E7E: clr.b $84(a6)
pub fn i_067e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E82;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $067E82: cmpi.w #$100, $22(a6)
pub fn i_067e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E88;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $067E88: bgt.w $67e92
pub fn i_067e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E8C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x67E92; return 10; }
    8
}
/// $067E8C: eori.b #$1, $3a(a6)
pub fn i_067e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E92;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $067E92: jsr $5e7c0.l
pub fn i_067e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67E98);
    cpu.pc = 0x5E7C0;
    16
}
/// $067E98: cmpi.b #$2, $98(a6)
pub fn i_067e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67E9E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $067E9E: bne.w $67eb0
pub fn i_067e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EA2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x67EB0; return 10; }
    8
}
/// $067EA2: jsr $5e9b6.l
pub fn i_067ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67EA8);
    cpu.pc = 0x5E9B6;
    16
}
/// $067EA8: andi.b #$1, d0
pub fn i_067ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EAC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $067EAC: move.b d0, $98(a6)
pub fn i_067eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EB0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $067EB0: tst.b $9a(a6)
pub fn i_067eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EB4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $067EB4: bne.w $67ed2
pub fn i_067eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EB8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x67ED2; return 10; }
    8
}
/// $067EB8: move.w #$38, d1
pub fn i_067eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EBC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067EBC: jsr $236e.l
pub fn i_067ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67EC2);
    cpu.pc = 0x236E;
    16
}
/// $067EC2: move.w #$1a, $1c(a6)
pub fn i_067ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067EC8: jsr $138fe.l
pub fn i_067ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67ECE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67ECE);
    cpu.pc = 0x138FE;
    16
}
/// $067ECE: bra.w $67ee8
pub fn i_067ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67ED2;
    cpu.ir = 0x6000;
    cpu.pc = 0x67EE8;
    10
}
/// $067ED2: move.w #$1c7, d1
pub fn i_067ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67ED6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067ED6: jsr $236e.l
pub fn i_067ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67EDC);
    cpu.pc = 0x236E;
    16
}
/// $067EDC: move.w #$1f, $1c(a6)
pub fn i_067edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EE2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067EE2: jsr $138fe.l
pub fn i_067ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67EE8);
    cpu.pc = 0x138FE;
    16
}
/// $067EE8: move.w #$8000, d0
pub fn i_067ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EEC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067EEC: jsr $28134.l
pub fn i_067eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67EF2);
    cpu.pc = 0x28134;
    16
}
/// $067EF2: andi.w #$ffe3, $38(a6)
pub fn i_067ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EF8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $067EF8: ori.w #$18, $38(a6)
pub fn i_067ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67EFE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $067EFE: clr.b $81(a6)
pub fn i_067efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F02;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x81);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $067F02: move.w #$1, $66(a6)
pub fn i_067f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F08;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067F08: move.l #$2c79e2, $48(a6)
pub fn i_067f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F10;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C79E2;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $067F10: move.l #$2c7c82, $60(a6)
pub fn i_067f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F18;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C7C82;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $067F18: move.l #$2c7d74, $94(a6)
pub fn i_067f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F20;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C7D74;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $067F20: lea.l $67b92(pc), a1
pub fn i_067f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67B92;
    16
}
/// $067F24: jsr $4ae.l
pub fn i_067f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67F2A);
    cpu.pc = 0x4AE;
    16
}
/// $067F2A: jsr $5dd02.l
pub fn i_067f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67F30);
    cpu.pc = 0x5DD02;
    16
}
/// $067F30: move.l a0, $90(a6)
pub fn i_067f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F34;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $067F34: lea.l $2bf8a4.l, a0
pub fn i_067f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF8A4;
    20
}
/// $067F3A: jsr $799de.l
pub fn i_067f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67F40);
    cpu.pc = 0x799DE;
    16
}
// ===== SetTaskW_067f40  ($067F40, 6 B, kernel/task_setters_w.c)
/// $067F40: move.w d0, $72(a6)
pub fn i_067f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F44;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067F44: rts
pub fn i_067f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_ScrollProbe_067f46  ($067F46, 22 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067F46: jsr $2783a.l
pub fn i_067f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67F4C);
    cpu.pc = 0x2783A;
    16
}
/// $067F4C: jsr $27eba.l
pub fn i_067f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67F52);
    cpu.pc = 0x27EBA;
    16
}
/// $067F52: bcc.w $67f5c
pub fn i_067f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67F5C; return 10; }
    8
}
/// $067F56: jsr $27c8c.l
pub fn i_067f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67F5C);
    cpu.pc = 0x27C8C;
    16
}
// ===== ClearXN_067f5c  ($067F5C, 6 B, thunks/ccr_helpers.c)
/// $067F5C: andi.b #$ee, ccr
pub fn i_067f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F60;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $067F60: rts
pub fn i_067f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_ScrollProbeB_067f62  ($067F62, 16 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067F62: jsr $2783a.l
pub fn i_067f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67F68);
    cpu.pc = 0x2783A;
    16
}
/// $067F68: jsr $27eba.l
pub fn i_067f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67F6E);
    cpu.pc = 0x27EBA;
    16
}
/// $067F6E: bcc.w $67f7a
pub fn i_067f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x67F7A; return 10; }
    8
}
// ===== JsrAbsThunk_067f72  ($067F72, 8 B, thunks/jsr_abs_thunks.c)
/// $067F72: jsr $27c8c.l
pub fn i_067f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67F78);
    cpu.pc = 0x27C8C;
    16
}
/// $067F78: rts
pub fn i_067f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_ProbeRevert_067f7a  ($067F7A, 6 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067F7A: jsr $27a92.l
pub fn i_067f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67F80);
    cpu.pc = 0x27A92;
    16
}
// ===== ClearXN_067f80  ($067F80, 6 B, thunks/ccr_helpers.c)
/// $067F80: andi.b #$ee, ccr
pub fn i_067f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F84;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $067F84: rts
pub fn i_067f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_RngRunSpeed_067f86  ($067F86, 50 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067F86: lea.l $2bf61c.l, a0
pub fn i_067f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF61C;
    20
}
/// $067F8C: jsr $799de.l
pub fn i_067f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x67F92);
    cpu.pc = 0x799DE;
    16
}
/// $067F92: move.w d0, d1
pub fn i_067f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F94;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $067F94: btst.b #$0, $3a(a6)
pub fn i_067f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F9A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $067F9A: bne.w $67fa0
pub fn i_067f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67F9E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x67FA0; return 10; }
    8
}
/// $067F9E: neg.w d0
pub fn i_067f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FA0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $067FA0: move.w d0, $28(a6)
pub fn i_067fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FA4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $067FA4: lsr.w #$7, d1
pub fn i_067fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FA6;
    cpu.ir = 0xEE49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $067FA6: andi.w #$c, d1
pub fn i_067fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FAA;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $067FAA: lea.l $2c7d64.l, a0
pub fn i_067faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7D64;
    20
}
/// $067FB0: move.l (a0, d1.w), $94(a6)
pub fn i_067fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FB6;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $067FB6: rts
pub fn i_067fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_TakeShieldHit_067fb8  ($067FB8, 40 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067FB8: tst.b $81(a6)
pub fn i_067fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FBC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $067FBC: bne.w $67fe6
pub fn i_067fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FC0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x67FE6; return 10; }
    8
}
/// $067FC0: movea.l $90(a6), a0
pub fn i_067fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FC4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $067FC4: btst.b #$0, $5a(a0)
pub fn i_067fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FCA;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $067FCA: beq.w $67fe6
pub fn i_067fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FCE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x67FE6; return 10; }
    8
}
/// $067FCE: bset.b #$3, $13(a6)
pub fn i_067fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FD4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $067FD4: bset.b #$0, $5a(a6)
pub fn i_067fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FDA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $067FDA: move.b $58(a0), $58(a6)
pub fn i_067fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FE0;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x58);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== SetXN_067fe0  ($067FE0, 6 B, thunks/ccr_helpers.c)
/// $067FE0: ori.b #$11, ccr
pub fn i_067fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FE4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $067FE4: rts
pub fn i_067fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_067fe6  ($067FE6, 6 B, thunks/ccr_helpers.c)
/// $067FE6: andi.b #$ee, ccr
pub fn i_067fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FEA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $067FEA: rts
pub fn i_067fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_AtScreenEdge_067fec  ($067FEC, 40 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $067FEC: cmpi.w #$20, $22(a6)
pub fn i_067fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FF2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $067FF2: bgt.w $68000
pub fn i_067ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FF6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x68000; return 10; }
    8
}
/// $067FF6: btst.b #$0, $3a(a6)
pub fn i_067ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x67FFC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $067FFC: bne.w $6801a
pub fn i_067ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68000;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6801A; return 10; }
    8
}
/// $068000: cmpi.w #$120, $22(a6)
pub fn i_068000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68006;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $068006: blt.w $68014
pub fn i_068006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6800A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x68014; return 10; }
    8
}
/// $06800A: btst.b #$0, $3a(a6)
pub fn i_06800a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68010;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $068010: beq.w $6801a
pub fn i_068010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68014;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6801A; return 10; }
    8
}
// ===== ClearXN_068014  ($068014, 6 B, thunks/ccr_helpers.c)
/// $068014: andi.b #$ee, ccr
pub fn i_068014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68018;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $068018: rts
pub fn i_068018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6801A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06801a  ($06801A, 6 B, thunks/ccr_helpers.c)
/// $06801A: ori.b #$11, ccr
pub fn i_06801a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6801E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06801E: rts
pub fn i_06801e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68020;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_CanAttackA_068020  ($068020, 68 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068020: cmpi.w #$128, $22(a6)
pub fn i_068020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68026;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x128, Size::Word, false);
    16
}
/// $068026: bgt.w $6806a
pub fn i_068026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6802A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6806A; return 10; }
    8
}
/// $06802A: cmpi.w #$18, $22(a6)
pub fn i_06802a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68030;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x18, Size::Word, false);
    16
}
/// $068030: blt.w $6806a
pub fn i_068030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68034;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6806A; return 10; }
    8
}
/// $068034: cmpi.b #$0, $98(a6)
pub fn i_068034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6803A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06803A: bne.w $6806a
pub fn i_06803a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6803E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6806A; return 10; }
    8
}
/// $06803E: cmpi.w #$0, $72(a6)
pub fn i_06803e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68044;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $068044: bgt.w $6806a
pub fn i_068044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68048;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6806A; return 10; }
    8
}
/// $068048: lea.l $2c7cce.l, a0
pub fn i_068048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6804E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7CCE;
    20
}
/// $06804E: jsr $5e086.l
pub fn i_06804e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68054;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68054);
    cpu.pc = 0x5E086;
    16
}
/// $068054: bcs.w $6806a
pub fn i_068054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68058;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6806A; return 10; }
    8
}
/// $068058: jsr $6824c(pc)
pub fn i_068058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6805C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6805C);
    cpu.pc = 0x6824C;
    16
}
/// $06805C: cmp.w $74(a6), d0
pub fn i_06805c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68060;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $068060: bgt.w $6806a
pub fn i_068060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68064;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6806A; return 10; }
    8
}
// ===== SetXN_068064  ($068064, 6 B, thunks/ccr_helpers.c)
/// $068064: ori.b #$11, ccr
pub fn i_068064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68068;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $068068: rts
pub fn i_068068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6806A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06806a  ($06806A, 6 B, thunks/ccr_helpers.c)
/// $06806A: andi.b #$ee, ccr
pub fn i_06806a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6806E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06806E: rts
pub fn i_06806e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68070;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_CanAttackB_068070  ($068070, 46 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068070: cmpi.w #$128, $22(a6)
pub fn i_068070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68076;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x128, Size::Word, false);
    16
}
/// $068076: bgt.b $6806a
pub fn i_068076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68078;
    cpu.ir = 0x6EF2;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6806A; return 10; }
    8
}
/// $068078: cmpi.w #$18, $22(a6)
pub fn i_068078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6807E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x18, Size::Word, false);
    16
}
/// $06807E: blt.b $6806a
pub fn i_06807e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68080;
    cpu.ir = 0x6DEA;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6806A; return 10; }
    8
}
/// $068080: cmpi.b #$0, $98(a6)
pub fn i_068080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68086;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $068086: bne.b $6806a
pub fn i_068086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68088;
    cpu.ir = 0x66E2;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6806A; return 10; }
    8
}
/// $068088: cmpi.w #$0, $72(a6)
pub fn i_068088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6808E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06808E: bgt.b $6806a
pub fn i_06808e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68090;
    cpu.ir = 0x6EDA;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6806A; return 10; }
    8
}
/// $068090: lea.l $2c7cce.l, a0
pub fn i_068090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68096;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7CCE;
    20
}
/// $068096: jsr $5e086.l
pub fn i_068096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6809C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6809C);
    cpu.pc = 0x5E086;
    16
}
/// $06809C: bcs.b $6806a
pub fn i_06809c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6809E;
    cpu.ir = 0x65CC;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6806A; return 10; }
    8
}
// ===== SetXN_06809e  ($06809E, 6 B, thunks/ccr_helpers.c)
/// $06809E: ori.b #$11, ccr
pub fn i_06809e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680A2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0680A2: rts
pub fn i_0680a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_CanAttackC_0680a4  ($0680A4, 56 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0680A4: cmpi.w #$128, $22(a6)
pub fn i_0680a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680AA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x128, Size::Word, false);
    16
}
/// $0680AA: bgt.w $680e2
pub fn i_0680aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680AE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x680E2; return 10; }
    8
}
/// $0680AE: cmpi.w #$18, $22(a6)
pub fn i_0680ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680B4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x18, Size::Word, false);
    16
}
/// $0680B4: blt.w $680e2
pub fn i_0680b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680B8;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x680E2; return 10; }
    8
}
/// $0680B8: cmpi.b #$1, $98(a6)
pub fn i_0680b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680BE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0680BE: bne.w $680e2
pub fn i_0680be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680C2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x680E2; return 10; }
    8
}
/// $0680C2: cmpi.w #$0, $72(a6)
pub fn i_0680c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680C8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0680C8: bgt.w $680e2
pub fn i_0680c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680CC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x680E2; return 10; }
    8
}
/// $0680CC: lea.l $2c7cd6.l, a0
pub fn i_0680cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7CD6;
    20
}
/// $0680D2: jsr $5e086.l
pub fn i_0680d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x680D8);
    cpu.pc = 0x5E086;
    16
}
/// $0680D8: bcs.w $680e2
pub fn i_0680d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680DC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x680E2; return 10; }
    8
}
// ===== SetXN_0680dc  ($0680DC, 6 B, thunks/ccr_helpers.c)
/// $0680DC: ori.b #$11, ccr
pub fn i_0680dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680E0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0680E0: rts
pub fn i_0680e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0680e2  ($0680E2, 6 B, thunks/ccr_helpers.c)
/// $0680E2: andi.b #$ee, ccr
pub fn i_0680e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680E6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0680E6: rts
pub fn i_0680e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_PlayerFar_0680e8  ($0680E8, 22 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0680E8: jsr $5e0d4.l
pub fn i_0680e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x680EE);
    cpu.pc = 0x5E0D4;
    16
}
/// $0680EE: bcs.w $68104
pub fn i_0680ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680F2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68104; return 10; }
    8
}
/// $0680F2: jsr $6824c(pc)
pub fn i_0680f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x680F6);
    cpu.pc = 0x6824C;
    16
}
/// $0680F6: cmp.w $74(a6), d0
pub fn i_0680f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680FA;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0680FA: ble.w $68104
pub fn i_0680fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x680FE;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x68104; return 10; }
    8
}
// ===== ClearXN_0680fe  ($0680FE, 6 B, thunks/ccr_helpers.c)
/// $0680FE: andi.b #$ee, ccr
pub fn i_0680fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68102;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $068102: rts
pub fn i_068102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68104;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_068104  ($068104, 6 B, thunks/ccr_helpers.c)
/// $068104: ori.b #$11, ccr
pub fn i_068104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68108;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $068108: rts
pub fn i_068108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6810A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_PlayerNear_06810a  ($06810A, 26 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06810A: jsr $5e0d4.l
pub fn i_06810a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68110;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68110);
    cpu.pc = 0x5E0D4;
    16
}
/// $068110: bcs.w $68124
pub fn i_068110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68114;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68124; return 10; }
    8
}
/// $068114: jsr $6824c(pc)
pub fn i_068114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68118;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68118);
    cpu.pc = 0x6824C;
    16
}
/// $068118: subi.w #$10, d0
pub fn i_068118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6811C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06811C: cmp.w $74(a6), d0
pub fn i_06811c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68120;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $068120: bgt.w $6812a
pub fn i_068120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68124;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6812A; return 10; }
    8
}
// ===== ClearXN_068124  ($068124, 6 B, thunks/ccr_helpers.c)
/// $068124: andi.b #$ee, ccr
pub fn i_068124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68128;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $068128: rts
pub fn i_068128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6812A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06812a  ($06812A, 6 B, thunks/ccr_helpers.c)
/// $06812A: ori.b #$11, ccr
pub fn i_06812a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6812E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06812E: rts
pub fn i_06812e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68130;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_PlayerFarOrEdge_068130  ($068130, 30 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068130: jsr $5e0d4.l
pub fn i_068130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68136;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68136);
    cpu.pc = 0x5E0D4;
    16
}
/// $068136: bcs.w $68154
pub fn i_068136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6813A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68154; return 10; }
    8
}
/// $06813A: jsr $6824c(pc)
pub fn i_06813a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6813E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6813E);
    cpu.pc = 0x6824C;
    16
}
/// $06813E: cmp.w $74(a6), d0
pub fn i_06813e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68142;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $068142: bgt.w $68154
pub fn i_068142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68146;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x68154; return 10; }
    8
}
/// $068146: jsr $67fec(pc)
pub fn i_068146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6814A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6814A);
    cpu.pc = 0x67FEC;
    16
}
/// $06814A: bcs.w $68154
pub fn i_06814a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6814E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68154; return 10; }
    8
}
// ===== ClearXN_06814e  ($06814E, 6 B, thunks/ccr_helpers.c)
/// $06814E: andi.b #$ee, ccr
pub fn i_06814e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68152;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $068152: rts
pub fn i_068152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68154;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_068154  ($068154, 6 B, thunks/ccr_helpers.c)
/// $068154: ori.b #$11, ccr
pub fn i_068154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68158;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $068158: rts
pub fn i_068158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6815A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_PlayerNearEdge_06815a  ($06815A, 32 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06815A: jsr $5e0d4.l
pub fn i_06815a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68160;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68160);
    cpu.pc = 0x5E0D4;
    16
}
/// $068160: bcs.w $68180
pub fn i_068160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68164;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68180; return 10; }
    8
}
/// $068164: jsr $67fec(pc)
pub fn i_068164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68168;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68168);
    cpu.pc = 0x67FEC;
    16
}
/// $068168: bcs.w $68180
pub fn i_068168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6816C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68180; return 10; }
    8
}
/// $06816C: jsr $6824c(pc)
pub fn i_06816c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68170;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68170);
    cpu.pc = 0x6824C;
    16
}
/// $068170: addq.w #$8, d0
pub fn i_068170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68172;
    cpu.ir = 0x5040;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $068172: cmp.w $74(a6), d0
pub fn i_068172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68176;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $068176: bgt.w $68180
pub fn i_068176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6817A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x68180; return 10; }
    8
}
// ===== SetXN_06817a  ($06817A, 6 B, thunks/ccr_helpers.c)
/// $06817A: ori.b #$11, ccr
pub fn i_06817a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6817E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06817E: rts
pub fn i_06817e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68180;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_068180  ($068180, 6 B, thunks/ccr_helpers.c)
/// $068180: andi.b #$ee, ccr
pub fn i_068180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68184;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $068184: rts
pub fn i_068184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68186;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_GuardTick_068186  ($068186, 52 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068186: addq.w #$1, $76(a6)
pub fn i_068186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6818A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06818A: cmpi.w #$f, $76(a6)
pub fn i_06818a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68190;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF, Size::Word, false);
    16
}
/// $068190: blt.w $681c0
pub fn i_068190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68194;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x681C0; return 10; }
    8
}
/// $068194: move.w #$f, $76(a6)
pub fn i_068194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6819A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06819A: lea.l $2c7cee.l, a0
pub fn i_06819a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7CEE;
    20
}
/// $0681A0: jsr $5e086.l
pub fn i_0681a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x681A6);
    cpu.pc = 0x5E086;
    16
}
/// $0681A6: bcc.w $681c0
pub fn i_0681a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681AA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x681C0; return 10; }
    8
}
/// $0681AA: lea.l $2c7cf6.l, a0
pub fn i_0681aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7CF6;
    20
}
/// $0681B0: jsr $5e086.l
pub fn i_0681b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x681B6);
    cpu.pc = 0x5E086;
    16
}
/// $0681B6: bcs.w $681c0
pub fn i_0681b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681BA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x681C0; return 10; }
    8
}
// ===== SetXN_0681ba  ($0681BA, 6 B, thunks/ccr_helpers.c)
/// $0681BA: ori.b #$11, ccr
pub fn i_0681ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681BE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0681BE: rts
pub fn i_0681be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0681c0  ($0681C0, 6 B, thunks/ccr_helpers.c)
/// $0681C0: andi.b #$ee, ccr
pub fn i_0681c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681C4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0681C4: rts
pub fn i_0681c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_FallCheck_0681c6  ($0681C6, 24 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0681C6: jsr $27eba.l
pub fn i_0681c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x681CC);
    cpu.pc = 0x27EBA;
    16
}
/// $0681CC: bcc.w $681e4
pub fn i_0681cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x681E4; return 10; }
    8
}
/// $0681D0: addq.b #$1, $83(a6)
pub fn i_0681d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681D4;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x83);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0681D4: cmpi.b #$3, $83(a6)
pub fn i_0681d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681DA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $0681DA: blt.w $681e4
pub fn i_0681da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681DE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x681E4; return 10; }
    8
}
// ===== SetXN_0681de  ($0681DE, 6 B, thunks/ccr_helpers.c)
/// $0681DE: ori.b #$11, ccr
pub fn i_0681de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681E2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0681E2: rts
pub fn i_0681e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_FallReset_0681e4  ($0681E4, 4 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0681E4: clr.b $83(a6)
pub fn i_0681e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681E8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x83);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== ClearXN_0681e8  ($0681E8, 6 B, thunks/ccr_helpers.c)
/// $0681E8: andi.b #$ee, ccr
pub fn i_0681e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681EC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0681EC: rts
pub fn i_0681ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_ShieldAlive_0681ee  ($0681EE, 16 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0681EE: movea.l $90(a6), a0
pub fn i_0681ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681F2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0681F2: cmpi.l #$ffffffff, $48(a0)
pub fn i_0681f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681FA;
    cpu.ir = 0xCA8;
    let ad2 = cpu.a[0].wrapping_add(0x48);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $0681FA: beq.w $68204
pub fn i_0681fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x681FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x68204; return 10; }
    8
}
// ===== JsrAbsThunk_0681fe  ($0681FE, 8 B, thunks/jsr_abs_thunks.c)
/// $0681FE: jsr $28998.l
pub fn i_0681fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68204);
    cpu.pc = 0x28998;
    16
}
/// $068204: rts
pub fn i_068204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68206;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Shield_Break_068206  ($068206, 70 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068206: move.w #$10ff, d0
pub fn i_068206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6820A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06820A: jsr $2352.l
pub fn i_06820a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68210;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68210);
    cpu.pc = 0x2352;
    16
}
/// $068210: lea.l $67e1a(pc), a1
pub fn i_068210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68214;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67E1A;
    16
}
/// $068214: jsr $4ae.l
pub fn i_068214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6821A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6821A);
    cpu.pc = 0x4AE;
    16
}
/// $06821A: jsr $5dd02.l
pub fn i_06821a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68220;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68220);
    cpu.pc = 0x5DD02;
    16
}
/// $068220: lea.l $67d6e(pc), a1
pub fn i_068220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68224;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x67D6E;
    16
}
/// $068224: jsr $4ae.l
pub fn i_068224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6822A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6822A);
    cpu.pc = 0x4AE;
    16
}
/// $06822A: jsr $5dd02.l
pub fn i_06822a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68230;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68230);
    cpu.pc = 0x5DD02;
    16
}
/// $068230: addi.w #$1c, $24(a0)
pub fn i_068230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68236;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $068236: move.w #$ffe2, d0
pub fn i_068236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6823A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFE2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06823A: btst.b #$0, $3a(a6)
pub fn i_06823a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68240;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $068240: beq.w $68246
pub fn i_068240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68244;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x68246; return 10; }
    8
}
/// $068244: neg.w d0
pub fn i_068244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68246;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $068246: add.w d0, $22(a0)
pub fn i_068246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6824A;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06824A: rts
pub fn i_06824a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6824C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_AbsDX_06824c  ($06824C, 20 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06824C: move.w $22(a6), d0
pub fn i_06824c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68250;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $068250: sub.w $22(a0), d0
pub fn i_068250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68254;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $068254: cmpi.w #$0, d0
pub fn i_068254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68258;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $068258: bge.w $6825e
pub fn i_068258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6825C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x6825E; return 10; }
    8
}
/// $06825C: neg.w d0
pub fn i_06825c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6825E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06825E: rts
pub fn i_06825e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68260;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_DropItem_068260  ($068260, 38 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068260: tst.b $84(a6)
pub fn i_068260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68264;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $068264: bne.w $68284
pub fn i_068264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68268;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68284; return 10; }
    8
}
/// $068268: move.b $9a(a6), d0
pub fn i_068268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6826C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06826C: move.w #$9b, d1
pub fn i_06826c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68270;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068270: jsr $9a7aa.l
pub fn i_068270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68276;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68276);
    cpu.pc = 0x9A7AA;
    16
}
/// $068276: bcs.w $68284
pub fn i_068276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6827A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68284; return 10; }
    8
}
/// $06827A: addq.w #$4, $24(a0)
pub fn i_06827a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6827E;
    cpu.ir = 0x5868;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06827E: move.b #$1, $84(a6)
pub fn i_06827e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68284;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $068284: rts
pub fn i_068284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68286;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_DropItemB_068286  ($068286, 46 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068286: tst.b $84(a6)
pub fn i_068286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6828A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06828A: bne.w $682b2
pub fn i_06828a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6828E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x682B2; return 10; }
    8
}
/// $06828E: move.b $9a(a6), d0
pub fn i_06828e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68292;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $068292: move.w #$9b, d1
pub fn i_068292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68296;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068296: jsr $9a7aa.l
pub fn i_068296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6829C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6829C);
    cpu.pc = 0x9A7AA;
    16
}
/// $06829C: bcs.w $682b2
pub fn i_06829c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682A0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x682B2; return 10; }
    8
}
/// $0682A0: addi.w #$10, $24(a0)
pub fn i_0682a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682A6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0682A6: move.w #$800, $2a(a0)
pub fn i_0682a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682AC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0682AC: move.b #$1, $84(a6)
pub fn i_0682ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682B2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0682B2: rts
pub fn i_0682b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShieldSoldier_HitCheck_0682b4  ($0682B4, 16 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0682B4: movea.l $8(a6), a1
pub fn i_0682b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682B8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0682B8: move.b $10(a6), d0
pub fn i_0682b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682BC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0682BC: cmp.b $10(a1), d0
pub fn i_0682bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682C0;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0682C0: bcs.w $682ca
pub fn i_0682c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682C4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x682CA; return 10; }
    8
}
// ===== ClearXN_0682c4  ($0682C4, 6 B, thunks/ccr_helpers.c)
/// $0682C4: andi.b #$ee, ccr
pub fn i_0682c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682C8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0682C8: rts
pub fn i_0682c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0682ca  ($0682CA, 6 B, thunks/ccr_helpers.c)
/// $0682CA: ori.b #$11, ccr
pub fn i_0682ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682CE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0682CE: rts
pub fn i_0682ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Gate_Init_0682d0  ($0682D0, 56 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0682D0: lea.l $2ba596.l, a0
pub fn i_0682d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA596;
    20
}
/// $0682D6: jsr $799de.l
pub fn i_0682d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x682DC);
    cpu.pc = 0x799DE;
    16
}
/// $0682DC: move.w d0, $76(a6)
pub fn i_0682dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682E0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0682E0: move.w #$ec0, $72(a6)
pub fn i_0682e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xEC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0682E6: lea.l $682ec(pc), a1
pub fn i_0682e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x682EC;
    16
}
/// $0682EA: move.l a1, (a6)
pub fn i_0682ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0682EC: cmpi.w #$e20, $106f50.l
pub fn i_0682ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682F4;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0xE20, Size::Word, false);
    20
}
/// $0682F4: blt.w $68300
pub fn i_0682f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x682F8;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x68300; return 10; }
    8
}
/// $0682F8: move.b #$4, $10e39c.l
pub fn i_0682f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68300;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39C, v1 as u8);
    16
}
/// $068300: jsr $689e6(pc)
pub fn i_068300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68304;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68304);
    cpu.pc = 0x689E6;
    16
}
/// $068304: bcc.w $6830e
pub fn i_068304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68308;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6830E; return 10; }
    8
}
// ===== SetTaskHandler_068308  ($068308, 8 B, kernel/task_handlers.c)
/// $068308: lea.l $68310(pc), a1
pub fn i_068308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6830C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68310;
    16
}
/// $06830C: move.l a1, (a6)
pub fn i_06830c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6830E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06830E: rts
pub fn i_06830e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68310;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Gate_WaitScroll_068310  ($068310, 46 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068310: move.b #$4, $10e39c.l
pub fn i_068310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68318;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39C, v1 as u8);
    16
}
/// $068318: move.w #$1000, $72(a6)
pub fn i_068318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6831E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06831E: lea.l $68324(pc), a1
pub fn i_06831e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68322;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68324;
    16
}
/// $068322: move.l a1, (a6)
pub fn i_068322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68324;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068324: cmpi.w #$1000, $106f50.l
pub fn i_068324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6832C;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F50) as u32;
    sub_with_flags(cpu, v2, 0x1000, Size::Word, false);
    20
}
/// $06832C: blt.w $68336
pub fn i_06832c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68330;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x68336; return 10; }
    8
}
/// $068330: lea.l $68346(pc), a1
pub fn i_068330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68334;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68346;
    16
}
/// $068334: move.l a1, (a6)
pub fn i_068334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68336;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068336: jsr $6896a(pc)
pub fn i_068336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6833A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6833A);
    cpu.pc = 0x6896A;
    16
}
/// $06833A: bcc.w $68344
pub fn i_06833a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6833E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x68344; return 10; }
    8
}
// ===== SetTaskHandler_06833e  ($06833E, 8 B, kernel/task_handlers.c)
/// $06833E: lea.l $68346(pc), a1
pub fn i_06833e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68342;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68346;
    16
}
/// $068342: move.l a1, (a6)
pub fn i_068342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68344;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068344: rts
pub fn i_068344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68346;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Gate_Begin_068346  ($068346, 82 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068346: move.w #$1032, d0
pub fn i_068346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6834A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1032;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06834A: jsr $2352.l
pub fn i_06834a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68350;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68350);
    cpu.pc = 0x2352;
    16
}
/// $068350: movea.l $c(a6), a0
pub fn i_068350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68354;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $068354: move.b #$1, $21(a0)
pub fn i_068354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6835A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06835A: move.w #$1, $106f5e.l
pub fn i_06835a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68362;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F5E, v1 as u16);
    16
}
/// $068362: clr.l $106f60.l
pub fn i_068362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68368;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F60);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F60, r2);
    20
}
/// $068368: clr.l $106f64.l
pub fn i_068368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6836E;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F64);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F64, r2);
    20
}
/// $06836E: move.w #$1000, $72(a6)
pub fn i_06836e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68374;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068374: lea.l $6837a(pc), a1
pub fn i_068374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68378;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6837A;
    16
}
/// $068378: move.l a1, (a6)
pub fn i_068378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6837A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06837A: addi.l #$8000, $106f60.l
pub fn i_06837a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68384;
    cpu.ir = 0x6B9;
    let v1 = bus.read32(0x106F60);
    let r2 = add_with_flags(cpu, v1, 0x8000, Size::Long, true);
    bus.write32(0x106F60, r2);
    24
}
/// $068384: cmpi.l #$60000, $106f60.l
pub fn i_068384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6838E;
    cpu.ir = 0xCB9;
    let v2 = bus.read32(0x106F60);
    sub_with_flags(cpu, v2, 0x60000, Size::Long, false);
    24
}
/// $06838E: blt.w $68398
pub fn i_06838e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68392;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x68398; return 10; }
    8
}
/// $068392: lea.l $6839e(pc), a1
pub fn i_068392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68396;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6839E;
    16
}
/// $068396: move.l a1, (a6)
pub fn i_068396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68398;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_068398  ($068398, 6 B, thunks/jsr_pc_thunks.c)
/// $068398: jsr $6896a(pc)
pub fn i_068398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6839C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6839C);
    cpu.pc = 0x6896A;
    16
}
/// $06839C: rts
pub fn i_06839c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6839E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Gate_HoldOffset_06839e  ($06839E, 16 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06839E: move.l #$60000, $106f60.l
pub fn i_06839e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683A8;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x60000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0683A8: lea.l $683ae(pc), a1
pub fn i_0683a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x683AE;
    16
}
/// $0683AC: move.l a1, (a6)
pub fn i_0683ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_0683ae  ($0683AE, 6 B, thunks/jsr_pc_thunks.c)
/// $0683AE: jsr $6896a(pc)
pub fn i_0683ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x683B2);
    cpu.pc = 0x6896A;
    16
}
/// $0683B2: rts
pub fn i_0683b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Gate_Release_0683b4  ($0683B4, 52 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0683B4: lea.l $683ba(pc), a1
pub fn i_0683b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x683BA;
    16
}
/// $0683B8: move.l a1, (a6)
pub fn i_0683b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0683BA: subi.l #$2000, $106f60.l
pub fn i_0683ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683C4;
    cpu.ir = 0x4B9;
    let v1 = bus.read32(0x106F60);
    let r2 = sub_with_flags(cpu, v1, 0x2000, Size::Long, true);
    bus.write32(0x106F60, r2);
    24
}
/// $0683C4: cmpi.l #$0, $106f60.l
pub fn i_0683c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683CE;
    cpu.ir = 0xCB9;
    let v2 = bus.read32(0x106F60);
    sub_with_flags(cpu, v2, 0x0, Size::Long, false);
    24
}
/// $0683CE: bgt.w $683ea
pub fn i_0683ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683D2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x683EA; return 10; }
    8
}
/// $0683D2: clr.l $106f60.l
pub fn i_0683d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683D8;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F60);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F60, r2);
    20
}
/// $0683D8: clr.l $106f64.l
pub fn i_0683d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683DE;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F64);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F64, r2);
    20
}
/// $0683DE: jsr $6896a(pc)
pub fn i_0683de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x683E2);
    cpu.pc = 0x6896A;
    16
}
/// $0683E2: jmp $518.l
pub fn i_0683e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683E8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_0683e8  ($0683E8, 2 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0683E8: rts
pub fn i_0683e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_0683ea  ($0683EA, 6 B, thunks/jsr_pc_thunks.c)
/// $0683EA: jsr $6896a(pc)
pub fn i_0683ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x683EE);
    cpu.pc = 0x6896A;
    16
}
/// $0683EE: rts
pub fn i_0683ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_Init_0683f0  ($0683F0, 196 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0683F0: move.w #$f00, d0
pub fn i_0683f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683F4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0683F4: move.w #$f8, d1
pub fn i_0683f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683F8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0683F8: jsr $440d0.l
pub fn i_0683f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x683FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x683FE);
    cpu.pc = 0x440D0;
    16
}
/// $0683FE: move.w d0, $22(a6)
pub fn i_0683fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68402;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068402: move.w d1, $24(a6)
pub fn i_068402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68406;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068406: jsr $5e7c0.l
pub fn i_068406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6840C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6840C);
    cpu.pc = 0x5E7C0;
    16
}
/// $06840C: addi.w #$130, $22(a6)
pub fn i_06840c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68412;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x130, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $068412: lea.l $6875e(pc), a1
pub fn i_068412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68416;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6875E;
    16
}
/// $068416: jsr $4ae.l
pub fn i_068416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6841C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6841C);
    cpu.pc = 0x4AE;
    16
}
/// $06841C: jsr $5dd02.l
pub fn i_06841c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68422);
    cpu.pc = 0x5DD02;
    16
}
/// $068422: clr.w $80(a0)
pub fn i_068422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68426;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $068426: lea.l $684b4(pc), a1
pub fn i_068426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6842A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x684B4;
    16
}
/// $06842A: jsr $4ae.l
pub fn i_06842a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68430);
    cpu.pc = 0x4AE;
    16
}
/// $068430: jsr $5dd02.l
pub fn i_068430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68436);
    cpu.pc = 0x5DD02;
    16
}
/// $068436: clr.w $80(a0)
pub fn i_068436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6843A;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06843A: lea.l $684b4(pc), a1
pub fn i_06843a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6843E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x684B4;
    16
}
/// $06843E: jsr $4ae.l
pub fn i_06843e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68444;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68444);
    cpu.pc = 0x4AE;
    16
}
/// $068444: jsr $5dd02.l
pub fn i_068444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6844A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6844A);
    cpu.pc = 0x5DD02;
    16
}
/// $06844A: move.w #$70, $80(a0)
pub fn i_06844a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68450;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068450: lea.l $684b4(pc), a1
pub fn i_068450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68454;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x684B4;
    16
}
/// $068454: jsr $4ae.l
pub fn i_068454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6845A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6845A);
    cpu.pc = 0x4AE;
    16
}
/// $06845A: jsr $5dd02.l
pub fn i_06845a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68460;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68460);
    cpu.pc = 0x5DD02;
    16
}
/// $068460: move.w #$e0, $80(a0)
pub fn i_068460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68466;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068466: move.w #$c000, d0
pub fn i_068466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6846A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06846A: jsr $28134.l
pub fn i_06846a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68470;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68470);
    cpu.pc = 0x28134;
    16
}
/// $068470: andi.w #$ffe3, $38(a6)
pub fn i_068470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68476;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $068476: ori.w #$8, $38(a6)
pub fn i_068476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6847C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06847C: lea.l $68482(pc), a1
pub fn i_06847c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68480;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68482;
    16
}
/// $068480: move.l a1, (a6)
pub fn i_068480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68482;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068482: jsr $68a5a(pc)
pub fn i_068482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68486;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68486);
    cpu.pc = 0x68A5A;
    16
}
/// $068486: jsr $68b64(pc)
pub fn i_068486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6848A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6848A);
    cpu.pc = 0x68B64;
    16
}
/// $06848A: tst.w $106f5e.l
pub fn i_06848a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68490;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F5E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $068490: beq.w $684a2
pub fn i_068490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68494;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x684A2; return 10; }
    8
}
/// $068494: cmpi.w #$30, $22(a6)
pub fn i_068494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6849A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x30, Size::Word, false);
    16
}
/// $06849A: ble.w $684a2
pub fn i_06849a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6849E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x684A2; return 10; }
    8
}
/// $06849E: subq.w #$1, $22(a6)
pub fn i_06849e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684A2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0684A2: cmpi.w #$30, $22(a6)
pub fn i_0684a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684A8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x30, Size::Word, false);
    16
}
/// $0684A8: bgt.w $684b2
pub fn i_0684a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684AC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x684B2; return 10; }
    8
}
/// $0684AC: move.w #$30, $22(a6)
pub fn i_0684ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684B2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0684B2: rts
pub fn i_0684b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_Part_0684b4  ($0684B4, 90 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0684B4: jsr $68a70(pc)
pub fn i_0684b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x684B8);
    cpu.pc = 0x68A70;
    16
}
/// $0684B8: lea.l $686ce(pc), a1
pub fn i_0684b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x686CE;
    16
}
/// $0684BC: jsr $4ae.l
pub fn i_0684bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x684C2);
    cpu.pc = 0x4AE;
    16
}
/// $0684C2: jsr $5dd02.l
pub fn i_0684c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x684C8);
    cpu.pc = 0x5DD02;
    16
}
/// $0684C8: lea.l $6872a(pc), a1
pub fn i_0684c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6872A;
    16
}
/// $0684CC: jsr $4ae.l
pub fn i_0684cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x684D2);
    cpu.pc = 0x4AE;
    16
}
/// $0684D2: jsr $5dd02.l
pub fn i_0684d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x684D8);
    cpu.pc = 0x5DD02;
    16
}
/// $0684D8: lea.l $687b4(pc), a1
pub fn i_0684d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x687B4;
    16
}
/// $0684DC: jsr $4ae.l
pub fn i_0684dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x684E2);
    cpu.pc = 0x4AE;
    16
}
/// $0684E2: jsr $5dd02.l
pub fn i_0684e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x684E8);
    cpu.pc = 0x5DD02;
    16
}
/// $0684E8: move.w #$1b7, d1
pub fn i_0684e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684EC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0684EC: jsr $236e.l
pub fn i_0684ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x684F2);
    cpu.pc = 0x236E;
    16
}
/// $0684F2: clr.w $82(a6)
pub fn i_0684f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684F6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0684F6: clr.w $84(a6)
pub fn i_0684f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x684FA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0684FA: bset.b #$6, $12(a6)
pub fn i_0684fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68500;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $068500: move.w #$7fff, $66(a6)
pub fn i_068500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68506;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068506: move.l #$2c8466, $48(a6)
pub fn i_068506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6850E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C8466;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== S5Airship_PartIdle_06850e  ($06850E, 112 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06850E: clr.b $20(a6)
pub fn i_06850e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68512;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $068512: lea.l $2c8536.l, a0
pub fn i_068512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68518;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8536;
    20
}
/// $068518: jsr $28cd4.l
pub fn i_068518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6851E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6851E);
    cpu.pc = 0x28CD4;
    16
}
/// $06851E: lea.l $68524(pc), a1
pub fn i_06851e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68522;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68524;
    16
}
/// $068522: move.l a1, (a6)
pub fn i_068522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68524;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068524: jsr $68ad0(pc)
pub fn i_068524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68528;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68528);
    cpu.pc = 0x68AD0;
    16
}
/// $068528: jsr $68a70(pc)
pub fn i_068528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6852C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6852C);
    cpu.pc = 0x68A70;
    16
}
/// $06852C: jsr $68b46(pc)
pub fn i_06852c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68530;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68530);
    cpu.pc = 0x68B46;
    16
}
/// $068530: jsr $68ab8(pc)
pub fn i_068530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68534;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68534);
    cpu.pc = 0x68AB8;
    16
}
/// $068534: movea.l $c(a6), a0
pub fn i_068534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68538;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $068538: movea.l $c(a0), a0
pub fn i_068538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6853C;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06853C: btst.b #$0, $13(a0)
pub fn i_06853c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68542;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $068542: bne.w $68570
pub fn i_068542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68546;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68570; return 10; }
    8
}
/// $068546: jsr $68af4(pc)
pub fn i_068546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6854A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6854A);
    cpu.pc = 0x68AF4;
    16
}
/// $06854A: bcc.w $68570
pub fn i_06854a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6854E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x68570; return 10; }
    8
}
/// $06854E: move.b $106f28.l, d0
pub fn i_06854e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68554;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $068554: andi.b #$1f, d0
pub fn i_068554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68558;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $068558: bne.w $68570
pub fn i_068558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6855C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68570; return 10; }
    8
}
/// $06855C: jsr $5e9b6.l
pub fn i_06855c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68562;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68562);
    cpu.pc = 0x5E9B6;
    16
}
/// $068562: andi.w #$3, d0
pub fn i_068562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68566;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $068566: bne.w $68570
pub fn i_068566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6856A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68570; return 10; }
    8
}
/// $06856A: lea.l $6857e(pc), a1
pub fn i_06856a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6856E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6857E;
    16
}
/// $06856E: move.l a1, (a6)
pub fn i_06856e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68570;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068570: bclr.b #$3, $13(a6)
pub fn i_068570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68576;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $068576: bclr.b #$0, $13(a6)
pub fn i_068576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6857C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06857C: rts
pub fn i_06857c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6857E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_PartHold_06857e  ($06857E, 82 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06857E: move.w #$96, $70(a6)
pub fn i_06857e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68584;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $068584: lea.l $2c8542.l, a0
pub fn i_068584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6858A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8542;
    20
}
/// $06858A: jsr $28cd4.l
pub fn i_06858a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68590);
    cpu.pc = 0x28CD4;
    16
}
/// $068590: lea.l $68596(pc), a1
pub fn i_068590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68594;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68596;
    16
}
/// $068594: move.l a1, (a6)
pub fn i_068594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68596;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068596: jsr $68ad0(pc)
pub fn i_068596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6859A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6859A);
    cpu.pc = 0x68AD0;
    16
}
/// $06859A: jsr $68a70(pc)
pub fn i_06859a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6859E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6859E);
    cpu.pc = 0x68A70;
    16
}
/// $06859E: jsr $68b46(pc)
pub fn i_06859e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x685A2);
    cpu.pc = 0x68B46;
    16
}
/// $0685A2: jsr $68ab8(pc)
pub fn i_0685a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x685A6);
    cpu.pc = 0x68AB8;
    16
}
/// $0685A6: subq.w #$1, $70(a6)
pub fn i_0685a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685AA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0685AA: cmpi.w #$0, $70(a6)
pub fn i_0685aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0685B0: bgt.w $685ba
pub fn i_0685b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685B4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x685BA; return 10; }
    8
}
/// $0685B4: lea.l $685d8(pc), a1
pub fn i_0685b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x685D8;
    16
}
/// $0685B8: move.l a1, (a6)
pub fn i_0685b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0685BA: bclr.b #$3, $13(a6)
pub fn i_0685ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685C0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0685C0: bclr.b #$0, $13(a6)
pub fn i_0685c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685C6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0685C6: tst.b $106ed3.l
pub fn i_0685c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685CC;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0685CC: bne.w $685d6
pub fn i_0685cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x685D6; return 10; }
    8
}
// ===== SetTaskHandler_0685d0  ($0685D0, 8 B, kernel/task_handlers.c)
/// $0685D0: lea.l $68684(pc), a1
pub fn i_0685d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68684;
    16
}
/// $0685D4: move.l a1, (a6)
pub fn i_0685d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0685D6: rts
pub fn i_0685d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_PartState1_0685d8  ($0685D8, 66 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0685D8: move.b #$1, $20(a6)
pub fn i_0685d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685DE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0685DE: lea.l $685e4(pc), a1
pub fn i_0685de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x685E4;
    16
}
/// $0685E2: move.l a1, (a6)
pub fn i_0685e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0685E4: jsr $68ad0(pc)
pub fn i_0685e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x685E8);
    cpu.pc = 0x68AD0;
    16
}
/// $0685E8: jsr $68a70(pc)
pub fn i_0685e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x685EC);
    cpu.pc = 0x68A70;
    16
}
/// $0685EC: jsr $68b46(pc)
pub fn i_0685ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x685F0);
    cpu.pc = 0x68B46;
    16
}
/// $0685F0: jsr $68ab8(pc)
pub fn i_0685f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x685F4);
    cpu.pc = 0x68AB8;
    16
}
/// $0685F4: cmpi.b #$1, $20(a6)
pub fn i_0685f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685FA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0685FA: beq.w $68604
pub fn i_0685fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x685FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x68604; return 10; }
    8
}
/// $0685FE: lea.l $68622(pc), a1
pub fn i_0685fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68602;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68622;
    16
}
/// $068602: move.l a1, (a6)
pub fn i_068602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68604;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068604: bclr.b #$3, $13(a6)
pub fn i_068604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6860A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06860A: bclr.b #$0, $13(a6)
pub fn i_06860a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68610;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $068610: tst.b $106ed3.l
pub fn i_068610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68616;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $068616: bne.w $68620
pub fn i_068616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6861A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68620; return 10; }
    8
}
// ===== SetTaskHandler_06861a  ($06861A, 8 B, kernel/task_handlers.c)
/// $06861A: lea.l $68684(pc), a1
pub fn i_06861a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6861E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68684;
    16
}
/// $06861E: move.l a1, (a6)
pub fn i_06861e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68620;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068620: rts
pub fn i_068620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68622;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_PartSway_068622  ($068622, 90 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068622: lea.l $2c85a2.l, a0
pub fn i_068622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68628;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C85A2;
    20
}
/// $068628: jsr $28cd4.l
pub fn i_068628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6862E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6862E);
    cpu.pc = 0x28CD4;
    16
}
/// $06862E: lea.l $68634(pc), a1
pub fn i_06862e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68632;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68634;
    16
}
/// $068632: move.l a1, (a6)
pub fn i_068632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68634;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068634: jsr $68ad0(pc)
pub fn i_068634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68638;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68638);
    cpu.pc = 0x68AD0;
    16
}
/// $068638: jsr $68a70(pc)
pub fn i_068638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6863C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6863C);
    cpu.pc = 0x68A70;
    16
}
/// $06863C: jsr $68b46(pc)
pub fn i_06863c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68640;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68640);
    cpu.pc = 0x68B46;
    16
}
/// $068640: jsr $68ab8(pc)
pub fn i_068640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68644;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68644);
    cpu.pc = 0x68AB8;
    16
}
/// $068644: move.b $106f28.l, d0
pub fn i_068644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6864A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $06864A: andi.b #$1f, d0
pub fn i_06864a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6864E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06864E: bne.w $68666
pub fn i_06864e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68652;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68666; return 10; }
    8
}
/// $068652: jsr $5e9b6.l
pub fn i_068652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68658;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x68658);
    cpu.pc = 0x5E9B6;
    16
}
/// $068658: andi.w #$7, d0
pub fn i_068658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6865C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06865C: bne.w $68666
pub fn i_06865c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68660;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68666; return 10; }
    8
}
/// $068660: lea.l $68684(pc), a1
pub fn i_068660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68664;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68684;
    16
}
/// $068664: move.l a1, (a6)
pub fn i_068664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68666;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068666: bclr.b #$3, $13(a6)
pub fn i_068666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6866C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06866C: bclr.b #$0, $13(a6)
pub fn i_06866c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68672;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $068672: tst.b $106ed3.l
pub fn i_068672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68678;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $068678: bne.w $68682
pub fn i_068678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6867C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x68682; return 10; }
    8
}
// ===== SetTaskHandler_06867c  ($06867C, 8 B, kernel/task_handlers.c)
/// $06867C: lea.l $6850e(pc), a1
pub fn i_06867c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68680;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6850E;
    16
}
/// $068680: move.l a1, (a6)
pub fn i_068680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68682;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068682: rts
pub fn i_068682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68684;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_PartState3_068684  ($068684, 66 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $068684: move.b #$3, $20(a6)
pub fn i_068684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6868A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06868A: lea.l $68690(pc), a1
pub fn i_06868a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6868E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68690;
    16
}
/// $06868E: move.l a1, (a6)
pub fn i_06868e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68690;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068690: jsr $68ad0(pc)
pub fn i_068690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68694;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68694);
    cpu.pc = 0x68AD0;
    16
}
/// $068694: jsr $68a70(pc)
pub fn i_068694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68698;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68698);
    cpu.pc = 0x68A70;
    16
}
/// $068698: jsr $68b46(pc)
pub fn i_068698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6869C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6869C);
    cpu.pc = 0x68B46;
    16
}
/// $06869C: jsr $68ab8(pc)
pub fn i_06869c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x686A0);
    cpu.pc = 0x68AB8;
    16
}
/// $0686A0: cmpi.b #$3, $20(a6)
pub fn i_0686a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686A6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $0686A6: beq.w $686b0
pub fn i_0686a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x686B0; return 10; }
    8
}
/// $0686AA: lea.l $6850e(pc), a1
pub fn i_0686aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6850E;
    16
}
/// $0686AE: move.l a1, (a6)
pub fn i_0686ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0686B0: bclr.b #$3, $13(a6)
pub fn i_0686b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686B6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0686B6: bclr.b #$0, $13(a6)
pub fn i_0686b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686BC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0686BC: tst.b $106ed3.l
pub fn i_0686bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686C2;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0686C2: bne.w $686cc
pub fn i_0686c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686C6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x686CC; return 10; }
    8
}
// ===== SetTaskHandler_0686c6  ($0686C6, 8 B, kernel/task_handlers.c)
/// $0686C6: lea.l $6850e(pc), a1
pub fn i_0686c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6850E;
    16
}
/// $0686CA: move.l a1, (a6)
pub fn i_0686ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0686CC: rts
pub fn i_0686cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_Light_0686ce  ($0686CE, 24 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0686CE: clr.w $82(a6)
pub fn i_0686ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686D2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0686D2: clr.w $84(a6)
pub fn i_0686d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686D6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0686D6: move.w #$1b7, d1
pub fn i_0686d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686DA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0686DA: jsr $236e.l
pub fn i_0686da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x686E0);
    cpu.pc = 0x236E;
    16
}
/// $0686E0: bset.b #$6, $12(a6)
pub fn i_0686e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686E6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== S5Airship_LightBlink_0686e6  ($0686E6, 62 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $0686E6: lea.l $2c8506.l, a0
pub fn i_0686e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8506;
    20
}
/// $0686EC: jsr $28cd4.l
pub fn i_0686ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x686F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0686F2: lea.l $686f8(pc), a1
pub fn i_0686f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x686F8;
    16
}
/// $0686F6: move.l a1, (a6)
pub fn i_0686f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0686F8: jsr $68af4(pc)
pub fn i_0686f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x686FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x686FC);
    cpu.pc = 0x68AF4;
    16
}
/// $0686FC: bcc.w $68720
pub fn i_0686fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68700;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x68720; return 10; }
    8
}
/// $068700: lea.l $2c8512.l, a0
pub fn i_068700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68706;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C8512;
    20
}
/// $068706: jsr $28cd4.l
pub fn i_068706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6870C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6870C);
    cpu.pc = 0x28CD4;
    16
}
/// $06870C: lea.l $68712(pc), a1
pub fn i_06870c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68710;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x68712;
    16
}
/// $068710: move.l a1, (a6)
pub fn i_068710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68712;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068712: jsr $68af4(pc)
pub fn i_068712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68716;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68716);
    cpu.pc = 0x68AF4;
    16
}
/// $068716: bcs.w $68720
pub fn i_068716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6871A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x68720; return 10; }
    8
}
/// $06871A: lea.l $686e6(pc), a1
pub fn i_06871a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6871E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x686E6;
    16
}
/// $06871E: move.l a1, (a6)
pub fn i_06871e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68720;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $068720: jsr $68a90(pc)
pub fn i_068720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68724;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68724);
    cpu.pc = 0x68A90;
    16
}
// ===== JsrPcThunk_068724  ($068724, 6 B, thunks/jsr_pc_thunks.c)
/// $068724: jsr $68ab8(pc)
pub fn i_068724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68728;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x68728);
    cpu.pc = 0x68AB8;
    16
}
/// $068728: rts
pub fn i_068728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6872A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5Airship_Hull_06872a  ($06872A, 46 B, enemies/soldiers/barrel_paratrooper_shield_tank_066000.s)
/// $06872A: clr.w $82(a6)
pub fn i_06872a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6872E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06872E: clr.w $84(a6)
pub fn i_06872e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68732;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $068732: move.w #$1b7, d1
pub fn i_068732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x68736;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $068736: jsr $236e.l
pub fn i_068736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6873C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6873C);
    cpu.pc = 0x236E;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0669D0, i_0669d0),
    (0x0669D6, i_0669d6),
    (0x0669DC, i_0669dc),
    (0x0669E0, i_0669e0),
    (0x0669E4, i_0669e4),
    (0x0669E6, i_0669e6),
    (0x0669EA, i_0669ea),
    (0x0669F0, i_0669f0),
    (0x0669F4, i_0669f4),
    (0x0669FA, i_0669fa),
    (0x0669FE, i_0669fe),
    (0x066A02, i_066a02),
    (0x066A08, i_066a08),
    (0x066A0C, i_066a0c),
    (0x066A12, i_066a12),
    (0x066A16, i_066a16),
    (0x066A1C, i_066a1c),
    (0x066A20, i_066a20),
    (0x066A22, i_066a22),
    (0x066A28, i_066a28),
    (0x066A2E, i_066a2e),
    (0x066A32, i_066a32),
    (0x066A36, i_066a36),
    (0x066A38, i_066a38),
    (0x066A3C, i_066a3c),
    (0x066A40, i_066a40),
    (0x066A44, i_066a44),
    (0x066A48, i_066a48),
    (0x066A4C, i_066a4c),
    (0x066A4E, i_066a4e),
    (0x066A52, i_066a52),
    (0x066A56, i_066a56),
    (0x066A5A, i_066a5a),
    (0x066A5C, i_066a5c),
    (0x066A62, i_066a62),
    (0x066A66, i_066a66),
    (0x066A6A, i_066a6a),
    (0x066A6C, i_066a6c),
    (0x066A70, i_066a70),
    (0x066A74, i_066a74),
    (0x066A7A, i_066a7a),
    (0x066A7E, i_066a7e),
    (0x066A82, i_066a82),
    (0x066A84, i_066a84),
    (0x066A86, i_066a86),
    (0x066A8C, i_066a8c),
    (0x066A8E, i_066a8e),
    (0x066A94, i_066a94),
    (0x066A9A, i_066a9a),
    (0x066A9C, i_066a9c),
    (0x066AA0, i_066aa0),
    (0x066AA6, i_066aa6),
    (0x066AAC, i_066aac),
    (0x066AB2, i_066ab2),
    (0x066AB8, i_066ab8),
    (0x066ABE, i_066abe),
    (0x066AC4, i_066ac4),
    (0x066AC8, i_066ac8),
    (0x066ACC, i_066acc),
    (0x066ACE, i_066ace),
    (0x066AD4, i_066ad4),
    (0x066ADA, i_066ada),
    (0x066ADE, i_066ade),
    (0x066AE2, i_066ae2),
    (0x066AE4, i_066ae4),
    (0x066AE6, i_066ae6),
    (0x066AE8, i_066ae8),
    (0x066AEA, i_066aea),
    (0x066AEE, i_066aee),
    (0x066AF2, i_066af2),
    (0x066AF6, i_066af6),
    (0x066AFC, i_066afc),
    (0x066B02, i_066b02),
    (0x066B08, i_066b08),
    (0x066B0C, i_066b0c),
    (0x066B12, i_066b12),
    (0x066B16, i_066b16),
    (0x066B1C, i_066b1c),
    (0x066B20, i_066b20),
    (0x066B22, i_066b22),
    (0x066B28, i_066b28),
    (0x066B2E, i_066b2e),
    (0x066B34, i_066b34),
    (0x066B3A, i_066b3a),
    (0x066B3E, i_066b3e),
    (0x066B42, i_066b42),
    (0x066B44, i_066b44),
    (0x066B4A, i_066b4a),
    (0x066B4E, i_066b4e),
    (0x066B52, i_066b52),
    (0x066B54, i_066b54),
    (0x066B5A, i_066b5a),
    (0x066B60, i_066b60),
    (0x066B64, i_066b64),
    (0x066B68, i_066b68),
    (0x066B6A, i_066b6a),
    (0x066B6C, i_066b6c),
    (0x066B70, i_066b70),
    (0x066B76, i_066b76),
    (0x066B7C, i_066b7c),
    (0x066B82, i_066b82),
    (0x066B86, i_066b86),
    (0x066B8C, i_066b8c),
    (0x066B90, i_066b90),
    (0x066B96, i_066b96),
    (0x066B9A, i_066b9a),
    (0x066B9C, i_066b9c),
    (0x066BA2, i_066ba2),
    (0x066BA8, i_066ba8),
    (0x066BAC, i_066bac),
    (0x066BB0, i_066bb0),
    (0x066BB2, i_066bb2),
    (0x066BB8, i_066bb8),
    (0x066BBE, i_066bbe),
    (0x066BC2, i_066bc2),
    (0x066BC6, i_066bc6),
    (0x066BC8, i_066bc8),
    (0x066BCA, i_066bca),
    (0x066BD0, i_066bd0),
    (0x066BD4, i_066bd4),
    (0x066BD8, i_066bd8),
    (0x066BDC, i_066bdc),
    (0x066BE0, i_066be0),
    (0x066BE2, i_066be2),
    (0x066BE6, i_066be6),
    (0x066BEA, i_066bea),
    (0x066BF0, i_066bf0),
    (0x066BF6, i_066bf6),
    (0x066BFA, i_066bfa),
    (0x066C00, i_066c00),
    (0x066C04, i_066c04),
    (0x066C06, i_066c06),
    (0x066C0C, i_066c0c),
    (0x066C10, i_066c10),
    (0x066C12, i_066c12),
    (0x066C16, i_066c16),
    (0x066C1C, i_066c1c),
    (0x066C20, i_066c20),
    (0x066C26, i_066c26),
    (0x066C2A, i_066c2a),
    (0x066C30, i_066c30),
    (0x066C34, i_066c34),
    (0x066C38, i_066c38),
    (0x066C3A, i_066c3a),
    (0x066C3E, i_066c3e),
    (0x066C40, i_066c40),
    (0x066C44, i_066c44),
    (0x066C4A, i_066c4a),
    (0x066C50, i_066c50),
    (0x066C56, i_066c56),
    (0x066C5C, i_066c5c),
    (0x066C60, i_066c60),
    (0x066C66, i_066c66),
    (0x066C6C, i_066c6c),
    (0x066C72, i_066c72),
    (0x066C78, i_066c78),
    (0x066C7E, i_066c7e),
    (0x066C80, i_066c80),
    (0x066C84, i_066c84),
    (0x066C8A, i_066c8a),
    (0x066C90, i_066c90),
    (0x066C96, i_066c96),
    (0x066C9C, i_066c9c),
    (0x066CA0, i_066ca0),
    (0x066CA6, i_066ca6),
    (0x066CAC, i_066cac),
    (0x066CB2, i_066cb2),
    (0x066CB8, i_066cb8),
    (0x066CBE, i_066cbe),
    (0x066CC0, i_066cc0),
    (0x066CC6, i_066cc6),
    (0x066CCC, i_066ccc),
    (0x066CD2, i_066cd2),
    (0x066CD6, i_066cd6),
    (0x0670B6, i_0670b6),
    (0x0670BA, i_0670ba),
    (0x0670BE, i_0670be),
    (0x0670C2, i_0670c2),
    (0x0670C6, i_0670c6),
    (0x0670CA, i_0670ca),
    (0x0670CC, i_0670cc),
    (0x0670D0, i_0670d0),
    (0x0670D2, i_0670d2),
    (0x0670D6, i_0670d6),
    (0x0670DC, i_0670dc),
    (0x0670E2, i_0670e2),
    (0x0670E6, i_0670e6),
    (0x0670EC, i_0670ec),
    (0x0670F0, i_0670f0),
    (0x0670F6, i_0670f6),
    (0x0670FA, i_0670fa),
    (0x0670FE, i_0670fe),
    (0x067104, i_067104),
    (0x06710A, i_06710a),
    (0x06710E, i_06710e),
    (0x067110, i_067110),
    (0x067114, i_067114),
    (0x067118, i_067118),
    (0x06711C, i_06711c),
    (0x067122, i_067122),
    (0x067126, i_067126),
    (0x06712A, i_06712a),
    (0x06712E, i_06712e),
    (0x067132, i_067132),
    (0x067136, i_067136),
    (0x067138, i_067138),
    (0x06713C, i_06713c),
    (0x067140, i_067140),
    (0x067144, i_067144),
    (0x067146, i_067146),
    (0x06714A, i_06714a),
    (0x06714E, i_06714e),
    (0x067152, i_067152),
    (0x067154, i_067154),
    (0x067158, i_067158),
    (0x06715C, i_06715c),
    (0x067160, i_067160),
    (0x067162, i_067162),
    (0x067166, i_067166),
    (0x06716A, i_06716a),
    (0x06716E, i_06716e),
    (0x067170, i_067170),
    (0x067174, i_067174),
    (0x067178, i_067178),
    (0x06717C, i_06717c),
    (0x06717E, i_06717e),
    (0x067182, i_067182),
    (0x067188, i_067188),
    (0x06718C, i_06718c),
    (0x067192, i_067192),
    (0x067198, i_067198),
    (0x06719E, i_06719e),
    (0x0671A2, i_0671a2),
    (0x0671A4, i_0671a4),
    (0x0671AA, i_0671aa),
    (0x0671AE, i_0671ae),
    (0x0671B2, i_0671b2),
    (0x0671B4, i_0671b4),
    (0x0671BA, i_0671ba),
    (0x0671BE, i_0671be),
    (0x0671C4, i_0671c4),
    (0x0671CA, i_0671ca),
    (0x0671D0, i_0671d0),
    (0x0671D4, i_0671d4),
    (0x0671D8, i_0671d8),
    (0x0671DA, i_0671da),
    (0x0671DC, i_0671dc),
    (0x0671E0, i_0671e0),
    (0x0671E6, i_0671e6),
    (0x0671EC, i_0671ec),
    (0x0671F0, i_0671f0),
    (0x0671F2, i_0671f2),
    (0x0671F6, i_0671f6),
    (0x0671FA, i_0671fa),
    (0x0671FE, i_0671fe),
    (0x067204, i_067204),
    (0x067208, i_067208),
    (0x06720C, i_06720c),
    (0x06720E, i_06720e),
    (0x067212, i_067212),
    (0x067216, i_067216),
    (0x06721A, i_06721a),
    (0x06721C, i_06721c),
    (0x067220, i_067220),
    (0x067226, i_067226),
    (0x06722C, i_06722c),
    (0x067230, i_067230),
    (0x067232, i_067232),
    (0x067236, i_067236),
    (0x06723A, i_06723a),
    (0x067240, i_067240),
    (0x067244, i_067244),
    (0x067248, i_067248),
    (0x06724A, i_06724a),
    (0x067250, i_067250),
    (0x067254, i_067254),
    (0x067258, i_067258),
    (0x067260, i_067260),
    (0x067266, i_067266),
    (0x06726C, i_06726c),
    (0x067270, i_067270),
    (0x067272, i_067272),
    (0x067276, i_067276),
    (0x06727A, i_06727a),
    (0x06727E, i_06727e),
    (0x067284, i_067284),
    (0x067288, i_067288),
    (0x06728C, i_06728c),
    (0x067290, i_067290),
    (0x067292, i_067292),
    (0x067296, i_067296),
    (0x06729A, i_06729a),
    (0x06729E, i_06729e),
    (0x0672A0, i_0672a0),
    (0x0672A4, i_0672a4),
    (0x0672A8, i_0672a8),
    (0x0672AC, i_0672ac),
    (0x0672AE, i_0672ae),
    (0x0672B2, i_0672b2),
    (0x0672B6, i_0672b6),
    (0x0672BA, i_0672ba),
    (0x0672BC, i_0672bc),
    (0x0672C0, i_0672c0),
    (0x0672C4, i_0672c4),
    (0x0672C8, i_0672c8),
    (0x0672CA, i_0672ca),
    (0x0672D0, i_0672d0),
    (0x0672D4, i_0672d4),
    (0x0672DC, i_0672dc),
    (0x0672E0, i_0672e0),
    (0x0672E6, i_0672e6),
    (0x0672EC, i_0672ec),
    (0x0672F2, i_0672f2),
    (0x0672F6, i_0672f6),
    (0x0672F8, i_0672f8),
    (0x0672FC, i_0672fc),
    (0x067302, i_067302),
    (0x067306, i_067306),
    (0x06730A, i_06730a),
    (0x06730C, i_06730c),
    (0x067310, i_067310),
    (0x067314, i_067314),
    (0x067318, i_067318),
    (0x06731A, i_06731a),
    (0x06731E, i_06731e),
    (0x067322, i_067322),
    (0x067326, i_067326),
    (0x06732C, i_06732c),
    (0x067332, i_067332),
    (0x067336, i_067336),
    (0x067338, i_067338),
    (0x06733C, i_06733c),
    (0x067340, i_067340),
    (0x067344, i_067344),
    (0x06734A, i_06734a),
    (0x06734E, i_06734e),
    (0x067352, i_067352),
    (0x067356, i_067356),
    (0x067358, i_067358),
    (0x06735C, i_06735c),
    (0x067360, i_067360),
    (0x067364, i_067364),
    (0x067366, i_067366),
    (0x06736A, i_06736a),
    (0x06736E, i_06736e),
    (0x067372, i_067372),
    (0x067374, i_067374),
    (0x067378, i_067378),
    (0x06737C, i_06737c),
    (0x067380, i_067380),
    (0x067382, i_067382),
    (0x067386, i_067386),
    (0x06738A, i_06738a),
    (0x06738E, i_06738e),
    (0x067390, i_067390),
    (0x067394, i_067394),
    (0x06739A, i_06739a),
    (0x0673A0, i_0673a0),
    (0x0673A4, i_0673a4),
    (0x0673AA, i_0673aa),
    (0x0673B0, i_0673b0),
    (0x0673B4, i_0673b4),
    (0x0673B6, i_0673b6),
    (0x0673BA, i_0673ba),
    (0x0673BE, i_0673be),
    (0x0673C2, i_0673c2),
    (0x0673C8, i_0673c8),
    (0x0673CC, i_0673cc),
    (0x0673D0, i_0673d0),
    (0x0673D2, i_0673d2),
    (0x0673D6, i_0673d6),
    (0x0673DA, i_0673da),
    (0x0673DE, i_0673de),
    (0x0673E0, i_0673e0),
    (0x0673E4, i_0673e4),
    (0x0673EA, i_0673ea),
    (0x0673F0, i_0673f0),
    (0x0673F4, i_0673f4),
    (0x0673FA, i_0673fa),
    (0x067400, i_067400),
    (0x067404, i_067404),
    (0x067406, i_067406),
    (0x06740A, i_06740a),
    (0x06740E, i_06740e),
    (0x067412, i_067412),
    (0x067418, i_067418),
    (0x06741C, i_06741c),
    (0x067422, i_067422),
    (0x067426, i_067426),
    (0x06742A, i_06742a),
    (0x06742C, i_06742c),
    (0x067430, i_067430),
    (0x067436, i_067436),
    (0x06743A, i_06743a),
    (0x06743E, i_06743e),
    (0x067440, i_067440),
    (0x067444, i_067444),
    (0x067448, i_067448),
    (0x06744C, i_06744c),
    (0x06744E, i_06744e),
    (0x067452, i_067452),
    (0x067458, i_067458),
    (0x06745E, i_06745e),
    (0x067462, i_067462),
    (0x067468, i_067468),
    (0x06746E, i_06746e),
    (0x067472, i_067472),
    (0x067474, i_067474),
    (0x067478, i_067478),
    (0x06747C, i_06747c),
    (0x067480, i_067480),
    (0x067486, i_067486),
    (0x06748A, i_06748a),
    (0x06748E, i_06748e),
    (0x067490, i_067490),
    (0x067494, i_067494),
    (0x067498, i_067498),
    (0x06749C, i_06749c),
    (0x06749E, i_06749e),
    (0x0674A2, i_0674a2),
    (0x0674A8, i_0674a8),
    (0x0674AE, i_0674ae),
    (0x0674B2, i_0674b2),
    (0x0674B8, i_0674b8),
    (0x0674BE, i_0674be),
    (0x0674C2, i_0674c2),
    (0x0674C4, i_0674c4),
    (0x0674C8, i_0674c8),
    (0x0674CC, i_0674cc),
    (0x0674D0, i_0674d0),
    (0x0674D6, i_0674d6),
    (0x0674DA, i_0674da),
    (0x0674DE, i_0674de),
    (0x0674E0, i_0674e0),
    (0x0674E4, i_0674e4),
    (0x0674E8, i_0674e8),
    (0x0674EC, i_0674ec),
    (0x0674EE, i_0674ee),
    (0x0674F2, i_0674f2),
    (0x0674F8, i_0674f8),
    (0x0674FE, i_0674fe),
    (0x067502, i_067502),
    (0x067508, i_067508),
    (0x06750E, i_06750e),
    (0x067512, i_067512),
    (0x067514, i_067514),
    (0x067518, i_067518),
    (0x06751C, i_06751c),
    (0x067520, i_067520),
    (0x067526, i_067526),
    (0x06752A, i_06752a),
    (0x067530, i_067530),
    (0x067534, i_067534),
    (0x067538, i_067538),
    (0x06753A, i_06753a),
    (0x06753E, i_06753e),
    (0x067544, i_067544),
    (0x067548, i_067548),
    (0x06754C, i_06754c),
    (0x06754E, i_06754e),
    (0x067554, i_067554),
    (0x06755A, i_06755a),
    (0x06755E, i_06755e),
    (0x067562, i_067562),
    (0x067566, i_067566),
    (0x067568, i_067568),
    (0x06756C, i_06756c),
    (0x067572, i_067572),
    (0x067578, i_067578),
    (0x06757C, i_06757c),
    (0x067582, i_067582),
    (0x067588, i_067588),
    (0x06758C, i_06758c),
    (0x06758E, i_06758e),
    (0x067592, i_067592),
    (0x067596, i_067596),
    (0x06759A, i_06759a),
    (0x0675A0, i_0675a0),
    (0x0675A4, i_0675a4),
    (0x0675A8, i_0675a8),
    (0x0675AA, i_0675aa),
    (0x0675AE, i_0675ae),
    (0x0675B2, i_0675b2),
    (0x0675B6, i_0675b6),
    (0x0675B8, i_0675b8),
    (0x0675BC, i_0675bc),
    (0x0675C2, i_0675c2),
    (0x0675C8, i_0675c8),
    (0x0675CC, i_0675cc),
    (0x0675CE, i_0675ce),
    (0x0675D2, i_0675d2),
    (0x0675D8, i_0675d8),
    (0x0675DE, i_0675de),
    (0x0675E2, i_0675e2),
    (0x0675E4, i_0675e4),
    (0x0675E8, i_0675e8),
    (0x0675EC, i_0675ec),
    (0x0675F0, i_0675f0),
    (0x0675F6, i_0675f6),
    (0x0675FA, i_0675fa),
    (0x0675FE, i_0675fe),
    (0x067600, i_067600),
    (0x067604, i_067604),
    (0x067608, i_067608),
    (0x06760C, i_06760c),
    (0x06760E, i_06760e),
    (0x067612, i_067612),
    (0x067618, i_067618),
    (0x06761C, i_06761c),
    (0x067622, i_067622),
    (0x067628, i_067628),
    (0x06762E, i_06762e),
    (0x067634, i_067634),
    (0x067638, i_067638),
    (0x06763E, i_06763e),
    (0x067644, i_067644),
    (0x06764A, i_06764a),
    (0x06764E, i_06764e),
    (0x067652, i_067652),
    (0x067654, i_067654),
    (0x067656, i_067656),
    (0x06765A, i_06765a),
    (0x067660, i_067660),
    (0x067666, i_067666),
    (0x06766A, i_06766a),
    (0x06766C, i_06766c),
    (0x067670, i_067670),
    (0x067676, i_067676),
    (0x06767A, i_06767a),
    (0x06767E, i_06767e),
    (0x067680, i_067680),
    (0x067688, i_067688),
    (0x06768C, i_06768c),
    (0x067692, i_067692),
    (0x067696, i_067696),
    (0x06769C, i_06769c),
    (0x0676A0, i_0676a0),
    (0x0676A4, i_0676a4),
    (0x0676A8, i_0676a8),
    (0x0676AE, i_0676ae),
    (0x0676B0, i_0676b0),
    (0x0676B2, i_0676b2),
    (0x0676B8, i_0676b8),
    (0x0676BA, i_0676ba),
    (0x0676BE, i_0676be),
    (0x0676C2, i_0676c2),
    (0x0676C6, i_0676c6),
    (0x0676CC, i_0676cc),
    (0x0676D2, i_0676d2),
    (0x0676D6, i_0676d6),
    (0x0676D8, i_0676d8),
    (0x0676DC, i_0676dc),
    (0x0676E0, i_0676e0),
    (0x0676E4, i_0676e4),
    (0x0676EA, i_0676ea),
    (0x0676EE, i_0676ee),
    (0x0676F2, i_0676f2),
    (0x0676F6, i_0676f6),
    (0x0676F8, i_0676f8),
    (0x0676FC, i_0676fc),
    (0x067700, i_067700),
    (0x067704, i_067704),
    (0x067706, i_067706),
    (0x06770A, i_06770a),
    (0x06770E, i_06770e),
    (0x067712, i_067712),
    (0x067714, i_067714),
    (0x067718, i_067718),
    (0x06771C, i_06771c),
    (0x067720, i_067720),
    (0x067722, i_067722),
    (0x067726, i_067726),
    (0x06772C, i_06772c),
    (0x067730, i_067730),
    (0x067736, i_067736),
    (0x06773C, i_06773c),
    (0x067742, i_067742),
    (0x067746, i_067746),
    (0x067748, i_067748),
    (0x06774E, i_06774e),
    (0x067752, i_067752),
    (0x067756, i_067756),
    (0x067758, i_067758),
    (0x06775E, i_06775e),
    (0x067762, i_067762),
    (0x067768, i_067768),
    (0x06776E, i_06776e),
    (0x067774, i_067774),
    (0x067778, i_067778),
    (0x06777C, i_06777c),
    (0x06777E, i_06777e),
    (0x067780, i_067780),
    (0x067784, i_067784),
    (0x06778A, i_06778a),
    (0x067790, i_067790),
    (0x067794, i_067794),
    (0x067796, i_067796),
    (0x06779A, i_06779a),
    (0x06779E, i_06779e),
    (0x0677A2, i_0677a2),
    (0x0677A8, i_0677a8),
    (0x0677AC, i_0677ac),
    (0x0677B0, i_0677b0),
    (0x0677B2, i_0677b2),
    (0x0677B6, i_0677b6),
    (0x0677BA, i_0677ba),
    (0x0677BE, i_0677be),
    (0x0677C0, i_0677c0),
    (0x0677C4, i_0677c4),
    (0x0677C8, i_0677c8),
    (0x0677D0, i_0677d0),
    (0x0677D6, i_0677d6),
    (0x0677DC, i_0677dc),
    (0x0677E0, i_0677e0),
    (0x0677E2, i_0677e2),
    (0x0677E6, i_0677e6),
    (0x0677EA, i_0677ea),
    (0x0677EE, i_0677ee),
    (0x0677F4, i_0677f4),
    (0x0677F8, i_0677f8),
    (0x0677FC, i_0677fc),
    (0x0677FE, i_0677fe),
    (0x067804, i_067804),
    (0x067808, i_067808),
    (0x06780C, i_06780c),
    (0x067810, i_067810),
    (0x067814, i_067814),
    (0x067818, i_067818),
    (0x06781C, i_06781c),
    (0x067820, i_067820),
    (0x067822, i_067822),
    (0x067828, i_067828),
    (0x06782C, i_06782c),
    (0x067834, i_067834),
    (0x067838, i_067838),
    (0x06783C, i_06783c),
    (0x067840, i_067840),
    (0x067842, i_067842),
    (0x067846, i_067846),
    (0x06784C, i_06784c),
    (0x067852, i_067852),
    (0x067858, i_067858),
    (0x06785C, i_06785c),
    (0x06785E, i_06785e),
    (0x067864, i_067864),
    (0x06786A, i_06786a),
    (0x06786E, i_06786e),
    (0x067872, i_067872),
    (0x067874, i_067874),
    (0x067878, i_067878),
    (0x06787C, i_06787c),
    (0x067880, i_067880),
    (0x067882, i_067882),
    (0x067886, i_067886),
    (0x06788C, i_06788c),
    (0x067892, i_067892),
    (0x067896, i_067896),
    (0x06789C, i_06789c),
    (0x0678A2, i_0678a2),
    (0x0678A6, i_0678a6),
    (0x0678A8, i_0678a8),
    (0x0678AC, i_0678ac),
    (0x0678B0, i_0678b0),
    (0x0678B4, i_0678b4),
    (0x0678BA, i_0678ba),
    (0x0678BE, i_0678be),
    (0x0678C2, i_0678c2),
    (0x0678C4, i_0678c4),
    (0x0678C8, i_0678c8),
    (0x0678CC, i_0678cc),
    (0x0678D0, i_0678d0),
    (0x0678D2, i_0678d2),
    (0x0678D6, i_0678d6),
    (0x0678DC, i_0678dc),
    (0x0678E2, i_0678e2),
    (0x0678E6, i_0678e6),
    (0x0678EC, i_0678ec),
    (0x0678F2, i_0678f2),
    (0x0678F6, i_0678f6),
    (0x0678F8, i_0678f8),
    (0x0678FC, i_0678fc),
    (0x067900, i_067900),
    (0x067904, i_067904),
    (0x06790A, i_06790a),
    (0x06790E, i_06790e),
    (0x067914, i_067914),
    (0x067918, i_067918),
    (0x06791C, i_06791c),
    (0x06791E, i_06791e),
    (0x067922, i_067922),
    (0x067928, i_067928),
    (0x06792C, i_06792c),
    (0x067930, i_067930),
    (0x067932, i_067932),
    (0x067936, i_067936),
    (0x06793A, i_06793a),
    (0x06793E, i_06793e),
    (0x067940, i_067940),
    (0x067944, i_067944),
    (0x06794A, i_06794a),
    (0x067950, i_067950),
    (0x067954, i_067954),
    (0x06795A, i_06795a),
    (0x067960, i_067960),
    (0x067964, i_067964),
    (0x067966, i_067966),
    (0x06796A, i_06796a),
    (0x06796E, i_06796e),
    (0x067972, i_067972),
    (0x067978, i_067978),
    (0x06797C, i_06797c),
    (0x067980, i_067980),
    (0x067982, i_067982),
    (0x067986, i_067986),
    (0x06798A, i_06798a),
    (0x06798E, i_06798e),
    (0x067990, i_067990),
    (0x067994, i_067994),
    (0x06799A, i_06799a),
    (0x0679A0, i_0679a0),
    (0x0679A4, i_0679a4),
    (0x0679AA, i_0679aa),
    (0x0679B0, i_0679b0),
    (0x0679B4, i_0679b4),
    (0x0679B6, i_0679b6),
    (0x0679BA, i_0679ba),
    (0x0679BE, i_0679be),
    (0x0679C2, i_0679c2),
    (0x0679C8, i_0679c8),
    (0x0679CC, i_0679cc),
    (0x0679D0, i_0679d0),
    (0x0679D2, i_0679d2),
    (0x0679D6, i_0679d6),
    (0x0679DA, i_0679da),
    (0x0679DE, i_0679de),
    (0x0679E0, i_0679e0),
    (0x0679E4, i_0679e4),
    (0x0679EA, i_0679ea),
    (0x0679F0, i_0679f0),
    (0x0679F4, i_0679f4),
    (0x0679FA, i_0679fa),
    (0x067A00, i_067a00),
    (0x067A04, i_067a04),
    (0x067A06, i_067a06),
    (0x067A0A, i_067a0a),
    (0x067A0E, i_067a0e),
    (0x067A12, i_067a12),
    (0x067A18, i_067a18),
    (0x067A1C, i_067a1c),
    (0x067A22, i_067a22),
    (0x067A26, i_067a26),
    (0x067A2A, i_067a2a),
    (0x067A2C, i_067a2c),
    (0x067A30, i_067a30),
    (0x067A36, i_067a36),
    (0x067A3A, i_067a3a),
    (0x067A3E, i_067a3e),
    (0x067A40, i_067a40),
    (0x067A46, i_067a46),
    (0x067A4C, i_067a4c),
    (0x067A50, i_067a50),
    (0x067A54, i_067a54),
    (0x067A58, i_067a58),
    (0x067A5A, i_067a5a),
    (0x067A5E, i_067a5e),
    (0x067A64, i_067a64),
    (0x067A6A, i_067a6a),
    (0x067A6E, i_067a6e),
    (0x067A74, i_067a74),
    (0x067A7A, i_067a7a),
    (0x067A7E, i_067a7e),
    (0x067A80, i_067a80),
    (0x067A84, i_067a84),
    (0x067A88, i_067a88),
    (0x067A8C, i_067a8c),
    (0x067A92, i_067a92),
    (0x067A96, i_067a96),
    (0x067A9A, i_067a9a),
    (0x067A9C, i_067a9c),
    (0x067AA0, i_067aa0),
    (0x067AA4, i_067aa4),
    (0x067AA8, i_067aa8),
    (0x067AAA, i_067aaa),
    (0x067AAE, i_067aae),
    (0x067AB4, i_067ab4),
    (0x067ABA, i_067aba),
    (0x067ABE, i_067abe),
    (0x067AC0, i_067ac0),
    (0x067AC4, i_067ac4),
    (0x067AC8, i_067ac8),
    (0x067ACE, i_067ace),
    (0x067AD2, i_067ad2),
    (0x067AD6, i_067ad6),
    (0x067AD8, i_067ad8),
    (0x067ADE, i_067ade),
    (0x067AE2, i_067ae2),
    (0x067AE8, i_067ae8),
    (0x067AEE, i_067aee),
    (0x067AF2, i_067af2),
    (0x067AF4, i_067af4),
    (0x067AF8, i_067af8),
    (0x067AFE, i_067afe),
    (0x067B04, i_067b04),
    (0x067B08, i_067b08),
    (0x067B0A, i_067b0a),
    (0x067B0E, i_067b0e),
    (0x067B12, i_067b12),
    (0x067B16, i_067b16),
    (0x067B1C, i_067b1c),
    (0x067B20, i_067b20),
    (0x067B24, i_067b24),
    (0x067B26, i_067b26),
    (0x067B2A, i_067b2a),
    (0x067B2E, i_067b2e),
    (0x067B32, i_067b32),
    (0x067B34, i_067b34),
    (0x067B38, i_067b38),
    (0x067B3E, i_067b3e),
    (0x067B42, i_067b42),
    (0x067B48, i_067b48),
    (0x067B4E, i_067b4e),
    (0x067B54, i_067b54),
    (0x067B5A, i_067b5a),
    (0x067B5E, i_067b5e),
    (0x067B64, i_067b64),
    (0x067B6A, i_067b6a),
    (0x067B70, i_067b70),
    (0x067B74, i_067b74),
    (0x067B78, i_067b78),
    (0x067B7A, i_067b7a),
    (0x067B7C, i_067b7c),
    (0x067B82, i_067b82),
    (0x067B84, i_067b84),
    (0x067B8A, i_067b8a),
    (0x067B90, i_067b90),
    (0x067B92, i_067b92),
    (0x067B96, i_067b96),
    (0x067B9A, i_067b9a),
    (0x067B9E, i_067b9e),
    (0x067BA2, i_067ba2),
    (0x067BA8, i_067ba8),
    (0x067BAE, i_067bae),
    (0x067BB4, i_067bb4),
    (0x067BB8, i_067bb8),
    (0x067BBC, i_067bbc),
    (0x067BC2, i_067bc2),
    (0x067BC8, i_067bc8),
    (0x067BCE, i_067bce),
    (0x067BD4, i_067bd4),
    (0x067BDA, i_067bda),
    (0x067BDE, i_067bde),
    (0x067BE6, i_067be6),
    (0x067BEA, i_067bea),
    (0x067BEC, i_067bec),
    (0x067BF2, i_067bf2),
    (0x067BF8, i_067bf8),
    (0x067BFC, i_067bfc),
    (0x067C00, i_067c00),
    (0x067C04, i_067c04),
    (0x067C08, i_067c08),
    (0x067C0C, i_067c0c),
    (0x067C12, i_067c12),
    (0x067C14, i_067c14),
    (0x067C18, i_067c18),
    (0x067C1E, i_067c1e),
    (0x067C22, i_067c22),
    (0x067C28, i_067c28),
    (0x067C2E, i_067c2e),
    (0x067C34, i_067c34),
    (0x067C38, i_067c38),
    (0x067C3E, i_067c3e),
    (0x067C44, i_067c44),
    (0x067C48, i_067c48),
    (0x067C4C, i_067c4c),
    (0x067C52, i_067c52),
    (0x067C56, i_067c56),
    (0x067C58, i_067c58),
    (0x067C5C, i_067c5c),
    (0x067C60, i_067c60),
    (0x067C64, i_067c64),
    (0x067C68, i_067c68),
    (0x067C6A, i_067c6a),
    (0x067C70, i_067c70),
    (0x067C74, i_067c74),
    (0x067C78, i_067c78),
    (0x067C7A, i_067c7a),
    (0x067C7C, i_067c7c),
    (0x067C82, i_067c82),
    (0x067C88, i_067c88),
    (0x067C8C, i_067c8c),
    (0x067C92, i_067c92),
    (0x067C96, i_067c96),
    (0x067C9C, i_067c9c),
    (0x067CA2, i_067ca2),
    (0x067CA8, i_067ca8),
    (0x067CAE, i_067cae),
    (0x067CB4, i_067cb4),
    (0x067CB8, i_067cb8),
    (0x067CBA, i_067cba),
    (0x067CC0, i_067cc0),
    (0x067CC4, i_067cc4),
    (0x067CC8, i_067cc8),
    (0x067CCA, i_067cca),
    (0x067CD0, i_067cd0),
    (0x067CD6, i_067cd6),
    (0x067CDC, i_067cdc),
    (0x067CE0, i_067ce0),
    (0x067CE4, i_067ce4),
    (0x067CE6, i_067ce6),
    (0x067CE8, i_067ce8),
    (0x067CEC, i_067cec),
    (0x067CF2, i_067cf2),
    (0x067CF6, i_067cf6),
    (0x067CFC, i_067cfc),
    (0x067D02, i_067d02),
    (0x067D08, i_067d08),
    (0x067D0E, i_067d0e),
    (0x067D14, i_067d14),
    (0x067D18, i_067d18),
    (0x067D1A, i_067d1a),
    (0x067D20, i_067d20),
    (0x067D24, i_067d24),
    (0x067D28, i_067d28),
    (0x067D2A, i_067d2a),
    (0x067D30, i_067d30),
    (0x067D32, i_067d32),
    (0x067D38, i_067d38),
    (0x067D3C, i_067d3c),
    (0x067D3E, i_067d3e),
    (0x067D44, i_067d44),
    (0x067D4C, i_067d4c),
    (0x067D50, i_067d50),
    (0x067D56, i_067d56),
    (0x067D5A, i_067d5a),
    (0x067D60, i_067d60),
    (0x067D64, i_067d64),
    (0x067D68, i_067d68),
    (0x067D6A, i_067d6a),
    (0x067D6E, i_067d6e),
    (0x067D74, i_067d74),
    (0x067D78, i_067d78),
    (0x067D7E, i_067d7e),
    (0x067D84, i_067d84),
    (0x067D8A, i_067d8a),
    (0x067D90, i_067d90),
    (0x067D96, i_067d96),
    (0x067D9C, i_067d9c),
    (0x067DA0, i_067da0),
    (0x067DA2, i_067da2),
    (0x067DA6, i_067da6),
    (0x067DAA, i_067daa),
    (0x067DB0, i_067db0),
    (0x067DB4, i_067db4),
    (0x067DBA, i_067dba),
    (0x067DBE, i_067dbe),
    (0x067DC4, i_067dc4),
    (0x067DCA, i_067dca),
    (0x067DD0, i_067dd0),
    (0x067DD4, i_067dd4),
    (0x067DD6, i_067dd6),
    (0x067DDC, i_067ddc),
    (0x067DE0, i_067de0),
    (0x067DE4, i_067de4),
    (0x067DE6, i_067de6),
    (0x067DEC, i_067dec),
    (0x067DF2, i_067df2),
    (0x067DF8, i_067df8),
    (0x067DFC, i_067dfc),
    (0x067E00, i_067e00),
    (0x067E02, i_067e02),
    (0x067E08, i_067e08),
    (0x067E0E, i_067e0e),
    (0x067E12, i_067e12),
    (0x067E16, i_067e16),
    (0x067E18, i_067e18),
    (0x067E1A, i_067e1a),
    (0x067E1E, i_067e1e),
    (0x067E24, i_067e24),
    (0x067E2A, i_067e2a),
    (0x067E30, i_067e30),
    (0x067E34, i_067e34),
    (0x067E3A, i_067e3a),
    (0x067E40, i_067e40),
    (0x067E46, i_067e46),
    (0x067E4A, i_067e4a),
    (0x067E4C, i_067e4c),
    (0x067E52, i_067e52),
    (0x067E58, i_067e58),
    (0x067E5C, i_067e5c),
    (0x067E60, i_067e60),
    (0x067E62, i_067e62),
    (0x067E68, i_067e68),
    (0x067E6E, i_067e6e),
    (0x067E72, i_067e72),
    (0x067E76, i_067e76),
    (0x067E78, i_067e78),
    (0x067E7A, i_067e7a),
    (0x067E7E, i_067e7e),
    (0x067E82, i_067e82),
    (0x067E88, i_067e88),
    (0x067E8C, i_067e8c),
    (0x067E92, i_067e92),
    (0x067E98, i_067e98),
    (0x067E9E, i_067e9e),
    (0x067EA2, i_067ea2),
    (0x067EA8, i_067ea8),
    (0x067EAC, i_067eac),
    (0x067EB0, i_067eb0),
    (0x067EB4, i_067eb4),
    (0x067EB8, i_067eb8),
    (0x067EBC, i_067ebc),
    (0x067EC2, i_067ec2),
    (0x067EC8, i_067ec8),
    (0x067ECE, i_067ece),
    (0x067ED2, i_067ed2),
    (0x067ED6, i_067ed6),
    (0x067EDC, i_067edc),
    (0x067EE2, i_067ee2),
    (0x067EE8, i_067ee8),
    (0x067EEC, i_067eec),
    (0x067EF2, i_067ef2),
    (0x067EF8, i_067ef8),
    (0x067EFE, i_067efe),
    (0x067F02, i_067f02),
    (0x067F08, i_067f08),
    (0x067F10, i_067f10),
    (0x067F18, i_067f18),
    (0x067F20, i_067f20),
    (0x067F24, i_067f24),
    (0x067F2A, i_067f2a),
    (0x067F30, i_067f30),
    (0x067F34, i_067f34),
    (0x067F3A, i_067f3a),
    (0x067F40, i_067f40),
    (0x067F44, i_067f44),
    (0x067F46, i_067f46),
    (0x067F4C, i_067f4c),
    (0x067F52, i_067f52),
    (0x067F56, i_067f56),
    (0x067F5C, i_067f5c),
    (0x067F60, i_067f60),
    (0x067F62, i_067f62),
    (0x067F68, i_067f68),
    (0x067F6E, i_067f6e),
    (0x067F72, i_067f72),
    (0x067F78, i_067f78),
    (0x067F7A, i_067f7a),
    (0x067F80, i_067f80),
    (0x067F84, i_067f84),
    (0x067F86, i_067f86),
    (0x067F8C, i_067f8c),
    (0x067F92, i_067f92),
    (0x067F94, i_067f94),
    (0x067F9A, i_067f9a),
    (0x067F9E, i_067f9e),
    (0x067FA0, i_067fa0),
    (0x067FA4, i_067fa4),
    (0x067FA6, i_067fa6),
    (0x067FAA, i_067faa),
    (0x067FB0, i_067fb0),
    (0x067FB6, i_067fb6),
    (0x067FB8, i_067fb8),
    (0x067FBC, i_067fbc),
    (0x067FC0, i_067fc0),
    (0x067FC4, i_067fc4),
    (0x067FCA, i_067fca),
    (0x067FCE, i_067fce),
    (0x067FD4, i_067fd4),
    (0x067FDA, i_067fda),
    (0x067FE0, i_067fe0),
    (0x067FE4, i_067fe4),
    (0x067FE6, i_067fe6),
    (0x067FEA, i_067fea),
    (0x067FEC, i_067fec),
    (0x067FF2, i_067ff2),
    (0x067FF6, i_067ff6),
    (0x067FFC, i_067ffc),
    (0x068000, i_068000),
    (0x068006, i_068006),
    (0x06800A, i_06800a),
    (0x068010, i_068010),
    (0x068014, i_068014),
    (0x068018, i_068018),
    (0x06801A, i_06801a),
    (0x06801E, i_06801e),
    (0x068020, i_068020),
    (0x068026, i_068026),
    (0x06802A, i_06802a),
    (0x068030, i_068030),
    (0x068034, i_068034),
    (0x06803A, i_06803a),
    (0x06803E, i_06803e),
    (0x068044, i_068044),
    (0x068048, i_068048),
    (0x06804E, i_06804e),
    (0x068054, i_068054),
    (0x068058, i_068058),
    (0x06805C, i_06805c),
    (0x068060, i_068060),
    (0x068064, i_068064),
    (0x068068, i_068068),
    (0x06806A, i_06806a),
    (0x06806E, i_06806e),
    (0x068070, i_068070),
    (0x068076, i_068076),
    (0x068078, i_068078),
    (0x06807E, i_06807e),
    (0x068080, i_068080),
    (0x068086, i_068086),
    (0x068088, i_068088),
    (0x06808E, i_06808e),
    (0x068090, i_068090),
    (0x068096, i_068096),
    (0x06809C, i_06809c),
    (0x06809E, i_06809e),
    (0x0680A2, i_0680a2),
    (0x0680A4, i_0680a4),
    (0x0680AA, i_0680aa),
    (0x0680AE, i_0680ae),
    (0x0680B4, i_0680b4),
    (0x0680B8, i_0680b8),
    (0x0680BE, i_0680be),
    (0x0680C2, i_0680c2),
    (0x0680C8, i_0680c8),
    (0x0680CC, i_0680cc),
    (0x0680D2, i_0680d2),
    (0x0680D8, i_0680d8),
    (0x0680DC, i_0680dc),
    (0x0680E0, i_0680e0),
    (0x0680E2, i_0680e2),
    (0x0680E6, i_0680e6),
    (0x0680E8, i_0680e8),
    (0x0680EE, i_0680ee),
    (0x0680F2, i_0680f2),
    (0x0680F6, i_0680f6),
    (0x0680FA, i_0680fa),
    (0x0680FE, i_0680fe),
    (0x068102, i_068102),
    (0x068104, i_068104),
    (0x068108, i_068108),
    (0x06810A, i_06810a),
    (0x068110, i_068110),
    (0x068114, i_068114),
    (0x068118, i_068118),
    (0x06811C, i_06811c),
    (0x068120, i_068120),
    (0x068124, i_068124),
    (0x068128, i_068128),
    (0x06812A, i_06812a),
    (0x06812E, i_06812e),
    (0x068130, i_068130),
    (0x068136, i_068136),
    (0x06813A, i_06813a),
    (0x06813E, i_06813e),
    (0x068142, i_068142),
    (0x068146, i_068146),
    (0x06814A, i_06814a),
    (0x06814E, i_06814e),
    (0x068152, i_068152),
    (0x068154, i_068154),
    (0x068158, i_068158),
    (0x06815A, i_06815a),
    (0x068160, i_068160),
    (0x068164, i_068164),
    (0x068168, i_068168),
    (0x06816C, i_06816c),
    (0x068170, i_068170),
    (0x068172, i_068172),
    (0x068176, i_068176),
    (0x06817A, i_06817a),
    (0x06817E, i_06817e),
    (0x068180, i_068180),
    (0x068184, i_068184),
    (0x068186, i_068186),
    (0x06818A, i_06818a),
    (0x068190, i_068190),
    (0x068194, i_068194),
    (0x06819A, i_06819a),
    (0x0681A0, i_0681a0),
    (0x0681A6, i_0681a6),
    (0x0681AA, i_0681aa),
    (0x0681B0, i_0681b0),
    (0x0681B6, i_0681b6),
    (0x0681BA, i_0681ba),
    (0x0681BE, i_0681be),
    (0x0681C0, i_0681c0),
    (0x0681C4, i_0681c4),
    (0x0681C6, i_0681c6),
    (0x0681CC, i_0681cc),
    (0x0681D0, i_0681d0),
    (0x0681D4, i_0681d4),
    (0x0681DA, i_0681da),
    (0x0681DE, i_0681de),
    (0x0681E2, i_0681e2),
    (0x0681E4, i_0681e4),
    (0x0681E8, i_0681e8),
    (0x0681EC, i_0681ec),
    (0x0681EE, i_0681ee),
    (0x0681F2, i_0681f2),
    (0x0681FA, i_0681fa),
    (0x0681FE, i_0681fe),
    (0x068204, i_068204),
    (0x068206, i_068206),
    (0x06820A, i_06820a),
    (0x068210, i_068210),
    (0x068214, i_068214),
    (0x06821A, i_06821a),
    (0x068220, i_068220),
    (0x068224, i_068224),
    (0x06822A, i_06822a),
    (0x068230, i_068230),
    (0x068236, i_068236),
    (0x06823A, i_06823a),
    (0x068240, i_068240),
    (0x068244, i_068244),
    (0x068246, i_068246),
    (0x06824A, i_06824a),
    (0x06824C, i_06824c),
    (0x068250, i_068250),
    (0x068254, i_068254),
    (0x068258, i_068258),
    (0x06825C, i_06825c),
    (0x06825E, i_06825e),
    (0x068260, i_068260),
    (0x068264, i_068264),
    (0x068268, i_068268),
    (0x06826C, i_06826c),
    (0x068270, i_068270),
    (0x068276, i_068276),
    (0x06827A, i_06827a),
    (0x06827E, i_06827e),
    (0x068284, i_068284),
    (0x068286, i_068286),
    (0x06828A, i_06828a),
    (0x06828E, i_06828e),
    (0x068292, i_068292),
    (0x068296, i_068296),
    (0x06829C, i_06829c),
    (0x0682A0, i_0682a0),
    (0x0682A6, i_0682a6),
    (0x0682AC, i_0682ac),
    (0x0682B2, i_0682b2),
    (0x0682B4, i_0682b4),
    (0x0682B8, i_0682b8),
    (0x0682BC, i_0682bc),
    (0x0682C0, i_0682c0),
    (0x0682C4, i_0682c4),
    (0x0682C8, i_0682c8),
    (0x0682CA, i_0682ca),
    (0x0682CE, i_0682ce),
    (0x0682D0, i_0682d0),
    (0x0682D6, i_0682d6),
    (0x0682DC, i_0682dc),
    (0x0682E0, i_0682e0),
    (0x0682E6, i_0682e6),
    (0x0682EA, i_0682ea),
    (0x0682EC, i_0682ec),
    (0x0682F4, i_0682f4),
    (0x0682F8, i_0682f8),
    (0x068300, i_068300),
    (0x068304, i_068304),
    (0x068308, i_068308),
    (0x06830C, i_06830c),
    (0x06830E, i_06830e),
    (0x068310, i_068310),
    (0x068318, i_068318),
    (0x06831E, i_06831e),
    (0x068322, i_068322),
    (0x068324, i_068324),
    (0x06832C, i_06832c),
    (0x068330, i_068330),
    (0x068334, i_068334),
    (0x068336, i_068336),
    (0x06833A, i_06833a),
    (0x06833E, i_06833e),
    (0x068342, i_068342),
    (0x068344, i_068344),
    (0x068346, i_068346),
    (0x06834A, i_06834a),
    (0x068350, i_068350),
    (0x068354, i_068354),
    (0x06835A, i_06835a),
    (0x068362, i_068362),
    (0x068368, i_068368),
    (0x06836E, i_06836e),
    (0x068374, i_068374),
    (0x068378, i_068378),
    (0x06837A, i_06837a),
    (0x068384, i_068384),
    (0x06838E, i_06838e),
    (0x068392, i_068392),
    (0x068396, i_068396),
    (0x068398, i_068398),
    (0x06839C, i_06839c),
    (0x06839E, i_06839e),
    (0x0683A8, i_0683a8),
    (0x0683AC, i_0683ac),
    (0x0683AE, i_0683ae),
    (0x0683B2, i_0683b2),
    (0x0683B4, i_0683b4),
    (0x0683B8, i_0683b8),
    (0x0683BA, i_0683ba),
    (0x0683C4, i_0683c4),
    (0x0683CE, i_0683ce),
    (0x0683D2, i_0683d2),
    (0x0683D8, i_0683d8),
    (0x0683DE, i_0683de),
    (0x0683E2, i_0683e2),
    (0x0683E8, i_0683e8),
    (0x0683EA, i_0683ea),
    (0x0683EE, i_0683ee),
    (0x0683F0, i_0683f0),
    (0x0683F4, i_0683f4),
    (0x0683F8, i_0683f8),
    (0x0683FE, i_0683fe),
    (0x068402, i_068402),
    (0x068406, i_068406),
    (0x06840C, i_06840c),
    (0x068412, i_068412),
    (0x068416, i_068416),
    (0x06841C, i_06841c),
    (0x068422, i_068422),
    (0x068426, i_068426),
    (0x06842A, i_06842a),
    (0x068430, i_068430),
    (0x068436, i_068436),
    (0x06843A, i_06843a),
    (0x06843E, i_06843e),
    (0x068444, i_068444),
    (0x06844A, i_06844a),
    (0x068450, i_068450),
    (0x068454, i_068454),
    (0x06845A, i_06845a),
    (0x068460, i_068460),
    (0x068466, i_068466),
    (0x06846A, i_06846a),
    (0x068470, i_068470),
    (0x068476, i_068476),
    (0x06847C, i_06847c),
    (0x068480, i_068480),
    (0x068482, i_068482),
    (0x068486, i_068486),
    (0x06848A, i_06848a),
    (0x068490, i_068490),
    (0x068494, i_068494),
    (0x06849A, i_06849a),
    (0x06849E, i_06849e),
    (0x0684A2, i_0684a2),
    (0x0684A8, i_0684a8),
    (0x0684AC, i_0684ac),
    (0x0684B2, i_0684b2),
    (0x0684B4, i_0684b4),
    (0x0684B8, i_0684b8),
    (0x0684BC, i_0684bc),
    (0x0684C2, i_0684c2),
    (0x0684C8, i_0684c8),
    (0x0684CC, i_0684cc),
    (0x0684D2, i_0684d2),
    (0x0684D8, i_0684d8),
    (0x0684DC, i_0684dc),
    (0x0684E2, i_0684e2),
    (0x0684E8, i_0684e8),
    (0x0684EC, i_0684ec),
    (0x0684F2, i_0684f2),
    (0x0684F6, i_0684f6),
    (0x0684FA, i_0684fa),
    (0x068500, i_068500),
    (0x068506, i_068506),
    (0x06850E, i_06850e),
    (0x068512, i_068512),
    (0x068518, i_068518),
    (0x06851E, i_06851e),
    (0x068522, i_068522),
    (0x068524, i_068524),
    (0x068528, i_068528),
    (0x06852C, i_06852c),
    (0x068530, i_068530),
    (0x068534, i_068534),
    (0x068538, i_068538),
    (0x06853C, i_06853c),
    (0x068542, i_068542),
    (0x068546, i_068546),
    (0x06854A, i_06854a),
    (0x06854E, i_06854e),
    (0x068554, i_068554),
    (0x068558, i_068558),
    (0x06855C, i_06855c),
    (0x068562, i_068562),
    (0x068566, i_068566),
    (0x06856A, i_06856a),
    (0x06856E, i_06856e),
    (0x068570, i_068570),
    (0x068576, i_068576),
    (0x06857C, i_06857c),
    (0x06857E, i_06857e),
    (0x068584, i_068584),
    (0x06858A, i_06858a),
    (0x068590, i_068590),
    (0x068594, i_068594),
    (0x068596, i_068596),
    (0x06859A, i_06859a),
    (0x06859E, i_06859e),
    (0x0685A2, i_0685a2),
    (0x0685A6, i_0685a6),
    (0x0685AA, i_0685aa),
    (0x0685B0, i_0685b0),
    (0x0685B4, i_0685b4),
    (0x0685B8, i_0685b8),
    (0x0685BA, i_0685ba),
    (0x0685C0, i_0685c0),
    (0x0685C6, i_0685c6),
    (0x0685CC, i_0685cc),
    (0x0685D0, i_0685d0),
    (0x0685D4, i_0685d4),
    (0x0685D6, i_0685d6),
    (0x0685D8, i_0685d8),
    (0x0685DE, i_0685de),
    (0x0685E2, i_0685e2),
    (0x0685E4, i_0685e4),
    (0x0685E8, i_0685e8),
    (0x0685EC, i_0685ec),
    (0x0685F0, i_0685f0),
    (0x0685F4, i_0685f4),
    (0x0685FA, i_0685fa),
    (0x0685FE, i_0685fe),
    (0x068602, i_068602),
    (0x068604, i_068604),
    (0x06860A, i_06860a),
    (0x068610, i_068610),
    (0x068616, i_068616),
    (0x06861A, i_06861a),
    (0x06861E, i_06861e),
    (0x068620, i_068620),
    (0x068622, i_068622),
    (0x068628, i_068628),
    (0x06862E, i_06862e),
    (0x068632, i_068632),
    (0x068634, i_068634),
    (0x068638, i_068638),
    (0x06863C, i_06863c),
    (0x068640, i_068640),
    (0x068644, i_068644),
    (0x06864A, i_06864a),
    (0x06864E, i_06864e),
    (0x068652, i_068652),
    (0x068658, i_068658),
    (0x06865C, i_06865c),
    (0x068660, i_068660),
    (0x068664, i_068664),
    (0x068666, i_068666),
    (0x06866C, i_06866c),
    (0x068672, i_068672),
    (0x068678, i_068678),
    (0x06867C, i_06867c),
    (0x068680, i_068680),
    (0x068682, i_068682),
    (0x068684, i_068684),
    (0x06868A, i_06868a),
    (0x06868E, i_06868e),
    (0x068690, i_068690),
    (0x068694, i_068694),
    (0x068698, i_068698),
    (0x06869C, i_06869c),
    (0x0686A0, i_0686a0),
    (0x0686A6, i_0686a6),
    (0x0686AA, i_0686aa),
    (0x0686AE, i_0686ae),
    (0x0686B0, i_0686b0),
    (0x0686B6, i_0686b6),
    (0x0686BC, i_0686bc),
    (0x0686C2, i_0686c2),
    (0x0686C6, i_0686c6),
    (0x0686CA, i_0686ca),
    (0x0686CC, i_0686cc),
    (0x0686CE, i_0686ce),
    (0x0686D2, i_0686d2),
    (0x0686D6, i_0686d6),
    (0x0686DA, i_0686da),
    (0x0686E0, i_0686e0),
    (0x0686E6, i_0686e6),
    (0x0686EC, i_0686ec),
    (0x0686F2, i_0686f2),
    (0x0686F6, i_0686f6),
    (0x0686F8, i_0686f8),
    (0x0686FC, i_0686fc),
    (0x068700, i_068700),
    (0x068706, i_068706),
    (0x06870C, i_06870c),
    (0x068710, i_068710),
    (0x068712, i_068712),
    (0x068716, i_068716),
    (0x06871A, i_06871a),
    (0x06871E, i_06871e),
    (0x068720, i_068720),
    (0x068724, i_068724),
    (0x068728, i_068728),
    (0x06872A, i_06872a),
    (0x06872E, i_06872e),
    (0x068732, i_068732),
    (0x068736, i_068736),
];
