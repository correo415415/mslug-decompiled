// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07_5) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $079E08: jsr $2870a.l
pub fn i_079e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E0E);
    cpu.pc = 0x2870A;
    16
}
/// $079E0E: bcc.w $79e18
pub fn i_079e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79E18; return 10; }
    8
}
/// $079E12: lea.l $79e2a(pc), a1
pub fn i_079e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79E2A;
    16
}
/// $079E16: move.l a1, (a6)
pub fn i_079e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079E18: jsr $7fde0.l
pub fn i_079e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E1E);
    cpu.pc = 0x7FDE0;
    16
}
/// $079E1E: bcc.w $79e28
pub fn i_079e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79E28; return 10; }
    8
}
// ===== SetTaskHandler_079e22  ($079E22, 8 B, kernel/task_handlers.c)
/// $079E22: lea.l $79e94(pc), a1
pub fn i_079e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79E94;
    16
}
/// $079E26: move.l a1, (a6)
pub fn i_079e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079E28: rts
pub fn i_079e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_HatchB_Land_079e2a  ($079E2A, 54 B, props/fx_explosions_platforms_crew_076000.s)
/// $079E2A: tst.b $10fd8f.l
pub fn i_079e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E30;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $079E30: bne.w $79e3e
pub fn i_079e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E34;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79E3E; return 10; }
    8
}
/// $079E34: move.w #$1d5, d1
pub fn i_079e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E38;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079E38: jsr $236e.l
pub fn i_079e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E3E);
    cpu.pc = 0x236E;
    16
}
/// $079E3E: lea.l $29c28c.l, a0
pub fn i_079e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C28C;
    20
}
/// $079E44: jsr $28cd4.l
pub fn i_079e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E4A);
    cpu.pc = 0x28CD4;
    16
}
/// $079E4A: lea.l $79e50(pc), a1
pub fn i_079e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79E50;
    16
}
/// $079E4E: move.l a1, (a6)
pub fn i_079e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E50;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079E50: jsr $7fdfa.l
pub fn i_079e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E56);
    cpu.pc = 0x7FDFA;
    16
}
/// $079E56: jsr $28d70.l
pub fn i_079e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E5C);
    cpu.pc = 0x28D70;
    16
}
/// $079E5C: bcc.w $79e66
pub fn i_079e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79E66; return 10; }
    8
}
// ===== SetTaskHandler_079e60  ($079E60, 8 B, kernel/task_handlers.c)
/// $079E60: lea.l $79e68(pc), a1
pub fn i_079e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79E68;
    16
}
/// $079E64: move.l a1, (a6)
pub fn i_079e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079E66: rts
pub fn i_079e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_HatchB_Flee_079e68  ($079E68, 44 B, props/fx_explosions_platforms_crew_076000.s)
/// $079E68: move.b #$28, $59(a6)
pub fn i_079e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E6E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079E6E: lea.l $79e74(pc), a1
pub fn i_079e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79E74;
    16
}
/// $079E72: move.l a1, (a6)
pub fn i_079e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079E74: jsr $7fdfa.l
pub fn i_079e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E7A);
    cpu.pc = 0x7FDFA;
    16
}
/// $079E7A: jsr $28d70.l
pub fn i_079e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E80);
    cpu.pc = 0x28D70;
    16
}
/// $079E80: tst.b $59(a6)
pub fn i_079e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E84;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $079E84: bne.w $79e9a
pub fn i_079e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E88;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79E9A; return 10; }
    8
}
/// $079E88: movea.l $c(a6), a0
pub fn i_079e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E8C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079E8C: move.w $72(a6), d0
pub fn i_079e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E90;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079E90: bclr.b d0, $70(a0)
pub fn i_079e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E94;
    cpu.ir = 0x1A8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== Crew_HatchB_Free_079e94  ($079E94, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $079E94: jmp $518.l
pub fn i_079e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E9A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $079E9A: rts
pub fn i_079e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_RetGateC_079e9c  ($079E9C, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079E9C: movea.l $8(a6), a1
pub fn i_079e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EA0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079EA0: move.b $10(a6), d0
pub fn i_079ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EA4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079EA4: cmp.b $10(a1), d0
pub fn i_079ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EA8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $079EA8: bcs.w $79eb2
pub fn i_079ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EAC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x79EB2; return 10; }
    8
}
// ===== ClearXN_079eac  ($079EAC, 6 B, thunks/ccr_helpers.c)
/// $079EAC: andi.b #$ee, ccr
pub fn i_079eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EB0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $079EB0: rts
pub fn i_079eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_079eb2  ($079EB2, 6 B, thunks/ccr_helpers.c)
/// $079EB2: ori.b #$11, ccr
pub fn i_079eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EB6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $079EB6: rts
pub fn i_079eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Tmpl127_079eb8  ($079EB8, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $079EB8: move.w #$ffff, $70(a6)
pub fn i_079eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EBE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079EBE: bra.w $79ec8
pub fn i_079ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x79EC8;
    10
}
// ===== Crew_Tmpl128_079ec2  ($079EC2, 134 B, props/fx_explosions_platforms_crew_076000.s)
/// $079EC2: move.w #$0, $70(a6)
pub fn i_079ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079EC8: moveq #$0, d0
pub fn i_079ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ECA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079ECA: move.b d0, $20(a6)
pub fn i_079eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ECE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079ECE: move.b d0, $21(a6)
pub fn i_079ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ED2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079ED2: lea.l $79fe8(pc), a1
pub fn i_079ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ED6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79FE8;
    16
}
/// $079ED6: jsr $4ae.l
pub fn i_079ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79EDC);
    cpu.pc = 0x4AE;
    16
}
/// $079EDC: jsr $5dd02.l
pub fn i_079edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79EE2);
    cpu.pc = 0x5DD02;
    16
}
/// $079EE2: subi.w #$40, $22(a0)
pub fn i_079ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EE8;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $079EE8: lea.l $7a19e(pc), a1
pub fn i_079ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A19E;
    16
}
/// $079EEC: jsr $4ae.l
pub fn i_079eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79EF2);
    cpu.pc = 0x4AE;
    16
}
/// $079EF2: jsr $5dd02.l
pub fn i_079ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79EF8);
    cpu.pc = 0x5DD02;
    16
}
/// $079EF8: subi.w #$20, $22(a0)
pub fn i_079ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EFE;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $079EFE: lea.l $79f48(pc), a1
pub fn i_079efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79F48;
    16
}
/// $079F02: jsr $4ae.l
pub fn i_079f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F08);
    cpu.pc = 0x4AE;
    16
}
/// $079F08: jsr $5dd02.l
pub fn i_079f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F0E);
    cpu.pc = 0x5DD02;
    16
}
/// $079F0E: bclr.b #$1, $12(a6)
pub fn i_079f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F14;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $079F14: lea.l $79f1a(pc), a1
pub fn i_079f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79F1A;
    16
}
/// $079F18: move.l a1, (a6)
pub fn i_079f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079F1A: tst.w $70(a6)
pub fn i_079f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F1E;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $079F1E: beq.w $79f30
pub fn i_079f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F22;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79F30; return 10; }
    8
}
/// $079F22: movea.l $c(a6), a0
pub fn i_079f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F26;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079F26: movea.l $c(a0), a0
pub fn i_079f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F2A;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079F2A: move.b $20(a0), $20(a6)
pub fn i_079f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F30;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $079F30: cmpi.b #$ff, $21(a6)
pub fn i_079f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F36;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $079F36: bne.w $79f40
pub fn i_079f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79F40; return 10; }
    8
}
/// $079F3A: jmp $518.l
pub fn i_079f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F40;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $079F40: move.w #$0, $72(a6)
pub fn i_079f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F46;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079F46: rts
pub fn i_079f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Gunner_079f48  ($079F48, 26 B, props/fx_explosions_platforms_crew_076000.s)
/// $079F48: move.w #$e, d1
pub fn i_079f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F4C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079F4C: jsr $236e.l
pub fn i_079f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F52);
    cpu.pc = 0x236E;
    16
}
/// $079F52: moveq #$0, d0
pub fn i_079f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F54;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079F54: move.b d0, $20(a6)
pub fn i_079f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F58;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079F58: move.b d0, $21(a6)
pub fn i_079f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F5C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079F5C: move.w #$1, $66(a6)
pub fn i_079f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F62;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_079f62  ($079F62, 8 B, kernel/task_handlers.c)
/// $079F62: lea.l $79f6a(pc), a1
pub fn i_079f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79F6A;
    16
}
/// $079F66: move.l a1, (a6)
pub fn i_079f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F68;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079F68: rts
pub fn i_079f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Gunner_Run_079f6a  ($079F6A, 118 B, props/fx_explosions_platforms_crew_076000.s)
/// $079F6A: jsr $5e7c0.l
pub fn i_079f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F70);
    cpu.pc = 0x5E7C0;
    16
}
/// $079F70: move.w #$8000, d0
pub fn i_079f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F74;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079F74: jsr $28134.l
pub fn i_079f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F7A);
    cpu.pc = 0x28134;
    16
}
/// $079F7A: andi.w #$ffe3, $38(a6)
pub fn i_079f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F80;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $079F80: ori.w #$c, $38(a6)
pub fn i_079f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F86;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $079F86: lea.l $2df4f2.l, a0
pub fn i_079f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF4F2;
    20
}
/// $079F8C: jsr $28cd4.l
pub fn i_079f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F92);
    cpu.pc = 0x28CD4;
    16
}
/// $079F92: lea.l $79f98(pc), a1
pub fn i_079f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79F98;
    16
}
/// $079F96: move.l a1, (a6)
pub fn i_079f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079F98: jsr $2783a.l
pub fn i_079f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F9E);
    cpu.pc = 0x2783A;
    16
}
/// $079F9E: jsr $28d70.l
pub fn i_079f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79FA4);
    cpu.pc = 0x28D70;
    16
}
/// $079FA4: movea.l $c(a6), a0
pub fn i_079fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FA8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079FA8: cmpi.b #$ff, $20(a0)
pub fn i_079fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FAE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $079FAE: beq.w $79fbc
pub fn i_079fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FB2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79FBC; return 10; }
    8
}
/// $079FB2: cmpi.b #$ff, $21(a0)
pub fn i_079fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FB8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $079FB8: bne.w $79fc4
pub fn i_079fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FBC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79FC4; return 10; }
    8
}
/// $079FBC: lea.l $58fc2.l, a1
pub fn i_079fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FC2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58FC2;
    20
}
/// $079FC2: move.l a1, (a6)
pub fn i_079fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079FC4: jsr $49fd0.l
pub fn i_079fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79FCA);
    cpu.pc = 0x49FD0;
    16
}
/// $079FCA: movea.l #$ffffffff, a0
pub fn i_079fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FD0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $079FD0: lea.l $2df594.l, a0
pub fn i_079fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FD6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF594;
    20
}
/// $079FD6: jsr $5dd5c.l
pub fn i_079fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79FDC);
    cpu.pc = 0x5DD5C;
    16
}
/// $079FDC: bcc.w $79fe6
pub fn i_079fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79FE6; return 10; }
    8
}
// ===== SetTaskHandler_079fe0  ($079FE0, 8 B, kernel/task_handlers.c)
/// $079FE0: lea.l $7a3e6(pc), a1
pub fn i_079fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3E6;
    16
}
/// $079FE4: move.l a1, (a6)
pub fn i_079fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079FE6: rts
pub fn i_079fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Driver_079fe8  ($079FE8, 26 B, props/fx_explosions_platforms_crew_076000.s)
/// $079FE8: move.w #$e, d1
pub fn i_079fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FEC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079FEC: jsr $236e.l
pub fn i_079fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79FF2);
    cpu.pc = 0x236E;
    16
}
/// $079FF2: move.w #$1, $66(a6)
pub fn i_079ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FF8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079FF8: moveq #$0, d0
pub fn i_079ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FFA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079FFA: move.b d0, $20(a6)
pub fn i_079ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FFE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079FFE: move.b d0, $21(a6)
pub fn i_079ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A002;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetTaskHandler_07a002  ($07A002, 8 B, kernel/task_handlers.c)
/// $07A002: lea.l $7a00a(pc), a1
pub fn i_07a002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A006;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A00A;
    16
}
/// $07A006: move.l a1, (a6)
pub fn i_07a006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A008;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A008: rts
pub fn i_07a008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A00A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hostage_Idle_07a00a  ($07A00A, 158 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A00A: jsr $5e7c0.l
pub fn i_07a00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A010;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A010);
    cpu.pc = 0x5E7C0;
    16
}
/// $07A010: move.w #$8000, d0
pub fn i_07a010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A014;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A014: jsr $28134.l
pub fn i_07a014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A01A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A01A);
    cpu.pc = 0x28134;
    16
}
/// $07A01A: andi.w #$ffe3, $38(a6)
pub fn i_07a01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A020;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A020: ori.w #$c, $38(a6)
pub fn i_07a020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A026;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A026: addq.w #$1, $38(a6)
pub fn i_07a026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A02A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07A02A: lea.l $29c36a.l, a0
pub fn i_07a02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A030;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C36A;
    20
}
/// $07A030: jsr $28cd4.l
pub fn i_07a030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A036;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A036);
    cpu.pc = 0x28CD4;
    16
}
/// $07A036: lea.l $7a03c(pc), a1
pub fn i_07a036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A03A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A03C;
    16
}
/// $07A03A: move.l a1, (a6)
pub fn i_07a03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A03C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A03C: jsr $2783a.l
pub fn i_07a03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A042;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A042);
    cpu.pc = 0x2783A;
    16
}
/// $07A042: jsr $28d70.l
pub fn i_07a042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A048;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A048);
    cpu.pc = 0x28D70;
    16
}
/// $07A048: movea.l $c(a6), a0
pub fn i_07a048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A04C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A04C: cmpi.w #$2, $72(a0)
pub fn i_07a04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A052;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $07A052: bne.w $7a05c
pub fn i_07a052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A056;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A05C; return 10; }
    8
}
/// $07A056: lea.l $7a0b8(pc), a1
pub fn i_07a056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A05A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A0B8;
    16
}
/// $07A05A: move.l a1, (a6)
pub fn i_07a05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A05C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A05C: movea.l $c(a6), a0
pub fn i_07a05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A060;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A060: cmpi.b #$ff, $20(a0)
pub fn i_07a060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A066;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07A066: beq.w $7a074
pub fn i_07a066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A06A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A074; return 10; }
    8
}
/// $07A06A: cmpi.b #$ff, $21(a0)
pub fn i_07a06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A070;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07A070: bne.w $7a082
pub fn i_07a070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A074;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A082; return 10; }
    8
}
/// $07A074: eori.b #$1, $3a(a6)
pub fn i_07a074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A07A;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07A07A: lea.l $58fc2.l, a1
pub fn i_07a07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A080;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58FC2;
    20
}
/// $07A080: move.l a1, (a6)
pub fn i_07a080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A082;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A082: jsr $49fd0.l
pub fn i_07a082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A088;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A088);
    cpu.pc = 0x49FD0;
    16
}
/// $07A088: btst.b #$3, $13(a6)
pub fn i_07a088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A08E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07A08E: bne.w $7a0b0
pub fn i_07a08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A092;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A0B0; return 10; }
    8
}
/// $07A092: movea.l #$ffffffff, a0
pub fn i_07a092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A098;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A098: lea.l $2df594.l, a0
pub fn i_07a098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A09E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF594;
    20
}
/// $07A09E: jsr $5dd5c.l
pub fn i_07a09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A0A4);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A0A4: bcc.w $7a0ae
pub fn i_07a0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A0AE; return 10; }
    8
}
// ===== SetTaskHandler_07a0a8  ($07A0A8, 8 B, kernel/task_handlers.c)
/// $07A0A8: lea.l $7a3e6(pc), a1
pub fn i_07a0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3E6;
    16
}
/// $07A0AC: move.l a1, (a6)
pub fn i_07a0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A0AE: rts
pub fn i_07a0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hostage_ToggleFacing_07a0b0  ($07A0B0, 122 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A0B0: eori.b #$1, $3a(a6)
pub fn i_07a0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0B6;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07A0B6: rts
pub fn i_07a0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07A0B8: lea.l $29c39e.l, a0
pub fn i_07a0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C39E;
    20
}
/// $07A0BE: jsr $28cd4.l
pub fn i_07a0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A0C4);
    cpu.pc = 0x28CD4;
    16
}
/// $07A0C4: lea.l $7a0ca(pc), a1
pub fn i_07a0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A0CA;
    16
}
/// $07A0C8: move.l a1, (a6)
pub fn i_07a0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A0CA: jsr $2783a.l
pub fn i_07a0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A0D0);
    cpu.pc = 0x2783A;
    16
}
/// $07A0D0: jsr $28d70.l
pub fn i_07a0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A0D6);
    cpu.pc = 0x28D70;
    16
}
/// $07A0D6: bcc.w $7a0e0
pub fn i_07a0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A0E0; return 10; }
    8
}
/// $07A0DA: lea.l $7a02a(pc), a1
pub fn i_07a0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A02A;
    16
}
/// $07A0DE: move.l a1, (a6)
pub fn i_07a0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A0E0: movea.l $c(a6), a0
pub fn i_07a0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0E4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A0E4: cmpi.b #$ff, $20(a0)
pub fn i_07a0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0EA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07A0EA: beq.w $7a0f8
pub fn i_07a0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0EE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A0F8; return 10; }
    8
}
/// $07A0EE: cmpi.b #$ff, $21(a0)
pub fn i_07a0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0F4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07A0F4: bne.w $7a106
pub fn i_07a0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A106; return 10; }
    8
}
/// $07A0F8: eori.b #$1, $3a(a6)
pub fn i_07a0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0FE;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07A0FE: lea.l $58fc2.l, a1
pub fn i_07a0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A104;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58FC2;
    20
}
/// $07A104: move.l a1, (a6)
pub fn i_07a104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A106;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A106: jsr $49fd0.l
pub fn i_07a106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A10C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A10C);
    cpu.pc = 0x49FD0;
    16
}
/// $07A10C: btst.b #$3, $13(a6)
pub fn i_07a10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A112;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07A112: bne.b $7a0b0
pub fn i_07a112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A114;
    cpu.ir = 0x669C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A0B0; return 10; }
    8
}
/// $07A114: movea.l #$ffffffff, a0
pub fn i_07a114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A11A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A11A: lea.l $2df594.l, a0
pub fn i_07a11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A120;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF594;
    20
}
/// $07A120: jsr $5dd5c.l
pub fn i_07a120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A126;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A126);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A126: bcc.w $7a130
pub fn i_07a126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A12A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A130; return 10; }
    8
}
// ===== SetTaskHandler_07a12a  ($07A12A, 8 B, kernel/task_handlers.c)
/// $07A12A: lea.l $7a3e6(pc), a1
pub fn i_07a12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A12E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3E6;
    16
}
/// $07A12E: move.l a1, (a6)
pub fn i_07a12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A130;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A130: rts
pub fn i_07a130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A132;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hostage_Dying_07a132  ($07A132, 100 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A132: move.w #$8, d1
pub fn i_07a132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A136;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A136: jsr $236e.l
pub fn i_07a136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A13C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A13C);
    cpu.pc = 0x236E;
    16
}
/// $07A13C: lea.l $29c4da.l, a0
pub fn i_07a13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A142;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C4DA;
    20
}
/// $07A142: jsr $28cd4.l
pub fn i_07a142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A148;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A148);
    cpu.pc = 0x28CD4;
    16
}
/// $07A148: lea.l $7a14e(pc), a1
pub fn i_07a148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A14C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A14E;
    16
}
/// $07A14C: move.l a1, (a6)
pub fn i_07a14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A14E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A14E: jsr $2783a.l
pub fn i_07a14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A154);
    cpu.pc = 0x2783A;
    16
}
/// $07A154: jsr $28d70.l
pub fn i_07a154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A15A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A15A);
    cpu.pc = 0x28D70;
    16
}
/// $07A15A: move.b #$1, $44(a6)
pub fn i_07a15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A160;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A160: movea.l $c(a6), a0
pub fn i_07a160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A164;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A164: tst.w $72(a0)
pub fn i_07a164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A168;
    cpu.ir = 0x4A68;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07A168: beq.w $7a170
pub fn i_07a168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A16C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A170; return 10; }
    8
}
/// $07A16C: clr.b $44(a6)
pub fn i_07a16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A170;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x44);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07A170: cmpi.b #$ff, $20(a0)
pub fn i_07a170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A176;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07A176: bne.w $7a180
pub fn i_07a176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A17A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A180; return 10; }
    8
}
/// $07A17A: lea.l $7a36e(pc), a1
pub fn i_07a17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A17E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A36E;
    16
}
/// $07A17E: move.l a1, (a6)
pub fn i_07a17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A180;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A180: movea.l #$ffffffff, a0
pub fn i_07a180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A186;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A186: lea.l $2df594.l, a0
pub fn i_07a186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A18C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF594;
    20
}
/// $07A18C: jsr $5dd5c.l
pub fn i_07a18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A192;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A192);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A192: bcc.w $7a19c
pub fn i_07a192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A196;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A19C; return 10; }
    8
}
// ===== SetTaskHandler_07a196  ($07A196, 8 B, kernel/task_handlers.c)
/// $07A196: lea.l $7a3e6(pc), a1
pub fn i_07a196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A19A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3E6;
    16
}
/// $07A19A: move.l a1, (a6)
pub fn i_07a19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A19C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A19C: rts
pub fn i_07a19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A19E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hostage_Init_07a19e  ($07A19E, 26 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A19E: move.w #$2e, d1
pub fn i_07a19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1A2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A1A2: jsr $236e.l
pub fn i_07a1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A1A8);
    cpu.pc = 0x236E;
    16
}
/// $07A1A8: moveq #$0, d0
pub fn i_07a1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1AA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07A1AA: move.b d0, $20(a6)
pub fn i_07a1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1AE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A1AE: move.b d0, $21(a6)
pub fn i_07a1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1B2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A1B2: move.w #$1, $66(a6)
pub fn i_07a1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_07a1b8  ($07A1B8, 8 B, kernel/task_handlers.c)
/// $07A1B8: lea.l $7a1c0(pc), a1
pub fn i_07a1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A1C0;
    16
}
/// $07A1BC: move.l a1, (a6)
pub fn i_07a1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A1BE: rts
pub fn i_07a1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Captor_Idle_07a1c0  ($07A1C0, 180 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A1C0: jsr $5e7c0.l
pub fn i_07a1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A1C6);
    cpu.pc = 0x5E7C0;
    16
}
/// $07A1C6: move.w #$8000, d0
pub fn i_07a1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1CA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A1CA: jsr $28134.l
pub fn i_07a1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A1D0);
    cpu.pc = 0x28134;
    16
}
/// $07A1D0: andi.w #$ffe3, $38(a6)
pub fn i_07a1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1D6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A1D6: ori.w #$c, $38(a6)
pub fn i_07a1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1DC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A1DC: addq.w #$1, $38(a6)
pub fn i_07a1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1E0;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07A1E0: lea.l $7a132(pc), a1
pub fn i_07a1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A132;
    16
}
/// $07A1E4: jsr $4ae.l
pub fn i_07a1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A1EA);
    cpu.pc = 0x4AE;
    16
}
/// $07A1EA: jsr $5dd02.l
pub fn i_07a1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A1F0);
    cpu.pc = 0x5DD02;
    16
}
/// $07A1F0: bra.w $7a20e
pub fn i_07a1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A20E;
    10
}
/// $07A1F4: movea.l $c(a6), a0
pub fn i_07a1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1F8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A1F8: tst.w $70(a0)
pub fn i_07a1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1FC;
    cpu.ir = 0x4A68;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07A1FC: beq.w $7a20e
