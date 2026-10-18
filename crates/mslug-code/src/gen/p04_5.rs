// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_5) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $048AEA: move.l a1, (a6)
pub fn i_048aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048AEC: bra.w $48b00
pub fn i_048aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AF0;
    cpu.ir = 0x6000;
    cpu.pc = 0x48B00;
    10
}
/// $048AF0: cmpi.w #$160, $22(a6)
pub fn i_048af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AF6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x160, Size::Word, false);
    16
}
/// $048AF6: blt.w $48b00
pub fn i_048af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AFA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x48B00; return 10; }
    8
}
/// $048AFA: lea.l $48b1e(pc), a1
pub fn i_048afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B1E;
    16
}
/// $048AFE: move.l a1, (a6)
pub fn i_048afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048B00: movea.l #$ffffffff, a0
pub fn i_048b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B06;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $048B06: lea.l $28e19e.l, a0
pub fn i_048b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B0C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E19E;
    20
}
/// $048B0C: jsr $5dd5c.l
pub fn i_048b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48B12);
    cpu.pc = 0x5DD5C;
    16
}
/// $048B12: bcc.w $48b1c
pub fn i_048b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B16;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48B1C; return 10; }
    8
}
// ===== SetTaskHandler_048b16  ($048B16, 8 B, kernel/task_handlers.c)
/// $048B16: lea.l $48b1e(pc), a1
pub fn i_048b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B1E;
    16
}
/// $048B1A: move.l a1, (a6)
pub fn i_048b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B1C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048B1C: rts
pub fn i_048b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_048b1e  ($048B1E, 8 B, kernel/jmp_to_scheduler.c)
/// $048B1E: jmp $518.l
pub fn i_048b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B24;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Jsr5B6ThenJmpScheduler_048b26  ($048B26, 14 B, kernel/jsr5b6_jmp518.c)
/// $048B26: jsr $5b6.l
pub fn i_048b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48B2C);
    cpu.pc = 0x5B6;
    16
}
/// $048B2C: jmp $518.l
pub fn i_048b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B32;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== PowItem_Settle_048b34  ($048B34, 34 B, pow/helpers_048a44.s)
/// $048B34: move.w #$2000, d0
pub fn i_048b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B38;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048B38: jsr $28134.l
pub fn i_048b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48B3E);
    cpu.pc = 0x28134;
    16
}
/// $048B3E: andi.w #$ffe3, $38(a6)
pub fn i_048b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B44;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $048B44: ori.w #$14, $38(a6)
pub fn i_048b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B4A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $048B4A: jsr $13600.l
pub fn i_048b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48B50);
    cpu.pc = 0x13600;
    16
}
/// $048B50: jmp $77f6a.l
pub fn i_048b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B56;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== PowFx_DirSprite_048b56  ($048B56, 66 B, pow/helpers_048a44.s)
/// $048B56: move.w #$4, d1
pub fn i_048b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B5A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048B5A: jsr $236e.l
pub fn i_048b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48B60);
    cpu.pc = 0x236E;
    16
}
/// $048B60: lea.l $28f556.l, a0
pub fn i_048b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B66;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28F556;
    20
}
/// $048B66: jsr $28cd4.l
pub fn i_048b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48B6C);
    cpu.pc = 0x28CD4;
    16
}
/// $048B6C: lea.l $48b72(pc), a1
pub fn i_048b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B72;
    16
}
/// $048B70: move.l a1, (a6)
pub fn i_048b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048B72: jsr $2783a.l
pub fn i_048b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48B78);
    cpu.pc = 0x2783A;
    16
}
/// $048B78: jsr $28d70.l
pub fn i_048b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48B7E);
    cpu.pc = 0x28D70;
    16
}
/// $048B7E: bcc.w $48b88
pub fn i_048b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B82;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48B88; return 10; }
    8
}
/// $048B82: lea.l $48b26(pc), a1
pub fn i_048b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B26;
    16
}
/// $048B86: move.l a1, (a6)
pub fn i_048b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048B88: movea.l #$ffffffff, a0
pub fn i_048b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B8E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $048B8E: jsr $5dd56.l
pub fn i_048b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48B94);
    cpu.pc = 0x5DD56;
    16
}
/// $048B94: bcc.w $48b9e
pub fn i_048b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B98;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48B9E; return 10; }
    8
}
// ===== SetTaskHandler_048b98  ($048B98, 8 B, kernel/task_handlers.c)
/// $048B98: lea.l $48b26(pc), a1
pub fn i_048b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B26;
    16
}
/// $048B9C: move.l a1, (a6)
pub fn i_048b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048B9E: rts
pub fn i_048b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowItem_Toss_048ba0  ($048BA0, 252 B, pow/helpers_048a44.s)
/// $048BA0: lea.l $2bfdb8.l, a0
pub fn i_048ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFDB8;
    20
}
/// $048BA6: jsr $799de.l
pub fn i_048ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48BAC);
    cpu.pc = 0x799DE;
    16
}
/// $048BAC: btst.b #$0, $3a(a6)
pub fn i_048bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BB2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $048BB2: bne.w $48bb8
pub fn i_048bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BB6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x48BB8; return 10; }
    8
}
/// $048BB6: neg.w d0
pub fn i_048bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BB8;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $048BB8: move.w d0, $28(a6)
pub fn i_048bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BBC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048BBC: clr.w $2a(a6)
pub fn i_048bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BC0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $048BC0: bra.w $48c04
pub fn i_048bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BC4;
    cpu.ir = 0x6000;
    cpu.pc = 0x48C04;
    10
}
/// $048BCA: lea.l $2bfdb8.l, a0
pub fn i_048bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BD0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFDB8;
    20
}
/// $048BD0: jsr $799de.l
pub fn i_048bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48BD6);
    cpu.pc = 0x799DE;
    16
}
/// $048BD6: move.b $7b(a6), d3
pub fn i_048bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BDA;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $048BDA: andi.w #$f, d3
pub fn i_048bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BDE;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $048BDE: lsl.w #$5, d3
pub fn i_048bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BE0;
    cpu.ir = 0xEB4B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $048BE0: lea.l $2c07ac.l, a1
pub fn i_048be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BE6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $048BE6: lea.l $2c072c.l, a2
pub fn i_048be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BEC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C072C;
    20
}
/// $048BEC: move.w (a1, d3.w), d1
pub fn i_048bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BF0;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $048BF0: move.w (a2, d3.w), d2
pub fn i_048bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BF4;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $048BF4: muls.w d0, d1
pub fn i_048bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BF6;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $048BF6: muls.w d0, d2
pub fn i_048bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BF8;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $048BF8: asr.l #$8, d1
pub fn i_048bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BFA;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $048BFA: asr.l #$8, d2
pub fn i_048bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BFC;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $048BFC: move.w d1, $28(a6)
pub fn i_048bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C00;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048C00: move.w d2, $2a(a6)
pub fn i_048c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C04;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048C04: move.w #$96, $70(a6)
pub fn i_048c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048C0A: move.w #$17b, d1
pub fn i_048c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C0E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048C0E: jsr $236e.l
pub fn i_048c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48C14);
    cpu.pc = 0x236E;
    16
}
/// $048C14: move.w #$d000, d0
pub fn i_048c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C18;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048C18: jsr $28134.l
pub fn i_048c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48C1E);
    cpu.pc = 0x28134;
    16
}
/// $048C1E: andi.w #$ffe3, $38(a6)
pub fn i_048c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C24;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $048C24: ori.w #$14, $38(a6)
pub fn i_048c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C2A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $048C2A: bset.b #$4, $6b(a6)
pub fn i_048c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C30;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $048C30: move.w #$1064, d0
pub fn i_048c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C34;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1064;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048C34: jsr $2352.l
pub fn i_048c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48C3A);
    cpu.pc = 0x2352;
    16
}
/// $048C3A: lea.l $28f51c.l, a0
pub fn i_048c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C40;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28F51C;
    20
}
/// $048C40: jsr $28cd4.l
pub fn i_048c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48C46);
    cpu.pc = 0x28CD4;
    16
}
/// $048C46: lea.l $48c4c(pc), a1
pub fn i_048c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48C4C;
    16
}
/// $048C4A: move.l a1, (a6)
pub fn i_048c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048C4C: jsr $27cee.l
pub fn i_048c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48C52);
    cpu.pc = 0x27CEE;
    16
}
/// $048C52: bcc.w $48c5c
pub fn i_048c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48C5C; return 10; }
    8
}
/// $048C56: lea.l $48b34(pc), a1
pub fn i_048c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B34;
    16
}
/// $048C5A: move.l a1, (a6)
pub fn i_048c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048C5C: jsr $28d70.l
pub fn i_048c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48C62);
    cpu.pc = 0x28D70;
    16
}
/// $048C62: subq.w #$1, $70(a6)
pub fn i_048c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C66;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $048C66: cmpi.w #$0, $70(a6)
pub fn i_048c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C6C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $048C6C: bgt.w $48c76
pub fn i_048c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C70;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x48C76; return 10; }
    8
}
/// $048C70: lea.l $48b34(pc), a1
pub fn i_048c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B34;
    16
}
/// $048C74: move.l a1, (a6)
pub fn i_048c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048C76: jsr $283d8.l
pub fn i_048c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48C7C);
    cpu.pc = 0x283D8;
    16
}
/// $048C7C: btst.b #$1, $13(a6)
pub fn i_048c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C82;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $048C82: beq.w $48c8c
pub fn i_048c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C86;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x48C8C; return 10; }
    8
}
/// $048C86: lea.l $48b34(pc), a1
pub fn i_048c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B34;
    16
}
/// $048C8A: move.l a1, (a6)
pub fn i_048c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048C8C: movea.l #$ffffffff, a0
pub fn i_048c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C92;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $048C92: jsr $5dd56.l
pub fn i_048c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48C98);
    cpu.pc = 0x5DD56;
    16
}
/// $048C98: bcc.w $48ca2
pub fn i_048c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48C9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48CA2; return 10; }
    8
}
// ===== SetTaskHandler_048c9c  ($048C9C, 8 B, kernel/task_handlers.c)
/// $048C9C: lea.l $48b26(pc), a1
pub fn i_048c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B26;
    16
}
/// $048CA0: move.l a1, (a6)
pub fn i_048ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048CA2: rts
pub fn i_048ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowFx_HitBurst_048ca4  ($048CA4, 98 B, pow/helpers_048a44.s)
/// $048CA4: move.w #$ffbc, d0
pub fn i_048ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CA8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFBC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048CA8: jsr $5dca4.l
pub fn i_048ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48CAE);
    cpu.pc = 0x5DCA4;
    16
}
/// $048CAE: move.w d0, $28(a6)
pub fn i_048cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CB2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048CB2: move.w #$32a, $2a(a6)
pub fn i_048cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CB8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048CB8: move.w #$ffca, $2e(a6)
pub fn i_048cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CBE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFCA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048CBE: move.w #$0, $2c(a6)
pub fn i_048cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CC4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048CC4: move.w #$38, d1
pub fn i_048cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CC8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048CC8: jsr $236e.l
pub fn i_048cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48CCE);
    cpu.pc = 0x236E;
    16
}
/// $048CCE: lea.l $29ca36.l, a0
pub fn i_048cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29CA36;
    20
}
/// $048CD4: jsr $28cd4.l
pub fn i_048cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48CDA);
    cpu.pc = 0x28CD4;
    16
}
/// $048CDA: lea.l $48ce0(pc), a1
pub fn i_048cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48CE0;
    16
}
/// $048CDE: move.l a1, (a6)
pub fn i_048cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048CE0: jsr $27bc8.l
pub fn i_048ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48CE6);
    cpu.pc = 0x27BC8;
    16
}
/// $048CE6: bcc.w $48cf0
pub fn i_048ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48CF0; return 10; }
    8
}
/// $048CEA: lea.l $48b26(pc), a1
pub fn i_048cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B26;
    16
}
/// $048CEE: move.l a1, (a6)
pub fn i_048cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048CF0: jsr $28d70.l
pub fn i_048cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48CF6);
    cpu.pc = 0x28D70;
    16
}
/// $048CF6: movea.l #$ffffffff, a0
pub fn i_048cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48CFC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $048CFC: jsr $5dd56.l
pub fn i_048cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48D02);
    cpu.pc = 0x5DD56;
    16
}
/// $048D02: bcc.w $48d0c
pub fn i_048d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48D0C; return 10; }
    8
}
// ===== SetTaskHandler_048d06  ($048D06, 8 B, kernel/task_handlers.c)
/// $048D06: lea.l $48b26(pc), a1
pub fn i_048d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B26;
    16
}
/// $048D0A: move.l a1, (a6)
pub fn i_048d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048D0C: rts
pub fn i_048d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowRope_Spawn_048d0e  ($048D0E, 34 B, pow/helpers_048a44.s)
/// $048D0E: move.w #$18d, d1
pub fn i_048d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D12;
    cpu.ir = 0x323C;
    let v1: u32 = 0x18D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048D12: jsr $236e.l
pub fn i_048d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48D18);
    cpu.pc = 0x236E;
    16
}
/// $048D18: move.l #$28e0e6, $48(a6)
pub fn i_048d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D20;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28E0E6;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048D20: lea.l $2bfe8a.l, a0
pub fn i_048d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFE8A;
    20
}
/// $048D26: jsr $799de.l
pub fn i_048d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48D2C);
    cpu.pc = 0x799DE;
    16
}
/// $048D2C: move.w d0, $66(a6)
pub fn i_048d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D30;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== PowRope_Idle_048d30  ($048D30, 74 B, pow/helpers_048a44.s)
/// $048D30: lea.l $28ef54.l, a0
pub fn i_048d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D36;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28EF54;
    20
}
/// $048D36: jsr $28cd4.l
pub fn i_048d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48D3C);
    cpu.pc = 0x28CD4;
    16
}
/// $048D3C: lea.l $48d42(pc), a1
pub fn i_048d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48D42;
    16
}
/// $048D40: move.l a1, (a6)
pub fn i_048d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048D42: jsr $5e45a.l
pub fn i_048d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48D48);
    cpu.pc = 0x5E45A;
    16
}
/// $048D48: bcs.w $48b26
pub fn i_048d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D4C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x48B26; return 10; }
    8
}
/// $048D4C: jsr $5e506.l
pub fn i_048d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48D52);
    cpu.pc = 0x5E506;
    16
}
/// $048D52: move.w $78(a0), $78(a6)
pub fn i_048d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D58;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad4, v3 as u16);
    20
}
/// $048D58: subq.w #$1, $38(a6)
pub fn i_048d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D5C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $048D5C: jsr $28d70.l
pub fn i_048d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48D62);
    cpu.pc = 0x28D70;
    16
}
/// $048D62: movea.l $c(a6), a0
pub fn i_048d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D66;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $048D66: cmpi.b #$1, $83(a0)
pub fn i_048d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D6C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $048D6C: bne.w $48d76
pub fn i_048d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D70;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x48D76; return 10; }
    8
}
/// $048D70: lea.l $48d7a(pc), a1
pub fn i_048d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48D7A;
    16
}
/// $048D74: move.l a1, (a6)
pub fn i_048d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048D76: bra.w $48e2c
pub fn i_048d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D7A;
    cpu.ir = 0x6000;
    cpu.pc = 0x48E2C;
    10
}
// ===== PowRope_Struggle_048d7a  ($048D7A, 98 B, pow/helpers_048a44.s)
/// $048D7A: movea.l $c(a6), a0
pub fn i_048d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D7E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $048D7E: move.w $78(a0), d0
pub fn i_048d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D82;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $048D82: lsr.w #$1, d0
pub fn i_048d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D84;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $048D84: andi.w #$f, d0
pub fn i_048d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D88;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $048D88: movea.l #$28e26e, a0
pub fn i_048d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D8E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28E26E;
    cpu.a[0] = v1;
    4
}
/// $048D8E: lsl.w #$2, d0
pub fn i_048d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D90;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $048D90: movea.l (a0, d0.w), a0
pub fn i_048d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D94;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $048D94: cmpa.l #$ffffffff, a0
pub fn i_048d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D9A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $048D9A: beq.w $48da4
pub fn i_048d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48D9E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x48DA4; return 10; }
    8
}
/// $048D9E: jsr $28cd4.l
pub fn i_048d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48DA4);
    cpu.pc = 0x28CD4;
    16
}
/// $048DA4: lea.l $48daa(pc), a1
pub fn i_048da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48DAA;
    16
}
/// $048DA8: move.l a1, (a6)
pub fn i_048da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048DAA: jsr $5e45a.l
pub fn i_048daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48DB0);
    cpu.pc = 0x5E45A;
    16
}
/// $048DB0: bcs.w $48b26
pub fn i_048db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DB4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x48B26; return 10; }
    8
}
/// $048DB4: jsr $5e506.l
pub fn i_048db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48DBA);
    cpu.pc = 0x5E506;
    16
}
/// $048DBA: subq.w #$1, $38(a6)
pub fn i_048dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DBE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $048DBE: jsr $28d70.l
pub fn i_048dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48DC4);
    cpu.pc = 0x28D70;
    16
}
/// $048DC4: movea.l $c(a6), a0
pub fn i_048dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DC8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $048DC8: cmpi.b #$0, $83(a0)
pub fn i_048dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DCE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $048DCE: bne.w $48dd8
pub fn i_048dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DD2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x48DD8; return 10; }
    8
}
/// $048DD2: lea.l $48d30(pc), a1
pub fn i_048dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48D30;
    16
}
/// $048DD6: move.l a1, (a6)
pub fn i_048dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048DD8: bra.w $48e2c
pub fn i_048dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DDC;
    cpu.ir = 0x6000;
    cpu.pc = 0x48E2C;
    10
}
// ===== PowRope_BrokenA_048ddc  ($048DDC, 16 B, pow/helpers_048a44.s)
/// $048DDC: lea.l $28f3cc.l, a0
pub fn i_048ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DE2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28F3CC;
    20
}
/// $048DE2: jsr $28cd4.l
pub fn i_048de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48DE8);
    cpu.pc = 0x28CD4;
    16
}
/// $048DE8: bra.w $48df8
pub fn i_048de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DEC;
    cpu.ir = 0x6000;
    cpu.pc = 0x48DF8;
    10
}
// ===== PowRope_BrokenB_048dec  ($048DEC, 56 B, pow/helpers_048a44.s)
/// $048DEC: lea.l $28f478.l, a0
pub fn i_048dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28F478;
    20
}
/// $048DF2: jsr $28cd4.l
pub fn i_048df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48DF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48DF8);
    cpu.pc = 0x28CD4;
    16
}
/// $048DF8: move.l #$ffffffff, $48(a6)
pub fn i_048df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E00;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048E00: lea.l $48e06(pc), a1
pub fn i_048e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48E06;
    16
}
/// $048E04: move.l a1, (a6)
pub fn i_048e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048E06: jsr $2783a.l
pub fn i_048e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48E0C);
    cpu.pc = 0x2783A;
    16
}
/// $048E0C: move.b $106f28.l, d0
pub fn i_048e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E12;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $048E12: andi.b #$1, d0
pub fn i_048e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E16;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $048E16: beq.w $48e2a
pub fn i_048e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E1A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x48E2A; return 10; }
    8
}
/// $048E1A: jsr $28d70.l
pub fn i_048e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48E20);
    cpu.pc = 0x28D70;
    16
}
/// $048E20: bcc.w $48e2a
pub fn i_048e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48E2A; return 10; }
    8
}
// ===== SetTaskHandler_048e24  ($048E24, 8 B, kernel/task_handlers.c)
/// $048E24: lea.l $48b26(pc), a1
pub fn i_048e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B26;
    16
}
/// $048E28: move.l a1, (a6)
pub fn i_048e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E2A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048E2A: rts
pub fn i_048e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowRope_CheckPhase3_048e2c  ($048E2C, 14 B, pow/helpers_048a44.s)
/// $048E2C: movea.l $c(a6), a0
pub fn i_048e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E30;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $048E30: cmpi.b #$3, $83(a0)
pub fn i_048e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E36;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $048E36: bne.w $48e42
pub fn i_048e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x48E42; return 10; }
    8
}
// ===== SetTaskHandler_048e3a  ($048E3A, 8 B, kernel/task_handlers.c)
/// $048E3A: lea.l $48ddc(pc), a1
pub fn i_048e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48DDC;
    16
}
/// $048E3E: move.l a1, (a6)
pub fn i_048e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048E40: rts
pub fn i_048e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowRope_CheckPhase2_048e42  ($048E42, 10 B, pow/helpers_048a44.s)
/// $048E42: cmpi.b #$2, $83(a0)
pub fn i_048e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E48;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $048E48: bne.w $48e54
pub fn i_048e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E4C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x48E54; return 10; }
    8
}
// ===== SetTaskHandler_048e4c  ($048E4C, 8 B, kernel/task_handlers.c)
/// $048E4C: lea.l $48dec(pc), a1
pub fn i_048e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48DEC;
    16
}
/// $048E50: move.l a1, (a6)
pub fn i_048e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048E52: rts
pub fn i_048e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowRope_HitCheck_048e54  ($048E54, 74 B, pow/helpers_048a44.s)
/// $048E54: jsr $2870a.l
pub fn i_048e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48E5A);
    cpu.pc = 0x2870A;
    16
}
/// $048E5A: bcc.w $48e64
pub fn i_048e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48E64; return 10; }
    8
}
/// $048E5E: bclr.b #$3, $13(a6)
pub fn i_048e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E64;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $048E64: jsr $28758.l
pub fn i_048e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48E6A);
    cpu.pc = 0x28758;
    16
}
/// $048E6A: bcc.w $48e94
pub fn i_048e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48E94; return 10; }
    8
}
/// $048E6E: movea.l $c(a6), a0
pub fn i_048e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E72;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $048E72: move.l a0, $50(a0)
pub fn i_048e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E76;
    cpu.ir = 0x2148;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x50);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048E76: move.b $58(a6), $58(a0)
pub fn i_048e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E7C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x58);
    bus.write8(ad4, v3 as u8);
    20
}
/// $048E7C: move.w $22(a0), $54(a0)
pub fn i_048e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E82;
    cpu.ir = 0x3168;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x54);
    bus.write16(ad4, v3 as u16);
    20
}
/// $048E82: move.w $24(a0), $56(a0)
pub fn i_048e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E88;
    cpu.ir = 0x3168;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x56);
    bus.write16(ad4, v3 as u16);
    20
}
/// $048E88: bset.b #$3, $13(a0)
pub fn i_048e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E8E;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $048E8E: lea.l $48dec(pc), a1
pub fn i_048e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48DEC;
    16
}
/// $048E92: move.l a1, (a6)
pub fn i_048e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048E94: jsr $5e45a.l
pub fn i_048e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48E9A);
    cpu.pc = 0x5E45A;
    16
}
/// $048E9A: bcc.w $48ea4
pub fn i_048e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48E9E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48EA4; return 10; }
    8
}
// ===== SetTaskHandler_048e9e  ($048E9E, 8 B, kernel/task_handlers.c)
/// $048E9E: lea.l $48b26(pc), a1
pub fn i_048e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B26;
    16
}
/// $048EA2: move.l a1, (a6)
pub fn i_048ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048EA4: rts
pub fn i_048ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_FreeInit_048ea6  ($048EA6, 94 B, pow/helpers_048a44.s)
/// $048EA6: jsr $267e2.l
pub fn i_048ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48EAC);
    cpu.pc = 0x267E2;
    16
}
/// $048EAC: cmpi.w #$a0, $22(a6)
pub fn i_048eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EB2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $048EB2: bgt.w $48ebc
pub fn i_048eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EB6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x48EBC; return 10; }
    8
}
/// $048EB6: ori.b #$1, $3a(a6)
pub fn i_048eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EBC;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $048EBC: move.w #$38, d1
pub fn i_048ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EC0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048EC0: jsr $236e.l
pub fn i_048ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48EC6);
    cpu.pc = 0x236E;
    16
}
/// $048EC6: move.w #$1, $66(a6)
pub fn i_048ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48ECC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048ECC: move.l #$28df42, $48(a6)
pub fn i_048ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48ED4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DF42;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048ED4: move.l #$28e364, $5c(a6)
pub fn i_048ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EDC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28E364;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048EDC: lea.l $2bfc32.l, a0
pub fn i_048edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EE2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFC32;
    20
}
/// $048EE2: jsr $799de.l
pub fn i_048ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48EE8);
    cpu.pc = 0x799DE;
    16
}
/// $048EE8: move.w d0, $72(a6)
pub fn i_048ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EEC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048EEC: move.w #$8000, d0
pub fn i_048eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EF0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048EF0: jsr $28134.l
pub fn i_048ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48EF6);
    cpu.pc = 0x28134;
    16
}
/// $048EF6: andi.w #$ffe3, $38(a6)
pub fn i_048ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48EFC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $048EFC: ori.w #$18, $38(a6)
pub fn i_048efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F02;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $048F02: rts
pub fn i_048f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_SetRunVelAndSprite_048f04  ($048F04, 42 B, pow/helpers_048a44.s)
/// $048F04: move.w $36(a6), d0
pub fn i_048f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F08;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $048F08: move.w d0, d1
pub fn i_048f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F0A;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048F0A: btst.b #$0, $3a(a6)
pub fn i_048f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F10;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $048F10: bne.w $48f16
pub fn i_048f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F14;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x48F16; return 10; }
    8
}
/// $048F14: neg.w d0
pub fn i_048f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F16;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $048F16: move.w d0, $28(a6)
pub fn i_048f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F1A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048F1A: lsr.w #$7, d1
pub fn i_048f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F1C;
    cpu.ir = 0xEE49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $048F1C: andi.w #$c, d1
pub fn i_048f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F20;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $048F20: lea.l $28e354.l, a0
pub fn i_048f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E354;
    20
}
/// $048F26: move.l (a0, d1.w), $5c(a6)
pub fn i_048f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F2C;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $048F2C: rts
pub fn i_048f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_ScrollProbeOrFall_048f2e  ($048F2E, 32 B, pow/helpers_048a44.s)
/// $048F2E: jsr $2783a.l
pub fn i_048f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48F34);
    cpu.pc = 0x2783A;
    16
}
/// $048F34: jsr $27eba.l
pub fn i_048f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48F3A);
    cpu.pc = 0x27EBA;
    16
}
/// $048F3A: bcc.w $48f48
pub fn i_048f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48F48; return 10; }
    8
}
/// $048F3E: jsr $27c8c.l
pub fn i_048f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48F44);
    cpu.pc = 0x27C8C;
    16
}
/// $048F44: bra.w $48f4e
pub fn i_048f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F48;
    cpu.ir = 0x6000;
    cpu.pc = 0x48F4E;
    10
}
/// $048F48: jsr $27a92.l
pub fn i_048f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48F4E);
    cpu.pc = 0x27A92;
    16
}
// ===== ClearXN_048f4e  ($048F4E, 6 B, thunks/ccr_helpers.c)
/// $048F4E: andi.b #$ee, ccr
pub fn i_048f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F52;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $048F52: rts
pub fn i_048f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TiedSwingStep_048f54  ($048F54, 92 B, pow/helpers_048a44.s)
/// $048F54: move.w #$b0, d1
pub fn i_048f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F58;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $048F58: subq.b #$2, $7f(a6)
pub fn i_048f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F5C;
    cpu.ir = 0x552E;
    let ad1 = cpu.a[6].wrapping_add(0x7F);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $048F5C: move.b $7f(a6), d0
