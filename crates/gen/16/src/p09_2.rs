// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p09_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $0991BC: movea.l #$ffffffff, a0
pub fn i_0991bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991C2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0991C2: lea.l $2f61ac.l, a0
pub fn i_0991c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $0991C8: jsr $5dd56.l
pub fn i_0991c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991CE);
    cpu.pc = 0x5DD56;
    16
}
/// $0991CE: bcc.w $991d8
pub fn i_0991ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x991D8; return 10; }
    8
}
// ===== SetTaskHandler_0991d2  ($0991D2, 8 B, kernel/task_handlers.c)
/// $0991D2: lea.l $99794(pc), a1
pub fn i_0991d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $0991D6: move.l a1, (a6)
pub fn i_0991d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0991D8: rts
pub fn i_0991d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl207_Init_0991da  ($0991DA, 56 B, frontend/hiscore_memcard_options_09773c.s)
/// $0991DA: move.w #$125, d1
pub fn i_0991da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991DE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0991DE: jsr $236e.l
pub fn i_0991de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991E4);
    cpu.pc = 0x236E;
    16
}
/// $0991E4: jsr $5e7c0.l
pub fn i_0991e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991EA);
    cpu.pc = 0x5E7C0;
    16
}
/// $0991EA: jsr $267e2.l
pub fn i_0991ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991F0);
    cpu.pc = 0x267E2;
    16
}
/// $0991F0: jsr $8f002.l
pub fn i_0991f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x991F6);
    cpu.pc = 0x8F002;
    16
}
/// $0991F6: move.w #$0, d0
pub fn i_0991f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x991FA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0991FA: jsr $28134.l
pub fn i_0991fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99200;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99200);
    cpu.pc = 0x28134;
    16
}
/// $099200: andi.w #$ffe3, $38(a6)
pub fn i_099200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99206;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099206: ori.w #$4, $38(a6)
pub fn i_099206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9920C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09920C: bclr.b #$1, $12(a6)
pub fn i_09920c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99212;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_099212  ($099212, 8 B, kernel/task_handlers.c)
/// $099212: lea.l $9921a(pc), a1
pub fn i_099212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99216;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9921A;
    16
}
/// $099216: move.l a1, (a6)
pub fn i_099216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99218;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099218: rts
pub fn i_099218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9921A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Sit_09921a  ($09921A, 102 B, frontend/hiscore_memcard_options_09773c.s)
/// $09921A: move.w #$3c, $74(a6)
pub fn i_09921a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99220;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099220: move.w #$1, $66(a6)
pub fn i_099220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99226;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099226: lea.l $2f5ed8.l, a0
pub fn i_099226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9922C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5ED8;
    20
}
/// $09922C: jsr $28cd4.l
pub fn i_09922c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99232;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99232);
    cpu.pc = 0x28CD4;
    16
}
/// $099232: lea.l $99238(pc), a1
pub fn i_099232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99236;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99238;
    16
}
/// $099236: move.l a1, (a6)
pub fn i_099236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99238;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099238: jsr $2783a.l
pub fn i_099238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9923E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9923E);
    cpu.pc = 0x2783A;
    16
}
/// $09923E: jsr $28d70.l
pub fn i_09923e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99244;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99244);
    cpu.pc = 0x28D70;
    16
}
/// $099244: tst.w $74(a6)
pub fn i_099244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99248;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $099248: beq.w $99254
pub fn i_099248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9924C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99254; return 10; }
    8
}
/// $09924C: subq.w #$1, $74(a6)
pub fn i_09924c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99250;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099250: bne.w $99264
pub fn i_099250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99254;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99264; return 10; }
    8
}
/// $099254: jsr $2870a.l
pub fn i_099254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9925A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9925A);
    cpu.pc = 0x2870A;
    16
}
/// $09925A: bcc.w $99264
pub fn i_09925a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9925E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99264; return 10; }
    8
}
/// $09925E: lea.l $99288(pc), a1
pub fn i_09925e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99262;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99288;
    16
}
/// $099262: move.l a1, (a6)
pub fn i_099262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99264;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099264: bclr.b #$3, $13(a6)
pub fn i_099264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9926A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09926A: movea.l #$ffffffff, a0
pub fn i_09926a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99270;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $099270: lea.l $2f61ac.l, a0
pub fn i_099270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99276;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $099276: jsr $5dd56.l
pub fn i_099276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9927C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9927C);
    cpu.pc = 0x5DD56;
    16
}
/// $09927C: bcc.w $99286
pub fn i_09927c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99280;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99286; return 10; }
    8
}
// ===== SetTaskHandler_099280  ($099280, 8 B, kernel/task_handlers.c)
/// $099280: lea.l $99794(pc), a1
pub fn i_099280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99284;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $099284: move.l a1, (a6)
pub fn i_099284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99286;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099286: rts
pub fn i_099286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99288;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Sit_Hop_099288  ($099288, 72 B, frontend/hiscore_memcard_options_09773c.s)
/// $099288: move.w #$28, $74(a6)
pub fn i_099288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9928E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09928E: lea.l $2f5ef8.l, a0
pub fn i_09928e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99294;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5EF8;
    20
}
/// $099294: jsr $28cd4.l
pub fn i_099294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9929A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9929A);
    cpu.pc = 0x28CD4;
    16
}
/// $09929A: lea.l $992a0(pc), a1
pub fn i_09929a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9929E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x992A0;
    16
}
/// $09929E: move.l a1, (a6)
pub fn i_09929e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0992A0: jsr $2783a.l
pub fn i_0992a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x992A6);
    cpu.pc = 0x2783A;
    16
}
/// $0992A6: jsr $28d70.l
pub fn i_0992a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x992AC);
    cpu.pc = 0x28D70;
    16
}
/// $0992AC: subq.w #$1, $74(a6)
pub fn i_0992ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992B0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0992B0: bne.w $992ba
pub fn i_0992b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992B4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x992BA; return 10; }
    8
}
/// $0992B4: lea.l $9921a(pc), a1
pub fn i_0992b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9921A;
    16
}
/// $0992B8: move.l a1, (a6)
pub fn i_0992b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0992BA: movea.l #$ffffffff, a0
pub fn i_0992ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992C0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0992C0: lea.l $2f61ac.l, a0
pub fn i_0992c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $0992C6: jsr $5dd56.l
pub fn i_0992c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x992CC);
    cpu.pc = 0x5DD56;
    16
}
/// $0992CC: bcc.w $992d6
pub fn i_0992cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x992D6; return 10; }
    8
}
// ===== SetTaskHandler_0992d0  ($0992D0, 8 B, kernel/task_handlers.c)
/// $0992D0: lea.l $99794(pc), a1
pub fn i_0992d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $0992D4: move.l a1, (a6)
pub fn i_0992d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0992D6: rts
pub fn i_0992d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl208_Init_0992d8  ($0992D8, 102 B, frontend/hiscore_memcard_options_09773c.s)
/// $0992D8: move.w #$126, d1
pub fn i_0992d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992DC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0992DC: jsr $236e.l
pub fn i_0992dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x992E2);
    cpu.pc = 0x236E;
    16
}
/// $0992E2: bra.w $992fe
pub fn i_0992e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x992E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x992FE;
    10
}
/// $0992FE: jsr $267e2.l
pub fn i_0992fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99304);
    cpu.pc = 0x267E2;
    16
}
/// $099304: jsr $5e7c0.l
pub fn i_099304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9930A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9930A);
    cpu.pc = 0x5E7C0;
    16
}
/// $09930A: jsr $8f002.l
pub fn i_09930a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99310;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99310);
    cpu.pc = 0x8F002;
    16
}
/// $099310: move.w #$8000, d0
pub fn i_099310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99314;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099314: jsr $28134.l
pub fn i_099314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9931A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9931A);
    cpu.pc = 0x28134;
    16
}
/// $09931A: andi.w #$ffe3, $38(a6)
pub fn i_09931a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99320;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099320: ori.w #$4, $38(a6)
pub fn i_099320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99326;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099326: move.w #$1, $66(a6)
pub fn i_099326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9932C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09932C: lea.l $2f5f18.l, a0
pub fn i_09932c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99332;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5F18;
    20
}
/// $099332: jsr $28cd4.l
pub fn i_099332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99338);
    cpu.pc = 0x28CD4;
    16
}
/// $099338: bclr.b #$1, $12(a6)
pub fn i_099338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9933E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_09933e  ($09933E, 8 B, kernel/task_handlers.c)
/// $09933E: lea.l $9976a(pc), a1
pub fn i_09933e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99342;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9976A;
    16
}
/// $099342: move.l a1, (a6)
pub fn i_099342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99344;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099344: rts
pub fn i_099344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99346;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl209_Init_099346  ($099346, 84 B, frontend/hiscore_memcard_options_09773c.s)
/// $099346: move.w #$126, d1
pub fn i_099346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9934A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09934A: jsr $236e.l
pub fn i_09934a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99350;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99350);
    cpu.pc = 0x236E;
    16
}
/// $099350: bra.w $9936c
pub fn i_099350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99354;
    cpu.ir = 0x6000;
    cpu.pc = 0x9936C;
    10
}
/// $09936C: lea.l $2f5f38.l, a0
pub fn i_09936c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99372;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5F38;
    20
}
/// $099372: jsr $28cd4.l
pub fn i_099372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99378;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99378);
    cpu.pc = 0x28CD4;
    16
}
/// $099378: move.w #$8000, d0
pub fn i_099378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9937C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09937C: jsr $28134.l
pub fn i_09937c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99382;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99382);
    cpu.pc = 0x28134;
    16
}
/// $099382: andi.w #$ffe3, $38(a6)
pub fn i_099382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99388;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099388: ori.w #$4, $38(a6)
pub fn i_099388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9938E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09938E: jsr $5e7c0.l
pub fn i_09938e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99394;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99394);
    cpu.pc = 0x5E7C0;
    16
}
/// $099394: bclr.b #$1, $12(a6)
pub fn i_099394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9939A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_09939a  ($09939A, 8 B, kernel/task_handlers.c)
/// $09939A: lea.l $993a2(pc), a1
pub fn i_09939a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9939E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x993A2;
    16
}
/// $09939E: move.l a1, (a6)
pub fn i_09939e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0993A0: rts
pub fn i_0993a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl209_Body_0993a2  ($0993A2, 80 B, frontend/hiscore_memcard_options_09773c.s)
/// $0993A2: jsr $267e2.l
pub fn i_0993a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993A8);
    cpu.pc = 0x267E2;
    16
}
/// $0993A8: jsr $8f002.l
pub fn i_0993a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993AE);
    cpu.pc = 0x8F002;
    16
}
/// $0993AE: jsr $8f010.l
pub fn i_0993ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993B4);
    cpu.pc = 0x8F010;
    16
}
/// $0993B4: move.w #$1, $66(a6)
pub fn i_0993b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993BA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0993BA: lea.l $993c0(pc), a1
pub fn i_0993ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x993C0;
    16
}
/// $0993BE: move.l a1, (a6)
pub fn i_0993be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0993C0: jsr $27a92.l
pub fn i_0993c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993C6);
    cpu.pc = 0x27A92;
    16
}
/// $0993C6: jsr $27eba.l
pub fn i_0993c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993CC);
    cpu.pc = 0x27EBA;
    16
}
/// $0993CC: bcc.w $993d6
pub fn i_0993cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x993D6; return 10; }
    8
}
/// $0993D0: lea.l $9936c(pc), a1
pub fn i_0993d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9936C;
    16
}
/// $0993D4: move.l a1, (a6)
pub fn i_0993d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0993D6: jsr $28d70.l
pub fn i_0993d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993DC);
    cpu.pc = 0x28D70;
    16
}
/// $0993DC: movea.l #$ffffffff, a0
pub fn i_0993dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993E2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0993E2: lea.l $2f61ac.l, a0
pub fn i_0993e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $0993E8: jsr $5dd56.l
pub fn i_0993e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x993EE);
    cpu.pc = 0x5DD56;
    16
}
/// $0993EE: bcc.w $993f8
pub fn i_0993ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x993F8; return 10; }
    8
}
// ===== SetTaskHandler_0993f2  ($0993F2, 8 B, kernel/task_handlers.c)
/// $0993F2: lea.l $99794(pc), a1
pub fn i_0993f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $0993F6: move.l a1, (a6)
pub fn i_0993f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0993F8: rts
pub fn i_0993f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x993FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Flee3_0993fa  ($0993FA, 68 B, frontend/hiscore_memcard_options_09773c.s)
/// $0993FA: jsr $267e2.l
pub fn i_0993fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99400;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99400);
    cpu.pc = 0x267E2;
    16
}
/// $099400: move.w #$ffe0, $2a(a6)
pub fn i_099400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99406;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099406: lea.l $9940c(pc), a1
pub fn i_099406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9940A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9940C;
    16
}
/// $09940A: move.l a1, (a6)
pub fn i_09940a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9940C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09940C: jsr $27c8c.l
pub fn i_09940c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99412;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99412);
    cpu.pc = 0x27C8C;
    16
}
/// $099412: jsr $27eba.l
pub fn i_099412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99418;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99418);
    cpu.pc = 0x27EBA;
    16
}
/// $099418: bcs.w $99422
pub fn i_099418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9941C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x99422; return 10; }
    8
}
/// $09941C: lea.l $9936c(pc), a1
pub fn i_09941c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99420;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9936C;
    16
}
/// $099420: move.l a1, (a6)
pub fn i_099420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99422;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099422: jsr $28d70.l
pub fn i_099422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99428;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99428);
    cpu.pc = 0x28D70;
    16
}
/// $099428: movea.l #$ffffffff, a0
pub fn i_099428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9942E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09942E: lea.l $2f61ac.l, a0
pub fn i_09942e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99434;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $099434: jsr $5dd56.l
pub fn i_099434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9943A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9943A);
    cpu.pc = 0x5DD56;
    16
}
/// $09943A: bcc.w $99444
pub fn i_09943a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9943E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99444; return 10; }
    8
}
// ===== SetTaskHandler_09943e  ($09943E, 8 B, kernel/task_handlers.c)
/// $09943E: lea.l $99794(pc), a1
pub fn i_09943e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99442;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $099442: move.l a1, (a6)
pub fn i_099442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99444;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099444: rts
pub fn i_099444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99446;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl210_Init_099446  ($099446, 84 B, frontend/hiscore_memcard_options_09773c.s)
/// $099446: move.w #$126, d1
pub fn i_099446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9944A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09944A: jsr $236e.l
pub fn i_09944a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99450;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99450);
    cpu.pc = 0x236E;
    16
}
/// $099450: bra.w $9946c
pub fn i_099450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99454;
    cpu.ir = 0x6000;
    cpu.pc = 0x9946C;
    10
}
/// $09946C: jsr $5e7c0.l
pub fn i_09946c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99472;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99472);
    cpu.pc = 0x5E7C0;
    16
}
/// $099472: move.w #$8000, d0
pub fn i_099472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99476;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099476: jsr $28134.l
pub fn i_099476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9947C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9947C);
    cpu.pc = 0x28134;
    16
}
/// $09947C: andi.w #$ffe3, $38(a6)
pub fn i_09947c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99482;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099482: ori.w #$4, $38(a6)
pub fn i_099482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99488;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099488: lea.l $2f5f80.l, a0
pub fn i_099488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9948E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5F80;
    20
}
/// $09948E: jsr $28cd4.l
pub fn i_09948e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99494;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99494);
    cpu.pc = 0x28CD4;
    16
}
/// $099494: bclr.b #$1, $12(a6)
pub fn i_099494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9949A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_09949a  ($09949A, 8 B, kernel/task_handlers.c)
/// $09949A: lea.l $993a2(pc), a1
pub fn i_09949a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9949E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x993A2;
    16
}
/// $09949E: move.l a1, (a6)
pub fn i_09949e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0994A0: rts
pub fn i_0994a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl211_Init_0994a2  ($0994A2, 84 B, frontend/hiscore_memcard_options_09773c.s)
/// $0994A2: move.w #$126, d1
pub fn i_0994a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0994A6: jsr $236e.l
pub fn i_0994a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x994AC);
    cpu.pc = 0x236E;
    16
}
/// $0994AC: bra.w $994c8
pub fn i_0994ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x994C8;
    10
}
/// $0994C8: jsr $5e7c0.l
pub fn i_0994c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x994CE);
    cpu.pc = 0x5E7C0;
    16
}
/// $0994CE: move.w #$8000, d0
pub fn i_0994ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994D2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0994D2: jsr $28134.l
pub fn i_0994d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x994D8);
    cpu.pc = 0x28134;
    16
}
/// $0994D8: andi.w #$ffe3, $38(a6)
pub fn i_0994d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994DE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0994DE: ori.w #$4, $38(a6)
pub fn i_0994de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994E4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0994E4: lea.l $2f5fc8.l, a0
pub fn i_0994e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F5FC8;
    20
}
/// $0994EA: jsr $28cd4.l
pub fn i_0994ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x994F0);
    cpu.pc = 0x28CD4;
    16
}
/// $0994F0: bclr.b #$1, $12(a6)
pub fn i_0994f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994F6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_0994f6  ($0994F6, 8 B, kernel/task_handlers.c)
/// $0994F6: lea.l $993a2(pc), a1
pub fn i_0994f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x993A2;
    16
}
/// $0994FA: move.l a1, (a6)
pub fn i_0994fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0994FC: rts
pub fn i_0994fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x994FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl220_Init_0994fe  ($0994FE, 56 B, frontend/hiscore_memcard_options_09773c.s)
/// $0994FE: move.w #$125, d1
pub fn i_0994fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99502;
    cpu.ir = 0x323C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099502: jsr $236e.l
pub fn i_099502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99508;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99508);
    cpu.pc = 0x236E;
    16
}
/// $099508: jsr $267e2.l
pub fn i_099508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9950E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9950E);
    cpu.pc = 0x267E2;
    16
}
/// $09950E: jsr $5e7c0.l
pub fn i_09950e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99514);
    cpu.pc = 0x5E7C0;
    16
}
/// $099514: jsr $8f002.l
pub fn i_099514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9951A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9951A);
    cpu.pc = 0x8F002;
    16
}
/// $09951A: move.w #$8000, d0
pub fn i_09951a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9951E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09951E: jsr $28134.l
pub fn i_09951e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99524;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99524);
    cpu.pc = 0x28134;
    16
}
/// $099524: andi.w #$ffe3, $38(a6)
pub fn i_099524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9952A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09952A: ori.w #$4, $38(a6)
pub fn i_09952a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99530;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099530: bclr.b #$1, $12(a6)
pub fn i_099530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99536;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_099536  ($099536, 8 B, kernel/task_handlers.c)
/// $099536: lea.l $9953e(pc), a1
pub fn i_099536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9953A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9953E;
    16
}
/// $09953A: move.l a1, (a6)
pub fn i_09953a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9953C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09953C: rts
pub fn i_09953c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9953E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_RunLeft_09953e  ($09953E, 74 B, frontend/hiscore_memcard_options_09773c.s)
/// $09953E: move.w #$1, $66(a6)
pub fn i_09953e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99544;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099544: lea.l $2f6010.l, a0
pub fn i_099544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9954A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6010;
    20
}
/// $09954A: jsr $28cd4.l
pub fn i_09954a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99550);
    cpu.pc = 0x28CD4;
    16
}
/// $099550: lea.l $99556(pc), a1
pub fn i_099550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99554;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99556;
    16
}
/// $099554: move.l a1, (a6)
pub fn i_099554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99556;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099556: jsr $2783a.l
pub fn i_099556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9955C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9955C);
    cpu.pc = 0x2783A;
    16
}
/// $09955C: jsr $28d70.l
pub fn i_09955c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99562;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99562);
    cpu.pc = 0x28D70;
    16
}
/// $099562: cmpi.w #$b0, $22(a6)
pub fn i_099562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99568;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xB0, Size::Word, false);
    16
}
/// $099568: bgt.w $99572
pub fn i_099568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9956C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x99572; return 10; }
    8
}
/// $09956C: lea.l $99590(pc), a1
pub fn i_09956c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99570;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99590;
    16
}
/// $099570: move.l a1, (a6)
pub fn i_099570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99572;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099572: movea.l #$ffffffff, a0
pub fn i_099572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99578;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $099578: lea.l $2f61ac.l, a0
pub fn i_099578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9957E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $09957E: jsr $5dd56.l
pub fn i_09957e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99584;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99584);
    cpu.pc = 0x5DD56;
    16
}
/// $099584: bcc.w $9958e
pub fn i_099584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99588;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9958E; return 10; }
    8
}
// ===== SetTaskHandler_099588  ($099588, 8 B, kernel/task_handlers.c)
/// $099588: lea.l $99794(pc), a1
pub fn i_099588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9958C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $09958C: move.l a1, (a6)
pub fn i_09958c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9958E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09958E: rts
pub fn i_09958e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99590;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_RunLeft_Stop_099590  ($099590, 52 B, frontend/hiscore_memcard_options_09773c.s)
/// $099590: lea.l $2f6022.l, a0
pub fn i_099590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99596;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6022;
    20
}
/// $099596: jsr $28cd4.l
pub fn i_099596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9959C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9959C);
    cpu.pc = 0x28CD4;
    16
}
/// $09959C: lea.l $995a2(pc), a1
pub fn i_09959c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x995A2;
    16
}
/// $0995A0: move.l a1, (a6)
pub fn i_0995a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0995A2: jsr $2783a.l
pub fn i_0995a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995A8);
    cpu.pc = 0x2783A;
    16
}
/// $0995A8: jsr $28d70.l
pub fn i_0995a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995AE);
    cpu.pc = 0x28D70;
    16
}
/// $0995AE: movea.l #$ffffffff, a0
pub fn i_0995ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995B4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0995B4: lea.l $2f61ac.l, a0
pub fn i_0995b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $0995BA: jsr $5dd56.l
pub fn i_0995ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995C0);
    cpu.pc = 0x5DD56;
    16
}
/// $0995C0: bcc.w $995ca
pub fn i_0995c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x995CA; return 10; }
    8
}
// ===== SetTaskHandler_0995c4  ($0995C4, 8 B, kernel/task_handlers.c)
/// $0995C4: lea.l $99794(pc), a1
pub fn i_0995c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $0995C8: move.l a1, (a6)
pub fn i_0995c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0995CA: rts
pub fn i_0995ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl221_Init_0995cc  ($0995CC, 60 B, frontend/hiscore_memcard_options_09773c.s)
/// $0995CC: move.w #$127, d1
pub fn i_0995cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995D0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x127;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0995D0: jsr $236e.l
pub fn i_0995d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995D6);
    cpu.pc = 0x236E;
    16
}
/// $0995D6: jsr $267e2.l
pub fn i_0995d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995DC);
    cpu.pc = 0x267E2;
    16
}
/// $0995DC: jsr $5e7c0.l
pub fn i_0995dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995E2);
    cpu.pc = 0x5E7C0;
    16
}
/// $0995E2: jsr $8f002.l
pub fn i_0995e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995E8);
    cpu.pc = 0x8F002;
    16
}
/// $0995E8: clr.b $21(a6)
pub fn i_0995e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995EC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0995EC: move.w #$8000, d0
pub fn i_0995ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995F0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0995F0: jsr $28134.l
pub fn i_0995f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x995F6);
    cpu.pc = 0x28134;
    16
}
/// $0995F6: andi.w #$ffe3, $38(a6)
pub fn i_0995f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x995FC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0995FC: ori.w #$4, $38(a6)
pub fn i_0995fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99602;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099602: bclr.b #$1, $12(a6)
pub fn i_099602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99608;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_099608  ($099608, 8 B, kernel/task_handlers.c)
/// $099608: lea.l $99610(pc), a1
pub fn i_099608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9960C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99610;
    16
}
/// $09960C: move.l a1, (a6)
pub fn i_09960c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9960E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09960E: rts
pub fn i_09960e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99610;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_RunLeft2_099610  ($099610, 88 B, frontend/hiscore_memcard_options_09773c.s)
/// $099610: move.w #$1, $66(a6)
pub fn i_099610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99616;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $099616: lea.l $2f6048.l, a0
pub fn i_099616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9961C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6048;
    20
}
/// $09961C: jsr $28cd4.l
pub fn i_09961c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99622;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99622);
    cpu.pc = 0x28CD4;
    16
}
/// $099622: lea.l $99628(pc), a1
pub fn i_099622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99626;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99628;
    16
}
/// $099626: move.l a1, (a6)
pub fn i_099626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99628;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099628: jsr $2783a.l
pub fn i_099628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9962E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9962E);
    cpu.pc = 0x2783A;
    16
}
/// $09962E: jsr $28d70.l
pub fn i_09962e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99634;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99634);
    cpu.pc = 0x28D70;
    16
}
/// $099634: tst.b $21(a6)
pub fn i_099634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99638;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $099638: bne.w $99652
pub fn i_099638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9963C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99652; return 10; }
    8
}
/// $09963C: cmpi.w #$b0, $22(a6)
pub fn i_09963c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99642;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xB0, Size::Word, false);
    16
}
/// $099642: bgt.w $99652
pub fn i_099642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99646;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x99652; return 10; }
    8
}
/// $099646: move.b #$ff, $21(a6)
pub fn i_099646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9964C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09964C: lea.l $99670(pc), a1
pub fn i_09964c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99650;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99670;
    16
}
/// $099650: move.l a1, (a6)
pub fn i_099650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99652;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099652: movea.l #$ffffffff, a0
pub fn i_099652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99658;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $099658: lea.l $2f61ac.l, a0
pub fn i_099658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9965E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $09965E: jsr $5dd56.l
pub fn i_09965e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99664;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99664);
    cpu.pc = 0x5DD56;
    16
}
/// $099664: bcc.w $9966e
pub fn i_099664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99668;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9966E; return 10; }
    8
}
// ===== SetTaskHandler_099668  ($099668, 8 B, kernel/task_handlers.c)
/// $099668: lea.l $99794(pc), a1
pub fn i_099668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9966C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $09966C: move.l a1, (a6)
pub fn i_09966c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9966E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09966E: rts
pub fn i_09966e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99670;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_RunLeft2_Wait_099670  ($099670, 62 B, frontend/hiscore_memcard_options_09773c.s)
/// $099670: lea.l $2f6068.l, a0
pub fn i_099670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99676;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6068;
    20
}
/// $099676: jsr $28cd4.l
pub fn i_099676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9967C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9967C);
    cpu.pc = 0x28CD4;
    16
}
/// $09967C: lea.l $99682(pc), a1
pub fn i_09967c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99680;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99682;
    16
}
/// $099680: move.l a1, (a6)
pub fn i_099680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99682;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099682: jsr $2783a.l
pub fn i_099682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99688);
    cpu.pc = 0x2783A;
    16
}
/// $099688: jsr $28d70.l
pub fn i_099688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9968E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9968E);
    cpu.pc = 0x28D70;
    16
}
/// $09968E: bcc.w $99698
pub fn i_09968e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99692;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99698; return 10; }
    8
}
/// $099692: lea.l $99610(pc), a1
pub fn i_099692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99696;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99610;
    16
}
/// $099696: move.l a1, (a6)
pub fn i_099696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99698;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099698: movea.l #$ffffffff, a0
pub fn i_099698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9969E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09969E: lea.l $2f61ac.l, a0
pub fn i_09969e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $0996A4: jsr $5dd56.l
pub fn i_0996a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x996AA);
    cpu.pc = 0x5DD56;
    16
}
/// $0996AA: bcc.w $996b4
pub fn i_0996aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x996B4; return 10; }
    8
}
// ===== SetTaskHandler_0996ae  ($0996AE, 8 B, kernel/task_handlers.c)
/// $0996AE: lea.l $99794(pc), a1
pub fn i_0996ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $0996B2: move.l a1, (a6)
pub fn i_0996b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0996B4: rts
pub fn i_0996b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_Tmpl222_Init_0996b6  ($0996B6, 172 B, frontend/hiscore_memcard_options_09773c.s)
/// $0996B6: move.w #$126, d1
pub fn i_0996b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996BA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x126;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0996BA: jsr $236e.l
pub fn i_0996ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x996C0);
    cpu.pc = 0x236E;
    16
}
/// $0996C0: lea.l $2f607a.l, a0
pub fn i_0996c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F607A;
    20
}
/// $0996C6: jsr $28cd4.l
pub fn i_0996c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x996CC);
    cpu.pc = 0x28CD4;
    16
}
/// $0996CC: bra.w $99734
pub fn i_0996cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x996D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x99734;
    10
}
/// $099734: jsr $267e2.l
pub fn i_099734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9973A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9973A);
    cpu.pc = 0x267E2;
    16
}
/// $09973A: jsr $5e7c0.l
pub fn i_09973a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99740;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99740);
    cpu.pc = 0x5E7C0;
    16
}
/// $099740: jsr $8f002.l
pub fn i_099740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99746;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99746);
    cpu.pc = 0x8F002;
    16
}
/// $099746: move.w #$8000, d0
pub fn i_099746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9974A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09974A: jsr $28134.l
pub fn i_09974a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99750;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99750);
    cpu.pc = 0x28134;
    16
}
/// $099750: andi.w #$ffe3, $38(a6)
pub fn i_099750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99756;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $099756: ori.w #$4, $38(a6)
pub fn i_099756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9975C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09975C: bclr.b #$1, $12(a6)
pub fn i_09975c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99762;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_099762  ($099762, 8 B, kernel/task_handlers.c)
/// $099762: lea.l $9976a(pc), a1
pub fn i_099762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99766;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9976A;
    16
}
/// $099766: move.l a1, (a6)
pub fn i_099766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99768;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099768: rts
pub fn i_099768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9976A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mob_PhysLoop_09976a  ($09976A, 34 B, frontend/hiscore_memcard_options_09773c.s)
/// $09976A: jsr $2783a.l
pub fn i_09976a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99770;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99770);
    cpu.pc = 0x2783A;
    16
}
/// $099770: jsr $28d70.l
pub fn i_099770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99776;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99776);
    cpu.pc = 0x28D70;
    16
}
/// $099776: movea.l #$ffffffff, a0
pub fn i_099776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9977C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09977C: lea.l $2f61ac.l, a0
pub fn i_09977c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99782;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F61AC;
    20
}
/// $099782: jsr $5dd56.l
pub fn i_099782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99788);
    cpu.pc = 0x5DD56;
    16
}
/// $099788: bcc.w $99792
pub fn i_099788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9978C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99792; return 10; }
    8
}
// ===== SetTaskHandler_09978c  ($09978C, 8 B, kernel/task_handlers.c)
/// $09978C: lea.l $99794(pc), a1
pub fn i_09978c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99790;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99794;
    16
}
/// $099790: move.l a1, (a6)
pub fn i_099790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99792;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099792: rts
pub fn i_099792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99794;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_099794  ($099794, 8 B, kernel/jmp_to_scheduler.c)
/// $099794: jmp $518.l
pub fn i_099794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9979A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Entity_CmpPrioWithSibling_09979c  ($09979C, 16 B, frontend/hiscore_memcard_options_09773c.s)
/// $09979C: movea.l $8(a6), a1
pub fn i_09979c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997A0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0997A0: move.b $10(a6), d0
pub fn i_0997a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997A4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0997A4: cmp.b $10(a1), d0
pub fn i_0997a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997A8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0997A8: bcs.w $997b2
pub fn i_0997a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997AC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x997B2; return 10; }
    8
}
// ===== ClearXN_0997ac  ($0997AC, 6 B, thunks/ccr_helpers.c)
/// $0997AC: andi.b #$ee, ccr
pub fn i_0997ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997B0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0997B0: rts
pub fn i_0997b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0997b2  ($0997B2, 6 B, thunks/ccr_helpers.c)
/// $0997B2: ori.b #$11, ccr
pub fn i_0997b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997B6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0997B6: rts
pub fn i_0997b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_RingReset_0997b8  ($0997B8, 20 B, frontend/hiscore_memcard_options_09773c.s)
/// $0997B8: clr.w $10e47e.l
pub fn i_0997b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997BE;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E47E) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E47E, r2 as u16);
    16
}
/// $0997BE: clr.w $10e480.l
pub fn i_0997be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997C4;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E480) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E480, r2 as u16);
    16
}
/// $0997C4: clr.w $10e482.l
pub fn i_0997c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997CA;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E482) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E482, r2 as u16);
    16
}
/// $0997CA: rts
pub fn i_0997ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_RingAdvance_0997cc  ($0997CC, 22 B, frontend/hiscore_memcard_options_09773c.s)
/// $0997CC: move.w $10e480.l, $10e47e.l
pub fn i_0997cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997D6;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x10E480) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E47E, v2 as u16);
    28
}
/// $0997D6: move.w $10e482.l, $10e480.l
pub fn i_0997d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997E0;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x10E482) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E480, v2 as u16);
    28
}
/// $0997E0: rts
pub fn i_0997e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_TrailRecord_Alt_0997e2  ($0997E2, 48 B, frontend/hiscore_memcard_options_09773c.s)
/// $0997E2: lea.l $10e3be.l, a0
pub fn i_0997e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $0997E8: move.b $6c(a6), d7
pub fn i_0997e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997EC;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0997EC: lsr.b #$4, d7
pub fn i_0997ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997EE;
    cpu.ir = 0xE80F;
    let h1 = cpu.d[7] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    14
}
/// $0997EE: cmpi.b #$e, d7
pub fn i_0997ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997F2;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $0997F2: beq.w $9980e
pub fn i_0997f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9980E; return 10; }
    8
}
/// $0997F6: move.b $6c(a6), d5
pub fn i_0997f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997FA;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0997FA: andi.b #$f, d5
pub fn i_0997fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x997FE;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0997FE: ori.b #$e0, d5
pub fn i_0997fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99802;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0xE0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $099802: move.b d5, $6c(a6)
pub fn i_099802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99806;
    cpu.ir = 0x1D45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $099806: clr.w d5