pub fn i_07a1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A200;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A20E; return 10; }
    8
}
/// $07A200: movea.l $c(a0), a0
pub fn i_07a200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A204;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A204: movea.l $c(a0), a0
pub fn i_07a204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A208;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A208: move.b #$2, $21(a0)
pub fn i_07a208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A20E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A20E: lea.l $29c3c4.l, a0
pub fn i_07a20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A214;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C3C4;
    20
}
/// $07A214: jsr $28cd4.l
pub fn i_07a214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A21A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A21A);
    cpu.pc = 0x28CD4;
    16
}
/// $07A21A: lea.l $7a220(pc), a1
pub fn i_07a21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A21E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A220;
    16
}
/// $07A21E: move.l a1, (a6)
pub fn i_07a21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A220;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A220: jsr $2783a.l
pub fn i_07a220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A226);
    cpu.pc = 0x2783A;
    16
}
/// $07A226: jsr $28d70.l
pub fn i_07a226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A22C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A22C);
    cpu.pc = 0x28D70;
    16
}
/// $07A22C: movea.l $c(a6), a0
pub fn i_07a22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A230;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A230: move.w $72(a0), d0
pub fn i_07a230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A234;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A234: cmpi.w #$1, d0
pub fn i_07a234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A238;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $07A238: beq.w $7a244
pub fn i_07a238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A23C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A244; return 10; }
    8
}
/// $07A23C: cmpi.w #$2, d0
pub fn i_07a23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A240;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $07A240: bne.w $7a24e
pub fn i_07a240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A244;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A24E; return 10; }
    8
}
/// $07A244: move.w d0, $72(a6)
pub fn i_07a244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A248;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A248: lea.l $7a27c(pc), a1
pub fn i_07a248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A24C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A27C;
    16
}
/// $07A24C: move.l a1, (a6)
pub fn i_07a24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A24E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A24E: jsr $2870a.l
pub fn i_07a24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A254;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A254);
    cpu.pc = 0x2870A;
    16
}
/// $07A254: bcc.w $7a25e
pub fn i_07a254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A258;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A25E; return 10; }
    8
}
/// $07A258: lea.l $7a338(pc), a1
pub fn i_07a258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A25C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A338;
    16
}
/// $07A25C: move.l a1, (a6)
pub fn i_07a25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A25E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A25E: movea.l #$ffffffff, a0
pub fn i_07a25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A264;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A264: lea.l $2df59c.l, a0
pub fn i_07a264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A26A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF59C;
    20
}
/// $07A26A: jsr $5dd5c.l
pub fn i_07a26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A270);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A270: bcc.w $7a27a
pub fn i_07a270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A274;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A27A; return 10; }
    8
}
// ===== SetTaskHandler_07a274  ($07A274, 8 B, kernel/task_handlers.c)
/// $07A274: lea.l $7a3ee(pc), a1
pub fn i_07a274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A278;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3EE;
    16
}
/// $07A278: move.l a1, (a6)
pub fn i_07a278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A27A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A27A: rts
pub fn i_07a27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A27C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Captor_Taunt_07a27c  ($07A27C, 130 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A27C: movea.l $c(a6), a0
pub fn i_07a27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A280;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A280: tst.w $70(a0)
pub fn i_07a280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A284;
    cpu.ir = 0x4A68;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07A284: beq.w $7a296
pub fn i_07a284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A288;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A296; return 10; }
    8
}
/// $07A288: movea.l $c(a0), a0
pub fn i_07a288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A28C;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A28C: movea.l $c(a0), a0
pub fn i_07a28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A290;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A290: move.b #$1, $21(a0)
pub fn i_07a290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A296;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A296: cmpi.w #$1, $72(a6)
pub fn i_07a296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A29C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $07A29C: beq.w $7a2b0
pub fn i_07a29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A2B0; return 10; }
    8
}
/// $07A2A0: lea.l $29c3f8.l, a0
pub fn i_07a2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C3F8;
    20
}
/// $07A2A6: jsr $28cd4.l
pub fn i_07a2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A2AC);
    cpu.pc = 0x28CD4;
    16
}
/// $07A2AC: bra.w $7a2bc
pub fn i_07a2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A2BC;
    10
}
/// $07A2B0: lea.l $29c48c.l, a0
pub fn i_07a2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C48C;
    20
}
/// $07A2B6: jsr $28cd4.l
pub fn i_07a2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A2BC);
    cpu.pc = 0x28CD4;
    16
}
/// $07A2BC: lea.l $7a2c2(pc), a1
pub fn i_07a2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A2C2;
    16
}
/// $07A2C0: move.l a1, (a6)
pub fn i_07a2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A2C2: jsr $2783a.l
pub fn i_07a2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A2C8);
    cpu.pc = 0x2783A;
    16
}
/// $07A2C8: jsr $28d70.l
pub fn i_07a2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A2CE);
    cpu.pc = 0x28D70;
    16
}
/// $07A2CE: bcc.w $7a2d8
pub fn i_07a2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A2D8; return 10; }
    8
}
/// $07A2D2: lea.l $7a1f4(pc), a1
pub fn i_07a2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A1F4;
    16
}
/// $07A2D6: move.l a1, (a6)
pub fn i_07a2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A2D8: jsr $2870a.l
pub fn i_07a2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A2DE);
    cpu.pc = 0x2870A;
    16
}
/// $07A2DE: bcc.w $7a2e8
pub fn i_07a2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A2E8; return 10; }
    8
}
/// $07A2E2: lea.l $7a338(pc), a1
pub fn i_07a2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A338;
    16
}
/// $07A2E6: move.l a1, (a6)
pub fn i_07a2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A2E8: movea.l #$ffffffff, a0
pub fn i_07a2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2EE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A2EE: lea.l $2df59c.l, a0
pub fn i_07a2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF59C;
    20
}
/// $07A2F4: jsr $5dd5c.l
pub fn i_07a2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A2FA);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A2FA: bcc.w $7a304
pub fn i_07a2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A304; return 10; }
    8
}
// ===== SetTaskHandler_07a2fe  ($07A2FE, 8 B, kernel/task_handlers.c)
/// $07A2FE: lea.l $7a3ee(pc), a1
pub fn i_07a2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A302;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3EE;
    16
}
/// $07A302: move.l a1, (a6)
pub fn i_07a302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A304;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A304: rts
pub fn i_07a304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A306;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Captor_ToHut_07a306  ($07A306, 50 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A306: eori.b #$1, $3a(a6)
pub fn i_07a306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A30C;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07A30C: lea.l $29bd40.l, a0
pub fn i_07a30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A312;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BD40;
    20
}
/// $07A312: jsr $28cd4.l
pub fn i_07a312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A318;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A318);
    cpu.pc = 0x28CD4;
    16
}
/// $07A318: lea.l $7a31e(pc), a1
pub fn i_07a318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A31C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A31E;
    16
}
/// $07A31C: move.l a1, (a6)
pub fn i_07a31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A31E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A31E: jsr $2783a.l
pub fn i_07a31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A324;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A324);
    cpu.pc = 0x2783A;
    16
}
/// $07A324: jsr $28d70.l
pub fn i_07a324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A32A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A32A);
    cpu.pc = 0x28D70;
    16
}
/// $07A32A: bcc.w $7a336
pub fn i_07a32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A32E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A336; return 10; }
    8
}
/// $07A32E: lea.l $5fa00.l, a1
pub fn i_07a32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A334;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x5FA00;
    20
}
/// $07A334: move.l a1, (a6)
pub fn i_07a334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A336;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A336: rts
pub fn i_07a336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A338;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Captor_Death_07a338  ($07A338, 54 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A338: move.b #$ff, $20(a6)
pub fn i_07a338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A33E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A33E: movea.l $c(a6), a0
pub fn i_07a33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A342;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A342: move.b #$ff, $21(a0)
pub fn i_07a342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A348;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A348: tst.w $70(a0)
pub fn i_07a348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A34C;
    cpu.ir = 0x4A68;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07A34C: beq.w $7a35e
pub fn i_07a34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A350;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A35E; return 10; }
    8
}
/// $07A350: movea.l $c(a0), a0
pub fn i_07a350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A354;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A354: movea.l $c(a0), a0
pub fn i_07a354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A358;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A358: move.b #$ff, $80(a0)
pub fn i_07a358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A35E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A35E: move.w #$109f, d0
pub fn i_07a35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A362;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A362: jsr $2352.l
pub fn i_07a362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A368;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A368);
    cpu.pc = 0x2352;
    16
}
/// $07A368: jmp $77f6a.l
pub fn i_07a368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A36E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Crew_Hostage_FadeOut_07a36e  ($07A36E, 78 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A36E: clr.b $44(a6)
pub fn i_07a36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A372;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x44);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07A372: move.b #$14, $59(a6)
pub fn i_07a372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A378;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A378: move.w #$14, $74(a6)
pub fn i_07a378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A37E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A37E: lea.l $7a384(pc), a1
pub fn i_07a37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A382;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A384;
    16
}
/// $07A382: move.l a1, (a6)
pub fn i_07a382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A384;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A384: jsr $2783a.l
pub fn i_07a384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A38A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A38A);
    cpu.pc = 0x2783A;
    16
}
/// $07A38A: jsr $28d70.l
pub fn i_07a38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A390;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A390);
    cpu.pc = 0x28D70;
    16
}
/// $07A390: movea.l #$ffffffff, a0
pub fn i_07a390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A396;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A396: lea.l $2df594.l, a0
pub fn i_07a396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A39C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF594;
    20
}
/// $07A39C: jsr $5dd5c.l
pub fn i_07a39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A3A2);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A3A2: bcc.w $7a3ac
pub fn i_07a3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3A6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A3AC; return 10; }
    8
}
/// $07A3A6: lea.l $7a3e6(pc), a1
pub fn i_07a3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3E6;
    16
}
/// $07A3AA: move.l a1, (a6)
pub fn i_07a3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A3AC: subq.w #$1, $74(a6)
pub fn i_07a3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3B0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07A3B0: bne.w $7a3ba
pub fn i_07a3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3B4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A3BA; return 10; }
    8
}
/// $07A3B4: jmp $518.l
pub fn i_07a3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3BA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07A3BA: rts
pub fn i_07a3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hostage_WaitOffWorld_07a3bc  ($07A3BC, 34 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A3BC: jsr $2783a.l
pub fn i_07a3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A3C2);
    cpu.pc = 0x2783A;
    16
}
/// $07A3C2: jsr $28d70.l
pub fn i_07a3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A3C8);
    cpu.pc = 0x28D70;
    16
}
/// $07A3C8: movea.l #$ffffffff, a0
pub fn i_07a3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3CE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A3CE: lea.l $2df594.l, a0
pub fn i_07a3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF594;
    20
}
/// $07A3D4: jsr $5dd5c.l
pub fn i_07a3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A3DA);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A3DA: bcc.w $7a3e4
pub fn i_07a3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A3E4; return 10; }
    8
}
// ===== SetTaskHandler_07a3de  ($07A3DE, 8 B, kernel/task_handlers.c)
/// $07A3DE: lea.l $7a3e6(pc), a1
pub fn i_07a3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3E6;
    16
}
/// $07A3E2: move.l a1, (a6)
pub fn i_07a3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A3E4: rts
pub fn i_07a3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_07a3e6  ($07A3E6, 8 B, kernel/jmp_to_scheduler.c)
/// $07A3E6: jmp $518.l
pub fn i_07a3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3EC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07A3EC: rts
pub fn i_07a3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Captor_MarkParentDead_07a3ee  ($07A3EE, 32 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A3EE: movea.l $c(a6), a0
pub fn i_07a3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3F2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A3F2: move.b #$ff, $21(a0)
pub fn i_07a3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3F8;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A3F8: tst.w $70(a0)
pub fn i_07a3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3FC;
    cpu.ir = 0x4A68;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07A3FC: beq.w $7a40e
pub fn i_07a3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A400;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A40E; return 10; }
    8
}
/// $07A400: movea.l $c(a0), a0
pub fn i_07a400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A404;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A404: movea.l $c(a0), a0
pub fn i_07a404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A408;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A408: move.b #$ff, $80(a0)
pub fn i_07a408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A40E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Jsr5B6ThenJmpScheduler_07a40e  ($07A40E, 14 B, kernel/jsr5b6_jmp518.c)
/// $07A40E: jsr $5b6.l
pub fn i_07a40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A414;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A414);
    cpu.pc = 0x5B6;
    16
}
/// $07A414: jmp $518.l
pub fn i_07a414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A41A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07A41A: rts
pub fn i_07a41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A41C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Captor_PickRandCmd_07a41c  ($07A41C, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A41C: jsr $5e9b6.l
pub fn i_07a41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A422);
    cpu.pc = 0x5E9B6;
    16
}
/// $07A422: andi.w #$3, d0
pub fn i_07a422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A426;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07A426: add.w d0, d0
pub fn i_07a426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A428;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A428: lea.l $2df4ea.l, a0
pub fn i_07a428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A42E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF4EA;
    20
}
/// $07A42E: movea.l $c(a6), a1
pub fn i_07a42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A432;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07A432: move.w (a0, d0.w), $72(a1)
pub fn i_07a432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A438;
    cpu.ir = 0x3370;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    22
}
/// $07A438: rts
pub fn i_07a438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A43A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_RetGateD_07a43a  ($07A43A, 16 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A43A: movea.l $8(a6), a1
pub fn i_07a43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A43E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07A43E: move.b $10(a6), d0
pub fn i_07a43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A442;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07A442: cmp.b $10(a1), d0
pub fn i_07a442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A446;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07A446: bcs.w $7a450
pub fn i_07a446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A44A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7A450; return 10; }
    8
}
// ===== ClearXN_07a44a  ($07A44A, 6 B, thunks/ccr_helpers.c)
/// $07A44A: andi.b #$ee, ccr
pub fn i_07a44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A44E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07A44E: rts
pub fn i_07a44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A450;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07a450  ($07A450, 6 B, thunks/ccr_helpers.c)
/// $07A450: ori.b #$11, ccr
pub fn i_07a450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A454;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07A454: rts
pub fn i_07a454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A456;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SoundTest_Init_07a456  ($07A456, 160 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A456: movea.l #$7072, a1
pub fn i_07a456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A45C;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7072;
    cpu.a[1] = v1;
    4
}
/// $07A45C: lea.l $7a7fa(pc), a2
pub fn i_07a45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A460;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x7A7FA;
    16
}
/// $07A460: move.w #$2300, d0
pub fn i_07a460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A464;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A464: jsr $5dad8.l
pub fn i_07a464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A46A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A46A);
    cpu.pc = 0x5DAD8;
    16
}
/// $07A46A: movea.l #$72d2, a1
pub fn i_07a46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A470;
    cpu.ir = 0x227C;
    let v1: u32 = 0x72D2;
    cpu.a[1] = v1;
    4
}
/// $07A470: lea.l $7a88e(pc), a2
pub fn i_07a470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A474;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x7A88E;
    16
}
/// $07A474: move.w #$2300, d0
pub fn i_07a474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A478;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A478: jsr $5dad8.l
pub fn i_07a478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A47E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A47E);
    cpu.pc = 0x5DAD8;
    16
}
/// $07A47E: move.w #$20, $70(a6)
pub fn i_07a47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A484;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A484: move.w #$20, $72(a6)
pub fn i_07a484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A48A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A48A: move.w #$20, $74(a6)
pub fn i_07a48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A490;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A490: move.w #$20, $76(a6)
pub fn i_07a490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A496;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A496: move.w #$0, $78(a6)
pub fn i_07a496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A49C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A49C: move.w #$0, $7a(a6)
pub fn i_07a49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4A2: move.w #$0, $7c(a6)
pub fn i_07a4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4A8: move.w #$0, $7e(a6)
pub fn i_07a4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4AE: move.w #$0, $80(a6)
pub fn i_07a4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4B4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4B4: lea.l $433be.l, a1
pub fn i_07a4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4BA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x433BE;
    20
}
/// $07A4BA: jsr $4ae.l
pub fn i_07a4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A4C0);
    cpu.pc = 0x4AE;
    16
}
/// $07A4C0: move.w #$f0, $22(a0)
pub fn i_07a4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4C6;
    cpu.ir = 0x317C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4C6: move.w #$180, $24(a0)
pub fn i_07a4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4CC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4CC: move.b #$ff, $98(a0)
pub fn i_07a4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4D2;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A4D2: lea.l $433be.l, a1
pub fn i_07a4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4D8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x433BE;
    20
}
/// $07A4D8: jsr $4ae.l
pub fn i_07a4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A4DE);
    cpu.pc = 0x4AE;
    16
}
/// $07A4DE: move.w #$50, $22(a0)
pub fn i_07a4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4E4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4E4: move.w #$180, $24(a0)
pub fn i_07a4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4EA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4EA: move.b #$0, $98(a0)
pub fn i_07a4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4F0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A4F0: move.l #$7a4f6, (a6)
pub fn i_07a4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4F6;
    cpu.ir = 0x2CBC;
    let v1: u32 = 0x7A4F6;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== SoundTest_Run_07a4f6  ($07A4F6, 746 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A4F6: movea.l #$71e8, a1
pub fn i_07a4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4FC;
    cpu.ir = 0x227C;
    let v1: u32 = 0x71E8;
    cpu.a[1] = v1;
    4
}
/// $07A4FC: lea.l $7a7e8(pc), a2
pub fn i_07a4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A500;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x7A7E8;
    16
}
/// $07A500: move.w #$2300, d0
pub fn i_07a500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A504;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A504: jsr $5dad8.l
pub fn i_07a504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A50A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A50A);
    cpu.pc = 0x5DAD8;
    16
}
/// $07A50A: move.b $10e203.l, d0
pub fn i_07a50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A510;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E203) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07A510: btst.b #$6, d0
pub fn i_07a510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A514;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A514: beq.w $7a526
pub fn i_07a514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A518;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A526; return 10; }
    8
}
/// $07A518: move.b #$5, d0
pub fn i_07a518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A51C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07A51C: jsr $2152.l
pub fn i_07a51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A522;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A522);
    cpu.pc = 0x2152;
    16
}
/// $07A522: bra.w $7a614
pub fn i_07a522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A526;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A614;
    10
}
/// $07A526: btst.b #$7, d0
pub fn i_07a526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A52A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A52A: beq.w $7a53c
pub fn i_07a52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A52E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A53C; return 10; }
    8
}
/// $07A52E: move.b #$6, d0
pub fn i_07a52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A532;
    cpu.ir = 0x103C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07A532: jsr $2152.l
pub fn i_07a532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A538;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A538);
    cpu.pc = 0x2152;
    16
}
/// $07A538: bra.w $7a614
pub fn i_07a538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A53C;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A614;
    10
}
/// $07A53C: btst.b #$0, d0
pub fn i_07a53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A540;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A540: beq.w $7a562
pub fn i_07a540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A544;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A562; return 10; }
    8
}
/// $07A544: move.w $80(a6), d1
pub fn i_07a544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A548;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A548: add.w d1, d1
pub fn i_07a548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A54A;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A54A: addq.w #$1, $70(a6, d1.w)
pub fn i_07a54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A54E;
    cpu.ir = 0x5276;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $07A54E: cmpi.w #$ff, $70(a6, d1.w)
pub fn i_07a54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A554;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Word, false);
    18
}
/// $07A554: ble.w $7a5e0
pub fn i_07a554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A558;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7A5E0; return 10; }
    8
}
/// $07A558: move.w #$20, $70(a6, d1.w)
pub fn i_07a558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A55E;
    cpu.ir = 0x3DBC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $07A55E: bra.w $7a5e0
pub fn i_07a55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A562;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A5E0;
    10
}
/// $07A562: btst.b #$1, d0
pub fn i_07a562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A566;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A566: beq.w $7a588
pub fn i_07a566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A56A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A588; return 10; }
    8
}
/// $07A56A: move.w $80(a6), d1
pub fn i_07a56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A56E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A56E: add.w d1, d1
pub fn i_07a56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A570;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A570: subq.w #$1, $70(a6, d1.w)
pub fn i_07a570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A574;
    cpu.ir = 0x5376;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $07A574: cmpi.w #$20, $70(a6, d1.w)
pub fn i_07a574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A57A;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    18
}
/// $07A57A: bge.w $7a5e0
pub fn i_07a57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A57E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x7A5E0; return 10; }
    8
}
/// $07A57E: move.w #$ff, $70(a6, d1.w)
pub fn i_07a57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A584;
    cpu.ir = 0x3DBC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $07A584: bra.w $7a5e0
pub fn i_07a584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A588;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A5E0;
    10
}
/// $07A588: btst.b #$2, d0
pub fn i_07a588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A58C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A58C: beq.w $7a5b6
pub fn i_07a58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A590;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A5B6; return 10; }
    8
}
/// $07A590: move.w $80(a6), d1
pub fn i_07a590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A594;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A594: add.w d1, d1
pub fn i_07a594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A596;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A596: subi.w #$10, $70(a6, d1.w)
pub fn i_07a596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A59C;
    cpu.ir = 0x476;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    18
}
/// $07A59C: cmpi.w #$20, $70(a6, d1.w)
pub fn i_07a59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5A2;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    18
}
/// $07A5A2: bge.w $7a5e0
pub fn i_07a5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5A6;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x7A5E0; return 10; }
    8
}
/// $07A5A6: andi.w #$f, $70(a6, d1.w)
pub fn i_07a5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5AC;
    cpu.ir = 0x276;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    18
}
/// $07A5AC: addi.w #$f0, $70(a6, d1.w)
pub fn i_07a5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5B2;
    cpu.ir = 0x676;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xF0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    18
}
/// $07A5B2: bra.w $7a5e0
pub fn i_07a5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A5E0;
    10
}
/// $07A5B6: btst.b #$3, d0
pub fn i_07a5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5BA;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A5BA: beq.w $7a5e0
pub fn i_07a5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A5E0; return 10; }
    8
}
/// $07A5BE: move.w $80(a6), d1
pub fn i_07a5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5C2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A5C2: add.w d1, d1
pub fn i_07a5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5C4;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A5C4: addi.w #$10, $70(a6, d1.w)
pub fn i_07a5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5CA;
    cpu.ir = 0x676;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    18
}
/// $07A5CA: cmpi.w #$ff, $70(a6, d1.w)
pub fn i_07a5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5D0;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Word, false);
    18
}
/// $07A5D0: ble.w $7a5e0
pub fn i_07a5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5D4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7A5E0; return 10; }
    8
}
/// $07A5D4: andi.w #$f, $70(a6, d1.w)
pub fn i_07a5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5DA;
    cpu.ir = 0x276;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    18
}
/// $07A5DA: addi.w #$20, $70(a6, d1.w)
pub fn i_07a5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5E0;
    cpu.ir = 0x676;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    18
}
/// $07A5E0: btst.b #$5, d0
pub fn i_07a5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5E4;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A5E4: beq.w $7a602
pub fn i_07a5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A602; return 10; }
    8
}
/// $07A5E8: move.w $80(a6), d1
pub fn i_07a5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5EC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A5EC: add.w d1, d1
pub fn i_07a5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5EE;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A5EE: addq.w #$1, $78(a6, d1.w)
pub fn i_07a5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5F2;
    cpu.ir = 0x5276;
    let ad1 = cpu.a[6].wrapping_add(0x78).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $07A5F2: cmpi.w #$3, $78(a6, d1.w)
pub fn i_07a5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5F8;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x78).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    18
}
/// $07A5F8: blt.w $7a602
pub fn i_07a5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5FC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7A602; return 10; }
    8
}
/// $07A5FC: move.w #$0, $78(a6, d1.w)
pub fn i_07a5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A602;
    cpu.ir = 0x3DBC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $07A602: btst.b #$4, d0
pub fn i_07a602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A606;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A606: beq.w $7a614
pub fn i_07a606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A60A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A614; return 10; }
    8
}
/// $07A60A: addq.w #$1, $80(a6)
pub fn i_07a60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A60E;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07A60E: andi.w #$3, $80(a6)
pub fn i_07a60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A614;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A614: move.b $10e209.l, d0
pub fn i_07a614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A61A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E209) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07A61A: btst.b #$0, d0
pub fn i_07a61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A61E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A61E: beq.w $7a630
pub fn i_07a61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A622;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A630; return 10; }
    8
}
/// $07A622: subq.w #$1, $80(a6)
pub fn i_07a622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A626;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07A626: andi.w #$3, $80(a6)
pub fn i_07a626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A62C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A62C: bra.w $7a6c4
pub fn i_07a62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A630;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A6C4;
    10
}
/// $07A630: btst.b #$1, d0
pub fn i_07a630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A634;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A634: beq.w $7a646
pub fn i_07a634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A638;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A646; return 10; }
    8
}
/// $07A638: addq.w #$1, $80(a6)
pub fn i_07a638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A63C;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07A63C: andi.w #$3, $80(a6)
pub fn i_07a63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A642;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A642: bra.w $7a6c4
pub fn i_07a642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A646;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A6C4;
    10
}
/// $07A646: move.b $10e20a.l, d0
pub fn i_07a646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A64C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E20A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07A64C: btst.b #$3, d0
pub fn i_07a64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A650;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A650: beq.w $7a672
pub fn i_07a650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A654;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A672; return 10; }
    8
}
/// $07A654: move.w $80(a6), d1
pub fn i_07a654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A658;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A658: add.w d1, d1
pub fn i_07a658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A65A;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A65A: addq.w #$1, $70(a6, d1.w)
pub fn i_07a65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A65E;
    cpu.ir = 0x5276;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $07A65E: cmpi.w #$ff, $70(a6, d1.w)
pub fn i_07a65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A664;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Word, false);
    18
}
/// $07A664: ble.w $7a6c4
pub fn i_07a664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A668;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7A6C4; return 10; }
    8
}
/// $07A668: move.w #$20, $70(a6, d1.w)
pub fn i_07a668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A66E;
    cpu.ir = 0x3DBC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $07A66E: bra.w $7a6c4
pub fn i_07a66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A672;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A6C4;
    10
}
/// $07A672: btst.b #$2, d0
pub fn i_07a672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A676;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A676: beq.w $7a698
pub fn i_07a676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A67A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A698; return 10; }
    8
}
/// $07A67A: move.w $80(a6), d1
pub fn i_07a67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A67E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A67E: add.w d1, d1
pub fn i_07a67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A680;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A680: subq.w #$1, $70(a6, d1.w)
pub fn i_07a680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A684;
    cpu.ir = 0x5376;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $07A684: cmpi.w #$20, $70(a6, d1.w)
pub fn i_07a684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A68A;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    18
}
/// $07A68A: bge.w $7a6c4
pub fn i_07a68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A68E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x7A6C4; return 10; }
    8
}
/// $07A68E: move.w #$ff, $70(a6, d1.w)
pub fn i_07a68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A694;
    cpu.ir = 0x3DBC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $07A694: bra.w $7a6c4
