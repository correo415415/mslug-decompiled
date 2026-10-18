// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $076188: tst.w $70(a6)
pub fn i_076188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7618C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07618C: bne.w $76196
pub fn i_07618c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76190;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x76196; return 10; }
    8
}
/// $076190: lea.l $7619a(pc), a1
pub fn i_076190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76194;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7619A;
    16
}
/// $076194: move.l a1, (a6)
pub fn i_076194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76196;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076196: bra.w $75ffc
pub fn i_076196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7619A;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
// ===== ScriptedProp_Tower_Aim_07619a  ($07619A, 76 B, props/fx_explosions_platforms_crew_076000.s)
/// $07619A: move.w $16(a6), $14(a6)
pub fn i_07619a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761A0;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0761A0: movea.l $c(a6), a0
pub fn i_0761a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761A4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0761A4: move.w $7e(a0), d0
pub fn i_0761a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761A8;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0761A8: cmp.w $66(a0), d0
pub fn i_0761a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761AC;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0761AC: bge.w $761b8
pub fn i_0761ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761B0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x761B8; return 10; }
    8
}
/// $0761B0: lea.l $75f3c(pc), a0
pub fn i_0761b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761B4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75F3C;
    16
}
/// $0761B4: bra.w $761cc
pub fn i_0761b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x761CC;
    10
}
/// $0761B8: move.l #$7619a, $80(a6)
pub fn i_0761b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761C0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x7619A;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0761C0: tst.b $84(a6)
pub fn i_0761c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761C4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0761C4: beq.w $760f4
pub fn i_0761c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x760F4; return 10; }
    8
}
/// $0761C8: lea.l $75f34(pc), a0
pub fn i_0761c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761CC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75F34;
    16
}
/// $0761CC: jsr $5e086.l
pub fn i_0761cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x761D2);
    cpu.pc = 0x5E086;
    16
}
/// $0761D2: bcc.w $763c8
pub fn i_0761d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x763C8; return 10; }
    8
}
/// $0761D6: lea.l $2bf20c.l, a0
pub fn i_0761d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF20C;
    20
}
/// $0761DC: jsr $799de.l
pub fn i_0761dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x761E2);
    cpu.pc = 0x799DE;
    16
}
/// $0761E2: move.w d0, $72(a6)
pub fn i_0761e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761E6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== ScriptedProp_Tower_Fire_0761e6  ($0761E6, 70 B, props/fx_explosions_platforms_crew_076000.s)
/// $0761E6: lea.l $75a82(pc), a0
pub fn i_0761e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761EA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75A82;
    16
}
/// $0761EA: jsr $28cd4.l
pub fn i_0761ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x761F0);
    cpu.pc = 0x28CD4;
    16
}
/// $0761F0: bsr.w $75fc8
pub fn i_0761f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761F4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x761F4);
    cpu.pc = 0x75FC8;
    18
}
/// $0761F4: lea.l $761fa(pc), a1
pub fn i_0761f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x761FA;
    16
}
/// $0761F8: move.l a1, (a6)
pub fn i_0761f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0761FA: bsr.w $75f44
pub fn i_0761fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x761FE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x761FE);
    cpu.pc = 0x75F44;
    18
}
/// $0761FE: jsr $28d70.l
pub fn i_0761fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76204);
    cpu.pc = 0x28D70;
    16
}
/// $076204: bcc.w $76228
pub fn i_076204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76208;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76228; return 10; }
    8
}
/// $076208: tst.w $72(a6)
pub fn i_076208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7620C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07620C: beq.w $76222
pub fn i_07620c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76210;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x76222; return 10; }
    8
}
/// $076210: tst.w $70(a6)
pub fn i_076210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76214;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $076214: bne.w $7621e
pub fn i_076214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76218;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7621E; return 10; }
    8
}
/// $076218: lea.l $761e6(pc), a1
pub fn i_076218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7621C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x761E6;
    16
}
/// $07621C: move.l a1, (a6)
pub fn i_07621c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7621E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07621E: bra.w $76228
pub fn i_07621e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76222;
    cpu.ir = 0x6000;
    cpu.pc = 0x76228;
    10
}
/// $076222: lea.l $7622c(pc), a1
pub fn i_076222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76226;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7622C;
    16
}
/// $076226: move.l a1, (a6)
pub fn i_076226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76228;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076228: bra.w $75ffc
pub fn i_076228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7622C;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
// ===== ScriptedProp_TowerB_Idle_07622c  ($07622C, 100 B, props/fx_explosions_platforms_crew_076000.s)
/// $07622C: move.w $16(a6), $14(a6)
pub fn i_07622c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76232;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076232: movea.l $c(a6), a1
pub fn i_076232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76236;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $076236: move.w $7e(a1), d0
pub fn i_076236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7623A;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07623A: cmp.w $66(a1), d0
pub fn i_07623a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7623E;
    cpu.ir = 0xB069;
    let ad2 = cpu.a[1].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07623E: bge.w $76250
pub fn i_07623e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76242;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x76250; return 10; }
    8
}
/// $076242: lea.l $75288(pc), a0
pub fn i_076242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76246;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75288;
    16
}
/// $076246: jsr $28cd4.l
pub fn i_076246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7624C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7624C);
    cpu.pc = 0x28CD4;
    16
}
/// $07624C: bra.w $7626a
pub fn i_07624c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76250;
    cpu.ir = 0x6000;
    cpu.pc = 0x7626A;
    10
}
/// $076250: move.l #$7622c, $80(a6)
pub fn i_076250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76258;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x7622C;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $076258: tst.b $84(a6)
pub fn i_076258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7625C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07625C: beq.w $760f4
pub fn i_07625c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76260;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x760F4; return 10; }
    8
}
/// $076260: lea.l $7534c(pc), a0
pub fn i_076260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76264;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x7534C;
    16
}
/// $076264: jsr $28cd4.l
pub fn i_076264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7626A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7626A);
    cpu.pc = 0x28CD4;
    16
}
/// $07626A: lea.l $76270(pc), a1
pub fn i_07626a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7626E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76270;
    16
}
/// $07626E: move.l a1, (a6)
pub fn i_07626e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76270;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076270: bsr.w $75f44
pub fn i_076270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76274;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x76274);
    cpu.pc = 0x75F44;
    18
}
/// $076274: jsr $28d70.l
pub fn i_076274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7627A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7627A);
    cpu.pc = 0x28D70;
    16
}
/// $07627A: bcc.w $7628c
pub fn i_07627a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7627E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7628C; return 10; }
    8
}
/// $07627E: tst.w $70(a6)
pub fn i_07627e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76282;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $076282: bne.w $7628c
pub fn i_076282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76286;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7628C; return 10; }
    8
}
/// $076286: lea.l $76290(pc), a1
pub fn i_076286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7628A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76290;
    16
}
/// $07628A: move.l a1, (a6)
pub fn i_07628a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7628C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07628C: bra.w $75ffc
pub fn i_07628c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76290;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
// ===== ScriptedProp_TowerB_Aim_076290  ($076290, 76 B, props/fx_explosions_platforms_crew_076000.s)
/// $076290: move.w $16(a6), $14(a6)
pub fn i_076290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76296;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076296: movea.l $c(a6), a0
pub fn i_076296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7629A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07629A: move.w $7e(a0), d0
pub fn i_07629a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7629E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07629E: cmp.w $66(a0), d0
pub fn i_07629e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762A2;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0762A2: bge.w $762ae
pub fn i_0762a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762A6;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x762AE; return 10; }
    8
}
/// $0762A6: lea.l $75f3c(pc), a0
pub fn i_0762a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762AA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75F3C;
    16
}
/// $0762AA: bra.w $762c2
pub fn i_0762aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x762C2;
    10
}
/// $0762AE: move.l #$76290, $80(a6)
pub fn i_0762ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762B6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x76290;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0762B6: tst.b $84(a6)
pub fn i_0762b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762BA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0762BA: beq.w $760f4
pub fn i_0762ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x760F4; return 10; }
    8
}
/// $0762BE: lea.l $75f34(pc), a0
pub fn i_0762be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762C2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75F34;
    16
}
/// $0762C2: jsr $5e086.l
pub fn i_0762c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x762C8);
    cpu.pc = 0x5E086;
    16
}
/// $0762C8: bcc.w $76322
pub fn i_0762c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76322; return 10; }
    8
}
/// $0762CC: lea.l $2bf18a.l, a0
pub fn i_0762cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF18A;
    20
}
/// $0762D2: jsr $799de.l
pub fn i_0762d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x762D8);
    cpu.pc = 0x799DE;
    16
}
/// $0762D8: move.w d0, $72(a6)
pub fn i_0762d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762DC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== ScriptedProp_TowerB_Fire_0762dc  ($0762DC, 116 B, props/fx_explosions_platforms_crew_076000.s)
/// $0762DC: lea.l $754b2(pc), a0
pub fn i_0762dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762E0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x754B2;
    16
}
/// $0762E0: jsr $28cd4.l
pub fn i_0762e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x762E6);
    cpu.pc = 0x28CD4;
    16
}
/// $0762E6: bsr.w $75fc8
pub fn i_0762e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762EA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x762EA);
    cpu.pc = 0x75FC8;
    18
}
/// $0762EA: lea.l $762f0(pc), a1
pub fn i_0762ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x762F0;
    16
}
/// $0762EE: move.l a1, (a6)
pub fn i_0762ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0762F0: bsr.w $75f44
pub fn i_0762f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762F4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x762F4);
    cpu.pc = 0x75F44;
    18
}
/// $0762F4: jsr $28d70.l
pub fn i_0762f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x762FA);
    cpu.pc = 0x28D70;
    16
}
/// $0762FA: bcc.w $7631e
pub fn i_0762fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x762FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7631E; return 10; }
    8
}
/// $0762FE: tst.w $72(a6)
pub fn i_0762fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76302;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $076302: beq.w $76318
pub fn i_076302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76306;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x76318; return 10; }
    8
}
/// $076306: tst.w $70(a6)
pub fn i_076306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7630A;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07630A: bne.w $76314
pub fn i_07630a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7630E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x76314; return 10; }
    8
}
/// $07630E: lea.l $762dc(pc), a1
pub fn i_07630e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76312;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x762DC;
    16
}
/// $076312: move.l a1, (a6)
pub fn i_076312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76314;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076314: bra.w $7631e
pub fn i_076314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76318;
    cpu.ir = 0x6000;
    cpu.pc = 0x7631E;
    10
}
/// $076318: lea.l $76138(pc), a1
pub fn i_076318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7631C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76138;
    16
}
/// $07631C: move.l a1, (a6)
pub fn i_07631c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7631E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07631E: bra.w $75ffc
pub fn i_07631e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76322;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
/// $076322: move.w $16(a6), $14(a6)
pub fn i_076322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76328;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076328: lea.l $75bd8(pc), a0
pub fn i_076328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7632C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75BD8;
    16
}
/// $07632C: jsr $28cd4.l
pub fn i_07632c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76332;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76332);
    cpu.pc = 0x28CD4;
    16
}
/// $076332: lea.l $76338(pc), a1
pub fn i_076332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76336;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76338;
    16
}
/// $076336: move.l a1, (a6)
pub fn i_076336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76338;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076338: bsr.w $75f44
pub fn i_076338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7633C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x7633C);
    cpu.pc = 0x75F44;
    18
}
/// $07633C: jsr $28d70.l
pub fn i_07633c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76342;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76342);
    cpu.pc = 0x28D70;
    16
}
/// $076342: bcc.w $7634c
pub fn i_076342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76346;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7634C; return 10; }
    8
}
/// $076346: lea.l $76350(pc), a1
pub fn i_076346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7634A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76350;
    16
}
/// $07634A: move.l a1, (a6)
pub fn i_07634a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7634C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07634C: bra.w $75ffc
pub fn i_07634c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76350;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
// ===== ScriptedProp_Door_Open_076350  ($076350, 74 B, props/fx_explosions_platforms_crew_076000.s)
/// $076350: move.w $16(a6), $14(a6)
pub fn i_076350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76356;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076356: lea.l $75c34(pc), a0
pub fn i_076356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7635A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75C34;
    16
}
/// $07635A: jsr $28cd4.l
pub fn i_07635a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76360;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76360);
    cpu.pc = 0x28CD4;
    16
}
/// $076360: move.w #$3, $72(a6)
pub fn i_076360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76366;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076366: lea.l $7636c(pc), a1
pub fn i_076366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7636A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7636C;
    16
}
/// $07636A: move.l a1, (a6)
pub fn i_07636a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7636C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07636C: bsr.w $75f44
pub fn i_07636c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76370;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x76370);
    cpu.pc = 0x75F44;
    18
}
/// $076370: jsr $28d70.l
pub fn i_076370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76376;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76376);
    cpu.pc = 0x28D70;
    16
}
/// $076376: bcc.w $76396
pub fn i_076376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7637A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76396; return 10; }
    8
}
/// $07637A: subq.w #$1, $72(a6)
pub fn i_07637a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7637E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07637E: bne.w $7638c
pub fn i_07637e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76382;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7638C; return 10; }
    8
}
/// $076382: lea.l $7639a(pc), a1
pub fn i_076382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76386;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7639A;
    16
}
/// $076386: move.l a1, (a6)
pub fn i_076386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76388;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076388: bra.w $76396
pub fn i_076388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7638C;
    cpu.ir = 0x6000;
    cpu.pc = 0x76396;
    10
}
/// $07638C: lea.l $75c34(pc), a0
pub fn i_07638c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76390;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75C34;
    16
}
/// $076390: jsr $28cd4.l
pub fn i_076390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76396;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76396);
    cpu.pc = 0x28CD4;
    16
}
/// $076396: bra.w $75ffc
pub fn i_076396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7639A;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
// ===== ScriptedProp_Door_Close_07639a  ($07639A, 92 B, props/fx_explosions_platforms_crew_076000.s)
/// $07639A: move.w $16(a6), $14(a6)
pub fn i_07639a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763A0;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0763A0: lea.l $75cd0(pc), a0
pub fn i_0763a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763A4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75CD0;
    16
}
/// $0763A4: jsr $28cd4.l
pub fn i_0763a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x763AA);
    cpu.pc = 0x28CD4;
    16
}
/// $0763AA: lea.l $763b0(pc), a1
pub fn i_0763aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x763B0;
    16
}
/// $0763AE: move.l a1, (a6)
pub fn i_0763ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0763B0: bsr.w $75f44
pub fn i_0763b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763B4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x763B4);
    cpu.pc = 0x75F44;
    18
}
/// $0763B4: jsr $28d70.l
pub fn i_0763b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x763BA);
    cpu.pc = 0x28D70;
    16
}
/// $0763BA: bcc.w $763c4
pub fn i_0763ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x763C4; return 10; }
    8
}
/// $0763BE: lea.l $76138(pc), a1
pub fn i_0763be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76138;
    16
}
/// $0763C2: move.l a1, (a6)
pub fn i_0763c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0763C4: bra.w $75ffc
pub fn i_0763c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
/// $0763C8: move.w $16(a6), $14(a6)
pub fn i_0763c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763CE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0763CE: lea.l $75d2c(pc), a0
pub fn i_0763ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763D2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75D2C;
    16
}
/// $0763D2: jsr $28cd4.l
pub fn i_0763d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x763D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0763D8: lea.l $763de(pc), a1
pub fn i_0763d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x763DE;
    16
}
/// $0763DC: move.l a1, (a6)
pub fn i_0763dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0763DE: bsr.w $75f44
pub fn i_0763de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763E2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x763E2);
    cpu.pc = 0x75F44;
    18
}
/// $0763E2: jsr $28d70.l
pub fn i_0763e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x763E8);
    cpu.pc = 0x28D70;
    16
}
/// $0763E8: bcc.w $763f2
pub fn i_0763e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x763F2; return 10; }
    8
}
/// $0763EC: lea.l $763f6(pc), a1
pub fn i_0763ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x763F6;
    16
}
/// $0763F0: move.l a1, (a6)
pub fn i_0763f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0763F2: bra.w $75ffc
pub fn i_0763f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
// ===== ScriptedProp_DoorB_Open_0763f6  ($0763F6, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $0763F6: move.w $16(a6), $14(a6)
pub fn i_0763f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x763FC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0763FC: lea.l $75dac(pc), a0
pub fn i_0763fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76400;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75DAC;
    16
}
// ===== ScriptedProp_DoorB_OpenLoop_076400  ($076400, 64 B, props/fx_explosions_platforms_crew_076000.s)
/// $076400: jsr $28cd4.l
pub fn i_076400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76406;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76406);
    cpu.pc = 0x28CD4;
    16
}
/// $076406: move.w #$3, $72(a6)
pub fn i_076406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7640C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07640C: lea.l $76412(pc), a1
pub fn i_07640c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76410;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76412;
    16
}
/// $076410: move.l a1, (a6)
pub fn i_076410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76412;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076412: bsr.w $75f44
pub fn i_076412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76416;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x76416);
    cpu.pc = 0x75F44;
    18
}
/// $076416: jsr $28d70.l
pub fn i_076416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7641C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7641C);
    cpu.pc = 0x28D70;
    16
}
/// $07641C: bcc.w $7643c
pub fn i_07641c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76420;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7643C; return 10; }
    8
}
/// $076420: subq.w #$1, $72(a6)
pub fn i_076420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76424;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $076424: bne.w $76432
pub fn i_076424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76428;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x76432; return 10; }
    8
}
/// $076428: lea.l $76440(pc), a1
pub fn i_076428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7642C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76440;
    16
}
/// $07642C: move.l a1, (a6)
pub fn i_07642c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7642E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07642E: bra.w $7643c
pub fn i_07642e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76432;
    cpu.ir = 0x6000;
    cpu.pc = 0x7643C;
    10
}
/// $076432: lea.l $75dac(pc), a0
pub fn i_076432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76436;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75DAC;
    16
}
/// $076436: jsr $28cd4.l
pub fn i_076436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7643C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7643C);
    cpu.pc = 0x28CD4;
    16
}
/// $07643C: bra.w $75ffc
pub fn i_07643c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76440;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
// ===== ScriptedProp_DoorB_Close_076440  ($076440, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $076440: move.w $16(a6), $14(a6)
pub fn i_076440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76446;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076446: lea.l $75e76(pc), a0
pub fn i_076446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7644A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75E76;
    16
}
/// $07644A: jsr $28cd4.l
pub fn i_07644a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76450;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76450);
    cpu.pc = 0x28CD4;
    16
}
/// $076450: lea.l $76456(pc), a1
pub fn i_076450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76454;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76456;
    16
}
/// $076454: move.l a1, (a6)
pub fn i_076454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76456;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076456: bsr.w $75f44
pub fn i_076456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7645A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x7645A);
    cpu.pc = 0x75F44;
    18
}
/// $07645A: jsr $28d70.l
pub fn i_07645a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76460;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76460);
    cpu.pc = 0x28D70;
    16
}
/// $076460: bcc.w $7646a
pub fn i_076460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76464;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7646A; return 10; }
    8
}
/// $076464: lea.l $7622c(pc), a1
pub fn i_076464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76468;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7622C;
    16
}
/// $076468: move.l a1, (a6)
pub fn i_076468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7646A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07646A: bra.w $75ffc
pub fn i_07646a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7646E;
    cpu.ir = 0x6000;
    cpu.pc = 0x75FFC;
    10
}
// ===== ScriptedProp_Child_Collapse_07646e  ($07646E, 66 B, props/fx_explosions_platforms_crew_076000.s)
/// $07646E: move.w $16(a6), $14(a6)
pub fn i_07646e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76474;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076474: bset.b #$0, $13(a6)
pub fn i_076474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7647A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07647A: lea.l $75bcc(pc), a0
pub fn i_07647a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7647E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75BCC;
    16
}
/// $07647E: jsr $28cd4.l
pub fn i_07647e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76484;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76484);
    cpu.pc = 0x28CD4;
    16
}
/// $076484: lea.l $7648a(pc), a1
pub fn i_076484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76488;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7648A;
    16
}
/// $076488: move.l a1, (a6)
pub fn i_076488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7648A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07648A: bsr.w $75f44
pub fn i_07648a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7648E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x7648E);
    cpu.pc = 0x75F44;
    18
}
/// $07648E: jsr $28d70.l
pub fn i_07648e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76494;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76494);
    cpu.pc = 0x28D70;
    16
}
/// $076494: bcc.w $764ae
pub fn i_076494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76498;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x764AE; return 10; }
    8
}
/// $076498: lea.l $749d6(pc), a1
pub fn i_076498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7649C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x749D6;
    16
}
/// $07649C: jsr $77c7e.l
pub fn i_07649c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x764A2);
    cpu.pc = 0x77C7E;
    16
}
/// $0764A2: jsr $13600.l
pub fn i_0764a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x764A8);
    cpu.pc = 0x13600;
    16
}
/// $0764A8: jmp $77f6a.l
pub fn i_0764a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764AE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
/// $0764AE: rts
pub fn i_0764ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Flag_Follow_0764b0  ($0764B0, 94 B, props/fx_explosions_platforms_crew_076000.s)
/// $0764B0: movea.l $c(a6), a0
pub fn i_0764b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764B4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0764B4: btst.b #$0, $13(a0)
pub fn i_0764b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764BA;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0764BA: beq.w $764ca
pub fn i_0764ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x764CA; return 10; }
    8
}
/// $0764BE: jsr $5b6.l
pub fn i_0764be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x764C4);
    cpu.pc = 0x5B6;
    16
}
/// $0764C4: jmp $518.l
pub fn i_0764c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764CA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0764CA: move.w $22(a0), $22(a6)
pub fn i_0764ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764D0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0764D0: move.w $24(a0), d0
pub fn i_0764d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764D4;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0764D4: add.w $78(a0), d0
pub fn i_0764d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764D8;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0764D8: move.w d0, $24(a6)
pub fn i_0764d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764DC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0764DC: move.w $38(a0), $38(a6)
pub fn i_0764dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764E2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0764E2: move.l $74(a0), $3c(a6)
pub fn i_0764e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764E8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0764E8: move.w $7a(a0), d1
pub fn i_0764e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764EC;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0764EC: cmp.w $7a(a6), d1
pub fn i_0764ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764F0;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0764F0: beq.w $76508
pub fn i_0764f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x76508; return 10; }
    8
}
/// $0764F4: move.w d1, $7a(a6)
pub fn i_0764f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764F8;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0764F8: jsr $13600.l
pub fn i_0764f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x764FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x764FE);
    cpu.pc = 0x13600;
    16
}
/// $0764FE: move.w $7a(a6), d1
pub fn i_0764fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76502;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076502: jsr $236e.l
pub fn i_076502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76508;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76508);
    cpu.pc = 0x236E;
    16
}
/// $076508: jmp $5ca2a.l
pub fn i_076508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7650E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CA2A;
    8
}
// ===== ScriptedProp_Cb_SpawnFlame_07650e  ($07650E, 40 B, props/fx_explosions_platforms_crew_076000.s)
/// $07650E: lea.l $76766(pc), a1
pub fn i_07650e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76512;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76766;
    16
}
/// $076512: jsr $4ae.l
pub fn i_076512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76518;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76518);
    cpu.pc = 0x4AE;
    16
}
/// $076518: move.l #$76576, $4c(a6)
pub fn i_076518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76520;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x76576;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $076520: jsr $283ca.l
pub fn i_076520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76526;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76526);
    cpu.pc = 0x283CA;
    16
}
/// $076526: jsr $283d8.l
pub fn i_076526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7652C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7652C);
    cpu.pc = 0x283D8;
    16
}
/// $07652C: move.l #$ffffffff, $4c(a6)
pub fn i_07652c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76534;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $076534: rts
pub fn i_076534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76536;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Cb_SpawnSmoke_076536  ($076536, 64 B, props/fx_explosions_platforms_crew_076000.s)
/// $076536: lea.l $767f4(pc), a1
pub fn i_076536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7653A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x767F4;
    16
}
/// $07653A: jsr $4ae.l
pub fn i_07653a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76540;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76540);
    cpu.pc = 0x4AE;
    16
}
/// $076540: move.w $22(a6), d0
pub fn i_076540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76544;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076544: move.w $24(a6), d1
pub fn i_076544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76548;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076548: subi.w #$0, d0
pub fn i_076548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7654C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07654C: subi.w #$40, d1
pub fn i_07654c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76550;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076550: move.w d0, $22(a0)
pub fn i_076550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76554;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076554: move.w d1, $24(a0)
pub fn i_076554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76558;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076558: move.l #$765ca, $4c(a6)
pub fn i_076558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76560;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x765CA;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $076560: jsr $283ca.l
pub fn i_076560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76566;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76566);
    cpu.pc = 0x283CA;
    16
}
/// $076566: jsr $283d8.l
pub fn i_076566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7656C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7656C);
    cpu.pc = 0x283D8;
    16
}
/// $07656C: move.l #$ffffffff, $4c(a6)
pub fn i_07656c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76574;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $076574: rts
pub fn i_076574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76576;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Flame_Run_076766  ($076766, 142 B, props/fx_explosions_platforms_crew_076000.s)
/// $076766: move.w #$156, d1
pub fn i_076766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7676A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x156;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07676A: jsr $236e.l
pub fn i_07676a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76770;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76770);
    cpu.pc = 0x236E;
    16
}
/// $076770: move.w #$0, $38(a6)
pub fn i_076770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76776;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076776: clr.b $20(a6)
pub fn i_076776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7677A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07677A: lea.l $75704(pc), a0
pub fn i_07677a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7677E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75704;
    16
}
/// $07677E: jsr $28cd4.l
pub fn i_07677e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76784;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76784);
    cpu.pc = 0x28CD4;
    16
}
/// $076784: lea.l $766c2(pc), a0
pub fn i_076784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76788;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x766C2;
    16
}
/// $076788: move.l a0, $4c(a6)
pub fn i_076788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7678C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07678C: jsr $283ca.l
pub fn i_07678c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76792;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76792);
    cpu.pc = 0x283CA;
    16
}
/// $076792: lea.l $76798(pc), a1
pub fn i_076792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76796;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76798;
    16
}
/// $076796: move.l a1, (a6)
pub fn i_076796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76798;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076798: movea.l $c(a6), a0
pub fn i_076798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7679C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07679C: move.w $22(a0), d0
pub fn i_07679c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767A0;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0767A0: subi.w #$20, d0
pub fn i_0767a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767A4;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0767A4: move.w d0, $22(a6)
pub fn i_0767a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767A8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0767A8: move.w $24(a0), d0
pub fn i_0767a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767AC;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0767AC: subi.w #$18, d0
pub fn i_0767ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767B0;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0767B0: move.w d0, $24(a6)
pub fn i_0767b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767B4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0767B4: btst.b #$0, $13(a0)
pub fn i_0767b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767BA;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0767BA: beq.w $767c4
pub fn i_0767ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x767C4; return 10; }
    8
}
/// $0767BE: jmp $518.l
pub fn i_0767be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767C4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0767C4: jsr $28d70.l
pub fn i_0767c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x767CA);
    cpu.pc = 0x28D70;
    16
}
/// $0767CA: bcc.w $767d4
pub fn i_0767ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x767D4; return 10; }
    8
}
/// $0767CE: jmp $518.l
pub fn i_0767ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767D4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0767D4: tst.b $20(a6)
pub fn i_0767d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767D8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0767D8: beq.w $767f2
pub fn i_0767d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x767F2; return 10; }
    8
}
/// $0767DC: clr.b $20(a6)
pub fn i_0767dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767E0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0767E0: move.l a6, -(a7)
pub fn i_0767e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767E2;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0767E2: movea.l $c(a6), a6
pub fn i_0767e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767E6;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $0767E6: lea.l $75688(pc), a0
pub fn i_0767e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767EA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75688;
    16
}
/// $0767EA: jsr $28cd4.l
pub fn i_0767ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x767F0);
    cpu.pc = 0x28CD4;
    16
}
/// $0767F0: movea.l (a7)+, a6
pub fn i_0767f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767F2;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $0767F2: rts
pub fn i_0767f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Smoke_Rise_0767f4  ($0767F4, 102 B, props/fx_explosions_platforms_crew_076000.s)
/// $0767F4: lea.l $75b98(pc), a0
pub fn i_0767f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767F8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x75B98;
    16
}
/// $0767F8: jsr $28cd4.l
pub fn i_0767f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x767FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x767FE);
    cpu.pc = 0x28CD4;
    16
}
/// $0767FE: move.w #$55, d1
pub fn i_0767fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76802;
    cpu.ir = 0x323C;
    let v1: u32 = 0x55;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076802: jsr $236e.l