pub fn i_099806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99808;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099808: clr.w d6
pub fn i_099808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9980A;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09980A: bra.w $99896
pub fn i_09980a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9980E;
    cpu.ir = 0x6000;
    cpu.pc = 0x99896;
    10
}
/// $09980E: bra.w $9985a
pub fn i_09980e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99812;
    cpu.ir = 0x6000;
    cpu.pc = 0x9985A;
    10
}
// ===== Entity_TrailRecord_099812  ($099812, 184 B, entity/trail_record_099812.s)
/// $099812: lea.l $10e3be.l, a0
pub fn i_099812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99818;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $099818: move.b $6c(a6), d7
pub fn i_099818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9981C;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09981C: lsr.b #$4, d7
pub fn i_09981c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9981E;
    cpu.ir = 0xE80F;
    let h1 = cpu.d[7] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    14
}
/// $09981E: cmpi.b #$f, d7
pub fn i_09981e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99822;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $099822: bne.w $9985a
pub fn i_099822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99826;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9985A; return 10; }
    8
}
/// $099826: move.b $10e484.l, d7
pub fn i_099826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9982C;
    cpu.ir = 0x1E39;
    let v2 = bus.read8(0x10E484) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v2 & 0xFF);
    16
}
/// $09982C: addq.b #$1, d7
pub fn i_09982c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9982E;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09982E: cmpi.b #$e, d7
pub fn i_09982e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99832;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $099832: bcs.w $9983a
pub fn i_099832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99836;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9983A; return 10; }
    8
}
/// $099836: move.b #$0, d7
pub fn i_099836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9983A;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09983A: move.b d7, $10e484.l
pub fn i_09983a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99840;
    cpu.ir = 0x13C7;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E484, v1 as u8);
    16
}
/// $099840: move.b d7, d6
pub fn i_099840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99842;
    cpu.ir = 0x1C07;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $099842: lsl.b #$4, d6
pub fn i_099842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99844;
    cpu.ir = 0xE90E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $099844: move.b $6c(a6), d5
pub fn i_099844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99848;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099848: andi.b #$f, d5
pub fn i_099848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9984C;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09984C: or.b d6, d5
pub fn i_09984c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9984E;
    cpu.ir = 0x8A06;
    let v1 = cpu.d[6] & 0xFF;
    let r = (v1 | cpu.d[5]) & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09984E: move.b d5, $6c(a6)
pub fn i_09984e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99852;
    cpu.ir = 0x1D45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $099852: clr.w d5
pub fn i_099852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99854;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099854: clr.w d6
pub fn i_099854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99856;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099856: bra.w $99896
pub fn i_099856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9985A;
    cpu.ir = 0x6000;
    cpu.pc = 0x99896;
    10
}
/// $09985A: move.w $10e47e.l, d3
pub fn i_09985a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99860;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x10E47E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $099860: cmp.w $10e480.l, d3
pub fn i_099860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99866;
    cpu.ir = 0xB679;
    let v2 = bus.read16(0x10E480) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $099866: bne.w $99872
pub fn i_099866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9986A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99872; return 10; }
    8
}
/// $09986A: clr.w d5
pub fn i_09986a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9986C;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09986C: clr.w d6
pub fn i_09986c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9986E;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09986E: bra.w $99896
pub fn i_09986e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99872;
    cpu.ir = 0x6000;
    cpu.pc = 0x99896;
    10
}
/// $099872: cmp.b (a0, d3.w), d7
pub fn i_099872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99876;
    cpu.ir = 0xBE30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $099876: beq.w $9988a
pub fn i_099876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9987A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9988A; return 10; }
    8
}
/// $09987A: addi.w #$c, d3
pub fn i_09987a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9987E;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09987E: cmpi.w #$c0, d3
pub fn i_09987e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99882;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $099882: bcs.w $99888
pub fn i_099882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99886;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x99888; return 10; }
    8
}
/// $099886: clr.w d3
pub fn i_099886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99888;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099888: bra.b $99860
pub fn i_099888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9988A;
    cpu.ir = 0x60D6;
    cpu.pc = 0x99860;
    10
}
/// $09988A: move.w d0, d5
pub fn i_09988a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9988C;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09988C: sub.w $2(a0, d3.w), d5
pub fn i_09988c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99890;
    cpu.ir = 0x9A70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $099890: move.w d1, d6
pub fn i_099890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99892;
    cpu.ir = 0x3C01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099892: sub.w $4(a0, d3.w), d6
pub fn i_099892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99896;
    cpu.ir = 0x9C70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $099896: move.w $10e482.l, d3
pub fn i_099896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9989C;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x10E482) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $09989C: move.b d7, (a0, d3.w)
pub fn i_09989c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998A0;
    cpu.ir = 0x1187;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $0998A0: move.w d0, $2(a0, d3.w)
pub fn i_0998a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998A4;
    cpu.ir = 0x3180;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998A4: move.w d1, $4(a0, d3.w)
pub fn i_0998a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998A8;
    cpu.ir = 0x3181;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998A8: move.w d2, $6(a0, d3.w)
pub fn i_0998a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998AC;
    cpu.ir = 0x3182;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x6).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998AC: move.w d5, $8(a0, d3.w)
pub fn i_0998ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998B0;
    cpu.ir = 0x3185;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x8).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998B0: move.w d6, $a(a0, d3.w)
pub fn i_0998b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998B4;
    cpu.ir = 0x3186;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xA).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0998B4: addi.w #$c, d3
pub fn i_0998b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998B8;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0998B8: cmpi.w #$c0, d3
pub fn i_0998b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998BC;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $0998BC: bcs.w $998c2
pub fn i_0998bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998C0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x998C2; return 10; }
    8
}
/// $0998C0: clr.w d3
pub fn i_0998c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998C2;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0998C2: move.w d3, $10e482.l
pub fn i_0998c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998C8;
    cpu.ir = 0x33C3;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E482, v1 as u16);
    16
}
/// $0998C8: rts
pub fn i_0998c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_FindByKeyRange_0998ca  ($0998CA, 30 B, frontend/hiscore_memcard_options_09773c.s)
/// $0998CA: movem.l d7/a0, -(a7)
pub fn i_0998ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998CE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0998CE: lea.l $10e3be.l, a0
pub fn i_0998ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $0998D4: move.w $10e480.l, d7
pub fn i_0998d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998DA;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E480) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $0998DA: cmp.w $10e47e.l, d7
pub fn i_0998da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998E0;
    cpu.ir = 0xBE79;
    let v2 = bus.read16(0x10E47E) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $0998E0: bne.w $998ee
pub fn i_0998e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x998EE; return 10; }
    8
}
/// $0998E4: movem.l (a7)+, d7/a0
pub fn i_0998e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998E8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== ClearC_0998e8  ($0998E8, 6 B, thunks/ccr_helpers.c)
/// $0998E8: andi.b #$fe, ccr
pub fn i_0998e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998EC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0998EC: rts
pub fn i_0998ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_FindByKeyRange_Step_0998ee  ($0998EE, 70 B, frontend/hiscore_memcard_options_09773c.s)
/// $0998EE: subi.w #$c, d7
pub fn i_0998ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998F2;
    cpu.ir = 0x447;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0998F2: bcc.w $998fa
pub fn i_0998f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x998FA; return 10; }
    8
}
/// $0998F6: addi.w #$c0, d7
pub fn i_0998f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998FA;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC0, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0998FA: cmp.w $4(a0, d7.w), d2
pub fn i_0998fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x998FE;
    cpu.ir = 0xB470;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $0998FE: bne.w $9993a
pub fn i_0998fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99902;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9993A; return 10; }
    8
}
/// $099902: move.w d1, d5
pub fn i_099902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99904;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099904: sub.w $2(a0, d7.w), d5
pub fn i_099904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99908;
    cpu.ir = 0x9A70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $099908: cmp.w $6(a0, d7.w), d5
pub fn i_099908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9990C;
    cpu.ir = 0xBA70;
    let ad2 = cpu.a[0].wrapping_add(0x6).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $09990C: bcc.w $9993a
pub fn i_09990c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99910;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9993A; return 10; }
    8
}
/// $099910: move.b $6c(a6), d5
pub fn i_099910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99914;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099914: lsr.b #$4, d5
pub fn i_099914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99916;
    cpu.ir = 0xE80D;
    let h1 = cpu.d[5] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    14
}
/// $099916: cmp.b (a0, d7.w), d5
pub fn i_099916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9991A;
    cpu.ir = 0xBA30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $09991A: beq.w $9993a
pub fn i_09991a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9991E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9993A; return 10; }
    8
}
/// $09991E: andi.b #$f0, $6c(a6)
pub fn i_09991e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99924;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x6C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $099924: move.b (a0, d7.w), d5
pub fn i_099924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99928;
    cpu.ir = 0x1A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    14
}
/// $099928: andi.b #$f, d5
pub fn i_099928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9992C;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09992C: or.b d5, $6c(a6)
pub fn i_09992c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99930;
    cpu.ir = 0x8B2E;
    let ad1 = cpu.a[6].wrapping_add(0x6C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $099930: movem.l (a7)+, d7/a0
pub fn i_099930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99934;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetC_099934  ($099934, 6 B, thunks/ccr_helpers.c)
/// $099934: ori.b #$1, ccr
pub fn i_099934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99938;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $099938: rts
pub fn i_099938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9993A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_FindByKeyRange_Next_09993a  ($09993A, 2 B, frontend/hiscore_memcard_options_09773c.s)
/// $09993A: bra.b $998da
pub fn i_09993a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9993C;
    cpu.ir = 0x609E;
    cpu.pc = 0x998DA;
    10
}
// ===== Trail_FindNearest_09993c  ($09993C, 120 B, frontend/hiscore_memcard_options_09773c.s)
/// $09993C: movem.l d7/a0, -(a7)
pub fn i_09993c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99940;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $099940: lea.l $10e3be.l, a0
pub fn i_099940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99946;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $099946: move.w $10e47e.l, d7
pub fn i_099946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9994C;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E47E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $09994C: cmp.w $10e480.l, d7
pub fn i_09994c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99952;
    cpu.ir = 0xBE79;
    let v2 = bus.read16(0x10E480) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $099952: bne.w $99976
pub fn i_099952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99956;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99976; return 10; }
    8
}
/// $099956: move.b $6c(a6), d7
pub fn i_099956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9995A;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09995A: andi.b #$f, d7
pub fn i_09995a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9995E;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09995E: cmpi.b #$e, d7
pub fn i_09995e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99962;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $099962: bne.w $9996c
pub fn i_099962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99966;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9996C; return 10; }
    8
}
/// $099966: ori.b #$f, $6c(a6)
pub fn i_099966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9996C;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x6C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09996C: move.b #$f, d6
pub fn i_09996c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99970;
    cpu.ir = 0x1C3C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $099970: movem.l (a7)+, d7/a0
pub fn i_099970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99974;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $099974: rts
pub fn i_099974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99976;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $099976: move.w $4(a0, d7.w), d5
pub fn i_099976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9997A;
    cpu.ir = 0x3A30;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09997A: cmp.w d5, d2
pub fn i_09997a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9997C;
    cpu.ir = 0xB445;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $09997C: bgt.w $999ba
pub fn i_09997c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99980;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x999BA; return 10; }
    8
}
/// $099980: subi.w #$14, d5
pub fn i_099980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99984;
    cpu.ir = 0x445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x14, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099984: cmp.w d5, d2
pub fn i_099984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99986;
    cpu.ir = 0xB445;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $099986: ble.w $999ba
pub fn i_099986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9998A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x999BA; return 10; }
    8
}
/// $09998A: move.w d1, d5
pub fn i_09998a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9998C;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09998C: sub.w $2(a0, d7.w), d5
pub fn i_09998c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99990;
    cpu.ir = 0x9A70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $099990: cmp.w $6(a0, d7.w), d5
pub fn i_099990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99994;
    cpu.ir = 0xBA70;
    let ad2 = cpu.a[0].wrapping_add(0x6).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $099994: bcc.w $999ba
pub fn i_099994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99998;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x999BA; return 10; }
    8
}
/// $099998: move.b $6c(a6), d6
pub fn i_099998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9999C;
    cpu.ir = 0x1C2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09999C: lsr.b #$4, d6
pub fn i_09999c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9999E;
    cpu.ir = 0xE80E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $09999E: cmp.b (a0, d7.w), d6
pub fn i_09999e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999A2;
    cpu.ir = 0xBC30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $0999A2: beq.w $999ba
pub fn i_0999a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x999BA; return 10; }
    8
}
/// $0999A6: move.w $4(a0, d7.w), d5
pub fn i_0999a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999AA;
    cpu.ir = 0x3A30;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0999AA: sub.w d2, d5
pub fn i_0999aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999AC;
    cpu.ir = 0x9A42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0999AC: move.b (a0, d7.w), d6
pub fn i_0999ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999B0;
    cpu.ir = 0x1C30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0999B0: movem.l (a7)+, d7/a0
pub fn i_0999b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999B4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
// ===== SetC_0999b4  ($0999B4, 6 B, thunks/ccr_helpers.c)
/// $0999B4: ori.b #$1, ccr
pub fn i_0999b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999B8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $0999B8: rts
pub fn i_0999b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_FindNearest_Next_0999ba  ($0999BA, 16 B, frontend/hiscore_memcard_options_09773c.s)
/// $0999BA: addi.w #$c, d7
pub fn i_0999ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999BE;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0999BE: cmpi.w #$c0, d7
pub fn i_0999be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999C2;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $0999C2: bcs.w $999c8
pub fn i_0999c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x999C8; return 10; }
    8
}
/// $0999C6: clr.w d7
pub fn i_0999c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999C8;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0999C8: bra.b $9994c
pub fn i_0999c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999CA;
    cpu.ir = 0x6082;
    cpu.pc = 0x9994C;
    10
}
// ===== Trail_MergeIdNibble_0999ca  ($0999CA, 14 B, frontend/hiscore_memcard_options_09773c.s)
/// $0999CA: move.b $6c(a6), d7
pub fn i_0999ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999CE;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0999CE: andi.w #$f0, d7
pub fn i_0999ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999D2;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0999D2: andi.w #$f, d0
pub fn i_0999d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999D6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0999D6: or.w d7, d0
pub fn i_0999d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999D8;
    cpu.ir = 0x8047;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
// ===== SetTaskB_0999d8  ($0999D8, 6 B, kernel/task_setters_b.c)
/// $0999D8: move.b d0, $6c(a6)
pub fn i_0999d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999DC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0999DC: rts
pub fn i_0999dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Clipping_Test_0999DE  ($0999DE, 24 B, video/clipping_test_0999de.s)
/// $0999DE: move.b $6c(a6), d5
pub fn i_0999de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999E2;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x6C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0999E2: move.b d5, d6
pub fn i_0999e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999E4;
    cpu.ir = 0x1C05;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0999E4: andi.b #$f, d5
pub fn i_0999e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999E8;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0999E8: cmpi.b #$f, d5
pub fn i_0999e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999EC;
    cpu.ir = 0xC05;
    let v1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $0999EC: bne.w $999f6
pub fn i_0999ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x999F6; return 10; }
    8
}
/// $0999F0: andi.b #$fe, ccr
pub fn i_0999f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999F4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0999F4: rts
pub fn i_0999f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_LookupById_0999f6  ($0999F6, 38 B, frontend/hiscore_memcard_options_09773c.s)
/// $0999F6: lsr.b #$4, d6
pub fn i_0999f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999F8;
    cpu.ir = 0xE80E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $0999F8: cmp.b d6, d5
pub fn i_0999f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999FA;
    cpu.ir = 0xBA06;
    let v1 = cpu.d[6] & 0xFF;
    let h1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $0999FA: bne.w $99a00
pub fn i_0999fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x999FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99A00; return 10; }
    8
}
/// $0999FE: trap #$f
pub fn i_0999fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A00;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $099A00: lea.l $10e3be.l, a0
pub fn i_099a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A06;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E3BE;
    20
}
/// $099A06: move.w $10e47e.l, d7
pub fn i_099a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A0C;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E47E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $099A0C: cmp.w $10e480.l, d7
pub fn i_099a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A12;
    cpu.ir = 0xBE79;
    let v2 = bus.read16(0x10E480) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $099A12: bne.w $99a22
pub fn i_099a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99A22; return 10; }
    8
}
/// $099A16: ori.b #$f, $6c(a6)
pub fn i_099a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A1C;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x6C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== ClearC_099a1c  ($099A1C, 6 B, thunks/ccr_helpers.c)
/// $099A1C: andi.b #$fe, ccr
pub fn i_099a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A20;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $099A20: rts
pub fn i_099a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trail_LookupById_Step_099a22  ($099A22, 34 B, frontend/hiscore_memcard_options_09773c.s)
/// $099A22: cmp.b (a0, d7.w), d5
pub fn i_099a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A26;
    cpu.ir = 0xBA30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[5] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $099A26: beq.w $99a3a
pub fn i_099a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99A3A; return 10; }
    8
}
/// $099A2A: addi.w #$c, d7
pub fn i_099a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A2E;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099A2E: cmpi.w #$c0, d7
pub fn i_099a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A32;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $099A32: bcs.w $99a38
pub fn i_099a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A36;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x99A38; return 10; }
    8
}
/// $099A36: clr.w d7
pub fn i_099a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A38;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099A38: bra.b $99a0c
pub fn i_099a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A3A;
    cpu.ir = 0x60D2;
    cpu.pc = 0x99A0C;
    10
}
/// $099A3A: adda.w d7, a0
pub fn i_099a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A3C;
    cpu.ir = 0xD0C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $099A3C: move.w $8(a0), d2
pub fn i_099a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A40;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099A40: move.w $a(a0), d3
pub fn i_099a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A44;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
// ===== SetC_099a44  ($099A44, 6 B, thunks/ccr_helpers.c)
/// $099A44: ori.b #$1, ccr
pub fn i_099a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A48;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $099A48: rts
pub fn i_099a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugCursor_Init_099a4a  ($099A4A, 18 B, frontend/hiscore_memcard_options_09773c.s)
/// $099A4A: move.w #$e, d1
pub fn i_099a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A4E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099A4E: jsr $236e.l
pub fn i_099a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99A54);
    cpu.pc = 0x236E;
    16
}
/// $099A54: move.l #$2507fa, $3c(a6)
pub fn i_099a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A5C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2507FA;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== SetTaskHandler_099a5c  ($099A5C, 8 B, kernel/task_handlers.c)
/// $099A5C: lea.l $99a64(pc), a1
pub fn i_099a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x99A64;
    16
}
/// $099A60: move.l a1, (a6)
pub fn i_099a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $099A62: rts
pub fn i_099a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugCursor_Run_099a64  ($099A64, 90 B, frontend/hiscore_memcard_options_09773c.s)
/// $099A64: move.b $10fd9c.l, d0
pub fn i_099a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A6A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10FD9C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $099A6A: btst.b #$0, d0
pub fn i_099a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A6E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099A6E: beq.w $99a76
pub fn i_099a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A72;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99A76; return 10; }
    8
}
/// $099A72: addq.w #$8, $2a(a6)
pub fn i_099a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A76;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099A76: btst.b #$1, d0
pub fn i_099a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A7A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099A7A: beq.w $99a82
pub fn i_099a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99A82; return 10; }
    8
}
/// $099A7E: subq.w #$8, $2a(a6)
pub fn i_099a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A82;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099A82: btst.b #$2, d0
pub fn i_099a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A86;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099A86: beq.w $99a8e
pub fn i_099a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A8A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99A8E; return 10; }
    8
}
/// $099A8A: subq.w #$8, $28(a6)
pub fn i_099a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A8E;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099A8E: btst.b #$3, d0
pub fn i_099a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A92;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099A92: beq.w $99a9a
pub fn i_099a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A96;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99A9A; return 10; }
    8
}
/// $099A96: addq.w #$8, $28(a6)
pub fn i_099a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99A9A;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099A9A: jsr $27cee.l
pub fn i_099a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99AA0);
    cpu.pc = 0x27CEE;
    16
}
/// $099AA0: move.w $22(a6), d0
pub fn i_099aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AA4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099AA4: move.w $24(a6), d1
pub fn i_099aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AA8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099AA8: subi.w #$10, d0
pub fn i_099aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AAC;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099AAC: addq.w #$7, d1
pub fn i_099aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AAE;
    cpu.ir = 0x5E41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 7, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099AAE: move.w #$20, d2
