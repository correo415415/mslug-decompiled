// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07_7) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $07F21E: bcc.w $7f228
pub fn i_07f21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F222;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F228; return 10; }
    8
}
// ===== SetTaskHandler_07f222  ($07F222, 8 B, kernel/task_handlers.c)
/// $07F222: lea.l $7fdbc(pc), a1
pub fn i_07f222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F226;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F226: move.l a1, (a6)
pub fn i_07f226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F228;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F228: rts
pub fn i_07f228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F22A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Rider_Init_07f22a  ($07F22A, 72 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F22A: jsr $5e9b6.l
pub fn i_07f22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F230;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F230);
    cpu.pc = 0x5E9B6;
    16
}
/// $07F230: andi.w #$1, d0
pub fn i_07f230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F234;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07F234: move.b d0, $3a(a6)
pub fn i_07f234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F238;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07F238: jsr $5e9b6.l
pub fn i_07f238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F23E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F23E);
    cpu.pc = 0x5E9B6;
    16
}
/// $07F23E: andi.w #$3, d0
pub fn i_07f23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F242;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07F242: cmpi.w #$3, d0
pub fn i_07f242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F246;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $07F246: bne.w $7f24e
pub fn i_07f246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F24A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F24E; return 10; }
    8
}
/// $07F24A: move.w #$2, d0
pub fn i_07f24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F24E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F24E: move.w d0, $72(a6)
pub fn i_07f24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F252;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F252: move.w #$0, $76(a6)
pub fn i_07f252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F258;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F258: movea.l $c(a6), a0
pub fn i_07f258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F25C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F25C: move.w $24(a0), d0
pub fn i_07f25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F260;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07F260: subi.w #$c, d0
pub fn i_07f260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F264;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07F264: move.w d0, $70(a6)
pub fn i_07f264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F268;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F268: cmpi.w #$1, $72(a6)
pub fn i_07f268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F26E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $07F26E: bne.w $7f27a
pub fn i_07f26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F272;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F27A; return 10; }
    8
}
// ===== JsrAbsThunk_07f272  ($07F272, 8 B, thunks/jsr_abs_thunks.c)
/// $07F272: jsr $4a166.l
pub fn i_07f272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F278;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F278);
    cpu.pc = 0x4A166;
    16
}
/// $07F278: rts
pub fn i_07f278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F27A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_07f27a  ($07F27A, 8 B, kernel/task_handlers.c)
/// $07F27A: lea.l $7f2ca(pc), a1
pub fn i_07f27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F27E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F2CA;
    16
}
/// $07F27E: move.l a1, (a6)
pub fn i_07f27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F280;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F280: rts
pub fn i_07f280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F282;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Rider_Ride_07f282  ($07F282, 64 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F282: move.w #$28, d1
pub fn i_07f282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F286;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F286: jsr $236e.l
pub fn i_07f286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F28C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F28C);
    cpu.pc = 0x236E;
    16
}
/// $07F28C: lea.l $29c77c.l, a0
pub fn i_07f28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F292;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C77C;
    20
}
/// $07F292: jsr $28cd4.l
pub fn i_07f292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F298);
    cpu.pc = 0x28CD4;
    16
}
/// $07F298: lea.l $7f29e(pc), a1
pub fn i_07f298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F29C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F29E;
    16
}
/// $07F29C: move.l a1, (a6)
pub fn i_07f29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F29E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F29E: jsr $7fdfa(pc)
pub fn i_07f29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F2A2);
    cpu.pc = 0x7FDFA;
    16
}
/// $07F2A2: jsr $28d70.l
pub fn i_07f2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F2A8);
    cpu.pc = 0x28D70;
    16
}
/// $07F2A8: bcc.w $7f2b2
pub fn i_07f2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F2B2; return 10; }
    8
}
/// $07F2AC: lea.l $7f2ca(pc), a1
pub fn i_07f2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F2CA;
    16
}
/// $07F2B0: move.l a1, (a6)
pub fn i_07f2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F2B2: movea.l #$ffffffff, a0
pub fn i_07f2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2B8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07F2B8: jsr $5dd56.l
pub fn i_07f2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F2BE);
    cpu.pc = 0x5DD56;
    16
}
/// $07F2BE: bcc.w $7f2c8
pub fn i_07f2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F2C8; return 10; }
    8
}
// ===== SetTaskHandler_07f2c2  ($07F2C2, 8 B, kernel/task_handlers.c)
/// $07F2C2: lea.l $7fdbc(pc), a1
pub fn i_07f2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F2C6: move.l a1, (a6)
pub fn i_07f2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F2C8: rts
pub fn i_07f2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Rider_Jump_07f2ca  ($07F2CA, 90 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F2CA: jsr $13600.l
pub fn i_07f2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F2D0);
    cpu.pc = 0x13600;
    16
}
/// $07F2D0: move.w #$28, d1
pub fn i_07f2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F2D4: jsr $236e.l
pub fn i_07f2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F2DA);
    cpu.pc = 0x236E;
    16
}
/// $07F2DA: move.w #$c000, d0
pub fn i_07f2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F2DE: jsr $28134.l
pub fn i_07f2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F2E4);
    cpu.pc = 0x28134;
    16
}
/// $07F2E4: andi.w #$ffe3, $38(a6)
pub fn i_07f2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2EA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F2EA: ori.w #$10, $38(a6)
pub fn i_07f2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2F0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F2F0: clr.b $20(a6)
pub fn i_07f2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2F4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07F2F4: jsr $267e2.l
pub fn i_07f2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F2FA);
    cpu.pc = 0x267E2;
    16
}
/// $07F2FA: lea.l $7f324(pc), a1
pub fn i_07f2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F2FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F324;
    16
}
/// $07F2FE: move.l a1, (a6)
pub fn i_07f2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F300;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F300: tst.w $72(a6)
pub fn i_07f300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F304;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07F304: bne.w $7f318
pub fn i_07f304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F308;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F318; return 10; }
    8
}
/// $07F308: lea.l $29c7c0.l, a0
pub fn i_07f308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F30E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C7C0;
    20
}
/// $07F30E: jsr $28cd4.l
pub fn i_07f30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F314;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F314);
    cpu.pc = 0x28CD4;
    16
}
/// $07F314: bra.w $7f324
pub fn i_07f314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F318;
    cpu.ir = 0x6000;
    cpu.pc = 0x7F324;
    10
}
/// $07F318: lea.l $29c858.l, a0
pub fn i_07f318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F31E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C858;
    20
}
/// $07F31E: jsr $28cd4.l
pub fn i_07f31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F324;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F324);
    cpu.pc = 0x28CD4;
    16
}
// ===== Carrier_Rider_JumpRun_07f324  ($07F324, 26 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F324: tst.b $20(a6)
pub fn i_07f324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F328;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07F328: bne.w $7f334
pub fn i_07f328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F32C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F334; return 10; }
    8
}
/// $07F32C: jsr $7fdfa(pc)
pub fn i_07f32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F330;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F330);
    cpu.pc = 0x7FDFA;
    16
}
/// $07F330: jmp $7f3ae(pc)
pub fn i_07f330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F334;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x7F3AE;
    8
}
/// $07F334: lea.l $7f3a8(pc), a1
pub fn i_07f334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F338;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F3A8;
    16
}
/// $07F338: move.l a1, (a6)
pub fn i_07f338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F33A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F33A: jmp $7f3a8(pc)
pub fn i_07f33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F33E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x7F3A8;
    8
}
// ===== Carrier_Rider_JumpB_07f33e  ($07F33E, 84 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F33E: move.w #$28, d1
pub fn i_07f33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F342;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F342: jsr $236e.l
pub fn i_07f342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F348;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F348);
    cpu.pc = 0x236E;
    16
}
/// $07F348: move.w #$c000, d0
pub fn i_07f348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F34C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F34C: jsr $28134.l
pub fn i_07f34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F352);
    cpu.pc = 0x28134;
    16
}
/// $07F352: andi.w #$ffe3, $38(a6)
pub fn i_07f352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F358;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F358: ori.w #$10, $38(a6)
pub fn i_07f358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F35E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F35E: clr.b $20(a6)
pub fn i_07f35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F362;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07F362: jsr $267e2.l
pub fn i_07f362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F368;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F368);
    cpu.pc = 0x267E2;
    16
}
/// $07F368: lea.l $7f392(pc), a1
pub fn i_07f368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F36C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F392;
    16
}
/// $07F36C: move.l a1, (a6)
pub fn i_07f36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F36E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F36E: tst.w $72(a6)
pub fn i_07f36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F372;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07F372: bne.w $7f386
pub fn i_07f372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F376;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F386; return 10; }
    8
}
/// $07F376: lea.l $29c7c0.l, a0
pub fn i_07f376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F37C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C7C0;
    20
}
/// $07F37C: jsr $28cd4.l
pub fn i_07f37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F382;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F382);
    cpu.pc = 0x28CD4;
    16
}
/// $07F382: bra.w $7f392
pub fn i_07f382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F386;
    cpu.ir = 0x6000;
    cpu.pc = 0x7F392;
    10
}
/// $07F386: lea.l $29c858.l, a0
pub fn i_07f386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F38C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C858;
    20
}
/// $07F38C: jsr $28cd4.l
pub fn i_07f38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F392;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F392);
    cpu.pc = 0x28CD4;
    16
}
// ===== Carrier_Rider_JumpBRun_07f392  ($07F392, 22 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F392: tst.b $20(a6)
pub fn i_07f392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F396;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07F396: bne.w $7f3a2
pub fn i_07f396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F39A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F3A2; return 10; }
    8
}
/// $07F39A: jsr $7fe66(pc)
pub fn i_07f39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F39E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F39E);
    cpu.pc = 0x7FE66;
    16
}
/// $07F39E: jmp $7f3ae(pc)
pub fn i_07f39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3A2;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x7F3AE;
    8
}
/// $07F3A2: lea.l $7f3a8(pc), a1
pub fn i_07f3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F3A8;
    16
}
/// $07F3A6: move.l a1, (a6)
pub fn i_07f3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Carrier_Rider_Fall_07f3a8  ($07F3A8, 56 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F3A8: jsr $27cee.l
pub fn i_07f3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F3AE);
    cpu.pc = 0x27CEE;
    16
}
/// $07F3AE: jsr $28d70.l
pub fn i_07f3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F3B4);
    cpu.pc = 0x28D70;
    16
}
/// $07F3B4: move.w $70(a6), d0
pub fn i_07f3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3B8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07F3B8: subi.w #$10, d0
pub fn i_07f3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3BC;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07F3BC: cmp.w $24(a6), d0
pub fn i_07f3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3C0;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07F3C0: blt.w $7f3d0
pub fn i_07f3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3C4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7F3D0; return 10; }
    8
}
/// $07F3C4: move.w $70(a6), $24(a6)
pub fn i_07f3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3CA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07F3CA: lea.l $7f3e8(pc), a1
pub fn i_07f3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F3E8;
    16
}
/// $07F3CE: move.l a1, (a6)
pub fn i_07f3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F3D0: movea.l #$ffffffff, a0
pub fn i_07f3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3D6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07F3D6: jsr $5dd56.l
pub fn i_07f3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F3DC);
    cpu.pc = 0x5DD56;
    16
}
/// $07F3DC: bcc.w $7f3e6
pub fn i_07f3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F3E6; return 10; }
    8
}
// ===== SetTaskHandler_07f3e0  ($07F3E0, 8 B, kernel/task_handlers.c)
/// $07F3E0: lea.l $7fdbc(pc), a1
pub fn i_07f3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F3E4: move.l a1, (a6)
pub fn i_07f3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F3E6: rts
pub fn i_07f3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Rider_Land_07f3e8  ($07F3E8, 116 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F3E8: move.w #$c, d1
pub fn i_07f3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3EC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F3EC: jsr $236e.l
pub fn i_07f3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F3F2);
    cpu.pc = 0x236E;
    16
}
/// $07F3F2: move.w #$c000, d0
pub fn i_07f3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3F6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F3F6: jsr $28134.l
pub fn i_07f3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F3FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F3FC);
    cpu.pc = 0x28134;
    16
}
/// $07F3FC: andi.w #$ffe3, $38(a6)
pub fn i_07f3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F402;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F402: ori.w #$10, $38(a6)
pub fn i_07f402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F408;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F408: jsr $267e2.l
pub fn i_07f408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F40E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F40E);
    cpu.pc = 0x267E2;
    16
}
/// $07F40E: btst.b #$0, $3a(a6)
pub fn i_07f40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F414;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07F414: beq.w $7f41c
pub fn i_07f414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F418;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7F41C; return 10; }
    8
}
/// $07F418: neg.w $28(a6)
pub fn i_07f418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F41C;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07F41C: lea.l $2de7c2.l, a0
pub fn i_07f41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F422;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE7C2;
    20
}
/// $07F422: jsr $28cd4.l
pub fn i_07f422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F428;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F428);
    cpu.pc = 0x28CD4;
    16
}
/// $07F428: lea.l $ffff.w, a0
pub fn i_07f428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F42C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $07F42C: move.l a0, $48(a6)
pub fn i_07f42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F430;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07F430: lea.l $7f436(pc), a1
pub fn i_07f430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F434;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F436;
    16
}
/// $07F434: move.l a1, (a6)
pub fn i_07f434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F436;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F436: jsr $27cee.l
pub fn i_07f436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F43C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F43C);
    cpu.pc = 0x27CEE;
    16
}
/// $07F43C: jsr $28d70.l
pub fn i_07f43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F442);
    cpu.pc = 0x28D70;
    16
}
/// $07F442: bcc.w $7f44c
pub fn i_07f442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F446;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F44C; return 10; }
    8
}
/// $07F446: lea.l $7f464(pc), a1
pub fn i_07f446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F44A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F464;
    16
}
/// $07F44A: move.l a1, (a6)
pub fn i_07f44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F44C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F44C: movea.l #$ffffffff, a0
pub fn i_07f44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F452;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07F452: jsr $5dd56.l
pub fn i_07f452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F458;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F458);
    cpu.pc = 0x5DD56;
    16
}
/// $07F458: bcc.w $7f462
pub fn i_07f458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F45C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F462; return 10; }
    8
}
// ===== SetTaskHandler_07f45c  ($07F45C, 8 B, kernel/task_handlers.c)
/// $07F45C: lea.l $7fdbc(pc), a1
pub fn i_07f45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F460;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F460: move.l a1, (a6)
pub fn i_07f460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F462;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F462: rts
pub fn i_07f462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F464;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Rider_Flee_07f464  ($07F464, 90 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F464: jsr $267e2.l
pub fn i_07f464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F46A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F46A);
    cpu.pc = 0x267E2;
    16
}
/// $07F46A: move.w #$ffa0, $28(a6)
pub fn i_07f46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F470;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F470: btst.b #$0, $3a(a6)
pub fn i_07f470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F476;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07F476: beq.w $7f47e
pub fn i_07f476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F47A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7F47E; return 10; }
    8
}
/// $07F47A: neg.w $28(a6)
pub fn i_07f47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F47E;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07F47E: lea.l $2de6e0.l, a0
pub fn i_07f47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F484;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE6E0;
    20
}
/// $07F484: jsr $28cd4.l
pub fn i_07f484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F48A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F48A);
    cpu.pc = 0x28CD4;
    16
}
/// $07F48A: lea.l $ffff.w, a0
pub fn i_07f48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F48E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $07F48E: move.l a0, $48(a6)
pub fn i_07f48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F492;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07F492: lea.l $7f498(pc), a1
pub fn i_07f492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F496;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F498;
    16
}
/// $07F496: move.l a1, (a6)
pub fn i_07f496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F498;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F498: jsr $27cee.l
pub fn i_07f498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F49E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F49E);
    cpu.pc = 0x27CEE;
    16
}
/// $07F49E: jsr $28d70.l
pub fn i_07f49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F4A4);
    cpu.pc = 0x28D70;
    16
}
/// $07F4A4: bcc.w $7f4ae
pub fn i_07f4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F4AE; return 10; }
    8
}
/// $07F4A8: lea.l $7f4c6(pc), a1
pub fn i_07f4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F4C6;
    16
}
/// $07F4AC: move.l a1, (a6)
pub fn i_07f4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F4AE: movea.l #$ffffffff, a0
pub fn i_07f4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4B4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07F4B4: jsr $5dd56.l
pub fn i_07f4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F4BA);
    cpu.pc = 0x5DD56;
    16
}
/// $07F4BA: bcc.w $7f4c4
pub fn i_07f4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F4C4; return 10; }
    8
}
// ===== SetTaskHandler_07f4be  ($07F4BE, 8 B, kernel/task_handlers.c)
/// $07F4BE: lea.l $7fdbc(pc), a1
pub fn i_07f4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F4C2: move.l a1, (a6)
pub fn i_07f4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F4C4: rts
pub fn i_07f4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Rider_Dive_07f4c6  ($07F4C6, 100 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F4C6: move.w #$28, d1
pub fn i_07f4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4CA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F4CA: jsr $236e.l
pub fn i_07f4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F4D0);
    cpu.pc = 0x236E;
    16
}
/// $07F4D0: subi.w #$18, $24(a6)
pub fn i_07f4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4D6;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F4D6: move.w #$8000, d0
pub fn i_07f4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4DA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F4DA: jsr $28134.l
pub fn i_07f4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F4E0);
    cpu.pc = 0x28134;
    16
}
/// $07F4E0: andi.w #$ffe3, $38(a6)
pub fn i_07f4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4E6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F4E6: ori.w #$c, $38(a6)
pub fn i_07f4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4EC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F4EC: lea.l $29c962.l, a0
pub fn i_07f4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C962;
    20
}
/// $07F4F2: jsr $28cd4.l
pub fn i_07f4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F4F8);
    cpu.pc = 0x28CD4;
    16
}
/// $07F4F8: lea.l $7f4fe(pc), a1
pub fn i_07f4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F4FE;
    16
}
/// $07F4FC: move.l a1, (a6)
pub fn i_07f4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F4FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F4FE: jsr $27cee.l
pub fn i_07f4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F504;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F504);
    cpu.pc = 0x27CEE;
    16
}
/// $07F504: jsr $28d70.l
pub fn i_07f504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F50A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F50A);
    cpu.pc = 0x28D70;
    16
}
/// $07F50A: bcc.w $7f514
pub fn i_07f50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F50E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F514; return 10; }
    8
}
/// $07F50E: lea.l $7f532(pc), a1
pub fn i_07f50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F512;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F532;
    16
}
/// $07F512: move.l a1, (a6)
pub fn i_07f512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F514;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F514: movea.l #$ffffffff, a0
pub fn i_07f514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F51A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07F51A: lea.l $2e2ae0.l, a0
pub fn i_07f51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F520;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2AE0;
    20
}
/// $07F520: jsr $5dd56.l
pub fn i_07f520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F526;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F526);
    cpu.pc = 0x5DD56;
    16
}
/// $07F526: bcc.w $7f530
pub fn i_07f526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F52A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F530; return 10; }
    8
}
// ===== SetTaskHandler_07f52a  ($07F52A, 8 B, kernel/task_handlers.c)
/// $07F52A: lea.l $7fdbc(pc), a1
pub fn i_07f52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F52E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F52E: move.l a1, (a6)
pub fn i_07f52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F530;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F530: rts
pub fn i_07f530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F532;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Rider_DiveRun_07f532  ($07F532, 106 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F532: lea.l $29c9c2.l, a0
pub fn i_07f532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F538;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C9C2;
    20
}
/// $07F538: jsr $28cd4.l
pub fn i_07f538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F53E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F53E);
    cpu.pc = 0x28CD4;
    16
}
/// $07F53E: move.w #$0, $70(a6)
pub fn i_07f53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F544;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F544: lea.l $7f54a(pc), a1
pub fn i_07f544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F548;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F54A;
    16
}
/// $07F548: move.l a1, (a6)
pub fn i_07f548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F54A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F54A: jsr $27cee.l
pub fn i_07f54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F550);
    cpu.pc = 0x27CEE;
    16
}
/// $07F550: move.w $28(a6), d0
pub fn i_07f550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F554;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07F554: add.w $70(a6), d0
pub fn i_07f554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F558;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07F558: move.w d0, $70(a6)
pub fn i_07f558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F55C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F55C: bpl.w $7f562
pub fn i_07f55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F560;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7F562; return 10; }
    8
}
/// $07F560: neg.w d0
pub fn i_07f560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F562;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07F562: cmpi.w #$4000, d0
pub fn i_07f562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F566;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x4000, Size::Word, false);
    8
}
/// $07F566: blt.w $7f570
pub fn i_07f566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F56A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7F570; return 10; }
    8
}
/// $07F56A: move.w #$ffa0, $2a(a6)
pub fn i_07f56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F570;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F570: jsr $28d70.l
pub fn i_07f570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F576;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F576);
    cpu.pc = 0x28D70;
    16
}
/// $07F576: jsr $2870a.l
pub fn i_07f576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F57C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F57C);
    cpu.pc = 0x2870A;
    16
}
/// $07F57C: bcc.w $7f586
pub fn i_07f57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F580;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F586; return 10; }
    8
}
/// $07F580: lea.l $7f5a4(pc), a1
pub fn i_07f580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F584;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F5A4;
    16
}
/// $07F584: move.l a1, (a6)
pub fn i_07f584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F586;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F586: movea.l #$ffffffff, a0
pub fn i_07f586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F58C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07F58C: lea.l $2e2ae0.l, a0
pub fn i_07f58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F592;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2AE0;
    20
}
/// $07F592: jsr $5dd56.l
pub fn i_07f592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F598;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F598);
    cpu.pc = 0x5DD56;
    16
}
/// $07F598: bcc.w $7f5a2
pub fn i_07f598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F59C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F5A2; return 10; }
    8
}
// ===== SetTaskHandler_07f59c  ($07F59C, 8 B, kernel/task_handlers.c)
/// $07F59C: lea.l $7fdbc(pc), a1
pub fn i_07f59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F5A0: move.l a1, (a6)
pub fn i_07f5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F5A2: rts
pub fn i_07f5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Rider_DiveLand_07f5a4  ($07F5A4, 62 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F5A4: lea.l $29c992.l, a0
pub fn i_07f5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C992;
    20
}
/// $07F5AA: jsr $28cd4.l
pub fn i_07f5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F5B0);
    cpu.pc = 0x28CD4;
    16
}
/// $07F5B0: lea.l $7f5b6(pc), a1
pub fn i_07f5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F5B6;
    16
}
/// $07F5B4: move.l a1, (a6)
pub fn i_07f5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F5B6: jsr $27cee.l
pub fn i_07f5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F5BC);
    cpu.pc = 0x27CEE;
    16
}
/// $07F5BC: jsr $28d70.l
pub fn i_07f5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F5C2);
    cpu.pc = 0x28D70;
    16
}
/// $07F5C2: bcc.w $7f5cc
pub fn i_07f5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F5CC; return 10; }
    8
}
/// $07F5C6: lea.l $7f5ea(pc), a1
pub fn i_07f5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F5EA;
    16
}
/// $07F5CA: move.l a1, (a6)
pub fn i_07f5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F5CC: movea.l #$ffffffff, a0
pub fn i_07f5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5D2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07F5D2: lea.l $2e2ae0.l, a0
pub fn i_07f5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2AE0;
    20
}
/// $07F5D8: jsr $5dd56.l
pub fn i_07f5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F5DE);
    cpu.pc = 0x5DD56;
    16
}
/// $07F5DE: bcc.w $7f5e8
pub fn i_07f5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F5E8; return 10; }
    8
}
// ===== SetTaskHandler_07f5e2  ($07F5E2, 8 B, kernel/task_handlers.c)
/// $07F5E2: lea.l $7fdbc(pc), a1
pub fn i_07f5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F5E6: move.l a1, (a6)
pub fn i_07f5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F5E8: rts
pub fn i_07f5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Rider_FleeB_07f5ea  ($07F5EA, 74 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F5EA: addi.w #$18, $24(a6)
pub fn i_07f5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5F0;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F5F0: jsr $267e2.l
pub fn i_07f5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F5F6);
    cpu.pc = 0x267E2;
    16
}
/// $07F5F6: lea.l $2de6e0.l, a0
pub fn i_07f5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F5FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE6E0;
    20
}
/// $07F5FC: jsr $28cd4.l
pub fn i_07f5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F602);
    cpu.pc = 0x28CD4;
    16
}
/// $07F602: lea.l $7f608(pc), a1
pub fn i_07f602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F606;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F608;
    16
}
/// $07F606: move.l a1, (a6)
pub fn i_07f606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F608;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F608: jsr $2783a.l
pub fn i_07f608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F60E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F60E);
    cpu.pc = 0x2783A;
    16
}
/// $07F60E: jsr $28d70.l
pub fn i_07f60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F614;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F614);
    cpu.pc = 0x28D70;
    16
}
/// $07F614: bcc.w $7f61e
pub fn i_07f614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F618;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F61E; return 10; }
    8
}
/// $07F618: lea.l $7fdbc(pc), a1
pub fn i_07f618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F61C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F61C: move.l a1, (a6)
pub fn i_07f61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F61E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F61E: movea.l #$ffffffff, a0
pub fn i_07f61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F624;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07F624: lea.l $2e2ae0.l, a0
pub fn i_07f624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F62A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2AE0;
    20
}
/// $07F62A: jsr $5dd56.l
pub fn i_07f62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F630;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F630);
    cpu.pc = 0x5DD56;
    16
}
/// $07F630: bcc.w $7f63a
pub fn i_07f630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F634;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F63A; return 10; }
    8
}
// ===== SetTaskHandler_07f634  ($07F634, 8 B, kernel/task_handlers.c)
/// $07F634: lea.l $7fdbc(pc), a1
pub fn i_07f634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F638;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F638: move.l a1, (a6)
pub fn i_07f638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F63A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F63A: rts
pub fn i_07f63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F63C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Rider_ParentDeadCheck_07f63c  ($07F63C, 40 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F63C: movea.l $c(a6), a0
pub fn i_07f63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F640;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F640: cmpi.b #$ff, $20(a0)
pub fn i_07f640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F646;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07F646: bne.w $7f66a
pub fn i_07f646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F64A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F66A; return 10; }
    8
}
/// $07F64A: movea.l $c(a6), a0
pub fn i_07f64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F64E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F64E: move.w $24(a0), d0
pub fn i_07f64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F652;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07F652: subi.w #$c, d0
pub fn i_07f652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F656;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07F656: move.w d0, $70(a6)
pub fn i_07f656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F65A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F65A: cmpi.w #$1, $72(a6)
pub fn i_07f65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F660;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $07F660: bne.w $7f66c
pub fn i_07f660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F664;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F66C; return 10; }
    8
}
// ===== JsrAbsThunk_07f664  ($07F664, 8 B, thunks/jsr_abs_thunks.c)
/// $07F664: jsr $4a166.l
pub fn i_07f664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F66A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F66A);
    cpu.pc = 0x4A166;
    16
}
/// $07F66A: rts
pub fn i_07f66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F66C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_07f66c  ($07F66C, 8 B, kernel/task_handlers.c)
/// $07F66C: lea.l $7f282(pc), a1
pub fn i_07f66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F670;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F282;
    16
}
/// $07F670: move.l a1, (a6)
pub fn i_07f670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F672;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F672: rts
pub fn i_07f672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F674;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Init_07f674  ($07F674, 236 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F674: lea.l $7f84a(pc), a1
pub fn i_07f674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F678;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F84A;
    16
}
/// $07F678: jsr $4ae.l
pub fn i_07f678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F67E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F67E);
    cpu.pc = 0x4AE;
    16
}
/// $07F67E: jsr $5dd02.l
pub fn i_07f67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F684;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F684);
    cpu.pc = 0x5DD02;
    16
}
/// $07F684: lea.l $7f768(pc), a1
pub fn i_07f684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F688;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F768;
    16
}
/// $07F688: jsr $4ae.l
pub fn i_07f688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F68E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F68E);
    cpu.pc = 0x4AE;
    16
}
/// $07F68E: jsr $5dd02.l
pub fn i_07f68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F694;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F694);
    cpu.pc = 0x5DD02;
    16
}
/// $07F694: moveq #$0, d0
pub fn i_07f694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F696;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07F696: move.b d0, $20(a6)
pub fn i_07f696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F69A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07F69A: move.b #$1, $21(a6)
pub fn i_07f69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6A0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07F6A0: move.w d0, $70(a6)
pub fn i_07f6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6A4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F6A4: move.w #$28, $72(a6)
pub fn i_07f6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6AA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F6AA: move.w #$f0, $74(a6)
pub fn i_07f6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F6B0: move.w #$3c, $76(a6)
pub fn i_07f6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F6B6: move.b d0, $78(a6)
pub fn i_07f6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6BA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07F6BA: lea.l $7f6c0(pc), a1
pub fn i_07f6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F6C0;
    16
}
/// $07F6BE: move.l a1, (a6)
pub fn i_07f6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F6C0: btst.b #$0, $21(a6)
pub fn i_07f6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6C6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07F6C6: bne.w $7f6e8
pub fn i_07f6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6CA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F6E8; return 10; }
    8
}
/// $07F6CA: subq.w #$1, $72(a6)
pub fn i_07f6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6CE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07F6CE: bne.w $7f6e8
pub fn i_07f6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F6E8; return 10; }
    8
}
/// $07F6D2: move.w #$28, $72(a6)
pub fn i_07f6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F6D8: lea.l $7f768(pc), a1
pub fn i_07f6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F768;
    16
}
/// $07F6DC: jsr $4ae.l
pub fn i_07f6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F6E2);
    cpu.pc = 0x4AE;
    16
}
/// $07F6E2: jsr $5dd02.l
pub fn i_07f6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F6E8);
    cpu.pc = 0x5DD02;
    16
}
/// $07F6E8: btst.b #$1, $21(a6)
pub fn i_07f6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6EE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07F6EE: bne.w $7f710
pub fn i_07f6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6F2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F710; return 10; }
    8
}
/// $07F6F2: subq.w #$1, $76(a6)
pub fn i_07f6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6F6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07F6F6: bne.w $7f710
pub fn i_07f6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F6FA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F710; return 10; }
    8
}
/// $07F6FA: move.w #$3c, $76(a6)
pub fn i_07f6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F700;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F700: lea.l $7f8dc(pc), a1
pub fn i_07f700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F704;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F8DC;
    16
}
/// $07F704: jsr $4ae.l
pub fn i_07f704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F70A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F70A);
    cpu.pc = 0x4AE;
    16
}
/// $07F70A: jsr $5dd02.l
pub fn i_07f70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F710);
    cpu.pc = 0x5DD02;
    16
}
/// $07F710: subq.w #$1, $74(a6)
pub fn i_07f710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F714;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07F714: bne.w $7f73e
pub fn i_07f714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F718;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F73E; return 10; }
    8
}
/// $07F718: jsr $5e9b6.l
pub fn i_07f718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F71E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F71E);
    cpu.pc = 0x5E9B6;
    16
}
/// $07F71E: andi.w #$7f, d0
pub fn i_07f71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F722;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07F722: subi.w #$3f, d0
pub fn i_07f722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F726;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x3F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07F726: addi.w #$258, d0
pub fn i_07f726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F72A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x258, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07F72A: move.w d0, $74(a6)
pub fn i_07f72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F72E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F72E: lea.l $7f902(pc), a1
pub fn i_07f72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F732;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F902;
    16
}
/// $07F732: jsr $4ae.l
pub fn i_07f732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F738;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F738);
    cpu.pc = 0x4AE;
    16
}
/// $07F738: jsr $5dd02.l
pub fn i_07f738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F73E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F73E);
    cpu.pc = 0x5DD02;
    16
}
/// $07F73E: movea.l $c(a6), a0
pub fn i_07f73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F742;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F742: cmpi.b #$3, $20(a0)
pub fn i_07f742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F748;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $07F748: beq.w $7f758
pub fn i_07f748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F74C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7F758; return 10; }
    8
}
/// $07F74C: move.b #$ff, $20(a6)
pub fn i_07f74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F752;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07F752: lea.l $7fdbc(pc), a1
pub fn i_07f752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F756;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F756: move.l a1, (a6)
pub fn i_07f756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F758;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F758: jsr $7fde0(pc)
pub fn i_07f758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F75C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F75C);
    cpu.pc = 0x7FDE0;
    16
}
/// $07F75C: bcc.w $7f766
pub fn i_07f75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F760;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F766; return 10; }
    8
}
// ===== SetTaskHandler_07f760  ($07F760, 8 B, kernel/task_handlers.c)
/// $07F760: lea.l $7fdb6(pc), a1
pub fn i_07f760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F764;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDB6;
    16
}
/// $07F764: move.l a1, (a6)
pub fn i_07f764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F766;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F766: rts
pub fn i_07f766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F768;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Reload_07f768  ($07F768, 128 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F768: move.w #$e, d1
pub fn i_07f768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F76C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F76C: jsr $236e.l
pub fn i_07f76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F772;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F772);
    cpu.pc = 0x236E;
    16
}
/// $07F772: addi.w #$18, $38(a6)
pub fn i_07f772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F778;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F778: movea.l $c(a6), a0
pub fn i_07f778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F77C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F77C: move.b #$1, $78(a0)
pub fn i_07f77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F782;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07F782: bset.b #$0, $21(a0)
pub fn i_07f782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F788;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07F788: move.b #$ff, $21(a6)
pub fn i_07f788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F78E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07F78E: jsr $267e2.l
pub fn i_07f78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F794;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F794);
    cpu.pc = 0x267E2;
    16
}
/// $07F794: move.w #$0, $76(a6)
pub fn i_07f794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F79A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F79A: move.w #$0, $78(a6)
pub fn i_07f79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7A0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F7A0: lea.l $2e2286.l, a0
pub fn i_07f7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2286;
    20
}
/// $07F7A6: jsr $28cd4.l
pub fn i_07f7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F7AC);
    cpu.pc = 0x28CD4;
    16
}
/// $07F7AC: lea.l $7f7b2(pc), a1
pub fn i_07f7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F7B2;
    16
}
/// $07F7B0: move.l a1, (a6)
pub fn i_07f7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F7B2: movea.l $c(a6), a0
pub fn i_07f7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7B6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F7B6: movea.l $c(a0), a0
pub fn i_07f7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7BA;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F7BA: move.w $78(a0), $22(a6)
pub fn i_07f7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7C0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07F7C0: move.w $7a(a0), $24(a6)
pub fn i_07f7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7C6;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07F7C6: subi.w #$9, $22(a6)
pub fn i_07f7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7CC;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x9, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F7CC: jsr $28d70.l
pub fn i_07f7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F7D2);
    cpu.pc = 0x28D70;
    16
}
/// $07F7D2: bcc.w $7f7dc
pub fn i_07f7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F7DC; return 10; }
    8
}
/// $07F7D6: lea.l $7f7f0(pc), a1
pub fn i_07f7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F7F0;
    16
}
/// $07F7DA: move.l a1, (a6)
pub fn i_07f7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F7DC: jsr $80054(pc)
pub fn i_07f7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F7E0);
    cpu.pc = 0x80054;
    16
}
/// $07F7E0: jsr $7fde0(pc)
pub fn i_07f7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F7E4);
    cpu.pc = 0x7FDE0;
    16
}
/// $07F7E4: bcc.w $7f7ee
pub fn i_07f7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F7EE; return 10; }
    8
}
// ===== SetTaskHandler_07f7e8  ($07F7E8, 8 B, kernel/task_handlers.c)
/// $07F7E8: lea.l $7fdbc(pc), a1
pub fn i_07f7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F7EC: move.l a1, (a6)
pub fn i_07f7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F7EE: rts
pub fn i_07f7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Hold_07f7f0  ($07F7F0, 82 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F7F0: movea.l $c(a6), a0
pub fn i_07f7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7F4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F7F4: move.b #$0, $78(a0)
pub fn i_07f7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F7FA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07F7FA: lea.l $2e2128.l, a0
pub fn i_07f7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F800;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2128;
    20
}
/// $07F800: jsr $28cd4.l
pub fn i_07f800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F806);
    cpu.pc = 0x28CD4;
    16
}
/// $07F806: lea.l $7f80c(pc), a1
pub fn i_07f806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F80A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F80C;
    16
}
/// $07F80A: move.l a1, (a6)
pub fn i_07f80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F80C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F80C: movea.l $c(a6), a0
pub fn i_07f80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F810;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F810: movea.l $c(a0), a0
pub fn i_07f810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F814;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F814: move.w $78(a0), $22(a6)
pub fn i_07f814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F81A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07F81A: move.w $7a(a0), $24(a6)
pub fn i_07f81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F820;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07F820: subi.w #$9, $22(a6)
pub fn i_07f820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F826;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x9, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F826: jsr $28d70.l
pub fn i_07f826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F82C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F82C);
    cpu.pc = 0x28D70;
    16
}
/// $07F82C: movea.l $c(a6), a0
pub fn i_07f82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F830;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F830: move.w $70(a6), $70(a0)
pub fn i_07f830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F836;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07F836: jsr $80054(pc)
pub fn i_07f836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F83A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F83A);
    cpu.pc = 0x80054;
    16
}
/// $07F83A: jsr $7fde0(pc)
pub fn i_07f83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F83E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F83E);
    cpu.pc = 0x7FDE0;
    16
}
/// $07F83E: bcc.w $7f848
pub fn i_07f83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F842;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F848; return 10; }
    8
}
// ===== SetTaskHandler_07f842  ($07F842, 8 B, kernel/task_handlers.c)
/// $07F842: lea.l $7fdbc(pc), a1
pub fn i_07f842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F846;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F846: move.l a1, (a6)
pub fn i_07f846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F848;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F848: rts
pub fn i_07f848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F84A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Pose_07f84a  ($07F84A, 42 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F84A: move.w #$26, d1
pub fn i_07f84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F84E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F84E: jsr $236e.l
pub fn i_07f84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F854;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F854);
    cpu.pc = 0x236E;
    16
}
/// $07F854: addi.w #$10, $38(a6)
pub fn i_07f854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F85A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F85A: clr.b $20(a6)
pub fn i_07f85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F85E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07F85E: clr.b $21(a6)
pub fn i_07f85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F862;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07F862: jsr $267e2.l
pub fn i_07f862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F868;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F868);
    cpu.pc = 0x267E2;
    16
}
/// $07F868: lea.l $2e21ac.l, a0
pub fn i_07f868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F86E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E21AC;
    20
}
/// $07F86E: jsr $28cd4.l
pub fn i_07f86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F874);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_07f874  ($07F874, 8 B, kernel/task_handlers.c)
/// $07F874: lea.l $7f87c(pc), a1
pub fn i_07f874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F878;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F87C;
    16
}
/// $07F878: move.l a1, (a6)
pub fn i_07f878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F87A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F87A: rts
pub fn i_07f87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F87C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_PoseRun_07f87c  ($07F87C, 88 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F87C: movea.l $c(a6), a0
pub fn i_07f87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F880;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F880: movea.l $c(a0), a0
pub fn i_07f880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F884;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F884: move.w $78(a0), $22(a6)
pub fn i_07f884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F88A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07F88A: move.w $7a(a0), $24(a6)
pub fn i_07f88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F890;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07F890: subi.w #$9, $22(a6)
pub fn i_07f890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F896;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x9, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F896: movea.l $c(a6), a0
pub fn i_07f896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F89A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F89A: cmpi.b #$1, $78(a0)
pub fn i_07f89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8A0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07F8A0: beq.w $7f8c6
pub fn i_07f8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7F8C6; return 10; }
    8
}
/// $07F8A4: cmpi.b #$ff, $20(a0)
pub fn i_07f8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8AA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07F8AA: beq.w $7fdbc
pub fn i_07f8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7FDBC; return 10; }
    8
}
/// $07F8AE: move.w $70(a0), d0
pub fn i_07f8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8B2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07F8B2: add.w d0, d0
pub fn i_07f8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8B4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07F8B4: add.w d0, d0
pub fn i_07f8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8B6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07F8B6: lea.l $2e2194.l, a0
pub fn i_07f8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2194;
    20
}
/// $07F8BC: movea.l (a0, d0.w), a0
pub fn i_07f8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8C0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07F8C0: jsr $28cd4.l
pub fn i_07f8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F8C6);
    cpu.pc = 0x28CD4;
    16
}
/// $07F8C6: jsr $28d70.l
pub fn i_07f8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F8CC);
    cpu.pc = 0x28D70;
    16
}
/// $07F8CC: jsr $7fde0(pc)
pub fn i_07f8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F8D0);
    cpu.pc = 0x7FDE0;
    16
}
/// $07F8D0: bcc.w $7f8da
pub fn i_07f8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F8DA; return 10; }
    8
}
// ===== SetTaskHandler_07f8d4  ($07F8D4, 8 B, kernel/task_handlers.c)
/// $07F8D4: lea.l $7fdbc(pc), a1
pub fn i_07f8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F8D8: move.l a1, (a6)
pub fn i_07f8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F8DA: rts
pub fn i_07f8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Fire_07f8dc  ($07F8DC, 164 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F8DC: movea.l $c(a6), a0
pub fn i_07f8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8E0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F8E0: bset.b #$1, $21(a0)
pub fn i_07f8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8E6;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07F8E6: move.b #$ff, $21(a6)
pub fn i_07f8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8EC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07F8EC: move.w #$ffff, $76(a6)
pub fn i_07f8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F8F2: move.w #$1, $78(a6)
pub fn i_07f8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F8F8: addi.w #$1c, $38(a6)
pub fn i_07f8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F8FE;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F8FE: bra.w $7f918
pub fn i_07f8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F902;
    cpu.ir = 0x6000;
    cpu.pc = 0x7F918;
    10
}
/// $07F918: move.w #$e, d1
pub fn i_07f918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F91C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07F91C: jsr $236e.l
pub fn i_07f91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F922;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F922);
    cpu.pc = 0x236E;
    16
}
/// $07F922: jsr $267e2.l
pub fn i_07f922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F928;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F928);
    cpu.pc = 0x267E2;
    16
}
/// $07F928: lea.l $2e2286.l, a0
pub fn i_07f928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F92E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2286;
    20
}
/// $07F92E: jsr $28cd4.l
pub fn i_07f92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F934;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F934);
    cpu.pc = 0x28CD4;
    16
}
/// $07F934: lea.l $7f93a(pc), a1
pub fn i_07f934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F938;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F93A;
    16
}
/// $07F938: move.l a1, (a6)
pub fn i_07f938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F93A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F93A: movea.l $c(a6), a0
pub fn i_07f93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F93E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F93E: movea.l $c(a0), a0
pub fn i_07f93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F942;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F942: move.w $78(a0), $22(a6)
pub fn i_07f942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F948;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07F948: move.w $7a(a0), $24(a6)
pub fn i_07f948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F94E;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07F94E: subi.w #$9, $22(a6)
pub fn i_07f94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F954;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x9, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07F954: jsr $28d70.l
pub fn i_07f954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F95A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F95A);
    cpu.pc = 0x28D70;
    16
}
/// $07F95A: bcc.w $7f974
pub fn i_07f95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F95E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F974; return 10; }
    8
}
/// $07F95E: jsr $5e9b6.l
pub fn i_07f95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F964);
    cpu.pc = 0x5E9B6;
    16
}
/// $07F964: andi.w #$1, d0
pub fn i_07f964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F968;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07F968: add.w d0, d0
pub fn i_07f968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F96A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07F96A: move.w d0, $72(a6)
pub fn i_07f96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F96E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F96E: lea.l $7f988(pc), a1
pub fn i_07f96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F972;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F988;
    16
}
/// $07F972: move.l a1, (a6)
pub fn i_07f972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F974;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F974: jsr $80054(pc)
pub fn i_07f974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F978;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F978);
    cpu.pc = 0x80054;
    16
}
/// $07F978: jsr $7fde0(pc)
pub fn i_07f978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F97C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F97C);
    cpu.pc = 0x7FDE0;
    16
}
/// $07F97C: bcc.w $7f986
pub fn i_07f97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F980;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F986; return 10; }
    8
}
// ===== SetTaskHandler_07f980  ($07F980, 8 B, kernel/task_handlers.c)
/// $07F980: lea.l $7fdbc(pc), a1
pub fn i_07f980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F984;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F984: move.l a1, (a6)
pub fn i_07f984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F986;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F986: rts
pub fn i_07f986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F988;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Land_07f988  ($07F988, 64 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F988: tst.w $76(a6)
pub fn i_07f988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F98C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07F98C: bne.w $7f990
pub fn i_07f98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F990;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F990; return 10; }
    8
}
/// $07F990: move.w #$0, $74(a6)
pub fn i_07f990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F996;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F996: lea.l $2e22e8.l, a0
pub fn i_07f996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F99C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E22E8;
    20
}
/// $07F99C: jsr $28cd4.l
pub fn i_07f99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F9A2);
    cpu.pc = 0x28CD4;
    16
}
/// $07F9A2: lea.l $7f9a8(pc), a1
pub fn i_07f9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F9A8;
    16
}
/// $07F9A6: move.l a1, (a6)
pub fn i_07f9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F9A8: jsr $7fe66(pc)
pub fn i_07f9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F9AC);
    cpu.pc = 0x7FE66;
    16
}
/// $07F9AC: jsr $28d70.l
pub fn i_07f9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F9B2);
    cpu.pc = 0x28D70;
    16
}
/// $07F9B2: bcc.w $7f9bc
pub fn i_07f9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F9BC; return 10; }
    8
}
/// $07F9B6: lea.l $7f9d0(pc), a1
pub fn i_07f9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F9D0;
    16
}
/// $07F9BA: move.l a1, (a6)
pub fn i_07f9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F9BC: jsr $80054(pc)
pub fn i_07f9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F9C0);
    cpu.pc = 0x80054;
    16
}
/// $07F9C0: jsr $7fde0(pc)
pub fn i_07f9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F9C4);
    cpu.pc = 0x7FDE0;
    16
}
/// $07F9C4: bcc.w $7f9ce
pub fn i_07f9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F9CE; return 10; }
    8
}
// ===== SetTaskHandler_07f9c8  ($07F9C8, 8 B, kernel/task_handlers.c)
/// $07F9C8: lea.l $7fdbc(pc), a1
pub fn i_07f9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F9CC: move.l a1, (a6)
pub fn i_07f9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F9CE: rts
pub fn i_07f9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Stand_07f9d0  ($07F9D0, 72 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F9D0: tst.w $76(a6)
pub fn i_07f9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9D4;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07F9D4: bne.w $7f9e6
pub fn i_07f9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F9E6; return 10; }
    8
}
/// $07F9D8: jsr $5e9b6.l
pub fn i_07f9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F9DE);
    cpu.pc = 0x5E9B6;
    16
}
/// $07F9DE: btst.b #$0, d0
pub fn i_07f9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9E2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07F9E2: bne.w $7fa26
pub fn i_07f9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FA26; return 10; }
    8
}
/// $07F9E6: lea.l $29bfb8.l, a0
pub fn i_07f9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BFB8;
    20
}
/// $07F9EC: jsr $28cd4.l
pub fn i_07f9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F9F2);
    cpu.pc = 0x28CD4;
    16
}
/// $07F9F2: lea.l $7f9f8(pc), a1
pub fn i_07f9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F9F8;
    16
}
/// $07F9F6: move.l a1, (a6)
pub fn i_07f9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F9F8: jsr $7fe66(pc)
pub fn i_07f9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F9FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F9FC);
    cpu.pc = 0x7FE66;
    16
}
/// $07F9FC: jsr $28d70.l
pub fn i_07f9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FA02);
    cpu.pc = 0x28D70;
    16
}
/// $07FA02: bcc.w $7fa0c
pub fn i_07fa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FA0C; return 10; }
    8
}
/// $07FA06: lea.l $7fa20(pc), a1
pub fn i_07fa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FA20;
    16
}
/// $07FA0A: move.l a1, (a6)
pub fn i_07fa0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FA0C: jsr $80054(pc)
pub fn i_07fa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FA10);
    cpu.pc = 0x80054;
    16
}
/// $07FA10: jsr $7fde0(pc)
pub fn i_07fa10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FA14);
    cpu.pc = 0x7FDE0;
    16
}
/// $07FA14: bcc.w $7fa1e
pub fn i_07fa14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA18;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FA1E; return 10; }
    8
}
// ===== SetTaskHandler_07fa18  ($07FA18, 8 B, kernel/task_handlers.c)
/// $07FA18: lea.l $7fdbc(pc), a1
pub fn i_07fa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FA1C: move.l a1, (a6)
pub fn i_07fa1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FA1E: rts
pub fn i_07fa1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Walk_07fa20  ($07FA20, 106 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07FA20: move.b #$1, $3a(a6)
pub fn i_07fa20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA26;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07FA26: jsr $267e2.l
pub fn i_07fa26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FA2C);
    cpu.pc = 0x267E2;
    16
}
/// $07FA2C: move.w #$fe00, $28(a6)
pub fn i_07fa2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA32;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FA32: btst.b #$0, $3a(a6)
pub fn i_07fa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA38;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07FA38: beq.w $7fa40
pub fn i_07fa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA3C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7FA40; return 10; }
    8
}
/// $07FA3C: neg.w $28(a6)
pub fn i_07fa3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA40;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FA40: clr.w $74(a6)
pub fn i_07fa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA44;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FA44: lea.l $29b744.l, a0
pub fn i_07fa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA4A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B744;
    20
}
/// $07FA4A: jsr $28cd4.l
pub fn i_07fa4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FA50);
    cpu.pc = 0x28CD4;
    16
}
/// $07FA50: lea.l $7fa56(pc), a1
pub fn i_07fa50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FA56;
    16
}
/// $07FA54: move.l a1, (a6)
pub fn i_07fa54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FA56: jsr $7fe66(pc)
pub fn i_07fa56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FA5A);
    cpu.pc = 0x7FE66;
    16
}
/// $07FA5A: jsr $28d70.l
pub fn i_07fa5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FA60);
    cpu.pc = 0x28D70;
    16
}
/// $07FA60: move.w $74(a6), d0
pub fn i_07fa60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FA64: bpl.w $7fa6a
pub fn i_07fa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA68;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7FA6A; return 10; }
    8
}
/// $07FA68: neg.w d0
pub fn i_07fa68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA6A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07FA6A: cmpi.w #$2000, d0
pub fn i_07fa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA6E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2000, Size::Word, false);
    8
}
/// $07FA6E: blt.w $7fa7e
pub fn i_07fa6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA72;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7FA7E; return 10; }
    8
}
/// $07FA72: jsr $267e2.l
pub fn i_07fa72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FA78);
    cpu.pc = 0x267E2;
    16
}
/// $07FA78: lea.l $7fa92(pc), a1
pub fn i_07fa78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FA92;
    16
}
/// $07FA7C: move.l a1, (a6)
pub fn i_07fa7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FA7E: jsr $80054(pc)
pub fn i_07fa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FA82);
    cpu.pc = 0x80054;
    16
}
/// $07FA82: jsr $7fde0(pc)
pub fn i_07fa82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA86;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FA86);
    cpu.pc = 0x7FDE0;
    16
}
/// $07FA86: bcc.w $7fa90
pub fn i_07fa86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FA90; return 10; }
    8
}
// ===== SetTaskHandler_07fa8a  ($07FA8A, 8 B, kernel/task_handlers.c)
/// $07FA8A: lea.l $7fdbc(pc), a1
pub fn i_07fa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FA8E: move.l a1, (a6)
pub fn i_07fa8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FA90: rts
pub fn i_07fa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Stop_07fa92  ($07FA92, 50 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07FA92: lea.l $29b7c8.l, a0
pub fn i_07fa92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA98;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B7C8;
    20
}
/// $07FA98: jsr $28cd4.l
pub fn i_07fa98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FA9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FA9E);
    cpu.pc = 0x28CD4;
    16
}
/// $07FA9E: lea.l $7faa4(pc), a1
pub fn i_07fa9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FAA4;
    16
}
/// $07FAA2: move.l a1, (a6)
pub fn i_07faa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FAA4: jsr $7fe66(pc)
pub fn i_07faa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAA8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FAA8);
    cpu.pc = 0x7FE66;
    16
}
/// $07FAA8: jsr $28d70.l
pub fn i_07faa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FAAE);
    cpu.pc = 0x28D70;
    16
}
/// $07FAAE: bcc.w $7fab8
pub fn i_07faae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FAB8; return 10; }
    8
}
/// $07FAB2: lea.l $7facc(pc), a1
pub fn i_07fab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FACC;
    16
}
/// $07FAB6: move.l a1, (a6)
pub fn i_07fab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FAB8: jsr $80054(pc)
pub fn i_07fab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FABC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FABC);
    cpu.pc = 0x80054;
    16
}
/// $07FABC: jsr $7fde0(pc)
pub fn i_07fabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FAC0);
    cpu.pc = 0x7FDE0;
    16
}
/// $07FAC0: bcc.w $7faca
pub fn i_07fac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FACA; return 10; }
    8
}
// ===== SetTaskHandler_07fac4  ($07FAC4, 8 B, kernel/task_handlers.c)
/// $07FAC4: lea.l $7fdbc(pc), a1
pub fn i_07fac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FAC8: move.l a1, (a6)
pub fn i_07fac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FACA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FACA: rts
pub fn i_07faca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FACC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Ride_07facc  ($07FACC, 84 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07FACC: tst.w $76(a6)
pub fn i_07facc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAD0;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07FAD0: bne.w $7fb28
pub fn i_07fad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAD4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FB28; return 10; }
    8
}
/// $07FAD4: move.w #$28, d1
pub fn i_07fad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAD8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07FAD8: jsr $236e.l
pub fn i_07fad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FADE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FADE);
    cpu.pc = 0x236E;
    16
}
/// $07FADE: lea.l $29c77c.l, a0
pub fn i_07fade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C77C;
    20
}
/// $07FAE4: jsr $28cd4.l
pub fn i_07fae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FAEA);
    cpu.pc = 0x28CD4;
    16
}
/// $07FAEA: lea.l $7faf0(pc), a1
pub fn i_07faea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FAF0;
    16
}
/// $07FAEE: move.l a1, (a6)
pub fn i_07faee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FAF0: jsr $7fe66(pc)
pub fn i_07faf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FAF4);
    cpu.pc = 0x7FE66;
    16
}
/// $07FAF4: jsr $28d70.l
pub fn i_07faf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FAFA);
    cpu.pc = 0x28D70;
    16
}
/// $07FAFA: bcc.w $7fb14
pub fn i_07fafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FB14; return 10; }
    8
}
/// $07FAFE: movea.l $c(a6), a0
pub fn i_07fafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB02;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FB02: move.w $24(a0), d0
pub fn i_07fb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB06;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FB06: subi.w #$c, d0
pub fn i_07fb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB0A;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07FB0A: move.w d0, $70(a6)
pub fn i_07fb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB0E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FB0E: lea.l $7f33e(pc), a1
pub fn i_07fb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F33E;
    16
}
/// $07FB12: move.l a1, (a6)
pub fn i_07fb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FB14: jsr $80054(pc)
pub fn i_07fb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FB18);
    cpu.pc = 0x80054;
    16
}
/// $07FB18: jsr $7fde0(pc)
pub fn i_07fb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FB1C);
    cpu.pc = 0x7FDE0;
    16
}
/// $07FB1C: bcc.w $7fb26
pub fn i_07fb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FB26; return 10; }
    8
}
// ===== SetTaskHandler_07fb20  ($07FB20, 8 B, kernel/task_handlers.c)
/// $07FB20: lea.l $7fdbc(pc), a1
pub fn i_07fb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FB24: move.l a1, (a6)
pub fn i_07fb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FB26: rts
pub fn i_07fb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Idle_07fb28  ($07FB28, 62 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07FB28: jsr $7ed6a(pc)
pub fn i_07fb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FB2C);
    cpu.pc = 0x7ED6A;
    16
}
/// $07FB2C: lea.l $29b816.l, a0
pub fn i_07fb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B816;
    20
}
/// $07FB32: jsr $28cd4.l
pub fn i_07fb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FB38);
    cpu.pc = 0x28CD4;
    16
}
/// $07FB38: lea.l $7fb3e(pc), a1
pub fn i_07fb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FB3E;
    16
}
/// $07FB3C: move.l a1, (a6)
pub fn i_07fb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FB3E: jsr $7fe66(pc)
pub fn i_07fb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FB42);
    cpu.pc = 0x7FE66;
    16
}
/// $07FB42: jsr $28d70.l
pub fn i_07fb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FB48);
    cpu.pc = 0x28D70;
    16
}
/// $07FB48: tst.w $90(a6)
pub fn i_07fb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB4C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07FB4C: bne.w $7fb56
pub fn i_07fb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB50;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FB56; return 10; }
    8
}
/// $07FB50: lea.l $7fb6e(pc), a1
pub fn i_07fb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FB6E;
    16
}
/// $07FB54: move.l a1, (a6)
pub fn i_07fb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FB56: subq.w #$1, $90(a6)
pub fn i_07fb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB5A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FB5A: jsr $80054(pc)
pub fn i_07fb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FB5E);
    cpu.pc = 0x80054;
    16
}
/// $07FB5E: jsr $7fde0(pc)
pub fn i_07fb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FB62);
    cpu.pc = 0x7FDE0;
    16
}
/// $07FB62: bcc.w $7fb6c
pub fn i_07fb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FB6C; return 10; }
    8
}
// ===== SetTaskHandler_07fb66  ($07FB66, 8 B, kernel/task_handlers.c)
/// $07FB66: lea.l $7fdbc(pc), a1
pub fn i_07fb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FB6A: move.l a1, (a6)
pub fn i_07fb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FB6C: rts
pub fn i_07fb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Decide_07fb6e  ($07FB6E, 92 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07FB6E: jsr $5e0d4.l
pub fn i_07fb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FB74);
    cpu.pc = 0x5E0D4;
    16
}
/// $07FB74: move.w $22(a0), d0
pub fn i_07fb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB78;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FB78: cmp.w $22(a6), d0
pub fn i_07fb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB7C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07FB7C: ble.w $7fb8e
pub fn i_07fb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB80;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7FB8E; return 10; }
    8
}
/// $07FB80: btst.b #$0, $3a(a6)
pub fn i_07fb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB86;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07FB86: bne.w $7fbd2
pub fn i_07fb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB8A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FBD2; return 10; }
    8
}
/// $07FB8A: bra.w $7fb98
pub fn i_07fb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x7FB98;
    10
}
/// $07FB8E: btst.b #$0, $3a(a6)
pub fn i_07fb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB94;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07FB94: beq.w $7fbd2
pub fn i_07fb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7FBD2; return 10; }
    8
}
/// $07FB98: lea.l $29bf34.l, a0
pub fn i_07fb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BF34;
    20
}
/// $07FB9E: jsr $28cd4.l
pub fn i_07fb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FBA4);
    cpu.pc = 0x28CD4;
    16
}
/// $07FBA4: lea.l $7fbaa(pc), a1
pub fn i_07fba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FBAA;
    16
}
/// $07FBA8: move.l a1, (a6)
pub fn i_07fba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FBAA: jsr $7fe66(pc)
pub fn i_07fbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FBAE);
    cpu.pc = 0x7FE66;
    16
}
/// $07FBAE: jsr $28d70.l
pub fn i_07fbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FBB4);
    cpu.pc = 0x28D70;
    16
}
/// $07FBB4: bcc.w $7fbbe
pub fn i_07fbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FBBE; return 10; }
    8
}
/// $07FBB8: lea.l $7fbd2(pc), a1
pub fn i_07fbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FBD2;
    16
}
/// $07FBBC: move.l a1, (a6)
pub fn i_07fbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FBBE: jsr $80054(pc)
pub fn i_07fbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FBC2);
    cpu.pc = 0x80054;
    16
}
/// $07FBC2: jsr $7fde0(pc)
pub fn i_07fbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FBC6);
    cpu.pc = 0x7FDE0;
    16
}
/// $07FBC6: bcc.w $7fbd0
pub fn i_07fbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBCA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FBD0; return 10; }
    8
}
// ===== SetTaskHandler_07fbca  ($07FBCA, 8 B, kernel/task_handlers.c)
/// $07FBCA: lea.l $7fdbc(pc), a1
pub fn i_07fbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FBCE: move.l a1, (a6)
pub fn i_07fbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FBD0: rts
pub fn i_07fbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_EscortInit_07fbd2  ($07FBD2, 64 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FBD2: jsr $5e9b6.l
pub fn i_07fbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FBD8);
    cpu.pc = 0x5E9B6;
    16
}
/// $07FBD8: andi.w #$3, d0
pub fn i_07fbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBDC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07FBDC: move.b d0, $5c(a6)
pub fn i_07fbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBE0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07FBE0: lea.l $29ba70.l, a0
pub fn i_07fbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBE6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BA70;
    20
}
/// $07FBE6: jsr $28cd4.l
pub fn i_07fbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FBEC);
    cpu.pc = 0x28CD4;
    16
}
/// $07FBEC: lea.l $7fbf2(pc), a1
pub fn i_07fbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FBF2;
    16
}
/// $07FBF0: move.l a1, (a6)
pub fn i_07fbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FBF2: jsr $7fe66(pc)
pub fn i_07fbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FBF6);
    cpu.pc = 0x7FE66;
    16
}
/// $07FBF6: jsr $28d70.l
pub fn i_07fbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FBFC);
    cpu.pc = 0x28D70;
    16
}
/// $07FBFC: bcc.w $7fc06
pub fn i_07fbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FC06; return 10; }
    8
}
/// $07FC00: lea.l $7fb28(pc), a1
pub fn i_07fc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FB28;
    16
}
/// $07FC04: move.l a1, (a6)
pub fn i_07fc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FC06: jsr $80054(pc)
pub fn i_07fc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC0A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FC0A);
    cpu.pc = 0x80054;
    16
}
/// $07FC0A: jsr $7fde0(pc)
pub fn i_07fc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FC0E);
    cpu.pc = 0x7FDE0;
    16
}
/// $07FC0E: bcc.w $7fc18
pub fn i_07fc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FC18; return 10; }
    8
}
// ===== SetTaskHandler_07fc12  ($07FC12, 8 B, kernel/task_handlers.c)
/// $07FC12: lea.l $7fdbc(pc), a1
pub fn i_07fc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FC16: move.l a1, (a6)
pub fn i_07fc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FC18: rts
pub fn i_07fc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_HatchRow3Spawn_07fc1a  ($07FC1A, 294 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FC1A: lea.l $77fd6.l, a1
pub fn i_07fc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC20;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $07FC20: jsr $4ae.l
pub fn i_07fc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FC26);
    cpu.pc = 0x4AE;
    16
}
/// $07FC26: jsr $5dd02.l
pub fn i_07fc26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FC2C);
    cpu.pc = 0x5DD02;
    16
}
/// $07FC2C: addi.w #$28, $38(a0)
pub fn i_07fc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC32;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FC32: subi.w #$30, $22(a0)
pub fn i_07fc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC38;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FC38: move.w #$ffb8, $70(a6)
pub fn i_07fc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC3E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFB8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FC3E: lea.l $2e1dca.l, a0
pub fn i_07fc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E1DCA;
    20
}
/// $07FC44: jsr $28cd4.l
pub fn i_07fc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FC4A);
    cpu.pc = 0x28CD4;
    16
}
/// $07FC4A: lea.l $7fcbe(pc), a1
pub fn i_07fc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FCBE;
    16
}
/// $07FC4E: move.l a1, (a6)
pub fn i_07fc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC50;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FC50: bra.w $7fcbe
pub fn i_07fc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC54;
    cpu.ir = 0x6000;
    cpu.pc = 0x7FCBE;
    10
}
/// $07FCBE: movea.l $c(a6), a0
pub fn i_07fcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCC2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FCC2: move.w $22(a0), $22(a6)
pub fn i_07fcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCC8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FCC8: move.w $24(a0), $24(a6)
pub fn i_07fcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCCE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FCCE: jsr $28d70.l
pub fn i_07fcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FCD4);
    cpu.pc = 0x28D70;
    16
}
/// $07FCD4: bclr.b #$3, $13(a6)
pub fn i_07fcd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCDA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07FCDA: movea.l $c(a6), a0
pub fn i_07fcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCDE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FCDE: cmpi.b #$3, $20(a0)
pub fn i_07fcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCE4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $07FCE4: bne.w $7fd38
pub fn i_07fce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FD38; return 10; }
    8
}
/// $07FCE8: tst.w $70(a6)
pub fn i_07fce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCEC;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07FCEC: bne.w $7fd2a
pub fn i_07fcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCF0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FD2A; return 10; }
    8
}
/// $07FCF0: lea.l $2e2ae8.l, a1
pub fn i_07fcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCF6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E2AE8;
    20
}
/// $07FCF6: jsr $77c7e.l
pub fn i_07fcf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FCFC);
    cpu.pc = 0x77C7E;
    16
}
/// $07FCFC: addi.w #$28, $38(a0)
pub fn i_07fcfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD02;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FD02: lea.l $2e2afa.l, a1
pub fn i_07fd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD08;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E2AFA;
    20
}
/// $07FD08: jsr $77c7e.l
pub fn i_07fd08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FD0E);
    cpu.pc = 0x77C7E;
    16
}
/// $07FD0E: addi.w #$28, $38(a0)
pub fn i_07fd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD14;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FD14: lea.l $7f22a(pc), a1
pub fn i_07fd14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F22A;
    16
}
/// $07FD18: jsr $4ae.l
pub fn i_07fd18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FD1E);
    cpu.pc = 0x4AE;
    16
}
/// $07FD1E: jsr $5dd02.l
pub fn i_07fd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FD24);
    cpu.pc = 0x5DD02;
    16
}
/// $07FD24: addi.w #$c000, $38(a0)
pub fn i_07fd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD2A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC000, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FD2A: lea.l $ffff.w, a0
pub fn i_07fd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD2E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $07FD2E: move.l a0, $48(a6)
pub fn i_07fd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD32;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07FD32: lea.l $7fd48(pc), a1
pub fn i_07fd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FD48;
    16
}
/// $07FD36: move.l a1, (a6)
pub fn i_07fd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FD38: jsr $7fdc4(pc)
pub fn i_07fd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FD3C);
    cpu.pc = 0x7FDC4;
    16
}
/// $07FD3C: bcc.w $7fd46
pub fn i_07fd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FD46; return 10; }
    8
}
// ===== SetTaskHandler_07fd40  ($07FD40, 8 B, kernel/task_handlers.c)
/// $07FD40: lea.l $7fdbc(pc), a1
pub fn i_07fd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FD44: move.l a1, (a6)
pub fn i_07fd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FD46: rts
pub fn i_07fd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_HatchDeployed_07fd48  ($07FD48, 88 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FD48: movea.l $c(a6), a0
pub fn i_07fd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD4C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FD4C: move.w $22(a0), $22(a6)
pub fn i_07fd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD52;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FD52: move.w $24(a0), $24(a6)
pub fn i_07fd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD58;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FD58: jsr $28d70.l
pub fn i_07fd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FD5E);
    cpu.pc = 0x28D70;
    16
}
/// $07FD5E: movea.l $c(a6), a0
pub fn i_07fd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD62;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FD62: cmpi.b #$1, $21(a0)
pub fn i_07fd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD68;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07FD68: bne.w $7fd98
pub fn i_07fd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FD98; return 10; }
    8
}
/// $07FD6C: move.w $70(a6), d0
pub fn i_07fd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD70;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FD70: add.w d0, $22(a6)
pub fn i_07fd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD74;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FD74: addi.w #$28, $38(a6)
pub fn i_07fd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD7A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FD7A: lea.l $ffff.w, a0
pub fn i_07fd7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD7E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $07FD7E: move.l a0, $48(a6)
pub fn i_07fd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD82;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07FD82: lea.l $77fd6.l, a1
pub fn i_07fd82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD88;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $07FD88: move.l a1, (a6)
pub fn i_07fd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FD8A: tst.w $70(a6)
pub fn i_07fd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD8E;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07FD8E: bne.w $7fd98
pub fn i_07fd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FD98; return 10; }
    8
}
/// $07FD92: jsr $434dc.l
pub fn i_07fd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FD98);
    cpu.pc = 0x434DC;
    16
}
/// $07FD98: jsr $7fdc4(pc)
pub fn i_07fd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FD9C);
    cpu.pc = 0x7FDC4;
    16
}
/// $07FD9C: bcc.w $7fda6
pub fn i_07fd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FDA6; return 10; }
    8
}
// ===== SetTaskHandler_07fda0  ($07FDA0, 8 B, kernel/task_handlers.c)
/// $07FDA0: lea.l $7fdbc(pc), a1
pub fn i_07fda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FDA4: move.l a1, (a6)
pub fn i_07fda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FDA6: rts
pub fn i_07fda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_LeaderNotify_07fda8  ($07FDA8, 44 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FDA8: movea.l $c(a6), a0
pub fn i_07fda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDAC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FDAC: move.b #$5, $20(a0)
pub fn i_07fdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDB2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07FDB2: bra.w $7fdbc
pub fn i_07fdb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x7FDBC;
    10
}
/// $07FDBC: jmp $518.l
pub fn i_07fdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDC2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07FDC4: lea.l $2e2ad8.l, a0
pub fn i_07fdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDCA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2AD8;
    20
}
/// $07FDCA: jsr $5dd5c.l
pub fn i_07fdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FDD0);
    cpu.pc = 0x5DD5C;
    16
}
/// $07FDD0: bcc.w $7fdda
pub fn i_07fdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDD4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FDDA; return 10; }
    8
}
// ===== SetC_07fdd4  ($07FDD4, 6 B, thunks/ccr_helpers.c)
/// $07FDD4: ori.b #$1, ccr
pub fn i_07fdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDD8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $07FDD8: rts
pub fn i_07fdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_07fdda  ($07FDDA, 6 B, thunks/ccr_helpers.c)
/// $07FDDA: andi.b #$fe, ccr
pub fn i_07fdda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDDE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $07FDDE: rts
pub fn i_07fdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_LeaderDeadGate_07fde0  ($07FDE0, 14 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FDE0: movea.l $c(a6), a0
pub fn i_07fde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDE4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FDE4: cmpi.b #$ff, $21(a0)
pub fn i_07fde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDEA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07FDEA: bne.w $7fdf4
pub fn i_07fdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDEE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FDF4; return 10; }
    8
}
// ===== SetC_07fdee  ($07FDEE, 6 B, thunks/ccr_helpers.c)
/// $07FDEE: ori.b #$1, ccr
pub fn i_07fdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDF2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $07FDF2: rts
pub fn i_07fdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_07fdf4  ($07FDF4, 6 B, thunks/ccr_helpers.c)
/// $07FDF4: andi.b #$fe, ccr
pub fn i_07fdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDF8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $07FDF8: rts
pub fn i_07fdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_TrackArc_07fdfa  ($07FDFA, 232 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FDFA: movea.l $c(a6), a0
pub fn i_07fdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDFE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FDFE: move.w $22(a0), $22(a6)
pub fn i_07fdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE04;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FE04: move.w $24(a0), $24(a6)
pub fn i_07fe04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE0A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FE0A: move.w $74(a6), d0
pub fn i_07fe0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE0E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FE0E: asr.w #$8, d0
pub fn i_07fe0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE10;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07FE10: add.w $22(a6), d0
pub fn i_07fe10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE14;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07FE14: move.w d0, $22(a6)
pub fn i_07fe14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE18;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FE18: addi.w #$8, $24(a6)
pub fn i_07fe18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE1E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FE1E: lea.l $2e22fa.l, a1
pub fn i_07fe1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE24;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E22FA;
    20
}
/// $07FE24: move.w $74(a6), d1
pub fn i_07fe24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE28;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FE28: asr.w #$8, d1
pub fn i_07fe28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE2A;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07FE2A: addi.w #$50, d1
pub fn i_07fe2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE2E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x50, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07FE2E: tst.w d1
pub fn i_07fe2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE30;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $07FE30: bpl.w $7fe3c
pub fn i_07fe30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE34;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7FE3C; return 10; }
    8
}
/// $07FE34: move.w #$b800, $74(a6)
pub fn i_07fe34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE3A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FE3A: clr.w d1
pub fn i_07fe3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE3C;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07FE3C: cmpi.w #$90, d1
pub fn i_07fe3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE40;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x90, Size::Word, false);
    8
}
/// $07FE40: ble.w $7fe4e
pub fn i_07fe40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE44;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7FE4E; return 10; }
    8
}
/// $07FE44: move.w #$4800, $74(a6)
pub fn i_07fe44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE4A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FE4A: move.w #$90, d1
pub fn i_07fe4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE4E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07FE4E: clr.w d0
pub fn i_07fe4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE50;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07FE50: lsr.w #$2, d1
pub fn i_07fe50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE52;
    cpu.ir = 0xE449;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07FE52: move.b (a1, d1.w), d0
