// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $040000: jsr $2870a.l
pub fn i_040000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40006;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40006);
    cpu.pc = 0x2870A;
    16
}
/// $040006: bcc.w $40010
pub fn i_040006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4000A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40010; return 10; }
    8
}
/// $04000A: jmp $518.l
pub fn i_04000a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40010;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $040010: jmp $3efce(pc)
pub fn i_040010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40014;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_WalkStop_040054  ($040054, 56 B, pow/results_squadleader_03daa8.s)
/// $040054: clr.w $28(a6)
pub fn i_040054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40058;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $040058: lea.l $3f488(pc), a0
pub fn i_040058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4005C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F488;
    16
}
/// $04005C: jsr $28cd4.l
pub fn i_04005c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40062;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40062);
    cpu.pc = 0x28CD4;
    16
}
/// $040062: lea.l $40068(pc), a1
pub fn i_040062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40066;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40068;
    16
}
/// $040066: move.l a1, (a6)
pub fn i_040066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40068;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040068: bsr.w $3fece
pub fn i_040068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4006C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4006C);
    cpu.pc = 0x3FECE;
    18
}
/// $04006C: jsr $28d70.l
pub fn i_04006c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40072;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40072);
    cpu.pc = 0x28D70;
    16
}
/// $040072: bcc.w $40088
pub fn i_040072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40076;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40088; return 10; }
    8
}
/// $040076: bchg.b #$0, $3a(a6)
pub fn i_040076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4007C;
    cpu.ir = 0x86E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04007C: move.w #$1e, $5c(a6)
pub fn i_04007c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40082;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040082: lea.l $3ff62(pc), a1
pub fn i_040082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40086;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FF62;
    16
}
/// $040086: move.l a1, (a6)
pub fn i_040086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40088;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040088: bra.w $3ffea
pub fn i_040088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4008C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FFEA;
    10
}
// ===== Pow_Rescued_040268  ($040268, 156 B, pow/results_squadleader_03daa8.s)
/// $040268: move.w #$113a, d0
pub fn i_040268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4026C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x113A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04026C: jsr $2352.l
pub fn i_04026c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40272;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40272);
    cpu.pc = 0x2352;
    16
}
/// $040272: move.b $71(a6), d0
pub fn i_040272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40276;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $040276: bsr.w $3efa2
pub fn i_040276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4027A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4027A);
    cpu.pc = 0x3EFA2;
    18
}
/// $04027A: lea.l $ffff.w, a0
pub fn i_04027a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4027E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $04027E: move.l a0, $48(a6)
pub fn i_04027e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40282;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $040282: lea.l $3f40a(pc), a0
pub fn i_040282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40286;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F40A;
    16
}
/// $040286: jsr $28cd4.l
pub fn i_040286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4028C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4028C);
    cpu.pc = 0x28CD4;
    16
}
/// $04028C: move.w #$c0, d0
pub fn i_04028c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40290;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $040290: sub.w $22(a6), d0
pub fn i_040290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40294;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $040294: bmi.w $402a8
pub fn i_040294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40298;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x402A8; return 10; }
    8
}
/// $040298: bset.b #$0, $3a(a6)
pub fn i_040298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4029E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04029E: move.w #$700, $28(a6)
pub fn i_04029e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x700;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0402A4: bra.w $402b4
pub fn i_0402a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x402B4;
    10
}
/// $0402A8: bclr.b #$0, $3a(a6)
pub fn i_0402a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402AE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0402AE: move.w #$f900, $28(a6)
pub fn i_0402ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402B4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF900;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0402B4: move.w #$4, $30(a6)
pub fn i_0402b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402BA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0402BA: lea.l $402c0(pc), a1
pub fn i_0402ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x402C0;
    16
}
/// $0402BE: move.l a1, (a6)
pub fn i_0402be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0402C0: bsr.w $3fece
pub fn i_0402c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402C4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x402C4);
    cpu.pc = 0x3FECE;
    18
}
/// $0402C4: jsr $28d70.l
pub fn i_0402c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x402CA);
    cpu.pc = 0x28D70;
    16
}
/// $0402CA: subq.w #$1, $30(a6)
pub fn i_0402ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402CE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0402CE: bne.w $402f0
pub fn i_0402ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x402F0; return 10; }
    8
}
/// $0402D2: bchg.b #$0, $3a(a6)
pub fn i_0402d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402D8;
    cpu.ir = 0x86E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0402D8: tst.b $9b(a6)
pub fn i_0402d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402DC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0402DC: beq.w $402ea
pub fn i_0402dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x402EA; return 10; }
    8
}
/// $0402E0: lea.l $40304(pc), a1
pub fn i_0402e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40304;
    16
}
/// $0402E4: move.l a1, (a6)
pub fn i_0402e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0402E6: bra.w $402f0
pub fn i_0402e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x402F0;
    10
}
/// $0402EA: lea.l $4032e(pc), a1
pub fn i_0402ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4032E;
    16
}
/// $0402EE: move.l a1, (a6)
pub fn i_0402ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0402F0: jsr $2870a.l
pub fn i_0402f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x402F6);
    cpu.pc = 0x2870A;
    16
}
/// $0402F6: bcc.w $40300
pub fn i_0402f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x402FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40300; return 10; }
    8
}
/// $0402FA: jmp $518.l
pub fn i_0402fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40300;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $040300: bra.w $3efce
pub fn i_040300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40304;
    cpu.ir = 0x6000;
    cpu.pc = 0x3EFCE;
    10
}
// ===== Pow_RescuedBow_040304  ($040304, 42 B, pow/results_squadleader_03daa8.s)
/// $040304: lea.l $3f49e(pc), a0
pub fn i_040304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40308;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F49E;
    16
}
/// $040308: jsr $28cd4.l
pub fn i_040308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4030E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4030E);
    cpu.pc = 0x28CD4;
    16
}
/// $04030E: clr.w $28(a6)
pub fn i_04030e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40312;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $040312: lea.l $40318(pc), a1
pub fn i_040312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40316;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40318;
    16
}
/// $040316: move.l a1, (a6)
pub fn i_040316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40318;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040318: bsr.w $3fece
pub fn i_040318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4031C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4031C);
    cpu.pc = 0x3FECE;
    18
}
/// $04031C: jsr $28d70.l
pub fn i_04031c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40322);
    cpu.pc = 0x28D70;
    16
}
/// $040322: bcc.w $4032c
pub fn i_040322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40326;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4032C; return 10; }
    8
}
/// $040326: lea.l $4032e(pc), a1
pub fn i_040326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4032A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4032E;
    16
}
/// $04032A: move.l a1, (a6)
pub fn i_04032a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4032C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04032C: bra.b $402f0
pub fn i_04032c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4032E;
    cpu.ir = 0x60C2;
    cpu.pc = 0x402F0;
    10
}
// ===== Pow_RescuedRun_04032e  ($04032E, 100 B, pow/results_squadleader_03daa8.s)
/// $04032E: lea.l $3f8f4(pc), a0
pub fn i_04032e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40332;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F8F4;
    16
}
/// $040332: jsr $28cd4.l
pub fn i_040332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40338);
    cpu.pc = 0x28CD4;
    16
}
/// $040338: clr.w $28(a6)
pub fn i_040338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4033C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04033C: lea.l $40342(pc), a1
pub fn i_04033c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40340;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40342;
    16
}
/// $040340: move.l a1, (a6)
pub fn i_040340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40342;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040342: jsr $3fece(pc)
pub fn i_040342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40346;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40346);
    cpu.pc = 0x3FECE;
    16
}
/// $040346: jsr $28d70.l
pub fn i_040346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4034C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4034C);
    cpu.pc = 0x28D70;
    16
}
/// $04034C: bcc.w $40356
pub fn i_04034c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40350;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40356; return 10; }
    8
}
/// $040350: lea.l $40358(pc), a1
pub fn i_040350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40354;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40358;
    16
}
/// $040354: move.l a1, (a6)
pub fn i_040354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40356;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040356: bra.b $402f0
pub fn i_040356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40358;
    cpu.ir = 0x6098;
    cpu.pc = 0x402F0;
    10
}
/// $040358: lea.l $3f89e(pc), a0
pub fn i_040358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4035C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F89E;
    16
}
/// $04035C: jsr $28cd4.l
pub fn i_04035c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40362;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40362);
    cpu.pc = 0x28CD4;
    16
}
/// $040362: bclr.b #$0, $3a(a6)
pub fn i_040362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40368;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $040368: move.w #$fc00, $28(a6)
pub fn i_040368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4036E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04036E: lea.l $40374(pc), a1
pub fn i_04036e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40372;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40374;
    16
}
/// $040372: move.l a1, (a6)
pub fn i_040372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40374;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040374: jsr $3fece(pc)
pub fn i_040374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40378;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40378);
    cpu.pc = 0x3FECE;
    16
}
/// $040378: jsr $28d70.l
pub fn i_040378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4037E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4037E);
    cpu.pc = 0x28D70;
    16
}
/// $04037E: btst.b #$5, $5a(a6)
pub fn i_04037e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40384;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $040384: beq.w $4038e
pub fn i_040384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40388;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4038E; return 10; }
    8
}
/// $040388: lea.l $40392(pc), a1
pub fn i_040388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4038C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40392;
    16
}
/// $04038C: move.l a1, (a6)
pub fn i_04038c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4038E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04038E: bra.w $402f0
pub fn i_04038e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40392;
    cpu.ir = 0x6000;
    cpu.pc = 0x402F0;
    10
}
// ===== MeleeGuard_TailDespawn_042576  ($042576, 24 B, enemies/soldiers/melee_guard_0422ea.s)
/// $042576: lea.l $28838c.l, a0
pub fn i_042576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4257C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28838C;
    20
}
/// $04257C: jsr $5dd56.l
pub fn i_04257c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42582;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42582);
    cpu.pc = 0x5DD56;
    16
}
/// $042582: bcc.w $4258c
pub fn i_042582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42586;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4258C; return 10; }
    8
}
/// $042586: jmp $518.l
pub fn i_042586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4258C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04258C: rts
pub fn i_04258c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4258E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Charger_TrackTarget_0428C6  ($0428C6, 62 B, enemies/soldiers/melee_guard_0422ea.s)
/// $0428C6: jsr $27f08.l
pub fn i_0428c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x428CC);
    cpu.pc = 0x27F08;
    16
}
/// $0428CC: bcc.w $428d8
pub fn i_0428cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x428D8; return 10; }
    8
}
/// $0428D0: move.b d3, $75(a6)
pub fn i_0428d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428D4;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0428D4: bra.w $428ee
pub fn i_0428d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x428EE;
    10
}
/// $0428D8: cmp.b $75(a6), d0
pub fn i_0428d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428DC;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0428DC: bne.w $428e8
pub fn i_0428dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x428E8; return 10; }
    8
}
/// $0428E0: move.b d3, $75(a6)
pub fn i_0428e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428E4;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0428E4: bra.w $428ee
pub fn i_0428e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x428EE;
    10
}
/// $0428E8: move.b #$ff, $75(a6)
pub fn i_0428e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0428EE: tst.b $75(a6)
pub fn i_0428ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428F2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0428F2: bne.w $42904
pub fn i_0428f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42904; return 10; }
    8
}
/// $0428F6: jsr $28364.l
pub fn i_0428f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x428FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x428FC);
    cpu.pc = 0x28364;
    16
}
/// $0428FC: scs.b $75(a6)
pub fn i_0428fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42900;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $042900: bra.w $4290a
pub fn i_042900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42904;
    cpu.ir = 0x6000;
    cpu.pc = 0x4290A;
    10
}
// ===== JsrAbsThunk_042904  ($042904, 8 B, thunks/jsr_abs_thunks.c)
/// $042904: jsr $28292.l
pub fn i_042904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4290A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4290A);
    cpu.pc = 0x28292;
    16
}
/// $04290A: rts
pub fn i_04290a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4290C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Charger_Init_04290C  ($04290C, 240 B, enemies/soldiers/charger_states_04290c.s)
/// $04290C: jsr $27f60.l
pub fn i_04290c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42912);
    cpu.pc = 0x27F60;
    16
}
/// $042912: scc.b $75(a6)
pub fn i_042912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42916;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $042916: clr.l $7c(a6)
pub fn i_042916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4291A;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $04291A: clr.b $76(a6)
pub fn i_04291a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4291E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04291E: move.w #$e, d1
pub fn i_04291e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42922;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042922: jsr $236e.l
pub fn i_042922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42928;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42928);
    cpu.pc = 0x236E;
    16
}
/// $042928: move.w #$8000, d0
pub fn i_042928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4292C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04292C: jsr $28134.l
pub fn i_04292c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42932);
    cpu.pc = 0x28134;
    16
}
/// $042932: andi.w #$ffe3, $38(a6)
pub fn i_042932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42938;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $042938: ori.w #$18, $38(a6)
pub fn i_042938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4293E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04293E: lea.l $776e2.l, a1
pub fn i_04293e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42944;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $042944: jsr $4ae.l
pub fn i_042944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4294A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4294A);
    cpu.pc = 0x4AE;
    16
}
/// $04294A: move.w $22(a6), d0
pub fn i_04294a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4294E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04294E: jsr $440bc.l
pub fn i_04294e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42954;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42954);
    cpu.pc = 0x440BC;
    16
}
/// $042954: move.w d0, $70(a6)
pub fn i_042954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42958;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $042958: moveq #$0, d0
pub fn i_042958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4295A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04295A: move.b $98(a6), d0
pub fn i_04295a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4295E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04295E: lsl.w #$3, d0
pub fn i_04295e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42960;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $042960: move.w d0, $72(a6)
pub fn i_042960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42964;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $042964: lea.l $2b74ec.l, a0
pub fn i_042964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4296A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B74EC;
    20
}
/// $04296A: jsr $799de.l
pub fn i_04296a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42970);
    cpu.pc = 0x799DE;
    16
}
/// $042970: clr.w d1
pub fn i_042970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42972;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $042972: move.b $9a(a6), d1
pub fn i_042972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42976;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $042976: mulu.w d0, d1
pub fn i_042976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42978;
    cpu.ir = 0xC2C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $042978: lsr.l #$8, d1
pub fn i_042978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4297A;
    cpu.ir = 0xE089;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[1] = r;
    22
}
/// $04297A: cmpi.w #$100, d1
pub fn i_04297a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4297E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $04297E: bcs.w $42986
pub fn i_04297e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42982;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x42986; return 10; }
    8
}
/// $042982: move.b #$ff, d1
pub fn i_042982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42986;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $042986: move.b d1, $74(a6)
pub fn i_042986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4298A;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04298A: lea.l $2b746a.l, a0
pub fn i_04298a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42990;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B746A;
    20
}
/// $042990: jsr $799de.l
pub fn i_042990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42996);
    cpu.pc = 0x799DE;
    16
}
/// $042996: clr.w d1
pub fn i_042996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42998;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $042998: move.b $9b(a6), d1
pub fn i_042998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4299C;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04299C: mulu.w d1, d0
pub fn i_04299c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4299E;
    cpu.ir = 0xC0C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $04299E: lsr.l #$8, d0
pub fn i_04299e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429A0;
    cpu.ir = 0xE088;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, false);
    cpu.d[0] = r;
    22
}
/// $0429A0: cmpi.w #$100, d0
pub fn i_0429a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429A4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $0429A4: bcs.w $429ac
pub fn i_0429a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429A8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x429AC; return 10; }
    8
}
/// $0429A8: move.b #$ff, d0
pub fn i_0429a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429AC;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0429AC: move.b d0, $9b(a6)
pub fn i_0429ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429B0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0429B0: jsr $27f08.l
pub fn i_0429b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x429B6);
    cpu.pc = 0x27F08;
    16
}
/// $0429B6: bcc.w $429c2
pub fn i_0429b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x429C2; return 10; }
    8
}
/// $0429BA: move.b d3, $75(a6)
pub fn i_0429ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429BE;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0429BE: bra.w $429d8
pub fn i_0429be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x429D8;
    10
}
/// $0429C2: cmp.b $75(a6), d0
pub fn i_0429c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429C6;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0429C6: bne.w $429d2
pub fn i_0429c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429CA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x429D2; return 10; }
    8
}
/// $0429CA: move.b d3, $75(a6)
pub fn i_0429ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429CE;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0429CE: bra.w $429d8
pub fn i_0429ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x429D8;
    10
}
/// $0429D2: move.b #$ff, $75(a6)
pub fn i_0429d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429D8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0429D8: move.w $22(a6), d1
pub fn i_0429d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429DC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0429DC: cmpi.w #$a0, d1
pub fn i_0429dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429E0;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0429E0: bge.w $429ee
pub fn i_0429e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429E4;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x429EE; return 10; }
    8
}
/// $0429E4: bset.b #$0, $3a(a6)
pub fn i_0429e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429EA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0429EA: bra.w $429f4
pub fn i_0429ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x429F4;
    10
}
/// $0429EE: bclr.b #$0, $3a(a6)
pub fn i_0429ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0429F4: tst.w $72(a6)
pub fn i_0429f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429F8;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0429F8: bne.w $42a04
pub fn i_0429f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x429FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42A04; return 10; }
    8
}
// ===== SetTaskHandler_0429fc  ($0429FC, 8 B, kernel/task_handlers.c)
/// $0429FC: lea.l $42acc(pc), a1
pub fn i_0429fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42ACC;
    16
}
/// $042A00: move.l a1, (a6)
pub fn i_042a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042A02: rts
pub fn i_042a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Charger_InitDispatch_042A04  ($042A04, 8 B, enemies/soldiers/charger_states_04290c.s)
/// $042A04: move.b $9d(a6), d0
pub fn i_042a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A08;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $042A08: bne.w $42a14
pub fn i_042a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A0C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42A14; return 10; }
    8
}
// ===== SetTaskHandler_042a0c  ($042A0C, 8 B, kernel/task_handlers.c)
/// $042A0C: lea.l $42a6e(pc), a1
pub fn i_042a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42A6E;
    16
}
/// $042A10: move.l a1, (a6)
pub fn i_042a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042A12: rts
pub fn i_042a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Charger_InitDispatch2_042A14  ($042A14, 6 B, enemies/soldiers/charger_states_04290c.s)
/// $042A14: subq.b #$1, d0
pub fn i_042a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A16;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $042A16: bne.w $42a22
pub fn i_042a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A1A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42A22; return 10; }
    8
}
// ===== SetTaskHandler_042a1a  ($042A1A, 8 B, kernel/task_handlers.c)
/// $042A1A: lea.l $42a56(pc), a1
pub fn i_042a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42A56;
    16
}
/// $042A1E: move.l a1, (a6)
pub fn i_042a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042A20: rts
pub fn i_042a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_042a22  ($042A22, 8 B, kernel/task_handlers.c)
/// $042A22: lea.l $42a44(pc), a1
pub fn i_042a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42A44;
    16
}
/// $042A26: move.l a1, (a6)
pub fn i_042a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042A28: rts
pub fn i_042a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Charger_ColumnDeltaTest_042A2A  ($042A2A, 162 B, enemies/soldiers/charger_states_04290c.s)
/// $042A2A: move.w $22(a6), d0
pub fn i_042a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A2E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $042A2E: jsr $440bc.l
pub fn i_042a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42A34);
    cpu.pc = 0x440BC;
    16
}
/// $042A34: sub.w $70(a6), d0
pub fn i_042a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A38;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $042A38: bpl.w $42a3e
pub fn i_042a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A3C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x42A3E; return 10; }
    8
}
/// $042A3C: neg.w d0
pub fn i_042a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A3E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $042A3E: cmp.w $72(a6), d0
pub fn i_042a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A42;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $042A42: rts
pub fn i_042a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $042A44: lea.l $427fa(pc), a0
pub fn i_042a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A48;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x427FA;
    16
}
/// $042A48: jsr $28cd4.l
pub fn i_042a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42A4E);
    cpu.pc = 0x28CD4;
    16
}
/// $042A4E: move.w #$100, d0
pub fn i_042a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A52;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042A52: bra.w $42a86
pub fn i_042a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A56;
    cpu.ir = 0x6000;
    cpu.pc = 0x42A86;
    10
}
/// $042A56: move.b #$ff, $76(a6)
pub fn i_042a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $042A5C: lea.l $4287e(pc), a0
pub fn i_042a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A60;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4287E;
    16
}
/// $042A60: jsr $28cd4.l
pub fn i_042a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42A66);
    cpu.pc = 0x28CD4;
    16
}
/// $042A66: move.w #$100, d0
pub fn i_042a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A6A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $042A6A: bra.w $42a86
pub fn i_042a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x42A86;
    10
}
/// $042A86: btst.b #$0, $3a(a6)
pub fn i_042a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A8C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $042A8C: bne.w $42a92
pub fn i_042a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A90;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x42A92; return 10; }
    8
}
/// $042A90: neg.w d0
pub fn i_042a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A92;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $042A92: move.w d0, $28(a6)
pub fn i_042a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A96;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $042A96: lea.l $42a9c(pc), a1
pub fn i_042a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42A9C;
    16
}
/// $042A9A: move.l a1, (a6)
pub fn i_042a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42A9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042A9C: jsr $428c6(pc)
pub fn i_042a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x42AA0);
    cpu.pc = 0x428C6;
    16
}
/// $042AA0: jsr $28d70.l
pub fn i_042aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42AA6);
    cpu.pc = 0x28D70;
    16
}
/// $042AA6: bsr.b $42a2a
pub fn i_042aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AA8;
    cpu.ir = 0x6182;
    cpu.push32(bus, 0x42AA8);
    cpu.pc = 0x42A2A;
    18
}
/// $042AA8: bcs.w $42ab2
pub fn i_042aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AAC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x42AB2; return 10; }
    8
}
/// $042AAC: lea.l $42acc(pc), a1
pub fn i_042aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42ACC;
    16
}
/// $042AB0: move.l a1, (a6)
pub fn i_042ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042AB2: jsr $574e8.l
pub fn i_042ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42AB8);
    cpu.pc = 0x574E8;
    16
}
/// $042AB8: bcc.w $42ac2
pub fn i_042ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42ABC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x42AC2; return 10; }
    8
}
/// $042ABC: lea.l $42b5c(pc), a1
pub fn i_042abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x42B5C;
    16
}
/// $042AC0: move.l a1, (a6)
pub fn i_042ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $042AC2: jsr $49fd0.l
pub fn i_042ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42AC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x42AC8);
    cpu.pc = 0x49FD0;
    16
}
/// $042AC8: bra.w $42576
pub fn i_042ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x42ACC;
    cpu.ir = 0x6000;
    cpu.pc = 0x42576;
    10
}
// ===== Camera_ResetSmoothing_0434EA  ($0434EA, 14 B, mission/scene_loader_0434ea.s)
/// $0434EA: clr.l $10817a.l
pub fn i_0434ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434F0;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x10817A);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x10817A, r2);
    20
}
/// $0434F0: clr.l $10817e.l
pub fn i_0434f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434F6;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x10817E);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x10817E, r2);
    20
}
/// $0434F6: rts
pub fn i_0434f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Camera_SmoothingIntegrate_0434F8  ($0434F8, 106 B, mission/scene_loader_0434ea.s)
/// $0434F8: move.l $10817a.l, d0
pub fn i_0434f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x434FE;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x10817A);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0434FE: move.l d0, d1
pub fn i_0434fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43500;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043500: move.l $10817e.l, d2
pub fn i_043500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43506;
    cpu.ir = 0x2439;
    let v2 = bus.read32(0x10817E);
    cpu.d[2] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043506: move.l d2, d3
pub fn i_043506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43508;
    cpu.ir = 0x2602;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043508: cmpi.l #$4000, d3
pub fn i_043508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4350E;
    cpu.ir = 0xC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x4000, Size::Long, false);
    8
}
/// $04350E: bge.b $4351a
pub fn i_04350e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43510;
    cpu.ir = 0x6C0A;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x4351A; return 10; }
    8
}
/// $043510: cmpi.l #$ffffc000, d3
pub fn i_043510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43516;
    cpu.ir = 0xC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFC000, Size::Long, false);
    8
}
/// $043516: bgt.w $4351c
pub fn i_043516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4351A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4351C; return 10; }
    8
}
/// $04351A: asr.l #$3, d3
pub fn i_04351a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4351C;
    cpu.ir = 0xE683;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Long, 0, false);
    cpu.d[3] = r;
    12
}
/// $04351C: add.l d1, d3
pub fn i_04351c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4351E;
    cpu.ir = 0xD681;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[3] = r;
    4
}
/// $04351E: sub.l d3, d2
pub fn i_04351e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43520;
    cpu.ir = 0x9483;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $043520: move.l d2, $10817e.l
pub fn i_043520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43526;
    cpu.ir = 0x23C2;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    bus.write32(0x10817E, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043526: add.l d0, d2
pub fn i_043526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43528;
    cpu.ir = 0xD480;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $043528: cmpi.l #$80000, d2
pub fn i_043528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4352E;
    cpu.ir = 0xC82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x80000, Size::Long, false);
    8
}
/// $04352E: ble.w $4353c
pub fn i_04352e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43532;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x4353C; return 10; }
    8
}
/// $043532: move.l #$80000, d2
pub fn i_043532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43538;
    cpu.ir = 0x243C;
    let v1: u32 = 0x80000;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043538: bra.w $4354c
pub fn i_043538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4353C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4354C;
    10
}
/// $04353C: cmpi.l #$fff80000, d2
pub fn i_04353c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43542;
    cpu.ir = 0xC82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFF80000, Size::Long, false);
    8
}
/// $043542: bgt.w $4354c
pub fn i_043542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43546;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4354C; return 10; }
    8
}
/// $043546: move.l #$fff80000, d2
pub fn i_043546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4354C;
    cpu.ir = 0x243C;
    let v1: u32 = 0xFFF80000;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04354C: move.l d2, $10817a.l
pub fn i_04354c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43552;
    cpu.ir = 0x23C2;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    bus.write32(0x10817A, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043552: swap d2
pub fn i_043552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43554;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043554: swap d0
pub fn i_043554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43556;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043556: sub.w d0, d2
pub fn i_043556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43558;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043558: move.w d2, d0
pub fn i_043558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4355A;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04355A: move.w d0, $108182.l
pub fn i_04355a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43560;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108182, v1 as u16);
    16
}
/// $043560: rts
pub fn i_043560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43562;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneLoader_ByIndex_043562  ($043562, 6 B, mission/scene_loader_0434ea.s)
/// $043562: clr.w d0
pub fn i_043562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43564;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043564: bra.w $43574
pub fn i_043564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43568;
    cpu.ir = 0x6000;
    cpu.pc = 0x43574;
    10
}
// ===== SceneLoader_Main_043568  ($043568, 374 B, mission/scene_loader_0434ea.s)
/// $043568: andi.w #$ff, d0
pub fn i_043568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4356C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04356C: lsl.w #$3, d0
pub fn i_04356c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4356E;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04356E: lea.l $916c8.l, a0
pub fn i_04356e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43574;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x916C8;
    20
}
/// $043574: move.l (a0, d0.w), $10815c.l
pub fn i_043574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4357C;
    cpu.ir = 0x23F0;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    bus.write32(0x10815C, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    34
}
/// $04357C: movea.l $4(a0, d0.w), a0
pub fn i_04357c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43580;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $043580: move.l a0, -(a7)
pub fn i_043580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43582;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $043582: bsr.w $43d6c
pub fn i_043582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43586;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43586);
    cpu.pc = 0x43D6C;
    18
}
/// $043586: movea.l (a7), a0
pub fn i_043586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43588;
    cpu.ir = 0x2057;
    let v2 = bus.read32(cpu.a[7]);
    cpu.a[0] = v2;
    12
}
/// $043588: moveq #$0, d1
pub fn i_043588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4358A;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04358A: move.l d1, d2
pub fn i_04358a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4358C;
    cpu.ir = 0x2401;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04358C: cmpi.b #$2, (a0)