pub fn i_07a694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A698;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A6C4;
    10
}
/// $07A698: move.b $10e209.l, d0
pub fn i_07a698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A69E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E209) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07A69E: btst.b #$4, d0
pub fn i_07a69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6A2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A6A2: beq.w $7a6c4
pub fn i_07a6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A6C4; return 10; }
    8
}
/// $07A6A6: move.l d0, -(a7)
pub fn i_07a6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6A8;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07A6A8: move.w $70(a6), d0
pub fn i_07a6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6AC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A6AC: move.w $78(a6), d1
pub fn i_07a6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6B0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A6B0: lsl.w #$1, d1
pub fn i_07a6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6B2;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A6B2: lea.l $7a7f4(pc), a1
pub fn i_07a6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A6B6: move.w (a1, d1.w), d1
pub fn i_07a6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6BA;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A6BA: add.w d1, d0
pub fn i_07a6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6BC;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A6BC: jsr $219c.l
pub fn i_07a6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A6C2);
    cpu.pc = 0x219C;
    16
}
/// $07A6C2: move.l (a7)+, d0
pub fn i_07a6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6C4;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A6C4: btst.b #$5, d0
pub fn i_07a6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6C8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A6C8: beq.w $7a6ea
pub fn i_07a6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A6EA; return 10; }
    8
}
/// $07A6CC: move.l d0, -(a7)
pub fn i_07a6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6CE;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07A6CE: move.w $72(a6), d0
pub fn i_07a6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6D2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A6D2: move.w $7a(a6), d1
pub fn i_07a6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6D6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A6D6: lsl.w #$1, d1
pub fn i_07a6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6D8;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A6D8: lea.l $7a7f4(pc), a1
pub fn i_07a6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A6DC: move.w (a1, d1.w), d1
pub fn i_07a6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6E0;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A6E0: add.w d1, d0
pub fn i_07a6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6E2;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A6E2: jsr $219c.l
pub fn i_07a6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A6E8);
    cpu.pc = 0x219C;
    16
}
/// $07A6E8: move.l (a7)+, d0
pub fn i_07a6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6EA;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A6EA: btst.b #$6, d0
pub fn i_07a6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6EE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A6EE: beq.w $7a710
pub fn i_07a6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A710; return 10; }
    8
}
/// $07A6F2: move.l d0, -(a7)
pub fn i_07a6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6F4;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07A6F4: move.w $74(a6), d0
pub fn i_07a6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6F8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A6F8: move.w $7c(a6), d1
pub fn i_07a6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6FC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A6FC: lsl.w #$1, d1
pub fn i_07a6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6FE;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A6FE: lea.l $7a7f4(pc), a1
pub fn i_07a6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A702;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A702: move.w (a1, d1.w), d1
pub fn i_07a702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A706;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A706: add.w d1, d0
pub fn i_07a706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A708;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A708: jsr $219c.l
pub fn i_07a708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A70E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A70E);
    cpu.pc = 0x219C;
    16
}
/// $07A70E: move.l (a7)+, d0
pub fn i_07a70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A710;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A710: btst.b #$7, d0
pub fn i_07a710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A714;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A714: beq.w $7a736
pub fn i_07a714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A718;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A736; return 10; }
    8
}
/// $07A718: move.l d0, -(a7)
pub fn i_07a718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A71A;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07A71A: move.w $76(a6), d0
pub fn i_07a71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A71E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A71E: move.w $7e(a6), d1
pub fn i_07a71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A722;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A722: lsl.w #$1, d1
pub fn i_07a722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A724;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A724: lea.l $7a7f4(pc), a1
pub fn i_07a724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A728;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A728: move.w (a1, d1.w), d1
pub fn i_07a728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A72C;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A72C: add.w d1, d0
pub fn i_07a72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A72E;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A72E: jsr $219c.l
pub fn i_07a72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A734;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A734);
    cpu.pc = 0x219C;
    16
}
/// $07A734: move.l (a7)+, d0
pub fn i_07a734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A736;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A736: move.w $70(a6), d0
pub fn i_07a736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A73A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A73A: move.w $78(a6), d1
pub fn i_07a73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A73E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A73E: lsl.w #$1, d1
pub fn i_07a73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A740;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A740: lea.l $7a7f4(pc), a1
pub fn i_07a740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A744;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A744: move.w (a1, d1.w), d1
pub fn i_07a744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A748;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A748: add.w d1, d0
pub fn i_07a748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A74A;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A74A: movea.l #$724a, a1
pub fn i_07a74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A750;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724A;
    cpu.a[1] = v1;
    4
}
/// $07A750: move.w #$2300, d1
pub fn i_07a750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A754;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A754: jsr $5d6c2.l
pub fn i_07a754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A75A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A75A);
    cpu.pc = 0x5D6C2;
    16
}
/// $07A75A: move.w $72(a6), d0
pub fn i_07a75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A75E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A75E: move.w $7a(a6), d1
pub fn i_07a75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A762;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A762: lsl.w #$1, d1
pub fn i_07a762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A764;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A764: lea.l $7a7f4(pc), a1
pub fn i_07a764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A768;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A768: move.w (a1, d1.w), d1
pub fn i_07a768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A76C;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A76C: add.w d1, d0
pub fn i_07a76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A76E;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A76E: movea.l #$724c, a1
pub fn i_07a76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A774;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724C;
    cpu.a[1] = v1;
    4
}
/// $07A774: move.w #$2300, d1
pub fn i_07a774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A778;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A778: jsr $5d6c2.l
pub fn i_07a778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A77E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A77E);
    cpu.pc = 0x5D6C2;
    16
}
/// $07A77E: move.w $74(a6), d0
pub fn i_07a77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A782;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A782: move.w $7c(a6), d1
pub fn i_07a782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A786;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A786: lsl.w #$1, d1
pub fn i_07a786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A788;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A788: lea.l $7a7f4(pc), a1
pub fn i_07a788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A78C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A78C: move.w (a1, d1.w), d1
pub fn i_07a78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A790;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A790: add.w d1, d0
pub fn i_07a790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A792;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A792: movea.l #$724e, a1
pub fn i_07a792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A798;
    cpu.ir = 0x227C;
    let v1: u32 = 0x724E;
    cpu.a[1] = v1;
    4
}
/// $07A798: move.w #$2300, d1
pub fn i_07a798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A79C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A79C: jsr $5d6c2.l
pub fn i_07a79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A7A2);
    cpu.pc = 0x5D6C2;
    16
}
/// $07A7A2: move.w $76(a6), d0
pub fn i_07a7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7A6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A7A6: move.w $7e(a6), d1
pub fn i_07a7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7AA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A7AA: lsl.w #$1, d1
pub fn i_07a7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7AC;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A7AC: lea.l $7a7f4(pc), a1
pub fn i_07a7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A7B0: move.w (a1, d1.w), d1
pub fn i_07a7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7B4;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A7B4: add.w d1, d0
pub fn i_07a7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7B6;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A7B6: movea.l #$7250, a1
pub fn i_07a7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7BC;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7250;
    cpu.a[1] = v1;
    4
}
/// $07A7BC: move.w #$2300, d1
pub fn i_07a7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A7C0: jsr $5d6c2.l
pub fn i_07a7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A7C6);
    cpu.pc = 0x5D6C2;
    16
}
/// $07A7C6: lea.l $7a914(pc), a0
pub fn i_07a7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7CA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x7A914;
    16
}
/// $07A7CA: move.w $80(a6), d0
pub fn i_07a7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7CE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A7CE: add.w d0, d0
pub fn i_07a7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7D0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A7D0: add.w d0, d0
pub fn i_07a7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7D2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A7D2: movea.l (a0, d0.w), a2
pub fn i_07a7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7D6;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $07A7D6: movea.l #$720a, a1
pub fn i_07a7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7DC;
    cpu.ir = 0x227C;
    let v1: u32 = 0x720A;
    cpu.a[1] = v1;
    4
}
/// $07A7DC: move.w #$2300, d0
pub fn i_07a7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7E0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_07a7e0  ($07A7E0, 8 B, thunks/jsr_abs_thunks.c)
/// $07A7E0: jsr $5dad8.l
pub fn i_07a7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A7E6);
    cpu.pc = 0x5DAD8;
    16
}
/// $07A7E6: rts
pub fn i_07a7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A7E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Banner_RetGate_07a954  ($07A954, 16 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A954: movea.l $8(a6), a1
pub fn i_07a954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A958;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07A958: move.b $10(a6), d0
pub fn i_07a958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A95C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07A95C: cmp.b $10(a1), d0
pub fn i_07a95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A960;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07A960: bcs.w $7a96a
pub fn i_07a960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A964;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7A96A; return 10; }
    8
}
// ===== ClearXN_07a964  ($07A964, 6 B, thunks/ccr_helpers.c)
/// $07A964: andi.b #$ee, ccr
pub fn i_07a964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A968;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07A968: rts
pub fn i_07a968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A96A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07a96a  ($07A96A, 6 B, thunks/ccr_helpers.c)
/// $07A96A: ori.b #$11, ccr
pub fn i_07a96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A96E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07A96E: rts
pub fn i_07a96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A970;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerStart_Boot_07A970  ($07A970, 120 B, mission/banner_07a970.s)
/// $07A970: jsr $4707e.l
pub fn i_07a970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A976);
    cpu.pc = 0x4707E;
    16
}
/// $07A976: move.b #$0, $7b(a6)
pub fn i_07a976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A97C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A97C: move.w #$0, $22(a6)
pub fn i_07a97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A982;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A982: move.w #$1ff, $24(a6)
pub fn i_07a982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A988;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A988: move.w #$ffff, $38(a6)
pub fn i_07a988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A98E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A98E: move.w #$0, $74(a6)
pub fn i_07a98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A994;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A994: move.b $106ece.l, d0
pub fn i_07a994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A99A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07A99A: cmpi.b #$5, d0
pub fn i_07a99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A99E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07A99E: bne.w $7a9aa
pub fn i_07a99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A9AA; return 10; }
    8
}
/// $07A9A2: jsr $7af50(pc)
pub fn i_07a9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7A9A6);
    cpu.pc = 0x7AF50;
    16
}
/// $07A9A6: bra.w $7a9ae
pub fn i_07a9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A9AE;
    10
}
/// $07A9AA: jsr $7acd0(pc)
pub fn i_07a9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7A9AE);
    cpu.pc = 0x7ACD0;
    16
}
/// $07A9AE: move.b #$0, $21(a6)
pub fn i_07a9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A9B4: move.b #$0, $20(a6)
pub fn i_07a9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9BA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A9BA: lea.l $7a9c0(pc), a1
pub fn i_07a9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A9C0;
    16
}
/// $07A9BE: move.l a1, (a6)
pub fn i_07a9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A9C0: move.b $106ece.l, d0
pub fn i_07a9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9C6;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07A9C6: cmpi.b #$5, d0
pub fn i_07a9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9CA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07A9CA: bne.w $7a9d6
pub fn i_07a9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A9D6; return 10; }
    8
}
/// $07A9CE: move.b #$12, d0
pub fn i_07a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9D2;
    cpu.ir = 0x103C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07A9D2: bra.w $7a9da
pub fn i_07a9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A9DA;
    10
}
/// $07A9D6: move.b #$e, d0
pub fn i_07a9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9DA;
    cpu.ir = 0x103C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07A9DA: cmp.b $21(a6), d0
pub fn i_07a9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9DE;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07A9DE: bne.w $7a9ee
pub fn i_07a9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A9EE; return 10; }
    8
}
/// $07A9E2: jsr $4709e.l
pub fn i_07a9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A9E8);
    cpu.pc = 0x4709E;
    16
}
// ===== SetTaskHandler_07a9e8  ($07A9E8, 8 B, kernel/task_handlers.c)
/// $07A9E8: lea.l $7aa78(pc), a1
pub fn i_07a9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AA78;
    16
}
/// $07A9EC: move.l a1, (a6)
pub fn i_07a9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A9EE: rts
pub fn i_07a9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerComplete_Boot_07A9F0  ($07A9F0, 128 B, mission/banner_07a970.s)
/// $07A9F0: move.w #$20, d0
pub fn i_07a9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9F4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A9F4: jsr $2352.l
pub fn i_07a9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A9FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A9FA);
    cpu.pc = 0x2352;
    16
}
/// $07A9FA: move.b #$1, $7b(a6)
pub fn i_07a9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA00;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AA00: move.w #$0, $22(a6)
pub fn i_07aa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA06;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AA06: move.w #$1ff, $24(a6)
pub fn i_07aa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA0C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AA0C: move.w #$ffff, $38(a6)
pub fn i_07aa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA12;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AA12: move.w #$0, $74(a6)
pub fn i_07aa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA18;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AA18: move.b $106ece.l, d0
pub fn i_07aa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA1E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07AA1E: cmpi.b #$5, d0
pub fn i_07aa1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA22;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07AA22: bne.w $7aa2e
pub fn i_07aa22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA26;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7AA2E; return 10; }
    8
}
/// $07AA26: jsr $7b598(pc)
pub fn i_07aa26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7AA2A);
    cpu.pc = 0x7B598;
    16
}
/// $07AA2A: bra.w $7aa32
pub fn i_07aa2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x7AA32;
    10
}
/// $07AA2E: jsr $7b28e(pc)
pub fn i_07aa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7AA32);
    cpu.pc = 0x7B28E;
    16
}
/// $07AA32: move.b #$0, $21(a6)
pub fn i_07aa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA38;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AA38: move.b #$0, $20(a6)
pub fn i_07aa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA3E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AA3E: lea.l $7aa44(pc), a1
pub fn i_07aa3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AA44;
    16
}
/// $07AA42: move.l a1, (a6)
pub fn i_07aa42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AA44: move.b $106ece.l, d0
pub fn i_07aa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA4A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07AA4A: cmpi.b #$5, d0
pub fn i_07aa4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA4E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07AA4E: bne.w $7aa5a
pub fn i_07aa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7AA5A; return 10; }
    8
}
/// $07AA52: move.b #$15, d0
pub fn i_07aa52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA56;
    cpu.ir = 0x103C;
    let v1: u32 = 0x15;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07AA56: bra.w $7aa5e
pub fn i_07aa56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x7AA5E;
    10
}
/// $07AA5A: move.b #$11, d0
pub fn i_07aa5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA5E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07AA5E: cmp.b $21(a6), d0
pub fn i_07aa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA62;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07AA62: bne.w $7aa76
pub fn i_07aa62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA66;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7AA76; return 10; }
    8
}
/// $07AA66: move.w #$1137, d0
pub fn i_07aa66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA6A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1137;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07AA6A: jsr $2352.l
pub fn i_07aa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AA70);
    cpu.pc = 0x2352;
    16
}
// ===== SetTaskHandler_07aa70  ($07AA70, 8 B, kernel/task_handlers.c)
/// $07AA70: lea.l $7aa78(pc), a1
pub fn i_07aa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AA78;
    16
}
/// $07AA74: move.l a1, (a6)
pub fn i_07aa74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AA76: rts
pub fn i_07aa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Banner_Wait26_07AA78  ($07AA78, 20 B, mission/banner_07a970.s)
/// $07AA78: lea.l $7aa7e(pc), a1
pub fn i_07aa78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AA7E;
    16
}
/// $07AA7C: move.l a1, (a6)
pub fn i_07aa7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AA7E: addq.w #$1, $74(a6)
pub fn i_07aa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA82;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07AA82: cmpi.w #$26, $74(a6)
pub fn i_07aa82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA88;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Word, false);
    16
}
/// $07AA88: blt.w $7aa92
pub fn i_07aa88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA8C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7AA92; return 10; }
    8
}
// ===== SetTaskHandler_07aa8c  ($07AA8C, 8 B, kernel/task_handlers.c)
/// $07AA8C: lea.l $7aa94(pc), a1
pub fn i_07aa8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AA94;
    16
}
/// $07AA90: move.l a1, (a6)
pub fn i_07aa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AA92: rts
pub fn i_07aa92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Banner_Close_07AA94  ($07AA94, 76 B, mission/banner_07a970.s)
/// $07AA94: move.b #$ff, $20(a6)
pub fn i_07aa94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AA9A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AA9A: cmpi.b #$0, $7b(a6)
pub fn i_07aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAA0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $07AAA0: bne.w $7aab4
pub fn i_07aaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7AAB4; return 10; }
    8
}
/// $07AAA4: movea.l $c(a6), a1
pub fn i_07aaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAA8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07AAA8: clr.b $21(a1)
pub fn i_07aaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAAC;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07AAAC: jmp $518.l
pub fn i_07aaac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAB2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07AAB2: rts
pub fn i_07aab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07AAB4: move.w #$2, d0
pub fn i_07aab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAB8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07AAB8: jsr $5239e.l
pub fn i_07aab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AABE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AABE);
    cpu.pc = 0x5239E;
    16
}
/// $07AABE: move.w #$28, $74(a6)
pub fn i_07aabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAC4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AAC4: lea.l $7aaca(pc), a1
pub fn i_07aac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AACA;
    16
}
/// $07AAC8: move.l a1, (a6)
pub fn i_07aac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AACA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AACA: subq.w #$1, $74(a6)
pub fn i_07aaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AACE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07AACE: bpl.w $7aade
pub fn i_07aace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAD2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7AADE; return 10; }
    8
}
/// $07AAD2: clr.b $106ed2.l
pub fn i_07aad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAD8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $07AAD8: jmp $518.l
pub fn i_07aad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AADE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07AADE: rts
pub fn i_07aade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLetter_Drop_07AAE0  ($07AAE0, 124 B, mission/banner_07a970.s)
/// $07AAE0: move.w $74(a6), $76(a6)
pub fn i_07aae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAE6;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07AAE6: lea.l $7aaec(pc), a1
pub fn i_07aae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAEC;
    16
}
/// $07AAEA: move.l a1, (a6)
pub fn i_07aaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AAEC: subq.w #$1, $74(a6)
pub fn i_07aaec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAF0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07AAF0: bmi.w $7aaf6
pub fn i_07aaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAF4;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x7AAF6; return 10; }
    8
}
/// $07AAF4: rts
pub fn i_07aaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07AAF6: move.w #$1be, d1
pub fn i_07aaf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AAFA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1BE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07AAFA: jsr $236e.l
pub fn i_07aafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AB00);
    cpu.pc = 0x236E;
    16
}
/// $07AB00: ori.b #$1, $3a(a6)
pub fn i_07ab00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB06;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07AB06: move.w #$1ff, $78(a6)
pub fn i_07ab06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB0C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AB0C: move.b #$ff, $33(a6)
pub fn i_07ab0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AB12: move.b #$0, $7a(a6)
pub fn i_07ab12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB18;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AB18: move.w #$ffff, $38(a6)
pub fn i_07ab18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AB1E: jsr $267e2.l
pub fn i_07ab1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AB24);
    cpu.pc = 0x267E2;
    16
}
/// $07AB24: clr.l d0
pub fn i_07ab24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB26;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $07AB26: move.b $20(a6), d0
pub fn i_07ab26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB2A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07AB2A: cmpi.b #$d, d0
pub fn i_07ab2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB2E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xD, Size::Byte, false);
    8
}
/// $07AB2E: bls.w $7ab36
pub fn i_07ab2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB32;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x7AB36; return 10; }
    8
}
/// $07AB32: move.b #$0, d0
pub fn i_07ab32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB36;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07AB36: movea.l #$2df684, a0
pub fn i_07ab36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB3C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2DF684;
    cpu.a[0] = v1;
    4
}
/// $07AB3C: lsl.w #$2, d0
pub fn i_07ab3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB3E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07AB3E: movea.l (a0, d0.w), a0
pub fn i_07ab3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB42;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07AB42: cmpa.l #$ffffffff, a0
pub fn i_07ab42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB48;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $07AB48: beq.w $7ab52
pub fn i_07ab48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7AB52; return 10; }
    8
}
/// $07AB4C: jsr $28cd4.l
pub fn i_07ab4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AB52);
    cpu.pc = 0x28CD4;
    16
}
/// $07AB52: lea.l $7abd0(pc), a1
pub fn i_07ab52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7ABD0;
    16
}
/// $07AB56: move.l a1, (a6)
pub fn i_07ab56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AB58: bra.w $7abd0
pub fn i_07ab58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x7ABD0;
    10
}
// ===== BannerDigit_Drop_07AB5C  ($07AB5C, 116 B, mission/banner_07a970.s)
/// $07AB5C: move.w $74(a6), $76(a6)
pub fn i_07ab5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB62;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07AB62: lea.l $7ab68(pc), a1
pub fn i_07ab62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AB68;
    16
}
/// $07AB66: move.l a1, (a6)
pub fn i_07ab66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB68;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AB68: subq.w #$1, $74(a6)
pub fn i_07ab68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB6C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07AB6C: bmi.w $7ab72
pub fn i_07ab6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB70;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x7AB72; return 10; }
    8
}
/// $07AB70: rts
pub fn i_07ab70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07AB72: move.w #$1be, d1
pub fn i_07ab72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB76;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1BE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07AB76: jsr $236e.l
pub fn i_07ab76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AB7C);
    cpu.pc = 0x236E;
    16
}
/// $07AB7C: ori.b #$1, $3a(a6)
pub fn i_07ab7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB82;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07AB82: move.w #$1ff, $78(a6)
pub fn i_07ab82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB88;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AB88: move.b #$ff, $33(a6)
pub fn i_07ab88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB8E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AB8E: move.b #$0, $7a(a6)
pub fn i_07ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB94;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AB94: jsr $267e2.l
pub fn i_07ab94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AB9A);
    cpu.pc = 0x267E2;
    16
}
/// $07AB9A: clr.l d0
pub fn i_07ab9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AB9C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $07AB9C: move.b $106ece.l, d0
pub fn i_07ab9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABA2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07ABA2: cmpi.b #$5, d0
pub fn i_07aba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABA6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07ABA6: bls.w $7abae
pub fn i_07aba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABAA;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x7ABAE; return 10; }
    8
}
/// $07ABAA: move.b #$0, d0
pub fn i_07abaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABAE;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07ABAE: movea.l #$2df71c, a0
pub fn i_07abae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABB4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2DF71C;
    cpu.a[0] = v1;
    4
}
/// $07ABB4: lsl.w #$2, d0
pub fn i_07abb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABB6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07ABB6: movea.l (a0, d0.w), a0
pub fn i_07abb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABBA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07ABBA: cmpa.l #$ffffffff, a0
pub fn i_07abba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABC0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $07ABC0: beq.w $7abca
pub fn i_07abc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABC4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7ABCA; return 10; }
    8
}
/// $07ABC4: jsr $28cd4.l
pub fn i_07abc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ABCA);
    cpu.pc = 0x28CD4;
    16
}
/// $07ABCA: lea.l $7abd0(pc), a1
pub fn i_07abca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7ABD0;
    16
}
/// $07ABCE: move.l a1, (a6)
pub fn i_07abce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== BannerLetter_Tick_07ABD0  ($07ABD0, 28 B, mission/banner_07a970.s)
/// $07ABD0: jsr $7b960(pc)
pub fn i_07abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7ABD4);
    cpu.pc = 0x7B960;
    16
}
/// $07ABD4: jsr $28d70.l
pub fn i_07abd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ABDA);
    cpu.pc = 0x28D70;
    16
}
/// $07ABDA: jsr $7b9a4(pc)
pub fn i_07abda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7ABDE);
    cpu.pc = 0x7B9A4;
    16
}
/// $07ABDE: bcc.w $7abf2
pub fn i_07abde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7ABF2; return 10; }
    8
}
/// $07ABE2: move.w #$0, d1
pub fn i_07abe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABE6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07ABE6: jsr $236e.l
pub fn i_07abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ABEC);
    cpu.pc = 0x236E;
    16
}
// ===== SetTaskHandler_07abec  ($07ABEC, 8 B, kernel/task_handlers.c)
/// $07ABEC: lea.l $7abf4(pc), a1
pub fn i_07abec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7ABF4;
    16
}
/// $07ABF0: move.l a1, (a6)
pub fn i_07abf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07ABF2: rts
pub fn i_07abf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Banner_Attach_07ABF4  ($07ABF4, 78 B, mission/banner_07a970.s)
/// $07ABF4: move.w $18(a6), $14(a6)
pub fn i_07abf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABFA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07ABFA: movea.l $c(a6), a0
pub fn i_07abfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ABFE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07ABFE: addq.b #$1, $21(a0)
pub fn i_07abfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC02;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $07AC02: move.b #$ff, $33(a6)
pub fn i_07ac02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC08;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AC08: move.b #$ff, $32(a6)
pub fn i_07ac08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC0E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AC0E: lea.l $7ac14(pc), a1
pub fn i_07ac0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AC14;
    16
}
/// $07AC12: move.l a1, (a6)
pub fn i_07ac12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AC14: cmpi.b #$0, $21(a6)
pub fn i_07ac14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC1A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $07AC1A: beq.w $7ac2e
pub fn i_07ac1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC1E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7AC2E; return 10; }
    8
}
/// $07AC1E: movea.l $c(a6), a0
pub fn i_07ac1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC22;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07AC22: move.w $74(a0), d0
pub fn i_07ac22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC26;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07AC26: btst.b #$3, d0
pub fn i_07ac26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC2A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07AC2A: bne.w $7ac34
pub fn i_07ac2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7AC34; return 10; }
    8
}
/// $07AC2E: jsr $28d70.l
pub fn i_07ac2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AC34);
    cpu.pc = 0x28D70;
    16
}
/// $07AC34: movea.l $c(a6), a0
pub fn i_07ac34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC38;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07AC38: cmpi.b #$ff, $20(a0)
pub fn i_07ac38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC3E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07AC3E: bne.w $7ac48
pub fn i_07ac3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC42;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7AC48; return 10; }
    8
}
// ===== SetTaskHandler_07ac42  ($07AC42, 8 B, kernel/task_handlers.c)
/// $07AC42: lea.l $7ac4a(pc), a1
pub fn i_07ac42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AC4A;
    16
}
/// $07AC46: move.l a1, (a6)
pub fn i_07ac46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AC48: rts
pub fn i_07ac48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLetter_FlyOff_07AC4A  ($07AC4A, 134 B, mission/banner_07a970.s)
/// $07AC4A: move.w #$0, $36(a6)
pub fn i_07ac4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AC50: lea.l $7ac56(pc), a1
pub fn i_07ac50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AC56;
    16
}
/// $07AC54: move.l a1, (a6)
pub fn i_07ac54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07AC56: addi.w #$1ff, $36(a6)
pub fn i_07ac56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC5C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1FF, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07AC5C: clr.l d0
pub fn i_07ac5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC5E;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $07AC5E: move.w $36(a6), d0
pub fn i_07ac5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC62;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07AC62: btst.b #$f, d0
pub fn i_07ac62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC66;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07AC66: beq.w $7ac70
pub fn i_07ac66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC6A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7AC70; return 10; }
    8
}
/// $07AC6A: move.w #$7fff, $36(a6)
pub fn i_07ac6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AC70: move.w #$a0, d0
pub fn i_07ac70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC74;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07AC74: move.w #$190, d1
pub fn i_07ac74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC78;
    cpu.ir = 0x323C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07AC78: sub.w $22(a6), d0
pub fn i_07ac78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC7C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07AC7C: sub.w $24(a6), d1
pub fn i_07ac7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC80;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07AC80: jsr $5e018.l
pub fn i_07ac80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AC86);
    cpu.pc = 0x5E018;
    16
}
/// $07AC86: addi.b #$7f, d0
pub fn i_07ac86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC8A;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x7F, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $07AC8A: andi.w #$ff, d0
pub fn i_07ac8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC8E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07AC8E: move.w $36(a6), d1
pub fn i_07ac8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC92;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07AC92: jsr $13c0e.l
pub fn i_07ac92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AC98);
    cpu.pc = 0x13C0E;
    16
}
/// $07AC98: move.w d1, $28(a6)
pub fn i_07ac98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AC9C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AC9C: move.w d2, $2a(a6)
pub fn i_07ac9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACA0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ACA0: bset.b #$6, $13(a6)
pub fn i_07aca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACA6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07ACA6: jsr $27cee.l
pub fn i_07aca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ACAC);
    cpu.pc = 0x27CEE;
    16
}
/// $07ACAC: jsr $28d70.l
pub fn i_07acac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ACB2);
    cpu.pc = 0x28D70;
    16
}
/// $07ACB2: movea.l #$ffffffff, a0
pub fn i_07acb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACB8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07ACB8: lea.l $2df734.l, a0
pub fn i_07acb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACBE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF734;
    20
}
/// $07ACBE: jsr $5dd5c.l
pub fn i_07acbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ACC4);
    cpu.pc = 0x5DD5C;
    16
}
/// $07ACC4: bcc.w $7acce
pub fn i_07acc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7ACCE; return 10; }
    8
}
/// $07ACC8: jmp $518.l
pub fn i_07acc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACCE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07ACCE: rts
pub fn i_07acce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLayout_Start_07ACD0  ($07ACD0, 640 B, mission/banner_07a970.s)
/// $07ACD0: lea.l $7aae0(pc), a1
pub fn i_07acd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07ACD4: jsr $4ae.l
pub fn i_07acd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ACDA);
    cpu.pc = 0x4AE;
    16
}
/// $07ACDA: jsr $5dd02.l
pub fn i_07acda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ACE0);
    cpu.pc = 0x5DD02;
    16
}
/// $07ACE0: move.b #$c, $20(a0)
pub fn i_07ace0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACE6;
    cpu.ir = 0x117C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ACE6: move.b #$1, $21(a0)
pub fn i_07ace6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACEC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ACEC: move.w #$18, $74(a0)
pub fn i_07acec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACF2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ACF2: move.w #$da, $70(a0)
pub fn i_07acf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACF8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xDA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ACF8: move.w #$178, $72(a0)
pub fn i_07acf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ACFE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ACFE: lea.l $7aae0(pc), a1
pub fn i_07acfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AD02: jsr $4ae.l
pub fn i_07ad02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD08);
    cpu.pc = 0x4AE;
    16
}
/// $07AD08: jsr $5dd02.l
pub fn i_07ad08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD0E);
    cpu.pc = 0x5DD02;
    16
}
/// $07AD0E: move.b #$b, $20(a0)
pub fn i_07ad0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD14;
    cpu.ir = 0x117C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD14: move.b #$1, $21(a0)