pub fn i_048f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F60;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $048F60: andi.w #$ff, d0
pub fn i_048f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F64;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $048F64: jsr $13c0e.l
pub fn i_048f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48F6A);
    cpu.pc = 0x13C0E;
    16
}
/// $048F6A: add.w $8c(a6), d1
pub fn i_048f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F6E;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $048F6E: move.w d1, $28(a6)
pub fn i_048f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F72;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048F72: asr.w #$1, d2
pub fn i_048f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F74;
    cpu.ir = 0xE242;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $048F74: cmpi.w #$0, d2
pub fn i_048f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F78;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $048F78: ble.w $48f7e
pub fn i_048f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F7C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x48F7E; return 10; }
    8
}
/// $048F7C: neg.w d2
pub fn i_048f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F7E;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $048F7E: addi.w #$20, d2
pub fn i_048f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F82;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $048F82: sub.w $8e(a6), d2
pub fn i_048f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F86;
    cpu.ir = 0x946E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $048F86: move.w d2, $2a(a6)
pub fn i_048f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F8A;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048F8A: jsr $27cee.l
pub fn i_048f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48F90);
    cpu.pc = 0x27CEE;
    16
}
/// $048F90: tst.b $80(a6)
pub fn i_048f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F94;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $048F94: beq.w $48fa4
pub fn i_048f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x48FA4; return 10; }
    8
}
/// $048F98: jsr $27fac.l
pub fn i_048f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48F9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48F9E);
    cpu.pc = 0x27FAC;
    16
}
/// $048F9E: eori.b #$11, ccr
pub fn i_048f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FA2;
    cpu.ir = 0xA3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) ^ 0x11);
    20
}
/// $048FA2: rts
pub fn i_048fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $048FA4: jsr $27eba.l
pub fn i_048fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48FAA);
    cpu.pc = 0x27EBA;
    16
}
/// $048FAA: eori.b #$11, ccr
pub fn i_048faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FAE;
    cpu.ir = 0xA3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) ^ 0x11);
    20
}
/// $048FAE: rts
pub fn i_048fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_ScrollAndProbe_048fb0  ($048FB0, 22 B, pow/helpers_048a44.s)
/// $048FB0: jsr $2783a.l
pub fn i_048fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48FB6);
    cpu.pc = 0x2783A;
    16
}
/// $048FB6: jsr $27eba.l
pub fn i_048fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48FBC);
    cpu.pc = 0x27EBA;
    16
}
/// $048FBC: bcc.w $48fc6
pub fn i_048fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FC0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48FC6; return 10; }
    8
}
/// $048FC0: jsr $27c8c.l
pub fn i_048fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48FC6);
    cpu.pc = 0x27C8C;
    16
}
// ===== ClearXN_048fc6  ($048FC6, 6 B, thunks/ccr_helpers.c)
/// $048FC6: andi.b #$ee, ccr
pub fn i_048fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FCA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $048FCA: rts
pub fn i_048fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FCC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_PickIdleSpriteIdx_048fcc  ($048FCC, 68 B, pow/helpers_048a44.s)
/// $048FCC: jsr $490fa(pc)
pub fn i_048fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48FD0);
    cpu.pc = 0x490FA;
    16
}
/// $048FD0: bcs.w $49000
pub fn i_048fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FD4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x49000; return 10; }
    8
}
/// $048FD4: cmpi.b #$c0, $7d(a6)
pub fn i_048fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FDA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Byte, false);
    16
}
/// $048FDA: beq.w $49004
pub fn i_048fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FDE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x49004; return 10; }
    8
}
/// $048FDE: jsr $49128(pc)
pub fn i_048fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48FE2);
    cpu.pc = 0x49128;
    16
}
/// $048FE2: bcc.w $49004
pub fn i_048fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49004; return 10; }
    8
}
/// $048FE6: movea.l $94(a6), a0
pub fn i_048fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FEA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $048FEA: jsr $4914c(pc)
pub fn i_048fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48FEE);
    cpu.pc = 0x4914C;
    16
}
/// $048FEE: jsr $5e9b6.l
pub fn i_048fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48FF4);
    cpu.pc = 0x5E9B6;
    16
}
/// $048FF4: andi.w #$1, d0
pub fn i_048ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FF8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $048FF8: beq.w $49004
pub fn i_048ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48FFC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x49004; return 10; }
    8
}
/// $048FFC: bra.w $4900a
pub fn i_048ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49000;
    cpu.ir = 0x6000;
    cpu.pc = 0x4900A;
    10
}
/// $049000: clr.w d0
pub fn i_049000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49002;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $049002: rts
pub fn i_049002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49004;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $049004: move.w #$1, d0
pub fn i_049004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49008;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $049008: rts
pub fn i_049008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4900A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04900A: move.w #$2, d0
pub fn i_04900a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4900E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04900E: rts
pub fn i_04900e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49010;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_CanBeRescued_049010  ($049010, 56 B, pow/helpers_048a44.s)
/// $049010: cmpi.b #$5, $77(a6)
pub fn i_049010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49016;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $049016: bgt.w $49030
pub fn i_049016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4901A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x49030; return 10; }
    8
}
/// $04901A: cmpi.w #$0, $72(a6)
pub fn i_04901a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49020;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $049020: bgt.w $4904e
pub fn i_049020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49024;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4904E; return 10; }
    8
}
/// $049024: clr.w $72(a6)
pub fn i_049024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49028;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $049028: jsr $49128(pc)
pub fn i_049028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4902C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4902C);
    cpu.pc = 0x49128;
    16
}
/// $04902C: bcc.w $4904e
pub fn i_04902c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49030;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4904E; return 10; }
    8
}
/// $049030: tst.b $98(a6)
pub fn i_049030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49034;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $049034: beq.w $49048
pub fn i_049034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49038;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x49048; return 10; }
    8
}
/// $049038: jsr $4939c(pc)
pub fn i_049038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4903C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4903C);
    cpu.pc = 0x4939C;
    16
}
/// $04903C: bcc.w $49048
pub fn i_04903c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49040;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49048; return 10; }
    8
}
/// $049040: jsr $4914c(pc)
pub fn i_049040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49044;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x49044);
    cpu.pc = 0x4914C;
    16
}
/// $049044: bcc.w $4904e
pub fn i_049044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49048;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4904E; return 10; }
    8
}
// ===== SetXN_049048  ($049048, 6 B, thunks/ccr_helpers.c)
/// $049048: ori.b #$11, ccr
pub fn i_049048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4904C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04904C: rts
pub fn i_04904c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4904E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04904e  ($04904E, 6 B, thunks/ccr_helpers.c)
/// $04904E: andi.b #$ee, ccr
pub fn i_04904e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49052;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $049052: rts
pub fn i_049052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49054;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TiedTurnTowardTarget_049054  ($049054, 136 B, pow/helpers_048a44.s)
/// $049054: movea.l $94(a6), a0
pub fn i_049054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49058;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $049058: jsr $5e070.l
pub fn i_049058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4905E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4905E);
    cpu.pc = 0x5E070;
    16
}
/// $04905E: addq.w #$8, d0
pub fn i_04905e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49060;
    cpu.ir = 0x5040;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $049060: lsr.w #$4, d0
pub fn i_049060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49062;
    cpu.ir = 0xE848;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $049062: andi.w #$f, d0
pub fn i_049062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49066;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $049066: tst.w d0
pub fn i_049066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49068;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $049068: beq.w $4909a
pub fn i_049068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4906C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4909A; return 10; }
    8
}
/// $04906C: move.w #$1, d1
pub fn i_04906c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49070;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $049070: subq.w #$4, d0
pub fn i_049070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49072;
    cpu.ir = 0x5940;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $049072: bcc.w $49084
pub fn i_049072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49076;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49084; return 10; }
    8
}
/// $049076: cmpi.w #$10, $78(a6)
pub fn i_049076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4907C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $04907C: beq.w $490f4
pub fn i_04907c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49080;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x490F4; return 10; }
    8
}
/// $049080: bra.w $490e2
pub fn i_049080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49084;
    cpu.ir = 0x6000;
    cpu.pc = 0x490E2;
    10
}
/// $049084: move.w #$ffff, d1
pub fn i_049084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49088;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $049088: subq.w #$4, d0
pub fn i_049088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4908A;
    cpu.ir = 0x5940;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04908A: bcc.w $4909e
pub fn i_04908a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4908E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4909E; return 10; }
    8
}
/// $04908E: tst.w $78(a6)
pub fn i_04908e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49092;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $049092: beq.w $490f4
pub fn i_049092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49096;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x490F4; return 10; }
    8
}
/// $049096: bra.w $490e2
pub fn i_049096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4909A;
    cpu.ir = 0x6000;
    cpu.pc = 0x490E2;
    10
}
/// $04909A: move.w #$8, d0
pub fn i_04909a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4909E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04909E: add.w d0, d0
pub fn i_04909e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490A0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0490A0: cmp.w $78(a6), d0
pub fn i_0490a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490A4;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0490A4: beq.w $490ce
pub fn i_0490a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x490CE; return 10; }
    8
}
/// $0490A8: bgt.w $490bc
pub fn i_0490a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490AC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x490BC; return 10; }
    8
}
/// $0490AC: move.w #$ffff, d1
pub fn i_0490ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490B0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0490B0: tst.w $78(a6)
pub fn i_0490b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490B4;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0490B4: beq.w $490f4
pub fn i_0490b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x490F4; return 10; }
    8
}
/// $0490B8: bra.w $490e2
pub fn i_0490b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x490E2;
    10
}
/// $0490BC: move.w #$1, d1
pub fn i_0490bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0490C0: cmpi.w #$10, $78(a6)
pub fn i_0490c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490C6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $0490C6: beq.w $490f4
pub fn i_0490c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490CA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x490F4; return 10; }
    8
}
/// $0490CA: bra.w $490e2
pub fn i_0490ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x490E2;
    10
}
/// $0490CE: cmpi.w #$0, $72(a6)
pub fn i_0490ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490D4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0490D4: bgt.w $490f4
pub fn i_0490d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490D8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x490F4; return 10; }
    8
}
/// $0490D8: clr.w $72(a6)
pub fn i_0490d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490DC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetXN_0490dc  ($0490DC, 6 B, thunks/ccr_helpers.c)
/// $0490DC: ori.b #$11, ccr
pub fn i_0490dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490E0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0490E0: rts
pub fn i_0490e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TiedTurnStep_0490e2  ($0490E2, 18 B, pow/helpers_048a44.s)
/// $0490E2: move.b $106f28.l, d0
pub fn i_0490e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490E8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0490E8: andi.b #$3, d0
pub fn i_0490e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490EC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0490EC: bne.w $490f4
pub fn i_0490ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x490F4; return 10; }
    8
}
/// $0490F0: add.w d1, $78(a6)
pub fn i_0490f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490F4;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== ClearXN_0490f4  ($0490F4, 6 B, thunks/ccr_helpers.c)
/// $0490F4: andi.b #$ee, ccr
pub fn i_0490f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490F8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0490F8: rts
pub fn i_0490f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x490FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TargetInReach_0490fa  ($0490FA, 34 B, pow/helpers_048a44.s)
/// $0490FA: jsr $5e0d4.l
pub fn i_0490fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49100;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49100);
    cpu.pc = 0x5E0D4;
    16
}
/// $049100: bcs.w $4911c
pub fn i_049100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49104;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4911C; return 10; }
    8
}
/// $049104: jsr $49128(pc)
pub fn i_049104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49108;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x49108);
    cpu.pc = 0x49128;
    16
}
/// $049108: bcc.w $49122
pub fn i_049108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4910C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49122; return 10; }
    8
}
/// $04910C: jsr $4939c(pc)
pub fn i_04910c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49110;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x49110);
    cpu.pc = 0x4939C;
    16
}
/// $049110: bcc.w $4911c
pub fn i_049110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49114;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4911C; return 10; }
    8
}
/// $049114: jsr $4914c(pc)
pub fn i_049114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49118;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x49118);
    cpu.pc = 0x4914C;
    16
}
/// $049118: bcc.w $49122
pub fn i_049118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4911C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49122; return 10; }
    8
}
// ===== SetXN_04911c  ($04911C, 6 B, thunks/ccr_helpers.c)
/// $04911C: ori.b #$11, ccr
pub fn i_04911c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49120;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $049120: rts
pub fn i_049120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49122;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_049122  ($049122, 6 B, thunks/ccr_helpers.c)
/// $049122: andi.b #$ee, ccr
pub fn i_049122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49126;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $049126: rts
pub fn i_049126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49128;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TargetInBox_049128  ($049128, 36 B, pow/helpers_048a44.s)
/// $049128: move.b $99(a6), d0
pub fn i_049128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4912C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04912C: andi.w #$3, d0
pub fn i_04912c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49130;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $049130: lsl.w #$2, d0
pub fn i_049130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49132;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $049132: lea.l $2bfe3a.l, a1
pub fn i_049132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49138;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2BFE3A;
    20
}
/// $049138: movea.l (a1, d0.w), a1
pub fn i_049138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4913C;
    cpu.ir = 0x2271;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $04913C: movea.l $94(a6), a0
pub fn i_04913c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49140;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $049140: jsr $5e260.l
pub fn i_049140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49146;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49146);
    cpu.pc = 0x5E260;
    16
}
/// $049146: eori.b #$11, ccr
pub fn i_049146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4914A;
    cpu.ir = 0xA3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) ^ 0x11);
    20
}
/// $04914A: rts
pub fn i_04914a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4914C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TargetAngleInMask_04914c  ($04914C, 26 B, pow/helpers_048a44.s)
/// $04914C: movea.l $94(a6), a0
pub fn i_04914c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49150;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $049150: jsr $5e070.l
pub fn i_049150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49156;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49156);
    cpu.pc = 0x5E070;
    16
}
/// $049156: move.b d0, d1
pub fn i_049156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49158;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $049158: add.b $7e(a6), d1
pub fn i_049158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4915C;
    cpu.ir = 0xD22E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    12
}
/// $04915C: and.b $7d(a6), d1
pub fn i_04915c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49160;
    cpu.ir = 0xC22E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $049160: cmp.b d0, d1
pub fn i_049160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49162;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $049162: bne.w $4916c
pub fn i_049162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49166;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4916C; return 10; }
    8
}
// ===== SetXN_049166  ($049166, 6 B, thunks/ccr_helpers.c)
/// $049166: ori.b #$11, ccr
pub fn i_049166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4916A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04916A: rts
pub fn i_04916a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4916C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04916c  ($04916C, 6 B, thunks/ccr_helpers.c)
/// $04916C: andi.b #$ee, ccr
pub fn i_04916c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49170;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $049170: rts
pub fn i_049170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49172;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TargetFarX_049172  ($049172, 24 B, pow/helpers_048a44.s)
/// $049172: jsr $49128(pc)
pub fn i_049172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49176;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x49176);
    cpu.pc = 0x49128;
    16
}
/// $049176: bcs.w $4918a
pub fn i_049176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4917A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4918A; return 10; }
    8
}
/// $04917A: movea.l $94(a6), a0
pub fn i_04917a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4917E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04917E: jsr $49388(pc)
pub fn i_04917e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49182;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x49182);
    cpu.pc = 0x49388;
    16
}
/// $049182: cmpi.w #$c0, d0
pub fn i_049182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49186;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $049186: bgt.w $49190
pub fn i_049186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4918A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x49190; return 10; }
    8
}
// ===== ClearXN_04918a  ($04918A, 6 B, thunks/ccr_helpers.c)
/// $04918A: andi.b #$ee, ccr
pub fn i_04918a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4918E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04918E: rts
pub fn i_04918e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49190;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_049190  ($049190, 6 B, thunks/ccr_helpers.c)
/// $049190: ori.b #$11, ccr
pub fn i_049190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49194;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $049194: rts
pub fn i_049194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49196;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_ShouldRunAway_049196  ($049196, 60 B, pow/helpers_048a44.s)
/// $049196: jsr $492f8(pc)
pub fn i_049196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4919A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4919A);
    cpu.pc = 0x492F8;
    16
}
/// $04919A: bcs.w $491d8
pub fn i_04919a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4919E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x491D8; return 10; }
    8
}
/// $04919E: lea.l $28e1b6.l, a0
pub fn i_04919e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E1B6;
    20
}
/// $0491A4: jsr $5e086.l
pub fn i_0491a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x491AA);
    cpu.pc = 0x5E086;
    16
}
/// $0491AA: bcs.w $491d8
pub fn i_0491aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491AE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x491D8; return 10; }
    8
}
/// $0491AE: jsr $493c0(pc)
pub fn i_0491ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x491B2);
    cpu.pc = 0x493C0;
    16
}
/// $0491B2: bcs.w $491d8
pub fn i_0491b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491B6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x491D8; return 10; }
    8
}
/// $0491B6: jsr $49388(pc)
pub fn i_0491b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x491BA);
    cpu.pc = 0x49388;
    16
}
/// $0491BA: lea.l $2bfe6a.l, a5
pub fn i_0491ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491C0;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x2BFE6A;
    20
}
/// $0491C0: move.b $9a(a6), d5
pub fn i_0491c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491C4;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0491C4: andi.w #$3, d5
pub fn i_0491c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491C8;
    cpu.ir = 0x245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0491C8: lsl.w #$3, d5
pub fn i_0491c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491CA;
    cpu.ir = 0xE74D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0491CA: cmp.w (a5, d5.w), d0
pub fn i_0491ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491CE;
    cpu.ir = 0xB075;
    let ad2 = cpu.a[5].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $0491CE: bge.w $491d8
pub fn i_0491ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491D2;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x491D8; return 10; }
    8
}
// ===== SetXN_0491d2  ($0491D2, 6 B, thunks/ccr_helpers.c)
/// $0491D2: ori.b #$11, ccr
pub fn i_0491d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491D6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0491D6: rts
pub fn i_0491d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0491d8  ($0491D8, 6 B, thunks/ccr_helpers.c)
/// $0491D8: andi.b #$ee, ccr
pub fn i_0491d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491DC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0491DC: rts
pub fn i_0491dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_ShouldWait_0491de  ($0491DE, 52 B, pow/helpers_048a44.s)
/// $0491DE: lea.l $28e1b6.l, a0
pub fn i_0491de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E1B6;
    20
}
/// $0491E4: jsr $5e086.l
pub fn i_0491e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x491EA);
    cpu.pc = 0x5E086;
    16
}
/// $0491EA: bcs.w $49218
pub fn i_0491ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491EE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x49218; return 10; }
    8
}
/// $0491EE: jsr $493c0(pc)
pub fn i_0491ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x491F2);
    cpu.pc = 0x493C0;
    16
}
/// $0491F2: bcs.w $49218
pub fn i_0491f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491F6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x49218; return 10; }
    8
}
/// $0491F6: jsr $49388(pc)
pub fn i_0491f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x491FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x491FA);
    cpu.pc = 0x49388;
    16
}
/// $0491FA: lea.l $2bfe6a.l, a5
pub fn i_0491fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49200;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x2BFE6A;
    20
}
/// $049200: move.b $9a(a6), d5
pub fn i_049200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49204;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $049204: andi.w #$3, d5
pub fn i_049204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49208;
    cpu.ir = 0x245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $049208: lsl.w #$3, d5
pub fn i_049208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4920A;
    cpu.ir = 0xE74D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04920A: cmp.w $2(a5, d5.w), d0
pub fn i_04920a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4920E;
    cpu.ir = 0xB075;
    let ad2 = cpu.a[5].wrapping_add(0x2).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $04920E: bge.w $49218
pub fn i_04920e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49212;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x49218; return 10; }
    8
}
// ===== SetXN_049212  ($049212, 6 B, thunks/ccr_helpers.c)
/// $049212: ori.b #$11, ccr
pub fn i_049212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49216;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $049216: rts
pub fn i_049216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49218;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_049218  ($049218, 6 B, thunks/ccr_helpers.c)
/// $049218: andi.b #$ee, ccr
pub fn i_049218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4921C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04921C: rts
pub fn i_04921c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4921E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TargetNearX_04921e  ($04921E, 44 B, pow/helpers_048a44.s)
/// $04921E: lea.l $28e1b6.l, a0
pub fn i_04921e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49224;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E1B6;
    20
}
/// $049224: jsr $5e086.l
pub fn i_049224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4922A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4922A);
    cpu.pc = 0x5E086;
    16
}
/// $04922A: bcs.w $49250
pub fn i_04922a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4922E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x49250; return 10; }
    8
}
/// $04922E: jsr $49388(pc)
pub fn i_04922e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49232;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x49232);
    cpu.pc = 0x49388;
    16
}
/// $049232: lea.l $2bfe6a.l, a5
pub fn i_049232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49238;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x2BFE6A;
    20
}
/// $049238: move.b $9a(a6), d5
pub fn i_049238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4923C;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04923C: andi.w #$3, d5
pub fn i_04923c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49240;
    cpu.ir = 0x245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $049240: lsl.w #$3, d5
pub fn i_049240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49242;
    cpu.ir = 0xE74D;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $049242: cmp.w $4(a5, d5.w), d0
pub fn i_049242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49246;
    cpu.ir = 0xB075;
    let ad2 = cpu.a[5].wrapping_add(0x4).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $049246: bge.w $49250