pub fn i_04358c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43590;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    12
}
/// $043590: beq.w $435b0
pub fn i_043590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43594;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x435B0; return 10; }
    8
}
/// $043594: move.b (a0), d0
pub fn i_043594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43596;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $043596: cmpi.b #$0, d0
pub fn i_043596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4359A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $04359A: bne.w $435a6
pub fn i_04359a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4359E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x435A6; return 10; }
    8
}
/// $04359E: addi.w #$20, d1
pub fn i_04359e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435A2;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0435A2: bra.w $435aa
pub fn i_0435a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x435AA;
    10
}
/// $0435A6: addi.w #$20, d2
pub fn i_0435a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435AA;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0435AA: adda.w #$e, a0
pub fn i_0435aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435AE;
    cpu.ir = 0xD0FC;
    let v1: u32 = 0xE;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $0435AE: bra.b $4358c
pub fn i_0435ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435B0;
    cpu.ir = 0x60DC;
    cpu.pc = 0x4358C;
    10
}
/// $0435B0: move.w d2, d0
pub fn i_0435b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435B2;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0435B2: moveq #$0, d2
pub fn i_0435b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435B4;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0435B4: move.b $1(a0), d2
pub fn i_0435b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435B8;
    cpu.ir = 0x1428;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0435B8: jsr $1390e.l
pub fn i_0435b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x435BE);
    cpu.pc = 0x1390E;
    16
}
/// $0435BE: movea.l (a7)+, a0
pub fn i_0435be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435C0;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $0435C0: cmpi.b #$2, (a0)
pub fn i_0435c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435C4;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    12
}
/// $0435C4: beq.w $43610
pub fn i_0435c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43610; return 10; }
    8
}
/// $0435C8: move.l a0, -(a7)
pub fn i_0435c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435CA;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0435CA: moveq #$20, d0
pub fn i_0435ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435CC;
    cpu.ir = 0x7020;
    cpu.d[0] = 0x20;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0435CC: cmpi.b #$0, (a0)
pub fn i_0435cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435D0;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    12
}
/// $0435D0: bne.w $435de
pub fn i_0435d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x435DE; return 10; }
    8
}
/// $0435D4: jsr $13982.l
pub fn i_0435d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x435DA);
    cpu.pc = 0x13982;
    16
}
/// $0435DA: bra.w $435e4
pub fn i_0435da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x435E4;
    10
}
/// $0435DE: jsr $13952.l
pub fn i_0435de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x435E4);
    cpu.pc = 0x13952;
    16
}
/// $0435E4: movea.l (a7), a2
pub fn i_0435e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435E6;
    cpu.ir = 0x2457;
    let v2 = bus.read32(cpu.a[7]);
    cpu.a[2] = v2;
    12
}
/// $0435E6: movea.l $2(a2), a0
pub fn i_0435e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435EA;
    cpu.ir = 0x206A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0435EA: lea.l $6(a2), a1
pub fn i_0435ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435EE;
    cpu.ir = 0x43EA;
    let ad1 = cpu.a[2].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $0435EE: jsr $51abe.l
pub fn i_0435ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x435F4);
    cpu.pc = 0x51ABE;
    16
}
/// $0435F4: move.w #$100, $74(a0)
pub fn i_0435f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x435FA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0435FA: move.w #$100, $76(a0)
pub fn i_0435fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43600;
    cpu.ir = 0x317C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $043600: clr.b $72(a0)
pub fn i_043600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43604;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $043604: clr.l $78(a0)
pub fn i_043604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43608;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x78);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $043608: movea.l (a7)+, a0
pub fn i_043608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4360A;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $04360A: adda.w #$e, a0
pub fn i_04360a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4360E;
    cpu.ir = 0xD0FC;
    let v1: u32 = 0xE;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $04360E: bra.b $435c0
pub fn i_04360e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43610;
    cpu.ir = 0x60B0;
    cpu.pc = 0x435C0;
    10
}
/// $043610: clr.w $106f5e.l
pub fn i_043610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43616;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F5E) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F5E, r2 as u16);
    16
}
/// $043616: clr.l $106f50.l
pub fn i_043616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4361C;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F50);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F50, r2);
    20
}
/// $04361C: clr.l $106f54.l
pub fn i_04361c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43622;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F54);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F54, r2);
    20
}
/// $043622: clr.l $106f58.l
pub fn i_043622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43628;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F58);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F58, r2);
    20
}
/// $043628: clr.w $108160.l
pub fn i_043628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4362E;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x108160) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x108160, r2 as u16);
    16
}
/// $04362E: clr.w $108162.l
pub fn i_04362e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43634;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x108162) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x108162, r2 as u16);
    16
}
/// $043634: move.w #$a0, $108164.l
pub fn i_043634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4363C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108164, v1 as u16);
    16
}
/// $04363C: move.w #$180, $108166.l
pub fn i_04363c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43644;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108166, v1 as u16);
    16
}
/// $043644: move.w #$0, $108168.l
pub fn i_043644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4364C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108168, v1 as u16);
    16
}
/// $04364C: move.w #$0, $10816a.l
pub fn i_04364c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43654;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816A, v1 as u16);
    16
}
/// $043654: move.w #$ffff, $10816c.l
pub fn i_043654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4365C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816C, v1 as u16);
    16
}
/// $04365C: move.w #$ffff, $10816e.l
pub fn i_04365c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43664;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816E, v1 as u16);
    16
}
/// $043664: move.w #$0, $108170.l
pub fn i_043664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4366C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108170, v1 as u16);
    16
}
/// $04366C: clr.w $106f5c.l
pub fn i_04366c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43672;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F5C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F5C, r2 as u16);
    16
}
/// $043672: clr.b $108179.l
pub fn i_043672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43678;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x108179) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x108179, r2 as u8);
    16
}
/// $043678: move.w $106f5c.l, $108172.l
pub fn i_043678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43682;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108172, v2 as u16);
    28
}
/// $043682: move.w $106f50.l, $108174.l
pub fn i_043682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4368C;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108174, v2 as u16);
    28
}
/// $04368C: move.w $106f58.l, $108176.l
pub fn i_04368c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43696;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108176, v2 as u16);
    28
}
/// $043696: lea.l $106f6c.l, a0
pub fn i_043696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4369C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $04369C: bsr.w $43eda
pub fn i_04369c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436A0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x436A0);
    cpu.pc = 0x43EDA;
    18
}
/// $0436A0: bsr.w $434ea
pub fn i_0436a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436A4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x436A4);
    cpu.pc = 0x434EA;
    18
}
/// $0436A4: clr.b $10e39c.l
pub fn i_0436a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436AA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39C, r2 as u8);
    16
}
/// $0436AA: clr.b $10e39d.l
pub fn i_0436aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436B0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39D) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39D, r2 as u8);
    16
}
/// $0436B0: clr.w $106f42.l
pub fn i_0436b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436B6;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F42) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F42, r2 as u16);
    16
}
/// $0436B6: move.b #$1, $106ed3.l
pub fn i_0436b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436BE;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106ED3, v1 as u8);
    16
}
/// $0436BE: jsr $7707c.l
pub fn i_0436be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x436C4);
    cpu.pc = 0x7707C;
    16
}
/// $0436C4: jsr $8f158.l
pub fn i_0436c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x436CA);
    cpu.pc = 0x8F158;
    16
}
/// $0436CA: jsr $3ee3a.l
pub fn i_0436ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x436D0);
    cpu.pc = 0x3EE3A;
    16
}
/// $0436D0: jsr $997b8.l
pub fn i_0436d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x436D6);
    cpu.pc = 0x997B8;
    16
}
/// $0436D6: jsr $4cb5c.l
pub fn i_0436d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x436DC);
    cpu.pc = 0x4CB5C;
    16
}
/// $0436DC: rts
pub fn i_0436dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Geom_Proj_Clamp_0436DE  ($0436DE, 252 B, video/geom_proj_clamp_0436de.s)
/// $0436DE: tst.b $108179.l
pub fn i_0436de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436E4;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x108179) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0436E4: bne.w $436f4
pub fn i_0436e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x436F4; return 10; }
    8
}
/// $0436E8: tst.l d0
pub fn i_0436e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436EA;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0436EA: bmi.w $436f4
pub fn i_0436ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436EE;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x436F4; return 10; }
    8
}
/// $0436EE: tst.l d1
pub fn i_0436ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436F0;
    cpu.ir = 0x4A81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0436F0: bpl.w $436fc
pub fn i_0436f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436F4;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x436FC; return 10; }
    8
}
/// $0436F4: clr.l d0
pub fn i_0436f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436F6;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0436F6: clr.l d1
pub fn i_0436f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436F8;
    cpu.ir = 0x4281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = r2;
    4
}
/// $0436F8: bra.w $437da
pub fn i_0436f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x436FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x437DA;
    10
}
/// $0436FC: move.w #$200, d4
pub fn i_0436fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43700;
    cpu.ir = 0x383C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043700: sub.w d4, d1
pub fn i_043700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43702;
    cpu.ir = 0x9244;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043702: neg.w d1
pub fn i_043702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43704;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043704: sub.w d4, d3
pub fn i_043704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43706;
    cpu.ir = 0x9644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043706: neg.w d3
pub fn i_043706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43708;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043708: cmp.w d0, d2
pub fn i_043708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4370A;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $04370A: bcc.w $43710
pub fn i_04370a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4370E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x43710; return 10; }
    8
}
/// $04370E: exg.l d0, d2
pub fn i_04370e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43710;
    cpu.ir = 0xC142;
    cpu.d.swap(0, 2);
    6
}
/// $043710: cmp.w d1, d3
pub fn i_043710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43712;
    cpu.ir = 0xB641;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $043712: bcc.w $43718
pub fn i_043712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43716;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x43718; return 10; }
    8
}
/// $043716: exg.l d1, d3
pub fn i_043716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43718;
    cpu.ir = 0xC343;
    cpu.d.swap(1, 3);
    6
}
/// $043718: move.w $108164.l, d4
pub fn i_043718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4371E;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x108164) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04371E: move.w #$1e, d5
pub fn i_04371e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43722;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043722: move.w #$117, d6
pub fn i_043722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43726;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x117;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043726: bsr.w $4374c
pub fn i_043726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4372A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4372A);
    cpu.pc = 0x4374C;
    18
}
/// $04372A: exg.l d1, d0
pub fn i_04372a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4372C;
    cpu.ir = 0xC340;
    cpu.d.swap(1, 0);
    6
}
/// $04372C: move.w d3, d2
pub fn i_04372c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4372E;
    cpu.ir = 0x3403;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04372E: move.w $108166.l, d4
pub fn i_04372e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43734;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x108166) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043734: sub.w $10817a.l, d4
pub fn i_043734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4373A;
    cpu.ir = 0x9879;
    let v2 = bus.read16(0x10817A) as u32;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04373A: move.w #$0, d5
pub fn i_04373a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4373E;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04373E: move.w #$d0, d6
pub fn i_04373e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43742;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0xD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043742: bsr.w $4374c
pub fn i_043742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43746;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43746);
    cpu.pc = 0x4374C;
    18
}
/// $043746: exg.l d1, d0
pub fn i_043746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43748;
    cpu.ir = 0xC340;
    cpu.d.swap(1, 0);
    6
}
/// $043748: bra.w $437da
pub fn i_043748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4374C;
    cpu.ir = 0x6000;
    cpu.pc = 0x437DA;
    10
}
/// $04374C: swap d0
pub fn i_04374c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4374E;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04374E: clr.w d0
pub fn i_04374e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43750;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043750: swap d2
pub fn i_043750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43752;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043752: clr.w d2
pub fn i_043752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43754;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043754: swap d4
pub fn i_043754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43756;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043756: clr.w d4
pub fn i_043756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43758;
    cpu.ir = 0x4244;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043758: swap d5
pub fn i_043758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4375A;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04375A: clr.w d5
pub fn i_04375a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4375C;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04375C: swap d6
pub fn i_04375c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4375E;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04375E: clr.w d6
pub fn i_04375e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43760;
    cpu.ir = 0x4246;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043760: move.l d0, d7
pub fn i_043760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43762;
    cpu.ir = 0x2E00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043762: add.l d2, d7
pub fn i_043762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43764;
    cpu.ir = 0xDE82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $043764: asr.l #$1, d7
pub fn i_043764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43766;
    cpu.ir = 0xE287;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, false);
    cpu.d[7] = r;
    8
}
/// $043766: sub.l d4, d7
pub fn i_043766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43768;
    cpu.ir = 0x9E84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $043768: addi.l #$10000, d7
pub fn i_043768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4376E;
    cpu.ir = 0x687;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10000, Size::Long, true);
    cpu.d[7] = r2;
    8
}
/// $04376E: cmpi.l #$20000, d7
pub fn i_04376e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43774;
    cpu.ir = 0xC87;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x20000, Size::Long, false);
    8
}
/// $043774: bcs.w $437d6
pub fn i_043774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43778;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x437D6; return 10; }
    8
}
/// $043778: subi.l #$10000, d7
pub fn i_043778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4377E;
    cpu.ir = 0x487;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10000, Size::Long, true);
    cpu.d[7] = r2;
    8
}
/// $04377E: sub.l d5, d0
pub fn i_04377e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43780;
    cpu.ir = 0x9085;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $043780: sub.l d6, d2
pub fn i_043780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43782;
    cpu.ir = 0x9486;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $043782: move.l d7, -(a7)
pub fn i_043782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43784;
    cpu.ir = 0x2F07;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $043784: asr.l #$1, d7
pub fn i_043784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43786;
    cpu.ir = 0xE287;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, false);
    cpu.d[7] = r;
    8
}
/// $043786: add.l (a7)+, d7
pub fn i_043786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43788;
    cpu.ir = 0xDE9F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Long, true);
    cpu.d[7] = r;
    12
}
/// $043788: bmi.w $437a4
pub fn i_043788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4378C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x437A4; return 10; }
    8
}
/// $04378C: tst.l d0
pub fn i_04378c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4378E;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04378E: bge.w $43798
pub fn i_04378e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43792;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x43798; return 10; }
    8
}
/// $043792: clr.l d7
pub fn i_043792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43794;
    cpu.ir = 0x4287;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = r2;
    4
}
/// $043794: bra.w $437a0
pub fn i_043794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43798;
    cpu.ir = 0x6000;
    cpu.pc = 0x437A0;
    10
}
/// $043798: cmp.l d0, d7
pub fn i_043798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4379A;
    cpu.ir = 0xBE80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $04379A: ble.w $437a0
pub fn i_04379a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4379E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x437A0; return 10; }
    8
}
/// $04379E: move.l d0, d7
pub fn i_04379e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437A0;
    cpu.ir = 0x2E00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437A0: bra.w $437b8
pub fn i_0437a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x437B8;
    10
}
/// $0437A4: tst.l d2
pub fn i_0437a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437A6;
    cpu.ir = 0x4A82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437A6: ble.w $437b0
pub fn i_0437a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437AA;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x437B0; return 10; }
    8
}
/// $0437AA: clr.l d7
pub fn i_0437aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437AC;
    cpu.ir = 0x4287;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = r2;
    4
}
/// $0437AC: bra.w $437b8
pub fn i_0437ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x437B8;
    10
}
/// $0437B0: cmp.l d2, d7
pub fn i_0437b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437B2;
    cpu.ir = 0xBE82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $0437B2: bge.w $437b8
pub fn i_0437b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437B6;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x437B8; return 10; }
    8
}
/// $0437B6: move.l d2, d7
pub fn i_0437b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437B8;
    cpu.ir = 0x2E02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437B8: asr.l #$4, d7
pub fn i_0437b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437BA;
    cpu.ir = 0xE887;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, false);
    cpu.d[7] = r;
    14
}
/// $0437BA: move.l #$80000, d2
pub fn i_0437ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437C0;
    cpu.ir = 0x243C;
    let v1: u32 = 0x80000;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437C0: cmp.l d2, d7
pub fn i_0437c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437C2;
    cpu.ir = 0xBE82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $0437C2: ble.w $437c8
pub fn i_0437c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437C6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x437C8; return 10; }
    8
}
/// $0437C6: move.l d2, d7
pub fn i_0437c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437C8;
    cpu.ir = 0x2E02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437C8: neg.l d2
pub fn i_0437c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437CA;
    cpu.ir = 0x4482;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.set(StatusRegister::V, v1 == 0x80000000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = r2;
    4
}
/// $0437CA: cmp.l d2, d7
pub fn i_0437ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437CC;
    cpu.ir = 0xBE82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $0437CC: bge.w $437d2
pub fn i_0437cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437D0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x437D2; return 10; }
    8
}
/// $0437D0: move.l d2, d7
pub fn i_0437d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437D2;
    cpu.ir = 0x2E02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437D2: move.l d7, d0
pub fn i_0437d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437D4;
    cpu.ir = 0x2007;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437D4: rts
pub fn i_0437d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0437D6: moveq #$0, d0
pub fn i_0437d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437D8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0437D8: rts
pub fn i_0437d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneScriptVM_Frame_0437DA  ($0437DA, 1288 B, mission/scene_script_vm_0437da.s)
/// $0437DA: tst.w $106f5e.l
pub fn i_0437da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437E0;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F5E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $0437E0: beq.w $437f0
pub fn i_0437e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437E4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x437F0; return 10; }
    8
}
/// $0437E4: move.l $106f60.l, d0
pub fn i_0437e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437EA;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F60);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0437EA: move.l $106f64.l, d1
pub fn i_0437ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437F0;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F64);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0437F0: movem.l d0-d1, -(a7)
pub fn i_0437f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437F4;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0437F4: jsr $77092.l
pub fn i_0437f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x437FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x437FA);
    cpu.pc = 0x77092;
    16
}
/// $0437FA: jsr $8f172.l
pub fn i_0437fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43800;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43800);
    cpu.pc = 0x8F172;
    16
}
/// $043800: jsr $997cc.l
pub fn i_043800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43806);
    cpu.pc = 0x997CC;
    16
}
/// $043806: jsr $8f8e2.l
pub fn i_043806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4380C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4380C);
    cpu.pc = 0x8F8E2;
    16
}
/// $04380C: movea.l $10815c.l, a1
pub fn i_04380c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43812;
    cpu.ir = 0x2279;
    let v2 = bus.read32(0x10815C);
    cpu.a[1] = v2;
    20
}
/// $043812: move.w (a1), d0
pub fn i_043812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43814;
    cpu.ir = 0x3011;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $043814: add.w d0, d0
pub fn i_043814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43816;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043816: add.w d0, d0
pub fn i_043816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43818;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043818: jmp $4381c(pc, d0.w)
pub fn i_043818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4381C;
    cpu.ir = 0x4EFB;
    let ad1 = 0x4381C_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $04381C: bra.w $43b66
pub fn i_04381c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43820;
    cpu.ir = 0x6000;
    cpu.pc = 0x43B66;
    10
}
/// $043820: bra.w $43ba2
pub fn i_043820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43824;
    cpu.ir = 0x6000;
    cpu.pc = 0x43BA2;
    10
}
/// $043824: bra.w $43bc6
pub fn i_043824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43828;
    cpu.ir = 0x6000;
    cpu.pc = 0x43BC6;
    10
}
/// $043828: bra.w $43afa
pub fn i_043828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4382C;
    cpu.ir = 0x6000;
    cpu.pc = 0x43AFA;
    10
}
/// $04382C: bra.w $43ada
pub fn i_04382c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43830;
    cpu.ir = 0x6000;
    cpu.pc = 0x43ADA;
    10
}
/// $043830: bra.w $43ab8
pub fn i_043830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43834;
    cpu.ir = 0x6000;
    cpu.pc = 0x43AB8;
    10
}
/// $043834: bra.w $43aa4
pub fn i_043834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43838;
    cpu.ir = 0x6000;
    cpu.pc = 0x43AA4;
    10
}
/// $043838: bra.w $43a74
pub fn i_043838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4383C;
    cpu.ir = 0x6000;
    cpu.pc = 0x43A74;
    10
}
/// $043840: bra.w $43a58
pub fn i_043840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43844;
    cpu.ir = 0x6000;
    cpu.pc = 0x43A58;
    10
}
/// $043844: bra.w $43a42
pub fn i_043844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43848;
    cpu.ir = 0x6000;
    cpu.pc = 0x43A42;
    10
}
/// $04384C: bra.w $43a08
pub fn i_04384c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43850;
    cpu.ir = 0x6000;
    cpu.pc = 0x43A08;
    10
}
/// $043850: bra.w $439f0
pub fn i_043850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43854;
    cpu.ir = 0x6000;
    cpu.pc = 0x439F0;
    10
}
/// $043858: bra.w $43984
pub fn i_043858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4385C;
    cpu.ir = 0x6000;
    cpu.pc = 0x43984;
    10
}
/// $04385C: bra.w $43966
pub fn i_04385c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43860;
    cpu.ir = 0x6000;
    cpu.pc = 0x43966;
    10
}
/// $043860: bra.w $4394a
pub fn i_043860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43864;
    cpu.ir = 0x6000;
    cpu.pc = 0x4394A;
    10
}
/// $04394A: lea.l $a(a1), a0
pub fn i_04394a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4394E;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0xA);
    cpu.a[0] = ad1;
    16
}
/// $04394E: move.l a0, $10815c.l
pub fn i_04394e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43954;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043954: movea.l $2(a1), a0
pub fn i_043954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43958;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043958: movea.l $6(a1), a1
pub fn i_043958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4395C;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04395C: jsr $51b38.l
pub fn i_04395c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43962;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43962);
    cpu.pc = 0x51B38;
    16
}
/// $043962: bra.w $4380c
pub fn i_043962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43966;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043966: lea.l $6(a1), a0
pub fn i_043966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4396A;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $04396A: move.l a0, $10815c.l
pub fn i_04396a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43970;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043970: move.w $2(a1), $108164.l
pub fn i_043970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43978;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108164, v3 as u16);
    24
}
/// $043978: move.w $4(a1), $108166.l
pub fn i_043978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43980;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108166, v3 as u16);
    24
}
/// $043980: bra.w $4380c
pub fn i_043980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43984;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043984: lea.l $c(a1), a0
pub fn i_043984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43988;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0xC);
    cpu.a[0] = ad1;
    16
}
/// $043988: move.l a0, $10815c.l
pub fn i_043988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4398E;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04398E: move.w $2(a1), $108168.l
pub fn i_04398e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43996;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108168, v3 as u16);
    24
}
/// $043996: move.w $4(a1), $10816a.l
pub fn i_043996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4399E;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816A, v3 as u16);
    24
}
/// $04399E: move.w $6(a1), $10816c.l
pub fn i_04399e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439A6;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816C, v3 as u16);
    24
}
/// $0439A6: move.w $8(a1), $10816e.l
pub fn i_0439a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439AE;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816E, v3 as u16);
    24
}
/// $0439AE: move.w $a(a1), $108170.l
pub fn i_0439ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439B6;
    cpu.ir = 0x33E9;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108170, v3 as u16);
    24
}
/// $0439B6: bra.w $4380c
pub fn i_0439b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $0439F0: lea.l $e(a1), a0
pub fn i_0439f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439F4;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0xE);
    cpu.a[0] = ad1;
    16
}
/// $0439F4: move.l a0, $10815c.l
pub fn i_0439f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439FA;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0439FA: lea.l $6(a1), a0
pub fn i_0439fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x439FE;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $0439FE: movea.l $2(a1), a1
pub fn i_0439fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A02;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $043A02: jsr (a1)
pub fn i_043a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A04;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x43A04);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $043A04: bra.w $4380c
pub fn i_043a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A08;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A08: lea.l $4(a1), a0
pub fn i_043a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A0C;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[0] = ad1;
    16
}
/// $043A0C: move.l a0, $10815c.l
pub fn i_043a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A12;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A12: move.b $2(a1), $106eac.l
pub fn i_043a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A1A;
    cpu.ir = 0x13E9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106EAC, v3 as u8);
    24
}
/// $043A1A: bra.w $4380c
pub fn i_043a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A1E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A42: lea.l $2(a1), a0
pub fn i_043a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A46;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.a[0] = ad1;
    16
}
/// $043A46: jsr $2b58.l
pub fn i_043a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43A4C);
    cpu.pc = 0x2B58;
    16
}
/// $043A4C: addq.w #$2, a0
pub fn i_043a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A4E;
    cpu.ir = 0x5448;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $043A4E: move.l a0, $10815c.l
pub fn i_043a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A54;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A54: bra.w $4380c
pub fn i_043a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A58;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A58: lea.l $6(a1), a0
pub fn i_043a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A5C;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043A5C: move.l a0, $10815c.l
pub fn i_043a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A62;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A62: movea.l $2(a1), a0
pub fn i_043a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A66;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043A66: bsr.w $43df4
pub fn i_043a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A6A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43A6A);
    cpu.pc = 0x43DF4;
    18
}
/// $043A6A: jsr $51ece.l
pub fn i_043a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43A70);
    cpu.pc = 0x51ECE;
    16
}
/// $043A70: bra.w $4380c
pub fn i_043a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A74;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043A74: lea.l $6(a1), a0
pub fn i_043a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A78;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043A78: move.l a0, $10815c.l
pub fn i_043a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A7E;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043A7E: movea.l $2(a1), a0
pub fn i_043a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A82;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043A82: jsr $51ece.l
pub fn i_043a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43A88);
    cpu.pc = 0x51ECE;
    16
}
/// $043A88: bra.w $4380c
pub fn i_043a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43A8C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043AA4: jsr $2(a1)
pub fn i_043aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AA8;
    cpu.ir = 0x4EA9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.push32(bus, 0x43AA8);
    cpu.jump_to(ad1);
    16
}
/// $043AA8: tst.b d0
pub fn i_043aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AAA;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $043AAA: bne.w $43bc6
pub fn i_043aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AAE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43BC6; return 10; }
    8
}
/// $043AAE: move.l a1, $10815c.l
pub fn i_043aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AB4;
    cpu.ir = 0x23C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043AB4: bra.w $4380c