pub fn i_076802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76808;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76808);
    cpu.pc = 0x236E;
    16
}
/// $076808: bset.b #$6, $12(a6)
pub fn i_076808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7680E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07680E: lea.l $7661e(pc), a0
pub fn i_07680e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76812;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x7661E;
    16
}
/// $076812: move.l a0, $4c(a6)
pub fn i_076812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76816;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $076816: jsr $283ca.l
pub fn i_076816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7681C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7681C);
    cpu.pc = 0x283CA;
    16
}
/// $07681C: lea.l $2bf28e.l, a0
pub fn i_07681c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76822;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF28E;
    20
}
/// $076822: jsr $799de.l
pub fn i_076822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76828;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76828);
    cpu.pc = 0x799DE;
    16
}
/// $076828: neg.w d0
pub fn i_076828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7682A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07682A: move.w d0, $28(a6)
pub fn i_07682a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7682E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07682E: clr.w $2a(a6)
pub fn i_07682e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76832;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $076832: lea.l $76838(pc), a1
pub fn i_076832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76836;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76838;
    16
}
/// $076836: move.l a1, (a6)
pub fn i_076836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76838;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076838: jsr $27cee.l
pub fn i_076838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7683E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7683E);
    cpu.pc = 0x27CEE;
    16
}
/// $07683E: jsr $283d8.l
pub fn i_07683e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76844;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76844);
    cpu.pc = 0x283D8;
    16
}
/// $076844: move.w $22(a6), d0
pub fn i_076844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76848;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076848: addi.w #$30, d0
pub fn i_076848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7684C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07684C: cmpi.w #$1a0, d0
pub fn i_07684c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76850;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1A0, Size::Word, false);
    8
}
/// $076850: bcs.w $7685a
pub fn i_076850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76854;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7685A; return 10; }
    8
}
/// $076854: jmp $518.l
pub fn i_076854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7685A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrAbsThunk_07685a  ($07685A, 8 B, thunks/jsr_abs_thunks.c)
/// $07685A: jsr $28d70.l
pub fn i_07685a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76860;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76860);
    cpu.pc = 0x28D70;
    16
}
/// $076860: rts
pub fn i_076860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76862;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Window_Init_07690a  ($07690A, 26 B, props/fx_explosions_platforms_crew_076000.s)
/// $07690A: move.w #$d000, $38(a6)
pub fn i_07690a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76910;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076910: move.w #$51, d1
pub fn i_076910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76914;
    cpu.ir = 0x323C;
    let v1: u32 = 0x51;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076914: jsr $236e.l
pub fn i_076914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7691A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7691A);
    cpu.pc = 0x236E;
    16
}
/// $07691A: lea.l $76862(pc), a0
pub fn i_07691a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7691E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x76862;
    16
}
/// $07691E: jsr $28cd4.l
pub fn i_07691e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76924;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76924);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_076924  ($076924, 8 B, kernel/task_handlers.c)
/// $076924: lea.l $7692c(pc), a1
pub fn i_076924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76928;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7692C;
    16
}
/// $076928: move.l a1, (a6)
pub fn i_076928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7692A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07692A: rts
pub fn i_07692a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7692C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Window_Idle_07692c  ($07692C, 38 B, props/fx_explosions_platforms_crew_076000.s)
/// $07692C: movea.l $c(a6), a0
pub fn i_07692c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76930;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $076930: tst.b $70(a0)
pub fn i_076930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76934;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $076934: beq.w $7693e
pub fn i_076934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76938;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7693E; return 10; }
    8
}
/// $076938: lea.l $7695a(pc), a1
pub fn i_076938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7693C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7695A;
    16
}
/// $07693C: move.l a1, (a6)
pub fn i_07693c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7693E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07693E: move.w #$fe0, d0
pub fn i_07693e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76942;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076942: move.w #$a0, d1
pub fn i_076942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76946;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076946: jsr $750c0(pc)
pub fn i_076946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7694A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7694A);
    cpu.pc = 0x750C0;
    16
}
/// $07694A: move.w d0, $22(a6)
pub fn i_07694a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7694E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07694E: move.w d1, $24(a6)
pub fn i_07694e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76952;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_076952  ($076952, 8 B, thunks/jsr_abs_thunks.c)
/// $076952: jsr $28d70.l
pub fn i_076952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76958;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76958);
    cpu.pc = 0x28D70;
    16
}
/// $076958: rts
pub fn i_076958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7695A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Window_Fall_07695a  ($07695A, 72 B, props/fx_explosions_platforms_crew_076000.s)
/// $07695A: lea.l $7686e(pc), a0
pub fn i_07695a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7695E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x7686E;
    16
}
/// $07695E: jsr $28cd4.l
pub fn i_07695e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76964);
    cpu.pc = 0x28CD4;
    16
}
/// $076964: clr.b $20(a6)
pub fn i_076964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76968;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $076968: clr.w $28(a6)
pub fn i_076968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7696C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07696C: clr.w $2a(a6)
pub fn i_07696c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76970;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $076970: lea.l $76976(pc), a1
pub fn i_076970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76974;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76976;
    16
}
/// $076974: move.l a1, (a6)
pub fn i_076974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76976;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076976: tst.b $20(a6)
pub fn i_076976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7697A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07697A: beq.w $76984
pub fn i_07697a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7697E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x76984; return 10; }
    8
}
/// $07697E: addi.w #$10, $28(a6)
pub fn i_07697e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76984;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $076984: jsr $27cee.l
pub fn i_076984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7698A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7698A);
    cpu.pc = 0x27CEE;
    16
}
/// $07698A: jsr $28d70.l
pub fn i_07698a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76990);
    cpu.pc = 0x28D70;
    16
}
/// $076990: cmpi.w #$180, $22(a6)
pub fn i_076990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76996;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x180, Size::Word, false);
    16
}
/// $076996: bcs.w $769a0
pub fn i_076996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7699A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x769A0; return 10; }
    8
}
/// $07699A: jmp $518.l
pub fn i_07699a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x769A0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0769A0: rts
pub fn i_0769a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x769A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Roof_SetSprite_076a0e  ($076A0E, 28 B, props/fx_explosions_platforms_crew_076000.s)
/// $076A0E: move.w #$50, d1
pub fn i_076a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A12;
    cpu.ir = 0x323C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076A12: jsr $236e.l
pub fn i_076a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76A18);
    cpu.pc = 0x236E;
    16
}
/// $076A18: lea.l $769a2(pc), a0
pub fn i_076a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A1C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x769A2;
    16
}
/// $076A1C: jsr $28cd4.l
pub fn i_076a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76A22);
    cpu.pc = 0x28CD4;
    16
}
/// $076A22: move.w #$a0, $66(a6)
pub fn i_076a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A28;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076A28: rts
pub fn i_076a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_RoofA_Run_076a2a  ($076A2A, 94 B, props/fx_explosions_platforms_crew_076000.s)
/// $076A2A: bsr.b $76a0e
pub fn i_076a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A2C;
    cpu.ir = 0x61E2;
    cpu.push32(bus, 0x76A2C);
    cpu.pc = 0x76A0E;
    18
}
/// $076A2C: lea.l $76a32(pc), a1
pub fn i_076a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76A32;
    16
}
/// $076A30: move.l a1, (a6)
pub fn i_076a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076A32: move.w #$f20, d0
pub fn i_076a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A36;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076A36: move.w #$30, d1
pub fn i_076a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A3A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076A3A: jsr $750c0(pc)
pub fn i_076a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x76A3E);
    cpu.pc = 0x750C0;
    16
}
/// $076A3E: move.w d0, $22(a6)
pub fn i_076a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A42;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076A42: move.w d1, $24(a6)
pub fn i_076a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A46;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076A46: jsr $28d70.l
pub fn i_076a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76A4C);
    cpu.pc = 0x28D70;
    16
}
/// $076A4C: movea.l $c(a6), a0
pub fn i_076a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A50;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $076A50: tst.b $76(a0)
pub fn i_076a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A54;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $076A54: bne.w $76a7e
pub fn i_076a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A58;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x76A7E; return 10; }
    8
}
/// $076A58: jsr $2870a.l
pub fn i_076a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76A5E);
    cpu.pc = 0x2870A;
    16
}
/// $076A5E: bcc.w $76a74
pub fn i_076a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A62;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76A74; return 10; }
    8
}
/// $076A62: lea.l $5e766.l, a0
pub fn i_076a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A68;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $076A68: jsr $5e798.l
pub fn i_076a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76A6E);
    cpu.pc = 0x5E798;
    16
}
/// $076A6E: bclr.b #$3, $13(a6)
pub fn i_076a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A74;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $076A74: jsr $28758.l
pub fn i_076a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76A7A);
    cpu.pc = 0x28758;
    16
}
/// $076A7A: bcc.w $76a8e
pub fn i_076a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A7E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76A8E; return 10; }
    8
}
/// $076A7E: lea.l $749b2(pc), a1
pub fn i_076a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x749B2;
    16
}
/// $076A82: jsr $77c7e.l
pub fn i_076a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76A88);
    cpu.pc = 0x77C7E;
    16
}
// ===== SetTaskHandler_076a88  ($076A88, 8 B, kernel/task_handlers.c)
/// $076A88: lea.l $76a90(pc), a1
pub fn i_076a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76A90;
    16
}
/// $076A8C: move.l a1, (a6)
pub fn i_076a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076A8E: rts
pub fn i_076a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_RoofA_Debris_076a90  ($076A90, 64 B, props/fx_explosions_platforms_crew_076000.s)
/// $076A90: lea.l $769ae(pc), a0
pub fn i_076a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A94;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x769AE;
    16
}
/// $076A94: jsr $28cd4.l
pub fn i_076a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76A9A);
    cpu.pc = 0x28CD4;
    16
}
/// $076A9A: lea.l $747b2(pc), a2
pub fn i_076a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76A9E;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x747B2;
    16
}
/// $076A9E: jsr $5022a.l
pub fn i_076a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76AA4);
    cpu.pc = 0x5022A;
    16
}
/// $076AA4: lea.l $76aaa(pc), a1
pub fn i_076aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76AAA;
    16
}
/// $076AA8: move.l a1, (a6)
pub fn i_076aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076AAA: move.w #$f20, d0
pub fn i_076aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AAE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076AAE: move.w #$30, d1
pub fn i_076aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AB2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076AB2: jsr $750c0(pc)
pub fn i_076ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AB6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x76AB6);
    cpu.pc = 0x750C0;
    16
}
/// $076AB6: move.w d0, $22(a6)
pub fn i_076ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ABA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076ABA: move.w d1, $24(a6)
pub fn i_076aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ABE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076ABE: jsr $28d70.l
pub fn i_076abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76AC4);
    cpu.pc = 0x28D70;
    16
}
/// $076AC4: movea.l $c(a6), a0
pub fn i_076ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AC8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $076AC8: tst.b $78(a0)
pub fn i_076ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ACC;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $076ACC: beq.w $76adc
pub fn i_076acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AD0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x76ADC; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_076ad0  ($076AD0, 14 B, kernel/jsr5b6_jmp518.c)
/// $076AD0: jsr $5b6.l
pub fn i_076ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76AD6);
    cpu.pc = 0x5B6;
    16
}
/// $076AD6: jmp $518.l
pub fn i_076ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ADC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $076ADC: rts
pub fn i_076adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ADE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_RoofB_Run_076ade  ($076ADE, 84 B, props/fx_explosions_platforms_crew_076000.s)
/// $076ADE: bsr.w $76a0e
pub fn i_076ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AE2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x76AE2);
    cpu.pc = 0x76A0E;
    18
}
/// $076AE2: lea.l $76ae8(pc), a1
pub fn i_076ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76AE8;
    16
}
/// $076AE6: move.l a1, (a6)
pub fn i_076ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AE8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076AE8: move.w #$fc8, d0
pub fn i_076ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AEC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFC8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076AEC: move.w #$30, d1
pub fn i_076aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AF0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076AF0: jsr $750c0(pc)
pub fn i_076af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x76AF4);
    cpu.pc = 0x750C0;
    16
}
/// $076AF4: move.w d0, $22(a6)
pub fn i_076af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AF8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076AF8: move.w d1, $24(a6)
pub fn i_076af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76AFC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076AFC: jsr $28d70.l
pub fn i_076afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76B02);
    cpu.pc = 0x28D70;
    16
}
/// $076B02: movea.l $c(a6), a0
pub fn i_076b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B06;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $076B06: tst.b $77(a0)
pub fn i_076b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B0A;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x77);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $076B0A: bne.w $76b28
pub fn i_076b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B0E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x76B28; return 10; }
    8
}
/// $076B0E: jsr $2870a.l
pub fn i_076b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76B14);
    cpu.pc = 0x2870A;
    16
}
/// $076B14: bcc.w $76b1e
pub fn i_076b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B18;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76B1E; return 10; }
    8
}
/// $076B18: bclr.b #$3, $13(a6)
pub fn i_076b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B1E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $076B1E: jsr $28758.l
pub fn i_076b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76B24);
    cpu.pc = 0x28758;
    16
}
/// $076B24: bcc.w $76b38
pub fn i_076b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B28;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76B38; return 10; }
    8
}
/// $076B28: lea.l $749c4(pc), a1
pub fn i_076b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x749C4;
    16
}
/// $076B2C: jsr $77c7e.l
pub fn i_076b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76B32);
    cpu.pc = 0x77C7E;
    16
}
// ===== SetTaskHandler_076b32  ($076B32, 8 B, kernel/task_handlers.c)
/// $076B32: lea.l $76b3a(pc), a1
pub fn i_076b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76B3A;
    16
}
/// $076B36: move.l a1, (a6)
pub fn i_076b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076B38: rts
pub fn i_076b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_RoofB_Debris_076b3a  ($076B3A, 100 B, props/fx_explosions_platforms_crew_076000.s)
/// $076B3A: lea.l $769ae(pc), a0
pub fn i_076b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B3E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x769AE;
    16
}
/// $076B3E: jsr $28cd4.l
pub fn i_076b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76B44);
    cpu.pc = 0x28CD4;
    16
}
/// $076B44: lea.l $747c6(pc), a2
pub fn i_076b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B48;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x747C6;
    16
}
/// $076B48: jsr $5022a.l
pub fn i_076b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76B4E);
    cpu.pc = 0x5022A;
    16
}
/// $076B4E: lea.l $747da(pc), a2
pub fn i_076b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B52;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x747DA;
    16
}
/// $076B52: jsr $5022a.l
pub fn i_076b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76B58);
    cpu.pc = 0x5022A;
    16
}
/// $076B58: lea.l $747ee(pc), a2
pub fn i_076b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B5C;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x747EE;
    16
}
/// $076B5C: jsr $5022a.l
pub fn i_076b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76B62);
    cpu.pc = 0x5022A;
    16
}
/// $076B62: lea.l $748b6(pc), a2
pub fn i_076b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B66;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x748B6;
    16
}
/// $076B66: jsr $5022a.l
pub fn i_076b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76B6C);
    cpu.pc = 0x5022A;
    16
}
/// $076B6C: lea.l $76b72(pc), a1
pub fn i_076b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76B72;
    16
}
/// $076B70: move.l a1, (a6)
pub fn i_076b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076B72: movea.l $c(a6), a0
pub fn i_076b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B76;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $076B76: tst.b $79(a0)
pub fn i_076b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B7A;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $076B7A: beq.w $76b8a
pub fn i_076b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x76B8A; return 10; }
    8
}
/// $076B7E: jsr $5b6.l
pub fn i_076b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76B84);
    cpu.pc = 0x5B6;
    16
}
/// $076B84: lea.l $76b8a(pc), a1
pub fn i_076b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76B8A;
    16
}
/// $076B88: move.l a1, (a6)
pub fn i_076b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076B8A: move.w #$fc8, d0
pub fn i_076b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B8E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFC8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076B8E: move.w #$30, d1
pub fn i_076b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B92;
    cpu.ir = 0x323C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076B92: jsr $750c0(pc)
pub fn i_076b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x76B96);
    cpu.pc = 0x750C0;
    16
}
/// $076B96: move.w d0, $22(a6)
pub fn i_076b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B9A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076B9A: move.w d1, $24(a6)
pub fn i_076b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76B9E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_076b9e  ($076B9E, 8 B, thunks/jsr_abs_thunks.c)
/// $076B9E: jsr $28d70.l
pub fn i_076b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76BA4);
    cpu.pc = 0x28D70;
    16
}
/// $076BA4: rts
pub fn i_076ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Gate_Init_076ba6  ($076BA6, 34 B, props/fx_explosions_platforms_crew_076000.s)
/// $076BA6: move.w #$52, d1
pub fn i_076ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BAA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x52;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076BAA: jsr $236e.l
pub fn i_076baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76BB0);
    cpu.pc = 0x236E;
    16
}
/// $076BB0: move.w #$167, d1
pub fn i_076bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BB4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x167;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076BB4: jsr $236e.l
pub fn i_076bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76BBA);
    cpu.pc = 0x236E;
    16
}
/// $076BBA: move.w #$0, $38(a6)
pub fn i_076bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076BC0: move.l #$2435e6, $3c(a6)
pub fn i_076bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BC8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2435E6;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== SetTaskHandler_076bc8  ($076BC8, 8 B, kernel/task_handlers.c)
/// $076BC8: lea.l $76bd0(pc), a1
pub fn i_076bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76BD0;
    16
}
/// $076BCC: move.l a1, (a6)
pub fn i_076bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076BCE: rts
pub fn i_076bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptedProp_Gate_Idle_076bd0  ($076BD0, 24 B, props/fx_explosions_platforms_crew_076000.s)
/// $076BD0: move.w #$fe0, d0
pub fn i_076bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BD4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076BD4: move.w #$a0, d1
pub fn i_076bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BD8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076BD8: jsr $750c0(pc)
pub fn i_076bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x76BDC);
    cpu.pc = 0x750C0;
    16
}
/// $076BDC: move.w d0, $22(a6)
pub fn i_076bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BE0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076BE0: move.w d1, $24(a6)
pub fn i_076be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BE4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076BE4: jmp $76c50(pc)
pub fn i_076be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BE8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x76C50;
    8
}
// ===== ScriptedProp_Base_Run_076be8  ($076BE8, 132 B, props/fx_explosions_platforms_crew_076000.s)
/// $076BE8: move.w #$a0, d1
pub fn i_076be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BEC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076BEC: jsr $236e.l
pub fn i_076bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76BF2);
    cpu.pc = 0x236E;
    16
}
/// $076BF2: move.w #$168, d1
pub fn i_076bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BF6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076BF6: jsr $236e.l
pub fn i_076bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76BFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76BFC);
    cpu.pc = 0x236E;
    16
}
/// $076BFC: move.l #$24369e, $3c(a6)
pub fn i_076bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C04;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x24369E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $076C04: move.w #$2000, $38(a6)
pub fn i_076c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076C0A: lea.l $76cd4(pc), a1
pub fn i_076c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76CD4;
    16
}
/// $076C0E: jsr $4ae.l
pub fn i_076c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76C14);
    cpu.pc = 0x4AE;
    16
}
/// $076C14: lea.l $76c1a(pc), a1
pub fn i_076c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76C1A;
    16
}
/// $076C18: move.l a1, (a6)
pub fn i_076c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076C1A: movea.l $c(a6), a0
pub fn i_076c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C1E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $076C1E: tst.b $7a(a0)
pub fn i_076c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C22;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $076C22: beq.w $76c3c
pub fn i_076c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C26;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x76C3C; return 10; }
    8
}
/// $076C26: jsr $5b6.l
pub fn i_076c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76C2C);
    cpu.pc = 0x5B6;
    16
}
/// $076C2C: jsr $13600.l
pub fn i_076c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76C32);
    cpu.pc = 0x13600;
    16
}
/// $076C32: lea.l $77fd6.l, a1
pub fn i_076c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C38;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $076C38: move.l a1, (a6)
pub fn i_076c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076C3A: rts
pub fn i_076c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $076C3C: move.w #$1028, d0
pub fn i_076c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C40;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076C40: move.w #$a0, d1
pub fn i_076c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C44;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076C44: jsr $750c0(pc)
pub fn i_076c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x76C48);
    cpu.pc = 0x750C0;
    16
}
/// $076C48: move.w d0, $22(a6)
pub fn i_076c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C4C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076C4C: move.w d1, $24(a6)
pub fn i_076c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C50;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076C50: movea.l $c(a6), a0
pub fn i_076c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C54;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $076C54: tst.b $7b(a0)
pub fn i_076c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C58;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $076C58: bne.w $76c66
pub fn i_076c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C5C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x76C66; return 10; }
    8
}
/// $076C5C: move.w $16(a6), $14(a6)
pub fn i_076c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C62;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076C62: bra.w $76c6c
pub fn i_076c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C66;
    cpu.ir = 0x6000;
    cpu.pc = 0x76C6C;
    10
}
/// $076C66: move.w $18(a6), $14(a6)
pub fn i_076c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C6C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
// ===== JsrAbsThunk_076c6c  ($076C6C, 8 B, thunks/jsr_abs_thunks.c)
/// $076C6C: jsr $5ca2a.l
pub fn i_076c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76C72);
    cpu.pc = 0x5CA2A;
    16
}
/// $076C72: rts
pub fn i_076c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76C74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $076CD4: move.w #$e, d1
pub fn i_076cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76CD8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076CD8: jsr $236e.l
pub fn i_076cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76CDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76CDE);
    cpu.pc = 0x236E;
    16
}
/// $076CDE: move.w #$0, $38(a6)
pub fn i_076cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76CE4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076CE4: move.b #$4, $5c(a6)
pub fn i_076ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76CEA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $076CEA: lea.l $76c74(pc), a0
pub fn i_076cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76CEE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x76C74;
    16
}
/// $076CEE: jsr $28cd4.l
pub fn i_076cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76CF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76CF4);
    cpu.pc = 0x28CD4;
    16
}
/// $076CF4: lea.l $76cfa(pc), a1
pub fn i_076cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76CF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76CFA;
    16
}
/// $076CF8: move.l a1, (a6)
pub fn i_076cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76CFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076CFA: move.w #$1028, d0
pub fn i_076cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76CFE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076CFE: move.w #$a0, d1
pub fn i_076cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76D02;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076D02: jsr $750c0(pc)
pub fn i_076d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76D06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x76D06);
    cpu.pc = 0x750C0;
    16
}
/// $076D06: move.w d0, $22(a6)
pub fn i_076d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76D0A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076D0A: move.w d1, $24(a6)
pub fn i_076d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76D0E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_076d0e  ($076D0E, 8 B, thunks/jsr_abs_thunks.c)
/// $076D0E: jsr $28d70.l
pub fn i_076d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76D14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76D14);
    cpu.pc = 0x28D70;
    16
}
/// $076D14: rts
pub fn i_076d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76D16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_SpawnChildAnim_076dd4  ($076DD4, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $076DD4: lea.l $76f42(pc), a1
pub fn i_076dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76DD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76F42;
    16
}
/// $076DD8: jsr $6fe.l
pub fn i_076dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76DDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76DDE);
    cpu.pc = 0x6FE;
    16
}
/// $076DDE: jsr $5dd02.l
pub fn i_076dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76DE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76DE4);
    cpu.pc = 0x5DD02;
    16
}
/// $076DE4: subq.w #$1, $38(a0)
pub fn i_076de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76DE8;
    cpu.ir = 0x5368;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $076DE8: move.b $5c(a6), d0
