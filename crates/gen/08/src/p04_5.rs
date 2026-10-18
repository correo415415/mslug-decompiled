// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_5) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $0483F8: tst.b $9b(a6)
pub fn i_0483f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x483FC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0483FC: bne.w $4857c
pub fn i_0483fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48400;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4857C; return 10; }
    8
}
/// $048400: bra.w $48710
pub fn i_048400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48404;
    cpu.ir = 0x6000;
    cpu.pc = 0x48710;
    10
}
/// $048404: tst.b $9c(a6)
pub fn i_048404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48408;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $048408: beq.w $48418
pub fn i_048408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4840C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x48418; return 10; }
    8
}
/// $04840C: move.b $9c(a6), d0
pub fn i_04840c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48410;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $048410: andi.w #$ff, d0
pub fn i_048410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48414;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $048414: move.w d0, $72(a6)
pub fn i_048414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48418;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048418: jsr $4932c(pc)
pub fn i_048418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4841C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4841C);
    cpu.pc = 0x4932C;
    16
}
/// $04841C: bcc.w $48492
pub fn i_04841c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48420;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48492; return 10; }
    8
}
// ===== Pow_FreeHurt_048420  ($048420, 54 B, pow/prisoner_0478fc.s)
/// $048420: move.w $28(a6), d0
pub fn i_048420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48424;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $048424: asr.w #$1, d0
pub fn i_048424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48426;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $048426: move.w d0, $28(a6)
pub fn i_048426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4842A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04842A: lea.l $28e75e.l, a0
pub fn i_04842a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48430;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E75E;
    20
}
/// $048430: jsr $28cd4.l
pub fn i_048430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48436);
    cpu.pc = 0x28CD4;
    16
}
/// $048436: lea.l $4843c(pc), a1
pub fn i_048436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4843A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4843C;
    16
}
/// $04843A: move.l a1, (a6)
pub fn i_04843a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4843C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04843C: jsr $27c8c.l
pub fn i_04843c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48442);
    cpu.pc = 0x27C8C;
    16
}
/// $048442: bcc.w $4844c
pub fn i_048442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48446;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4844C; return 10; }
    8
}
/// $048446: lea.l $48456(pc), a1
pub fn i_048446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4844A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48456;
    16
}
/// $04844A: move.l a1, (a6)
pub fn i_04844a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4844C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04844C: jsr $28d70.l
pub fn i_04844c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48452);
    cpu.pc = 0x28D70;
    16
}
/// $048452: bra.w $489f8
pub fn i_048452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48456;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeGetUp_048456  ($048456, 60 B, pow/prisoner_0478fc.s)
/// $048456: clr.w $28(a6)
pub fn i_048456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4845A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04845A: lea.l $28e79c.l, a0
pub fn i_04845a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48460;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E79C;
    20
}
/// $048460: jsr $28cd4.l
pub fn i_048460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48466;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48466);
    cpu.pc = 0x28CD4;
    16
}
/// $048466: lea.l $4846c(pc), a1
pub fn i_048466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4846A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4846C;
    16
}
/// $04846A: move.l a1, (a6)
pub fn i_04846a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4846C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04846C: jsr $48fb0(pc)
pub fn i_04846c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48470;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48470);
    cpu.pc = 0x48FB0;
    16
}
/// $048470: jsr $28d70.l
pub fn i_048470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48476;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48476);
    cpu.pc = 0x28D70;
    16
}
/// $048476: bcc.w $48480
pub fn i_048476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4847A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48480; return 10; }
    8
}
/// $04847A: lea.l $48492(pc), a1
pub fn i_04847a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4847E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48492;
    16
}
/// $04847E: move.l a1, (a6)
pub fn i_04847e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48480;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048480: jsr $4932c(pc)
pub fn i_048480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48484;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48484);
    cpu.pc = 0x4932C;
    16
}
/// $048484: bcc.w $4848e
pub fn i_048484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48488;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4848E; return 10; }
    8
}
/// $048488: lea.l $48420(pc), a1
pub fn i_048488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4848C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $04848C: move.l a1, (a6)
pub fn i_04848c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4848E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04848E: bra.w $489f8
pub fn i_04848e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48492;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeWalkOut_048492  ($048492, 112 B, pow/prisoner_0478fc.s)
/// $048492: lea.l $2bfb2e.l, a0
pub fn i_048492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48498;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFB2E;
    20
}
/// $048498: jsr $799de.l
pub fn i_048498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4849E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4849E);
    cpu.pc = 0x799DE;
    16
}
/// $04849E: move.w d0, $36(a6)
pub fn i_04849e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484A2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0484A2: jsr $48f04(pc)
pub fn i_0484a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x484A6);
    cpu.pc = 0x48F04;
    16
}
/// $0484A6: lea.l $28e394.l, a0
pub fn i_0484a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E394;
    20
}
/// $0484AC: jsr $28cd4.l
pub fn i_0484ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x484B2);
    cpu.pc = 0x28CD4;
    16
}
/// $0484B2: lea.l $484b8(pc), a1
pub fn i_0484b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x484B8;
    16
}
/// $0484B6: move.l a1, (a6)
pub fn i_0484b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0484B8: jsr $48f2e(pc)
pub fn i_0484b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x484BC);
    cpu.pc = 0x48F2E;
    16
}
/// $0484BC: jsr $28d70.l
pub fn i_0484bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x484C2);
    cpu.pc = 0x28D70;
    16
}
/// $0484C2: move.b $98(a6), d0
pub fn i_0484c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484C6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0484C6: andi.w #$ff, d0
pub fn i_0484c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484CA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0484CA: lsl.w #$4, d0
pub fn i_0484ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484CC;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0484CC: btst.b #$0, $3a(a6)
pub fn i_0484cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484D2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0484D2: bne.w $484e2
pub fn i_0484d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484D6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x484E2; return 10; }
    8
}
/// $0484D6: cmp.w $22(a6), d0
pub fn i_0484d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484DA;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0484DA: bgt.w $484ea
pub fn i_0484da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484DE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x484EA; return 10; }
    8
}
/// $0484DE: bra.w $484f0
pub fn i_0484de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x484F0;
    10
}
/// $0484E2: cmp.w $22(a6), d0
pub fn i_0484e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484E6;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0484E6: bgt.w $484f0
pub fn i_0484e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484EA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x484F0; return 10; }
    8
}
/// $0484EA: lea.l $48502(pc), a1
pub fn i_0484ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48502;
    16
}
/// $0484EE: move.l a1, (a6)
pub fn i_0484ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0484F0: jsr $4932c(pc)
pub fn i_0484f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x484F4);
    cpu.pc = 0x4932C;
    16
}
/// $0484F4: bcc.w $484fe
pub fn i_0484f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x484FE; return 10; }
    8
}
/// $0484F8: lea.l $48420(pc), a1
pub fn i_0484f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $0484FC: move.l a1, (a6)
pub fn i_0484fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x484FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0484FE: bra.w $489f8
pub fn i_0484fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48502;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeJumpOut_048502  ($048502, 122 B, pow/prisoner_0478fc.s)
/// $048502: tst.b $9b(a6)
pub fn i_048502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48506;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $048506: beq.w $486d4
pub fn i_048506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4850A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x486D4; return 10; }
    8
}
/// $04850A: move.w #$fccd, d0
pub fn i_04850a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4850E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFCCD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04850E: jsr $5dca4.l
pub fn i_04850e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48514);
    cpu.pc = 0x5DCA4;
    16
}
/// $048514: move.w d0, $28(a6)
pub fn i_048514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48518;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048518: move.w #$ccb, $2a(a6)
pub fn i_048518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4851E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xCCB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04851E: move.w #$fae2, $2e(a6)
pub fn i_04851e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48524;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFAE2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048524: move.w #$0, $2c(a6)
pub fn i_048524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4852A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04852A: lea.l $28e656.l, a0
pub fn i_04852a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48530;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E656;
    20
}
/// $048530: jsr $28cd4.l
pub fn i_048530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48536;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48536);
    cpu.pc = 0x28CD4;
    16
}
/// $048536: lea.l $4853c(pc), a1
pub fn i_048536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4853A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4853C;
    16
}
/// $04853A: move.l a1, (a6)
pub fn i_04853a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4853C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04853C: jsr $27bc8.l
pub fn i_04853c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48542;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48542);
    cpu.pc = 0x27BC8;
    16
}
/// $048542: bcc.w $4854a
pub fn i_048542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48546;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4854A; return 10; }
    8
}
/// $048546: clr.w $28(a6)
pub fn i_048546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4854A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04854A: jsr $28d70.l
pub fn i_04854a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48550);
    cpu.pc = 0x28D70;
    16
}
/// $048550: bcc.w $48562
pub fn i_048550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48554;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48562; return 10; }
    8
}
/// $048554: tst.w $28(a6)
pub fn i_048554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48558;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $048558: bne.w $48562
pub fn i_048558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4855C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x48562; return 10; }
    8
}
/// $04855C: lea.l $4857c(pc), a1
pub fn i_04855c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48560;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4857C;
    16
}
/// $048560: move.l a1, (a6)
pub fn i_048560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48562;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048562: tst.b $86(a6)
pub fn i_048562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48566;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $048566: beq.w $48578
pub fn i_048566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4856A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x48578; return 10; }
    8
}
/// $04856A: jsr $4932c(pc)
pub fn i_04856a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4856E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4856E);
    cpu.pc = 0x4932C;
    16
}
/// $04856E: bcc.w $48578
pub fn i_04856e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48572;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48578; return 10; }
    8
}
/// $048572: lea.l $48420(pc), a1
pub fn i_048572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48576;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $048576: move.l a1, (a6)
pub fn i_048576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48578;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048578: bra.w $489f8
pub fn i_048578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4857C;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeThanksLoop_04857c  ($04857C, 108 B, pow/prisoner_0478fc.s)
/// $04857C: clr.w $28(a6)
pub fn i_04857c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48580;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $048580: move.l #$28e03e, $48(a6)
pub fn i_048580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48588;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28E03E;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048588: lea.l $28e6b0.l, a0
pub fn i_048588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4858E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6B0;
    20
}
/// $04858E: jsr $28cd4.l
pub fn i_04858e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48594;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48594);
    cpu.pc = 0x28CD4;
    16
}
/// $048594: lea.l $4859a(pc), a1
pub fn i_048594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48598;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4859A;
    16
}
/// $048598: move.l a1, (a6)
pub fn i_048598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4859A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04859A: jsr $48fb0(pc)
pub fn i_04859a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4859E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4859E);
    cpu.pc = 0x48FB0;
    16
}
/// $04859E: jsr $28d70.l
pub fn i_04859e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x485A4);
    cpu.pc = 0x28D70;
    16
}
/// $0485A4: jsr $492a4(pc)
pub fn i_0485a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x485A8);
    cpu.pc = 0x492A4;
    16
}
/// $0485A8: bcc.w $485b2
pub fn i_0485a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x485B2; return 10; }
    8
}
/// $0485AC: lea.l $4867a(pc), a1
pub fn i_0485ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4867A;
    16
}
/// $0485B0: move.l a1, (a6)
pub fn i_0485b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0485B2: cmpi.w #$18, $22(a6)
pub fn i_0485b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485B8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x18, Size::Word, false);
    16
}
/// $0485B8: blt.w $485d6
pub fn i_0485b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485BC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x485D6; return 10; }
    8
}
/// $0485BC: cmpi.w #$128, $22(a6)
pub fn i_0485bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485C2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x128, Size::Word, false);
    16
}
/// $0485C2: bgt.w $485d6
pub fn i_0485c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485C6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x485D6; return 10; }
    8
}
/// $0485C6: cmpi.w #$0, $72(a6)
pub fn i_0485c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485CC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0485CC: bgt.w $485d6
pub fn i_0485cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485D0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x485D6; return 10; }
    8
}
/// $0485D0: lea.l $485e8(pc), a1
pub fn i_0485d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x485E8;
    16
}
/// $0485D4: move.l a1, (a6)
pub fn i_0485d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0485D6: jsr $4932c(pc)
pub fn i_0485d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x485DA);
    cpu.pc = 0x4932C;
    16
}
/// $0485DA: bcc.w $485e4
pub fn i_0485da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x485E4; return 10; }
    8
}
/// $0485DE: lea.l $48420(pc), a1
pub fn i_0485de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $0485E2: move.l a1, (a6)
pub fn i_0485e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0485E4: bra.w $48a44
pub fn i_0485e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A44;
    10
}
// ===== Pow_FreeThanksInit_0485e8  ($0485E8, 20 B, pow/prisoner_0478fc.s)
/// $0485E8: clr.w $28(a6)
pub fn i_0485e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485EC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0485EC: lea.l $2bfd36.l, a0
pub fn i_0485ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFD36;
    20
}
/// $0485F2: jsr $799de.l
pub fn i_0485f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x485F8);
    cpu.pc = 0x799DE;
    16
}
/// $0485F8: move.b d0, $82(a6)
pub fn i_0485f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x485FC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Pow_FreeThanks_0485fc  ($0485FC, 126 B, pow/prisoner_0478fc.s)
/// $0485FC: lea.l $2bfcb4.l, a0
pub fn i_0485fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48602;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFCB4;
    20
}
/// $048602: jsr $799de.l
pub fn i_048602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48608;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48608);
    cpu.pc = 0x799DE;
    16
}
/// $048608: move.w d0, $72(a6)
pub fn i_048608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4860C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04860C: move.l #$28e092, $48(a6)
pub fn i_04860c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48614;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28E092;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048614: lea.l $28e6c8.l, a0
pub fn i_048614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4861A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6C8;
    20
}
/// $04861A: jsr $28cd4.l
pub fn i_04861a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48620);
    cpu.pc = 0x28CD4;
    16
}
/// $048620: lea.l $48626(pc), a1
pub fn i_048620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48624;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48626;
    16
}
/// $048624: move.l a1, (a6)
pub fn i_048624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48626;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048626: jsr $48fb0(pc)
pub fn i_048626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4862A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4862A);
    cpu.pc = 0x48FB0;
    16
}
/// $04862A: jsr $28d70.l
pub fn i_04862a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48630;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48630);
    cpu.pc = 0x28D70;
    16
}
/// $048630: bcc.w $48668
pub fn i_048630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48634;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48668; return 10; }
    8
}
/// $048634: cmpi.w #$0, $72(a6)
pub fn i_048634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4863A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04863A: bgt.w $48668
pub fn i_04863a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4863E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x48668; return 10; }
    8
}
/// $04863E: lea.l $485fc(pc), a1
pub fn i_04863e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48642;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x485FC;
    16
}
/// $048642: move.l a1, (a6)
pub fn i_048642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48644;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048644: subq.b #$1, $82(a6)
pub fn i_048644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48648;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $048648: cmpi.b #$0, $82(a6)
pub fn i_048648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4864E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $04864E: bgt.w $48668
pub fn i_04864e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48652;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x48668; return 10; }
    8
}
/// $048652: lea.l $4857c(pc), a1
pub fn i_048652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48656;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4857C;
    16
}
/// $048656: move.l a1, (a6)
pub fn i_048656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48658;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048658: lea.l $2bfc32.l, a0
pub fn i_048658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4865E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFC32;
    20
}
/// $04865E: jsr $799de.l
pub fn i_04865e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48664;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48664);
    cpu.pc = 0x799DE;
    16
}
/// $048664: move.w d0, $72(a6)
pub fn i_048664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48668;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048668: jsr $4932c(pc)
pub fn i_048668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4866C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4866C);
    cpu.pc = 0x4932C;
    16
}
/// $04866C: bcc.w $48676
pub fn i_04866c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48670;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48676; return 10; }
    8
}
/// $048670: lea.l $48420(pc), a1
pub fn i_048670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48674;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $048674: move.l a1, (a6)
pub fn i_048674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48676;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048676: bra.w $48a44
pub fn i_048676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4867A;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A44;
    10
}
// ===== Pow_FreeLeave_04867a  ($04867A, 150 B, pow/prisoner_0478fc.s)
/// $04867A: clr.w $28(a6)
pub fn i_04867a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4867E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04867E: move.l #$28df42, $48(a6)
pub fn i_04867e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48686;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DF42;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048686: lea.l $28e6ee.l, a0
pub fn i_048686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4868C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6EE;
    20
}
/// $04868C: jsr $28cd4.l
pub fn i_04868c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48692;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48692);
    cpu.pc = 0x28CD4;
    16
}
/// $048692: lea.l $48698(pc), a1
pub fn i_048692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48696;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48698;
    16
}
/// $048696: move.l a1, (a6)
pub fn i_048696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48698;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048698: jsr $48fb0(pc)
pub fn i_048698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4869C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4869C);
    cpu.pc = 0x48FB0;
    16
}
/// $04869C: jsr $28d70.l
pub fn i_04869c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x486A2);
    cpu.pc = 0x28D70;
    16
}
/// $0486A2: bcc.w $486c2
pub fn i_0486a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486A6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x486C2; return 10; }
    8
}
/// $0486A6: jsr $58fe2.l
pub fn i_0486a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x486AC);
    cpu.pc = 0x58FE2;
    16
}
/// $0486AC: lea.l $48ca4(pc), a1
pub fn i_0486ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48CA4;
    16
}
/// $0486B0: jsr $4ae.l
pub fn i_0486b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x486B6);
    cpu.pc = 0x4AE;
    16
}
/// $0486B6: jsr $5dd02.l
pub fn i_0486b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x486BC);
    cpu.pc = 0x5DD02;
    16
}
/// $0486BC: addi.w #$18, $24(a0)
pub fn i_0486bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486C2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0486C2: jsr $4932c(pc)
pub fn i_0486c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x486C6);
    cpu.pc = 0x4932C;
    16
}
/// $0486C6: bcc.w $486d0
pub fn i_0486c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x486D0; return 10; }
    8
}
/// $0486CA: lea.l $48420(pc), a1
pub fn i_0486ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $0486CE: move.l a1, (a6)
pub fn i_0486ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0486D0: bra.w $48a44
pub fn i_0486d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A44;
    10
}
/// $0486D4: clr.w $28(a6)
pub fn i_0486d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486D8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0486D8: lea.l $28e4ee.l, a0
pub fn i_0486d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E4EE;
    20
}
/// $0486DE: jsr $28cd4.l
pub fn i_0486de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x486E4);
    cpu.pc = 0x28CD4;
    16
}
/// $0486E4: lea.l $486ea(pc), a1
pub fn i_0486e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x486EA;
    16
}
/// $0486E8: move.l a1, (a6)
pub fn i_0486e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0486EA: jsr $48fb0(pc)
pub fn i_0486ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x486EE);
    cpu.pc = 0x48FB0;
    16
}
/// $0486EE: jsr $28d70.l
pub fn i_0486ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x486F4);
    cpu.pc = 0x28D70;
    16
}
/// $0486F4: bcc.w $486fe
pub fn i_0486f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x486FE; return 10; }
    8
}
/// $0486F8: lea.l $48710(pc), a1
pub fn i_0486f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48710;
    16
}
/// $0486FC: move.l a1, (a6)
pub fn i_0486fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x486FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0486FE: jsr $4932c(pc)
pub fn i_0486fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48702;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48702);
    cpu.pc = 0x4932C;
    16
}
/// $048702: bcc.w $4870c
pub fn i_048702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48706;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4870C; return 10; }
    8
}
/// $048706: lea.l $48420(pc), a1
pub fn i_048706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4870A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $04870A: move.l a1, (a6)
pub fn i_04870a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4870C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04870C: bra.w $489f8
pub fn i_04870c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48710;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeStand_048710  ($048710, 88 B, pow/prisoner_0478fc.s)
/// $048710: clr.w $28(a6)
pub fn i_048710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48714;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $048714: move.l #$28dfea, $48(a6)
pub fn i_048714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4871C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DFEA;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04871C: lea.l $28e536.l, a0
pub fn i_04871c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48722;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E536;
    20
}
/// $048722: jsr $28cd4.l
pub fn i_048722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48728;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48728);
    cpu.pc = 0x28CD4;
    16
}
/// $048728: lea.l $4872e(pc), a1
pub fn i_048728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4872C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4872E;
    16
}
/// $04872C: move.l a1, (a6)
pub fn i_04872c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4872E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04872E: jsr $48fb0(pc)
pub fn i_04872e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48732;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48732);
    cpu.pc = 0x48FB0;
    16
}
/// $048732: jsr $28d70.l
pub fn i_048732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48738;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48738);
    cpu.pc = 0x28D70;
    16
}
/// $048738: jsr $492a4(pc)
pub fn i_048738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4873C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4873C);
    cpu.pc = 0x492A4;
    16
}
/// $04873C: bcc.w $48746
pub fn i_04873c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48740;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48746; return 10; }
    8
}
/// $048740: lea.l $4883e(pc), a1
pub fn i_048740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48744;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4883E;
    16
}
/// $048744: move.l a1, (a6)
pub fn i_048744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48746;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048746: cmpi.w #$0, $72(a6)
pub fn i_048746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4874C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04874C: bgt.w $48756
pub fn i_04874c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48750;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x48756; return 10; }
    8
}
/// $048750: lea.l $48768(pc), a1
pub fn i_048750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48754;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48768;
    16
}
/// $048754: move.l a1, (a6)
pub fn i_048754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48756;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048756: jsr $4932c(pc)
pub fn i_048756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4875A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4875A);
    cpu.pc = 0x4932C;
    16
}
/// $04875A: bcc.w $48764
pub fn i_04875a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4875E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48764; return 10; }
    8
}
/// $04875E: lea.l $48420(pc), a1
pub fn i_04875e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48762;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $048762: move.l a1, (a6)
pub fn i_048762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48764;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048764: bra.w $489f8
pub fn i_048764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48768;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeSaluteInit_048768  ($048768, 44 B, pow/prisoner_0478fc.s)
/// $048768: clr.w $28(a6)
pub fn i_048768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4876C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04876C: move.l #$28df42, $48(a6)
pub fn i_04876c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48774;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DF42;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048774: lea.l $2bfd36.l, a0
pub fn i_048774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4877A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFD36;
    20
}
/// $04877A: jsr $799de.l
pub fn i_04877a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48780;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48780);
    cpu.pc = 0x799DE;
    16
}
/// $048780: move.b d0, $82(a6)
pub fn i_048780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48784;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $048784: lea.l $28e5b6.l, a0
pub fn i_048784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4878A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E5B6;
    20
}
/// $04878A: jsr $28cd4.l
pub fn i_04878a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48790);
    cpu.pc = 0x28CD4;
    16
}
/// $048790: bra.w $487a0
pub fn i_048790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48794;
    cpu.ir = 0x6000;
    cpu.pc = 0x487A0;
    10
}
// ===== Pow_FreeSalute_048794  ($048794, 102 B, pow/prisoner_0478fc.s)
/// $048794: lea.l $28e610.l, a0
pub fn i_048794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4879A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E610;
    20
}
/// $04879A: jsr $28cd4.l
pub fn i_04879a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x487A0);
    cpu.pc = 0x28CD4;
    16
}
/// $0487A0: lea.l $487a6(pc), a1
pub fn i_0487a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x487A6;
    16
}
/// $0487A4: move.l a1, (a6)
pub fn i_0487a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0487A6: jsr $48fb0(pc)
pub fn i_0487a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x487AA);
    cpu.pc = 0x48FB0;
    16
}
/// $0487AA: jsr $28d70.l
pub fn i_0487aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x487B0);
    cpu.pc = 0x28D70;
    16
}
/// $0487B0: bcc.w $487e8
pub fn i_0487b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x487E8; return 10; }
    8
}
/// $0487B4: cmpi.w #$0, $72(a6)
pub fn i_0487b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487BA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0487BA: bgt.w $487e8
pub fn i_0487ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487BE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x487E8; return 10; }
    8
}
/// $0487BE: lea.l $48794(pc), a1
pub fn i_0487be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48794;
    16
}
/// $0487C2: move.l a1, (a6)
pub fn i_0487c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0487C4: subq.b #$1, $82(a6)
pub fn i_0487c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487C8;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0487C8: cmpi.b #$0, $82(a6)
pub fn i_0487c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487CE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0487CE: bgt.w $487e8
pub fn i_0487ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487D2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x487E8; return 10; }
    8
}
/// $0487D2: lea.l $487fa(pc), a1
pub fn i_0487d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x487FA;
    16
}
/// $0487D6: move.l a1, (a6)
pub fn i_0487d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0487D8: lea.l $2bfc32.l, a0
pub fn i_0487d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFC32;
    20
}
/// $0487DE: jsr $799de.l
pub fn i_0487de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x487E4);
    cpu.pc = 0x799DE;
    16
}
/// $0487E4: move.w d0, $72(a6)
pub fn i_0487e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487E8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0487E8: jsr $4932c(pc)
pub fn i_0487e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x487EC);
    cpu.pc = 0x4932C;
    16
}
/// $0487EC: bcc.w $487f6
pub fn i_0487ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x487F6; return 10; }
    8
}
/// $0487F0: lea.l $48420(pc), a1
pub fn i_0487f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $0487F4: move.l a1, (a6)
pub fn i_0487f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0487F6: bra.w $489f8
pub fn i_0487f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeIdle_0487fa  ($0487FA, 68 B, pow/prisoner_0478fc.s)
/// $0487FA: clr.w $28(a6)
pub fn i_0487fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x487FE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0487FE: move.l #$28dfea, $48(a6)
pub fn i_0487fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48806;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DFEA;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $048806: lea.l $28e632.l, a0
pub fn i_048806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4880C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E632;
    20
}
/// $04880C: jsr $28cd4.l
pub fn i_04880c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48812;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48812);
    cpu.pc = 0x28CD4;
    16
}
/// $048812: lea.l $48818(pc), a1
pub fn i_048812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48816;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48818;
    16
}
/// $048816: move.l a1, (a6)
pub fn i_048816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48818;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048818: jsr $48fb0(pc)
pub fn i_048818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4881C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4881C);
    cpu.pc = 0x48FB0;
    16
}
/// $04881C: jsr $28d70.l
pub fn i_04881c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48822);
    cpu.pc = 0x28D70;
    16
}
/// $048822: bcc.w $4882c
pub fn i_048822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48826;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4882C; return 10; }
    8
}
/// $048826: lea.l $48710(pc), a1
pub fn i_048826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4882A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48710;
    16
}
/// $04882A: move.l a1, (a6)
pub fn i_04882a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4882C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04882C: jsr $4932c(pc)
pub fn i_04882c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48830;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48830);
    cpu.pc = 0x4932C;
    16
}
/// $048830: bcc.w $4883a
pub fn i_048830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48834;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4883A; return 10; }
    8
}
/// $048834: lea.l $48420(pc), a1
pub fn i_048834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48838;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $048838: move.l a1, (a6)
pub fn i_048838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4883A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04883A: bra.w $489f8
pub fn i_04883a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4883E;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_FreeExit_04883e  ($04883E, 90 B, pow/prisoner_0478fc.s)
/// $04883E: clr.w $28(a6)
pub fn i_04883e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48842;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $048842: move.l #$28df42, $48(a6)
pub fn i_048842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4884A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DF42;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04884A: lea.l $28e578.l, a0
pub fn i_04884a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48850;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E578;
    20
}
/// $048850: jsr $28cd4.l
pub fn i_048850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48856;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48856);
    cpu.pc = 0x28CD4;
    16
}
/// $048856: lea.l $4885c(pc), a1
pub fn i_048856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4885A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4885C;
    16
}
/// $04885A: move.l a1, (a6)
pub fn i_04885a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4885C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04885C: jsr $48fb0(pc)
pub fn i_04885c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48860;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48860);
    cpu.pc = 0x48FB0;
    16
}
/// $048860: jsr $28d70.l
pub fn i_048860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48866;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48866);
    cpu.pc = 0x28D70;
    16
}
/// $048866: bcc.w $48886
pub fn i_048866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4886A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48886; return 10; }
    8
}
/// $04886A: jsr $58fe2.l
pub fn i_04886a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48870;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48870);
    cpu.pc = 0x58FE2;
    16
}
/// $048870: lea.l $48ca4(pc), a1
pub fn i_048870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48874;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48CA4;
    16
}
/// $048874: jsr $4ae.l
pub fn i_048874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4887A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4887A);
    cpu.pc = 0x4AE;
    16
}
/// $04887A: jsr $5dd02.l
pub fn i_04887a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48880;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48880);
    cpu.pc = 0x5DD02;
    16
}
/// $048880: addi.w #$18, $24(a0)
pub fn i_048880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48886;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $048886: jsr $4932c(pc)
pub fn i_048886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4888A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4888A);
    cpu.pc = 0x4932C;
    16
}
/// $04888A: bcc.w $48894
pub fn i_04888a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4888E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48894; return 10; }
    8
}
/// $04888E: lea.l $48420(pc), a1
pub fn i_04888e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48892;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48420;
    16
}
/// $048892: move.l a1, (a6)
pub fn i_048892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48894;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048894: bra.w $489f8
pub fn i_048894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48898;
    cpu.ir = 0x6000;
    cpu.pc = 0x489F8;
    10
}
// ===== Pow_SpawnTiedVariant_048898  ($048898, 130 B, pow/prisoner_0478fc.s)
/// $048898: clr.b $80(a6)
pub fn i_048898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4889C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04889C: bra.w $488a6
pub fn i_04889c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x488A6;
    10
}
/// $0488A0: move.b #$1, $80(a6)
pub fn i_0488a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488A6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0488A6: move.w #$38, d1
pub fn i_0488a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0488AA: jsr $236e.l
pub fn i_0488aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x488B0);
    cpu.pc = 0x236E;
    16
}
/// $0488B0: move.w #$1, $66(a6)
pub fn i_0488b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0488B6: move.w #$8000, d0
pub fn i_0488b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488BA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0488BA: jsr $28134.l
pub fn i_0488ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x488C0);
    cpu.pc = 0x28134;
    16
}
/// $0488C0: andi.w #$ffe3, $38(a6)
pub fn i_0488c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488C6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0488C6: ori.w #$18, $38(a6)
pub fn i_0488c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488CC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0488CC: move.l #$28df96, $48(a6)
pub fn i_0488cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488D4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28DF96;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0488D4: move.b $9e(a6), d0
pub fn i_0488d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488D8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0488D8: ext.w d0
pub fn i_0488d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488DA;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0488DA: lsl.w #$4, d0
pub fn i_0488da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488DC;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0488DC: move.w d0, $8c(a6)
pub fn i_0488dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488E0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0488E0: lea.l $2bff8e.l, a0
pub fn i_0488e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488E6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFF8E;
    20
}
/// $0488E6: jsr $799de.l
pub fn i_0488e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x488EC);
    cpu.pc = 0x799DE;
    16
}
/// $0488EC: move.w d0, $8e(a6)
pub fn i_0488ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488F0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0488F0: jsr $5e0d4.l
pub fn i_0488f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x488F6);
    cpu.pc = 0x5E0D4;
    16
}
/// $0488F6: move.l a0, $94(a6)
pub fn i_0488f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488FA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0488FA: clr.w $78(a6)
pub fn i_0488fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x488FE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0488FE: move.b #$8, $7e(a6)
pub fn i_0488fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48904;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $048904: move.b #$f0, $7d(a6)
pub fn i_048904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4890A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04890A: lea.l $48d0e(pc), a1
pub fn i_04890a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4890E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48D0E;
    16
}
/// $04890E: jsr $4ae.l
pub fn i_04890e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48914;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48914);
    cpu.pc = 0x4AE;
    16
}
/// $048914: jsr $5dd02.l
pub fn i_048914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4891A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4891A);
    cpu.pc = 0x5DD02;
    16
}
// ===== Pow_TiedIdle_04891a  ($04891A, 84 B, pow/prisoner_0478fc.s)
/// $04891A: lea.l $2bff0c.l, a0
pub fn i_04891a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48920;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BFF0C;
    20
}
/// $048920: jsr $799de.l
pub fn i_048920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48926;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48926);
    cpu.pc = 0x799DE;
    16
}
/// $048926: move.w d0, $72(a6)
pub fn i_048926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4892A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04892A: move.b #$0, $83(a6)
pub fn i_04892a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48930;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $048930: lea.l $28ed0a.l, a0
pub fn i_048930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48936;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28ED0A;
    20
}
/// $048936: jsr $28cd4.l
pub fn i_048936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4893C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4893C);
    cpu.pc = 0x28CD4;
    16
}
/// $04893C: lea.l $48942(pc), a1
pub fn i_04893c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48940;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48942;
    16
}
/// $048940: move.l a1, (a6)
pub fn i_048940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48942;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048942: jsr $48f54(pc)
pub fn i_048942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48946;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48946);
    cpu.pc = 0x48F54;
    16
}
/// $048946: bcc.w $48950
pub fn i_048946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4894A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48950; return 10; }
    8
}
/// $04894A: lea.l $489c6(pc), a1
pub fn i_04894a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4894E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x489C6;
    16
}
/// $04894E: move.l a1, (a6)
pub fn i_04894e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48950;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048950: jsr $28d70.l
pub fn i_048950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48956;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48956);
    cpu.pc = 0x28D70;
    16
}
/// $048956: jsr $49054(pc)
pub fn i_048956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4895A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4895A);
    cpu.pc = 0x49054;
    16
}
/// $04895A: bcc.w $4896a
pub fn i_04895a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4895E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4896A; return 10; }
    8
}
/// $04895E: move.b #$1, $83(a6)
pub fn i_04895e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48964;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $048964: lea.l $4896e(pc), a1
pub fn i_048964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48968;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4896E;
    16
}
/// $048968: move.l a1, (a6)
pub fn i_048968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4896A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04896A: bra.w $48a90
pub fn i_04896a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4896E;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A90;
    10
}
// ===== Pow_TiedStruggle_04896e  ($04896E, 88 B, pow/prisoner_0478fc.s)
/// $04896E: move.b #$1, $83(a6)
pub fn i_04896e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48974;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $048974: jsr $493e4(pc)
pub fn i_048974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48978;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48978);
    cpu.pc = 0x493E4;
    16
}
/// $048978: move.w $78(a6), d0
pub fn i_048978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4897C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04897C: lsr.w #$1, d0
pub fn i_04897c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4897E;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $04897E: andi.w #$f, d0
pub fn i_04897e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48982;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $048982: movea.l #$28e24a, a0
pub fn i_048982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48988;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28E24A;
    cpu.a[0] = v1;
    4
}
/// $048988: lsl.w #$2, d0
pub fn i_048988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4898A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04898A: movea.l (a0, d0.w), a0
pub fn i_04898a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4898E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04898E: cmpa.l #$ffffffff, a0
pub fn i_04898e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48994;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $048994: beq.w $4899e
pub fn i_048994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48998;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4899E; return 10; }
    8
}
/// $048998: jsr $28cd4.l
pub fn i_048998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4899E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4899E);
    cpu.pc = 0x28CD4;
    16
}
/// $04899E: lea.l $489a4(pc), a1
pub fn i_04899e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x489A4;
    16
}
/// $0489A2: move.l a1, (a6)
pub fn i_0489a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0489A4: jsr $48f54(pc)
pub fn i_0489a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x489A8);
    cpu.pc = 0x48F54;
    16
}
/// $0489A8: bcc.w $489b2
pub fn i_0489a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x489B2; return 10; }
    8
}
/// $0489AC: lea.l $489c6(pc), a1
pub fn i_0489ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x489C6;
    16
}
/// $0489B0: move.l a1, (a6)
pub fn i_0489b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0489B2: jsr $28d70.l
pub fn i_0489b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x489B8);
    cpu.pc = 0x28D70;
    16
}
/// $0489B8: bcc.w $489c2
pub fn i_0489b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x489C2; return 10; }
    8
}
/// $0489BC: lea.l $4891a(pc), a1
pub fn i_0489bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4891A;
    16
}
/// $0489C0: move.l a1, (a6)
pub fn i_0489c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0489C2: bra.w $48a90
pub fn i_0489c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A90;
    10
}
// ===== Pow_TiedFreed_0489c6  ($0489C6, 118 B, pow/prisoner_0478fc.s)
/// $0489C6: move.b #$3, $83(a6)
pub fn i_0489c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0489CC: lea.l $28ef0c.l, a0
pub fn i_0489cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28EF0C;
    20
}
/// $0489D2: jsr $28cd4.l
pub fn i_0489d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x489D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0489D8: lea.l $489de(pc), a1
pub fn i_0489d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x489DE;
    16
}
/// $0489DC: move.l a1, (a6)
pub fn i_0489dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0489DE: jsr $2783a.l
pub fn i_0489de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x489E4);
    cpu.pc = 0x2783A;
    16
}
/// $0489E4: jsr $28d70.l
pub fn i_0489e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x489EA);
    cpu.pc = 0x28D70;
    16
}
/// $0489EA: bcc.w $489f4
pub fn i_0489ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x489F4; return 10; }
    8
}
/// $0489EE: lea.l $4797c(pc), a1
pub fn i_0489ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4797C;
    16
}
/// $0489F2: move.l a1, (a6)
pub fn i_0489f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0489F4: bra.w $48a90
pub fn i_0489f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x48A90;
    10
}
/// $0489F8: jsr $4936e(pc)
pub fn i_0489f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x489FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x489FC);
    cpu.pc = 0x4936E;
    16
}
/// $0489FC: subq.w #$1, $72(a6)
pub fn i_0489fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A00;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $048A00: jsr $2870a.l
pub fn i_048a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A06);
    cpu.pc = 0x2870A;
    16
}
/// $048A06: bcc.w $48a26
pub fn i_048a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48A26; return 10; }
    8
}
/// $048A0A: lea.l $48ca4(pc), a1
pub fn i_048a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48CA4;
    16
}
/// $048A0E: jsr $4ae.l
pub fn i_048a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A14);
    cpu.pc = 0x4AE;
    16
}
/// $048A14: jsr $5dd02.l
pub fn i_048a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A1A);
    cpu.pc = 0x5DD02;
    16
}
/// $048A1A: addi.w #$18, $24(a0)
pub fn i_048a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A20;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $048A20: jsr $49fd0.l
pub fn i_048a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A26);
    cpu.pc = 0x49FD0;
    16
}
/// $048A26: movea.l #$ffffffff, a0
pub fn i_048a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A2C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $048A2C: lea.l $28e196.l, a0
pub fn i_048a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E196;
    20
}
/// $048A32: jsr $5dd5c.l
pub fn i_048a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A38);
    cpu.pc = 0x5DD5C;
    16
}
/// $048A38: bcc.w $48a42
pub fn i_048a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48A42; return 10; }
    8
}
// ===== SetTaskHandler_048a3c  ($048A3C, 8 B, kernel/task_handlers.c)
/// $048A3C: lea.l $48b1e(pc), a1
pub fn i_048a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B1E;
    16
}
/// $048A40: move.l a1, (a6)
pub fn i_048a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048A42: rts
pub fn i_048a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_FreeStateTail_048a44  ($048A44, 68 B, pow/helpers_048a44.s)
/// $048A44: jsr $4936e(pc)
pub fn i_048a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48A48);
    cpu.pc = 0x4936E;
    16
}
/// $048A48: subq.w #$1, $72(a6)
pub fn i_048a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A4C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $048A4C: jsr $2870a.l
pub fn i_048a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A52);
    cpu.pc = 0x2870A;
    16
}
/// $048A52: bcc.w $48a72
pub fn i_048a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48A72; return 10; }
    8
}
/// $048A56: lea.l $48ca4(pc), a1
pub fn i_048a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48CA4;
    16
}
/// $048A5A: jsr $4ae.l
pub fn i_048a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A60);
    cpu.pc = 0x4AE;
    16
}
/// $048A60: jsr $5dd02.l
pub fn i_048a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A66);
    cpu.pc = 0x5DD02;
    16
}
/// $048A66: addi.w #$10, $24(a0)
pub fn i_048a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A6C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $048A6C: jsr $49ff2.l
pub fn i_048a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A72);
    cpu.pc = 0x49FF2;
    16
}
/// $048A72: movea.l #$ffffffff, a0
pub fn i_048a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A78;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $048A78: lea.l $28e196.l, a0
pub fn i_048a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E196;
    20
}
/// $048A7E: jsr $5dd5c.l
pub fn i_048a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A84);
    cpu.pc = 0x5DD5C;
    16
}
/// $048A84: bcc.w $48a8e
pub fn i_048a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48A8E; return 10; }
    8
}
// ===== SetTaskHandler_048a88  ($048A88, 8 B, kernel/task_handlers.c)
/// $048A88: lea.l $48b1e(pc), a1
pub fn i_048a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B1E;
    16
}
/// $048A8C: move.l a1, (a6)
pub fn i_048a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $048A8E: rts
pub fn i_048a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_TiedStateTail_048a90  ($048A90, 134 B, pow/helpers_048a44.s)
/// $048A90: jsr $4936e(pc)
pub fn i_048a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x48A94);
    cpu.pc = 0x4936E;
    16
}
/// $048A94: subq.w #$1, $72(a6)
pub fn i_048a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A98;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $048A98: jsr $2870a.l
pub fn i_048a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48A9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48A9E);
    cpu.pc = 0x2870A;
    16
}
/// $048A9E: bcc.w $48ad2
pub fn i_048a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AA2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x48AD2; return 10; }
    8
}
/// $048AA2: lea.l $48ca4(pc), a1
pub fn i_048aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48CA4;
    16
}
/// $048AA6: jsr $4ae.l
pub fn i_048aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48AAC);
    cpu.pc = 0x4AE;
    16
}
/// $048AAC: jsr $5dd02.l
pub fn i_048aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48AB2);
    cpu.pc = 0x5DD02;
    16
}
/// $048AB2: addi.w #$18, $24(a0)
pub fn i_048ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AB8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $048AB8: movea.l $50(a6), a1
pub fn i_048ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48ABC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $048ABC: move.w $28(a1), d0
pub fn i_048abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AC0;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $048AC0: asr.w #$4, d0
pub fn i_048ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AC2;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $048AC2: move.w d0, $28(a0)
pub fn i_048ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AC6;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $048AC6: jsr $49fd0.l
pub fn i_048ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48ACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x48ACC);
    cpu.pc = 0x49FD0;
    16
}
/// $048ACC: move.b #$2, $83(a6)
pub fn i_048acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AD2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $048AD2: cmpi.b #$0, $9e(a6)
pub fn i_048ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AD8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $048AD8: bge.w $48af0
pub fn i_048ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48ADC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x48AF0; return 10; }
    8
}
/// $048ADC: cmpi.w #$ffe0, $22(a6)
pub fn i_048adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AE2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFE0, Size::Word, false);
    16
}
/// $048AE2: bgt.w $48aec
pub fn i_048ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AE6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x48AEC; return 10; }
    8
}
/// $048AE6: lea.l $48b1e(pc), a1
pub fn i_048ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48AEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x48B1E;
    16
}
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
/// $048B24: rts
pub fn i_048b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
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
/// $048B32: rts
pub fn i_048b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48B34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
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
/// $048BC4: move.b $98(a6), $7b(a6)
pub fn i_048bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x48BCA;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad4, v3 as u8);
    20
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
/// $0493FA: move.b #$8, $7b(a6)
pub fn i_0493fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49400;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049400: btst.b #$0, $3a(a6)
pub fn i_049400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49406;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $049406: beq.w $4940e
pub fn i_049406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4940A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4940E; return 10; }
    8
}
/// $04940A: clr.b $7b(a6)
pub fn i_04940a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4940E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
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
/// $049554: move.b #$1, $7f(a6)
pub fn i_049554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4955A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    bus.write8(ad2, v1 as u8);
    12
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
// ===== PowHang_Struggle_04968a  ($04968A, 184 B, pow/hang_049430.s)
/// $04968A: clr.l d0
pub fn i_04968a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4968C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $04968C: move.b $71(a6), d0
pub fn i_04968c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49690;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $049690: movea.l #$28f838, a0
pub fn i_049690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49696;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28F838;
    cpu.a[0] = v1;
    4
}
/// $049696: lsl.w #$2, d0
pub fn i_049696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49698;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $049698: movea.l (a0, d0.w), a0
pub fn i_049698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4969C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04969C: cmpa.l #$ffffffff, a0
pub fn i_04969c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496A2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0496A2: beq.w $496ac
pub fn i_0496a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x496AC; return 10; }
    8
}
/// $0496A6: jsr $28cd4.l
pub fn i_0496a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x496AC);
    cpu.pc = 0x28CD4;
    16
}
/// $0496AC: move.b #$1, $78(a6)
pub fn i_0496ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496B2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0496B2: lea.l $496b8(pc), a1
pub fn i_0496b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x496B8;
    16
}
/// $0496B6: move.l a1, (a6)
pub fn i_0496b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0496B8: jsr $499dc(pc)
pub fn i_0496b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x496BC);
    cpu.pc = 0x499DC;
    16
}
/// $0496BC: jsr $28d70.l
pub fn i_0496bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x496C2);
    cpu.pc = 0x28D70;
    16
}
/// $0496C2: bcc.w $496cc
pub fn i_0496c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x496CC; return 10; }
    8
}
/// $0496C6: lea.l $495d6(pc), a1
pub fn i_0496c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x495D6;
    16
}
/// $0496CA: move.l a1, (a6)
pub fn i_0496ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0496CC: jsr $49b58(pc)
pub fn i_0496cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x496D0);
    cpu.pc = 0x49B58;
    16
}
/// $0496D0: bcc.w $496de
pub fn i_0496d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x496DE; return 10; }
    8
}
/// $0496D4: move.b #$fd, $78(a6)
pub fn i_0496d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFD;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0496DA: bra.w $496e8
pub fn i_0496da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x496E8;
    10
}
/// $0496DE: cmpi.b #$ff, $79(a6)
pub fn i_0496de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496E4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0496E4: bne.w $49708
pub fn i_0496e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49708; return 10; }
    8
}
/// $0496E8: move.w #$c, d0
pub fn i_0496e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496EC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0496EC: sub.w $34(a6), d0
pub fn i_0496ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496F0;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0496F0: asr.w #$8, d0
pub fn i_0496f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496F2;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0496F2: asr.w #$7, d0
pub fn i_0496f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496F4;
    cpu.ir = 0xEE40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $0496F4: andi.b #$1, d0