pub fn i_07ad14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD1A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD1A: move.w #$16, $74(a0)
pub fn i_07ad1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD20;
    cpu.ir = 0x317C;
    let v1: u32 = 0x16;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD20: move.w #$c4, $70(a0)
pub fn i_07ad20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD26;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD26: move.w #$178, $72(a0)
pub fn i_07ad26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD2C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD2C: lea.l $7aae0(pc), a1
pub fn i_07ad2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AD30: jsr $4ae.l
pub fn i_07ad30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD36);
    cpu.pc = 0x4AE;
    16
}
/// $07AD36: jsr $5dd02.l
pub fn i_07ad36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD3C);
    cpu.pc = 0x5DD02;
    16
}
/// $07AD3C: move.b #$6, $20(a0)
pub fn i_07ad3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD42;
    cpu.ir = 0x117C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD42: move.b #$1, $21(a0)
pub fn i_07ad42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD48;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD48: move.w #$14, $74(a0)
pub fn i_07ad48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD4E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD4E: move.w #$ad, $70(a0)
pub fn i_07ad4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD54;
    cpu.ir = 0x317C;
    let v1: u32 = 0xAD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD54: move.w #$178, $72(a0)
pub fn i_07ad54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD5A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD5A: lea.l $7aae0(pc), a1
pub fn i_07ad5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AD5E: jsr $4ae.l
pub fn i_07ad5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD64);
    cpu.pc = 0x4AE;
    16
}
/// $07AD64: jsr $5dd02.l
pub fn i_07ad64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD6A);
    cpu.pc = 0x5DD02;
    16
}
/// $07AD6A: move.b #$5, $20(a0)
pub fn i_07ad6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD70;
    cpu.ir = 0x117C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD70: move.b #$1, $21(a0)
pub fn i_07ad70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD76;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD76: move.w #$12, $74(a0)
pub fn i_07ad76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD7C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD7C: move.w #$94, $70(a0)
pub fn i_07ad7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD82;
    cpu.ir = 0x317C;
    let v1: u32 = 0x94;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD82: move.w #$178, $72(a0)
pub fn i_07ad82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD88;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AD88: lea.l $7aae0(pc), a1
pub fn i_07ad88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AD8C: jsr $4ae.l
pub fn i_07ad8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD92);
    cpu.pc = 0x4AE;
    16
}
/// $07AD92: jsr $5dd02.l
pub fn i_07ad92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AD98);
    cpu.pc = 0x5DD02;
    16
}
/// $07AD98: move.b #$b, $20(a0)
pub fn i_07ad98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AD9E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AD9E: move.b #$1, $21(a0)
pub fn i_07ad9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADA4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ADA4: move.w #$10, $74(a0)
pub fn i_07ada4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADAA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ADAA: move.w #$7e, $70(a0)
pub fn i_07adaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADB0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x7E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ADB0: move.w #$178, $72(a0)
pub fn i_07adb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADB6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ADB6: lea.l $7aae0(pc), a1
pub fn i_07adb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07ADBA: jsr $4ae.l
pub fn i_07adba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ADC0);
    cpu.pc = 0x4AE;
    16
}
/// $07ADC0: jsr $5dd02.l
pub fn i_07adc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ADC6);
    cpu.pc = 0x5DD02;
    16
}
/// $07ADC6: move.b #$2, $20(a0)
pub fn i_07adc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADCC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ADCC: move.b #$1, $21(a0)
pub fn i_07adcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADD2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ADD2: move.w #$e, $74(a0)
pub fn i_07add2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADD8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ADD8: move.w #$64, $70(a0)
pub fn i_07add8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADDE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ADDE: move.w #$178, $72(a0)
pub fn i_07adde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADE4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ADE4: lea.l $7ab5c(pc), a1
pub fn i_07ade4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AB5C;
    16
}
/// $07ADE8: jsr $4ae.l
pub fn i_07ade8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ADEE);
    cpu.pc = 0x4AE;
    16
}
/// $07ADEE: jsr $5dd02.l
pub fn i_07adee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ADF4);
    cpu.pc = 0x5DD02;
    16
}
/// $07ADF4: move.b #$0, $21(a0)
pub fn i_07adf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ADFA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ADFA: move.w #$e, $74(a0)
pub fn i_07adfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE00;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE00: move.w #$f0, $70(a0)
pub fn i_07ae00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE06;
    cpu.ir = 0x317C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE06: move.w #$1a8, $72(a0)
pub fn i_07ae06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE0C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE0C: lea.l $7aae0(pc), a1
pub fn i_07ae0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AE10: jsr $4ae.l
pub fn i_07ae10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AE16);
    cpu.pc = 0x4AE;
    16
}
/// $07AE16: jsr $5dd02.l
pub fn i_07ae16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AE1C);
    cpu.pc = 0x5DD02;
    16
}
/// $07AE1C: move.b #$4, $20(a0)
pub fn i_07ae1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE22;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AE22: move.b #$0, $21(a0)
pub fn i_07ae22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE28;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AE28: move.w #$c, $74(a0)
pub fn i_07ae28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE2E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE2E: move.w #$d0, $70(a0)
pub fn i_07ae2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE34;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE34: move.w #$1a8, $72(a0)
pub fn i_07ae34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE3A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE3A: lea.l $7aae0(pc), a1
pub fn i_07ae3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AE3E: jsr $4ae.l
pub fn i_07ae3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AE44);
    cpu.pc = 0x4AE;
    16
}
/// $07AE44: jsr $5dd02.l
pub fn i_07ae44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AE4A);
    cpu.pc = 0x5DD02;
    16
}
/// $07AE4A: move.b #$3, $20(a0)
pub fn i_07ae4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE50;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AE50: move.b #$0, $21(a0)
pub fn i_07ae50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE56;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AE56: move.w #$a, $74(a0)
pub fn i_07ae56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE5C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE5C: move.w #$b7, $70(a0)
pub fn i_07ae5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE62;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE62: move.w #$1a8, $72(a0)
pub fn i_07ae62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE68;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE68: lea.l $7aae0(pc), a1
pub fn i_07ae68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AE6C: jsr $4ae.l
pub fn i_07ae6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AE72);
    cpu.pc = 0x4AE;
    16
}
/// $07AE72: jsr $5dd02.l
pub fn i_07ae72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AE78);
    cpu.pc = 0x5DD02;
    16
}
/// $07AE78: move.b #$1, $20(a0)
pub fn i_07ae78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE7E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AE7E: move.b #$0, $21(a0)
pub fn i_07ae7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE84;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AE84: move.w #$8, $74(a0)
pub fn i_07ae84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE8A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE8A: move.w #$a4, $70(a0)
pub fn i_07ae8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE90;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE90: move.w #$1a8, $72(a0)
pub fn i_07ae90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE96;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AE96: lea.l $7aae0(pc), a1
pub fn i_07ae96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AE9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AE9A: jsr $4ae.l
pub fn i_07ae9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AEA0);
    cpu.pc = 0x4AE;
    16
}
/// $07AEA0: jsr $5dd02.l
pub fn i_07aea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AEA6);
    cpu.pc = 0x5DD02;
    16
}
/// $07AEA6: move.b #$2, $20(a0)
pub fn i_07aea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEAC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AEAC: move.b #$0, $21(a0)
pub fn i_07aeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEB2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AEB2: move.w #$6, $74(a0)
pub fn i_07aeb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEB8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AEB8: move.w #$90, $70(a0)
pub fn i_07aeb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEBE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AEBE: move.w #$1a8, $72(a0)
pub fn i_07aebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEC4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AEC4: lea.l $7aae0(pc), a1
pub fn i_07aec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AEC8: jsr $4ae.l
pub fn i_07aec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AECE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AECE);
    cpu.pc = 0x4AE;
    16
}
/// $07AECE: jsr $5dd02.l
pub fn i_07aece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AED4);
    cpu.pc = 0x5DD02;
    16
}
/// $07AED4: move.b #$2, $20(a0)
pub fn i_07aed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEDA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AEDA: move.b #$0, $21(a0)
pub fn i_07aeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEE0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AEE0: move.w #$4, $74(a0)
pub fn i_07aee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEE6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AEE6: move.w #$77, $70(a0)
pub fn i_07aee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEEC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AEEC: move.w #$1a8, $72(a0)
pub fn i_07aeec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEF2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AEF2: lea.l $7aae0(pc), a1
pub fn i_07aef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AEF6: jsr $4ae.l
pub fn i_07aef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AEFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AEFC);
    cpu.pc = 0x4AE;
    16
}
/// $07AEFC: jsr $5dd02.l
pub fn i_07aefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF02);
    cpu.pc = 0x5DD02;
    16
}
/// $07AF02: move.b #$1, $20(a0)
pub fn i_07af02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF08;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF08: move.b #$0, $21(a0)
pub fn i_07af08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF0E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF0E: move.w #$2, $74(a0)
pub fn i_07af0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF14;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF14: move.w #$64, $70(a0)
pub fn i_07af14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF1A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF1A: move.w #$1a8, $72(a0)
pub fn i_07af1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF20;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF20: lea.l $7aae0(pc), a1
pub fn i_07af20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AF24: jsr $4ae.l
pub fn i_07af24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF2A);
    cpu.pc = 0x4AE;
    16
}
/// $07AF2A: jsr $5dd02.l
pub fn i_07af2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF30);
    cpu.pc = 0x5DD02;
    16
}
/// $07AF30: move.b #$0, $20(a0)
pub fn i_07af30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF36;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF36: move.b #$0, $21(a0)
pub fn i_07af36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF3C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF3C: move.w #$0, $74(a0)
pub fn i_07af3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF42;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF42: move.w #$50, $70(a0)
pub fn i_07af42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF48;
    cpu.ir = 0x317C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF48: move.w #$1a8, $72(a0)
pub fn i_07af48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF4E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF4E: rts
pub fn i_07af4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLayout_StartFinal_07AF50  ($07AF50, 830 B, mission/banner_07a970.s)
/// $07AF50: lea.l $7aae0(pc), a1
pub fn i_07af50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AF54: jsr $4ae.l
pub fn i_07af54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF5A);
    cpu.pc = 0x4AE;
    16
}
/// $07AF5A: jsr $5dd02.l
pub fn i_07af5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF60);
    cpu.pc = 0x5DD02;
    16
}
/// $07AF60: move.b #$c, $20(a0)
pub fn i_07af60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF66;
    cpu.ir = 0x117C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF66: move.b #$1, $21(a0)
pub fn i_07af66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF6C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF6C: move.w #$22, $74(a0)
pub fn i_07af6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF72;
    cpu.ir = 0x317C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF72: move.w #$da, $70(a0)
pub fn i_07af72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF78;
    cpu.ir = 0x317C;
    let v1: u32 = 0xDA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF78: move.w #$168, $72(a0)
pub fn i_07af78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF7E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AF7E: lea.l $7aae0(pc), a1
pub fn i_07af7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AF82: jsr $4ae.l
pub fn i_07af82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF88);
    cpu.pc = 0x4AE;
    16
}
/// $07AF88: jsr $5dd02.l
pub fn i_07af88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AF8E);
    cpu.pc = 0x5DD02;
    16
}
/// $07AF8E: move.b #$b, $20(a0)
pub fn i_07af8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF94;
    cpu.ir = 0x117C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF94: move.b #$1, $21(a0)
pub fn i_07af94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AF9A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AF9A: move.w #$20, $74(a0)
pub fn i_07af9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFA0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFA0: move.w #$c6, $70(a0)
pub fn i_07afa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFA6;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFA6: move.w #$168, $72(a0)
pub fn i_07afa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFAC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFAC: lea.l $7aae0(pc), a1
pub fn i_07afac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AFB0: jsr $4ae.l
pub fn i_07afb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AFB6);
    cpu.pc = 0x4AE;
    16
}
/// $07AFB6: jsr $5dd02.l
pub fn i_07afb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AFBC);
    cpu.pc = 0x5DD02;
    16
}
/// $07AFBC: move.b #$6, $20(a0)
pub fn i_07afbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFC2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AFC2: move.b #$1, $21(a0)
pub fn i_07afc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFC8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AFC8: move.w #$1e, $74(a0)
pub fn i_07afc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFCE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFCE: move.w #$af, $70(a0)
pub fn i_07afce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFD4;
    cpu.ir = 0x317C;
    let v1: u32 = 0xAF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFD4: move.w #$168, $72(a0)
pub fn i_07afd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFDA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFDA: lea.l $7aae0(pc), a1
pub fn i_07afda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07AFDE: jsr $4ae.l
pub fn i_07afde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AFE4);
    cpu.pc = 0x4AE;
    16
}
/// $07AFE4: jsr $5dd02.l
pub fn i_07afe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7AFEA);
    cpu.pc = 0x5DD02;
    16
}
/// $07AFEA: move.b #$5, $20(a0)
pub fn i_07afea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFF0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AFF0: move.b #$1, $21(a0)
pub fn i_07aff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFF6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07AFF6: move.w #$1c, $74(a0)
pub fn i_07aff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7AFFC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07AFFC: move.w #$96, $70(a0)
pub fn i_07affc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B002;
    cpu.ir = 0x317C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B002: move.w #$168, $72(a0)
pub fn i_07b002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B008;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B008: lea.l $7aae0(pc), a1
pub fn i_07b008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B00C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B00C: jsr $4ae.l
pub fn i_07b00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B012;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B012);
    cpu.pc = 0x4AE;
    16
}
/// $07B012: jsr $5dd02.l
pub fn i_07b012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B018;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B018);
    cpu.pc = 0x5DD02;
    16
}
/// $07B018: move.b #$b, $20(a0)
pub fn i_07b018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B01E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B01E: move.b #$1, $21(a0)
pub fn i_07b01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B024;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B024: move.w #$1a, $74(a0)
pub fn i_07b024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B02A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B02A: move.w #$80, $70(a0)
pub fn i_07b02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B030;
    cpu.ir = 0x317C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B030: move.w #$168, $72(a0)
pub fn i_07b030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B036;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B036: lea.l $7aae0(pc), a1
pub fn i_07b036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B03A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B03A: jsr $4ae.l
pub fn i_07b03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B040;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B040);
    cpu.pc = 0x4AE;
    16
}
/// $07B040: jsr $5dd02.l
pub fn i_07b040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B046;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B046);
    cpu.pc = 0x5DD02;
    16
}
/// $07B046: move.b #$2, $20(a0)
pub fn i_07b046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B04C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B04C: move.b #$1, $21(a0)
pub fn i_07b04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B052;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B052: move.w #$18, $74(a0)
pub fn i_07b052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B058;
    cpu.ir = 0x317C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B058: move.w #$66, $70(a0)
pub fn i_07b058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B05E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x66;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B05E: move.w #$168, $72(a0)
pub fn i_07b05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B064;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B064: lea.l $7aae0(pc), a1
pub fn i_07b064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B068;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B068: jsr $4ae.l
pub fn i_07b068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B06E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B06E);
    cpu.pc = 0x4AE;
    16
}
/// $07B06E: jsr $5dd02.l
pub fn i_07b06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B074;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B074);
    cpu.pc = 0x5DD02;
    16
}
/// $07B074: move.b #$4, $20(a0)
pub fn i_07b074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B07A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B07A: move.b #$0, $21(a0)
pub fn i_07b07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B080;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B080: move.w #$16, $74(a0)
pub fn i_07b080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B086;
    cpu.ir = 0x317C;
    let v1: u32 = 0x16;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B086: move.w #$e0, $70(a0)
pub fn i_07b086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B08C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B08C: move.w #$198, $72(a0)
pub fn i_07b08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B092;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B092: lea.l $7aae0(pc), a1
pub fn i_07b092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B096;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B096: jsr $4ae.l
pub fn i_07b096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B09C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B09C);
    cpu.pc = 0x4AE;
    16
}
/// $07B09C: jsr $5dd02.l
pub fn i_07b09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B0A2);
    cpu.pc = 0x5DD02;
    16
}
/// $07B0A2: move.b #$3, $20(a0)
pub fn i_07b0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0A8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B0A8: move.b #$0, $21(a0)
pub fn i_07b0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0AE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B0AE: move.w #$14, $74(a0)
pub fn i_07b0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0B4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B0B4: move.w #$c7, $70(a0)
pub fn i_07b0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0BA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B0BA: move.w #$198, $72(a0)
pub fn i_07b0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B0C0: lea.l $7aae0(pc), a1
pub fn i_07b0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B0C4: jsr $4ae.l
pub fn i_07b0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B0CA);
    cpu.pc = 0x4AE;
    16
}
/// $07B0CA: jsr $5dd02.l
pub fn i_07b0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B0D0);
    cpu.pc = 0x5DD02;
    16
}
/// $07B0D0: move.b #$1, $20(a0)
pub fn i_07b0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0D6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B0D6: move.b #$0, $21(a0)
pub fn i_07b0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0DC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B0DC: move.w #$12, $74(a0)
pub fn i_07b0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0E2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B0E2: move.w #$b4, $70(a0)
pub fn i_07b0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0E8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B0E8: move.w #$198, $72(a0)
pub fn i_07b0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0EE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B0EE: lea.l $7aae0(pc), a1
pub fn i_07b0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B0F2: jsr $4ae.l
pub fn i_07b0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B0F8);
    cpu.pc = 0x4AE;
    16
}
/// $07B0F8: jsr $5dd02.l
pub fn i_07b0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B0FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B0FE);
    cpu.pc = 0x5DD02;
    16
}
/// $07B0FE: move.b #$2, $20(a0)
pub fn i_07b0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B104;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B104: move.b #$0, $21(a0)
pub fn i_07b104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B10A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B10A: move.w #$10, $74(a0)
pub fn i_07b10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B110;
    cpu.ir = 0x317C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B110: move.w #$a0, $70(a0)
pub fn i_07b110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B116;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B116: move.w #$198, $72(a0)
pub fn i_07b116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B11C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B11C: lea.l $7aae0(pc), a1
pub fn i_07b11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B120;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B120: jsr $4ae.l
pub fn i_07b120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B126;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B126);
    cpu.pc = 0x4AE;
    16
}
/// $07B126: jsr $5dd02.l
pub fn i_07b126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B12C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B12C);
    cpu.pc = 0x5DD02;
    16
}
/// $07B12C: move.b #$2, $20(a0)
pub fn i_07b12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B132;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B132: move.b #$0, $21(a0)
pub fn i_07b132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B138;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B138: move.w #$e, $74(a0)
pub fn i_07b138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B13E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B13E: move.w #$87, $70(a0)
pub fn i_07b13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B144;
    cpu.ir = 0x317C;
    let v1: u32 = 0x87;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B144: move.w #$198, $72(a0)
pub fn i_07b144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B14A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B14A: lea.l $7aae0(pc), a1
pub fn i_07b14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B14E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B14E: jsr $4ae.l
pub fn i_07b14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B154);
    cpu.pc = 0x4AE;
    16
}
/// $07B154: jsr $5dd02.l
pub fn i_07b154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B15A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B15A);
    cpu.pc = 0x5DD02;
    16
}
/// $07B15A: move.b #$1, $20(a0)
pub fn i_07b15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B160;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B160: move.b #$0, $21(a0)
pub fn i_07b160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B166;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B166: move.w #$c, $74(a0)
pub fn i_07b166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B16C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B16C: move.w #$74, $70(a0)
pub fn i_07b16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B172;
    cpu.ir = 0x317C;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B172: move.w #$198, $72(a0)
pub fn i_07b172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B178;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B178: lea.l $7aae0(pc), a1
pub fn i_07b178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B17C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B17C: jsr $4ae.l
pub fn i_07b17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B182;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B182);
    cpu.pc = 0x4AE;
    16
}
/// $07B182: jsr $5dd02.l
pub fn i_07b182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B188;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B188);
    cpu.pc = 0x5DD02;
    16
}
/// $07B188: move.b #$0, $20(a0)
pub fn i_07b188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B18E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B18E: move.b #$0, $21(a0)
pub fn i_07b18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B194;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B194: move.w #$a, $74(a0)
pub fn i_07b194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B19A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B19A: move.w #$60, $70(a0)
pub fn i_07b19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1A0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1A0: move.w #$198, $72(a0)
pub fn i_07b1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1A6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1A6: lea.l $7aae0(pc), a1
pub fn i_07b1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B1AA: jsr $4ae.l
pub fn i_07b1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B1B0);
    cpu.pc = 0x4AE;
    16
}
/// $07B1B0: jsr $5dd02.l
pub fn i_07b1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B1B6);
    cpu.pc = 0x5DD02;
    16
}
/// $07B1B6: move.b #$9, $20(a0)
pub fn i_07b1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1BC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B1BC: move.b #$0, $21(a0)
pub fn i_07b1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1C2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B1C2: move.w #$8, $74(a0)
pub fn i_07b1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1C8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1C8: move.w #$cd, $70(a0)
pub fn i_07b1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1CE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xCD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1CE: move.w #$1b8, $72(a0)
pub fn i_07b1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1D4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1D4: lea.l $7aae0(pc), a1
pub fn i_07b1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B1D8: jsr $4ae.l
pub fn i_07b1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B1DE);
    cpu.pc = 0x4AE;
    16
}
/// $07B1DE: jsr $5dd02.l
pub fn i_07b1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B1E4);
    cpu.pc = 0x5DD02;
    16
}
/// $07B1E4: move.b #$5, $20(a0)
pub fn i_07b1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1EA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B1EA: move.b #$0, $21(a0)
pub fn i_07b1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1F0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B1F0: move.w #$6, $74(a0)
pub fn i_07b1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1F6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1F6: move.w #$b4, $70(a0)
pub fn i_07b1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B1FC;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B1FC: move.w #$1b8, $72(a0)
pub fn i_07b1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B202;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B202: lea.l $7aae0(pc), a1
pub fn i_07b202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B206;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B206: jsr $4ae.l
pub fn i_07b206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B20C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B20C);
    cpu.pc = 0x4AE;
    16
}
/// $07B20C: jsr $5dd02.l
pub fn i_07b20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B212;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B212);
    cpu.pc = 0x5DD02;
    16
}
/// $07B212: move.b #$4, $20(a0)
pub fn i_07b212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B218;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B218: move.b #$0, $21(a0)
pub fn i_07b218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B21E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B21E: move.w #$4, $74(a0)
pub fn i_07b21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B224;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B224: move.w #$9c, $70(a0)
pub fn i_07b224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B22A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x9C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B22A: move.w #$1b8, $72(a0)
pub fn i_07b22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B230;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B230: lea.l $7aae0(pc), a1
pub fn i_07b230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B234;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B234: jsr $4ae.l
pub fn i_07b234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B23A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B23A);
    cpu.pc = 0x4AE;
    16
}
/// $07B23A: jsr $5dd02.l
pub fn i_07b23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B240);
    cpu.pc = 0x5DD02;
    16
}
/// $07B240: move.b #$1, $20(a0)
pub fn i_07b240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B246;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B246: move.b #$0, $21(a0)
pub fn i_07b246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B24C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B24C: move.w #$2, $74(a0)
pub fn i_07b24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B252;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B252: move.w #$89, $70(a0)
pub fn i_07b252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B258;
    cpu.ir = 0x317C;
    let v1: u32 = 0x89;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B258: move.w #$1b8, $72(a0)
pub fn i_07b258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B25E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B25E: lea.l $7aae0(pc), a1
pub fn i_07b25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B262;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B262: jsr $4ae.l
pub fn i_07b262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B268;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B268);
    cpu.pc = 0x4AE;
    16
}
/// $07B268: jsr $5dd02.l
pub fn i_07b268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B26E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B26E);
    cpu.pc = 0x5DD02;
    16
}
/// $07B26E: move.b #$d, $20(a0)
pub fn i_07b26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B274;
    cpu.ir = 0x117C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B274: move.b #$0, $21(a0)
pub fn i_07b274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B27A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B27A: move.w #$0, $74(a0)
pub fn i_07b27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B280;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B280: move.w #$76, $70(a0)
pub fn i_07b280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B286;
    cpu.ir = 0x317C;
    let v1: u32 = 0x76;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B286: move.w #$1b8, $72(a0)
pub fn i_07b286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B28C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B28C: rts
pub fn i_07b28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B28E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLayout_Complete_07B28E  ($07B28E, 778 B, mission/banner_07a970.s)
/// $07B28E: lea.l $7aae0(pc), a1
pub fn i_07b28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B292;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B292: jsr $4ae.l
pub fn i_07b292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B298);
    cpu.pc = 0x4AE;
    16
}
/// $07B298: jsr $5dd02.l
pub fn i_07b298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B29E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B29E);
    cpu.pc = 0x5DD02;
    16
}
/// $07B29E: move.b #$c, $20(a0)
pub fn i_07b29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2A4;
    cpu.ir = 0x117C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B2A4: move.b #$1, $21(a0)
pub fn i_07b2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2AA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B2AA: move.w #$20, $74(a0)
pub fn i_07b2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2B0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B2B0: move.w #$102, $70(a0)
pub fn i_07b2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2B6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x102;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B2B6: move.w #$178, $72(a0)
pub fn i_07b2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2BC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B2BC: lea.l $7aae0(pc), a1
pub fn i_07b2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B2C0: jsr $4ae.l
pub fn i_07b2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B2C6);
    cpu.pc = 0x4AE;
    16
}
/// $07B2C6: jsr $5dd02.l
pub fn i_07b2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B2CC);
    cpu.pc = 0x5DD02;
    16
}
/// $07B2CC: move.b #$a, $20(a0)
pub fn i_07b2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2D2;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B2D2: move.b #$1, $21(a0)
pub fn i_07b2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2D8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B2D8: move.w #$1e, $74(a0)
pub fn i_07b2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2DE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B2DE: move.w #$ec, $70(a0)
pub fn i_07b2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2E4;
    cpu.ir = 0x317C;
    let v1: u32 = 0xEC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B2E4: move.w #$178, $72(a0)
pub fn i_07b2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2EA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B2EA: lea.l $7aae0(pc), a1
pub fn i_07b2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B2EE: jsr $4ae.l
pub fn i_07b2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B2F4);
    cpu.pc = 0x4AE;
    16
}
/// $07B2F4: jsr $5dd02.l
pub fn i_07b2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B2FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B2FA);
    cpu.pc = 0x5DD02;
    16
}
/// $07B2FA: move.b #$b, $20(a0)
pub fn i_07b2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B300;
    cpu.ir = 0x117C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B300: move.b #$1, $21(a0)