pub fn i_099aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AB2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099AB2: andi.w #$1ff, d0
pub fn i_099ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AB6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099AB6: andi.w #$1ff, d1
pub fn i_099ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ABA;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099ABA: jsr $99812(pc)
pub fn i_099aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ABE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99ABE);
    cpu.pc = 0x99812;
    16
}
// ===== JsrAbsThunk_099abe  ($099ABE, 8 B, thunks/jsr_abs_thunks.c)
/// $099ABE: jsr $5ca2a.l
pub fn i_099abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99AC4);
    cpu.pc = 0x5CA2A;
    16
}
/// $099AC4: rts
pub fn i_099ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_099ac6  ($099AC6, 16 B, frontend/hiscore_memcard_options_09773c.s)
/// $099AC6: movea.l $8(a6), a1
pub fn i_099ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ACA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $099ACA: move.b $10(a6), d0
pub fn i_099aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ACE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099ACE: cmp.b $10(a1), d0
pub fn i_099ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AD2;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $099AD2: bcs.w $99adc
pub fn i_099ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AD6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x99ADC; return 10; }
    8
}
// ===== ClearXN_099ad6  ($099AD6, 6 B, thunks/ccr_helpers.c)
/// $099AD6: andi.b #$ee, ccr
pub fn i_099ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ADA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $099ADA: rts
pub fn i_099ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ADC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_099adc  ($099ADC, 6 B, thunks/ccr_helpers.c)
/// $099ADC: ori.b #$11, ccr
pub fn i_099adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AE0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $099AE0: rts
pub fn i_099ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AttractInit_Single_099AE2  ($099AE2, 26 B, frontend/attract_init_single_099ae2.s)
/// $099AE2: move.b #$3, $10fd88.l
pub fn i_099ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AEA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD88, v1 as u8);
    16
}
/// $099AEA: move.b #$2, $10fd8b.l
pub fn i_099aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AF2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10FD8B, v1 as u8);
    16
}
/// $099AF2: move.b #$0, $10e486.l
pub fn i_099af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AFA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E486, v1 as u8);
    16
}
/// $099AFA: rts
pub fn i_099afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99AFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Global_Clear10E486_099AFC  ($099AFC, 10 B, kernel/global_clear_10e486_099afc.s)
/// $099AFC: move.b #$0, $10e486.l
pub fn i_099afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B04;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E486, v1 as u8);
    16
}
/// $099B04: rts
pub fn i_099b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_099b7e  ($099B7E, 6 B, thunks/jsr_pc_thunks.c)
/// $099B7E: jsr $99ee4(pc)
pub fn i_099b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99B82);
    cpu.pc = 0x99EE4;
    16
}
/// $099B82: rts
pub fn i_099b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionSelect2_Left_099b84  ($099B84, 26 B, frontend/hiscore_memcard_options_09773c.s)
/// $099B84: jsr $99ed2(pc)
pub fn i_099b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B88;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99B88);
    cpu.pc = 0x99ED2;
    16
}
/// $099B88: btst.b #$0, d0
pub fn i_099b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B8C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099B8C: beq.w $99ba4
pub fn i_099b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99BA4; return 10; }
    8
}
/// $099B90: jsr $99f10(pc)
pub fn i_099b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99B94);
    cpu.pc = 0x99F10;
    16
}
/// $099B94: subq.w #$1, $78(a6)
pub fn i_099b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B98;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099B98: andi.w #$1, $78(a6)
pub fn i_099b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99B9E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== JsrPcThunk_099b9e  ($099B9E, 6 B, thunks/jsr_pc_thunks.c)
/// $099B9E: jsr $99ee4(pc)
pub fn i_099b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99BA2);
    cpu.pc = 0x99EE4;
    16
}
/// $099BA2: rts
pub fn i_099ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionSelect2_Rts_099ba4  ($099BA4, 2 B, frontend/hiscore_memcard_options_09773c.s)
/// $099BA4: rts
pub fn i_099ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99BA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_099d62  ($099D62, 6 B, thunks/jsr_pc_thunks.c)
/// $099D62: jsr $99de4(pc)
pub fn i_099d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99D66);
    cpu.pc = 0x99DE4;
    16
}
/// $099D66: rts
pub fn i_099d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_Row1_099d68  ($099D68, 10 B, frontend/hiscore_memcard_options_09773c.s)
/// $099D68: cmpi.w #$1, $78(a6)
pub fn i_099d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D6E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $099D6E: bne.w $99d78
pub fn i_099d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99D78; return 10; }
    8
}
// ===== JsrPcThunk_099d72  ($099D72, 6 B, thunks/jsr_pc_thunks.c)
/// $099D72: jsr $99e14(pc)
pub fn i_099d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99D76);
    cpu.pc = 0x99E14;
    16
}
/// $099D76: rts
pub fn i_099d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_Row2_099d78  ($099D78, 22 B, frontend/hiscore_memcard_options_09773c.s)
/// $099D78: cmpi.b #$2, $10fd83.l
pub fn i_099d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D80;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $099D80: bne.w $99d94
pub fn i_099d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99D94; return 10; }
    8
}
/// $099D84: cmpi.w #$2, $78(a6)
pub fn i_099d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D8A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $099D8A: bne.w $99d94
pub fn i_099d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D8E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99D94; return 10; }
    8
}
// ===== JsrPcThunk_099d8e  ($099D8E, 6 B, thunks/jsr_pc_thunks.c)
/// $099D8E: jsr $99e9c(pc)
pub fn i_099d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99D92);
    cpu.pc = 0x99E9C;
    16
}
/// $099D92: rts
pub fn i_099d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_NavDown_099d94  ($099D94, 36 B, frontend/hiscore_memcard_options_09773c.s)
/// $099D94: jsr $99ed2(pc)
pub fn i_099d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99D98);
    cpu.pc = 0x99ED2;
    16
}
/// $099D98: btst.b #$1, d0
pub fn i_099d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99D9C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099D9C: beq.w $99dbe
pub fn i_099d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DA0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99DBE; return 10; }
    8
}
/// $099DA0: jsr $99f86(pc)
pub fn i_099da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99DA4);
    cpu.pc = 0x99F86;
    16
}
/// $099DA4: addq.w #$1, $78(a6)
pub fn i_099da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DA8;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099DA8: move.w $78(a6), d0
pub fn i_099da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DAC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099DAC: cmp.w $7a(a6), d0
pub fn i_099dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DB0;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $099DB0: bls.w $99db8
pub fn i_099db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DB4;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x99DB8; return 10; }
    8
}
/// $099DB4: clr.w $78(a6)
pub fn i_099db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DB8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== JsrPcThunk_099db8  ($099DB8, 6 B, thunks/jsr_pc_thunks.c)
/// $099DB8: jsr $99f3a(pc)
pub fn i_099db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99DBC);
    cpu.pc = 0x99F3A;
    16
}
/// $099DBC: rts
pub fn i_099dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_NavUp_099dbe  ($099DBE, 30 B, frontend/hiscore_memcard_options_09773c.s)
/// $099DBE: jsr $99ed2(pc)
pub fn i_099dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99DC2);
    cpu.pc = 0x99ED2;
    16
}
/// $099DC2: btst.b #$0, d0
pub fn i_099dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DC6;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099DC6: beq.w $99de2
pub fn i_099dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DCA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99DE2; return 10; }
    8
}
/// $099DCA: jsr $99f86(pc)
pub fn i_099dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DCE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99DCE);
    cpu.pc = 0x99F86;
    16
}
/// $099DCE: subq.w #$1, $78(a6)
pub fn i_099dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DD2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $099DD2: bcc.w $99ddc
pub fn i_099dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99DDC; return 10; }
    8
}
/// $099DD6: move.w $7a(a6), $78(a6)
pub fn i_099dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DDC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad4, v3 as u16);
    20
}
// ===== JsrPcThunk_099ddc  ($099DDC, 6 B, thunks/jsr_pc_thunks.c)
/// $099DDC: jsr $99f3a(pc)
pub fn i_099ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99DE0);
    cpu.pc = 0x99F3A;
    16
}
/// $099DE0: rts
pub fn i_099de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_Rts_099de2  ($099DE2, 2 B, frontend/hiscore_memcard_options_09773c.s)
/// $099DE2: rts
pub fn i_099de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_AdjDifficulty_099de4  ($099DE4, 18 B, frontend/hiscore_memcard_options_09773c.s)
/// $099DE4: btst.b #$2, d0
pub fn i_099de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DE8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099DE8: beq.w $99dfc
pub fn i_099de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DEC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99DFC; return 10; }
    8
}
/// $099DEC: subq.b #$1, $72(a6)
pub fn i_099dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DF0;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099DF0: andi.b #$3, $72(a6)
pub fn i_099df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DF6;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== JsrPcThunk_099df6  ($099DF6, 6 B, thunks/jsr_pc_thunks.c)
/// $099DF6: jsr $99fd2(pc)
pub fn i_099df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99DFA);
    cpu.pc = 0x99FD2;
    16
}
/// $099DFA: rts
pub fn i_099dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99DFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_AdjDifficulty_Right_099dfc  ($099DFC, 18 B, frontend/hiscore_memcard_options_09773c.s)
/// $099DFC: btst.b #$3, d0
pub fn i_099dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E00;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099E00: beq.w $99e12
pub fn i_099e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99E12; return 10; }
    8
}
/// $099E04: addq.b #$1, $72(a6)
pub fn i_099e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E08;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099E08: andi.b #$3, $72(a6)
pub fn i_099e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E0E;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== JsrPcThunk_099e0e  ($099E0E, 6 B, thunks/jsr_pc_thunks.c)
/// $099E0E: jsr $99fd2(pc)
pub fn i_099e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99E12);
    cpu.pc = 0x99FD2;
    16
}
/// $099E12: rts
pub fn i_099e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_AdjLives_099e14  ($099E14, 28 B, frontend/hiscore_memcard_options_09773c.s)
/// $099E14: btst.b #$2, d0
pub fn i_099e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E18;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099E18: beq.w $99e36
pub fn i_099e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E1C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99E36; return 10; }
    8
}
/// $099E1C: subq.b #$1, $73(a6)
pub fn i_099e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E20;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099E20: cmpi.b #$1, $73(a6)
pub fn i_099e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E26;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $099E26: bcc.w $99e30
pub fn i_099e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99E30; return 10; }
    8
}
/// $099E2A: move.b #$1, $73(a6)
pub fn i_099e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E30;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_099e30  ($099E30, 6 B, thunks/jsr_pc_thunks.c)
/// $099E30: jsr $99ff2(pc)
pub fn i_099e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99E34);
    cpu.pc = 0x99FF2;
    16
}
/// $099E34: rts
pub fn i_099e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_AdjLives_Right_099e36  ($099E36, 28 B, frontend/hiscore_memcard_options_09773c.s)
/// $099E36: btst.b #$3, d0
pub fn i_099e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E3A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099E3A: beq.w $99e56
pub fn i_099e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99E56; return 10; }
    8
}
/// $099E3E: addq.b #$1, $73(a6)
pub fn i_099e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E42;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099E42: cmpi.b #$5, $73(a6)
pub fn i_099e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E48;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $099E48: bls.w $99e52
pub fn i_099e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E4C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x99E52; return 10; }
    8
}
/// $099E4C: move.b #$5, $73(a6)
pub fn i_099e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_099e52  ($099E52, 6 B, thunks/jsr_pc_thunks.c)
/// $099E52: jsr $99ff2(pc)
pub fn i_099e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99E56);
    cpu.pc = 0x99FF2;
    16
}
/// $099E56: rts
pub fn i_099e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_AdjCredits_099e58  ($099E58, 28 B, frontend/hiscore_memcard_options_09773c.s)
/// $099E58: btst.b #$2, d0
pub fn i_099e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E5C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099E5C: beq.w $99e7a
pub fn i_099e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99E7A; return 10; }
    8
}
/// $099E60: subq.b #$1, $74(a6)
pub fn i_099e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E64;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099E64: cmpi.b #$1, $74(a6)
pub fn i_099e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E6A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $099E6A: bcc.w $99e74
pub fn i_099e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x99E74; return 10; }
    8
}
/// $099E6E: move.b #$1, $74(a6)
pub fn i_099e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E74;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_099e74  ($099E74, 6 B, thunks/jsr_pc_thunks.c)
/// $099E74: jsr $9a03c(pc)
pub fn i_099e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99E78);
    cpu.pc = 0x9A03C;
    16
}
/// $099E78: rts
pub fn i_099e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_AdjCredits_Right_099e7a  ($099E7A, 28 B, frontend/hiscore_memcard_options_09773c.s)
/// $099E7A: btst.b #$3, d0
pub fn i_099e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E7E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099E7E: beq.w $99e9a
pub fn i_099e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99E9A; return 10; }
    8
}
/// $099E82: addq.b #$1, $74(a6)
pub fn i_099e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E86;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $099E86: cmpi.b #$5, $74(a6)
pub fn i_099e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E8C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $099E8C: bls.w $99e96
pub fn i_099e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E90;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x99E96; return 10; }
    8
}
/// $099E90: move.b #$5, $74(a6)
pub fn i_099e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E96;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_099e96  ($099E96, 6 B, thunks/jsr_pc_thunks.c)
/// $099E96: jsr $9a03c(pc)
pub fn i_099e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99E9A);
    cpu.pc = 0x9A03C;
    16
}
/// $099E9A: rts
pub fn i_099e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99E9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_Adj2PMode_099e9c  ($099E9C, 28 B, frontend/hiscore_memcard_options_09773c.s)
/// $099E9C: cmpi.b #$2, $10fd83.l
pub fn i_099e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EA4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $099EA4: beq.w $99eaa
pub fn i_099ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99EAA; return 10; }
    8
}
/// $099EA8: rts
pub fn i_099ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $099EAA: btst.b #$2, d0
pub fn i_099eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EAE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099EAE: beq.w $99ebe
pub fn i_099eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EB2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99EBE; return 10; }
    8
}
/// $099EB2: move.b #$0, $76(a6)
pub fn i_099eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EB8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_099eb8  ($099EB8, 6 B, thunks/jsr_pc_thunks.c)
/// $099EB8: jsr $9a086(pc)
pub fn i_099eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99EBC);
    cpu.pc = 0x9A086;
    16
}
/// $099EBC: rts
pub fn i_099ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionsMenu_Adj2PMode_Right_099ebe  ($099EBE, 14 B, frontend/hiscore_memcard_options_09773c.s)
/// $099EBE: btst.b #$3, d0
pub fn i_099ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EC2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $099EC2: beq.w $99ed0
pub fn i_099ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EC6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x99ED0; return 10; }
    8
}
/// $099EC6: move.b #$1, $76(a6)
pub fn i_099ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ECC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== JsrPcThunk_099ecc  ($099ECC, 6 B, thunks/jsr_pc_thunks.c)
/// $099ECC: jsr $9a086(pc)
pub fn i_099ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ED0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x99ED0);
    cpu.pc = 0x9A086;
    16
}
/// $099ED0: rts
pub fn i_099ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ED2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_PressedEither_099ed2  ($099ED2, 18 B, frontend/hiscore_memcard_options_09773c.s)
/// $099ED2: move.b $10e215.l, d0
pub fn i_099ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99ED8;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E215) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $099ED8: bne.w $99ee2
pub fn i_099ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EDC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x99EE2; return 10; }
    8
}
/// $099EDC: move.b $10e21b.l, d0
pub fn i_099edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EE2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E21B) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $099EE2: rts
pub fn i_099ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionSelect_DrawCursor_099ee4  ($099EE4, 44 B, frontend/hiscore_memcard_options_09773c.s)
/// $099EE4: lea.l $2f6232.l, a1
pub fn i_099ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EEA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F6232;
    20
}
/// $099EEA: move.w $78(a6), d2
pub fn i_099eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EEE;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099EEE: lsl.w #$1, d2
pub fn i_099eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EF0;
    cpu.ir = 0xE34A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $099EF0: move.w (a1, d2.w), d0
pub fn i_099ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EF4;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $099EF4: move.w #$237b, d1
pub fn i_099ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99EF8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x237B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099EF8: movem.w d0-d1, $3c0000.l
pub fn i_099ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F00;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F00: addi.w #$20, d0
pub fn i_099f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F04;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F04: addq.w #$1, d1
pub fn i_099f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F06;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099F06: movem.w d0-d1, $3c0000.l
pub fn i_099f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F0E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F0E: rts
pub fn i_099f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== OptionSelect_EraseCursor_099f10  ($099F10, 42 B, frontend/hiscore_memcard_options_09773c.s)
/// $099F10: lea.l $2f6232.l, a1
pub fn i_099f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F16;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F6232;
    20
}
/// $099F16: move.w $78(a6), d2
pub fn i_099f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F1A;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099F1A: lsl.w #$1, d2
pub fn i_099f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F1C;
    cpu.ir = 0xE34A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $099F1C: move.w (a1, d2.w), d0
pub fn i_099f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F20;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $099F20: move.w #$2320, d1
pub fn i_099f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F24;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099F24: movem.w d0-d1, $3c0000.l
pub fn i_099f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F2C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F2C: addi.w #$20, d0
pub fn i_099f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F30;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F30: movem.w d0-d1, $3c0000.l
pub fn i_099f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F38;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F38: rts
pub fn i_099f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixGlyph16_DrawCursorA_099F3A  ($099F3A, 76 B, video/hud_fix_glyph16_099f3a.s)
/// $099F3A: movea.l $80(a6), a1
pub fn i_099f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F3E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $099F3E: move.w $78(a6), d2
pub fn i_099f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F42;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099F42: lsl.w #$1, d2
pub fn i_099f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F44;
    cpu.ir = 0xE34A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $099F44: move.w (a1, d2.w), d0
pub fn i_099f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F48;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $099F48: move.w #$4b22, d1
pub fn i_099f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F4C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4B22;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099F4C: movem.w d0-d1, $3c0000.l
pub fn i_099f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F54;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F54: addi.w #$20, d0
pub fn i_099f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F58;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F58: addq.w #$1, d1
pub fn i_099f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F5A;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099F5A: movem.w d0-d1, $3c0000.l
pub fn i_099f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F62;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F62: subi.w #$1f, d0
pub fn i_099f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F66;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F66: addi.w #$f, d1
pub fn i_099f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F6A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F6A: movem.w d0-d1, $3c0000.l
pub fn i_099f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F72;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F72: addi.w #$20, d0
pub fn i_099f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F76;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F76: addq.w #$1, d1
pub fn i_099f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F78;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099F78: movem.w d0-d1, $3c0000.l
pub fn i_099f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F80;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099F80: subi.w #$21, d0
pub fn i_099f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F84;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099F84: rts
pub fn i_099f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixGlyph16_DrawCursorB_099F86  ($099F86, 76 B, video/hud_fix_glyph16_099f3a.s)
/// $099F86: movea.l $80(a6), a1
pub fn i_099f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F8A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $099F8A: move.w $78(a6), d2
pub fn i_099f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F8E;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $099F8E: lsl.w #$1, d2
pub fn i_099f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F90;
    cpu.ir = 0xE34A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $099F90: move.w (a1, d2.w), d0
pub fn i_099f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F94;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $099F94: move.w #$4b40, d1
pub fn i_099f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99F98;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4B40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099F98: movem.w d0-d1, $3c0000.l
pub fn i_099f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FA0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099FA0: addi.w #$20, d0
pub fn i_099fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FA4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099FA4: addq.w #$1, d1
pub fn i_099fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FA6;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099FA6: movem.w d0-d1, $3c0000.l
pub fn i_099fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FAE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099FAE: subi.w #$1f, d0
pub fn i_099fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FB2;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099FB2: addi.w #$f, d1
pub fn i_099fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FB6;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099FB6: movem.w d0-d1, $3c0000.l
pub fn i_099fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FBE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099FBE: addi.w #$20, d0
pub fn i_099fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FC2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099FC2: addq.w #$1, d1
pub fn i_099fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FC4;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099FC4: movem.w d0-d1, $3c0000.l
pub fn i_099fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FCC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $099FCC: subi.w #$21, d0
pub fn i_099fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FD0;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $099FD0: rts
pub fn i_099fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixGlyphRun_Draw2F61F0_099FD2  ($099FD2, 24 B, video/hud_fix_glyph16_099f3a.s)
/// $099FD2: clr.l d0
pub fn i_099fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FD4;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $099FD4: move.b $72(a6), d0
pub fn i_099fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FD8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099FD8: lsl.l #$3, d0
pub fn i_099fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FDA;
    cpu.ir = 0xE788;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 1, true);
    cpu.d[0] = r;
    12
}
/// $099FDA: lea.l $2f61f0.l, a2
pub fn i_099fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FE0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F61F0;
    20
}
/// $099FE0: adda.l d0, a2
pub fn i_099fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FE2;
    cpu.ir = 0xD5C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $099FE2: movea.w #$72eb, a1
pub fn i_099fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FE6;
    cpu.ir = 0x327C;
    let v1: u32 = 0x72EB;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $099FE6: move.w #$4, d1
pub fn i_099fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FEA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_099fea  ($099FEA, 8 B, thunks/jsr_abs_thunks.c)
/// $099FEA: jsr $4784c.l
pub fn i_099fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x99FF0);
    cpu.pc = 0x4784C;
    16
}
/// $099FF0: rts
pub fn i_099ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixGlyph16_DrawDigit72EF_099FF2  ($099FF2, 74 B, video/hud_fix_glyph16_099f3a.s)
/// $099FF2: move.w #$72ef, d0
pub fn i_099ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FF6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x72EF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $099FF6: clr.w d1
pub fn i_099ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FF8;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $099FF8: move.b $73(a6), d1
pub fn i_099ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FFC;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $099FFC: lsl.w #$1, d1
pub fn i_099ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x99FFE;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $099FFE: addi.w #$4b60, d1
pub fn i_099ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A002;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4B60, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A002: movem.w d0-d1, $3c0000.l
pub fn i_09a002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A00A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A00A: addi.w #$20, d0
pub fn i_09a00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A00E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A00E: addq.w #$1, d1
pub fn i_09a00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A010;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A010: movem.w d0-d1, $3c0000.l
pub fn i_09a010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A018;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A018: subi.w #$1f, d0
pub fn i_09a018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A01C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A01C: addi.w #$f, d1
pub fn i_09a01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A020;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A020: movem.w d0-d1, $3c0000.l
pub fn i_09a020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A028;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A028: addi.w #$20, d0
pub fn i_09a028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A02C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A02C: addq.w #$1, d1
pub fn i_09a02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A02E;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A02E: movem.w d0-d1, $3c0000.l
pub fn i_09a02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A036;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A036: subi.w #$21, d0
pub fn i_09a036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A03A;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A03A: rts
pub fn i_09a03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A03C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixGlyph16_DrawDigit72F3_09A03C  ($09A03C, 74 B, video/hud_fix_glyph16_099f3a.s)
/// $09A03C: move.w #$72f3, d0
pub fn i_09a03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A040;
    cpu.ir = 0x303C;
    let v1: u32 = 0x72F3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A040: clr.w d1
pub fn i_09a040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A042;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A042: move.b $74(a6), d1
pub fn i_09a042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A046;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A046: lsl.w #$1, d1
pub fn i_09a046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A048;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $09A048: addi.w #$4b60, d1
pub fn i_09a048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A04C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4B60, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A04C: movem.w d0-d1, $3c0000.l
pub fn i_09a04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A054;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A054: addi.w #$20, d0
pub fn i_09a054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A058;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A058: addq.w #$1, d1
pub fn i_09a058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A05A;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A05A: movem.w d0-d1, $3c0000.l
pub fn i_09a05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A062;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A062: subi.w #$1f, d0
pub fn i_09a062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A066;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A066: addi.w #$f, d1
pub fn i_09a066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A06A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A06A: movem.w d0-d1, $3c0000.l
pub fn i_09a06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A072;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A072: addi.w #$20, d0
pub fn i_09a072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A076;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A076: addq.w #$1, d1
pub fn i_09a076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A078;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A078: movem.w d0-d1, $3c0000.l
pub fn i_09a078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A080;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $09A080: subi.w #$21, d0
pub fn i_09a080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A084;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A084: rts
pub fn i_09a084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A086;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixGlyphRun_DrawPad2P_09A086  ($09A086, 46 B, video/hud_fix_glyph16_099f3a.s)
/// $09A086: cmpi.b #$2, $10fd83.l
pub fn i_09a086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A08E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $09A08E: bne.w $9a0ba
pub fn i_09a08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A092;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A0BA; return 10; }
    8
}
/// $09A092: cmpi.b #$1, $76(a6)
pub fn i_09a092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A098;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $09A098: bne.w $9a0a6
pub fn i_09a098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A09C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A0A6; return 10; }
    8
}
/// $09A09C: lea.l $2f6226.l, a2
pub fn i_09a09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0A2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F6226;
    20
}
/// $09A0A2: bra.w $9a0ac
pub fn i_09a0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A0AC;
    10
}
/// $09A0A6: lea.l $2f621a.l, a2
pub fn i_09a0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0AC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F621A;
    20
}
/// $09A0AC: movea.w #$72f3, a1
pub fn i_09a0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0B0;
    cpu.ir = 0x327C;
    let v1: u32 = 0x72F3;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $09A0B0: move.w #$4, d1
pub fn i_09a0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_09a0b4  ($09A0B4, 8 B, thunks/jsr_abs_thunks.c)
/// $09A0B4: jsr $477fc.l
pub fn i_09a0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A0BA);
    cpu.pc = 0x477FC;
    16
}
/// $09A0BA: rts
pub fn i_09a0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_09a0bc  ($09A0BC, 16 B, props/items_score_crates_09a0bc.s)
/// $09A0BC: movea.l $8(a6), a1
pub fn i_09a0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0C0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09A0C0: move.b $10(a6), d0
pub fn i_09a0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0C4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A0C4: cmp.b $10(a1), d0
pub fn i_09a0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0C8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $09A0C8: bcs.w $9a0d2
pub fn i_09a0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0CC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A0D2; return 10; }
    8
}
// ===== ClearXN_09a0cc  ($09A0CC, 6 B, thunks/ccr_helpers.c)
/// $09A0CC: andi.b #$ee, ccr
pub fn i_09a0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0D0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09A0D0: rts
pub fn i_09a0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_09a0d2  ($09A0D2, 6 B, thunks/ccr_helpers.c)
/// $09A0D2: ori.b #$11, ccr
pub fn i_09a0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0D6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09A0D6: rts
pub fn i_09a0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gun_Hitbox_09a0d8  ($09A0D8, 8 B, props/items_score_crates_09a0bc.s)
/// $09A0D8: dc.w $ffd0
pub fn i_09a0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0DA;
    cpu.ir = 0xFFD0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Gun_GroundTail_09a0e0  ($09A0E0, 14 B, props/items_score_crates_09a0bc.s)
/// $09A0E0: lea.l $9a0d8(pc), a0
pub fn i_09a0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0E4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A0D8;
    16
}
/// $09A0E4: jsr $5dd5c.l
pub fn i_09a0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A0EA);
    cpu.pc = 0x5DD5C;
    16
}
/// $09A0EA: bcc.w $9a0fa
pub fn i_09a0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A0FA; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_09a0ee  ($09A0EE, 14 B, kernel/jsr5b6_jmp518.c)
/// $09A0EE: jsr $5b6.l
pub fn i_09a0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A0F4);
    cpu.pc = 0x5B6;
    16
}
/// $09A0F4: jmp $518.l
pub fn i_09a0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0FA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A0FA: rts
pub fn i_09a0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A0FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gun_UpdateAngleSprite_09a0fc  ($09A0FC, 138 B, props/items_score_crates_09a0bc.s)
/// $09A0FC: move.w $70(a6), d0
pub fn i_09a0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A100;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A100: asr.w #$8, d0
pub fn i_09a100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A102;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $09A102: addq.w #$4, d0
pub fn i_09a102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A104;
    cpu.ir = 0x5840;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A104: andi.w #$f8, d0
pub fn i_09a104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A108;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A108: cmpi.b #$88, d0
pub fn i_09a108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A10C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x88, Size::Byte, false);
    8
}
/// $09A10C: bcs.w $9a112
pub fn i_09a10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A110;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A112; return 10; }
    8
}
/// $09A110: neg.b d0
pub fn i_09a110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A112;
    cpu.ir = 0x4400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A112: cmpi.w #$40, d0
pub fn i_09a112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A116;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $09A116: bcs.w $9a12e
pub fn i_09a116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A11A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A12E; return 10; }
    8
}
/// $09A11A: bset.b #$0, $3a(a6)
pub fn i_09a11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A120;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09A120: subi.b #$40, d0
pub fn i_09a120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A124;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x40, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09A124: neg.b d0
pub fn i_09a124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A126;
    cpu.ir = 0x4400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A126: addi.b #$40, d0
pub fn i_09a126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A12A;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09A12A: bra.w $9a134
pub fn i_09a12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A12E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A134;
    10
}
/// $09A12E: bclr.b #$0, $3a(a6)
pub fn i_09a12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A134;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09A134: andi.w #$78, d0
pub fn i_09a134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A138;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x78) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A138: lea.l $2f6540.l, a0
pub fn i_09a138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A13E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6540;
    20
}
/// $09A13E: movea.l (a0, d0.w), a1
pub fn i_09a13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A142;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $09A142: move.b $7(a0, d0.w), d0
pub fn i_09a142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A146;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x7).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09A146: btst.b #$0, $3a(a6)
pub fn i_09a146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A14C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09A14C: beq.w $9a15a
pub fn i_09a14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A150;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A15A; return 10; }
    8
}
/// $09A150: subi.b #$40, d0
pub fn i_09a150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A154;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x40, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09A154: neg.b d0
pub fn i_09a154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A156;
    cpu.ir = 0x4400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A156: addi.b #$40, d0