pub fn i_0496f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496F8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0496F8: move.b d0, $3a(a6)
pub fn i_0496f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x496FC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0496FC: lea.l $4797c.l, a1
pub fn i_0496fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49702;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4797C;
    20
}
/// $049702: move.l a1, (a6)
pub fn i_049702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49704;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049704: bra.w $49740
pub fn i_049704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49708;
    cpu.ir = 0x6000;
    cpu.pc = 0x49740;
    10
}
/// $049708: jsr $2870a.l
pub fn i_049708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4970E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4970E);
    cpu.pc = 0x2870A;
    16
}
/// $04970E: bcc.w $4971e
pub fn i_04970e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49712;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4971E; return 10; }
    8
}
/// $049712: move.b #$ff, $78(a6)
pub fn i_049712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49718;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $049718: lea.l $49742(pc), a1
pub fn i_049718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4971C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49742;
    16
}
/// $04971C: move.l a1, (a6)
pub fn i_04971c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4971E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04971E: movea.l #$ffffffff, a0
pub fn i_04971e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49724;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $049724: lea.l $28ffd2.l, a0
pub fn i_049724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4972A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28FFD2;
    20
}
/// $04972A: jsr $5dd5c.l
pub fn i_04972a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49730;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x49730);
    cpu.pc = 0x5DD5C;
    16
}
/// $049730: bcc.w $49740
pub fn i_049730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49734;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x49740; return 10; }
    8
}
/// $049734: move.b #$fe, $78(a6)
pub fn i_049734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4973A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04973A: jmp $518.l
pub fn i_04973a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49740;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $049740: rts
pub fn i_049740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49742;
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
// ===== PowHang_RopeStruggle_049884  ($049884, 170 B, pow/hang_049430.s)
/// $049884: clr.l d0
pub fn i_049884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49886;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $049886: movea.l $c(a6), a0
pub fn i_049886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4988A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04988A: move.b $71(a0), d0
pub fn i_04988a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4988E;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04988E: movea.l #$28fdbc, a0
pub fn i_04988e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49894;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28FDBC;
    cpu.a[0] = v1;
    4
}
/// $049894: lsl.w #$2, d0
pub fn i_049894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49896;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $049896: movea.l (a0, d0.w), a0
pub fn i_049896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4989A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04989A: cmpa.l #$ffffffff, a0
pub fn i_04989a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498A0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0498A0: beq.w $498aa
pub fn i_0498a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x498AA; return 10; }
    8
}
/// $0498A4: jsr $28cd4.l
pub fn i_0498a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x498AA);
    cpu.pc = 0x28CD4;
    16
}
/// $0498AA: lea.l $498b0(pc), a1
pub fn i_0498aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x498B0;
    16
}
/// $0498AE: move.l a1, (a6)
pub fn i_0498ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0498B0: jsr $5e4ee.l
pub fn i_0498b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x498B6);
    cpu.pc = 0x5E4EE;
    16
}
/// $0498B6: subq.w #$1, $38(a6)
pub fn i_0498b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498BA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0498BA: jsr $28d70.l
pub fn i_0498ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x498C0);
    cpu.pc = 0x28D70;
    16
}
/// $0498C0: bcc.w $498ca
pub fn i_0498c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x498CA; return 10; }
    8
}
/// $0498C4: lea.l $497d0(pc), a1
pub fn i_0498c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x497D0;
    16
}
/// $0498C8: move.l a1, (a6)
pub fn i_0498c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0498CA: jsr $28758.l
pub fn i_0498ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x498D0);
    cpu.pc = 0x28758;
    16
}
/// $0498D0: bcc.w $498e8
pub fn i_0498d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x498E8; return 10; }
    8
}
/// $0498D4: movea.l $c(a6), a0
pub fn i_0498d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498D8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0498D8: move.b #$ff, $79(a0)
pub fn i_0498d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498DE;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0498DE: lea.l $49976(pc), a1
pub fn i_0498de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49976;
    16
}
/// $0498E2: move.l a1, (a6)
pub fn i_0498e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0498E4: bra.w $4992c
pub fn i_0498e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x4992C;
    10
}
/// $0498E8: movea.l $c(a6), a0
pub fn i_0498e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498EC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0498EC: cmpi.b #$fd, $78(a0)
pub fn i_0498ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498F2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFD, Size::Byte, false);
    16
}
/// $0498F2: bne.w $49904
pub fn i_0498f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x49904; return 10; }
    8
}
/// $0498F6: move.l a6, $c(a6)
pub fn i_0498f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498FA;
    cpu.ir = 0x2D4E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0498FA: lea.l $4992e(pc), a1