pub fn i_07b300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B306;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B306: move.w #$1c, $74(a0)
pub fn i_07b306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B30C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B30C: move.w #$d4, $70(a0)
pub fn i_07b30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B312;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B312: move.w #$178, $72(a0)
pub fn i_07b312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B318;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B318: lea.l $7aae0(pc), a1
pub fn i_07b318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B31C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B31C: jsr $4ae.l
pub fn i_07b31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B322);
    cpu.pc = 0x4AE;
    16
}
/// $07B322: jsr $5dd02.l
pub fn i_07b322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B328);
    cpu.pc = 0x5DD02;
    16
}
/// $07B328: move.b #$a, $20(a0)
pub fn i_07b328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B32E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B32E: move.b #$1, $21(a0)
pub fn i_07b32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B334;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B334: move.w #$1a, $74(a0)
pub fn i_07b334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B33A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B33A: move.w #$bc, $70(a0)
pub fn i_07b33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B340;
    cpu.ir = 0x317C;
    let v1: u32 = 0xBC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B340: move.w #$178, $72(a0)
pub fn i_07b340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B346;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B346: lea.l $7aae0(pc), a1
pub fn i_07b346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B34A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B34A: jsr $4ae.l
pub fn i_07b34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B350;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B350);
    cpu.pc = 0x4AE;
    16
}
/// $07B350: jsr $5dd02.l
pub fn i_07b350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B356;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B356);
    cpu.pc = 0x5DD02;
    16
}
/// $07B356: move.b #$9, $20(a0)
pub fn i_07b356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B35C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B35C: move.b #$1, $21(a0)
pub fn i_07b35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B362;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B362: move.w #$18, $74(a0)
pub fn i_07b362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B368;
    cpu.ir = 0x317C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B368: move.w #$a5, $70(a0)
pub fn i_07b368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B36E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B36E: move.w #$178, $72(a0)
pub fn i_07b36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B374;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B374: lea.l $7aae0(pc), a1
pub fn i_07b374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B378;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B378: jsr $4ae.l
pub fn i_07b378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B37E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B37E);
    cpu.pc = 0x4AE;
    16
}
/// $07B37E: jsr $5dd02.l
pub fn i_07b37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B384);
    cpu.pc = 0x5DD02;
    16
}
/// $07B384: move.b #$8, $20(a0)
pub fn i_07b384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B38A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B38A: move.b #$1, $21(a0)
pub fn i_07b38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B390;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B390: move.w #$16, $74(a0)
pub fn i_07b390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B396;
    cpu.ir = 0x317C;
    let v1: u32 = 0x16;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B396: move.w #$8d, $70(a0)
pub fn i_07b396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B39C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B39C: move.w #$178, $72(a0)
pub fn i_07b39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3A2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3A2: lea.l $7aae0(pc), a1
pub fn i_07b3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B3A6: jsr $4ae.l
pub fn i_07b3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B3AC);
    cpu.pc = 0x4AE;
    16
}
/// $07B3AC: jsr $5dd02.l
pub fn i_07b3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B3B2);
    cpu.pc = 0x5DD02;
    16
}
/// $07B3B2: move.b #$0, $20(a0)
pub fn i_07b3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3B8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B3B8: move.b #$1, $21(a0)
pub fn i_07b3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3BE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B3BE: move.w #$14, $74(a0)
pub fn i_07b3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3C4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3C4: move.w #$74, $70(a0)
pub fn i_07b3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3CA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3CA: move.w #$178, $72(a0)
pub fn i_07b3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3D0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3D0: lea.l $7aae0(pc), a1
pub fn i_07b3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B3D4: jsr $4ae.l
pub fn i_07b3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B3DA);
    cpu.pc = 0x4AE;
    16
}
/// $07B3DA: jsr $5dd02.l
pub fn i_07b3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B3E0);
    cpu.pc = 0x5DD02;
    16
}
/// $07B3E0: move.b #$3, $20(a0)
pub fn i_07b3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3E6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B3E6: move.b #$1, $21(a0)
pub fn i_07b3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3EC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B3EC: move.w #$12, $74(a0)
pub fn i_07b3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3F2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3F2: move.w #$5b, $70(a0)
pub fn i_07b3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3F8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x5B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3F8: move.w #$178, $72(a0)
pub fn i_07b3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B3FE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B3FE: lea.l $7aae0(pc), a1
pub fn i_07b3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B402;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B402: jsr $4ae.l
pub fn i_07b402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B408);
    cpu.pc = 0x4AE;
    16
}
/// $07B408: jsr $5dd02.l
pub fn i_07b408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B40E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B40E);
    cpu.pc = 0x5DD02;
    16
}
/// $07B40E: move.b #$7, $20(a0)
pub fn i_07b40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B414;
    cpu.ir = 0x117C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B414: move.b #$1, $21(a0)
pub fn i_07b414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B41A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B41A: move.w #$10, $74(a0)
pub fn i_07b41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B420;
    cpu.ir = 0x317C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B420: move.w #$42, $70(a0)
pub fn i_07b420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B426;
    cpu.ir = 0x317C;
    let v1: u32 = 0x42;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B426: move.w #$178, $72(a0)
pub fn i_07b426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B42C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B42C: lea.l $7ab5c(pc), a1
pub fn i_07b42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B430;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AB5C;
    16
}
/// $07B430: jsr $4ae.l
pub fn i_07b430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B436);
    cpu.pc = 0x4AE;
    16
}
/// $07B436: jsr $5dd02.l
pub fn i_07b436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B43C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B43C);
    cpu.pc = 0x5DD02;
    16
}
/// $07B43C: move.b #$0, $21(a0)
pub fn i_07b43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B442;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B442: move.w #$e, $74(a0)
pub fn i_07b442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B448;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B448: move.w #$f0, $70(a0)
pub fn i_07b448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B44E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B44E: move.w #$1a8, $72(a0)
pub fn i_07b44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B454;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B454: lea.l $7aae0(pc), a1
pub fn i_07b454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B458;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B458: jsr $4ae.l
pub fn i_07b458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B45E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B45E);
    cpu.pc = 0x4AE;
    16
}
/// $07B45E: jsr $5dd02.l
pub fn i_07b45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B464;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B464);
    cpu.pc = 0x5DD02;
    16
}
/// $07B464: move.b #$4, $20(a0)
pub fn i_07b464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B46A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B46A: move.b #$0, $21(a0)
pub fn i_07b46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B470;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B470: move.w #$c, $74(a0)
pub fn i_07b470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B476;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B476: move.w #$d0, $70(a0)
pub fn i_07b476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B47C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B47C: move.w #$1a8, $72(a0)
pub fn i_07b47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B482;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B482: lea.l $7aae0(pc), a1
pub fn i_07b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B486;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B486: jsr $4ae.l
pub fn i_07b486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B48C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B48C);
    cpu.pc = 0x4AE;
    16
}
/// $07B48C: jsr $5dd02.l
pub fn i_07b48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B492;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B492);
    cpu.pc = 0x5DD02;
    16
}
/// $07B492: move.b #$3, $20(a0)
pub fn i_07b492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B498;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B498: move.b #$0, $21(a0)
pub fn i_07b498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B49E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B49E: move.w #$a, $74(a0)
pub fn i_07b49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4A4;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B4A4: move.w #$b7, $70(a0)
pub fn i_07b4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4AA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B4AA: move.w #$1a8, $72(a0)
pub fn i_07b4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4B0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B4B0: lea.l $7aae0(pc), a1
pub fn i_07b4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B4B4: jsr $4ae.l
pub fn i_07b4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B4BA);
    cpu.pc = 0x4AE;
    16
}
/// $07B4BA: jsr $5dd02.l
pub fn i_07b4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B4C0);
    cpu.pc = 0x5DD02;
    16
}
/// $07B4C0: move.b #$1, $20(a0)
pub fn i_07b4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4C6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B4C6: move.b #$0, $21(a0)
pub fn i_07b4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4CC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B4CC: move.w #$8, $74(a0)
pub fn i_07b4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4D2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B4D2: move.w #$a4, $70(a0)
pub fn i_07b4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4D8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B4D8: move.w #$1a8, $72(a0)
pub fn i_07b4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4DE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B4DE: lea.l $7aae0(pc), a1
pub fn i_07b4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B4E2: jsr $4ae.l
pub fn i_07b4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B4E8);
    cpu.pc = 0x4AE;
    16
}
/// $07B4E8: jsr $5dd02.l
pub fn i_07b4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B4EE);
    cpu.pc = 0x5DD02;
    16
}
/// $07B4EE: move.b #$2, $20(a0)
pub fn i_07b4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4F4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B4F4: move.b #$0, $21(a0)
pub fn i_07b4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B4FA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B4FA: move.w #$6, $74(a0)
pub fn i_07b4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B500;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B500: move.w #$90, $70(a0)
pub fn i_07b500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B506;
    cpu.ir = 0x317C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B506: move.w #$1a8, $72(a0)
pub fn i_07b506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B50C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B50C: lea.l $7aae0(pc), a1
pub fn i_07b50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B510;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B510: jsr $4ae.l
pub fn i_07b510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B516;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B516);
    cpu.pc = 0x4AE;
    16
}
/// $07B516: jsr $5dd02.l
pub fn i_07b516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B51C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B51C);
    cpu.pc = 0x5DD02;
    16
}
/// $07B51C: move.b #$2, $20(a0)
pub fn i_07b51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B522;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B522: move.b #$0, $21(a0)
pub fn i_07b522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B528;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B528: move.w #$4, $74(a0)
pub fn i_07b528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B52E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B52E: move.w #$77, $70(a0)
pub fn i_07b52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B534;
    cpu.ir = 0x317C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B534: move.w #$1a8, $72(a0)
pub fn i_07b534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B53A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B53A: lea.l $7aae0(pc), a1
pub fn i_07b53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B53E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B53E: jsr $4ae.l
pub fn i_07b53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B544;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B544);
    cpu.pc = 0x4AE;
    16
}
/// $07B544: jsr $5dd02.l
pub fn i_07b544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B54A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B54A);
    cpu.pc = 0x5DD02;
    16
}
/// $07B54A: move.b #$1, $20(a0)
pub fn i_07b54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B550;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B550: move.b #$0, $21(a0)
pub fn i_07b550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B556;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B556: move.w #$2, $74(a0)
pub fn i_07b556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B55C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B55C: move.w #$64, $70(a0)
pub fn i_07b55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B562;
    cpu.ir = 0x317C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B562: move.w #$1a8, $72(a0)
pub fn i_07b562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B568;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B568: lea.l $7aae0(pc), a1
pub fn i_07b568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B56C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B56C: jsr $4ae.l
pub fn i_07b56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B572;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B572);
    cpu.pc = 0x4AE;
    16
}
/// $07B572: jsr $5dd02.l
pub fn i_07b572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B578;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B578);
    cpu.pc = 0x5DD02;
    16
}
/// $07B578: move.b #$0, $20(a0)
pub fn i_07b578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B57E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B57E: move.b #$0, $21(a0)
pub fn i_07b57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B584;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B584: move.w #$0, $74(a0)
pub fn i_07b584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B58A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B58A: move.w #$50, $70(a0)
pub fn i_07b58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B590;
    cpu.ir = 0x317C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B590: move.w #$1a8, $72(a0)
pub fn i_07b590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B596;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B596: rts
pub fn i_07b596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B598;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLayout_CompleteFinal_07B598  ($07B598, 968 B, mission/banner_07a970.s)
/// $07B598: lea.l $7aae0(pc), a1
pub fn i_07b598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B59C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B59C: jsr $4ae.l
pub fn i_07b59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B5A2);
    cpu.pc = 0x4AE;
    16
}
/// $07B5A2: jsr $5dd02.l
pub fn i_07b5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B5A8);
    cpu.pc = 0x5DD02;
    16
}
/// $07B5A8: move.b #$c, $20(a0)
pub fn i_07b5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5AE;
    cpu.ir = 0x117C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B5AE: move.b #$1, $21(a0)
pub fn i_07b5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5B4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B5B4: move.w #$28, $74(a0)
pub fn i_07b5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5BA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B5BA: move.w #$104, $70(a0)
pub fn i_07b5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x104;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B5C0: move.w #$168, $72(a0)
pub fn i_07b5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5C6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B5C6: lea.l $7aae0(pc), a1
pub fn i_07b5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B5CA: jsr $4ae.l
pub fn i_07b5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B5D0);
    cpu.pc = 0x4AE;
    16
}
/// $07B5D0: jsr $5dd02.l
pub fn i_07b5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B5D6);
    cpu.pc = 0x5DD02;
    16
}
/// $07B5D6: move.b #$a, $20(a0)
pub fn i_07b5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5DC;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B5DC: move.b #$1, $21(a0)
pub fn i_07b5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5E2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B5E2: move.w #$26, $74(a0)
pub fn i_07b5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5E8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B5E8: move.w #$ee, $70(a0)
pub fn i_07b5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5EE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xEE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B5EE: move.w #$168, $72(a0)
pub fn i_07b5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5F4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B5F4: lea.l $7aae0(pc), a1
pub fn i_07b5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B5F8: jsr $4ae.l
pub fn i_07b5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B5FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B5FE);
    cpu.pc = 0x4AE;
    16
}
/// $07B5FE: jsr $5dd02.l
pub fn i_07b5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B604;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B604);
    cpu.pc = 0x5DD02;
    16
}
/// $07B604: move.b #$b, $20(a0)
pub fn i_07b604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B60A;
    cpu.ir = 0x117C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B60A: move.b #$1, $21(a0)
pub fn i_07b60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B610;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B610: move.w #$24, $74(a0)
pub fn i_07b610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B616;
    cpu.ir = 0x317C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B616: move.w #$d6, $70(a0)
pub fn i_07b616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B61C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xD6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B61C: move.w #$168, $72(a0)
pub fn i_07b61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B622;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B622: lea.l $7aae0(pc), a1
pub fn i_07b622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B626;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B626: jsr $4ae.l
pub fn i_07b626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B62C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B62C);
    cpu.pc = 0x4AE;
    16
}
/// $07B62C: jsr $5dd02.l
pub fn i_07b62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B632;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B632);
    cpu.pc = 0x5DD02;
    16
}
/// $07B632: move.b #$a, $20(a0)
pub fn i_07b632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B638;
    cpu.ir = 0x117C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B638: move.b #$1, $21(a0)
pub fn i_07b638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B63E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B63E: move.w #$22, $74(a0)
pub fn i_07b63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B644;
    cpu.ir = 0x317C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B644: move.w #$be, $70(a0)
pub fn i_07b644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B64A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xBE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B64A: move.w #$168, $72(a0)
pub fn i_07b64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B650;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B650: lea.l $7aae0(pc), a1
pub fn i_07b650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B654;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B654: jsr $4ae.l
pub fn i_07b654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B65A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B65A);
    cpu.pc = 0x4AE;
    16
}
/// $07B65A: jsr $5dd02.l
pub fn i_07b65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B660;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B660);
    cpu.pc = 0x5DD02;
    16
}
/// $07B660: move.b #$9, $20(a0)
pub fn i_07b660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B666;
    cpu.ir = 0x117C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B666: move.b #$1, $21(a0)
pub fn i_07b666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B66C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B66C: move.w #$20, $74(a0)
pub fn i_07b66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B672;
    cpu.ir = 0x317C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B672: move.w #$a7, $70(a0)
pub fn i_07b672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B678;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B678: move.w #$168, $72(a0)
pub fn i_07b678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B67E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B67E: lea.l $7aae0(pc), a1
pub fn i_07b67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B682;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B682: jsr $4ae.l
pub fn i_07b682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B688);
    cpu.pc = 0x4AE;
    16
}
/// $07B688: jsr $5dd02.l
pub fn i_07b688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B68E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B68E);
    cpu.pc = 0x5DD02;
    16
}
/// $07B68E: move.b #$8, $20(a0)
pub fn i_07b68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B694;
    cpu.ir = 0x117C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B694: move.b #$1, $21(a0)
pub fn i_07b694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B69A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B69A: move.w #$1e, $74(a0)
pub fn i_07b69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6A0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6A0: move.w #$8f, $70(a0)
pub fn i_07b6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6A6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6A6: move.w #$168, $72(a0)
pub fn i_07b6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6AC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6AC: lea.l $7aae0(pc), a1
pub fn i_07b6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B6B0: jsr $4ae.l
pub fn i_07b6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B6B6);
    cpu.pc = 0x4AE;
    16
}
/// $07B6B6: jsr $5dd02.l
pub fn i_07b6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B6BC);
    cpu.pc = 0x5DD02;
    16
}
/// $07B6BC: move.b #$0, $20(a0)
pub fn i_07b6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6C2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B6C2: move.b #$1, $21(a0)
pub fn i_07b6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6C8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B6C8: move.w #$1c, $74(a0)
pub fn i_07b6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6CE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6CE: move.w #$76, $70(a0)
pub fn i_07b6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6D4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x76;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6D4: move.w #$168, $72(a0)
pub fn i_07b6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6DA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6DA: lea.l $7aae0(pc), a1
pub fn i_07b6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B6DE: jsr $4ae.l
pub fn i_07b6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B6E4);
    cpu.pc = 0x4AE;
    16
}
/// $07B6E4: jsr $5dd02.l
pub fn i_07b6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B6EA);
    cpu.pc = 0x5DD02;
    16
}
/// $07B6EA: move.b #$3, $20(a0)
pub fn i_07b6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6F0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B6F0: move.b #$1, $21(a0)
pub fn i_07b6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6F6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B6F6: move.w #$1a, $74(a0)
pub fn i_07b6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B6FC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B6FC: move.w #$5d, $70(a0)
pub fn i_07b6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B702;
    cpu.ir = 0x317C;
    let v1: u32 = 0x5D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B702: move.w #$168, $72(a0)
pub fn i_07b702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B708;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B708: lea.l $7aae0(pc), a1
pub fn i_07b708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B70C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B70C: jsr $4ae.l
pub fn i_07b70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B712;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B712);
    cpu.pc = 0x4AE;
    16
}
/// $07B712: jsr $5dd02.l
pub fn i_07b712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B718;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B718);
    cpu.pc = 0x5DD02;
    16
}
/// $07B718: move.b #$7, $20(a0)
pub fn i_07b718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B71E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B71E: move.b #$1, $21(a0)
pub fn i_07b71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B724;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B724: move.w #$18, $74(a0)
pub fn i_07b724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B72A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B72A: move.w #$44, $70(a0)
pub fn i_07b72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B730;
    cpu.ir = 0x317C;
    let v1: u32 = 0x44;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B730: move.w #$168, $72(a0)
pub fn i_07b730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B736;
    cpu.ir = 0x317C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B736: lea.l $7aae0(pc), a1
pub fn i_07b736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B73A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B73A: jsr $4ae.l
pub fn i_07b73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B740;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B740);
    cpu.pc = 0x4AE;
    16
}
/// $07B740: jsr $5dd02.l
pub fn i_07b740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B746;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B746);
    cpu.pc = 0x5DD02;
    16
}
/// $07B746: move.b #$4, $20(a0)
pub fn i_07b746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B74C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B74C: move.b #$0, $21(a0)
pub fn i_07b74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B752;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B752: move.w #$16, $74(a0)
pub fn i_07b752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B758;
    cpu.ir = 0x317C;
    let v1: u32 = 0x16;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B758: move.w #$e0, $70(a0)
pub fn i_07b758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B75E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B75E: move.w #$198, $72(a0)
pub fn i_07b75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B764;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B764: lea.l $7aae0(pc), a1
pub fn i_07b764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B768;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B768: jsr $4ae.l
pub fn i_07b768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B76E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B76E);
    cpu.pc = 0x4AE;
    16
}
/// $07B76E: jsr $5dd02.l
pub fn i_07b76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B774;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B774);
    cpu.pc = 0x5DD02;
    16
}
/// $07B774: move.b #$3, $20(a0)
pub fn i_07b774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B77A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B77A: move.b #$0, $21(a0)
pub fn i_07b77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B780;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B780: move.w #$14, $74(a0)
pub fn i_07b780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B786;
    cpu.ir = 0x317C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B786: move.w #$c7, $70(a0)
pub fn i_07b786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B78C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B78C: move.w #$198, $72(a0)
pub fn i_07b78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B792;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B792: lea.l $7aae0(pc), a1
pub fn i_07b792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B796;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B796: jsr $4ae.l
pub fn i_07b796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B79C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B79C);
    cpu.pc = 0x4AE;
    16
}
/// $07B79C: jsr $5dd02.l
pub fn i_07b79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B7A2);
    cpu.pc = 0x5DD02;
    16
}
/// $07B7A2: move.b #$1, $20(a0)
pub fn i_07b7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7A8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B7A8: move.b #$0, $21(a0)
pub fn i_07b7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7AE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B7AE: move.w #$12, $74(a0)
pub fn i_07b7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7B4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B7B4: move.w #$b4, $70(a0)
pub fn i_07b7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7BA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B7BA: move.w #$198, $72(a0)
pub fn i_07b7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B7C0: lea.l $7aae0(pc), a1
pub fn i_07b7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B7C4: jsr $4ae.l
pub fn i_07b7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B7CA);
    cpu.pc = 0x4AE;
    16
}
/// $07B7CA: jsr $5dd02.l
pub fn i_07b7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B7D0);
    cpu.pc = 0x5DD02;
    16
}
/// $07B7D0: move.b #$2, $20(a0)
pub fn i_07b7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7D6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B7D6: move.b #$0, $21(a0)
pub fn i_07b7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7DC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B7DC: move.w #$10, $74(a0)
pub fn i_07b7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7E2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B7E2: move.w #$a0, $70(a0)
pub fn i_07b7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7E8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B7E8: move.w #$198, $72(a0)
pub fn i_07b7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7EE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B7EE: lea.l $7aae0(pc), a1
pub fn i_07b7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B7F2: jsr $4ae.l
pub fn i_07b7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B7F8);
    cpu.pc = 0x4AE;
    16
}
/// $07B7F8: jsr $5dd02.l
pub fn i_07b7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B7FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B7FE);
    cpu.pc = 0x5DD02;
    16
}
/// $07B7FE: move.b #$2, $20(a0)
pub fn i_07b7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B804;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B804: move.b #$0, $21(a0)
pub fn i_07b804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B80A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B80A: move.w #$e, $74(a0)
pub fn i_07b80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B810;
    cpu.ir = 0x317C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B810: move.w #$87, $70(a0)
pub fn i_07b810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B816;
    cpu.ir = 0x317C;
    let v1: u32 = 0x87;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B816: move.w #$198, $72(a0)
pub fn i_07b816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B81C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B81C: lea.l $7aae0(pc), a1
pub fn i_07b81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B820;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B820: jsr $4ae.l
pub fn i_07b820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B826);
    cpu.pc = 0x4AE;
    16
}
/// $07B826: jsr $5dd02.l
pub fn i_07b826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B82C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B82C);
    cpu.pc = 0x5DD02;
    16
}
/// $07B82C: move.b #$1, $20(a0)
pub fn i_07b82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B832;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B832: move.b #$0, $21(a0)
pub fn i_07b832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B838;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B838: move.w #$c, $74(a0)
pub fn i_07b838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B83E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B83E: move.w #$74, $70(a0)
pub fn i_07b83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B844;
    cpu.ir = 0x317C;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B844: move.w #$198, $72(a0)
pub fn i_07b844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B84A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B84A: lea.l $7aae0(pc), a1
pub fn i_07b84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B84E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B84E: jsr $4ae.l
pub fn i_07b84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B854;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B854);
    cpu.pc = 0x4AE;
    16
}
/// $07B854: jsr $5dd02.l
pub fn i_07b854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B85A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B85A);
    cpu.pc = 0x5DD02;
    16
}
/// $07B85A: move.b #$0, $20(a0)
pub fn i_07b85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B860;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B860: move.b #$0, $21(a0)
pub fn i_07b860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B866;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B866: move.w #$a, $74(a0)
pub fn i_07b866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B86C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B86C: move.w #$60, $70(a0)
pub fn i_07b86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B872;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B872: move.w #$198, $72(a0)
pub fn i_07b872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B878;
    cpu.ir = 0x317C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B878: lea.l $7aae0(pc), a1
pub fn i_07b878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B87C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B87C: jsr $4ae.l
pub fn i_07b87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B882);
    cpu.pc = 0x4AE;
    16
}
/// $07B882: jsr $5dd02.l
pub fn i_07b882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B888);
    cpu.pc = 0x5DD02;
    16
}
/// $07B888: move.b #$9, $20(a0)
pub fn i_07b888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B88E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B88E: move.b #$0, $21(a0)
pub fn i_07b88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B894;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B894: move.w #$8, $74(a0)
pub fn i_07b894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B89A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B89A: move.w #$cd, $70(a0)
pub fn i_07b89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8A0;
    cpu.ir = 0x317C;
    let v1: u32 = 0xCD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8A0: move.w #$1b8, $72(a0)
pub fn i_07b8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8A6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8A6: lea.l $7aae0(pc), a1
pub fn i_07b8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B8AA: jsr $4ae.l
pub fn i_07b8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B8B0);
    cpu.pc = 0x4AE;
    16
}
/// $07B8B0: jsr $5dd02.l
pub fn i_07b8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B8B6);
    cpu.pc = 0x5DD02;
    16
}
/// $07B8B6: move.b #$5, $20(a0)
pub fn i_07b8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8BC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B8BC: move.b #$0, $21(a0)
pub fn i_07b8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8C2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B8C2: move.w #$6, $74(a0)
pub fn i_07b8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8C8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8C8: move.w #$b4, $70(a0)
pub fn i_07b8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8CE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8CE: move.w #$1b8, $72(a0)
pub fn i_07b8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8D4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8D4: lea.l $7aae0(pc), a1
pub fn i_07b8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B8D8: jsr $4ae.l
pub fn i_07b8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B8DE);
    cpu.pc = 0x4AE;
    16
}
/// $07B8DE: jsr $5dd02.l
pub fn i_07b8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B8E4);
    cpu.pc = 0x5DD02;
    16
}
/// $07B8E4: move.b #$4, $20(a0)
pub fn i_07b8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8EA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B8EA: move.b #$0, $21(a0)
pub fn i_07b8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8F0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B8F0: move.w #$4, $74(a0)
pub fn i_07b8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8F6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8F6: move.w #$9c, $70(a0)
pub fn i_07b8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B8FC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x9C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B8FC: move.w #$1b8, $72(a0)
pub fn i_07b8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B902;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B902: lea.l $7aae0(pc), a1
pub fn i_07b902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B906;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B906: jsr $4ae.l
pub fn i_07b906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B90C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B90C);
    cpu.pc = 0x4AE;
    16
}
/// $07B90C: jsr $5dd02.l
pub fn i_07b90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B912);
    cpu.pc = 0x5DD02;
    16
}
/// $07B912: move.b #$1, $20(a0)
pub fn i_07b912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B918;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B918: move.b #$0, $21(a0)
pub fn i_07b918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B91E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B91E: move.w #$2, $74(a0)
pub fn i_07b91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B924;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B924: move.w #$89, $70(a0)
pub fn i_07b924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B92A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x89;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B92A: move.w #$1b8, $72(a0)
pub fn i_07b92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B930;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B930: lea.l $7aae0(pc), a1
pub fn i_07b930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B934;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7AAE0;
    16
}
/// $07B934: jsr $4ae.l
pub fn i_07b934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B93A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B93A);
    cpu.pc = 0x4AE;
    16
}
/// $07B93A: jsr $5dd02.l
pub fn i_07b93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B940;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B940);
    cpu.pc = 0x5DD02;
    16
}
/// $07B940: move.b #$d, $20(a0)
pub fn i_07b940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B946;
    cpu.ir = 0x117C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B946: move.b #$0, $21(a0)