pub fn i_076de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76DEC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $076DEC: addq.b #$1, d0
pub fn i_076dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76DEE;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $076DEE: andi.w #$3, d0
pub fn i_076dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76DF2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076DF2: cmpi.w #$3, d0
pub fn i_076df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76DF6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $076DF6: blt.w $76dfc
pub fn i_076df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76DFA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x76DFC; return 10; }
    8
}
/// $076DFA: clr.w d0
pub fn i_076dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76DFC;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $076DFC: move.w d0, d1
pub fn i_076dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76DFE;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076DFE: move.b d0, $5c(a6)
pub fn i_076dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E02;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $076E02: add.w d1, d1
pub fn i_076e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E04;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $076E04: addi.w #$16, d1
pub fn i_076e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E08;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x16, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E08: move.w (a6, d1.w), $14(a6)
pub fn i_076e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E0E;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $076E0E: rts
pub fn i_076e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_Launch_076e10  ($076E10, 240 B, props/fx_explosions_platforms_crew_076000.s)
/// $076E10: move.w #$10ea, d0
pub fn i_076e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E14;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10EA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076E14: jsr $2352.l
pub fn i_076e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E1A);
    cpu.pc = 0x2352;
    16
}
/// $076E1A: bset.b #$4, $6b(a6)
pub fn i_076e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E20;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $076E20: jsr $76f78(pc)
pub fn i_076e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E24;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x76E24);
    cpu.pc = 0x76F78;
    16
}
/// $076E24: movea.l $7c(a6), a0
pub fn i_076e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E28;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $076E28: jsr $5e086.l
pub fn i_076e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E2E);
    cpu.pc = 0x5E086;
    16
}
/// $076E2E: bcc.w $76e54
pub fn i_076e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76E54; return 10; }
    8
}
/// $076E32: move.w #$100, d0
pub fn i_076e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E36;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076E36: jsr $5e9e4.l
pub fn i_076e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E3C);
    cpu.pc = 0x5E9E4;
    16
}
/// $076E3C: move.w d0, -(a7)
pub fn i_076e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E3E;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $076E3E: move.w #$10, d0
pub fn i_076e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E42;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076E42: jsr $5e9e4.l
pub fn i_076e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E48);
    cpu.pc = 0x5E9E4;
    16
}
/// $076E48: addi.w #$10, d0
pub fn i_076e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E4C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E4C: move.w d0, d1
pub fn i_076e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E4E;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076E4E: move.w (a7)+, d0
pub fn i_076e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E50;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $076E50: bra.w $76eae
pub fn i_076e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E54;
    cpu.ir = 0x6000;
    cpu.pc = 0x76EAE;
    10
}
/// $076E54: move.w $22(a0), d0
pub fn i_076e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E58;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076E58: sub.w $22(a6), d0
pub fn i_076e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E5C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $076E5C: move.w $24(a0), d1
pub fn i_076e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E60;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076E60: addi.w #$10, d1
pub fn i_076e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E64;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E64: sub.w $24(a6), d1
pub fn i_076e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E68;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $076E68: movem.w d0-d1, -(a7)
pub fn i_076e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E6C;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $076E6C: jsr $5e9b6.l
pub fn i_076e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E72);
    cpu.pc = 0x5E9B6;
    16
}
/// $076E72: andi.w #$3, d0
pub fn i_076e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E76;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E76: bne.w $76e82
pub fn i_076e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E7A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x76E82; return 10; }
    8
}
/// $076E7A: clr.w d2
pub fn i_076e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E7C;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $076E7C: clr.w d3
pub fn i_076e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E7E;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $076E7E: bra.w $76ea0
pub fn i_076e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E82;
    cpu.ir = 0x6000;
    cpu.pc = 0x76EA0;
    10
}
/// $076E82: jsr $5e9b6.l
pub fn i_076e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E88);
    cpu.pc = 0x5E9B6;
    16
}
/// $076E88: andi.w #$7f, d0
pub fn i_076e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E8C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E8C: subi.w #$3f, d0
pub fn i_076e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E90;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x3F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E90: move.w d0, -(a7)
pub fn i_076e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E92;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $076E92: jsr $5e9b6.l
pub fn i_076e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76E98);
    cpu.pc = 0x5E9B6;
    16
}
/// $076E98: andi.w #$f, d0
pub fn i_076e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E9C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076E9C: move.w d0, d3
pub fn i_076e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76E9E;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076E9E: move.w (a7)+, d2
pub fn i_076e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EA0;
    cpu.ir = 0x341F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $076EA0: movem.w (a7)+, d0-d1
pub fn i_076ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EA4;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $076EA4: add.w d2, d0
pub fn i_076ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EA6;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $076EA6: add.w d3, d1
pub fn i_076ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EA8;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $076EA8: bpl.w $76eae
pub fn i_076ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EAC;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x76EAE; return 10; }
    8
}
/// $076EAC: clr.w d1
pub fn i_076eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EAE;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $076EAE: movem.w d0-d1, -(a7)
pub fn i_076eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EB2;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $076EB2: lea.l $2bf310.l, a0
pub fn i_076eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BF310;
    20
}
/// $076EB8: jsr $799de.l
pub fn i_076eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76EBE);
    cpu.pc = 0x799DE;
    16
}
/// $076EBE: move.w d0, d2
pub fn i_076ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EC0;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076EC0: movem.w (a7)+, d0-d1
pub fn i_076ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EC4;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $076EC4: bsr.w $76f9e
pub fn i_076ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EC8;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x76EC8);
    cpu.pc = 0x76F9E;
    18
}
/// $076EC8: move.w d0, $28(a6)
pub fn i_076ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ECC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076ECC: move.w d1, $2a(a6)
pub fn i_076ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ED0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076ED0: move.w d2, $2e(a6)
pub fn i_076ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ED4;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $076ED4: move.w #$d000, d0
pub fn i_076ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76ED8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076ED8: jsr $28134.l
pub fn i_076ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76EDE);
    cpu.pc = 0x28134;
    16
}
/// $076EDE: andi.w #$ffe3, $38(a6)
pub fn i_076ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EE4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $076EE4: ori.w #$14, $38(a6)
pub fn i_076ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EEA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $076EEA: lea.l $76d6a(pc), a0
pub fn i_076eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EEE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x76D6A;
    16
}
/// $076EEE: jsr $28cd4.l
pub fn i_076eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76EF4);
    cpu.pc = 0x28CD4;
    16
}
/// $076EF4: lea.l $76efa(pc), a1
pub fn i_076ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76EFA;
    16
}
/// $076EF8: move.l a1, (a6)
pub fn i_076ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76EFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076EFA: jsr $2783a.l
pub fn i_076efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F00);
    cpu.pc = 0x2783A;
    16
}
// ===== Frag_Fly_076f00  ($076F00, 66 B, props/fx_explosions_platforms_crew_076000.s)
/// $076F00: move.w $2e(a6), d0
pub fn i_076f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F04;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076F04: add.w d0, $2a(a6)
pub fn i_076f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F08;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $076F08: jsr $9c072.l
pub fn i_076f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F0E);
    cpu.pc = 0x9C072;
    16
}
/// $076F0E: jsr $28d70.l
pub fn i_076f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F14);
    cpu.pc = 0x28D70;
    16
}
/// $076F14: jsr $283d8.l
pub fn i_076f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F1A);
    cpu.pc = 0x283D8;
    16
}
/// $076F1A: cmpi.w #$140, $22(a6)
pub fn i_076f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F20;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $076F20: bcs.w $76f2a
pub fn i_076f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F24;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x76F2A; return 10; }
    8
}
/// $076F24: jmp $518.l
pub fn i_076f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F2A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $076F2A: move.w $24(a6), d0
pub fn i_076f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F2E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076F2E: subi.w #$100, d0
pub fn i_076f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F32;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076F32: cmpi.w #$140, d0
pub fn i_076f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F36;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x140, Size::Word, false);
    8
}
/// $076F36: bcs.w $76f40
pub fn i_076f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F3A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x76F40; return 10; }
    8
}
/// $076F3A: jmp $518.l
pub fn i_076f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F40;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $076F40: rts
pub fn i_076f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_Child_Run_076f42  ($076F42, 54 B, props/fx_explosions_platforms_crew_076000.s)
/// $076F42: jsr $76f78(pc)
pub fn i_076f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x76F46);
    cpu.pc = 0x76F78;
    16
}
/// $076F46: lea.l $76d96(pc), a0
pub fn i_076f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F4A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x76D96;
    16
}
/// $076F4A: jsr $28cd4.l
pub fn i_076f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F50);
    cpu.pc = 0x28CD4;
    16
}
/// $076F50: lea.l $76f56(pc), a1
pub fn i_076f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x76F56;
    16
}
/// $076F54: move.l a1, (a6)
pub fn i_076f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $076F56: jsr $2783a.l
pub fn i_076f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F5C);
    cpu.pc = 0x2783A;
    16
}
/// $076F5C: movea.l $c(a6), a0
pub fn i_076f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F60;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $076F60: move.w $14(a0), $14(a6)
pub fn i_076f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F66;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076F66: jsr $28d70.l
pub fn i_076f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F6C);
    cpu.pc = 0x28D70;
    16
}
/// $076F6C: bcc.w $76f76
pub fn i_076f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76F76; return 10; }
    8
}
/// $076F70: jmp $518.l
pub fn i_076f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F76;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $076F76: rts
pub fn i_076f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_PlayHitSounds_076f78  ($076F78, 38 B, props/fx_explosions_platforms_crew_076000.s)
/// $076F78: move.w #$1e1, d1
pub fn i_076f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F7C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076F7C: jsr $236e.l
pub fn i_076f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F82);
    cpu.pc = 0x236E;
    16
}
/// $076F82: move.w #$1e2, d1
pub fn i_076f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F86;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076F86: jsr $236e.l
pub fn i_076f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F8C);
    cpu.pc = 0x236E;
    16
}
/// $076F8C: move.w #$1e3, d1
pub fn i_076f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F90;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076F90: jsr $236e.l
pub fn i_076f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x76F96);
    cpu.pc = 0x236E;
    16
}
/// $076F96: move.w $16(a6), $14(a6)
pub fn i_076f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F9C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $076F9C: rts
pub fn i_076f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76F9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Frag_VelFromScale_076f9e  ($076F9E, 48 B, props/fx_explosions_platforms_crew_076000.s)
/// $076F9E: cmpi.w #$2, d2
pub fn i_076f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FA2;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $076FA2: bcc.w $76faa
pub fn i_076fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FA6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x76FAA; return 10; }
    8
}
/// $076FA6: move.w #$2, d2
pub fn i_076fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FAA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076FAA: move.l #$10000, d4
pub fn i_076faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FB0;
    cpu.ir = 0x283C;
    let v1: u32 = 0x10000;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $076FB0: divu.w d2, d4
pub fn i_076fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FB2;
    cpu.ir = 0x88C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[4] / dv; let rm = cpu.d[4] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[4] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $076FB2: muls.w d4, d0
pub fn i_076fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FB4;
    cpu.ir = 0xC1C4;
    let v1 = cpu.d[4] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $076FB4: asr.l #$8, d0
pub fn i_076fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FB6;
    cpu.ir = 0xE080;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[0] = r;
    22
}
/// $076FB6: move.w #$200, d2
pub fn i_076fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FBA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $076FBA: mulu.w d4, d2
pub fn i_076fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FBC;
    cpu.ir = 0xC4C4;
    let v1 = cpu.d[4] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[2] & 0xFFFF);
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $076FBC: lsr.l #$8, d2
pub fn i_076fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FBE;
    cpu.ir = 0xE08A;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[2] = r;
    22
}
/// $076FBE: mulu.w d4, d2
pub fn i_076fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FC0;
    cpu.ir = 0xC4C4;
    let v1 = cpu.d[4] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[2] & 0xFFFF);
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $076FC0: swap d2
pub fn i_076fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FC2;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $076FC2: neg.w d2
pub fn i_076fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FC4;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $076FC4: addi.w #$100, d1
pub fn i_076fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FC8;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $076FC8: mulu.w d4, d1
pub fn i_076fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FCA;
    cpu.ir = 0xC2C4;
    let v1 = cpu.d[4] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $076FCA: lsr.l #$8, d1
pub fn i_076fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FCC;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $076FCC: rts
pub fn i_076fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_RetGate_076fce  ($076FCE, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $076FCE: movea.l $8(a6), a1
pub fn i_076fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FD2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $076FD2: move.b $10(a6), d0
pub fn i_076fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FD6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $076FD6: cmp.b $10(a1), d0
pub fn i_076fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FDA;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $076FDA: bcs.w $76fe4
pub fn i_076fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FDE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x76FE4; return 10; }
    8
}
// ===== ClearXN_076fde  ($076FDE, 6 B, thunks/ccr_helpers.c)
/// $076FDE: andi.b #$ee, ccr
pub fn i_076fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FE2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $076FE2: rts
pub fn i_076fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_076fe4  ($076FE4, 6 B, thunks/ccr_helpers.c)
/// $076FE4: ori.b #$11, ccr
pub fn i_076fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FE8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $076FE8: rts
pub fn i_076fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_Register_076fea  ($076FEA, 26 B, props/fx_explosions_platforms_crew_076000.s)
/// $076FEA: move.w $30(a0), d0
pub fn i_076fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FEE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $076FEE: cmpi.w #$30, d0
pub fn i_076fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FF2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $076FF2: bcc.w $7700a
pub fn i_076ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7700A; return 10; }
    8
}
/// $076FF6: lea.l (a0, d0.w), a1
pub fn i_076ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FFA;
    cpu.ir = 0x43F0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $076FFA: lea.l $70(a6), a2
pub fn i_076ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x76FFE;
    cpu.ir = 0x45EE;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    cpu.a[2] = ad1;
    16
}
/// $076FFE: move.w (a2)+, (a1)+
pub fn i_076ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77000;
    cpu.ir = 0x32DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $077000: move.w (a2)+, (a1)+
pub fn i_077000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77002;
    cpu.ir = 0x32DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $077002: move.w (a2)+, (a1)+
pub fn i_077002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77004;
    cpu.ir = 0x32DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
// ===== Platform_AdvanceCount_077004  ($077004, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $077004: addq.w #$6, d0
pub fn i_077004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77006;
    cpu.ir = 0x5C40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077006: move.w d0, $30(a0)
pub fn i_077006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7700A;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07700A: rts
pub fn i_07700a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7700C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_FindNearestX_07700c  ($07700C, 112 B, props/fx_explosions_platforms_crew_076000.s)
/// $07700C: clr.w d7
pub fn i_07700c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7700E;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07700E: moveq #$ff, d6
pub fn i_07700e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77010;
    cpu.ir = 0x7CFF;
    cpu.d[6] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $077010: move.w d6, d5
pub fn i_077010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77012;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077012: lea.l (a0), a5
pub fn i_077012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77014;
    cpu.ir = 0x4BD0;
    cpu.a[5] = cpu.a[0];
    12
}
/// $077014: cmp.w $30(a0), d7
pub fn i_077014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77018;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $077018: bcc.w $77054
pub fn i_077018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7701C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77054; return 10; }
    8
}
/// $07701C: move.w $2(a5, d7.w), d2
pub fn i_07701c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77020;
    cpu.ir = 0x3435;
    let ad2 = cpu.a[5].wrapping_add(0x2).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $077020: sub.w d1, d2
pub fn i_077020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77022;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077022: clr.w d3
pub fn i_077022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77024;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077024: move.b $4(a5, d7.w), d3
pub fn i_077024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77028;
    cpu.ir = 0x1635;
    let ad2 = cpu.a[5].wrapping_add(0x4).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    14
}
/// $077028: cmp.w d3, d2
pub fn i_077028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7702A;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $07702A: bhi.w $77050
pub fn i_07702a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7702E;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x77050; return 10; }
    8
}
/// $07702E: lsr.w #$3, d2
pub fn i_07702e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77030;
    cpu.ir = 0xE64A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $077030: neg.w d2
pub fn i_077030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77032;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077032: add.w (a5, d7.w), d2
pub fn i_077032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77036;
    cpu.ir = 0xD475;
    let ad2 = cpu.a[5].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $077036: move.w d2, d3
pub fn i_077036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77038;
    cpu.ir = 0x3602;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077038: sub.w d0, d3
pub fn i_077038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7703A;
    cpu.ir = 0x9640;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07703A: smi.b d4
pub fn i_07703a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7703C;
    cpu.ir = 0x5BC4;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[4] & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (c1 & 0xFF);
    4
}
/// $07703C: ext.w d4
pub fn i_07703c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7703E;
    cpu.ir = 0x4884;
    let v = cpu.d[4] as i8 as i16 as u16 as u32; cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $07703E: eor.w d4, d3
pub fn i_07703e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77040;
    cpu.ir = 0xB943;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[4]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077040: sub.w d4, d3
pub fn i_077040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77042;
    cpu.ir = 0x9644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077042: cmp.w d3, d5
pub fn i_077042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77044;
    cpu.ir = 0xBA43;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $077044: bls.w $77050
pub fn i_077044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77048;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x77050; return 10; }
    8
}
/// $077048: move.w d3, d5
pub fn i_077048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7704A;
    cpu.ir = 0x3A03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07704A: move.w d2, d6
pub fn i_07704a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7704C;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07704C: lea.l (a5, d7.w), a1
pub fn i_07704c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77050;
    cpu.ir = 0x43F5;
    let ad1 = cpu.a[5].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $077050: addq.w #$6, d7
pub fn i_077050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77052;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077052: bra.b $77014
pub fn i_077052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77054;
    cpu.ir = 0x60C0;
    cpu.pc = 0x77014;
    10
}
/// $077054: rts
pub fn i_077054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77056;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $077056: moveq #$0, d7
pub fn i_077056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77058;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $077058: movea.l d7, a1
pub fn i_077058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7705A;
    cpu.ir = 0x2247;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $07705A: moveq #$ff, d6
pub fn i_07705a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7705C;
    cpu.ir = 0x7CFF;
    cpu.d[6] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07705C: lea.l (a0), a5
pub fn i_07705c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7705E;
    cpu.ir = 0x4BD0;
    cpu.a[5] = cpu.a[0];
    12
}
/// $07705E: cmp.w $30(a0), d7
pub fn i_07705e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77062;
    cpu.ir = 0xBE68;
    let ad2 = cpu.a[0].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $077062: bcc.w $7707a
pub fn i_077062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77066;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7707A; return 10; }
    8
}
/// $077066: cmp.b $5(a5, d7.w), d0
pub fn i_077066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7706A;
    cpu.ir = 0xB035;
    let ad2 = cpu.a[5].wrapping_add(0x5).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    14
}
/// $07706A: bne.w $77076
pub fn i_07706a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7706E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77076; return 10; }
    8
}
/// $07706E: lea.l (a5, d7.w), a1
pub fn i_07706e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77072;
    cpu.ir = 0x43F5;
    let ad1 = cpu.a[5].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $077072: bra.w $7707a
pub fn i_077072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77076;
    cpu.ir = 0x6000;
    cpu.pc = 0x7707A;
    10
}
/// $077076: addq.w #$6, d7
pub fn i_077076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77078;
    cpu.ir = 0x5C47;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077078: bra.b $7705e
pub fn i_077078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7707A;
    cpu.ir = 0x60E4;
    cpu.pc = 0x7705E;
    10
}
/// $07707A: rts
pub fn i_07707a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7707C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_ListsInit_07707c  ($07707C, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $07707C: lea.l $10e27c.l, a0
pub fn i_07707c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77082;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $077082: clr.w $30(a0)
pub fn i_077082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77086;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $077086: lea.l $10e2ae.l, a0
pub fn i_077086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7708C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $07708C: clr.w $30(a0)
pub fn i_07708c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77090;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $077090: rts
pub fn i_077090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77092;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_ClearListByScroll_077092  ($077092, 58 B, props/fx_explosions_platforms_crew_076000.s)
/// $077092: lea.l $10e27c.l, a0
pub fn i_077092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77098;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $077098: btst.b #$0, $106f28.l
pub fn i_077098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770A0;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0770A0: beq.w $770aa
pub fn i_0770a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x770AA; return 10; }
    8
}
/// $0770A4: lea.l $10e2ae.l, a0
pub fn i_0770a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $0770AA: clr.w $30(a0)
pub fn i_0770aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770AE;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0770AE: rts
pub fn i_0770ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0770B0: lea.l $10e27c.l, a0
pub fn i_0770b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $0770B6: btst.b #$0, $106f28.l
pub fn i_0770b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770BE;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0770BE: beq.w $770c8
pub fn i_0770be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770C2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x770C8; return 10; }
    8
}
/// $0770C2: lea.l $10e2ae.l, a0
pub fn i_0770c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $0770C8: bra.w $76fea
pub fn i_0770c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770CC;
    cpu.ir = 0x6000;
    cpu.pc = 0x76FEA;
    10
}
// ===== Platform_ProbeUnderFeet_0770cc  ($0770CC, 120 B, props/fx_explosions_platforms_crew_076000.s)
/// $0770CC: lea.l $10e27c.l, a0
pub fn i_0770cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $0770D2: btst.b #$0, $106f28.l
pub fn i_0770d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770DA;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0770DA: bne.w $770e4
pub fn i_0770da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x770E4; return 10; }
    8
}
/// $0770DE: lea.l $10e2ae.l, a0
pub fn i_0770de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $0770E4: move.w $22(a6), d0
pub fn i_0770e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770E8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0770E8: move.w $24(a6), d1
pub fn i_0770e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770EC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0770EC: add.w $106f50.l, d0
pub fn i_0770ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770F2;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0770F2: subi.w #$200, d1
pub fn i_0770f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770F6;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0770F6: neg.w d1
pub fn i_0770f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770F8;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0770F8: add.w $106f54.l, d1
pub fn i_0770f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x770FE;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0770FE: bsr.w $7700c
pub fn i_0770fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77102;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x77102);
    cpu.pc = 0x7700C;
    18
}
/// $077102: addq.w #$1, d5
pub fn i_077102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77104;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077104: beq.w $77144
pub fn i_077104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77108;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77144; return 10; }
    8
}
/// $077108: clr.w d2
pub fn i_077108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7710A;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07710A: move.b $26(a6), d2
pub fn i_07710a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7710E;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x26);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07710E: add.w $28(a6), d2
pub fn i_07710e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77112;
    cpu.ir = 0xD46E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $077112: asr.w #$8, d2
pub fn i_077112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77114;
    cpu.ir = 0xE042;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $077114: add.w d0, d2
pub fn i_077114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77116;
    cpu.ir = 0xD440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077116: sub.w d6, d2
pub fn i_077116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77118;
    cpu.ir = 0x9446;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077118: smi.b d2
pub fn i_077118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7711A;
    cpu.ir = 0x5BC2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $07711A: sub.w d6, d0
pub fn i_07711a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7711C;
    cpu.ir = 0x9046;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07711C: smi.b d0
pub fn i_07711c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7711E;
    cpu.ir = 0x5BC0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $07711E: eor.b d2, d0
pub fn i_07711e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77120;
    cpu.ir = 0xB500;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $077120: beq.w $77144
pub fn i_077120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77124;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77144; return 10; }
    8
}
/// $077124: clr.w d4
pub fn i_077124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77126;
    cpu.ir = 0x4244;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077126: clr.w d2
pub fn i_077126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77128;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077128: move.b $4(a1), d2
pub fn i_077128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7712C;
    cpu.ir = 0x1429;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07712C: lsr.w #$1, d2
pub fn i_07712c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7712E;
    cpu.ir = 0xE24A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07712E: neg.w d2
pub fn i_07712e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77130;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077130: add.w $2(a1), d2
pub fn i_077130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77134;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $077134: cmp.w d2, d1
pub fn i_077134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77136;
    cpu.ir = 0xB242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $077136: spl.b d1
pub fn i_077136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77138;
    cpu.ir = 0x5AC1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $077138: addi.b #$2, d1
pub fn i_077138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7713C;
    cpu.ir = 0x601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $07713C: clr.w d0
pub fn i_07713c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7713E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07713E: move.b $5(a1), d0
pub fn i_07713e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77142;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $077142: rts
pub fn i_077142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77144;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RetMinus1_00077144  ($077144, 4 B, thunks/trivial_returns.c)
/// $077144: moveq #$ff, d0
pub fn i_077144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77146;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $077146: rts
pub fn i_077146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77148;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_ProbeNearest_077148  ($077148, 72 B, props/fx_explosions_platforms_crew_076000.s)
/// $077148: lea.l $10e27c.l, a0
pub fn i_077148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7714E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $07714E: btst.b #$0, $106f28.l
pub fn i_07714e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77156;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $077156: bne.w $77160
pub fn i_077156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7715A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77160; return 10; }
    8
}
/// $07715A: lea.l $10e2ae.l, a0
pub fn i_07715a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77160;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $077160: move.w $22(a6), d0
pub fn i_077160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77164;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077164: move.w $24(a6), d1
pub fn i_077164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77168;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077168: add.w $106f50.l, d0
pub fn i_077168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7716E;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $07716E: subi.w #$200, d1
pub fn i_07716e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77172;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $077172: neg.w d1
pub fn i_077172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77174;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077174: add.w $106f54.l, d1
pub fn i_077174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7717A;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $07717A: bsr.w $7700c
pub fn i_07717a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7717E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x7717E);
    cpu.pc = 0x7700C;
    18
}
/// $07717E: addq.w #$1, d5
pub fn i_07717e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77180;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077180: beq.b $77144
pub fn i_077180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77182;
    cpu.ir = 0x67C2;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77144; return 10; }
    8
}
/// $077182: sub.w $106f50.l, d6
pub fn i_077182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77188;
    cpu.ir = 0x9C79;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $077188: clr.w d0
pub fn i_077188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7718A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07718A: move.b $5(a1), d0
pub fn i_07718a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7718E;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07718E: rts
pub fn i_07718e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77190;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Platform_SnapToId_077190  ($077190, 148 B, props/fx_explosions_platforms_crew_076000.s)
/// $077190: lea.l $10e27c.l, a0
pub fn i_077190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77196;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E27C;
    20
}
/// $077196: btst.b #$0, $106f28.l
pub fn i_077196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7719E;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $07719E: bne.w $771a8
pub fn i_07719e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x771A8; return 10; }
    8
}
/// $0771A2: lea.l $10e2ae.l, a0
pub fn i_0771a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2AE;
    20
}
/// $0771A8: bsr.w $77056
pub fn i_0771a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771AC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x771AC);
    cpu.pc = 0x77056;
    18
}
/// $0771AC: move.l a1, d0
pub fn i_0771ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771AE;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0771AE: beq.w $7721a
pub fn i_0771ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7721A; return 10; }
    8
}
/// $0771B2: clr.w -(a7)
pub fn i_0771b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771B4;
    cpu.ir = 0x4267;
    let ad1 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad1;
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    10
}
/// $0771B4: move.w $2(a1), d0
pub fn i_0771b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771B8;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0771B8: sub.w $106f54.l, d0
pub fn i_0771b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771BE;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0771BE: neg.w d0
pub fn i_0771be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771C0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0771C0: addi.w #$200, d0
pub fn i_0771c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771C4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0771C4: clr.w d1
pub fn i_0771c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771C6;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0771C6: move.b $4(a1), d1
pub fn i_0771c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771CA;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0771CA: add.w d0, d1
pub fn i_0771ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771CC;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0771CC: addq.w #$1, d1
pub fn i_0771cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771CE;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0771CE: move.w d1, (a7)
pub fn i_0771ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771D0;
    cpu.ir = 0x3E81;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[7], v1 as u16);
    8
}
/// $0771D0: clr.w d1
pub fn i_0771d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771D2;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0771D2: move.b $27(a6), d1
pub fn i_0771d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771D6;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x27);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0771D6: add.w $2a(a6), d1
pub fn i_0771d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771DA;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0771DA: add.w $2e(a6), d1
pub fn i_0771da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771DE;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0771DE: asr.w #$8, d1
pub fn i_0771de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771E0;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $0771E0: add.w $24(a6), d1
pub fn i_0771e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771E4;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0771E4: sub.w d0, d1
pub fn i_0771e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771E6;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0771E6: cmpi.w #$fff8, d1
pub fn i_0771e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771EA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFF8, Size::Word, false);
    8
}
/// $0771EA: bge.w $771f2
pub fn i_0771ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771EE;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x771F2; return 10; }
    8
}
/// $0771EE: move.w #$ffff, (a7)
pub fn i_0771ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771F2;
    cpu.ir = 0x3EBC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[7], v1 as u16);
    8
}
/// $0771F2: neg.w d1
pub fn i_0771f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771F4;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0771F4: asr.w #$3, d1
pub fn i_0771f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771F6;
    cpu.ir = 0xE641;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0771F6: add.w (a1), d1
pub fn i_0771f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771F8;
    cpu.ir = 0xD251;
    let v2 = bus.read16(cpu.a[1]) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0771F8: sub.w $106f50.l, d1