pub fn i_07fe52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE56;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $07FE56: cmpi.w #$1, $72(a6)
pub fn i_07fe56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE5C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $07FE5C: beq.w $7fe64
pub fn i_07fe5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7FE64; return 10; }
    8
}
/// $07FE60: add.w d0, $24(a6)
pub fn i_07fe60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE64;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FE64: rts
pub fn i_07fe64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07FE66: movea.l $c(a6), a0
pub fn i_07fe66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE6A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FE6A: movea.l $c(a0), a0
pub fn i_07fe6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE6E;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FE6E: move.w $78(a0), $22(a6)
pub fn i_07fe6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE74;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FE74: move.w $7a(a0), $24(a6)
pub fn i_07fe74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE7A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FE7A: subq.w #$8, $22(a6)
pub fn i_07fe7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE7E;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FE7E: move.w $74(a6), d0
pub fn i_07fe7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE82;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FE82: add.w $28(a6), d0
pub fn i_07fe82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE86;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07FE86: move.w d0, $74(a6)
pub fn i_07fe86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE8A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FE8A: asr.w #$8, d0
pub fn i_07fe8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE8C;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07FE8C: add.w d0, $22(a6)
pub fn i_07fe8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE90;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FE90: tst.w $76(a6)
pub fn i_07fe90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE94;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07FE94: beq.w $7feb2
pub fn i_07fe94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7FEB2; return 10; }
    8
}
/// $07FE98: move.w $74(a6), d0
pub fn i_07fe98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE9C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FE9C: lsr.w #$8, d0
pub fn i_07fe9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE9E;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07FE9E: lsr.w #$1, d0
pub fn i_07fe9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEA0;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07FEA0: lea.l $2e2320.l, a0
pub fn i_07fea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2320;
    20
}
/// $07FEA6: move.b (a0, d0.w), d0
pub fn i_07fea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEAA;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $07FEAA: andi.w #$ff, d0
pub fn i_07feaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEAE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07FEAE: add.w d0, $24(a6)
pub fn i_07feae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEB2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FEB2: rts
pub fn i_07feb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07FEB4: addi.w #$2, $74(a6)
pub fn i_07feb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEBA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FEBA: andi.w #$7f, $74(a6)
pub fn i_07feba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEC0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FEC0: move.w $76(a6), d0
pub fn i_07fec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEC4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FEC4: sub.w d0, $24(a6)
pub fn i_07fec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEC8;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FEC8: move.w $74(a6), d0
pub fn i_07fec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FECC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FECC: add.w d0, d0
pub fn i_07fecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FECE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07FECE: lea.l $2c072c.l, a0
pub fn i_07fece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FED4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C072C;
    20
}
/// $07FED4: move.w (a0, d0.w), d0
pub fn i_07fed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FED8;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07FED8: muls.w #$4, d0
pub fn i_07fed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEDC;
    cpu.ir = 0xC1FC;
    let v1: u32 = 0x4;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $07FEDC: asr.l #$8, d0