pub fn i_0498fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x498FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4992E;
    16
}
/// $0498FE: move.l a1, (a6)
pub fn i_0498fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49900;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049900: bra.w $4992c
pub fn i_049900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49904;
    cpu.ir = 0x6000;
    cpu.pc = 0x4992C;
    10
}
/// $049904: cmpi.b #$ff, $78(a0)
pub fn i_049904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4990A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04990A: bne.w $4991c
pub fn i_04990a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4990E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4991C; return 10; }
    8
}
/// $04990E: move.l a6, $c(a6)
pub fn i_04990e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49912;
    cpu.ir = 0x2D4E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $049912: lea.l $49976(pc), a1
pub fn i_049912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49916;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x49976;
    16
}
/// $049916: move.l a1, (a6)
pub fn i_049916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49918;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $049918: bra.w $4992c
pub fn i_049918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4991C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4992C;
    10
}
/// $04991C: cmpi.b #$fe, $78(a0)
pub fn i_04991c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49922;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFE, Size::Byte, false);
    16
}
/// $049922: bne.w $4992c
pub fn i_049922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x49926;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4992C; return 10; }
    8
}
/// $049926: jmp $518.l
pub fn i_049926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4992C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04992C: rts
pub fn i_04992c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4992E;
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

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0483F8, i_0483f8),
    (0x0483FC, i_0483fc),
    (0x048400, i_048400),
    (0x048404, i_048404),
    (0x048408, i_048408),
    (0x04840C, i_04840c),
    (0x048410, i_048410),
    (0x048414, i_048414),
    (0x048418, i_048418),
    (0x04841C, i_04841c),
    (0x048420, i_048420),
    (0x048424, i_048424),
    (0x048426, i_048426),
    (0x04842A, i_04842a),
    (0x048430, i_048430),
    (0x048436, i_048436),
    (0x04843A, i_04843a),
    (0x04843C, i_04843c),
    (0x048442, i_048442),
    (0x048446, i_048446),
    (0x04844A, i_04844a),
    (0x04844C, i_04844c),
    (0x048452, i_048452),
    (0x048456, i_048456),
    (0x04845A, i_04845a),
    (0x048460, i_048460),
    (0x048466, i_048466),
    (0x04846A, i_04846a),
    (0x04846C, i_04846c),
    (0x048470, i_048470),
    (0x048476, i_048476),
    (0x04847A, i_04847a),
    (0x04847E, i_04847e),
    (0x048480, i_048480),
    (0x048484, i_048484),
    (0x048488, i_048488),
    (0x04848C, i_04848c),
    (0x04848E, i_04848e),
    (0x048492, i_048492),
    (0x048498, i_048498),
    (0x04849E, i_04849e),
    (0x0484A2, i_0484a2),
    (0x0484A6, i_0484a6),
    (0x0484AC, i_0484ac),
    (0x0484B2, i_0484b2),
    (0x0484B6, i_0484b6),
    (0x0484B8, i_0484b8),
    (0x0484BC, i_0484bc),
    (0x0484C2, i_0484c2),
    (0x0484C6, i_0484c6),
    (0x0484CA, i_0484ca),
    (0x0484CC, i_0484cc),
    (0x0484D2, i_0484d2),
    (0x0484D6, i_0484d6),
    (0x0484DA, i_0484da),
    (0x0484DE, i_0484de),
    (0x0484E2, i_0484e2),
    (0x0484E6, i_0484e6),
    (0x0484EA, i_0484ea),
    (0x0484EE, i_0484ee),
    (0x0484F0, i_0484f0),
    (0x0484F4, i_0484f4),
    (0x0484F8, i_0484f8),
    (0x0484FC, i_0484fc),
    (0x0484FE, i_0484fe),
    (0x048502, i_048502),
    (0x048506, i_048506),
    (0x04850A, i_04850a),
    (0x04850E, i_04850e),
    (0x048514, i_048514),
    (0x048518, i_048518),
    (0x04851E, i_04851e),
    (0x048524, i_048524),
    (0x04852A, i_04852a),
    (0x048530, i_048530),
    (0x048536, i_048536),
    (0x04853A, i_04853a),
    (0x04853C, i_04853c),
    (0x048542, i_048542),
    (0x048546, i_048546),
    (0x04854A, i_04854a),
    (0x048550, i_048550),
    (0x048554, i_048554),
    (0x048558, i_048558),
    (0x04855C, i_04855c),
    (0x048560, i_048560),
    (0x048562, i_048562),
    (0x048566, i_048566),
    (0x04856A, i_04856a),
    (0x04856E, i_04856e),
    (0x048572, i_048572),
    (0x048576, i_048576),
    (0x048578, i_048578),
    (0x04857C, i_04857c),
    (0x048580, i_048580),
    (0x048588, i_048588),
    (0x04858E, i_04858e),
    (0x048594, i_048594),
    (0x048598, i_048598),
    (0x04859A, i_04859a),
    (0x04859E, i_04859e),
    (0x0485A4, i_0485a4),
    (0x0485A8, i_0485a8),
    (0x0485AC, i_0485ac),
    (0x0485B0, i_0485b0),
    (0x0485B2, i_0485b2),
    (0x0485B8, i_0485b8),
    (0x0485BC, i_0485bc),
    (0x0485C2, i_0485c2),
    (0x0485C6, i_0485c6),
    (0x0485CC, i_0485cc),
    (0x0485D0, i_0485d0),
    (0x0485D4, i_0485d4),
    (0x0485D6, i_0485d6),
    (0x0485DA, i_0485da),
    (0x0485DE, i_0485de),
    (0x0485E2, i_0485e2),
    (0x0485E4, i_0485e4),
    (0x0485E8, i_0485e8),
    (0x0485EC, i_0485ec),
    (0x0485F2, i_0485f2),
    (0x0485F8, i_0485f8),
    (0x0485FC, i_0485fc),
    (0x048602, i_048602),
    (0x048608, i_048608),
    (0x04860C, i_04860c),
    (0x048614, i_048614),
    (0x04861A, i_04861a),
    (0x048620, i_048620),
    (0x048624, i_048624),
    (0x048626, i_048626),
    (0x04862A, i_04862a),
    (0x048630, i_048630),
    (0x048634, i_048634),
    (0x04863A, i_04863a),
    (0x04863E, i_04863e),
    (0x048642, i_048642),
    (0x048644, i_048644),
    (0x048648, i_048648),
    (0x04864E, i_04864e),
    (0x048652, i_048652),
    (0x048656, i_048656),
    (0x048658, i_048658),
    (0x04865E, i_04865e),
    (0x048664, i_048664),
    (0x048668, i_048668),
    (0x04866C, i_04866c),
    (0x048670, i_048670),
    (0x048674, i_048674),
    (0x048676, i_048676),
    (0x04867A, i_04867a),
    (0x04867E, i_04867e),
    (0x048686, i_048686),
    (0x04868C, i_04868c),
    (0x048692, i_048692),
    (0x048696, i_048696),
    (0x048698, i_048698),
    (0x04869C, i_04869c),
    (0x0486A2, i_0486a2),
    (0x0486A6, i_0486a6),
    (0x0486AC, i_0486ac),
    (0x0486B0, i_0486b0),
    (0x0486B6, i_0486b6),
    (0x0486BC, i_0486bc),
    (0x0486C2, i_0486c2),
    (0x0486C6, i_0486c6),
    (0x0486CA, i_0486ca),
    (0x0486CE, i_0486ce),
    (0x0486D0, i_0486d0),
    (0x0486D4, i_0486d4),
    (0x0486D8, i_0486d8),
    (0x0486DE, i_0486de),
    (0x0486E4, i_0486e4),
    (0x0486E8, i_0486e8),
    (0x0486EA, i_0486ea),
    (0x0486EE, i_0486ee),
    (0x0486F4, i_0486f4),
    (0x0486F8, i_0486f8),
    (0x0486FC, i_0486fc),
    (0x0486FE, i_0486fe),
    (0x048702, i_048702),
    (0x048706, i_048706),
    (0x04870A, i_04870a),
    (0x04870C, i_04870c),
    (0x048710, i_048710),
    (0x048714, i_048714),
    (0x04871C, i_04871c),
    (0x048722, i_048722),
    (0x048728, i_048728),
    (0x04872C, i_04872c),
    (0x04872E, i_04872e),
    (0x048732, i_048732),
    (0x048738, i_048738),
    (0x04873C, i_04873c),
    (0x048740, i_048740),
    (0x048744, i_048744),
    (0x048746, i_048746),
    (0x04874C, i_04874c),
    (0x048750, i_048750),
    (0x048754, i_048754),
    (0x048756, i_048756),
    (0x04875A, i_04875a),
    (0x04875E, i_04875e),
    (0x048762, i_048762),
    (0x048764, i_048764),
    (0x048768, i_048768),
    (0x04876C, i_04876c),
    (0x048774, i_048774),
    (0x04877A, i_04877a),
    (0x048780, i_048780),
    (0x048784, i_048784),
    (0x04878A, i_04878a),
    (0x048790, i_048790),
    (0x048794, i_048794),
    (0x04879A, i_04879a),
    (0x0487A0, i_0487a0),
    (0x0487A4, i_0487a4),
    (0x0487A6, i_0487a6),
    (0x0487AA, i_0487aa),
    (0x0487B0, i_0487b0),
    (0x0487B4, i_0487b4),
    (0x0487BA, i_0487ba),
    (0x0487BE, i_0487be),
    (0x0487C2, i_0487c2),
    (0x0487C4, i_0487c4),
    (0x0487C8, i_0487c8),
    (0x0487CE, i_0487ce),
    (0x0487D2, i_0487d2),
    (0x0487D6, i_0487d6),
    (0x0487D8, i_0487d8),
    (0x0487DE, i_0487de),
    (0x0487E4, i_0487e4),
    (0x0487E8, i_0487e8),
    (0x0487EC, i_0487ec),
    (0x0487F0, i_0487f0),
    (0x0487F4, i_0487f4),
    (0x0487F6, i_0487f6),
    (0x0487FA, i_0487fa),
    (0x0487FE, i_0487fe),
    (0x048806, i_048806),
    (0x04880C, i_04880c),
    (0x048812, i_048812),
    (0x048816, i_048816),
    (0x048818, i_048818),
    (0x04881C, i_04881c),
    (0x048822, i_048822),
    (0x048826, i_048826),
    (0x04882A, i_04882a),
    (0x04882C, i_04882c),
    (0x048830, i_048830),
    (0x048834, i_048834),
    (0x048838, i_048838),
    (0x04883A, i_04883a),
    (0x04883E, i_04883e),
    (0x048842, i_048842),
    (0x04884A, i_04884a),
    (0x048850, i_048850),
    (0x048856, i_048856),
    (0x04885A, i_04885a),
    (0x04885C, i_04885c),
    (0x048860, i_048860),
    (0x048866, i_048866),
    (0x04886A, i_04886a),
    (0x048870, i_048870),
    (0x048874, i_048874),
    (0x04887A, i_04887a),
    (0x048880, i_048880),
    (0x048886, i_048886),
    (0x04888A, i_04888a),
    (0x04888E, i_04888e),
    (0x048892, i_048892),
    (0x048894, i_048894),
    (0x048898, i_048898),
    (0x04889C, i_04889c),
    (0x0488A0, i_0488a0),
    (0x0488A6, i_0488a6),
    (0x0488AA, i_0488aa),
    (0x0488B0, i_0488b0),
    (0x0488B6, i_0488b6),
    (0x0488BA, i_0488ba),
    (0x0488C0, i_0488c0),
    (0x0488C6, i_0488c6),
    (0x0488CC, i_0488cc),
    (0x0488D4, i_0488d4),
    (0x0488D8, i_0488d8),
    (0x0488DA, i_0488da),
    (0x0488DC, i_0488dc),
    (0x0488E0, i_0488e0),
    (0x0488E6, i_0488e6),
    (0x0488EC, i_0488ec),
    (0x0488F0, i_0488f0),
    (0x0488F6, i_0488f6),
    (0x0488FA, i_0488fa),
    (0x0488FE, i_0488fe),
    (0x048904, i_048904),
    (0x04890A, i_04890a),
    (0x04890E, i_04890e),
    (0x048914, i_048914),
    (0x04891A, i_04891a),
    (0x048920, i_048920),
    (0x048926, i_048926),
    (0x04892A, i_04892a),
    (0x048930, i_048930),
    (0x048936, i_048936),
    (0x04893C, i_04893c),
    (0x048940, i_048940),
    (0x048942, i_048942),
    (0x048946, i_048946),
    (0x04894A, i_04894a),
    (0x04894E, i_04894e),
    (0x048950, i_048950),
    (0x048956, i_048956),
    (0x04895A, i_04895a),
    (0x04895E, i_04895e),
    (0x048964, i_048964),
    (0x048968, i_048968),
    (0x04896A, i_04896a),
    (0x04896E, i_04896e),
    (0x048974, i_048974),
    (0x048978, i_048978),
    (0x04897C, i_04897c),
    (0x04897E, i_04897e),
    (0x048982, i_048982),
    (0x048988, i_048988),
    (0x04898A, i_04898a),
    (0x04898E, i_04898e),
    (0x048994, i_048994),
    (0x048998, i_048998),
    (0x04899E, i_04899e),
    (0x0489A2, i_0489a2),
    (0x0489A4, i_0489a4),
    (0x0489A8, i_0489a8),
    (0x0489AC, i_0489ac),
    (0x0489B0, i_0489b0),
    (0x0489B2, i_0489b2),
    (0x0489B8, i_0489b8),
    (0x0489BC, i_0489bc),
    (0x0489C0, i_0489c0),
    (0x0489C2, i_0489c2),
    (0x0489C6, i_0489c6),
    (0x0489CC, i_0489cc),
    (0x0489D2, i_0489d2),
    (0x0489D8, i_0489d8),
    (0x0489DC, i_0489dc),
    (0x0489DE, i_0489de),
    (0x0489E4, i_0489e4),
    (0x0489EA, i_0489ea),
    (0x0489EE, i_0489ee),
    (0x0489F2, i_0489f2),
    (0x0489F4, i_0489f4),
    (0x0489F8, i_0489f8),
    (0x0489FC, i_0489fc),
    (0x048A00, i_048a00),
    (0x048A06, i_048a06),
    (0x048A0A, i_048a0a),
    (0x048A0E, i_048a0e),
    (0x048A14, i_048a14),
    (0x048A1A, i_048a1a),
    (0x048A20, i_048a20),
    (0x048A26, i_048a26),
    (0x048A2C, i_048a2c),
    (0x048A32, i_048a32),
    (0x048A38, i_048a38),
    (0x048A3C, i_048a3c),
    (0x048A40, i_048a40),
    (0x048A42, i_048a42),
    (0x048A44, i_048a44),
    (0x048A48, i_048a48),
    (0x048A4C, i_048a4c),
    (0x048A52, i_048a52),
    (0x048A56, i_048a56),
    (0x048A5A, i_048a5a),
    (0x048A60, i_048a60),
    (0x048A66, i_048a66),
    (0x048A6C, i_048a6c),
    (0x048A72, i_048a72),
    (0x048A78, i_048a78),
    (0x048A7E, i_048a7e),
    (0x048A84, i_048a84),
    (0x048A88, i_048a88),
    (0x048A8C, i_048a8c),
    (0x048A8E, i_048a8e),
    (0x048A90, i_048a90),
    (0x048A94, i_048a94),
    (0x048A98, i_048a98),
    (0x048A9E, i_048a9e),
    (0x048AA2, i_048aa2),
    (0x048AA6, i_048aa6),
    (0x048AAC, i_048aac),
    (0x048AB2, i_048ab2),
    (0x048AB8, i_048ab8),
    (0x048ABC, i_048abc),
    (0x048AC0, i_048ac0),
    (0x048AC2, i_048ac2),
    (0x048AC6, i_048ac6),
    (0x048ACC, i_048acc),
    (0x048AD2, i_048ad2),
    (0x048AD8, i_048ad8),
    (0x048ADC, i_048adc),
    (0x048AE2, i_048ae2),
    (0x048AE6, i_048ae6),
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
    (0x048B24, i_048b24),
    (0x048B26, i_048b26),
    (0x048B2C, i_048b2c),
    (0x048B32, i_048b32),
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
    (0x048BC4, i_048bc4),
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
    (0x0493FA, i_0493fa),
    (0x049400, i_049400),
    (0x049406, i_049406),
    (0x04940A, i_04940a),
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
    (0x049554, i_049554),
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
    (0x04968A, i_04968a),
    (0x04968C, i_04968c),
    (0x049690, i_049690),
    (0x049696, i_049696),
    (0x049698, i_049698),
    (0x04969C, i_04969c),
    (0x0496A2, i_0496a2),
    (0x0496A6, i_0496a6),
    (0x0496AC, i_0496ac),
    (0x0496B2, i_0496b2),
    (0x0496B6, i_0496b6),
    (0x0496B8, i_0496b8),
    (0x0496BC, i_0496bc),
    (0x0496C2, i_0496c2),
    (0x0496C6, i_0496c6),
    (0x0496CA, i_0496ca),
    (0x0496CC, i_0496cc),
    (0x0496D0, i_0496d0),
    (0x0496D4, i_0496d4),
    (0x0496DA, i_0496da),
    (0x0496DE, i_0496de),
    (0x0496E4, i_0496e4),
    (0x0496E8, i_0496e8),
    (0x0496EC, i_0496ec),
    (0x0496F0, i_0496f0),
    (0x0496F2, i_0496f2),
    (0x0496F4, i_0496f4),
    (0x0496F8, i_0496f8),
    (0x0496FC, i_0496fc),
    (0x049702, i_049702),
    (0x049704, i_049704),
    (0x049708, i_049708),
    (0x04970E, i_04970e),
    (0x049712, i_049712),
    (0x049718, i_049718),
    (0x04971C, i_04971c),
    (0x04971E, i_04971e),
    (0x049724, i_049724),
    (0x04972A, i_04972a),
    (0x049730, i_049730),
    (0x049734, i_049734),
    (0x04973A, i_04973a),
    (0x049740, i_049740),
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
    (0x049884, i_049884),
    (0x049886, i_049886),
    (0x04988A, i_04988a),
    (0x04988E, i_04988e),
    (0x049894, i_049894),
    (0x049896, i_049896),
    (0x04989A, i_04989a),
    (0x0498A0, i_0498a0),
    (0x0498A4, i_0498a4),
    (0x0498AA, i_0498aa),
    (0x0498AE, i_0498ae),
    (0x0498B0, i_0498b0),
    (0x0498B6, i_0498b6),
    (0x0498BA, i_0498ba),
    (0x0498C0, i_0498c0),
    (0x0498C4, i_0498c4),
    (0x0498C8, i_0498c8),
    (0x0498CA, i_0498ca),
    (0x0498D0, i_0498d0),
    (0x0498D4, i_0498d4),
    (0x0498D8, i_0498d8),
    (0x0498DE, i_0498de),
    (0x0498E2, i_0498e2),
    (0x0498E4, i_0498e4),
    (0x0498E8, i_0498e8),
    (0x0498EC, i_0498ec),
    (0x0498F2, i_0498f2),
    (0x0498F6, i_0498f6),
    (0x0498FA, i_0498fa),
    (0x0498FE, i_0498fe),
    (0x049900, i_049900),
    (0x049904, i_049904),
    (0x04990A, i_04990a),
    (0x04990E, i_04990e),
    (0x049912, i_049912),
    (0x049916, i_049916),
    (0x049918, i_049918),
    (0x04991C, i_04991c),
    (0x049922, i_049922),
    (0x049926, i_049926),
    (0x04992C, i_04992c),
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
];