pub fn i_0771f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x771FE;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0771FE: sub.w $22(a6), d1
pub fn i_0771fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77202;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $077202: lsl.w #$8, d1
pub fn i_077202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77204;
    cpu.ir = 0xE149;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $077204: move.w d1, $28(a6)
pub fn i_077204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77208;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077208: clr.w $2c(a6)
pub fn i_077208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7720C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07720C: jsr $27d50.l
pub fn i_07720c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77212;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77212);
    cpu.pc = 0x27D50;
    16
}
/// $077212: clr.w $28(a6)
pub fn i_077212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77216;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $077216: move.w (a7)+, d0
pub fn i_077216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77218;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $077218: rts
pub fn i_077218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7721A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07721A: jsr $27d50.l
pub fn i_07721a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77220;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77220);
    cpu.pc = 0x27D50;
    16
}
/// $077220: moveq #$ff, d0
pub fn i_077220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77222;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $077222: rts
pub fn i_077222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77224;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MovingPlatform_Tmpl143_077224  ($077224, 4 B, props/fx_explosions_platforms_crew_076000.s)
/// $077224: clr.b $99(a6)
pub fn i_077224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77228;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== MovingPlatform_Init_077228  ($077228, 58 B, props/fx_explosions_platforms_crew_076000.s)
/// $077228: move.w $22(a6), d0
pub fn i_077228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7722C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07722C: add.w $106f50.l, d0
pub fn i_07722c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77232;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $077232: move.w d0, $70(a6)
pub fn i_077232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77236;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077236: move.w $24(a6), d0
pub fn i_077236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7723A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07723A: subi.w #$200, d0
pub fn i_07723a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7723E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07723E: neg.w d0
pub fn i_07723e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77240;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077240: add.w $106f54.l, d0
pub fn i_077240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77246;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $077246: move.w d0, $72(a6)
pub fn i_077246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7724A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07724A: move.b $98(a6), d0
pub fn i_07724a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7724E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07724E: move.b d0, $74(a6)
pub fn i_07724e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77252;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $077252: lea.l $10e2e0.l, a0
pub fn i_077252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77258;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E2E0;
    20
}
/// $077258: move.b (a0), d0
pub fn i_077258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7725A;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $07725A: addq.b #$1, d0
pub fn i_07725a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7725C;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $07725C: move.b d0, (a0)
pub fn i_07725c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7725E;
    cpu.ir = 0x1080;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[0], v1 as u8);
    8
}
/// $07725E: move.b d0, $75(a6)
pub fn i_07725e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77262;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetTaskHandler_077262  ($077262, 8 B, kernel/task_handlers.c)
/// $077262: lea.l $7726a(pc), a1
pub fn i_077262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77266;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7726A;
    16
}
/// $077266: move.l a1, (a6)
pub fn i_077266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77268;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077268: rts
pub fn i_077268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7726A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MovingPlatform_Run_07726a  ($07726A, 62 B, props/fx_explosions_platforms_crew_076000.s)
/// $07726A: move.w $70(a6), d0
pub fn i_07726a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7726E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07726E: sub.w $106f50.l, d0
pub fn i_07726e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77274;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $077274: addi.w #$40, d0
pub fn i_077274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77278;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $077278: subi.w #$380, d0
pub fn i_077278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7727C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x380, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07727C: bcs.w $77286
pub fn i_07727c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77280;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x77286; return 10; }
    8
}
/// $077280: jmp $518.l
pub fn i_077280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77286;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $077286: moveq #$0, d0
pub fn i_077286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77288;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $077288: move.b $99(a6), d0
pub fn i_077288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7728C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07728C: beq.w $772a2
pub fn i_07728c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77290;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x772A2; return 10; }
    8
}
/// $077290: movea.l $c(a6), a0
pub fn i_077290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77294;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $077294: tst.b (a0, d0.w)
pub fn i_077294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77298;
    cpu.ir = 0x4A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $077298: beq.w $772a2
pub fn i_077298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7729C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x772A2; return 10; }
    8
}
/// $07729C: jmp $518.l
pub fn i_07729c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x772A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0772A2: bsr.w $770b0
pub fn i_0772a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x772A6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x772A6);
    cpu.pc = 0x770B0;
    18
}
/// $0772A6: rts
pub fn i_0772a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x772A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BridgePlank_PosFromRecord_07749e  ($07749E, 78 B, props/fx_explosions_platforms_crew_076000.s)
/// $07749E: movea.l $70(a6), a0
pub fn i_07749e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774A2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0774A2: move.w $8(a0), d0
pub fn i_0774a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774A6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0774A6: move.w $a(a0), d1
pub fn i_0774a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774AA;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0774AA: lsl.w #$4, d0
pub fn i_0774aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774AC;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0774AC: lsl.w #$4, d1
pub fn i_0774ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774AE;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0774AE: jsr $44022.l
pub fn i_0774ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x774B4);
    cpu.pc = 0x44022;
    16
}
/// $0774B4: move.w d0, $22(a6)
pub fn i_0774b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774B8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0774B8: move.w d1, $24(a6)
pub fn i_0774b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774BC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0774BC: rts
pub fn i_0774bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0774BE: lea.l $77224(pc), a1
pub fn i_0774be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77224;
    16
}
/// $0774C2: jsr $4ae.l
pub fn i_0774c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x774C8);
    cpu.pc = 0x4AE;
    16
}
/// $0774C8: move.w $22(a6), d0
pub fn i_0774c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774CC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0774CC: move.w $24(a6), d1
pub fn i_0774cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774D0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0774D0: addi.w #$20, d0
pub fn i_0774d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774D4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0774D4: addi.w #$10, d0
pub fn i_0774d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774D8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0774D8: subi.w #$80, d1
pub fn i_0774d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774DC;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0774DC: move.w d0, $22(a0)
pub fn i_0774dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774E0;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0774E0: move.w d1, $24(a0)
pub fn i_0774e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774E4;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0774E4: move.b #$80, $98(a0)
pub fn i_0774e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774EA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0774EA: rts
pub fn i_0774ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BridgePlank_Spawn0_0774ec  ($0774EC, 20 B, props/fx_explosions_platforms_crew_076000.s)
/// $0774EC: move.l #$7730a, $70(a6)
pub fn i_0774ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774F4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x7730A;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0774F4: bsr.b $7749e
pub fn i_0774f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774F6;
    cpu.ir = 0x61A8;
    cpu.push32(bus, 0x774F6);
    cpu.pc = 0x7749E;
    18
}
/// $0774F6: subi.w #$10, $24(a6)
pub fn i_0774f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x774FC;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0774FC: bra.w $7757c
pub fn i_0774fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77500;
    cpu.ir = 0x6000;
    cpu.pc = 0x7757C;
    10
}
// ===== BridgePlank_Spawn1_077500  ($077500, 20 B, props/fx_explosions_platforms_crew_076000.s)
/// $077500: move.l #$77332, $70(a6)
pub fn i_077500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77508;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x77332;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077508: bsr.b $7749e
pub fn i_077508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7750A;
    cpu.ir = 0x6194;
    cpu.push32(bus, 0x7750A);
    cpu.pc = 0x7749E;
    18
}
/// $07750A: subi.w #$20, $24(a6)
pub fn i_07750a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77510;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $077510: bra.w $7757c
pub fn i_077510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77514;
    cpu.ir = 0x6000;
    cpu.pc = 0x7757C;
    10
}
// ===== BridgePlank_Spawn2_077514  ($077514, 20 B, props/fx_explosions_platforms_crew_076000.s)
/// $077514: move.l #$7735a, $70(a6)
pub fn i_077514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7751C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x7735A;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07751C: bsr.b $7749e
pub fn i_07751c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7751E;
    cpu.ir = 0x6180;
    cpu.push32(bus, 0x7751E);
    cpu.pc = 0x7749E;
    18
}
/// $07751E: subi.w #$20, $24(a6)
pub fn i_07751e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77524;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $077524: bra.w $7757c
pub fn i_077524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77528;
    cpu.ir = 0x6000;
    cpu.pc = 0x7757C;
    10
}
// ===== BridgePlank_Spawn3_077528  ($077528, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $077528: move.l #$77382, $70(a6)
pub fn i_077528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77530;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x77382;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077530: bsr.w $7749e
pub fn i_077530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77534;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x77534);
    cpu.pc = 0x7749E;
    18
}
/// $077534: subi.w #$20, $24(a6)
pub fn i_077534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7753A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07753A: bra.w $7757c
pub fn i_07753a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7753E;
    cpu.ir = 0x6000;
    cpu.pc = 0x7757C;
    10
}
// ===== BridgePlank_Spawn4_07753e  ($07753E, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $07753E: move.l #$773aa, $70(a6)
pub fn i_07753e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77546;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x773AA;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077546: bsr.w $7749e
pub fn i_077546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7754A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x7754A);
    cpu.pc = 0x7749E;
    18
}
/// $07754A: subi.w #$10, $24(a6)
pub fn i_07754a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77550;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $077550: bra.w $7757c
pub fn i_077550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77554;
    cpu.ir = 0x6000;
    cpu.pc = 0x7757C;
    10
}
// ===== BridgePlank_Spawn5_077554  ($077554, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $077554: move.l #$773d2, $70(a6)
pub fn i_077554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7755C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x773D2;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07755C: bsr.w $7749e
pub fn i_07755c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77560;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x77560);
    cpu.pc = 0x7749E;
    18
}
/// $077560: subi.w #$20, $24(a6)
pub fn i_077560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77566;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $077566: bra.w $7757c
pub fn i_077566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7756A;
    cpu.ir = 0x6000;
    cpu.pc = 0x7757C;
    10
}
// ===== BridgePlank_Spawn6_07756a  ($07756A, 66 B, props/fx_explosions_platforms_crew_076000.s)
/// $07756A: move.l #$773fa, $70(a6)
pub fn i_07756a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77572;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x773FA;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077572: bsr.w $7749e
pub fn i_077572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77576;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x77576);
    cpu.pc = 0x7749E;
    18
}
/// $077576: subi.w #$20, $24(a6)
pub fn i_077576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7757C;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07757C: lea.l $ffff.w, a0
pub fn i_07757c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77580;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $077580: move.l a0, $48(a6)
pub fn i_077580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77584;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077584: move.w #$3c, $66(a6)
pub fn i_077584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7758A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07758A: lea.l $77590(pc), a1
pub fn i_07758a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7758E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77590;
    16
}
/// $07758E: move.l a1, (a6)
pub fn i_07758e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77590;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077590: jsr $2783a.l
pub fn i_077590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77596);
    cpu.pc = 0x2783A;
    16
}
/// $077596: move.w $22(a6), d0
pub fn i_077596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7759A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07759A: bmi.w $775a0
pub fn i_07759a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7759E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x775A0; return 10; }
    8
}
/// $07759E: rts
pub fn i_07759e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0775A0: jsr $5b6.l
pub fn i_0775a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x775A6);
    cpu.pc = 0x5B6;
    16
}
/// $0775A6: jmp $518.l
pub fn i_0775a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775AC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== MuzzleFx_RetGate_0775ac  ($0775AC, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $0775AC: movea.l $8(a6), a1
pub fn i_0775ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775B0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0775B0: move.b $10(a6), d0
pub fn i_0775b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775B4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0775B4: cmp.b $10(a1), d0
pub fn i_0775b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775B8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0775B8: bcs.w $775c2
pub fn i_0775b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775BC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x775C2; return 10; }
    8
}
// ===== ClearXN_0775bc  ($0775BC, 6 B, thunks/ccr_helpers.c)
/// $0775BC: andi.b #$ee, ccr
pub fn i_0775bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775C0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0775C0: rts
pub fn i_0775c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0775c2  ($0775C2, 6 B, thunks/ccr_helpers.c)
/// $0775C2: ori.b #$11, ccr
pub fn i_0775c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775C6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0775C6: rts
pub fn i_0775c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SpawnC_0775c8  ($0775C8, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $0775C8: lea.l $776e2(pc), a1
pub fn i_0775c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x776E2;
    16
}
/// $0775CC: jsr $4ae.l
pub fn i_0775cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x775D2);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_0775d2  ($0775D2, 8 B, thunks/jsr_abs_thunks.c)
/// $0775D2: jsr $5dd02.l
pub fn i_0775d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x775D8);
    cpu.pc = 0x5DD02;
    16
}
/// $0775D8: rts
pub fn i_0775d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SpawnB_0775da  ($0775DA, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $0775DA: lea.l $776b4(pc), a1
pub fn i_0775da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x776B4;
    16
}
/// $0775DE: jsr $4ae.l
pub fn i_0775de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x775E4);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_0775e4  ($0775E4, 8 B, thunks/jsr_abs_thunks.c)
/// $0775E4: jsr $5dd02.l
pub fn i_0775e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x775EA);
    cpu.pc = 0x5DD02;
    16
}
/// $0775EA: rts
pub fn i_0775ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x775EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SceneGate_077640  ($077640, 48 B, props/fx_explosions_platforms_crew_076000.s)
/// $077640: cmpi.b #$0, $106ece.l
pub fn i_077640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77648;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $077648: beq.w $7765e
pub fn i_077648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7764C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7765E; return 10; }
    8
}
/// $07764C: cmpi.b #$9, $106ece.l
pub fn i_07764c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77654;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x9, Size::Byte, false);
    20
}
/// $077654: beq.w $7765e
pub fn i_077654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77658;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7765E; return 10; }
    8
}
/// $077658: jmp $518.l
pub fn i_077658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7765E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07765E: jsr $77670(pc)
pub fn i_07765e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77662;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77662);
    cpu.pc = 0x77670;
    16
}
/// $077662: jsr $236e.l
pub fn i_077662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77668;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77668);
    cpu.pc = 0x236E;
    16
}
/// $077668: move.w #$e000, $38(a6)
pub fn i_077668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7766E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07766E: rts
pub fn i_07766e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77670;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SndByScene_077670  ($077670, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $077670: move.w #$10, d1
pub fn i_077670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77674;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077674: cmpi.b #$5, $106ece.l
pub fn i_077674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7767C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x5, Size::Byte, false);
    20
}
/// $07767C: bne.w $77684
pub fn i_07767c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77680;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77684; return 10; }
    8
}
/// $077680: move.w #$138, d1
pub fn i_077680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77684;
    cpu.ir = 0x323C;
    let v1: u32 = 0x138;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077684: rts
pub fn i_077684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77686;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_InitA_077686  ($077686, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $077686: jsr $77640(pc)
pub fn i_077686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7768A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7768A);
    cpu.pc = 0x77640;
    16
}
/// $07768A: move.l #$2dcb84, $74(a6)
pub fn i_07768a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77692;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCB84;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077692: move.l #$2dca62, $78(a6)
pub fn i_077692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7769A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA62;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07769A: move.l #$2dca82, $7c(a6)
pub fn i_07769a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776A2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA82;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776A2: move.l #$2dca1a, $80(a6)
pub fn i_0776a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776AA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA1A;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776AA: lea.l $778a2(pc), a1
pub fn i_0776aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $0776AE: move.l a1, (a6)
pub fn i_0776ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0776B0: bra.w $778a2
pub fn i_0776b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x778A2;
    10
}
// ===== MuzzleFx_InitB_0776b4  ($0776B4, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $0776B4: jsr $77640(pc)
pub fn i_0776b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x776B8);
    cpu.pc = 0x77640;
    16
}
/// $0776B8: move.l #$2dca8a, $74(a6)
pub fn i_0776b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776C0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA8A;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776C0: move.l #$2dca4a, $78(a6)
pub fn i_0776c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776C8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA4A;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776C8: move.l #$2dca6a, $7c(a6)
pub fn i_0776c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776D0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA6A;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776D0: move.l #$2dc98a, $80(a6)
pub fn i_0776d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776D8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DC98A;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776D8: lea.l $778a2(pc), a1
pub fn i_0776d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $0776DC: move.l a1, (a6)
pub fn i_0776dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0776DE: bra.w $778a2
pub fn i_0776de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x778A2;
    10
}
// ===== MuzzleFx_InitC_0776e2  ($0776E2, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $0776E2: jsr $77640(pc)
pub fn i_0776e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x776E6);
    cpu.pc = 0x77640;
    16
}
/// $0776E6: move.l #$2dcabc, $74(a6)
pub fn i_0776e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776EE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCABC;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776EE: move.l #$2dca52, $78(a6)
pub fn i_0776ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776F6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA52;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776F6: move.l #$2dca72, $7c(a6)
pub fn i_0776f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x776FE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA72;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0776FE: move.l #$2dc9ba, $80(a6)
pub fn i_0776fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77706;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DC9BA;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077706: lea.l $778a2(pc), a1
pub fn i_077706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7770A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $07770A: move.l a1, (a6)
pub fn i_07770a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7770C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07770C: bra.w $778a2
pub fn i_07770c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77710;
    cpu.ir = 0x6000;
    cpu.pc = 0x778A2;
    10
}
// ===== MuzzleFx_InitD_077710  ($077710, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $077710: jsr $77640(pc)
pub fn i_077710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77714;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77714);
    cpu.pc = 0x77640;
    16
}
/// $077714: move.l #$2dcaee, $74(a6)
pub fn i_077714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7771C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCAEE;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07771C: move.l #$2dca5a, $78(a6)
pub fn i_07771c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77724;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA5A;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077724: move.l #$2dca7a, $7c(a6)
pub fn i_077724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7772C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA7A;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07772C: move.l #$2dc9ea, $80(a6)
pub fn i_07772c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77734;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DC9EA;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077734: lea.l $778a2(pc), a1
pub fn i_077734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77738;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $077738: move.l a1, (a6)
pub fn i_077738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7773A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07773A: bra.w $778a2
pub fn i_07773a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7773E;
    cpu.ir = 0x6000;
    cpu.pc = 0x778A2;
    10
}
// ===== MuzzleFx_InitE_07773e  ($07773E, 46 B, props/fx_explosions_platforms_crew_076000.s)
/// $07773E: jsr $77640(pc)
pub fn i_07773e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77742;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77742);
    cpu.pc = 0x77640;
    16
}
/// $077742: move.l #$2dcb84, $74(a6)
pub fn i_077742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7774A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCB84;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07774A: move.l #$2dca62, $78(a6)
pub fn i_07774a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77752;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA62;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077752: move.l #$2dca82, $7c(a6)
pub fn i_077752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7775A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA82;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07775A: move.l #$2dca1a, $80(a6)
pub fn i_07775a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77762;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA1A;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077762: lea.l $778a2(pc), a1
pub fn i_077762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77766;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $077766: move.l a1, (a6)
pub fn i_077766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77768;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077768: bra.w $778a2
pub fn i_077768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7776C;
    cpu.ir = 0x6000;
    cpu.pc = 0x778A2;
    10
}
// ===== MuzzleFx_Attach_07776c  ($07776C, 302 B, props/fx_explosions_platforms_crew_076000.s)
/// $07776C: move.w #$ffff, $70(a6)
pub fn i_07776c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77772;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077772: lea.l $77778(pc), a1
pub fn i_077772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77776;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77778;
    16
}
/// $077776: move.l a1, (a6)
pub fn i_077776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77778;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077778: jsr $77c0c(pc)
pub fn i_077778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7777C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7777C);
    cpu.pc = 0x77C0C;
    16
}
/// $07777C: jsr $77bc6(pc)
pub fn i_07777c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77780;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77780);
    cpu.pc = 0x77BC6;
    16
}
/// $077780: movea.l $c(a6), a2
pub fn i_077780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77784;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077784: move.w $22(a2), d1
pub fn i_077784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77788;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077788: move.w $24(a2), d2
pub fn i_077788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7778C;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07778C: move.w d1, $22(a6)
pub fn i_07778c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77790;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077790: move.w d2, $24(a6)
pub fn i_077790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77794;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077794: jsr $77c28(pc)
pub fn i_077794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77798;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77798);
    cpu.pc = 0x77C28;
    16
}
/// $077798: clr.w d0
pub fn i_077798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7779A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07779A: tst.w $28(a2)
pub fn i_07779a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7779E;
    cpu.ir = 0x4A6A;
    let ad2 = cpu.a[2].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07779E: beq.w $777b2
pub fn i_07779e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x777B2; return 10; }
    8
}
/// $0777A2: move.w #$4, d0
pub fn i_0777a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0777A6: tst.w $28(a2)
pub fn i_0777a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777AA;
    cpu.ir = 0x4A6A;
    let ad2 = cpu.a[2].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0777AA: bge.w $777b2