pub fn i_07fedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEDE;
    cpu.ir = 0xE080;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[0] = r;
    22
}
/// $07FEDE: add.w d0, $24(a6)
pub fn i_07fede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEE2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetTaskW_07fee2  ($07FEE2, 6 B, kernel/task_setters_w.c)
/// $07FEE2: move.w d0, $76(a6)
pub fn i_07fee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEE6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FEE6: rts
pub fn i_07fee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_AIDecide_07fee8  ($07FEE8, 326 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FEE8: cmpi.w #$ffff, $80(a6)
pub fn i_07fee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEEE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $07FEEE: beq.w $7ff28
pub fn i_07feee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEF2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7FF28; return 10; }
    8
}
/// $07FEF2: move.w $80(a6), $66(a6)
pub fn i_07fef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEF8;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FEF8: tst.b $9d(a6)
pub fn i_07fef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEFC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07FEFC: beq.w $7ff0e
pub fn i_07fefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7FF0E; return 10; }
    8
}
/// $07FF00: cmpi.w #$50, $22(a6)
pub fn i_07ff00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF06;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x50, Size::Word, false);
    16
}
/// $07FF06: blt.w $7ff96
pub fn i_07ff06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF0A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7FF96; return 10; }
    8
}
/// $07FF0A: bra.w $7ff18
pub fn i_07ff0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF0E;
    cpu.ir = 0x6000;
    cpu.pc = 0x7FF18;
    10
}
/// $07FF0E: cmpi.w #$f0, $22(a6)
pub fn i_07ff0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF14;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF0, Size::Word, false);
    16
}
/// $07FF14: bgt.w $7ff5e
pub fn i_07ff14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF18;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7FF5E; return 10; }
    8
}
/// $07FF18: move.w #$ffff, $80(a6)
pub fn i_07ff18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FF1E: lea.l $2e251c.l, a0
pub fn i_07ff1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E251C;
    20
}
/// $07FF24: move.l a0, $48(a6)
pub fn i_07ff24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF28;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07FF28: jsr $5e0d4.l
pub fn i_07ff28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FF2E);
    cpu.pc = 0x5E0D4;
    16
}
/// $07FF2E: bcs.w $7ffbc
pub fn i_07ff2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF32;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7FFBC; return 10; }
    8
}
/// $07FF32: clr.b d2
pub fn i_07ff32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF34;
    cpu.ir = 0x4202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $07FF34: move.w $22(a6), d0