pub fn i_07b946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B94C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B94C: move.w #$0, $74(a0)
pub fn i_07b94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B952;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B952: move.w #$76, $70(a0)
pub fn i_07b952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B958;
    cpu.ir = 0x317C;
    let v1: u32 = 0x76;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B958: move.w #$1b8, $72(a0)
pub fn i_07b958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B95E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B95E: rts
pub fn i_07b95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B960;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLetter_Seek_07B960  ($07B960, 56 B, mission/banner_07a970.s)
/// $07B960: move.w $70(a6), d0
pub fn i_07b960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B964;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B964: move.w $72(a6), d1
pub fn i_07b964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B968;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B968: sub.w $22(a6), d0
pub fn i_07b968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B96C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07B96C: sub.w $24(a6), d1
pub fn i_07b96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B970;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07B970: cmpi.w #$0, d0
pub fn i_07b970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B974;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $07B974: bne.w $7b980
pub fn i_07b974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B978;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7B980; return 10; }
    8
}
/// $07B978: cmpi.w #$0, d1
pub fn i_07b978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B97C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $07B97C: beq.w $7b99e
pub fn i_07b97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B980;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7B99E; return 10; }
    8
}
/// $07B980: asl.w #$6, d0
pub fn i_07b980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B982;
    cpu.ir = 0xED40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $07B982: asl.w #$6, d1
pub fn i_07b982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B984;
    cpu.ir = 0xED41;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $07B984: move.w d0, $28(a6)
pub fn i_07b984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B988;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B988: move.w d1, $2a(a6)
pub fn i_07b988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B98C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07B98C: bset.b #$6, $13(a6)
pub fn i_07b98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B992;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07B992: jsr $27cee.l
pub fn i_07b992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B998;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B998);
    cpu.pc = 0x27CEE;
    16
}
// ===== ClearXN_07b998  ($07B998, 6 B, thunks/ccr_helpers.c)
/// $07B998: andi.b #$ee, ccr
pub fn i_07b998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B99C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07B99C: rts
pub fn i_07b99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B99E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07b99e  ($07B99E, 6 B, thunks/ccr_helpers.c)
/// $07B99E: ori.b #$11, ccr
pub fn i_07b99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9A2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07B9A2: rts
pub fn i_07b9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLetter_CheckSettle_07B9A4  ($07B9A4, 10 B, mission/banner_07a970.s)
/// $07B9A4: cmpi.w #$0, $78(a6)
pub fn i_07b9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9AA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $07B9AA: bne.w $7b9b4
pub fn i_07b9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7B9B4; return 10; }
    8
}
// ===== SetXN_07b9ae  ($07B9AE, 6 B, thunks/ccr_helpers.c)
/// $07B9AE: ori.b #$11, ccr
pub fn i_07b9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9B2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07B9B2: rts
pub fn i_07b9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLetter_Approach_07B9B4  ($07B9B4, 48 B, mission/banner_07a970.s)
/// $07B9B4: move.w $70(a6), d2
pub fn i_07b9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9B8;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B9B8: move.w $72(a6), d3
pub fn i_07b9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9BC;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B9BC: move.w $22(a6), d0
pub fn i_07b9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9C0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B9C0: move.w $24(a6), d1
pub fn i_07b9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9C4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B9C4: jsr $5e23a.l
pub fn i_07b9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7B9CA);
    cpu.pc = 0x5E23A;
    16
}
/// $07B9CA: cmpi.w #$30, d0
pub fn i_07b9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9CE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $07B9CE: bgt.w $7b9ea
pub fn i_07b9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9D2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7B9EA; return 10; }
    8
}
/// $07B9D2: move.w #$18, d0
pub fn i_07b9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9D6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07B9D6: sub.w d0, $78(a6)
pub fn i_07b9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9DA;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07B9DA: bpl.w $7b9ea
pub fn i_07b9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9DE;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7B9EA; return 10; }
    8
}
/// $07B9DE: move.w #$0, $78(a6)
pub fn i_07b9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9E4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_07b9e4  ($07B9E4, 6 B, thunks/ccr_helpers.c)
/// $07B9E4: ori.b #$11, ccr
pub fn i_07b9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9E8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07B9E8: rts
pub fn i_07b9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BannerLetter_Blink_07B9EA  ($07B9EA, 62 B, mission/banner_07a970.s)
/// $07B9EA: move.w $78(a6), d0
pub fn i_07b9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9EE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07B9EE: btst.b #$8, d0
pub fn i_07b9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9F2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07B9F2: bne.w $7b9f8
pub fn i_07b9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7B9F8; return 10; }
    8
}
/// $07B9F6: neg.b d0
pub fn i_07b9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9F8;
    cpu.ir = 0x4400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $07B9F8: move.b d0, $32(a6)
pub fn i_07b9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7B9FC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07B9FC: move.w $78(a6), d0
pub fn i_07b9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA00;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BA00: btst.b #$8, d0
pub fn i_07ba00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA04;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((8) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07BA04: bne.w $7ba28
pub fn i_07ba04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA08;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BA28; return 10; }
    8
}
/// $07BA08: cmpi.b #$0, $7a(a6)
pub fn i_07ba08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA0E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $07BA0E: bne.w $7ba28
pub fn i_07ba0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA12;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7BA28; return 10; }
    8
}
/// $07BA12: move.w #$1aa, d1
pub fn i_07ba12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA16;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BA16: jsr $236e.l
pub fn i_07ba16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BA1C);
    cpu.pc = 0x236E;
    16
}
/// $07BA1C: move.b #$ff, $7a(a6)
pub fn i_07ba1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA22;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BA22: move.b #$0, $3a(a6)
pub fn i_07ba22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA28;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClearXN_07ba28  ($07BA28, 6 B, thunks/ccr_helpers.c)
/// $07BA28: andi.b #$ee, ccr
pub fn i_07ba28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA2C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07BA2C: rts
pub fn i_07ba2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Banner_LetterTimingFromDist_07ba2e  ($07BA2E, 50 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BA2E: move.w $70(a6), d2
pub fn i_07ba2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA32;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BA32: move.w $72(a6), d3
pub fn i_07ba32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA36;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BA36: move.w $22(a6), d0
pub fn i_07ba36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA3A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BA3A: move.w $24(a6), d1
pub fn i_07ba3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA3E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BA3E: jsr $5e23a.l
pub fn i_07ba3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BA44);
    cpu.pc = 0x5E23A;
    16
}
/// $07BA44: asr.w #$4, d0
pub fn i_07ba44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA46;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $07BA46: clr.w d1
pub fn i_07ba46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA48;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07BA48: move.b $32(a6), d1
pub fn i_07ba48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA4C;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07BA4C: sub.w d0, d1
pub fn i_07ba4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA4E;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07BA4E: move.b d1, $32(a6)
pub fn i_07ba4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA52;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BA52: clr.w d1
pub fn i_07ba52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA54;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07BA54: move.b $33(a6), d1
pub fn i_07ba54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA58;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07BA58: sub.w d0, d1
pub fn i_07ba58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA5A;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07BA5A: move.b d1, $33(a6)
pub fn i_07ba5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA5E;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BA5E: rts
pub fn i_07ba5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss2Intro_RetGate_07ba60  ($07BA60, 16 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BA60: movea.l $8(a6), a1
pub fn i_07ba60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA64;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07BA64: move.b $10(a6), d0
pub fn i_07ba64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA68;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07BA68: cmp.b $10(a1), d0
pub fn i_07ba68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA6C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07BA6C: bcs.w $7ba76
pub fn i_07ba6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA70;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7BA76; return 10; }
    8
}
// ===== ClearXN_07ba70  ($07BA70, 6 B, thunks/ccr_helpers.c)
/// $07BA70: andi.b #$ee, ccr
pub fn i_07ba70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA74;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07BA74: rts
pub fn i_07ba74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07ba76  ($07BA76, 6 B, thunks/ccr_helpers.c)
/// $07BA76: ori.b #$11, ccr
pub fn i_07ba76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA7A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07BA7A: rts
pub fn i_07ba7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Tmpl132_07ba7c  ($07BA7C, 74 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BA7C: move.w #$34, d1
pub fn i_07ba7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA80;
    cpu.ir = 0x323C;
    let v1: u32 = 0x34;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BA80: jsr $236e.l
pub fn i_07ba80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BA86);
    cpu.pc = 0x236E;
    16
}
/// $07BA86: move.w #$a90, d0
pub fn i_07ba86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA8A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BA8A: move.l #$100, d1
pub fn i_07ba8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA90;
    cpu.ir = 0x223C;
    let v1: u32 = 0x100;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07BA90: jsr $96a5a.l