pub fn i_09a156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A15A;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09A15A: move.b d0, $82(a6)
pub fn i_09a15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A15E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A15E: moveq #$0, d0
pub fn i_09a15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A160;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09A160: move.b $3b(a6), d0
pub fn i_09a160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A164;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A164: beq.w $9a16e
pub fn i_09a164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A168;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A16E; return 10; }
    8
}
/// $09A168: subq.b #$1, d0
pub fn i_09a168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A16A;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A16A: move.b d0, $3b(a6)
pub fn i_09a16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A16E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A16E: add.w d0, d0
pub fn i_09a16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A170;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A170: add.w d0, d0
pub fn i_09a170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A172;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A172: add.w d0, d0
pub fn i_09a172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A174;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A174: move.l (a1, d0.w), $74(a6)
pub fn i_09a174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A17A;
    cpu.ir = 0x2D71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $09A17A: move.l $4(a1, d0.w), $3c(a6)
pub fn i_09a17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A180;
    cpu.ir = 0x2D71;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $09A180: jmp $5ca2a.l
pub fn i_09a180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A186;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CA2A;
    8
}
// ===== Gun_RegisterTarget_09a186  ($09A186, 48 B, props/items_score_crates_09a0bc.s)
/// $09A186: move.b $70(a6), d0
pub fn i_09a186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A18A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A18A: andi.w #$f8, d0
pub fn i_09a18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A18E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A18E: cmpi.b #$88, d0
pub fn i_09a18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A192;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x88, Size::Byte, false);
    8
}
/// $09A192: bcs.w $9a198
pub fn i_09a192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A196;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A198; return 10; }
    8
}
/// $09A196: neg.b d0
pub fn i_09a196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A198;
    cpu.ir = 0x4400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A198: lsr.w #$2, d0
pub fn i_09a198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A19A;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09A19A: lea.l $2f6588.l, a0
pub fn i_09a19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6588;
    20
}
/// $09A1A0: move.w (a0, d0.w), d0
pub fn i_09a1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1A4;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09A1A4: add.w $22(a6), d0
pub fn i_09a1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1A8;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A1A8: move.w $24(a6), d1
pub fn i_09a1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1AC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A1AC: move.b $72(a6), d2
pub fn i_09a1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1B0;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A1B0: jmp $8f3be.l
pub fn i_09a1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1B6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x8F3BE;
    8
}
// ===== Gun_MuzzlePos_09a1b6  ($09A1B6, 202 B, props/items_score_crates_09a0bc.s)
/// $09A1B6: move.b $82(a6), d7
pub fn i_09a1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1BA;
    cpu.ir = 0x1E2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A1BA: andi.w #$f0, d7
pub fn i_09a1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1BE;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A1BE: lsr.w #$1, d7
pub fn i_09a1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1C0;
    cpu.ir = 0xE24F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $09A1C0: lea.l $2f65aa.l, a1
pub fn i_09a1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1C6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F65AA;
    20
}
/// $09A1C6: move.w (a1, d7.w), d0
pub fn i_09a1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1CA;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09A1CA: move.w $2(a1, d7.w), d1
pub fn i_09a1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1CE;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09A1CE: add.w $22(a6), d0
pub fn i_09a1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1D2;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A1D2: add.w $24(a6), d1
pub fn i_09a1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1D6;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A1D6: rts
pub fn i_09a1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09A1D8: tst.b $7b(a6)
pub fn i_09a1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1DC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09A1DC: bne.w $9a1ea
pub fn i_09a1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A1EA; return 10; }
    8
}
/// $09A1E0: jmp $2783a.l
pub fn i_09a1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1E6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x2783A;
    8
}
/// $09A1EA: movea.l $c(a6), a0
pub fn i_09a1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1EE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A1EE: move.w $22(a0), d0
pub fn i_09a1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1F2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A1F2: add.w $7c(a6), d0
pub fn i_09a1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1F6;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A1F6: move.w d0, $22(a6)
pub fn i_09a1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1FA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A1FA: move.w $24(a0), d0
pub fn i_09a1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A1FE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A1FE: add.w $7e(a6), d0
pub fn i_09a1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A202;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A202: move.w d0, $24(a6)
pub fn i_09a202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A206;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A206: jmp $28108.l
pub fn i_09a206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A20C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x28108;
    8
}
/// $09A20E: move.b #$5, $3b(a6)
pub fn i_09a20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A214;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A214: move.l a6, -(a7)
pub fn i_09a214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A216;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $09A216: lea.l $100800.l, a6
pub fn i_09a216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A21C;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $09A21C: move.w #$10f4, d0
pub fn i_09a21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A220;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10F4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A220: jsr $2352.l
pub fn i_09a220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A226);
    cpu.pc = 0x2352;
    16
}
/// $09A226: lea.l $9a520(pc), a1
pub fn i_09a226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A22A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A520;
    16
}
/// $09A22A: jsr $4ae.l
pub fn i_09a22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A230;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A230);
    cpu.pc = 0x4AE;
    16
}
/// $09A230: movea.l (a7)+, a6
pub fn i_09a230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A232;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $09A232: move.b $82(a6), $98(a0)
pub fn i_09a232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A238;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09A238: bsr.w $9a1b6
pub fn i_09a238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A23C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A23C);
    cpu.pc = 0x9A1B6;
    18
}
/// $09A23C: move.w d0, $22(a0)
pub fn i_09a23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A240;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A240: move.w d1, $24(a0)
pub fn i_09a240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A244;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A244: rts
pub fn i_09a244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A246;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09A246: move.b #$5, $3b(a6)
pub fn i_09a246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A24C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A24C: move.l a6, -(a7)
pub fn i_09a24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A24E;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $09A24E: lea.l $100800.l, a6
pub fn i_09a24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A254;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $09A254: move.w #$10f4, d0
pub fn i_09a254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A258;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10F4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A258: jsr $2352.l
pub fn i_09a258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A25E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A25E);
    cpu.pc = 0x2352;
    16
}
/// $09A25E: lea.l $3093a.l, a1
pub fn i_09a25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A264;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3093A;
    20
}
/// $09A264: jsr $4ae.l
pub fn i_09a264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A26A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A26A);
    cpu.pc = 0x4AE;
    16
}
/// $09A26A: movea.l (a7)+, a6
pub fn i_09a26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A26C;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $09A26C: move.b $82(a6), $98(a0)
pub fn i_09a26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A272;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09A272: bsr.w $9a1b6
pub fn i_09a272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A276;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A276);
    cpu.pc = 0x9A1B6;
    18
}
/// $09A276: move.w d0, $22(a0)
pub fn i_09a276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A27A;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A27A: move.w d1, $24(a0)
pub fn i_09a27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A27E;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A27E: rts
pub fn i_09a27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A280;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gun_Child_Init_09a280  ($09A280, 48 B, props/items_score_crates_09a0bc.s)
/// $09A280: movea.l $c(a6), a0
pub fn i_09a280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A284;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A284: move.b $73(a0), d0
pub fn i_09a284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A288;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A288: bmi.w $9a2b6
pub fn i_09a288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A28C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x9A2B6; return 10; }
    8
}
/// $09A28C: jsr $13600.l
pub fn i_09a28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A292;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A292);
    cpu.pc = 0x13600;
    16
}
/// $09A292: movea.l $c(a6), a0
pub fn i_09a292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A296;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A296: move.b $73(a0), d0
pub fn i_09a296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A29A;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A29A: andi.w #$ff, d0
pub fn i_09a29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A29E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A29E: add.w d0, d0
pub fn i_09a29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2A0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A2A0: lea.l $2f65ee.l, a0
pub fn i_09a2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F65EE;
    20
}
/// $09A2A6: move.w (a0, d0.w), d1
pub fn i_09a2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2AA;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09A2AA: jsr $236e.l
pub fn i_09a2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A2B0);
    cpu.pc = 0x236E;
    16
}
// ===== SetTaskHandler_09a2b0  ($09A2B0, 8 B, kernel/task_handlers.c)
/// $09A2B0: lea.l $9a2b8(pc), a1
pub fn i_09a2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A2B8;
    16
}
/// $09A2B4: move.l a1, (a6)
pub fn i_09a2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A2B6: rts
pub fn i_09a2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gun_Child_Sync_09a2b8  ($09A2B8, 64 B, props/items_score_crates_09a0bc.s)
/// $09A2B8: movea.l $c(a6), a0
pub fn i_09a2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2BC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A2BC: move.b $3a(a0), $3a(a6)
pub fn i_09a2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2C2;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09A2C2: movea.l $74(a0), a1
pub fn i_09a2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2C6;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09A2C6: move.b $73(a0), d0
pub fn i_09a2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2CA;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A2CA: bmi.w $9a2f8
pub fn i_09a2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2CE;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x9A2F8; return 10; }
    8
}
/// $09A2CE: andi.w #$ff, d0
pub fn i_09a2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2D2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A2D2: add.w d0, d0
pub fn i_09a2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2D4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A2D4: add.w d0, d0
pub fn i_09a2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2D6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A2D6: move.l (a1, d0.w), $3c(a6)
pub fn i_09a2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2DC;
    cpu.ir = 0x2D71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $09A2DC: move.w $22(a0), $22(a6)
pub fn i_09a2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2E2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A2E2: move.w $24(a0), $24(a6)
pub fn i_09a2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2E8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A2E8: move.w $38(a0), $38(a6)
pub fn i_09a2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2EE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A2EE: jsr $5ca2a.l
pub fn i_09a2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A2F4);
    cpu.pc = 0x5CA2A;
    16
}
/// $09A2F4: bra.w $9a2fe
pub fn i_09a2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A2FE;
    10
}
// ===== SetTaskHandler_09a2f8  ($09A2F8, 8 B, kernel/task_handlers.c)
/// $09A2F8: lea.l $9a280(pc), a1
pub fn i_09a2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A280;
    16
}
/// $09A2FC: move.l a1, (a6)
pub fn i_09a2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A2FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A2FE: rts
pub fn i_09a2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A300;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gun_Init_09a300  ($09A300, 116 B, props/items_score_crates_09a0bc.s)
/// $09A300: move.b #$ff, $7b(a6)
pub fn i_09a300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A306;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A306: movea.l $c(a6), a0
pub fn i_09a306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A30A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A30A: move.w $22(a6), $7c(a6)
pub fn i_09a30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A310;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A310: move.w $24(a6), $7e(a6)
pub fn i_09a310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A316;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A316: bra.w $9a31e
pub fn i_09a316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A31A;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A31E;
    10
}
/// $09A31E: clr.b $3b(a6)
pub fn i_09a31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A322;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09A322: move.w #$d1, d1
pub fn i_09a322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A326;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A326: jsr $236e.l
pub fn i_09a326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A32C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A32C);
    cpu.pc = 0x236E;
    16
}
/// $09A32C: move.w #$8000, d0
pub fn i_09a32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A330;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A330: jsr $28134.l
pub fn i_09a330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A336);
    cpu.pc = 0x28134;
    16
}
/// $09A336: andi.w #$ffe3, $38(a6)
pub fn i_09a336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A33C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A33C: ori.w #$4, $38(a6)
pub fn i_09a33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A342;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A342: jsr $8f3a6.l
pub fn i_09a342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A348;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A348);
    cpu.pc = 0x8F3A6;
    16
}
/// $09A348: not.b d0
pub fn i_09a348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A34A;
    cpu.ir = 0x4600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A34A: move.b d0, $72(a6)
pub fn i_09a34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A34E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A34E: clr.w $70(a6)
pub fn i_09a34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A352;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A352: lea.l $9a280(pc), a1
pub fn i_09a352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A356;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A280;
    16
}
/// $09A356: jsr $4ae.l
pub fn i_09a356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A35C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A35C);
    cpu.pc = 0x4AE;
    16
}
/// $09A35C: move.w #$64, $66(a6)
pub fn i_09a35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A362;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A362: move.w #$2, $1c(a6)
pub fn i_09a362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A368;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A368: jsr $138fe.l
pub fn i_09a368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A36E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A36E);
    cpu.pc = 0x138FE;
    16
}
/// $09A36E: lea.l $9a374(pc), a1
pub fn i_09a36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A372;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A374;
    16
}
/// $09A372: move.l a1, (a6)
pub fn i_09a372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A374;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Gun_Idle_09a374  ($09A374, 60 B, props/items_score_crates_09a0bc.s)
/// $09A374: clr.b $7a(a6)
pub fn i_09a374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A378;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09A378: clr.b $3b(a6)
pub fn i_09a378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A37C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09A37C: move.b #$ff, $73(a6)
pub fn i_09a37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A382;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A382: move.l #$ffffffff, $48(a6)
pub fn i_09a382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A38A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09A38A: lea.l $9a390(pc), a1
pub fn i_09a38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A38E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A390;
    16
}
/// $09A38E: move.l a1, (a6)
pub fn i_09a38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A390;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A390: bsr.w $9a1d8
pub fn i_09a390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A394;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A394);
    cpu.pc = 0x9A1D8;
    18
}
/// $09A394: bsr.w $9a0fc
pub fn i_09a394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A398;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A398);
    cpu.pc = 0x9A0FC;
    18
}
/// $09A398: bsr.w $9a186
pub fn i_09a398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A39C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A39C);
    cpu.pc = 0x9A186;
    18
}
/// $09A39C: tst.b d4
pub fn i_09a39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A39E;
    cpu.ir = 0x4A04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09A39E: bmi.w $9a3ac
pub fn i_09a39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3A2;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x9A3AC; return 10; }
    8
}
/// $09A3A2: move.b d4, $73(a6)
pub fn i_09a3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3A6;
    cpu.ir = 0x1D44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A3A6: lea.l $9a3b0(pc), a1
pub fn i_09a3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A3B0;
    16
}
/// $09A3AA: move.l a1, (a6)
pub fn i_09a3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A3AC: bra.w $9a0e0
pub fn i_09a3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A0E0;
    10
}
// ===== Gun_Aim_09a3b0  ($09A3B0, 346 B, props/items_score_crates_09a0bc.s)
/// $09A3B0: lea.l $9a3b6(pc), a1
pub fn i_09a3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A3B6;
    16
}
/// $09A3B4: move.l a1, (a6)
pub fn i_09a3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A3B6: cmpi.b #$2, $73(a6)
pub fn i_09a3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3BC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $09A3BC: bne.w $9a43a
pub fn i_09a3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3C0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A43A; return 10; }
    8
}
/// $09A3C0: jsr $5e136.l
pub fn i_09a3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A3C6);
    cpu.pc = 0x5E136;
    16
}
/// $09A3C6: bcs.w $9a436
pub fn i_09a3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3CA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A436; return 10; }
    8
}
/// $09A3CA: cmpi.b #$3, $7a(a6)
pub fn i_09a3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3D0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $09A3D0: bne.w $9a3e0
pub fn i_09a3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A3E0; return 10; }
    8
}
/// $09A3D4: lsl.w #$8, d0
pub fn i_09a3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3D6;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $09A3D6: sub.w $70(a6), d0
pub fn i_09a3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3DA;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A3DA: asr.w #$3, d0
pub fn i_09a3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3DC;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A3DC: add.w d0, $70(a6)
pub fn i_09a3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3E0;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A3E0: tst.w $78(a6)
pub fn i_09a3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3E4;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $09A3E4: beq.w $9a3f0
pub fn i_09a3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A3F0; return 10; }
    8
}
/// $09A3E8: subq.w #$1, $78(a6)
pub fn i_09a3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3EC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A3EC: bra.w $9a436
pub fn i_09a3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A436;
    10
}
/// $09A3F0: addi.w #$80, d0
pub fn i_09a3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3F4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A3F4: andi.w #$ff00, d0
pub fn i_09a3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3F8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A3F8: bne.w $9a436
pub fn i_09a3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A3FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A436; return 10; }
    8
}
/// $09A3FC: move.b $7a(a6), d1
pub fn i_09a3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A400;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A400: bne.w $9a41e
pub fn i_09a400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A404;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A41E; return 10; }
    8
}
/// $09A404: move.b #$3, $7a(a6)
pub fn i_09a404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A40A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A40A: lea.l $2f6648.l, a0
pub fn i_09a40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A410;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6648;
    20
}
/// $09A410: jsr $799de.l
pub fn i_09a410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A416;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A416);
    cpu.pc = 0x799DE;
    16
}
/// $09A416: move.w d0, $78(a6)
pub fn i_09a416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A41A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A41A: bra.w $9a436
pub fn i_09a41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A41E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A436;
    10
}
/// $09A41E: subq.w #$1, $78(a6)
pub fn i_09a41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A422;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A422: bne.w $9a436
pub fn i_09a422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A426;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A436; return 10; }
    8
}
/// $09A426: move.w #$2, $78(a6)
pub fn i_09a426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A42C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A42C: subq.b #$1, d1
pub fn i_09a42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A42E;
    cpu.ir = 0x5301;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A42E: move.b d1, $7a(a6)
pub fn i_09a42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A432;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A432: bsr.w $9a20e
pub fn i_09a432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A436;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A436);
    cpu.pc = 0x9A20E;
    18
}
/// $09A436: bra.w $9a4dc
pub fn i_09a436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A43A;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A4DC;
    10
}
/// $09A43A: cmpi.b #$0, $73(a6)
pub fn i_09a43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A440;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $09A440: bne.w $9a44e
pub fn i_09a440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A444;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A44E; return 10; }
    8
}
/// $09A444: lea.l $10e200.l, a2
pub fn i_09a444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A44A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $09A44A: bra.w $9a454
pub fn i_09a44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A44E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A454;
    10
}
/// $09A44E: lea.l $10e206.l, a2
pub fn i_09a44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A454;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $09A454: move.b $2(a2), d0
pub fn i_09a454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A458;
    cpu.ir = 0x102A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A458: andi.w #$c, d0
pub fn i_09a458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A45C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A45C: beq.w $9a4a2
pub fn i_09a45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A460;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A4A2; return 10; }
    8
}
/// $09A460: andi.b #$4, d0
pub fn i_09a460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A464;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09A464: bne.w $9a470
pub fn i_09a464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A468;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A470; return 10; }
    8
}
/// $09A468: move.w #$8000, d0
pub fn i_09a468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A46C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A46C: bra.w $9a472
pub fn i_09a46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A470;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A472;
    10
}
/// $09A470: clr.w d0
pub fn i_09a470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A472;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A472: sub.w $70(a6), d0
pub fn i_09a472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A476;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A476: asr.w #$2, d0
pub fn i_09a476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A478;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09A478: ext.l d0
pub fn i_09a478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A47A;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09A47A: move.l d0, d1
pub fn i_09a47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A47C;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09A47C: swap d1
pub fn i_09a47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A47E;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09A47E: eor.w d1, d0
pub fn i_09a47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A480;
    cpu.ir = 0xB340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A480: sub.w d1, d0
pub fn i_09a480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A482;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A482: move.w $80(a6), d2
pub fn i_09a482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A486;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A486: addi.w #$100, d2
pub fn i_09a486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A48A;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A48A: cmp.w d0, d2
pub fn i_09a48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A48C;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $09A48C: bcs.w $9a492
pub fn i_09a48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A490;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A492; return 10; }
    8
}
/// $09A490: move.w d0, d2
pub fn i_09a490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A492;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A492: move.w d2, $80(a6)
pub fn i_09a492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A496;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A496: eor.w d1, d2
pub fn i_09a496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A498;
    cpu.ir = 0xB342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A498: sub.w d1, d2
pub fn i_09a498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A49A;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A49A: add.w d2, $70(a6)
pub fn i_09a49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A49E;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A49E: bra.w $9a4a6
pub fn i_09a49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A4A6;
    10
}
/// $09A4A2: clr.w $80(a6)
pub fn i_09a4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4A6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A4A6: btst.b #$4, $3(a2)
pub fn i_09a4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4AC;
    cpu.ir = 0x82A;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[2].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09A4AC: beq.w $9a4bc
pub fn i_09a4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A4BC; return 10; }
    8
}
/// $09A4B0: move.b #$3, $7a(a6)
pub fn i_09a4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4B6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A4B6: move.b #$1, $78(a6)
pub fn i_09a4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4BC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A4BC: move.b $7a(a6), d0
pub fn i_09a4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4C0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A4C0: beq.w $9a4dc
pub fn i_09a4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A4DC; return 10; }
    8
}
/// $09A4C4: subq.b #$1, $78(a6)
pub fn i_09a4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4C8;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09A4C8: bne.w $9a4dc
pub fn i_09a4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4CC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A4DC; return 10; }
    8
}
/// $09A4CC: move.b #$2, $78(a6)
pub fn i_09a4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4D2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A4D2: subq.b #$1, d0
pub fn i_09a4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4D4;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A4D4: move.b d0, $7a(a6)
pub fn i_09a4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4D8;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A4D8: bsr.w $9a246
pub fn i_09a4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4DC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A4DC);
    cpu.pc = 0x9A246;
    18
}
/// $09A4DC: bsr.w $9a1d8
pub fn i_09a4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4E0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A4E0);
    cpu.pc = 0x9A1D8;
    18
}
/// $09A4E0: bsr.w $9a0fc
pub fn i_09a4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4E4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A4E4);
    cpu.pc = 0x9A0FC;
    18
}
/// $09A4E4: jsr $28d70.l
pub fn i_09a4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A4EA);
    cpu.pc = 0x28D70;
    16
}
/// $09A4EA: move.w $22(a6), d0
pub fn i_09a4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4EE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A4EE: move.w $24(a6), d1
pub fn i_09a4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4F2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A4F2: move.b $72(a6), d2
pub fn i_09a4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4F6;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A4F6: bsr.w $9a186
pub fn i_09a4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4FA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x9A4FA);
    cpu.pc = 0x9A186;
    18
}
/// $09A4FA: tst.b d4
pub fn i_09a4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A4FC;
    cpu.ir = 0x4A04;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09A4FC: bpl.w $9a506
pub fn i_09a4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A500;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x9A506; return 10; }
    8
}
/// $09A500: lea.l $9a374(pc), a1
pub fn i_09a500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A504;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A374;
    16
}
/// $09A504: move.l a1, (a6)
pub fn i_09a504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A506;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A506: bra.w $9a0e0
pub fn i_09a506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A50A;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A0E0;
    10
}
// ===== Gun_Destroyed_09a50a  ($09A50A, 22 B, props/items_score_crates_09a0bc.s)
/// $09A50A: move.w #$1021, d0
pub fn i_09a50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A50E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1021;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A50E: jsr $2352.l
pub fn i_09a50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A514);
    cpu.pc = 0x2352;
    16
}
/// $09A514: jsr $13600.l
pub fn i_09a514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A51A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A51A);
    cpu.pc = 0x13600;
    16
}
/// $09A51A: jmp $77fd6.l
pub fn i_09a51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A520;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77FD6;
    8
}
// ===== Gun_Shell_09a520  ($09A520, 184 B, props/items_score_crates_09a0bc.s)
/// $09A520: move.w #$d1, d1
pub fn i_09a520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A524;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A524: jsr $236e.l
pub fn i_09a524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A52A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A52A);
    cpu.pc = 0x236E;
    16
}
/// $09A52A: bset.b #$4, $6b(a6)
pub fn i_09a52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A530;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09A530: lea.l $2f65f4.l, a0
pub fn i_09a530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A536;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F65F4;
    20
}
/// $09A536: move.l a0, $4c(a6)
pub fn i_09a536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A53A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09A53A: jsr $283ca.l
pub fn i_09a53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A540;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A540);
    cpu.pc = 0x283CA;
    16
}
/// $09A540: move.w #$d000, $38(a6)
pub fn i_09a540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A546;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A546: lea.l $2f66ca.l, a0
pub fn i_09a546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A54C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F66CA;
    20
}
/// $09A54C: jsr $799de.l
pub fn i_09a54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A552);
    cpu.pc = 0x799DE;
    16
}
/// $09A552: move.w d0, d1
pub fn i_09a552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A554;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A554: move.b $98(a6), d0
pub fn i_09a554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A558;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A558: addq.b #$8, d0
pub fn i_09a558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A55A;
    cpu.ir = 0x5000;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09A55A: andi.w #$f0, d0
pub fn i_09a55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A55E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A55E: jsr $13c0e.l
pub fn i_09a55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A564);
    cpu.pc = 0x13C0E;
    16
}
/// $09A564: move.w d1, $28(a6)
pub fn i_09a564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A568;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A568: move.w d2, $2a(a6)
pub fn i_09a568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A56C;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A56C: move.b $98(a6), d0
pub fn i_09a56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A570;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A570: andi.w #$ff, d0
pub fn i_09a570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A574;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A574: asr.w #$4, d0
pub fn i_09a574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A576;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $09A576: movea.l #$29d452, a0
pub fn i_09a576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A57C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x29D452;
    cpu.a[0] = v1;
    4
}
/// $09A57C: lsl.w #$2, d0
pub fn i_09a57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A57E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09A57E: movea.l (a0, d0.w), a0
pub fn i_09a57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A582;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $09A582: cmpa.l #$ffffffff, a0
pub fn i_09a582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A588;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $09A588: beq.w $9a592
pub fn i_09a588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A58C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A592; return 10; }
    8
}
/// $09A58C: jsr $28cd4.l
pub fn i_09a58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A592;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A592);
    cpu.pc = 0x28CD4;
    16
}
/// $09A592: lea.l $9a598(pc), a1
pub fn i_09a592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A596;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A598;
    16
}
/// $09A596: move.l a1, (a6)
pub fn i_09a596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A598;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A598: bset.b #$6, $13(a6)
pub fn i_09a598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A59E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09A59E: jsr $27cee.l
pub fn i_09a59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A5A4);
    cpu.pc = 0x27CEE;
    16
}
/// $09A5A4: jsr $28d70.l
pub fn i_09a5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A5AA);
    cpu.pc = 0x28D70;
    16
}
/// $09A5AA: jsr $283d8.l
pub fn i_09a5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A5B0);
    cpu.pc = 0x283D8;
    16
}
/// $09A5B0: btst.b #$1, $13(a6)
pub fn i_09a5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5B6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09A5B6: beq.w $9a5c0
pub fn i_09a5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A5C0; return 10; }
    8
}
/// $09A5BA: jmp $518.l
pub fn i_09a5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5C0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A5C0: movea.l #$ffffffff, a0
pub fn i_09a5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5C6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09A5C6: jsr $5dd56.l
pub fn i_09a5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A5CC);
    cpu.pc = 0x5DD56;
    16
}
/// $09A5CC: bcc.w $9a5d6
pub fn i_09a5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A5D6; return 10; }
    8
}
/// $09A5D0: jmp $518.l
pub fn i_09a5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5D6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A5D6: rts
pub fn i_09a5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_09a5d8  ($09A5D8, 16 B, props/items_score_crates_09a0bc.s)
/// $09A5D8: movea.l $8(a6), a1
pub fn i_09a5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5DC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09A5DC: move.b $10(a6), d0
pub fn i_09a5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5E0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A5E0: cmp.b $10(a1), d0
pub fn i_09a5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5E4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $09A5E4: bcs.w $9a5ee
pub fn i_09a5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5E8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A5EE; return 10; }
    8
}
// ===== ClearXN_09a5e8  ($09A5E8, 6 B, thunks/ccr_helpers.c)
/// $09A5E8: andi.b #$ee, ccr
pub fn i_09a5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5EC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09A5EC: rts
pub fn i_09a5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_09a5ee  ($09A5EE, 6 B, thunks/ccr_helpers.c)
/// $09A5EE: ori.b #$11, ccr
pub fn i_09a5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5F2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09A5F2: rts
pub fn i_09a5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A5F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_GroundTail_09a75a  ($09A75A, 80 B, props/items_score_crates_09a0bc.s)
/// $09A75A: cmpi.b #$1, $90(a6)
pub fn i_09a75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A760;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $09A760: bne.w $9a76a
pub fn i_09a760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A764;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A76A; return 10; }
    8
}
/// $09A764: addq.b #$1, $10e48b.l
pub fn i_09a764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A76A;
    cpu.ir = 0x5239;
    let v1 = bus.read8(0x10E48B) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10E48B, r2 as u8);
    16
}
/// $09A76A: tst.b $9f(a6)
pub fn i_09a76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A76E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09A76E: beq.w $9a78e
pub fn i_09a76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A772;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A78E; return 10; }
    8
}
/// $09A772: movea.l #$ffffffff, a0
pub fn i_09a772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A778;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09A778: lea.l $9a5f4(pc), a0
pub fn i_09a778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A77C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A5F4;
    16
}
/// $09A77C: jsr $5dd56.l
pub fn i_09a77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A782);
    cpu.pc = 0x5DD56;
    16
}
/// $09A782: bcc.w $9a78c
pub fn i_09a782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A786;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A78C; return 10; }
    8
}
/// $09A786: jmp $518.l
pub fn i_09a786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A78C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A78C: rts
pub fn i_09a78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A78E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09A78E: movea.l #$ffffffff, a0
pub fn i_09a78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A794;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09A794: lea.l $9a5f4(pc), a0
pub fn i_09a794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A798;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A5F4;
    16
}
/// $09A798: jsr $5dd5c.l
pub fn i_09a798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A79E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A79E);
    cpu.pc = 0x5DD5C;
    16
}
/// $09A79E: bcc.w $9a7a8
pub fn i_09a79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A7A8; return 10; }
    8
}
/// $09A7A2: jmp $518.l
pub fn i_09a7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7A8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09A7A8: rts
pub fn i_09a7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeMoveX_09A7AA  ($09A7AA, 34 B, entity/probe_move_x_09a7aa.s)
/// $09A7AA: jsr $9a7cc(pc)
pub fn i_09a7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A7AE);
    cpu.pc = 0x9A7CC;
    16
}
/// $09A7AE: bcs.w $9a7ca
pub fn i_09a7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7B2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A7CA; return 10; }
    8
}
/// $09A7B2: move.w #$10, d0
pub fn i_09a7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A7B6: btst.b #$0, $3a(a6)
pub fn i_09a7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7BC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09A7BC: beq.w $9a7c2
pub fn i_09a7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A7C2; return 10; }
    8
}
/// $09A7C0: neg.w d0
pub fn i_09a7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7C2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A7C2: add.w d0, $22(a0)
pub fn i_09a7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7C6;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A7C6: andi.b #$ee, ccr
pub fn i_09a7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7CA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09A7CA: rts
pub fn i_09a7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_SpawnFromParent_09a7cc  ($09A7CC, 106 B, props/items_score_crates_09a0bc.s)
/// $09A7CC: tst.b d0
pub fn i_09a7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7CE;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09A7CE: beq.w $9a830
pub fn i_09a7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7D2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A830; return 10; }
    8
}
/// $09A7D2: move.w d1, -(a7)
pub fn i_09a7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7D4;
    cpu.ir = 0x3F01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $09A7D4: andi.w #$3f, d0