pub fn i_07ff34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF38;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FF38: sub.w $22(a0), d0
pub fn i_07ff38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF3C;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07FF3C: cmpi.w #$60, d0
pub fn i_07ff3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF40;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $07FF40: blt.w $7ff84
pub fn i_07ff40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF44;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7FF84; return 10; }
    8
}
/// $07FF44: cmpi.w #$90, d0
pub fn i_07ff44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF48;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x90, Size::Word, false);
    8
}
/// $07FF48: ble.w $7ffbc
pub fn i_07ff48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF4C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7FFBC; return 10; }
    8
}
/// $07FF4C: moveq #$0, d0
pub fn i_07ff4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF4E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07FF4E: move.b $98(a6), d0
pub fn i_07ff4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF52;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07FF52: asl.w #$4, d0
pub fn i_07ff52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF54;
    cpu.ir = 0xE940;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $07FF54: cmp.w $22(a6), d0
pub fn i_07ff54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF58;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07FF58: bge.w $7ffbc
pub fn i_07ff58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF5C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x7FFBC; return 10; }
    8
}
/// $07FF5C: moveq #$0, d0
pub fn i_07ff5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF5E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07FF5E: move.w #$fffe, d1
pub fn i_07ff5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF62;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07FF62: add.w $28(a6), d1
pub fn i_07ff62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF66;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07FF66: cmpi.w #$fee0, d1
pub fn i_07ff66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF6A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFEE0, Size::Word, false);
    8
}
/// $07FF6A: bgt.w $7ff72
pub fn i_07ff6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF6E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7FF72; return 10; }
    8
}
/// $07FF6E: move.w #$fee0, d1
pub fn i_07ff6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF72;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFEE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07FF72: move.w d1, $28(a6)
pub fn i_07ff72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF76;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FF76: move.w #$18, $70(a6)
pub fn i_07ff76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF7C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FF7C: move.w #$1, $7c(a6)
pub fn i_07ff7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF82;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FF82: rts
pub fn i_07ff82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07FF84: moveq #$0, d0
pub fn i_07ff84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF86;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07FF86: move.b $99(a6), d0
pub fn i_07ff86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF8A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07FF8A: asl.w #$4, d0
pub fn i_07ff8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF8C;
    cpu.ir = 0xE940;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $07FF8C: cmp.w $22(a6), d0