pub fn i_049246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4924A;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x49250; return 10; }
    8
}
// ===== SetXN_04924a  ($04924A, 6 B, thunks/ccr_helpers.c)
/// $04924A: ori.b #$11, ccr
pub fn i_04924a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4924E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04924E: rts
pub fn i_04924e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49250;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_049250  ($049250, 6 B, thunks/ccr_helpers.c)
/// $049250: andi.b #$ee, ccr
pub fn i_049250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49254;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $049254: rts
pub fn i_049254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49256;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TurnTimerAndCheck_049256  ($049256, 20 B, pow/helpers_048a44.s)
/// $049256: addq.b #$1, $76(a6)
pub fn i_049256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4925A;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04925A: cmpi.b #$1e, $76(a6)
pub fn i_04925a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49260;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Byte, false);
    16
}
/// $049260: blt.w $4929e
pub fn i_049260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49264;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4929E; return 10; }
    8
}
/// $049264: move.b #$1e, $76(a6)
pub fn i_049264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4926A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Pow_ShouldTurn_04926a  ($04926A, 46 B, pow/helpers_048a44.s)
/// $04926A: lea.l $28e1a6.l, a0
pub fn i_04926a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49270;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E1A6;
    20
}
/// $049270: jsr $5e086.l
pub fn i_049270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49276;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49276);
    cpu.pc = 0x5E086;
    16
}
/// $049276: bcc.w $4929e
pub fn i_049276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4927A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4929E; return 10; }
    8
}
/// $04927A: lea.l $28e1ae.l, a0
pub fn i_04927a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49280;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E1AE;
    20
}
/// $049280: jsr $5e086.l
pub fn i_049280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49286;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49286);
    cpu.pc = 0x5E086;
    16
}
/// $049286: bcc.w $49298
pub fn i_049286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4928A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49298; return 10; }
    8
}
/// $04928A: movea.l $94(a6), a0
pub fn i_04928a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4928E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04928E: jsr $5e618.l
pub fn i_04928e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49294;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49294);
    cpu.pc = 0x5E618;
    16
}
/// $049294: bcc.w $4929e
pub fn i_049294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49298;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4929E; return 10; }
    8
}
// ===== SetXN_049298  ($049298, 6 B, thunks/ccr_helpers.c)
/// $049298: ori.b #$11, ccr
pub fn i_049298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4929C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04929C: rts
pub fn i_04929c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4929E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04929e  ($04929E, 6 B, thunks/ccr_helpers.c)
/// $04929E: andi.b #$ee, ccr
pub fn i_04929e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492A2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0492A2: rts
pub fn i_0492a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TargetWithin30_0492a4  ($0492A4, 30 B, pow/helpers_048a44.s)
/// $0492A4: jsr $5e0d4.l
pub fn i_0492a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x492AA);
    cpu.pc = 0x5E0D4;
    16
}
/// $0492AA: bcs.w $492c8
pub fn i_0492aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492AE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x492C8; return 10; }
    8
}
/// $0492AE: jsr $493c0(pc)
pub fn i_0492ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x492B2);
    cpu.pc = 0x493C0;
    16
}
/// $0492B2: bcs.w $492c8
pub fn i_0492b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492B6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x492C8; return 10; }
    8
}
/// $0492B6: jsr $49388(pc)
pub fn i_0492b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x492BA);
    cpu.pc = 0x49388;
    16
}
/// $0492BA: cmpi.w #$30, d0
pub fn i_0492ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492BE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $0492BE: bgt.w $492c8
pub fn i_0492be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492C2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x492C8; return 10; }
    8
}
// ===== SetXN_0492c2  ($0492C2, 6 B, thunks/ccr_helpers.c)
/// $0492C2: ori.b #$11, ccr
pub fn i_0492c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492C6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0492C6: rts
pub fn i_0492c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0492c8  ($0492C8, 6 B, thunks/ccr_helpers.c)
/// $0492C8: andi.b #$ee, ccr
pub fn i_0492c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492CC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0492CC: rts
pub fn i_0492cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TargetWithin60_0492ce  ($0492CE, 30 B, pow/helpers_048a44.s)
/// $0492CE: jsr $5e0d4.l
pub fn i_0492ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x492D4);
    cpu.pc = 0x5E0D4;
    16
}
/// $0492D4: bcs.w $492f2
pub fn i_0492d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492D8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x492F2; return 10; }
    8
}
/// $0492D8: jsr $493c0(pc)
pub fn i_0492d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x492DC);
    cpu.pc = 0x493C0;
    16
}
/// $0492DC: bcs.w $492f2
pub fn i_0492dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492E0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x492F2; return 10; }
    8
}
/// $0492E0: jsr $49388(pc)
pub fn i_0492e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x492E4);
    cpu.pc = 0x49388;
    16
}
/// $0492E4: cmpi.w #$60, d0
pub fn i_0492e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492E8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $0492E8: bgt.w $492f2
pub fn i_0492e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492EC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x492F2; return 10; }
    8
}
// ===== SetXN_0492ec  ($0492EC, 6 B, thunks/ccr_helpers.c)
/// $0492EC: ori.b #$11, ccr
pub fn i_0492ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492F0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0492F0: rts
pub fn i_0492f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0492f2  ($0492F2, 6 B, thunks/ccr_helpers.c)
/// $0492F2: andi.b #$ee, ccr
pub fn i_0492f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492F6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0492F6: rts
pub fn i_0492f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_AtScreenEdge_0492f8  ($0492F8, 40 B, pow/helpers_048a44.s)
/// $0492F8: cmpi.w #$20, $22(a6)
pub fn i_0492f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x492FE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $0492FE: bgt.w $4930c
pub fn i_0492fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49302;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4930C; return 10; }
    8
}
/// $049302: btst.b #$0, $7c(a6)
pub fn i_049302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49308;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $049308: beq.w $49326
pub fn i_049308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4930C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x49326; return 10; }
    8
}
/// $04930C: cmpi.w #$120, $22(a6)
pub fn i_04930c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49312;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $049312: blt.w $49320
pub fn i_049312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49316;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x49320; return 10; }
    8
}
/// $049316: btst.b #$0, $7c(a6)
pub fn i_049316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4931C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04931C: bne.w $49326
pub fn i_04931c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49320;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49326; return 10; }
    8
}
// ===== ClearXN_049320  ($049320, 6 B, thunks/ccr_helpers.c)
/// $049320: andi.b #$ee, ccr
pub fn i_049320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49324;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $049324: rts
pub fn i_049324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49326;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_049326  ($049326, 6 B, thunks/ccr_helpers.c)
/// $049326: ori.b #$11, ccr
pub fn i_049326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4932A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04932A: rts
pub fn i_04932a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4932C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_HitReceivedCheck_04932c  ($04932C, 10 B, pow/helpers_048a44.s)
/// $04932C: jsr $27eba.l
pub fn i_04932c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49332;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49332);
    cpu.pc = 0x27EBA;
    16
}
/// $049332: bcc.w $4933c
pub fn i_049332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49336;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4933C; return 10; }
    8
}
// ===== SetXN_049336  ($049336, 6 B, thunks/ccr_helpers.c)
/// $049336: ori.b #$11, ccr
pub fn i_049336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4933A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04933A: rts
pub fn i_04933a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4933C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_ClearHitFlag_04933c  ($04933C, 4 B, pow/helpers_048a44.s)
/// $04933C: clr.b $75(a6)
pub fn i_04933c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49340;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x75);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== ClearXN_049340  ($049340, 6 B, thunks/ccr_helpers.c)
/// $049340: andi.b #$ee, ccr
pub fn i_049340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49344;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $049344: rts
pub fn i_049344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49346;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_BlockedTimer_049346  ($049346, 24 B, pow/helpers_048a44.s)
/// $049346: btst.b #$5, $5a(a6)
pub fn i_049346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4934C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04934C: beq.w $49364
pub fn i_04934c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49350;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x49364; return 10; }
    8
}
/// $049350: addq.b #$1, $74(a6)
pub fn i_049350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49354;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $049354: cmpi.b #$1e, $74(a6)
pub fn i_049354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4935A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Byte, false);
    16
}
/// $04935A: blt.w $49364
pub fn i_04935a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4935E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x49364; return 10; }
    8
}
// ===== SetXN_04935e  ($04935E, 6 B, thunks/ccr_helpers.c)
/// $04935E: ori.b #$11, ccr
pub fn i_04935e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49362;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $049362: rts
pub fn i_049362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49364;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_BlockedTimerReset_049364  ($049364, 4 B, pow/helpers_048a44.s)
/// $049364: clr.b $74(a6)
pub fn i_049364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49368;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== ClearXN_049368  ($049368, 6 B, thunks/ccr_helpers.c)
/// $049368: andi.b #$ee, ccr
pub fn i_049368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4936C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04936C: rts
pub fn i_04936c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4936E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_RetargetIfLost_04936e  ($04936E, 26 B, pow/helpers_048a44.s)
/// $04936E: movea.l $94(a6), a0
pub fn i_04936e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49372;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $049372: jsr $5e338.l
pub fn i_049372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49378;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49378);
    cpu.pc = 0x5E338;
    16
}
/// $049378: bcc.w $49386
pub fn i_049378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4937C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49386; return 10; }
    8
}
/// $04937C: jsr $5e1ea.l
pub fn i_04937c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49382;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49382);
    cpu.pc = 0x5E1EA;
    16
}
/// $049382: move.l a0, $94(a6)
pub fn i_049382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49386;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $049386: rts
pub fn i_049386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49388;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_AbsDistX_049388  ($049388, 20 B, pow/helpers_048a44.s)
/// $049388: move.w $22(a6), d0
pub fn i_049388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4938C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04938C: sub.w $22(a0), d0
pub fn i_04938c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49390;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $049390: cmpi.w #$0, d0
pub fn i_049390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49394;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $049394: bge.w $4939a
pub fn i_049394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49398;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x4939A; return 10; }
    8
}
/// $049398: neg.w d0
pub fn i_049398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4939A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04939A: rts
pub fn i_04939a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4939C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TargetYNear_04939c  ($04939C, 24 B, pow/helpers_048a44.s)
/// $04939C: move.w $24(a0), d0
pub fn i_04939c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493A0;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0493A0: sub.w $24(a6), d0
pub fn i_0493a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493A4;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0493A4: cmpi.w #$18, d0
pub fn i_0493a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493A8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x18, Size::Word, false);
    8
}
/// $0493A8: bgt.w $493ba
pub fn i_0493a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493AC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x493BA; return 10; }
    8
}
/// $0493AC: cmpi.w #$fff0, d0
pub fn i_0493ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493B0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFF0, Size::Word, false);
    8
}
/// $0493B0: blt.w $493ba
pub fn i_0493b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493B4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x493BA; return 10; }
    8
}
// ===== ClearXN_0493b4  ($0493B4, 6 B, thunks/ccr_helpers.c)
/// $0493B4: andi.b #$ee, ccr
pub fn i_0493b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493B8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0493B8: rts
pub fn i_0493b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0493ba  ($0493BA, 6 B, thunks/ccr_helpers.c)
/// $0493BA: ori.b #$11, ccr
pub fn i_0493ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493BE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0493BE: rts
pub fn i_0493be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TargetYNearB_0493c0  ($0493C0, 24 B, pow/helpers_048a44.s)
/// $0493C0: move.w $24(a0), d0
pub fn i_0493c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493C4;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0493C4: sub.w $24(a6), d0
pub fn i_0493c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493C8;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0493C8: cmpi.w #$18, d0
pub fn i_0493c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493CC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x18, Size::Word, false);
    8
}
/// $0493CC: bgt.w $493de
pub fn i_0493cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493D0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x493DE; return 10; }
    8
}
/// $0493D0: cmpi.w #$fff0, d0
pub fn i_0493d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493D4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFF0, Size::Word, false);
    8
}
/// $0493D4: blt.w $493de
pub fn i_0493d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493D8;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x493DE; return 10; }
    8
}
// ===== ClearXN_0493d8  ($0493D8, 6 B, thunks/ccr_helpers.c)
/// $0493D8: andi.b #$ee, ccr
pub fn i_0493d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493DC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0493DC: rts
pub fn i_0493dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0493de  ($0493DE, 6 B, thunks/ccr_helpers.c)
/// $0493DE: ori.b #$11, ccr
pub fn i_0493de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493E2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0493E2: rts
pub fn i_0493e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_SpawnFxFromTurnAngle_0493e4  ($0493E4, 42 B, pow/helpers_048a44.s)
/// $0493E4: lea.l $28e2d2.l, a0
pub fn i_0493e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E2D2;
    20
}
/// $0493EA: move.w $78(a6), d0
pub fn i_0493ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493EE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0493EE: lsr.w #$1, d0
pub fn i_0493ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493F0;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0493F0: addq.w #$8, d0
pub fn i_0493f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493F2;
    cpu.ir = 0x5040;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0493F2: move.b d0, $7b(a6)
pub fn i_0493f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493F6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0493F6: bra.w $49418
pub fn i_0493f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x493FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x49418;
    10
}
// ===== Pow_SpawnFxByDir_04940e  ($04940E, 34 B, pow/helpers_048a44.s)
/// $04940E: lea.l $28e292.l, a0
pub fn i_04940e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49414;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E292;
    20
}
/// $049414: move.b $7b(a6), d0
pub fn i_049414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49418;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $049418: andi.l #$f, d0
pub fn i_049418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4941E;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $04941E: lsl.w #$2, d0
pub fn i_04941e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49420;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $049420: add.l a0, d0
pub fn i_049420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49422;
    cpu.ir = 0xD088;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $049422: move.l d0, $5c(a6)
pub fn i_049422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49426;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $049426: lea.l $48b56(pc), a1
pub fn i_049426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4942A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B56;
    16
}
/// $04942A: jsr $4ae.l
pub fn i_04942a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49430);
    cpu.pc = 0x4AE;
    16
}
// ===== PowHang_SpawnItemAndFxAtSprite_049430  ($049430, 66 B, pow/hang_049430.s)
/// $049430: jsr $5dd02.l
pub fn i_049430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49436);
    cpu.pc = 0x5DD02;
    16
}
/// $049436: movea.l $5c(a6), a1
pub fn i_049436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4943A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04943A: move.w (a1), d0
pub fn i_04943a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4943C;
    cpu.ir = 0x3011;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $04943C: move.w $2(a1), d1
pub fn i_04943c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49440;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $049440: add.w d0, $22(a0)
pub fn i_049440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49444;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $049444: add.w d1, $24(a0)
pub fn i_049444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49448;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $049448: lea.l $48bca(pc), a1
pub fn i_049448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4944C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48BCA;
    16
}
/// $04944C: jsr $4ae.l
pub fn i_04944c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49452);
    cpu.pc = 0x4AE;
    16
}
/// $049452: jsr $5dd02.l
pub fn i_049452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49458;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49458);
    cpu.pc = 0x5DD02;
    16
}
/// $049458: movea.l $5c(a6), a1
pub fn i_049458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4945C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04945C: move.w (a1), d0
pub fn i_04945c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4945E;
    cpu.ir = 0x3011;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $04945E: move.w $2(a1), d1
pub fn i_04945e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49462;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $049462: add.w d0, $22(a0)
pub fn i_049462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49466;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $049466: add.w d1, $24(a0)
pub fn i_049466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4946A;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04946A: move.b $7b(a6), $7b(a0)
pub fn i_04946a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49470;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x7B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $049470: rts
pub fn i_049470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49472;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_SpawnFxAndItemA_049472  ($049472, 86 B, pow/hang_049430.s)
/// $049472: lea.l $48b56(pc), a1
pub fn i_049472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49476;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B56;
    16
}
/// $049476: jsr $4ae.l
pub fn i_049476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4947C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4947C);
    cpu.pc = 0x4AE;
    16
}
/// $04947C: jsr $5dd02.l
pub fn i_04947c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49482;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49482);
    cpu.pc = 0x5DD02;
    16
}
/// $049482: move.w #$20, d0
pub fn i_049482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49486;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $049486: btst.b #$0, $3a(a6)
pub fn i_049486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4948C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04948C: bne.w $49492
pub fn i_04948c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49490;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49492; return 10; }
    8
}
/// $049490: neg.w d0
pub fn i_049490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49492;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $049492: add.w d0, $22(a0)
pub fn i_049492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49496;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $049496: addi.w #$e, $24(a0)
pub fn i_049496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4949C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xE, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04949C: lea.l $48ba0(pc), a1
pub fn i_04949c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48BA0;
    16
}
/// $0494A0: jsr $4ae.l
pub fn i_0494a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x494A6);
    cpu.pc = 0x4AE;
    16
}
/// $0494A6: jsr $5dd02.l
pub fn i_0494a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x494AC);
    cpu.pc = 0x5DD02;
    16
}
/// $0494AC: move.w #$20, d0
pub fn i_0494ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494B0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0494B0: btst.b #$0, $3a(a6)
pub fn i_0494b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494B6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0494B6: bne.w $494bc
pub fn i_0494b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494BA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x494BC; return 10; }
    8
}
/// $0494BA: neg.w d0
pub fn i_0494ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494BC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0494BC: add.w d0, $22(a0)
pub fn i_0494bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494C0;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0494C0: addi.w #$e, $24(a0)
pub fn i_0494c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494C6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xE, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0494C6: rts
pub fn i_0494c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_SpawnFxAndItemB_0494c8  ($0494C8, 86 B, pow/hang_049430.s)
/// $0494C8: lea.l $48b56(pc), a1
pub fn i_0494c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B56;
    16
}
/// $0494CC: jsr $4ae.l
pub fn i_0494cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x494D2);
    cpu.pc = 0x4AE;
    16
}
/// $0494D2: jsr $5dd02.l
pub fn i_0494d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x494D8);
    cpu.pc = 0x5DD02;
    16
}
/// $0494D8: move.w #$1e, d0
pub fn i_0494d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494DC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0494DC: btst.b #$0, $3a(a6)
pub fn i_0494dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494E2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0494E2: bne.w $494e8
pub fn i_0494e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x494E8; return 10; }
    8
}
/// $0494E6: neg.w d0
pub fn i_0494e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494E8;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0494E8: add.w d0, $22(a0)
pub fn i_0494e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494EC;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0494EC: addi.w #$22, $24(a0)
pub fn i_0494ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494F2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x22, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0494F2: lea.l $48ba0(pc), a1
pub fn i_0494f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48BA0;
    16
}
/// $0494F6: jsr $4ae.l
pub fn i_0494f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x494FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x494FC);
    cpu.pc = 0x4AE;
    16
}
/// $0494FC: jsr $5dd02.l
pub fn i_0494fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49502);
    cpu.pc = 0x5DD02;
    16
}
/// $049502: move.w #$1e, d0
pub fn i_049502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49506;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $049506: btst.b #$0, $3a(a6)
pub fn i_049506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4950C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04950C: bne.w $49512
pub fn i_04950c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49510;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49512; return 10; }
    8
}
/// $049510: neg.w d0
pub fn i_049510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49512;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $049512: add.w d0, $22(a0)
pub fn i_049512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49516;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $049516: addi.w #$22, $24(a0)
pub fn i_049516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4951C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x22, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04951C: rts
pub fn i_04951c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4951E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_LoadTimerAndSlotCheck_04951e  ($04951E, 32 B, pow/hang_049430.s)
/// $04951E: lea.l $2bfcb4.l, a0
pub fn i_04951e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49524;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFCB4;
    20
}
/// $049524: jsr $799de.l
pub fn i_049524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4952A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4952A);
    cpu.pc = 0x799DE;
    16
}
/// $04952A: move.w d0, $72(a6)
pub fn i_04952a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4952E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04952E: movea.l $8(a6), a1
pub fn i_04952e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49532;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $049532: move.b $10(a6), d0
pub fn i_049532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49536;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $049536: cmp.b $10(a1), d0
pub fn i_049536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4953A;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04953A: bcs.w $49544
pub fn i_04953a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4953E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x49544; return 10; }
    8
}
// ===== ClearXN_04953e  ($04953E, 6 B, thunks/ccr_helpers.c)
/// $04953E: andi.b #$ee, ccr
pub fn i_04953e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49542;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $049542: rts
pub fn i_049542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49544;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_049544  ($049544, 6 B, thunks/ccr_helpers.c)
/// $049544: ori.b #$11, ccr
pub fn i_049544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49548;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $049548: rts
pub fn i_049548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4954A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_SpawnVariants_04954a  ($04954A, 140 B, pow/hang_049430.s)
/// $04954A: move.b #$0, $7f(a6)
pub fn i_04954a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49550;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049550: bra.w $4955a
pub fn i_049550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49554;
    cpu.ir = 0x6000;
    cpu.pc = 0x4955A;
    10
}
/// $04955A: move.w #$38, d1
pub fn i_04955a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4955E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04955E: jsr $236e.l
pub fn i_04955e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49564);
    cpu.pc = 0x236E;
    16
}
/// $049564: move.w #$1, $66(a6)
pub fn i_049564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4956A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04956A: move.b #$3, $70(a6)
pub fn i_04956a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49570;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049570: move.w #$c, $34(a6)
pub fn i_049570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49576;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $049576: move.w #$8000, $38(a6)
pub fn i_049576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4957C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04957C: jsr $267e2.l
pub fn i_04957c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49582;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49582);
    cpu.pc = 0x267E2;
    16
}
/// $049582: jsr $27cee.l
pub fn i_049582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49588;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49588);
    cpu.pc = 0x27CEE;
    16
}
/// $049588: lea.l $28ff2a.l, a0
pub fn i_049588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4958E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28FF2A;
    20
}
/// $04958E: move.l a0, $48(a6)
pub fn i_04958e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49592;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $049592: lea.l $2bff0c.l, a0
pub fn i_049592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49598;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFF0C;
    20
}
/// $049598: jsr $799de.l
pub fn i_049598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4959E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4959E);
    cpu.pc = 0x799DE;
    16
}
/// $04959E: move.w d0, $76(a6)
pub fn i_04959e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495A2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0495A2: lea.l $2bff8e.l, a0
pub fn i_0495a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFF8E;
    20
}
/// $0495A8: jsr $799de.l
pub fn i_0495a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x495AE);
    cpu.pc = 0x799DE;
    16
}
/// $0495AE: move.w d0, $7c(a6)
pub fn i_0495ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495B2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0495B2: clr.w d0
pub fn i_0495b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495B4;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0495B4: move.b $9e(a6), d0
pub fn i_0495b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495B8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0495B8: ext.w d0
pub fn i_0495b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495BA;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0495BA: asl.w #$4, d0
pub fn i_0495ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495BC;
    cpu.ir = 0xE940;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0495BC: move.w d0, $7a(a6)
pub fn i_0495bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495C0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0495C0: move.b #$0, $79(a6)
pub fn i_0495c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495C6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0495C6: lea.l $497ac(pc), a1
pub fn i_0495c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x497AC;
    16
}
/// $0495CA: jsr $4ae.l
pub fn i_0495ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x495D0);
    cpu.pc = 0x4AE;
    16
}
/// $0495D0: jsr $5dd02.l
pub fn i_0495d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x495D6);
    cpu.pc = 0x5DD02;
    16
}
// ===== PowHang_Swing_0495d6  ($0495D6, 180 B, pow/hang_049430.s)
/// $0495D6: clr.l d0
pub fn i_0495d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495D8;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0495D8: move.w $34(a6), d0
pub fn i_0495d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495DC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0495DC: andi.w #$f, d0
pub fn i_0495dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495E0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0495E0: movea.l #$28f708, a0
pub fn i_0495e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495E6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28F708;
    cpu.a[0] = v1;
    4
}
/// $0495E6: lsl.w #$2, d0
pub fn i_0495e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495E8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0495E8: movea.l (a0, d0.w), a0
pub fn i_0495e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495EC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0495EC: cmpa.l #$ffffffff, a0
pub fn i_0495ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495F2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0495F2: beq.w $495fc
pub fn i_0495f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x495FC; return 10; }
    8
}
/// $0495F6: jsr $28cd4.l
pub fn i_0495f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x495FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x495FC);
    cpu.pc = 0x28CD4;
    16
}
/// $0495FC: move.b #$0, $78(a6)
pub fn i_0495fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49602;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049602: lea.l $49608(pc), a1
pub fn i_049602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49606;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49608;
    16
}
/// $049606: move.l a1, (a6)
pub fn i_049606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49608;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049608: jsr $499dc(pc)
pub fn i_049608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4960C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4960C);
    cpu.pc = 0x499DC;
    16
}
/// $04960C: jsr $49a20(pc)
pub fn i_04960c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49610;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x49610);
    cpu.pc = 0x49A20;
    16
}
/// $049610: jsr $49b58(pc)
pub fn i_049610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49614;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x49614);
    cpu.pc = 0x49B58;
    16
}
/// $049614: bcc.w $49622
pub fn i_049614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49618;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49622; return 10; }
    8
}
/// $049618: move.b #$fd, $78(a6)
pub fn i_049618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4961E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFD;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04961E: bra.w $4962c
pub fn i_04961e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49622;
    cpu.ir = 0x6000;
    cpu.pc = 0x4962C;
    10
}
/// $049622: cmpi.b #$ff, $79(a6)
pub fn i_049622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49628;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $049628: bne.w $4964c
pub fn i_049628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4962C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4964C; return 10; }
    8
}
/// $04962C: move.w #$c, d0
pub fn i_04962c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49630;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $049630: sub.w $34(a6), d0
pub fn i_049630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49634;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $049634: asr.w #$8, d0
pub fn i_049634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49636;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $049636: asr.w #$7, d0
pub fn i_049636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49638;
    cpu.ir = 0xEE40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $049638: andi.b #$1, d0
pub fn i_049638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4963C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04963C: move.b d0, $3a(a6)
pub fn i_04963c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49640;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049640: lea.l $4797c.l, a1
pub fn i_049640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49646;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4797C;
    20
}
/// $049646: move.l a1, (a6)
pub fn i_049646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49648;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049648: bra.w $49688
pub fn i_049648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4964C;
    cpu.ir = 0x6000;
    cpu.pc = 0x49688;
    10
}
/// $04964C: jsr $49b06(pc)
pub fn i_04964c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49650;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x49650);
    cpu.pc = 0x49B06;
    16
}
/// $049650: jsr $2870a.l
pub fn i_049650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49656);
    cpu.pc = 0x2870A;
    16
}
/// $049656: bcc.w $49666
pub fn i_049656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4965A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49666; return 10; }
    8
}
/// $04965A: move.b #$ff, $78(a6)
pub fn i_04965a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49660;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049660: lea.l $49742(pc), a1
pub fn i_049660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49664;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49742;
    16
}
/// $049664: move.l a1, (a6)
pub fn i_049664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49666;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049666: movea.l #$ffffffff, a0
pub fn i_049666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4966C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04966C: lea.l $28ffd2.l, a0
pub fn i_04966c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49672;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28FFD2;
    20
}
/// $049672: jsr $5dd5c.l
pub fn i_049672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49678;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49678);
    cpu.pc = 0x5DD5C;
    16
}
/// $049678: bcc.w $49688
pub fn i_049678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4967C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49688; return 10; }
    8
}
/// $04967C: move.b #$fe, $78(a6)
pub fn i_04967c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49682;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049682: jmp $518.l
pub fn i_049682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49688;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $049688: rts
pub fn i_049688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4968A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_Freed_049742  ($049742, 106 B, pow/hang_049430.s)
/// $049742: move.w #$c, d0
pub fn i_049742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49746;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $049746: sub.w $34(a6), d0
pub fn i_049746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4974A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04974A: asr.w #$8, d0
pub fn i_04974a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4974C;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $04974C: asr.w #$7, d0
pub fn i_04974c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4974E;
    cpu.ir = 0xEE40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $04974E: andi.b #$1, d0
pub fn i_04974e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49752;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $049752: move.b d0, $3a(a6)
pub fn i_049752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49756;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049756: lea.l $4acfe.l, a0
pub fn i_049756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4975C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x4ACFE;
    20
}
/// $04975C: jsr $28cd4.l
pub fn i_04975c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49762;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49762);
    cpu.pc = 0x28CD4;
    16
}
/// $049762: lea.l $48ca4.l, a1
pub fn i_049762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49768;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x48CA4;
    20
}
/// $049768: jsr $4ae.l
pub fn i_049768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4976E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4976E);
    cpu.pc = 0x4AE;
    16
}
/// $04976E: jsr $5dd02.l
pub fn i_04976e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49774;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49774);
    cpu.pc = 0x5DD02;
    16
}
/// $049774: addi.w #$18, $24(a0)
pub fn i_049774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4977A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04977A: move.b $3a(a6), $3a(a0)
pub fn i_04977a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49780;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $049780: movea.l $50(a6), a1
pub fn i_049780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49784;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $049784: move.w $28(a1), d0
pub fn i_049784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49788;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $049788: asr.w #$4, d0
pub fn i_049788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4978A;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $04978A: move.w d0, $28(a0)
pub fn i_04978a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4978E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04978E: lea.l $49794(pc), a1
pub fn i_04978e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49792;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49794;
    16
}
/// $049792: move.l a1, (a6)
pub fn i_049792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49794;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049794: jsr $2783a.l
pub fn i_049794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4979A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4979A);
    cpu.pc = 0x2783A;
    16
}
/// $04979A: jsr $28d70.l
pub fn i_04979a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x497A0);
    cpu.pc = 0x28D70;
    16
}
/// $0497A0: bcc.w $497aa
pub fn i_0497a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497A4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x497AA; return 10; }
    8
}
/// $0497A4: jmp $518.l
pub fn i_0497a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497AA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0497AA: rts
pub fn i_0497aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_RopeSpawn_0497ac  ($0497AC, 36 B, pow/hang_049430.s)
/// $0497AC: move.w #$18d, d1
pub fn i_0497ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497B0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x18D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0497B0: jsr $236e.l
pub fn i_0497b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x497B6);
    cpu.pc = 0x236E;
    16
}
/// $0497B6: lea.l $2bfe8a.l, a0
pub fn i_0497b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFE8A;
    20
}
/// $0497BC: jsr $799de.l
pub fn i_0497bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x497C2);
    cpu.pc = 0x799DE;
    16
}
/// $0497C2: move.w d0, $66(a6)
pub fn i_0497c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497C6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0497C6: lea.l $28ff7e.l, a0
pub fn i_0497c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28FF7E;
    20
}
/// $0497CC: move.l a0, $48(a6)
pub fn i_0497cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497D0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== PowHang_RopeIdle_0497d0  ($0497D0, 180 B, pow/hang_049430.s)
/// $0497D0: lea.l $497d6(pc), a1
pub fn i_0497d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x497D6;
    16
}
/// $0497D4: move.l a1, (a6)
pub fn i_0497d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0497D6: jsr $5e4ee.l
pub fn i_0497d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x497DC);
    cpu.pc = 0x5E4EE;
    16
}
/// $0497DC: subq.w #$1, $38(a6)
pub fn i_0497dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497E0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0497E0: clr.l d0
pub fn i_0497e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497E2;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0497E2: movea.l $c(a6), a0
pub fn i_0497e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497E6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0497E6: move.b $71(a0), d0
pub fn i_0497e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497EA;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0497EA: movea.l #$28f8d4, a0
pub fn i_0497ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497F0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28F8D4;
    cpu.a[0] = v1;
    4
}
/// $0497F0: lsl.w #$2, d0
pub fn i_0497f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497F2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0497F2: movea.l (a0, d0.w), a0
pub fn i_0497f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497F6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0497F6: cmpa.l #$ffffffff, a0
pub fn i_0497f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x497FC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0497FC: beq.w $49806
pub fn i_0497fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49800;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x49806; return 10; }
    8
}
/// $049800: jsr $28cd4.l
pub fn i_049800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49806);
    cpu.pc = 0x28CD4;
    16
}
/// $049806: jsr $28d70.l
pub fn i_049806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4980C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4980C);
    cpu.pc = 0x28D70;
    16
}
/// $04980C: jsr $28758.l
pub fn i_04980c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49812;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49812);
    cpu.pc = 0x28758;
    16
}
/// $049812: bcc.w $4982a
pub fn i_049812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49816;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4982A; return 10; }
    8
}
/// $049816: movea.l $c(a6), a0
pub fn i_049816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4981A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04981A: move.b #$ff, $79(a0)
pub fn i_04981a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49820;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049820: lea.l $49976(pc), a1
pub fn i_049820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49824;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49976;
    16
}
/// $049824: move.l a1, (a6)
pub fn i_049824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49826;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049826: bra.w $49882
pub fn i_049826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4982A;
    cpu.ir = 0x6000;
    cpu.pc = 0x49882;
    10
}
/// $04982A: movea.l $c(a6), a0
pub fn i_04982a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4982E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04982E: cmpi.b #$1, $78(a0)
pub fn i_04982e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49834;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $049834: bne.w $49842
pub fn i_049834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49838;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49842; return 10; }
    8
}
/// $049838: lea.l $49884(pc), a1
pub fn i_049838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4983C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49884;
    16
}
/// $04983C: move.l a1, (a6)
pub fn i_04983c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4983E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04983E: bra.w $49882
pub fn i_04983e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49842;
    cpu.ir = 0x6000;
    cpu.pc = 0x49882;
    10
}
/// $049842: cmpi.b #$fd, $78(a0)
pub fn i_049842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49848;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFD, Size::Byte, false);
    16
}
/// $049848: bne.w $4985a
pub fn i_049848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4984C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4985A; return 10; }
    8
}
/// $04984C: move.l a6, $c(a6)
pub fn i_04984c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49850;
    cpu.ir = 0x2D4E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $049850: lea.l $4992e(pc), a1