pub fn i_09a7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7D8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A7D8: cmpi.w #$1e, d0
pub fn i_09a7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7DC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1E, Size::Word, false);
    8
}
/// $09A7DC: bcs.w $9a7e4
pub fn i_09a7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7E0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9A7E4; return 10; }
    8
}
/// $09A7E0: move.w #$a, d0
pub fn i_09a7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7E4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A7E4: add.w d0, d0
pub fn i_09a7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7E6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A7E6: add.w d0, d0
pub fn i_09a7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7E8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09A7E8: lea.l $9a6de(pc), a0
pub fn i_09a7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7EC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A6DE;
    16
}
/// $09A7EC: movea.l (a0, d0.w), a1
pub fn i_09a7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7F0;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $09A7F0: move.l a6, -(a7)
pub fn i_09a7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7F2;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $09A7F2: lea.l $100800.l, a6
pub fn i_09a7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7F8;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $09A7F8: jsr $4ae.l
pub fn i_09a7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A7FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A7FE);
    cpu.pc = 0x4AE;
    16
}
/// $09A7FE: movea.l (a7)+, a6
pub fn i_09a7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A800;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $09A800: move.w $24(a6), $24(a0)
pub fn i_09a800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A806;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09A806: move.w #$10, d0
pub fn i_09a806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A80A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A80A: btst.b #$0, $3a(a6)
pub fn i_09a80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A810;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09A810: bne.w $9a816
pub fn i_09a810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A814;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A816; return 10; }
    8
}
/// $09A814: neg.w d0
pub fn i_09a814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A816;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09A816: add.w $22(a6), d0
pub fn i_09a816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A81A;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09A81A: move.w d0, $22(a0)
pub fn i_09a81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A81E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09A81E: move.w (a7)+, d1
pub fn i_09a81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A820;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $09A820: move.b (a6, d1.w), $98(a0)
pub fn i_09a820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A826;
    cpu.ir = 0x1176;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    22
}
/// $09A826: move.b $1(a6, d1.w), $99(a0)
pub fn i_09a826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A82C;
    cpu.ir = 0x1176;
    let ad2 = cpu.a[6].wrapping_add(0x1).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad4, v3 as u8);
    22
}
/// $09A82C: ori.b #$11, ccr
pub fn i_09a82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A830;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09A830: eori.b #$11, ccr
pub fn i_09a830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A834;
    cpu.ir = 0xA3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) ^ 0x11);
    20
}
/// $09A834: rts
pub fn i_09a834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A836;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_PickupTestA_09a836  ($09A836, 12 B, props/items_score_crates_09a0bc.s)
/// $09A836: lea.l $9a6ce(pc), a1
pub fn i_09a836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A83A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A6CE;
    16
}
/// $09A83A: tst.b $45(a6)
pub fn i_09a83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A83E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09A83E: beq.w $9a848
pub fn i_09a83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A842;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A848; return 10; }
    8
}
// ===== ClearC_09a842  ($09A842, 6 B, thunks/ccr_helpers.c)
/// $09A842: andi.b #$fe, ccr
pub fn i_09a842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A846;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $09A846: rts
pub fn i_09a846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A848;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_09a848  ($09A848, 6 B, thunks/jmp_abs_thunks.c)
/// $09A848: jmp $3ee48.l
pub fn i_09a848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A84E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x3EE48;
    8
}
// ===== Item_PickupTestB_09a84e  ($09A84E, 6 B, props/items_score_crates_09a0bc.s)
/// $09A84E: lea.l $9a6d6(pc), a1
pub fn i_09a84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A852;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A6D6;
    16
}
/// $09A852: bra.b $9a83a
pub fn i_09a852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A854;
    cpu.ir = 0x60E6;
    cpu.pc = 0x9A83A;
    10
}
// ===== Item_GiveAmmoKind_09a854  ($09A854, 34 B, props/items_score_crates_09a0bc.s)
/// $09A854: move.b $98(a6), d2
pub fn i_09a854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A858;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09A858: jsr $32b58.l
pub fn i_09a858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A85E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A85E);
    cpu.pc = 0x32B58;
    16
}
/// $09A85E: cmpi.b #$2, $70(a1)
pub fn i_09a85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A864;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $09A864: bne.w $9a876
pub fn i_09a864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A868;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A876; return 10; }
    8
}
/// $09A868: move.l a1, -(a7)
pub fn i_09a868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A86A;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $09A86A: move.w #$10fd, d0
pub fn i_09a86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A86E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10FD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A86E: jsr $2352.l
pub fn i_09a86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A874);
    cpu.pc = 0x2352;
    16
}
/// $09A874: movea.l (a7)+, a1
pub fn i_09a874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A876;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
// ===== JsrAbsThunk_09a876  ($09A876, 8 B, thunks/jsr_abs_thunks.c)
/// $09A876: jsr $32cba.l
pub fn i_09a876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A87C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A87C);
    cpu.pc = 0x32CBA;
    16
}
/// $09A87C: rts
pub fn i_09a87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A87E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_SetMapFromSeq_09a87e  ($09A87E, 56 B, props/items_score_crates_09a0bc.s)
/// $09A87E: movea.l $76(a6), a0
pub fn i_09a87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A882;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A882: move.w $7a(a6), d0
pub fn i_09a882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A886;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09A886: move.b (a0, d0.w), d0
pub fn i_09a886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A88A;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09A88A: cmpi.b #$ff, d0
pub fn i_09a88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A88E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $09A88E: bne.w $9a89c
pub fn i_09a88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A892;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9A89C; return 10; }
    8
}
/// $09A892: clr.w $7a(a6)
pub fn i_09a892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A896;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09A896: movea.l $76(a6), a0
pub fn i_09a896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A89A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09A89A: move.b (a0), d0
pub fn i_09a89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A89C;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $09A89C: andi.w #$3, d0
pub fn i_09a89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8A0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09A8A0: movea.l #$2f7a28, a0
pub fn i_09a8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8A6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F7A28;
    cpu.a[0] = v1;
    4
}
/// $09A8A6: lsl.w #$2, d0
pub fn i_09a8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8A8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09A8A8: movea.l (a0, d0.w), a0
pub fn i_09a8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8AC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $09A8AC: cmpa.l #$ffffffff, a0
pub fn i_09a8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8B2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $09A8B2: beq.w $9a8bc
pub fn i_09a8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9A8BC; return 10; }
    8
}
// ===== JsrAbsThunk_09a8b6  ($09A8B6, 8 B, thunks/jsr_abs_thunks.c)
/// $09A8B6: jsr $28cd4.l
pub fn i_09a8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A8BC);
    cpu.pc = 0x28CD4;
    16
}
/// $09A8BC: rts
pub fn i_09a8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_Tmpl_PowCounted_09a8be  ($09A8BE, 18 B, props/items_score_crates_09a0bc.s)
/// $09A8BE: move.b #$1, $90(a6)
pub fn i_09a8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A8C4: cmpi.b #$2, $10e48a.l
pub fn i_09a8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8CC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E48A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $09A8CC: blt.w $9a8de
pub fn i_09a8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8D0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x9A8DE; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_09a8d0  ($09A8D0, 14 B, kernel/jsr5b6_jmp518.c)
/// $09A8D0: jsr $5b6.l
pub fn i_09a8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A8D6);
    cpu.pc = 0x5B6;
    16
}
/// $09A8D6: jmp $518.l
pub fn i_09a8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8DC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Item_FlyAway_09a8de  ($09A8DE, 58 B, props/items_score_crates_09a0bc.s)
/// $09A8DE: move.w #$180, d1
pub fn i_09a8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A8E2: jsr $236e.l
pub fn i_09a8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A8E8);
    cpu.pc = 0x236E;
    16
}
/// $09A8E8: lea.l $2f678a.l, a0
pub fn i_09a8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F678A;
    20
}
/// $09A8EE: jsr $28cd4.l
pub fn i_09a8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A8F4);
    cpu.pc = 0x28CD4;
    16
}
/// $09A8F4: lea.l $9a8fa(pc), a1
pub fn i_09a8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A8FA;
    16
}
/// $09A8F8: move.l a1, (a6)
pub fn i_09a8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A8FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A8FA: jsr $2783a.l
pub fn i_09a8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A900);
    cpu.pc = 0x2783A;
    16
}
/// $09A900: jsr $28d70.l
pub fn i_09a900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A906;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A906);
    cpu.pc = 0x28D70;
    16
}
/// $09A906: bcc.w $9a914
pub fn i_09a906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A90A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A914; return 10; }
    8
}
/// $09A90A: jsr $13600.l
pub fn i_09a90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A910;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A910);
    cpu.pc = 0x13600;
    16
}
/// $09A910: move.l $5c(a6), (a6)
pub fn i_09a910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A914;
    cpu.ir = 0x2CAE;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    bus.write32(cpu.a[6], v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $09A914: bra.w $9a75a
pub fn i_09a914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A918;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl284_Ammo4_09a918  ($09A918, 94 B, props/items_score_crates_09a0bc.s)
/// $09A918: move.w #$17d, d1
pub fn i_09a918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A91C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A91C: jsr $236e.l
pub fn i_09a91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A922;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A922);
    cpu.pc = 0x236E;
    16
}
/// $09A922: move.b #$8, $45(a6)
pub fn i_09a922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A928;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A928: move.w #$8000, d0
pub fn i_09a928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A92C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A92C: jsr $28134.l
pub fn i_09a92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A932);
    cpu.pc = 0x28134;
    16
}
/// $09A932: andi.w #$ffe3, $38(a6)
pub fn i_09a932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A938;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A938: ori.w #$0, $38(a6)
pub fn i_09a938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A93E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A93E: lea.l $2f67dc.l, a0
pub fn i_09a93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A944;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F67DC;
    20
}
/// $09A944: jsr $28cd4.l
pub fn i_09a944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A94A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A94A);
    cpu.pc = 0x28CD4;
    16
}
/// $09A94A: lea.l $9a950(pc), a1
pub fn i_09a94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A94E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A950;
    16
}
/// $09A94E: move.l a1, (a6)
pub fn i_09a94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A950;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A950: jsr $27c8c.l
pub fn i_09a950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A956;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A956);
    cpu.pc = 0x27C8C;
    16
}
/// $09A956: jsr $28d70.l
pub fn i_09a956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A95C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A95C);
    cpu.pc = 0x28D70;
    16
}
/// $09A95C: jsr $9a836(pc)
pub fn i_09a95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A960;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A960);
    cpu.pc = 0x9A836;
    16
}
/// $09A960: bcc.w $9a972
pub fn i_09a960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A964;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A972; return 10; }
    8
}
/// $09A964: move.b #$4, d1
pub fn i_09a964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A968;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09A968: jsr $9a854(pc)
pub fn i_09a968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A96C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A96C);
    cpu.pc = 0x9A854;
    16
}
/// $09A96C: lea.l $9ae26(pc), a1
pub fn i_09a96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A970;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE26;
    16
}
/// $09A970: move.l a1, (a6)
pub fn i_09a970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A972;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A972: bra.w $9a75a
pub fn i_09a972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A976;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl285_Ammo3_09a976  ($09A976, 94 B, props/items_score_crates_09a0bc.s)
/// $09A976: move.w #$17d, d1
pub fn i_09a976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A97A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A97A: jsr $236e.l
pub fn i_09a97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A980;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A980);
    cpu.pc = 0x236E;
    16
}
/// $09A980: move.b #$8, $45(a6)
pub fn i_09a980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A986;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A986: move.w #$8000, d0
pub fn i_09a986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A98A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A98A: jsr $28134.l
pub fn i_09a98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A990);
    cpu.pc = 0x28134;
    16
}
/// $09A990: andi.w #$ffe3, $38(a6)
pub fn i_09a990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A996;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A996: ori.w #$0, $38(a6)
pub fn i_09a996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A99C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A99C: lea.l $2f67e8.l, a0
pub fn i_09a99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F67E8;
    20
}
/// $09A9A2: jsr $28cd4.l
pub fn i_09a9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A9A8);
    cpu.pc = 0x28CD4;
    16
}
/// $09A9A8: lea.l $9a9ae(pc), a1
pub fn i_09a9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9A9AE;
    16
}
/// $09A9AC: move.l a1, (a6)
pub fn i_09a9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A9AE: jsr $27c8c.l
pub fn i_09a9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A9B4);
    cpu.pc = 0x27C8C;
    16
}
/// $09A9B4: jsr $28d70.l
pub fn i_09a9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A9BA);
    cpu.pc = 0x28D70;
    16
}
/// $09A9BA: jsr $9a836(pc)
pub fn i_09a9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A9BE);
    cpu.pc = 0x9A836;
    16
}
/// $09A9BE: bcc.w $9a9d0
pub fn i_09a9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9A9D0; return 10; }
    8
}
/// $09A9C2: move.b #$3, d1
pub fn i_09a9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9C6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09A9C6: jsr $9a854(pc)
pub fn i_09a9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9A9CA);
    cpu.pc = 0x9A854;
    16
}
/// $09A9CA: lea.l $9ae26(pc), a1
pub fn i_09a9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE26;
    16
}
/// $09A9CE: move.l a1, (a6)
pub fn i_09a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09A9D0: bra.w $9a75a
pub fn i_09a9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl286_Ammo1_09a9d4  ($09A9D4, 94 B, props/items_score_crates_09a0bc.s)
/// $09A9D4: move.w #$17d, d1
pub fn i_09a9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9D8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A9D8: jsr $236e.l
pub fn i_09a9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A9DE);
    cpu.pc = 0x236E;
    16
}
/// $09A9DE: move.b #$8, $45(a6)
pub fn i_09a9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09A9E4: move.w #$8000, d0
pub fn i_09a9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9E8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09A9E8: jsr $28134.l
pub fn i_09a9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9A9EE);
    cpu.pc = 0x28134;
    16
}
/// $09A9EE: andi.w #$ffe3, $38(a6)
pub fn i_09a9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9F4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A9F4: ori.w #$0, $38(a6)
pub fn i_09a9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9A9FA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09A9FA: lea.l $2f67f4.l, a0
pub fn i_09a9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA00;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F67F4;
    20
}
/// $09AA00: jsr $28cd4.l
pub fn i_09aa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA06);
    cpu.pc = 0x28CD4;
    16
}
/// $09AA06: lea.l $9aa0c(pc), a1
pub fn i_09aa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AA0C;
    16
}
/// $09AA0A: move.l a1, (a6)
pub fn i_09aa0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AA0C: jsr $27c8c.l
pub fn i_09aa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA12);
    cpu.pc = 0x27C8C;
    16
}
/// $09AA12: jsr $28d70.l
pub fn i_09aa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA18);
    cpu.pc = 0x28D70;
    16
}
/// $09AA18: jsr $9a836(pc)
pub fn i_09aa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AA1C);
    cpu.pc = 0x9A836;
    16
}
/// $09AA1C: bcc.w $9aa2e
pub fn i_09aa1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AA2E; return 10; }
    8
}
/// $09AA20: move.b #$1, d1
pub fn i_09aa20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA24;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09AA24: jsr $9a854(pc)
pub fn i_09aa24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AA28);
    cpu.pc = 0x9A854;
    16
}
/// $09AA28: lea.l $9ae26(pc), a1
pub fn i_09aa28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE26;
    16
}
/// $09AA2C: move.l a1, (a6)
pub fn i_09aa2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AA2E: bra.w $9a75a
pub fn i_09aa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA32;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl287_Ammo2_09aa32  ($09AA32, 94 B, props/items_score_crates_09a0bc.s)
/// $09AA32: move.w #$17d, d1
pub fn i_09aa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA36;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AA36: jsr $236e.l
pub fn i_09aa36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA3C);
    cpu.pc = 0x236E;
    16
}
/// $09AA3C: move.b #$8, $45(a6)
pub fn i_09aa3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA42;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AA42: move.w #$8000, d0
pub fn i_09aa42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA46;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AA46: jsr $28134.l
pub fn i_09aa46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA4C);
    cpu.pc = 0x28134;
    16
}
/// $09AA4C: andi.w #$ffe3, $38(a6)
pub fn i_09aa4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA52;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AA52: ori.w #$0, $38(a6)
pub fn i_09aa52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA58;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AA58: lea.l $2f6800.l, a0
pub fn i_09aa58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6800;
    20
}
/// $09AA5E: jsr $28cd4.l
pub fn i_09aa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA64);
    cpu.pc = 0x28CD4;
    16
}
/// $09AA64: lea.l $9aa6a(pc), a1
pub fn i_09aa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AA6A;
    16
}
/// $09AA68: move.l a1, (a6)
pub fn i_09aa68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AA6A: jsr $27c8c.l
pub fn i_09aa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA70);
    cpu.pc = 0x27C8C;
    16
}
/// $09AA70: jsr $28d70.l
pub fn i_09aa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA76);
    cpu.pc = 0x28D70;
    16
}
/// $09AA76: jsr $9a836(pc)
pub fn i_09aa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA7A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AA7A);
    cpu.pc = 0x9A836;
    16
}
/// $09AA7A: bcc.w $9aa8c
pub fn i_09aa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA7E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AA8C; return 10; }
    8
}
/// $09AA7E: move.b #$2, d1
pub fn i_09aa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA82;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09AA82: jsr $9a854(pc)
pub fn i_09aa82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA86;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AA86);
    cpu.pc = 0x9A854;
    16
}
/// $09AA86: lea.l $9ae26(pc), a1
pub fn i_09aa86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE26;
    16
}
/// $09AA8A: move.l a1, (a6)
pub fn i_09aa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AA8C: bra.w $9a75a
pub fn i_09aa8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA90;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl288_AmmoSeq_09aa90  ($09AA90, 82 B, props/items_score_crates_09a0bc.s)
/// $09AA90: move.w #$17d, d1
pub fn i_09aa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA94;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AA94: jsr $236e.l
pub fn i_09aa94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AA9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AA9A);
    cpu.pc = 0x236E;
    16
}
/// $09AA9A: lea.l $2f79ce.l, a0
pub fn i_09aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F79CE;
    20
}
/// $09AAA0: move.b $98(a6), d0
pub fn i_09aaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAA4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09AAA4: cmpi.b #$b, d0
pub fn i_09aaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAA8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $09AAA8: bcs.w $9aab0
pub fn i_09aaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAAC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9AAB0; return 10; }
    8
}
/// $09AAAC: move.b #$a, d0
pub fn i_09aaac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAB0;
    cpu.ir = 0x103C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09AAB0: andi.w #$f, d0
pub fn i_09aab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAB4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09AAB4: lsl.w #$2, d0
pub fn i_09aab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAB6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09AAB6: move.l (a0, d0.w), d0
pub fn i_09aab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AABA;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $09AABA: move.l d0, $76(a6)
pub fn i_09aaba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AABE;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09AABE: clr.w $7a(a6)
pub fn i_09aabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAC2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09AAC2: jsr $9a87e(pc)
pub fn i_09aac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AAC6);
    cpu.pc = 0x9A87E;
    16
}
/// $09AAC6: move.b #$8, $45(a6)
pub fn i_09aac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AACC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AACC: move.w #$8000, d0
pub fn i_09aacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAD0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AAD0: jsr $28134.l
pub fn i_09aad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AAD6);
    cpu.pc = 0x28134;
    16
}
/// $09AAD6: andi.w #$ffe3, $38(a6)
pub fn i_09aad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AADC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AADC: ori.w #$0, $38(a6)
pub fn i_09aadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAE2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== Item_AmmoSeq_Show_09aae2  ($09AAE2, 114 B, props/items_score_crates_09a0bc.s)
/// $09AAE2: move.b #$ff, $32(a6)
pub fn i_09aae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAE8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AAE8: move.w #$50, $70(a6)
pub fn i_09aae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09AAEE: lea.l $9aaf4(pc), a1
pub fn i_09aaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AAF4;
    16
}
/// $09AAF2: move.l a1, (a6)
pub fn i_09aaf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AAF4: subq.w #$1, $70(a6)
pub fn i_09aaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAF8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09AAF8: cmpi.w #$0, $70(a6)
pub fn i_09aaf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AAFE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09AAFE: bgt.w $9ab08
pub fn i_09aafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB02;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9AB08; return 10; }
    8
}
/// $09AB02: lea.l $9ab5c(pc), a1
pub fn i_09ab02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AB5C;
    16
}
/// $09AB06: move.l a1, (a6)
pub fn i_09ab06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB08;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AB08: jsr $27c8c.l
pub fn i_09ab08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AB0E);
    cpu.pc = 0x27C8C;
    16
}
/// $09AB0E: jsr $28d70.l
pub fn i_09ab0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AB14);
    cpu.pc = 0x28D70;
    16
}
/// $09AB14: jsr $9a836(pc)
pub fn i_09ab14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AB18);
    cpu.pc = 0x9A836;
    16
}
/// $09AB18: bcc.w $9ab50
pub fn i_09ab18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AB50; return 10; }
    8
}
/// $09AB1C: movea.l $76(a6), a2
pub fn i_09ab1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB20;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $09AB20: move.w $7a(a6), d2
pub fn i_09ab20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB24;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09AB24: move.b (a2, d2.w), d2
pub fn i_09ab24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB28;
    cpu.ir = 0x1432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09AB28: andi.w #$3, d2
pub fn i_09ab28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB2C;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09AB2C: lea.l $2f7a38.l, a2
pub fn i_09ab2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB32;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F7A38;
    20
}
/// $09AB32: move.b (a2, d2.w), d1
pub fn i_09ab32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB36;
    cpu.ir = 0x1232;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09AB36: lea.l $2f7a3c.l, a2
pub fn i_09ab36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB3C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2F7A3C;
    20
}
/// $09AB3C: move.b (a2, d2.w), d2
pub fn i_09ab3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB40;
    cpu.ir = 0x1432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09AB40: jsr $9a858(pc)
pub fn i_09ab40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AB44);
    cpu.pc = 0x9A858;
    16
}
/// $09AB44: move.b #$ff, $32(a6)
pub fn i_09ab44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB4A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AB4A: lea.l $9ae26(pc), a1
pub fn i_09ab4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE26;
    16
}
/// $09AB4E: move.l a1, (a6)
pub fn i_09ab4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB50;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AB50: bra.w $9a75a
pub fn i_09ab50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB54;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_FadeRamp_09ab54  ($09AB54, 8 B, props/items_score_crates_09a0bc.s)
/// $09AB54: dc.w $f0e0
pub fn i_09ab54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB56;
    cpu.ir = 0xF0E0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Item_AmmoSeq_FadeOut_09ab5c  ($09AB5C, 50 B, props/items_score_crates_09a0bc.s)