pub fn i_07ff8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF90;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07FF90: ble.w $7ffbc
pub fn i_07ff90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF94;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7FFBC; return 10; }
    8
}
/// $07FF94: moveq #$0, d0
pub fn i_07ff94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF96;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07FF96: move.w #$2, d1
pub fn i_07ff96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF9A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07FF9A: add.w $28(a6), d1
pub fn i_07ff9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF9E;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07FF9E: cmpi.w #$120, d1
pub fn i_07ff9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFA2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x120, Size::Word, false);
    8
}
/// $07FFA2: blt.w $7ffaa
pub fn i_07ffa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFA6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7FFAA; return 10; }
    8
}
/// $07FFA6: move.w #$120, d1
pub fn i_07ffa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFAA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07FFAA: move.w d1, $28(a6)
pub fn i_07ffaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFAE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFAE: move.w #$30, $70(a6)
pub fn i_07ffae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFB4: move.w #$2, $7c(a6)
pub fn i_07ffb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFBA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFBA: rts
pub fn i_07ffba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07FFBC: move.w #$24, $70(a6)
pub fn i_07ffbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFC2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFC2: move.w $28(a6), d1
pub fn i_07ffc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFC6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FFC6: tst.w d1
pub fn i_07ffc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFC8;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $07FFC8: bpl.w $7ffe0
pub fn i_07ffc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFCC;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7FFE0; return 10; }
    8
}
/// $07FFCC: move.w #$1, $7c(a6)
pub fn i_07ffcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFD2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFD2: addq.w #$6, d1
pub fn i_07ffd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFD4;
    cpu.ir = 0x5C41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07FFD4: tst.w d1