pub fn i_049850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49854;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4992E;
    16
}
/// $049854: move.l a1, (a6)
pub fn i_049854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49856;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049856: bra.w $49882
pub fn i_049856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4985A;
    cpu.ir = 0x6000;
    cpu.pc = 0x49882;
    10
}
/// $04985A: cmpi.b #$ff, $78(a0)
pub fn i_04985a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49860;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $049860: bne.w $49872
pub fn i_049860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49864;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49872; return 10; }
    8
}
/// $049864: move.l a6, $c(a6)
pub fn i_049864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49868;
    cpu.ir = 0x2D4E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $049868: lea.l $49976(pc), a1
pub fn i_049868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4986C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49976;
    16
}
/// $04986C: move.l a1, (a6)
pub fn i_04986c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4986E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04986E: bra.w $49882
pub fn i_04986e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49872;
    cpu.ir = 0x6000;
    cpu.pc = 0x49882;
    10
}
/// $049872: cmpi.b #$fe, $78(a0)
pub fn i_049872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49878;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    16
}
/// $049878: bne.w $49882
pub fn i_049878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4987C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49882; return 10; }
    8
}
/// $04987C: jmp $518.l
pub fn i_04987c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49882;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $049882: rts
pub fn i_049882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49884;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_RopeCut_04992e  ($04992E, 72 B, pow/hang_049430.s)
/// $04992E: movea.l $c(a6), a0
pub fn i_04992e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49932;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $049932: move.b $3a(a0), $3a(a6)
pub fn i_049932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49938;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $049938: lea.l $28fe2c.l, a0
pub fn i_049938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4993E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28FE2C;
    20
}
/// $04993E: jsr $28cd4.l
pub fn i_04993e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49944;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49944);
    cpu.pc = 0x28CD4;
    16
}
/// $049944: lea.l $4994a(pc), a1
pub fn i_049944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49948;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4994A;
    16
}
/// $049948: move.l a1, (a6)
pub fn i_049948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4994A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04994A: jsr $2783a.l
pub fn i_04994a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49950;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49950);
    cpu.pc = 0x2783A;
    16
}
/// $049950: jsr $28d70.l
pub fn i_049950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49956;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49956);
    cpu.pc = 0x28D70;
    16
}
/// $049956: bcc.w $49960
pub fn i_049956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4995A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49960; return 10; }
    8
}
/// $04995A: jmp $518.l
pub fn i_04995a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49960;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $049960: movea.l $c(a6), a0
pub fn i_049960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49964;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $049964: cmpi.b #$fe, $78(a0)
pub fn i_049964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4996A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    16
}
/// $04996A: bne.w $49974
pub fn i_04996a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4996E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49974; return 10; }
    8
}
/// $04996E: jmp $518.l
pub fn i_04996e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49974;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $049974: rts
pub fn i_049974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49976;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_RopeBroken_049976  ($049976, 102 B, pow/hang_049430.s)
/// $049976: movea.l $c(a6), a0
pub fn i_049976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4997A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04997A: move.b $3a(a0), $3a(a6)
pub fn i_04997a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49980;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $049980: move.b #$0, $7e(a6)
pub fn i_049980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49986;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049986: lea.l $28fde0.l, a0
pub fn i_049986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4998C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28FDE0;
    20
}
/// $04998C: jsr $28cd4.l
pub fn i_04998c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49992;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49992);
    cpu.pc = 0x28CD4;
    16
}
/// $049992: lea.l $49998(pc), a1
pub fn i_049992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49996;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49998;
    16
}
/// $049996: move.l a1, (a6)
pub fn i_049996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49998;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049998: jsr $2783a.l
pub fn i_049998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4999E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4999E);
    cpu.pc = 0x2783A;
    16
}
/// $04999E: cmpi.b #$0, $7e(a6)
pub fn i_04999e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499A4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0499A4: beq.w $499b6
pub fn i_0499a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x499B6; return 10; }
    8
}
/// $0499A8: move.b $106f28.l, d0
pub fn i_0499a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499AE;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0499AE: btst.b #$0, d0
pub fn i_0499ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499B2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0499B2: bne.w $499bc
pub fn i_0499b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x499BC; return 10; }
    8
}
/// $0499B6: jsr $28d70.l
pub fn i_0499b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x499BC);
    cpu.pc = 0x28D70;
    16
}
/// $0499BC: bcc.w $499c6
pub fn i_0499bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499C0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x499C6; return 10; }
    8
}
/// $0499C0: jmp $518.l
pub fn i_0499c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499C6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0499C6: movea.l $c(a6), a0
pub fn i_0499c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499CA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0499CA: cmpi.b #$fe, $78(a0)
pub fn i_0499ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499D0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    16
}
/// $0499D0: bne.w $499da
pub fn i_0499d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x499DA; return 10; }
    8
}
/// $0499D4: jmp $518.l
pub fn i_0499d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0499DA: rts
pub fn i_0499da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_SwingStep_0499dc  ($0499DC, 60 B, pow/hang_049430.s)
/// $0499DC: subi.w #$2, $74(a6)
pub fn i_0499dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499E2;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0499E2: andi.w #$ff, $74(a6)
pub fn i_0499e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499E8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0499E8: move.w $74(a6), d0
pub fn i_0499e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499EC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0499EC: move.w #$b0, d1
pub fn i_0499ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0499F0: jsr $13c0e.l
pub fn i_0499f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x499F6);
    cpu.pc = 0x13C0E;
    16
}
/// $0499F6: add.w $7a(a6), d1
pub fn i_0499f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499FA;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0499FA: move.w d1, $28(a6)
pub fn i_0499fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x499FE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0499FE: asr.w #$1, d2
pub fn i_0499fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A00;
    cpu.ir = 0xE242;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $049A00: btst.b #$f, d2
pub fn i_049a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A04;
    cpu.ir = 0x802;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $049A04: bne.w $49a0a
pub fn i_049a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A08;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49A0A; return 10; }
    8
}
/// $049A08: neg.w d2
pub fn i_049a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A0A;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $049A0A: addi.w #$20, d2
pub fn i_049a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A0E;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $049A0E: move.w $7c(a6), d0
pub fn i_049a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A12;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $049A12: sub.w d0, d2
pub fn i_049a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A14;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $049A14: move.w d2, $2a(a6)
pub fn i_049a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A18;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_049a18  ($049A18, 8 B, thunks/jsr_abs_thunks.c)
/// $049A18: jsr $27cee.l
pub fn i_049a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49A1E);
    cpu.pc = 0x27CEE;
    16
}
/// $049A1E: rts
pub fn i_049a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_SwingAnimByAngle_049a20  ($049A20, 222 B, pow/hang_049430.s)
/// $049A20: move.w $34(a6), $72(a6)
pub fn i_049a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A26;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $049A26: jsr $28d70.l
pub fn i_049a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49A2C);
    cpu.pc = 0x28D70;
    16
}
/// $049A2C: bcc.w $49b04
pub fn i_049a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A30;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49B04; return 10; }
    8
}
/// $049A30: jsr $5e136.l
pub fn i_049a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49A36);
    cpu.pc = 0x5E136;
    16
}
/// $049A36: bcs.w $49b04
pub fn i_049a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A3A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x49B04; return 10; }
    8
}
/// $049A3A: subq.b #$1, $70(a6)
pub fn i_049a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A3E;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $049A3E: bne.w $49b04
pub fn i_049a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A42;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49B04; return 10; }
    8
}
/// $049A42: move.b #$3, $70(a6)
pub fn i_049a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A48;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049A48: asr.w #$4, d0
pub fn i_049a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A4A;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $049A4A: andi.w #$f, d0
pub fn i_049a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A4E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $049A4E: btst.b #$3, d0
pub fn i_049a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A52;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $049A52: bne.w $49a5c
pub fn i_049a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49A5C; return 10; }
    8
}
/// $049A56: andi.w #$fc, d0
pub fn i_049a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A5A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $049A5A: asl.w #$1, d0
pub fn i_049a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A5C;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $049A5C: cmpi.w #$8, $34(a6)
pub fn i_049a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A62;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Word, false);
    16
}
/// $049A62: bne.w $49a72
pub fn i_049a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A66;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49A72; return 10; }
    8
}
/// $049A66: cmpi.w #$0, d0
pub fn i_049a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A6A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $049A6A: bne.w $49a72
pub fn i_049a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A6E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49A72; return 10; }
    8
}
/// $049A6E: move.w #$f, d0
pub fn i_049a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A72;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $049A72: jsr $4d4d0.l
pub fn i_049a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49A78);
    cpu.pc = 0x4D4D0;
    16
}
/// $049A78: add.w d0, $34(a6)
pub fn i_049a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A7C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $049A7C: move.w $34(a6), d0
pub fn i_049a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A80;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $049A80: andi.w #$f, d0
pub fn i_049a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A84;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $049A84: btst.b #$3, d0
pub fn i_049a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A88;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $049A88: bne.w $49a92
pub fn i_049a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A8C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49A92; return 10; }
    8
}
/// $049A8C: andi.w #$fc, d0
pub fn i_049a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A90;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $049A90: asl.w #$1, d0
pub fn i_049a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A92;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $049A92: move.w d0, $34(a6)
pub fn i_049a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A96;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $049A96: move.w $72(a6), d0
pub fn i_049a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A9A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $049A9A: move.w $34(a6), d1
pub fn i_049a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49A9E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $049A9E: cmpi.w #$d, d0
pub fn i_049a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AA2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xD, Size::Word, false);
    8
}
/// $049AA2: bne.w $49abe
pub fn i_049aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49ABE; return 10; }
    8
}
/// $049AA6: cmpi.w #$c, d1
pub fn i_049aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AAA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC, Size::Word, false);
    8
}
/// $049AAA: bne.w $49abe
pub fn i_049aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AAE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49ABE; return 10; }
    8
}
/// $049AAE: lea.l $28f748.l, a0
pub fn i_049aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AB4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28F748;
    20
}
/// $049AB4: jsr $28cd4.l
pub fn i_049ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49ABA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49ABA);
    cpu.pc = 0x28CD4;
    16
}
/// $049ABA: bra.w $49b04
pub fn i_049aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49ABE;
    cpu.ir = 0x6000;
    cpu.pc = 0x49B04;
    10
}
/// $049ABE: cmpi.w #$c, d0
pub fn i_049abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AC2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC, Size::Word, false);
    8
}
/// $049AC2: bne.w $49ade
pub fn i_049ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49ADE; return 10; }
    8
}
/// $049AC6: cmpi.w #$d, d1
pub fn i_049ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49ACA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xD, Size::Word, false);
    8
}
/// $049ACA: bne.w $49ade
pub fn i_049aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49ACE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49ADE; return 10; }
    8
}
/// $049ACE: lea.l $28f782.l, a0
pub fn i_049ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28F782;
    20
}
/// $049AD4: jsr $28cd4.l
pub fn i_049ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49ADA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49ADA);
    cpu.pc = 0x28CD4;
    16
}
/// $049ADA: bra.w $49b04
pub fn i_049ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49ADE;
    cpu.ir = 0x6000;
    cpu.pc = 0x49B04;
    10
}
/// $049ADE: clr.l d0
pub fn i_049ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AE0;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $049AE0: move.w $34(a6), d0
pub fn i_049ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AE4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $049AE4: andi.w #$f, d0
pub fn i_049ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AE8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $049AE8: movea.l #$28f708, a0
pub fn i_049ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AEE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28F708;
    cpu.a[0] = v1;
    4
}
/// $049AEE: lsl.w #$2, d0
pub fn i_049aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AF0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $049AF0: movea.l (a0, d0.w), a0
pub fn i_049af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AF4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $049AF4: cmpa.l #$ffffffff, a0
pub fn i_049af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AFA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $049AFA: beq.w $49b04
pub fn i_049afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49AFE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x49B04; return 10; }
    8
}
// ===== JsrAbsThunk_049afe  ($049AFE, 8 B, thunks/jsr_abs_thunks.c)
/// $049AFE: jsr $28cd4.l
pub fn i_049afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49B04);
    cpu.pc = 0x28CD4;
    16
}
/// $049B04: rts
pub fn i_049b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_DropFxTimer_049b06  ($049B06, 74 B, pow/hang_049430.s)
/// $049B06: subq.w #$1, $76(a6)
pub fn i_049b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B0A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $049B0A: cmpi.w #$0, $76(a6)
pub fn i_049b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B10;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $049B10: bgt.w $49b56
pub fn i_049b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B14;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x49B56; return 10; }
    8
}
/// $049B14: cmpi.b #$1, $20(a6)
pub fn i_049b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B1A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $049B1A: beq.w $49b56
pub fn i_049b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B1E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x49B56; return 10; }
    8
}
/// $049B1E: lea.l $2bff0c.l, a0
pub fn i_049b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFF0C;
    20
}
/// $049B24: jsr $799de.l
pub fn i_049b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49B2A);
    cpu.pc = 0x799DE;
    16
}
/// $049B2A: move.w d0, $76(a6)
pub fn i_049b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B2E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $049B2E: lea.l $48b56.l, a1
pub fn i_049b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B34;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x48B56;
    20
}
/// $049B34: jsr $4ae.l
pub fn i_049b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49B3A);
    cpu.pc = 0x4AE;
    16
}
/// $049B3A: jsr $5dd02.l
pub fn i_049b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49B40);
    cpu.pc = 0x5DD02;
    16
}
/// $049B40: move.w $80(a6), d0
pub fn i_049b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B44;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $049B44: move.w $82(a6), d1
pub fn i_049b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B48;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $049B48: add.w d0, $22(a0)
pub fn i_049b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B4C;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $049B4C: add.w d1, $24(a0)
pub fn i_049b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B50;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetTaskHandler_049b50  ($049B50, 8 B, kernel/task_handlers.c)
/// $049B50: lea.l $4968a(pc), a1
pub fn i_049b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4968A;
    16
}
/// $049B54: move.l a1, (a6)
pub fn i_049b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049B56: rts
pub fn i_049b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_HitCheck_049b58  ($049B58, 20 B, pow/hang_049430.s)
/// $049B58: cmpi.b #$0, $7f(a6)
pub fn i_049b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B5E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $049B5E: bne.w $49b78
pub fn i_049b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B62;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49B78; return 10; }
    8
}
/// $049B62: jsr $27eba.l
pub fn i_049b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49B68);
    cpu.pc = 0x27EBA;
    16
}
/// $049B68: bcc.w $49b72
pub fn i_049b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B6C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49B72; return 10; }
    8
}
// ===== ClearXN_049b6c  ($049B6C, 6 B, thunks/ccr_helpers.c)
/// $049B6C: andi.b #$ee, ccr
pub fn i_049b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B70;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $049B70: rts
pub fn i_049b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_049b72  ($049B72, 6 B, thunks/ccr_helpers.c)
/// $049B72: ori.b #$11, ccr
pub fn i_049b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B76;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $049B76: rts
pub fn i_049b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_HitCheckAlt_049b78  ($049B78, 10 B, pow/hang_049430.s)
/// $049B78: jsr $27fac.l
pub fn i_049b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49B7E);
    cpu.pc = 0x27FAC;
    16
}
/// $049B7E: bcc.w $49b88
pub fn i_049b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B82;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49B88; return 10; }
    8
}
// ===== ClearXN_049b82  ($049B82, 6 B, thunks/ccr_helpers.c)
/// $049B82: andi.b #$ee, ccr
pub fn i_049b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B86;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $049B86: rts
pub fn i_049b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_049b88  ($049B88, 6 B, thunks/ccr_helpers.c)
/// $049B88: ori.b #$11, ccr
pub fn i_049b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B8C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $049B8C: rts
pub fn i_049b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowHang_SlotPrioCheck_049b8e  ($049B8E, 16 B, pow/hang_049430.s)
/// $049B8E: movea.l $8(a6), a1
pub fn i_049b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B92;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $049B92: move.b $10(a6), d0
pub fn i_049b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B96;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $049B96: cmp.b $10(a1), d0
pub fn i_049b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B9A;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $049B9A: bcs.w $49ba4
pub fn i_049b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49B9E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x49BA4; return 10; }
    8
}
// ===== ClearXN_049b9e  ($049B9E, 6 B, thunks/ccr_helpers.c)
/// $049B9E: andi.b #$ee, ccr
pub fn i_049b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BA2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $049BA2: rts
pub fn i_049ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_049ba4  ($049BA4, 6 B, thunks/ccr_helpers.c)
/// $049BA4: ori.b #$11, ccr
pub fn i_049ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BA8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $049BA8: rts
pub fn i_049ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowFall_Spawn_049baa  ($049BAA, 250 B, pow/hang_049430.s)
/// $049BAA: jsr $5e7c0.l
pub fn i_049baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49BB0);
    cpu.pc = 0x5E7C0;
    16
}
/// $049BB0: move.w #$e, d1
pub fn i_049bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BB4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $049BB4: jsr $236e.l
pub fn i_049bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49BBA);
    cpu.pc = 0x236E;
    16
}
/// $049BBA: move.w #$8000, $38(a6)
pub fn i_049bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $049BC0: jsr $267e2.l
pub fn i_049bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49BC6);
    cpu.pc = 0x267E2;
    16
}
/// $049BC6: move.w #$1, $66(a6)
pub fn i_049bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BCC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $049BCC: lea.l $28ffdc.l, a0
pub fn i_049bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BD2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28FFDC;
    20
}
/// $049BD2: jsr $28cd4.l
pub fn i_049bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49BD8);
    cpu.pc = 0x28CD4;
    16
}
/// $049BD8: lea.l $2bdc88.l, a0
pub fn i_049bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BDE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDC88;
    20
}
/// $049BDE: jsr $799de.l
pub fn i_049bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49BE4);
    cpu.pc = 0x799DE;
    16
}
/// $049BE4: move.w d0, d0
pub fn i_049be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BE6;
    cpu.ir = 0x3000;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $049BE6: move.b $98(a6), d1
pub fn i_049be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BEA;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $049BEA: move.b d1, $3a(a6)
pub fn i_049bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BEE;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049BEE: btst.b #$0, d1
pub fn i_049bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BF2;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $049BF2: bne.w $49bf8
pub fn i_049bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BF6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49BF8; return 10; }
    8
}
/// $049BF6: neg.w d0
pub fn i_049bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BF8;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $049BF8: move.w d0, $28(a6)
pub fn i_049bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49BFC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $049BFC: move.w #$ffa0, $2e(a6)
pub fn i_049bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C02;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $049C02: move.l $106f54.l, d0
pub fn i_049c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C08;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F54);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $049C08: swap d0
pub fn i_049c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C0A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $049C0A: cmpi.w #$d0, d0
pub fn i_049c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C0E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xD0, Size::Word, false);
    8
}
/// $049C0E: blt.w $49c30
pub fn i_049c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C12;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x49C30; return 10; }
    8
}
/// $049C12: jsr $5e9b6.l
pub fn i_049c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49C18);
    cpu.pc = 0x5E9B6;
    16
}
/// $049C18: andi.w #$7, d0
pub fn i_049c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C1C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $049C1C: bne.w $49c30
pub fn i_049c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C20;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49C30; return 10; }
    8
}
/// $049C20: move.w $28(a6), d0
pub fn i_049c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C24;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $049C24: asr.w #$1, d0
pub fn i_049c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C26;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $049C26: add.w d0, $28(a6)
pub fn i_049c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C2A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $049C2A: asr.w #$1, d0
pub fn i_049c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C2C;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $049C2C: add.w d0, $28(a6)
pub fn i_049c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C30;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $049C30: move.b #$0, $20(a6)
pub fn i_049c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C36;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049C36: lea.l $49ca4(pc), a1
pub fn i_049c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49CA4;
    16
}
/// $049C3A: jsr $4ae.l
pub fn i_049c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49C40);
    cpu.pc = 0x4AE;
    16
}
/// $049C40: jsr $5dd02.l
pub fn i_049c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49C46);
    cpu.pc = 0x5DD02;
    16
}
/// $049C46: move.w $28(a6), $28(a0)
pub fn i_049c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C4C;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    20
}
/// $049C4C: move.w $2a(a6), $2a(a0)
pub fn i_049c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C52;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $049C52: move.b $3a(a6), $3a(a0)
pub fn i_049c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C58;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $049C58: lea.l $49c5e(pc), a1
pub fn i_049c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49C5E;
    16
}
/// $049C5C: move.l a1, (a6)
pub fn i_049c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049C5E: jsr $27a92.l
pub fn i_049c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49C64);
    cpu.pc = 0x27A92;
    16
}
/// $049C64: jsr $28d70.l
pub fn i_049c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49C6A);
    cpu.pc = 0x28D70;
    16
}
/// $049C6A: jsr $2870a.l
pub fn i_049c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49C70);
    cpu.pc = 0x2870A;
    16
}
/// $049C70: bcc.w $49c80
pub fn i_049c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C74;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49C80; return 10; }
    8
}
/// $049C74: move.b #$ff, $20(a6)
pub fn i_049c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C7A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049C7A: jsr $49fd0.l
pub fn i_049c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49C80);
    cpu.pc = 0x49FD0;
    16
}
/// $049C80: movea.l #$ffffffff, a0
pub fn i_049c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C86;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $049C86: lea.l $2902c4.l, a0
pub fn i_049c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2902C4;
    20
}
/// $049C8C: jsr $5dd5c.l
pub fn i_049c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49C92);
    cpu.pc = 0x5DD5C;
    16
}
/// $049C92: bcc.w $49ca2
pub fn i_049c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49CA2; return 10; }
    8
}
/// $049C96: move.b #$ff, $20(a6)
pub fn i_049c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49C9C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049C9C: jmp $518.l
pub fn i_049c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CA2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $049CA2: rts
pub fn i_049ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowFall_Shadow_049ca4  ($049CA4, 202 B, pow/hang_049430.s)
/// $049CA4: jsr $5e7c0.l
pub fn i_049ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49CAA);
    cpu.pc = 0x5E7C0;
    16
}
/// $049CAA: move.w #$144, d1
pub fn i_049caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CAE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x144;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $049CAE: jsr $236e.l
pub fn i_049cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49CB4);
    cpu.pc = 0x236E;
    16
}
/// $049CB4: subq.w #$1, $38(a6)
pub fn i_049cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CB8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $049CB8: lea.l $290090.l, a0
pub fn i_049cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CBE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290090;
    20
}
/// $049CBE: jsr $28cd4.l
pub fn i_049cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49CC4);
    cpu.pc = 0x28CD4;
    16
}
/// $049CC4: lea.l $290106.l, a0
pub fn i_049cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CCA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290106;
    20
}
/// $049CCA: move.l a0, $4c(a6)
pub fn i_049cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CCE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $049CCE: jsr $283ca.l
pub fn i_049cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49CD4);
    cpu.pc = 0x283CA;
    16
}
/// $049CD4: lea.l $29024e.l, a0
pub fn i_049cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CDA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29024E;
    20
}
/// $049CDA: move.l a0, $48(a6)
pub fn i_049cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CDE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $049CDE: lea.l $49ce4(pc), a1
pub fn i_049cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49CE4;
    16
}
/// $049CE2: move.l a1, (a6)
pub fn i_049ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049CE4: movea.l $c(a6), a0
pub fn i_049ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CE8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $049CE8: cmpi.b #$ff, $20(a0)
pub fn i_049ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CEE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $049CEE: bne.w $49cfc
pub fn i_049cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CF2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49CFC; return 10; }
    8
}
/// $049CF2: jsr $27a92.l
pub fn i_049cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49CF8);
    cpu.pc = 0x27A92;
    16
}
/// $049CF8: bra.w $49d32
pub fn i_049cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49CFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x49D32;
    10
}
/// $049CFC: move.w $22(a0), $22(a6)
pub fn i_049cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D02;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $049D02: move.w #$20, d0
pub fn i_049d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D06;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $049D06: btst.b #$0, $3a(a6)
pub fn i_049d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D0C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $049D0C: bne.w $49d12
pub fn i_049d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D10;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49D12; return 10; }
    8
}
/// $049D10: neg.w d0
pub fn i_049d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D12;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $049D12: add.w d0, $22(a6)
pub fn i_049d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D16;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $049D16: move.w $28(a6), d0
pub fn i_049d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D1A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $049D1A: movem.w d0, -(a7)
pub fn i_049d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D1E;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $049D1E: move.w #$0, $28(a6)
pub fn i_049d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $049D24: jsr $5e7c0.l
pub fn i_049d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49D2A);
    cpu.pc = 0x5E7C0;
    16
}
/// $049D2A: movem.w (a7)+, d0
pub fn i_049d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D2E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $049D2E: move.w d0, $28(a6)
pub fn i_049d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D32;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $049D32: jsr $28d70.l
pub fn i_049d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49D38);
    cpu.pc = 0x28D70;
    16
}
/// $049D38: jsr $283d8.l
pub fn i_049d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49D3E);
    cpu.pc = 0x283D8;
    16
}
/// $049D3E: jsr $2870a.l
pub fn i_049d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49D44);
    cpu.pc = 0x2870A;
    16
}
/// $049D44: move.w #$7fff, $66(a6)
pub fn i_049d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D4A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $049D4A: bclr.b #$3, $13(a6)
pub fn i_049d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D50;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $049D50: movea.l #$ffffffff, a0
pub fn i_049d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D56;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $049D56: lea.l $2902ce.l, a0
pub fn i_049d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D5C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2902CE;
    20
}
/// $049D5C: jsr $5dd5c.l
pub fn i_049d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49D62);
    cpu.pc = 0x5DD5C;
    16
}
/// $049D62: bcc.w $49d6c
pub fn i_049d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49D6C; return 10; }
    8
}
/// $049D66: jmp $518.l
pub fn i_049d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D6C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $049D6C: rts
pub fn i_049d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PowFall_SlotPrioCheck_049d6e  ($049D6E, 16 B, pow/hang_049430.s)
/// $049D6E: movea.l $8(a6), a1
pub fn i_049d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D72;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $049D72: move.b $10(a6), d0
pub fn i_049d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D76;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $049D76: cmp.b $10(a1), d0
pub fn i_049d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D7A;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $049D7A: bcs.w $49d84
pub fn i_049d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D7E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x49D84; return 10; }
    8
}
// ===== ClearXN_049d7e  ($049D7E, 6 B, thunks/ccr_helpers.c)
/// $049D7E: andi.b #$ee, ccr
pub fn i_049d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D82;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $049D82: rts
pub fn i_049d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_049d84  ($049D84, 6 B, thunks/ccr_helpers.c)
/// $049D84: ori.b #$11, ccr
pub fn i_049d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D88;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $049D88: rts
pub fn i_049d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49D8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_HitCheckUnlessCutscene_049fba  ($049FBA, 10 B, pow/hang_049430.s)
/// $049FBA: tst.b $106ed3.l
pub fn i_049fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FC0;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $049FC0: bne.w $49fca
pub fn i_049fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FC4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49FCA; return 10; }
    8
}
// ===== SetC_049fc4  ($049FC4, 6 B, thunks/ccr_helpers.c)
/// $049FC4: ori.b #$1, ccr
pub fn i_049fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FC8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $049FC8: rts
pub fn i_049fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_049fca  ($049FCA, 6 B, thunks/jmp_abs_thunks.c)
/// $049FCA: jmp $2870a.l
pub fn i_049fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FD0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x2870A;
    8
}
// ===== Entity_ProbeAndInstallHandler_049FD0  ($049FD0, 34 B, entity/probe_install_handler_049fd0.s)
/// $049FD0: bsr.b $49fba
pub fn i_049fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FD2;
    cpu.ir = 0x61E8;
    cpu.push32(bus, 0x49FD2);
    cpu.pc = 0x49FBA;
    18
}
/// $049FD2: bcc.w $49ff0
pub fn i_049fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49FF0; return 10; }
    8
}
/// $049FD6: jsr $27eba.l
pub fn i_049fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49FDC);
    cpu.pc = 0x27EBA;
    16
}
/// $049FDC: bcc.w $49fea
pub fn i_049fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49FEA; return 10; }
    8
}
/// $049FE0: lea.l $4a034(pc), a1
pub fn i_049fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A034;
    16
}
/// $049FE4: move.l a1, (a6)
pub fn i_049fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049FE6: bra.w $49ff0
pub fn i_049fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FEA;
    cpu.ir = 0x6000;
    cpu.pc = 0x49FF0;
    10
}
/// $049FEA: lea.l $4a014(pc), a1
pub fn i_049fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A014;
    16
}
/// $049FEE: move.l a1, (a6)
pub fn i_049fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049FF0: rts
pub fn i_049ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_Dispatch_049ff2  ($049FF2, 26 B, pow/human_death_049ff2.s)
/// $049FF2: bsr.b $49fba
pub fn i_049ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FF4;
    cpu.ir = 0x61C6;
    cpu.push32(bus, 0x49FF4);
    cpu.pc = 0x49FBA;
    18
}
/// $049FF4: bcc.w $4a012
pub fn i_049ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FF8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A012; return 10; }
    8
}
/// $049FF8: jsr $27eba.l
pub fn i_049ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49FFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49FFE);
    cpu.pc = 0x27EBA;
    16
}
/// $049FFE: bcc.w $4a00c
pub fn i_049ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A002;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A00C; return 10; }
    8
}
/// $04A002: lea.l $4a034(pc), a1
pub fn i_04a002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A006;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A034;
    16
}
/// $04A006: move.l a1, (a6)
pub fn i_04a006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A008;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A008: bra.w $4a012
pub fn i_04a008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A00C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A012;
    10
}
// ===== SetTaskHandler_04a00c  ($04A00C, 8 B, kernel/task_handlers.c)
/// $04A00C: lea.l $4a024(pc), a1
pub fn i_04a00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A010;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A024;
    16
}
/// $04A010: move.l a1, (a6)
pub fn i_04a010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A012;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A012: rts
pub fn i_04a012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A014;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_EntryKind0_04a014  ($04A014, 16 B, pow/human_death_049ff2.s)
/// $04A014: jsr $27f60.l
pub fn i_04a014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A01A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A01A);
    cpu.pc = 0x27F60;
    16
}
/// $04A01A: scc.b $70(a6)
pub fn i_04a01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A01E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A01E: moveq #$0, d7
pub fn i_04a01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A020;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A020: bra.w $4a050
pub fn i_04a020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A024;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A050;
    10
}
// ===== HumanDeath_EntryKind1_04a024  ($04A024, 16 B, pow/human_death_049ff2.s)
/// $04A024: jsr $27f60.l
pub fn i_04a024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A02A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A02A);
    cpu.pc = 0x27F60;
    16
}
/// $04A02A: scc.b $70(a6)
pub fn i_04a02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A02E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A02E: moveq #$1, d7
pub fn i_04a02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A030;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A030: bra.w $4a050
pub fn i_04a030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A034;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A050;
    10
}
// ===== HumanDeath_EntryKind2_04a034  ($04A034, 104 B, pow/human_death_049ff2.s)
/// $04A034: jsr $27f60.l
pub fn i_04a034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A03A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A03A);
    cpu.pc = 0x27F60;
    16
}
/// $04A03A: scc.b $70(a6)
pub fn i_04a03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A03E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A03E: moveq #$2, d7
pub fn i_04a03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A040;
    cpu.ir = 0x7E02;
    cpu.d[7] = 0x2;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A040: bra.w $4a050