pub fn i_043ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AB8;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043AB8: lea.l $6(a1), a0
pub fn i_043ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ABC;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043ABC: move.l a0, $10815c.l
pub fn i_043abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AC2;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043AC2: movea.l $2(a1), a0
pub fn i_043ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AC6;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043AC6: clr.l $e(a0)
pub fn i_043ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ACA;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0xE);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $043ACA: jsr $51ed6.l
pub fn i_043aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43AD0);
    cpu.pc = 0x51ED6;
    16
}
/// $043AD0: jsr $51f30.l
pub fn i_043ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43AD6);
    cpu.pc = 0x51F30;
    16
}
/// $043AD6: bra.w $4380c
pub fn i_043ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ADA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043ADA: lea.l $8(a1), a0
pub fn i_043ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ADE;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x8);
    cpu.a[0] = ad1;
    16
}
/// $043ADE: move.l a0, $10815c.l
pub fn i_043ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AE4;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043AE4: move.b $6(a1), d0
pub fn i_043ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AE8;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $043AE8: move.b $7(a1), d1
pub fn i_043ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AEC;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $043AEC: movea.l $2(a1), a0
pub fn i_043aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AF0;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043AF0: jsr $51b1c.l
pub fn i_043af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43AF6);
    cpu.pc = 0x51B1C;
    16
}
/// $043AF6: bra.w $4380c
pub fn i_043af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043AFA: lea.l $6(a1), a0
pub fn i_043afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43AFE;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[0] = ad1;
    16
}
/// $043AFE: move.l a0, $10815c.l
pub fn i_043afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B04;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B04: move.w $2(a1), d0
pub fn i_043b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B08;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043B08: move.w $4(a1), d1
pub fn i_043b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B0C;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043B0C: move.b #$1, $108178.l
pub fn i_043b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B14;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x108178, v1 as u8);
    16
}
/// $043B14: clr.w $106f5c.l
pub fn i_043b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B1A;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F5C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F5C, r2 as u16);
    16
}
/// $043B1A: swap d0
pub fn i_043b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B1C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043B1C: swap d1
pub fn i_043b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B1E;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043B1E: clr.w d0
pub fn i_043b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B20;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043B20: clr.w d1
pub fn i_043b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B22;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043B22: move.l d0, $106f50.l
pub fn i_043b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B28;
    cpu.ir = 0x23C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(0x106F50, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B28: move.l d1, $106f54.l
pub fn i_043b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B2E;
    cpu.ir = 0x23C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    bus.write32(0x106F54, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B2E: move.l d1, $106f58.l
pub fn i_043b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B34;
    cpu.ir = 0x23C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    bus.write32(0x106F58, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B34: clr.w $108160.l
pub fn i_043b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B3A;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x108160) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x108160, r2 as u16);
    16
}
/// $043B3A: clr.w $108162.l
pub fn i_043b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B40;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x108162) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x108162, r2 as u16);
    16
}
/// $043B40: move.w $106f5c.l, $108172.l
pub fn i_043b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B4A;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108172, v2 as u16);
    28
}
/// $043B4A: move.w $106f50.l, $108174.l
pub fn i_043b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B54;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108174, v2 as u16);
    28
}
/// $043B54: move.w $106f58.l, $108176.l
pub fn i_043b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B5E;
    cpu.ir = 0x33F9;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108176, v2 as u16);
    28
}
/// $043B5E: bsr.w $434ea
pub fn i_043b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B62;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43B62);
    cpu.pc = 0x434EA;
    18
}
/// $043B62: bra.w $4380c
pub fn i_043b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B66;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043B66: lea.l $16(a1), a0
pub fn i_043b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B6A;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0x16);
    cpu.a[0] = ad1;
    16
}
/// $043B6A: move.l a0, $10815c.l
pub fn i_043b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B70;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043B70: movea.l $2(a1), a0
pub fn i_043b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B74;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043B74: move.l $e(a1), $78(a0)
pub fn i_043b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B7A;
    cpu.ir = 0x2169;
    let ad2 = cpu.a[1].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0].wrapping_add(0x78);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $043B7A: move.w $106f50.l, d0
pub fn i_043b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B80;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043B80: move.w $106f58.l, d1
pub fn i_043b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B86;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043B86: add.w $10817a.l, d1
pub fn i_043b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B8C;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x10817A) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043B8C: sub.w $12(a1), d0
pub fn i_043b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B90;
    cpu.ir = 0x9069;
    let ad2 = cpu.a[1].wrapping_add(0x12);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043B90: sub.w $14(a1), d1
pub fn i_043b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B94;
    cpu.ir = 0x9269;
    let ad2 = cpu.a[1].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043B94: lea.l $6(a1), a1
pub fn i_043b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B98;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $043B98: jsr $51b3e.l
pub fn i_043b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43B9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43B9E);
    cpu.pc = 0x51B3E;
    16
}
/// $043B9E: bra.w $4380c
pub fn i_043b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043BA2: lea.l $c(a1), a0
pub fn i_043ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BA6;
    cpu.ir = 0x41E9;
    let ad1 = cpu.a[1].wrapping_add(0xC);
    cpu.a[0] = ad1;
    16
}
/// $043BA6: move.l a0, $10815c.l
pub fn i_043ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BAC;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x10815C, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043BAC: movea.l $2(a1), a0
pub fn i_043bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BB0;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $043BB0: move.b $6(a1), $72(a0)
pub fn i_043bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BB6;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x72);
    bus.write8(ad4, v3 as u8);
    20
}
/// $043BB6: move.w $8(a1), $74(a0)
pub fn i_043bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BBC;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad4, v3 as u16);
    20
}
/// $043BBC: move.w $a(a1), $76(a0)
pub fn i_043bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BC2;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $043BC2: bra.w $4380c
pub fn i_043bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BC6;
    cpu.ir = 0x6000;
    cpu.pc = 0x4380C;
    10
}
/// $043BC6: move.l (a7)+, d0
pub fn i_043bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BC8;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $043BC8: move.l $106f50.l, d1
pub fn i_043bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BCE;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F50);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043BCE: move.w $108168.l, d2
pub fn i_043bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BD4;
    cpu.ir = 0x3439;
    let v2 = bus.read16(0x108168) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043BD4: move.w $10816c.l, d3
pub fn i_043bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BDA;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x10816C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043BDA: bsr.w $43e3a
pub fn i_043bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BDE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43BDE);
    cpu.pc = 0x43E3A;
    18
}
/// $043BDE: move.w d0, $108160.l
pub fn i_043bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BE4;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108160, v1 as u16);
    16
}
/// $043BE4: move.l d1, $106f50.l
pub fn i_043be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BEA;
    cpu.ir = 0x23C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    bus.write32(0x106F50, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043BEA: move.l (a7)+, d0
pub fn i_043bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BEC;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $043BEC: move.l $106f58.l, d1
pub fn i_043bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BF2;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F58);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043BF2: move.w $10816a.l, d2
pub fn i_043bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BF8;
    cpu.ir = 0x3439;
    let v2 = bus.read16(0x10816A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043BF8: move.w $10816e.l, d3
pub fn i_043bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43BFE;
    cpu.ir = 0x3639;
    let v2 = bus.read16(0x10816E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043BFE: move.w $108170.l, d4
pub fn i_043bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C04;
    cpu.ir = 0x3839;
    let v2 = bus.read16(0x108170) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C04: beq.w $43c36
pub fn i_043c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43C36; return 10; }
    8
}
/// $043C08: bmi.w $43c18
pub fn i_043c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C0C;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x43C18; return 10; }
    8
}
/// $043C0C: move.w $108160.l, d0
pub fn i_043c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C12;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108160) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C12: neg.w d0
pub fn i_043c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C14;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043C14: bra.w $43c1e
pub fn i_043c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C18;
    cpu.ir = 0x6000;
    cpu.pc = 0x43C1E;
    10
}
/// $043C18: move.w $108160.l, d0
pub fn i_043c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C1E;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108160) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C1E: swap d0
pub fn i_043c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C20;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043C20: clr.w d0
pub fn i_043c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C22;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043C22: move.w $10816c.l, d5
pub fn i_043c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C28;
    cpu.ir = 0x3A39;
    let v2 = bus.read16(0x10816C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C28: sub.w $106f50.l, d5
pub fn i_043c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C2E;
    cpu.ir = 0x9A79;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043C2E: muls.w d5, d4
pub fn i_043c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C30;
    cpu.ir = 0xC9C5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[4] as u16 as i16 as i32)) as u32;
    cpu.d[4] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $043C30: asr.l #$8, d4
pub fn i_043c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C32;
    cpu.ir = 0xE084;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[4] = r;
    22
}
/// $043C32: add.w d4, d2
pub fn i_043c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C34;
    cpu.ir = 0xD444;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043C34: add.w d4, d3
pub fn i_043c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C36;
    cpu.ir = 0xD644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043C36: bsr.w $43e3a
pub fn i_043c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C3A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43C3A);
    cpu.pc = 0x43E3A;
    18
}
/// $043C3A: move.w d0, $108162.l
pub fn i_043c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C40;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x108162, v1 as u16);
    16
}
/// $043C40: move.l d1, $106f58.l
pub fn i_043c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C46;
    cpu.ir = 0x23C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    bus.write32(0x106F58, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043C46: move.b $108178.l, d0
pub fn i_043c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C4C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x108178) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $043C4C: cmpi.b #$3, d0
pub fn i_043c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C50;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $043C50: bne.w $43c64
pub fn i_043c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C54;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43C64; return 10; }
    8
}
/// $043C54: move.w $106f58.l, d1
pub fn i_043c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C5A;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C5A: sub.w $108176.l, d1
pub fn i_043c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C60;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x108176) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043C60: bra.w $43ca0
pub fn i_043c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C64;
    cpu.ir = 0x6000;
    cpu.pc = 0x43CA0;
    10
}
/// $043C64: cmpi.b #$2, d0
pub fn i_043c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C68;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $043C68: bne.w $43c7c
pub fn i_043c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43C7C; return 10; }
    8
}
/// $043C6C: move.w $108176.l, d1
pub fn i_043c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C72;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x108176) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C72: sub.w $106f58.l, d1
pub fn i_043c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C78;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F58) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043C78: bra.w $43ca0
pub fn i_043c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x43CA0;
    10
}
/// $043C7C: cmpi.b #$0, d0
pub fn i_043c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C80;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $043C80: bne.w $43c94
pub fn i_043c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43C94; return 10; }
    8
}
/// $043C84: move.w $108174.l, d1
pub fn i_043c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C8A;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x108174) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C8A: sub.w $106f50.l, d1
pub fn i_043c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C90;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043C90: bra.w $43ca0
pub fn i_043c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C94;
    cpu.ir = 0x6000;
    cpu.pc = 0x43CA0;
    10
}
/// $043C94: move.w $106f50.l, d1
pub fn i_043c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43C9A;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043C9A: sub.w $108174.l, d1
pub fn i_043c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CA0;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x108174) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043CA0: add.w $108172.l, d1
pub fn i_043ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CA6;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x108172) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $043CA6: cmp.w $106f5c.l, d1
pub fn i_043ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CAC;
    cpu.ir = 0xB279;
    let v2 = bus.read16(0x106F5C) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043CAC: bls.w $43cb6
pub fn i_043cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CB0;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x43CB6; return 10; }
    8
}
/// $043CB0: move.w d1, $106f5c.l
pub fn i_043cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CB6;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F5C, v1 as u16);
    16
}
/// $043CB6: bsr.w $434f8
pub fn i_043cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CBA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43CBA);
    cpu.pc = 0x434F8;
    18
}
/// $043CBA: move.l $10817a.l, d0
pub fn i_043cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CC0;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x10817A);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043CC0: add.l $106f58.l, d0
pub fn i_043cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CC6;
    cpu.ir = 0xD0B9;
    let v2 = bus.read32(0x106F58);
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Long, true);
    cpu.d[0] = r;
    20
}
/// $043CC6: move.l d0, $106f54.l
pub fn i_043cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CCC;
    cpu.ir = 0x23C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    bus.write32(0x106F54, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $043CCC: move.w $108182.l, d0
pub fn i_043ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CD2;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108182) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043CD2: add.w d0, $108162.l
pub fn i_043cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CD8;
    cpu.ir = 0xD179;
    let v1 = bus.read16(0x108162) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, h1, Size::Word, true);
    bus.write16(0x108162, r2 as u16);
    16
}
/// $043CD8: jsr $4cb88.l
pub fn i_043cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43CDE);
    cpu.pc = 0x4CB88;
    16
}
/// $043CDE: bra.w $43d86
pub fn i_043cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x43D86;
    10
}
// ===== SceneScript_EdgeArrivalTest_043CE2  ($043CE2, 126 B, mission/scene_script_vm_0437da.s)
/// $043CE2: tst.b $108179.l
pub fn i_043ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CE8;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x108179) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $043CE8: bne.w $43d66
pub fn i_043ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043CEC: tst.b $10e39a.l
pub fn i_043cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CF2;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E39A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $043CF2: beq.w $43d66
pub fn i_043cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043CF6: move.b $108178.l, d0
pub fn i_043cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43CFC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x108178) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $043CFC: cmpi.b #$3, d0
pub fn i_043cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D00;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $043D00: bne.w $43d18
pub fn i_043d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D04;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43D18; return 10; }
    8
}
/// $043D04: move.w $106f58.l, d0
pub fn i_043d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D0A;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043D0A: cmp.w $10816e.l, d0
pub fn i_043d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D10;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x10816E) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043D10: beq.w $43d66
pub fn i_043d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D14;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043D14: bra.w $43d60
pub fn i_043d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D18;
    cpu.ir = 0x6000;
    cpu.pc = 0x43D60;
    10
}
/// $043D18: cmpi.b #$2, d0
pub fn i_043d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D1C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $043D1C: bne.w $43d34
pub fn i_043d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D20;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43D34; return 10; }
    8
}
/// $043D20: move.w $106f58.l, d0
pub fn i_043d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D26;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F58) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043D26: cmp.w $10816a.l, d0
pub fn i_043d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D2C;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x10816A) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043D2C: beq.w $43d66
pub fn i_043d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D30;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043D30: bra.w $43d60
pub fn i_043d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D34;
    cpu.ir = 0x6000;
    cpu.pc = 0x43D60;
    10
}
/// $043D34: cmpi.b #$0, d0
pub fn i_043d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D38;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $043D38: bne.w $43d50
pub fn i_043d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D3C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43D50; return 10; }
    8
}
/// $043D3C: move.w $106f50.l, d0
pub fn i_043d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D42;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043D42: cmp.w $108168.l, d0
pub fn i_043d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D48;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x108168) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043D48: beq.w $43d66
pub fn i_043d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
/// $043D4C: bra.w $43d60
pub fn i_043d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D50;
    cpu.ir = 0x6000;
    cpu.pc = 0x43D60;
    10
}
/// $043D50: move.w $106f50.l, d0
pub fn i_043d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D56;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043D56: cmp.w $10816c.l, d0
pub fn i_043d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D5C;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x10816C) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $043D5C: beq.w $43d66
pub fn i_043d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43D66; return 10; }
    8
}
// ===== ClearC_043d60  ($043D60, 6 B, thunks/ccr_helpers.c)
/// $043D60: andi.b #$fe, ccr
pub fn i_043d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D64;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $043D64: rts
pub fn i_043d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_043d66  ($043D66, 6 B, thunks/ccr_helpers.c)
/// $043D66: ori.b #$1, ccr
pub fn i_043d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D6A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $043D6A: rts
pub fn i_043d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Reset4CameraLongs_043D6C  ($043D6C, 26 B, video/camera_list_ctx_helpers_025012.s)
/// $043D6C: clr.l $106f7a.l
pub fn i_043d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D72;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x106F7A);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x106F7A, r2);
    20
}
/// $043D72: clr.l $107ff6.l
pub fn i_043d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D78;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x107FF6);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x107FF6, r2);
    20
}
/// $043D78: clr.l $108072.l
pub fn i_043d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D7E;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x108072);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x108072, r2);
    20
}
/// $043D7E: clr.l $1080ee.l
pub fn i_043d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D84;
    cpu.ir = 0x42B9;
    let v1 = bus.read32(0x1080EE);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write32(0x1080EE, r2);
    20
}
/// $043D84: rts
pub fn i_043d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CameraApplyAll4_043D86  ($043D86, 36 B, video/camera_list_ctx_helpers_025012.s)
/// $043D86: lea.l $106f6c.l, a0
pub fn i_043d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $043D8C: bsr.w $43daa
pub fn i_043d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D90;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43D90);
    cpu.pc = 0x43DAA;
    18
}
/// $043D90: lea.l $107fe8.l, a0
pub fn i_043d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D96;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x107FE8;
    20
}
/// $043D96: bsr.w $43daa
pub fn i_043d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43D9A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43D9A);
    cpu.pc = 0x43DAA;
    18
}
/// $043D9A: lea.l $108064.l, a0
pub fn i_043d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x108064;
    20
}
/// $043DA0: bsr.w $43daa
pub fn i_043da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DA4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43DA4);
    cpu.pc = 0x43DAA;
    18
}
/// $043DA4: lea.l $1080e0.l, a0
pub fn i_043da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1080E0;
    20
}
// ===== CameraApplyOne_043DAA  ($043DAA, 74 B, video/camera_apply_043daa.s)
/// $043DAA: tst.l $e(a0)
pub fn i_043daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DAE;
    cpu.ir = 0x4AA8;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read32(ad2);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $043DAE: beq.w $43df2
pub fn i_043dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DB2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43DF2; return 10; }
    8
}
/// $043DB2: move.w $108160.l, d0
pub fn i_043db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DB8;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x108160) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043DB8: move.w $108162.l, d1
pub fn i_043db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DBE;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x108162) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $043DBE: muls.w $74(a0), d0
pub fn i_043dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DC2;
    cpu.ir = 0xC1E8;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let r = ((v3 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    78
}
/// $043DC2: asl.l #$8, d0
pub fn i_043dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DC4;
    cpu.ir = 0xE180;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, true);
    cpu.d[0] = r;
    22
}
/// $043DC4: muls.w $76(a0), d1
pub fn i_043dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DC8;
    cpu.ir = 0xC3E8;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    let r = ((v3 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    78
}
/// $043DC8: asl.l #$8, d1
pub fn i_043dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DCA;
    cpu.ir = 0xE181;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, true);
    cpu.d[1] = r;
    22
}
/// $043DCA: jsr $51b80.l
pub fn i_043dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43DD0);
    cpu.pc = 0x51B80;
    16
}
/// $043DD0: btst.b #$0, $72(a0)
pub fn i_043dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DD6;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $043DD6: beq.w $43dde
pub fn i_043dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DDA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43DDE; return 10; }
    8
}
/// $043DDA: bsr.w $43e0e
pub fn i_043dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DDE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43DDE);
    cpu.pc = 0x43E0E;
    18
}
/// $043DDE: btst.b #$1, $72(a0)
pub fn i_043dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DE4;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $043DE4: beq.w $43dec
pub fn i_043de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43DEC; return 10; }
    8
}
/// $043DE8: bsr.w $43e24
pub fn i_043de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DEC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x43DEC);
    cpu.pc = 0x43E24;
    18
}
/// $043DEC: jsr $51f30.l
pub fn i_043dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43DF2);
    cpu.pc = 0x51F30;
    16
}
/// $043DF2: rts
pub fn i_043df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CameraHook_Probe08_043DF4  ($043DF4, 26 B, video/camera_apply_043daa.s)
/// $043DF4: jsr $51c08.l
pub fn i_043df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43DFA);
    cpu.pc = 0x51C08;
    16
}
/// $043DFA: bcc.w $43e0c
pub fn i_043dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43DFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x43E0C; return 10; }
    8
}
/// $043DFE: movea.l $78(a0), a1
pub fn i_043dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E02;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $043E02: move.l a1, d7
pub fn i_043e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E04;
    cpu.ir = 0x2E09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E04: beq.w $43e0c
pub fn i_043e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43E0C; return 10; }
    8
}
/// $043E08: bra.w $43e8c
pub fn i_043e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x43E8C;
    10
}
/// $043E0C: rts
pub fn i_043e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CameraHook_Probe82_043E0E  ($043E0E, 22 B, video/camera_apply_043daa.s)
/// $043E0E: jsr $51c82.l
pub fn i_043e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43E14);
    cpu.pc = 0x51C82;
    16
}
/// $043E14: bcc.b $43e0c
pub fn i_043e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E16;
    cpu.ir = 0x64F6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x43E0C; return 10; }
    8
}
/// $043E16: movea.l $78(a0), a1
pub fn i_043e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E1A;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $043E1A: move.l a1, d7
pub fn i_043e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E1C;
    cpu.ir = 0x2E09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E1C: beq.b $43e0c
pub fn i_043e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E1E;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43E0C; return 10; }
    8
}
/// $043E1E: bra.w $43e8c
pub fn i_043e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E22;
    cpu.ir = 0x6000;
    cpu.pc = 0x43E8C;
    10
}
// ===== CameraHook_ProbeF6_043E24  ($043E24, 22 B, video/camera_apply_043daa.s)
/// $043E24: jsr $51cf6.l
pub fn i_043e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x43E2A);
    cpu.pc = 0x51CF6;
    16
}
/// $043E2A: bcc.b $43e0c
pub fn i_043e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E2C;
    cpu.ir = 0x64E0;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x43E0C; return 10; }
    8
}
/// $043E2C: movea.l $78(a0), a1
pub fn i_043e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E30;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $043E30: move.l a1, d7
pub fn i_043e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E32;
    cpu.ir = 0x2E09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E32: beq.b $43e0c
pub fn i_043e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E34;
    cpu.ir = 0x67D8;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43E0C; return 10; }
    8
}
/// $043E34: bra.w $43e8c
pub fn i_043e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E38;
    cpu.ir = 0x6000;
    cpu.pc = 0x43E8C;
    10
}
// ===== Scroll_ClampToRange_043E3A  ($043E3A, 78 B, mission/scene_script_vm_0437da.s)
/// $043E3A: swap d2
pub fn i_043e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E3C;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E3C: swap d3
pub fn i_043e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E3E;
    cpu.ir = 0x4843;
    cpu.d[3] = cpu.d[3].rotate_left(16);
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E3E: clr.w d2
pub fn i_043e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E40;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043E40: move.w #$ffff, d3
pub fn i_043e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E44;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043E44: tst.l d0
pub fn i_043e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E46;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E46: beq.w $43e88
pub fn i_043e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E4A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43E88; return 10; }
    8
}
/// $043E4A: bmi.w $43e5a
pub fn i_043e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E4E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x43E5A; return 10; }
    8
}
/// $043E4E: cmp.l d1, d3
pub fn i_043e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E50;
    cpu.ir = 0xB681;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $043E50: bcs.w $43e88
pub fn i_043e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E54;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x43E88; return 10; }
    8
}
/// $043E54: move.l d3, d2
pub fn i_043e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E56;
    cpu.ir = 0x2403;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E56: bra.w $43e60
pub fn i_043e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x43E60;
    10
}
/// $043E5A: cmp.l d1, d2
pub fn i_043e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E5C;
    cpu.ir = 0xB481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $043E5C: bcc.w $43e88
pub fn i_043e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x43E88; return 10; }
    8
}
/// $043E60: add.l d1, d0
pub fn i_043e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E62;
    cpu.ir = 0xD081;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $043E62: cmpi.l #$ffffffff, d2
pub fn i_043e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E68;
    cpu.ir = 0xC82;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $043E68: beq.w $43e7c
pub fn i_043e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E6C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43E7C; return 10; }
    8
}
/// $043E6C: move.l d2, d3
pub fn i_043e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E6E;
    cpu.ir = 0x2602;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E6E: move.l d2, d4
pub fn i_043e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E70;
    cpu.ir = 0x2802;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E70: sub.l d0, d3
pub fn i_043e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E72;
    cpu.ir = 0x9680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[3] = r;
    4
}
/// $043E72: sub.l d1, d4
pub fn i_043e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E74;
    cpu.ir = 0x9881;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[4] = r;
    4
}
/// $043E74: eor.l d3, d4
pub fn i_043e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E76;
    cpu.ir = 0xB784;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 ^ cpu.d[3]) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    4
}
/// $043E76: bpl.w $43e7c
pub fn i_043e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E7A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x43E7C; return 10; }
    8
}
/// $043E7A: move.l d2, d0
pub fn i_043e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E7C;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E7C: move.l d0, d2
pub fn i_043e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E7E;
    cpu.ir = 0x2400;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[2] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E7E: swap d0
pub fn i_043e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E80;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E80: swap d1
pub fn i_043e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E82;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E82: sub.w d1, d0
pub fn i_043e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E84;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043E84: move.l d2, d1
pub fn i_043e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E86;
    cpu.ir = 0x2202;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043E86: rts
pub fn i_043e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClrWD0_00043E88  ($043E88, 4 B, thunks/trivial_returns.c)
/// $043E88: clr.w d0
pub fn i_043e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E8A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043E8A: rts
pub fn i_043e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== BlitterTile_2D_043E8C  ($043E8C, 78 B, video/camera_sprite_helpers_043e8c.s)
/// $043E8C: add.w $22(a0), d0
pub fn i_043e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E90;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043E90: add.w $24(a0), d1
pub fn i_043e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E94;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043E94: mulu.w d6, d2
pub fn i_043e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E96;
    cpu.ir = 0xC4C6;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[2] & 0xFFFF);
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $043E96: add.w d3, d2
pub fn i_043e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E98;
    cpu.ir = 0xD443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043E98: sub.w d5, d6
pub fn i_043e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E9A;
    cpu.ir = 0x9C45;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043E9A: add.w d2, d2
pub fn i_043e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E9C;
    cpu.ir = 0xD442;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043E9C: add.w d6, d6
pub fn i_043e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43E9E;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043E9E: add.w d2, d2
pub fn i_043e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EA0;
    cpu.ir = 0xD442;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043EA0: add.w d6, d6
pub fn i_043ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EA2;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043EA2: lea.l (a1, d2.w), a1
pub fn i_043ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EA6;
    cpu.ir = 0x43F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $043EA6: lea.l $7c(a0), a2
pub fn i_043ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EAA;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x7C);
    cpu.a[2] = ad1;
    16
}
/// $043EAA: lsl.w #$7, d0
pub fn i_043eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EAC;
    cpu.ir = 0xEF48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $043EAC: lsl.w #$2, d1
pub fn i_043eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EAE;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $043EAE: subq.w #$1, d4
pub fn i_043eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EB0;
    cpu.ir = 0x5344;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043EB0: subq.w #$1, d5
pub fn i_043eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EB2;
    cpu.ir = 0x5345;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043EB2: move.w d5, d2
pub fn i_043eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EB4;
    cpu.ir = 0x3405;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043EB4: move.w d1, d3
pub fn i_043eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EB6;
    cpu.ir = 0x3601;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043EB6: andi.w #$f80, d0