pub fn i_0777aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777AE;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x777B2; return 10; }
    8
}
/// $0777AE: move.w #$8, d0
pub fn i_0777ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777B2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0777B2: cmpi.b #$80, $72(a6)
pub fn i_0777b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777B8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $0777B8: bne.w $777fc
pub fn i_0777b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777BC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x777FC; return 10; }
    8
}
/// $0777BC: cmpi.b #$82, $88(a6)
pub fn i_0777bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777C2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x82, Size::Byte, false);
    16
}
/// $0777C2: beq.w $777d0
pub fn i_0777c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x777D0; return 10; }
    8
}
/// $0777C6: cmpi.b #$8c, $88(a6)
pub fn i_0777c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777CC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8C, Size::Byte, false);
    16
}
/// $0777CC: bne.w $777d2
pub fn i_0777cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x777D2; return 10; }
    8
}
/// $0777D0: addq.w #$2, d0
pub fn i_0777d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777D2;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0777D2: movem.w d0, -(a7)
pub fn i_0777d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777D6;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $0777D6: move.w $22(a6), d0
pub fn i_0777d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777DA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0777DA: move.w $24(a6), d1
pub fn i_0777da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777DE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0777DE: jsr $440bc.l
pub fn i_0777de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x777E4);
    cpu.pc = 0x440BC;
    16
}
/// $0777E4: move.w d0, d2
pub fn i_0777e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777E6;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0777E6: movem.w (a7)+, d0
pub fn i_0777e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777EA;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $0777EA: cmpi.w #$d20, d2
pub fn i_0777ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777EE;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xD20, Size::Word, false);
    8
}
/// $0777EE: bcs.w $777fc
pub fn i_0777ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777F2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x777FC; return 10; }
    8
}
/// $0777F2: cmpi.w #$e58, d2
pub fn i_0777f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777F6;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE58, Size::Word, false);
    8
}
/// $0777F6: bcc.w $777fc
pub fn i_0777f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x777FC; return 10; }
    8
}
/// $0777FA: addq.w #$1, d0
pub fn i_0777fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x777FC;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0777FC: cmp.w $70(a6), d0
pub fn i_0777fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77800;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $077800: beq.w $77822
pub fn i_077800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77804;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77822; return 10; }
    8
}
/// $077804: move.w d0, $70(a6)
pub fn i_077804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77808;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077808: movea.l $80(a6), a0
pub fn i_077808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7780C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07780C: lsl.w #$2, d0
pub fn i_07780c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7780E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07780E: movea.l (a0, d0.w), a0
pub fn i_07780e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77812;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $077812: cmpa.l #$ffffffff, a0
pub fn i_077812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77818;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $077818: beq.w $77822
pub fn i_077818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7781C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77822; return 10; }
    8
}
/// $07781C: jsr $28cd4.l
pub fn i_07781c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77822);
    cpu.pc = 0x28CD4;
    16
}
/// $077822: cmpi.b #$0, $72(a6)
pub fn i_077822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77828;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $077828: beq.w $77890
pub fn i_077828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7782C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77890; return 10; }
    8
}
/// $07782C: cmpi.b #$c0, $72(a6)
pub fn i_07782c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77832;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Byte, false);
    16
}
/// $077832: beq.w $77890
pub fn i_077832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77836;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77890; return 10; }
    8
}
/// $077836: cmpi.b #$1, $73(a6)
pub fn i_077836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7783C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07783C: beq.w $77860
pub fn i_07783c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77840;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77860; return 10; }
    8
}
/// $077840: jsr $77bae(pc)
pub fn i_077840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77844;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77844);
    cpu.pc = 0x77BAE;
    16
}
/// $077844: move.w $70(a6), d0
pub fn i_077844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77848;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077848: lea.l $2dc946.l, a0
pub fn i_077848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7784E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DC946;
    20
}
/// $07784E: asl.w #$2, d0
pub fn i_07784e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77850;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $077850: move.l (a0, d0.w), $5c(a6)
pub fn i_077850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77856;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $077856: jsr $28d70.l
pub fn i_077856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7785C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7785C);
    cpu.pc = 0x28D70;
    16
}
/// $07785C: bra.w $77890
pub fn i_07785c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77860;
    cpu.ir = 0x6000;
    cpu.pc = 0x77890;
    10
}
/// $077860: move.w $84(a6), d1
pub fn i_077860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77864;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077864: move.w $86(a6), d2
pub fn i_077864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77868;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077868: lea.l $778a2(pc), a1
pub fn i_077868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7786C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $07786C: move.l a1, (a6)
pub fn i_07786c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7786E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07786E: lea.l $77a16(pc), a1
pub fn i_07786e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77872;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A16;
    16
}
/// $077872: jsr $6fe.l
pub fn i_077872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77878;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77878);
    cpu.pc = 0x6FE;
    16
}
/// $077878: jsr $5dd02.l
pub fn i_077878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7787E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7787E);
    cpu.pc = 0x5DD02;
    16
}
/// $07787E: move.w $88(a6), $88(a0)
pub fn i_07787e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77884;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x88);
    bus.write16(ad4, v3 as u16);
    20
}
/// $077884: move.b $12(a6), d0
pub fn i_077884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77888;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $077888: andi.b #$40, d0
pub fn i_077888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7788C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $07788C: or.b d0, $12(a0)
pub fn i_07788c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77890;
    cpu.ir = 0x8128;
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $077890: jsr $7b2.l
pub fn i_077890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77896;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77896);
    cpu.pc = 0x7B2;
    16
}
/// $077896: bcc.w $778a0
pub fn i_077896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7789A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x778A0; return 10; }
    8
}
// ===== SetTaskHandler_07789a  ($07789A, 8 B, kernel/task_handlers.c)
/// $07789A: lea.l $77a8e(pc), a1
pub fn i_07789a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7789E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $07789E: move.l a1, (a6)
pub fn i_07789e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0778A0: rts
pub fn i_0778a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_Track_0778a2  ($0778A2, 82 B, props/fx_explosions_platforms_crew_076000.s)
/// $0778A2: jsr $77c0c(pc)
pub fn i_0778a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x778A6);
    cpu.pc = 0x77C0C;
    16
}
/// $0778A6: movea.l $c(a6), a2
pub fn i_0778a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778AA;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0778AA: move.w $22(a2), d1
pub fn i_0778aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778AE;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0778AE: move.w $24(a2), d2
pub fn i_0778ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778B2;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0778B2: move.w d1, $22(a6)
pub fn i_0778b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778B6;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0778B6: move.w d2, $24(a6)
pub fn i_0778b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778BA;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0778BA: jsr $77bc6(pc)
pub fn i_0778ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x778BE);
    cpu.pc = 0x77BC6;
    16
}
/// $0778BE: bcc.w $778e0
pub fn i_0778be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x778E0; return 10; }
    8
}
/// $0778C2: cmpi.b #$40, $72(a6)
pub fn i_0778c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778C8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $0778C8: beq.w $778ea
pub fn i_0778c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x778EA; return 10; }
    8
}
/// $0778CC: cmpi.b #$80, $72(a6)
pub fn i_0778cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778D2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $0778D2: beq.w $778ea
pub fn i_0778d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x778EA; return 10; }
    8
}
/// $0778D6: lea.l $778fc(pc), a1
pub fn i_0778d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778FC;
    16
}
/// $0778DA: move.l a1, (a6)
pub fn i_0778da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0778DC: bra.w $778ea
pub fn i_0778dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x778EA;
    10
}
/// $0778E0: jsr $77bae(pc)
pub fn i_0778e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x778E4);
    cpu.pc = 0x77BAE;
    16
}
/// $0778E4: lea.l $77912(pc), a1
pub fn i_0778e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77912;
    16
}
/// $0778E8: move.l a1, (a6)
pub fn i_0778e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0778EA: jsr $7b2.l
pub fn i_0778ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x778F0);
    cpu.pc = 0x7B2;
    16
}
/// $0778F0: bcc.w $778fa
pub fn i_0778f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x778FA; return 10; }
    8
}
// ===== SetTaskHandler_0778f4  ($0778F4, 8 B, kernel/task_handlers.c)
/// $0778F4: lea.l $77a8e(pc), a1
pub fn i_0778f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $0778F8: move.l a1, (a6)
pub fn i_0778f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0778FA: rts
pub fn i_0778fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x778FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_WaitAim_0778fc  ($0778FC, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $0778FC: jsr $77bc6(pc)
pub fn i_0778fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77900;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77900);
    cpu.pc = 0x77BC6;
    16
}
/// $077900: bcs.w $7790e
pub fn i_077900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77904;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7790E; return 10; }
    8
}
/// $077904: jsr $77bae(pc)
pub fn i_077904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77908;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77908);
    cpu.pc = 0x77BAE;
    16
}
/// $077908: lea.l $77912(pc), a1
pub fn i_077908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7790C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77912;
    16
}
/// $07790C: move.l a1, (a6)
pub fn i_07790c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7790E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07790E: jmp $778ea(pc)
pub fn i_07790e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77912;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x778EA;
    8
}
// ===== MuzzleFx_Flash_077912  ($077912, 252 B, props/fx_explosions_platforms_crew_076000.s)
/// $077912: jsr $77bc6(pc)
pub fn i_077912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77916;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77916);
    cpu.pc = 0x77BC6;
    16
}
/// $077916: clr.w d0
pub fn i_077916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77918;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077918: cmpi.b #$80, $72(a6)
pub fn i_077918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7791E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $07791E: bne.w $77938
pub fn i_07791e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77922;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77938; return 10; }
    8
}
/// $077922: cmpi.b #$82, $88(a6)
pub fn i_077922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77928;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x82, Size::Byte, false);
    16
}
/// $077928: beq.w $77936
pub fn i_077928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7792C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77936; return 10; }
    8
}
/// $07792C: cmpi.b #$8c, $88(a6)
pub fn i_07792c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77932;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8C, Size::Byte, false);
    16
}
/// $077932: bne.w $77938
pub fn i_077932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77936;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77938; return 10; }
    8
}
/// $077936: addq.w #$1, d0
pub fn i_077936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77938;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077938: movea.l $7c(a6), a0
pub fn i_077938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7793C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07793C: lsl.w #$2, d0
pub fn i_07793c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7793E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07793E: movea.l (a0, d0.w), a0
pub fn i_07793e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77942;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $077942: cmpa.l #$ffffffff, a0
pub fn i_077942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77948;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $077948: beq.w $77952
pub fn i_077948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7794C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77952; return 10; }
    8
}
/// $07794C: jsr $28cd4.l
pub fn i_07794c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77952);
    cpu.pc = 0x28CD4;
    16
}
/// $077952: lea.l $77958(pc), a1
pub fn i_077952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77956;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77958;
    16
}
/// $077956: move.l a1, (a6)
pub fn i_077956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77958;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077958: jsr $77c0c(pc)
pub fn i_077958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7795C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7795C);
    cpu.pc = 0x77C0C;
    16
}
/// $07795C: movea.l $c(a6), a2
pub fn i_07795c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77960;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077960: move.w $22(a2), d1
pub fn i_077960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77964;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077964: move.w $24(a2), d2
pub fn i_077964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77968;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077968: move.w d1, $22(a6)
pub fn i_077968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7796C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07796C: move.w d2, $24(a6)
pub fn i_07796c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77970;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077970: jsr $77bc6(pc)
pub fn i_077970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77974;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77974);
    cpu.pc = 0x77BC6;
    16
}
/// $077974: bcc.w $779d8
pub fn i_077974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77978;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x779D8; return 10; }
    8
}
/// $077978: jsr $77c28(pc)
pub fn i_077978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7797C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7797C);
    cpu.pc = 0x77C28;
    16
}
/// $07797C: lea.l $778a2(pc), a1
pub fn i_07797c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77980;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x778A2;
    16
}
/// $077980: move.l a1, (a6)
pub fn i_077980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77982;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077982: move.w $84(a6), d1
pub fn i_077982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77986;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077986: jsr $77bc6(pc)
pub fn i_077986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7798A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7798A);
    cpu.pc = 0x77BC6;
    16
}
/// $07798A: cmpi.b #$0, $72(a6)
pub fn i_07798a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77990;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $077990: beq.w $77a04
pub fn i_077990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77994;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $077994: cmpi.b #$c0, $72(a6)
pub fn i_077994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7799A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Byte, false);
    16
}
/// $07799A: beq.w $77a04
pub fn i_07799a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7799E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $07799E: lea.l $77a16(pc), a1
pub fn i_07799e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A16;
    16
}
/// $0779A2: jsr $6fe.l
pub fn i_0779a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x779A8);
    cpu.pc = 0x6FE;
    16
}
/// $0779A8: jsr $5dd02.l
pub fn i_0779a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x779AE);
    cpu.pc = 0x5DD02;
    16
}
/// $0779AE: movea.l $c(a6), a2
pub fn i_0779ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779B2;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0779B2: move.w $34(a2), $34(a0)
pub fn i_0779b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779B8;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0779B8: move.w $88(a6), $88(a0)
pub fn i_0779b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779BE;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x88);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0779BE: bclr.b #$6, $12(a0)
pub fn i_0779be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779C4;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0779C4: btst.b #$6, $12(a6)
pub fn i_0779c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779CA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0779CA: beq.w $77a04
pub fn i_0779ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $0779CE: bset.b #$6, $12(a0)
pub fn i_0779ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779D4;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0779D4: bra.w $77a04
pub fn i_0779d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x77A04;
    10
}
/// $0779D8: cmpi.b #$0, $72(a6)
pub fn i_0779d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779DE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0779DE: beq.w $77a04
pub fn i_0779de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $0779E2: cmpi.b #$c0, $72(a6)
pub fn i_0779e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779E8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Byte, false);
    16
}
/// $0779E8: beq.w $77a04
pub fn i_0779e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $0779EC: jsr $77c28(pc)
pub fn i_0779ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x779F0);
    cpu.pc = 0x77C28;
    16
}
/// $0779F0: jsr $77bae(pc)
pub fn i_0779f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x779F4);
    cpu.pc = 0x77BAE;
    16
}
/// $0779F4: jsr $28d70.l
pub fn i_0779f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x779FA);
    cpu.pc = 0x28D70;
    16
}
/// $0779FA: bcc.w $77a04
pub fn i_0779fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x779FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77A04; return 10; }
    8
}
/// $0779FE: lea.l $7776c(pc), a1
pub fn i_0779fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7776C;
    16
}
/// $077A02: move.l a1, (a6)
pub fn i_077a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A04;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077A04: jsr $7b2.l
pub fn i_077a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77A0A);
    cpu.pc = 0x7B2;
    16
}
/// $077A0A: bcc.w $77a14
pub fn i_077a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A0E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77A14; return 10; }
    8
}
// ===== SetTaskHandler_077a0e  ($077A0E, 8 B, kernel/task_handlers.c)
/// $077A0E: lea.l $77a8e(pc), a1
pub fn i_077a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $077A12: move.l a1, (a6)
pub fn i_077a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077A14: rts
pub fn i_077a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_Detached_077a16  ($077A16, 112 B, props/fx_explosions_platforms_crew_076000.s)
/// $077A16: clr.w d0
pub fn i_077a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A18;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077A18: cmpi.b #$82, $88(a6)
pub fn i_077a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A1E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x82, Size::Byte, false);
    16
}
/// $077A1E: beq.w $77a2e
pub fn i_077a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A22;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A2E; return 10; }
    8
}
/// $077A22: cmpi.b #$8c, $88(a6)
pub fn i_077a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A28;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8C, Size::Byte, false);
    16
}
/// $077A28: bne.w $77a2e
pub fn i_077a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A2C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77A2E; return 10; }
    8
}
/// $077A2C: addq.w #$1, d0
pub fn i_077a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A2E;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077A2E: movea.l $c(a6), a2
pub fn i_077a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A32;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077A32: movea.l $78(a2), a0
pub fn i_077a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A36;
    cpu.ir = 0x206A;
    let ad2 = cpu.a[2].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $077A36: lsl.w #$2, d0
pub fn i_077a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A38;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $077A38: movea.l (a0, d0.w), a0
pub fn i_077a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A3C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $077A3C: cmpa.l #$ffffffff, a0
pub fn i_077a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A42;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $077A42: beq.w $77a4c
pub fn i_077a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A46;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77A4C; return 10; }
    8
}
/// $077A46: jsr $28cd4.l
pub fn i_077a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77A4C);
    cpu.pc = 0x28CD4;
    16
}
/// $077A4C: jsr $77670(pc)
pub fn i_077a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77A50);
    cpu.pc = 0x77670;
    16
}
/// $077A50: jsr $236e.l
pub fn i_077a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77A56);
    cpu.pc = 0x236E;
    16
}
/// $077A56: addq.w #$1, $38(a6)
pub fn i_077a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A5A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $077A5A: lea.l $77a60(pc), a1
pub fn i_077a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A60;
    16
}
/// $077A5E: move.l a1, (a6)
pub fn i_077a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077A60: jsr $2783a.l
pub fn i_077a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77A66);
    cpu.pc = 0x2783A;
    16
}
/// $077A66: jsr $28d70.l
pub fn i_077a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77A6C);
    cpu.pc = 0x28D70;
    16
}
/// $077A6C: bcc.w $77a76
pub fn i_077a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77A76; return 10; }
    8
}
/// $077A70: lea.l $77a8e(pc), a1
pub fn i_077a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $077A74: move.l a1, (a6)
pub fn i_077a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077A76: movea.l #$ffffffff, a0
pub fn i_077a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A7C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $077A7C: jsr $5dd56.l
pub fn i_077a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77A82);
    cpu.pc = 0x5DD56;
    16
}
/// $077A82: bcc.w $77a8c
pub fn i_077a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A86;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77A8C; return 10; }
    8
}
// ===== SetTaskHandler_077a86  ($077A86, 8 B, kernel/task_handlers.c)
/// $077A86: lea.l $77a8e(pc), a1
pub fn i_077a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $077A8A: move.l a1, (a6)
pub fn i_077a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077A8C: rts
pub fn i_077a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_077a8e  ($077A8E, 8 B, kernel/jmp_to_scheduler.c)
/// $077A8E: jmp $518.l
pub fn i_077a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A94;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $077A94: rts
pub fn i_077a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_ShotA_077a96  ($077A96, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $077A96: jsr $77640(pc)
pub fn i_077a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77A9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77A9A);
    cpu.pc = 0x77640;
    16
}
/// $077A9A: move.l #$2dca6a, $78(a6)
pub fn i_077a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AA2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA6A;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077AA2: jmp $77ad6(pc)
pub fn i_077aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AA6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x77AD6;
    8
}
// ===== MuzzleFx_ShotB_077aa6  ($077AA6, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $077AA6: jsr $77640(pc)
pub fn i_077aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AAA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77AAA);
    cpu.pc = 0x77640;
    16
}
/// $077AAA: move.l #$2dca72, $78(a6)
pub fn i_077aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AB2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA72;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077AB2: jmp $77ad6(pc)
pub fn i_077ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AB6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x77AD6;
    8
}
// ===== MuzzleFx_ShotC_077ab6  ($077AB6, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $077AB6: jsr $77640(pc)
pub fn i_077ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77ABA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77ABA);
    cpu.pc = 0x77640;
    16
}
/// $077ABA: move.l #$2dca7a, $78(a6)
pub fn i_077aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AC2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA7A;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077AC2: jmp $77ad6(pc)
pub fn i_077ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AC6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x77AD6;
    8
}
// ===== MuzzleFx_ShotD_077ac6  ($077AC6, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $077AC6: jsr $77640(pc)
pub fn i_077ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77ACA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77ACA);
    cpu.pc = 0x77640;
    16
}
/// $077ACA: move.l #$2dca82, $78(a6)
pub fn i_077aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AD2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DCA82;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077AD2: jmp $77ad6(pc)
pub fn i_077ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AD6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x77AD6;
    8
}
// ===== MuzzleFx_Shot_Run_077ad6  ($077AD6, 138 B, props/fx_explosions_platforms_crew_076000.s)
/// $077AD6: jsr $77c0c(pc)
pub fn i_077ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77ADA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77ADA);
    cpu.pc = 0x77C0C;
    16
}
/// $077ADA: movea.l $c(a6), a2
pub fn i_077ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77ADE;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077ADE: move.w $22(a2), d1
pub fn i_077ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AE2;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077AE2: move.w $24(a2), d2
pub fn i_077ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AE6;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077AE6: move.w d1, $22(a6)
pub fn i_077ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AEA;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077AEA: move.w d2, $24(a6)
pub fn i_077aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AEE;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077AEE: jsr $77bc6(pc)
pub fn i_077aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77AF2);
    cpu.pc = 0x77BC6;
    16
}
/// $077AF2: bcs.w $77b14
pub fn i_077af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AF6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x77B14; return 10; }
    8
}
/// $077AF6: cmpi.b #$40, $72(a6)
pub fn i_077af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77AFC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $077AFC: beq.w $77b28
pub fn i_077afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77B28; return 10; }
    8
}
/// $077B00: cmpi.b #$80, $72(a6)
pub fn i_077b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B06;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $077B06: beq.w $77b2c
pub fn i_077b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B0A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77B2C; return 10; }
    8
}
/// $077B0A: lea.l $77a8e(pc), a1
pub fn i_077b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $077B0E: move.l a1, (a6)
pub fn i_077b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077B10: bra.w $77b40
pub fn i_077b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B14;
    cpu.ir = 0x6000;
    cpu.pc = 0x77B40;
    10
}
/// $077B14: cmpi.b #$80, $72(a6)
pub fn i_077b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B1A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $077B1A: beq.w $77b2c
pub fn i_077b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B1E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77B2C; return 10; }
    8
}
/// $077B1E: cmpi.b #$40, $72(a6)
pub fn i_077b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B24;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $077B24: bne.w $77b40
pub fn i_077b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B28;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77B40; return 10; }
    8
}
/// $077B28: jsr $77c28(pc)
pub fn i_077b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77B2C);
    cpu.pc = 0x77C28;
    16
}
/// $077B2C: lea.l $77b68(pc), a1
pub fn i_077b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77B68;
    16
}
/// $077B30: move.l a1, (a6)
pub fn i_077b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077B32: movea.l $c(a6), a2
pub fn i_077b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B36;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077B36: move.w $34(a2), $34(a6)
pub fn i_077b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B3C;
    cpu.ir = 0x3D6A;
    let ad2 = cpu.a[2].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $077B3C: jsr $77bae(pc)
pub fn i_077b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77B40);
    cpu.pc = 0x77BAE;
    16
}
/// $077B40: movea.l #$ffffffff, a0
pub fn i_077b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B46;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $077B46: jsr $5dd56.l
pub fn i_077b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77B4C);
    cpu.pc = 0x5DD56;
    16
}
/// $077B4C: bcc.w $77b56
pub fn i_077b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77B56; return 10; }
    8
}
/// $077B50: lea.l $77a8e(pc), a1
pub fn i_077b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $077B54: move.l a1, (a6)
pub fn i_077b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077B56: jsr $7b2.l
pub fn i_077b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77B5C);
    cpu.pc = 0x7B2;
    16
}
/// $077B5C: bcc.w $77b66
pub fn i_077b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77B66; return 10; }
    8
}
// ===== SetTaskHandler_077b60  ($077B60, 8 B, kernel/task_handlers.c)
/// $077B60: lea.l $77a8e(pc), a1
pub fn i_077b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77A8E;
    16
}
/// $077B64: move.l a1, (a6)
pub fn i_077b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077B66: rts
pub fn i_077b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_Shot_Fire_077b68  ($077B68, 70 B, props/fx_explosions_platforms_crew_076000.s)
/// $077B68: cmpi.l #$2dca6a, $78(a6)
pub fn i_077b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B70;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x2DCA6A, Size::Long, false);
    20
}
/// $077B70: bne.w $77b8e
pub fn i_077b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77B8E; return 10; }
    8
}
/// $077B74: cmpi.b #$80, $72(a6)
pub fn i_077b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B7A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $077B7A: beq.w $77b8e
pub fn i_077b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77B8E; return 10; }
    8
}
/// $077B7E: jsr $5e9b6.l
pub fn i_077b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77B84);
    cpu.pc = 0x5E9B6;
    16
}
/// $077B84: andi.w #$f, d0
pub fn i_077b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B88;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $077B88: subq.w #$8, d0
pub fn i_077b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B8A;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077B8A: add.w d0, $22(a6)
pub fn i_077b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B8E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $077B8E: clr.w d0
pub fn i_077b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B90;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077B90: cmpi.b #$82, $88(a6)
pub fn i_077b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B96;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x82, Size::Byte, false);
    16
}
/// $077B96: beq.w $77ba4
pub fn i_077b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77B9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77BA4; return 10; }
    8
}
/// $077B9A: cmpi.b #$8c, $88(a6)
pub fn i_077b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BA0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8C, Size::Byte, false);
    16
}
/// $077BA0: bne.w $77ba6
pub fn i_077ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77BA6; return 10; }
    8
}
/// $077BA4: addq.w #$1, d0
pub fn i_077ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BA6;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077BA6: movea.l $78(a6), a0
pub fn i_077ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BAA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $077BAA: bra.w $77a36
pub fn i_077baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x77A36;
    10
}
// ===== MuzzleFx_SetFlipByDir_077bae  ($077BAE, 24 B, props/fx_explosions_platforms_crew_076000.s)
/// $077BAE: bset.b #$6, $12(a6)
pub fn i_077bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BB4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $077BB4: cmpi.b #$80, $72(a6)
pub fn i_077bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BBA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $077BBA: bne.w $77bc4
pub fn i_077bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BBE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77BC4; return 10; }
    8
}
/// $077BBE: bclr.b #$6, $12(a6)
pub fn i_077bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BC4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $077BC4: rts
pub fn i_077bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_ReadParentAim_077bc6  ($077BC6, 38 B, props/fx_explosions_platforms_crew_076000.s)
/// $077BC6: movem.l a6, -(a7)
pub fn i_077bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BCA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $077BCA: movea.l $c(a6), a6
pub fn i_077bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BCE;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $077BCE: jsr $27fac.l
pub fn i_077bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77BD4);
    cpu.pc = 0x27FAC;
    16
}
/// $077BD4: movem.l (a7)+, a6
pub fn i_077bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BD8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $077BD8: bcc.w $77bf2
pub fn i_077bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77BF2; return 10; }
    8
}
/// $077BDC: move.b d7, $72(a6)
pub fn i_077bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BE0;
    cpu.ir = 0x1D47;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $077BE0: andi.b #$c0, $72(a6)
pub fn i_077be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BE6;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $077BE6: move.b #$1, $73(a6)
pub fn i_077be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BEC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_077bec  ($077BEC, 6 B, thunks/ccr_helpers.c)
/// $077BEC: ori.b #$11, ccr
pub fn i_077bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BF0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $077BF0: rts
pub fn i_077bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_StoreAim_077bf2  ($077BF2, 20 B, props/fx_explosions_platforms_crew_076000.s)
/// $077BF2: move.b d7, $72(a6)
pub fn i_077bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BF6;
    cpu.ir = 0x1D47;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $077BF6: move.b d7, $88(a6)
pub fn i_077bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77BFA;
    cpu.ir = 0x1D47;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $077BFA: andi.b #$c0, $72(a6)
pub fn i_077bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C00;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $077C00: move.b #$0, $73(a6)
pub fn i_077c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C06;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClearXN_077c06  ($077C06, 6 B, thunks/ccr_helpers.c)
/// $077C06: andi.b #$ee, ccr
pub fn i_077c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C0A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $077C0A: rts
pub fn i_077c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_SavePos_077c0c  ($077C0C, 14 B, props/fx_explosions_platforms_crew_076000.s)
/// $077C0C: move.w $22(a6), $84(a6)
pub fn i_077c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C12;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad4, v3 as u16);
    20
}
/// $077C12: move.w $24(a6), $86(a6)
pub fn i_077c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C18;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad4, v3 as u16);
    20
}
/// $077C18: rts
pub fn i_077c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_IsDirUp_077c1a  ($077C1A, 12 B, props/fx_explosions_platforms_crew_076000.s)
/// $077C1A: cmpi.b #$80, $72(a6)
pub fn i_077c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C20;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $077C20: beq.w $77c26
pub fn i_077c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C24;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77C26; return 10; }
    8
}
/// $077C24: rts
pub fn i_077c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00077C26  ($077C26, 2 B, thunks/trivial_rts.c)
/// $077C26: rts
pub fn i_077c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_ClampGroundY_077c28  ($077C28, 58 B, props/fx_explosions_platforms_crew_076000.s)
/// $077C28: cmpi.b #$40, $72(a6)
pub fn i_077c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C2E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $077C2E: bne.w $77c60
pub fn i_077c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C32;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77C60; return 10; }
    8
}
/// $077C32: move.w #$0, d0
pub fn i_077c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C36;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077C36: move.w #$167, d1
pub fn i_077c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C3A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x167;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077C3A: cmpi.b #$6, $106ece.l
pub fn i_077c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C42;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x6, Size::Byte, false);
    20
}
/// $077C42: beq.w $77c52
pub fn i_077c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C46;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77C52; return 10; }
    8
}
/// $077C46: cmpi.b #$9, $106ece.l
pub fn i_077c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C4E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x9, Size::Byte, false);
    20
}
/// $077C4E: bne.w $77c56
pub fn i_077c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77C56; return 10; }
    8
}
/// $077C52: move.w #$1ef, d1
pub fn i_077c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C56;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1EF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077C56: jsr $440d0.l
pub fn i_077c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77C5C);
    cpu.pc = 0x440D0;
    16
}
/// $077C5C: move.w d1, $24(a6)
pub fn i_077c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C60;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077C60: rts
pub fn i_077c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MuzzleFx_RetGateB_077c62  ($077C62, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $077C62: movea.l $8(a6), a1
pub fn i_077c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C66;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $077C66: move.b $10(a6), d0
pub fn i_077c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C6A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $077C6A: cmp.b $10(a1), d0
pub fn i_077c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C6E;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $077C6E: bcs.w $77c78
pub fn i_077c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C72;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x77C78; return 10; }
    8
}
// ===== ClearXN_077c72  ($077C72, 6 B, thunks/ccr_helpers.c)
/// $077C72: andi.b #$ee, ccr
pub fn i_077c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C76;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $077C76: rts
pub fn i_077c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_077c78  ($077C78, 6 B, thunks/ccr_helpers.c)
/// $077C78: ori.b #$11, ccr
pub fn i_077c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C7C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $077C7C: rts
pub fn i_077c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_InstallHandlerAndCopyXf  ($077C7E, 26 B, entity/install_handler_077c7e.s)
/// $077C7E: move.l a1, -(a7)
pub fn i_077c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C80;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $077C80: lea.l $77c98(pc), a1
pub fn i_077c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77C98;
    16
}
/// $077C84: jsr $6fe.l
pub fn i_077c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77C8A);
    cpu.pc = 0x6FE;
    16
}
/// $077C8A: jsr $5dd02.l
pub fn i_077c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77C90);
    cpu.pc = 0x5DD02;
    16
}
/// $077C90: movea.l (a7)+, a1
pub fn i_077c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C92;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $077C92: move.l a1, $70(a0)
pub fn i_077c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C96;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077C96: rts
pub fn i_077c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C98;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawner_Handler_077c98  ($077C98, 236 B, props/fx_explosions_platforms_crew_076000.s)
/// $077C98: movea.l $70(a6), a0
pub fn i_077c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77C9C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $077C9C: move.b $1(a0), $74(a6)
pub fn i_077c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CA2;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad4, v3 as u8);
    20
}
/// $077CA2: move.w $2(a0), $76(a6)
pub fn i_077ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CA8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $077CA8: lea.l $77cae(pc), a1
pub fn i_077ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77CAE;
    16
}
/// $077CAC: move.l a1, (a6)
pub fn i_077cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077CAE: jsr $44048.l
pub fn i_077cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77CB4);
    cpu.pc = 0x44048;
    16
}
/// $077CB4: movea.l $c(a6), a0
pub fn i_077cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CB8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $077CB8: move.b $32(a0), $32(a6)
pub fn i_077cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CBE;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $077CBE: move.b $33(a0), $33(a6)
pub fn i_077cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CC4;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $077CC4: cmpi.b #$ff, $74(a6)
pub fn i_077cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CCA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $077CCA: beq.w $77ce0
pub fn i_077cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CCE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77CE0; return 10; }
    8
}
/// $077CCE: subq.b #$1, $74(a6)
pub fn i_077cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CD2;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $077CD2: bne.w $77d76
pub fn i_077cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CD6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77D76; return 10; }
    8
}
/// $077CD6: movea.l $70(a6), a2
pub fn i_077cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CDA;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077CDA: move.b $1(a2), $74(a6)
pub fn i_077cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CE0;
    cpu.ir = 0x1D6A;
    let ad2 = cpu.a[2].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad4, v3 as u8);
    20
}
/// $077CE0: movea.l $70(a6), a2
pub fn i_077ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CE4;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077CE4: cmpi.b #$0, (a2)
pub fn i_077ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CE8;
    cpu.ir = 0xC12;
    let v2 = bus.read8(cpu.a[2]) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    12
}
/// $077CE8: beq.w $77cf0
pub fn i_077ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CEC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77CF0; return 10; }
    8
}
/// $077CEC: jsr $77da6(pc)
pub fn i_077cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CF0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77CF0);
    cpu.pc = 0x77DA6;
    16
}
/// $077CF0: movea.l $70(a6), a2
pub fn i_077cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CF4;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $077CF4: movea.l $e(a2), a1
pub fn i_077cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CF8;
    cpu.ir = 0x226A;
    let ad2 = cpu.a[2].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $077CF8: cmpi.b #$0, $d(a2)