pub fn i_04a040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A044;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A050;
    10
}
/// $04A044: jsr $27f60.l
pub fn i_04a044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A04A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A04A);
    cpu.pc = 0x27F60;
    16
}
/// $04A04A: scc.b $70(a6)
pub fn i_04a04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A04E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A04E: moveq #$3, d7
pub fn i_04a04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A050;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A050: move.w $22(a6), d0
pub fn i_04a050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A054;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A054: sub.w $54(a6), d0
pub fn i_04a054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A058;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04A058: scc.b d0
pub fn i_04a058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A05A;
    cpu.ir = 0x54C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $04A05A: btst.b #$0, $3a(a6)
pub fn i_04a05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A060;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04A060: sne.b d1
pub fn i_04a060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A062;
    cpu.ir = 0x56C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $04A062: eor.b d0, d1
pub fn i_04a062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A064;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04A064: move.b d1, $71(a6)
pub fn i_04a064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A068;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A068: andi.w #$ff, d7
pub fn i_04a068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A06C;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04A06C: lsl.w #$2, d7
pub fn i_04a06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A06E;
    cpu.ir = 0xE54F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04A06E: lea.l $49faa(pc), a0
pub fn i_04a06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A072;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x49FAA;
    16
}
/// $04A072: movea.l (a0, d7.w), a0
pub fn i_04a072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A076;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04A076: moveq #$0, d0
pub fn i_04a076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A078;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A078: move.b $58(a6), d0
pub fn i_04a078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A07C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04A07C: cmpi.w #$22, d0
pub fn i_04a07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A080;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x22, Size::Word, false);
    8
}
/// $04A080: bcs.w $4a088
pub fn i_04a080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A084;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4A088; return 10; }
    8
}
/// $04A084: move.w #$f, d0
pub fn i_04a084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A088;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A088: lsl.w #$2, d0
pub fn i_04a088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A08A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04A08A: move.l (a0, d0.w), -(a7)
pub fn i_04a08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A08E;
    cpu.ir = 0x2F30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $04A08E: bsr.w $4a09c
pub fn i_04a08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A092;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A092);
    cpu.pc = 0x4A09C;
    18
}
/// $04A092: jsr $8f308.l
pub fn i_04a092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A098;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A098);
    cpu.pc = 0x8F308;
    16
}
/// $04A098: movea.l (a7)+, a0
pub fn i_04a098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A09A;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $04A09A: jmp (a0)
pub fn i_04a09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A09C;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
// ===== HumanDeath_ResetBody_04a09c  ($04A09C, 56 B, pow/human_death_049ff2.s)
/// $04A09C: jsr $2783a.l
pub fn i_04a09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A0A2);
    cpu.pc = 0x2783A;
    16
}
/// $04A0A2: bsr.w $4a0d4
pub fn i_04a0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0A6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A0A6);
    cpu.pc = 0x4A0D4;
    18
}
/// $04A0A6: bclr.b #$1, $12(a6)
pub fn i_04a0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0AC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A0AC: move.w #$ffff, $34(a6)
pub fn i_04a0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0B2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A0B2: move.l #$ffffffff, $60(a6)
pub fn i_04a0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0BA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04A0BA: jsr $27f60.l
pub fn i_04a0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A0C0);
    cpu.pc = 0x27F60;
    16
}
/// $04A0C0: scc.b $70(a6)
pub fn i_04a0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0C4;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A0C4: clr.w $2c(a6)
pub fn i_04a0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0C8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A0C8: clr.w $2e(a6)
pub fn i_04a0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0CC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A0CC: move.w #$c000, $38(a6)
pub fn i_04a0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A0D2: rts
pub fn i_04a0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_PlayCryByKind_04a0d4  ($04A0D4, 146 B, pow/human_death_049ff2.s)
/// $04A0D4: move.w #$e, d1
pub fn i_04a0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0D8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A0D8: tst.b $10fd8f.l
pub fn i_04a0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0DE;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $04A0DE: bne.w $4a0e6
pub fn i_04a0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A0E6; return 10; }
    8
}
/// $04A0E2: move.w #$2f, d1
pub fn i_04a0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0E6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A0E6: bra.w $4a0f4
pub fn i_04a0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A0F4;
    10
}
/// $04A0F4: movem.w d1, -(a7)
pub fn i_04a0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0F8;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $04A0F8: jsr $13600.l
pub fn i_04a0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A0FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A0FE);
    cpu.pc = 0x13600;
    16
}
/// $04A0FE: movem.w (a7)+, d1
pub fn i_04a0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A102;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $04A102: jmp $236e.l
pub fn i_04a102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A108;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x236E;
    8
}
/// $04A110: move.w #$15c, d1
pub fn i_04a110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A114;
    cpu.ir = 0x323C;
    let v1: u32 = 0x15C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A114: bra.w $4a132
pub fn i_04a114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A118;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A132;
    10
}
/// $04A120: move.w #$e, d1
pub fn i_04a120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A124;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A124: bra.w $4a132
pub fn i_04a124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A128;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A132;
    10
}
/// $04A128: move.w #$185, d1
pub fn i_04a128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A12C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x185;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A12C: bra.w $4a132
pub fn i_04a12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A130;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A132;
    10
}
/// $04A132: movem.w d1, -(a7)
pub fn i_04a132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A136;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $04A136: jsr $13600.l
pub fn i_04a136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A13C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A13C);
    cpu.pc = 0x13600;
    16
}
/// $04A13C: movem.w (a7)+, d1
pub fn i_04a13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A140;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $04A140: jmp $236e.l
pub fn i_04a140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A146;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x236E;
    8
}
// ===== SetTaskHandler_04a166  ($04A166, 8 B, kernel/task_handlers.c)
/// $04A166: lea.l $4a18c(pc), a1
pub fn i_04a166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A16A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A18C;
    16
}
/// $04A16A: move.l a1, (a6)
pub fn i_04a16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A16C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A16C: rts
pub fn i_04a16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A16E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_SpawnCorpseA_04a16e  ($04A16E, 30 B, pow/human_death_049ff2.s)
/// $04A16E: lea.l $4a18c(pc), a1
pub fn i_04a16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A172;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A18C;
    16
}
/// $04A172: jsr $4ae.l
pub fn i_04a172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A178;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A178);
    cpu.pc = 0x4AE;
    16
}
/// $04A178: jsr $5dd02.l
pub fn i_04a178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A17E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A17E);
    cpu.pc = 0x5DD02;
    16
}
/// $04A17E: move.w $22(a6), $54(a0)
pub fn i_04a17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A184;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x54);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04A184: move.w $24(a6), $54(a0)
pub fn i_04a184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A18A;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x54);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04A18A: rts
pub fn i_04a18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A18C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_CorpseA_04a18c  ($04A18C, 8 B, pow/human_death_049ff2.s)
/// $04A18C: bsr.w $4a09c
pub fn i_04a18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A190;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A190);
    cpu.pc = 0x4A09C;
    18
}
/// $04A190: bra.w $4a72e
pub fn i_04a190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A194;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A72E;
    10
}
// ===== HumanDeath_SpawnCorpseB_04a194  ($04A194, 30 B, pow/human_death_049ff2.s)
/// $04A194: lea.l $4a1b2(pc), a1
pub fn i_04a194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A198;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A1B2;
    16
}
/// $04A198: jsr $4ae.l
pub fn i_04a198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A19E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A19E);
    cpu.pc = 0x4AE;
    16
}
/// $04A19E: jsr $5dd02.l
pub fn i_04a19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A1A4);
    cpu.pc = 0x5DD02;
    16
}
/// $04A1A4: move.w $22(a6), $54(a0)
pub fn i_04a1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1AA;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x54);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04A1AA: move.w $24(a6), $54(a0)
pub fn i_04a1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1B0;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x54);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04A1B0: rts
pub fn i_04a1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_CorpseB_04a1b2  ($04A1B2, 8 B, pow/human_death_049ff2.s)
/// $04A1B2: bsr.w $4a09c
pub fn i_04a1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1B6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A1B6);
    cpu.pc = 0x4A09C;
    18
}
/// $04A1B6: bra.w $4a802
pub fn i_04a1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A802;
    10
}
// ===== HumanDeath_LoadTimer_04a1ba  ($04A1BA, 4 B, pow/human_death_049ff2.s)
/// $04A1BA: move.w $74(a6), d0
pub fn i_04a1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1BE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
// ===== JsrAbsThunk_04a1be  ($04A1BE, 8 B, thunks/jsr_abs_thunks.c)
/// $04A1BE: jsr $2352.l
pub fn i_04a1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A1C4);
    cpu.pc = 0x2352;
    16
}
/// $04A1C4: rts
pub fn i_04a1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_DampVelocity_04a1c6  ($04A1C6, 42 B, pow/human_death_049ff2.s)
/// $04A1C6: move.w $28(a6), d0
pub fn i_04a1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1CA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A1CA: beq.w $4a1da
pub fn i_04a1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A1DA; return 10; }
    8
}
/// $04A1CE: asr.w #$5, d0
pub fn i_04a1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1D0;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04A1D0: seq.b d1
pub fn i_04a1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1D2;
    cpu.ir = 0x57C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $04A1D2: ext.w d1
pub fn i_04a1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1D4;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A1D4: sub.w d1, d0
pub fn i_04a1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1D6;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04A1D6: sub.w d0, $28(a6)
pub fn i_04a1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1DA;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A1DA: move.w $2a(a6), d0
pub fn i_04a1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1DE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A1DE: beq.w $4a1ee
pub fn i_04a1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A1EE; return 10; }
    8
}
/// $04A1E2: asr.w #$5, d0
pub fn i_04a1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1E4;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04A1E4: seq.b d1
pub fn i_04a1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1E6;
    cpu.ir = 0x57C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $04A1E6: ext.w d1
pub fn i_04a1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1E8;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A1E8: sub.w d1, d0
pub fn i_04a1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1EA;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04A1EA: sub.w d0, $2a(a6)
pub fn i_04a1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1EE;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A1EE: rts
pub fn i_04a1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_SetVelXByFacing_04a1f0  ($04A1F0, 12 B, pow/human_death_049ff2.s)
/// $04A1F0: btst.b #$0, $3a(a6)
pub fn i_04a1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1F6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04A1F6: bne.w $4a1fc
pub fn i_04a1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1FA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A1FC; return 10; }
    8
}
/// $04A1FA: neg.w d0
pub fn i_04a1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A1FC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_04a1fc  ($04A1FC, 6 B, kernel/task_setters_w.c)
/// $04A1FC: move.w d0, $28(a6)
pub fn i_04a1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A200;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A200: rts
pub fn i_04a200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A202;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_RandBelowY_04a202  ($04A202, 14 B, pow/human_death_049ff2.s)
/// $04A202: jsr $5e9b6.l
pub fn i_04a202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A208;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A208);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A208: move.w $24(a6), d1
pub fn i_04a208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A20C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A20C: cmp.b d1, d0
pub fn i_04a20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A20E;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $04A20E: rts
pub fn i_04a20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A210;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_04a210  ($04A210, 8 B, kernel/jmp_to_scheduler.c)
/// $04A210: jmp $518.l
pub fn i_04a210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A216;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== HumanDeath_PhysicsAir_04a218  ($04A218, 72 B, pow/human_death_049ff2.s)
/// $04A218: jsr $27f08.l
pub fn i_04a218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A21E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A21E);
    cpu.pc = 0x27F08;
    16
}
/// $04A21E: bcc.w $4a22a
pub fn i_04a21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A222;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A22A; return 10; }
    8
}
/// $04A222: move.b d3, $70(a6)
pub fn i_04a222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A226;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A226: bra.w $4a240
pub fn i_04a226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A22A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A240;
    10
}
/// $04A22A: cmp.b $70(a6), d0
pub fn i_04a22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A22E;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04A22E: bne.w $4a23a
pub fn i_04a22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A232;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A23A; return 10; }
    8
}
/// $04A232: move.b d3, $70(a6)
pub fn i_04a232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A236;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A236: bra.w $4a240
pub fn i_04a236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A23A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A240;
    10
}
/// $04A23A: move.b #$ff, $70(a6)
pub fn i_04a23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A240;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A240: bsr.b $4a1c6
pub fn i_04a240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A242;
    cpu.ir = 0x6184;
    cpu.push32(bus, 0x4A242);
    cpu.pc = 0x4A1C6;
    18
}
/// $04A242: tst.b $70(a6)
pub fn i_04a242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A246;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A246: bne.w $4a25c
pub fn i_04a246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A24A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A25C; return 10; }
    8
}
/// $04A24A: move.w #$ff40, $2e(a6)
pub fn i_04a24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A250;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A250: jsr $27bc8.l
pub fn i_04a250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A256;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A256);
    cpu.pc = 0x27BC8;
    16
}
/// $04A256: scs.b $70(a6)
pub fn i_04a256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A25A;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A25A: rts
pub fn i_04a25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A25C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04A25C: bsr.w $4a1c6
pub fn i_04a25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A260;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A260);
    cpu.pc = 0x4A1C6;
    18
}
// ===== JsrAbsThunk_04a260  ($04A260, 8 B, thunks/jsr_abs_thunks.c)
/// $04A260: jsr $27a92.l
pub fn i_04a260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A266;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A266);
    cpu.pc = 0x27A92;
    16
}
/// $04A266: rts
pub fn i_04a266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A268;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_PhysicsGround_04a268  ($04A268, 86 B, pow/human_death_049ff2.s)
/// $04A268: jsr $27fd8.l
pub fn i_04a268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A26E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A26E);
    cpu.pc = 0x27FD8;
    16
}
/// $04A26E: bcc.w $4a27a
pub fn i_04a26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A272;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A27A; return 10; }
    8
}
/// $04A272: move.b d3, $70(a6)
pub fn i_04a272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A276;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A276: bra.w $4a290
pub fn i_04a276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A27A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A290;
    10
}
/// $04A27A: cmp.b $70(a6), d0
pub fn i_04a27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A27E;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04A27E: bne.w $4a28a
pub fn i_04a27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A282;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A28A; return 10; }
    8
}
/// $04A282: move.b d3, $70(a6)
pub fn i_04a282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A286;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A286: bra.w $4a290
pub fn i_04a286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A28A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A290;
    10
}
/// $04A28A: move.b #$ff, $70(a6)
pub fn i_04a28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A290;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A290: bsr.w $4a1c6
pub fn i_04a290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A294;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A294);
    cpu.pc = 0x4A1C6;
    18
}
/// $04A294: tst.b $70(a6)
pub fn i_04a294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A298;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A298: bne.w $4a2a8
pub fn i_04a298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A29C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A2A8; return 10; }
    8
}
/// $04A29C: jsr $27d50.l
pub fn i_04a29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A2A2);
    cpu.pc = 0x27D50;
    16
}
/// $04A2A2: scs.b $70(a6)
pub fn i_04a2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2A6;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A2A6: rts
pub fn i_04a2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04A2A8: bsr.w $4a1c6
pub fn i_04a2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2AC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A2AC);
    cpu.pc = 0x4A1C6;
    18
}
/// $04A2AC: jsr $27a92.l
pub fn i_04a2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A2B2);
    cpu.pc = 0x27A92;
    16
}
/// $04A2B2: jsr $27fac.l
pub fn i_04a2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A2B8);
    cpu.pc = 0x27FAC;
    16
}
/// $04A2B8: scc.b $70(a6)
pub fn i_04a2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2BC;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A2BC: rts
pub fn i_04a2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_PhysicsFall_04a2be  ($04A2BE, 224 B, pow/human_death_049ff2.s)
/// $04A2BE: jsr $27f08.l
pub fn i_04a2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A2C4);
    cpu.pc = 0x27F08;
    16
}
/// $04A2C4: bcc.w $4a2d0
pub fn i_04a2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A2D0; return 10; }
    8
}
/// $04A2C8: move.b d3, $70(a6)
pub fn i_04a2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2CC;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A2CC: bra.w $4a2e6
pub fn i_04a2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A2E6;
    10
}
/// $04A2D0: cmp.b $70(a6), d0
pub fn i_04a2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2D4;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04A2D4: bne.w $4a2e0
pub fn i_04a2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A2E0; return 10; }
    8
}
/// $04A2D8: move.b d3, $70(a6)
pub fn i_04a2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2DC;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A2DC: bra.w $4a2e6
pub fn i_04a2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A2E6;
    10
}
/// $04A2E0: move.b #$ff, $70(a6)
pub fn i_04a2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2E6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A2E6: tst.b $70(a6)
pub fn i_04a2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2EA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A2EA: bne.w $4a2fe
pub fn i_04a2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A2FE; return 10; }
    8
}
/// $04A2EE: move.w #$ff40, $2e(a6)
pub fn i_04a2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A2F4: jsr $27bc8.l
pub fn i_04a2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A2FA);
    cpu.pc = 0x27BC8;
    16
}
/// $04A2FA: scs.b $70(a6)
pub fn i_04a2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A2FE;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A2FE: jsr $27a92.l
pub fn i_04a2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A304);
    cpu.pc = 0x27A92;
    16
}
/// $04A304: jsr $27eba.l
pub fn i_04a304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A30A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A30A);
    cpu.pc = 0x27EBA;
    16
}
/// $04A30A: scc.b $70(a6)
pub fn i_04a30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A30E;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A30E: rts
pub fn i_04a30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A310;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04A35E: move.w $22(a6), d0
pub fn i_04a35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A362;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A362: addi.w #$20, d0
pub fn i_04a362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A366;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04A366: cmpi.w #$180, d0
pub fn i_04a366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A36A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x180, Size::Word, false);
    8
}
/// $04A36A: bcs.w $4a372
pub fn i_04a36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A36E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4A372; return 10; }
    8
}
/// $04A36E: jmp $4a210(pc)
pub fn i_04a36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A372;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x4A210;
    8
}
/// $04A372: cmpi.w #$e0, $24(a6)
pub fn i_04a372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A378;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xE0, Size::Word, false);
    16
}
/// $04A378: bcc.w $4a380
pub fn i_04a378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A37C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A380; return 10; }
    8
}
/// $04A37C: jmp $4a210(pc)
pub fn i_04a37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A380;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x4A210;
    8
}
/// $04A380: rts
pub fn i_04a380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A382;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04A382: lea.l $4a388(pc), a1
pub fn i_04a382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A386;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A388;
    16
}
/// $04A386: move.l a1, (a6)
pub fn i_04a386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A388;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A388: bsr.w $4a218
pub fn i_04a388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A38C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A38C);
    cpu.pc = 0x4A218;
    18
}
/// $04A38C: jsr $28d70.l
pub fn i_04a38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A392;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A392);
    cpu.pc = 0x28D70;
    16
}
/// $04A392: bcc.w $4a39c
pub fn i_04a392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A396;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A39C; return 10; }
    8
}
/// $04A396: lea.l $4a39e(pc), a1
pub fn i_04a396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A39A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A39E;
    16
}
/// $04A39A: move.l a1, (a6)
pub fn i_04a39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A39C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A39C: bra.b $4a35e
pub fn i_04a39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A39E;
    cpu.ir = 0x60C0;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_FadeOut_04a39e  ($04A39E, 106 B, pow/human_death_049ff2.s)