pub fn i_043eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EBA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF80) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043EBA: lea.l (a2, d0.w), a3
pub fn i_043eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EBE;
    cpu.ir = 0x47F2;
    let ad1 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[3] = ad1;
    18
}
/// $043EBE: andi.w #$7c, d3
pub fn i_043ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EC2;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x7C) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043EC2: move.l (a1)+, (a3, d3.w)
pub fn i_043ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EC6;
    cpu.ir = 0x2799;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $043EC6: addq.w #$4, d3
pub fn i_043ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EC8;
    cpu.ir = 0x5843;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043EC8: dbra d2, $43ebe
pub fn i_043ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ECC;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x43ECC; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x43ECC; return 14; }
    cpu.pc = 0x43EBE;
    10
}
/// $043ECC: lea.l (a1, d6.w), a1
pub fn i_043ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ED0;
    cpu.ir = 0x43F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.a[1] = ad1;
    18
}
/// $043ED0: addi.w #$80, d0
pub fn i_043ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ED4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043ED4: dbra d4, $43eb2
pub fn i_043ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43ED8;
    cpu.ir = 0x51CC;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x43ED8; return 12; }
    let n = (cpu.d[4] as u16).wrapping_sub(1);
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x43ED8; return 14; }
    cpu.pc = 0x43EB2;
    10
}
/// $043ED8: rts
pub fn i_043ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Buffer_ClearBlock1024L_043EDA  ($043EDA, 18 B, video/coord_camera_043eda.s)
/// $043EDA: lea.l $7c(a0), a1
pub fn i_043eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EDE;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x7C);
    cpu.a[1] = ad1;
    16
}
/// $043EDE: move.l #$3ff, d7
pub fn i_043ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EE4;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x3FF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $043EE4: clr.l (a1)+
pub fn i_043ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EE6;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $043EE6: dbra d7, $43ee4
pub fn i_043ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EEA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x43EEA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x43EEA; return 14; }
    cpu.pc = 0x43EE4;
    10
}
/// $043EEA: rts
pub fn i_043eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43EEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CollMap_TestPoint_043F02  ($043F02, 80 B, mission/scene_script_vm_0437da.s)
/// $043F02: lea.l $106f6c.l, a0
pub fn i_043f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $043F08: add.w $4(a0), d0
pub fn i_043f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F0C;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F0C: neg.w d1
pub fn i_043f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F0E;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043F0E: add.w $8(a0), d1
pub fn i_043f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F12;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F12: move.w $22(a0), d2
pub fn i_043f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F16;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043F16: lsl.w #$4, d2
pub fn i_043f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F18;
    cpu.ir = 0xE94A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $043F18: add.w d2, d0
pub fn i_043f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F1A;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043F1A: move.w d0, d3
pub fn i_043f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F1C;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043F1C: andi.w #$7, d3
pub fn i_043f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F20;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F20: move.w $24(a0), d2
pub fn i_043f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F24;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043F24: lsl.w #$4, d2
pub fn i_043f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F26;
    cpu.ir = 0xE94A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $043F26: add.w d2, d1
pub fn i_043f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F28;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043F28: move.w d1, d4
pub fn i_043f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F2A;
    cpu.ir = 0x3801;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043F2A: andi.w #$7, d4
pub fn i_043f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F2E;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F2E: move.w d0, d2
pub fn i_043f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F30;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043F30: andi.w #$1f0, d0
pub fn i_043f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F34;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F34: andi.w #$8, d2
pub fn i_043f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F38;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F38: andi.w #$1f8, d1
pub fn i_043f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F3C;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F3C: ror.w #$2, d1
pub fn i_043f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F3E;
    cpu.ir = 0xE459;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 3, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $043F3E: ror.w #$3, d2
pub fn i_043f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F40;
    cpu.ir = 0xE65A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 3, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F40: rol.w #$3, d0
pub fn i_043f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F42;
    cpu.ir = 0xE758;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 3, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F42: add.w d1, d0
pub fn i_043f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F44;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043F44: add.w d2, d0
pub fn i_043f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F46;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043F46: lea.l $7c(a0), a0
pub fn i_043f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F4A;
    cpu.ir = 0x41E8;
    let ad1 = cpu.a[0].wrapping_add(0x7C);
    cpu.a[0] = ad1;
    16
}
/// $043F4A: move.b (a0, d0.w), d0
pub fn i_043f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F4E;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $043F4E: bne.w $43f58
pub fn i_043f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43F58; return 10; }
    8
}
// ===== ClearC_043f52  ($043F52, 6 B, thunks/ccr_helpers.c)
/// $043F52: andi.b #$fe, ccr
pub fn i_043f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F56;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $043F56: rts
pub fn i_043f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_043f58  ($043F58, 6 B, thunks/ccr_helpers.c)
/// $043F58: ori.b #$1, ccr
pub fn i_043f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F5C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $043F5C: rts
pub fn i_043f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlit_TileByCoord_043F5E  ($043F5E, 78 B, video/coord_camera_043eda.s)
/// $043F5E: lea.l $106f6c.l, a0
pub fn i_043f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106F6C;
    20
}
/// $043F64: add.w $4(a0), d0
pub fn i_043f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F68;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F68: neg.w d1
pub fn i_043f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F6A;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043F6A: add.w $8(a0), d1
pub fn i_043f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F6E;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F6E: move.w $22(a0), d2
pub fn i_043f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F72;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043F72: lsl.w #$4, d2
pub fn i_043f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F74;
    cpu.ir = 0xE94A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $043F74: add.w d2, d0
pub fn i_043f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F76;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043F76: move.w d0, d3
pub fn i_043f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F78;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043F78: andi.w #$7, d3
pub fn i_043f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F7C;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F7C: move.w $24(a0), d2
pub fn i_043f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F80;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043F80: lsl.w #$4, d2
pub fn i_043f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F82;
    cpu.ir = 0xE94A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $043F82: add.w d2, d1
pub fn i_043f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F84;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043F84: move.w d1, d4
pub fn i_043f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F86;
    cpu.ir = 0x3801;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043F86: andi.w #$7, d4
pub fn i_043f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F8A;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F8A: move.w d0, d2
pub fn i_043f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F8C;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $043F8C: andi.w #$1f0, d0
pub fn i_043f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F90;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F90: andi.w #$8, d2
pub fn i_043f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F94;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F94: andi.w #$1f8, d1
pub fn i_043f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F98;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $043F98: ror.w #$2, d1
pub fn i_043f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F9A;
    cpu.ir = 0xE459;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 3, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $043F9A: ror.w #$3, d2
pub fn i_043f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F9C;
    cpu.ir = 0xE65A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 3, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F9C: rol.w #$3, d0
pub fn i_043f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43F9E;
    cpu.ir = 0xE758;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 3, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043F9E: add.w d1, d0
pub fn i_043f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FA0;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043FA0: add.w d2, d0
pub fn i_043fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FA2;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043FA2: lea.l $7c(a0), a0
pub fn i_043fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FA6;
    cpu.ir = 0x41E8;
    let ad1 = cpu.a[0].wrapping_add(0x7C);
    cpu.a[0] = ad1;
    16
}
/// $043FA6: move.b d7, (a0, d0.w)
pub fn i_043fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FAA;
    cpu.ir = 0x1187;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $043FAA: rts
pub fn i_043faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FAC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_MultiBlitClippedX  ($043FAC, 98 B, video/sprite_multi_blit_043fac.s)
/// $043FAC: move.w $22(a6), d0
pub fn i_043fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FB0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043FB0: move.w $24(a6), d1
pub fn i_043fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FB4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043FB4: move.w (a1), d2
pub fn i_043fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FB6;
    cpu.ir = 0x3411;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $043FB6: btst.b #$0, $3a(a6)
pub fn i_043fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FBC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $043FBC: beq.w $43fc0
pub fn i_043fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x43FC0; return 10; }
    8
}
/// $043FC0: add.w d2, d0
pub fn i_043fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FC2;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $043FC2: add.w $2(a1), d1
pub fn i_043fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FC6;
    cpu.ir = 0xD269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $043FC6: move.w $4(a1), d2
pub fn i_043fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FCA;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043FCA: move.w $6(a1), d3
pub fn i_043fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FCE;
    cpu.ir = 0x3629;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $043FCE: addq.l #$8, a1
pub fn i_043fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FD0;
    cpu.ir = 0x5089;
    cpu.a[1] = cpu.a[1].wrapping_add(8);
    8
}
/// $043FD0: movem.l d1/d3, -(a7)
pub fn i_043fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FD4;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $043FD4: cmpi.w #$ffb0, d0
pub fn i_043fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FD8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFB0, Size::Word, false);
    8
}
/// $043FD8: blt.w $43ff2
pub fn i_043fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FDC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x43FF2; return 10; }
    8
}
/// $043FDC: cmpi.w #$188, d0
pub fn i_043fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FE0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x188, Size::Word, false);
    8
}
/// $043FE0: bgt.w $43ff2
pub fn i_043fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FE4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x43FF2; return 10; }
    8
}
/// $043FE4: move.b (a1)+, d7
pub fn i_043fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FE6;
    cpu.ir = 0x1E19;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    8
}
/// $043FE6: movem.l d0-d3/a1, -(a7)
pub fn i_043fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FEA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $043FEA: jsr $43f5e(pc)
pub fn i_043fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x43FEE);
    cpu.pc = 0x43F5E;
    16
}
/// $043FEE: movem.l (a7)+, d0-d3/a1
pub fn i_043fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FF2;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $043FF2: subq.w #$8, d1
pub fn i_043ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FF4;
    cpu.ir = 0x5141;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043FF4: subq.w #$1, d3
pub fn i_043ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FF6;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043FF6: bne.b $43fd4
pub fn i_043ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FF8;
    cpu.ir = 0x66DC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43FD4; return 10; }
    8
}
/// $043FF8: movem.l (a7)+, d1/d3
pub fn i_043ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FFC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $043FFC: addq.w #$8, d0
pub fn i_043ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x43FFE;
    cpu.ir = 0x5040;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $043FFE: btst.b #$0, $3a(a6)
pub fn i_043ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44004;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044004: beq.w $44008
pub fn i_044004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44008;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44008; return 10; }
    8
}
/// $044008: subq.w #$1, d2
pub fn i_044008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4400A;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04400A: bne.b $43fd0
pub fn i_04400a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4400C;
    cpu.ir = 0x66C4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x43FD0; return 10; }
    8
}
/// $04400C: rts
pub fn i_04400c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4400E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_ApplyCameraDeltaToSelf_044048  ($044048, 22 B, video/coord_camera_043eda.s)
/// $044048: move.w $106f6c.l, d0
pub fn i_044048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4404E;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04404E: sub.w d0, $22(a6)
pub fn i_04404e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44052;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $044052: move.w $106f6e.l, d0
pub fn i_044052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44058;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $044058: add.w d0, $24(a6)
pub fn i_044058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4405C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04405C: rts
pub fn i_04405c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4405E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_ApplyCameraDeltaToA0Mark_04405E  ($04405E, 28 B, video/coord_camera_043eda.s)
/// $04405E: move.w $106f6c.l, d0
pub fn i_04405e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44064;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $044064: sub.w d0, $22(a0)
pub fn i_044064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44068;
    cpu.ir = 0x9168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $044068: move.w $106f6e.l, d0
pub fn i_044068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4406E;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F6E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04406E: add.w d0, $24(a0)
pub fn i_04406e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44072;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $044072: bset.b #$6, $13(a0)
pub fn i_044072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44078;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044078: rts
pub fn i_044078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4407A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_LocalToScreenSecondary_0440BC  ($0440BC, 20 B, video/coord_camera_043eda.s)
/// $0440BC: add.w $106f50.l, d0
pub fn i_0440bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440C2;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0440C2: subi.w #$200, d1
pub fn i_0440c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440C6;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0440C6: neg.w d1
pub fn i_0440c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440C8;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0440C8: add.w $106f54.l, d1
pub fn i_0440c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440CE;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0440CE: rts
pub fn i_0440ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Coord_ScreenToLocalSecondary_0440D0  ($0440D0, 20 B, video/coord_camera_043eda.s)
/// $0440D0: sub.w $106f50.l, d0
pub fn i_0440d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440D6;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0440D6: sub.w $106f54.l, d1
pub fn i_0440d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440DC;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0440DC: neg.w d1
pub fn i_0440dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440DE;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0440DE: addi.w #$200, d1
pub fn i_0440de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440E2;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0440E2: rts
pub fn i_0440e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Handler_ApplyCameraSelf_0440E4  ($0440E4, 158 B, video/coord_transform_handlers_0440e4.s)
/// $0440E4: btst.b #$6, $13(a6)
pub fn i_0440e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440EA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0440EA: bne.w $44180
pub fn i_0440ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44180; return 10; }
    8
}
/// $0440EE: move.w $22(a6), d0
pub fn i_0440ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440F2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0440F2: move.w $24(a6), d1
pub fn i_0440f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440F6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0440F6: bset.b #$6, $13(a6)
pub fn i_0440f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x440FC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0440FC: jsr $999de.l
pub fn i_0440fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44102;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44102);
    cpu.pc = 0x999DE;
    16
}
/// $044102: bcs.w $44116
pub fn i_044102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44106;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x44116; return 10; }
    8
}
/// $044106: sub.w $106f6c.l, d0
pub fn i_044106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4410C;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F6C) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04410C: add.w $106f6e.l, d1
pub fn i_04410c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44112;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F6E) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044112: bra.w $4415e
pub fn i_044112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44116;
    cpu.ir = 0x6000;
    cpu.pc = 0x4415E;
    10
}
/// $044116: btst.b #$7, $13(a6)
pub fn i_044116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4411C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04411C: beq.w $44128
pub fn i_04411c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44120;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44128; return 10; }
    8
}
/// $044120: add.w d2, d0
pub fn i_044120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44122;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044122: add.w d3, d1
pub fn i_044122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44124;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044124: bra.w $4415e
pub fn i_044124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44128;
    cpu.ir = 0x6000;
    cpu.pc = 0x4415E;
    10
}
/// $044128: btst.b #$6, $6b(a6)
pub fn i_044128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4412E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04412E: bne.w $44148
pub fn i_04412e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44132;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44148; return 10; }
    8
}
/// $044132: btst.b #$1, $6b(a6)
pub fn i_044132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44138;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044138: bne.w $44148
pub fn i_044138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4413C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44148; return 10; }
    8
}
/// $04413C: btst.b #$0, $6b(a6)
pub fn i_04413c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44142;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044142: bne.w $44148
pub fn i_044142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44146;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44148; return 10; }
    8
}
/// $044146: bra.b $44120
pub fn i_044146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44148;
    cpu.ir = 0x60D8;
    cpu.pc = 0x44120;
    10
}
/// $044148: cmpi.w #$1e, $22(a6)
pub fn i_044148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4414E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Word, false);
    16
}
/// $04414E: sgt.b d4
pub fn i_04414e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44150;
    cpu.ir = 0x5EC4;
    let c1: u32 = if cpu.sr.evaluate(Condition::Gt) { 0xFF } else { 0 };
    let v2 = cpu.d[4] & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (c1 & 0xFF);
    4
}
/// $044150: tst.w d2
pub fn i_044150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44152;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $044152: spl.b d5
pub fn i_044152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44154;
    cpu.ir = 0x5AC5;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[5] & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (c1 & 0xFF);
    4
}
/// $044154: eor.b d4, d5
pub fn i_044154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44156;
    cpu.ir = 0xB905;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 ^ cpu.d[4]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $044156: beq.w $4415c
pub fn i_044156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4415A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4415C; return 10; }
    8
}
/// $04415A: add.w d2, d0
pub fn i_04415a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4415C;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04415C: add.w d3, d1
pub fn i_04415c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4415E;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04415E: cmpi.b #$1, $106ece.l
pub fn i_04415e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44166;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $044166: bne.w $44178
pub fn i_044166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4416A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44178; return 10; }
    8
}
/// $04416A: tst.w $106f5e.l
pub fn i_04416a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44170;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F5E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $044170: beq.w $44178
pub fn i_044170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44174;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44178; return 10; }
    8
}
/// $044174: move.w $22(a6), d0
pub fn i_044174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44178;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $044178: move.w d0, $22(a6)
pub fn i_044178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4417C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04417C: move.w d1, $24(a6)
pub fn i_04417c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44180;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044180: rts
pub fn i_044180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44182;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Handler_ApplyCameraGlobals_044182  ($044182, 168 B, video/coord_transform_handlers_0440e4.s)
/// $044182: btst.b #$6, $13(a6)
pub fn i_044182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44188;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044188: bne.w $44228
pub fn i_044188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4418C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44228; return 10; }
    8
}
/// $04418C: move.w $106f38.l, d0
pub fn i_04418c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44192;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F38) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $044192: move.w $106f3a.l, d1
pub fn i_044192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44198;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F3A) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $044198: bset.b #$6, $13(a6)
pub fn i_044198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4419E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04419E: jsr $999de.l
pub fn i_04419e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x441A4);
    cpu.pc = 0x999DE;
    16
}
/// $0441A4: bcs.w $441b8
pub fn i_0441a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441A8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x441B8; return 10; }
    8
}
/// $0441A8: sub.w $106f6c.l, d0
pub fn i_0441a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441AE;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F6C) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0441AE: add.w $106f6e.l, d1
pub fn i_0441ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441B4;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F6E) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0441B4: bra.w $44200
pub fn i_0441b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x44200;
    10
}
/// $0441B8: btst.b #$7, $13(a6)
pub fn i_0441b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441BE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0441BE: beq.w $441ca
pub fn i_0441be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441C2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x441CA; return 10; }
    8
}
/// $0441C2: add.w d2, d0
pub fn i_0441c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441C4;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0441C4: add.w d3, d1
pub fn i_0441c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441C6;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0441C6: bra.w $44200
pub fn i_0441c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x44200;
    10
}
/// $0441CA: btst.b #$6, $6b(a6)
pub fn i_0441ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441D0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0441D0: bne.w $441ea
pub fn i_0441d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x441EA; return 10; }
    8
}
/// $0441D4: btst.b #$1, $6b(a6)
pub fn i_0441d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441DA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0441DA: bne.w $441ea
pub fn i_0441da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x441EA; return 10; }
    8
}
/// $0441DE: btst.b #$0, $6b(a6)
pub fn i_0441de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441E4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0441E4: bne.w $441ea
pub fn i_0441e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x441EA; return 10; }
    8
}
/// $0441E8: bra.b $441c2
pub fn i_0441e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441EA;
    cpu.ir = 0x60D8;
    cpu.pc = 0x441C2;
    10
}
/// $0441EA: cmpi.w #$1e, $22(a6)
pub fn i_0441ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441F0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Word, false);
    16
}
/// $0441F0: sgt.b d4
pub fn i_0441f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441F2;
    cpu.ir = 0x5EC4;
    let c1: u32 = if cpu.sr.evaluate(Condition::Gt) { 0xFF } else { 0 };
    let v2 = cpu.d[4] & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0441F2: tst.w d2
pub fn i_0441f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441F4;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0441F4: spl.b d5
pub fn i_0441f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441F6;
    cpu.ir = 0x5AC5;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[5] & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0441F6: eor.b d4, d5
pub fn i_0441f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441F8;
    cpu.ir = 0xB905;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 ^ cpu.d[4]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0441F8: beq.w $441fe
pub fn i_0441f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x441FE; return 10; }
    8
}
/// $0441FC: add.w d2, d0
pub fn i_0441fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x441FE;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0441FE: add.w d3, d1
pub fn i_0441fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44200;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044200: cmpi.b #$1, $106ece.l
pub fn i_044200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44208;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $044208: bne.w $4421c
pub fn i_044208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4420C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4421C; return 10; }
    8
}
/// $04420C: tst.w $106f5e.l
pub fn i_04420c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44212;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106F5E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $044212: beq.w $4421c
pub fn i_044212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44216;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4421C; return 10; }
    8
}
/// $044216: move.w $106f38.l, d0
pub fn i_044216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4421C;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F38) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04421C: move.w d0, $106f38.l
pub fn i_04421c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44222;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F38, v1 as u16);
    16
}
/// $044222: move.w d1, $106f3a.l
pub fn i_044222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44228;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F3A, v1 as u16);
    16
}
/// $044228: rts
pub fn i_044228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4422A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionBoot_Run_044236  ($044236, 16 B, mission/event_vm_04422a.s)
/// $044236: lea.l $4422e(pc), a0
pub fn i_044236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4423A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4422E;
    16
}
/// $04423A: jsr $43562(pc)
pub fn i_04423a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4423E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4423E);
    cpu.pc = 0x43562;
    16
}
/// $04423E: moveq #$0, d0
pub fn i_04423e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44240;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $044240: moveq #$0, d1
pub fn i_044240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44242;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $044242: jmp $437da(pc)
pub fn i_044242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44246;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x437DA;
    8
}
// ===== MissionDriver_Init_0442E6  ($0442E6, 130 B, mission/event_vm_04422a.s)
/// $0442E6: moveq #$0, d0
pub fn i_0442e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442E8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0442E8: move.b $106ece.l, d0
pub fn i_0442e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442EE;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0442EE: bpl.w $44302
pub fn i_0442ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442F2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x44302; return 10; }
    8
}
/// $0442F2: jsr $5b6.l
pub fn i_0442f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x442F8);
    cpu.pc = 0x5B6;
    16
}
/// $0442F8: lea.l $400.l, a1
pub fn i_0442f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x442FE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $0442FE: move.l a1, (a6)
pub fn i_0442fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44300;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $044300: rts
pub fn i_044300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44302;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $044302: add.w d0, d0
pub fn i_044302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44304;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044304: add.w d0, d0
pub fn i_044304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44306;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044306: lea.l $44266(pc), a0
pub fn i_044306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4430A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x44266;
    16
}
/// $04430A: move.l (a0, d0.w), d0
pub fn i_04430a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4430E;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $04430E: cmpi.l #$f1cd4, d0
pub fn i_04430e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44314;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xF1CD4, Size::Long, false);
    8
}
/// $044314: bne.w $44328
pub fn i_044314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44318;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44328; return 10; }
    8
}
/// $044318: tst.b $10fd83.l
pub fn i_044318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4431E;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD83) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $04431E: beq.w $44328
pub fn i_04431e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44322;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44328; return 10; }
    8
}
/// $044322: move.l #$f260e, d0
pub fn i_044322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44328;
    cpu.ir = 0x203C;
    let v1: u32 = 0xF260E;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $044328: move.l d0, $70(a6)
pub fn i_044328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4432C;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04432C: move.l a6, -(a7)
pub fn i_04432c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4432E;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $04432E: lea.l $1008a0.l, a6
pub fn i_04432e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44334;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x1008A0;
    20
}
/// $044334: jsr $8c85c.l
pub fn i_044334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4433A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4433A);
    cpu.pc = 0x8C85C;
    16
}
/// $04433A: lea.l $8c864.l, a1
pub fn i_04433a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44340;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x8C864;
    20
}
/// $044340: jsr $4ae.l
pub fn i_044340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44346);
    cpu.pc = 0x4AE;
    16
}
/// $044346: lea.l $79298.l, a1
pub fn i_044346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4434C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x79298;
    20
}
/// $04434C: jsr $4ae.l
pub fn i_04434c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44352);
    cpu.pc = 0x4AE;
    16
}
/// $044352: movea.l (a7)+, a6
pub fn i_044352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44354;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $044354: move.b #$0, $74(a6)
pub fn i_044354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4435A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04435A: clr.w $76(a6)
pub fn i_04435a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4435E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04435E: clr.w $7a(a6)
pub fn i_04435e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44362;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $044362: lea.l $44368(pc), a0
pub fn i_044362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44366;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x44368;
    16
}
/// $044366: move.l a0, (a6)
pub fn i_044366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44368;
    cpu.ir = 0x2C88;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== MissionDriver_Loop_044368  ($044368, 88 B, mission/event_vm_04422a.s)
/// $044368: movea.l $70(a6), a1
pub fn i_044368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4436C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04436C: cmpi.b #$0, $74(a6)
pub fn i_04436c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44372;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $044372: bne.w $44396
pub fn i_044372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44376;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44396; return 10; }
    8
}
/// $044376: move.w $2(a1), d0
pub fn i_044376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4437A;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04437A: cmpi.w #$ffff, d0
pub fn i_04437a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4437E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $04437E: beq.w $44390
pub fn i_04437e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44382;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44390; return 10; }
    8
}
/// $044382: move.w d0, $7a(a6)
pub fn i_044382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44386;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044386: cmp.w $106f5c.l, d0
pub fn i_044386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4438C;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106F5C) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $04438C: bls.w $443b6
pub fn i_04438c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44390;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x443B6; return 10; }
    8
}
/// $044390: rts
pub fn i_044390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44392;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $044396: move.w $76(a6), d0
pub fn i_044396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4439A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04439A: beq.w $443a8
pub fn i_04439a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4439E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x443A8; return 10; }
    8
}
/// $04439E: subq.w #$1, d0
pub fn i_04439e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443A0;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0443A0: move.w d0, $76(a6)
pub fn i_0443a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443A4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0443A4: bra.w $443ac
pub fn i_0443a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x443AC;
    10
}
/// $0443A8: move.w $2(a1), d0
pub fn i_0443a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443AC;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0443AC: move.w d0, $76(a6)
pub fn i_0443ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443B0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0443B0: beq.w $443b6
pub fn i_0443b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x443B6; return 10; }
    8
}
/// $0443B4: rts
pub fn i_0443b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0443B6: bsr.w $443c0
pub fn i_0443b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443BA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x443BA);
    cpu.pc = 0x443C0;
    18
}
/// $0443BA: move.l a1, $70(a6)
pub fn i_0443ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443BE;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0443BE: bra.b $4436c
pub fn i_0443be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443C0;
    cpu.ir = 0x60AC;
    cpu.pc = 0x4436C;
    10
}
// ===== MissionVM_ExecOp_0443C0  ($0443C0, 96 B, mission/event_vm_04422a.s)
/// $0443C0: moveq #$0, d0
pub fn i_0443c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443C2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0443C2: move.b (a1), d0
pub fn i_0443c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443C4;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0443C4: cmpi.b #$c, d0
pub fn i_0443c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443C8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $0443C8: bls.w $443d8
pub fn i_0443c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443CC;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x443D8; return 10; }
    8
}
/// $0443CC: nop
pub fn i_0443cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443CE;
    cpu.ir = 0x4E71;
    4
}
/// $0443CE: nop
pub fn i_0443ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443D0;
    cpu.ir = 0x4E71;
    4
}
/// $0443D0: cmpi.b #$c, d0
pub fn i_0443d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443D4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $0443D4: nop
pub fn i_0443d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443D6;
    cpu.ir = 0x4E71;
    4
}
/// $0443D6: trap #$f
pub fn i_0443d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443D8;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0443D8: add.w d0, d0
pub fn i_0443d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443DA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0443DA: add.w d0, d0
pub fn i_0443da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443DC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0443DC: lea.l $443e4(pc), a0
pub fn i_0443dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443E0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x443E4;
    16
}
/// $0443E0: jmp (a0, d0.w)
pub fn i_0443e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443E4;
    cpu.ir = 0x4EF0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $0443E4: bra.w $446a8