/// $09AB5C: clr.w $70(a6)
pub fn i_09ab5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB60;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09AB60: lea.l $9ab66(pc), a1
pub fn i_09ab60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AB66;
    16
}
/// $09AB64: move.l a1, (a6)
pub fn i_09ab64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AB66: lea.l $9ab54(pc), a0
pub fn i_09ab66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB6A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9AB54;
    16
}
/// $09AB6A: move.w $70(a6), d0
pub fn i_09ab6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB6E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09AB6E: move.b (a0, d0.w), d0
pub fn i_09ab6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB72;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09AB72: move.b d0, $32(a6)
pub fn i_09ab72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB76;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AB76: addq.w #$1, $70(a6)
pub fn i_09ab76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB7A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09AB7A: cmpi.w #$7, $70(a6)
pub fn i_09ab7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB80;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7, Size::Word, false);
    16
}
/// $09AB80: bne.w $9ab8a
pub fn i_09ab80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9AB8A; return 10; }
    8
}
/// $09AB84: lea.l $9ab8e(pc), a1
pub fn i_09ab84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AB8E;
    16
}
/// $09AB88: move.l a1, (a6)
pub fn i_09ab88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AB8A: bra.w $9ab08
pub fn i_09ab8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AB08;
    10
}
// ===== Item_AmmoSeq_FadeIn_09ab8e  ($09AB8E, 52 B, props/items_score_crates_09a0bc.s)
/// $09AB8E: addq.w #$1, $7a(a6)
pub fn i_09ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB92;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09AB92: jsr $9a87e(pc)
pub fn i_09ab92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AB96);
    cpu.pc = 0x9A87E;
    16
}
/// $09AB96: lea.l $9ab9c(pc), a1
pub fn i_09ab96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AB9C;
    16
}
/// $09AB9A: move.l a1, (a6)
pub fn i_09ab9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AB9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AB9C: lea.l $9ab54(pc), a0
pub fn i_09ab9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABA0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9AB54;
    16
}
/// $09ABA0: move.w $70(a6), d0
pub fn i_09aba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABA4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09ABA4: move.b (a0, d0.w), d0
pub fn i_09aba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABA8;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09ABA8: move.b d0, $32(a6)
pub fn i_09aba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABAC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09ABAC: subq.w #$1, $70(a6)
pub fn i_09abac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABB0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09ABB0: tst.w $70(a6)
pub fn i_09abb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABB4;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $09ABB4: bne.w $9abbe
pub fn i_09abb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABB8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9ABBE; return 10; }
    8
}
/// $09ABB8: lea.l $9aae2(pc), a1
pub fn i_09abb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AAE2;
    16
}
/// $09ABBC: move.l a1, (a6)
pub fn i_09abbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09ABBE: bra.w $9ab08
pub fn i_09abbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AB08;
    10
}
// ===== Item_Tmpl289_Weapon_09abc2  ($09ABC2, 144 B, props/items_score_crates_09a0bc.s)
/// $09ABC2: move.w #$17f, d1
pub fn i_09abc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABC6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09ABC6: jsr $236e.l
pub fn i_09abc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ABCC);
    cpu.pc = 0x236E;
    16
}
/// $09ABCC: move.w #$1a7, d1
pub fn i_09abcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABD0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09ABD0: jsr $236e.l
pub fn i_09abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ABD6);
    cpu.pc = 0x236E;
    16
}
/// $09ABD6: move.b #$8, $45(a6)
pub fn i_09abd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABDC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09ABDC: move.w #$8000, d0
pub fn i_09abdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABE0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09ABE0: jsr $28134.l
pub fn i_09abe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ABE6);
    cpu.pc = 0x28134;
    16
}
/// $09ABE6: andi.w #$ffe3, $38(a6)
pub fn i_09abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABEC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09ABEC: ori.w #$0, $38(a6)
pub fn i_09abec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABF2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09ABF2: lea.l $2f680c.l, a0
pub fn i_09abf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABF8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F680C;
    20
}
/// $09ABF8: jsr $28cd4.l
pub fn i_09abf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ABFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ABFE);
    cpu.pc = 0x28CD4;
    16
}
/// $09ABFE: tst.b $98(a6)
pub fn i_09abfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC02;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09AC02: bne.w $9ac0c
pub fn i_09ac02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9AC0C; return 10; }
    8
}
/// $09AC06: move.b #$a, $98(a6)
pub fn i_09ac06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC0C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AC0C: tst.b $99(a6)
pub fn i_09ac0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC10;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09AC10: bne.w $9ac1a
pub fn i_09ac10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC14;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9AC1A; return 10; }
    8
}
/// $09AC14: move.b #$64, $99(a6)
pub fn i_09ac14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC1A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AC1A: lea.l $9ac20(pc), a1
pub fn i_09ac1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AC20;
    16
}
/// $09AC1E: move.l a1, (a6)
pub fn i_09ac1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AC20: jsr $27c8c.l
pub fn i_09ac20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC26);
    cpu.pc = 0x27C8C;
    16
}
/// $09AC26: jsr $28d70.l
pub fn i_09ac26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC2C);
    cpu.pc = 0x28D70;
    16
}
/// $09AC2C: jsr $9a836(pc)
pub fn i_09ac2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AC30);
    cpu.pc = 0x9A836;
    16
}
/// $09AC30: bcc.w $9ac4e
pub fn i_09ac30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC34;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AC4E; return 10; }
    8
}
/// $09AC34: move.b d0, $68(a6)
pub fn i_09ac34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC38;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AC38: jsr $32c12.l
pub fn i_09ac38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC3E);
    cpu.pc = 0x32C12;
    16
}
/// $09AC3E: lea.l $9ae36(pc), a1
pub fn i_09ac3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE36;
    16
}
/// $09AC42: move.l a1, (a6)
pub fn i_09ac42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AC44: move.w #$1124, d0
pub fn i_09ac44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC48;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AC48: jsr $2352.l
pub fn i_09ac48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC4E);
    cpu.pc = 0x2352;
    16
}
/// $09AC4E: bra.w $9a75a
pub fn i_09ac4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC52;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl290_WeaponSwap_09ac52  ($09AC52, 258 B, props/items_score_crates_09a0bc.s)
/// $09AC52: move.w #$17e, d1
pub fn i_09ac52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC56;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AC56: jsr $236e.l
pub fn i_09ac56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC5C);
    cpu.pc = 0x236E;
    16
}
/// $09AC5C: move.w #$1a8, d1
pub fn i_09ac5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC60;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AC60: jsr $236e.l
pub fn i_09ac60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC66);
    cpu.pc = 0x236E;
    16
}
/// $09AC66: move.b #$8, $45(a6)
pub fn i_09ac66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC6C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AC6C: move.w #$8000, d0
pub fn i_09ac6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AC70: jsr $28134.l
pub fn i_09ac70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC76);
    cpu.pc = 0x28134;
    16
}
/// $09AC76: andi.w #$ffe3, $38(a6)
pub fn i_09ac76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC7C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AC7C: ori.w #$0, $38(a6)
pub fn i_09ac7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC82;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AC82: lea.l $2f685c.l, a0
pub fn i_09ac82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC88;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F685C;
    20
}
/// $09AC88: jsr $28cd4.l
pub fn i_09ac88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AC8E);
    cpu.pc = 0x28CD4;
    16
}
/// $09AC8E: tst.b $98(a6)
pub fn i_09ac8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC92;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09AC92: bne.w $9ac9c
pub fn i_09ac92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9AC9C; return 10; }
    8
}
/// $09AC96: move.b #$a, $98(a6)
pub fn i_09ac96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AC9C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AC9C: tst.b $99(a6)
pub fn i_09ac9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACA0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09ACA0: bne.w $9acaa
pub fn i_09aca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9ACAA; return 10; }
    8
}
/// $09ACA4: move.b #$a, $99(a6)
pub fn i_09aca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09ACAA: lea.l $9acb0(pc), a1
pub fn i_09acaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9ACB0;
    16
}
/// $09ACAE: move.l a1, (a6)
pub fn i_09acae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09ACB0: jsr $27c8c.l
pub fn i_09acb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ACB6);
    cpu.pc = 0x27C8C;
    16
}
/// $09ACB6: jsr $28d70.l
pub fn i_09acb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ACBC);
    cpu.pc = 0x28D70;
    16
}
/// $09ACBC: jsr $9a836(pc)
pub fn i_09acbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9ACC0);
    cpu.pc = 0x9A836;
    16
}
/// $09ACC0: bcc.w $9ad50
pub fn i_09acc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AD50; return 10; }
    8
}
/// $09ACC4: cmpi.b #$0, d0
pub fn i_09acc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACC8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $09ACC8: bne.w $9acd6
pub fn i_09acc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACCC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9ACD6; return 10; }
    8
}
/// $09ACCC: lea.l $100440.l, a1
pub fn i_09accc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACD2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $09ACD2: bra.w $9acdc
pub fn i_09acd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACD6;
    cpu.ir = 0x6000;
    cpu.pc = 0x9ACDC;
    10
}
/// $09ACD6: lea.l $1004e0.l, a1
pub fn i_09acd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACDC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $09ACDC: cmpi.b #$2, $70(a1)
pub fn i_09acdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACE2;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $09ACE2: bne.w $9ad16
pub fn i_09ace2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACE6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9AD16; return 10; }
    8
}
/// $09ACE6: move.b d0, $68(a6)
pub fn i_09ace6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACEA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09ACEA: move.b $99(a6), $98(a6)
pub fn i_09acea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACF0;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09ACF0: jsr $2a28e.l
pub fn i_09acf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ACF6);
    cpu.pc = 0x2A28E;
    16
}
/// $09ACF6: lea.l $9b48a(pc), a1
pub fn i_09acf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ACFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B48A;
    16
}
/// $09ACFA: jsr $4ae.l
pub fn i_09acfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD00);
    cpu.pc = 0x4AE;
    16
}
/// $09AD00: jsr $5dd02.l
pub fn i_09ad00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD06);
    cpu.pc = 0x5DD02;
    16
}
/// $09AD06: move.b $68(a6), $68(a0)
pub fn i_09ad06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD0C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09AD0C: move.b #$1, $98(a0)
pub fn i_09ad0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD12;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AD12: bra.w $9ad40
pub fn i_09ad12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD16;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AD40;
    10
}
/// $09AD16: move.b d0, $68(a6)
pub fn i_09ad16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD1A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AD1A: move.b $98(a6), $98(a6)
pub fn i_09ad1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD20;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09AD20: jsr $32c7e.l
pub fn i_09ad20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD26);
    cpu.pc = 0x32C7E;
    16
}
/// $09AD26: lea.l $9b48a(pc), a1
pub fn i_09ad26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B48A;
    16
}
/// $09AD2A: jsr $4ae.l
pub fn i_09ad2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD30);
    cpu.pc = 0x4AE;
    16
}
/// $09AD30: jsr $5dd02.l
pub fn i_09ad30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD36);
    cpu.pc = 0x5DD02;
    16
}
/// $09AD36: move.b $68(a6), $68(a0)
pub fn i_09ad36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD3C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09AD3C: clr.b $98(a0)
pub fn i_09ad3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD40;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AD40: lea.l $9ae2e(pc), a1
pub fn i_09ad40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE2E;
    16
}
/// $09AD44: move.l a1, (a6)
pub fn i_09ad44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AD46: move.w #$1124, d0
pub fn i_09ad46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AD4A: jsr $2352.l
pub fn i_09ad4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD50);
    cpu.pc = 0x2352;
    16
}
/// $09AD50: bra.w $9a75a
pub fn i_09ad50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD54;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl291_Bombs_09ad54  ($09AD54, 210 B, props/items_score_crates_09a0bc.s)
/// $09AD54: move.w #$180, d1
pub fn i_09ad54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD58;
    cpu.ir = 0x323C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AD58: jsr $236e.l
pub fn i_09ad58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD5E);
    cpu.pc = 0x236E;
    16
}
/// $09AD5E: move.w #$1a9, d1
pub fn i_09ad5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD62;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AD62: jsr $236e.l
pub fn i_09ad62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD68);
    cpu.pc = 0x236E;
    16
}
/// $09AD68: move.b #$8, $45(a6)
pub fn i_09ad68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD6E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AD6E: move.w #$8000, d0
pub fn i_09ad6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD72;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AD72: jsr $28134.l
pub fn i_09ad72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD78);
    cpu.pc = 0x28134;
    16
}
/// $09AD78: andi.w #$ffe3, $38(a6)
pub fn i_09ad78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD7E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AD7E: ori.w #$0, $38(a6)
pub fn i_09ad7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD84;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AD84: lea.l $2f68ac.l, a0
pub fn i_09ad84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD8A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F68AC;
    20
}
/// $09AD8A: jsr $28cd4.l
pub fn i_09ad8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD90);
    cpu.pc = 0x28CD4;
    16
}
/// $09AD90: lea.l $9ad96(pc), a1
pub fn i_09ad90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AD96;
    16
}
/// $09AD94: move.l a1, (a6)
pub fn i_09ad94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AD96: jsr $27c8c.l
pub fn i_09ad96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AD9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AD9C);
    cpu.pc = 0x27C8C;
    16
}
/// $09AD9C: jsr $28d70.l
pub fn i_09ad9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ADA2);
    cpu.pc = 0x28D70;
    16
}
/// $09ADA2: jsr $9a836(pc)
pub fn i_09ada2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADA6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9ADA6);
    cpu.pc = 0x9A836;
    16
}
/// $09ADA6: bcc.w $9ae22
pub fn i_09ada6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AE22; return 10; }
    8
}
/// $09ADAA: cmpi.b #$0, d0
pub fn i_09adaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADAE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $09ADAE: bne.w $9adbc
pub fn i_09adae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADB2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9ADBC; return 10; }
    8
}
/// $09ADB2: lea.l $100440.l, a1
pub fn i_09adb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADB8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $09ADB8: bra.w $9adc2
pub fn i_09adb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x9ADC2;
    10
}
/// $09ADBC: lea.l $1004e0.l, a1
pub fn i_09adbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADC2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $09ADC2: move.b d0, $68(a6)
pub fn i_09adc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADC6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09ADC6: cmpi.b #$2, $70(a1)
pub fn i_09adc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADCC;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $09ADCC: bne.w $9ae0e
pub fn i_09adcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADD0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9AE0E; return 10; }
    8
}
/// $09ADD0: move.b d0, $68(a6)
pub fn i_09add0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADD4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09ADD4: move.b $98(a6), d0
pub fn i_09add4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADD8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09ADD8: andi.w #$ff, d0
pub fn i_09add8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADDC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09ADDC: jsr $2a2ba.l
pub fn i_09addc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ADE2);
    cpu.pc = 0x2A2BA;
    16
}
/// $09ADE2: bcc.w $9adee
pub fn i_09ade2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9ADEE; return 10; }
    8
}
/// $09ADE6: jsr $9ba34(pc)
pub fn i_09ade6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9ADEA);
    cpu.pc = 0x9BA34;
    16
}
/// $09ADEA: bra.w $9ae0a
pub fn i_09adea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AE0A;
    10
}
/// $09ADEE: lea.l $9b48a(pc), a1
pub fn i_09adee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B48A;
    16
}
/// $09ADF2: jsr $4ae.l
pub fn i_09adf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ADF8);
    cpu.pc = 0x4AE;
    16
}
/// $09ADF8: jsr $5dd02.l
pub fn i_09adf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9ADFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9ADFE);
    cpu.pc = 0x5DD02;
    16
}
/// $09ADFE: move.b $68(a6), $68(a0)
pub fn i_09adfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE04;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09AE04: move.b #$3, $98(a0)
pub fn i_09ae04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE0A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AE0A: bra.w $9ae12
pub fn i_09ae0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE0E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AE12;
    10
}
/// $09AE0E: jsr $9ba34(pc)
pub fn i_09ae0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9AE12);
    cpu.pc = 0x9BA34;
    16
}
/// $09AE12: lea.l $9ae3e(pc), a1
pub fn i_09ae12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE3E;
    16
}
/// $09AE16: move.l a1, (a6)
pub fn i_09ae16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AE18: move.w #$1124, d0
pub fn i_09ae18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE1C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1124;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE1C: jsr $2352.l
pub fn i_09ae1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AE22);
    cpu.pc = 0x2352;
    16
}
/// $09AE22: bra.w $9a75a
pub fn i_09ae22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE26;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Taken_SndA_09ae26  ($09AE26, 8 B, props/items_score_crates_09a0bc.s)
/// $09AE26: move.w #$1a6, d1
pub fn i_09ae26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE2A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE2A: bra.w $9ae42
pub fn i_09ae2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AE42;
    10
}
// ===== Item_Taken_SndB_09ae2e  ($09AE2E, 8 B, props/items_score_crates_09a0bc.s)
/// $09AE2E: move.w #$1a7, d1
pub fn i_09ae2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE32;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE32: bra.w $9ae42
pub fn i_09ae32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE36;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AE42;
    10
}
// ===== Item_Taken_SndC_09ae36  ($09AE36, 8 B, props/items_score_crates_09a0bc.s)
/// $09AE36: move.w #$1a8, d1
pub fn i_09ae36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE3A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE3A: bra.w $9ae42
pub fn i_09ae3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9AE42;
    10
}
// ===== Item_Taken_SndD_09ae3e  ($09AE3E, 80 B, props/items_score_crates_09a0bc.s)
/// $09AE3E: move.w #$1a9, d1
pub fn i_09ae3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE42;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE42: jsr $236e.l
pub fn i_09ae42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AE48);
    cpu.pc = 0x236E;
    16
}
/// $09AE48: move.w #$2, $70(a6)
pub fn i_09ae48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09AE4E: move.w #$c000, d0
pub fn i_09ae4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE52;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE52: jsr $28134.l
pub fn i_09ae52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AE58);
    cpu.pc = 0x28134;
    16
}
/// $09AE58: andi.w #$ffe3, $38(a6)
pub fn i_09ae58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE5E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AE5E: ori.w #$0, $38(a6)
pub fn i_09ae5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE64;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09AE64: lea.l $9ae6a(pc), a1
pub fn i_09ae64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE6A;
    16
}
/// $09AE68: move.l a1, (a6)
pub fn i_09ae68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AE6A: jsr $2783a.l
pub fn i_09ae6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AE70);
    cpu.pc = 0x2783A;
    16
}
/// $09AE70: jsr $28d70.l
pub fn i_09ae70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AE76);
    cpu.pc = 0x28D70;
    16
}
/// $09AE76: subq.w #$1, $70(a6)
pub fn i_09ae76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE7A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09AE7A: cmpi.w #$0, $70(a6)
pub fn i_09ae7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE80;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09AE80: bgt.w $9ae8a
pub fn i_09ae80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE84;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9AE8A; return 10; }
    8
}
/// $09AE84: lea.l $9ae8e(pc), a1
pub fn i_09ae84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AE8E;
    16
}
/// $09AE88: move.l a1, (a6)
pub fn i_09ae88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AE8A: bra.w $9a76a
pub fn i_09ae8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A76A;
    10
}
// ===== Item_Taken_Rise_09ae8e  ($09AE8E, 54 B, props/items_score_crates_09a0bc.s)
/// $09AE8E: move.w #$180, d1
pub fn i_09ae8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE92;
    cpu.ir = 0x323C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09AE92: jsr $236e.l