pub fn i_077cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77CFE;
    cpu.ir = 0xC2A;
    let ad2 = cpu.a[2].wrapping_add(0xD);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $077CFE: beq.w $77d18
pub fn i_077cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D02;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77D18; return 10; }
    8
}
/// $077D02: move.w #$10, d0
pub fn i_077d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D06;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077D06: jsr $5e9e4.l
pub fn i_077d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77D0C);
    cpu.pc = 0x5E9E4;
    16
}
/// $077D0C: asl.w #$2, d0
pub fn i_077d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D0E;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $077D0E: andi.l #$ffff, d0
pub fn i_077d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D14;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $077D14: movea.l (a1, d0.w), a1
pub fn i_077d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D18;
    cpu.ir = 0x2271;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $077D18: jsr $6fe.l
pub fn i_077d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77D1E);
    cpu.pc = 0x6FE;
    16
}
/// $077D1E: jsr $5dd02.l
pub fn i_077d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77D24);
    cpu.pc = 0x5DD02;
    16
}
/// $077D24: move.w $6(a2), d0
pub fn i_077d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D28;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077D28: jsr $5e9e4.l
pub fn i_077d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77D2E);
    cpu.pc = 0x5E9E4;
    16
}
/// $077D2E: jsr $77de4(pc)
pub fn i_077d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77D32);
    cpu.pc = 0x77DE4;
    16
}
/// $077D32: add.w d0, $22(a0)
pub fn i_077d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D36;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $077D36: move.w $a(a2), d0
pub fn i_077d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D3A;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077D3A: jsr $5e9e4.l
pub fn i_077d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77D40);
    cpu.pc = 0x5E9E4;
    16
}
/// $077D40: jsr $77df2(pc)
pub fn i_077d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77D44);
    cpu.pc = 0x77DF2;
    16
}
/// $077D44: add.w d0, $24(a0)
pub fn i_077d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D48;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $077D48: move.b $12(a6), d0
pub fn i_077d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D4C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $077D4C: andi.b #$40, d0
pub fn i_077d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D50;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $077D50: or.b d0, $12(a0)
pub fn i_077d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D54;
    cpu.ir = 0x8128;
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $077D54: move.b $32(a6), $32(a0)
pub fn i_077d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D5A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $077D5A: move.b $33(a6), $33(a0)
pub fn i_077d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D60;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $077D60: cmpi.b #$ff, $74(a6)
pub fn i_077d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D66;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $077D66: bne.w $77d76
pub fn i_077d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D6A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77D76; return 10; }
    8
}
/// $077D6A: subq.w #$1, $76(a6)
pub fn i_077d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D6E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $077D6E: bne.w $77cf0
pub fn i_077d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77CF0; return 10; }
    8
}
/// $077D72: bra.w $77d7e
pub fn i_077d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D76;
    cpu.ir = 0x6000;
    cpu.pc = 0x77D7E;
    10
}
/// $077D76: subq.w #$1, $76(a6)
pub fn i_077d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D7A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $077D7A: bne.w $77d86
pub fn i_077d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D7E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x77D86; return 10; }
    8
}
/// $077D7E: jmp $518.l
pub fn i_077d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D84;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Spawner_Rts_077d84  ($077D84, 2 B, props/fx_explosions_platforms_crew_076000.s)
/// $077D84: rts
pub fn i_077d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00077D86  ($077D86, 2 B, thunks/trivial_rts.c)
/// $077D86: rts
pub fn i_077d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawner_SpawnAtGoal_077d88  ($077D88, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $077D88: lea.l $77e10(pc), a1
pub fn i_077d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77E10;
    16
}
/// $077D8C: jsr $6fe.l
pub fn i_077d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77D92);
    cpu.pc = 0x6FE;
    16
}
/// $077D92: jsr $5dd02.l
pub fn i_077d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77D98);
    cpu.pc = 0x5DD02;
    16
}
/// $077D98: move.w $54(a6), $22(a0)
pub fn i_077d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77D9E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $077D9E: move.w $56(a6), $24(a0)
pub fn i_077d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DA4;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $077DA4: rts
pub fn i_077da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawner_OffsetByParent_077da6  ($077DA6, 56 B, props/fx_explosions_platforms_crew_076000.s)
/// $077DA6: movea.l $70(a6), a0
pub fn i_077da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DAA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $077DAA: movea.l $c(a6), a1
pub fn i_077daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DAE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $077DAE: move.w $22(a1), d0
pub fn i_077dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DB2;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077DB2: move.w $4(a0), d1
pub fn i_077db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DB6;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077DB6: btst.b #$0, $3a(a1)
pub fn i_077db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DBC;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $077DBC: beq.w $77dc2
pub fn i_077dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77DC2; return 10; }
    8
}
/// $077DC0: neg.w d1
pub fn i_077dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DC2;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077DC2: add.w d1, d0
pub fn i_077dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DC4;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $077DC4: move.w d0, $22(a6)
pub fn i_077dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DC8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077DC8: move.w $24(a1), d0
pub fn i_077dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DCC;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077DCC: move.w $8(a0), d1
pub fn i_077dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DD0;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $077DD0: btst.b #$0, $3a(a1)
pub fn i_077dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DD6;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $077DD6: beq.w $77ddc
pub fn i_077dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DDA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77DDC; return 10; }
    8
}
/// $077DDA: neg.w d1
pub fn i_077dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DDC;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077DDC: add.w d1, d0
pub fn i_077ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DDE;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
// ===== SetTaskW_077dde  ($077DDE, 6 B, kernel/task_setters_w.c)
/// $077DDE: move.w d0, $24(a6)
pub fn i_077dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DE2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $077DE2: rts
pub fn i_077de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawner_NegIfLeft_077de4  ($077DE4, 14 B, props/fx_explosions_platforms_crew_076000.s)
/// $077DE4: btst.b #$0, $3a(a6)
pub fn i_077de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DEA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $077DEA: beq.w $77df0
pub fn i_077dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77DF0; return 10; }
    8
}
/// $077DEE: neg.w d0
pub fn i_077dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DF0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077DF0: rts
pub fn i_077df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawner_NegIfLeftB_077df2  ($077DF2, 14 B, props/fx_explosions_platforms_crew_076000.s)
/// $077DF2: btst.b #$1, $3a(a6)
pub fn i_077df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DF8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $077DF8: beq.w $77dfe
pub fn i_077df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DFC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x77DFE; return 10; }
    8
}
/// $077DFC: neg.w d0
pub fn i_077dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77DFE;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $077DFE: rts
pub fn i_077dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fx_RandFlipFacing_077e00  ($077E00, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $077E00: jsr $5e9b6.l
pub fn i_077e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77E06);
    cpu.pc = 0x5E9B6;
    16
}
/// $077E06: andi.b #$1, d0
pub fn i_077e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E0A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $077E0A: or.b d0, $3a(a6)
pub fn i_077e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E0E;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $077E0E: rts
pub fn i_077e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Explosion_Small_077e10  ($077E10, 100 B, props/fx_explosions_platforms_crew_076000.s)
/// $077E10: move.w #$4, d1
pub fn i_077e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E14;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077E14: jsr $236e.l
pub fn i_077e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77E1A);
    cpu.pc = 0x236E;
    16
}
/// $077E1A: move.b #$ff, $32(a6)
pub fn i_077e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E20;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $077E20: move.b #$ff, $33(a6)
pub fn i_077e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E26;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $077E26: jsr $77e00(pc)
pub fn i_077e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x77E2A);
    cpu.pc = 0x77E00;
    16
}
/// $077E2A: lea.l $2dd2d4.l, a0
pub fn i_077e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD2D4;
    20
}
/// $077E30: jsr $28cd4.l
pub fn i_077e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77E36);
    cpu.pc = 0x28CD4;
    16
}
/// $077E36: lea.l $2df1f2.l, a0
pub fn i_077e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E3C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF1F2;
    20
}
/// $077E3C: jsr $5dd56.l
pub fn i_077e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77E42);
    cpu.pc = 0x5DD56;
    16
}
/// $077E42: bcs.w $77e6c
pub fn i_077e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E46;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x77E6C; return 10; }
    8
}
/// $077E46: lea.l $77e4c(pc), a1
pub fn i_077e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77E4C;
    16
}
/// $077E4A: move.l a1, (a6)
pub fn i_077e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077E4C: jsr $44048.l
pub fn i_077e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77E52);
    cpu.pc = 0x44048;
    16
}
/// $077E52: jsr $28d70.l
pub fn i_077e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77E58);
    cpu.pc = 0x28D70;
    16
}
/// $077E58: bcs.w $77e6c
pub fn i_077e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E5C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x77E6C; return 10; }
    8
}
/// $077E5C: lea.l $2df1f2.l, a0
pub fn i_077e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E62;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF1F2;
    20
}
/// $077E62: jsr $5dd56.l
pub fn i_077e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77E68);
    cpu.pc = 0x5DD56;
    16
}
/// $077E68: bcc.w $77e72
pub fn i_077e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E6C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77E72; return 10; }
    8
}
/// $077E6C: jmp $518.l
pub fn i_077e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E72;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $077E72: rts
pub fn i_077e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Explosion_Big_077e74  ($077E74, 102 B, props/fx_explosions_platforms_crew_076000.s)
/// $077E74: lea.l $2df066.l, a0
pub fn i_077e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF066;
    20
}
/// $077E7A: jsr $28cd4.l
pub fn i_077e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77E80);
    cpu.pc = 0x28CD4;
    16
}
/// $077E80: lea.l $2df22e.l, a0
pub fn i_077e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E86;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF22E;
    20
}
/// $077E86: move.l a0, $70(a6)
pub fn i_077e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E8A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077E8A: move.w #$d, d1
pub fn i_077e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E8E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077E8E: jsr $236e.l
pub fn i_077e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77E94);
    cpu.pc = 0x236E;
    16
}
/// $077E94: move.b #$ff, $32(a6)
pub fn i_077e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77E9A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $077E9A: move.b #$ff, $33(a6)
pub fn i_077e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EA0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $077EA0: movea.l $70(a6), a0
pub fn i_077ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EA4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $077EA4: jsr $5dd56.l
pub fn i_077ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77EAA);
    cpu.pc = 0x5DD56;
    16
}
/// $077EAA: bcs.w $78128
pub fn i_077eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EAE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78128; return 10; }
    8
}
/// $077EAE: lea.l $77eb4(pc), a1
pub fn i_077eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x77EB4;
    16
}
/// $077EB2: move.l a1, (a6)
pub fn i_077eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $077EB4: jsr $44048.l
pub fn i_077eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77EBA);
    cpu.pc = 0x44048;
    16
}
/// $077EBA: jsr $28d70.l
pub fn i_077eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77EC0);
    cpu.pc = 0x28D70;
    16
}
/// $077EC0: bcs.w $77ed2
pub fn i_077ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EC4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x77ED2; return 10; }
    8
}
/// $077EC4: movea.l $70(a6), a0
pub fn i_077ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EC8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $077EC8: jsr $5dd56.l
pub fn i_077ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77ECE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77ECE);
    cpu.pc = 0x5DD56;
    16
}
/// $077ECE: bcc.w $77ed8
pub fn i_077ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77ED2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x77ED8; return 10; }
    8
}
/// $077ED2: jmp $518.l
pub fn i_077ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77ED8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $077ED8: rts
pub fn i_077ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Explosion_Water_077eda  ($077EDA, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $077EDA: lea.l $2df180.l, a0
pub fn i_077eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EE0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF180;
    20
}
/// $077EE0: jsr $28cd4.l
pub fn i_077ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77EE6);
    cpu.pc = 0x28CD4;
    16
}
/// $077EE6: lea.l $2df1fc.l, a0
pub fn i_077ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF1FC;
    20
}
/// $077EEC: move.l a0, $70(a6)
pub fn i_077eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EF0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077EF0: move.w #$16c, d1
pub fn i_077ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EF4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x16C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077EF4: jsr $236e.l
pub fn i_077ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77EFA);
    cpu.pc = 0x236E;
    16
}
/// $077EFA: bra.w $780e2
pub fn i_077efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77EFE;
    cpu.ir = 0x6000;
    cpu.pc = 0x780E2;
    10
}
// ===== Explosion_Std_077efe  ($077EFE, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $077EFE: lea.l $2dd4ba.l, a0
pub fn i_077efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F04;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD4BA;
    20
}
/// $077F04: jsr $28cd4.l
pub fn i_077f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77F0A);
    cpu.pc = 0x28CD4;
    16
}
/// $077F0A: lea.l $2df1fc.l, a0
pub fn i_077f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF1FC;
    20
}
/// $077F10: move.l a0, $70(a6)
pub fn i_077f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F14;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077F14: move.w #$4, d1
pub fn i_077f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F18;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077F18: jsr $236e.l
pub fn i_077f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77F1E);
    cpu.pc = 0x236E;
    16
}
/// $077F1E: bra.w $780de
pub fn i_077f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F22;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
// ===== Explosion_StdSndB_077f22  ($077F22, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $077F22: lea.l $2dd4ba.l, a0
pub fn i_077f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD4BA;
    20
}
/// $077F28: jsr $28cd4.l
pub fn i_077f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77F2E);
    cpu.pc = 0x28CD4;
    16
}
/// $077F2E: lea.l $2df1fc.l, a0
pub fn i_077f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF1FC;
    20
}
/// $077F34: move.l a0, $70(a6)
pub fn i_077f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F38;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077F38: move.w #$12d, d1
pub fn i_077f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F3C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077F3C: jsr $236e.l
pub fn i_077f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77F42);
    cpu.pc = 0x236E;
    16
}
/// $077F42: bra.w $780de
pub fn i_077f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F46;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
// ===== Explosion_StdB_077f46  ($077F46, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $077F46: lea.l $2dd4d6.l, a0
pub fn i_077f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F4C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD4D6;
    20
}
/// $077F4C: jsr $28cd4.l
pub fn i_077f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77F52);
    cpu.pc = 0x28CD4;
    16
}
/// $077F52: lea.l $2df1fc.l, a0
pub fn i_077f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF1FC;
    20
}
/// $077F58: move.l a0, $70(a6)
pub fn i_077f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F5C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077F5C: move.w #$145, d1
pub fn i_077f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F60;
    cpu.ir = 0x323C;
    let v1: u32 = 0x145;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077F60: jsr $236e.l