/// $04A39E: move.b #$a, $5c(a6)
pub fn i_04a39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3A4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A3A4: lea.l $4a3aa(pc), a1
pub fn i_04a3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A3AA;
    16
}
/// $04A3A8: move.l a1, (a6)
pub fn i_04a3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A3AA: bsr.w $4a218
pub fn i_04a3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3AE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A3AE);
    cpu.pc = 0x4A218;
    18
}
/// $04A3AE: jsr $28d70.l
pub fn i_04a3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A3B4);
    cpu.pc = 0x28D70;
    16
}
/// $04A3B4: subq.b #$1, $5c(a6)
pub fn i_04a3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3B8;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04A3B8: bne.w $4a3c2
pub fn i_04a3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3BC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A3C2; return 10; }
    8
}
/// $04A3BC: lea.l $4a3c4(pc), a1
pub fn i_04a3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A3C4;
    16
}
/// $04A3C0: move.l a1, (a6)
pub fn i_04a3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A3C2: bra.b $4a35e
pub fn i_04a3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3C4;
    cpu.ir = 0x609A;
    cpu.pc = 0x4A35E;
    10
}
/// $04A3C4: move.b #$a, $5c(a6)
pub fn i_04a3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A3CA: lea.l $4a3d0(pc), a1
pub fn i_04a3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A3D0;
    16
}
/// $04A3CE: move.l a1, (a6)
pub fn i_04a3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A3D0: jsr $4a218(pc)
pub fn i_04a3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4A3D4);
    cpu.pc = 0x4A218;
    16
}
/// $04A3D4: subq.b #$1, $5c(a6)
pub fn i_04a3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3D8;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04A3D8: bne.w $4a3e4
pub fn i_04a3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A3E4; return 10; }
    8
}
/// $04A3DC: jmp $4a210(pc)
pub fn i_04a3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3E0;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x4A210;
    8
}
/// $04A3E4: btst.b #$0, $5c(a6)
pub fn i_04a3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3EA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04A3EA: bne.w $4a3f4
pub fn i_04a3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A3F4; return 10; }
    8
}
/// $04A3EE: jsr $28d70.l
pub fn i_04a3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A3F4);
    cpu.pc = 0x28D70;
    16
}
/// $04A3F4: bra.w $4a35e
pub fn i_04a3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
/// $04A3F8: bsr.w $4aae0
pub fn i_04a3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A3FC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A3FC);
    cpu.pc = 0x4AAE0;
    18
}
/// $04A3FC: tst.b $71(a6)
pub fn i_04a3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A400;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A400: bne.w $4a420
pub fn i_04a400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A404;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A420; return 10; }
    8
}
/// $04A404: bra.w $4a4c8
pub fn i_04a404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A408;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A4C8;
    10
}
// ===== HumanDeath_TumbleBackStart_04a420  ($04A420, 24 B, pow/human_death_049ff2.s)
/// $04A420: bsr.w $4a202
pub fn i_04a420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A424;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A424);
    cpu.pc = 0x4A202;
    18
}
/// $04A424: bcs.w $4a620
pub fn i_04a424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A428;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4A620; return 10; }
    8
}
/// $04A428: jsr $5e9b6.l
pub fn i_04a428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A42E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A42E);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A42E: tst.w d0
pub fn i_04a42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A430;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A430: bmi.w $4a568
pub fn i_04a430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A434;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x4A568; return 10; }
    8
}
/// $04A434: clr.b $72(a6)
pub fn i_04a434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A438;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== HumanDeath_TumbleBack_04a438  ($04A438, 106 B, pow/human_death_049ff2.s)
/// $04A438: lea.l $4a408(pc), a0
pub fn i_04a438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A43C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4A408;
    16
}
/// $04A43C: moveq #$0, d0
pub fn i_04a43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A43E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A43E: move.b $72(a6), d0
pub fn i_04a43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A442;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04A442: lsl.w #$2, d0
pub fn i_04a442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A444;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04A444: move.l (a0, d0.w), d0
pub fn i_04a444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A448;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $04A448: beq.w $4a4a2
pub fn i_04a448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A44C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A4A2; return 10; }
    8
}
/// $04A44C: movea.l d0, a0
pub fn i_04a44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A44E;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04A44E: jsr $28cd4.l
pub fn i_04a44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A454);
    cpu.pc = 0x28CD4;
    16
}
/// $04A454: addq.b #$1, $72(a6)
pub fn i_04a454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A458;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04A458: bclr.b #$3, $13(a6)
pub fn i_04a458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A45E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A45E: move.w #$ff80, d0
pub fn i_04a45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A462;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A462: bsr.w $4a1f0
pub fn i_04a462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A466;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A466);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A466: addi.w #$80, $2a(a6)
pub fn i_04a466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A46C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04A46C: addq.w #$2, $24(a6)
pub fn i_04a46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A470;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A470: lea.l $4a476(pc), a1
pub fn i_04a470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A474;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A476;
    16
}
/// $04A474: move.l a1, (a6)
pub fn i_04a474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A476;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A476: bsr.w $4a218
pub fn i_04a476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A47A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A47A);
    cpu.pc = 0x4A218;
    18
}
/// $04A47A: jsr $28d70.l
pub fn i_04a47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A480;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A480);
    cpu.pc = 0x28D70;
    16
}
/// $04A480: bcc.w $4a49e
pub fn i_04a480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A484;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A49E; return 10; }
    8
}
/// $04A484: jsr $2870a.l
pub fn i_04a484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A48A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A48A);
    cpu.pc = 0x2870A;
    16
}
/// $04A48A: bcc.w $4a498
pub fn i_04a48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A48E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A498; return 10; }
    8
}
/// $04A48E: lea.l $4a438(pc), a1
pub fn i_04a48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A492;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A438;
    16
}
/// $04A492: move.l a1, (a6)
pub fn i_04a492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A494;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A494: bra.w $4a49e
pub fn i_04a494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A498;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A49E;
    10
}
/// $04A498: lea.l $4a4a2(pc), a1
pub fn i_04a498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A49C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A4A2;
    16
}
/// $04A49C: move.l a1, (a6)
pub fn i_04a49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A49E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A49E: bra.w $4a35e
pub fn i_04a49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_TumbleBackLand_04a4a2  ($04A4A2, 14 B, pow/human_death_049ff2.s)
/// $04A4A2: lea.l $4b6b8(pc), a0
pub fn i_04a4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4A6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B6B8;
    16
}
/// $04A4A6: jsr $28cd4.l
pub fn i_04a4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A4AC);
    cpu.pc = 0x28CD4;
    16
}
/// $04A4AC: bra.w $4a382
pub fn i_04a4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
// ===== HumanDeath_TumbleFwdStart_04a4c8  ($04A4C8, 24 B, pow/human_death_049ff2.s)
/// $04A4C8: bsr.w $4a202
pub fn i_04a4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4CC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A4CC);
    cpu.pc = 0x4A202;
    18
}
/// $04A4CC: bcs.w $4a620
pub fn i_04a4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4D0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4A620; return 10; }
    8
}
/// $04A4D0: jsr $5e9b6.l
pub fn i_04a4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A4D6);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A4D6: tst.w d0
pub fn i_04a4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4D8;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A4D8: bmi.w $4a568
pub fn i_04a4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4DC;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x4A568; return 10; }
    8
}
/// $04A4DC: clr.b $72(a6)
pub fn i_04a4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4E0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== HumanDeath_TumbleFwd_04a4e0  ($04A4E0, 106 B, pow/human_death_049ff2.s)
/// $04A4E0: lea.l $4a4b0(pc), a0
pub fn i_04a4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4E4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4A4B0;
    16
}
/// $04A4E4: moveq #$0, d0
pub fn i_04a4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4E6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A4E6: move.b $72(a6), d0
pub fn i_04a4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4EA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04A4EA: lsl.w #$2, d0
pub fn i_04a4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4EC;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04A4EC: move.l (a0, d0.w), d0
pub fn i_04a4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4F0;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $04A4F0: beq.w $4a54a
pub fn i_04a4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A54A; return 10; }
    8
}
/// $04A4F4: movea.l d0, a0
pub fn i_04a4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4F6;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04A4F6: jsr $28cd4.l
pub fn i_04a4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A4FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A4FC);
    cpu.pc = 0x28CD4;
    16
}
/// $04A4FC: addq.b #$1, $72(a6)
pub fn i_04a4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A500;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04A500: bclr.b #$3, $13(a6)
pub fn i_04a500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A506;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A506: move.w #$80, d0
pub fn i_04a506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A50A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A50A: bsr.w $4a1f0
pub fn i_04a50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A50E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A50E);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A50E: addi.w #$80, $2a(a6)
pub fn i_04a50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A514;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04A514: addq.w #$2, $24(a6)
pub fn i_04a514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A518;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A518: lea.l $4a51e(pc), a1
pub fn i_04a518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A51C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A51E;
    16
}
/// $04A51C: move.l a1, (a6)
pub fn i_04a51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A51E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A51E: bsr.w $4a218
pub fn i_04a51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A522;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A522);
    cpu.pc = 0x4A218;
    18
}
/// $04A522: jsr $28d70.l
pub fn i_04a522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A528;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A528);
    cpu.pc = 0x28D70;
    16
}
/// $04A528: bcc.w $4a546
pub fn i_04a528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A52C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A546; return 10; }
    8
}
/// $04A52C: jsr $2870a.l
pub fn i_04a52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A532);
    cpu.pc = 0x2870A;
    16
}
/// $04A532: bcc.w $4a540
pub fn i_04a532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A536;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A540; return 10; }
    8
}
/// $04A536: lea.l $4a4e0(pc), a1
pub fn i_04a536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A53A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A4E0;
    16
}
/// $04A53A: move.l a1, (a6)
pub fn i_04a53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A53C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A53C: bra.w $4a546
pub fn i_04a53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A540;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A546;
    10
}
/// $04A540: lea.l $4a54a(pc), a1
pub fn i_04a540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A544;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A54A;
    16
}
/// $04A544: move.l a1, (a6)
pub fn i_04a544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A546;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A546: bra.w $4a35e
pub fn i_04a546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A54A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_Collapse_04a54a  ($04A54A, 296 B, pow/human_death_049ff2.s)
/// $04A54A: jsr $5e9b6.l
pub fn i_04a54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A550);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A550: lea.l $4b57c(pc), a0
pub fn i_04a550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A554;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B57C;
    16
}
/// $04A554: tst.w d0
pub fn i_04a554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A556;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A556: bpl.w $4a55e
pub fn i_04a556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A55A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x4A55E; return 10; }
    8
}
/// $04A55A: lea.l $4b61a(pc), a0
pub fn i_04a55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A55E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B61A;
    16
}
/// $04A55E: jsr $28cd4.l
pub fn i_04a55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A564);
    cpu.pc = 0x28CD4;
    16
}
/// $04A564: bra.w $4a382
pub fn i_04a564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A568;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A568: tst.b $71(a6)
pub fn i_04a568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A56C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A56C: beq.w $4a586
pub fn i_04a56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A570;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A586; return 10; }
    8
}
/// $04A570: lea.l $4b1f6(pc), a0
pub fn i_04a570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A574;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B1F6;
    16
}
/// $04A574: jsr $28cd4.l
pub fn i_04a574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A57A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A57A);
    cpu.pc = 0x28CD4;
    16
}
/// $04A57A: move.w #$fc00, d0
pub fn i_04a57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A57E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A57E: bsr.w $4a1f0
pub fn i_04a57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A582;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A582);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A582: bra.w $4a598
pub fn i_04a582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A586;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A598;
    10
}
/// $04A586: lea.l $4b28c(pc), a0
pub fn i_04a586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A58A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B28C;
    16
}
/// $04A58A: jsr $28cd4.l
pub fn i_04a58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A590);
    cpu.pc = 0x28CD4;
    16
}
/// $04A590: move.w #$400, d0
pub fn i_04a590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A594;
    cpu.ir = 0x303C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A594: bsr.w $4a1f0
pub fn i_04a594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A598;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A598);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A598: move.w #$100, $2a(a6)
pub fn i_04a598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A59E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A59E: addq.w #$2, $24(a6)
pub fn i_04a59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5A2;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A5A2: bra.w $4a382
pub fn i_04a5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A5A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A612: lea.l $4b78c(pc), a0
pub fn i_04a612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A616;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B78C;
    16
}
/// $04A616: jsr $28cd4.l
pub fn i_04a616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A61C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A61C);
    cpu.pc = 0x28CD4;
    16
}
/// $04A61C: bra.w $4a62a
pub fn i_04a61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A620;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A62A;
    10
}
/// $04A620: lea.l $4b74c(pc), a0
pub fn i_04a620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A624;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B74C;
    16
}
/// $04A624: jsr $28cd4.l
pub fn i_04a624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A62A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A62A);
    cpu.pc = 0x28CD4;
    16
}
/// $04A62A: tst.b $71(a6)
pub fn i_04a62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A62E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A62E: bne.w $4a638
pub fn i_04a62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A632;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4A638; return 10; }
    8
}
/// $04A632: bchg.b #$0, $3a(a6)
pub fn i_04a632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A638;
    cpu.ir = 0x86E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A638: move.w #$ff38, d0
pub fn i_04a638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A63C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A63C: bsr.w $4a1f0
pub fn i_04a63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A640;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A640);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A640: move.w #$ff40, $2e(a6)
pub fn i_04a640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A646;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A646: move.w #$0, $2a(a6)
pub fn i_04a646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A64C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A64C: lea.l $4a652(pc), a1
pub fn i_04a64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A650;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A652;
    16
}
/// $04A650: move.l a1, (a6)
pub fn i_04a650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A652;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A652: bsr.w $4a268
pub fn i_04a652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A656;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A656);
    cpu.pc = 0x4A268;
    18
}
/// $04A656: jsr $28d70.l
pub fn i_04a656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A65C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A65C);
    cpu.pc = 0x28D70;
    16
}
/// $04A65C: bcc.w $4a66e
pub fn i_04a65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A660;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A66E; return 10; }
    8
}
/// $04A660: tst.b $70(a6)
pub fn i_04a660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A664;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A664: beq.w $4a66e
pub fn i_04a664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A668;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A66E; return 10; }
    8
}
/// $04A668: lea.l $4a672(pc), a1
pub fn i_04a668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A66C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A672;
    16
}
/// $04A66C: move.l a1, (a6)
pub fn i_04a66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A66E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A66E: bra.w $4a35e
pub fn i_04a66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A672;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_Knockdown_04a672  ($04A672, 64 B, pow/human_death_049ff2.s)
/// $04A672: lea.l $4b7b8(pc), a0
pub fn i_04a672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A676;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B7B8;
    16
}
/// $04A676: jsr $28cd4.l
pub fn i_04a676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A67C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A67C);
    cpu.pc = 0x28CD4;
    16
}
/// $04A67C: bra.w $4a382
pub fn i_04a67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A680;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A680: jsr $5e9b6.l
pub fn i_04a680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A686;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A686);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A686: tst.w d0
pub fn i_04a686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A688;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A688: bmi.w $4a612
pub fn i_04a688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A68C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x4A612; return 10; }
    8
}
/// $04A68C: lea.l $4acfe(pc), a0
pub fn i_04a68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A690;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4ACFE;
    16
}
/// $04A690: jsr $28cd4.l
pub fn i_04a690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A696;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A696);
    cpu.pc = 0x28CD4;
    16
}
/// $04A696: lea.l $4a69c(pc), a1
pub fn i_04a696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A69A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A69C;
    16
}
/// $04A69A: move.l a1, (a6)
pub fn i_04a69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A69C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A69C: bsr.w $4a268
pub fn i_04a69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6A0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A6A0);
    cpu.pc = 0x4A268;
    18
}
/// $04A6A0: jsr $28d70.l
pub fn i_04a6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A6A6);
    cpu.pc = 0x28D70;
    16
}
/// $04A6A6: bcc.w $4a6ae
pub fn i_04a6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6AA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A6AE; return 10; }
    8
}
/// $04A6AA: jmp $4a210(pc)
pub fn i_04a6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6AE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x4A210;
    8
}
/// $04A6AE: bra.w $4a35e
pub fn i_04a6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_InitBurst_04a6b2  ($04A6B2, 230 B, pow/human_death_049ff2.s)
/// $04A6B2: move.w #$0, d0
pub fn i_04a6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A6B6: jsr $5dca4.l
pub fn i_04a6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A6BC);
    cpu.pc = 0x5DCA4;
    16
}
/// $04A6BC: move.w d0, $28(a6)
pub fn i_04a6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6C0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A6C0: move.w #$cc6, $2a(a6)
pub fn i_04a6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6C6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xCC6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A6C6: move.w #$ff93, $2e(a6)
pub fn i_04a6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF93;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A6CC: move.w #$0, $2c(a6)
pub fn i_04a6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A6D2: jsr $5e9b6.l
pub fn i_04a6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A6D8);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A6D8: andi.w #$3ff, d0
pub fn i_04a6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6DC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04A6DC: move.w $22(a6), d1
pub fn i_04a6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6E0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A6E0: sub.w $54(a6), d1
pub fn i_04a6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6E4;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04A6E4: smi.b d1
pub fn i_04a6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6E6;
    cpu.ir = 0x5BC1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $04A6E6: ext.w d1
pub fn i_04a6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6E8;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A6E8: eor.w d1, d0
pub fn i_04a6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6EA;
    cpu.ir = 0xB340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04A6EA: move.w d0, $28(a6)
pub fn i_04a6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6EE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A6EE: jsr $5e9b6.l
pub fn i_04a6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A6F4);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A6F4: andi.w #$3ff, d0
pub fn i_04a6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6F8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04A6F8: add.w d0, $2a(a6)
pub fn i_04a6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A6FC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A6FC: clr.b $70(a6)
pub fn i_04a6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A700;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04A700: rts
pub fn i_04a700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A702;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $04A72E: addq.w #$4, $24(a6)
pub fn i_04a72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A732;
    cpu.ir = 0x586E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A732: lea.l $4af08(pc), a0
pub fn i_04a732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A736;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4AF08;
    16
}
/// $04A736: jsr $28cd4.l
pub fn i_04a736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A73C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A73C);
    cpu.pc = 0x28CD4;
    16
}
/// $04A73C: bsr.w $4a6b2
pub fn i_04a73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A740;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A740);
    cpu.pc = 0x4A6B2;
    18
}
/// $04A740: lea.l $ffff.w, a0
pub fn i_04a740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A744;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04A744: move.l a0, $48(a6)
pub fn i_04a744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A748;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04A748: bclr.b #$3, $13(a6)
pub fn i_04a748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A74E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A74E: lea.l $4a754(pc), a1
pub fn i_04a74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A752;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A754;
    16
}
/// $04A752: move.l a1, (a6)
pub fn i_04a752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A754;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A754: move.w $2a(a6), d0
pub fn i_04a754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A758;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A758: bpl.w $4a76a
pub fn i_04a758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A75C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x4A76A; return 10; }
    8
}
/// $04A75C: lea.l $4aeb4(pc), a0
pub fn i_04a75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A760;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4AEB4;
    16
}
/// $04A760: move.l a0, $48(a6)
pub fn i_04a760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A764;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04A764: lea.l $4a76a(pc), a1
pub fn i_04a764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A768;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A76A;
    16
}
/// $04A768: move.l a1, (a6)
pub fn i_04a768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A76A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A76A: jsr $4a268(pc)
pub fn i_04a76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A76E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4A76E);
    cpu.pc = 0x4A268;
    16
}
/// $04A76E: jsr $5e9b6.l
pub fn i_04a76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A774;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A774);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A774: jsr $4aba8(pc)
pub fn i_04a774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A778;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4A778);
    cpu.pc = 0x4ABA8;
    16
}
/// $04A778: jsr $2870a.l
pub fn i_04a778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A77E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A77E);
    cpu.pc = 0x2870A;
    16
}
/// $04A77E: bcc.w $4a786
pub fn i_04a77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A782;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A786; return 10; }
    8
}
/// $04A782: bsr.w $4a044
pub fn i_04a782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A786;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A786);
    cpu.pc = 0x4A044;
    18
}
/// $04A786: tst.b $70(a6)
pub fn i_04a786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A78A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A78A: beq.w $4a794
pub fn i_04a78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A78E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A794; return 10; }
    8
}
/// $04A78E: lea.l $4a798(pc), a1
pub fn i_04a78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A792;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A798;
    16
}
/// $04A792: move.l a1, (a6)
pub fn i_04a792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A794;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A794: bra.w $4a35e
pub fn i_04a794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A798;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_BurstLand_04a798  ($04A798, 92 B, pow/human_death_049ff2.s)
/// $04A798: lea.l $4af9e(pc), a0
pub fn i_04a798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A79C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4AF9E;
    16
}
/// $04A79C: jsr $28cd4.l
pub fn i_04a79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7A2);
    cpu.pc = 0x28CD4;
    16
}
/// $04A7A2: bra.w $4a382
pub fn i_04a7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A7B0: addq.w #$4, $24(a6)
pub fn i_04a7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7B4;
    cpu.ir = 0x586E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04A7B4: lea.l $4b078(pc), a0
pub fn i_04a7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7B8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B078;
    16
}
/// $04A7B8: jsr $28cd4.l
pub fn i_04a7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7BE);
    cpu.pc = 0x28CD4;
    16
}
/// $04A7BE: bsr.w $4a6b2
pub fn i_04a7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7C2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A7C2);
    cpu.pc = 0x4A6B2;
    18
}
/// $04A7C2: jsr $13600.l
pub fn i_04a7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7C8);
    cpu.pc = 0x13600;
    16
}
/// $04A7C8: move.w #$19, d1
pub fn i_04a7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7CC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A7CC: jsr $236e.l
pub fn i_04a7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7D2);
    cpu.pc = 0x236E;
    16
}
/// $04A7D2: lea.l $4a7d8(pc), a1
pub fn i_04a7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A7D8;
    16
}
/// $04A7D6: move.l a1, (a6)
pub fn i_04a7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A7D8: jsr $4a268(pc)
pub fn i_04a7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4A7DC);
    cpu.pc = 0x4A268;
    16
}
/// $04A7DC: jsr $28d70.l
pub fn i_04a7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7E2);
    cpu.pc = 0x28D70;
    16
}
/// $04A7E2: tst.b $70(a6)
pub fn i_04a7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7E6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A7E6: beq.w $4a7f0
pub fn i_04a7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7EA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A7F0; return 10; }
    8
}
/// $04A7EA: lea.l $4a7f4(pc), a1
pub fn i_04a7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A7F4;
    16
}
/// $04A7EE: move.l a1, (a6)
pub fn i_04a7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A7F0: bra.w $4a35e
pub fn i_04a7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_Launched_04a7f4  ($04A7F4, 444 B, pow/human_death_049ff2.s)
/// $04A7F4: lea.l $4b0c6(pc), a0
pub fn i_04a7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7F8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B0C6;
    16
}
/// $04A7F8: jsr $28cd4.l
pub fn i_04a7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A7FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A7FE);
    cpu.pc = 0x28CD4;
    16
}
/// $04A7FE: bra.w $4a382
pub fn i_04a7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A802;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A802: jsr $27f60.l
pub fn i_04a802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A808;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A808);
    cpu.pc = 0x27F60;
    16
}
/// $04A808: scc.b $70(a6)
pub fn i_04a808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A80C;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04A80C: jsr $13600.l
pub fn i_04a80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A812;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A812);
    cpu.pc = 0x13600;
    16
}
/// $04A812: move.w #$8a, d1
pub fn i_04a812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A816;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A816: jsr $236e.l
pub fn i_04a816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A81C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A81C);
    cpu.pc = 0x236E;
    16
}
/// $04A81C: move.w #$fffc, d0
pub fn i_04a81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A820;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A820: jsr $5dca4.l
pub fn i_04a820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A826);
    cpu.pc = 0x5DCA4;
    16
}
/// $04A826: move.w d0, $28(a6)
pub fn i_04a826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A82A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A82A: move.w #$870, $2a(a6)
pub fn i_04a82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A830;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x870;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A830: move.w #$ffb8, $2e(a6)
pub fn i_04a830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A836;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFB8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A836: move.w #$0, $2c(a6)
pub fn i_04a836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A83C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A83C: lea.l $4b7ec(pc), a0
pub fn i_04a83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A840;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B7EC;
    16
}
/// $04A840: jsr $28cd4.l
pub fn i_04a840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A846);
    cpu.pc = 0x28CD4;
    16
}
/// $04A846: move.w #$2800, $5c(a6)
pub fn i_04a846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A84C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A84C: move.w #$ffff, $38(a6)
pub fn i_04a84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A852;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A852: bset.b #$6, $12(a6)
pub fn i_04a852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A858;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04A858: lea.l $4a85e(pc), a1
pub fn i_04a858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A85C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A85E;
    16
}
/// $04A85C: move.l a1, (a6)
pub fn i_04a85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A85E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A85E: move.w $5c(a6), d0
pub fn i_04a85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A862;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A862: move.w d0, d1
pub fn i_04a862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A864;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A864: lsr.w #$5, d0
pub fn i_04a864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A866;
    cpu.ir = 0xEA48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04A866: add.w d0, d1
pub fn i_04a866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A868;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04A868: bcc.w $4a86e
pub fn i_04a868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A86C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4A86E; return 10; }
    8
}
/// $04A86C: moveq #$ff, d1
pub fn i_04a86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A86E;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04A86E: move.w d1, $5c(a6)
pub fn i_04a86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A872;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A872: lsr.w #$8, d1
pub fn i_04a872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A874;
    cpu.ir = 0xE049;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $04A874: move.b d1, $32(a6)