pub fn i_0443e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x446A8;
    10
}
/// $0443E8: bra.w $4469e
pub fn i_0443e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x4469E;
    10
}
/// $0443EC: bra.w $4460a
pub fn i_0443ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4460A;
    10
}
/// $0443F0: bra.w $44638
pub fn i_0443f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x44638;
    10
}
/// $0443F4: bra.w $4446c
pub fn i_0443f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x443F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x4446C;
    10
}
// ===== MissionOp04_PlayersPastY_04446C  ($04446C, 124 B, mission/event_vm_04422a.s)
/// $04446C: clr.w d0
pub fn i_04446c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4446E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04446E: clr.w d1
pub fn i_04446e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44470;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $044470: clr.w d2
pub fn i_044470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44472;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $044472: clr.w d3
pub fn i_044472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44474;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $044474: cmpi.b #$1, $10fdb6.l
pub fn i_044474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4447C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB6) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $04447C: bne.w $4448e
pub fn i_04447c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44480;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4448E; return 10; }
    8
}
/// $044480: lea.l $100440.l, a0
pub fn i_044480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44486;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $044486: move.w $22(a0), d0
pub fn i_044486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4448A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04448A: move.w $24(a0), d1
pub fn i_04448a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4448E;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04448E: cmpi.b #$1, $10fdb7.l
pub fn i_04448e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44496;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB7) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $044496: bne.w $444a8
pub fn i_044496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4449A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x444A8; return 10; }
    8
}
/// $04449A: lea.l $1004e0.l, a0
pub fn i_04449a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $0444A0: move.w $22(a0), d2
pub fn i_0444a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444A4;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0444A4: move.w $24(a0), d3
pub fn i_0444a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444A8;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0444A8: cmp.w d2, d0
pub fn i_0444a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444AA;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0444AA: bge.w $444b0
pub fn i_0444aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444AE;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x444B0; return 10; }
    8
}
/// $0444AE: exg.l d1, d3
pub fn i_0444ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444B0;
    cpu.ir = 0xC343;
    cpu.d.swap(1, 3);
    6
}
/// $0444B0: subi.w #$200, d1
pub fn i_0444b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444B4;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0444B4: neg.w d1
pub fn i_0444b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444B6;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0444B6: cmpi.b #$0, $1(a1)
pub fn i_0444b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444BC;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0444BC: bne.w $444c6
pub fn i_0444bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444C0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x444C6; return 10; }
    8
}
/// $0444C0: add.w $106f54.l, d1
pub fn i_0444c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444C6;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0444C6: cmp.w $4(a1), d1
pub fn i_0444c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444CA;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0444CA: lea.l $6(a1), a1
pub fn i_0444ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444CE;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $0444CE: bpl.w $444de
pub fn i_0444ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444D2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x444DE; return 10; }
    8
}
/// $0444D2: bsr.w $443c0
pub fn i_0444d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444D6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x444D6);
    cpu.pc = 0x443C0;
    18
}
/// $0444D6: bsr.w $446b6
pub fn i_0444d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444DA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x444DA);
    cpu.pc = 0x446B6;
    18
}
/// $0444DA: bra.w $444e6
pub fn i_0444da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x444E6;
    10
}
/// $0444DE: bsr.w $446b6
pub fn i_0444de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444E2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x444E2);
    cpu.pc = 0x446B6;
    18
}
/// $0444E2: bsr.w $443c0
pub fn i_0444e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444E6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x444E6);
    cpu.pc = 0x443C0;
    18
}
/// $0444E6: rts
pub fn i_0444e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x444E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp02_SkipTo0D_04460A  ($04460A, 46 B, mission/event_vm_04422a.s)
/// $04460A: lea.l $4(a1), a1
pub fn i_04460a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4460E;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $04460E: move.b (a1), d0
pub fn i_04460e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44610;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $044610: cmpi.b #$c, d0
pub fn i_044610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44614;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044614: bne.w $44624
pub fn i_044614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44618;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44624; return 10; }
    8
}
/// $044618: nop
pub fn i_044618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4461A;
    cpu.ir = 0x4E71;
    4
}
/// $04461A: nop
pub fn i_04461a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4461C;
    cpu.ir = 0x4E71;
    4
}
/// $04461C: cmpi.b #$c, d0
pub fn i_04461c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44620;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044620: nop
pub fn i_044620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44622;
    cpu.ir = 0x4E71;
    4
}
/// $044622: trap #$f
pub fn i_044622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44624;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $044624: cmpi.b #$d, d0
pub fn i_044624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44628;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xD, Size::Byte, false);
    8
}
/// $044628: beq.w $44632
pub fn i_044628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4462C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44632; return 10; }
    8
}
/// $04462C: bsr.w $443c0
pub fn i_04462c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44630;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44630);
    cpu.pc = 0x443C0;
    18
}
/// $044630: bra.b $4460e
pub fn i_044630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44632;
    cpu.ir = 0x60DC;
    cpu.pc = 0x4460E;
    10
}
/// $044632: lea.l $2(a1), a1
pub fn i_044632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44636;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.a[1] = ad1;
    16
}
/// $044636: rts
pub fn i_044636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44638;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp03_CondSelect_044638  ($044638, 102 B, mission/event_vm_04422a.s)
/// $044638: moveq #$0, d7
pub fn i_044638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4463A;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04463A: lea.l $4(a1), a1
pub fn i_04463a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4463E;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $04463E: move.b (a1), d0
pub fn i_04463e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44640;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $044640: cmpi.b #$c, d0
pub fn i_044640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44644;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044644: bne.w $44654
pub fn i_044644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44648;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44654; return 10; }
    8
}
/// $044648: nop
pub fn i_044648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4464A;
    cpu.ir = 0x4E71;
    4
}
/// $04464A: nop
pub fn i_04464a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4464C;
    cpu.ir = 0x4E71;
    4
}
/// $04464C: cmpi.b #$c, d0
pub fn i_04464c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44650;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044650: nop
pub fn i_044650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44652;
    cpu.ir = 0x4E71;
    4
}
/// $044652: trap #$f
pub fn i_044652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44654;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $044654: cmpi.b #$e, d0
pub fn i_044654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44658;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $044658: beq.w $44666
pub fn i_044658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4465C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44666; return 10; }
    8
}
/// $04465C: move.l a1, -(a7)
pub fn i_04465c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4465E;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $04465E: addq.w #$1, d7
pub fn i_04465e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44660;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $044660: bsr.w $446b6
pub fn i_044660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44664;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44664);
    cpu.pc = 0x446B6;
    18
}
/// $044664: bra.b $4463e
pub fn i_044664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44666;
    cpu.ir = 0x60D8;
    cpu.pc = 0x4463E;
    10
}
/// $044666: lea.l $2(a1), a2
pub fn i_044666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4466A;
    cpu.ir = 0x45E9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.a[2] = ad1;
    16
}
/// $04466A: move.w d7, d0
pub fn i_04466a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4466C;
    cpu.ir = 0x3007;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04466C: beq.w $4469a
pub fn i_04466c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44670;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4469A; return 10; }
    8
}
/// $044670: move.l a4, -(a7)
pub fn i_044670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44672;
    cpu.ir = 0x2F0C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $044672: movem.w d6-d7, -(a7)
pub fn i_044672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44676;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[7] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[6] as u16);
    cpu.a[7] = a;
    20
}
/// $044676: jsr $5e9e4.l
pub fn i_044676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4467C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4467C);
    cpu.pc = 0x5E9E4;
    16
}
/// $04467C: movem.w (a7)+, d6-d7
pub fn i_04467c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44680;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[7] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $044680: movea.l (a7)+, a4
pub fn i_044680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44682;
    cpu.ir = 0x285F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[4] = v3;
    12
}
/// $044682: add.w d0, d0
pub fn i_044682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44684;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044684: add.w d0, d0
pub fn i_044684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44686;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044686: add.w d7, d7
pub fn i_044686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44688;
    cpu.ir = 0xDE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $044688: add.w d7, d7
pub fn i_044688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4468A;
    cpu.ir = 0xDE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04468A: movea.l (a7, d0.w), a1
pub fn i_04468a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4468E;
    cpu.ir = 0x2277;
    let ad2 = cpu.a[7].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $04468E: lea.l (a7, d7.w), a7
pub fn i_04468e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44692;
    cpu.ir = 0x4FF7;
    let ad1 = cpu.a[7].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[7] = ad1;
    18
}
/// $044692: move.l a2, -(a7)
pub fn i_044692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44694;
    cpu.ir = 0x2F0A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $044694: bsr.w $443c0
pub fn i_044694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44698;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44698);
    cpu.pc = 0x443C0;
    18
}
/// $044698: movea.l (a7)+, a2
pub fn i_044698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4469A;
    cpu.ir = 0x245F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    12
}
/// $04469A: movea.l a2, a1
pub fn i_04469a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4469C;
    cpu.ir = 0x224A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $04469C: rts
pub fn i_04469c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4469E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp01_PeriodicSpawn_04469E  ($04469E, 10 B, mission/event_vm_04422a.s)
/// $04469E: bsr.w $447c6
pub fn i_04469e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446A2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x446A2);
    cpu.pc = 0x447C6;
    18
}
/// $0446A2: bsr.w $446b6
pub fn i_0446a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446A6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x446A6);
    cpu.pc = 0x446B6;
    18
}
/// $0446A6: rts
pub fn i_0446a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionOp00_SpawnDirect_0446A8  ($0446A8, 14 B, mission/event_vm_04422a.s)
/// $0446A8: move.l a1, -(a7)
pub fn i_0446a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446AA;
    cpu.ir = 0x2F09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0446AA: bsr.w $448a6
pub fn i_0446aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446AE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x446AE);
    cpu.pc = 0x448A6;
    18
}
/// $0446AE: movea.l (a7)+, a1
pub fn i_0446ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446B0;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $0446B0: lea.l $12(a1), a1
pub fn i_0446b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446B4;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x12);
    cpu.a[1] = ad1;
    16
}
/// $0446B4: rts
pub fn i_0446b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionVM_SkipOp_0446B6  ($0446B6, 88 B, mission/event_vm_04422a.s)
/// $0446B6: moveq #$0, d0
pub fn i_0446b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446B8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0446B8: move.b (a1), d0
pub fn i_0446b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446BA;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0446BA: cmpi.b #$c, d0
pub fn i_0446ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446BE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $0446BE: bls.w $446ce
pub fn i_0446be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446C2;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x446CE; return 10; }
    8
}
/// $0446C2: nop
pub fn i_0446c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446C4;
    cpu.ir = 0x4E71;
    4
}
/// $0446C4: nop
pub fn i_0446c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446C6;
    cpu.ir = 0x4E71;
    4
}
/// $0446C6: cmpi.b #$c, d0
pub fn i_0446c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446CA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $0446CA: nop
pub fn i_0446ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446CC;
    cpu.ir = 0x4E71;
    4
}
/// $0446CC: trap #$f
pub fn i_0446cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446CE;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0446CE: add.w d0, d0
pub fn i_0446ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446D0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0446D0: add.w d0, d0
pub fn i_0446d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446D2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0446D2: lea.l $446da(pc), a0
pub fn i_0446d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446D6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x446DA;
    16
}
/// $0446D6: jmp (a0, d0.w)
pub fn i_0446d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446DA;
    cpu.ir = 0x4EF0;
    let ad1 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $0446DA: bra.w $4472c
pub fn i_0446da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x4472C;
    10
}
/// $0446E2: bra.w $4473c
pub fn i_0446e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x4473C;
    10
}
/// $0446E6: bra.w $4476a
pub fn i_0446e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4476A;
    10
}
/// $0446EA: bra.w $44798
pub fn i_0446ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x446EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x44798;
    10
}
// ===== MissionSkip_Op00_04472C  ($04472C, 6 B, mission/event_vm_04422a.s)
/// $04472C: lea.l $12(a1), a1
pub fn i_04472c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44730;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x12);
    cpu.a[1] = ad1;
    16
}
/// $044730: rts
pub fn i_044730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44732;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionSkip_Op02_04473C  ($04473C, 46 B, mission/event_vm_04422a.s)
/// $04473C: lea.l $4(a1), a1
pub fn i_04473c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44740;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $044740: move.b (a1), d0
pub fn i_044740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44742;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $044742: cmpi.b #$c, d0
pub fn i_044742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44746;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044746: bne.w $44756
pub fn i_044746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4474A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44756; return 10; }
    8
}
/// $04474A: nop
pub fn i_04474a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4474C;
    cpu.ir = 0x4E71;
    4
}
/// $04474C: nop
pub fn i_04474c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4474E;
    cpu.ir = 0x4E71;
    4
}
/// $04474E: cmpi.b #$c, d0
pub fn i_04474e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44752;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044752: nop
pub fn i_044752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44754;
    cpu.ir = 0x4E71;
    4
}
/// $044754: trap #$f
pub fn i_044754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44756;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $044756: cmpi.b #$d, d0
pub fn i_044756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4475A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xD, Size::Byte, false);
    8
}
/// $04475A: beq.w $44764
pub fn i_04475a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4475E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44764; return 10; }
    8
}
/// $04475E: bsr.w $446b6
pub fn i_04475e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44762;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44762);
    cpu.pc = 0x446B6;
    18
}
/// $044762: bra.b $44740
pub fn i_044762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44764;
    cpu.ir = 0x60DC;
    cpu.pc = 0x44740;
    10
}
/// $044764: lea.l $2(a1), a1
pub fn i_044764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44768;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.a[1] = ad1;
    16
}
/// $044768: rts
pub fn i_044768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4476A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionSkip_Op03_04476A  ($04476A, 46 B, mission/event_vm_04422a.s)
/// $04476A: lea.l $4(a1), a1
pub fn i_04476a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4476E;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x4);
    cpu.a[1] = ad1;
    16
}
/// $04476E: move.b (a1), d0
pub fn i_04476e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44770;
    cpu.ir = 0x1011;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $044770: cmpi.b #$c, d0
pub fn i_044770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44774;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044774: bne.w $44784
pub fn i_044774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44778;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44784; return 10; }
    8
}
/// $044778: nop
pub fn i_044778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4477A;
    cpu.ir = 0x4E71;
    4
}
/// $04477A: nop
pub fn i_04477a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4477C;
    cpu.ir = 0x4E71;
    4
}
/// $04477C: cmpi.b #$c, d0
pub fn i_04477c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44780;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xC, Size::Byte, false);
    8
}
/// $044780: nop
pub fn i_044780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44782;
    cpu.ir = 0x4E71;
    4
}
/// $044782: trap #$f
pub fn i_044782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44784;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $044784: cmpi.b #$e, d0
pub fn i_044784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44788;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $044788: beq.w $44792
pub fn i_044788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4478C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44792; return 10; }
    8
}
/// $04478C: bsr.w $446b6
pub fn i_04478c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44790;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x44790);
    cpu.pc = 0x446B6;
    18
}
/// $044790: bra.b $4476e
pub fn i_044790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44792;
    cpu.ir = 0x60DC;
    cpu.pc = 0x4476E;
    10
}
/// $044792: lea.l $2(a1), a1
pub fn i_044792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44796;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x2);
    cpu.a[1] = ad1;
    16
}
/// $044796: rts
pub fn i_044796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44798;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MissionSkip_Op04_044798  ($044798, 14 B, mission/event_vm_04422a.s)
/// $044798: lea.l $6(a1), a1
pub fn i_044798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4479C;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0x6);
    cpu.a[1] = ad1;
    16
}
/// $04479C: bsr.w $446b6
pub fn i_04479c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447A0;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x447A0);
    cpu.pc = 0x446B6;
    18
}
/// $0447A0: bsr.w $446b6
pub fn i_0447a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447A4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x447A4);
    cpu.pc = 0x446B6;
    18
}
/// $0447A4: rts
pub fn i_0447a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PeriodicSpawner_Setup_0447C6  ($0447C6, 100 B, mission/event_vm_04422a.s)
/// $0447C6: movem.l a1, -(a7)
pub fn i_0447c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447CA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0447CA: lea.l $4482a(pc), a1
pub fn i_0447ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4482A;
    16
}
/// $0447CE: jsr $4ae.l
pub fn i_0447ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x447D4);
    cpu.pc = 0x4AE;
    16
}
/// $0447D4: movem.l (a7)+, a1
pub fn i_0447d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447D8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0447D8: move.b $5(a1), d0
pub fn i_0447d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447DC;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0447DC: andi.w #$ff, d0
pub fn i_0447dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447E0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0447E0: tst.b $8(a1)
pub fn i_0447e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447E4;
    cpu.ir = 0x4A29;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0447E4: beq.w $447ea
pub fn i_0447e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x447EA; return 10; }
    8
}
/// $0447E8: lsl.w #$5, d0
pub fn i_0447e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447EA;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0447EA: move.w d0, $80(a0)
pub fn i_0447ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447EE;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0447EE: move.b $1(a1), $7c(a0)
pub fn i_0447ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447F4;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x7C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0447F4: moveq #$0, d0
pub fn i_0447f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447F6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0447F6: move.b $6(a1), d0
pub fn i_0447f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447FA;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0447FA: tst.b $9(a1)
pub fn i_0447fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x447FE;
    cpu.ir = 0x4A29;
    let ad2 = cpu.a[1].wrapping_add(0x9);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0447FE: beq.w $44804
pub fn i_0447fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44802;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44804; return 10; }
    8
}
/// $044802: lsl.w #$5, d0
pub fn i_044802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44804;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044804: move.w d0, $7e(a0)
pub fn i_044804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44808;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044808: move.b $7(a1), $84(a0)
pub fn i_044808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4480E;
    cpu.ir = 0x1169;
    let ad2 = cpu.a[1].wrapping_add(0x7);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x84);
    bus.write8(ad4, v3 as u8);
    20
}
/// $04480E: move.b $4(a1), d0
pub fn i_04480e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44812;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $044812: andi.w #$ff, d0
pub fn i_044812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44816;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $044816: lsl.w #$4, d0
pub fn i_044816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44818;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $044818: add.w $7a(a6), d0
pub fn i_044818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4481C;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04481C: move.w d0, $82(a0)
pub fn i_04481c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44820;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044820: lea.l $a(a1), a1
pub fn i_044820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44824;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0xA);
    cpu.a[1] = ad1;
    16
}
/// $044824: move.l a1, $70(a0)
pub fn i_044824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44828;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $044828: rts
pub fn i_044828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4482A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PeriodicSpawner_Task_04482A  ($04482A, 24 B, mission/event_vm_04422a.s)
/// $04482A: tst.b $106ed3.l
pub fn i_04482a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44830;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED3) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $044830: beq.w $44842
pub fn i_044830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44834;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44842; return 10; }
    8
}
/// $044834: move.w $106f5c.l, d0
pub fn i_044834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4483A;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04483A: cmp.w $82(a6), d0
pub fn i_04483a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4483E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $04483E: bcs.w $44850
pub fn i_04483e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44842;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x44850; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_044842  ($044842, 14 B, kernel/jsr5b6_jmp518.c)
/// $044842: jsr $5b6.l
pub fn i_044842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44848;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44848);
    cpu.pc = 0x5B6;
    16
}
/// $044848: jmp $518.l
pub fn i_044848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4484E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== PeriodicSpawner_Tick_044850  ($044850, 10 B, mission/event_vm_04422a.s)
/// $044850: move.w $80(a6), d0
pub fn i_044850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44854;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $044854: beq.w $44860
pub fn i_044854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44858;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44860; return 10; }
    8
}
/// $044858: subq.w #$1, d0
pub fn i_044858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4485A;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_04485a  ($04485A, 6 B, kernel/task_setters_w.c)
/// $04485A: move.w d0, $80(a6)
pub fn i_04485a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4485E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04485E: rts
pub fn i_04485e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44860;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PeriodicSpawner_Fire_044860  ($044860, 54 B, mission/event_vm_04422a.s)
/// $044860: clr.w d0
pub fn i_044860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44862;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $044862: move.b $84(a6), d0
pub fn i_044862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44866;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $044866: cmpi.b #$ff, d0
pub fn i_044866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4486A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $04486A: beq.b $44876
pub fn i_04486a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4486C;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44876; return 10; }
    8
}
/// $04486C: cmp.w $106e88.l, d0
pub fn i_04486c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44872;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106E88) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $044872: bcs.w $448a4
pub fn i_044872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44876;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x448A4; return 10; }
    8
}
/// $044876: movea.l $70(a6), a1
pub fn i_044876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4487A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04487A: bsr.w $443c0
pub fn i_04487a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4487E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4487E);
    cpu.pc = 0x443C0;
    18
}
/// $04487E: move.w $7e(a6), $80(a6)
pub fn i_04487e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44884;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $044884: move.b $7c(a6), d0
pub fn i_044884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44888;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $044888: beq.w $448a4
pub fn i_044888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4488C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x448A4; return 10; }
    8
}
/// $04488C: subq.b #$1, d0
pub fn i_04488c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4488E;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04488E: move.b d0, $7c(a6)
pub fn i_04488e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44892;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $044892: bne.w $448a4
pub fn i_044892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44896;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x448A4; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_044896  ($044896, 14 B, kernel/jsr5b6_jmp518.c)
/// $044896: jsr $5b6.l
pub fn i_044896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4489C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4489C);
    cpu.pc = 0x5B6;
    16
}
/// $04489C: jmp $518.l
pub fn i_04489c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Stub_000448A4  ($0448A4, 2 B, thunks/trivial_rts.c)
/// $0448A4: rts
pub fn i_0448a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_FromStream_0448A6  ($0448A6, 232 B, mission/spawn_boss_0448a6.s)
/// $0448A6: cmpi.b #$0, (a1)
pub fn i_0448a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448AA;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    12
}
/// $0448AA: beq.w $448ba
pub fn i_0448aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x448BA; return 10; }
    8
}
/// $0448AE: nop
pub fn i_0448ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448B0;
    cpu.ir = 0x4E71;
    4
}
/// $0448B0: nop
pub fn i_0448b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448B2;
    cpu.ir = 0x4E71;
    4
}
/// $0448B2: cmpi.b #$0, (a1)
pub fn i_0448b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448B6;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    12
}
/// $0448B6: nop
pub fn i_0448b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448B8;
    cpu.ir = 0x4E71;
    4
}
/// $0448B8: trap #$f
pub fn i_0448b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448BA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0448BA: move.b $1(a1), d7
pub fn i_0448ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448BE;
    cpu.ir = 0x1E29;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0448BE: andi.b #$3, d7
pub fn i_0448be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448C2;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0448C2: beq.w $448e6
pub fn i_0448c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x448E6; return 10; }
    8
}
/// $0448C6: cmpi.b #$3, d7
pub fn i_0448c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448CA;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $0448CA: bne.w $448dc
pub fn i_0448ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x448DC; return 10; }
    8
}
/// $0448CE: jsr $2ac6a.l
pub fn i_0448ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x448D4);
    cpu.pc = 0x2AC6A;
    16
}
/// $0448D4: bcc.w $4498c
pub fn i_0448d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4498C; return 10; }
    8
}
/// $0448D8: bra.w $448e6
pub fn i_0448d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x448E6;
    10
}
/// $0448DC: cmp.b $106ed1.l, d7
pub fn i_0448dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448E2;
    cpu.ir = 0xBE39;
    let v2 = bus.read8(0x106ED1) as u32;
    let h1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    16
}
/// $0448E2: bne.w $4498c
pub fn i_0448e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4498C; return 10; }
    8
}
/// $0448E6: moveq #$0, d0
pub fn i_0448e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448E8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0448E8: move.w $4(a1), d0
pub fn i_0448e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448EC;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0448EC: add.w d0, d0
pub fn i_0448ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448EE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0448EE: add.w d0, d0
pub fn i_0448ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448F0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0448F0: lea.l $e8000.l, a0
pub fn i_0448f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xE8000;
    20
}
/// $0448F6: movem.l a1/a6, -(a7)
pub fn i_0448f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448FA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0448FA: movea.l (a0, d0.w), a1
pub fn i_0448fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x448FE;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $0448FE: lea.l $100800.l, a6
pub fn i_0448fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44904;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $044904: jsr $4ae.l
pub fn i_044904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4490A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4490A);
    cpu.pc = 0x4AE;
    16
}
/// $04490A: movem.l (a7)+, a1/a6
pub fn i_04490a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4490E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $04490E: move.b $1(a1), d7
pub fn i_04490e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44912;
    cpu.ir = 0x1E29;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v3 & 0xFF);
    12
}
/// $044912: btst.b #$4, d7
pub fn i_044912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44916;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $044916: bne.w $44920
pub fn i_044916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4491A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44920; return 10; }
    8
}
/// $04491A: bset.b #$1, $12(a0)
pub fn i_04491a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44920;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $044920: move.w $6(a1), d0
pub fn i_044920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44924;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $044924: btst.b #$7, d7
pub fn i_044924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44928;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $044928: bne.w $44932
pub fn i_044928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4492C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44932; return 10; }
    8
}
/// $04492C: sub.w $106f50.l, d0
pub fn i_04492c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44932;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044932: move.w d0, $22(a0)
pub fn i_044932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44936;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044936: move.w $8(a1), d0
pub fn i_044936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4493A;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04493A: btst.b #$6, d7
pub fn i_04493a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4493E;
    cpu.ir = 0x807;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04493E: bne.w $44948
pub fn i_04493e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44942;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x44948; return 10; }
    8
}
/// $044942: sub.w $106f54.l, d0
pub fn i_044942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44948;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $044948: neg.w d0
pub fn i_044948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4494A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04494A: addi.w #$200, d0
pub fn i_04494a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4494E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04494E: move.w d0, $24(a0)
pub fn i_04494e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44952;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $044952: cmpa.l #$100800, a6
pub fn i_044952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44958;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100800;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $044958: beq.w $44972
pub fn i_044958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4495C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x44972; return 10; }
    8
}
/// $04495C: btst.b #$1, $12(a0)
pub fn i_04495c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44962;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $044962: beq.w $4496c
pub fn i_044962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44966;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4496C; return 10; }
    8
}
/// $044966: addq.w #$1, $106e8a.l
pub fn i_044966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4496C;
    cpu.ir = 0x5279;
    let v1 = bus.read16(0x106E8A) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x106E8A, r2 as u16);
    16
}
/// $04496C: jsr $4405e.l
pub fn i_04496c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44972;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x44972);
    cpu.pc = 0x4405E;
    16
}
/// $044972: andi.b #$c, d7
pub fn i_044972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44976;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $044976: lsr.w #$2, d7
pub fn i_044976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44978;
    cpu.ir = 0xE44F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $044978: move.b d7, $11(a0)
pub fn i_044978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4497C;
    cpu.ir = 0x1147;
    let v1 = cpu.d[7] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04497C: lea.l $a(a1), a1