pub fn i_07ffd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFD6;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $07FFD6: bpl.w $7ffee
pub fn i_07ffd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFDA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7FFEE; return 10; }
    8
}
/// $07FFDA: move.w d1, $28(a6)
pub fn i_07ffda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFDE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFDE: rts
pub fn i_07ffde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07FFE0: move.w #$2, $7c(a6)
pub fn i_07ffe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFE6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFE6: addi.w #$fffa, d1
pub fn i_07ffe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFEA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFFFA, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07FFEA: tst.w d1
pub fn i_07ffea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFEC;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $07FFEC: bpl.b $7ffda
pub fn i_07ffec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFEE;
    cpu.ir = 0x6AEC;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7FFDA; return 10; }
    8
}
/// $07FFEE: jsr $267e2.l
pub fn i_07ffee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FFF4);
    cpu.pc = 0x267E2;
    16
}
/// $07FFF4: move.w #$0, $7c(a6)
pub fn i_07fff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFFA: rts
pub fn i_07fffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07FFFC: move.w $72(a6), d0
pub fn i_07fffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80000;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x07F21E, i_07f21e),
    (0x07F222, i_07f222),
    (0x07F226, i_07f226),
    (0x07F228, i_07f228),
    (0x07F22A, i_07f22a),
    (0x07F230, i_07f230),
    (0x07F234, i_07f234),
    (0x07F238, i_07f238),
    (0x07F23E, i_07f23e),
    (0x07F242, i_07f242),
    (0x07F246, i_07f246),
    (0x07F24A, i_07f24a),
    (0x07F24E, i_07f24e),
    (0x07F252, i_07f252),
    (0x07F258, i_07f258),
    (0x07F25C, i_07f25c),
    (0x07F260, i_07f260),
    (0x07F264, i_07f264),
    (0x07F268, i_07f268),
    (0x07F26E, i_07f26e),
    (0x07F272, i_07f272),
    (0x07F278, i_07f278),
    (0x07F27A, i_07f27a),
    (0x07F27E, i_07f27e),
    (0x07F280, i_07f280),
    (0x07F282, i_07f282),
    (0x07F286, i_07f286),
    (0x07F28C, i_07f28c),
    (0x07F292, i_07f292),
    (0x07F298, i_07f298),
    (0x07F29C, i_07f29c),
    (0x07F29E, i_07f29e),
    (0x07F2A2, i_07f2a2),
    (0x07F2A8, i_07f2a8),
    (0x07F2AC, i_07f2ac),
    (0x07F2B0, i_07f2b0),
    (0x07F2B2, i_07f2b2),
    (0x07F2B8, i_07f2b8),
    (0x07F2BE, i_07f2be),
    (0x07F2C2, i_07f2c2),
    (0x07F2C6, i_07f2c6),
    (0x07F2C8, i_07f2c8),
    (0x07F2CA, i_07f2ca),
    (0x07F2D0, i_07f2d0),
    (0x07F2D4, i_07f2d4),
    (0x07F2DA, i_07f2da),
    (0x07F2DE, i_07f2de),
    (0x07F2E4, i_07f2e4),
    (0x07F2EA, i_07f2ea),
    (0x07F2F0, i_07f2f0),
    (0x07F2F4, i_07f2f4),
    (0x07F2FA, i_07f2fa),
    (0x07F2FE, i_07f2fe),
    (0x07F300, i_07f300),
    (0x07F304, i_07f304),
    (0x07F308, i_07f308),
    (0x07F30E, i_07f30e),
    (0x07F314, i_07f314),
    (0x07F318, i_07f318),
    (0x07F31E, i_07f31e),
    (0x07F324, i_07f324),
    (0x07F328, i_07f328),
    (0x07F32C, i_07f32c),
    (0x07F330, i_07f330),
    (0x07F334, i_07f334),
    (0x07F338, i_07f338),
    (0x07F33A, i_07f33a),
    (0x07F33E, i_07f33e),
    (0x07F342, i_07f342),
    (0x07F348, i_07f348),
    (0x07F34C, i_07f34c),
    (0x07F352, i_07f352),
    (0x07F358, i_07f358),
    (0x07F35E, i_07f35e),
    (0x07F362, i_07f362),
    (0x07F368, i_07f368),
    (0x07F36C, i_07f36c),
    (0x07F36E, i_07f36e),
    (0x07F372, i_07f372),
    (0x07F376, i_07f376),
    (0x07F37C, i_07f37c),
    (0x07F382, i_07f382),
    (0x07F386, i_07f386),
    (0x07F38C, i_07f38c),
    (0x07F392, i_07f392),
    (0x07F396, i_07f396),
    (0x07F39A, i_07f39a),
    (0x07F39E, i_07f39e),
    (0x07F3A2, i_07f3a2),
    (0x07F3A6, i_07f3a6),
    (0x07F3A8, i_07f3a8),
    (0x07F3AE, i_07f3ae),
    (0x07F3B4, i_07f3b4),
    (0x07F3B8, i_07f3b8),
    (0x07F3BC, i_07f3bc),
    (0x07F3C0, i_07f3c0),
    (0x07F3C4, i_07f3c4),
    (0x07F3CA, i_07f3ca),
    (0x07F3CE, i_07f3ce),
    (0x07F3D0, i_07f3d0),
    (0x07F3D6, i_07f3d6),
    (0x07F3DC, i_07f3dc),
    (0x07F3E0, i_07f3e0),
    (0x07F3E4, i_07f3e4),
    (0x07F3E6, i_07f3e6),
    (0x07F3E8, i_07f3e8),
    (0x07F3EC, i_07f3ec),
    (0x07F3F2, i_07f3f2),
    (0x07F3F6, i_07f3f6),
    (0x07F3FC, i_07f3fc),
    (0x07F402, i_07f402),
    (0x07F408, i_07f408),
    (0x07F40E, i_07f40e),
    (0x07F414, i_07f414),
    (0x07F418, i_07f418),
    (0x07F41C, i_07f41c),
    (0x07F422, i_07f422),
    (0x07F428, i_07f428),
    (0x07F42C, i_07f42c),
    (0x07F430, i_07f430),
    (0x07F434, i_07f434),
    (0x07F436, i_07f436),
    (0x07F43C, i_07f43c),
    (0x07F442, i_07f442),
    (0x07F446, i_07f446),
    (0x07F44A, i_07f44a),
    (0x07F44C, i_07f44c),
    (0x07F452, i_07f452),
    (0x07F458, i_07f458),
    (0x07F45C, i_07f45c),
    (0x07F460, i_07f460),
    (0x07F462, i_07f462),
    (0x07F464, i_07f464),
    (0x07F46A, i_07f46a),
    (0x07F470, i_07f470),
    (0x07F476, i_07f476),
    (0x07F47A, i_07f47a),
    (0x07F47E, i_07f47e),
    (0x07F484, i_07f484),
    (0x07F48A, i_07f48a),
    (0x07F48E, i_07f48e),
    (0x07F492, i_07f492),
    (0x07F496, i_07f496),
    (0x07F498, i_07f498),
    (0x07F49E, i_07f49e),
    (0x07F4A4, i_07f4a4),
    (0x07F4A8, i_07f4a8),
    (0x07F4AC, i_07f4ac),
    (0x07F4AE, i_07f4ae),
    (0x07F4B4, i_07f4b4),
    (0x07F4BA, i_07f4ba),
    (0x07F4BE, i_07f4be),
    (0x07F4C2, i_07f4c2),
    (0x07F4C4, i_07f4c4),
    (0x07F4C6, i_07f4c6),
    (0x07F4CA, i_07f4ca),
    (0x07F4D0, i_07f4d0),
    (0x07F4D6, i_07f4d6),
    (0x07F4DA, i_07f4da),
    (0x07F4E0, i_07f4e0),
    (0x07F4E6, i_07f4e6),
    (0x07F4EC, i_07f4ec),
    (0x07F4F2, i_07f4f2),
    (0x07F4F8, i_07f4f8),
    (0x07F4FC, i_07f4fc),
    (0x07F4FE, i_07f4fe),
    (0x07F504, i_07f504),
    (0x07F50A, i_07f50a),
    (0x07F50E, i_07f50e),
    (0x07F512, i_07f512),
    (0x07F514, i_07f514),
    (0x07F51A, i_07f51a),
    (0x07F520, i_07f520),
    (0x07F526, i_07f526),
    (0x07F52A, i_07f52a),
    (0x07F52E, i_07f52e),
    (0x07F530, i_07f530),
    (0x07F532, i_07f532),
    (0x07F538, i_07f538),
    (0x07F53E, i_07f53e),
    (0x07F544, i_07f544),
    (0x07F548, i_07f548),
    (0x07F54A, i_07f54a),
    (0x07F550, i_07f550),
    (0x07F554, i_07f554),
    (0x07F558, i_07f558),
    (0x07F55C, i_07f55c),
    (0x07F560, i_07f560),
    (0x07F562, i_07f562),
    (0x07F566, i_07f566),
    (0x07F56A, i_07f56a),
    (0x07F570, i_07f570),
    (0x07F576, i_07f576),
    (0x07F57C, i_07f57c),
    (0x07F580, i_07f580),
    (0x07F584, i_07f584),
    (0x07F586, i_07f586),
    (0x07F58C, i_07f58c),
    (0x07F592, i_07f592),
    (0x07F598, i_07f598),
    (0x07F59C, i_07f59c),
    (0x07F5A0, i_07f5a0),
    (0x07F5A2, i_07f5a2),
    (0x07F5A4, i_07f5a4),
    (0x07F5AA, i_07f5aa),
    (0x07F5B0, i_07f5b0),
    (0x07F5B4, i_07f5b4),
    (0x07F5B6, i_07f5b6),
    (0x07F5BC, i_07f5bc),
    (0x07F5C2, i_07f5c2),
    (0x07F5C6, i_07f5c6),
    (0x07F5CA, i_07f5ca),
    (0x07F5CC, i_07f5cc),
    (0x07F5D2, i_07f5d2),
    (0x07F5D8, i_07f5d8),
    (0x07F5DE, i_07f5de),
    (0x07F5E2, i_07f5e2),
    (0x07F5E6, i_07f5e6),
    (0x07F5E8, i_07f5e8),
    (0x07F5EA, i_07f5ea),
    (0x07F5F0, i_07f5f0),
    (0x07F5F6, i_07f5f6),
    (0x07F5FC, i_07f5fc),
    (0x07F602, i_07f602),
    (0x07F606, i_07f606),
    (0x07F608, i_07f608),
    (0x07F60E, i_07f60e),
    (0x07F614, i_07f614),
    (0x07F618, i_07f618),
    (0x07F61C, i_07f61c),
    (0x07F61E, i_07f61e),
    (0x07F624, i_07f624),
    (0x07F62A, i_07f62a),
    (0x07F630, i_07f630),
    (0x07F634, i_07f634),
    (0x07F638, i_07f638),
    (0x07F63A, i_07f63a),
    (0x07F63C, i_07f63c),
    (0x07F640, i_07f640),
    (0x07F646, i_07f646),
    (0x07F64A, i_07f64a),
    (0x07F64E, i_07f64e),
    (0x07F652, i_07f652),
    (0x07F656, i_07f656),
    (0x07F65A, i_07f65a),
    (0x07F660, i_07f660),
    (0x07F664, i_07f664),
    (0x07F66A, i_07f66a),
    (0x07F66C, i_07f66c),
    (0x07F670, i_07f670),
    (0x07F672, i_07f672),
    (0x07F674, i_07f674),
    (0x07F678, i_07f678),
    (0x07F67E, i_07f67e),
    (0x07F684, i_07f684),
    (0x07F688, i_07f688),
    (0x07F68E, i_07f68e),
    (0x07F694, i_07f694),
    (0x07F696, i_07f696),
    (0x07F69A, i_07f69a),
    (0x07F6A0, i_07f6a0),
    (0x07F6A4, i_07f6a4),
    (0x07F6AA, i_07f6aa),
    (0x07F6B0, i_07f6b0),
    (0x07F6B6, i_07f6b6),
    (0x07F6BA, i_07f6ba),
    (0x07F6BE, i_07f6be),
    (0x07F6C0, i_07f6c0),
    (0x07F6C6, i_07f6c6),
    (0x07F6CA, i_07f6ca),
    (0x07F6CE, i_07f6ce),
    (0x07F6D2, i_07f6d2),
    (0x07F6D8, i_07f6d8),
    (0x07F6DC, i_07f6dc),
    (0x07F6E2, i_07f6e2),
    (0x07F6E8, i_07f6e8),
    (0x07F6EE, i_07f6ee),
    (0x07F6F2, i_07f6f2),
    (0x07F6F6, i_07f6f6),
    (0x07F6FA, i_07f6fa),
    (0x07F700, i_07f700),
    (0x07F704, i_07f704),
    (0x07F70A, i_07f70a),
    (0x07F710, i_07f710),
    (0x07F714, i_07f714),
    (0x07F718, i_07f718),
    (0x07F71E, i_07f71e),
    (0x07F722, i_07f722),
    (0x07F726, i_07f726),
    (0x07F72A, i_07f72a),
    (0x07F72E, i_07f72e),
    (0x07F732, i_07f732),
    (0x07F738, i_07f738),
    (0x07F73E, i_07f73e),
    (0x07F742, i_07f742),
    (0x07F748, i_07f748),
    (0x07F74C, i_07f74c),
    (0x07F752, i_07f752),
    (0x07F756, i_07f756),
    (0x07F758, i_07f758),
    (0x07F75C, i_07f75c),
    (0x07F760, i_07f760),
    (0x07F764, i_07f764),
    (0x07F766, i_07f766),
    (0x07F768, i_07f768),
    (0x07F76C, i_07f76c),
    (0x07F772, i_07f772),
    (0x07F778, i_07f778),
    (0x07F77C, i_07f77c),
    (0x07F782, i_07f782),
    (0x07F788, i_07f788),
    (0x07F78E, i_07f78e),
    (0x07F794, i_07f794),
    (0x07F79A, i_07f79a),
    (0x07F7A0, i_07f7a0),
    (0x07F7A6, i_07f7a6),
    (0x07F7AC, i_07f7ac),
    (0x07F7B0, i_07f7b0),
    (0x07F7B2, i_07f7b2),
    (0x07F7B6, i_07f7b6),
    (0x07F7BA, i_07f7ba),
    (0x07F7C0, i_07f7c0),
    (0x07F7C6, i_07f7c6),
    (0x07F7CC, i_07f7cc),
    (0x07F7D2, i_07f7d2),
    (0x07F7D6, i_07f7d6),
    (0x07F7DA, i_07f7da),
    (0x07F7DC, i_07f7dc),
    (0x07F7E0, i_07f7e0),
    (0x07F7E4, i_07f7e4),
    (0x07F7E8, i_07f7e8),
    (0x07F7EC, i_07f7ec),
    (0x07F7EE, i_07f7ee),
    (0x07F7F0, i_07f7f0),
    (0x07F7F4, i_07f7f4),
    (0x07F7FA, i_07f7fa),
    (0x07F800, i_07f800),
    (0x07F806, i_07f806),
    (0x07F80A, i_07f80a),
    (0x07F80C, i_07f80c),
    (0x07F810, i_07f810),
    (0x07F814, i_07f814),
    (0x07F81A, i_07f81a),
    (0x07F820, i_07f820),
    (0x07F826, i_07f826),
    (0x07F82C, i_07f82c),
    (0x07F830, i_07f830),
    (0x07F836, i_07f836),
    (0x07F83A, i_07f83a),
    (0x07F83E, i_07f83e),
    (0x07F842, i_07f842),
    (0x07F846, i_07f846),
    (0x07F848, i_07f848),
    (0x07F84A, i_07f84a),
    (0x07F84E, i_07f84e),
    (0x07F854, i_07f854),
    (0x07F85A, i_07f85a),
    (0x07F85E, i_07f85e),
    (0x07F862, i_07f862),
    (0x07F868, i_07f868),
    (0x07F86E, i_07f86e),
    (0x07F874, i_07f874),
    (0x07F878, i_07f878),
    (0x07F87A, i_07f87a),
    (0x07F87C, i_07f87c),
    (0x07F880, i_07f880),
    (0x07F884, i_07f884),
    (0x07F88A, i_07f88a),
    (0x07F890, i_07f890),
    (0x07F896, i_07f896),
    (0x07F89A, i_07f89a),
    (0x07F8A0, i_07f8a0),
    (0x07F8A4, i_07f8a4),
    (0x07F8AA, i_07f8aa),
    (0x07F8AE, i_07f8ae),
    (0x07F8B2, i_07f8b2),
    (0x07F8B4, i_07f8b4),
    (0x07F8B6, i_07f8b6),
    (0x07F8BC, i_07f8bc),
    (0x07F8C0, i_07f8c0),
    (0x07F8C6, i_07f8c6),
    (0x07F8CC, i_07f8cc),
    (0x07F8D0, i_07f8d0),
    (0x07F8D4, i_07f8d4),
    (0x07F8D8, i_07f8d8),
    (0x07F8DA, i_07f8da),
    (0x07F8DC, i_07f8dc),
    (0x07F8E0, i_07f8e0),
    (0x07F8E6, i_07f8e6),
    (0x07F8EC, i_07f8ec),
    (0x07F8F2, i_07f8f2),
    (0x07F8F8, i_07f8f8),
    (0x07F8FE, i_07f8fe),
    (0x07F918, i_07f918),
    (0x07F91C, i_07f91c),
    (0x07F922, i_07f922),
    (0x07F928, i_07f928),
    (0x07F92E, i_07f92e),
    (0x07F934, i_07f934),
    (0x07F938, i_07f938),
    (0x07F93A, i_07f93a),
    (0x07F93E, i_07f93e),
    (0x07F942, i_07f942),
    (0x07F948, i_07f948),
    (0x07F94E, i_07f94e),
    (0x07F954, i_07f954),
    (0x07F95A, i_07f95a),
    (0x07F95E, i_07f95e),
    (0x07F964, i_07f964),
    (0x07F968, i_07f968),
    (0x07F96A, i_07f96a),
    (0x07F96E, i_07f96e),
    (0x07F972, i_07f972),
    (0x07F974, i_07f974),
    (0x07F978, i_07f978),
    (0x07F97C, i_07f97c),
    (0x07F980, i_07f980),
    (0x07F984, i_07f984),
    (0x07F986, i_07f986),
    (0x07F988, i_07f988),
    (0x07F98C, i_07f98c),
    (0x07F990, i_07f990),
    (0x07F996, i_07f996),
    (0x07F99C, i_07f99c),
    (0x07F9A2, i_07f9a2),
    (0x07F9A6, i_07f9a6),
    (0x07F9A8, i_07f9a8),
    (0x07F9AC, i_07f9ac),
    (0x07F9B2, i_07f9b2),
    (0x07F9B6, i_07f9b6),
    (0x07F9BA, i_07f9ba),
    (0x07F9BC, i_07f9bc),
    (0x07F9C0, i_07f9c0),
    (0x07F9C4, i_07f9c4),
    (0x07F9C8, i_07f9c8),
    (0x07F9CC, i_07f9cc),
    (0x07F9CE, i_07f9ce),
    (0x07F9D0, i_07f9d0),
    (0x07F9D4, i_07f9d4),
    (0x07F9D8, i_07f9d8),
    (0x07F9DE, i_07f9de),
    (0x07F9E2, i_07f9e2),
    (0x07F9E6, i_07f9e6),
    (0x07F9EC, i_07f9ec),
    (0x07F9F2, i_07f9f2),
    (0x07F9F6, i_07f9f6),
    (0x07F9F8, i_07f9f8),
    (0x07F9FC, i_07f9fc),
    (0x07FA02, i_07fa02),
    (0x07FA06, i_07fa06),
    (0x07FA0A, i_07fa0a),
    (0x07FA0C, i_07fa0c),
    (0x07FA10, i_07fa10),
    (0x07FA14, i_07fa14),
    (0x07FA18, i_07fa18),
    (0x07FA1C, i_07fa1c),
    (0x07FA1E, i_07fa1e),
    (0x07FA20, i_07fa20),
    (0x07FA26, i_07fa26),
    (0x07FA2C, i_07fa2c),
    (0x07FA32, i_07fa32),
    (0x07FA38, i_07fa38),
    (0x07FA3C, i_07fa3c),
    (0x07FA40, i_07fa40),
    (0x07FA44, i_07fa44),
    (0x07FA4A, i_07fa4a),
    (0x07FA50, i_07fa50),
    (0x07FA54, i_07fa54),
    (0x07FA56, i_07fa56),
    (0x07FA5A, i_07fa5a),
    (0x07FA60, i_07fa60),
    (0x07FA64, i_07fa64),
    (0x07FA68, i_07fa68),
    (0x07FA6A, i_07fa6a),
    (0x07FA6E, i_07fa6e),
    (0x07FA72, i_07fa72),
    (0x07FA78, i_07fa78),
    (0x07FA7C, i_07fa7c),
    (0x07FA7E, i_07fa7e),
    (0x07FA82, i_07fa82),
    (0x07FA86, i_07fa86),
    (0x07FA8A, i_07fa8a),
    (0x07FA8E, i_07fa8e),
    (0x07FA90, i_07fa90),
    (0x07FA92, i_07fa92),
    (0x07FA98, i_07fa98),
    (0x07FA9E, i_07fa9e),
    (0x07FAA2, i_07faa2),
    (0x07FAA4, i_07faa4),
    (0x07FAA8, i_07faa8),
    (0x07FAAE, i_07faae),
    (0x07FAB2, i_07fab2),
    (0x07FAB6, i_07fab6),
    (0x07FAB8, i_07fab8),
    (0x07FABC, i_07fabc),
    (0x07FAC0, i_07fac0),
    (0x07FAC4, i_07fac4),
    (0x07FAC8, i_07fac8),
    (0x07FACA, i_07faca),
    (0x07FACC, i_07facc),
    (0x07FAD0, i_07fad0),
    (0x07FAD4, i_07fad4),
    (0x07FAD8, i_07fad8),
    (0x07FADE, i_07fade),
    (0x07FAE4, i_07fae4),
    (0x07FAEA, i_07faea),
    (0x07FAEE, i_07faee),
    (0x07FAF0, i_07faf0),
    (0x07FAF4, i_07faf4),
    (0x07FAFA, i_07fafa),
    (0x07FAFE, i_07fafe),
    (0x07FB02, i_07fb02),
    (0x07FB06, i_07fb06),
    (0x07FB0A, i_07fb0a),
    (0x07FB0E, i_07fb0e),
    (0x07FB12, i_07fb12),
    (0x07FB14, i_07fb14),
    (0x07FB18, i_07fb18),
    (0x07FB1C, i_07fb1c),
    (0x07FB20, i_07fb20),
    (0x07FB24, i_07fb24),
    (0x07FB26, i_07fb26),
    (0x07FB28, i_07fb28),
    (0x07FB2C, i_07fb2c),
    (0x07FB32, i_07fb32),
    (0x07FB38, i_07fb38),
    (0x07FB3C, i_07fb3c),
    (0x07FB3E, i_07fb3e),
    (0x07FB42, i_07fb42),
    (0x07FB48, i_07fb48),
    (0x07FB4C, i_07fb4c),
    (0x07FB50, i_07fb50),
    (0x07FB54, i_07fb54),
    (0x07FB56, i_07fb56),
    (0x07FB5A, i_07fb5a),
    (0x07FB5E, i_07fb5e),
    (0x07FB62, i_07fb62),
    (0x07FB66, i_07fb66),
    (0x07FB6A, i_07fb6a),
    (0x07FB6C, i_07fb6c),
    (0x07FB6E, i_07fb6e),
    (0x07FB74, i_07fb74),
    (0x07FB78, i_07fb78),
    (0x07FB7C, i_07fb7c),
    (0x07FB80, i_07fb80),
    (0x07FB86, i_07fb86),
    (0x07FB8A, i_07fb8a),
    (0x07FB8E, i_07fb8e),
    (0x07FB94, i_07fb94),
    (0x07FB98, i_07fb98),
    (0x07FB9E, i_07fb9e),
    (0x07FBA4, i_07fba4),
    (0x07FBA8, i_07fba8),
    (0x07FBAA, i_07fbaa),
    (0x07FBAE, i_07fbae),
    (0x07FBB4, i_07fbb4),
    (0x07FBB8, i_07fbb8),
    (0x07FBBC, i_07fbbc),
    (0x07FBBE, i_07fbbe),
    (0x07FBC2, i_07fbc2),
    (0x07FBC6, i_07fbc6),
    (0x07FBCA, i_07fbca),
    (0x07FBCE, i_07fbce),
    (0x07FBD0, i_07fbd0),
    (0x07FBD2, i_07fbd2),
    (0x07FBD8, i_07fbd8),
    (0x07FBDC, i_07fbdc),
    (0x07FBE0, i_07fbe0),
    (0x07FBE6, i_07fbe6),
    (0x07FBEC, i_07fbec),
    (0x07FBF0, i_07fbf0),
    (0x07FBF2, i_07fbf2),
    (0x07FBF6, i_07fbf6),
    (0x07FBFC, i_07fbfc),
    (0x07FC00, i_07fc00),
    (0x07FC04, i_07fc04),
    (0x07FC06, i_07fc06),
    (0x07FC0A, i_07fc0a),
    (0x07FC0E, i_07fc0e),
    (0x07FC12, i_07fc12),
    (0x07FC16, i_07fc16),
    (0x07FC18, i_07fc18),
    (0x07FC1A, i_07fc1a),
    (0x07FC20, i_07fc20),
    (0x07FC26, i_07fc26),
    (0x07FC2C, i_07fc2c),
    (0x07FC32, i_07fc32),
    (0x07FC38, i_07fc38),
    (0x07FC3E, i_07fc3e),
    (0x07FC44, i_07fc44),
    (0x07FC4A, i_07fc4a),
    (0x07FC4E, i_07fc4e),
    (0x07FC50, i_07fc50),
    (0x07FCBE, i_07fcbe),
    (0x07FCC2, i_07fcc2),
    (0x07FCC8, i_07fcc8),
    (0x07FCCE, i_07fcce),
    (0x07FCD4, i_07fcd4),
    (0x07FCDA, i_07fcda),
    (0x07FCDE, i_07fcde),
    (0x07FCE4, i_07fce4),
    (0x07FCE8, i_07fce8),
    (0x07FCEC, i_07fcec),
    (0x07FCF0, i_07fcf0),
    (0x07FCF6, i_07fcf6),
    (0x07FCFC, i_07fcfc),
    (0x07FD02, i_07fd02),
    (0x07FD08, i_07fd08),
    (0x07FD0E, i_07fd0e),
    (0x07FD14, i_07fd14),
    (0x07FD18, i_07fd18),
    (0x07FD1E, i_07fd1e),
    (0x07FD24, i_07fd24),
    (0x07FD2A, i_07fd2a),
    (0x07FD2E, i_07fd2e),
    (0x07FD32, i_07fd32),
    (0x07FD36, i_07fd36),
    (0x07FD38, i_07fd38),
    (0x07FD3C, i_07fd3c),
    (0x07FD40, i_07fd40),
    (0x07FD44, i_07fd44),
    (0x07FD46, i_07fd46),
    (0x07FD48, i_07fd48),
    (0x07FD4C, i_07fd4c),
    (0x07FD52, i_07fd52),
    (0x07FD58, i_07fd58),
    (0x07FD5E, i_07fd5e),
    (0x07FD62, i_07fd62),
    (0x07FD68, i_07fd68),
    (0x07FD6C, i_07fd6c),
    (0x07FD70, i_07fd70),
    (0x07FD74, i_07fd74),
    (0x07FD7A, i_07fd7a),
    (0x07FD7E, i_07fd7e),
    (0x07FD82, i_07fd82),
    (0x07FD88, i_07fd88),
    (0x07FD8A, i_07fd8a),
    (0x07FD8E, i_07fd8e),
    (0x07FD92, i_07fd92),
    (0x07FD98, i_07fd98),
    (0x07FD9C, i_07fd9c),
    (0x07FDA0, i_07fda0),
    (0x07FDA4, i_07fda4),
    (0x07FDA6, i_07fda6),
    (0x07FDA8, i_07fda8),
    (0x07FDAC, i_07fdac),
    (0x07FDB2, i_07fdb2),
    (0x07FDBC, i_07fdbc),
    (0x07FDC4, i_07fdc4),
    (0x07FDCA, i_07fdca),
    (0x07FDD0, i_07fdd0),
    (0x07FDD4, i_07fdd4),
    (0x07FDD8, i_07fdd8),
    (0x07FDDA, i_07fdda),
    (0x07FDDE, i_07fdde),
    (0x07FDE0, i_07fde0),
    (0x07FDE4, i_07fde4),
    (0x07FDEA, i_07fdea),
    (0x07FDEE, i_07fdee),
    (0x07FDF2, i_07fdf2),
    (0x07FDF4, i_07fdf4),
    (0x07FDF8, i_07fdf8),
    (0x07FDFA, i_07fdfa),
    (0x07FDFE, i_07fdfe),
    (0x07FE04, i_07fe04),
    (0x07FE0A, i_07fe0a),
    (0x07FE0E, i_07fe0e),
    (0x07FE10, i_07fe10),
    (0x07FE14, i_07fe14),
    (0x07FE18, i_07fe18),
    (0x07FE1E, i_07fe1e),
    (0x07FE24, i_07fe24),
    (0x07FE28, i_07fe28),
    (0x07FE2A, i_07fe2a),
    (0x07FE2E, i_07fe2e),
    (0x07FE30, i_07fe30),
    (0x07FE34, i_07fe34),
    (0x07FE3A, i_07fe3a),
    (0x07FE3C, i_07fe3c),
    (0x07FE40, i_07fe40),
    (0x07FE44, i_07fe44),
    (0x07FE4A, i_07fe4a),
    (0x07FE4E, i_07fe4e),
    (0x07FE50, i_07fe50),
    (0x07FE52, i_07fe52),
    (0x07FE56, i_07fe56),
    (0x07FE5C, i_07fe5c),
    (0x07FE60, i_07fe60),
    (0x07FE64, i_07fe64),
    (0x07FE66, i_07fe66),
    (0x07FE6A, i_07fe6a),
    (0x07FE6E, i_07fe6e),
    (0x07FE74, i_07fe74),
    (0x07FE7A, i_07fe7a),
    (0x07FE7E, i_07fe7e),
    (0x07FE82, i_07fe82),
    (0x07FE86, i_07fe86),
    (0x07FE8A, i_07fe8a),
    (0x07FE8C, i_07fe8c),
    (0x07FE90, i_07fe90),
    (0x07FE94, i_07fe94),
    (0x07FE98, i_07fe98),
    (0x07FE9C, i_07fe9c),
    (0x07FE9E, i_07fe9e),
    (0x07FEA0, i_07fea0),
    (0x07FEA6, i_07fea6),
    (0x07FEAA, i_07feaa),
    (0x07FEAE, i_07feae),
    (0x07FEB2, i_07feb2),
    (0x07FEB4, i_07feb4),
    (0x07FEBA, i_07feba),
    (0x07FEC0, i_07fec0),
    (0x07FEC4, i_07fec4),
    (0x07FEC8, i_07fec8),
    (0x07FECC, i_07fecc),
    (0x07FECE, i_07fece),
    (0x07FED4, i_07fed4),
    (0x07FED8, i_07fed8),
    (0x07FEDC, i_07fedc),
    (0x07FEDE, i_07fede),
    (0x07FEE2, i_07fee2),
    (0x07FEE6, i_07fee6),
    (0x07FEE8, i_07fee8),
    (0x07FEEE, i_07feee),
    (0x07FEF2, i_07fef2),
    (0x07FEF8, i_07fef8),
    (0x07FEFC, i_07fefc),
    (0x07FF00, i_07ff00),
    (0x07FF06, i_07ff06),
    (0x07FF0A, i_07ff0a),
    (0x07FF0E, i_07ff0e),
    (0x07FF14, i_07ff14),
    (0x07FF18, i_07ff18),
    (0x07FF1E, i_07ff1e),
    (0x07FF24, i_07ff24),
    (0x07FF28, i_07ff28),
    (0x07FF2E, i_07ff2e),
    (0x07FF32, i_07ff32),
    (0x07FF34, i_07ff34),
    (0x07FF38, i_07ff38),
    (0x07FF3C, i_07ff3c),
    (0x07FF40, i_07ff40),
    (0x07FF44, i_07ff44),
    (0x07FF48, i_07ff48),
    (0x07FF4C, i_07ff4c),
    (0x07FF4E, i_07ff4e),
    (0x07FF52, i_07ff52),
    (0x07FF54, i_07ff54),
    (0x07FF58, i_07ff58),
    (0x07FF5C, i_07ff5c),
    (0x07FF5E, i_07ff5e),
    (0x07FF62, i_07ff62),
    (0x07FF66, i_07ff66),
    (0x07FF6A, i_07ff6a),
    (0x07FF6E, i_07ff6e),
    (0x07FF72, i_07ff72),
    (0x07FF76, i_07ff76),
    (0x07FF7C, i_07ff7c),
    (0x07FF82, i_07ff82),
    (0x07FF84, i_07ff84),
    (0x07FF86, i_07ff86),
    (0x07FF8A, i_07ff8a),
    (0x07FF8C, i_07ff8c),
    (0x07FF90, i_07ff90),
    (0x07FF94, i_07ff94),
    (0x07FF96, i_07ff96),
    (0x07FF9A, i_07ff9a),
    (0x07FF9E, i_07ff9e),
    (0x07FFA2, i_07ffa2),
    (0x07FFA6, i_07ffa6),
    (0x07FFAA, i_07ffaa),
    (0x07FFAE, i_07ffae),
    (0x07FFB4, i_07ffb4),
    (0x07FFBA, i_07ffba),
    (0x07FFBC, i_07ffbc),
    (0x07FFC2, i_07ffc2),
    (0x07FFC6, i_07ffc6),
    (0x07FFC8, i_07ffc8),
    (0x07FFCC, i_07ffcc),
    (0x07FFD2, i_07ffd2),
    (0x07FFD4, i_07ffd4),
    (0x07FFD6, i_07ffd6),
    (0x07FFDA, i_07ffda),
    (0x07FFDE, i_07ffde),
    (0x07FFE0, i_07ffe0),
    (0x07FFE6, i_07ffe6),
    (0x07FFEA, i_07ffea),
    (0x07FFEC, i_07ffec),
    (0x07FFEE, i_07ffee),
    (0x07FFF4, i_07fff4),
    (0x07FFFA, i_07fffa),
    (0x07FFFC, i_07fffc),
];