pub fn i_09ae92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AE98);
    cpu.pc = 0x236E;
    16
}
/// $09AE98: lea.l $2f674c.l, a0
pub fn i_09ae98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AE9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F674C;
    20
}
/// $09AE9E: jsr $28cd4.l
pub fn i_09ae9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AEA4);
    cpu.pc = 0x28CD4;
    16
}
/// $09AEA4: lea.l $9aeaa(pc), a1
pub fn i_09aea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9AEAA;
    16
}
/// $09AEA8: move.l a1, (a6)
pub fn i_09aea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09AEAA: jsr $2783a.l
pub fn i_09aeaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AEB0);
    cpu.pc = 0x2783A;
    16
}
/// $09AEB0: jsr $28d70.l
pub fn i_09aeb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AEB6);
    cpu.pc = 0x28D70;
    16
}
/// $09AEB6: bcc.w $9aec0
pub fn i_09aeb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9AEC0; return 10; }
    8
}
/// $09AEBA: jmp $518.l
pub fn i_09aeba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEC0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09AEC0: bra.w $9a76a
pub fn i_09aec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEC4;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A76A;
    10
}
// ===== Item_Tmpl294_Food_09aec4  ($09AEC4, 484 B, props/items_score_crates_09a0bc.s)
/// $09AEC4: clr.b $9c(a6)
pub fn i_09aec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AEC8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AEC8: lea.l $2f6b78.l, a0
pub fn i_09aec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AECE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6B78;
    20
}
/// $09AECE: jsr $28cd4.l
pub fn i_09aece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AED4);
    cpu.pc = 0x28CD4;
    16
}
/// $09AED4: bra.w $9b05c
pub fn i_09aed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AED8;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09AF2A: clr.b $9c(a6)
pub fn i_09af2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF2E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AF2E: lea.l $2f6cca.l, a0
pub fn i_09af2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6CCA;
    20
}
/// $09AF34: jsr $28cd4.l
pub fn i_09af34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF3A);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF3A: bra.w $9b064
pub fn i_09af3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09AF3E: clr.b $9c(a6)
pub fn i_09af3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF42;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AF42: lea.l $2f6e4c.l, a0
pub fn i_09af42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6E4C;
    20
}
/// $09AF48: jsr $28cd4.l
pub fn i_09af48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF4E);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF4E: bra.w $9b064
pub fn i_09af4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF52;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09AF52: move.b #$1, $9c(a6)
pub fn i_09af52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF58;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09AF58: lea.l $2f7002.l, a0
pub fn i_09af58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7002;
    20
}
/// $09AF5E: jsr $28cd4.l
pub fn i_09af5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF64);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF64: bra.w $9b05c
pub fn i_09af64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF68;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09AF68: clr.b $9c(a6)
pub fn i_09af68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF6C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AF6C: lea.l $2f706c.l, a0
pub fn i_09af6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF72;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F706C;
    20
}
/// $09AF72: jsr $28cd4.l
pub fn i_09af72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AF78);
    cpu.pc = 0x28CD4;
    16
}
/// $09AF78: bra.w $9b05c
pub fn i_09af78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AF7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09AFE0: clr.b $9c(a6)
pub fn i_09afe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFE4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AFE4: lea.l $2f745e.l, a0
pub fn i_09afe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F745E;
    20
}
/// $09AFEA: jsr $28cd4.l
pub fn i_09afea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9AFF0);
    cpu.pc = 0x28CD4;
    16
}
/// $09AFF0: bra.w $9b064
pub fn i_09aff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B064;
    10
}
/// $09AFF4: clr.b $9c(a6)
pub fn i_09aff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFF8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09AFF8: lea.l $2f74a0.l, a0
pub fn i_09aff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9AFFE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F74A0;
    20
}
/// $09AFFE: jsr $28cd4.l
pub fn i_09affe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B004;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B004);
    cpu.pc = 0x28CD4;
    16
}
/// $09B004: bra.w $9b05c
pub fn i_09b004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B008;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B05C;
    10
}
/// $09B05C: move.w #$1ac, d1
pub fn i_09b05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B060;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B060: bra.w $9b068
pub fn i_09b060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B064;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B068;
    10
}
/// $09B064: move.w #$1c6, d1
pub fn i_09b064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B068;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B068: cmpi.b #$ff, $9a(a6)
pub fn i_09b068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B06E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B06E: beq.w $9b078
pub fn i_09b06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B072;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B078; return 10; }
    8
}
/// $09B072: eori.b #$1, $3a(a6)
pub fn i_09b072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B078;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09B078: jsr $236e.l
pub fn i_09b078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B07E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B07E);
    cpu.pc = 0x236E;
    16
}
/// $09B07E: move.b #$8, $45(a6)
pub fn i_09b07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B084;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B084: move.w #$8000, d0
pub fn i_09b084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B088;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B088: jsr $28134.l
pub fn i_09b088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B08E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B08E);
    cpu.pc = 0x28134;
    16
}
/// $09B08E: andi.w #$ffe3, $38(a6)
pub fn i_09b08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B094;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B094: ori.w #$0, $38(a6)
pub fn i_09b094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B09A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B09A: jsr $9bb3c(pc)
pub fn i_09b09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B09E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B09E);
    cpu.pc = 0x9BB3C;
    16
}
/// $09B09E: move.w d0, $70(a6)
pub fn i_09b09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0A2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B0A2: lea.l $9b0a8(pc), a1
pub fn i_09b0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0A8;
    16
}
/// $09B0A6: move.l a1, (a6)
pub fn i_09b0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Item_Food_Wait_09b0a8  ($09B0A8, 48 B, props/items_score_crates_09a0bc.s)
/// $09B0A8: jsr $9b8c0(pc)
pub fn i_09b0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B0AC);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B0AC: jsr $28d70.l
pub fn i_09b0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B0B2);
    cpu.pc = 0x28D70;
    16
}
/// $09B0B2: subq.w #$1, $70(a6)
pub fn i_09b0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0B6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B0B6: cmpi.w #$0, $70(a6)
pub fn i_09b0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0BC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B0BC: bgt.w $9b0c6
pub fn i_09b0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0C0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B0C6; return 10; }
    8
}
/// $09B0C0: lea.l $9b0d8(pc), a1
pub fn i_09b0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0D8;
    16
}
/// $09B0C4: move.l a1, (a6)
pub fn i_09b0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0991BC, i_0991bc),
    (0x0991C2, i_0991c2),
    (0x0991C8, i_0991c8),
    (0x0991CE, i_0991ce),
    (0x0991D2, i_0991d2),
    (0x0991D6, i_0991d6),
    (0x0991D8, i_0991d8),
    (0x0991DA, i_0991da),
    (0x0991DE, i_0991de),
    (0x0991E4, i_0991e4),
    (0x0991EA, i_0991ea),
    (0x0991F0, i_0991f0),
    (0x0991F6, i_0991f6),
    (0x0991FA, i_0991fa),
    (0x099200, i_099200),
    (0x099206, i_099206),
    (0x09920C, i_09920c),
    (0x099212, i_099212),
    (0x099216, i_099216),
    (0x099218, i_099218),
    (0x09921A, i_09921a),
    (0x099220, i_099220),
    (0x099226, i_099226),
    (0x09922C, i_09922c),
    (0x099232, i_099232),
    (0x099236, i_099236),
    (0x099238, i_099238),
    (0x09923E, i_09923e),
    (0x099244, i_099244),
    (0x099248, i_099248),
    (0x09924C, i_09924c),
    (0x099250, i_099250),
    (0x099254, i_099254),
    (0x09925A, i_09925a),
    (0x09925E, i_09925e),
    (0x099262, i_099262),
    (0x099264, i_099264),
    (0x09926A, i_09926a),
    (0x099270, i_099270),
    (0x099276, i_099276),
    (0x09927C, i_09927c),
    (0x099280, i_099280),
    (0x099284, i_099284),
    (0x099286, i_099286),
    (0x099288, i_099288),
    (0x09928E, i_09928e),
    (0x099294, i_099294),
    (0x09929A, i_09929a),
    (0x09929E, i_09929e),
    (0x0992A0, i_0992a0),
    (0x0992A6, i_0992a6),
    (0x0992AC, i_0992ac),
    (0x0992B0, i_0992b0),
    (0x0992B4, i_0992b4),
    (0x0992B8, i_0992b8),
    (0x0992BA, i_0992ba),
    (0x0992C0, i_0992c0),
    (0x0992C6, i_0992c6),
    (0x0992CC, i_0992cc),
    (0x0992D0, i_0992d0),
    (0x0992D4, i_0992d4),
    (0x0992D6, i_0992d6),
    (0x0992D8, i_0992d8),
    (0x0992DC, i_0992dc),
    (0x0992E2, i_0992e2),
    (0x0992FE, i_0992fe),
    (0x099304, i_099304),
    (0x09930A, i_09930a),
    (0x099310, i_099310),
    (0x099314, i_099314),
    (0x09931A, i_09931a),
    (0x099320, i_099320),
    (0x099326, i_099326),
    (0x09932C, i_09932c),
    (0x099332, i_099332),
    (0x099338, i_099338),
    (0x09933E, i_09933e),
    (0x099342, i_099342),
    (0x099344, i_099344),
    (0x099346, i_099346),
    (0x09934A, i_09934a),
    (0x099350, i_099350),
    (0x09936C, i_09936c),
    (0x099372, i_099372),
    (0x099378, i_099378),
    (0x09937C, i_09937c),
    (0x099382, i_099382),
    (0x099388, i_099388),
    (0x09938E, i_09938e),
    (0x099394, i_099394),
    (0x09939A, i_09939a),
    (0x09939E, i_09939e),
    (0x0993A0, i_0993a0),
    (0x0993A2, i_0993a2),
    (0x0993A8, i_0993a8),
    (0x0993AE, i_0993ae),
    (0x0993B4, i_0993b4),
    (0x0993BA, i_0993ba),
    (0x0993BE, i_0993be),
    (0x0993C0, i_0993c0),
    (0x0993C6, i_0993c6),
    (0x0993CC, i_0993cc),
    (0x0993D0, i_0993d0),
    (0x0993D4, i_0993d4),
    (0x0993D6, i_0993d6),
    (0x0993DC, i_0993dc),
    (0x0993E2, i_0993e2),
    (0x0993E8, i_0993e8),
    (0x0993EE, i_0993ee),
    (0x0993F2, i_0993f2),
    (0x0993F6, i_0993f6),
    (0x0993F8, i_0993f8),
    (0x0993FA, i_0993fa),
    (0x099400, i_099400),
    (0x099406, i_099406),
    (0x09940A, i_09940a),
    (0x09940C, i_09940c),
    (0x099412, i_099412),
    (0x099418, i_099418),
    (0x09941C, i_09941c),
    (0x099420, i_099420),
    (0x099422, i_099422),
    (0x099428, i_099428),
    (0x09942E, i_09942e),
    (0x099434, i_099434),
    (0x09943A, i_09943a),
    (0x09943E, i_09943e),
    (0x099442, i_099442),
    (0x099444, i_099444),
    (0x099446, i_099446),
    (0x09944A, i_09944a),
    (0x099450, i_099450),
    (0x09946C, i_09946c),
    (0x099472, i_099472),
    (0x099476, i_099476),
    (0x09947C, i_09947c),
    (0x099482, i_099482),
    (0x099488, i_099488),
    (0x09948E, i_09948e),
    (0x099494, i_099494),
    (0x09949A, i_09949a),
    (0x09949E, i_09949e),
    (0x0994A0, i_0994a0),
    (0x0994A2, i_0994a2),
    (0x0994A6, i_0994a6),
    (0x0994AC, i_0994ac),
    (0x0994C8, i_0994c8),
    (0x0994CE, i_0994ce),
    (0x0994D2, i_0994d2),
    (0x0994D8, i_0994d8),
    (0x0994DE, i_0994de),
    (0x0994E4, i_0994e4),
    (0x0994EA, i_0994ea),
    (0x0994F0, i_0994f0),
    (0x0994F6, i_0994f6),
    (0x0994FA, i_0994fa),
    (0x0994FC, i_0994fc),
    (0x0994FE, i_0994fe),
    (0x099502, i_099502),
    (0x099508, i_099508),
    (0x09950E, i_09950e),
    (0x099514, i_099514),
    (0x09951A, i_09951a),
    (0x09951E, i_09951e),
    (0x099524, i_099524),
    (0x09952A, i_09952a),
    (0x099530, i_099530),
    (0x099536, i_099536),
    (0x09953A, i_09953a),
    (0x09953C, i_09953c),
    (0x09953E, i_09953e),
    (0x099544, i_099544),
    (0x09954A, i_09954a),
    (0x099550, i_099550),
    (0x099554, i_099554),
    (0x099556, i_099556),
    (0x09955C, i_09955c),
    (0x099562, i_099562),
    (0x099568, i_099568),
    (0x09956C, i_09956c),
    (0x099570, i_099570),
    (0x099572, i_099572),
    (0x099578, i_099578),
    (0x09957E, i_09957e),
    (0x099584, i_099584),
    (0x099588, i_099588),
    (0x09958C, i_09958c),
    (0x09958E, i_09958e),
    (0x099590, i_099590),
    (0x099596, i_099596),
    (0x09959C, i_09959c),
    (0x0995A0, i_0995a0),
    (0x0995A2, i_0995a2),
    (0x0995A8, i_0995a8),
    (0x0995AE, i_0995ae),
    (0x0995B4, i_0995b4),
    (0x0995BA, i_0995ba),
    (0x0995C0, i_0995c0),
    (0x0995C4, i_0995c4),
    (0x0995C8, i_0995c8),
    (0x0995CA, i_0995ca),
    (0x0995CC, i_0995cc),
    (0x0995D0, i_0995d0),
    (0x0995D6, i_0995d6),
    (0x0995DC, i_0995dc),
    (0x0995E2, i_0995e2),
    (0x0995E8, i_0995e8),
    (0x0995EC, i_0995ec),
    (0x0995F0, i_0995f0),
    (0x0995F6, i_0995f6),
    (0x0995FC, i_0995fc),
    (0x099602, i_099602),
    (0x099608, i_099608),
    (0x09960C, i_09960c),
    (0x09960E, i_09960e),
    (0x099610, i_099610),
    (0x099616, i_099616),
    (0x09961C, i_09961c),
    (0x099622, i_099622),
    (0x099626, i_099626),
    (0x099628, i_099628),
    (0x09962E, i_09962e),
    (0x099634, i_099634),
    (0x099638, i_099638),
    (0x09963C, i_09963c),
    (0x099642, i_099642),
    (0x099646, i_099646),
    (0x09964C, i_09964c),
    (0x099650, i_099650),
    (0x099652, i_099652),
    (0x099658, i_099658),
    (0x09965E, i_09965e),
    (0x099664, i_099664),
    (0x099668, i_099668),
    (0x09966C, i_09966c),
    (0x09966E, i_09966e),
    (0x099670, i_099670),
    (0x099676, i_099676),
    (0x09967C, i_09967c),
    (0x099680, i_099680),
    (0x099682, i_099682),
    (0x099688, i_099688),
    (0x09968E, i_09968e),
    (0x099692, i_099692),
    (0x099696, i_099696),
    (0x099698, i_099698),
    (0x09969E, i_09969e),
    (0x0996A4, i_0996a4),
    (0x0996AA, i_0996aa),
    (0x0996AE, i_0996ae),
    (0x0996B2, i_0996b2),
    (0x0996B4, i_0996b4),
    (0x0996B6, i_0996b6),
    (0x0996BA, i_0996ba),
    (0x0996C0, i_0996c0),
    (0x0996C6, i_0996c6),
    (0x0996CC, i_0996cc),
    (0x099734, i_099734),
    (0x09973A, i_09973a),
    (0x099740, i_099740),
    (0x099746, i_099746),
    (0x09974A, i_09974a),
    (0x099750, i_099750),
    (0x099756, i_099756),
    (0x09975C, i_09975c),
    (0x099762, i_099762),
    (0x099766, i_099766),
    (0x099768, i_099768),
    (0x09976A, i_09976a),
    (0x099770, i_099770),
    (0x099776, i_099776),
    (0x09977C, i_09977c),
    (0x099782, i_099782),
    (0x099788, i_099788),
    (0x09978C, i_09978c),
    (0x099790, i_099790),
    (0x099792, i_099792),
    (0x099794, i_099794),
    (0x09979C, i_09979c),
    (0x0997A0, i_0997a0),
    (0x0997A4, i_0997a4),
    (0x0997A8, i_0997a8),
    (0x0997AC, i_0997ac),
    (0x0997B0, i_0997b0),
    (0x0997B2, i_0997b2),
    (0x0997B6, i_0997b6),
    (0x0997B8, i_0997b8),
    (0x0997BE, i_0997be),
    (0x0997C4, i_0997c4),
    (0x0997CA, i_0997ca),
    (0x0997CC, i_0997cc),
    (0x0997D6, i_0997d6),
    (0x0997E0, i_0997e0),
    (0x0997E2, i_0997e2),
    (0x0997E8, i_0997e8),
    (0x0997EC, i_0997ec),
    (0x0997EE, i_0997ee),
    (0x0997F2, i_0997f2),
    (0x0997F6, i_0997f6),
    (0x0997FA, i_0997fa),
    (0x0997FE, i_0997fe),
    (0x099802, i_099802),
    (0x099806, i_099806),
    (0x099808, i_099808),
    (0x09980A, i_09980a),
    (0x09980E, i_09980e),
    (0x099812, i_099812),
    (0x099818, i_099818),
    (0x09981C, i_09981c),
    (0x09981E, i_09981e),
    (0x099822, i_099822),
    (0x099826, i_099826),
    (0x09982C, i_09982c),
    (0x09982E, i_09982e),
    (0x099832, i_099832),
    (0x099836, i_099836),
    (0x09983A, i_09983a),
    (0x099840, i_099840),
    (0x099842, i_099842),
    (0x099844, i_099844),
    (0x099848, i_099848),
    (0x09984C, i_09984c),
    (0x09984E, i_09984e),
    (0x099852, i_099852),
    (0x099854, i_099854),
    (0x099856, i_099856),
    (0x09985A, i_09985a),
    (0x099860, i_099860),
    (0x099866, i_099866),
    (0x09986A, i_09986a),
    (0x09986C, i_09986c),
    (0x09986E, i_09986e),
    (0x099872, i_099872),
    (0x099876, i_099876),
    (0x09987A, i_09987a),
    (0x09987E, i_09987e),
    (0x099882, i_099882),
    (0x099886, i_099886),
    (0x099888, i_099888),
    (0x09988A, i_09988a),
    (0x09988C, i_09988c),
    (0x099890, i_099890),
    (0x099892, i_099892),
    (0x099896, i_099896),
    (0x09989C, i_09989c),
    (0x0998A0, i_0998a0),
    (0x0998A4, i_0998a4),
    (0x0998A8, i_0998a8),
    (0x0998AC, i_0998ac),
    (0x0998B0, i_0998b0),
    (0x0998B4, i_0998b4),
    (0x0998B8, i_0998b8),
    (0x0998BC, i_0998bc),
    (0x0998C0, i_0998c0),
    (0x0998C2, i_0998c2),
    (0x0998C8, i_0998c8),
    (0x0998CA, i_0998ca),
    (0x0998CE, i_0998ce),
    (0x0998D4, i_0998d4),
    (0x0998DA, i_0998da),
    (0x0998E0, i_0998e0),
    (0x0998E4, i_0998e4),
    (0x0998E8, i_0998e8),
    (0x0998EC, i_0998ec),
    (0x0998EE, i_0998ee),
    (0x0998F2, i_0998f2),
    (0x0998F6, i_0998f6),
    (0x0998FA, i_0998fa),
    (0x0998FE, i_0998fe),
    (0x099902, i_099902),
    (0x099904, i_099904),
    (0x099908, i_099908),
    (0x09990C, i_09990c),
    (0x099910, i_099910),
    (0x099914, i_099914),
    (0x099916, i_099916),
    (0x09991A, i_09991a),
    (0x09991E, i_09991e),
    (0x099924, i_099924),
    (0x099928, i_099928),
    (0x09992C, i_09992c),
    (0x099930, i_099930),
    (0x099934, i_099934),
    (0x099938, i_099938),
    (0x09993A, i_09993a),
    (0x09993C, i_09993c),
    (0x099940, i_099940),
    (0x099946, i_099946),
    (0x09994C, i_09994c),
    (0x099952, i_099952),
    (0x099956, i_099956),
    (0x09995A, i_09995a),
    (0x09995E, i_09995e),
    (0x099962, i_099962),
    (0x099966, i_099966),
    (0x09996C, i_09996c),
    (0x099970, i_099970),
    (0x099974, i_099974),
    (0x099976, i_099976),
    (0x09997A, i_09997a),
    (0x09997C, i_09997c),
    (0x099980, i_099980),
    (0x099984, i_099984),
    (0x099986, i_099986),
    (0x09998A, i_09998a),
    (0x09998C, i_09998c),
    (0x099990, i_099990),
    (0x099994, i_099994),
    (0x099998, i_099998),
    (0x09999C, i_09999c),
    (0x09999E, i_09999e),
    (0x0999A2, i_0999a2),
    (0x0999A6, i_0999a6),
    (0x0999AA, i_0999aa),
    (0x0999AC, i_0999ac),
    (0x0999B0, i_0999b0),
    (0x0999B4, i_0999b4),
    (0x0999B8, i_0999b8),
    (0x0999BA, i_0999ba),
    (0x0999BE, i_0999be),
    (0x0999C2, i_0999c2),
    (0x0999C6, i_0999c6),
    (0x0999C8, i_0999c8),
    (0x0999CA, i_0999ca),
    (0x0999CE, i_0999ce),
    (0x0999D2, i_0999d2),
    (0x0999D6, i_0999d6),
    (0x0999D8, i_0999d8),
    (0x0999DC, i_0999dc),
    (0x0999DE, i_0999de),
    (0x0999E2, i_0999e2),
    (0x0999E4, i_0999e4),
    (0x0999E8, i_0999e8),
    (0x0999EC, i_0999ec),
    (0x0999F0, i_0999f0),
    (0x0999F4, i_0999f4),
    (0x0999F6, i_0999f6),
    (0x0999F8, i_0999f8),
    (0x0999FA, i_0999fa),
    (0x0999FE, i_0999fe),
    (0x099A00, i_099a00),
    (0x099A06, i_099a06),
    (0x099A0C, i_099a0c),
    (0x099A12, i_099a12),
    (0x099A16, i_099a16),
    (0x099A1C, i_099a1c),
    (0x099A20, i_099a20),
    (0x099A22, i_099a22),
    (0x099A26, i_099a26),
    (0x099A2A, i_099a2a),
    (0x099A2E, i_099a2e),
    (0x099A32, i_099a32),
    (0x099A36, i_099a36),
    (0x099A38, i_099a38),
    (0x099A3A, i_099a3a),
    (0x099A3C, i_099a3c),
    (0x099A40, i_099a40),
    (0x099A44, i_099a44),
    (0x099A48, i_099a48),
    (0x099A4A, i_099a4a),
    (0x099A4E, i_099a4e),
    (0x099A54, i_099a54),
    (0x099A5C, i_099a5c),
    (0x099A60, i_099a60),
    (0x099A62, i_099a62),
    (0x099A64, i_099a64),
    (0x099A6A, i_099a6a),
    (0x099A6E, i_099a6e),
    (0x099A72, i_099a72),
    (0x099A76, i_099a76),
    (0x099A7A, i_099a7a),
    (0x099A7E, i_099a7e),
    (0x099A82, i_099a82),
    (0x099A86, i_099a86),
    (0x099A8A, i_099a8a),
    (0x099A8E, i_099a8e),
    (0x099A92, i_099a92),
    (0x099A96, i_099a96),
    (0x099A9A, i_099a9a),
    (0x099AA0, i_099aa0),
    (0x099AA4, i_099aa4),
    (0x099AA8, i_099aa8),
    (0x099AAC, i_099aac),
    (0x099AAE, i_099aae),
    (0x099AB2, i_099ab2),
    (0x099AB6, i_099ab6),
    (0x099ABA, i_099aba),
    (0x099ABE, i_099abe),
    (0x099AC4, i_099ac4),
    (0x099AC6, i_099ac6),
    (0x099ACA, i_099aca),
    (0x099ACE, i_099ace),
    (0x099AD2, i_099ad2),
    (0x099AD6, i_099ad6),
    (0x099ADA, i_099ada),
    (0x099ADC, i_099adc),
    (0x099AE0, i_099ae0),
    (0x099AE2, i_099ae2),
    (0x099AEA, i_099aea),
    (0x099AF2, i_099af2),
    (0x099AFA, i_099afa),
    (0x099AFC, i_099afc),
    (0x099B04, i_099b04),
    (0x099B7E, i_099b7e),
    (0x099B82, i_099b82),
    (0x099B84, i_099b84),
    (0x099B88, i_099b88),
    (0x099B8C, i_099b8c),
    (0x099B90, i_099b90),
    (0x099B94, i_099b94),
    (0x099B98, i_099b98),
    (0x099B9E, i_099b9e),
    (0x099BA2, i_099ba2),
    (0x099BA4, i_099ba4),
    (0x099D62, i_099d62),
    (0x099D66, i_099d66),
    (0x099D68, i_099d68),
    (0x099D6E, i_099d6e),
    (0x099D72, i_099d72),
    (0x099D76, i_099d76),
    (0x099D78, i_099d78),
    (0x099D80, i_099d80),
    (0x099D84, i_099d84),
    (0x099D8A, i_099d8a),
    (0x099D8E, i_099d8e),
    (0x099D92, i_099d92),
    (0x099D94, i_099d94),
    (0x099D98, i_099d98),
    (0x099D9C, i_099d9c),
    (0x099DA0, i_099da0),
    (0x099DA4, i_099da4),
    (0x099DA8, i_099da8),
    (0x099DAC, i_099dac),
    (0x099DB0, i_099db0),
    (0x099DB4, i_099db4),
    (0x099DB8, i_099db8),
    (0x099DBC, i_099dbc),
    (0x099DBE, i_099dbe),
    (0x099DC2, i_099dc2),
    (0x099DC6, i_099dc6),
    (0x099DCA, i_099dca),
    (0x099DCE, i_099dce),
    (0x099DD2, i_099dd2),
    (0x099DD6, i_099dd6),
    (0x099DDC, i_099ddc),
    (0x099DE0, i_099de0),
    (0x099DE2, i_099de2),
    (0x099DE4, i_099de4),
    (0x099DE8, i_099de8),
    (0x099DEC, i_099dec),
    (0x099DF0, i_099df0),
    (0x099DF6, i_099df6),
    (0x099DFA, i_099dfa),
    (0x099DFC, i_099dfc),
    (0x099E00, i_099e00),
    (0x099E04, i_099e04),
    (0x099E08, i_099e08),
    (0x099E0E, i_099e0e),
    (0x099E12, i_099e12),
    (0x099E14, i_099e14),
    (0x099E18, i_099e18),
    (0x099E1C, i_099e1c),
    (0x099E20, i_099e20),
    (0x099E26, i_099e26),
    (0x099E2A, i_099e2a),
    (0x099E30, i_099e30),
    (0x099E34, i_099e34),
    (0x099E36, i_099e36),
    (0x099E3A, i_099e3a),
    (0x099E3E, i_099e3e),
    (0x099E42, i_099e42),
    (0x099E48, i_099e48),
    (0x099E4C, i_099e4c),
    (0x099E52, i_099e52),
    (0x099E56, i_099e56),
    (0x099E58, i_099e58),
    (0x099E5C, i_099e5c),
    (0x099E60, i_099e60),
    (0x099E64, i_099e64),
    (0x099E6A, i_099e6a),
    (0x099E6E, i_099e6e),
    (0x099E74, i_099e74),
    (0x099E78, i_099e78),
    (0x099E7A, i_099e7a),
    (0x099E7E, i_099e7e),
    (0x099E82, i_099e82),
    (0x099E86, i_099e86),
    (0x099E8C, i_099e8c),
    (0x099E90, i_099e90),
    (0x099E96, i_099e96),
    (0x099E9A, i_099e9a),
    (0x099E9C, i_099e9c),
    (0x099EA4, i_099ea4),
    (0x099EA8, i_099ea8),
    (0x099EAA, i_099eaa),
    (0x099EAE, i_099eae),
    (0x099EB2, i_099eb2),
    (0x099EB8, i_099eb8),
    (0x099EBC, i_099ebc),
    (0x099EBE, i_099ebe),
    (0x099EC2, i_099ec2),
    (0x099EC6, i_099ec6),
    (0x099ECC, i_099ecc),
    (0x099ED0, i_099ed0),
    (0x099ED2, i_099ed2),
    (0x099ED8, i_099ed8),
    (0x099EDC, i_099edc),
    (0x099EE2, i_099ee2),
    (0x099EE4, i_099ee4),
    (0x099EEA, i_099eea),
    (0x099EEE, i_099eee),
    (0x099EF0, i_099ef0),
    (0x099EF4, i_099ef4),
    (0x099EF8, i_099ef8),
    (0x099F00, i_099f00),
    (0x099F04, i_099f04),
    (0x099F06, i_099f06),
    (0x099F0E, i_099f0e),
    (0x099F10, i_099f10),
    (0x099F16, i_099f16),
    (0x099F1A, i_099f1a),
    (0x099F1C, i_099f1c),
    (0x099F20, i_099f20),
    (0x099F24, i_099f24),
    (0x099F2C, i_099f2c),
    (0x099F30, i_099f30),
    (0x099F38, i_099f38),
    (0x099F3A, i_099f3a),
    (0x099F3E, i_099f3e),
    (0x099F42, i_099f42),
    (0x099F44, i_099f44),
    (0x099F48, i_099f48),
    (0x099F4C, i_099f4c),
    (0x099F54, i_099f54),
    (0x099F58, i_099f58),
    (0x099F5A, i_099f5a),
    (0x099F62, i_099f62),
    (0x099F66, i_099f66),
    (0x099F6A, i_099f6a),
    (0x099F72, i_099f72),
    (0x099F76, i_099f76),
    (0x099F78, i_099f78),
    (0x099F80, i_099f80),
    (0x099F84, i_099f84),
    (0x099F86, i_099f86),
    (0x099F8A, i_099f8a),
    (0x099F8E, i_099f8e),
    (0x099F90, i_099f90),
    (0x099F94, i_099f94),
    (0x099F98, i_099f98),
    (0x099FA0, i_099fa0),
    (0x099FA4, i_099fa4),
    (0x099FA6, i_099fa6),
    (0x099FAE, i_099fae),
    (0x099FB2, i_099fb2),
    (0x099FB6, i_099fb6),
    (0x099FBE, i_099fbe),
    (0x099FC2, i_099fc2),
    (0x099FC4, i_099fc4),
    (0x099FCC, i_099fcc),
    (0x099FD0, i_099fd0),
    (0x099FD2, i_099fd2),
    (0x099FD4, i_099fd4),
    (0x099FD8, i_099fd8),
    (0x099FDA, i_099fda),
    (0x099FE0, i_099fe0),
    (0x099FE2, i_099fe2),
    (0x099FE6, i_099fe6),
    (0x099FEA, i_099fea),
    (0x099FF0, i_099ff0),
    (0x099FF2, i_099ff2),
    (0x099FF6, i_099ff6),
    (0x099FF8, i_099ff8),
    (0x099FFC, i_099ffc),
    (0x099FFE, i_099ffe),
    (0x09A002, i_09a002),
    (0x09A00A, i_09a00a),
    (0x09A00E, i_09a00e),
    (0x09A010, i_09a010),
    (0x09A018, i_09a018),
    (0x09A01C, i_09a01c),
    (0x09A020, i_09a020),
    (0x09A028, i_09a028),
    (0x09A02C, i_09a02c),
    (0x09A02E, i_09a02e),
    (0x09A036, i_09a036),
    (0x09A03A, i_09a03a),
    (0x09A03C, i_09a03c),
    (0x09A040, i_09a040),
    (0x09A042, i_09a042),
    (0x09A046, i_09a046),
    (0x09A048, i_09a048),
    (0x09A04C, i_09a04c),
    (0x09A054, i_09a054),
    (0x09A058, i_09a058),
    (0x09A05A, i_09a05a),
    (0x09A062, i_09a062),
    (0x09A066, i_09a066),
    (0x09A06A, i_09a06a),
    (0x09A072, i_09a072),
    (0x09A076, i_09a076),
    (0x09A078, i_09a078),
    (0x09A080, i_09a080),
    (0x09A084, i_09a084),
    (0x09A086, i_09a086),
    (0x09A08E, i_09a08e),
    (0x09A092, i_09a092),
    (0x09A098, i_09a098),
    (0x09A09C, i_09a09c),
    (0x09A0A2, i_09a0a2),
    (0x09A0A6, i_09a0a6),
    (0x09A0AC, i_09a0ac),
    (0x09A0B0, i_09a0b0),
    (0x09A0B4, i_09a0b4),
    (0x09A0BA, i_09a0ba),
    (0x09A0BC, i_09a0bc),
    (0x09A0C0, i_09a0c0),
    (0x09A0C4, i_09a0c4),
    (0x09A0C8, i_09a0c8),
    (0x09A0CC, i_09a0cc),
    (0x09A0D0, i_09a0d0),
    (0x09A0D2, i_09a0d2),
    (0x09A0D6, i_09a0d6),
    (0x09A0D8, i_09a0d8),
    (0x09A0E0, i_09a0e0),
    (0x09A0E4, i_09a0e4),
    (0x09A0EA, i_09a0ea),
    (0x09A0EE, i_09a0ee),
    (0x09A0F4, i_09a0f4),
    (0x09A0FA, i_09a0fa),
    (0x09A0FC, i_09a0fc),
    (0x09A100, i_09a100),
    (0x09A102, i_09a102),
    (0x09A104, i_09a104),
    (0x09A108, i_09a108),
    (0x09A10C, i_09a10c),
    (0x09A110, i_09a110),
    (0x09A112, i_09a112),
    (0x09A116, i_09a116),
    (0x09A11A, i_09a11a),
    (0x09A120, i_09a120),
    (0x09A124, i_09a124),
    (0x09A126, i_09a126),
    (0x09A12A, i_09a12a),
    (0x09A12E, i_09a12e),
    (0x09A134, i_09a134),
    (0x09A138, i_09a138),
    (0x09A13E, i_09a13e),
    (0x09A142, i_09a142),
    (0x09A146, i_09a146),
    (0x09A14C, i_09a14c),
    (0x09A150, i_09a150),
    (0x09A154, i_09a154),
    (0x09A156, i_09a156),
    (0x09A15A, i_09a15a),
    (0x09A15E, i_09a15e),
    (0x09A160, i_09a160),
    (0x09A164, i_09a164),
    (0x09A168, i_09a168),
    (0x09A16A, i_09a16a),
    (0x09A16E, i_09a16e),
    (0x09A170, i_09a170),
    (0x09A172, i_09a172),
    (0x09A174, i_09a174),
    (0x09A17A, i_09a17a),
    (0x09A180, i_09a180),
    (0x09A186, i_09a186),
    (0x09A18A, i_09a18a),
    (0x09A18E, i_09a18e),
    (0x09A192, i_09a192),
    (0x09A196, i_09a196),
    (0x09A198, i_09a198),
    (0x09A19A, i_09a19a),
    (0x09A1A0, i_09a1a0),
    (0x09A1A4, i_09a1a4),
    (0x09A1A8, i_09a1a8),
    (0x09A1AC, i_09a1ac),
    (0x09A1B0, i_09a1b0),
    (0x09A1B6, i_09a1b6),
    (0x09A1BA, i_09a1ba),
    (0x09A1BE, i_09a1be),
    (0x09A1C0, i_09a1c0),
    (0x09A1C6, i_09a1c6),
    (0x09A1CA, i_09a1ca),
    (0x09A1CE, i_09a1ce),
    (0x09A1D2, i_09a1d2),
    (0x09A1D6, i_09a1d6),
    (0x09A1D8, i_09a1d8),
    (0x09A1DC, i_09a1dc),
    (0x09A1E0, i_09a1e0),
    (0x09A1EA, i_09a1ea),
    (0x09A1EE, i_09a1ee),
    (0x09A1F2, i_09a1f2),
    (0x09A1F6, i_09a1f6),
    (0x09A1FA, i_09a1fa),
    (0x09A1FE, i_09a1fe),
    (0x09A202, i_09a202),
    (0x09A206, i_09a206),
    (0x09A20E, i_09a20e),
    (0x09A214, i_09a214),
    (0x09A216, i_09a216),
    (0x09A21C, i_09a21c),
    (0x09A220, i_09a220),
    (0x09A226, i_09a226),
    (0x09A22A, i_09a22a),
    (0x09A230, i_09a230),
    (0x09A232, i_09a232),
    (0x09A238, i_09a238),
    (0x09A23C, i_09a23c),
    (0x09A240, i_09a240),
    (0x09A244, i_09a244),
    (0x09A246, i_09a246),
    (0x09A24C, i_09a24c),
    (0x09A24E, i_09a24e),
    (0x09A254, i_09a254),
    (0x09A258, i_09a258),
    (0x09A25E, i_09a25e),
    (0x09A264, i_09a264),
    (0x09A26A, i_09a26a),
    (0x09A26C, i_09a26c),
    (0x09A272, i_09a272),
    (0x09A276, i_09a276),
    (0x09A27A, i_09a27a),
    (0x09A27E, i_09a27e),
    (0x09A280, i_09a280),
    (0x09A284, i_09a284),
    (0x09A288, i_09a288),
    (0x09A28C, i_09a28c),
    (0x09A292, i_09a292),
    (0x09A296, i_09a296),
    (0x09A29A, i_09a29a),
    (0x09A29E, i_09a29e),
    (0x09A2A0, i_09a2a0),
    (0x09A2A6, i_09a2a6),
    (0x09A2AA, i_09a2aa),
    (0x09A2B0, i_09a2b0),
    (0x09A2B4, i_09a2b4),
    (0x09A2B6, i_09a2b6),
    (0x09A2B8, i_09a2b8),
    (0x09A2BC, i_09a2bc),
    (0x09A2C2, i_09a2c2),
    (0x09A2C6, i_09a2c6),
    (0x09A2CA, i_09a2ca),
    (0x09A2CE, i_09a2ce),
    (0x09A2D2, i_09a2d2),
    (0x09A2D4, i_09a2d4),
    (0x09A2D6, i_09a2d6),
    (0x09A2DC, i_09a2dc),
    (0x09A2E2, i_09a2e2),
    (0x09A2E8, i_09a2e8),
    (0x09A2EE, i_09a2ee),
    (0x09A2F4, i_09a2f4),
    (0x09A2F8, i_09a2f8),
    (0x09A2FC, i_09a2fc),
    (0x09A2FE, i_09a2fe),
    (0x09A300, i_09a300),
    (0x09A306, i_09a306),
    (0x09A30A, i_09a30a),
    (0x09A310, i_09a310),
    (0x09A316, i_09a316),
    (0x09A31E, i_09a31e),
    (0x09A322, i_09a322),
    (0x09A326, i_09a326),
    (0x09A32C, i_09a32c),
    (0x09A330, i_09a330),
    (0x09A336, i_09a336),
    (0x09A33C, i_09a33c),
    (0x09A342, i_09a342),
    (0x09A348, i_09a348),
    (0x09A34A, i_09a34a),
    (0x09A34E, i_09a34e),
    (0x09A352, i_09a352),
    (0x09A356, i_09a356),
    (0x09A35C, i_09a35c),
    (0x09A362, i_09a362),
    (0x09A368, i_09a368),
    (0x09A36E, i_09a36e),
    (0x09A372, i_09a372),
    (0x09A374, i_09a374),
    (0x09A378, i_09a378),
    (0x09A37C, i_09a37c),
    (0x09A382, i_09a382),
    (0x09A38A, i_09a38a),
    (0x09A38E, i_09a38e),
    (0x09A390, i_09a390),
    (0x09A394, i_09a394),
    (0x09A398, i_09a398),
    (0x09A39C, i_09a39c),
    (0x09A39E, i_09a39e),
    (0x09A3A2, i_09a3a2),
    (0x09A3A6, i_09a3a6),
    (0x09A3AA, i_09a3aa),
    (0x09A3AC, i_09a3ac),
    (0x09A3B0, i_09a3b0),
    (0x09A3B4, i_09a3b4),
    (0x09A3B6, i_09a3b6),
    (0x09A3BC, i_09a3bc),
    (0x09A3C0, i_09a3c0),
    (0x09A3C6, i_09a3c6),
    (0x09A3CA, i_09a3ca),
    (0x09A3D0, i_09a3d0),
    (0x09A3D4, i_09a3d4),
    (0x09A3D6, i_09a3d6),
    (0x09A3DA, i_09a3da),
    (0x09A3DC, i_09a3dc),
    (0x09A3E0, i_09a3e0),
    (0x09A3E4, i_09a3e4),
    (0x09A3E8, i_09a3e8),
    (0x09A3EC, i_09a3ec),
    (0x09A3F0, i_09a3f0),
    (0x09A3F4, i_09a3f4),
    (0x09A3F8, i_09a3f8),
    (0x09A3FC, i_09a3fc),
    (0x09A400, i_09a400),
    (0x09A404, i_09a404),
    (0x09A40A, i_09a40a),
    (0x09A410, i_09a410),
    (0x09A416, i_09a416),
    (0x09A41A, i_09a41a),
    (0x09A41E, i_09a41e),
    (0x09A422, i_09a422),
    (0x09A426, i_09a426),
    (0x09A42C, i_09a42c),
    (0x09A42E, i_09a42e),
    (0x09A432, i_09a432),
    (0x09A436, i_09a436),
    (0x09A43A, i_09a43a),
    (0x09A440, i_09a440),
    (0x09A444, i_09a444),
    (0x09A44A, i_09a44a),
    (0x09A44E, i_09a44e),
    (0x09A454, i_09a454),
    (0x09A458, i_09a458),
    (0x09A45C, i_09a45c),
    (0x09A460, i_09a460),
    (0x09A464, i_09a464),
    (0x09A468, i_09a468),
    (0x09A46C, i_09a46c),
    (0x09A470, i_09a470),
    (0x09A472, i_09a472),
    (0x09A476, i_09a476),
    (0x09A478, i_09a478),
    (0x09A47A, i_09a47a),
    (0x09A47C, i_09a47c),
    (0x09A47E, i_09a47e),
    (0x09A480, i_09a480),
    (0x09A482, i_09a482),
    (0x09A486, i_09a486),
    (0x09A48A, i_09a48a),
    (0x09A48C, i_09a48c),
    (0x09A490, i_09a490),
    (0x09A492, i_09a492),
    (0x09A496, i_09a496),
    (0x09A498, i_09a498),
    (0x09A49A, i_09a49a),
    (0x09A49E, i_09a49e),
    (0x09A4A2, i_09a4a2),
    (0x09A4A6, i_09a4a6),
    (0x09A4AC, i_09a4ac),
    (0x09A4B0, i_09a4b0),
    (0x09A4B6, i_09a4b6),
    (0x09A4BC, i_09a4bc),
    (0x09A4C0, i_09a4c0),
    (0x09A4C4, i_09a4c4),
    (0x09A4C8, i_09a4c8),
    (0x09A4CC, i_09a4cc),
    (0x09A4D2, i_09a4d2),
    (0x09A4D4, i_09a4d4),
    (0x09A4D8, i_09a4d8),
    (0x09A4DC, i_09a4dc),
    (0x09A4E0, i_09a4e0),
    (0x09A4E4, i_09a4e4),
    (0x09A4EA, i_09a4ea),
    (0x09A4EE, i_09a4ee),
    (0x09A4F2, i_09a4f2),
    (0x09A4F6, i_09a4f6),
    (0x09A4FA, i_09a4fa),
    (0x09A4FC, i_09a4fc),
    (0x09A500, i_09a500),
    (0x09A504, i_09a504),
    (0x09A506, i_09a506),
    (0x09A50A, i_09a50a),
    (0x09A50E, i_09a50e),
    (0x09A514, i_09a514),
    (0x09A51A, i_09a51a),
    (0x09A520, i_09a520),
    (0x09A524, i_09a524),
    (0x09A52A, i_09a52a),
    (0x09A530, i_09a530),
    (0x09A536, i_09a536),
    (0x09A53A, i_09a53a),
    (0x09A540, i_09a540),
    (0x09A546, i_09a546),
    (0x09A54C, i_09a54c),
    (0x09A552, i_09a552),
    (0x09A554, i_09a554),
    (0x09A558, i_09a558),
    (0x09A55A, i_09a55a),
    (0x09A55E, i_09a55e),
    (0x09A564, i_09a564),
    (0x09A568, i_09a568),
    (0x09A56C, i_09a56c),
    (0x09A570, i_09a570),
    (0x09A574, i_09a574),
    (0x09A576, i_09a576),
    (0x09A57C, i_09a57c),
    (0x09A57E, i_09a57e),
    (0x09A582, i_09a582),
    (0x09A588, i_09a588),
    (0x09A58C, i_09a58c),
    (0x09A592, i_09a592),
    (0x09A596, i_09a596),
    (0x09A598, i_09a598),
    (0x09A59E, i_09a59e),
    (0x09A5A4, i_09a5a4),
    (0x09A5AA, i_09a5aa),
    (0x09A5B0, i_09a5b0),
    (0x09A5B6, i_09a5b6),
    (0x09A5BA, i_09a5ba),
    (0x09A5C0, i_09a5c0),
    (0x09A5C6, i_09a5c6),
    (0x09A5CC, i_09a5cc),
    (0x09A5D0, i_09a5d0),
    (0x09A5D6, i_09a5d6),
    (0x09A5D8, i_09a5d8),
    (0x09A5DC, i_09a5dc),
    (0x09A5E0, i_09a5e0),
    (0x09A5E4, i_09a5e4),
    (0x09A5E8, i_09a5e8),
    (0x09A5EC, i_09a5ec),
    (0x09A5EE, i_09a5ee),
    (0x09A5F2, i_09a5f2),
    (0x09A75A, i_09a75a),
    (0x09A760, i_09a760),
    (0x09A764, i_09a764),
    (0x09A76A, i_09a76a),
    (0x09A76E, i_09a76e),
    (0x09A772, i_09a772),
    (0x09A778, i_09a778),
    (0x09A77C, i_09a77c),
    (0x09A782, i_09a782),
    (0x09A786, i_09a786),
    (0x09A78C, i_09a78c),
    (0x09A78E, i_09a78e),
    (0x09A794, i_09a794),
    (0x09A798, i_09a798),
    (0x09A79E, i_09a79e),
    (0x09A7A2, i_09a7a2),
    (0x09A7A8, i_09a7a8),
    (0x09A7AA, i_09a7aa),
    (0x09A7AE, i_09a7ae),
    (0x09A7B2, i_09a7b2),
    (0x09A7B6, i_09a7b6),
    (0x09A7BC, i_09a7bc),
    (0x09A7C0, i_09a7c0),
    (0x09A7C2, i_09a7c2),
    (0x09A7C6, i_09a7c6),
    (0x09A7CA, i_09a7ca),
    (0x09A7CC, i_09a7cc),
    (0x09A7CE, i_09a7ce),
    (0x09A7D2, i_09a7d2),
    (0x09A7D4, i_09a7d4),
    (0x09A7D8, i_09a7d8),
    (0x09A7DC, i_09a7dc),
    (0x09A7E0, i_09a7e0),
    (0x09A7E4, i_09a7e4),
    (0x09A7E6, i_09a7e6),
    (0x09A7E8, i_09a7e8),
    (0x09A7EC, i_09a7ec),
    (0x09A7F0, i_09a7f0),
    (0x09A7F2, i_09a7f2),
    (0x09A7F8, i_09a7f8),
    (0x09A7FE, i_09a7fe),
    (0x09A800, i_09a800),
    (0x09A806, i_09a806),
    (0x09A80A, i_09a80a),
    (0x09A810, i_09a810),
    (0x09A814, i_09a814),
    (0x09A816, i_09a816),
    (0x09A81A, i_09a81a),
    (0x09A81E, i_09a81e),
    (0x09A820, i_09a820),
    (0x09A826, i_09a826),
    (0x09A82C, i_09a82c),
    (0x09A830, i_09a830),
    (0x09A834, i_09a834),
    (0x09A836, i_09a836),
    (0x09A83A, i_09a83a),
    (0x09A83E, i_09a83e),
    (0x09A842, i_09a842),
    (0x09A846, i_09a846),
    (0x09A848, i_09a848),
    (0x09A84E, i_09a84e),
    (0x09A852, i_09a852),
    (0x09A854, i_09a854),
    (0x09A858, i_09a858),
    (0x09A85E, i_09a85e),
    (0x09A864, i_09a864),
    (0x09A868, i_09a868),
    (0x09A86A, i_09a86a),
    (0x09A86E, i_09a86e),
    (0x09A874, i_09a874),
    (0x09A876, i_09a876),
    (0x09A87C, i_09a87c),
    (0x09A87E, i_09a87e),
    (0x09A882, i_09a882),
    (0x09A886, i_09a886),
    (0x09A88A, i_09a88a),
    (0x09A88E, i_09a88e),
    (0x09A892, i_09a892),
    (0x09A896, i_09a896),
    (0x09A89A, i_09a89a),
    (0x09A89C, i_09a89c),
    (0x09A8A0, i_09a8a0),
    (0x09A8A6, i_09a8a6),
    (0x09A8A8, i_09a8a8),
    (0x09A8AC, i_09a8ac),
    (0x09A8B2, i_09a8b2),
    (0x09A8B6, i_09a8b6),
    (0x09A8BC, i_09a8bc),
    (0x09A8BE, i_09a8be),
    (0x09A8C4, i_09a8c4),
    (0x09A8CC, i_09a8cc),
    (0x09A8D0, i_09a8d0),
    (0x09A8D6, i_09a8d6),
    (0x09A8DE, i_09a8de),
    (0x09A8E2, i_09a8e2),
    (0x09A8E8, i_09a8e8),
    (0x09A8EE, i_09a8ee),
    (0x09A8F4, i_09a8f4),
    (0x09A8F8, i_09a8f8),
    (0x09A8FA, i_09a8fa),
    (0x09A900, i_09a900),
    (0x09A906, i_09a906),
    (0x09A90A, i_09a90a),
    (0x09A910, i_09a910),
    (0x09A914, i_09a914),
    (0x09A918, i_09a918),
    (0x09A91C, i_09a91c),
    (0x09A922, i_09a922),
    (0x09A928, i_09a928),
    (0x09A92C, i_09a92c),
    (0x09A932, i_09a932),
    (0x09A938, i_09a938),
    (0x09A93E, i_09a93e),
    (0x09A944, i_09a944),
    (0x09A94A, i_09a94a),
    (0x09A94E, i_09a94e),
    (0x09A950, i_09a950),
    (0x09A956, i_09a956),
    (0x09A95C, i_09a95c),
    (0x09A960, i_09a960),
    (0x09A964, i_09a964),
    (0x09A968, i_09a968),
    (0x09A96C, i_09a96c),
    (0x09A970, i_09a970),
    (0x09A972, i_09a972),
    (0x09A976, i_09a976),
    (0x09A97A, i_09a97a),
    (0x09A980, i_09a980),
    (0x09A986, i_09a986),
    (0x09A98A, i_09a98a),
    (0x09A990, i_09a990),
    (0x09A996, i_09a996),
    (0x09A99C, i_09a99c),
    (0x09A9A2, i_09a9a2),
    (0x09A9A8, i_09a9a8),
    (0x09A9AC, i_09a9ac),
    (0x09A9AE, i_09a9ae),
    (0x09A9B4, i_09a9b4),
    (0x09A9BA, i_09a9ba),
    (0x09A9BE, i_09a9be),
    (0x09A9C2, i_09a9c2),
    (0x09A9C6, i_09a9c6),
    (0x09A9CA, i_09a9ca),
    (0x09A9CE, i_09a9ce),
    (0x09A9D0, i_09a9d0),
    (0x09A9D4, i_09a9d4),
    (0x09A9D8, i_09a9d8),
    (0x09A9DE, i_09a9de),
    (0x09A9E4, i_09a9e4),
    (0x09A9E8, i_09a9e8),
    (0x09A9EE, i_09a9ee),
    (0x09A9F4, i_09a9f4),
    (0x09A9FA, i_09a9fa),
    (0x09AA00, i_09aa00),
    (0x09AA06, i_09aa06),
    (0x09AA0A, i_09aa0a),
    (0x09AA0C, i_09aa0c),
    (0x09AA12, i_09aa12),
    (0x09AA18, i_09aa18),
    (0x09AA1C, i_09aa1c),
    (0x09AA20, i_09aa20),
    (0x09AA24, i_09aa24),
    (0x09AA28, i_09aa28),
    (0x09AA2C, i_09aa2c),
    (0x09AA2E, i_09aa2e),
    (0x09AA32, i_09aa32),
    (0x09AA36, i_09aa36),
    (0x09AA3C, i_09aa3c),
    (0x09AA42, i_09aa42),
    (0x09AA46, i_09aa46),
    (0x09AA4C, i_09aa4c),
    (0x09AA52, i_09aa52),
    (0x09AA58, i_09aa58),
    (0x09AA5E, i_09aa5e),
    (0x09AA64, i_09aa64),
    (0x09AA68, i_09aa68),
    (0x09AA6A, i_09aa6a),
    (0x09AA70, i_09aa70),
    (0x09AA76, i_09aa76),
    (0x09AA7A, i_09aa7a),
    (0x09AA7E, i_09aa7e),
    (0x09AA82, i_09aa82),
    (0x09AA86, i_09aa86),
    (0x09AA8A, i_09aa8a),
    (0x09AA8C, i_09aa8c),
    (0x09AA90, i_09aa90),
    (0x09AA94, i_09aa94),
    (0x09AA9A, i_09aa9a),
    (0x09AAA0, i_09aaa0),
    (0x09AAA4, i_09aaa4),
    (0x09AAA8, i_09aaa8),
    (0x09AAAC, i_09aaac),
    (0x09AAB0, i_09aab0),
    (0x09AAB4, i_09aab4),
    (0x09AAB6, i_09aab6),
    (0x09AABA, i_09aaba),
    (0x09AABE, i_09aabe),
    (0x09AAC2, i_09aac2),
    (0x09AAC6, i_09aac6),
    (0x09AACC, i_09aacc),
    (0x09AAD0, i_09aad0),
    (0x09AAD6, i_09aad6),
    (0x09AADC, i_09aadc),
    (0x09AAE2, i_09aae2),
    (0x09AAE8, i_09aae8),
    (0x09AAEE, i_09aaee),
    (0x09AAF2, i_09aaf2),
    (0x09AAF4, i_09aaf4),
    (0x09AAF8, i_09aaf8),
    (0x09AAFE, i_09aafe),
    (0x09AB02, i_09ab02),
    (0x09AB06, i_09ab06),
    (0x09AB08, i_09ab08),
    (0x09AB0E, i_09ab0e),
    (0x09AB14, i_09ab14),
    (0x09AB18, i_09ab18),
    (0x09AB1C, i_09ab1c),
    (0x09AB20, i_09ab20),
    (0x09AB24, i_09ab24),
    (0x09AB28, i_09ab28),
    (0x09AB2C, i_09ab2c),
    (0x09AB32, i_09ab32),
    (0x09AB36, i_09ab36),
    (0x09AB3C, i_09ab3c),
    (0x09AB40, i_09ab40),
    (0x09AB44, i_09ab44),
    (0x09AB4A, i_09ab4a),
    (0x09AB4E, i_09ab4e),
    (0x09AB50, i_09ab50),
    (0x09AB54, i_09ab54),
    (0x09AB5C, i_09ab5c),
    (0x09AB60, i_09ab60),
    (0x09AB64, i_09ab64),
    (0x09AB66, i_09ab66),
    (0x09AB6A, i_09ab6a),
    (0x09AB6E, i_09ab6e),
    (0x09AB72, i_09ab72),
    (0x09AB76, i_09ab76),
    (0x09AB7A, i_09ab7a),
    (0x09AB80, i_09ab80),
    (0x09AB84, i_09ab84),
    (0x09AB88, i_09ab88),
    (0x09AB8A, i_09ab8a),
    (0x09AB8E, i_09ab8e),
    (0x09AB92, i_09ab92),
    (0x09AB96, i_09ab96),
    (0x09AB9A, i_09ab9a),
    (0x09AB9C, i_09ab9c),
    (0x09ABA0, i_09aba0),
    (0x09ABA4, i_09aba4),
    (0x09ABA8, i_09aba8),
    (0x09ABAC, i_09abac),
    (0x09ABB0, i_09abb0),
    (0x09ABB4, i_09abb4),
    (0x09ABB8, i_09abb8),
    (0x09ABBC, i_09abbc),
    (0x09ABBE, i_09abbe),
    (0x09ABC2, i_09abc2),
    (0x09ABC6, i_09abc6),
    (0x09ABCC, i_09abcc),
    (0x09ABD0, i_09abd0),
    (0x09ABD6, i_09abd6),
    (0x09ABDC, i_09abdc),
    (0x09ABE0, i_09abe0),
    (0x09ABE6, i_09abe6),
    (0x09ABEC, i_09abec),
    (0x09ABF2, i_09abf2),
    (0x09ABF8, i_09abf8),
    (0x09ABFE, i_09abfe),
    (0x09AC02, i_09ac02),
    (0x09AC06, i_09ac06),
    (0x09AC0C, i_09ac0c),
    (0x09AC10, i_09ac10),
    (0x09AC14, i_09ac14),
    (0x09AC1A, i_09ac1a),
    (0x09AC1E, i_09ac1e),
    (0x09AC20, i_09ac20),
    (0x09AC26, i_09ac26),
    (0x09AC2C, i_09ac2c),
    (0x09AC30, i_09ac30),
    (0x09AC34, i_09ac34),
    (0x09AC38, i_09ac38),
    (0x09AC3E, i_09ac3e),
    (0x09AC42, i_09ac42),
    (0x09AC44, i_09ac44),
    (0x09AC48, i_09ac48),
    (0x09AC4E, i_09ac4e),
    (0x09AC52, i_09ac52),
    (0x09AC56, i_09ac56),
    (0x09AC5C, i_09ac5c),
    (0x09AC60, i_09ac60),
    (0x09AC66, i_09ac66),
    (0x09AC6C, i_09ac6c),
    (0x09AC70, i_09ac70),
    (0x09AC76, i_09ac76),
    (0x09AC7C, i_09ac7c),
    (0x09AC82, i_09ac82),
    (0x09AC88, i_09ac88),
    (0x09AC8E, i_09ac8e),
    (0x09AC92, i_09ac92),
    (0x09AC96, i_09ac96),
    (0x09AC9C, i_09ac9c),
    (0x09ACA0, i_09aca0),
    (0x09ACA4, i_09aca4),
    (0x09ACAA, i_09acaa),
    (0x09ACAE, i_09acae),
    (0x09ACB0, i_09acb0),
    (0x09ACB6, i_09acb6),
    (0x09ACBC, i_09acbc),
    (0x09ACC0, i_09acc0),
    (0x09ACC4, i_09acc4),
    (0x09ACC8, i_09acc8),
    (0x09ACCC, i_09accc),
    (0x09ACD2, i_09acd2),
    (0x09ACD6, i_09acd6),
    (0x09ACDC, i_09acdc),
    (0x09ACE2, i_09ace2),
    (0x09ACE6, i_09ace6),
    (0x09ACEA, i_09acea),
    (0x09ACF0, i_09acf0),
    (0x09ACF6, i_09acf6),
    (0x09ACFA, i_09acfa),
    (0x09AD00, i_09ad00),
    (0x09AD06, i_09ad06),
    (0x09AD0C, i_09ad0c),
    (0x09AD12, i_09ad12),
    (0x09AD16, i_09ad16),
    (0x09AD1A, i_09ad1a),
    (0x09AD20, i_09ad20),
    (0x09AD26, i_09ad26),
    (0x09AD2A, i_09ad2a),
    (0x09AD30, i_09ad30),
    (0x09AD36, i_09ad36),
    (0x09AD3C, i_09ad3c),
    (0x09AD40, i_09ad40),
    (0x09AD44, i_09ad44),
    (0x09AD46, i_09ad46),
    (0x09AD4A, i_09ad4a),
    (0x09AD50, i_09ad50),
    (0x09AD54, i_09ad54),
    (0x09AD58, i_09ad58),
    (0x09AD5E, i_09ad5e),
    (0x09AD62, i_09ad62),
    (0x09AD68, i_09ad68),
    (0x09AD6E, i_09ad6e),
    (0x09AD72, i_09ad72),
    (0x09AD78, i_09ad78),
    (0x09AD7E, i_09ad7e),
    (0x09AD84, i_09ad84),
    (0x09AD8A, i_09ad8a),
    (0x09AD90, i_09ad90),
    (0x09AD94, i_09ad94),
    (0x09AD96, i_09ad96),
    (0x09AD9C, i_09ad9c),
    (0x09ADA2, i_09ada2),
    (0x09ADA6, i_09ada6),
    (0x09ADAA, i_09adaa),
    (0x09ADAE, i_09adae),
    (0x09ADB2, i_09adb2),
    (0x09ADB8, i_09adb8),
    (0x09ADBC, i_09adbc),
    (0x09ADC2, i_09adc2),
    (0x09ADC6, i_09adc6),
    (0x09ADCC, i_09adcc),
    (0x09ADD0, i_09add0),
    (0x09ADD4, i_09add4),
    (0x09ADD8, i_09add8),
    (0x09ADDC, i_09addc),
    (0x09ADE2, i_09ade2),
    (0x09ADE6, i_09ade6),
    (0x09ADEA, i_09adea),
    (0x09ADEE, i_09adee),
    (0x09ADF2, i_09adf2),
    (0x09ADF8, i_09adf8),
    (0x09ADFE, i_09adfe),
    (0x09AE04, i_09ae04),
    (0x09AE0A, i_09ae0a),
    (0x09AE0E, i_09ae0e),
    (0x09AE12, i_09ae12),
    (0x09AE16, i_09ae16),
    (0x09AE18, i_09ae18),
    (0x09AE1C, i_09ae1c),
    (0x09AE22, i_09ae22),
    (0x09AE26, i_09ae26),
    (0x09AE2A, i_09ae2a),
    (0x09AE2E, i_09ae2e),
    (0x09AE32, i_09ae32),
    (0x09AE36, i_09ae36),
    (0x09AE3A, i_09ae3a),
    (0x09AE3E, i_09ae3e),
    (0x09AE42, i_09ae42),
    (0x09AE48, i_09ae48),
    (0x09AE4E, i_09ae4e),
    (0x09AE52, i_09ae52),
    (0x09AE58, i_09ae58),
    (0x09AE5E, i_09ae5e),
    (0x09AE64, i_09ae64),
    (0x09AE68, i_09ae68),
    (0x09AE6A, i_09ae6a),
    (0x09AE70, i_09ae70),
    (0x09AE76, i_09ae76),
    (0x09AE7A, i_09ae7a),
    (0x09AE80, i_09ae80),
    (0x09AE84, i_09ae84),
    (0x09AE88, i_09ae88),
    (0x09AE8A, i_09ae8a),
    (0x09AE8E, i_09ae8e),
    (0x09AE92, i_09ae92),
    (0x09AE98, i_09ae98),
    (0x09AE9E, i_09ae9e),
    (0x09AEA4, i_09aea4),
    (0x09AEA8, i_09aea8),
    (0x09AEAA, i_09aeaa),
    (0x09AEB0, i_09aeb0),
    (0x09AEB6, i_09aeb6),
    (0x09AEBA, i_09aeba),
    (0x09AEC0, i_09aec0),
    (0x09AEC4, i_09aec4),
    (0x09AEC8, i_09aec8),
    (0x09AECE, i_09aece),
    (0x09AED4, i_09aed4),
    (0x09AF2A, i_09af2a),
    (0x09AF2E, i_09af2e),
    (0x09AF34, i_09af34),
    (0x09AF3A, i_09af3a),
    (0x09AF3E, i_09af3e),
    (0x09AF42, i_09af42),
    (0x09AF48, i_09af48),
    (0x09AF4E, i_09af4e),
    (0x09AF52, i_09af52),
    (0x09AF58, i_09af58),
    (0x09AF5E, i_09af5e),
    (0x09AF64, i_09af64),
    (0x09AF68, i_09af68),
    (0x09AF6C, i_09af6c),
    (0x09AF72, i_09af72),
    (0x09AF78, i_09af78),
    (0x09AFE0, i_09afe0),
    (0x09AFE4, i_09afe4),
    (0x09AFEA, i_09afea),
    (0x09AFF0, i_09aff0),
    (0x09AFF4, i_09aff4),
    (0x09AFF8, i_09aff8),
    (0x09AFFE, i_09affe),
    (0x09B004, i_09b004),
    (0x09B05C, i_09b05c),
    (0x09B060, i_09b060),
    (0x09B064, i_09b064),
    (0x09B068, i_09b068),
    (0x09B06E, i_09b06e),
    (0x09B072, i_09b072),
    (0x09B078, i_09b078),
    (0x09B07E, i_09b07e),
    (0x09B084, i_09b084),
    (0x09B088, i_09b088),
    (0x09B08E, i_09b08e),
    (0x09B094, i_09b094),
    (0x09B09A, i_09b09a),
    (0x09B09E, i_09b09e),
    (0x09B0A2, i_09b0a2),
    (0x09B0A6, i_09b0a6),
    (0x09B0A8, i_09b0a8),
    (0x09B0AC, i_09b0ac),
    (0x09B0B2, i_09b0b2),
    (0x09B0B6, i_09b0b6),
    (0x09B0BC, i_09b0bc),
    (0x09B0C0, i_09b0c0),
    (0x09B0C4, i_09b0c4),
];