pub fn i_04497c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44980;
    cpu.ir = 0x43E9;
    let ad1 = cpu.a[1].wrapping_add(0xA);
    cpu.a[1] = ad1;
    16
}
/// $044980: lea.l $98(a0), a2
pub fn i_044980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44984;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    cpu.a[2] = ad1;
    16
}
/// $044984: moveq #$7, d7
pub fn i_044984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44986;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $044986: move.b (a1)+, (a2)+
pub fn i_044986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44988;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $044988: dbra d7, $44986
pub fn i_044988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4498C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x4498C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x4498C; return 14; }
    cpu.pc = 0x44986;
    10
}
/// $04498C: rts
pub fn i_04498c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4498E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_MarkPending_04498E  ($04498E, 24 B, mission/spawn_boss_0448a6.s)
/// $04498E: move.l a6, -(a7)
pub fn i_04498e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44990;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $044990: lea.l $100800.l, a6
pub fn i_044990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x44996;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $044996: jsr $4ae.l
pub fn i_044996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4499C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4499C);
    cpu.pc = 0x4AE;
    16
}
/// $04499C: bset.b #$1, $12(a0)
pub fn i_04499c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449A2;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0449A2: movea.l (a7)+, a6
pub fn i_0449a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449A4;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $0449A4: rts
pub fn i_0449a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x449A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Ent_SfxByMode_0457A6  ($0457A6, 38 B, input/entity_aim_input_044f8a.s)
/// $0457A6: move.w #$1d6, d1
pub fn i_0457a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0457AA: move.b $6d(a6), d0
pub fn i_0457aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457AE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0457AE: cmpi.b #$1, d0
pub fn i_0457ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457B2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $0457B2: beq.w $457c6
pub fn i_0457b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x457C6; return 10; }
    8
}
/// $0457B6: move.w #$1d6, d1
pub fn i_0457b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457BA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0457BA: cmpi.b #$2, d0
pub fn i_0457ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457BE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $0457BE: beq.w $457c6
pub fn i_0457be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457C2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x457C6; return 10; }
    8
}
/// $0457C2: move.w #$2, d1
pub fn i_0457c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457C6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0457C6: jmp $236e.l
pub fn i_0457c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457CC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x236E;
    8
}
// ===== Ent_AnimFrame_0457CC  ($0457CC, 58 B, input/entity_aim_input_044f8a.s)
/// $0457CC: lea.l $289204.l, a0
pub fn i_0457cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x289204;
    20
}
/// $0457D2: move.b $34(a6), d0
pub fn i_0457d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457D6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0457D6: addq.w #$4, d0
pub fn i_0457d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457D8;
    cpu.ir = 0x5840;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0457D8: andi.w #$f8, d0
pub fn i_0457d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457DC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0457DC: move.w d0, d1
pub fn i_0457dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457DE;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0457DE: add.w d0, d0
pub fn i_0457de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457E0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0457E0: add.w d0, d1
pub fn i_0457e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457E2;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0457E2: move.b $3b(a6), d0
pub fn i_0457e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457E6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0457E6: andi.w #$ff, d0
pub fn i_0457e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457EA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0457EA: add.w d0, d0
pub fn i_0457ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457EC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0457EC: add.w d0, d0
pub fn i_0457ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457EE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0457EE: add.w d1, d0
pub fn i_0457ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457F0;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0457F0: move.l (a0, d0.w), $3c(a6)
pub fn i_0457f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457F6;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $0457F6: jsr $5ca2a.l
pub fn i_0457f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x457FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x457FC);
    cpu.pc = 0x5CA2A;
    16
}
/// $0457FC: move.b $3b(a6), d0
pub fn i_0457fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45800;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045800: beq.w $4580a
pub fn i_045800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45804;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4580A; return 10; }
    8
}
/// $045804: subq.b #$1, d0
pub fn i_045804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45806;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
// ===== SetTaskB_045806  ($045806, 6 B, kernel/task_setters_b.c)
/// $045806: move.b d0, $3b(a6)
pub fn i_045806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4580A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04580A: rts
pub fn i_04580a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4580C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret_FireGroundSnap_04580C  ($04580C, 208 B, enemies/vehicles/turret_boss2_04580c.s)
/// $04580C: movea.l $c(a6), a0
pub fn i_04580c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45810;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $045810: move.b $68(a0), $68(a6)
pub fn i_045810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45816;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $045816: lea.l $45fde(pc), a1
pub fn i_045816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4581A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45FDE;
    16
}
/// $04581A: jsr $6fe.l
pub fn i_04581a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45820;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45820);
    cpu.pc = 0x6FE;
    16
}
/// $045820: jsr $5dd02.l
pub fn i_045820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45826);
    cpu.pc = 0x5DD02;
    16
}
/// $045826: jsr $517fe.l
pub fn i_045826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4582C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4582C);
    cpu.pc = 0x517FE;
    16
}
/// $04582C: move.b $34(a6), d0
pub fn i_04582c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45830;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045830: addq.b #$4, d0
pub fn i_045830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45832;
    cpu.ir = 0x5800;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $045832: move.b d0, $98(a0)
pub fn i_045832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45836;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045836: move.b $80(a6), $9a(a0)
pub fn i_045836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4583C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $04583C: move.b $81(a6), $9b(a0)
pub fn i_04583c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45842;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $045842: addi.b #$1, $81(a6)
pub fn i_045842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45848;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x81);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $045848: movea.l $c(a6), a0
pub fn i_045848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4584C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04584C: bset.b #$2, $8c(a0)
pub fn i_04584c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45852;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $045852: subi.b #$1, $80(a6)
pub fn i_045852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45858;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $045858: move.b $34(a6), d0
pub fn i_045858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4585C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04585C: addi.w #$4, d0
pub fn i_04585c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45860;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $045860: andi.w #$f8, d0
pub fn i_045860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45864;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $045864: lsr.w #$1, d0
pub fn i_045864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45866;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $045866: lea.l $29d752.l, a0
pub fn i_045866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4586C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D752;
    20
}
/// $04586C: move.w $22(a6), d1
pub fn i_04586c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45870;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $045870: move.w $24(a6), d2
pub fn i_045870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45874;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $045874: add.w (a0, d0.w), d1
pub fn i_045874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45878;
    cpu.ir = 0xD270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $045878: add.w $2(a0, d0.w), d2
pub fn i_045878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4587C;
    cpu.ir = 0xD470;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $04587C: jsr $280c6.l
pub fn i_04587c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45882);
    cpu.pc = 0x280C6;
    16
}
/// $045882: cmpi.b #$0, d0
pub fn i_045882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45886;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $045886: beq.w $458d4
pub fn i_045886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4588A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x458D4; return 10; }
    8
}
/// $04588A: cmpi.b #$10, d0
pub fn i_04588a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4588E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $04588E: beq.w $458ae
pub fn i_04588e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45892;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x458AE; return 10; }
    8
}
/// $045892: cmpi.b #$20, d0
pub fn i_045892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45896;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $045896: beq.w $458ae
pub fn i_045896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4589A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x458AE; return 10; }
    8
}
/// $04589A: cmpi.b #$21, d0
pub fn i_04589a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4589E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x21, Size::Byte, false);
    8
}
/// $04589E: beq.w $458ae
pub fn i_04589e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x458AE; return 10; }
    8
}
/// $0458A2: andi.b #$f0, d0
pub fn i_0458a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458A6;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0458A6: cmpi.b #$0, d0
pub fn i_0458a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458AA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0458AA: bne.w $458d4
pub fn i_0458aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x458D4; return 10; }
    8
}
/// $0458AE: lea.l $30c14.l, a1
pub fn i_0458ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458B4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x30C14;
    20
}
/// $0458B4: jsr $4ae.l
pub fn i_0458b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x458BA);
    cpu.pc = 0x4AE;
    16
}
/// $0458BA: jsr $5dd02.l
pub fn i_0458ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x458C0);
    cpu.pc = 0x5DD02;
    16
}
/// $0458C0: move.b $34(a6), d0
pub fn i_0458c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458C4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0458C4: addi.w #$4, d0
pub fn i_0458c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458C8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0458C8: andi.w #$f8, d0
pub fn i_0458c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458CC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0458CC: lsr.w #$3, d0
pub fn i_0458cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458CE;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0458CE: move.b d0, $98(a0)
pub fn i_0458ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458D2;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0458D2: rts
pub fn i_0458d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0458D4: move.b #$5, $3b(a6)
pub fn i_0458d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0458DA: rts
pub fn i_0458da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret_FollowParent_0458DC  ($0458DC, 94 B, enemies/vehicles/turret_boss2_04580c.s)
/// $0458DC: movea.l $c(a6), a0
pub fn i_0458dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458E0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0458E0: move.b $59(a0), $59(a6)
pub fn i_0458e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458E6;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0458E6: move.w $38(a0), d0
pub fn i_0458e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458EA;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0458EA: move.w $22(a0), d1
pub fn i_0458ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458EE;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0458EE: move.w $24(a0), d2
pub fn i_0458ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458F2;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0458F2: movea.l $70(a0), a3
pub fn i_0458f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458F6;
    cpu.ir = 0x2668;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $0458F6: add.w $4(a3), d1
pub fn i_0458f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458FA;
    cpu.ir = 0xD26B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0458FA: add.w $6(a3), d2
pub fn i_0458fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x458FE;
    cpu.ir = 0xD46B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0458FE: movea.l (a3), a3
pub fn i_0458fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45900;
    cpu.ir = 0x2653;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[3] = v2;
    12
}
/// $045900: add.w (a3), d1
pub fn i_045900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45902;
    cpu.ir = 0xD253;
    let v2 = bus.read16(cpu.a[3]) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $045902: add.w $2(a3), d2
pub fn i_045902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45906;
    cpu.ir = 0xD46B;
    let ad2 = cpu.a[3].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $045906: ori.w #$1, d0
pub fn i_045906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4590A;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04590A: move.w d0, $38(a6)
pub fn i_04590a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4590E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04590E: move.w d1, $22(a6)
pub fn i_04590e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45912;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045912: move.w d2, $24(a6)
pub fn i_045912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45916;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045916: move.b $6d(a0), d0
pub fn i_045916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4591A;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04591A: cmp.b $6d(a6), d0
pub fn i_04591a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4591E;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04591E: beq.w $45938
pub fn i_04591e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45922;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45938; return 10; }
    8
}
/// $045922: move.b d0, $6d(a6)
pub fn i_045922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45926;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045926: move.b $68(a0), d0
pub fn i_045926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4592A;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04592A: move.b d0, $68(a6)
pub fn i_04592a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4592E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04592E: jsr $13600.l
pub fn i_04592e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45934;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45934);
    cpu.pc = 0x13600;
    16
}
/// $045934: bsr.w $457a6
pub fn i_045934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45938;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45938);
    cpu.pc = 0x457A6;
    18
}
/// $045938: rts
pub fn i_045938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4593A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret_ReadPlayerInput_04593A  ($04593A, 110 B, enemies/vehicles/turret_boss2_04580c.s)
/// $04593A: move.b $6d(a6), d0
pub fn i_04593a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4593E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04593E: bpl.w $4594e
pub fn i_04593e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45942;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x4594E; return 10; }
    8
}
/// $045942: clr.b $71(a6)
pub fn i_045942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45946;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x71);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $045946: clr.b $7c(a6)
pub fn i_045946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4594A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04594A: bra.w $459a6
pub fn i_04594a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4594E;
    cpu.ir = 0x6000;
    cpu.pc = 0x459A6;
    10
}
/// $04594E: cmpi.b #$1, d0
pub fn i_04594e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45952;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $045952: bne.w $45960
pub fn i_045952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45956;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45960; return 10; }
    8
}
/// $045956: lea.l $100440.l, a1
pub fn i_045956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4595C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $04595C: bra.w $45966
pub fn i_04595c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45960;
    cpu.ir = 0x6000;
    cpu.pc = 0x45966;
    10
}
/// $045960: lea.l $1004e0.l, a1
pub fn i_045960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45966;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $045966: movea.l $c(a1), a1
pub fn i_045966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4596A;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04596A: movea.l $72(a1), a2
pub fn i_04596a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4596E;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $04596E: jsr $5d674.l
pub fn i_04596e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45974;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45974);
    cpu.pc = 0x5D674;
    16
}
/// $045974: move.b $2(a2), d0
pub fn i_045974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45978;
    cpu.ir = 0x102A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045978: andi.b #$f, d0
pub fn i_045978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4597C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04597C: move.b d0, $71(a6)
pub fn i_04597c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45980;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045980: move.b $3(a2), d0
pub fn i_045980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45984;
    cpu.ir = 0x102A;
    let ad2 = cpu.a[2].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045984: andi.b #$10, d0
pub fn i_045984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45988;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $045988: beq.w $459a6
pub fn i_045988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4598C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x459A6; return 10; }
    8
}
/// $04598C: move.w #$108c, d0
pub fn i_04598c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45990;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045990: jsr $2352.l
pub fn i_045990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45996);
    cpu.pc = 0x2352;
    16
}
/// $045996: tst.b $7c(a6)
pub fn i_045996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4599A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04599A: sne.b d0
pub fn i_04599a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4599C;
    cpu.ir = 0x56C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $04599C: move.b #$3, $7c(a6)
pub fn i_04599c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459A2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0459A2: and.b d0, $7d(a6)
pub fn i_0459a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459A6;
    cpu.ir = 0xC12E;
    let ad1 = cpu.a[6].wrapping_add(0x7D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0459A6: rts
pub fn i_0459a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret_InitDir3_0459F0  ($0459F0, 26 B, enemies/vehicles/turret_boss2_04580c.s)
/// $0459F0: move.b #$3, $70(a6)
pub fn i_0459f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459F6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0459F6: move.l #$2895c4, $74(a6)
pub fn i_0459f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x459FE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2895C4;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0459FE: move.w #$0, $34(a6)
pub fn i_0459fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A04;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045A04: bra.w $45a0a
pub fn i_045a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A08;
    cpu.ir = 0x6000;
    cpu.pc = 0x45A0A;
    10
}
// ===== Turret_SetupCommon_045A0A  ($045A0A, 46 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045A0A: jsr $13600.l
pub fn i_045a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45A10);
    cpu.pc = 0x13600;
    16
}
/// $045A10: move.w #$2, d1
pub fn i_045a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A14;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045A14: jsr $236e.l
pub fn i_045a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45A1A);
    cpu.pc = 0x236E;
    16
}
/// $045A1A: move.b #$ff, $68(a6)
pub fn i_045a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A20;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045A20: move.b #$ff, $6d(a6)
pub fn i_045a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A26;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045A26: clr.b $81(a6)
pub fn i_045a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A2A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x81);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $045A2A: clr.b $3b(a6)
pub fn i_045a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A2E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $045A2E: clr.w $78(a6)
pub fn i_045a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A32;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $045A32: lea.l $45a38(pc), a1
pub fn i_045a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45A38;
    16
}
/// $045A36: move.l a1, (a6)
pub fn i_045a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Turret_Track_045A38  ($045A38, 134 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045A38: bsr.w $458dc
pub fn i_045a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A3C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45A3C);
    cpu.pc = 0x458DC;
    18
}
/// $045A3C: bsr.w $4593a
pub fn i_045a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A40;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45A40);
    cpu.pc = 0x4593A;
    18
}
/// $045A40: movea.l $74(a6), a0
pub fn i_045a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A44;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $045A44: move.b $71(a6), d0
pub fn i_045a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A48;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045A48: andi.w #$f, d0
pub fn i_045a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A4C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $045A4C: add.w d0, d0
pub fn i_045a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A4E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $045A4E: add.w d0, d0
pub fn i_045a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A50;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $045A50: move.l (a0, d0.w), d0
pub fn i_045a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A54;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $045A54: bmi.w $45a88
pub fn i_045a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A58;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x45A88; return 10; }
    8
}
/// $045A58: sub.w $34(a6), d0
pub fn i_045a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A5C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $045A5C: ext.l d0
pub fn i_045a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A5E;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $045A5E: bpl.w $45a64
pub fn i_045a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A62;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x45A64; return 10; }
    8
}
/// $045A62: neg.w d0
pub fn i_045a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A64;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $045A64: asr.w #$2, d0
pub fn i_045a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A66;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $045A66: move.w $78(a6), d1
pub fn i_045a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A6A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $045A6A: addi.w #$100, d1
pub fn i_045a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A6E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $045A6E: cmp.w d0, d1
pub fn i_045a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A70;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $045A70: bcs.w $45a76
pub fn i_045a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A74;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x45A76; return 10; }
    8
}
/// $045A74: move.w d0, d1
pub fn i_045a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A76;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $045A76: move.w d1, $78(a6)
pub fn i_045a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A7A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $045A7A: swap d0
pub fn i_045a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A7C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $045A7C: eor.w d0, d1
pub fn i_045a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A7E;
    cpu.ir = 0xB141;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $045A7E: sub.w d0, d1
pub fn i_045a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A80;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $045A80: add.w d1, $34(a6)
pub fn i_045a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A84;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $045A84: bra.w $45a8c
pub fn i_045a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A88;
    cpu.ir = 0x6000;
    cpu.pc = 0x45A8C;
    10
}
/// $045A88: clr.w $78(a6)
pub fn i_045a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A8C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $045A8C: move.b $7d(a6), d0
pub fn i_045a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A90;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045A90: beq.w $45a9e
pub fn i_045a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45A9E; return 10; }
    8
}
/// $045A94: subq.b #$1, d0
pub fn i_045a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A96;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $045A96: move.b d0, $7d(a6)
pub fn i_045a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A9A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045A9A: bra.w $45ab6
pub fn i_045a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45A9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x45AB6;
    10
}
/// $045A9E: move.b #$1, $7d(a6)
pub fn i_045a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AA4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045AA4: move.b $7c(a6), d0
pub fn i_045aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AA8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $045AA8: beq.w $45ab6
pub fn i_045aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x45AB6; return 10; }
    8
}
/// $045AAC: subq.b #$1, d0
pub fn i_045aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AAE;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $045AAE: move.b d0, $7c(a6)
pub fn i_045aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AB2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $045AB2: bsr.w $4580c
pub fn i_045ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45AB6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45AB6);
    cpu.pc = 0x4580C;
    18
}
/// $045AB6: bsr.w $457cc
pub fn i_045ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45ABA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x45ABA);
    cpu.pc = 0x457CC;
    18
}
/// $045ABA: bra.w $45b9c
pub fn i_045aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45ABE;
    cpu.ir = 0x6000;
    cpu.pc = 0x45B9C;
    10
}
// ===== Turret_Tail_045B9C  ($045B9C, 118 B, enemies/vehicles/turret_boss2_04580c.s)
/// $045B9C: movea.l $c(a6), a0
pub fn i_045b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BA0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $045BA0: move.b $68(a0), $68(a6)
pub fn i_045ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BA6;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $045BA6: bclr.b #$3, $8c(a6)
pub fn i_045ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BAC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $045BAC: movem.l a6, -(a7)
pub fn i_045bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BB0;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $045BB0: movea.l $c(a6), a6
pub fn i_045bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BB4;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $045BB4: move.b #$8, d1
pub fn i_045bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BB8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $045BB8: jsr $8f714.l
pub fn i_045bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45BBE);
    cpu.pc = 0x8F714;
    16
}
/// $045BBE: movem.l (a7)+, a6
pub fn i_045bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BC2;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $045BC2: bcc.w $45bcc
pub fn i_045bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x45BCC; return 10; }
    8
}
/// $045BC6: lea.l $45c20(pc), a1
pub fn i_045bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45C20;
    16
}
/// $045BCA: move.l a1, (a6)
pub fn i_045bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $045BCC: jsr $5e452.l
pub fn i_045bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45BD2);
    cpu.pc = 0x5E452;
    16
}
/// $045BD2: bcc.w $45be6
pub fn i_045bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x45BE6; return 10; }
    8
}
/// $045BD6: movea.l $c(a6), a0
pub fn i_045bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BDA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $045BDA: btst.b #$0, $13(a0)
pub fn i_045bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BE0;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $045BE0: bne.w $45be6
pub fn i_045be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x45BE6; return 10; }
    8
}
/// $045BE4: rts
pub fn i_045be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $045BE6: move.l a6, -(a7)
pub fn i_045be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BE8;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $045BE8: lea.l $100800.l, a6
pub fn i_045be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BEE;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $045BEE: lea.l $45d3a(pc), a1
pub fn i_045bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x45D3A;
    16
}
/// $045BF2: jsr $4ae.l
pub fn i_045bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45BF8);
    cpu.pc = 0x4AE;
    16
}
/// $045BF8: movea.l (a7)+, a6
pub fn i_045bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45BFA;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $045BFA: move.w $22(a6), $22(a0)
pub fn i_045bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C00;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $045C00: move.w $24(a6), $24(a0)
pub fn i_045c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C06;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $045C06: move.b $68(a6), $68(a0)
pub fn i_045c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C0C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $045C0C: move.b $6d(a6), $6d(a0)
pub fn i_045c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C12;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x6D);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== Jsr5B6ThenJmpScheduler_045c12  ($045C12, 14 B, kernel/jsr5b6_jmp518.c)
/// $045C12: jsr $5b6.l
pub fn i_045c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x45C18);
    cpu.pc = 0x5B6;
    16
}
/// $045C18: jmp $518.l
pub fn i_045c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45C1E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Vehicle_JmpDeploy_045FDE  ($045FDE, 6 B, enemies/vehicles/deploy_045f2c.s)
/// $045FDE: jmp $308b0.l
pub fn i_045fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x45FE4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x308B0;
    8
}
// ===== Fade_WhiteFlash_Task_046608  ($046608, 84 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046608: move.w #$0, d0
pub fn i_046608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4660C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04660C: move.w #$1f, d1
pub fn i_04660c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46610;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046610: move.w #$1f, d2
pub fn i_046610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46614;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046614: move.w #$10, d3
pub fn i_046614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46618;
    cpu.ir = 0x363C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046618: jsr $52580.l
pub fn i_046618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4661E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4661E);
    cpu.pc = 0x52580;
    16
}
/// $04661E: move.b #$ff, $1081b1.l
pub fn i_04661e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46626;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081B1, v1 as u8);
    16
}
/// $046626: move.w #$20, $70(a6)
pub fn i_046626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4662C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04662C: lea.l $46632(pc), a1
pub fn i_04662c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46630;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46632;
    16
}
/// $046630: move.l a1, (a6)
pub fn i_046630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46632;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $046632: subq.w #$1, $70(a6)
pub fn i_046632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46636;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $046636: cmpi.w #$0, $70(a6)
pub fn i_046636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4663C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04663C: bgt.w $46662
pub fn i_04663c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46640;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x46662; return 10; }
    8
}
/// $046640: move.w #$1f, d0
pub fn i_046640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46644;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046644: move.w #$1f, d1
pub fn i_046644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46648;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046648: move.w #$1f, d2
pub fn i_046648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4664C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04664C: move.w #$10, d3
pub fn i_04664c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46650;
    cpu.ir = 0x363C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $046650: jsr $52580.l
pub fn i_046650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46656);
    cpu.pc = 0x52580;
    16
}
/// $046656: move.w #$10, $70(a6)
pub fn i_046656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4665C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_04665c  ($04665C, 8 B, kernel/task_handlers.c)
/// $04665C: lea.l $46664(pc), a1
pub fn i_04665c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46660;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x46664;
    16
}
/// $046660: move.l a1, (a6)
pub fn i_046660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46662;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $046662: rts
pub fn i_046662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46664;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fade_WhiteFlash_Done_046664  ($046664, 30 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046664: subq.w #$1, $70(a6)
pub fn i_046664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46668;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $046668: cmpi.w #$0, $70(a6)
pub fn i_046668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4666E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04666E: bgt.w $46680
pub fn i_04666e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46672;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x46680; return 10; }
    8
}
/// $046672: movea.l $c(a6), a1
pub fn i_046672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46676;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $046676: clr.b $21(a1)
pub fn i_046676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4667A;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04667A: jmp $518.l
pub fn i_04667a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46680;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $046680: rts
pub fn i_046680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46682;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SceneC_Load_Task_046682  ($046682, 42 B, enemies/vehicles/enemy46_drops_banners_046260.s)
/// $046682: jsr $22c8.l
pub fn i_046682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46688);
    cpu.pc = 0x22C8;
    16
}
/// $046688: move.b #$c, d0
pub fn i_046688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4668C;
    cpu.ir = 0x103C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04668C: jsr $43568.l