pub fn i_07ba90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BA96);
    cpu.pc = 0x96A5A;
    16
}
/// $07BA96: subq.w #$1, d0
pub fn i_07ba96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA98;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07BA98: move.w d0, $22(a6)
pub fn i_07ba98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BA9C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BA9C: move.w d1, $24(a6)
pub fn i_07ba9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAA0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BAA0: move.w #$0, d0
pub fn i_07baa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAA4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BAA4: jsr $28134.l
pub fn i_07baa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BAAA);
    cpu.pc = 0x28134;
    16
}
/// $07BAAA: andi.w #$ffe3, $38(a6)
pub fn i_07baaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAB0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BAB0: ori.w #$1c, $38(a6)
pub fn i_07bab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAB6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BAB6: move.b #$1, $20(a6)
pub fn i_07bab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BABC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07BABC: clr.b $21(a6)
pub fn i_07babc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAC0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07BAC0: jsr $267e2.l
pub fn i_07bac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BAC6);
    cpu.pc = 0x267E2;
    16
}
// ===== SetTaskHandler_07bac6  ($07BAC6, 8 B, kernel/task_handlers.c)
/// $07BAC6: lea.l $7bace(pc), a1
pub fn i_07bac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BACA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BACE;
    16
}
/// $07BACA: move.l a1, (a6)
pub fn i_07baca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BACC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BACC: rts
pub fn i_07bacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BACE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_WaitScroll_07bace  ($07BACE, 68 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BACE: lea.l $7bc8e(pc), a1
pub fn i_07bace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BC8E;
    16
}
/// $07BAD2: jsr $4ae.l
pub fn i_07bad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BAD8);
    cpu.pc = 0x4AE;
    16
}
/// $07BAD8: jsr $5dd02.l
pub fn i_07bad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BADE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BADE);
    cpu.pc = 0x5DD02;
    16
}
/// $07BADE: lea.l $2e0148.l, a0
pub fn i_07bade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0148;
    20
}
/// $07BAE4: jsr $28cd4.l
pub fn i_07bae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BAEA);
    cpu.pc = 0x28CD4;
    16
}
/// $07BAEA: lea.l $7baf0(pc), a1
pub fn i_07baea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BAF0;
    16
}
/// $07BAEE: move.l a1, (a6)
pub fn i_07baee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BAF0: jsr $2783a.l
pub fn i_07baf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BAF6);
    cpu.pc = 0x2783A;
    16
}
/// $07BAF6: jsr $28d70.l
pub fn i_07baf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BAFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BAFC);
    cpu.pc = 0x28D70;
    16
}
/// $07BAFC: move.w $2bcf52.l, d0
pub fn i_07bafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB02;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x2BCF52) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $07BB02: cmp.w $106f50.l, d0
pub fn i_07bb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB08;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $07BB08: bgt.w $7bb18
pub fn i_07bb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB0C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7BB18; return 10; }
    8
}
/// $07BB0C: clr.b $10e39a.l
pub fn i_07bb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB12;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
// ===== SetTaskHandler_07bb12  ($07BB12, 8 B, kernel/task_handlers.c)
/// $07BB12: lea.l $7bb1a(pc), a1
pub fn i_07bb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BB1A;
    16
}
/// $07BB16: move.l a1, (a6)
pub fn i_07bb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BB18: rts
pub fn i_07bb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_IntroAnim_07bb1a  ($07BB1A, 50 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BB1A: move.w #$28, d0
pub fn i_07bb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB1E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BB1E: jsr $2352.l
pub fn i_07bb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BB24);
    cpu.pc = 0x2352;
    16
}
/// $07BB24: lea.l $2e015a.l, a0
pub fn i_07bb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB2A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E015A;
    20
}
/// $07BB2A: jsr $28cd4.l
pub fn i_07bb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BB30);
    cpu.pc = 0x28CD4;
    16
}
/// $07BB30: lea.l $7bb36(pc), a1
pub fn i_07bb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BB36;
    16
}
/// $07BB34: move.l a1, (a6)
pub fn i_07bb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BB36: jsr $2783a.l
pub fn i_07bb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BB3C);
    cpu.pc = 0x2783A;
    16
}
/// $07BB3C: jsr $28d70.l
pub fn i_07bb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BB42);
    cpu.pc = 0x28D70;
    16
}
/// $07BB42: bcc.w $7bb4c
pub fn i_07bb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB46;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7BB4C; return 10; }
    8
}
/// $07BB46: lea.l $7bb54(pc), a1
pub fn i_07bb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BB54;
    16
}
/// $07BB4A: move.l a1, (a6)
pub fn i_07bb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== ClrRamWord_07bb4c  ($07BB4C, 8 B, thunks/clr_ram_word.c)
/// $07BB4C: clr.b $10e39a.l
pub fn i_07bb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB52;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $07BB52: rts
pub fn i_07bb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_WaitDeath_07bb54  ($07BB54, 26 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BB54: jsr $2783a.l
pub fn i_07bb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BB5A);
    cpu.pc = 0x2783A;
    16
}
/// $07BB5A: tst.b $21(a6)
pub fn i_07bb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB5E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07BB5E: beq.w $7bb70
pub fn i_07bb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB62;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7BB70; return 10; }
    8
}
/// $07BB62: bclr.b #$1, $12(a6)
pub fn i_07bb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB68;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07BB68: jmp $518.l
pub fn i_07bb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB6E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== M2Boss_Rts_07bb6e  ($07BB6E, 2 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BB6E: rts
pub fn i_07bb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClrRamWord_07bb70  ($07BB70, 8 B, thunks/clr_ram_word.c)
/// $07BB70: clr.b $10e39a.l
pub fn i_07bb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB76;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $07BB76: rts
pub fn i_07bb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Body_Init_07bb78  ($07BB78, 270 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07BB78: move.w #$2a, d1
pub fn i_07bb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB7C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BB7C: jsr $236e.l
pub fn i_07bb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BB82);
    cpu.pc = 0x236E;
    16
}
/// $07BB82: move.w #$4, $1c(a6)
pub fn i_07bb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB88;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BB88: jsr $138fe.l
pub fn i_07bb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BB8E);
    cpu.pc = 0x138FE;
    16
}
/// $07BB8E: move.w #$0, d0
pub fn i_07bb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB92;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07BB92: jsr $28134.l
pub fn i_07bb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BB98);
    cpu.pc = 0x28134;
    16
}
/// $07BB98: andi.w #$ffe3, $38(a6)
pub fn i_07bb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BB9E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BB9E: ori.w #$8, $38(a6)
pub fn i_07bb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBA4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07BBA4: move.w #$1, $66(a6)
pub fn i_07bba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBAA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BBAA: jsr $267e2.l
pub fn i_07bbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BBB0);
    cpu.pc = 0x267E2;
    16
}
/// $07BBB0: clr.b $21(a6)
pub fn i_07bbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBB4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07BBB4: move.w #$ffff, $7c(a6)
pub fn i_07bbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBBA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BBBA: lea.l $7bbc0(pc), a1
pub fn i_07bbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7BBC0;
    16
}
/// $07BBBE: move.l a1, (a6)
pub fn i_07bbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BBC0: movea.l $c(a6), a0
pub fn i_07bbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBC4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07BBC4: move.w $22(a0), $22(a6)
pub fn i_07bbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBCA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07BBCA: move.w $24(a0), $24(a6)
pub fn i_07bbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBD0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07BBD0: movea.l $c(a6), a0
pub fn i_07bbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBD4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07BBD4: move.w $7c(a0), d0
pub fn i_07bbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBD8;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07BBD8: cmp.w $7c(a6), d0
pub fn i_07bbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBDC;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07BBDC: beq.w $7bc00
pub fn i_07bbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBE0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7BC00; return 10; }
    8
}
/// $07BBE0: move.l d0, -(a7)
pub fn i_07bbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBE2;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07BBE2: movea.l #$2df7d6, a0
pub fn i_07bbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBE8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2DF7D6;
    cpu.a[0] = v1;
    4
}
/// $07BBE8: lsl.w #$2, d0
pub fn i_07bbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBEA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07BBEA: movea.l (a0, d0.w), a0
pub fn i_07bbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBEE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07BBEE: cmpa.l #$ffffffff, a0
pub fn i_07bbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBF4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $07BBF4: beq.w $7bbfe
pub fn i_07bbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBF8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7BBFE; return 10; }
    8
}
/// $07BBF8: jsr $28cd4.l
pub fn i_07bbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BBFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7BBFE);
    cpu.pc = 0x28CD4;
    16
}
/// $07BBFE: move.l (a7)+, d0
pub fn i_07bbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC00;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07BC00: move.w d0, $7c(a6)
pub fn i_07bc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC04;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07BC04: movea.l $c(a6), a0
pub fn i_07bc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC08;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07BC08: btst.b #$7, $5a(a0)
pub fn i_07bc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7BC0E;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x079E08, i_079e08),
    (0x079E0E, i_079e0e),
    (0x079E12, i_079e12),
    (0x079E16, i_079e16),
    (0x079E18, i_079e18),
    (0x079E1E, i_079e1e),
    (0x079E22, i_079e22),
    (0x079E26, i_079e26),
    (0x079E28, i_079e28),
    (0x079E2A, i_079e2a),
    (0x079E30, i_079e30),
    (0x079E34, i_079e34),
    (0x079E38, i_079e38),
    (0x079E3E, i_079e3e),
    (0x079E44, i_079e44),
    (0x079E4A, i_079e4a),
    (0x079E4E, i_079e4e),
    (0x079E50, i_079e50),
    (0x079E56, i_079e56),
    (0x079E5C, i_079e5c),
    (0x079E60, i_079e60),
    (0x079E64, i_079e64),
    (0x079E66, i_079e66),
    (0x079E68, i_079e68),
    (0x079E6E, i_079e6e),
    (0x079E72, i_079e72),
    (0x079E74, i_079e74),
    (0x079E7A, i_079e7a),
    (0x079E80, i_079e80),
    (0x079E84, i_079e84),
    (0x079E88, i_079e88),
    (0x079E8C, i_079e8c),
    (0x079E90, i_079e90),
    (0x079E94, i_079e94),
    (0x079E9A, i_079e9a),
    (0x079E9C, i_079e9c),
    (0x079EA0, i_079ea0),
    (0x079EA4, i_079ea4),
    (0x079EA8, i_079ea8),
    (0x079EAC, i_079eac),
    (0x079EB0, i_079eb0),
    (0x079EB2, i_079eb2),
    (0x079EB6, i_079eb6),
    (0x079EB8, i_079eb8),
    (0x079EBE, i_079ebe),
    (0x079EC2, i_079ec2),
    (0x079EC8, i_079ec8),
    (0x079ECA, i_079eca),
    (0x079ECE, i_079ece),
    (0x079ED2, i_079ed2),
    (0x079ED6, i_079ed6),
    (0x079EDC, i_079edc),
    (0x079EE2, i_079ee2),
    (0x079EE8, i_079ee8),
    (0x079EEC, i_079eec),
    (0x079EF2, i_079ef2),
    (0x079EF8, i_079ef8),
    (0x079EFE, i_079efe),
    (0x079F02, i_079f02),
    (0x079F08, i_079f08),
    (0x079F0E, i_079f0e),
    (0x079F14, i_079f14),
    (0x079F18, i_079f18),
    (0x079F1A, i_079f1a),
    (0x079F1E, i_079f1e),
    (0x079F22, i_079f22),
    (0x079F26, i_079f26),
    (0x079F2A, i_079f2a),
    (0x079F30, i_079f30),
    (0x079F36, i_079f36),
    (0x079F3A, i_079f3a),
    (0x079F40, i_079f40),
    (0x079F46, i_079f46),
    (0x079F48, i_079f48),
    (0x079F4C, i_079f4c),
    (0x079F52, i_079f52),
    (0x079F54, i_079f54),
    (0x079F58, i_079f58),
    (0x079F5C, i_079f5c),
    (0x079F62, i_079f62),
    (0x079F66, i_079f66),
    (0x079F68, i_079f68),
    (0x079F6A, i_079f6a),
    (0x079F70, i_079f70),
    (0x079F74, i_079f74),
    (0x079F7A, i_079f7a),
    (0x079F80, i_079f80),
    (0x079F86, i_079f86),
    (0x079F8C, i_079f8c),
    (0x079F92, i_079f92),
    (0x079F96, i_079f96),
    (0x079F98, i_079f98),
    (0x079F9E, i_079f9e),
    (0x079FA4, i_079fa4),
    (0x079FA8, i_079fa8),
    (0x079FAE, i_079fae),
    (0x079FB2, i_079fb2),
    (0x079FB8, i_079fb8),
    (0x079FBC, i_079fbc),
    (0x079FC2, i_079fc2),
    (0x079FC4, i_079fc4),
    (0x079FCA, i_079fca),
    (0x079FD0, i_079fd0),
    (0x079FD6, i_079fd6),
    (0x079FDC, i_079fdc),
    (0x079FE0, i_079fe0),
    (0x079FE4, i_079fe4),
    (0x079FE6, i_079fe6),
    (0x079FE8, i_079fe8),
    (0x079FEC, i_079fec),
    (0x079FF2, i_079ff2),
    (0x079FF8, i_079ff8),
    (0x079FFA, i_079ffa),
    (0x079FFE, i_079ffe),
    (0x07A002, i_07a002),
    (0x07A006, i_07a006),
    (0x07A008, i_07a008),
    (0x07A00A, i_07a00a),
    (0x07A010, i_07a010),
    (0x07A014, i_07a014),
    (0x07A01A, i_07a01a),
    (0x07A020, i_07a020),
    (0x07A026, i_07a026),
    (0x07A02A, i_07a02a),
    (0x07A030, i_07a030),
    (0x07A036, i_07a036),
    (0x07A03A, i_07a03a),
    (0x07A03C, i_07a03c),
    (0x07A042, i_07a042),
    (0x07A048, i_07a048),
    (0x07A04C, i_07a04c),
    (0x07A052, i_07a052),
    (0x07A056, i_07a056),
    (0x07A05A, i_07a05a),
    (0x07A05C, i_07a05c),
    (0x07A060, i_07a060),
    (0x07A066, i_07a066),
    (0x07A06A, i_07a06a),
    (0x07A070, i_07a070),
    (0x07A074, i_07a074),
    (0x07A07A, i_07a07a),
    (0x07A080, i_07a080),
    (0x07A082, i_07a082),
    (0x07A088, i_07a088),
    (0x07A08E, i_07a08e),
    (0x07A092, i_07a092),
    (0x07A098, i_07a098),
    (0x07A09E, i_07a09e),
    (0x07A0A4, i_07a0a4),
    (0x07A0A8, i_07a0a8),
    (0x07A0AC, i_07a0ac),
    (0x07A0AE, i_07a0ae),
    (0x07A0B0, i_07a0b0),
    (0x07A0B6, i_07a0b6),
    (0x07A0B8, i_07a0b8),
    (0x07A0BE, i_07a0be),
    (0x07A0C4, i_07a0c4),
    (0x07A0C8, i_07a0c8),
    (0x07A0CA, i_07a0ca),
    (0x07A0D0, i_07a0d0),
    (0x07A0D6, i_07a0d6),
    (0x07A0DA, i_07a0da),
    (0x07A0DE, i_07a0de),
    (0x07A0E0, i_07a0e0),
    (0x07A0E4, i_07a0e4),
    (0x07A0EA, i_07a0ea),
    (0x07A0EE, i_07a0ee),
    (0x07A0F4, i_07a0f4),
    (0x07A0F8, i_07a0f8),
    (0x07A0FE, i_07a0fe),
    (0x07A104, i_07a104),
    (0x07A106, i_07a106),
    (0x07A10C, i_07a10c),
    (0x07A112, i_07a112),
    (0x07A114, i_07a114),
    (0x07A11A, i_07a11a),
    (0x07A120, i_07a120),
    (0x07A126, i_07a126),
    (0x07A12A, i_07a12a),
    (0x07A12E, i_07a12e),
    (0x07A130, i_07a130),
    (0x07A132, i_07a132),
    (0x07A136, i_07a136),
    (0x07A13C, i_07a13c),
    (0x07A142, i_07a142),
    (0x07A148, i_07a148),
    (0x07A14C, i_07a14c),
    (0x07A14E, i_07a14e),
    (0x07A154, i_07a154),
    (0x07A15A, i_07a15a),
    (0x07A160, i_07a160),
    (0x07A164, i_07a164),
    (0x07A168, i_07a168),
    (0x07A16C, i_07a16c),
    (0x07A170, i_07a170),
    (0x07A176, i_07a176),
    (0x07A17A, i_07a17a),
    (0x07A17E, i_07a17e),
    (0x07A180, i_07a180),
    (0x07A186, i_07a186),
    (0x07A18C, i_07a18c),
    (0x07A192, i_07a192),
    (0x07A196, i_07a196),
    (0x07A19A, i_07a19a),
    (0x07A19C, i_07a19c),
    (0x07A19E, i_07a19e),
    (0x07A1A2, i_07a1a2),
    (0x07A1A8, i_07a1a8),
    (0x07A1AA, i_07a1aa),
    (0x07A1AE, i_07a1ae),
    (0x07A1B2, i_07a1b2),
    (0x07A1B8, i_07a1b8),
    (0x07A1BC, i_07a1bc),
    (0x07A1BE, i_07a1be),
    (0x07A1C0, i_07a1c0),
    (0x07A1C6, i_07a1c6),
    (0x07A1CA, i_07a1ca),
    (0x07A1D0, i_07a1d0),
    (0x07A1D6, i_07a1d6),
    (0x07A1DC, i_07a1dc),
    (0x07A1E0, i_07a1e0),
    (0x07A1E4, i_07a1e4),
    (0x07A1EA, i_07a1ea),
    (0x07A1F0, i_07a1f0),
    (0x07A1F4, i_07a1f4),
    (0x07A1F8, i_07a1f8),
    (0x07A1FC, i_07a1fc),
    (0x07A200, i_07a200),
    (0x07A204, i_07a204),
    (0x07A208, i_07a208),
    (0x07A20E, i_07a20e),
    (0x07A214, i_07a214),
    (0x07A21A, i_07a21a),
    (0x07A21E, i_07a21e),
    (0x07A220, i_07a220),
    (0x07A226, i_07a226),
    (0x07A22C, i_07a22c),
    (0x07A230, i_07a230),
    (0x07A234, i_07a234),
    (0x07A238, i_07a238),
    (0x07A23C, i_07a23c),
    (0x07A240, i_07a240),
    (0x07A244, i_07a244),
    (0x07A248, i_07a248),
    (0x07A24C, i_07a24c),
    (0x07A24E, i_07a24e),
    (0x07A254, i_07a254),
    (0x07A258, i_07a258),
    (0x07A25C, i_07a25c),
    (0x07A25E, i_07a25e),
    (0x07A264, i_07a264),
    (0x07A26A, i_07a26a),
    (0x07A270, i_07a270),
    (0x07A274, i_07a274),
    (0x07A278, i_07a278),
    (0x07A27A, i_07a27a),
    (0x07A27C, i_07a27c),
    (0x07A280, i_07a280),
    (0x07A284, i_07a284),
    (0x07A288, i_07a288),
    (0x07A28C, i_07a28c),
    (0x07A290, i_07a290),
    (0x07A296, i_07a296),
    (0x07A29C, i_07a29c),
    (0x07A2A0, i_07a2a0),
    (0x07A2A6, i_07a2a6),
    (0x07A2AC, i_07a2ac),
    (0x07A2B0, i_07a2b0),
    (0x07A2B6, i_07a2b6),
    (0x07A2BC, i_07a2bc),
    (0x07A2C0, i_07a2c0),
    (0x07A2C2, i_07a2c2),
    (0x07A2C8, i_07a2c8),
    (0x07A2CE, i_07a2ce),
    (0x07A2D2, i_07a2d2),
    (0x07A2D6, i_07a2d6),
    (0x07A2D8, i_07a2d8),
    (0x07A2DE, i_07a2de),
    (0x07A2E2, i_07a2e2),
    (0x07A2E6, i_07a2e6),
    (0x07A2E8, i_07a2e8),
    (0x07A2EE, i_07a2ee),
    (0x07A2F4, i_07a2f4),
    (0x07A2FA, i_07a2fa),
    (0x07A2FE, i_07a2fe),
    (0x07A302, i_07a302),
    (0x07A304, i_07a304),
    (0x07A306, i_07a306),
    (0x07A30C, i_07a30c),
    (0x07A312, i_07a312),
    (0x07A318, i_07a318),
    (0x07A31C, i_07a31c),
    (0x07A31E, i_07a31e),
    (0x07A324, i_07a324),
    (0x07A32A, i_07a32a),
    (0x07A32E, i_07a32e),
    (0x07A334, i_07a334),
    (0x07A336, i_07a336),
    (0x07A338, i_07a338),
    (0x07A33E, i_07a33e),
    (0x07A342, i_07a342),
    (0x07A348, i_07a348),
    (0x07A34C, i_07a34c),
    (0x07A350, i_07a350),
    (0x07A354, i_07a354),
    (0x07A358, i_07a358),
    (0x07A35E, i_07a35e),
    (0x07A362, i_07a362),
    (0x07A368, i_07a368),
    (0x07A36E, i_07a36e),
    (0x07A372, i_07a372),
    (0x07A378, i_07a378),
    (0x07A37E, i_07a37e),
    (0x07A382, i_07a382),
    (0x07A384, i_07a384),
    (0x07A38A, i_07a38a),
    (0x07A390, i_07a390),
    (0x07A396, i_07a396),
    (0x07A39C, i_07a39c),
    (0x07A3A2, i_07a3a2),
    (0x07A3A6, i_07a3a6),
    (0x07A3AA, i_07a3aa),
    (0x07A3AC, i_07a3ac),
    (0x07A3B0, i_07a3b0),
    (0x07A3B4, i_07a3b4),
    (0x07A3BA, i_07a3ba),
    (0x07A3BC, i_07a3bc),
    (0x07A3C2, i_07a3c2),
    (0x07A3C8, i_07a3c8),
    (0x07A3CE, i_07a3ce),
    (0x07A3D4, i_07a3d4),
    (0x07A3DA, i_07a3da),
    (0x07A3DE, i_07a3de),
    (0x07A3E2, i_07a3e2),
    (0x07A3E4, i_07a3e4),
    (0x07A3E6, i_07a3e6),
    (0x07A3EC, i_07a3ec),
    (0x07A3EE, i_07a3ee),
    (0x07A3F2, i_07a3f2),
    (0x07A3F8, i_07a3f8),
    (0x07A3FC, i_07a3fc),
    (0x07A400, i_07a400),
    (0x07A404, i_07a404),
    (0x07A408, i_07a408),
    (0x07A40E, i_07a40e),
    (0x07A414, i_07a414),
    (0x07A41A, i_07a41a),
    (0x07A41C, i_07a41c),
    (0x07A422, i_07a422),
    (0x07A426, i_07a426),
    (0x07A428, i_07a428),
    (0x07A42E, i_07a42e),
    (0x07A432, i_07a432),
    (0x07A438, i_07a438),
    (0x07A43A, i_07a43a),
    (0x07A43E, i_07a43e),
    (0x07A442, i_07a442),
    (0x07A446, i_07a446),
    (0x07A44A, i_07a44a),
    (0x07A44E, i_07a44e),
    (0x07A450, i_07a450),
    (0x07A454, i_07a454),
    (0x07A456, i_07a456),
    (0x07A45C, i_07a45c),
    (0x07A460, i_07a460),
    (0x07A464, i_07a464),
    (0x07A46A, i_07a46a),
    (0x07A470, i_07a470),
    (0x07A474, i_07a474),
    (0x07A478, i_07a478),
    (0x07A47E, i_07a47e),
    (0x07A484, i_07a484),
    (0x07A48A, i_07a48a),
    (0x07A490, i_07a490),
    (0x07A496, i_07a496),
    (0x07A49C, i_07a49c),
    (0x07A4A2, i_07a4a2),
    (0x07A4A8, i_07a4a8),
    (0x07A4AE, i_07a4ae),
    (0x07A4B4, i_07a4b4),
    (0x07A4BA, i_07a4ba),
    (0x07A4C0, i_07a4c0),
    (0x07A4C6, i_07a4c6),
    (0x07A4CC, i_07a4cc),
    (0x07A4D2, i_07a4d2),
    (0x07A4D8, i_07a4d8),
    (0x07A4DE, i_07a4de),
    (0x07A4E4, i_07a4e4),
    (0x07A4EA, i_07a4ea),
    (0x07A4F0, i_07a4f0),
    (0x07A4F6, i_07a4f6),
    (0x07A4FC, i_07a4fc),
    (0x07A500, i_07a500),
    (0x07A504, i_07a504),
    (0x07A50A, i_07a50a),
    (0x07A510, i_07a510),
    (0x07A514, i_07a514),
    (0x07A518, i_07a518),
    (0x07A51C, i_07a51c),
    (0x07A522, i_07a522),
    (0x07A526, i_07a526),
    (0x07A52A, i_07a52a),
    (0x07A52E, i_07a52e),
    (0x07A532, i_07a532),
    (0x07A538, i_07a538),
    (0x07A53C, i_07a53c),
    (0x07A540, i_07a540),
    (0x07A544, i_07a544),
    (0x07A548, i_07a548),
    (0x07A54A, i_07a54a),
    (0x07A54E, i_07a54e),
    (0x07A554, i_07a554),
    (0x07A558, i_07a558),
    (0x07A55E, i_07a55e),
    (0x07A562, i_07a562),
    (0x07A566, i_07a566),
    (0x07A56A, i_07a56a),
    (0x07A56E, i_07a56e),
    (0x07A570, i_07a570),
    (0x07A574, i_07a574),
    (0x07A57A, i_07a57a),
    (0x07A57E, i_07a57e),
    (0x07A584, i_07a584),
    (0x07A588, i_07a588),
    (0x07A58C, i_07a58c),
    (0x07A590, i_07a590),
    (0x07A594, i_07a594),
    (0x07A596, i_07a596),
    (0x07A59C, i_07a59c),
    (0x07A5A2, i_07a5a2),
    (0x07A5A6, i_07a5a6),
    (0x07A5AC, i_07a5ac),
    (0x07A5B2, i_07a5b2),
    (0x07A5B6, i_07a5b6),
    (0x07A5BA, i_07a5ba),
    (0x07A5BE, i_07a5be),
    (0x07A5C2, i_07a5c2),
    (0x07A5C4, i_07a5c4),
    (0x07A5CA, i_07a5ca),
    (0x07A5D0, i_07a5d0),
    (0x07A5D4, i_07a5d4),
    (0x07A5DA, i_07a5da),
    (0x07A5E0, i_07a5e0),
    (0x07A5E4, i_07a5e4),
    (0x07A5E8, i_07a5e8),
    (0x07A5EC, i_07a5ec),
    (0x07A5EE, i_07a5ee),
    (0x07A5F2, i_07a5f2),
    (0x07A5F8, i_07a5f8),
    (0x07A5FC, i_07a5fc),
    (0x07A602, i_07a602),
    (0x07A606, i_07a606),
    (0x07A60A, i_07a60a),
    (0x07A60E, i_07a60e),
    (0x07A614, i_07a614),
    (0x07A61A, i_07a61a),
    (0x07A61E, i_07a61e),
    (0x07A622, i_07a622),
    (0x07A626, i_07a626),
    (0x07A62C, i_07a62c),
    (0x07A630, i_07a630),
    (0x07A634, i_07a634),
    (0x07A638, i_07a638),
    (0x07A63C, i_07a63c),
    (0x07A642, i_07a642),
    (0x07A646, i_07a646),
    (0x07A64C, i_07a64c),
    (0x07A650, i_07a650),
    (0x07A654, i_07a654),
    (0x07A658, i_07a658),
    (0x07A65A, i_07a65a),
    (0x07A65E, i_07a65e),
    (0x07A664, i_07a664),
    (0x07A668, i_07a668),
    (0x07A66E, i_07a66e),
    (0x07A672, i_07a672),
    (0x07A676, i_07a676),
    (0x07A67A, i_07a67a),
    (0x07A67E, i_07a67e),
    (0x07A680, i_07a680),
    (0x07A684, i_07a684),
    (0x07A68A, i_07a68a),
    (0x07A68E, i_07a68e),
    (0x07A694, i_07a694),
    (0x07A698, i_07a698),
    (0x07A69E, i_07a69e),
    (0x07A6A2, i_07a6a2),
    (0x07A6A6, i_07a6a6),
    (0x07A6A8, i_07a6a8),
    (0x07A6AC, i_07a6ac),
    (0x07A6B0, i_07a6b0),
    (0x07A6B2, i_07a6b2),
    (0x07A6B6, i_07a6b6),
    (0x07A6BA, i_07a6ba),
    (0x07A6BC, i_07a6bc),
    (0x07A6C2, i_07a6c2),
    (0x07A6C4, i_07a6c4),
    (0x07A6C8, i_07a6c8),
    (0x07A6CC, i_07a6cc),
    (0x07A6CE, i_07a6ce),
    (0x07A6D2, i_07a6d2),
    (0x07A6D6, i_07a6d6),
    (0x07A6D8, i_07a6d8),
    (0x07A6DC, i_07a6dc),
    (0x07A6E0, i_07a6e0),
    (0x07A6E2, i_07a6e2),
    (0x07A6E8, i_07a6e8),
    (0x07A6EA, i_07a6ea),
    (0x07A6EE, i_07a6ee),
    (0x07A6F2, i_07a6f2),
    (0x07A6F4, i_07a6f4),
    (0x07A6F8, i_07a6f8),
    (0x07A6FC, i_07a6fc),
    (0x07A6FE, i_07a6fe),
    (0x07A702, i_07a702),
    (0x07A706, i_07a706),
    (0x07A708, i_07a708),
    (0x07A70E, i_07a70e),
    (0x07A710, i_07a710),
    (0x07A714, i_07a714),
    (0x07A718, i_07a718),
    (0x07A71A, i_07a71a),
    (0x07A71E, i_07a71e),
    (0x07A722, i_07a722),
    (0x07A724, i_07a724),
    (0x07A728, i_07a728),
    (0x07A72C, i_07a72c),
    (0x07A72E, i_07a72e),
    (0x07A734, i_07a734),
    (0x07A736, i_07a736),
    (0x07A73A, i_07a73a),
    (0x07A73E, i_07a73e),
    (0x07A740, i_07a740),
    (0x07A744, i_07a744),
    (0x07A748, i_07a748),
    (0x07A74A, i_07a74a),
    (0x07A750, i_07a750),
    (0x07A754, i_07a754),
    (0x07A75A, i_07a75a),
    (0x07A75E, i_07a75e),
    (0x07A762, i_07a762),
    (0x07A764, i_07a764),
    (0x07A768, i_07a768),
    (0x07A76C, i_07a76c),
    (0x07A76E, i_07a76e),
    (0x07A774, i_07a774),
    (0x07A778, i_07a778),
    (0x07A77E, i_07a77e),
    (0x07A782, i_07a782),
    (0x07A786, i_07a786),
    (0x07A788, i_07a788),
    (0x07A78C, i_07a78c),
    (0x07A790, i_07a790),
    (0x07A792, i_07a792),
    (0x07A798, i_07a798),
    (0x07A79C, i_07a79c),
    (0x07A7A2, i_07a7a2),
    (0x07A7A6, i_07a7a6),
    (0x07A7AA, i_07a7aa),
    (0x07A7AC, i_07a7ac),
    (0x07A7B0, i_07a7b0),
    (0x07A7B4, i_07a7b4),
    (0x07A7B6, i_07a7b6),
    (0x07A7BC, i_07a7bc),
    (0x07A7C0, i_07a7c0),
    (0x07A7C6, i_07a7c6),
    (0x07A7CA, i_07a7ca),
    (0x07A7CE, i_07a7ce),
    (0x07A7D0, i_07a7d0),
    (0x07A7D2, i_07a7d2),
    (0x07A7D6, i_07a7d6),
    (0x07A7DC, i_07a7dc),
    (0x07A7E0, i_07a7e0),
    (0x07A7E6, i_07a7e6),
    (0x07A954, i_07a954),
    (0x07A958, i_07a958),
    (0x07A95C, i_07a95c),
    (0x07A960, i_07a960),
    (0x07A964, i_07a964),
    (0x07A968, i_07a968),
    (0x07A96A, i_07a96a),
    (0x07A96E, i_07a96e),
    (0x07A970, i_07a970),
    (0x07A976, i_07a976),
    (0x07A97C, i_07a97c),
    (0x07A982, i_07a982),
    (0x07A988, i_07a988),
    (0x07A98E, i_07a98e),
    (0x07A994, i_07a994),
    (0x07A99A, i_07a99a),
    (0x07A99E, i_07a99e),
    (0x07A9A2, i_07a9a2),
    (0x07A9A6, i_07a9a6),
    (0x07A9AA, i_07a9aa),
    (0x07A9AE, i_07a9ae),
    (0x07A9B4, i_07a9b4),
    (0x07A9BA, i_07a9ba),
    (0x07A9BE, i_07a9be),
    (0x07A9C0, i_07a9c0),
    (0x07A9C6, i_07a9c6),
    (0x07A9CA, i_07a9ca),
    (0x07A9CE, i_07a9ce),
    (0x07A9D2, i_07a9d2),
    (0x07A9D6, i_07a9d6),
    (0x07A9DA, i_07a9da),
    (0x07A9DE, i_07a9de),
    (0x07A9E2, i_07a9e2),
    (0x07A9E8, i_07a9e8),
    (0x07A9EC, i_07a9ec),
    (0x07A9EE, i_07a9ee),
    (0x07A9F0, i_07a9f0),
    (0x07A9F4, i_07a9f4),
    (0x07A9FA, i_07a9fa),
    (0x07AA00, i_07aa00),
    (0x07AA06, i_07aa06),
    (0x07AA0C, i_07aa0c),
    (0x07AA12, i_07aa12),
    (0x07AA18, i_07aa18),
    (0x07AA1E, i_07aa1e),
    (0x07AA22, i_07aa22),
    (0x07AA26, i_07aa26),
    (0x07AA2A, i_07aa2a),
    (0x07AA2E, i_07aa2e),
    (0x07AA32, i_07aa32),
    (0x07AA38, i_07aa38),
    (0x07AA3E, i_07aa3e),
    (0x07AA42, i_07aa42),
    (0x07AA44, i_07aa44),
    (0x07AA4A, i_07aa4a),
    (0x07AA4E, i_07aa4e),
    (0x07AA52, i_07aa52),
    (0x07AA56, i_07aa56),
    (0x07AA5A, i_07aa5a),
    (0x07AA5E, i_07aa5e),
    (0x07AA62, i_07aa62),
    (0x07AA66, i_07aa66),
    (0x07AA6A, i_07aa6a),
    (0x07AA70, i_07aa70),
    (0x07AA74, i_07aa74),
    (0x07AA76, i_07aa76),
    (0x07AA78, i_07aa78),
    (0x07AA7C, i_07aa7c),
    (0x07AA7E, i_07aa7e),
    (0x07AA82, i_07aa82),
    (0x07AA88, i_07aa88),
    (0x07AA8C, i_07aa8c),
    (0x07AA90, i_07aa90),
    (0x07AA92, i_07aa92),
    (0x07AA94, i_07aa94),
    (0x07AA9A, i_07aa9a),
    (0x07AAA0, i_07aaa0),
    (0x07AAA4, i_07aaa4),
    (0x07AAA8, i_07aaa8),
    (0x07AAAC, i_07aaac),
    (0x07AAB2, i_07aab2),
    (0x07AAB4, i_07aab4),
    (0x07AAB8, i_07aab8),
    (0x07AABE, i_07aabe),
    (0x07AAC4, i_07aac4),
    (0x07AAC8, i_07aac8),
    (0x07AACA, i_07aaca),
    (0x07AACE, i_07aace),
    (0x07AAD2, i_07aad2),
    (0x07AAD8, i_07aad8),
    (0x07AADE, i_07aade),
    (0x07AAE0, i_07aae0),
    (0x07AAE6, i_07aae6),
    (0x07AAEA, i_07aaea),
    (0x07AAEC, i_07aaec),
    (0x07AAF0, i_07aaf0),
    (0x07AAF4, i_07aaf4),
    (0x07AAF6, i_07aaf6),
    (0x07AAFA, i_07aafa),
    (0x07AB00, i_07ab00),
    (0x07AB06, i_07ab06),
    (0x07AB0C, i_07ab0c),
    (0x07AB12, i_07ab12),
    (0x07AB18, i_07ab18),
    (0x07AB1E, i_07ab1e),
    (0x07AB24, i_07ab24),
    (0x07AB26, i_07ab26),
    (0x07AB2A, i_07ab2a),
    (0x07AB2E, i_07ab2e),
    (0x07AB32, i_07ab32),
    (0x07AB36, i_07ab36),
    (0x07AB3C, i_07ab3c),
    (0x07AB3E, i_07ab3e),
    (0x07AB42, i_07ab42),
    (0x07AB48, i_07ab48),
    (0x07AB4C, i_07ab4c),
    (0x07AB52, i_07ab52),
    (0x07AB56, i_07ab56),
    (0x07AB58, i_07ab58),
    (0x07AB5C, i_07ab5c),
    (0x07AB62, i_07ab62),
    (0x07AB66, i_07ab66),
    (0x07AB68, i_07ab68),
    (0x07AB6C, i_07ab6c),
    (0x07AB70, i_07ab70),
    (0x07AB72, i_07ab72),
    (0x07AB76, i_07ab76),
    (0x07AB7C, i_07ab7c),
    (0x07AB82, i_07ab82),
    (0x07AB88, i_07ab88),
    (0x07AB8E, i_07ab8e),
    (0x07AB94, i_07ab94),
    (0x07AB9A, i_07ab9a),
    (0x07AB9C, i_07ab9c),
    (0x07ABA2, i_07aba2),
    (0x07ABA6, i_07aba6),
    (0x07ABAA, i_07abaa),
    (0x07ABAE, i_07abae),
    (0x07ABB4, i_07abb4),
    (0x07ABB6, i_07abb6),
    (0x07ABBA, i_07abba),
    (0x07ABC0, i_07abc0),
    (0x07ABC4, i_07abc4),
    (0x07ABCA, i_07abca),
    (0x07ABCE, i_07abce),
    (0x07ABD0, i_07abd0),
    (0x07ABD4, i_07abd4),
    (0x07ABDA, i_07abda),
    (0x07ABDE, i_07abde),
    (0x07ABE2, i_07abe2),
    (0x07ABE6, i_07abe6),
    (0x07ABEC, i_07abec),
    (0x07ABF0, i_07abf0),
    (0x07ABF2, i_07abf2),
    (0x07ABF4, i_07abf4),
    (0x07ABFA, i_07abfa),
    (0x07ABFE, i_07abfe),
    (0x07AC02, i_07ac02),
    (0x07AC08, i_07ac08),
    (0x07AC0E, i_07ac0e),
    (0x07AC12, i_07ac12),
    (0x07AC14, i_07ac14),
    (0x07AC1A, i_07ac1a),
    (0x07AC1E, i_07ac1e),
    (0x07AC22, i_07ac22),
    (0x07AC26, i_07ac26),
    (0x07AC2A, i_07ac2a),
    (0x07AC2E, i_07ac2e),
    (0x07AC34, i_07ac34),
    (0x07AC38, i_07ac38),
    (0x07AC3E, i_07ac3e),
    (0x07AC42, i_07ac42),
    (0x07AC46, i_07ac46),
    (0x07AC48, i_07ac48),
    (0x07AC4A, i_07ac4a),
    (0x07AC50, i_07ac50),
    (0x07AC54, i_07ac54),
    (0x07AC56, i_07ac56),
    (0x07AC5C, i_07ac5c),
    (0x07AC5E, i_07ac5e),
    (0x07AC62, i_07ac62),
    (0x07AC66, i_07ac66),
    (0x07AC6A, i_07ac6a),
    (0x07AC70, i_07ac70),
    (0x07AC74, i_07ac74),
    (0x07AC78, i_07ac78),
    (0x07AC7C, i_07ac7c),
    (0x07AC80, i_07ac80),
    (0x07AC86, i_07ac86),
    (0x07AC8A, i_07ac8a),
    (0x07AC8E, i_07ac8e),
    (0x07AC92, i_07ac92),
    (0x07AC98, i_07ac98),
    (0x07AC9C, i_07ac9c),
    (0x07ACA0, i_07aca0),
    (0x07ACA6, i_07aca6),
    (0x07ACAC, i_07acac),
    (0x07ACB2, i_07acb2),
    (0x07ACB8, i_07acb8),
    (0x07ACBE, i_07acbe),
    (0x07ACC4, i_07acc4),
    (0x07ACC8, i_07acc8),
    (0x07ACCE, i_07acce),
    (0x07ACD0, i_07acd0),
    (0x07ACD4, i_07acd4),
    (0x07ACDA, i_07acda),
    (0x07ACE0, i_07ace0),
    (0x07ACE6, i_07ace6),
    (0x07ACEC, i_07acec),
    (0x07ACF2, i_07acf2),
    (0x07ACF8, i_07acf8),
    (0x07ACFE, i_07acfe),
    (0x07AD02, i_07ad02),
    (0x07AD08, i_07ad08),
    (0x07AD0E, i_07ad0e),
    (0x07AD14, i_07ad14),
    (0x07AD1A, i_07ad1a),
    (0x07AD20, i_07ad20),
    (0x07AD26, i_07ad26),
    (0x07AD2C, i_07ad2c),
    (0x07AD30, i_07ad30),
    (0x07AD36, i_07ad36),
    (0x07AD3C, i_07ad3c),
    (0x07AD42, i_07ad42),
    (0x07AD48, i_07ad48),
    (0x07AD4E, i_07ad4e),
    (0x07AD54, i_07ad54),
    (0x07AD5A, i_07ad5a),
    (0x07AD5E, i_07ad5e),
    (0x07AD64, i_07ad64),
    (0x07AD6A, i_07ad6a),
    (0x07AD70, i_07ad70),
    (0x07AD76, i_07ad76),
    (0x07AD7C, i_07ad7c),
    (0x07AD82, i_07ad82),
    (0x07AD88, i_07ad88),
    (0x07AD8C, i_07ad8c),
    (0x07AD92, i_07ad92),
    (0x07AD98, i_07ad98),
    (0x07AD9E, i_07ad9e),
    (0x07ADA4, i_07ada4),
    (0x07ADAA, i_07adaa),
    (0x07ADB0, i_07adb0),
    (0x07ADB6, i_07adb6),
    (0x07ADBA, i_07adba),
    (0x07ADC0, i_07adc0),
    (0x07ADC6, i_07adc6),
    (0x07ADCC, i_07adcc),
    (0x07ADD2, i_07add2),
    (0x07ADD8, i_07add8),
    (0x07ADDE, i_07adde),
    (0x07ADE4, i_07ade4),
    (0x07ADE8, i_07ade8),
    (0x07ADEE, i_07adee),
    (0x07ADF4, i_07adf4),
    (0x07ADFA, i_07adfa),
    (0x07AE00, i_07ae00),
    (0x07AE06, i_07ae06),
    (0x07AE0C, i_07ae0c),
    (0x07AE10, i_07ae10),
    (0x07AE16, i_07ae16),
    (0x07AE1C, i_07ae1c),
    (0x07AE22, i_07ae22),
    (0x07AE28, i_07ae28),
    (0x07AE2E, i_07ae2e),
    (0x07AE34, i_07ae34),
    (0x07AE3A, i_07ae3a),
    (0x07AE3E, i_07ae3e),
    (0x07AE44, i_07ae44),
    (0x07AE4A, i_07ae4a),
    (0x07AE50, i_07ae50),
    (0x07AE56, i_07ae56),
    (0x07AE5C, i_07ae5c),
    (0x07AE62, i_07ae62),
    (0x07AE68, i_07ae68),
    (0x07AE6C, i_07ae6c),
    (0x07AE72, i_07ae72),
    (0x07AE78, i_07ae78),
    (0x07AE7E, i_07ae7e),
    (0x07AE84, i_07ae84),
    (0x07AE8A, i_07ae8a),
    (0x07AE90, i_07ae90),
    (0x07AE96, i_07ae96),
    (0x07AE9A, i_07ae9a),
    (0x07AEA0, i_07aea0),
    (0x07AEA6, i_07aea6),
    (0x07AEAC, i_07aeac),
    (0x07AEB2, i_07aeb2),
    (0x07AEB8, i_07aeb8),
    (0x07AEBE, i_07aebe),
    (0x07AEC4, i_07aec4),
    (0x07AEC8, i_07aec8),
    (0x07AECE, i_07aece),
    (0x07AED4, i_07aed4),
    (0x07AEDA, i_07aeda),
    (0x07AEE0, i_07aee0),
    (0x07AEE6, i_07aee6),
    (0x07AEEC, i_07aeec),
    (0x07AEF2, i_07aef2),
    (0x07AEF6, i_07aef6),
    (0x07AEFC, i_07aefc),
    (0x07AF02, i_07af02),
    (0x07AF08, i_07af08),
    (0x07AF0E, i_07af0e),
    (0x07AF14, i_07af14),
    (0x07AF1A, i_07af1a),
    (0x07AF20, i_07af20),
    (0x07AF24, i_07af24),
    (0x07AF2A, i_07af2a),
    (0x07AF30, i_07af30),
    (0x07AF36, i_07af36),
    (0x07AF3C, i_07af3c),
    (0x07AF42, i_07af42),
    (0x07AF48, i_07af48),
    (0x07AF4E, i_07af4e),
    (0x07AF50, i_07af50),
    (0x07AF54, i_07af54),
    (0x07AF5A, i_07af5a),
    (0x07AF60, i_07af60),
    (0x07AF66, i_07af66),
    (0x07AF6C, i_07af6c),
    (0x07AF72, i_07af72),
    (0x07AF78, i_07af78),
    (0x07AF7E, i_07af7e),
    (0x07AF82, i_07af82),
    (0x07AF88, i_07af88),
    (0x07AF8E, i_07af8e),
    (0x07AF94, i_07af94),
    (0x07AF9A, i_07af9a),
    (0x07AFA0, i_07afa0),
    (0x07AFA6, i_07afa6),
    (0x07AFAC, i_07afac),
    (0x07AFB0, i_07afb0),
    (0x07AFB6, i_07afb6),
    (0x07AFBC, i_07afbc),
    (0x07AFC2, i_07afc2),
    (0x07AFC8, i_07afc8),
    (0x07AFCE, i_07afce),
    (0x07AFD4, i_07afd4),
    (0x07AFDA, i_07afda),
    (0x07AFDE, i_07afde),
    (0x07AFE4, i_07afe4),
    (0x07AFEA, i_07afea),
    (0x07AFF0, i_07aff0),
    (0x07AFF6, i_07aff6),
    (0x07AFFC, i_07affc),
    (0x07B002, i_07b002),
    (0x07B008, i_07b008),
    (0x07B00C, i_07b00c),
    (0x07B012, i_07b012),
    (0x07B018, i_07b018),
    (0x07B01E, i_07b01e),
    (0x07B024, i_07b024),
    (0x07B02A, i_07b02a),
    (0x07B030, i_07b030),
    (0x07B036, i_07b036),
    (0x07B03A, i_07b03a),
    (0x07B040, i_07b040),
    (0x07B046, i_07b046),
    (0x07B04C, i_07b04c),
    (0x07B052, i_07b052),
    (0x07B058, i_07b058),
    (0x07B05E, i_07b05e),
    (0x07B064, i_07b064),
    (0x07B068, i_07b068),
    (0x07B06E, i_07b06e),
    (0x07B074, i_07b074),
    (0x07B07A, i_07b07a),
    (0x07B080, i_07b080),
    (0x07B086, i_07b086),
    (0x07B08C, i_07b08c),
    (0x07B092, i_07b092),
    (0x07B096, i_07b096),
    (0x07B09C, i_07b09c),
    (0x07B0A2, i_07b0a2),
    (0x07B0A8, i_07b0a8),
    (0x07B0AE, i_07b0ae),
    (0x07B0B4, i_07b0b4),
    (0x07B0BA, i_07b0ba),
    (0x07B0C0, i_07b0c0),
    (0x07B0C4, i_07b0c4),
    (0x07B0CA, i_07b0ca),
    (0x07B0D0, i_07b0d0),
    (0x07B0D6, i_07b0d6),
    (0x07B0DC, i_07b0dc),
    (0x07B0E2, i_07b0e2),
    (0x07B0E8, i_07b0e8),
    (0x07B0EE, i_07b0ee),
    (0x07B0F2, i_07b0f2),
    (0x07B0F8, i_07b0f8),
    (0x07B0FE, i_07b0fe),
    (0x07B104, i_07b104),
    (0x07B10A, i_07b10a),
    (0x07B110, i_07b110),
    (0x07B116, i_07b116),
    (0x07B11C, i_07b11c),
    (0x07B120, i_07b120),
    (0x07B126, i_07b126),
    (0x07B12C, i_07b12c),
    (0x07B132, i_07b132),
    (0x07B138, i_07b138),
    (0x07B13E, i_07b13e),
    (0x07B144, i_07b144),
    (0x07B14A, i_07b14a),
    (0x07B14E, i_07b14e),
    (0x07B154, i_07b154),
    (0x07B15A, i_07b15a),
    (0x07B160, i_07b160),
    (0x07B166, i_07b166),
    (0x07B16C, i_07b16c),
    (0x07B172, i_07b172),
    (0x07B178, i_07b178),
    (0x07B17C, i_07b17c),
    (0x07B182, i_07b182),
    (0x07B188, i_07b188),
    (0x07B18E, i_07b18e),
    (0x07B194, i_07b194),
    (0x07B19A, i_07b19a),
    (0x07B1A0, i_07b1a0),
    (0x07B1A6, i_07b1a6),
    (0x07B1AA, i_07b1aa),
    (0x07B1B0, i_07b1b0),
    (0x07B1B6, i_07b1b6),
    (0x07B1BC, i_07b1bc),
    (0x07B1C2, i_07b1c2),
    (0x07B1C8, i_07b1c8),
    (0x07B1CE, i_07b1ce),
    (0x07B1D4, i_07b1d4),
    (0x07B1D8, i_07b1d8),
    (0x07B1DE, i_07b1de),
    (0x07B1E4, i_07b1e4),
    (0x07B1EA, i_07b1ea),
    (0x07B1F0, i_07b1f0),
    (0x07B1F6, i_07b1f6),
    (0x07B1FC, i_07b1fc),
    (0x07B202, i_07b202),
    (0x07B206, i_07b206),
    (0x07B20C, i_07b20c),
    (0x07B212, i_07b212),
    (0x07B218, i_07b218),
    (0x07B21E, i_07b21e),
    (0x07B224, i_07b224),
    (0x07B22A, i_07b22a),
    (0x07B230, i_07b230),
    (0x07B234, i_07b234),
    (0x07B23A, i_07b23a),
    (0x07B240, i_07b240),
    (0x07B246, i_07b246),
    (0x07B24C, i_07b24c),
    (0x07B252, i_07b252),
    (0x07B258, i_07b258),
    (0x07B25E, i_07b25e),
    (0x07B262, i_07b262),
    (0x07B268, i_07b268),
    (0x07B26E, i_07b26e),
    (0x07B274, i_07b274),
    (0x07B27A, i_07b27a),
    (0x07B280, i_07b280),
    (0x07B286, i_07b286),
    (0x07B28C, i_07b28c),
    (0x07B28E, i_07b28e),
    (0x07B292, i_07b292),
    (0x07B298, i_07b298),
    (0x07B29E, i_07b29e),
    (0x07B2A4, i_07b2a4),
    (0x07B2AA, i_07b2aa),
    (0x07B2B0, i_07b2b0),
    (0x07B2B6, i_07b2b6),
    (0x07B2BC, i_07b2bc),
    (0x07B2C0, i_07b2c0),
    (0x07B2C6, i_07b2c6),
    (0x07B2CC, i_07b2cc),
    (0x07B2D2, i_07b2d2),
    (0x07B2D8, i_07b2d8),
    (0x07B2DE, i_07b2de),
    (0x07B2E4, i_07b2e4),
    (0x07B2EA, i_07b2ea),
    (0x07B2EE, i_07b2ee),
    (0x07B2F4, i_07b2f4),
    (0x07B2FA, i_07b2fa),
    (0x07B300, i_07b300),
    (0x07B306, i_07b306),
    (0x07B30C, i_07b30c),
    (0x07B312, i_07b312),
    (0x07B318, i_07b318),
    (0x07B31C, i_07b31c),
    (0x07B322, i_07b322),
    (0x07B328, i_07b328),
    (0x07B32E, i_07b32e),
    (0x07B334, i_07b334),
    (0x07B33A, i_07b33a),
    (0x07B340, i_07b340),
    (0x07B346, i_07b346),
    (0x07B34A, i_07b34a),
    (0x07B350, i_07b350),
    (0x07B356, i_07b356),
    (0x07B35C, i_07b35c),
    (0x07B362, i_07b362),
    (0x07B368, i_07b368),
    (0x07B36E, i_07b36e),
    (0x07B374, i_07b374),
    (0x07B378, i_07b378),
    (0x07B37E, i_07b37e),
    (0x07B384, i_07b384),
    (0x07B38A, i_07b38a),
    (0x07B390, i_07b390),
    (0x07B396, i_07b396),
    (0x07B39C, i_07b39c),
    (0x07B3A2, i_07b3a2),
    (0x07B3A6, i_07b3a6),
    (0x07B3AC, i_07b3ac),
    (0x07B3B2, i_07b3b2),
    (0x07B3B8, i_07b3b8),
    (0x07B3BE, i_07b3be),
    (0x07B3C4, i_07b3c4),
    (0x07B3CA, i_07b3ca),
    (0x07B3D0, i_07b3d0),
    (0x07B3D4, i_07b3d4),
    (0x07B3DA, i_07b3da),
    (0x07B3E0, i_07b3e0),
    (0x07B3E6, i_07b3e6),
    (0x07B3EC, i_07b3ec),
    (0x07B3F2, i_07b3f2),
    (0x07B3F8, i_07b3f8),
    (0x07B3FE, i_07b3fe),
    (0x07B402, i_07b402),
    (0x07B408, i_07b408),
    (0x07B40E, i_07b40e),
    (0x07B414, i_07b414),
    (0x07B41A, i_07b41a),
    (0x07B420, i_07b420),
    (0x07B426, i_07b426),
    (0x07B42C, i_07b42c),
    (0x07B430, i_07b430),
    (0x07B436, i_07b436),
    (0x07B43C, i_07b43c),
    (0x07B442, i_07b442),
    (0x07B448, i_07b448),
    (0x07B44E, i_07b44e),
    (0x07B454, i_07b454),
    (0x07B458, i_07b458),
    (0x07B45E, i_07b45e),
    (0x07B464, i_07b464),
    (0x07B46A, i_07b46a),
    (0x07B470, i_07b470),
    (0x07B476, i_07b476),
    (0x07B47C, i_07b47c),
    (0x07B482, i_07b482),
    (0x07B486, i_07b486),
    (0x07B48C, i_07b48c),
    (0x07B492, i_07b492),
    (0x07B498, i_07b498),
    (0x07B49E, i_07b49e),
    (0x07B4A4, i_07b4a4),
    (0x07B4AA, i_07b4aa),
    (0x07B4B0, i_07b4b0),
    (0x07B4B4, i_07b4b4),
    (0x07B4BA, i_07b4ba),
    (0x07B4C0, i_07b4c0),
    (0x07B4C6, i_07b4c6),
    (0x07B4CC, i_07b4cc),
    (0x07B4D2, i_07b4d2),
    (0x07B4D8, i_07b4d8),
    (0x07B4DE, i_07b4de),
    (0x07B4E2, i_07b4e2),
    (0x07B4E8, i_07b4e8),
    (0x07B4EE, i_07b4ee),
    (0x07B4F4, i_07b4f4),
    (0x07B4FA, i_07b4fa),
    (0x07B500, i_07b500),
    (0x07B506, i_07b506),
    (0x07B50C, i_07b50c),
    (0x07B510, i_07b510),
    (0x07B516, i_07b516),
    (0x07B51C, i_07b51c),
    (0x07B522, i_07b522),
    (0x07B528, i_07b528),
    (0x07B52E, i_07b52e),
    (0x07B534, i_07b534),
    (0x07B53A, i_07b53a),
    (0x07B53E, i_07b53e),
    (0x07B544, i_07b544),
    (0x07B54A, i_07b54a),
    (0x07B550, i_07b550),
    (0x07B556, i_07b556),
    (0x07B55C, i_07b55c),
    (0x07B562, i_07b562),
    (0x07B568, i_07b568),
    (0x07B56C, i_07b56c),
    (0x07B572, i_07b572),
    (0x07B578, i_07b578),
    (0x07B57E, i_07b57e),
    (0x07B584, i_07b584),
    (0x07B58A, i_07b58a),
    (0x07B590, i_07b590),
    (0x07B596, i_07b596),
    (0x07B598, i_07b598),
    (0x07B59C, i_07b59c),
    (0x07B5A2, i_07b5a2),
    (0x07B5A8, i_07b5a8),
    (0x07B5AE, i_07b5ae),
    (0x07B5B4, i_07b5b4),
    (0x07B5BA, i_07b5ba),
    (0x07B5C0, i_07b5c0),
    (0x07B5C6, i_07b5c6),
    (0x07B5CA, i_07b5ca),
    (0x07B5D0, i_07b5d0),
    (0x07B5D6, i_07b5d6),
    (0x07B5DC, i_07b5dc),
    (0x07B5E2, i_07b5e2),
    (0x07B5E8, i_07b5e8),
    (0x07B5EE, i_07b5ee),
    (0x07B5F4, i_07b5f4),
    (0x07B5F8, i_07b5f8),
    (0x07B5FE, i_07b5fe),
    (0x07B604, i_07b604),
    (0x07B60A, i_07b60a),
    (0x07B610, i_07b610),
    (0x07B616, i_07b616),
    (0x07B61C, i_07b61c),
    (0x07B622, i_07b622),
    (0x07B626, i_07b626),
    (0x07B62C, i_07b62c),
    (0x07B632, i_07b632),
    (0x07B638, i_07b638),
    (0x07B63E, i_07b63e),
    (0x07B644, i_07b644),
    (0x07B64A, i_07b64a),
    (0x07B650, i_07b650),
    (0x07B654, i_07b654),
    (0x07B65A, i_07b65a),
    (0x07B660, i_07b660),
    (0x07B666, i_07b666),
    (0x07B66C, i_07b66c),
    (0x07B672, i_07b672),
    (0x07B678, i_07b678),
    (0x07B67E, i_07b67e),
    (0x07B682, i_07b682),
    (0x07B688, i_07b688),
    (0x07B68E, i_07b68e),
    (0x07B694, i_07b694),
    (0x07B69A, i_07b69a),
    (0x07B6A0, i_07b6a0),
    (0x07B6A6, i_07b6a6),
    (0x07B6AC, i_07b6ac),
    (0x07B6B0, i_07b6b0),
    (0x07B6B6, i_07b6b6),
    (0x07B6BC, i_07b6bc),
    (0x07B6C2, i_07b6c2),
    (0x07B6C8, i_07b6c8),
    (0x07B6CE, i_07b6ce),
    (0x07B6D4, i_07b6d4),
    (0x07B6DA, i_07b6da),
    (0x07B6DE, i_07b6de),
    (0x07B6E4, i_07b6e4),
    (0x07B6EA, i_07b6ea),
    (0x07B6F0, i_07b6f0),
    (0x07B6F6, i_07b6f6),
    (0x07B6FC, i_07b6fc),
    (0x07B702, i_07b702),
    (0x07B708, i_07b708),
    (0x07B70C, i_07b70c),
    (0x07B712, i_07b712),
    (0x07B718, i_07b718),
    (0x07B71E, i_07b71e),
    (0x07B724, i_07b724),
    (0x07B72A, i_07b72a),
    (0x07B730, i_07b730),
    (0x07B736, i_07b736),
    (0x07B73A, i_07b73a),
    (0x07B740, i_07b740),
    (0x07B746, i_07b746),
    (0x07B74C, i_07b74c),
    (0x07B752, i_07b752),
    (0x07B758, i_07b758),
    (0x07B75E, i_07b75e),
    (0x07B764, i_07b764),
    (0x07B768, i_07b768),
    (0x07B76E, i_07b76e),
    (0x07B774, i_07b774),
    (0x07B77A, i_07b77a),
    (0x07B780, i_07b780),
    (0x07B786, i_07b786),
    (0x07B78C, i_07b78c),
    (0x07B792, i_07b792),
    (0x07B796, i_07b796),
    (0x07B79C, i_07b79c),
    (0x07B7A2, i_07b7a2),
    (0x07B7A8, i_07b7a8),
    (0x07B7AE, i_07b7ae),
    (0x07B7B4, i_07b7b4),
    (0x07B7BA, i_07b7ba),
    (0x07B7C0, i_07b7c0),
    (0x07B7C4, i_07b7c4),
    (0x07B7CA, i_07b7ca),
    (0x07B7D0, i_07b7d0),
    (0x07B7D6, i_07b7d6),
    (0x07B7DC, i_07b7dc),
    (0x07B7E2, i_07b7e2),
    (0x07B7E8, i_07b7e8),
    (0x07B7EE, i_07b7ee),
    (0x07B7F2, i_07b7f2),
    (0x07B7F8, i_07b7f8),
    (0x07B7FE, i_07b7fe),
    (0x07B804, i_07b804),
    (0x07B80A, i_07b80a),
    (0x07B810, i_07b810),
    (0x07B816, i_07b816),
    (0x07B81C, i_07b81c),
    (0x07B820, i_07b820),
    (0x07B826, i_07b826),
    (0x07B82C, i_07b82c),
    (0x07B832, i_07b832),
    (0x07B838, i_07b838),
    (0x07B83E, i_07b83e),
    (0x07B844, i_07b844),
    (0x07B84A, i_07b84a),
    (0x07B84E, i_07b84e),
    (0x07B854, i_07b854),
    (0x07B85A, i_07b85a),
    (0x07B860, i_07b860),
    (0x07B866, i_07b866),
    (0x07B86C, i_07b86c),
    (0x07B872, i_07b872),
    (0x07B878, i_07b878),
    (0x07B87C, i_07b87c),
    (0x07B882, i_07b882),
    (0x07B888, i_07b888),
    (0x07B88E, i_07b88e),
    (0x07B894, i_07b894),
    (0x07B89A, i_07b89a),
    (0x07B8A0, i_07b8a0),
    (0x07B8A6, i_07b8a6),
    (0x07B8AA, i_07b8aa),
    (0x07B8B0, i_07b8b0),
    (0x07B8B6, i_07b8b6),
    (0x07B8BC, i_07b8bc),
    (0x07B8C2, i_07b8c2),
    (0x07B8C8, i_07b8c8),
    (0x07B8CE, i_07b8ce),
    (0x07B8D4, i_07b8d4),
    (0x07B8D8, i_07b8d8),
    (0x07B8DE, i_07b8de),
    (0x07B8E4, i_07b8e4),
    (0x07B8EA, i_07b8ea),
    (0x07B8F0, i_07b8f0),
    (0x07B8F6, i_07b8f6),
    (0x07B8FC, i_07b8fc),
    (0x07B902, i_07b902),
    (0x07B906, i_07b906),
    (0x07B90C, i_07b90c),
    (0x07B912, i_07b912),
    (0x07B918, i_07b918),
    (0x07B91E, i_07b91e),
    (0x07B924, i_07b924),
    (0x07B92A, i_07b92a),
    (0x07B930, i_07b930),
    (0x07B934, i_07b934),
    (0x07B93A, i_07b93a),
    (0x07B940, i_07b940),
    (0x07B946, i_07b946),
    (0x07B94C, i_07b94c),
    (0x07B952, i_07b952),
    (0x07B958, i_07b958),
    (0x07B95E, i_07b95e),
    (0x07B960, i_07b960),
    (0x07B964, i_07b964),
    (0x07B968, i_07b968),
    (0x07B96C, i_07b96c),
    (0x07B970, i_07b970),
    (0x07B974, i_07b974),
    (0x07B978, i_07b978),
    (0x07B97C, i_07b97c),
    (0x07B980, i_07b980),
    (0x07B982, i_07b982),
    (0x07B984, i_07b984),
    (0x07B988, i_07b988),
    (0x07B98C, i_07b98c),
    (0x07B992, i_07b992),
    (0x07B998, i_07b998),
    (0x07B99C, i_07b99c),
    (0x07B99E, i_07b99e),
    (0x07B9A2, i_07b9a2),
    (0x07B9A4, i_07b9a4),
    (0x07B9AA, i_07b9aa),
    (0x07B9AE, i_07b9ae),
    (0x07B9B2, i_07b9b2),
    (0x07B9B4, i_07b9b4),
    (0x07B9B8, i_07b9b8),
    (0x07B9BC, i_07b9bc),
    (0x07B9C0, i_07b9c0),
    (0x07B9C4, i_07b9c4),
    (0x07B9CA, i_07b9ca),
    (0x07B9CE, i_07b9ce),
    (0x07B9D2, i_07b9d2),
    (0x07B9D6, i_07b9d6),
    (0x07B9DA, i_07b9da),
    (0x07B9DE, i_07b9de),
    (0x07B9E4, i_07b9e4),
    (0x07B9E8, i_07b9e8),
    (0x07B9EA, i_07b9ea),
    (0x07B9EE, i_07b9ee),
    (0x07B9F2, i_07b9f2),
    (0x07B9F6, i_07b9f6),
    (0x07B9F8, i_07b9f8),
    (0x07B9FC, i_07b9fc),
    (0x07BA00, i_07ba00),
    (0x07BA04, i_07ba04),
    (0x07BA08, i_07ba08),
    (0x07BA0E, i_07ba0e),
    (0x07BA12, i_07ba12),
    (0x07BA16, i_07ba16),
    (0x07BA1C, i_07ba1c),
    (0x07BA22, i_07ba22),
    (0x07BA28, i_07ba28),
    (0x07BA2C, i_07ba2c),
    (0x07BA2E, i_07ba2e),
    (0x07BA32, i_07ba32),
    (0x07BA36, i_07ba36),
    (0x07BA3A, i_07ba3a),
    (0x07BA3E, i_07ba3e),
    (0x07BA44, i_07ba44),
    (0x07BA46, i_07ba46),
    (0x07BA48, i_07ba48),
    (0x07BA4C, i_07ba4c),
    (0x07BA4E, i_07ba4e),
    (0x07BA52, i_07ba52),
    (0x07BA54, i_07ba54),
    (0x07BA58, i_07ba58),
    (0x07BA5A, i_07ba5a),
    (0x07BA5E, i_07ba5e),
    (0x07BA60, i_07ba60),
    (0x07BA64, i_07ba64),
    (0x07BA68, i_07ba68),
    (0x07BA6C, i_07ba6c),
    (0x07BA70, i_07ba70),
    (0x07BA74, i_07ba74),
    (0x07BA76, i_07ba76),
    (0x07BA7A, i_07ba7a),
    (0x07BA7C, i_07ba7c),
    (0x07BA80, i_07ba80),
    (0x07BA86, i_07ba86),
    (0x07BA8A, i_07ba8a),
    (0x07BA90, i_07ba90),
    (0x07BA96, i_07ba96),
    (0x07BA98, i_07ba98),
    (0x07BA9C, i_07ba9c),
    (0x07BAA0, i_07baa0),
    (0x07BAA4, i_07baa4),
    (0x07BAAA, i_07baaa),
    (0x07BAB0, i_07bab0),
    (0x07BAB6, i_07bab6),
    (0x07BABC, i_07babc),
    (0x07BAC0, i_07bac0),
    (0x07BAC6, i_07bac6),
    (0x07BACA, i_07baca),
    (0x07BACC, i_07bacc),
    (0x07BACE, i_07bace),
    (0x07BAD2, i_07bad2),
    (0x07BAD8, i_07bad8),
    (0x07BADE, i_07bade),
    (0x07BAE4, i_07bae4),
    (0x07BAEA, i_07baea),
    (0x07BAEE, i_07baee),
    (0x07BAF0, i_07baf0),
    (0x07BAF6, i_07baf6),
    (0x07BAFC, i_07bafc),
    (0x07BB02, i_07bb02),
    (0x07BB08, i_07bb08),
    (0x07BB0C, i_07bb0c),
    (0x07BB12, i_07bb12),
    (0x07BB16, i_07bb16),
    (0x07BB18, i_07bb18),
    (0x07BB1A, i_07bb1a),
    (0x07BB1E, i_07bb1e),
    (0x07BB24, i_07bb24),
    (0x07BB2A, i_07bb2a),
    (0x07BB30, i_07bb30),
    (0x07BB34, i_07bb34),
    (0x07BB36, i_07bb36),
    (0x07BB3C, i_07bb3c),
    (0x07BB42, i_07bb42),
    (0x07BB46, i_07bb46),
    (0x07BB4A, i_07bb4a),
    (0x07BB4C, i_07bb4c),
    (0x07BB52, i_07bb52),
    (0x07BB54, i_07bb54),
    (0x07BB5A, i_07bb5a),
    (0x07BB5E, i_07bb5e),
    (0x07BB62, i_07bb62),
    (0x07BB68, i_07bb68),
    (0x07BB6E, i_07bb6e),
    (0x07BB70, i_07bb70),
    (0x07BB76, i_07bb76),
    (0x07BB78, i_07bb78),
    (0x07BB7C, i_07bb7c),
    (0x07BB82, i_07bb82),
    (0x07BB88, i_07bb88),
    (0x07BB8E, i_07bb8e),
    (0x07BB92, i_07bb92),
    (0x07BB98, i_07bb98),
    (0x07BB9E, i_07bb9e),
    (0x07BBA4, i_07bba4),
    (0x07BBAA, i_07bbaa),
    (0x07BBB0, i_07bbb0),
    (0x07BBB4, i_07bbb4),
    (0x07BBBA, i_07bbba),
    (0x07BBBE, i_07bbbe),
    (0x07BBC0, i_07bbc0),
    (0x07BBC4, i_07bbc4),
    (0x07BBCA, i_07bbca),
    (0x07BBD0, i_07bbd0),
    (0x07BBD4, i_07bbd4),
    (0x07BBD8, i_07bbd8),
    (0x07BBDC, i_07bbdc),
    (0x07BBE0, i_07bbe0),
    (0x07BBE2, i_07bbe2),
    (0x07BBE8, i_07bbe8),
    (0x07BBEA, i_07bbea),
    (0x07BBEE, i_07bbee),
    (0x07BBF4, i_07bbf4),
    (0x07BBF8, i_07bbf8),
    (0x07BBFE, i_07bbfe),
    (0x07BC00, i_07bc00),
    (0x07BC04, i_07bc04),
    (0x07BC08, i_07bc08),
];