pub fn i_077f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77F66);
    cpu.pc = 0x236E;
    16
}
/// $077F66: bra.w $780de
pub fn i_077f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F6A;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
// ===== Explosion_Fire_077f6a  ($077F6A, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $077F6A: lea.l $2dd594.l, a0
pub fn i_077f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD594;
    20
}
/// $077F70: jsr $28cd4.l
pub fn i_077f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77F76);
    cpu.pc = 0x28CD4;
    16
}
/// $077F76: lea.l $2df206.l, a0
pub fn i_077f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF206;
    20
}
/// $077F7C: move.l a0, $70(a6)
pub fn i_077f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F80;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077F80: move.w #$4, d1
pub fn i_077f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F84;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077F84: jsr $236e.l
pub fn i_077f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77F8A);
    cpu.pc = 0x236E;
    16
}
/// $077F8A: bra.w $780de
pub fn i_077f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
// ===== Explosion_FireSndB_077f8e  ($077F8E, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $077F8E: lea.l $2dd594.l, a0
pub fn i_077f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD594;
    20
}
/// $077F94: jsr $28cd4.l
pub fn i_077f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77F9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77F9A);
    cpu.pc = 0x28CD4;
    16
}
/// $077F9A: lea.l $2df206.l, a0
pub fn i_077f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF206;
    20
}
/// $077FA0: move.l a0, $70(a6)
pub fn i_077fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FA4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077FA4: move.w #$82, d1
pub fn i_077fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FA8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077FA8: jsr $236e.l
pub fn i_077fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77FAE);
    cpu.pc = 0x236E;
    16
}
/// $077FAE: bra.w $780de
pub fn i_077fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
// ===== Explosion_Flash_077fb2  ($077FB2, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $077FB2: lea.l $2dd37e.l, a0
pub fn i_077fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD37E;
    20
}
/// $077FB8: jsr $28cd4.l
pub fn i_077fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77FBE);
    cpu.pc = 0x28CD4;
    16
}
/// $077FBE: lea.l $2df210.l, a0
pub fn i_077fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FC4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF210;
    20
}
/// $077FC4: move.l a0, $70(a6)
pub fn i_077fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FC8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077FC8: move.w #$82, d1
pub fn i_077fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FCC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077FCC: jsr $236e.l
pub fn i_077fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77FD2);
    cpu.pc = 0x236E;
    16
}
/// $077FD2: bra.w $780de
pub fn i_077fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FD6;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
// ===== Explosion_FlashSndB_077fd6  ($077FD6, 72 B, props/fx_explosions_platforms_crew_076000.s)
/// $077FD6: lea.l $2dd37e.l, a0
pub fn i_077fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD37E;
    20
}
/// $077FDC: jsr $28cd4.l
pub fn i_077fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77FE2);
    cpu.pc = 0x28CD4;
    16
}
/// $077FE2: lea.l $2df210.l, a0
pub fn i_077fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FE8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF210;
    20
}
/// $077FE8: move.l a0, $70(a6)
pub fn i_077fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FEC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $077FEC: move.w #$4, d1
pub fn i_077fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FF0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $077FF0: jsr $236e.l
pub fn i_077ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x77FF6);
    cpu.pc = 0x236E;
    16
}
/// $077FF6: bra.w $780de
pub fn i_077ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x77FFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
/// $077FFA: lea.l $2dd39a.l, a0
pub fn i_077ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78000;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD39A;
    20
}
/// $078000: jsr $28cd4.l
pub fn i_078000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78006;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78006);
    cpu.pc = 0x28CD4;
    16
}
/// $078006: lea.l $2df210.l, a0
pub fn i_078006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7800C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF210;
    20
}
/// $07800C: move.l a0, $70(a6)
pub fn i_07800c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78010;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078010: move.w #$4, d1
pub fn i_078010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78014;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078014: jsr $236e.l
pub fn i_078014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7801A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7801A);
    cpu.pc = 0x236E;
    16
}
/// $07801A: bra.w $780de
pub fn i_07801a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7801E;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
// ===== Explosion_Spark_07801e  ($07801E, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $07801E: lea.l $2dd6a6.l, a0
pub fn i_07801e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78024;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD6A6;
    20
}
/// $078024: jsr $28cd4.l
pub fn i_078024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7802A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7802A);
    cpu.pc = 0x28CD4;
    16
}
/// $07802A: lea.l $2df210.l, a0
pub fn i_07802a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78030;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF210;
    20
}
/// $078030: move.l a0, $70(a6)
pub fn i_078030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78034;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078034: move.w #$4, d1
pub fn i_078034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78038;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078038: jsr $236e.l
pub fn i_078038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7803E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7803E);
    cpu.pc = 0x236E;
    16
}
/// $07803E: bra.w $780de
pub fn i_07803e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78042;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
// ===== Explosion_SmokePuff_078042  ($078042, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $078042: lea.l $2dd6d2.l, a0
pub fn i_078042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78048;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD6D2;
    20
}
/// $078048: jsr $28cd4.l
pub fn i_078048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7804E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7804E);
    cpu.pc = 0x28CD4;
    16
}
/// $07804E: lea.l $2df1fc.l, a0
pub fn i_07804e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78054;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF1FC;
    20
}
/// $078054: move.l a0, $70(a6)
pub fn i_078054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78058;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078058: move.w #$4, d1
pub fn i_078058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7805C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07805C: jsr $236e.l
pub fn i_07805c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78062;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78062);
    cpu.pc = 0x236E;
    16
}
/// $078062: bra.w $780de
pub fn i_078062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78066;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
// ===== Explosion_FireLoud_078066  ($078066, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $078066: lea.l $2dd594.l, a0
pub fn i_078066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7806C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD594;
    20
}
/// $07806C: jsr $28cd4.l
pub fn i_07806c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78072;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78072);
    cpu.pc = 0x28CD4;
    16
}
/// $078072: lea.l $2df206.l, a0
pub fn i_078072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78078;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF206;
    20
}
/// $078078: move.l a0, $70(a6)
pub fn i_078078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7807C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07807C: move.w #$6c, d1
pub fn i_07807c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78080;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078080: jsr $236e.l
pub fn i_078080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78086;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78086);
    cpu.pc = 0x236E;
    16
}
/// $078086: bra.w $780de
pub fn i_078086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7808A;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
// ===== Explosion_FlashLoud_07808a  ($07808A, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $07808A: lea.l $2dd37e.l, a0
pub fn i_07808a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78090;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD37E;
    20
}
/// $078090: jsr $28cd4.l
pub fn i_078090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78096;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78096);
    cpu.pc = 0x28CD4;
    16
}
/// $078096: lea.l $2df210.l, a0
pub fn i_078096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7809C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF210;
    20
}
/// $07809C: move.l a0, $70(a6)
pub fn i_07809c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780A0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0780A0: move.w #$6c, d1
pub fn i_0780a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780A4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0780A4: jsr $236e.l
pub fn i_0780a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x780AA);
    cpu.pc = 0x236E;
    16
}
/// $0780AA: bra.w $780de
pub fn i_0780aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
// ===== Explosion_FireFront_0780ae  ($0780AE, 130 B, props/fx_explosions_platforms_crew_076000.s)
/// $0780AE: lea.l $2dd594.l, a0
pub fn i_0780ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD594;
    20
}
/// $0780B4: jsr $28cd4.l
pub fn i_0780b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x780BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0780BA: lea.l $2df206.l, a0
pub fn i_0780ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF206;
    20
}
/// $0780C0: move.l a0, $70(a6)
pub fn i_0780c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780C4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0780C4: move.w #$4, d1
pub fn i_0780c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0780C8: jsr $236e.l
pub fn i_0780c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x780CE);
    cpu.pc = 0x236E;
    16
}
/// $0780CE: andi.w #$fff, $38(a6)
pub fn i_0780ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780D4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0780D4: ori.w #$c000, $38(a6)
pub fn i_0780d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780DA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC000) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0780DA: bra.w $780de
pub fn i_0780da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x780DE;
    10
}
/// $0780DE: jsr $77e00(pc)
pub fn i_0780de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x780E2);
    cpu.pc = 0x77E00;
    16
}
/// $0780E2: move.b #$ff, $32(a6)
pub fn i_0780e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0780E8: move.b #$ff, $33(a6)
pub fn i_0780e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0780EE: movea.l $70(a6), a0
pub fn i_0780ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780F2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0780F2: jsr $5dd56.l
pub fn i_0780f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x780F8);
    cpu.pc = 0x5DD56;
    16
}
/// $0780F8: bcs.w $78128
pub fn i_0780f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x780FC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78128; return 10; }
    8
}
/// $0780FC: lea.l $ffff.w, a0
pub fn i_0780fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78100;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $078100: move.l a0, $48(a6)
pub fn i_078100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78104;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078104: lea.l $7810a(pc), a1
pub fn i_078104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78108;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7810A;
    16
}
/// $078108: move.l a1, (a6)
pub fn i_078108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7810A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07810A: jsr $44048.l
pub fn i_07810a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78110;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78110);
    cpu.pc = 0x44048;
    16
}
/// $078110: jsr $28d70.l
pub fn i_078110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78116;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78116);
    cpu.pc = 0x28D70;
    16
}
/// $078116: bcs.w $78128
pub fn i_078116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7811A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78128; return 10; }
    8
}
/// $07811A: movea.l $70(a6), a0
pub fn i_07811a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7811E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07811E: jsr $5dd56.l
pub fn i_07811e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78124;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78124);
    cpu.pc = 0x5DD56;
    16
}
/// $078124: bcc.w $7812e
pub fn i_078124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78128;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7812E; return 10; }
    8
}
/// $078128: jmp $518.l
pub fn i_078128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7812E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07812E: rts
pub fn i_07812e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78130;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Explosion_StdAttack_078130  ($078130, 42 B, props/fx_explosions_platforms_crew_076000.s)
/// $078130: lea.l $2dd4ba.l, a0
pub fn i_078130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78136;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD4BA;
    20
}
/// $078136: jsr $28cd4.l
pub fn i_078136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7813C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7813C);
    cpu.pc = 0x28CD4;
    16
}
/// $07813C: lea.l $2df1fc.l, a0
pub fn i_07813c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78142;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF1FC;
    20
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x076188, i_076188),
    (0x07618C, i_07618c),
    (0x076190, i_076190),
    (0x076194, i_076194),
    (0x076196, i_076196),
    (0x07619A, i_07619a),
    (0x0761A0, i_0761a0),
    (0x0761A4, i_0761a4),
    (0x0761A8, i_0761a8),
    (0x0761AC, i_0761ac),
    (0x0761B0, i_0761b0),
    (0x0761B4, i_0761b4),
    (0x0761B8, i_0761b8),
    (0x0761C0, i_0761c0),
    (0x0761C4, i_0761c4),
    (0x0761C8, i_0761c8),
    (0x0761CC, i_0761cc),
    (0x0761D2, i_0761d2),
    (0x0761D6, i_0761d6),
    (0x0761DC, i_0761dc),
    (0x0761E2, i_0761e2),
    (0x0761E6, i_0761e6),
    (0x0761EA, i_0761ea),
    (0x0761F0, i_0761f0),
    (0x0761F4, i_0761f4),
    (0x0761F8, i_0761f8),
    (0x0761FA, i_0761fa),
    (0x0761FE, i_0761fe),
    (0x076204, i_076204),
    (0x076208, i_076208),
    (0x07620C, i_07620c),
    (0x076210, i_076210),
    (0x076214, i_076214),
    (0x076218, i_076218),
    (0x07621C, i_07621c),
    (0x07621E, i_07621e),
    (0x076222, i_076222),
    (0x076226, i_076226),
    (0x076228, i_076228),
    (0x07622C, i_07622c),
    (0x076232, i_076232),
    (0x076236, i_076236),
    (0x07623A, i_07623a),
    (0x07623E, i_07623e),
    (0x076242, i_076242),
    (0x076246, i_076246),
    (0x07624C, i_07624c),
    (0x076250, i_076250),
    (0x076258, i_076258),
    (0x07625C, i_07625c),
    (0x076260, i_076260),
    (0x076264, i_076264),
    (0x07626A, i_07626a),
    (0x07626E, i_07626e),
    (0x076270, i_076270),
    (0x076274, i_076274),
    (0x07627A, i_07627a),
    (0x07627E, i_07627e),
    (0x076282, i_076282),
    (0x076286, i_076286),
    (0x07628A, i_07628a),
    (0x07628C, i_07628c),
    (0x076290, i_076290),
    (0x076296, i_076296),
    (0x07629A, i_07629a),
    (0x07629E, i_07629e),
    (0x0762A2, i_0762a2),
    (0x0762A6, i_0762a6),
    (0x0762AA, i_0762aa),
    (0x0762AE, i_0762ae),
    (0x0762B6, i_0762b6),
    (0x0762BA, i_0762ba),
    (0x0762BE, i_0762be),
    (0x0762C2, i_0762c2),
    (0x0762C8, i_0762c8),
    (0x0762CC, i_0762cc),
    (0x0762D2, i_0762d2),
    (0x0762D8, i_0762d8),
    (0x0762DC, i_0762dc),
    (0x0762E0, i_0762e0),
    (0x0762E6, i_0762e6),
    (0x0762EA, i_0762ea),
    (0x0762EE, i_0762ee),
    (0x0762F0, i_0762f0),
    (0x0762F4, i_0762f4),
    (0x0762FA, i_0762fa),
    (0x0762FE, i_0762fe),
    (0x076302, i_076302),
    (0x076306, i_076306),
    (0x07630A, i_07630a),
    (0x07630E, i_07630e),
    (0x076312, i_076312),
    (0x076314, i_076314),
    (0x076318, i_076318),
    (0x07631C, i_07631c),
    (0x07631E, i_07631e),
    (0x076322, i_076322),
    (0x076328, i_076328),
    (0x07632C, i_07632c),
    (0x076332, i_076332),
    (0x076336, i_076336),
    (0x076338, i_076338),
    (0x07633C, i_07633c),
    (0x076342, i_076342),
    (0x076346, i_076346),
    (0x07634A, i_07634a),
    (0x07634C, i_07634c),
    (0x076350, i_076350),
    (0x076356, i_076356),
    (0x07635A, i_07635a),
    (0x076360, i_076360),
    (0x076366, i_076366),
    (0x07636A, i_07636a),
    (0x07636C, i_07636c),
    (0x076370, i_076370),
    (0x076376, i_076376),
    (0x07637A, i_07637a),
    (0x07637E, i_07637e),
    (0x076382, i_076382),
    (0x076386, i_076386),
    (0x076388, i_076388),
    (0x07638C, i_07638c),
    (0x076390, i_076390),
    (0x076396, i_076396),
    (0x07639A, i_07639a),
    (0x0763A0, i_0763a0),
    (0x0763A4, i_0763a4),
    (0x0763AA, i_0763aa),
    (0x0763AE, i_0763ae),
    (0x0763B0, i_0763b0),
    (0x0763B4, i_0763b4),
    (0x0763BA, i_0763ba),
    (0x0763BE, i_0763be),
    (0x0763C2, i_0763c2),
    (0x0763C4, i_0763c4),
    (0x0763C8, i_0763c8),
    (0x0763CE, i_0763ce),
    (0x0763D2, i_0763d2),
    (0x0763D8, i_0763d8),
    (0x0763DC, i_0763dc),
    (0x0763DE, i_0763de),
    (0x0763E2, i_0763e2),
    (0x0763E8, i_0763e8),
    (0x0763EC, i_0763ec),
    (0x0763F0, i_0763f0),
    (0x0763F2, i_0763f2),
    (0x0763F6, i_0763f6),
    (0x0763FC, i_0763fc),
    (0x076400, i_076400),
    (0x076406, i_076406),
    (0x07640C, i_07640c),
    (0x076410, i_076410),
    (0x076412, i_076412),
    (0x076416, i_076416),
    (0x07641C, i_07641c),
    (0x076420, i_076420),
    (0x076424, i_076424),
    (0x076428, i_076428),
    (0x07642C, i_07642c),
    (0x07642E, i_07642e),
    (0x076432, i_076432),
    (0x076436, i_076436),
    (0x07643C, i_07643c),
    (0x076440, i_076440),
    (0x076446, i_076446),
    (0x07644A, i_07644a),
    (0x076450, i_076450),
    (0x076454, i_076454),
    (0x076456, i_076456),
    (0x07645A, i_07645a),
    (0x076460, i_076460),
    (0x076464, i_076464),
    (0x076468, i_076468),
    (0x07646A, i_07646a),
    (0x07646E, i_07646e),
    (0x076474, i_076474),
    (0x07647A, i_07647a),
    (0x07647E, i_07647e),
    (0x076484, i_076484),
    (0x076488, i_076488),
    (0x07648A, i_07648a),
    (0x07648E, i_07648e),
    (0x076494, i_076494),
    (0x076498, i_076498),
    (0x07649C, i_07649c),
    (0x0764A2, i_0764a2),
    (0x0764A8, i_0764a8),
    (0x0764AE, i_0764ae),
    (0x0764B0, i_0764b0),
    (0x0764B4, i_0764b4),
    (0x0764BA, i_0764ba),
    (0x0764BE, i_0764be),
    (0x0764C4, i_0764c4),
    (0x0764CA, i_0764ca),
    (0x0764D0, i_0764d0),
    (0x0764D4, i_0764d4),
    (0x0764D8, i_0764d8),
    (0x0764DC, i_0764dc),
    (0x0764E2, i_0764e2),
    (0x0764E8, i_0764e8),
    (0x0764EC, i_0764ec),
    (0x0764F0, i_0764f0),
    (0x0764F4, i_0764f4),
    (0x0764F8, i_0764f8),
    (0x0764FE, i_0764fe),
    (0x076502, i_076502),
    (0x076508, i_076508),
    (0x07650E, i_07650e),
    (0x076512, i_076512),
    (0x076518, i_076518),
    (0x076520, i_076520),
    (0x076526, i_076526),
    (0x07652C, i_07652c),
    (0x076534, i_076534),
    (0x076536, i_076536),
    (0x07653A, i_07653a),
    (0x076540, i_076540),
    (0x076544, i_076544),
    (0x076548, i_076548),
    (0x07654C, i_07654c),
    (0x076550, i_076550),
    (0x076554, i_076554),
    (0x076558, i_076558),
    (0x076560, i_076560),
    (0x076566, i_076566),
    (0x07656C, i_07656c),
    (0x076574, i_076574),
    (0x076766, i_076766),
    (0x07676A, i_07676a),
    (0x076770, i_076770),
    (0x076776, i_076776),
    (0x07677A, i_07677a),
    (0x07677E, i_07677e),
    (0x076784, i_076784),
    (0x076788, i_076788),
    (0x07678C, i_07678c),
    (0x076792, i_076792),
    (0x076796, i_076796),
    (0x076798, i_076798),
    (0x07679C, i_07679c),
    (0x0767A0, i_0767a0),
    (0x0767A4, i_0767a4),
    (0x0767A8, i_0767a8),
    (0x0767AC, i_0767ac),
    (0x0767B0, i_0767b0),
    (0x0767B4, i_0767b4),
    (0x0767BA, i_0767ba),
    (0x0767BE, i_0767be),
    (0x0767C4, i_0767c4),
    (0x0767CA, i_0767ca),
    (0x0767CE, i_0767ce),
    (0x0767D4, i_0767d4),
    (0x0767D8, i_0767d8),
    (0x0767DC, i_0767dc),
    (0x0767E0, i_0767e0),
    (0x0767E2, i_0767e2),
    (0x0767E6, i_0767e6),
    (0x0767EA, i_0767ea),
    (0x0767F0, i_0767f0),
    (0x0767F2, i_0767f2),
    (0x0767F4, i_0767f4),
    (0x0767F8, i_0767f8),
    (0x0767FE, i_0767fe),
    (0x076802, i_076802),
    (0x076808, i_076808),
    (0x07680E, i_07680e),
    (0x076812, i_076812),
    (0x076816, i_076816),
    (0x07681C, i_07681c),
    (0x076822, i_076822),
    (0x076828, i_076828),
    (0x07682A, i_07682a),
    (0x07682E, i_07682e),
    (0x076832, i_076832),
    (0x076836, i_076836),
    (0x076838, i_076838),
    (0x07683E, i_07683e),
    (0x076844, i_076844),
    (0x076848, i_076848),
    (0x07684C, i_07684c),
    (0x076850, i_076850),
    (0x076854, i_076854),
    (0x07685A, i_07685a),
    (0x076860, i_076860),
    (0x07690A, i_07690a),
    (0x076910, i_076910),
    (0x076914, i_076914),
    (0x07691A, i_07691a),
    (0x07691E, i_07691e),
    (0x076924, i_076924),
    (0x076928, i_076928),
    (0x07692A, i_07692a),
    (0x07692C, i_07692c),
    (0x076930, i_076930),
    (0x076934, i_076934),
    (0x076938, i_076938),
    (0x07693C, i_07693c),
    (0x07693E, i_07693e),
    (0x076942, i_076942),
    (0x076946, i_076946),
    (0x07694A, i_07694a),
    (0x07694E, i_07694e),
    (0x076952, i_076952),
    (0x076958, i_076958),
    (0x07695A, i_07695a),
    (0x07695E, i_07695e),
    (0x076964, i_076964),
    (0x076968, i_076968),
    (0x07696C, i_07696c),
    (0x076970, i_076970),
    (0x076974, i_076974),
    (0x076976, i_076976),
    (0x07697A, i_07697a),
    (0x07697E, i_07697e),
    (0x076984, i_076984),
    (0x07698A, i_07698a),
    (0x076990, i_076990),
    (0x076996, i_076996),
    (0x07699A, i_07699a),
    (0x0769A0, i_0769a0),
    (0x076A0E, i_076a0e),
    (0x076A12, i_076a12),
    (0x076A18, i_076a18),
    (0x076A1C, i_076a1c),
    (0x076A22, i_076a22),
    (0x076A28, i_076a28),
    (0x076A2A, i_076a2a),
    (0x076A2C, i_076a2c),
    (0x076A30, i_076a30),
    (0x076A32, i_076a32),
    (0x076A36, i_076a36),
    (0x076A3A, i_076a3a),
    (0x076A3E, i_076a3e),
    (0x076A42, i_076a42),
    (0x076A46, i_076a46),
    (0x076A4C, i_076a4c),
    (0x076A50, i_076a50),
    (0x076A54, i_076a54),
    (0x076A58, i_076a58),
    (0x076A5E, i_076a5e),
    (0x076A62, i_076a62),
    (0x076A68, i_076a68),
    (0x076A6E, i_076a6e),
    (0x076A74, i_076a74),
    (0x076A7A, i_076a7a),
    (0x076A7E, i_076a7e),
    (0x076A82, i_076a82),
    (0x076A88, i_076a88),
    (0x076A8C, i_076a8c),
    (0x076A8E, i_076a8e),
    (0x076A90, i_076a90),
    (0x076A94, i_076a94),
    (0x076A9A, i_076a9a),
    (0x076A9E, i_076a9e),
    (0x076AA4, i_076aa4),
    (0x076AA8, i_076aa8),
    (0x076AAA, i_076aaa),
    (0x076AAE, i_076aae),
    (0x076AB2, i_076ab2),
    (0x076AB6, i_076ab6),
    (0x076ABA, i_076aba),
    (0x076ABE, i_076abe),
    (0x076AC4, i_076ac4),
    (0x076AC8, i_076ac8),
    (0x076ACC, i_076acc),
    (0x076AD0, i_076ad0),
    (0x076AD6, i_076ad6),
    (0x076ADC, i_076adc),
    (0x076ADE, i_076ade),
    (0x076AE2, i_076ae2),
    (0x076AE6, i_076ae6),
    (0x076AE8, i_076ae8),
    (0x076AEC, i_076aec),
    (0x076AF0, i_076af0),
    (0x076AF4, i_076af4),
    (0x076AF8, i_076af8),
    (0x076AFC, i_076afc),
    (0x076B02, i_076b02),
    (0x076B06, i_076b06),
    (0x076B0A, i_076b0a),
    (0x076B0E, i_076b0e),
    (0x076B14, i_076b14),
    (0x076B18, i_076b18),
    (0x076B1E, i_076b1e),
    (0x076B24, i_076b24),
    (0x076B28, i_076b28),
    (0x076B2C, i_076b2c),
    (0x076B32, i_076b32),
    (0x076B36, i_076b36),
    (0x076B38, i_076b38),
    (0x076B3A, i_076b3a),
    (0x076B3E, i_076b3e),
    (0x076B44, i_076b44),
    (0x076B48, i_076b48),
    (0x076B4E, i_076b4e),
    (0x076B52, i_076b52),
    (0x076B58, i_076b58),
    (0x076B5C, i_076b5c),
    (0x076B62, i_076b62),
    (0x076B66, i_076b66),
    (0x076B6C, i_076b6c),
    (0x076B70, i_076b70),
    (0x076B72, i_076b72),
    (0x076B76, i_076b76),
    (0x076B7A, i_076b7a),
    (0x076B7E, i_076b7e),
    (0x076B84, i_076b84),
    (0x076B88, i_076b88),
    (0x076B8A, i_076b8a),
    (0x076B8E, i_076b8e),
    (0x076B92, i_076b92),
    (0x076B96, i_076b96),
    (0x076B9A, i_076b9a),
    (0x076B9E, i_076b9e),
    (0x076BA4, i_076ba4),
    (0x076BA6, i_076ba6),
    (0x076BAA, i_076baa),
    (0x076BB0, i_076bb0),
    (0x076BB4, i_076bb4),
    (0x076BBA, i_076bba),
    (0x076BC0, i_076bc0),
    (0x076BC8, i_076bc8),
    (0x076BCC, i_076bcc),
    (0x076BCE, i_076bce),
    (0x076BD0, i_076bd0),
    (0x076BD4, i_076bd4),
    (0x076BD8, i_076bd8),
    (0x076BDC, i_076bdc),
    (0x076BE0, i_076be0),
    (0x076BE4, i_076be4),
    (0x076BE8, i_076be8),
    (0x076BEC, i_076bec),
    (0x076BF2, i_076bf2),
    (0x076BF6, i_076bf6),
    (0x076BFC, i_076bfc),
    (0x076C04, i_076c04),
    (0x076C0A, i_076c0a),
    (0x076C0E, i_076c0e),
    (0x076C14, i_076c14),
    (0x076C18, i_076c18),
    (0x076C1A, i_076c1a),
    (0x076C1E, i_076c1e),
    (0x076C22, i_076c22),
    (0x076C26, i_076c26),
    (0x076C2C, i_076c2c),
    (0x076C32, i_076c32),
    (0x076C38, i_076c38),
    (0x076C3A, i_076c3a),
    (0x076C3C, i_076c3c),
    (0x076C40, i_076c40),
    (0x076C44, i_076c44),
    (0x076C48, i_076c48),
    (0x076C4C, i_076c4c),
    (0x076C50, i_076c50),
    (0x076C54, i_076c54),
    (0x076C58, i_076c58),
    (0x076C5C, i_076c5c),
    (0x076C62, i_076c62),
    (0x076C66, i_076c66),
    (0x076C6C, i_076c6c),
    (0x076C72, i_076c72),
    (0x076CD4, i_076cd4),
    (0x076CD8, i_076cd8),
    (0x076CDE, i_076cde),
    (0x076CE4, i_076ce4),
    (0x076CEA, i_076cea),
    (0x076CEE, i_076cee),
    (0x076CF4, i_076cf4),
    (0x076CF8, i_076cf8),
    (0x076CFA, i_076cfa),
    (0x076CFE, i_076cfe),
    (0x076D02, i_076d02),
    (0x076D06, i_076d06),
    (0x076D0A, i_076d0a),
    (0x076D0E, i_076d0e),
    (0x076D14, i_076d14),
    (0x076DD4, i_076dd4),
    (0x076DD8, i_076dd8),
    (0x076DDE, i_076dde),
    (0x076DE4, i_076de4),
    (0x076DE8, i_076de8),
    (0x076DEC, i_076dec),
    (0x076DEE, i_076dee),
    (0x076DF2, i_076df2),
    (0x076DF6, i_076df6),
    (0x076DFA, i_076dfa),
    (0x076DFC, i_076dfc),
    (0x076DFE, i_076dfe),
    (0x076E02, i_076e02),
    (0x076E04, i_076e04),
    (0x076E08, i_076e08),
    (0x076E0E, i_076e0e),
    (0x076E10, i_076e10),
    (0x076E14, i_076e14),
    (0x076E1A, i_076e1a),
    (0x076E20, i_076e20),
    (0x076E24, i_076e24),
    (0x076E28, i_076e28),
    (0x076E2E, i_076e2e),
    (0x076E32, i_076e32),
    (0x076E36, i_076e36),
    (0x076E3C, i_076e3c),
    (0x076E3E, i_076e3e),
    (0x076E42, i_076e42),
    (0x076E48, i_076e48),
    (0x076E4C, i_076e4c),
    (0x076E4E, i_076e4e),
    (0x076E50, i_076e50),
    (0x076E54, i_076e54),
    (0x076E58, i_076e58),
    (0x076E5C, i_076e5c),
    (0x076E60, i_076e60),
    (0x076E64, i_076e64),
    (0x076E68, i_076e68),
    (0x076E6C, i_076e6c),
    (0x076E72, i_076e72),
    (0x076E76, i_076e76),
    (0x076E7A, i_076e7a),
    (0x076E7C, i_076e7c),
    (0x076E7E, i_076e7e),
    (0x076E82, i_076e82),
    (0x076E88, i_076e88),
    (0x076E8C, i_076e8c),
    (0x076E90, i_076e90),
    (0x076E92, i_076e92),
    (0x076E98, i_076e98),
    (0x076E9C, i_076e9c),
    (0x076E9E, i_076e9e),
    (0x076EA0, i_076ea0),
    (0x076EA4, i_076ea4),
    (0x076EA6, i_076ea6),
    (0x076EA8, i_076ea8),
    (0x076EAC, i_076eac),
    (0x076EAE, i_076eae),
    (0x076EB2, i_076eb2),
    (0x076EB8, i_076eb8),
    (0x076EBE, i_076ebe),
    (0x076EC0, i_076ec0),
    (0x076EC4, i_076ec4),
    (0x076EC8, i_076ec8),
    (0x076ECC, i_076ecc),
    (0x076ED0, i_076ed0),
    (0x076ED4, i_076ed4),
    (0x076ED8, i_076ed8),
    (0x076EDE, i_076ede),
    (0x076EE4, i_076ee4),
    (0x076EEA, i_076eea),
    (0x076EEE, i_076eee),
    (0x076EF4, i_076ef4),
    (0x076EF8, i_076ef8),
    (0x076EFA, i_076efa),
    (0x076F00, i_076f00),
    (0x076F04, i_076f04),
    (0x076F08, i_076f08),
    (0x076F0E, i_076f0e),
    (0x076F14, i_076f14),
    (0x076F1A, i_076f1a),
    (0x076F20, i_076f20),
    (0x076F24, i_076f24),
    (0x076F2A, i_076f2a),
    (0x076F2E, i_076f2e),
    (0x076F32, i_076f32),
    (0x076F36, i_076f36),
    (0x076F3A, i_076f3a),
    (0x076F40, i_076f40),
    (0x076F42, i_076f42),
    (0x076F46, i_076f46),
    (0x076F4A, i_076f4a),
    (0x076F50, i_076f50),
    (0x076F54, i_076f54),
    (0x076F56, i_076f56),
    (0x076F5C, i_076f5c),
    (0x076F60, i_076f60),
    (0x076F66, i_076f66),
    (0x076F6C, i_076f6c),
    (0x076F70, i_076f70),
    (0x076F76, i_076f76),
    (0x076F78, i_076f78),
    (0x076F7C, i_076f7c),
    (0x076F82, i_076f82),
    (0x076F86, i_076f86),
    (0x076F8C, i_076f8c),
    (0x076F90, i_076f90),
    (0x076F96, i_076f96),
    (0x076F9C, i_076f9c),
    (0x076F9E, i_076f9e),
    (0x076FA2, i_076fa2),
    (0x076FA6, i_076fa6),
    (0x076FAA, i_076faa),
    (0x076FB0, i_076fb0),
    (0x076FB2, i_076fb2),
    (0x076FB4, i_076fb4),
    (0x076FB6, i_076fb6),
    (0x076FBA, i_076fba),
    (0x076FBC, i_076fbc),
    (0x076FBE, i_076fbe),
    (0x076FC0, i_076fc0),
    (0x076FC2, i_076fc2),
    (0x076FC4, i_076fc4),
    (0x076FC8, i_076fc8),
    (0x076FCA, i_076fca),
    (0x076FCC, i_076fcc),
    (0x076FCE, i_076fce),
    (0x076FD2, i_076fd2),
    (0x076FD6, i_076fd6),
    (0x076FDA, i_076fda),
    (0x076FDE, i_076fde),
    (0x076FE2, i_076fe2),
    (0x076FE4, i_076fe4),
    (0x076FE8, i_076fe8),
    (0x076FEA, i_076fea),
    (0x076FEE, i_076fee),
    (0x076FF2, i_076ff2),
    (0x076FF6, i_076ff6),
    (0x076FFA, i_076ffa),
    (0x076FFE, i_076ffe),
    (0x077000, i_077000),
    (0x077002, i_077002),
    (0x077004, i_077004),
    (0x077006, i_077006),
    (0x07700A, i_07700a),
    (0x07700C, i_07700c),
    (0x07700E, i_07700e),
    (0x077010, i_077010),
    (0x077012, i_077012),
    (0x077014, i_077014),
    (0x077018, i_077018),
    (0x07701C, i_07701c),
    (0x077020, i_077020),
    (0x077022, i_077022),
    (0x077024, i_077024),
    (0x077028, i_077028),
    (0x07702A, i_07702a),
    (0x07702E, i_07702e),
    (0x077030, i_077030),
    (0x077032, i_077032),
    (0x077036, i_077036),
    (0x077038, i_077038),
    (0x07703A, i_07703a),
    (0x07703C, i_07703c),
    (0x07703E, i_07703e),
    (0x077040, i_077040),
    (0x077042, i_077042),
    (0x077044, i_077044),
    (0x077048, i_077048),
    (0x07704A, i_07704a),
    (0x07704C, i_07704c),
    (0x077050, i_077050),
    (0x077052, i_077052),
    (0x077054, i_077054),
    (0x077056, i_077056),
    (0x077058, i_077058),
    (0x07705A, i_07705a),
    (0x07705C, i_07705c),
    (0x07705E, i_07705e),
    (0x077062, i_077062),
    (0x077066, i_077066),
    (0x07706A, i_07706a),
    (0x07706E, i_07706e),
    (0x077072, i_077072),
    (0x077076, i_077076),
    (0x077078, i_077078),
    (0x07707A, i_07707a),
    (0x07707C, i_07707c),
    (0x077082, i_077082),
    (0x077086, i_077086),
    (0x07708C, i_07708c),
    (0x077090, i_077090),
    (0x077092, i_077092),
    (0x077098, i_077098),
    (0x0770A0, i_0770a0),
    (0x0770A4, i_0770a4),
    (0x0770AA, i_0770aa),
    (0x0770AE, i_0770ae),
    (0x0770B0, i_0770b0),
    (0x0770B6, i_0770b6),
    (0x0770BE, i_0770be),
    (0x0770C2, i_0770c2),
    (0x0770C8, i_0770c8),
    (0x0770CC, i_0770cc),
    (0x0770D2, i_0770d2),
    (0x0770DA, i_0770da),
    (0x0770DE, i_0770de),
    (0x0770E4, i_0770e4),
    (0x0770E8, i_0770e8),
    (0x0770EC, i_0770ec),
    (0x0770F2, i_0770f2),
    (0x0770F6, i_0770f6),
    (0x0770F8, i_0770f8),
    (0x0770FE, i_0770fe),
    (0x077102, i_077102),
    (0x077104, i_077104),
    (0x077108, i_077108),
    (0x07710A, i_07710a),
    (0x07710E, i_07710e),
    (0x077112, i_077112),
    (0x077114, i_077114),
    (0x077116, i_077116),
    (0x077118, i_077118),
    (0x07711A, i_07711a),
    (0x07711C, i_07711c),
    (0x07711E, i_07711e),
    (0x077120, i_077120),
    (0x077124, i_077124),
    (0x077126, i_077126),
    (0x077128, i_077128),
    (0x07712C, i_07712c),
    (0x07712E, i_07712e),
    (0x077130, i_077130),
    (0x077134, i_077134),
    (0x077136, i_077136),
    (0x077138, i_077138),
    (0x07713C, i_07713c),
    (0x07713E, i_07713e),
    (0x077142, i_077142),
    (0x077144, i_077144),
    (0x077146, i_077146),
    (0x077148, i_077148),
    (0x07714E, i_07714e),
    (0x077156, i_077156),
    (0x07715A, i_07715a),
    (0x077160, i_077160),
    (0x077164, i_077164),
    (0x077168, i_077168),
    (0x07716E, i_07716e),
    (0x077172, i_077172),
    (0x077174, i_077174),
    (0x07717A, i_07717a),
    (0x07717E, i_07717e),
    (0x077180, i_077180),
    (0x077182, i_077182),
    (0x077188, i_077188),
    (0x07718A, i_07718a),
    (0x07718E, i_07718e),
    (0x077190, i_077190),
    (0x077196, i_077196),
    (0x07719E, i_07719e),
    (0x0771A2, i_0771a2),
    (0x0771A8, i_0771a8),
    (0x0771AC, i_0771ac),
    (0x0771AE, i_0771ae),
    (0x0771B2, i_0771b2),
    (0x0771B4, i_0771b4),
    (0x0771B8, i_0771b8),
    (0x0771BE, i_0771be),
    (0x0771C0, i_0771c0),
    (0x0771C4, i_0771c4),
    (0x0771C6, i_0771c6),
    (0x0771CA, i_0771ca),
    (0x0771CC, i_0771cc),
    (0x0771CE, i_0771ce),
    (0x0771D0, i_0771d0),
    (0x0771D2, i_0771d2),
    (0x0771D6, i_0771d6),
    (0x0771DA, i_0771da),
    (0x0771DE, i_0771de),
    (0x0771E0, i_0771e0),
    (0x0771E4, i_0771e4),
    (0x0771E6, i_0771e6),
    (0x0771EA, i_0771ea),
    (0x0771EE, i_0771ee),
    (0x0771F2, i_0771f2),
    (0x0771F4, i_0771f4),
    (0x0771F6, i_0771f6),
    (0x0771F8, i_0771f8),
    (0x0771FE, i_0771fe),
    (0x077202, i_077202),
    (0x077204, i_077204),
    (0x077208, i_077208),
    (0x07720C, i_07720c),
    (0x077212, i_077212),
    (0x077216, i_077216),
    (0x077218, i_077218),
    (0x07721A, i_07721a),
    (0x077220, i_077220),
    (0x077222, i_077222),
    (0x077224, i_077224),
    (0x077228, i_077228),
    (0x07722C, i_07722c),
    (0x077232, i_077232),
    (0x077236, i_077236),
    (0x07723A, i_07723a),
    (0x07723E, i_07723e),
    (0x077240, i_077240),
    (0x077246, i_077246),
    (0x07724A, i_07724a),
    (0x07724E, i_07724e),
    (0x077252, i_077252),
    (0x077258, i_077258),
    (0x07725A, i_07725a),
    (0x07725C, i_07725c),
    (0x07725E, i_07725e),
    (0x077262, i_077262),
    (0x077266, i_077266),
    (0x077268, i_077268),
    (0x07726A, i_07726a),
    (0x07726E, i_07726e),
    (0x077274, i_077274),
    (0x077278, i_077278),
    (0x07727C, i_07727c),
    (0x077280, i_077280),
    (0x077286, i_077286),
    (0x077288, i_077288),
    (0x07728C, i_07728c),
    (0x077290, i_077290),
    (0x077294, i_077294),
    (0x077298, i_077298),
    (0x07729C, i_07729c),
    (0x0772A2, i_0772a2),
    (0x0772A6, i_0772a6),
    (0x07749E, i_07749e),
    (0x0774A2, i_0774a2),
    (0x0774A6, i_0774a6),
    (0x0774AA, i_0774aa),
    (0x0774AC, i_0774ac),
    (0x0774AE, i_0774ae),
    (0x0774B4, i_0774b4),
    (0x0774B8, i_0774b8),
    (0x0774BC, i_0774bc),
    (0x0774BE, i_0774be),
    (0x0774C2, i_0774c2),
    (0x0774C8, i_0774c8),
    (0x0774CC, i_0774cc),
    (0x0774D0, i_0774d0),
    (0x0774D4, i_0774d4),
    (0x0774D8, i_0774d8),
    (0x0774DC, i_0774dc),
    (0x0774E0, i_0774e0),
    (0x0774E4, i_0774e4),
    (0x0774EA, i_0774ea),
    (0x0774EC, i_0774ec),
    (0x0774F4, i_0774f4),
    (0x0774F6, i_0774f6),
    (0x0774FC, i_0774fc),
    (0x077500, i_077500),
    (0x077508, i_077508),
    (0x07750A, i_07750a),
    (0x077510, i_077510),
    (0x077514, i_077514),
    (0x07751C, i_07751c),
    (0x07751E, i_07751e),
    (0x077524, i_077524),
    (0x077528, i_077528),
    (0x077530, i_077530),
    (0x077534, i_077534),
    (0x07753A, i_07753a),
    (0x07753E, i_07753e),
    (0x077546, i_077546),
    (0x07754A, i_07754a),
    (0x077550, i_077550),
    (0x077554, i_077554),
    (0x07755C, i_07755c),
    (0x077560, i_077560),
    (0x077566, i_077566),
    (0x07756A, i_07756a),
    (0x077572, i_077572),
    (0x077576, i_077576),
    (0x07757C, i_07757c),
    (0x077580, i_077580),
    (0x077584, i_077584),
    (0x07758A, i_07758a),
    (0x07758E, i_07758e),
    (0x077590, i_077590),
    (0x077596, i_077596),
    (0x07759A, i_07759a),
    (0x07759E, i_07759e),
    (0x0775A0, i_0775a0),
    (0x0775A6, i_0775a6),
    (0x0775AC, i_0775ac),
    (0x0775B0, i_0775b0),
    (0x0775B4, i_0775b4),
    (0x0775B8, i_0775b8),
    (0x0775BC, i_0775bc),
    (0x0775C0, i_0775c0),
    (0x0775C2, i_0775c2),
    (0x0775C6, i_0775c6),
    (0x0775C8, i_0775c8),
    (0x0775CC, i_0775cc),
    (0x0775D2, i_0775d2),
    (0x0775D8, i_0775d8),
    (0x0775DA, i_0775da),
    (0x0775DE, i_0775de),
    (0x0775E4, i_0775e4),
    (0x0775EA, i_0775ea),
    (0x077640, i_077640),
    (0x077648, i_077648),
    (0x07764C, i_07764c),
    (0x077654, i_077654),
    (0x077658, i_077658),
    (0x07765E, i_07765e),
    (0x077662, i_077662),
    (0x077668, i_077668),
    (0x07766E, i_07766e),
    (0x077670, i_077670),
    (0x077674, i_077674),
    (0x07767C, i_07767c),
    (0x077680, i_077680),
    (0x077684, i_077684),
    (0x077686, i_077686),
    (0x07768A, i_07768a),
    (0x077692, i_077692),
    (0x07769A, i_07769a),
    (0x0776A2, i_0776a2),
    (0x0776AA, i_0776aa),
    (0x0776AE, i_0776ae),
    (0x0776B0, i_0776b0),
    (0x0776B4, i_0776b4),
    (0x0776B8, i_0776b8),
    (0x0776C0, i_0776c0),
    (0x0776C8, i_0776c8),
    (0x0776D0, i_0776d0),
    (0x0776D8, i_0776d8),
    (0x0776DC, i_0776dc),
    (0x0776DE, i_0776de),
    (0x0776E2, i_0776e2),
    (0x0776E6, i_0776e6),
    (0x0776EE, i_0776ee),
    (0x0776F6, i_0776f6),
    (0x0776FE, i_0776fe),
    (0x077706, i_077706),
    (0x07770A, i_07770a),
    (0x07770C, i_07770c),
    (0x077710, i_077710),
    (0x077714, i_077714),
    (0x07771C, i_07771c),
    (0x077724, i_077724),
    (0x07772C, i_07772c),
    (0x077734, i_077734),
    (0x077738, i_077738),
    (0x07773A, i_07773a),
    (0x07773E, i_07773e),
    (0x077742, i_077742),
    (0x07774A, i_07774a),
    (0x077752, i_077752),
    (0x07775A, i_07775a),
    (0x077762, i_077762),
    (0x077766, i_077766),
    (0x077768, i_077768),
    (0x07776C, i_07776c),
    (0x077772, i_077772),
    (0x077776, i_077776),
    (0x077778, i_077778),
    (0x07777C, i_07777c),
    (0x077780, i_077780),
    (0x077784, i_077784),
    (0x077788, i_077788),
    (0x07778C, i_07778c),
    (0x077790, i_077790),
    (0x077794, i_077794),
    (0x077798, i_077798),
    (0x07779A, i_07779a),
    (0x07779E, i_07779e),
    (0x0777A2, i_0777a2),
    (0x0777A6, i_0777a6),
    (0x0777AA, i_0777aa),
    (0x0777AE, i_0777ae),
    (0x0777B2, i_0777b2),
    (0x0777B8, i_0777b8),
    (0x0777BC, i_0777bc),
    (0x0777C2, i_0777c2),
    (0x0777C6, i_0777c6),
    (0x0777CC, i_0777cc),
    (0x0777D0, i_0777d0),
    (0x0777D2, i_0777d2),
    (0x0777D6, i_0777d6),
    (0x0777DA, i_0777da),
    (0x0777DE, i_0777de),
    (0x0777E4, i_0777e4),
    (0x0777E6, i_0777e6),
    (0x0777EA, i_0777ea),
    (0x0777EE, i_0777ee),
    (0x0777F2, i_0777f2),
    (0x0777F6, i_0777f6),
    (0x0777FA, i_0777fa),
    (0x0777FC, i_0777fc),
    (0x077800, i_077800),
    (0x077804, i_077804),
    (0x077808, i_077808),
    (0x07780C, i_07780c),
    (0x07780E, i_07780e),
    (0x077812, i_077812),
    (0x077818, i_077818),
    (0x07781C, i_07781c),
    (0x077822, i_077822),
    (0x077828, i_077828),
    (0x07782C, i_07782c),
    (0x077832, i_077832),
    (0x077836, i_077836),
    (0x07783C, i_07783c),
    (0x077840, i_077840),
    (0x077844, i_077844),
    (0x077848, i_077848),
    (0x07784E, i_07784e),
    (0x077850, i_077850),
    (0x077856, i_077856),
    (0x07785C, i_07785c),
    (0x077860, i_077860),
    (0x077864, i_077864),
    (0x077868, i_077868),
    (0x07786C, i_07786c),
    (0x07786E, i_07786e),
    (0x077872, i_077872),
    (0x077878, i_077878),
    (0x07787E, i_07787e),
    (0x077884, i_077884),
    (0x077888, i_077888),
    (0x07788C, i_07788c),
    (0x077890, i_077890),
    (0x077896, i_077896),
    (0x07789A, i_07789a),
    (0x07789E, i_07789e),
    (0x0778A0, i_0778a0),
    (0x0778A2, i_0778a2),
    (0x0778A6, i_0778a6),
    (0x0778AA, i_0778aa),
    (0x0778AE, i_0778ae),
    (0x0778B2, i_0778b2),
    (0x0778B6, i_0778b6),
    (0x0778BA, i_0778ba),
    (0x0778BE, i_0778be),
    (0x0778C2, i_0778c2),
    (0x0778C8, i_0778c8),
    (0x0778CC, i_0778cc),
    (0x0778D2, i_0778d2),
    (0x0778D6, i_0778d6),
    (0x0778DA, i_0778da),
    (0x0778DC, i_0778dc),
    (0x0778E0, i_0778e0),
    (0x0778E4, i_0778e4),
    (0x0778E8, i_0778e8),
    (0x0778EA, i_0778ea),
    (0x0778F0, i_0778f0),
    (0x0778F4, i_0778f4),
    (0x0778F8, i_0778f8),
    (0x0778FA, i_0778fa),
    (0x0778FC, i_0778fc),
    (0x077900, i_077900),
    (0x077904, i_077904),
    (0x077908, i_077908),
    (0x07790C, i_07790c),
    (0x07790E, i_07790e),
    (0x077912, i_077912),
    (0x077916, i_077916),
    (0x077918, i_077918),
    (0x07791E, i_07791e),
    (0x077922, i_077922),
    (0x077928, i_077928),
    (0x07792C, i_07792c),
    (0x077932, i_077932),
    (0x077936, i_077936),
    (0x077938, i_077938),
    (0x07793C, i_07793c),
    (0x07793E, i_07793e),
    (0x077942, i_077942),
    (0x077948, i_077948),
    (0x07794C, i_07794c),
    (0x077952, i_077952),
    (0x077956, i_077956),
    (0x077958, i_077958),
    (0x07795C, i_07795c),
    (0x077960, i_077960),
    (0x077964, i_077964),
    (0x077968, i_077968),
    (0x07796C, i_07796c),
    (0x077970, i_077970),
    (0x077974, i_077974),
    (0x077978, i_077978),
    (0x07797C, i_07797c),
    (0x077980, i_077980),
    (0x077982, i_077982),
    (0x077986, i_077986),
    (0x07798A, i_07798a),
    (0x077990, i_077990),
    (0x077994, i_077994),
    (0x07799A, i_07799a),
    (0x07799E, i_07799e),
    (0x0779A2, i_0779a2),
    (0x0779A8, i_0779a8),
    (0x0779AE, i_0779ae),
    (0x0779B2, i_0779b2),
    (0x0779B8, i_0779b8),
    (0x0779BE, i_0779be),
    (0x0779C4, i_0779c4),
    (0x0779CA, i_0779ca),
    (0x0779CE, i_0779ce),
    (0x0779D4, i_0779d4),
    (0x0779D8, i_0779d8),
    (0x0779DE, i_0779de),
    (0x0779E2, i_0779e2),
    (0x0779E8, i_0779e8),
    (0x0779EC, i_0779ec),
    (0x0779F0, i_0779f0),
    (0x0779F4, i_0779f4),
    (0x0779FA, i_0779fa),
    (0x0779FE, i_0779fe),
    (0x077A02, i_077a02),
    (0x077A04, i_077a04),
    (0x077A0A, i_077a0a),
    (0x077A0E, i_077a0e),
    (0x077A12, i_077a12),
    (0x077A14, i_077a14),
    (0x077A16, i_077a16),
    (0x077A18, i_077a18),
    (0x077A1E, i_077a1e),
    (0x077A22, i_077a22),
    (0x077A28, i_077a28),
    (0x077A2C, i_077a2c),
    (0x077A2E, i_077a2e),
    (0x077A32, i_077a32),
    (0x077A36, i_077a36),
    (0x077A38, i_077a38),
    (0x077A3C, i_077a3c),
    (0x077A42, i_077a42),
    (0x077A46, i_077a46),
    (0x077A4C, i_077a4c),
    (0x077A50, i_077a50),
    (0x077A56, i_077a56),
    (0x077A5A, i_077a5a),
    (0x077A5E, i_077a5e),
    (0x077A60, i_077a60),
    (0x077A66, i_077a66),
    (0x077A6C, i_077a6c),
    (0x077A70, i_077a70),
    (0x077A74, i_077a74),
    (0x077A76, i_077a76),
    (0x077A7C, i_077a7c),
    (0x077A82, i_077a82),
    (0x077A86, i_077a86),
    (0x077A8A, i_077a8a),
    (0x077A8C, i_077a8c),
    (0x077A8E, i_077a8e),
    (0x077A94, i_077a94),
    (0x077A96, i_077a96),
    (0x077A9A, i_077a9a),
    (0x077AA2, i_077aa2),
    (0x077AA6, i_077aa6),
    (0x077AAA, i_077aaa),
    (0x077AB2, i_077ab2),
    (0x077AB6, i_077ab6),
    (0x077ABA, i_077aba),
    (0x077AC2, i_077ac2),
    (0x077AC6, i_077ac6),
    (0x077ACA, i_077aca),
    (0x077AD2, i_077ad2),
    (0x077AD6, i_077ad6),
    (0x077ADA, i_077ada),
    (0x077ADE, i_077ade),
    (0x077AE2, i_077ae2),
    (0x077AE6, i_077ae6),
    (0x077AEA, i_077aea),
    (0x077AEE, i_077aee),
    (0x077AF2, i_077af2),
    (0x077AF6, i_077af6),
    (0x077AFC, i_077afc),
    (0x077B00, i_077b00),
    (0x077B06, i_077b06),
    (0x077B0A, i_077b0a),
    (0x077B0E, i_077b0e),
    (0x077B10, i_077b10),
    (0x077B14, i_077b14),
    (0x077B1A, i_077b1a),
    (0x077B1E, i_077b1e),
    (0x077B24, i_077b24),
    (0x077B28, i_077b28),
    (0x077B2C, i_077b2c),
    (0x077B30, i_077b30),
    (0x077B32, i_077b32),
    (0x077B36, i_077b36),
    (0x077B3C, i_077b3c),
    (0x077B40, i_077b40),
    (0x077B46, i_077b46),
    (0x077B4C, i_077b4c),
    (0x077B50, i_077b50),
    (0x077B54, i_077b54),
    (0x077B56, i_077b56),
    (0x077B5C, i_077b5c),
    (0x077B60, i_077b60),
    (0x077B64, i_077b64),
    (0x077B66, i_077b66),
    (0x077B68, i_077b68),
    (0x077B70, i_077b70),
    (0x077B74, i_077b74),
    (0x077B7A, i_077b7a),
    (0x077B7E, i_077b7e),
    (0x077B84, i_077b84),
    (0x077B88, i_077b88),
    (0x077B8A, i_077b8a),
    (0x077B8E, i_077b8e),
    (0x077B90, i_077b90),
    (0x077B96, i_077b96),
    (0x077B9A, i_077b9a),
    (0x077BA0, i_077ba0),
    (0x077BA4, i_077ba4),
    (0x077BA6, i_077ba6),
    (0x077BAA, i_077baa),
    (0x077BAE, i_077bae),
    (0x077BB4, i_077bb4),
    (0x077BBA, i_077bba),
    (0x077BBE, i_077bbe),
    (0x077BC4, i_077bc4),
    (0x077BC6, i_077bc6),
    (0x077BCA, i_077bca),
    (0x077BCE, i_077bce),
    (0x077BD4, i_077bd4),
    (0x077BD8, i_077bd8),
    (0x077BDC, i_077bdc),
    (0x077BE0, i_077be0),
    (0x077BE6, i_077be6),
    (0x077BEC, i_077bec),
    (0x077BF0, i_077bf0),
    (0x077BF2, i_077bf2),
    (0x077BF6, i_077bf6),
    (0x077BFA, i_077bfa),
    (0x077C00, i_077c00),
    (0x077C06, i_077c06),
    (0x077C0A, i_077c0a),
    (0x077C0C, i_077c0c),
    (0x077C12, i_077c12),
    (0x077C18, i_077c18),
    (0x077C1A, i_077c1a),
    (0x077C20, i_077c20),
    (0x077C24, i_077c24),
    (0x077C26, i_077c26),
    (0x077C28, i_077c28),
    (0x077C2E, i_077c2e),
    (0x077C32, i_077c32),
    (0x077C36, i_077c36),
    (0x077C3A, i_077c3a),
    (0x077C42, i_077c42),
    (0x077C46, i_077c46),
    (0x077C4E, i_077c4e),
    (0x077C52, i_077c52),
    (0x077C56, i_077c56),
    (0x077C5C, i_077c5c),
    (0x077C60, i_077c60),
    (0x077C62, i_077c62),
    (0x077C66, i_077c66),
    (0x077C6A, i_077c6a),
    (0x077C6E, i_077c6e),
    (0x077C72, i_077c72),
    (0x077C76, i_077c76),
    (0x077C78, i_077c78),
    (0x077C7C, i_077c7c),
    (0x077C7E, i_077c7e),
    (0x077C80, i_077c80),
    (0x077C84, i_077c84),
    (0x077C8A, i_077c8a),
    (0x077C90, i_077c90),
    (0x077C92, i_077c92),
    (0x077C96, i_077c96),
    (0x077C98, i_077c98),
    (0x077C9C, i_077c9c),
    (0x077CA2, i_077ca2),
    (0x077CA8, i_077ca8),
    (0x077CAC, i_077cac),
    (0x077CAE, i_077cae),
    (0x077CB4, i_077cb4),
    (0x077CB8, i_077cb8),
    (0x077CBE, i_077cbe),
    (0x077CC4, i_077cc4),
    (0x077CCA, i_077cca),
    (0x077CCE, i_077cce),
    (0x077CD2, i_077cd2),
    (0x077CD6, i_077cd6),
    (0x077CDA, i_077cda),
    (0x077CE0, i_077ce0),
    (0x077CE4, i_077ce4),
    (0x077CE8, i_077ce8),
    (0x077CEC, i_077cec),
    (0x077CF0, i_077cf0),
    (0x077CF4, i_077cf4),
    (0x077CF8, i_077cf8),
    (0x077CFE, i_077cfe),
    (0x077D02, i_077d02),
    (0x077D06, i_077d06),
    (0x077D0C, i_077d0c),
    (0x077D0E, i_077d0e),
    (0x077D14, i_077d14),
    (0x077D18, i_077d18),
    (0x077D1E, i_077d1e),
    (0x077D24, i_077d24),
    (0x077D28, i_077d28),
    (0x077D2E, i_077d2e),
    (0x077D32, i_077d32),
    (0x077D36, i_077d36),
    (0x077D3A, i_077d3a),
    (0x077D40, i_077d40),
    (0x077D44, i_077d44),
    (0x077D48, i_077d48),
    (0x077D4C, i_077d4c),
    (0x077D50, i_077d50),
    (0x077D54, i_077d54),
    (0x077D5A, i_077d5a),
    (0x077D60, i_077d60),
    (0x077D66, i_077d66),
    (0x077D6A, i_077d6a),
    (0x077D6E, i_077d6e),
    (0x077D72, i_077d72),
    (0x077D76, i_077d76),
    (0x077D7A, i_077d7a),
    (0x077D7E, i_077d7e),
    (0x077D84, i_077d84),
    (0x077D86, i_077d86),
    (0x077D88, i_077d88),
    (0x077D8C, i_077d8c),
    (0x077D92, i_077d92),
    (0x077D98, i_077d98),
    (0x077D9E, i_077d9e),
    (0x077DA4, i_077da4),
    (0x077DA6, i_077da6),
    (0x077DAA, i_077daa),
    (0x077DAE, i_077dae),
    (0x077DB2, i_077db2),
    (0x077DB6, i_077db6),
    (0x077DBC, i_077dbc),
    (0x077DC0, i_077dc0),
    (0x077DC2, i_077dc2),
    (0x077DC4, i_077dc4),
    (0x077DC8, i_077dc8),
    (0x077DCC, i_077dcc),
    (0x077DD0, i_077dd0),
    (0x077DD6, i_077dd6),
    (0x077DDA, i_077dda),
    (0x077DDC, i_077ddc),
    (0x077DDE, i_077dde),
    (0x077DE2, i_077de2),
    (0x077DE4, i_077de4),
    (0x077DEA, i_077dea),
    (0x077DEE, i_077dee),
    (0x077DF0, i_077df0),
    (0x077DF2, i_077df2),
    (0x077DF8, i_077df8),
    (0x077DFC, i_077dfc),
    (0x077DFE, i_077dfe),
    (0x077E00, i_077e00),
    (0x077E06, i_077e06),
    (0x077E0A, i_077e0a),
    (0x077E0E, i_077e0e),
    (0x077E10, i_077e10),
    (0x077E14, i_077e14),
    (0x077E1A, i_077e1a),
    (0x077E20, i_077e20),
    (0x077E26, i_077e26),
    (0x077E2A, i_077e2a),
    (0x077E30, i_077e30),
    (0x077E36, i_077e36),
    (0x077E3C, i_077e3c),
    (0x077E42, i_077e42),
    (0x077E46, i_077e46),
    (0x077E4A, i_077e4a),
    (0x077E4C, i_077e4c),
    (0x077E52, i_077e52),
    (0x077E58, i_077e58),
    (0x077E5C, i_077e5c),
    (0x077E62, i_077e62),
    (0x077E68, i_077e68),
    (0x077E6C, i_077e6c),
    (0x077E72, i_077e72),
    (0x077E74, i_077e74),
    (0x077E7A, i_077e7a),
    (0x077E80, i_077e80),
    (0x077E86, i_077e86),
    (0x077E8A, i_077e8a),
    (0x077E8E, i_077e8e),
    (0x077E94, i_077e94),
    (0x077E9A, i_077e9a),
    (0x077EA0, i_077ea0),
    (0x077EA4, i_077ea4),
    (0x077EAA, i_077eaa),
    (0x077EAE, i_077eae),
    (0x077EB2, i_077eb2),
    (0x077EB4, i_077eb4),
    (0x077EBA, i_077eba),
    (0x077EC0, i_077ec0),
    (0x077EC4, i_077ec4),
    (0x077EC8, i_077ec8),
    (0x077ECE, i_077ece),
    (0x077ED2, i_077ed2),
    (0x077ED8, i_077ed8),
    (0x077EDA, i_077eda),
    (0x077EE0, i_077ee0),
    (0x077EE6, i_077ee6),
    (0x077EEC, i_077eec),
    (0x077EF0, i_077ef0),
    (0x077EF4, i_077ef4),
    (0x077EFA, i_077efa),
    (0x077EFE, i_077efe),
    (0x077F04, i_077f04),
    (0x077F0A, i_077f0a),
    (0x077F10, i_077f10),
    (0x077F14, i_077f14),
    (0x077F18, i_077f18),
    (0x077F1E, i_077f1e),
    (0x077F22, i_077f22),
    (0x077F28, i_077f28),
    (0x077F2E, i_077f2e),
    (0x077F34, i_077f34),
    (0x077F38, i_077f38),
    (0x077F3C, i_077f3c),
    (0x077F42, i_077f42),
    (0x077F46, i_077f46),
    (0x077F4C, i_077f4c),
    (0x077F52, i_077f52),
    (0x077F58, i_077f58),
    (0x077F5C, i_077f5c),
    (0x077F60, i_077f60),
    (0x077F66, i_077f66),
    (0x077F6A, i_077f6a),
    (0x077F70, i_077f70),
    (0x077F76, i_077f76),
    (0x077F7C, i_077f7c),
    (0x077F80, i_077f80),
    (0x077F84, i_077f84),
    (0x077F8A, i_077f8a),
    (0x077F8E, i_077f8e),
    (0x077F94, i_077f94),
    (0x077F9A, i_077f9a),
    (0x077FA0, i_077fa0),
    (0x077FA4, i_077fa4),
    (0x077FA8, i_077fa8),
    (0x077FAE, i_077fae),
    (0x077FB2, i_077fb2),
    (0x077FB8, i_077fb8),
    (0x077FBE, i_077fbe),
    (0x077FC4, i_077fc4),
    (0x077FC8, i_077fc8),
    (0x077FCC, i_077fcc),
    (0x077FD2, i_077fd2),
    (0x077FD6, i_077fd6),
    (0x077FDC, i_077fdc),
    (0x077FE2, i_077fe2),
    (0x077FE8, i_077fe8),
    (0x077FEC, i_077fec),
    (0x077FF0, i_077ff0),
    (0x077FF6, i_077ff6),
    (0x077FFA, i_077ffa),
    (0x078000, i_078000),
    (0x078006, i_078006),
    (0x07800C, i_07800c),
    (0x078010, i_078010),
    (0x078014, i_078014),
    (0x07801A, i_07801a),
    (0x07801E, i_07801e),
    (0x078024, i_078024),
    (0x07802A, i_07802a),
    (0x078030, i_078030),
    (0x078034, i_078034),
    (0x078038, i_078038),
    (0x07803E, i_07803e),
    (0x078042, i_078042),
    (0x078048, i_078048),
    (0x07804E, i_07804e),
    (0x078054, i_078054),
    (0x078058, i_078058),
    (0x07805C, i_07805c),
    (0x078062, i_078062),
    (0x078066, i_078066),
    (0x07806C, i_07806c),
    (0x078072, i_078072),
    (0x078078, i_078078),
    (0x07807C, i_07807c),
    (0x078080, i_078080),
    (0x078086, i_078086),
    (0x07808A, i_07808a),
    (0x078090, i_078090),
    (0x078096, i_078096),
    (0x07809C, i_07809c),
    (0x0780A0, i_0780a0),
    (0x0780A4, i_0780a4),
    (0x0780AA, i_0780aa),
    (0x0780AE, i_0780ae),
    (0x0780B4, i_0780b4),
    (0x0780BA, i_0780ba),
    (0x0780C0, i_0780c0),
    (0x0780C4, i_0780c4),
    (0x0780C8, i_0780c8),
    (0x0780CE, i_0780ce),
    (0x0780D4, i_0780d4),
    (0x0780DA, i_0780da),
    (0x0780DE, i_0780de),
    (0x0780E2, i_0780e2),
    (0x0780E8, i_0780e8),
    (0x0780EE, i_0780ee),
    (0x0780F2, i_0780f2),
    (0x0780F8, i_0780f8),
    (0x0780FC, i_0780fc),
    (0x078100, i_078100),
    (0x078104, i_078104),
    (0x078108, i_078108),
    (0x07810A, i_07810a),
    (0x078110, i_078110),
    (0x078116, i_078116),
    (0x07811A, i_07811a),
    (0x07811E, i_07811e),
    (0x078124, i_078124),
    (0x078128, i_078128),
    (0x07812E, i_07812e),
    (0x078130, i_078130),
    (0x078136, i_078136),
    (0x07813C, i_07813c),
];