pub fn i_04668c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46692;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x46692);
    cpu.pc = 0x43568;
    16
}
/// $046692: moveq #$0, d0
pub fn i_046692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46694;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $046694: moveq #$0, d1
pub fn i_046694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x46696;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $046696: jsr $437da.l
pub fn i_046696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4669C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4669C);
    cpu.pc = 0x437DA;
    16
}
/// $04669C: move.w #$3, d0
pub fn i_04669c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0466A0: jsr $523b2.l
pub fn i_0466a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x466A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x466A6);
    cpu.pc = 0x523B2;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x040000, i_040000),
    (0x040006, i_040006),
    (0x04000A, i_04000a),
    (0x040010, i_040010),
    (0x040054, i_040054),
    (0x040058, i_040058),
    (0x04005C, i_04005c),
    (0x040062, i_040062),
    (0x040066, i_040066),
    (0x040068, i_040068),
    (0x04006C, i_04006c),
    (0x040072, i_040072),
    (0x040076, i_040076),
    (0x04007C, i_04007c),
    (0x040082, i_040082),
    (0x040086, i_040086),
    (0x040088, i_040088),
    (0x040268, i_040268),
    (0x04026C, i_04026c),
    (0x040272, i_040272),
    (0x040276, i_040276),
    (0x04027A, i_04027a),
    (0x04027E, i_04027e),
    (0x040282, i_040282),
    (0x040286, i_040286),
    (0x04028C, i_04028c),
    (0x040290, i_040290),
    (0x040294, i_040294),
    (0x040298, i_040298),
    (0x04029E, i_04029e),
    (0x0402A4, i_0402a4),
    (0x0402A8, i_0402a8),
    (0x0402AE, i_0402ae),
    (0x0402B4, i_0402b4),
    (0x0402BA, i_0402ba),
    (0x0402BE, i_0402be),
    (0x0402C0, i_0402c0),
    (0x0402C4, i_0402c4),
    (0x0402CA, i_0402ca),
    (0x0402CE, i_0402ce),
    (0x0402D2, i_0402d2),
    (0x0402D8, i_0402d8),
    (0x0402DC, i_0402dc),
    (0x0402E0, i_0402e0),
    (0x0402E4, i_0402e4),
    (0x0402E6, i_0402e6),
    (0x0402EA, i_0402ea),
    (0x0402EE, i_0402ee),
    (0x0402F0, i_0402f0),
    (0x0402F6, i_0402f6),
    (0x0402FA, i_0402fa),
    (0x040300, i_040300),
    (0x040304, i_040304),
    (0x040308, i_040308),
    (0x04030E, i_04030e),
    (0x040312, i_040312),
    (0x040316, i_040316),
    (0x040318, i_040318),
    (0x04031C, i_04031c),
    (0x040322, i_040322),
    (0x040326, i_040326),
    (0x04032A, i_04032a),
    (0x04032C, i_04032c),
    (0x04032E, i_04032e),
    (0x040332, i_040332),
    (0x040338, i_040338),
    (0x04033C, i_04033c),
    (0x040340, i_040340),
    (0x040342, i_040342),
    (0x040346, i_040346),
    (0x04034C, i_04034c),
    (0x040350, i_040350),
    (0x040354, i_040354),
    (0x040356, i_040356),
    (0x040358, i_040358),
    (0x04035C, i_04035c),
    (0x040362, i_040362),
    (0x040368, i_040368),
    (0x04036E, i_04036e),
    (0x040372, i_040372),
    (0x040374, i_040374),
    (0x040378, i_040378),
    (0x04037E, i_04037e),
    (0x040384, i_040384),
    (0x040388, i_040388),
    (0x04038C, i_04038c),
    (0x04038E, i_04038e),
    (0x042576, i_042576),
    (0x04257C, i_04257c),
    (0x042582, i_042582),
    (0x042586, i_042586),
    (0x04258C, i_04258c),
    (0x0428C6, i_0428c6),
    (0x0428CC, i_0428cc),
    (0x0428D0, i_0428d0),
    (0x0428D4, i_0428d4),
    (0x0428D8, i_0428d8),
    (0x0428DC, i_0428dc),
    (0x0428E0, i_0428e0),
    (0x0428E4, i_0428e4),
    (0x0428E8, i_0428e8),
    (0x0428EE, i_0428ee),
    (0x0428F2, i_0428f2),
    (0x0428F6, i_0428f6),
    (0x0428FC, i_0428fc),
    (0x042900, i_042900),
    (0x042904, i_042904),
    (0x04290A, i_04290a),
    (0x04290C, i_04290c),
    (0x042912, i_042912),
    (0x042916, i_042916),
    (0x04291A, i_04291a),
    (0x04291E, i_04291e),
    (0x042922, i_042922),
    (0x042928, i_042928),
    (0x04292C, i_04292c),
    (0x042932, i_042932),
    (0x042938, i_042938),
    (0x04293E, i_04293e),
    (0x042944, i_042944),
    (0x04294A, i_04294a),
    (0x04294E, i_04294e),
    (0x042954, i_042954),
    (0x042958, i_042958),
    (0x04295A, i_04295a),
    (0x04295E, i_04295e),
    (0x042960, i_042960),
    (0x042964, i_042964),
    (0x04296A, i_04296a),
    (0x042970, i_042970),
    (0x042972, i_042972),
    (0x042976, i_042976),
    (0x042978, i_042978),
    (0x04297A, i_04297a),
    (0x04297E, i_04297e),
    (0x042982, i_042982),
    (0x042986, i_042986),
    (0x04298A, i_04298a),
    (0x042990, i_042990),
    (0x042996, i_042996),
    (0x042998, i_042998),
    (0x04299C, i_04299c),
    (0x04299E, i_04299e),
    (0x0429A0, i_0429a0),
    (0x0429A4, i_0429a4),
    (0x0429A8, i_0429a8),
    (0x0429AC, i_0429ac),
    (0x0429B0, i_0429b0),
    (0x0429B6, i_0429b6),
    (0x0429BA, i_0429ba),
    (0x0429BE, i_0429be),
    (0x0429C2, i_0429c2),
    (0x0429C6, i_0429c6),
    (0x0429CA, i_0429ca),
    (0x0429CE, i_0429ce),
    (0x0429D2, i_0429d2),
    (0x0429D8, i_0429d8),
    (0x0429DC, i_0429dc),
    (0x0429E0, i_0429e0),
    (0x0429E4, i_0429e4),
    (0x0429EA, i_0429ea),
    (0x0429EE, i_0429ee),
    (0x0429F4, i_0429f4),
    (0x0429F8, i_0429f8),
    (0x0429FC, i_0429fc),
    (0x042A00, i_042a00),
    (0x042A02, i_042a02),
    (0x042A04, i_042a04),
    (0x042A08, i_042a08),
    (0x042A0C, i_042a0c),
    (0x042A10, i_042a10),
    (0x042A12, i_042a12),
    (0x042A14, i_042a14),
    (0x042A16, i_042a16),
    (0x042A1A, i_042a1a),
    (0x042A1E, i_042a1e),
    (0x042A20, i_042a20),
    (0x042A22, i_042a22),
    (0x042A26, i_042a26),
    (0x042A28, i_042a28),
    (0x042A2A, i_042a2a),
    (0x042A2E, i_042a2e),
    (0x042A34, i_042a34),
    (0x042A38, i_042a38),
    (0x042A3C, i_042a3c),
    (0x042A3E, i_042a3e),
    (0x042A42, i_042a42),
    (0x042A44, i_042a44),
    (0x042A48, i_042a48),
    (0x042A4E, i_042a4e),
    (0x042A52, i_042a52),
    (0x042A56, i_042a56),
    (0x042A5C, i_042a5c),
    (0x042A60, i_042a60),
    (0x042A66, i_042a66),
    (0x042A6A, i_042a6a),
    (0x042A86, i_042a86),
    (0x042A8C, i_042a8c),
    (0x042A90, i_042a90),
    (0x042A92, i_042a92),
    (0x042A96, i_042a96),
    (0x042A9A, i_042a9a),
    (0x042A9C, i_042a9c),
    (0x042AA0, i_042aa0),
    (0x042AA6, i_042aa6),
    (0x042AA8, i_042aa8),
    (0x042AAC, i_042aac),
    (0x042AB0, i_042ab0),
    (0x042AB2, i_042ab2),
    (0x042AB8, i_042ab8),
    (0x042ABC, i_042abc),
    (0x042AC0, i_042ac0),
    (0x042AC2, i_042ac2),
    (0x042AC8, i_042ac8),
    (0x0434EA, i_0434ea),
    (0x0434F0, i_0434f0),
    (0x0434F6, i_0434f6),
    (0x0434F8, i_0434f8),
    (0x0434FE, i_0434fe),
    (0x043500, i_043500),
    (0x043506, i_043506),
    (0x043508, i_043508),
    (0x04350E, i_04350e),
    (0x043510, i_043510),
    (0x043516, i_043516),
    (0x04351A, i_04351a),
    (0x04351C, i_04351c),
    (0x04351E, i_04351e),
    (0x043520, i_043520),
    (0x043526, i_043526),
    (0x043528, i_043528),
    (0x04352E, i_04352e),
    (0x043532, i_043532),
    (0x043538, i_043538),
    (0x04353C, i_04353c),
    (0x043542, i_043542),
    (0x043546, i_043546),
    (0x04354C, i_04354c),
    (0x043552, i_043552),
    (0x043554, i_043554),
    (0x043556, i_043556),
    (0x043558, i_043558),
    (0x04355A, i_04355a),
    (0x043560, i_043560),
    (0x043562, i_043562),
    (0x043564, i_043564),
    (0x043568, i_043568),
    (0x04356C, i_04356c),
    (0x04356E, i_04356e),
    (0x043574, i_043574),
    (0x04357C, i_04357c),
    (0x043580, i_043580),
    (0x043582, i_043582),
    (0x043586, i_043586),
    (0x043588, i_043588),
    (0x04358A, i_04358a),
    (0x04358C, i_04358c),
    (0x043590, i_043590),
    (0x043594, i_043594),
    (0x043596, i_043596),
    (0x04359A, i_04359a),
    (0x04359E, i_04359e),
    (0x0435A2, i_0435a2),
    (0x0435A6, i_0435a6),
    (0x0435AA, i_0435aa),
    (0x0435AE, i_0435ae),
    (0x0435B0, i_0435b0),
    (0x0435B2, i_0435b2),
    (0x0435B4, i_0435b4),
    (0x0435B8, i_0435b8),
    (0x0435BE, i_0435be),
    (0x0435C0, i_0435c0),
    (0x0435C4, i_0435c4),
    (0x0435C8, i_0435c8),
    (0x0435CA, i_0435ca),
    (0x0435CC, i_0435cc),
    (0x0435D0, i_0435d0),
    (0x0435D4, i_0435d4),
    (0x0435DA, i_0435da),
    (0x0435DE, i_0435de),
    (0x0435E4, i_0435e4),
    (0x0435E6, i_0435e6),
    (0x0435EA, i_0435ea),
    (0x0435EE, i_0435ee),
    (0x0435F4, i_0435f4),
    (0x0435FA, i_0435fa),
    (0x043600, i_043600),
    (0x043604, i_043604),
    (0x043608, i_043608),
    (0x04360A, i_04360a),
    (0x04360E, i_04360e),
    (0x043610, i_043610),
    (0x043616, i_043616),
    (0x04361C, i_04361c),
    (0x043622, i_043622),
    (0x043628, i_043628),
    (0x04362E, i_04362e),
    (0x043634, i_043634),
    (0x04363C, i_04363c),
    (0x043644, i_043644),
    (0x04364C, i_04364c),
    (0x043654, i_043654),
    (0x04365C, i_04365c),
    (0x043664, i_043664),
    (0x04366C, i_04366c),
    (0x043672, i_043672),
    (0x043678, i_043678),
    (0x043682, i_043682),
    (0x04368C, i_04368c),
    (0x043696, i_043696),
    (0x04369C, i_04369c),
    (0x0436A0, i_0436a0),
    (0x0436A4, i_0436a4),
    (0x0436AA, i_0436aa),
    (0x0436B0, i_0436b0),
    (0x0436B6, i_0436b6),
    (0x0436BE, i_0436be),
    (0x0436C4, i_0436c4),
    (0x0436CA, i_0436ca),
    (0x0436D0, i_0436d0),
    (0x0436D6, i_0436d6),
    (0x0436DC, i_0436dc),
    (0x0436DE, i_0436de),
    (0x0436E4, i_0436e4),
    (0x0436E8, i_0436e8),
    (0x0436EA, i_0436ea),
    (0x0436EE, i_0436ee),
    (0x0436F0, i_0436f0),
    (0x0436F4, i_0436f4),
    (0x0436F6, i_0436f6),
    (0x0436F8, i_0436f8),
    (0x0436FC, i_0436fc),
    (0x043700, i_043700),
    (0x043702, i_043702),
    (0x043704, i_043704),
    (0x043706, i_043706),
    (0x043708, i_043708),
    (0x04370A, i_04370a),
    (0x04370E, i_04370e),
    (0x043710, i_043710),
    (0x043712, i_043712),
    (0x043716, i_043716),
    (0x043718, i_043718),
    (0x04371E, i_04371e),
    (0x043722, i_043722),
    (0x043726, i_043726),
    (0x04372A, i_04372a),
    (0x04372C, i_04372c),
    (0x04372E, i_04372e),
    (0x043734, i_043734),
    (0x04373A, i_04373a),
    (0x04373E, i_04373e),
    (0x043742, i_043742),
    (0x043746, i_043746),
    (0x043748, i_043748),
    (0x04374C, i_04374c),
    (0x04374E, i_04374e),
    (0x043750, i_043750),
    (0x043752, i_043752),
    (0x043754, i_043754),
    (0x043756, i_043756),
    (0x043758, i_043758),
    (0x04375A, i_04375a),
    (0x04375C, i_04375c),
    (0x04375E, i_04375e),
    (0x043760, i_043760),
    (0x043762, i_043762),
    (0x043764, i_043764),
    (0x043766, i_043766),
    (0x043768, i_043768),
    (0x04376E, i_04376e),
    (0x043774, i_043774),
    (0x043778, i_043778),
    (0x04377E, i_04377e),
    (0x043780, i_043780),
    (0x043782, i_043782),
    (0x043784, i_043784),
    (0x043786, i_043786),
    (0x043788, i_043788),
    (0x04378C, i_04378c),
    (0x04378E, i_04378e),
    (0x043792, i_043792),
    (0x043794, i_043794),
    (0x043798, i_043798),
    (0x04379A, i_04379a),
    (0x04379E, i_04379e),
    (0x0437A0, i_0437a0),
    (0x0437A4, i_0437a4),
    (0x0437A6, i_0437a6),
    (0x0437AA, i_0437aa),
    (0x0437AC, i_0437ac),
    (0x0437B0, i_0437b0),
    (0x0437B2, i_0437b2),
    (0x0437B6, i_0437b6),
    (0x0437B8, i_0437b8),
    (0x0437BA, i_0437ba),
    (0x0437C0, i_0437c0),
    (0x0437C2, i_0437c2),
    (0x0437C6, i_0437c6),
    (0x0437C8, i_0437c8),
    (0x0437CA, i_0437ca),
    (0x0437CC, i_0437cc),
    (0x0437D0, i_0437d0),
    (0x0437D2, i_0437d2),
    (0x0437D4, i_0437d4),
    (0x0437D6, i_0437d6),
    (0x0437D8, i_0437d8),
    (0x0437DA, i_0437da),
    (0x0437E0, i_0437e0),
    (0x0437E4, i_0437e4),
    (0x0437EA, i_0437ea),
    (0x0437F0, i_0437f0),
    (0x0437F4, i_0437f4),
    (0x0437FA, i_0437fa),
    (0x043800, i_043800),
    (0x043806, i_043806),
    (0x04380C, i_04380c),
    (0x043812, i_043812),
    (0x043814, i_043814),
    (0x043816, i_043816),
    (0x043818, i_043818),
    (0x04381C, i_04381c),
    (0x043820, i_043820),
    (0x043824, i_043824),
    (0x043828, i_043828),
    (0x04382C, i_04382c),
    (0x043830, i_043830),
    (0x043834, i_043834),
    (0x043838, i_043838),
    (0x043840, i_043840),
    (0x043844, i_043844),
    (0x04384C, i_04384c),
    (0x043850, i_043850),
    (0x043858, i_043858),
    (0x04385C, i_04385c),
    (0x043860, i_043860),
    (0x04394A, i_04394a),
    (0x04394E, i_04394e),
    (0x043954, i_043954),
    (0x043958, i_043958),
    (0x04395C, i_04395c),
    (0x043962, i_043962),
    (0x043966, i_043966),
    (0x04396A, i_04396a),
    (0x043970, i_043970),
    (0x043978, i_043978),
    (0x043980, i_043980),
    (0x043984, i_043984),
    (0x043988, i_043988),
    (0x04398E, i_04398e),
    (0x043996, i_043996),
    (0x04399E, i_04399e),
    (0x0439A6, i_0439a6),
    (0x0439AE, i_0439ae),
    (0x0439B6, i_0439b6),
    (0x0439F0, i_0439f0),
    (0x0439F4, i_0439f4),
    (0x0439FA, i_0439fa),
    (0x0439FE, i_0439fe),
    (0x043A02, i_043a02),
    (0x043A04, i_043a04),
    (0x043A08, i_043a08),
    (0x043A0C, i_043a0c),
    (0x043A12, i_043a12),
    (0x043A1A, i_043a1a),
    (0x043A42, i_043a42),
    (0x043A46, i_043a46),
    (0x043A4C, i_043a4c),
    (0x043A4E, i_043a4e),
    (0x043A54, i_043a54),
    (0x043A58, i_043a58),
    (0x043A5C, i_043a5c),
    (0x043A62, i_043a62),
    (0x043A66, i_043a66),
    (0x043A6A, i_043a6a),
    (0x043A70, i_043a70),
    (0x043A74, i_043a74),
    (0x043A78, i_043a78),
    (0x043A7E, i_043a7e),
    (0x043A82, i_043a82),
    (0x043A88, i_043a88),
    (0x043AA4, i_043aa4),
    (0x043AA8, i_043aa8),
    (0x043AAA, i_043aaa),
    (0x043AAE, i_043aae),
    (0x043AB4, i_043ab4),
    (0x043AB8, i_043ab8),
    (0x043ABC, i_043abc),
    (0x043AC2, i_043ac2),
    (0x043AC6, i_043ac6),
    (0x043ACA, i_043aca),
    (0x043AD0, i_043ad0),
    (0x043AD6, i_043ad6),
    (0x043ADA, i_043ada),
    (0x043ADE, i_043ade),
    (0x043AE4, i_043ae4),
    (0x043AE8, i_043ae8),
    (0x043AEC, i_043aec),
    (0x043AF0, i_043af0),
    (0x043AF6, i_043af6),
    (0x043AFA, i_043afa),
    (0x043AFE, i_043afe),
    (0x043B04, i_043b04),
    (0x043B08, i_043b08),
    (0x043B0C, i_043b0c),
    (0x043B14, i_043b14),
    (0x043B1A, i_043b1a),
    (0x043B1C, i_043b1c),
    (0x043B1E, i_043b1e),
    (0x043B20, i_043b20),
    (0x043B22, i_043b22),
    (0x043B28, i_043b28),
    (0x043B2E, i_043b2e),
    (0x043B34, i_043b34),
    (0x043B3A, i_043b3a),
    (0x043B40, i_043b40),
    (0x043B4A, i_043b4a),
    (0x043B54, i_043b54),
    (0x043B5E, i_043b5e),
    (0x043B62, i_043b62),
    (0x043B66, i_043b66),
    (0x043B6A, i_043b6a),
    (0x043B70, i_043b70),
    (0x043B74, i_043b74),
    (0x043B7A, i_043b7a),
    (0x043B80, i_043b80),
    (0x043B86, i_043b86),
    (0x043B8C, i_043b8c),
    (0x043B90, i_043b90),
    (0x043B94, i_043b94),
    (0x043B98, i_043b98),
    (0x043B9E, i_043b9e),
    (0x043BA2, i_043ba2),
    (0x043BA6, i_043ba6),
    (0x043BAC, i_043bac),
    (0x043BB0, i_043bb0),
    (0x043BB6, i_043bb6),
    (0x043BBC, i_043bbc),
    (0x043BC2, i_043bc2),
    (0x043BC6, i_043bc6),
    (0x043BC8, i_043bc8),
    (0x043BCE, i_043bce),
    (0x043BD4, i_043bd4),
    (0x043BDA, i_043bda),
    (0x043BDE, i_043bde),
    (0x043BE4, i_043be4),
    (0x043BEA, i_043bea),
    (0x043BEC, i_043bec),
    (0x043BF2, i_043bf2),
    (0x043BF8, i_043bf8),
    (0x043BFE, i_043bfe),
    (0x043C04, i_043c04),
    (0x043C08, i_043c08),
    (0x043C0C, i_043c0c),
    (0x043C12, i_043c12),
    (0x043C14, i_043c14),
    (0x043C18, i_043c18),
    (0x043C1E, i_043c1e),
    (0x043C20, i_043c20),
    (0x043C22, i_043c22),
    (0x043C28, i_043c28),
    (0x043C2E, i_043c2e),
    (0x043C30, i_043c30),
    (0x043C32, i_043c32),
    (0x043C34, i_043c34),
    (0x043C36, i_043c36),
    (0x043C3A, i_043c3a),
    (0x043C40, i_043c40),
    (0x043C46, i_043c46),
    (0x043C4C, i_043c4c),
    (0x043C50, i_043c50),
    (0x043C54, i_043c54),
    (0x043C5A, i_043c5a),
    (0x043C60, i_043c60),
    (0x043C64, i_043c64),
    (0x043C68, i_043c68),
    (0x043C6C, i_043c6c),
    (0x043C72, i_043c72),
    (0x043C78, i_043c78),
    (0x043C7C, i_043c7c),
    (0x043C80, i_043c80),
    (0x043C84, i_043c84),
    (0x043C8A, i_043c8a),
    (0x043C90, i_043c90),
    (0x043C94, i_043c94),
    (0x043C9A, i_043c9a),
    (0x043CA0, i_043ca0),
    (0x043CA6, i_043ca6),
    (0x043CAC, i_043cac),
    (0x043CB0, i_043cb0),
    (0x043CB6, i_043cb6),
    (0x043CBA, i_043cba),
    (0x043CC0, i_043cc0),
    (0x043CC6, i_043cc6),
    (0x043CCC, i_043ccc),
    (0x043CD2, i_043cd2),
    (0x043CD8, i_043cd8),
    (0x043CDE, i_043cde),
    (0x043CE2, i_043ce2),
    (0x043CE8, i_043ce8),
    (0x043CEC, i_043cec),
    (0x043CF2, i_043cf2),
    (0x043CF6, i_043cf6),
    (0x043CFC, i_043cfc),
    (0x043D00, i_043d00),
    (0x043D04, i_043d04),
    (0x043D0A, i_043d0a),
    (0x043D10, i_043d10),
    (0x043D14, i_043d14),
    (0x043D18, i_043d18),
    (0x043D1C, i_043d1c),
    (0x043D20, i_043d20),
    (0x043D26, i_043d26),
    (0x043D2C, i_043d2c),
    (0x043D30, i_043d30),
    (0x043D34, i_043d34),
    (0x043D38, i_043d38),
    (0x043D3C, i_043d3c),
    (0x043D42, i_043d42),
    (0x043D48, i_043d48),
    (0x043D4C, i_043d4c),
    (0x043D50, i_043d50),
    (0x043D56, i_043d56),
    (0x043D5C, i_043d5c),
    (0x043D60, i_043d60),
    (0x043D64, i_043d64),
    (0x043D66, i_043d66),
    (0x043D6A, i_043d6a),
    (0x043D6C, i_043d6c),
    (0x043D72, i_043d72),
    (0x043D78, i_043d78),
    (0x043D7E, i_043d7e),
    (0x043D84, i_043d84),
    (0x043D86, i_043d86),
    (0x043D8C, i_043d8c),
    (0x043D90, i_043d90),
    (0x043D96, i_043d96),
    (0x043D9A, i_043d9a),
    (0x043DA0, i_043da0),
    (0x043DA4, i_043da4),
    (0x043DAA, i_043daa),
    (0x043DAE, i_043dae),
    (0x043DB2, i_043db2),
    (0x043DB8, i_043db8),
    (0x043DBE, i_043dbe),
    (0x043DC2, i_043dc2),
    (0x043DC4, i_043dc4),
    (0x043DC8, i_043dc8),
    (0x043DCA, i_043dca),
    (0x043DD0, i_043dd0),
    (0x043DD6, i_043dd6),
    (0x043DDA, i_043dda),
    (0x043DDE, i_043dde),
    (0x043DE4, i_043de4),
    (0x043DE8, i_043de8),
    (0x043DEC, i_043dec),
    (0x043DF2, i_043df2),
    (0x043DF4, i_043df4),
    (0x043DFA, i_043dfa),
    (0x043DFE, i_043dfe),
    (0x043E02, i_043e02),
    (0x043E04, i_043e04),
    (0x043E08, i_043e08),
    (0x043E0C, i_043e0c),
    (0x043E0E, i_043e0e),
    (0x043E14, i_043e14),
    (0x043E16, i_043e16),
    (0x043E1A, i_043e1a),
    (0x043E1C, i_043e1c),
    (0x043E1E, i_043e1e),
    (0x043E24, i_043e24),
    (0x043E2A, i_043e2a),
    (0x043E2C, i_043e2c),
    (0x043E30, i_043e30),
    (0x043E32, i_043e32),
    (0x043E34, i_043e34),
    (0x043E3A, i_043e3a),
    (0x043E3C, i_043e3c),
    (0x043E3E, i_043e3e),
    (0x043E40, i_043e40),
    (0x043E44, i_043e44),
    (0x043E46, i_043e46),
    (0x043E4A, i_043e4a),
    (0x043E4E, i_043e4e),
    (0x043E50, i_043e50),
    (0x043E54, i_043e54),
    (0x043E56, i_043e56),
    (0x043E5A, i_043e5a),
    (0x043E5C, i_043e5c),
    (0x043E60, i_043e60),
    (0x043E62, i_043e62),
    (0x043E68, i_043e68),
    (0x043E6C, i_043e6c),
    (0x043E6E, i_043e6e),
    (0x043E70, i_043e70),
    (0x043E72, i_043e72),
    (0x043E74, i_043e74),
    (0x043E76, i_043e76),
    (0x043E7A, i_043e7a),
    (0x043E7C, i_043e7c),
    (0x043E7E, i_043e7e),
    (0x043E80, i_043e80),
    (0x043E82, i_043e82),
    (0x043E84, i_043e84),
    (0x043E86, i_043e86),
    (0x043E88, i_043e88),
    (0x043E8A, i_043e8a),
    (0x043E8C, i_043e8c),
    (0x043E90, i_043e90),
    (0x043E94, i_043e94),
    (0x043E96, i_043e96),
    (0x043E98, i_043e98),
    (0x043E9A, i_043e9a),
    (0x043E9C, i_043e9c),
    (0x043E9E, i_043e9e),
    (0x043EA0, i_043ea0),
    (0x043EA2, i_043ea2),
    (0x043EA6, i_043ea6),
    (0x043EAA, i_043eaa),
    (0x043EAC, i_043eac),
    (0x043EAE, i_043eae),
    (0x043EB0, i_043eb0),
    (0x043EB2, i_043eb2),
    (0x043EB4, i_043eb4),
    (0x043EB6, i_043eb6),
    (0x043EBA, i_043eba),
    (0x043EBE, i_043ebe),
    (0x043EC2, i_043ec2),
    (0x043EC6, i_043ec6),
    (0x043EC8, i_043ec8),
    (0x043ECC, i_043ecc),
    (0x043ED0, i_043ed0),
    (0x043ED4, i_043ed4),
    (0x043ED8, i_043ed8),
    (0x043EDA, i_043eda),
    (0x043EDE, i_043ede),
    (0x043EE4, i_043ee4),
    (0x043EE6, i_043ee6),
    (0x043EEA, i_043eea),
    (0x043F02, i_043f02),
    (0x043F08, i_043f08),
    (0x043F0C, i_043f0c),
    (0x043F0E, i_043f0e),
    (0x043F12, i_043f12),
    (0x043F16, i_043f16),
    (0x043F18, i_043f18),
    (0x043F1A, i_043f1a),
    (0x043F1C, i_043f1c),
    (0x043F20, i_043f20),
    (0x043F24, i_043f24),
    (0x043F26, i_043f26),
    (0x043F28, i_043f28),
    (0x043F2A, i_043f2a),
    (0x043F2E, i_043f2e),
    (0x043F30, i_043f30),
    (0x043F34, i_043f34),
    (0x043F38, i_043f38),
    (0x043F3C, i_043f3c),
    (0x043F3E, i_043f3e),
    (0x043F40, i_043f40),
    (0x043F42, i_043f42),
    (0x043F44, i_043f44),
    (0x043F46, i_043f46),
    (0x043F4A, i_043f4a),
    (0x043F4E, i_043f4e),
    (0x043F52, i_043f52),
    (0x043F56, i_043f56),
    (0x043F58, i_043f58),
    (0x043F5C, i_043f5c),
    (0x043F5E, i_043f5e),
    (0x043F64, i_043f64),
    (0x043F68, i_043f68),
    (0x043F6A, i_043f6a),
    (0x043F6E, i_043f6e),
    (0x043F72, i_043f72),
    (0x043F74, i_043f74),
    (0x043F76, i_043f76),
    (0x043F78, i_043f78),
    (0x043F7C, i_043f7c),
    (0x043F80, i_043f80),
    (0x043F82, i_043f82),
    (0x043F84, i_043f84),
    (0x043F86, i_043f86),
    (0x043F8A, i_043f8a),
    (0x043F8C, i_043f8c),
    (0x043F90, i_043f90),
    (0x043F94, i_043f94),
    (0x043F98, i_043f98),
    (0x043F9A, i_043f9a),
    (0x043F9C, i_043f9c),
    (0x043F9E, i_043f9e),
    (0x043FA0, i_043fa0),
    (0x043FA2, i_043fa2),
    (0x043FA6, i_043fa6),
    (0x043FAA, i_043faa),
    (0x043FAC, i_043fac),
    (0x043FB0, i_043fb0),
    (0x043FB4, i_043fb4),
    (0x043FB6, i_043fb6),
    (0x043FBC, i_043fbc),
    (0x043FC0, i_043fc0),
    (0x043FC2, i_043fc2),
    (0x043FC6, i_043fc6),
    (0x043FCA, i_043fca),
    (0x043FCE, i_043fce),
    (0x043FD0, i_043fd0),
    (0x043FD4, i_043fd4),
    (0x043FD8, i_043fd8),
    (0x043FDC, i_043fdc),
    (0x043FE0, i_043fe0),
    (0x043FE4, i_043fe4),
    (0x043FE6, i_043fe6),
    (0x043FEA, i_043fea),
    (0x043FEE, i_043fee),
    (0x043FF2, i_043ff2),
    (0x043FF4, i_043ff4),
    (0x043FF6, i_043ff6),
    (0x043FF8, i_043ff8),
    (0x043FFC, i_043ffc),
    (0x043FFE, i_043ffe),
    (0x044004, i_044004),
    (0x044008, i_044008),
    (0x04400A, i_04400a),
    (0x04400C, i_04400c),
    (0x044048, i_044048),
    (0x04404E, i_04404e),
    (0x044052, i_044052),
    (0x044058, i_044058),
    (0x04405C, i_04405c),
    (0x04405E, i_04405e),
    (0x044064, i_044064),
    (0x044068, i_044068),
    (0x04406E, i_04406e),
    (0x044072, i_044072),
    (0x044078, i_044078),
    (0x0440BC, i_0440bc),
    (0x0440C2, i_0440c2),
    (0x0440C6, i_0440c6),
    (0x0440C8, i_0440c8),
    (0x0440CE, i_0440ce),
    (0x0440D0, i_0440d0),
    (0x0440D6, i_0440d6),
    (0x0440DC, i_0440dc),
    (0x0440DE, i_0440de),
    (0x0440E2, i_0440e2),
    (0x0440E4, i_0440e4),
    (0x0440EA, i_0440ea),
    (0x0440EE, i_0440ee),
    (0x0440F2, i_0440f2),
    (0x0440F6, i_0440f6),
    (0x0440FC, i_0440fc),
    (0x044102, i_044102),
    (0x044106, i_044106),
    (0x04410C, i_04410c),
    (0x044112, i_044112),
    (0x044116, i_044116),
    (0x04411C, i_04411c),
    (0x044120, i_044120),
    (0x044122, i_044122),
    (0x044124, i_044124),
    (0x044128, i_044128),
    (0x04412E, i_04412e),
    (0x044132, i_044132),
    (0x044138, i_044138),
    (0x04413C, i_04413c),
    (0x044142, i_044142),
    (0x044146, i_044146),
    (0x044148, i_044148),
    (0x04414E, i_04414e),
    (0x044150, i_044150),
    (0x044152, i_044152),
    (0x044154, i_044154),
    (0x044156, i_044156),
    (0x04415A, i_04415a),
    (0x04415C, i_04415c),
    (0x04415E, i_04415e),
    (0x044166, i_044166),
    (0x04416A, i_04416a),
    (0x044170, i_044170),
    (0x044174, i_044174),
    (0x044178, i_044178),
    (0x04417C, i_04417c),
    (0x044180, i_044180),
    (0x044182, i_044182),
    (0x044188, i_044188),
    (0x04418C, i_04418c),
    (0x044192, i_044192),
    (0x044198, i_044198),
    (0x04419E, i_04419e),
    (0x0441A4, i_0441a4),
    (0x0441A8, i_0441a8),
    (0x0441AE, i_0441ae),
    (0x0441B4, i_0441b4),
    (0x0441B8, i_0441b8),
    (0x0441BE, i_0441be),
    (0x0441C2, i_0441c2),
    (0x0441C4, i_0441c4),
    (0x0441C6, i_0441c6),
    (0x0441CA, i_0441ca),
    (0x0441D0, i_0441d0),
    (0x0441D4, i_0441d4),
    (0x0441DA, i_0441da),
    (0x0441DE, i_0441de),
    (0x0441E4, i_0441e4),
    (0x0441E8, i_0441e8),
    (0x0441EA, i_0441ea),
    (0x0441F0, i_0441f0),
    (0x0441F2, i_0441f2),
    (0x0441F4, i_0441f4),
    (0x0441F6, i_0441f6),
    (0x0441F8, i_0441f8),
    (0x0441FC, i_0441fc),
    (0x0441FE, i_0441fe),
    (0x044200, i_044200),
    (0x044208, i_044208),
    (0x04420C, i_04420c),
    (0x044212, i_044212),
    (0x044216, i_044216),
    (0x04421C, i_04421c),
    (0x044222, i_044222),
    (0x044228, i_044228),
    (0x044236, i_044236),
    (0x04423A, i_04423a),
    (0x04423E, i_04423e),
    (0x044240, i_044240),
    (0x044242, i_044242),
    (0x0442E6, i_0442e6),
    (0x0442E8, i_0442e8),
    (0x0442EE, i_0442ee),
    (0x0442F2, i_0442f2),
    (0x0442F8, i_0442f8),
    (0x0442FE, i_0442fe),
    (0x044300, i_044300),
    (0x044302, i_044302),
    (0x044304, i_044304),
    (0x044306, i_044306),
    (0x04430A, i_04430a),
    (0x04430E, i_04430e),
    (0x044314, i_044314),
    (0x044318, i_044318),
    (0x04431E, i_04431e),
    (0x044322, i_044322),
    (0x044328, i_044328),
    (0x04432C, i_04432c),
    (0x04432E, i_04432e),
    (0x044334, i_044334),
    (0x04433A, i_04433a),
    (0x044340, i_044340),
    (0x044346, i_044346),
    (0x04434C, i_04434c),
    (0x044352, i_044352),
    (0x044354, i_044354),
    (0x04435A, i_04435a),
    (0x04435E, i_04435e),
    (0x044362, i_044362),
    (0x044366, i_044366),
    (0x044368, i_044368),
    (0x04436C, i_04436c),
    (0x044372, i_044372),
    (0x044376, i_044376),
    (0x04437A, i_04437a),
    (0x04437E, i_04437e),
    (0x044382, i_044382),
    (0x044386, i_044386),
    (0x04438C, i_04438c),
    (0x044390, i_044390),
    (0x044396, i_044396),
    (0x04439A, i_04439a),
    (0x04439E, i_04439e),
    (0x0443A0, i_0443a0),
    (0x0443A4, i_0443a4),
    (0x0443A8, i_0443a8),
    (0x0443AC, i_0443ac),
    (0x0443B0, i_0443b0),
    (0x0443B4, i_0443b4),
    (0x0443B6, i_0443b6),
    (0x0443BA, i_0443ba),
    (0x0443BE, i_0443be),
    (0x0443C0, i_0443c0),
    (0x0443C2, i_0443c2),
    (0x0443C4, i_0443c4),
    (0x0443C8, i_0443c8),
    (0x0443CC, i_0443cc),
    (0x0443CE, i_0443ce),
    (0x0443D0, i_0443d0),
    (0x0443D4, i_0443d4),
    (0x0443D6, i_0443d6),
    (0x0443D8, i_0443d8),
    (0x0443DA, i_0443da),
    (0x0443DC, i_0443dc),
    (0x0443E0, i_0443e0),
    (0x0443E4, i_0443e4),
    (0x0443E8, i_0443e8),
    (0x0443EC, i_0443ec),
    (0x0443F0, i_0443f0),
    (0x0443F4, i_0443f4),
    (0x04446C, i_04446c),
    (0x04446E, i_04446e),
    (0x044470, i_044470),
    (0x044472, i_044472),
    (0x044474, i_044474),
    (0x04447C, i_04447c),
    (0x044480, i_044480),
    (0x044486, i_044486),
    (0x04448A, i_04448a),
    (0x04448E, i_04448e),
    (0x044496, i_044496),
    (0x04449A, i_04449a),
    (0x0444A0, i_0444a0),
    (0x0444A4, i_0444a4),
    (0x0444A8, i_0444a8),
    (0x0444AA, i_0444aa),
    (0x0444AE, i_0444ae),
    (0x0444B0, i_0444b0),
    (0x0444B4, i_0444b4),
    (0x0444B6, i_0444b6),
    (0x0444BC, i_0444bc),
    (0x0444C0, i_0444c0),
    (0x0444C6, i_0444c6),
    (0x0444CA, i_0444ca),
    (0x0444CE, i_0444ce),
    (0x0444D2, i_0444d2),
    (0x0444D6, i_0444d6),
    (0x0444DA, i_0444da),
    (0x0444DE, i_0444de),
    (0x0444E2, i_0444e2),
    (0x0444E6, i_0444e6),
    (0x04460A, i_04460a),
    (0x04460E, i_04460e),
    (0x044610, i_044610),
    (0x044614, i_044614),
    (0x044618, i_044618),
    (0x04461A, i_04461a),
    (0x04461C, i_04461c),
    (0x044620, i_044620),
    (0x044622, i_044622),
    (0x044624, i_044624),
    (0x044628, i_044628),
    (0x04462C, i_04462c),
    (0x044630, i_044630),
    (0x044632, i_044632),
    (0x044636, i_044636),
    (0x044638, i_044638),
    (0x04463A, i_04463a),
    (0x04463E, i_04463e),
    (0x044640, i_044640),
    (0x044644, i_044644),
    (0x044648, i_044648),
    (0x04464A, i_04464a),
    (0x04464C, i_04464c),
    (0x044650, i_044650),
    (0x044652, i_044652),
    (0x044654, i_044654),
    (0x044658, i_044658),
    (0x04465C, i_04465c),
    (0x04465E, i_04465e),
    (0x044660, i_044660),
    (0x044664, i_044664),
    (0x044666, i_044666),
    (0x04466A, i_04466a),
    (0x04466C, i_04466c),
    (0x044670, i_044670),
    (0x044672, i_044672),
    (0x044676, i_044676),
    (0x04467C, i_04467c),
    (0x044680, i_044680),
    (0x044682, i_044682),
    (0x044684, i_044684),
    (0x044686, i_044686),
    (0x044688, i_044688),
    (0x04468A, i_04468a),
    (0x04468E, i_04468e),
    (0x044692, i_044692),
    (0x044694, i_044694),
    (0x044698, i_044698),
    (0x04469A, i_04469a),
    (0x04469C, i_04469c),
    (0x04469E, i_04469e),
    (0x0446A2, i_0446a2),
    (0x0446A6, i_0446a6),
    (0x0446A8, i_0446a8),
    (0x0446AA, i_0446aa),
    (0x0446AE, i_0446ae),
    (0x0446B0, i_0446b0),
    (0x0446B4, i_0446b4),
    (0x0446B6, i_0446b6),
    (0x0446B8, i_0446b8),
    (0x0446BA, i_0446ba),
    (0x0446BE, i_0446be),
    (0x0446C2, i_0446c2),
    (0x0446C4, i_0446c4),
    (0x0446C6, i_0446c6),
    (0x0446CA, i_0446ca),
    (0x0446CC, i_0446cc),
    (0x0446CE, i_0446ce),
    (0x0446D0, i_0446d0),
    (0x0446D2, i_0446d2),
    (0x0446D6, i_0446d6),
    (0x0446DA, i_0446da),
    (0x0446E2, i_0446e2),
    (0x0446E6, i_0446e6),
    (0x0446EA, i_0446ea),
    (0x04472C, i_04472c),
    (0x044730, i_044730),
    (0x04473C, i_04473c),
    (0x044740, i_044740),
    (0x044742, i_044742),
    (0x044746, i_044746),
    (0x04474A, i_04474a),
    (0x04474C, i_04474c),
    (0x04474E, i_04474e),
    (0x044752, i_044752),
    (0x044754, i_044754),
    (0x044756, i_044756),
    (0x04475A, i_04475a),
    (0x04475E, i_04475e),
    (0x044762, i_044762),
    (0x044764, i_044764),
    (0x044768, i_044768),
    (0x04476A, i_04476a),
    (0x04476E, i_04476e),
    (0x044770, i_044770),
    (0x044774, i_044774),
    (0x044778, i_044778),
    (0x04477A, i_04477a),
    (0x04477C, i_04477c),
    (0x044780, i_044780),
    (0x044782, i_044782),
    (0x044784, i_044784),
    (0x044788, i_044788),
    (0x04478C, i_04478c),
    (0x044790, i_044790),
    (0x044792, i_044792),
    (0x044796, i_044796),
    (0x044798, i_044798),
    (0x04479C, i_04479c),
    (0x0447A0, i_0447a0),
    (0x0447A4, i_0447a4),
    (0x0447C6, i_0447c6),
    (0x0447CA, i_0447ca),
    (0x0447CE, i_0447ce),
    (0x0447D4, i_0447d4),
    (0x0447D8, i_0447d8),
    (0x0447DC, i_0447dc),
    (0x0447E0, i_0447e0),
    (0x0447E4, i_0447e4),
    (0x0447E8, i_0447e8),
    (0x0447EA, i_0447ea),
    (0x0447EE, i_0447ee),
    (0x0447F4, i_0447f4),
    (0x0447F6, i_0447f6),
    (0x0447FA, i_0447fa),
    (0x0447FE, i_0447fe),
    (0x044802, i_044802),
    (0x044804, i_044804),
    (0x044808, i_044808),
    (0x04480E, i_04480e),
    (0x044812, i_044812),
    (0x044816, i_044816),
    (0x044818, i_044818),
    (0x04481C, i_04481c),
    (0x044820, i_044820),
    (0x044824, i_044824),
    (0x044828, i_044828),
    (0x04482A, i_04482a),
    (0x044830, i_044830),
    (0x044834, i_044834),
    (0x04483A, i_04483a),
    (0x04483E, i_04483e),
    (0x044842, i_044842),
    (0x044848, i_044848),
    (0x044850, i_044850),
    (0x044854, i_044854),
    (0x044858, i_044858),
    (0x04485A, i_04485a),
    (0x04485E, i_04485e),
    (0x044860, i_044860),
    (0x044862, i_044862),
    (0x044866, i_044866),
    (0x04486A, i_04486a),
    (0x04486C, i_04486c),
    (0x044872, i_044872),
    (0x044876, i_044876),
    (0x04487A, i_04487a),
    (0x04487E, i_04487e),
    (0x044884, i_044884),
    (0x044888, i_044888),
    (0x04488C, i_04488c),
    (0x04488E, i_04488e),
    (0x044892, i_044892),
    (0x044896, i_044896),
    (0x04489C, i_04489c),
    (0x0448A4, i_0448a4),
    (0x0448A6, i_0448a6),
    (0x0448AA, i_0448aa),
    (0x0448AE, i_0448ae),
    (0x0448B0, i_0448b0),
    (0x0448B2, i_0448b2),
    (0x0448B6, i_0448b6),
    (0x0448B8, i_0448b8),
    (0x0448BA, i_0448ba),
    (0x0448BE, i_0448be),
    (0x0448C2, i_0448c2),
    (0x0448C6, i_0448c6),
    (0x0448CA, i_0448ca),
    (0x0448CE, i_0448ce),
    (0x0448D4, i_0448d4),
    (0x0448D8, i_0448d8),
    (0x0448DC, i_0448dc),
    (0x0448E2, i_0448e2),
    (0x0448E6, i_0448e6),
    (0x0448E8, i_0448e8),
    (0x0448EC, i_0448ec),
    (0x0448EE, i_0448ee),
    (0x0448F0, i_0448f0),
    (0x0448F6, i_0448f6),
    (0x0448FA, i_0448fa),
    (0x0448FE, i_0448fe),
    (0x044904, i_044904),
    (0x04490A, i_04490a),
    (0x04490E, i_04490e),
    (0x044912, i_044912),
    (0x044916, i_044916),
    (0x04491A, i_04491a),
    (0x044920, i_044920),
    (0x044924, i_044924),
    (0x044928, i_044928),
    (0x04492C, i_04492c),
    (0x044932, i_044932),
    (0x044936, i_044936),
    (0x04493A, i_04493a),
    (0x04493E, i_04493e),
    (0x044942, i_044942),
    (0x044948, i_044948),
    (0x04494A, i_04494a),
    (0x04494E, i_04494e),
    (0x044952, i_044952),
    (0x044958, i_044958),
    (0x04495C, i_04495c),
    (0x044962, i_044962),
    (0x044966, i_044966),
    (0x04496C, i_04496c),
    (0x044972, i_044972),
    (0x044976, i_044976),
    (0x044978, i_044978),
    (0x04497C, i_04497c),
    (0x044980, i_044980),
    (0x044984, i_044984),
    (0x044986, i_044986),
    (0x044988, i_044988),
    (0x04498C, i_04498c),
    (0x04498E, i_04498e),
    (0x044990, i_044990),
    (0x044996, i_044996),
    (0x04499C, i_04499c),
    (0x0449A2, i_0449a2),
    (0x0449A4, i_0449a4),
    (0x0457A6, i_0457a6),
    (0x0457AA, i_0457aa),
    (0x0457AE, i_0457ae),
    (0x0457B2, i_0457b2),
    (0x0457B6, i_0457b6),
    (0x0457BA, i_0457ba),
    (0x0457BE, i_0457be),
    (0x0457C2, i_0457c2),
    (0x0457C6, i_0457c6),
    (0x0457CC, i_0457cc),
    (0x0457D2, i_0457d2),
    (0x0457D6, i_0457d6),
    (0x0457D8, i_0457d8),
    (0x0457DC, i_0457dc),
    (0x0457DE, i_0457de),
    (0x0457E0, i_0457e0),
    (0x0457E2, i_0457e2),
    (0x0457E6, i_0457e6),
    (0x0457EA, i_0457ea),
    (0x0457EC, i_0457ec),
    (0x0457EE, i_0457ee),
    (0x0457F0, i_0457f0),
    (0x0457F6, i_0457f6),
    (0x0457FC, i_0457fc),
    (0x045800, i_045800),
    (0x045804, i_045804),
    (0x045806, i_045806),
    (0x04580A, i_04580a),
    (0x04580C, i_04580c),
    (0x045810, i_045810),
    (0x045816, i_045816),
    (0x04581A, i_04581a),
    (0x045820, i_045820),
    (0x045826, i_045826),
    (0x04582C, i_04582c),
    (0x045830, i_045830),
    (0x045832, i_045832),
    (0x045836, i_045836),
    (0x04583C, i_04583c),
    (0x045842, i_045842),
    (0x045848, i_045848),
    (0x04584C, i_04584c),
    (0x045852, i_045852),
    (0x045858, i_045858),
    (0x04585C, i_04585c),
    (0x045860, i_045860),
    (0x045864, i_045864),
    (0x045866, i_045866),
    (0x04586C, i_04586c),
    (0x045870, i_045870),
    (0x045874, i_045874),
    (0x045878, i_045878),
    (0x04587C, i_04587c),
    (0x045882, i_045882),
    (0x045886, i_045886),
    (0x04588A, i_04588a),
    (0x04588E, i_04588e),
    (0x045892, i_045892),
    (0x045896, i_045896),
    (0x04589A, i_04589a),
    (0x04589E, i_04589e),
    (0x0458A2, i_0458a2),
    (0x0458A6, i_0458a6),
    (0x0458AA, i_0458aa),
    (0x0458AE, i_0458ae),
    (0x0458B4, i_0458b4),
    (0x0458BA, i_0458ba),
    (0x0458C0, i_0458c0),
    (0x0458C4, i_0458c4),
    (0x0458C8, i_0458c8),
    (0x0458CC, i_0458cc),
    (0x0458CE, i_0458ce),
    (0x0458D2, i_0458d2),
    (0x0458D4, i_0458d4),
    (0x0458DA, i_0458da),
    (0x0458DC, i_0458dc),
    (0x0458E0, i_0458e0),
    (0x0458E6, i_0458e6),
    (0x0458EA, i_0458ea),
    (0x0458EE, i_0458ee),
    (0x0458F2, i_0458f2),
    (0x0458F6, i_0458f6),
    (0x0458FA, i_0458fa),
    (0x0458FE, i_0458fe),
    (0x045900, i_045900),
    (0x045902, i_045902),
    (0x045906, i_045906),
    (0x04590A, i_04590a),
    (0x04590E, i_04590e),
    (0x045912, i_045912),
    (0x045916, i_045916),
    (0x04591A, i_04591a),
    (0x04591E, i_04591e),
    (0x045922, i_045922),
    (0x045926, i_045926),
    (0x04592A, i_04592a),
    (0x04592E, i_04592e),
    (0x045934, i_045934),
    (0x045938, i_045938),
    (0x04593A, i_04593a),
    (0x04593E, i_04593e),
    (0x045942, i_045942),
    (0x045946, i_045946),
    (0x04594A, i_04594a),
    (0x04594E, i_04594e),
    (0x045952, i_045952),
    (0x045956, i_045956),
    (0x04595C, i_04595c),
    (0x045960, i_045960),
    (0x045966, i_045966),
    (0x04596A, i_04596a),
    (0x04596E, i_04596e),
    (0x045974, i_045974),
    (0x045978, i_045978),
    (0x04597C, i_04597c),
    (0x045980, i_045980),
    (0x045984, i_045984),
    (0x045988, i_045988),
    (0x04598C, i_04598c),
    (0x045990, i_045990),
    (0x045996, i_045996),
    (0x04599A, i_04599a),
    (0x04599C, i_04599c),
    (0x0459A2, i_0459a2),
    (0x0459A6, i_0459a6),
    (0x0459F0, i_0459f0),
    (0x0459F6, i_0459f6),
    (0x0459FE, i_0459fe),
    (0x045A04, i_045a04),
    (0x045A0A, i_045a0a),
    (0x045A10, i_045a10),
    (0x045A14, i_045a14),
    (0x045A1A, i_045a1a),
    (0x045A20, i_045a20),
    (0x045A26, i_045a26),
    (0x045A2A, i_045a2a),
    (0x045A2E, i_045a2e),
    (0x045A32, i_045a32),
    (0x045A36, i_045a36),
    (0x045A38, i_045a38),
    (0x045A3C, i_045a3c),
    (0x045A40, i_045a40),
    (0x045A44, i_045a44),
    (0x045A48, i_045a48),
    (0x045A4C, i_045a4c),
    (0x045A4E, i_045a4e),
    (0x045A50, i_045a50),
    (0x045A54, i_045a54),
    (0x045A58, i_045a58),
    (0x045A5C, i_045a5c),
    (0x045A5E, i_045a5e),
    (0x045A62, i_045a62),
    (0x045A64, i_045a64),
    (0x045A66, i_045a66),
    (0x045A6A, i_045a6a),
    (0x045A6E, i_045a6e),
    (0x045A70, i_045a70),
    (0x045A74, i_045a74),
    (0x045A76, i_045a76),
    (0x045A7A, i_045a7a),
    (0x045A7C, i_045a7c),
    (0x045A7E, i_045a7e),
    (0x045A80, i_045a80),
    (0x045A84, i_045a84),
    (0x045A88, i_045a88),
    (0x045A8C, i_045a8c),
    (0x045A90, i_045a90),
    (0x045A94, i_045a94),
    (0x045A96, i_045a96),
    (0x045A9A, i_045a9a),
    (0x045A9E, i_045a9e),
    (0x045AA4, i_045aa4),
    (0x045AA8, i_045aa8),
    (0x045AAC, i_045aac),
    (0x045AAE, i_045aae),
    (0x045AB2, i_045ab2),
    (0x045AB6, i_045ab6),
    (0x045ABA, i_045aba),
    (0x045B9C, i_045b9c),
    (0x045BA0, i_045ba0),
    (0x045BA6, i_045ba6),
    (0x045BAC, i_045bac),
    (0x045BB0, i_045bb0),
    (0x045BB4, i_045bb4),
    (0x045BB8, i_045bb8),
    (0x045BBE, i_045bbe),
    (0x045BC2, i_045bc2),
    (0x045BC6, i_045bc6),
    (0x045BCA, i_045bca),
    (0x045BCC, i_045bcc),
    (0x045BD2, i_045bd2),
    (0x045BD6, i_045bd6),
    (0x045BDA, i_045bda),
    (0x045BE0, i_045be0),
    (0x045BE4, i_045be4),
    (0x045BE6, i_045be6),
    (0x045BE8, i_045be8),
    (0x045BEE, i_045bee),
    (0x045BF2, i_045bf2),
    (0x045BF8, i_045bf8),
    (0x045BFA, i_045bfa),
    (0x045C00, i_045c00),
    (0x045C06, i_045c06),
    (0x045C0C, i_045c0c),
    (0x045C12, i_045c12),
    (0x045C18, i_045c18),
    (0x045FDE, i_045fde),
    (0x046608, i_046608),
    (0x04660C, i_04660c),
    (0x046610, i_046610),
    (0x046614, i_046614),
    (0x046618, i_046618),
    (0x04661E, i_04661e),
    (0x046626, i_046626),
    (0x04662C, i_04662c),
    (0x046630, i_046630),
    (0x046632, i_046632),
    (0x046636, i_046636),
    (0x04663C, i_04663c),
    (0x046640, i_046640),
    (0x046644, i_046644),
    (0x046648, i_046648),
    (0x04664C, i_04664c),
    (0x046650, i_046650),
    (0x046656, i_046656),
    (0x04665C, i_04665c),
    (0x046660, i_046660),
    (0x046662, i_046662),
    (0x046664, i_046664),
    (0x046668, i_046668),
    (0x04666E, i_04666e),
    (0x046672, i_046672),
    (0x046676, i_046676),
    (0x04667A, i_04667a),
    (0x046680, i_046680),
    (0x046682, i_046682),
    (0x046688, i_046688),
    (0x04668C, i_04668c),
    (0x046692, i_046692),
    (0x046694, i_046694),
    (0x046696, i_046696),
    (0x04669C, i_04669c),
    (0x0466A0, i_0466a0),
];