pub fn i_04a874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A878;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A878: move.b d1, $33(a6)
pub fn i_04a878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A87C;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A87C: jsr $27cee.l
pub fn i_04a87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A882);
    cpu.pc = 0x27CEE;
    16
}
/// $04A882: jsr $28d70.l
pub fn i_04a882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A888);
    cpu.pc = 0x28D70;
    16
}
/// $04A888: bra.w $4a35e
pub fn i_04a888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A88C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
/// $04A948: move.w #$8000, d0
pub fn i_04a948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A94C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A94C: jsr $28134.l
pub fn i_04a94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A952);
    cpu.pc = 0x28134;
    16
}
/// $04A952: andi.w #$ffe3, $38(a6)
pub fn i_04a952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A958;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04A958: ori.w #$0, $38(a6)
pub fn i_04a958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A95E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04A95E: jsr $5e9b6.l
pub fn i_04a95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A964);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A964: andi.b #$3, d0
pub fn i_04a964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A968;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04A968: beq.w $4a9be
pub fn i_04a968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A96C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A9BE; return 10; }
    8
}
/// $04A96C: lea.l $4b82a(pc), a0
pub fn i_04a96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A970;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B82A;
    16
}
/// $04A970: jsr $28cd4.l
pub fn i_04a970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A976);
    cpu.pc = 0x28CD4;
    16
}
/// $04A976: move.w #$1e, $30(a6)
pub fn i_04a976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A97C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A97C: lea.l $4a982(pc), a1
pub fn i_04a97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A980;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A982;
    16
}
/// $04A980: move.l a1, (a6)
pub fn i_04a980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A982;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A982: bsr.w $4a218
pub fn i_04a982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A986;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A986);
    cpu.pc = 0x4A218;
    18
}
/// $04A986: jsr $28d70.l
pub fn i_04a986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A98C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A98C);
    cpu.pc = 0x28D70;
    16
}
/// $04A98C: move.w $30(a6), d0
pub fn i_04a98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A990;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A990: beq.w $4a99e
pub fn i_04a990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A994;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A99E; return 10; }
    8
}
/// $04A994: subq.w #$1, d0
pub fn i_04a994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A996;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04A996: move.w d0, $30(a6)
pub fn i_04a996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A99A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A99A: bra.w $4a9ac
pub fn i_04a99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A99E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A9AC;
    10
}
/// $04A99E: tst.b $70(a6)
pub fn i_04a99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9A2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A9A2: beq.w $4a9ac
pub fn i_04a9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A9AC; return 10; }
    8
}
/// $04A9A6: lea.l $4a9b0(pc), a1
pub fn i_04a9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A9B0;
    16
}
/// $04A9AA: move.l a1, (a6)
pub fn i_04a9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x048AEA, i_048aea),
    (0x048AEC, i_048aec),
    (0x048AF0, i_048af0),
    (0x048AF6, i_048af6),
    (0x048AFA, i_048afa),
    (0x048AFE, i_048afe),
    (0x048B00, i_048b00),
    (0x048B06, i_048b06),
    (0x048B0C, i_048b0c),
    (0x048B12, i_048b12),
    (0x048B16, i_048b16),
    (0x048B1A, i_048b1a),
    (0x048B1C, i_048b1c),
    (0x048B1E, i_048b1e),
    (0x048B26, i_048b26),
    (0x048B2C, i_048b2c),
    (0x048B34, i_048b34),
    (0x048B38, i_048b38),
    (0x048B3E, i_048b3e),
    (0x048B44, i_048b44),
    (0x048B4A, i_048b4a),
    (0x048B50, i_048b50),
    (0x048B56, i_048b56),
    (0x048B5A, i_048b5a),
    (0x048B60, i_048b60),
    (0x048B66, i_048b66),
    (0x048B6C, i_048b6c),
    (0x048B70, i_048b70),
    (0x048B72, i_048b72),
    (0x048B78, i_048b78),
    (0x048B7E, i_048b7e),
    (0x048B82, i_048b82),
    (0x048B86, i_048b86),
    (0x048B88, i_048b88),
    (0x048B8E, i_048b8e),
    (0x048B94, i_048b94),
    (0x048B98, i_048b98),
    (0x048B9C, i_048b9c),
    (0x048B9E, i_048b9e),
    (0x048BA0, i_048ba0),
    (0x048BA6, i_048ba6),
    (0x048BAC, i_048bac),
    (0x048BB2, i_048bb2),
    (0x048BB6, i_048bb6),
    (0x048BB8, i_048bb8),
    (0x048BBC, i_048bbc),
    (0x048BC0, i_048bc0),
    (0x048BCA, i_048bca),
    (0x048BD0, i_048bd0),
    (0x048BD6, i_048bd6),
    (0x048BDA, i_048bda),
    (0x048BDE, i_048bde),
    (0x048BE0, i_048be0),
    (0x048BE6, i_048be6),
    (0x048BEC, i_048bec),
    (0x048BF0, i_048bf0),
    (0x048BF4, i_048bf4),
    (0x048BF6, i_048bf6),
    (0x048BF8, i_048bf8),
    (0x048BFA, i_048bfa),
    (0x048BFC, i_048bfc),
    (0x048C00, i_048c00),
    (0x048C04, i_048c04),
    (0x048C0A, i_048c0a),
    (0x048C0E, i_048c0e),
    (0x048C14, i_048c14),
    (0x048C18, i_048c18),
    (0x048C1E, i_048c1e),
    (0x048C24, i_048c24),
    (0x048C2A, i_048c2a),
    (0x048C30, i_048c30),
    (0x048C34, i_048c34),
    (0x048C3A, i_048c3a),
    (0x048C40, i_048c40),
    (0x048C46, i_048c46),
    (0x048C4A, i_048c4a),
    (0x048C4C, i_048c4c),
    (0x048C52, i_048c52),
    (0x048C56, i_048c56),
    (0x048C5A, i_048c5a),
    (0x048C5C, i_048c5c),
    (0x048C62, i_048c62),
    (0x048C66, i_048c66),
    (0x048C6C, i_048c6c),
    (0x048C70, i_048c70),
    (0x048C74, i_048c74),
    (0x048C76, i_048c76),
    (0x048C7C, i_048c7c),
    (0x048C82, i_048c82),
    (0x048C86, i_048c86),
    (0x048C8A, i_048c8a),
    (0x048C8C, i_048c8c),
    (0x048C92, i_048c92),
    (0x048C98, i_048c98),
    (0x048C9C, i_048c9c),
    (0x048CA0, i_048ca0),
    (0x048CA2, i_048ca2),
    (0x048CA4, i_048ca4),
    (0x048CA8, i_048ca8),
    (0x048CAE, i_048cae),
    (0x048CB2, i_048cb2),
    (0x048CB8, i_048cb8),
    (0x048CBE, i_048cbe),
    (0x048CC4, i_048cc4),
    (0x048CC8, i_048cc8),
    (0x048CCE, i_048cce),
    (0x048CD4, i_048cd4),
    (0x048CDA, i_048cda),
    (0x048CDE, i_048cde),
    (0x048CE0, i_048ce0),
    (0x048CE6, i_048ce6),
    (0x048CEA, i_048cea),
    (0x048CEE, i_048cee),
    (0x048CF0, i_048cf0),
    (0x048CF6, i_048cf6),
    (0x048CFC, i_048cfc),
    (0x048D02, i_048d02),
    (0x048D06, i_048d06),
    (0x048D0A, i_048d0a),
    (0x048D0C, i_048d0c),
    (0x048D0E, i_048d0e),
    (0x048D12, i_048d12),
    (0x048D18, i_048d18),
    (0x048D20, i_048d20),
    (0x048D26, i_048d26),
    (0x048D2C, i_048d2c),
    (0x048D30, i_048d30),
    (0x048D36, i_048d36),
    (0x048D3C, i_048d3c),
    (0x048D40, i_048d40),
    (0x048D42, i_048d42),
    (0x048D48, i_048d48),
    (0x048D4C, i_048d4c),
    (0x048D52, i_048d52),
    (0x048D58, i_048d58),
    (0x048D5C, i_048d5c),
    (0x048D62, i_048d62),
    (0x048D66, i_048d66),
    (0x048D6C, i_048d6c),
    (0x048D70, i_048d70),
    (0x048D74, i_048d74),
    (0x048D76, i_048d76),
    (0x048D7A, i_048d7a),
    (0x048D7E, i_048d7e),
    (0x048D82, i_048d82),
    (0x048D84, i_048d84),
    (0x048D88, i_048d88),
    (0x048D8E, i_048d8e),
    (0x048D90, i_048d90),
    (0x048D94, i_048d94),
    (0x048D9A, i_048d9a),
    (0x048D9E, i_048d9e),
    (0x048DA4, i_048da4),
    (0x048DA8, i_048da8),
    (0x048DAA, i_048daa),
    (0x048DB0, i_048db0),
    (0x048DB4, i_048db4),
    (0x048DBA, i_048dba),
    (0x048DBE, i_048dbe),
    (0x048DC4, i_048dc4),
    (0x048DC8, i_048dc8),
    (0x048DCE, i_048dce),
    (0x048DD2, i_048dd2),
    (0x048DD6, i_048dd6),
    (0x048DD8, i_048dd8),
    (0x048DDC, i_048ddc),
    (0x048DE2, i_048de2),
    (0x048DE8, i_048de8),
    (0x048DEC, i_048dec),
    (0x048DF2, i_048df2),
    (0x048DF8, i_048df8),
    (0x048E00, i_048e00),
    (0x048E04, i_048e04),
    (0x048E06, i_048e06),
    (0x048E0C, i_048e0c),
    (0x048E12, i_048e12),
    (0x048E16, i_048e16),
    (0x048E1A, i_048e1a),
    (0x048E20, i_048e20),
    (0x048E24, i_048e24),
    (0x048E28, i_048e28),
    (0x048E2A, i_048e2a),
    (0x048E2C, i_048e2c),
    (0x048E30, i_048e30),
    (0x048E36, i_048e36),
    (0x048E3A, i_048e3a),
    (0x048E3E, i_048e3e),
    (0x048E40, i_048e40),
    (0x048E42, i_048e42),
    (0x048E48, i_048e48),
    (0x048E4C, i_048e4c),
    (0x048E50, i_048e50),
    (0x048E52, i_048e52),
    (0x048E54, i_048e54),
    (0x048E5A, i_048e5a),
    (0x048E5E, i_048e5e),
    (0x048E64, i_048e64),
    (0x048E6A, i_048e6a),
    (0x048E6E, i_048e6e),
    (0x048E72, i_048e72),
    (0x048E76, i_048e76),
    (0x048E7C, i_048e7c),
    (0x048E82, i_048e82),
    (0x048E88, i_048e88),
    (0x048E8E, i_048e8e),
    (0x048E92, i_048e92),
    (0x048E94, i_048e94),
    (0x048E9A, i_048e9a),
    (0x048E9E, i_048e9e),
    (0x048EA2, i_048ea2),
    (0x048EA4, i_048ea4),
    (0x048EA6, i_048ea6),
    (0x048EAC, i_048eac),
    (0x048EB2, i_048eb2),
    (0x048EB6, i_048eb6),
    (0x048EBC, i_048ebc),
    (0x048EC0, i_048ec0),
    (0x048EC6, i_048ec6),
    (0x048ECC, i_048ecc),
    (0x048ED4, i_048ed4),
    (0x048EDC, i_048edc),
    (0x048EE2, i_048ee2),
    (0x048EE8, i_048ee8),
    (0x048EEC, i_048eec),
    (0x048EF0, i_048ef0),
    (0x048EF6, i_048ef6),
    (0x048EFC, i_048efc),
    (0x048F02, i_048f02),
    (0x048F04, i_048f04),
    (0x048F08, i_048f08),
    (0x048F0A, i_048f0a),
    (0x048F10, i_048f10),
    (0x048F14, i_048f14),
    (0x048F16, i_048f16),
    (0x048F1A, i_048f1a),
    (0x048F1C, i_048f1c),
    (0x048F20, i_048f20),
    (0x048F26, i_048f26),
    (0x048F2C, i_048f2c),
    (0x048F2E, i_048f2e),
    (0x048F34, i_048f34),
    (0x048F3A, i_048f3a),
    (0x048F3E, i_048f3e),
    (0x048F44, i_048f44),
    (0x048F48, i_048f48),
    (0x048F4E, i_048f4e),
    (0x048F52, i_048f52),
    (0x048F54, i_048f54),
    (0x048F58, i_048f58),
    (0x048F5C, i_048f5c),
    (0x048F60, i_048f60),
    (0x048F64, i_048f64),
    (0x048F6A, i_048f6a),
    (0x048F6E, i_048f6e),
    (0x048F72, i_048f72),
    (0x048F74, i_048f74),
    (0x048F78, i_048f78),
    (0x048F7C, i_048f7c),
    (0x048F7E, i_048f7e),
    (0x048F82, i_048f82),
    (0x048F86, i_048f86),
    (0x048F8A, i_048f8a),
    (0x048F90, i_048f90),
    (0x048F94, i_048f94),
    (0x048F98, i_048f98),
    (0x048F9E, i_048f9e),
    (0x048FA2, i_048fa2),
    (0x048FA4, i_048fa4),
    (0x048FAA, i_048faa),
    (0x048FAE, i_048fae),
    (0x048FB0, i_048fb0),
    (0x048FB6, i_048fb6),
    (0x048FBC, i_048fbc),
    (0x048FC0, i_048fc0),
    (0x048FC6, i_048fc6),
    (0x048FCA, i_048fca),
    (0x048FCC, i_048fcc),
    (0x048FD0, i_048fd0),
    (0x048FD4, i_048fd4),
    (0x048FDA, i_048fda),
    (0x048FDE, i_048fde),
    (0x048FE2, i_048fe2),
    (0x048FE6, i_048fe6),
    (0x048FEA, i_048fea),
    (0x048FEE, i_048fee),
    (0x048FF4, i_048ff4),
    (0x048FF8, i_048ff8),
    (0x048FFC, i_048ffc),
    (0x049000, i_049000),
    (0x049002, i_049002),
    (0x049004, i_049004),
    (0x049008, i_049008),
    (0x04900A, i_04900a),
    (0x04900E, i_04900e),
    (0x049010, i_049010),
    (0x049016, i_049016),
    (0x04901A, i_04901a),
    (0x049020, i_049020),
    (0x049024, i_049024),
    (0x049028, i_049028),
    (0x04902C, i_04902c),
    (0x049030, i_049030),
    (0x049034, i_049034),
    (0x049038, i_049038),
    (0x04903C, i_04903c),
    (0x049040, i_049040),
    (0x049044, i_049044),
    (0x049048, i_049048),
    (0x04904C, i_04904c),
    (0x04904E, i_04904e),
    (0x049052, i_049052),
    (0x049054, i_049054),
    (0x049058, i_049058),
    (0x04905E, i_04905e),
    (0x049060, i_049060),
    (0x049062, i_049062),
    (0x049066, i_049066),
    (0x049068, i_049068),
    (0x04906C, i_04906c),
    (0x049070, i_049070),
    (0x049072, i_049072),
    (0x049076, i_049076),
    (0x04907C, i_04907c),
    (0x049080, i_049080),
    (0x049084, i_049084),
    (0x049088, i_049088),
    (0x04908A, i_04908a),
    (0x04908E, i_04908e),
    (0x049092, i_049092),
    (0x049096, i_049096),
    (0x04909A, i_04909a),
    (0x04909E, i_04909e),
    (0x0490A0, i_0490a0),
    (0x0490A4, i_0490a4),
    (0x0490A8, i_0490a8),
    (0x0490AC, i_0490ac),
    (0x0490B0, i_0490b0),
    (0x0490B4, i_0490b4),
    (0x0490B8, i_0490b8),
    (0x0490BC, i_0490bc),
    (0x0490C0, i_0490c0),
    (0x0490C6, i_0490c6),
    (0x0490CA, i_0490ca),
    (0x0490CE, i_0490ce),
    (0x0490D4, i_0490d4),
    (0x0490D8, i_0490d8),
    (0x0490DC, i_0490dc),
    (0x0490E0, i_0490e0),
    (0x0490E2, i_0490e2),
    (0x0490E8, i_0490e8),
    (0x0490EC, i_0490ec),
    (0x0490F0, i_0490f0),
    (0x0490F4, i_0490f4),
    (0x0490F8, i_0490f8),
    (0x0490FA, i_0490fa),
    (0x049100, i_049100),
    (0x049104, i_049104),
    (0x049108, i_049108),
    (0x04910C, i_04910c),
    (0x049110, i_049110),
    (0x049114, i_049114),
    (0x049118, i_049118),
    (0x04911C, i_04911c),
    (0x049120, i_049120),
    (0x049122, i_049122),
    (0x049126, i_049126),
    (0x049128, i_049128),
    (0x04912C, i_04912c),
    (0x049130, i_049130),
    (0x049132, i_049132),
    (0x049138, i_049138),
    (0x04913C, i_04913c),
    (0x049140, i_049140),
    (0x049146, i_049146),
    (0x04914A, i_04914a),
    (0x04914C, i_04914c),
    (0x049150, i_049150),
    (0x049156, i_049156),
    (0x049158, i_049158),
    (0x04915C, i_04915c),
    (0x049160, i_049160),
    (0x049162, i_049162),
    (0x049166, i_049166),
    (0x04916A, i_04916a),
    (0x04916C, i_04916c),
    (0x049170, i_049170),
    (0x049172, i_049172),
    (0x049176, i_049176),
    (0x04917A, i_04917a),
    (0x04917E, i_04917e),
    (0x049182, i_049182),
    (0x049186, i_049186),
    (0x04918A, i_04918a),
    (0x04918E, i_04918e),
    (0x049190, i_049190),
    (0x049194, i_049194),
    (0x049196, i_049196),
    (0x04919A, i_04919a),
    (0x04919E, i_04919e),
    (0x0491A4, i_0491a4),
    (0x0491AA, i_0491aa),
    (0x0491AE, i_0491ae),
    (0x0491B2, i_0491b2),
    (0x0491B6, i_0491b6),
    (0x0491BA, i_0491ba),
    (0x0491C0, i_0491c0),
    (0x0491C4, i_0491c4),
    (0x0491C8, i_0491c8),
    (0x0491CA, i_0491ca),
    (0x0491CE, i_0491ce),
    (0x0491D2, i_0491d2),
    (0x0491D6, i_0491d6),
    (0x0491D8, i_0491d8),
    (0x0491DC, i_0491dc),
    (0x0491DE, i_0491de),
    (0x0491E4, i_0491e4),
    (0x0491EA, i_0491ea),
    (0x0491EE, i_0491ee),
    (0x0491F2, i_0491f2),
    (0x0491F6, i_0491f6),
    (0x0491FA, i_0491fa),
    (0x049200, i_049200),
    (0x049204, i_049204),
    (0x049208, i_049208),
    (0x04920A, i_04920a),
    (0x04920E, i_04920e),
    (0x049212, i_049212),
    (0x049216, i_049216),
    (0x049218, i_049218),
    (0x04921C, i_04921c),
    (0x04921E, i_04921e),
    (0x049224, i_049224),
    (0x04922A, i_04922a),
    (0x04922E, i_04922e),
    (0x049232, i_049232),
    (0x049238, i_049238),
    (0x04923C, i_04923c),
    (0x049240, i_049240),
    (0x049242, i_049242),
    (0x049246, i_049246),
    (0x04924A, i_04924a),
    (0x04924E, i_04924e),
    (0x049250, i_049250),
    (0x049254, i_049254),
    (0x049256, i_049256),
    (0x04925A, i_04925a),
    (0x049260, i_049260),
    (0x049264, i_049264),
    (0x04926A, i_04926a),
    (0x049270, i_049270),
    (0x049276, i_049276),
    (0x04927A, i_04927a),
    (0x049280, i_049280),
    (0x049286, i_049286),
    (0x04928A, i_04928a),
    (0x04928E, i_04928e),
    (0x049294, i_049294),
    (0x049298, i_049298),
    (0x04929C, i_04929c),
    (0x04929E, i_04929e),
    (0x0492A2, i_0492a2),
    (0x0492A4, i_0492a4),
    (0x0492AA, i_0492aa),
    (0x0492AE, i_0492ae),
    (0x0492B2, i_0492b2),
    (0x0492B6, i_0492b6),
    (0x0492BA, i_0492ba),
    (0x0492BE, i_0492be),
    (0x0492C2, i_0492c2),
    (0x0492C6, i_0492c6),
    (0x0492C8, i_0492c8),
    (0x0492CC, i_0492cc),
    (0x0492CE, i_0492ce),
    (0x0492D4, i_0492d4),
    (0x0492D8, i_0492d8),
    (0x0492DC, i_0492dc),
    (0x0492E0, i_0492e0),
    (0x0492E4, i_0492e4),
    (0x0492E8, i_0492e8),
    (0x0492EC, i_0492ec),
    (0x0492F0, i_0492f0),
    (0x0492F2, i_0492f2),
    (0x0492F6, i_0492f6),
    (0x0492F8, i_0492f8),
    (0x0492FE, i_0492fe),
    (0x049302, i_049302),
    (0x049308, i_049308),
    (0x04930C, i_04930c),
    (0x049312, i_049312),
    (0x049316, i_049316),
    (0x04931C, i_04931c),
    (0x049320, i_049320),
    (0x049324, i_049324),
    (0x049326, i_049326),
    (0x04932A, i_04932a),
    (0x04932C, i_04932c),
    (0x049332, i_049332),
    (0x049336, i_049336),
    (0x04933A, i_04933a),
    (0x04933C, i_04933c),
    (0x049340, i_049340),
    (0x049344, i_049344),
    (0x049346, i_049346),
    (0x04934C, i_04934c),
    (0x049350, i_049350),
    (0x049354, i_049354),
    (0x04935A, i_04935a),
    (0x04935E, i_04935e),
    (0x049362, i_049362),
    (0x049364, i_049364),
    (0x049368, i_049368),
    (0x04936C, i_04936c),
    (0x04936E, i_04936e),
    (0x049372, i_049372),
    (0x049378, i_049378),
    (0x04937C, i_04937c),
    (0x049382, i_049382),
    (0x049386, i_049386),
    (0x049388, i_049388),
    (0x04938C, i_04938c),
    (0x049390, i_049390),
    (0x049394, i_049394),
    (0x049398, i_049398),
    (0x04939A, i_04939a),
    (0x04939C, i_04939c),
    (0x0493A0, i_0493a0),
    (0x0493A4, i_0493a4),
    (0x0493A8, i_0493a8),
    (0x0493AC, i_0493ac),
    (0x0493B0, i_0493b0),
    (0x0493B4, i_0493b4),
    (0x0493B8, i_0493b8),
    (0x0493BA, i_0493ba),
    (0x0493BE, i_0493be),
    (0x0493C0, i_0493c0),
    (0x0493C4, i_0493c4),
    (0x0493C8, i_0493c8),
    (0x0493CC, i_0493cc),
    (0x0493D0, i_0493d0),
    (0x0493D4, i_0493d4),
    (0x0493D8, i_0493d8),
    (0x0493DC, i_0493dc),
    (0x0493DE, i_0493de),
    (0x0493E2, i_0493e2),
    (0x0493E4, i_0493e4),
    (0x0493EA, i_0493ea),
    (0x0493EE, i_0493ee),
    (0x0493F0, i_0493f0),
    (0x0493F2, i_0493f2),
    (0x0493F6, i_0493f6),
    (0x04940E, i_04940e),
    (0x049414, i_049414),
    (0x049418, i_049418),
    (0x04941E, i_04941e),
    (0x049420, i_049420),
    (0x049422, i_049422),
    (0x049426, i_049426),
    (0x04942A, i_04942a),
    (0x049430, i_049430),
    (0x049436, i_049436),
    (0x04943A, i_04943a),
    (0x04943C, i_04943c),
    (0x049440, i_049440),
    (0x049444, i_049444),
    (0x049448, i_049448),
    (0x04944C, i_04944c),
    (0x049452, i_049452),
    (0x049458, i_049458),
    (0x04945C, i_04945c),
    (0x04945E, i_04945e),
    (0x049462, i_049462),
    (0x049466, i_049466),
    (0x04946A, i_04946a),
    (0x049470, i_049470),
    (0x049472, i_049472),
    (0x049476, i_049476),
    (0x04947C, i_04947c),
    (0x049482, i_049482),
    (0x049486, i_049486),
    (0x04948C, i_04948c),
    (0x049490, i_049490),
    (0x049492, i_049492),
    (0x049496, i_049496),
    (0x04949C, i_04949c),
    (0x0494A0, i_0494a0),
    (0x0494A6, i_0494a6),
    (0x0494AC, i_0494ac),
    (0x0494B0, i_0494b0),
    (0x0494B6, i_0494b6),
    (0x0494BA, i_0494ba),
    (0x0494BC, i_0494bc),
    (0x0494C0, i_0494c0),
    (0x0494C6, i_0494c6),
    (0x0494C8, i_0494c8),
    (0x0494CC, i_0494cc),
    (0x0494D2, i_0494d2),
    (0x0494D8, i_0494d8),
    (0x0494DC, i_0494dc),
    (0x0494E2, i_0494e2),
    (0x0494E6, i_0494e6),
    (0x0494E8, i_0494e8),
    (0x0494EC, i_0494ec),
    (0x0494F2, i_0494f2),
    (0x0494F6, i_0494f6),
    (0x0494FC, i_0494fc),
    (0x049502, i_049502),
    (0x049506, i_049506),
    (0x04950C, i_04950c),
    (0x049510, i_049510),
    (0x049512, i_049512),
    (0x049516, i_049516),
    (0x04951C, i_04951c),
    (0x04951E, i_04951e),
    (0x049524, i_049524),
    (0x04952A, i_04952a),
    (0x04952E, i_04952e),
    (0x049532, i_049532),
    (0x049536, i_049536),
    (0x04953A, i_04953a),
    (0x04953E, i_04953e),
    (0x049542, i_049542),
    (0x049544, i_049544),
    (0x049548, i_049548),
    (0x04954A, i_04954a),
    (0x049550, i_049550),
    (0x04955A, i_04955a),
    (0x04955E, i_04955e),
    (0x049564, i_049564),
    (0x04956A, i_04956a),
    (0x049570, i_049570),
    (0x049576, i_049576),
    (0x04957C, i_04957c),
    (0x049582, i_049582),
    (0x049588, i_049588),
    (0x04958E, i_04958e),
    (0x049592, i_049592),
    (0x049598, i_049598),
    (0x04959E, i_04959e),
    (0x0495A2, i_0495a2),
    (0x0495A8, i_0495a8),
    (0x0495AE, i_0495ae),
    (0x0495B2, i_0495b2),
    (0x0495B4, i_0495b4),
    (0x0495B8, i_0495b8),
    (0x0495BA, i_0495ba),
    (0x0495BC, i_0495bc),
    (0x0495C0, i_0495c0),
    (0x0495C6, i_0495c6),
    (0x0495CA, i_0495ca),
    (0x0495D0, i_0495d0),
    (0x0495D6, i_0495d6),
    (0x0495D8, i_0495d8),
    (0x0495DC, i_0495dc),
    (0x0495E0, i_0495e0),
    (0x0495E6, i_0495e6),
    (0x0495E8, i_0495e8),
    (0x0495EC, i_0495ec),
    (0x0495F2, i_0495f2),
    (0x0495F6, i_0495f6),
    (0x0495FC, i_0495fc),
    (0x049602, i_049602),
    (0x049606, i_049606),
    (0x049608, i_049608),
    (0x04960C, i_04960c),
    (0x049610, i_049610),
    (0x049614, i_049614),
    (0x049618, i_049618),
    (0x04961E, i_04961e),
    (0x049622, i_049622),
    (0x049628, i_049628),
    (0x04962C, i_04962c),
    (0x049630, i_049630),
    (0x049634, i_049634),
    (0x049636, i_049636),
    (0x049638, i_049638),
    (0x04963C, i_04963c),
    (0x049640, i_049640),
    (0x049646, i_049646),
    (0x049648, i_049648),
    (0x04964C, i_04964c),
    (0x049650, i_049650),
    (0x049656, i_049656),
    (0x04965A, i_04965a),
    (0x049660, i_049660),
    (0x049664, i_049664),
    (0x049666, i_049666),
    (0x04966C, i_04966c),
    (0x049672, i_049672),
    (0x049678, i_049678),
    (0x04967C, i_04967c),
    (0x049682, i_049682),
    (0x049688, i_049688),
    (0x049742, i_049742),
    (0x049746, i_049746),
    (0x04974A, i_04974a),
    (0x04974C, i_04974c),
    (0x04974E, i_04974e),
    (0x049752, i_049752),
    (0x049756, i_049756),
    (0x04975C, i_04975c),
    (0x049762, i_049762),
    (0x049768, i_049768),
    (0x04976E, i_04976e),
    (0x049774, i_049774),
    (0x04977A, i_04977a),
    (0x049780, i_049780),
    (0x049784, i_049784),
    (0x049788, i_049788),
    (0x04978A, i_04978a),
    (0x04978E, i_04978e),
    (0x049792, i_049792),
    (0x049794, i_049794),
    (0x04979A, i_04979a),
    (0x0497A0, i_0497a0),
    (0x0497A4, i_0497a4),
    (0x0497AA, i_0497aa),
    (0x0497AC, i_0497ac),
    (0x0497B0, i_0497b0),
    (0x0497B6, i_0497b6),
    (0x0497BC, i_0497bc),
    (0x0497C2, i_0497c2),
    (0x0497C6, i_0497c6),
    (0x0497CC, i_0497cc),
    (0x0497D0, i_0497d0),
    (0x0497D4, i_0497d4),
    (0x0497D6, i_0497d6),
    (0x0497DC, i_0497dc),
    (0x0497E0, i_0497e0),
    (0x0497E2, i_0497e2),
    (0x0497E6, i_0497e6),
    (0x0497EA, i_0497ea),
    (0x0497F0, i_0497f0),
    (0x0497F2, i_0497f2),
    (0x0497F6, i_0497f6),
    (0x0497FC, i_0497fc),
    (0x049800, i_049800),
    (0x049806, i_049806),
    (0x04980C, i_04980c),
    (0x049812, i_049812),
    (0x049816, i_049816),
    (0x04981A, i_04981a),
    (0x049820, i_049820),
    (0x049824, i_049824),
    (0x049826, i_049826),
    (0x04982A, i_04982a),
    (0x04982E, i_04982e),
    (0x049834, i_049834),
    (0x049838, i_049838),
    (0x04983C, i_04983c),
    (0x04983E, i_04983e),
    (0x049842, i_049842),
    (0x049848, i_049848),
    (0x04984C, i_04984c),
    (0x049850, i_049850),
    (0x049854, i_049854),
    (0x049856, i_049856),
    (0x04985A, i_04985a),
    (0x049860, i_049860),
    (0x049864, i_049864),
    (0x049868, i_049868),
    (0x04986C, i_04986c),
    (0x04986E, i_04986e),
    (0x049872, i_049872),
    (0x049878, i_049878),
    (0x04987C, i_04987c),
    (0x049882, i_049882),
    (0x04992E, i_04992e),
    (0x049932, i_049932),
    (0x049938, i_049938),
    (0x04993E, i_04993e),
    (0x049944, i_049944),
    (0x049948, i_049948),
    (0x04994A, i_04994a),
    (0x049950, i_049950),
    (0x049956, i_049956),
    (0x04995A, i_04995a),
    (0x049960, i_049960),
    (0x049964, i_049964),
    (0x04996A, i_04996a),
    (0x04996E, i_04996e),
    (0x049974, i_049974),
    (0x049976, i_049976),
    (0x04997A, i_04997a),
    (0x049980, i_049980),
    (0x049986, i_049986),
    (0x04998C, i_04998c),
    (0x049992, i_049992),
    (0x049996, i_049996),
    (0x049998, i_049998),
    (0x04999E, i_04999e),
    (0x0499A4, i_0499a4),
    (0x0499A8, i_0499a8),
    (0x0499AE, i_0499ae),
    (0x0499B2, i_0499b2),
    (0x0499B6, i_0499b6),
    (0x0499BC, i_0499bc),
    (0x0499C0, i_0499c0),
    (0x0499C6, i_0499c6),
    (0x0499CA, i_0499ca),
    (0x0499D0, i_0499d0),
    (0x0499D4, i_0499d4),
    (0x0499DA, i_0499da),
    (0x0499DC, i_0499dc),
    (0x0499E2, i_0499e2),
    (0x0499E8, i_0499e8),
    (0x0499EC, i_0499ec),
    (0x0499F0, i_0499f0),
    (0x0499F6, i_0499f6),
    (0x0499FA, i_0499fa),
    (0x0499FE, i_0499fe),
    (0x049A00, i_049a00),
    (0x049A04, i_049a04),
    (0x049A08, i_049a08),
    (0x049A0A, i_049a0a),
    (0x049A0E, i_049a0e),
    (0x049A12, i_049a12),
    (0x049A14, i_049a14),
    (0x049A18, i_049a18),
    (0x049A1E, i_049a1e),
    (0x049A20, i_049a20),
    (0x049A26, i_049a26),
    (0x049A2C, i_049a2c),
    (0x049A30, i_049a30),
    (0x049A36, i_049a36),
    (0x049A3A, i_049a3a),
    (0x049A3E, i_049a3e),
    (0x049A42, i_049a42),
    (0x049A48, i_049a48),
    (0x049A4A, i_049a4a),
    (0x049A4E, i_049a4e),
    (0x049A52, i_049a52),
    (0x049A56, i_049a56),
    (0x049A5A, i_049a5a),
    (0x049A5C, i_049a5c),
    (0x049A62, i_049a62),
    (0x049A66, i_049a66),
    (0x049A6A, i_049a6a),
    (0x049A6E, i_049a6e),
    (0x049A72, i_049a72),
    (0x049A78, i_049a78),
    (0x049A7C, i_049a7c),
    (0x049A80, i_049a80),
    (0x049A84, i_049a84),
    (0x049A88, i_049a88),
    (0x049A8C, i_049a8c),
    (0x049A90, i_049a90),
    (0x049A92, i_049a92),
    (0x049A96, i_049a96),
    (0x049A9A, i_049a9a),
    (0x049A9E, i_049a9e),
    (0x049AA2, i_049aa2),
    (0x049AA6, i_049aa6),
    (0x049AAA, i_049aaa),
    (0x049AAE, i_049aae),
    (0x049AB4, i_049ab4),
    (0x049ABA, i_049aba),
    (0x049ABE, i_049abe),
    (0x049AC2, i_049ac2),
    (0x049AC6, i_049ac6),
    (0x049ACA, i_049aca),
    (0x049ACE, i_049ace),
    (0x049AD4, i_049ad4),
    (0x049ADA, i_049ada),
    (0x049ADE, i_049ade),
    (0x049AE0, i_049ae0),
    (0x049AE4, i_049ae4),
    (0x049AE8, i_049ae8),
    (0x049AEE, i_049aee),
    (0x049AF0, i_049af0),
    (0x049AF4, i_049af4),
    (0x049AFA, i_049afa),
    (0x049AFE, i_049afe),
    (0x049B04, i_049b04),
    (0x049B06, i_049b06),
    (0x049B0A, i_049b0a),
    (0x049B10, i_049b10),
    (0x049B14, i_049b14),
    (0x049B1A, i_049b1a),
    (0x049B1E, i_049b1e),
    (0x049B24, i_049b24),
    (0x049B2A, i_049b2a),
    (0x049B2E, i_049b2e),
    (0x049B34, i_049b34),
    (0x049B3A, i_049b3a),
    (0x049B40, i_049b40),
    (0x049B44, i_049b44),
    (0x049B48, i_049b48),
    (0x049B4C, i_049b4c),
    (0x049B50, i_049b50),
    (0x049B54, i_049b54),
    (0x049B56, i_049b56),
    (0x049B58, i_049b58),
    (0x049B5E, i_049b5e),
    (0x049B62, i_049b62),
    (0x049B68, i_049b68),
    (0x049B6C, i_049b6c),
    (0x049B70, i_049b70),
    (0x049B72, i_049b72),
    (0x049B76, i_049b76),
    (0x049B78, i_049b78),
    (0x049B7E, i_049b7e),
    (0x049B82, i_049b82),
    (0x049B86, i_049b86),
    (0x049B88, i_049b88),
    (0x049B8C, i_049b8c),
    (0x049B8E, i_049b8e),
    (0x049B92, i_049b92),
    (0x049B96, i_049b96),
    (0x049B9A, i_049b9a),
    (0x049B9E, i_049b9e),
    (0x049BA2, i_049ba2),
    (0x049BA4, i_049ba4),
    (0x049BA8, i_049ba8),
    (0x049BAA, i_049baa),
    (0x049BB0, i_049bb0),
    (0x049BB4, i_049bb4),
    (0x049BBA, i_049bba),
    (0x049BC0, i_049bc0),
    (0x049BC6, i_049bc6),
    (0x049BCC, i_049bcc),
    (0x049BD2, i_049bd2),
    (0x049BD8, i_049bd8),
    (0x049BDE, i_049bde),
    (0x049BE4, i_049be4),
    (0x049BE6, i_049be6),
    (0x049BEA, i_049bea),
    (0x049BEE, i_049bee),
    (0x049BF2, i_049bf2),
    (0x049BF6, i_049bf6),
    (0x049BF8, i_049bf8),
    (0x049BFC, i_049bfc),
    (0x049C02, i_049c02),
    (0x049C08, i_049c08),
    (0x049C0A, i_049c0a),
    (0x049C0E, i_049c0e),
    (0x049C12, i_049c12),
    (0x049C18, i_049c18),
    (0x049C1C, i_049c1c),
    (0x049C20, i_049c20),
    (0x049C24, i_049c24),
    (0x049C26, i_049c26),
    (0x049C2A, i_049c2a),
    (0x049C2C, i_049c2c),
    (0x049C30, i_049c30),
    (0x049C36, i_049c36),
    (0x049C3A, i_049c3a),
    (0x049C40, i_049c40),
    (0x049C46, i_049c46),
    (0x049C4C, i_049c4c),
    (0x049C52, i_049c52),
    (0x049C58, i_049c58),
    (0x049C5C, i_049c5c),
    (0x049C5E, i_049c5e),
    (0x049C64, i_049c64),
    (0x049C6A, i_049c6a),
    (0x049C70, i_049c70),
    (0x049C74, i_049c74),
    (0x049C7A, i_049c7a),
    (0x049C80, i_049c80),
    (0x049C86, i_049c86),
    (0x049C8C, i_049c8c),
    (0x049C92, i_049c92),
    (0x049C96, i_049c96),
    (0x049C9C, i_049c9c),
    (0x049CA2, i_049ca2),
    (0x049CA4, i_049ca4),
    (0x049CAA, i_049caa),
    (0x049CAE, i_049cae),
    (0x049CB4, i_049cb4),
    (0x049CB8, i_049cb8),
    (0x049CBE, i_049cbe),
    (0x049CC4, i_049cc4),
    (0x049CCA, i_049cca),
    (0x049CCE, i_049cce),
    (0x049CD4, i_049cd4),
    (0x049CDA, i_049cda),
    (0x049CDE, i_049cde),
    (0x049CE2, i_049ce2),
    (0x049CE4, i_049ce4),
    (0x049CE8, i_049ce8),
    (0x049CEE, i_049cee),
    (0x049CF2, i_049cf2),
    (0x049CF8, i_049cf8),
    (0x049CFC, i_049cfc),
    (0x049D02, i_049d02),
    (0x049D06, i_049d06),
    (0x049D0C, i_049d0c),
    (0x049D10, i_049d10),
    (0x049D12, i_049d12),
    (0x049D16, i_049d16),
    (0x049D1A, i_049d1a),
    (0x049D1E, i_049d1e),
    (0x049D24, i_049d24),
    (0x049D2A, i_049d2a),
    (0x049D2E, i_049d2e),
    (0x049D32, i_049d32),
    (0x049D38, i_049d38),
    (0x049D3E, i_049d3e),
    (0x049D44, i_049d44),
    (0x049D4A, i_049d4a),
    (0x049D50, i_049d50),
    (0x049D56, i_049d56),
    (0x049D5C, i_049d5c),
    (0x049D62, i_049d62),
    (0x049D66, i_049d66),
    (0x049D6C, i_049d6c),
    (0x049D6E, i_049d6e),
    (0x049D72, i_049d72),
    (0x049D76, i_049d76),
    (0x049D7A, i_049d7a),
    (0x049D7E, i_049d7e),
    (0x049D82, i_049d82),
    (0x049D84, i_049d84),
    (0x049D88, i_049d88),
    (0x049FBA, i_049fba),
    (0x049FC0, i_049fc0),
    (0x049FC4, i_049fc4),
    (0x049FC8, i_049fc8),
    (0x049FCA, i_049fca),
    (0x049FD0, i_049fd0),
    (0x049FD2, i_049fd2),
    (0x049FD6, i_049fd6),
    (0x049FDC, i_049fdc),
    (0x049FE0, i_049fe0),
    (0x049FE4, i_049fe4),
    (0x049FE6, i_049fe6),
    (0x049FEA, i_049fea),
    (0x049FEE, i_049fee),
    (0x049FF0, i_049ff0),
    (0x049FF2, i_049ff2),
    (0x049FF4, i_049ff4),
    (0x049FF8, i_049ff8),
    (0x049FFE, i_049ffe),
    (0x04A002, i_04a002),
    (0x04A006, i_04a006),
    (0x04A008, i_04a008),
    (0x04A00C, i_04a00c),
    (0x04A010, i_04a010),
    (0x04A012, i_04a012),
    (0x04A014, i_04a014),
    (0x04A01A, i_04a01a),
    (0x04A01E, i_04a01e),
    (0x04A020, i_04a020),
    (0x04A024, i_04a024),
    (0x04A02A, i_04a02a),
    (0x04A02E, i_04a02e),
    (0x04A030, i_04a030),
    (0x04A034, i_04a034),
    (0x04A03A, i_04a03a),
    (0x04A03E, i_04a03e),
    (0x04A040, i_04a040),
    (0x04A044, i_04a044),
    (0x04A04A, i_04a04a),
    (0x04A04E, i_04a04e),
    (0x04A050, i_04a050),
    (0x04A054, i_04a054),
    (0x04A058, i_04a058),
    (0x04A05A, i_04a05a),
    (0x04A060, i_04a060),
    (0x04A062, i_04a062),
    (0x04A064, i_04a064),
    (0x04A068, i_04a068),
    (0x04A06C, i_04a06c),
    (0x04A06E, i_04a06e),
    (0x04A072, i_04a072),
    (0x04A076, i_04a076),
    (0x04A078, i_04a078),
    (0x04A07C, i_04a07c),
    (0x04A080, i_04a080),
    (0x04A084, i_04a084),
    (0x04A088, i_04a088),
    (0x04A08A, i_04a08a),
    (0x04A08E, i_04a08e),
    (0x04A092, i_04a092),
    (0x04A098, i_04a098),
    (0x04A09A, i_04a09a),
    (0x04A09C, i_04a09c),
    (0x04A0A2, i_04a0a2),
    (0x04A0A6, i_04a0a6),
    (0x04A0AC, i_04a0ac),
    (0x04A0B2, i_04a0b2),
    (0x04A0BA, i_04a0ba),
    (0x04A0C0, i_04a0c0),
    (0x04A0C4, i_04a0c4),
    (0x04A0C8, i_04a0c8),
    (0x04A0CC, i_04a0cc),
    (0x04A0D2, i_04a0d2),
    (0x04A0D4, i_04a0d4),
    (0x04A0D8, i_04a0d8),
    (0x04A0DE, i_04a0de),
    (0x04A0E2, i_04a0e2),
    (0x04A0E6, i_04a0e6),
    (0x04A0F4, i_04a0f4),
    (0x04A0F8, i_04a0f8),
    (0x04A0FE, i_04a0fe),
    (0x04A102, i_04a102),
    (0x04A110, i_04a110),
    (0x04A114, i_04a114),
    (0x04A120, i_04a120),
    (0x04A124, i_04a124),
    (0x04A128, i_04a128),
    (0x04A12C, i_04a12c),
    (0x04A132, i_04a132),
    (0x04A136, i_04a136),
    (0x04A13C, i_04a13c),
    (0x04A140, i_04a140),
    (0x04A166, i_04a166),
    (0x04A16A, i_04a16a),
    (0x04A16C, i_04a16c),
    (0x04A16E, i_04a16e),
    (0x04A172, i_04a172),
    (0x04A178, i_04a178),
    (0x04A17E, i_04a17e),
    (0x04A184, i_04a184),
    (0x04A18A, i_04a18a),
    (0x04A18C, i_04a18c),
    (0x04A190, i_04a190),
    (0x04A194, i_04a194),
    (0x04A198, i_04a198),
    (0x04A19E, i_04a19e),
    (0x04A1A4, i_04a1a4),
    (0x04A1AA, i_04a1aa),
    (0x04A1B0, i_04a1b0),
    (0x04A1B2, i_04a1b2),
    (0x04A1B6, i_04a1b6),
    (0x04A1BA, i_04a1ba),
    (0x04A1BE, i_04a1be),
    (0x04A1C4, i_04a1c4),
    (0x04A1C6, i_04a1c6),
    (0x04A1CA, i_04a1ca),
    (0x04A1CE, i_04a1ce),
    (0x04A1D0, i_04a1d0),
    (0x04A1D2, i_04a1d2),
    (0x04A1D4, i_04a1d4),
    (0x04A1D6, i_04a1d6),
    (0x04A1DA, i_04a1da),
    (0x04A1DE, i_04a1de),
    (0x04A1E2, i_04a1e2),
    (0x04A1E4, i_04a1e4),
    (0x04A1E6, i_04a1e6),
    (0x04A1E8, i_04a1e8),
    (0x04A1EA, i_04a1ea),
    (0x04A1EE, i_04a1ee),
    (0x04A1F0, i_04a1f0),
    (0x04A1F6, i_04a1f6),
    (0x04A1FA, i_04a1fa),
    (0x04A1FC, i_04a1fc),
    (0x04A200, i_04a200),
    (0x04A202, i_04a202),
    (0x04A208, i_04a208),
    (0x04A20C, i_04a20c),
    (0x04A20E, i_04a20e),
    (0x04A210, i_04a210),
    (0x04A218, i_04a218),
    (0x04A21E, i_04a21e),
    (0x04A222, i_04a222),
    (0x04A226, i_04a226),
    (0x04A22A, i_04a22a),
    (0x04A22E, i_04a22e),
    (0x04A232, i_04a232),
    (0x04A236, i_04a236),
    (0x04A23A, i_04a23a),
    (0x04A240, i_04a240),
    (0x04A242, i_04a242),
    (0x04A246, i_04a246),
    (0x04A24A, i_04a24a),
    (0x04A250, i_04a250),
    (0x04A256, i_04a256),
    (0x04A25A, i_04a25a),
    (0x04A25C, i_04a25c),
    (0x04A260, i_04a260),
    (0x04A266, i_04a266),
    (0x04A268, i_04a268),
    (0x04A26E, i_04a26e),
    (0x04A272, i_04a272),
    (0x04A276, i_04a276),
    (0x04A27A, i_04a27a),
    (0x04A27E, i_04a27e),
    (0x04A282, i_04a282),
    (0x04A286, i_04a286),
    (0x04A28A, i_04a28a),
    (0x04A290, i_04a290),
    (0x04A294, i_04a294),
    (0x04A298, i_04a298),
    (0x04A29C, i_04a29c),
    (0x04A2A2, i_04a2a2),
    (0x04A2A6, i_04a2a6),
    (0x04A2A8, i_04a2a8),
    (0x04A2AC, i_04a2ac),
    (0x04A2B2, i_04a2b2),
    (0x04A2B8, i_04a2b8),
    (0x04A2BC, i_04a2bc),
    (0x04A2BE, i_04a2be),
    (0x04A2C4, i_04a2c4),
    (0x04A2C8, i_04a2c8),
    (0x04A2CC, i_04a2cc),
    (0x04A2D0, i_04a2d0),
    (0x04A2D4, i_04a2d4),
    (0x04A2D8, i_04a2d8),
    (0x04A2DC, i_04a2dc),
    (0x04A2E0, i_04a2e0),
    (0x04A2E6, i_04a2e6),
    (0x04A2EA, i_04a2ea),
    (0x04A2EE, i_04a2ee),
    (0x04A2F4, i_04a2f4),
    (0x04A2FA, i_04a2fa),
    (0x04A2FE, i_04a2fe),
    (0x04A304, i_04a304),
    (0x04A30A, i_04a30a),
    (0x04A30E, i_04a30e),
    (0x04A35E, i_04a35e),
    (0x04A362, i_04a362),
    (0x04A366, i_04a366),
    (0x04A36A, i_04a36a),
    (0x04A36E, i_04a36e),
    (0x04A372, i_04a372),
    (0x04A378, i_04a378),
    (0x04A37C, i_04a37c),
    (0x04A380, i_04a380),
    (0x04A382, i_04a382),
    (0x04A386, i_04a386),
    (0x04A388, i_04a388),
    (0x04A38C, i_04a38c),
    (0x04A392, i_04a392),
    (0x04A396, i_04a396),
    (0x04A39A, i_04a39a),
    (0x04A39C, i_04a39c),
    (0x04A39E, i_04a39e),
    (0x04A3A4, i_04a3a4),
    (0x04A3A8, i_04a3a8),
    (0x04A3AA, i_04a3aa),
    (0x04A3AE, i_04a3ae),
    (0x04A3B4, i_04a3b4),
    (0x04A3B8, i_04a3b8),
    (0x04A3BC, i_04a3bc),
    (0x04A3C0, i_04a3c0),
    (0x04A3C2, i_04a3c2),
    (0x04A3C4, i_04a3c4),
    (0x04A3CA, i_04a3ca),
    (0x04A3CE, i_04a3ce),
    (0x04A3D0, i_04a3d0),
    (0x04A3D4, i_04a3d4),
    (0x04A3D8, i_04a3d8),
    (0x04A3DC, i_04a3dc),
    (0x04A3E4, i_04a3e4),
    (0x04A3EA, i_04a3ea),
    (0x04A3EE, i_04a3ee),
    (0x04A3F4, i_04a3f4),
    (0x04A3F8, i_04a3f8),
    (0x04A3FC, i_04a3fc),
    (0x04A400, i_04a400),
    (0x04A404, i_04a404),
    (0x04A420, i_04a420),
    (0x04A424, i_04a424),
    (0x04A428, i_04a428),
    (0x04A42E, i_04a42e),
    (0x04A430, i_04a430),
    (0x04A434, i_04a434),
    (0x04A438, i_04a438),
    (0x04A43C, i_04a43c),
    (0x04A43E, i_04a43e),
    (0x04A442, i_04a442),
    (0x04A444, i_04a444),
    (0x04A448, i_04a448),
    (0x04A44C, i_04a44c),
    (0x04A44E, i_04a44e),
    (0x04A454, i_04a454),
    (0x04A458, i_04a458),
    (0x04A45E, i_04a45e),
    (0x04A462, i_04a462),
    (0x04A466, i_04a466),
    (0x04A46C, i_04a46c),
    (0x04A470, i_04a470),
    (0x04A474, i_04a474),
    (0x04A476, i_04a476),
    (0x04A47A, i_04a47a),
    (0x04A480, i_04a480),
    (0x04A484, i_04a484),
    (0x04A48A, i_04a48a),
    (0x04A48E, i_04a48e),
    (0x04A492, i_04a492),
    (0x04A494, i_04a494),
    (0x04A498, i_04a498),
    (0x04A49C, i_04a49c),
    (0x04A49E, i_04a49e),
    (0x04A4A2, i_04a4a2),
    (0x04A4A6, i_04a4a6),
    (0x04A4AC, i_04a4ac),
    (0x04A4C8, i_04a4c8),
    (0x04A4CC, i_04a4cc),
    (0x04A4D0, i_04a4d0),
    (0x04A4D6, i_04a4d6),
    (0x04A4D8, i_04a4d8),
    (0x04A4DC, i_04a4dc),
    (0x04A4E0, i_04a4e0),
    (0x04A4E4, i_04a4e4),
    (0x04A4E6, i_04a4e6),
    (0x04A4EA, i_04a4ea),
    (0x04A4EC, i_04a4ec),
    (0x04A4F0, i_04a4f0),
    (0x04A4F4, i_04a4f4),
    (0x04A4F6, i_04a4f6),
    (0x04A4FC, i_04a4fc),
    (0x04A500, i_04a500),
    (0x04A506, i_04a506),
    (0x04A50A, i_04a50a),
    (0x04A50E, i_04a50e),
    (0x04A514, i_04a514),
    (0x04A518, i_04a518),
    (0x04A51C, i_04a51c),
    (0x04A51E, i_04a51e),
    (0x04A522, i_04a522),
    (0x04A528, i_04a528),
    (0x04A52C, i_04a52c),
    (0x04A532, i_04a532),
    (0x04A536, i_04a536),
    (0x04A53A, i_04a53a),
    (0x04A53C, i_04a53c),
    (0x04A540, i_04a540),
    (0x04A544, i_04a544),
    (0x04A546, i_04a546),
    (0x04A54A, i_04a54a),
    (0x04A550, i_04a550),
    (0x04A554, i_04a554),
    (0x04A556, i_04a556),
    (0x04A55A, i_04a55a),
    (0x04A55E, i_04a55e),
    (0x04A564, i_04a564),
    (0x04A568, i_04a568),
    (0x04A56C, i_04a56c),
    (0x04A570, i_04a570),
    (0x04A574, i_04a574),
    (0x04A57A, i_04a57a),
    (0x04A57E, i_04a57e),
    (0x04A582, i_04a582),
    (0x04A586, i_04a586),
    (0x04A58A, i_04a58a),
    (0x04A590, i_04a590),
    (0x04A594, i_04a594),
    (0x04A598, i_04a598),
    (0x04A59E, i_04a59e),
    (0x04A5A2, i_04a5a2),
    (0x04A612, i_04a612),
    (0x04A616, i_04a616),
    (0x04A61C, i_04a61c),
    (0x04A620, i_04a620),
    (0x04A624, i_04a624),
    (0x04A62A, i_04a62a),
    (0x04A62E, i_04a62e),
    (0x04A632, i_04a632),
    (0x04A638, i_04a638),
    (0x04A63C, i_04a63c),
    (0x04A640, i_04a640),
    (0x04A646, i_04a646),
    (0x04A64C, i_04a64c),
    (0x04A650, i_04a650),
    (0x04A652, i_04a652),
    (0x04A656, i_04a656),
    (0x04A65C, i_04a65c),
    (0x04A660, i_04a660),
    (0x04A664, i_04a664),
    (0x04A668, i_04a668),
    (0x04A66C, i_04a66c),
    (0x04A66E, i_04a66e),
    (0x04A672, i_04a672),
    (0x04A676, i_04a676),
    (0x04A67C, i_04a67c),
    (0x04A680, i_04a680),
    (0x04A686, i_04a686),
    (0x04A688, i_04a688),
    (0x04A68C, i_04a68c),
    (0x04A690, i_04a690),
    (0x04A696, i_04a696),
    (0x04A69A, i_04a69a),
    (0x04A69C, i_04a69c),
    (0x04A6A0, i_04a6a0),
    (0x04A6A6, i_04a6a6),
    (0x04A6AA, i_04a6aa),
    (0x04A6AE, i_04a6ae),
    (0x04A6B2, i_04a6b2),
    (0x04A6B6, i_04a6b6),
    (0x04A6BC, i_04a6bc),
    (0x04A6C0, i_04a6c0),
    (0x04A6C6, i_04a6c6),
    (0x04A6CC, i_04a6cc),
    (0x04A6D2, i_04a6d2),
    (0x04A6D8, i_04a6d8),
    (0x04A6DC, i_04a6dc),
    (0x04A6E0, i_04a6e0),
    (0x04A6E4, i_04a6e4),
    (0x04A6E6, i_04a6e6),
    (0x04A6E8, i_04a6e8),
    (0x04A6EA, i_04a6ea),
    (0x04A6EE, i_04a6ee),
    (0x04A6F4, i_04a6f4),
    (0x04A6F8, i_04a6f8),
    (0x04A6FC, i_04a6fc),
    (0x04A700, i_04a700),
    (0x04A72E, i_04a72e),
    (0x04A732, i_04a732),
    (0x04A736, i_04a736),
    (0x04A73C, i_04a73c),
    (0x04A740, i_04a740),
    (0x04A744, i_04a744),
    (0x04A748, i_04a748),
    (0x04A74E, i_04a74e),
    (0x04A752, i_04a752),
    (0x04A754, i_04a754),
    (0x04A758, i_04a758),
    (0x04A75C, i_04a75c),
    (0x04A760, i_04a760),
    (0x04A764, i_04a764),
    (0x04A768, i_04a768),
    (0x04A76A, i_04a76a),
    (0x04A76E, i_04a76e),
    (0x04A774, i_04a774),
    (0x04A778, i_04a778),
    (0x04A77E, i_04a77e),
    (0x04A782, i_04a782),
    (0x04A786, i_04a786),
    (0x04A78A, i_04a78a),
    (0x04A78E, i_04a78e),
    (0x04A792, i_04a792),
    (0x04A794, i_04a794),
    (0x04A798, i_04a798),
    (0x04A79C, i_04a79c),
    (0x04A7A2, i_04a7a2),
    (0x04A7B0, i_04a7b0),
    (0x04A7B4, i_04a7b4),
    (0x04A7B8, i_04a7b8),
    (0x04A7BE, i_04a7be),
    (0x04A7C2, i_04a7c2),
    (0x04A7C8, i_04a7c8),
    (0x04A7CC, i_04a7cc),
    (0x04A7D2, i_04a7d2),
    (0x04A7D6, i_04a7d6),
    (0x04A7D8, i_04a7d8),
    (0x04A7DC, i_04a7dc),
    (0x04A7E2, i_04a7e2),
    (0x04A7E6, i_04a7e6),
    (0x04A7EA, i_04a7ea),
    (0x04A7EE, i_04a7ee),
    (0x04A7F0, i_04a7f0),
    (0x04A7F4, i_04a7f4),
    (0x04A7F8, i_04a7f8),
    (0x04A7FE, i_04a7fe),
    (0x04A802, i_04a802),
    (0x04A808, i_04a808),
    (0x04A80C, i_04a80c),
    (0x04A812, i_04a812),
    (0x04A816, i_04a816),
    (0x04A81C, i_04a81c),
    (0x04A820, i_04a820),
    (0x04A826, i_04a826),
    (0x04A82A, i_04a82a),
    (0x04A830, i_04a830),
    (0x04A836, i_04a836),
    (0x04A83C, i_04a83c),
    (0x04A840, i_04a840),
    (0x04A846, i_04a846),
    (0x04A84C, i_04a84c),
    (0x04A852, i_04a852),
    (0x04A858, i_04a858),
    (0x04A85C, i_04a85c),
    (0x04A85E, i_04a85e),
    (0x04A862, i_04a862),
    (0x04A864, i_04a864),
    (0x04A866, i_04a866),
    (0x04A868, i_04a868),
    (0x04A86C, i_04a86c),
    (0x04A86E, i_04a86e),
    (0x04A872, i_04a872),
    (0x04A874, i_04a874),
    (0x04A878, i_04a878),
    (0x04A87C, i_04a87c),
    (0x04A882, i_04a882),
    (0x04A888, i_04a888),
    (0x04A948, i_04a948),
    (0x04A94C, i_04a94c),
    (0x04A952, i_04a952),
    (0x04A958, i_04a958),
    (0x04A95E, i_04a95e),
    (0x04A964, i_04a964),
    (0x04A968, i_04a968),
    (0x04A96C, i_04a96c),
    (0x04A970, i_04a970),
    (0x04A976, i_04a976),
    (0x04A97C, i_04a97c),
    (0x04A980, i_04a980),
    (0x04A982, i_04a982),
    (0x04A986, i_04a986),
    (0x04A98C, i_04a98c),
    (0x04A990, i_04a990),
    (0x04A994, i_04a994),
    (0x04A996, i_04a996),
    (0x04A99A, i_04a99a),
    (0x04A99E, i_04a99e),
    (0x04A9A2, i_04a9a2),
    (0x04A9A6, i_04a9a6),
    (0x04A9AA, i_04a9aa),
];
