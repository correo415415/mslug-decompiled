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
// ===== Pow_WalkTurn_040014  ($040014, 64 B, pow/results_squadleader_03daa8.s)
/// $040014: lea.l $3f01c(pc), a0
pub fn i_040014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40018;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F01C;
    16
}
/// $040018: move.l a0, $48(a6)
pub fn i_040018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4001C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04001C: lea.l $3f194(pc), a0
pub fn i_04001c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40020;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F194;
    16
}
/// $040020: jsr $28cd4.l
pub fn i_040020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40026);
    cpu.pc = 0x28CD4;
    16
}
/// $040026: clr.w $28(a6)
pub fn i_040026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4002A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04002A: clr.w $2a(a6)
pub fn i_04002a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4002E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04002E: lea.l $40034(pc), a1
pub fn i_04002e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40032;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40034;
    16
}
/// $040032: move.l a1, (a6)
pub fn i_040032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40034;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040034: jsr $28364.l
pub fn i_040034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4003A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4003A);
    cpu.pc = 0x28364;
    16
}
/// $04003A: scs.b $70(a6)
pub fn i_04003a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4003E;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $04003E: tst.b $70(a6)
pub fn i_04003e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40042;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $040042: beq.w $4004c
pub fn i_040042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40046;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4004C; return 10; }
    8
}
/// $040046: lea.l $3ff62(pc), a1
pub fn i_040046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4004A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FF62;
    16
}
/// $04004A: move.l a1, (a6)
pub fn i_04004a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4004C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04004C: jsr $28d70.l
pub fn i_04004c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40052;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40052);
    cpu.pc = 0x28D70;
    16
}
/// $040052: bra.b $3ffea
pub fn i_040052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40054;
    cpu.ir = 0x6096;
    cpu.pc = 0x3FFEA;
    10
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
// ===== Pow_Jump_04008c  ($04008C, 54 B, pow/results_squadleader_03daa8.s)
/// $04008C: lea.l $3f596(pc), a0
pub fn i_04008c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40090;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F596;
    16
}
/// $040090: jsr $28cd4.l
pub fn i_040090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40096;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40096);
    cpu.pc = 0x28CD4;
    16
}
/// $040096: lea.l $4009c(pc), a1
pub fn i_040096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4009A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4009C;
    16
}
/// $04009A: move.l a1, (a6)
pub fn i_04009a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4009C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04009C: clr.w $2e(a6)
pub fn i_04009c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400A0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0400A0: clr.w $2a(a6)
pub fn i_0400a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400A4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0400A4: move.b $72(a6), d0
pub fn i_0400a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400A8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0400A8: jsr $77190.l
pub fn i_0400a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x400AE);
    cpu.pc = 0x77190;
    16
}
/// $0400AE: jsr $28d70.l
pub fn i_0400ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x400B4);
    cpu.pc = 0x28D70;
    16
}
/// $0400B4: bcc.w $400be
pub fn i_0400b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400B8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x400BE; return 10; }
    8
}
/// $0400B8: lea.l $400c2(pc), a1
pub fn i_0400b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x400C2;
    16
}
/// $0400BC: move.l a1, (a6)
pub fn i_0400bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0400BE: bra.w $3ffea
pub fn i_0400be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FFEA;
    10
}
// ===== Pow_JumpAir_0400c2  ($0400C2, 94 B, pow/results_squadleader_03daa8.s)
/// $0400C2: lea.l $3f5ac(pc), a0
pub fn i_0400c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400C6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F5AC;
    16
}
/// $0400C6: jsr $28cd4.l
pub fn i_0400c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x400CC);
    cpu.pc = 0x28CD4;
    16
}
/// $0400CC: clr.w $2e(a6)
pub fn i_0400cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400D0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0400D0: move.w #$154, $2a(a6)
pub fn i_0400d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x154;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0400D6: lea.l $400dc(pc), a1
pub fn i_0400d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x400DC;
    16
}
/// $0400DA: move.l a1, (a6)
pub fn i_0400da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0400DC: move.b $72(a6), d0
pub fn i_0400dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400E0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0400E0: jsr $77190.l
pub fn i_0400e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x400E6);
    cpu.pc = 0x77190;
    16
}
/// $0400E6: tst.w d0
pub fn i_0400e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400E8;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0400E8: bpl.w $400f6
pub fn i_0400e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400EC;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x400F6; return 10; }
    8
}
/// $0400EC: lea.l $3ff24(pc), a1
pub fn i_0400ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FF24;
    16
}
/// $0400F0: move.l a1, (a6)
pub fn i_0400f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0400F2: bra.w $40108
pub fn i_0400f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x40108;
    10
}
/// $0400F6: subi.w #$24, d0
pub fn i_0400f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400FA;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x24, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0400FA: sub.w $24(a6), d0
pub fn i_0400fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x400FE;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0400FE: bcc.w $40108
pub fn i_0400fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40102;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40108; return 10; }
    8
}
/// $040102: lea.l $40120(pc), a1
pub fn i_040102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40106;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40120;
    16
}
/// $040106: move.l a1, (a6)
pub fn i_040106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40108;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040108: jsr $28d70.l
pub fn i_040108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4010E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4010E);
    cpu.pc = 0x28D70;
    16
}
/// $04010E: bcc.w $4011c
pub fn i_04010e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40112;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4011C; return 10; }
    8
}
/// $040112: lea.l $3f5ac(pc), a0
pub fn i_040112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40116;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F5AC;
    16
}
/// $040116: jsr $28cd4.l
pub fn i_040116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4011C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4011C);
    cpu.pc = 0x28CD4;
    16
}
/// $04011C: bra.w $3ffea
pub fn i_04011c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40120;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FFEA;
    10
}
// ===== Pow_Crouch_040120  ($040120, 42 B, pow/results_squadleader_03daa8.s)
/// $040120: lea.l $3f636(pc), a0
pub fn i_040120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40124;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F636;
    16
}
/// $040124: jsr $28cd4.l
pub fn i_040124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4012A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4012A);
    cpu.pc = 0x28CD4;
    16
}
/// $04012A: lea.l $40130(pc), a1
pub fn i_04012a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4012E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40130;
    16
}
/// $04012E: move.l a1, (a6)
pub fn i_04012e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40130;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040130: jsr $2783a.l
pub fn i_040130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40136;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40136);
    cpu.pc = 0x2783A;
    16
}
/// $040136: jsr $28d70.l
pub fn i_040136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4013C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4013C);
    cpu.pc = 0x28D70;
    16
}
/// $04013C: bcc.w $40146
pub fn i_04013c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40140;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40146; return 10; }
    8
}
/// $040140: lea.l $4014a(pc), a1
pub fn i_040140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40144;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4014A;
    16
}
/// $040144: move.l a1, (a6)
pub fn i_040144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40146;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040146: bra.w $3ffea
pub fn i_040146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4014A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FFEA;
    10
}
// ===== Pow_CrouchHop_04014a  ($04014A, 94 B, pow/results_squadleader_03daa8.s)
/// $04014A: lea.l $3f71e(pc), a0
pub fn i_04014a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4014E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F71E;
    16
}
/// $04014E: jsr $28cd4.l
pub fn i_04014e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40154);
    cpu.pc = 0x28CD4;
    16
}
/// $040154: move.w #$fccd, d0
pub fn i_040154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40158;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFCCD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $040158: jsr $5dca4.l
pub fn i_040158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4015E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4015E);
    cpu.pc = 0x5DCA4;
    16
}
/// $04015E: move.w d0, $28(a6)
pub fn i_04015e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40162;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040162: move.w #$4c9, $2a(a6)
pub fn i_040162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40168;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4C9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040168: move.w #$ff0b, $2e(a6)
pub fn i_040168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4016E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF0B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04016E: move.w #$0, $2c(a6)
pub fn i_04016e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40174;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040174: lea.l $4017a(pc), a1
pub fn i_040174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40178;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4017A;
    16
}
/// $040178: move.l a1, (a6)
pub fn i_040178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4017A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04017A: move.w $28(a6), d0
pub fn i_04017a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4017E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04017E: beq.w $40190
pub fn i_04017e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40182;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x40190; return 10; }
    8
}
/// $040182: asr.w #$4, d0
pub fn i_040182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40184;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $040184: seq.b d1
pub fn i_040184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40186;
    cpu.ir = 0x57C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Eq) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $040186: ext.w d1
pub fn i_040186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40188;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $040188: eor.w d1, d0
pub fn i_040188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4018A;
    cpu.ir = 0xB340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04018A: sub.w d1, d0
pub fn i_04018a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4018C;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04018C: sub.w d0, $28(a6)
pub fn i_04018c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40190;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $040190: bsr.w $3fece
pub fn i_040190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40194;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x40194);
    cpu.pc = 0x3FECE;
    18
}
/// $040194: jsr $28d70.l
pub fn i_040194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4019A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4019A);
    cpu.pc = 0x28D70;
    16
}
/// $04019A: bcc.w $401a4
pub fn i_04019a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4019E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x401A4; return 10; }
    8
}
/// $04019E: lea.l $3ff24(pc), a1
pub fn i_04019e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FF24;
    16
}
/// $0401A2: move.l a1, (a6)
pub fn i_0401a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0401A4: bra.w $3ffea
pub fn i_0401a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FFEA;
    10
}
// ===== Pow_Fall_0401a8  ($0401A8, 54 B, pow/results_squadleader_03daa8.s)
/// $0401A8: lea.l $3f72a(pc), a0
pub fn i_0401a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401AC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F72A;
    16
}
/// $0401AC: jsr $28cd4.l
pub fn i_0401ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x401B2);
    cpu.pc = 0x28CD4;
    16
}
/// $0401B2: clr.w $2a(a6)
pub fn i_0401b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401B6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0401B6: clr.w $2e(a6)
pub fn i_0401b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401BA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0401BA: lea.l $401c0(pc), a1
pub fn i_0401ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x401C0;
    16
}
/// $0401BE: move.l a1, (a6)
pub fn i_0401be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0401C0: move.b $72(a6), d0
pub fn i_0401c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401C4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0401C4: jsr $77190.l
pub fn i_0401c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x401CA);
    cpu.pc = 0x77190;
    16
}
/// $0401CA: jsr $28d70.l
pub fn i_0401ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x401D0);
    cpu.pc = 0x28D70;
    16
}
/// $0401D0: bcc.w $401da
pub fn i_0401d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x401DA; return 10; }
    8
}
/// $0401D4: lea.l $401de(pc), a1
pub fn i_0401d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x401DE;
    16
}
/// $0401D8: move.l a1, (a6)
pub fn i_0401d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0401DA: bra.w $3ffea
pub fn i_0401da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FFEA;
    10
}
// ===== Pow_FallAir_0401de  ($0401DE, 86 B, pow/results_squadleader_03daa8.s)
/// $0401DE: lea.l $3f80e(pc), a0
pub fn i_0401de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401E2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F80E;
    16
}
/// $0401E2: jsr $28cd4.l
pub fn i_0401e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x401E8);
    cpu.pc = 0x28CD4;
    16
}
/// $0401E8: clr.w $2e(a6)
pub fn i_0401e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401EC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0401EC: move.w #$feac, $2a(a6)
pub fn i_0401ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFEAC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0401F2: lea.l $401f8(pc), a1
pub fn i_0401f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x401F8;
    16
}
/// $0401F6: move.l a1, (a6)
pub fn i_0401f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0401F8: move.b $72(a6), d0
pub fn i_0401f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x401FC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0401FC: jsr $77190.l
pub fn i_0401fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40202;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40202);
    cpu.pc = 0x77190;
    16
}
/// $040202: move.w d0, -(a7)
pub fn i_040202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40204;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $040204: jsr $28d70.l
pub fn i_040204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4020A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4020A);
    cpu.pc = 0x28D70;
    16
}
/// $04020A: bcc.w $40214
pub fn i_04020a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4020E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40214; return 10; }
    8
}
/// $04020E: lea.l $401de(pc), a1
pub fn i_04020e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40212;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x401DE;
    16
}
/// $040212: move.l a1, (a6)
pub fn i_040212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40214;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040214: move.w (a7)+, d0
pub fn i_040214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40216;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $040216: bpl.w $40220
pub fn i_040216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4021A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x40220; return 10; }
    8
}
/// $04021A: lea.l $40014(pc), a1
pub fn i_04021a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4021E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40014;
    16
}
/// $04021E: move.l a1, (a6)
pub fn i_04021e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40220;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040220: jsr $27eba.l
pub fn i_040220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40226);
    cpu.pc = 0x27EBA;
    16
}
/// $040226: bcs.w $40230
pub fn i_040226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4022A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x40230; return 10; }
    8
}
/// $04022A: lea.l $40234(pc), a1
pub fn i_04022a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4022E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40234;
    16
}
/// $04022E: move.l a1, (a6)
pub fn i_04022e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40230;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040230: bra.w $3ffea
pub fn i_040230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40234;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FFEA;
    10
}
// ===== Pow_Land_040234  ($040234, 52 B, pow/results_squadleader_03daa8.s)
/// $040234: lea.l $3f888(pc), a0
pub fn i_040234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40238;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F888;
    16
}
/// $040238: jsr $28cd4.l
pub fn i_040238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4023E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4023E);
    cpu.pc = 0x28CD4;
    16
}
/// $04023E: clr.w $28(a6)
pub fn i_04023e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40242;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $040242: clr.w $2a(a6)
pub fn i_040242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40246;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $040246: clr.b $70(a6)
pub fn i_040246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4024A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04024A: lea.l $40250(pc), a1
pub fn i_04024a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4024E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40250;
    16
}
/// $04024E: move.l a1, (a6)
pub fn i_04024e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40250;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040250: bsr.w $3fece
pub fn i_040250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40254;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x40254);
    cpu.pc = 0x3FECE;
    18
}
/// $040254: jsr $28d70.l
pub fn i_040254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4025A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4025A);
    cpu.pc = 0x28D70;
    16
}
/// $04025A: bcc.w $40264
pub fn i_04025a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4025E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40264; return 10; }
    8
}
/// $04025E: lea.l $3ff24(pc), a1
pub fn i_04025e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40262;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FF24;
    16
}
/// $040262: move.l a1, (a6)
pub fn i_040262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40264;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040264: bra.w $3ffea
pub fn i_040264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40268;
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
// ===== Pow_RescuedExit_040392  ($040392, 54 B, pow/results_squadleader_03daa8.s)
/// $040392: move.b #$1e, $78(a6)
pub fn i_040392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40398;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040398: lea.l $4039e(pc), a1
pub fn i_040398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4039C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4039E;
    16
}
/// $04039C: move.l a1, (a6)
pub fn i_04039c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4039E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04039E: jsr $3fece(pc)
pub fn i_04039e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x403A2);
    cpu.pc = 0x3FECE;
    16
}
/// $0403A2: subq.b #$1, $78(a6)
pub fn i_0403a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403A6;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0403A6: btst.b #$0, $78(a6)
pub fn i_0403a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403AC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0403AC: beq.w $403b6
pub fn i_0403ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x403B6; return 10; }
    8
}
/// $0403B0: jsr $28d70.l
pub fn i_0403b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x403B6);
    cpu.pc = 0x28D70;
    16
}
/// $0403B6: tst.b $78(a6)
pub fn i_0403b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403BA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0403BA: bne.w $403c4
pub fn i_0403ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403BE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x403C4; return 10; }
    8
}
/// $0403BE: jmp $518.l
pub fn i_0403be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403C4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0403C4: bra.w $402f0
pub fn i_0403c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x402F0;
    10
}
// ===== Pow_RescuedRts_0403c8  ($0403C8, 16 B, pow/results_squadleader_03daa8.s)
/// $0403C8: movea.l $8(a6), a1
pub fn i_0403c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403CC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0403CC: move.b $10(a6), d0
pub fn i_0403cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403D0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0403D0: cmp.b $10(a1), d0
pub fn i_0403d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403D4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0403D4: bcs.w $403de
pub fn i_0403d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403D8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x403DE; return 10; }
    8
}
// ===== ClearXN_0403d8  ($0403D8, 6 B, thunks/ccr_helpers.c)
/// $0403D8: andi.b #$ee, ccr
pub fn i_0403d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403DC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0403DC: rts
pub fn i_0403dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0403de  ($0403DE, 6 B, thunks/ccr_helpers.c)
/// $0403DE: ori.b #$11, ccr
pub fn i_0403de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403E2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0403E2: rts
pub fn i_0403e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadLeader_Spawn_0403e4  ($0403E4, 176 B, pow/results_squadleader_03daa8.s)
/// $0403E4: move.w #$9a, d1
pub fn i_0403e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403E8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0403E8: jsr $236e.l
pub fn i_0403e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x403EE);
    cpu.pc = 0x236E;
    16
}
/// $0403EE: move.w #$13, $1c(a6)
pub fn i_0403ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x13;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0403F4: jsr $138fe.l
pub fn i_0403f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x403FA);
    cpu.pc = 0x138FE;
    16
}
/// $0403FA: clr.b $7c(a6)
pub fn i_0403fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x403FE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0403FE: move.w #$4000, d0
pub fn i_0403fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40402;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $040402: jsr $28134.l
pub fn i_040402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40408);
    cpu.pc = 0x28134;
    16
}
/// $040408: andi.w #$ffe3, $38(a6)
pub fn i_040408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4040E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04040E: ori.w #$14, $38(a6)
pub fn i_04040e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40414;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $040414: lea.l $2bb7b6.l, a0
pub fn i_040414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4041A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB7B6;
    20
}
/// $04041A: jsr $799de.l
pub fn i_04041a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40420;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40420);
    cpu.pc = 0x799DE;
    16
}
/// $040420: move.w d0, $66(a6)
pub fn i_040420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40424;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040424: lea.l $723d2.l, a1
pub fn i_040424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4042A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x723D2;
    20
}
/// $04042A: jsr $4ae.l
pub fn i_04042a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40430);
    cpu.pc = 0x4AE;
    16
}
/// $040430: jsr $5dd02.l
pub fn i_040430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40436);
    cpu.pc = 0x5DD02;
    16
}
/// $040436: clr.w $98(a0)
pub fn i_040436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4043A;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04043A: jsr $41fb4(pc)
pub fn i_04043a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4043E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4043E);
    cpu.pc = 0x41FB4;
    16
}
/// $04043E: move.b #$80, $80(a6)
pub fn i_04043e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40444;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040444: move.b #$80, $81(a6)
pub fn i_040444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4044A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04044A: move.b #$80, $82(a6)
pub fn i_04044a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40450;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040450: move.b #$80, $83(a6)
pub fn i_040450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40456;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040456: move.b #$80, $84(a6)
pub fn i_040456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4045C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04045C: move.b #$80, $85(a6)
pub fn i_04045c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40462;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040462: move.b #$80, $86(a6)
pub fn i_040462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40468;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040468: move.b #$80, $87(a6)
pub fn i_040468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4046E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04046E: lea.l $2863e8.l, a0
pub fn i_04046e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40474;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2863E8;
    20
}
/// $040474: jsr $28cd4.l
pub fn i_040474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4047A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4047A);
    cpu.pc = 0x28CD4;
    16
}
/// $04047A: move.w #$3c, $70(a6)
pub fn i_04047a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40480;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040480: lea.l $40486(pc), a1
pub fn i_040480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40484;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40486;
    16
}
/// $040484: move.l a1, (a6)
pub fn i_040484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40486;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040486: subq.w #$1, $70(a6)
pub fn i_040486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4048A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04048A: cmpi.w #$0, $70(a6)
pub fn i_04048a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40490;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $040490: bgt.w $4049a
pub fn i_040490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40494;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4049A; return 10; }
    8
}
// ===== SetTaskHandler_040494  ($040494, 8 B, kernel/task_handlers.c)
/// $040494: lea.l $4049c(pc), a1
pub fn i_040494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40498;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4049C;
    16
}
/// $040498: move.l a1, (a6)
pub fn i_040498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4049A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04049A: rts
pub fn i_04049a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4049C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadLeader_Enter_04049c  ($04049C, 76 B, pow/results_squadleader_03daa8.s)
/// $04049C: move.w #$10a2, d0
pub fn i_04049c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404A0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0404A0: jsr $2352.l
pub fn i_0404a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x404A6);
    cpu.pc = 0x2352;
    16
}
/// $0404A6: move.w #$c000, $34(a6)
pub fn i_0404a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404AC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0404AC: move.w #$200, $36(a6)
pub fn i_0404ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404B2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0404B2: move.w #$160, $22(a6)
pub fn i_0404b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0404B8: move.w #$160, $24(a6)
pub fn i_0404b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0404BE: move.w #$f0, $8a(a6)
pub fn i_0404be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0404C4: move.w #$180, $8c(a6)
pub fn i_0404c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404CA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0404CA: lea.l $404d0(pc), a1
pub fn i_0404ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x404D0;
    16
}
/// $0404CE: move.l a1, (a6)
pub fn i_0404ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0404D0: move.w #$14, d5
pub fn i_0404d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404D4;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0404D4: jsr $41ce0(pc)
pub fn i_0404d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x404D8);
    cpu.pc = 0x41CE0;
    16
}
/// $0404D8: bcc.w $404e2
pub fn i_0404d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x404E2; return 10; }
    8
}
/// $0404DC: lea.l $404ee(pc), a1
pub fn i_0404dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x404EE;
    16
}
/// $0404E0: move.l a1, (a6)
pub fn i_0404e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0404E2: jsr $27cee.l
pub fn i_0404e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x404E8);
    cpu.pc = 0x27CEE;
    16
}
// ===== JsrPcThunk_0404e8  ($0404E8, 6 B, thunks/jsr_pc_thunks.c)
/// $0404E8: jsr $41e02(pc)
pub fn i_0404e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x404EC);
    cpu.pc = 0x41E02;
    16
}
/// $0404EC: rts
pub fn i_0404ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadLeader_Turn_0404ee  ($0404EE, 56 B, pow/results_squadleader_03daa8.s)
/// $0404EE: move.w #$200, $36(a6)
pub fn i_0404ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0404F4: lea.l $404fa(pc), a1
pub fn i_0404f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x404FA;
    16
}
/// $0404F8: move.l a1, (a6)
pub fn i_0404f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0404FA: move.w #$ffe0, d1
pub fn i_0404fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x404FE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0404FE: clr.w d0
pub fn i_0404fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40500;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $040500: jsr $41d6c(pc)
pub fn i_040500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40504;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40504);
    cpu.pc = 0x41D6C;
    16
}
/// $040504: bcc.w $4050e
pub fn i_040504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40508;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4050E; return 10; }
    8
}
/// $040508: lea.l $4052c(pc), a1
pub fn i_040508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4050C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4052C;
    16
}
/// $04050C: move.l a1, (a6)
pub fn i_04050c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4050E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04050E: move.w #$8, d5
pub fn i_04050e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40512;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $040512: jsr $41ce0(pc)
pub fn i_040512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40516;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40516);
    cpu.pc = 0x41CE0;
    16
}
/// $040516: bcc.w $40520
pub fn i_040516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4051A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40520; return 10; }
    8
}
/// $04051A: lea.l $4052c(pc), a1
pub fn i_04051a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4051E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4052C;
    16
}
/// $04051E: move.l a1, (a6)
pub fn i_04051e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40520;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040520: jsr $27cee.l
pub fn i_040520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40526;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40526);
    cpu.pc = 0x27CEE;
    16
}
// ===== JsrPcThunk_040526  ($040526, 6 B, thunks/jsr_pc_thunks.c)
/// $040526: jsr $41e02(pc)
pub fn i_040526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4052A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4052A);
    cpu.pc = 0x41E02;
    16
}
/// $04052A: rts
pub fn i_04052a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4052C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadLeader_Hover_04052c  ($04052C, 134 B, pow/results_squadleader_03daa8.s)
/// $04052C: move.w #$100, $36(a6)
pub fn i_04052c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40532;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040532: clr.b $21(a6)
pub fn i_040532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40536;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $040536: clr.w $76(a6)
pub fn i_040536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4053A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04053A: move.w #$2d, $70(a6)
pub fn i_04053a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40540;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040540: lea.l $40546(pc), a1
pub fn i_040540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40544;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40546;
    16
}
/// $040544: move.l a1, (a6)
pub fn i_040544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40546;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040546: cmpi.w #$110, $22(a6)
pub fn i_040546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4054C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x110, Size::Word, false);
    16
}
/// $04054C: ble.w $4056c
pub fn i_04054c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40550;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x4056C; return 10; }
    8
}
/// $040550: move.w #$1, d0
pub fn i_040550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40554;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $040554: jsr $41ce0(pc)
pub fn i_040554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40558;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40558);
    cpu.pc = 0x41CE0;
    16
}
/// $040558: bcc.w $40562
pub fn i_040558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4055C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40562; return 10; }
    8
}
/// $04055C: eori.w #$10, $8c(a6)
pub fn i_04055c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40562;
    cpu.ir = 0xA6E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 ^ 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $040562: jsr $27cee.l
pub fn i_040562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40568;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40568);
    cpu.pc = 0x27CEE;
    16
}
/// $040568: bra.w $40572
pub fn i_040568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4056C;
    cpu.ir = 0x6000;
    cpu.pc = 0x40572;
    10
}
/// $04056C: jsr $2783a.l
pub fn i_04056c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40572;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40572);
    cpu.pc = 0x2783A;
    16
}
/// $040572: jsr $41ddc(pc)
pub fn i_040572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40576;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40576);
    cpu.pc = 0x41DDC;
    16
}
/// $040576: subq.w #$1, $70(a6)
pub fn i_040576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4057A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04057A: cmpi.w #$0, $70(a6)
pub fn i_04057a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40580;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $040580: bne.w $40590
pub fn i_040580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40584;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x40590; return 10; }
    8
}
/// $040584: move.b #$2, $81(a6)
pub fn i_040584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4058A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04058A: move.b #$8, $84(a6)
pub fn i_04058a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40590;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040590: btst.b #$4, $21(a6)
pub fn i_040590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40596;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $040596: beq.w $405ae
pub fn i_040596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4059A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x405AE; return 10; }
    8
}
/// $04059A: move.b #$82, $84(a6)
pub fn i_04059a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405A0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0405A0: move.l #$285d16, $48(a6)
pub fn i_0405a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405A8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x285D16;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0405A8: lea.l $405b2(pc), a1
pub fn i_0405a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x405B2;
    16
}
/// $0405AC: move.l a1, (a6)
pub fn i_0405ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0405AE: bra.w $40b68
pub fn i_0405ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x40B68;
    10
}
// ===== SquadLeader_Formation_0405b2  ($0405B2, 258 B, pow/results_squadleader_03daa8.s)
/// $0405B2: lea.l $2bb8ba.l, a0
pub fn i_0405b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB8BA;
    20
}
/// $0405B8: cmpi.b #$ff, $7c(a6)
pub fn i_0405b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405BE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0405BE: bne.w $405c8
pub fn i_0405be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405C2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x405C8; return 10; }
    8
}
/// $0405C2: lea.l $2bb93c.l, a0
pub fn i_0405c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB93C;
    20
}
/// $0405C8: jsr $799de.l
pub fn i_0405c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x405CE);
    cpu.pc = 0x799DE;
    16
}
/// $0405CE: move.w d0, $70(a6)
pub fn i_0405ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405D2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0405D2: jsr $41c52(pc)
pub fn i_0405d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x405D6);
    cpu.pc = 0x41C52;
    16
}
/// $0405D6: clr.b $21(a6)
pub fn i_0405d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405DA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0405DA: lea.l $405e0(pc), a1
pub fn i_0405da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x405E0;
    16
}
/// $0405DE: move.l a1, (a6)
pub fn i_0405de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0405E0: move.w #$8, d5
pub fn i_0405e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405E4;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0405E4: jsr $41ce0(pc)
pub fn i_0405e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x405E8);
    cpu.pc = 0x41CE0;
    16
}
/// $0405E8: bcc.w $405f0
pub fn i_0405e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x405F0; return 10; }
    8
}
/// $0405EC: jsr $41c52(pc)
pub fn i_0405ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x405F0);
    cpu.pc = 0x41C52;
    16
}
/// $0405F0: jsr $27cee.l
pub fn i_0405f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x405F6);
    cpu.pc = 0x27CEE;
    16
}
/// $0405F6: jsr $41ddc(pc)
pub fn i_0405f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x405FA);
    cpu.pc = 0x41DDC;
    16
}
/// $0405FA: subq.w #$1, $70(a6)
pub fn i_0405fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x405FE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0405FE: cmpi.w #$0, $70(a6)
pub fn i_0405fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40604;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $040604: bgt.w $40622
pub fn i_040604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40608;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x40622; return 10; }
    8
}
/// $040608: cmpi.w #$110, $22(a6)
pub fn i_040608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4060E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x110, Size::Word, false);
    16
}
/// $04060E: bgt.w $40622
pub fn i_04060e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40612;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x40622; return 10; }
    8
}
/// $040612: tst.b $92(a6)
pub fn i_040612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40616;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $040616: bne.w $40622
pub fn i_040616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4061A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x40622; return 10; }
    8
}
/// $04061A: jsr $41e70(pc)
pub fn i_04061a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4061E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4061E);
    cpu.pc = 0x41E70;
    16
}
/// $04061E: jsr $41e4e(pc)
pub fn i_04061e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40622;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40622);
    cpu.pc = 0x41E4E;
    16
}
/// $040622: lea.l $2bb838.l, a0
pub fn i_040622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40628;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BB838;
    20
}
/// $040628: jsr $799de.l
pub fn i_040628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4062E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4062E);
    cpu.pc = 0x799DE;
    16
}
/// $04062E: cmp.w $66(a6), d0
pub fn i_04062e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40632;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $040632: blt.w $4063c
pub fn i_040632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40636;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4063C; return 10; }
    8
}
/// $040636: move.b #$ff, $7c(a6)
pub fn i_040636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4063C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04063C: bra.w $40b68
pub fn i_04063c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40640;
    cpu.ir = 0x6000;
    cpu.pc = 0x40B68;
    10
}
// ===== SquadLeader_Swoop_0406b4  ($0406B4, 74 B, pow/results_squadleader_03daa8.s)
/// $0406B4: lea.l $406ba(pc), a1
pub fn i_0406b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x406BA;
    16
}
/// $0406B8: move.l a1, (a6)
pub fn i_0406b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0406BA: move.w $74(a6), d0
pub fn i_0406ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406BE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0406BE: move.w d0, d1
pub fn i_0406be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406C0;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0406C0: lsr.w #$5, d1
pub fn i_0406c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406C2;
    cpu.ir = 0xEA49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0406C2: neg.w d1
pub fn i_0406c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406C4;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0406C4: clr.w d0
pub fn i_0406c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406C6;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0406C6: jsr $41d6c(pc)
pub fn i_0406c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x406CA);
    cpu.pc = 0x41D6C;
    16
}
/// $0406CA: jsr $41c1a(pc)
pub fn i_0406ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x406CE);
    cpu.pc = 0x41C1A;
    16
}
/// $0406CE: move.w #$4, d5
pub fn i_0406ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406D2;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0406D2: jsr $41ce0(pc)
pub fn i_0406d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x406D6);
    cpu.pc = 0x41CE0;
    16
}
/// $0406D6: bcc.w $406ea
pub fn i_0406d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x406EA; return 10; }
    8
}
/// $0406DA: btst.b #$4, $21(a6)
pub fn i_0406da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406E0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0406E0: beq.w $406ea
pub fn i_0406e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406E4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x406EA; return 10; }
    8
}
/// $0406E4: lea.l $406fe(pc), a1
pub fn i_0406e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x406FE;
    16
}
/// $0406E8: move.l a1, (a6)
pub fn i_0406e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0406EA: jsr $27cee.l
pub fn i_0406ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x406F0);
    cpu.pc = 0x27CEE;
    16
}
/// $0406F0: jsr $28d70.l
pub fn i_0406f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x406F6);
    cpu.pc = 0x28D70;
    16
}
/// $0406F6: clr.w $8e(a6)
pub fn i_0406f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406FA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0406FA: bra.w $40b68
pub fn i_0406fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x406FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x40B68;
    10
}
// ===== SquadLeader_PickAttack_0406fe  ($0406FE, 184 B, pow/results_squadleader_03daa8.s)
/// $0406FE: lea.l $2bbac2.l, a0
pub fn i_0406fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40704;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BBAC2;
    20
}
/// $040704: cmpi.b #$ff, $7c(a6)
pub fn i_040704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4070A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04070A: bne.w $40714
pub fn i_04070a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4070E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x40714; return 10; }
    8
}
/// $04070E: lea.l $2bbb44.l, a0
pub fn i_04070e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40714;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BBB44;
    20
}
/// $040714: jsr $799de.l
pub fn i_040714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4071A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4071A);
    cpu.pc = 0x799DE;
    16
}
/// $04071A: move.w d0, $70(a6)
pub fn i_04071a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4071E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04071E: lea.l $2bbdce.l, a0
pub fn i_04071e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40724;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BBDCE;
    20
}
/// $040724: cmpi.b #$ff, $7c(a6)
pub fn i_040724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4072A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04072A: bne.w $40734
pub fn i_04072a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4072E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x40734; return 10; }
    8
}
/// $04072E: lea.l $2bbe50.l, a0
pub fn i_04072e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40734;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BBE50;
    20
}
/// $040734: jsr $799de.l
pub fn i_040734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4073A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4073A);
    cpu.pc = 0x799DE;
    16
}
/// $04073A: move.w d0, $72(a6)
pub fn i_04073a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4073E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04073E: lea.l $2bbcca.l, a0
pub fn i_04073e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40744;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BBCCA;
    20
}
/// $040744: cmpi.b #$ff, $7c(a6)
pub fn i_040744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4074A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04074A: bne.w $40754
pub fn i_04074a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4074E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x40754; return 10; }
    8
}
/// $04074E: lea.l $2bbd4c.l, a0
pub fn i_04074e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40754;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BBD4C;
    20
}
/// $040754: jsr $799de.l
pub fn i_040754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4075A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4075A);
    cpu.pc = 0x799DE;
    16
}
/// $04075A: move.b d0, $7b(a6)
pub fn i_04075a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4075E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04075E: bset.b #$4, $21(a6)
pub fn i_04075e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40764;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $040764: bra.w $4077c
pub fn i_040764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40768;
    cpu.ir = 0x6000;
    cpu.pc = 0x4077C;
    10
}
/// $040768: clr.b $21(a6)
pub fn i_040768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4076C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04076C: subq.b #$1, $7b(a6)
pub fn i_04076c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40770;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $040770: move.b #$86, $84(a6)
pub fn i_040770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40776;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x86;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040776: move.w $72(a6), $70(a6)
pub fn i_040776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4077C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04077C: lea.l $40782(pc), a1
pub fn i_04077c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40780;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40782;
    16
}
/// $040780: move.l a1, (a6)
pub fn i_040780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40782;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040782: subq.w #$1, $70(a6)
pub fn i_040782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40786;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $040786: cmpi.w #$0, $70(a6)
pub fn i_040786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4078C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $04078C: bgt.w $407a8
pub fn i_04078c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40790;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x407A8; return 10; }
    8
}
/// $040790: btst.b #$4, $21(a6)
pub fn i_040790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40796;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $040796: beq.w $407a8
pub fn i_040796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4079A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x407A8; return 10; }
    8
}
/// $04079A: cmpi.b #$0, $7b(a6)
pub fn i_04079a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407A0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0407A0: bgt.b $40768
pub fn i_0407a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407A2;
    cpu.ir = 0x6EC6;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x40768; return 10; }
    8
}
/// $0407A2: lea.l $407b6(pc), a1
pub fn i_0407a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x407B6;
    16
}
/// $0407A6: move.l a1, (a6)
pub fn i_0407a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0407A8: jsr $2783a.l
pub fn i_0407a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x407AE);
    cpu.pc = 0x2783A;
    16
}
/// $0407AE: jsr $41e02(pc)
pub fn i_0407ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x407B2);
    cpu.pc = 0x41E02;
    16
}
/// $0407B2: bra.w $40b68
pub fn i_0407b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x40B68;
    10
}
// ===== SquadLeader_Dive_0407b6  ($0407B6, 136 B, pow/results_squadleader_03daa8.s)
/// $0407B6: move.w #$c000, $76(a6)
pub fn i_0407b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0407BC: move.w #$100, $8a(a6)
pub fn i_0407bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407C2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0407C2: move.w #$190, $8c(a6)
pub fn i_0407c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0407C8: move.b #$84, $84(a6)
pub fn i_0407c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407CE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0407CE: lea.l $407d4(pc), a1
pub fn i_0407ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x407D4;
    16
}
/// $0407D2: move.l a1, (a6)
pub fn i_0407d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0407D4: move.w #$200, d0
pub fn i_0407d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407D8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0407D8: move.w #$20, d1
pub fn i_0407d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407DC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0407DC: jsr $41d6c(pc)
pub fn i_0407dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x407E0);
    cpu.pc = 0x41D6C;
    16
}
/// $0407E0: jsr $41c1a(pc)
pub fn i_0407e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x407E4);
    cpu.pc = 0x41C1A;
    16
}
/// $0407E4: move.w #$8, d5
pub fn i_0407e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407E8;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0407E8: jsr $41ce0(pc)
pub fn i_0407e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x407EC);
    cpu.pc = 0x41CE0;
    16
}
/// $0407EC: bcc.w $407f6
pub fn i_0407ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x407F6; return 10; }
    8
}
/// $0407F0: lea.l $405b2(pc), a1
pub fn i_0407f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x405B2;
    16
}
/// $0407F4: move.l a1, (a6)
pub fn i_0407f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0407F6: jsr $27cee.l
pub fn i_0407f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x407FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x407FC);
    cpu.pc = 0x27CEE;
    16
}
/// $0407FC: jsr $28d70.l
pub fn i_0407fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40802;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40802);
    cpu.pc = 0x28D70;
    16
}
/// $040802: clr.w $8e(a6)
pub fn i_040802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40806;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $040806: bra.w $40b68
pub fn i_040806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4080A;
    cpu.ir = 0x6000;
    cpu.pc = 0x40B68;
    10
}
// ===== SquadLeader_DiveTurn_04083e  ($04083E, 72 B, pow/results_squadleader_03daa8.s)
/// $04083E: move.w #$c000, $76(a6)
pub fn i_04083e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40844;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040844: move.w #$120, $8a(a6)
pub fn i_040844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4084A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04084A: move.w #$200, $8c(a6)
pub fn i_04084a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40850;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040850: lea.l $40856(pc), a1
pub fn i_040850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40854;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40856;
    16
}
/// $040854: move.l a1, (a6)
pub fn i_040854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40856;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040856: move.w #$200, d0
pub fn i_040856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4085A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04085A: move.w #$20, d1
pub fn i_04085a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4085E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04085E: jsr $41d6c(pc)
pub fn i_04085e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40862;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40862);
    cpu.pc = 0x41D6C;
    16
}
/// $040862: move.w #$10, d5
pub fn i_040862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40866;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $040866: jsr $41ce0(pc)
pub fn i_040866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4086A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4086A);
    cpu.pc = 0x41CE0;
    16
}
/// $04086A: bcc.w $40874
pub fn i_04086a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4086E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40874; return 10; }
    8
}
/// $04086E: lea.l $40886(pc), a1
pub fn i_04086e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40872;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40886;
    16
}
/// $040872: move.l a1, (a6)
pub fn i_040872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40874;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040874: jsr $27cee.l
pub fn i_040874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4087A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4087A);
    cpu.pc = 0x27CEE;
    16
}
/// $04087A: clr.w $8e(a6)
pub fn i_04087a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4087E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04087E: jsr $41e02(pc)
pub fn i_04087e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40882;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40882);
    cpu.pc = 0x41E02;
    16
}
/// $040882: bra.w $40bc4
pub fn i_040882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40886;
    cpu.ir = 0x6000;
    cpu.pc = 0x40BC4;
    10
}
// ===== SquadLeader_Regroup_040886  ($040886, 160 B, pow/results_squadleader_03daa8.s)
/// $040886: lea.l $2bbed2.l, a0
pub fn i_040886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4088C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BBED2;
    20
}
/// $04088C: cmpi.b #$ff, $7c(a6)
pub fn i_04088c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40892;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $040892: bne.w $4089c
pub fn i_040892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40896;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4089C; return 10; }
    8
}
/// $040896: lea.l $2bbf54.l, a0
pub fn i_040896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4089C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BBF54;
    20
}
/// $04089C: jsr $799de.l
pub fn i_04089c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x408A2);
    cpu.pc = 0x799DE;
    16
}
/// $0408A2: lea.l $2bbfd6.l, a0
pub fn i_0408a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BBFD6;
    20
}
/// $0408A8: lsl.w #$2, d0
pub fn i_0408a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408AA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0408AA: move.w (a0, d0.w), $90(a6)
pub fn i_0408aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408B0;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0408B0: addq.w #$2, d0
pub fn i_0408b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408B2;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0408B2: move.w (a0, d0.w), d1
pub fn i_0408b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408B6;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0408B6: move.w d1, $36(a6)
pub fn i_0408b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408BA;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0408BA: neg.w d1
pub fn i_0408ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408BC;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0408BC: move.w d1, $28(a6)
pub fn i_0408bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408C0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0408C0: clr.w $70(a6)
pub fn i_0408c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408C4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0408C4: bset.b #$5, $21(a6)
pub fn i_0408c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408CA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0408CA: lea.l $408d0(pc), a1
pub fn i_0408ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x408D0;
    16
}
/// $0408CE: move.l a1, (a6)
pub fn i_0408ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0408D0: subq.w #$1, $70(a6)
pub fn i_0408d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408D4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0408D4: cmpi.w #$0, $70(a6)
pub fn i_0408d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408DA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0408DA: bgt.w $40908
pub fn i_0408da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408DE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x40908; return 10; }
    8
}
/// $0408DE: btst.b #$5, $21(a6)
pub fn i_0408de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408E4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0408E4: beq.w $40908
pub fn i_0408e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x40908; return 10; }
    8
}
/// $0408E8: move.w $90(a6), $70(a6)
pub fn i_0408e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408EE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0408EE: cmpi.w #$12, $70(a6)
pub fn i_0408ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408F4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x12, Size::Word, false);
    16
}
/// $0408F4: ble.w $40902
pub fn i_0408f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408F8;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x40902; return 10; }
    8
}
/// $0408F8: move.b #$d, $85(a6)
pub fn i_0408f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x408FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0408FE: beq.w $40908
pub fn i_0408fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40902;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x40908; return 10; }
    8
}
/// $040902: move.b #$e, $85(a6)
pub fn i_040902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40908;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040908: jsr $27cee.l
pub fn i_040908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4090E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4090E);
    cpu.pc = 0x27CEE;
    16
}
/// $04090E: jsr $41e02(pc)
pub fn i_04090e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40912;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40912);
    cpu.pc = 0x41E02;
    16
}
/// $040912: cmpi.w #$ffe2, $22(a6)
pub fn i_040912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40918;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFE2, Size::Word, false);
    16
}
/// $040918: bgt.w $40922
pub fn i_040918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4091C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x40922; return 10; }
    8
}
/// $04091C: lea.l $40926(pc), a1
pub fn i_04091c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40920;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40926;
    16
}
/// $040920: move.l a1, (a6)
pub fn i_040920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40922;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040922: bra.w $40bc4
pub fn i_040922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40926;
    cpu.ir = 0x6000;
    cpu.pc = 0x40BC4;
    10
}
// ===== SquadLeader_Reform_040926  ($040926, 214 B, pow/results_squadleader_03daa8.s)
/// $040926: move.b #$80, $82(a6)
pub fn i_040926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4092C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04092C: move.w #$160, $22(a6)
pub fn i_04092c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40932;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040932: move.w #$160, $8a(a6)
pub fn i_040932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40938;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040938: move.w $24(a6), $8c(a6)
pub fn i_040938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4093E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04093E: move.w #$3c, $70(a6)
pub fn i_04093e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40944;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040944: lea.l $4094a(pc), a1
pub fn i_040944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40948;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4094A;
    16
}
/// $040948: move.l a1, (a6)
pub fn i_040948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4094A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04094A: cmpi.w #$1e, $70(a6)
pub fn i_04094a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40950;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Word, false);
    16
}
/// $040950: bgt.w $4095e
pub fn i_040950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40954;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4095E; return 10; }
    8
}
/// $040954: jsr $41c52(pc)
pub fn i_040954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40958;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40958);
    cpu.pc = 0x41C52;
    16
}
/// $040958: lea.l $4095e(pc), a1
pub fn i_040958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4095C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4095E;
    16
}
/// $04095C: move.l a1, (a6)
pub fn i_04095c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4095E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04095E: move.w #$8, d5
pub fn i_04095e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40962;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $040962: jsr $41ce0(pc)
pub fn i_040962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40966;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40966);
    cpu.pc = 0x41CE0;
    16
}
/// $040966: move.b #$80, $85(a6)
pub fn i_040966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4096C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04096C: jsr $27cee.l
pub fn i_04096c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40972;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40972);
    cpu.pc = 0x27CEE;
    16
}
/// $040972: jsr $41ddc(pc)
pub fn i_040972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40976;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40976);
    cpu.pc = 0x41DDC;
    16
}
/// $040976: subq.w #$1, $70(a6)
pub fn i_040976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4097A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04097A: cmpi.w #$0, $70(a6)
pub fn i_04097a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40980;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $040980: bgt.w $409a4
pub fn i_040980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40984;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x409A4; return 10; }
    8
}
/// $040984: lea.l $2bca3e.l, a0
pub fn i_040984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4098A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCA3E;
    20
}
/// $04098A: cmpi.b #$ff, $7c(a6)
pub fn i_04098a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40990;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $040990: bne.w $4099a
pub fn i_040990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40994;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4099A; return 10; }
    8
}
/// $040994: lea.l $2bcac0.l, a0
pub fn i_040994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4099A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCAC0;
    20
}
/// $04099A: jsr $799de.l
pub fn i_04099a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x409A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x409A0);
    cpu.pc = 0x799DE;
    16
}
/// $0409A0: jsr $41e4e(pc)
pub fn i_0409a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x409A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x409A4);
    cpu.pc = 0x41E4E;
    16
}
/// $0409A4: bra.w $40bc4
pub fn i_0409a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x409A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x40BC4;
    10
}
// ===== SquadLeader_Order86_0409fc  ($0409FC, 46 B, pow/results_squadleader_03daa8.s)
/// $0409FC: clr.b $21(a6)
pub fn i_0409fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A00;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $040A00: move.b #$86, $84(a6)
pub fn i_040a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A06;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x86;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040A06: lea.l $40a0c(pc), a1
pub fn i_040a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40A0C;
    16
}
/// $040A0A: move.l a1, (a6)
pub fn i_040a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040A0C: btst.b #$4, $21(a6)
pub fn i_040a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A12;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $040A12: beq.w $40a1c
pub fn i_040a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x40A1C; return 10; }
    8
}
/// $040A16: lea.l $40a2a(pc), a1
pub fn i_040a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40A2A;
    16
}
/// $040A1A: move.l a1, (a6)
pub fn i_040a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A1C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040A1C: jsr $2783a.l
pub fn i_040a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40A22);
    cpu.pc = 0x2783A;
    16
}
/// $040A22: jsr $41ddc(pc)
pub fn i_040a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40A26);
    cpu.pc = 0x41DDC;
    16
}
/// $040A26: bra.w $40b68
pub fn i_040a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A2A;
    cpu.ir = 0x6000;
    cpu.pc = 0x40B68;
    10
}
// ===== SquadLeader_Order84_040a2a  ($040A2A, 60 B, pow/results_squadleader_03daa8.s)
/// $040A2A: clr.b $21(a6)
pub fn i_040a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A2E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $040A2E: move.b #$84, $84(a6)
pub fn i_040a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A34;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040A34: bra.w $405b2
pub fn i_040a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A38;
    cpu.ir = 0x6000;
    cpu.pc = 0x405B2;
    10
}
// ===== SquadLeader_PickOrder_040a66  ($040A66, 64 B, pow/results_squadleader_03daa8.s)
/// $040A66: lea.l $2bc0fa.l, a0
pub fn i_040a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A6C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC0FA;
    20
}
/// $040A6C: cmpi.b #$ff, $7c(a6)
pub fn i_040a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A72;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $040A72: bne.w $40a7c
pub fn i_040a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x40A7C; return 10; }
    8
}
/// $040A76: lea.l $2bc17c.l, a0
pub fn i_040a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC17C;
    20
}
/// $040A7C: jsr $799de.l
pub fn i_040a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40A82);
    cpu.pc = 0x799DE;
    16
}
/// $040A82: move.b d0, $7b(a6)
pub fn i_040a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A86;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040A86: lea.l $2bc322.l, a0
pub fn i_040a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC322;
    20
}
/// $040A8C: cmpi.b #$ff, $7c(a6)
pub fn i_040a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A92;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $040A92: bne.w $40a9c
pub fn i_040a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x40A9C; return 10; }
    8
}
/// $040A96: lea.l $2bc3a4.l, a0
pub fn i_040a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40A9C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC3A4;
    20
}
/// $040A9C: jsr $799de.l
pub fn i_040a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40AA2);
    cpu.pc = 0x799DE;
    16
}
/// $040AA2: move.w d0, $72(a6)
pub fn i_040aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AA6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SquadLeader_Circle_040aa6  ($040AA6, 154 B, pow/results_squadleader_03daa8.s)
/// $040AA6: cmpi.b #$0, $7b(a6)
pub fn i_040aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AAC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $040AAC: ble.w $40ae8
pub fn i_040aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AB0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x40AE8; return 10; }
    8
}
/// $040AB0: subq.b #$1, $7b(a6)
pub fn i_040ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AB4;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $040AB4: move.w $72(a6), $70(a6)
pub fn i_040ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40ABA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $040ABA: move.b #$b, $80(a6)
pub fn i_040aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AC0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040AC0: lea.l $40ac6(pc), a1
pub fn i_040ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40AC6;
    16
}
/// $040AC4: move.l a1, (a6)
pub fn i_040ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040AC6: jsr $2783a.l
pub fn i_040ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40ACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40ACC);
    cpu.pc = 0x2783A;
    16
}
/// $040ACC: jsr $41e02(pc)
pub fn i_040acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40AD0);
    cpu.pc = 0x41E02;
    16
}
/// $040AD0: subq.w #$1, $70(a6)
pub fn i_040ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AD4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $040AD4: cmpi.w #$0, $70(a6)
pub fn i_040ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40ADA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $040ADA: bgt.w $40ae4
pub fn i_040ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40ADE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x40AE4; return 10; }
    8
}
/// $040ADE: lea.l $40aa6(pc), a1
pub fn i_040ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40AA6;
    16
}
/// $040AE2: move.l a1, (a6)
pub fn i_040ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040AE4: bra.w $40b68
pub fn i_040ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x40B68;
    10
}
/// $040AE8: move.b #$1, $80(a6)
pub fn i_040ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AEE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040AEE: lea.l $40af4(pc), a1
pub fn i_040aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40AF4;
    16
}
/// $040AF2: move.l a1, (a6)
pub fn i_040af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040AF4: jsr $2783a.l
pub fn i_040af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40AFA);
    cpu.pc = 0x2783A;
    16
}
/// $040AFA: jsr $41e02(pc)
pub fn i_040afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40AFE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40AFE);
    cpu.pc = 0x41E02;
    16
}
/// $040AFE: btst.b #$0, $21(a6)
pub fn i_040afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B04;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $040B04: beq.w $40b0e
pub fn i_040b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x40B0E; return 10; }
    8
}
/// $040B08: lea.l $405b2(pc), a1
pub fn i_040b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x405B2;
    16
}
/// $040B0C: move.l a1, (a6)
pub fn i_040b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040B0E: bra.w $40b68
pub fn i_040b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B12;
    cpu.ir = 0x6000;
    cpu.pc = 0x40B68;
    10
}
/// $040B32: jsr $2783a.l
pub fn i_040b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40B38);
    cpu.pc = 0x2783A;
    16
}
/// $040B38: jsr $41ddc(pc)
pub fn i_040b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40B3C);
    cpu.pc = 0x41DDC;
    16
}
/// $040B3C: bra.w $40b68
pub fn i_040b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B40;
    cpu.ir = 0x6000;
    cpu.pc = 0x40B68;
    10
}
// ===== SquadLeader_HitCheck_040b40  ($040B40, 166 B, pow/results_squadleader_03daa8.s)
/// $040B40: clr.b $21(a6)
pub fn i_040b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B44;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $040B44: move.b #$81, $83(a6)
pub fn i_040b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B4A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x81;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040B4A: lea.l $40b50(pc), a1
pub fn i_040b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40B50;
    16
}
/// $040B4E: move.l a1, (a6)
pub fn i_040b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B50;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040B50: btst.b #$3, $21(a6)
pub fn i_040b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B56;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $040B56: beq.w $40b66
pub fn i_040b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B5A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x40B66; return 10; }
    8
}
/// $040B5A: move.b #$80, $83(a6)
pub fn i_040b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B60;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040B60: lea.l $405b2(pc), a1
pub fn i_040b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x405B2;
    16
}
/// $040B64: move.l a1, (a6)
pub fn i_040b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040B66: bra.b $40b32
pub fn i_040b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B68;
    cpu.ir = 0x60CA;
    cpu.pc = 0x40B32;
    10
}
/// $040B68: clr.b $92(a6)
pub fn i_040b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B6C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x92);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $040B6C: jsr $2870a.l
pub fn i_040b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40B72);
    cpu.pc = 0x2870A;
    16
}
/// $040B72: bclr.b #$3, $13(a6)
pub fn i_040b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B78;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $040B78: lea.l $5e766.l, a0
pub fn i_040b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $040B7E: jsr $5e770.l
pub fn i_040b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40B84);
    cpu.pc = 0x5E770;
    16
}
/// $040B84: jsr $28758.l
pub fn i_040b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40B8A);
    cpu.pc = 0x28758;
    16
}
/// $040B8A: bcc.w $40bc2
pub fn i_040b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40BC2; return 10; }
    8
}
/// $040B8E: cmpi.w #$90, $22(a6)
pub fn i_040b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B94;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x90, Size::Word, false);
    16
}
/// $040B94: blt.w $40bb6
pub fn i_040b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B98;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x40BB6; return 10; }
    8
}
/// $040B98: cmpi.w #$120, $22(a6)
pub fn i_040b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40B9E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $040B9E: bgt.w $40bb6
pub fn i_040b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BA2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x40BB6; return 10; }
    8
}
/// $040BA2: jsr $5e366.l
pub fn i_040ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40BA8);
    cpu.pc = 0x5E366;
    16
}
/// $040BA8: bcs.w $40bb6
pub fn i_040ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BAC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x40BB6; return 10; }
    8
}
/// $040BAC: lea.l $40be6(pc), a1
pub fn i_040bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40BE6;
    16
}
/// $040BB0: move.l a1, (a6)
pub fn i_040bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040BB2: bra.w $40bc2
pub fn i_040bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x40BC2;
    10
}
/// $040BB6: move.w #$1, $66(a6)
pub fn i_040bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BBC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040BBC: bclr.b #$0, $13(a6)
pub fn i_040bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BC2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $040BC2: rts
pub fn i_040bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BC4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $040BC4: jsr $2870a.l
pub fn i_040bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40BCA);
    cpu.pc = 0x2870A;
    16
}
/// $040BCA: bclr.b #$3, $13(a6)
pub fn i_040bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BD0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $040BD0: lea.l $5e766.l, a0
pub fn i_040bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BD6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $040BD6: jsr $5e770.l
pub fn i_040bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40BDC);
    cpu.pc = 0x5E770;
    16
}
/// $040BDC: jsr $28758.l
pub fn i_040bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40BE2);
    cpu.pc = 0x28758;
    16
}
/// $040BE2: bcc.b $40bc2
pub fn i_040be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BE4;
    cpu.ir = 0x64DE;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40BC2; return 10; }
    8
}
/// $040BE4: bra.b $40bb6
pub fn i_040be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BE6;
    cpu.ir = 0x60D0;
    cpu.pc = 0x40BB6;
    10
}
// ===== SquadLeader_Death_040be6  ($040BE6, 298 B, pow/results_squadleader_03daa8.s)
/// $040BE6: clr.b $106ed3.l
pub fn i_040be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BEC;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED3) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED3, r2 as u8);
    16
}
/// $040BEC: move.b #$0, $10a2d0.l
pub fn i_040bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BF4;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D0, v1 as u8);
    16
}
/// $040BF4: move.b #$0, $10a2d1.l
pub fn i_040bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40BFC;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $040BFC: move.l #$ffffffff, $48(a6)
pub fn i_040bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C04;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $040C04: move.b #$80, $84(a6)
pub fn i_040c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C0A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040C0A: lea.l $41408(pc), a1
pub fn i_040c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41408;
    16
}
/// $040C0E: jsr $4ae.l
pub fn i_040c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C14);
    cpu.pc = 0x4AE;
    16
}
/// $040C14: jsr $5dd02.l
pub fn i_040c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C1A);
    cpu.pc = 0x5DD02;
    16
}
/// $040C1A: lea.l $41586(pc), a1
pub fn i_040c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41586;
    16
}
/// $040C1E: jsr $4ae.l
pub fn i_040c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C24);
    cpu.pc = 0x4AE;
    16
}
/// $040C24: jsr $5dd02.l
pub fn i_040c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C2A);
    cpu.pc = 0x5DD02;
    16
}
/// $040C2A: lea.l $8121c.l, a1
pub fn i_040c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C30;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x8121C;
    20
}
/// $040C30: jsr $4ae.l
pub fn i_040c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C36);
    cpu.pc = 0x4AE;
    16
}
/// $040C36: jsr $5dd02.l
pub fn i_040c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C3C);
    cpu.pc = 0x5DD02;
    16
}
/// $040C3C: move.w #$c000, $38(a0)
pub fn i_040c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C42;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040C42: lea.l $8123c.l, a1
pub fn i_040c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C48;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x8123C;
    20
}
/// $040C48: jsr $4ae.l
pub fn i_040c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C4E);
    cpu.pc = 0x4AE;
    16
}
/// $040C4E: jsr $5dd02.l
pub fn i_040c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C54);
    cpu.pc = 0x5DD02;
    16
}
/// $040C54: move.w #$c000, $38(a0)
pub fn i_040c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C5A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040C5A: lea.l $81260.l, a1
pub fn i_040c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C60;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x81260;
    20
}
/// $040C60: jsr $4ae.l
pub fn i_040c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C66);
    cpu.pc = 0x4AE;
    16
}
/// $040C66: jsr $5dd02.l
pub fn i_040c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C6C);
    cpu.pc = 0x5DD02;
    16
}
/// $040C6C: move.w #$c000, $38(a0)
pub fn i_040c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C72;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040C72: lea.l $81284.l, a1
pub fn i_040c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C78;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x81284;
    20
}
/// $040C78: jsr $4ae.l
pub fn i_040c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C7E);
    cpu.pc = 0x4AE;
    16
}
/// $040C7E: jsr $5dd02.l
pub fn i_040c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C84);
    cpu.pc = 0x5DD02;
    16
}
/// $040C84: move.w #$c000, $38(a0)
pub fn i_040c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C8A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040C8A: move.w #$1e, $70(a6)
pub fn i_040c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C90;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040C90: jsr $267e2.l
pub fn i_040c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40C96);
    cpu.pc = 0x267E2;
    16
}
/// $040C96: lea.l $40c9c(pc), a1
pub fn i_040c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40C9C;
    16
}
/// $040C9A: move.l a1, (a6)
pub fn i_040c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40C9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040C9C: movea.l #$ffffffff, a0
pub fn i_040c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CA2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $040CA2: lea.l $286102.l, a0
pub fn i_040ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x286102;
    20
}
/// $040CA8: jsr $5dd5c.l
pub fn i_040ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40CAE);
    cpu.pc = 0x5DD5C;
    16
}
/// $040CAE: bcc.w $40cc2
pub fn i_040cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40CC2; return 10; }
    8
}
/// $040CB2: move.w #$10a2, d0
pub fn i_040cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CB6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $040CB6: jsr $2222.l
pub fn i_040cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40CBC);
    cpu.pc = 0x2222;
    16
}
/// $040CBC: lea.l $40cc2(pc), a1
pub fn i_040cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40CC2;
    16
}
/// $040CC0: move.l a1, (a6)
pub fn i_040cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040CC2: jsr $27d50.l
pub fn i_040cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40CC8);
    cpu.pc = 0x27D50;
    16
}
/// $040CC8: bcc.w $40cd2
pub fn i_040cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CCC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40CD2; return 10; }
    8
}
/// $040CCC: lea.l $40d18(pc), a1
pub fn i_040ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40D18;
    16
}
/// $040CD0: move.l a1, (a6)
pub fn i_040cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040CD2: jsr $41db6(pc)
pub fn i_040cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40CD6);
    cpu.pc = 0x41DB6;
    16
}
/// $040CD6: subq.w #$1, $70(a6)
pub fn i_040cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CDA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $040CDA: cmpi.w #$0, $70(a6)
pub fn i_040cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CE0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $040CE0: bne.w $40d06
pub fn i_040ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x40D06; return 10; }
    8
}
/// $040CE4: move.w #$c0, $28(a6)
pub fn i_040ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CEA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040CEA: move.w #$ffc0, $2a(a6)
pub fn i_040cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CF0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040CF0: move.w #$1032, d0
pub fn i_040cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CF4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1032;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $040CF4: jsr $2352.l
pub fn i_040cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40CFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40CFA);
    cpu.pc = 0x2352;
    16
}
/// $040CFA: lea.l $286112.l, a1
pub fn i_040cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D00;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x286112;
    20
}
/// $040D00: jsr $77c7e.l
pub fn i_040d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40D06);
    cpu.pc = 0x77C7E;
    16
}
/// $040D06: cmpi.w #$120, $22(a6)
pub fn i_040d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D0C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $040D0C: blt.w $40d16
pub fn i_040d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D10;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x40D16; return 10; }
    8
}
// ===== SetTaskHandler_040d10  ($040D10, 8 B, kernel/task_handlers.c)
/// $040D10: lea.l $40d18(pc), a1
pub fn i_040d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40D18;
    16
}
/// $040D14: move.l a1, (a6)
pub fn i_040d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040D16: rts
pub fn i_040d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadLeader_DeathDone_040d18  ($040D18, 84 B, pow/results_squadleader_03daa8.s)
/// $040D18: clr.b $106ed2.l
pub fn i_040d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D1E;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $040D1E: jsr $434ce.l
pub fn i_040d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40D24);
    cpu.pc = 0x434CE;
    16
}
/// $040D24: move.w $22(a6), $5c(a6)
pub fn i_040d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D2A;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $040D2A: move.w #$120, $22(a6)
pub fn i_040d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D30;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040D30: move.w #$1033, d0
pub fn i_040d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D34;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $040D34: jsr $2352.l
pub fn i_040d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40D3A);
    cpu.pc = 0x2352;
    16
}
/// $040D3A: move.b #$4, $10a2d0.l
pub fn i_040d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D42;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D0, v1 as u8);
    16
}
/// $040D42: move.b #$0, $10a2d1.l
pub fn i_040d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D4A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $040D4A: lea.l $77fd6.l, a1
pub fn i_040d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D50;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $040D50: jsr $4ae.l
pub fn i_040d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40D56);
    cpu.pc = 0x4AE;
    16
}
/// $040D56: jsr $5dd02.l
pub fn i_040d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40D5C);
    cpu.pc = 0x5DD02;
    16
}
/// $040D5C: jsr $628ea.l
pub fn i_040d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40D62);
    cpu.pc = 0x628EA;
    16
}
/// $040D62: move.w $5c(a6), $22(a6)
pub fn i_040d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D68;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $040D68: bra.w $40eba
pub fn i_040d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x40EBA;
    10
}
// ===== SquadLeader_Respawn_040d6c  ($040D6C, 156 B, pow/results_squadleader_03daa8.s)
/// $040D6C: move.w #$9a, d1
pub fn i_040d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D70;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $040D70: jsr $236e.l
pub fn i_040d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40D76);
    cpu.pc = 0x236E;
    16
}
/// $040D76: move.w #$4000, d0
pub fn i_040d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D7A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $040D7A: jsr $28134.l
pub fn i_040d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40D80);
    cpu.pc = 0x28134;
    16
}
/// $040D80: andi.w #$ffe3, $38(a6)
pub fn i_040d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D86;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $040D86: ori.w #$14, $38(a6)
pub fn i_040d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D8C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $040D8C: jsr $41fb4(pc)
pub fn i_040d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D90;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40D90);
    cpu.pc = 0x41FB4;
    16
}
/// $040D90: move.b #$80, $80(a6)
pub fn i_040d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D96;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040D96: move.b #$80, $81(a6)
pub fn i_040d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40D9C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040D9C: move.b #$80, $82(a6)
pub fn i_040d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DA2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040DA2: move.b #$80, $83(a6)
pub fn i_040da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DA8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040DA8: move.b #$80, $84(a6)
pub fn i_040da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DAE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040DAE: move.b #$80, $85(a6)
pub fn i_040dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DB4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040DB4: move.b #$80, $86(a6)
pub fn i_040db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DBA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040DBA: move.b #$80, $87(a6)
pub fn i_040dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DC0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040DC0: move.w #$400, $36(a6)
pub fn i_040dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DC6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040DC6: lea.l $2863e8.l, a0
pub fn i_040dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DCC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2863E8;
    20
}
/// $040DCC: jsr $28cd4.l
pub fn i_040dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40DD2);
    cpu.pc = 0x28CD4;
    16
}
/// $040DD2: clr.w $8e(a6)
pub fn i_040dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DD6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $040DD6: move.w #$180, $22(a6)
pub fn i_040dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DDC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040DDC: move.w #$1e0, $24(a6)
pub fn i_040ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DE2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040DE2: move.l #$288382, $90(a6)
pub fn i_040de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DEA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x288382;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $040DEA: clr.w $94(a6)
pub fn i_040dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DEE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $040DEE: clr.b $96(a6)
pub fn i_040dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DF2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x96);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $040DF2: lea.l $40df8(pc), a1
pub fn i_040df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40DF8;
    16
}
/// $040DF6: move.l a1, (a6)
pub fn i_040df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040DF8: jsr $78f8a.l
pub fn i_040df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40DFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40DFE);
    cpu.pc = 0x78F8A;
    16
}
/// $040DFE: bcc.w $40e08
pub fn i_040dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E02;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40E08; return 10; }
    8
}
/// $040E02: lea.l $40e0e(pc), a1
pub fn i_040e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E06;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40E0E;
    16
}
/// $040E06: move.l a1, (a6)
pub fn i_040e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E08;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_040e08  ($040E08, 6 B, thunks/jsr_pc_thunks.c)
/// $040E08: jsr $41ddc(pc)
pub fn i_040e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40E0C);
    cpu.pc = 0x41DDC;
    16
}
/// $040E0C: rts
pub fn i_040e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadLeader_RespawnWait_040e0e  ($040E0E, 62 B, pow/results_squadleader_03daa8.s)
/// $040E0E: clr.b $21(a6)
pub fn i_040e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E12;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $040E12: move.w #$2d, $70(a6)
pub fn i_040e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E18;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040E18: move.b #$2, $81(a6)
pub fn i_040e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E1E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040E1E: move.b #$8, $84(a6)
pub fn i_040e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E24;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040E24: lea.l $40e2a(pc), a1
pub fn i_040e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40E2A;
    16
}
/// $040E28: move.l a1, (a6)
pub fn i_040e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E2A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040E2A: jsr $2783a.l
pub fn i_040e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40E30);
    cpu.pc = 0x2783A;
    16
}
/// $040E30: jsr $41ddc(pc)
pub fn i_040e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40E34);
    cpu.pc = 0x41DDC;
    16
}
/// $040E34: subq.w #$1, $70(a6)
pub fn i_040e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E38;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $040E38: cmpi.w #$0, $70(a6)
pub fn i_040e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E3E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $040E3E: bgt.w $40e52
pub fn i_040e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E42;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x40E52; return 10; }
    8
}
/// $040E42: btst.b #$4, $21(a6)
pub fn i_040e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E48;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $040E48: beq.w $40e52
pub fn i_040e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E4C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x40E52; return 10; }
    8
}
// ===== SetTaskHandler_040e4c  ($040E4C, 8 B, kernel/task_handlers.c)
/// $040E4C: lea.l $40e54(pc), a1
pub fn i_040e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40E54;
    16
}
/// $040E50: move.l a1, (a6)
pub fn i_040e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040E52: rts
pub fn i_040e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadLeader_Order88_040e54  ($040E54, 94 B, pow/results_squadleader_03daa8.s)
/// $040E54: move.w #$f, $70(a6)
pub fn i_040e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E5A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040E5A: clr.b $21(a6)
pub fn i_040e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E5E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $040E5E: move.b #$88, $84(a6)
pub fn i_040e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E64;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040E64: lea.l $40e6a(pc), a1
pub fn i_040e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40E6A;
    16
}
/// $040E68: move.l a1, (a6)
pub fn i_040e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040E6A: btst.b #$4, $21(a6)
pub fn i_040e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E70;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $040E70: beq.w $40e92
pub fn i_040e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E74;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x40E92; return 10; }
    8
}
/// $040E74: subq.w #$1, $70(a6)
pub fn i_040e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E78;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $040E78: cmpi.w #$0, $70(a6)
pub fn i_040e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E7E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $040E7E: bgt.w $40e92
pub fn i_040e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E82;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x40E92; return 10; }
    8
}
/// $040E82: clr.b $21(a6)
pub fn i_040e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E86;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $040E86: move.b #$89, $84(a6)
pub fn i_040e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E8C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x89;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040E8C: lea.l $40e92(pc), a1
pub fn i_040e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40E92;
    16
}
/// $040E90: move.l a1, (a6)
pub fn i_040e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040E92: jsr $2783a.l
pub fn i_040e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40E98);
    cpu.pc = 0x2783A;
    16
}
/// $040E98: jsr $41ddc(pc)
pub fn i_040e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40E9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40E9C);
    cpu.pc = 0x41DDC;
    16
}
/// $040E9C: movea.l #$ffffffff, a0
pub fn i_040e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EA2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $040EA2: lea.l $286102.l, a0
pub fn i_040ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x286102;
    20
}
/// $040EA8: jsr $5dd56.l
pub fn i_040ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40EAE);
    cpu.pc = 0x5DD56;
    16
}
/// $040EAE: bcc.w $40eb8
pub fn i_040eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40EB8; return 10; }
    8
}
// ===== SetTaskHandler_040eb2  ($040EB2, 8 B, kernel/task_handlers.c)
/// $040EB2: lea.l $40ef2(pc), a1
pub fn i_040eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $040EB6: move.l a1, (a6)
pub fn i_040eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040EB8: rts
pub fn i_040eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadLeader_Order8A_040eba  ($040EBA, 54 B, pow/results_squadleader_03daa8.s)
/// $040EBA: move.b #$8a, $80(a6)
pub fn i_040eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EC0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040EC0: move.b #$8a, $81(a6)
pub fn i_040ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EC6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040EC6: move.b #$8a, $82(a6)
pub fn i_040ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40ECC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040ECC: move.b #$8a, $83(a6)
pub fn i_040ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40ED2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040ED2: move.b #$8a, $84(a6)
pub fn i_040ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40ED8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040ED8: move.b #$8a, $85(a6)
pub fn i_040ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EDE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040EDE: move.b #$8a, $86(a6)
pub fn i_040ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EE4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040EE4: move.b #$8a, $87(a6)
pub fn i_040ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EEA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8A;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040EEA: jmp $518.l
pub fn i_040eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EF0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_040ef0  ($040EF0, 2 B, pow/results_squadleader_03daa8.s)
/// $040EF0: rts
pub fn i_040ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_040ef2  ($040EF2, 14 B, kernel/jsr5b6_jmp518.c)
/// $040EF2: jsr $5b6.l
pub fn i_040ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40EF8);
    cpu.pc = 0x5B6;
    16
}
/// $040EF8: jmp $518.l
pub fn i_040ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40EFE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== SquadMember_Handler_040F00  ($040F00, 130 B, enemies/squad/member_states_040f00.s)
/// $040F00: move.b $85(a6), d0
pub fn i_040f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F04;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $040F04: andi.w #$7, d0
pub fn i_040f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F08;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $040F08: add.w d0, d0
pub fn i_040f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F0A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $040F0A: lea.l $28617c.l, a0
pub fn i_040f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28617C;
    20
}
/// $040F10: move.w (a0, d0.w), d1
pub fn i_040f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F14;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $040F14: jsr $236e.l
pub fn i_040f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40F1A);
    cpu.pc = 0x236E;
    16
}
/// $040F1A: move.w #$13, $1c(a6)
pub fn i_040f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F20;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x13;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $040F20: jsr $138fe.l
pub fn i_040f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40F26);
    cpu.pc = 0x138FE;
    16
}
/// $040F26: move.b #$80, $80(a6)
pub fn i_040f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F2C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040F2C: move.b #$80, $81(a6)
pub fn i_040f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F32;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040F32: move.b $85(a6), d0
pub fn i_040f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F36;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $040F36: andi.w #$7, d0
pub fn i_040f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F3A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $040F3A: movea.l #$28635c, a0
pub fn i_040f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F40;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28635C;
    cpu.a[0] = v1;
    4
}
/// $040F40: lsl.w #$2, d0
pub fn i_040f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F42;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $040F42: movea.l (a0, d0.w), a0
pub fn i_040f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F46;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $040F46: cmpa.l #$ffffffff, a0
pub fn i_040f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F4C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $040F4C: beq.w $40f56
pub fn i_040f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F50;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x40F56; return 10; }
    8
}
/// $040F50: jsr $28cd4.l
pub fn i_040f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40F56);
    cpu.pc = 0x28CD4;
    16
}
/// $040F56: lea.l $40f5c(pc), a1
pub fn i_040f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40F5C;
    16
}
/// $040F5A: move.l a1, (a6)
pub fn i_040f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040F5C: jsr $41f84(pc)
pub fn i_040f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40F60);
    cpu.pc = 0x41F84;
    16
}
/// $040F60: movea.l $c(a6), a0
pub fn i_040f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F64;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $040F64: btst.b #$7, $5a(a0)
pub fn i_040f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F6A;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $040F6A: beq.w $40f74
pub fn i_040f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F6E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x40F74; return 10; }
    8
}
/// $040F6E: bset.b #$0, $5a(a6)
pub fn i_040f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F74;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $040F74: jsr $28d70.l
pub fn i_040f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40F7A);
    cpu.pc = 0x28D70;
    16
}
/// $040F7A: jsr $41ff6(pc)
pub fn i_040f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F7E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40F7E);
    cpu.pc = 0x41FF6;
    16
}
/// $040F7E: bra.w $41316
pub fn i_040f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F82;
    cpu.ir = 0x6000;
    cpu.pc = 0x41316;
    10
}
// ===== SquadMember_OnStateChange_040F82  ($040F82, 108 B, enemies/squad/member_states_040f00.s)
/// $040F82: clr.b $86(a6)
pub fn i_040f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F86;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $040F86: move.b $80(a6), d0
pub fn i_040f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F8A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $040F8A: andi.w #$f, d0
pub fn i_040f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F8E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $040F8E: movea.l #$28637c, a0
pub fn i_040f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F94;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28637C;
    cpu.a[0] = v1;
    4
}
/// $040F94: lsl.w #$2, d0
pub fn i_040f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F96;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $040F96: movea.l (a0, d0.w), a0
pub fn i_040f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40F9A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $040F9A: cmpa.l #$ffffffff, a0
pub fn i_040f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FA0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $040FA0: beq.w $40faa
pub fn i_040fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FA4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x40FAA; return 10; }
    8
}
/// $040FA4: jsr $28cd4.l
pub fn i_040fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40FAA);
    cpu.pc = 0x28CD4;
    16
}
/// $040FAA: lea.l $40fb0(pc), a1
pub fn i_040faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40FB0;
    16
}
/// $040FAE: move.l a1, (a6)
pub fn i_040fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $040FB0: jsr $41f84(pc)
pub fn i_040fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40FB4);
    cpu.pc = 0x41F84;
    16
}
/// $040FB4: movea.l $c(a6), a0
pub fn i_040fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FB8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $040FB8: btst.b #$7, $5a(a0)
pub fn i_040fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FBE;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $040FBE: beq.w $40fc8
pub fn i_040fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FC2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x40FC8; return 10; }
    8
}
/// $040FC2: bset.b #$0, $5a(a6)
pub fn i_040fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FC8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $040FC8: jsr $28d70.l
pub fn i_040fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x40FCE);
    cpu.pc = 0x28D70;
    16
}
/// $040FCE: bcc.w $40fea
pub fn i_040fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FD2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40FEA; return 10; }
    8
}
/// $040FD2: jsr $41ff6(pc)
pub fn i_040fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40FD6);
    cpu.pc = 0x41FF6;
    16
}
/// $040FD6: cmpi.b #$0, $86(a6)
pub fn i_040fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FDC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $040FDC: bne.w $40fea
pub fn i_040fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x40FEA; return 10; }
    8
}
/// $040FE0: move.b #$ff, $86(a6)
pub fn i_040fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $040FE6: jsr $41f3a(pc)
pub fn i_040fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x40FEA);
    cpu.pc = 0x41F3A;
    16
}
/// $040FEA: bra.w $41316
pub fn i_040fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40FEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x41316;
    10
}
// ===== SquadMember_SetPose2_0410BA  ($0410BA, 10 B, enemies/squad/member_states_040f00.s)
/// $0410BA: move.w #$2, $76(a6)
pub fn i_0410ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0410C0: bra.w $410ee
pub fn i_0410c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x410EE;
    10
}
// ===== SquadMember_HoldPose_0410C4  ($0410C4, 42 B, enemies/squad/member_states_040f00.s)
/// $0410C4: bclr.b #$3, $13(a6)
pub fn i_0410c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410CA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0410CA: lea.l $286f74.l, a0
pub fn i_0410ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x286F74;
    20
}
/// $0410D0: jsr $28cd4.l
pub fn i_0410d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x410D6);
    cpu.pc = 0x28CD4;
    16
}
/// $0410D6: lea.l $410dc(pc), a1
pub fn i_0410d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x410DC;
    16
}
/// $0410DA: move.l a1, (a6)
pub fn i_0410da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0410DC: jsr $41f84(pc)
pub fn i_0410dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x410E0);
    cpu.pc = 0x41F84;
    16
}
/// $0410E0: jsr $28d70.l
pub fn i_0410e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x410E6);
    cpu.pc = 0x28D70;
    16
}
/// $0410E6: jsr $41ff6(pc)
pub fn i_0410e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x410EA);
    cpu.pc = 0x41FF6;
    16
}
/// $0410EA: bra.w $412fc
pub fn i_0410ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x412FC;
    10
}
// ===== SquadMember_AimTrackTarget_0410EE  ($0410EE, 168 B, enemies/squad/member_states_040f00.s)
/// $0410EE: move.w #$a, $78(a6)
pub fn i_0410ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0410F4: lea.l $286f74.l, a0
pub fn i_0410f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x410FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x286F74;
    20
}
/// $0410FA: jsr $28cd4.l
pub fn i_0410fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41100;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41100);
    cpu.pc = 0x28CD4;
    16
}
/// $041100: jsr $5e0d4.l
pub fn i_041100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41106;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41106);
    cpu.pc = 0x5E0D4;
    16
}
/// $041106: movea.l $c(a6), a1
pub fn i_041106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4110A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04110A: move.l a0, $9c(a1)
pub fn i_04110a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4110E;
    cpu.ir = 0x2348;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x9C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04110E: move.l a0, $9c(a6)
pub fn i_04110e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41112;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $041112: bclr.b #$3, $13(a6)
pub fn i_041112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41118;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $041118: lea.l $4111e(pc), a1
pub fn i_041118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4111C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4111E;
    16
}
/// $04111C: move.l a1, (a6)
pub fn i_04111c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4111E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04111E: jsr $41f84(pc)
pub fn i_04111e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41122;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41122);
    cpu.pc = 0x41F84;
    16
}
/// $041122: movea.l $c(a6), a0
pub fn i_041122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41126;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $041126: move.b #$0, $7d(a0)
pub fn i_041126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4112C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04112C: addq.w #$1, $70(a6)
pub fn i_04112c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41130;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $041130: andi.w #$1, $70(a6)
pub fn i_041130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41136;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $041136: bne.w $41188
pub fn i_041136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4113A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x41188; return 10; }
    8
}
/// $04113A: movea.l $9c(a6), a0
pub fn i_04113a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4113E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04113E: move.w $22(a0), d0
pub fn i_04113e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41142;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041142: move.w $24(a0), d1
pub fn i_041142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41146;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041146: sub.w $22(a6), d0
pub fn i_041146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4114A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04114A: sub.w $24(a6), d1
pub fn i_04114a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4114E;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04114E: addi.w #$20, d0
pub fn i_04114e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41152;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $041152: jsr $5e018.l
pub fn i_041152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41158;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41158);
    cpu.pc = 0x5E018;
    16
}
/// $041158: jsr $4206a(pc)
pub fn i_041158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4115C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4115C);
    cpu.pc = 0x4206A;
    16
}
/// $04115C: move.w d0, $78(a6)
pub fn i_04115c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41160;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041160: move.w #$ffff, d1
pub fn i_041160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41164;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041164: move.w $78(a6), d0
pub fn i_041164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41168;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041168: cmp.w $76(a6), d0
pub fn i_041168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4116C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $04116C: beq.w $4117e
pub fn i_04116c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41170;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4117E; return 10; }
    8
}
/// $041170: bcs.w $41176
pub fn i_041170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41174;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x41176; return 10; }
    8
}
/// $041174: neg.w d1
pub fn i_041174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41176;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $041176: add.w d1, $76(a6)
pub fn i_041176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4117A;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04117A: bra.w $41188
pub fn i_04117a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4117E;
    cpu.ir = 0x6000;
    cpu.pc = 0x41188;
    10
}
/// $04117E: movea.l $c(a6), a0
pub fn i_04117e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41182;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $041182: move.b #$1, $7d(a0)
pub fn i_041182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41188;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041188: jsr $28d70.l
pub fn i_041188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4118E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4118E);
    cpu.pc = 0x28D70;
    16
}
/// $04118E: jsr $41ff6(pc)
pub fn i_04118e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41192;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41192);
    cpu.pc = 0x41FF6;
    16
}
/// $041192: bra.w $412fc
pub fn i_041192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41196;
    cpu.ir = 0x6000;
    cpu.pc = 0x412FC;
    10
}
// ===== SquadMember_AlignHeading_041196  ($041196, 98 B, enemies/squad/member_states_040f00.s)
/// $041196: bclr.b #$3, $13(a6)
pub fn i_041196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4119C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04119C: clr.w $70(a6)
pub fn i_04119c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411A0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0411A0: clr.b $86(a6)
pub fn i_0411a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411A4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0411A4: move.w #$2, $78(a6)
pub fn i_0411a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411AA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0411AA: lea.l $286f74.l, a0
pub fn i_0411aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x286F74;
    20
}
/// $0411B0: jsr $28cd4.l
pub fn i_0411b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x411B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0411B6: lea.l $411bc(pc), a1
pub fn i_0411b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x411BC;
    16
}
/// $0411BA: move.l a1, (a6)
pub fn i_0411ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0411BC: jsr $41f84(pc)
pub fn i_0411bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x411C0);
    cpu.pc = 0x41F84;
    16
}
/// $0411C0: jsr $28d70.l
pub fn i_0411c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x411C6);
    cpu.pc = 0x28D70;
    16
}
/// $0411C6: addq.w #$1, $70(a6)
pub fn i_0411c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411CA;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0411CA: andi.w #$1, $70(a6)
pub fn i_0411ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411D0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0411D0: bne.w $411f0
pub fn i_0411d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x411F0; return 10; }
    8
}
/// $0411D4: jsr $41f50(pc)
pub fn i_0411d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x411D8);
    cpu.pc = 0x41F50;
    16
}
/// $0411D8: bcc.w $411f0
pub fn i_0411d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x411F0; return 10; }
    8
}
/// $0411DC: cmpi.b #$0, $86(a6)
pub fn i_0411dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411E2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0411E2: bne.w $411f0
pub fn i_0411e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x411F0; return 10; }
    8
}
/// $0411E6: move.b #$ff, $86(a6)
pub fn i_0411e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411EC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0411EC: jsr $41f3a(pc)
pub fn i_0411ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x411F0);
    cpu.pc = 0x41F3A;
    16
}
/// $0411F0: jsr $41ff6(pc)
pub fn i_0411f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x411F4);
    cpu.pc = 0x41FF6;
    16
}
/// $0411F4: bra.w $412fc
pub fn i_0411f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x412FC;
    10
}
// ===== SquadMember_PoseFromHeading_0411F8  ($0411F8, 134 B, enemies/squad/member_states_040f00.s)
/// $0411F8: bclr.b #$3, $13(a6)
pub fn i_0411f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x411FE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0411FE: clr.b $86(a6)
pub fn i_0411fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41202;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $041202: move.l #$ffffffff, $48(a6)
pub fn i_041202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4120A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04120A: move.w $76(a6), d0
pub fn i_04120a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4120E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04120E: lsr.w #$1, d0
pub fn i_04120e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41210;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $041210: cmpi.w #$6, d0
pub fn i_041210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41214;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $041214: bcs.w $4121c
pub fn i_041214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41218;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4121C; return 10; }
    8
}
/// $041218: move.w #$5, d0
pub fn i_041218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4121C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04121C: movea.l #$28618c, a0
pub fn i_04121c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41222;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28618C;
    cpu.a[0] = v1;
    4
}
/// $041222: lsl.w #$2, d0
pub fn i_041222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41224;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $041224: movea.l (a0, d0.w), a0
pub fn i_041224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41228;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $041228: cmpa.l #$ffffffff, a0
pub fn i_041228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4122E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04122E: beq.w $41238
pub fn i_04122e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41232;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x41238; return 10; }
    8
}
/// $041232: jsr $28cd4.l
pub fn i_041232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41238;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41238);
    cpu.pc = 0x28CD4;
    16
}
/// $041238: lea.l $4123e(pc), a1
pub fn i_041238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4123C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4123E;
    16
}
/// $04123C: move.l a1, (a6)
pub fn i_04123c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4123E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04123E: jsr $41f84(pc)
pub fn i_04123e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41242;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41242);
    cpu.pc = 0x41F84;
    16
}
/// $041242: move.b #$2, $45(a6)
pub fn i_041242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41248;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041248: jsr $28d70.l
pub fn i_041248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4124E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4124E);
    cpu.pc = 0x28D70;
    16
}
/// $04124E: bcc.w $41276
pub fn i_04124e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41252;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41276; return 10; }
    8
}
/// $041252: cmpi.b #$0, $86(a6)
pub fn i_041252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41258;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $041258: bne.w $41276
pub fn i_041258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4125C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x41276; return 10; }
    8
}
/// $04125C: move.b #$ff, $86(a6)
pub fn i_04125c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41262;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041262: jsr $41f3a(pc)
pub fn i_041262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41266;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41266);
    cpu.pc = 0x41F3A;
    16
}
/// $041266: move.b #$83, $80(a6)
pub fn i_041266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4126C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x83;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04126C: jsr $42040(pc)
pub fn i_04126c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41270;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41270);
    cpu.pc = 0x42040;
    16
}
/// $041270: lea.l $410c4(pc), a1
pub fn i_041270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41274;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x410C4;
    16
}
/// $041274: move.l a1, (a6)
pub fn i_041274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41276;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041276: jsr $41ff6(pc)
pub fn i_041276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4127A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4127A);
    cpu.pc = 0x41FF6;
    16
}
/// $04127A: bra.w $412fc
pub fn i_04127a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4127E;
    cpu.ir = 0x6000;
    cpu.pc = 0x412FC;
    10
}
// ===== SquadMember_AckAndHold_04127E  ($04127E, 62 B, enemies/squad/member_states_040f00.s)
/// $04127E: bclr.b #$3, $13(a6)
pub fn i_04127e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41284;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $041284: lea.l $286f74.l, a0
pub fn i_041284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4128A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x286F74;
    20
}
/// $04128A: jsr $28cd4.l
pub fn i_04128a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41290;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41290);
    cpu.pc = 0x28CD4;
    16
}
/// $041290: lea.l $41296(pc), a1
pub fn i_041290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41294;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41296;
    16
}
/// $041294: move.l a1, (a6)
pub fn i_041294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41296;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041296: jsr $41f84(pc)
pub fn i_041296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4129A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4129A);
    cpu.pc = 0x41F84;
    16
}
/// $04129A: jsr $28d70.l
pub fn i_04129a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x412A0);
    cpu.pc = 0x28D70;
    16
}
/// $0412A0: jsr $41f3a(pc)
pub fn i_0412a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x412A4);
    cpu.pc = 0x41F3A;
    16
}
/// $0412A4: move.b #$83, $80(a6)
pub fn i_0412a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412AA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x83;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0412AA: jsr $42040(pc)
pub fn i_0412aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x412AE);
    cpu.pc = 0x42040;
    16
}
/// $0412AE: lea.l $410c4(pc), a1
pub fn i_0412ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x410C4;
    16
}
/// $0412B2: move.l a1, (a6)
pub fn i_0412b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0412B4: jsr $41ff6(pc)
pub fn i_0412b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x412B8);
    cpu.pc = 0x41FF6;
    16
}
/// $0412B8: bra.w $412fc
pub fn i_0412b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x412FC;
    10
}
// ===== SquadMember_HitRecoil_0412BC  ($0412BC, 64 B, enemies/squad/member_states_040f00.s)
/// $0412BC: move.w #$1, $66(a6)
pub fn i_0412bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412C2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0412C2: lea.l $2872a2.l, a0
pub fn i_0412c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2872A2;
    20
}
/// $0412C8: jsr $28cd4.l
pub fn i_0412c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x412CE);
    cpu.pc = 0x28CD4;
    16
}
/// $0412CE: lea.l $412d4(pc), a1
pub fn i_0412ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x412D4;
    16
}
/// $0412D2: move.l a1, (a6)
pub fn i_0412d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0412D4: jsr $41f84(pc)
pub fn i_0412d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x412D8);
    cpu.pc = 0x41F84;
    16
}
/// $0412D8: jsr $28d70.l
pub fn i_0412d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x412DE);
    cpu.pc = 0x28D70;
    16
}
/// $0412DE: bcc.w $412f4
pub fn i_0412de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x412F4; return 10; }
    8
}
/// $0412E2: bclr.b #$3, $13(a6)
pub fn i_0412e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412E8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0412E8: move.w #$1, $66(a6)
pub fn i_0412e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0412EE: move.b #$0, $80(a6)
pub fn i_0412ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0412F4: jsr $41ff6(pc)
pub fn i_0412f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x412F8);
    cpu.pc = 0x41FF6;
    16
}
/// $0412F8: bra.w $41316
pub fn i_0412f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x412FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x41316;
    10
}
// ===== SquadMember_FrameTail_0412FC  ($0412FC, 36 B, enemies/squad/member_states_040f00.s)
/// $0412FC: jsr $2870a.l
pub fn i_0412fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41302;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41302);
    cpu.pc = 0x2870A;
    16
}
/// $041302: bcc.w $41316
pub fn i_041302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41306;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41316; return 10; }
    8
}
/// $041306: addq.b #$1, $87(a6)
pub fn i_041306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4130A;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x87);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04130A: move.b #$1e, $45(a6)
pub fn i_04130a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41310;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041310: lea.l $412bc(pc), a1
pub fn i_041310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41314;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x412BC;
    16
}
/// $041314: move.l a1, (a6)
pub fn i_041314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41316;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041316: jsr $5e45a.l
pub fn i_041316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4131C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4131C);
    cpu.pc = 0x5E45A;
    16
}
/// $04131C: bcc.w $41326
pub fn i_04131c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41320;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41326; return 10; }
    8
}
// ===== SetTaskHandler_041320  ($041320, 8 B, kernel/task_handlers.c)
/// $041320: lea.l $40ef2(pc), a1
pub fn i_041320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41324;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $041324: move.l a1, (a6)
pub fn i_041324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41326;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041326: rts
pub fn i_041326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41328;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadMember_AlignHeadingB_041328  ($041328, 98 B, enemies/squad/member_states_040f00.s)
/// $041328: move.w #$2, $76(a6)
pub fn i_041328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4132E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04132E: move.l #$ffffffff, $48(a6)
pub fn i_04132e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41336;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $041336: clr.w $70(a6)
pub fn i_041336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4133A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04133A: clr.b $86(a6)
pub fn i_04133a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4133E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04133E: move.w #$8, $78(a6)
pub fn i_04133e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41344;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041344: lea.l $286f7a.l, a0
pub fn i_041344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4134A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x286F7A;
    20
}
/// $04134A: jsr $28cd4.l
pub fn i_04134a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41350;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41350);
    cpu.pc = 0x28CD4;
    16
}
/// $041350: lea.l $41356(pc), a1
pub fn i_041350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41354;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41356;
    16
}
/// $041354: move.l a1, (a6)
pub fn i_041354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41356;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041356: jsr $41f84(pc)
pub fn i_041356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4135A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4135A);
    cpu.pc = 0x41F84;
    16
}
/// $04135A: jsr $28d70.l
pub fn i_04135a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41360;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41360);
    cpu.pc = 0x28D70;
    16
}
/// $041360: addq.w #$1, $70(a6)
pub fn i_041360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41364;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $041364: andi.w #$1, $70(a6)
pub fn i_041364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4136A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04136A: bne.w $4138a
pub fn i_04136a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4136E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4138A; return 10; }
    8
}
/// $04136E: jsr $41f50(pc)
pub fn i_04136e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41372;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41372);
    cpu.pc = 0x41F50;
    16
}
/// $041372: bcc.w $4138a
pub fn i_041372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41376;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4138A; return 10; }
    8
}
/// $041376: cmpi.b #$0, $86(a6)
pub fn i_041376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4137C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $04137C: bne.w $4138a
pub fn i_04137c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41380;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4138A; return 10; }
    8
}
/// $041380: move.b #$ff, $86(a6)
pub fn i_041380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41386;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041386: jsr $41f3a(pc)
pub fn i_041386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4138A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4138A);
    cpu.pc = 0x41F3A;
    16
}
// ===== JsrPcThunk_04138a  ($04138A, 6 B, thunks/jsr_pc_thunks.c)
/// $04138A: jsr $41ff6(pc)
pub fn i_04138a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4138E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4138E);
    cpu.pc = 0x41FF6;
    16
}
/// $04138E: rts
pub fn i_04138e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41390;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadMember_PoseFromHeadingB_041390  ($041390, 114 B, enemies/squad/member_states_040f00.s)
/// $041390: clr.b $86(a6)
pub fn i_041390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41394;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $041394: move.l #$ffffffff, $48(a6)
pub fn i_041394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4139C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04139C: move.w $76(a6), d0
pub fn i_04139c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413A0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0413A0: lsr.w #$1, d0
pub fn i_0413a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413A2;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0413A2: cmpi.w #$6, d0
pub fn i_0413a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413A6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $0413A6: bcs.w $413ae
pub fn i_0413a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413AA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x413AE; return 10; }
    8
}
/// $0413AA: move.w #$5, d0
pub fn i_0413aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413AE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0413AE: movea.l #$2861a4, a0
pub fn i_0413ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413B4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2861A4;
    cpu.a[0] = v1;
    4
}
/// $0413B4: lsl.w #$2, d0
pub fn i_0413b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413B6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0413B6: movea.l (a0, d0.w), a0
pub fn i_0413b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413BA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0413BA: cmpa.l #$ffffffff, a0
pub fn i_0413ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413C0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0413C0: beq.w $413ca
pub fn i_0413c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x413CA; return 10; }
    8
}
/// $0413C4: jsr $28cd4.l
pub fn i_0413c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x413CA);
    cpu.pc = 0x28CD4;
    16
}
/// $0413CA: lea.l $413d0(pc), a1
pub fn i_0413ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x413D0;
    16
}
/// $0413CE: move.l a1, (a6)
pub fn i_0413ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0413D0: jsr $41f84(pc)
pub fn i_0413d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x413D4);
    cpu.pc = 0x41F84;
    16
}
/// $0413D4: jsr $28d70.l
pub fn i_0413d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x413DA);
    cpu.pc = 0x28D70;
    16
}
/// $0413DA: bcc.w $41402
pub fn i_0413da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41402; return 10; }
    8
}
/// $0413DE: cmpi.b #$0, $86(a6)
pub fn i_0413de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413E4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0413E4: bne.w $41402
pub fn i_0413e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x41402; return 10; }
    8
}
/// $0413E8: move.b #$ff, $86(a6)
pub fn i_0413e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0413EE: jsr $41f3a(pc)
pub fn i_0413ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x413F2);
    cpu.pc = 0x41F3A;
    16
}
/// $0413F2: move.b #$83, $80(a6)
pub fn i_0413f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413F8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x83;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0413F8: jsr $42040(pc)
pub fn i_0413f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x413FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x413FC);
    cpu.pc = 0x42040;
    16
}
/// $0413FC: lea.l $4132e(pc), a1
pub fn i_0413fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41400;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4132E;
    16
}
/// $041400: move.l a1, (a6)
pub fn i_041400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41402;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_041402  ($041402, 6 B, thunks/jsr_pc_thunks.c)
/// $041402: jsr $41ff6(pc)
pub fn i_041402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41406;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41406);
    cpu.pc = 0x41FF6;
    16
}
/// $041406: rts
pub fn i_041406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41408;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadChild_SwoopPhysics_041408  ($041408, 122 B, enemies/squad/children_handlers_041408.s)
/// $041408: move.w #$d000, d0
pub fn i_041408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4140C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04140C: jsr $28134.l
pub fn i_04140c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41412;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41412);
    cpu.pc = 0x28134;
    16
}
/// $041412: andi.w #$ffe3, $38(a6)
pub fn i_041412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41418;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $041418: ori.w #$18, $38(a6)
pub fn i_041418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4141E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04141E: move.w #$fc00, $28(a6)
pub fn i_04141e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41424;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041424: move.w #$20, $2c(a6)
pub fn i_041424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4142A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04142A: move.w #$100, $2a(a6)
pub fn i_04142a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41430;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041430: move.w #$ffd8, $2e(a6)
pub fn i_041430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41436;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFD8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041436: move.w #$98, d1
pub fn i_041436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4143A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x98;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04143A: clr.b $21(a6)
pub fn i_04143a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4143E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04143E: jsr $236e.l
pub fn i_04143e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41444;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41444);
    cpu.pc = 0x236E;
    16
}
/// $041444: lea.l $2872e4.l, a0
pub fn i_041444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4144A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2872E4;
    20
}
/// $04144A: jsr $28cd4.l
pub fn i_04144a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41450;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41450);
    cpu.pc = 0x28CD4;
    16
}
/// $041450: lea.l $41456(pc), a1
pub fn i_041450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41454;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41456;
    16
}
/// $041454: move.l a1, (a6)
pub fn i_041454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41456;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041456: jsr $27d50.l
pub fn i_041456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4145C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4145C);
    cpu.pc = 0x27D50;
    16
}
/// $04145C: bcc.w $41466
pub fn i_04145c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41460;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41466; return 10; }
    8
}
/// $041460: lea.l $4148a(pc), a1
pub fn i_041460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41464;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4148A;
    16
}
/// $041464: move.l a1, (a6)
pub fn i_041464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41466;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041466: jsr $28d70.l
pub fn i_041466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4146C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4146C);
    cpu.pc = 0x28D70;
    16
}
/// $04146C: cmpi.w #$ff80, $28(a6)
pub fn i_04146c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41472;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $041472: blt.w $4147a
pub fn i_041472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41476;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4147A; return 10; }
    8
}
/// $041476: clr.w $2c(a6)
pub fn i_041476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4147A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04147A: tst.b $21(a6)
pub fn i_04147a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4147E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04147E: beq.w $41488
pub fn i_04147e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41482;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x41488; return 10; }
    8
}
// ===== SetTaskHandler_041482  ($041482, 8 B, kernel/task_handlers.c)
/// $041482: lea.l $4155a(pc), a1
pub fn i_041482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41486;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4155A;
    16
}
/// $041486: move.l a1, (a6)
pub fn i_041486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41488;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041488: rts
pub fn i_041488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4148A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadChild_FlipTouchdown_04148A  ($04148A, 200 B, enemies/squad/children_handlers_041408.s)
/// $04148A: addi.w #$ffe0, $22(a6)
pub fn i_04148a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41490;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $041490: addi.w #$fff8, $24(a6)
pub fn i_041490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41496;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $041496: eori.b #$1, $3a(a6)
pub fn i_041496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4149C;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04149C: jsr $4aad2.l
pub fn i_04149c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x414A2);
    cpu.pc = 0x4AAD2;
    16
}
/// $0414A2: eori.b #$1, $3a(a6)
pub fn i_0414a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414A8;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0414A8: jsr $4aad2.l
pub fn i_0414a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x414AE);
    cpu.pc = 0x4AAD2;
    16
}
/// $0414AE: subi.w #$ffe0, $22(a6)
pub fn i_0414ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414B4;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0414B4: subi.w #$fff8, $24(a6)
pub fn i_0414b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414BA;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xFFF8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0414BA: jsr $267e2.l
pub fn i_0414ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x414C0);
    cpu.pc = 0x267E2;
    16
}
/// $0414C0: move.w #$21c, $2a(a6)
pub fn i_0414c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414C6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x21C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0414C6: move.w #$ffdc, $2e(a6)
pub fn i_0414c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFDC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0414CC: lea.l $28732c.l, a0
pub fn i_0414cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28732C;
    20
}
/// $0414D2: jsr $28cd4.l
pub fn i_0414d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x414D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0414D8: lea.l $414de(pc), a1
pub fn i_0414d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x414DE;
    16
}
/// $0414DC: move.l a1, (a6)
pub fn i_0414dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0414DE: jsr $27d50.l
pub fn i_0414de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x414E4);
    cpu.pc = 0x27D50;
    16
}
/// $0414E4: bcc.w $4150e
pub fn i_0414e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4150E; return 10; }
    8
}
/// $0414E8: bset.b #$1, $12(a6)
pub fn i_0414e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414EE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0414EE: move.l #$285dbe, $48(a6)
pub fn i_0414ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414F6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x285DBE;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0414F6: lea.l $28735e.l, a0
pub fn i_0414f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x414FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28735E;
    20
}
/// $0414FC: jsr $28cd4.l
pub fn i_0414fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41502);
    cpu.pc = 0x28CD4;
    16
}
/// $041502: lea.l $41508(pc), a1
pub fn i_041502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41506;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41508;
    16
}
/// $041506: move.l a1, (a6)
pub fn i_041506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41508;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041508: jsr $2783a.l
pub fn i_041508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4150E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4150E);
    cpu.pc = 0x2783A;
    16
}
/// $04150E: jsr $28d70.l
pub fn i_04150e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41514);
    cpu.pc = 0x28D70;
    16
}
/// $041514: jsr $2870a.l
pub fn i_041514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4151A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4151A);
    cpu.pc = 0x2870A;
    16
}
/// $04151A: bcc.w $41558
pub fn i_04151a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4151E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41558; return 10; }
    8
}
/// $04151E: bclr.b #$3, $13(a6)
pub fn i_04151e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41524;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $041524: move.w #$103f, d0
pub fn i_041524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41528;
    cpu.ir = 0x303C;
    let v1: u32 = 0x103F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041528: jsr $2352.l
pub fn i_041528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4152E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4152E);
    cpu.pc = 0x2352;
    16
}
/// $04152E: addi.w #$ffe0, $22(a6)
pub fn i_04152e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41534;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $041534: addi.w #$fff8, $24(a6)
pub fn i_041534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4153A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04153A: jsr $4aae0.l
pub fn i_04153a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41540;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41540);
    cpu.pc = 0x4AAE0;
    16
}
/// $041540: subi.w #$ffe0, $22(a6)
pub fn i_041540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41546;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $041546: subi.w #$fff8, $24(a6)
pub fn i_041546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4154C;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xFFF8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04154C: lea.l $287372.l, a0
pub fn i_04154c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41552;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x287372;
    20
}
// ===== JsrAbsThunk_041552  ($041552, 8 B, thunks/jsr_abs_thunks.c)
/// $041552: jsr $28cd4.l
pub fn i_041552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41558;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41558);
    cpu.pc = 0x28CD4;
    16
}
/// $041558: rts
pub fn i_041558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4155A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadChild_TouchdownIdle_04155A  ($04155A, 28 B, enemies/squad/children_handlers_041408.s)
/// $04155A: jsr $267e2.l
pub fn i_04155a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41560;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41560);
    cpu.pc = 0x267E2;
    16
}
/// $041560: lea.l $41566(pc), a1
pub fn i_041560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41564;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41566;
    16
}
/// $041564: move.l a1, (a6)
pub fn i_041564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41566;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041566: jsr $2783a.l
pub fn i_041566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4156C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4156C);
    cpu.pc = 0x2783A;
    16
}
/// $04156C: cmpi.b #$1, $21(a6)
pub fn i_04156c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41572;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $041572: beq.w $4157c
pub fn i_041572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41576;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4157C; return 10; }
    8
}
// ===== SetTaskHandler_041576  ($041576, 8 B, kernel/task_handlers.c)
/// $041576: lea.l $4157e(pc), a1
pub fn i_041576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4157A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4157E;
    16
}
/// $04157A: move.l a1, (a6)
pub fn i_04157a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4157C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04157C: rts
pub fn i_04157c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4157E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_04157e  ($04157E, 8 B, kernel/jmp_to_scheduler.c)
/// $04157E: jmp $518.l
pub fn i_04157e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41584;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== SquadChild_DieToScheduler_041586  ($041586, 28 B, enemies/squad/children_handlers_041408.s)
/// $041586: move.w #$d000, d0
pub fn i_041586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4158A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04158A: jsr $28134.l
pub fn i_04158a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41590);
    cpu.pc = 0x28134;
    16
}
/// $041590: andi.w #$ffe3, $38(a6)
pub fn i_041590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41596;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $041596: ori.w #$1c, $38(a6)
pub fn i_041596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4159C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04159C: jmp $77fd6.l
pub fn i_04159c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77FD6;
    8
}
// ===== SquadChild_DespawnNoLink_0415A2  ($0415A2, 36 B, enemies/squad/children_handlers_041408.s)
/// $0415A2: move.w #$4000, d0
pub fn i_0415a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0415A6: jsr $28134.l
pub fn i_0415a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x415AC);
    cpu.pc = 0x28134;
    16
}
/// $0415AC: andi.w #$ffe3, $38(a6)
pub fn i_0415ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415B2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0415B2: ori.w #$0, $38(a6)
pub fn i_0415b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415B8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0415B8: move.l #$ffffffff, $48(a6)
pub fn i_0415b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415C0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0415C0: jmp $77f6a.l
pub fn i_0415c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415C6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== PairChild_HandlerA_0415C6  ($0415C6, 88 B, enemies/squad/children_handlers_041408.s)
/// $0415C6: move.w #$d000, d0
pub fn i_0415c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415CA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0415CA: jsr $28134.l
pub fn i_0415ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x415D0);
    cpu.pc = 0x28134;
    16
}
/// $0415D0: andi.w #$ffe3, $38(a6)
pub fn i_0415d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415D6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0415D6: ori.w #$14, $38(a6)
pub fn i_0415d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415DC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0415DC: move.w #$b3, d1
pub fn i_0415dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415E0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0415E0: jsr $236e.l
pub fn i_0415e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x415E6);
    cpu.pc = 0x236E;
    16
}
/// $0415E6: lea.l $288200.l, a0
pub fn i_0415e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x288200;
    20
}
/// $0415EC: jsr $28cd4.l
pub fn i_0415ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x415F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0415F2: lea.l $415f8(pc), a1
pub fn i_0415f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x415F8;
    16
}
/// $0415F6: move.l a1, (a6)
pub fn i_0415f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0415F8: jsr $2783a.l
pub fn i_0415f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x415FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x415FE);
    cpu.pc = 0x2783A;
    16
}
/// $0415FE: jsr $28d70.l
pub fn i_0415fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41604;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41604);
    cpu.pc = 0x28D70;
    16
}
/// $041604: bcc.w $4160e
pub fn i_041604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41608;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4160E; return 10; }
    8
}
/// $041608: lea.l $40ef2(pc), a1
pub fn i_041608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4160C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $04160C: move.l a1, (a6)
pub fn i_04160c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4160E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04160E: movea.l #$ffffffff, a0
pub fn i_04160e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41614;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $041614: jsr $5dd56.l
pub fn i_041614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4161A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4161A);
    cpu.pc = 0x5DD56;
    16
}
/// $04161A: bcc.w $41624
pub fn i_04161a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4161E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41624; return 10; }
    8
}
// ===== SetTaskHandler_04161e  ($04161E, 8 B, kernel/task_handlers.c)
/// $04161E: lea.l $40ef2(pc), a1
pub fn i_04161e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41622;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $041622: move.l a1, (a6)
pub fn i_041622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41624;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041624: rts
pub fn i_041624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41626;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PairChild_HandlerB_041626  ($041626, 308 B, enemies/squad/children_handlers_041408.s)
/// $041626: bset.b #$4, $6b(a6)
pub fn i_041626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4162C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04162C: tst.b $88(a6)
pub fn i_04162c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41630;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $041630: bne.w $41658
pub fn i_041630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41634;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x41658; return 10; }
    8
}
/// $041634: lea.l $2bbbc6.l, a0
pub fn i_041634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4163A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BBBC6;
    20
}
/// $04163A: movea.l $c(a6), a1
pub fn i_04163a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4163E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04163E: cmpi.b #$ff, $7c(a1)
pub fn i_04163e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41644;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $041644: bne.w $4164e
pub fn i_041644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41648;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4164E; return 10; }
    8
}
/// $041648: lea.l $2bbc48.l, a0
pub fn i_041648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4164E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BBC48;
    20
}
/// $04164E: jsr $799de.l
pub fn i_04164e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41654;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41654);
    cpu.pc = 0x799DE;
    16
}
/// $041654: bra.w $41664
pub fn i_041654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41658;
    cpu.ir = 0x6000;
    cpu.pc = 0x41664;
    10
}
/// $041658: lea.l $2ba052.l, a0
pub fn i_041658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4165E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA052;
    20
}
/// $04165E: jsr $799de.l
pub fn i_04165e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41664;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41664);
    cpu.pc = 0x799DE;
    16
}
/// $041664: move.w d0, $36(a6)
pub fn i_041664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41668;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041668: lea.l $2861ec.l, a0
pub fn i_041668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4166E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2861EC;
    20
}
/// $04166E: move.w $76(a6), d1
pub fn i_04166e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41672;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041672: add.w d1, d1
pub fn i_041672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41674;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $041674: move.w (a0, d1.w), d0
pub fn i_041674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41678;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $041678: jsr $420a4(pc)
pub fn i_041678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4167C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4167C);
    cpu.pc = 0x420A4;
    16
}
/// $04167C: move.l #$285fb6, $4c(a6)
pub fn i_04167c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41684;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x285FB6;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $041684: move.l #$285e12, $48(a6)
pub fn i_041684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4168C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x285E12;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04168C: move.w #$32, $66(a6)
pub fn i_04168c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41692;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041692: move.w #$b4, d1
pub fn i_041692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41696;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041696: jsr $236e.l
pub fn i_041696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4169C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4169C);
    cpu.pc = 0x236E;
    16
}
/// $04169C: jsr $283ca.l
pub fn i_04169c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x416A2);
    cpu.pc = 0x283CA;
    16
}
/// $0416A2: move.w $76(a6), d0
pub fn i_0416a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416A6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0416A6: movea.l #$2861bc, a0
pub fn i_0416a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416AC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2861BC;
    cpu.a[0] = v1;
    4
}
/// $0416AC: lsl.w #$2, d0
pub fn i_0416ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416AE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0416AE: movea.l (a0, d0.w), a0
pub fn i_0416ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416B2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0416B2: cmpa.l #$ffffffff, a0
pub fn i_0416b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416B8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0416B8: beq.w $416c2
pub fn i_0416b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x416C2; return 10; }
    8
}
/// $0416BC: jsr $28cd4.l
pub fn i_0416bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x416C2);
    cpu.pc = 0x28CD4;
    16
}
/// $0416C2: move.w #$d000, d0
pub fn i_0416c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0416C6: jsr $28134.l
pub fn i_0416c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x416CC);
    cpu.pc = 0x28134;
    16
}
/// $0416CC: andi.w #$ffe3, $38(a6)
pub fn i_0416cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416D2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0416D2: ori.w #$14, $38(a6)
pub fn i_0416d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416D8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0416D8: lea.l $416de(pc), a1
pub fn i_0416d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x416DE;
    16
}
/// $0416DC: move.l a1, (a6)
pub fn i_0416dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0416DE: tst.b $88(a6)
pub fn i_0416de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416E2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0416E2: beq.w $41700
pub fn i_0416e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x41700; return 10; }
    8
}
/// $0416E6: bset.b #$6, $13(a6)
pub fn i_0416e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416EC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0416EC: jsr $27bc8.l
pub fn i_0416ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x416F2);
    cpu.pc = 0x27BC8;
    16
}
/// $0416F2: bcc.w $416fc
pub fn i_0416f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x416FC; return 10; }
    8
}
/// $0416F6: lea.l $41762(pc), a1
pub fn i_0416f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41762;
    16
}
/// $0416FA: move.l a1, (a6)
pub fn i_0416fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x416FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0416FC: bra.w $41710
pub fn i_0416fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41700;
    cpu.ir = 0x6000;
    cpu.pc = 0x41710;
    10
}
/// $041700: jsr $27cee.l
pub fn i_041700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41706;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41706);
    cpu.pc = 0x27CEE;
    16
}
/// $041706: bcc.w $41710
pub fn i_041706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4170A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41710; return 10; }
    8
}
/// $04170A: lea.l $41762(pc), a1
pub fn i_04170a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4170E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41762;
    16
}
/// $04170E: move.l a1, (a6)
pub fn i_04170e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41710;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041710: jsr $28d70.l
pub fn i_041710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41716;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41716);
    cpu.pc = 0x28D70;
    16
}
/// $041716: tst.b $7f(a6)
pub fn i_041716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4171A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04171A: bne.w $4174a
pub fn i_04171a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4171E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4174A; return 10; }
    8
}
/// $04171E: jsr $283d8.l
pub fn i_04171e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41724;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41724);
    cpu.pc = 0x283D8;
    16
}
/// $041724: btst.b #$1, $13(a6)
pub fn i_041724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4172A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04172A: beq.w $41734
pub fn i_04172a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4172E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x41734; return 10; }
    8
}
/// $04172E: lea.l $41788(pc), a1
pub fn i_04172e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41732;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41788;
    16
}
/// $041732: move.l a1, (a6)
pub fn i_041732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41734;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041734: bclr.b #$3, $13(a6)
pub fn i_041734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4173A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04173A: jsr $28758.l
pub fn i_04173a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41740;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41740);
    cpu.pc = 0x28758;
    16
}
/// $041740: bcc.w $4174a
pub fn i_041740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41744;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4174A; return 10; }
    8
}
/// $041744: lea.l $41762(pc), a1
pub fn i_041744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41748;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41762;
    16
}
/// $041748: move.l a1, (a6)
pub fn i_041748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4174A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04174A: movea.l #$ffffffff, a0
pub fn i_04174a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41750;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $041750: jsr $5dd56.l
pub fn i_041750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41756;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41756);
    cpu.pc = 0x5DD56;
    16
}
/// $041756: bcc.w $41760
pub fn i_041756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4175A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41760; return 10; }
    8
}
// ===== SetTaskHandler_04175a  ($04175A, 8 B, kernel/task_handlers.c)
/// $04175A: lea.l $40ef2(pc), a1
pub fn i_04175a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4175E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $04175E: move.l a1, (a6)
pub fn i_04175e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41760;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041760: rts
pub fn i_041760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41762;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PairChild_DeathCry_041762  ($041762, 74 B, enemies/squad/children_handlers_041408.s)
/// $041762: move.w #$1022, d0
pub fn i_041762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41766;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041766: jsr $2352.l
pub fn i_041766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4176C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4176C);
    cpu.pc = 0x2352;
    16
}
/// $04176C: tst.b $7f(a6)
pub fn i_04176c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41770;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $041770: beq.w $41788
pub fn i_041770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41774;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x41788; return 10; }
    8
}
/// $041774: move.l #$28605e, $4c(a6)
pub fn i_041774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4177C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x28605E;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04177C: jsr $283ca.l
pub fn i_04177c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41782);
    cpu.pc = 0x283CA;
    16
}
/// $041782: jsr $283d8.l
pub fn i_041782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41788);
    cpu.pc = 0x283D8;
    16
}
/// $041788: move.l #$ffffffff, $48(a6)
pub fn i_041788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41790;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $041790: move.w #$4000, d0
pub fn i_041790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41794;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041794: jsr $28134.l
pub fn i_041794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4179A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4179A);
    cpu.pc = 0x28134;
    16
}
/// $04179A: andi.w #$ffe3, $38(a6)
pub fn i_04179a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417A0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0417A0: ori.w #$0, $38(a6)
pub fn i_0417a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417A6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0417A6: jmp $77fd6.l
pub fn i_0417a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417AC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77FD6;
    8
}
// ===== SquadChild_DropSpawnAtTop_0417AC  ($0417AC, 104 B, enemies/squad/children_handlers_041408.s)
/// $0417AC: bset.b #$4, $6b(a6)
pub fn i_0417ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417B2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0417B2: lea.l $28760c.l, a0
pub fn i_0417b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28760C;
    20
}
/// $0417B8: jsr $28cd4.l
pub fn i_0417b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x417BE);
    cpu.pc = 0x28CD4;
    16
}
/// $0417BE: jsr $5e9b6.l
pub fn i_0417be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x417C4);
    cpu.pc = 0x5E9B6;
    16
}
/// $0417C4: andi.w #$7, d0
pub fn i_0417c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417C8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0417C8: subq.w #$4, d0
pub fn i_0417c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417CA;
    cpu.ir = 0x5940;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0417CA: add.w d0, $22(a6)
pub fn i_0417ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417CE;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0417CE: move.w #$f80, d0
pub fn i_0417ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417D2;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0417D2: move.w #$560, d1
pub fn i_0417d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417D6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x560;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0417D6: subi.w #$280, d1
pub fn i_0417d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417DA;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x280, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0417DA: jsr $440d0.l
pub fn i_0417da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x417E0);
    cpu.pc = 0x440D0;
    16
}
/// $0417E0: move.w d1, $24(a6)
pub fn i_0417e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417E4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0417E4: movea.l #$ffffffff, a0
pub fn i_0417e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417EA;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0417EA: jsr $5dd56.l
pub fn i_0417ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x417F0);
    cpu.pc = 0x5DD56;
    16
}
/// $0417F0: bcs.w $40ef2
pub fn i_0417f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417F4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x40EF2; return 10; }
    8
}
/// $0417F4: move.w #$d000, d0
pub fn i_0417f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0417F8: jsr $28134.l
pub fn i_0417f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x417FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x417FE);
    cpu.pc = 0x28134;
    16
}
/// $0417FE: andi.w #$ffe3, $38(a6)
pub fn i_0417fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41804;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $041804: ori.w #$14, $38(a6)
pub fn i_041804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4180A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04180A: move.w #$193, d1
pub fn i_04180a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4180E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x193;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04180E: jsr $236e.l
pub fn i_04180e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41814);
    cpu.pc = 0x236E;
    16
}
// ===== SetTaskHandler_041814  ($041814, 8 B, kernel/task_handlers.c)
/// $041814: lea.l $4181c(pc), a1
pub fn i_041814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41818;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4181C;
    16
}
/// $041818: move.l a1, (a6)
pub fn i_041818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4181A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04181A: rts
pub fn i_04181a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4181C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadChild_DropRun_04181C  ($04181C, 44 B, enemies/squad/children_handlers_041408.s)
/// $04181C: jsr $2783a.l
pub fn i_04181c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41822);
    cpu.pc = 0x2783A;
    16
}
/// $041822: jsr $28d70.l
pub fn i_041822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41828;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41828);
    cpu.pc = 0x28D70;
    16
}
/// $041828: bcc.w $41832
pub fn i_041828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4182C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41832; return 10; }
    8
}
/// $04182C: lea.l $40ef2(pc), a1
pub fn i_04182c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41830;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $041830: move.l a1, (a6)
pub fn i_041830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41832;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041832: jsr $283d8.l
pub fn i_041832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41838);
    cpu.pc = 0x283D8;
    16
}
/// $041838: movea.l #$ffffffff, a0
pub fn i_041838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4183E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04183E: jsr $5dd56.l
pub fn i_04183e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41844;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41844);
    cpu.pc = 0x5DD56;
    16
}
/// $041844: bcc.w $4184e
pub fn i_041844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41848;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4184E; return 10; }
    8
}
// ===== SetTaskHandler_041848  ($041848, 8 B, kernel/task_handlers.c)
/// $041848: lea.l $40ef2(pc), a1
pub fn i_041848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4184C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $04184C: move.l a1, (a6)
pub fn i_04184c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4184E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04184E: rts
pub fn i_04184e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41850;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadChild_ZigzagFall_041850  ($041850, 194 B, enemies/squad/children_handlers_041408.s)
/// $041850: jsr $5e9b6.l
pub fn i_041850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41856;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41856);
    cpu.pc = 0x5E9B6;
    16
}
/// $041856: andi.w #$3, d0
pub fn i_041856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4185A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04185A: lsl.w #$2, d0
pub fn i_04185a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4185C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04185C: lea.l $2861f8.l, a0
pub fn i_04185c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41862;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2861F8;
    20
}
/// $041862: movea.l (a0, d0.w), a0
pub fn i_041862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41866;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $041866: move.w $34(a6), d0
pub fn i_041866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4186A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04186A: andi.w #$f, d0
pub fn i_04186a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4186E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04186E: move.w d0, d1
pub fn i_04186e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41870;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041870: add.w d0, d0
pub fn i_041870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41872;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $041872: add.w d1, d0
pub fn i_041872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41874;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $041874: add.w d0, d0
pub fn i_041874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41876;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $041876: ext.l d0
pub fn i_041876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41878;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $041878: adda.l d0, a0
pub fn i_041878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4187A;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $04187A: move.w (a0), $28(a6)
pub fn i_04187a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4187E;
    cpu.ir = 0x3D50;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad3, v2 as u16);
    16
}
/// $04187E: move.w $2(a0), d0
pub fn i_04187e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41882;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041882: add.w d0, d0
pub fn i_041882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41884;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $041884: move.w d0, $2e(a6)
pub fn i_041884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41888;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041888: clr.w $2a(a6)
pub fn i_041888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4188C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04188C: clr.w $70(a6)
pub fn i_04188c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41890;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $041890: move.w #$9b, d1
pub fn i_041890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41894;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041894: jsr $236e.l
pub fn i_041894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4189A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4189A);
    cpu.pc = 0x236E;
    16
}
/// $04189A: move.w #$4000, d0
pub fn i_04189a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4189E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04189E: jsr $28134.l
pub fn i_04189e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x418A4);
    cpu.pc = 0x28134;
    16
}
/// $0418A4: andi.w #$ffe3, $38(a6)
pub fn i_0418a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418AA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0418AA: ori.w #$1c, $38(a6)
pub fn i_0418aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418B0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0418B0: lea.l $287684.l, a0
pub fn i_0418b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x287684;
    20
}
/// $0418B6: jsr $28cd4.l
pub fn i_0418b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x418BC);
    cpu.pc = 0x28CD4;
    16
}
/// $0418BC: lea.l $418c2(pc), a1
pub fn i_0418bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x418C2;
    16
}
/// $0418C0: move.l a1, (a6)
pub fn i_0418c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0418C2: jsr $27d50.l
pub fn i_0418c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x418C8);
    cpu.pc = 0x27D50;
    16
}
/// $0418C8: bcc.w $418d2
pub fn i_0418c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x418D2; return 10; }
    8
}
/// $0418CC: lea.l $40ef2(pc), a1
pub fn i_0418cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $0418D0: move.l a1, (a6)
pub fn i_0418d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0418D2: addq.w #$1, $70(a6)
pub fn i_0418d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418D6;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0418D6: cmpi.w #$8, $70(a6)
pub fn i_0418d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418DC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Word, false);
    16
}
/// $0418DC: blt.w $418fc
pub fn i_0418dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418E0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x418FC; return 10; }
    8
}
/// $0418E0: cmpi.w #$f, $70(a6)
pub fn i_0418e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418E6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF, Size::Word, false);
    16
}
/// $0418E6: blt.w $418f0
pub fn i_0418e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418EA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x418F0; return 10; }
    8
}
/// $0418EA: lea.l $40ef2(pc), a1
pub fn i_0418ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $0418EE: move.l a1, (a6)
pub fn i_0418ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0418F0: move.w $70(a6), d0
pub fn i_0418f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418F4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0418F4: btst.b #$0, d0
pub fn i_0418f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418F8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0418F8: beq.w $41902
pub fn i_0418f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x418FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x41902; return 10; }
    8
}
/// $0418FC: jsr $28d70.l
pub fn i_0418fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41902;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41902);
    cpu.pc = 0x28D70;
    16
}
/// $041902: movea.l #$ffffffff, a0
pub fn i_041902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41908;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $041908: jsr $5dd56.l
pub fn i_041908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4190E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4190E);
    cpu.pc = 0x5DD56;
    16
}
/// $04190E: bcc.w $41918
pub fn i_04190e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41912;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41918; return 10; }
    8
}
// ===== SetTaskHandler_041912  ($041912, 8 B, kernel/task_handlers.c)
/// $041912: lea.l $40ef2(pc), a1
pub fn i_041912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41916;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $041916: move.l a1, (a6)
pub fn i_041916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41918;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041918: rts
pub fn i_041918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4191A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadChild_GlideAttack_04191A  ($04191A, 156 B, enemies/squad/children_handlers_041408.s)
/// $04191A: bset.b #$4, $6b(a6)
pub fn i_04191a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41920;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $041920: move.w #$99, d1
pub fn i_041920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41924;
    cpu.ir = 0x323C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041924: jsr $236e.l
pub fn i_041924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4192A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4192A);
    cpu.pc = 0x236E;
    16
}
/// $04192A: move.w #$15, $1c(a6)
pub fn i_04192a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41930;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x15;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041930: jsr $138fe.l
pub fn i_041930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41936);
    cpu.pc = 0x138FE;
    16
}
/// $041936: move.w #$fe00, $2a(a6)
pub fn i_041936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4193C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04193C: move.w #$ffc0, $2e(a6)
pub fn i_04193c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41942;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041942: lea.l $2bbff6.l, a0
pub fn i_041942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41948;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BBFF6;
    20
}
/// $041948: jsr $799de.l
pub fn i_041948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4194E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4194E);
    cpu.pc = 0x799DE;
    16
}
/// $04194E: move.w d0, $66(a6)
pub fn i_04194e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41952;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041952: lea.l $2876ae.l, a0
pub fn i_041952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41958;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2876AE;
    20
}
/// $041958: jsr $28cd4.l
pub fn i_041958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4195E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4195E);
    cpu.pc = 0x28CD4;
    16
}
/// $04195E: lea.l $41964(pc), a1
pub fn i_04195e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41962;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41964;
    16
}
/// $041962: move.l a1, (a6)
pub fn i_041962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41964;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041964: jsr $27d50.l
pub fn i_041964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4196A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4196A);
    cpu.pc = 0x27D50;
    16
}
/// $04196A: bcc.w $41974
pub fn i_04196a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4196E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41974; return 10; }
    8
}
/// $04196E: lea.l $419be(pc), a1
pub fn i_04196e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41972;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x419BE;
    16
}
/// $041972: move.l a1, (a6)
pub fn i_041972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41974;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041974: jsr $28d70.l
pub fn i_041974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4197A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4197A);
    cpu.pc = 0x28D70;
    16
}
/// $04197A: jsr $283d8.l
pub fn i_04197a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41980;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41980);
    cpu.pc = 0x283D8;
    16
}
/// $041980: btst.b #$1, $13(a6)
pub fn i_041980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41986;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $041986: beq.w $41990
pub fn i_041986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4198A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x41990; return 10; }
    8
}
/// $04198A: lea.l $415a2(pc), a1
pub fn i_04198a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4198E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x415A2;
    16
}
/// $04198E: move.l a1, (a6)
pub fn i_04198e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41990;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041990: bclr.b #$3, $13(a6)
pub fn i_041990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41996;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $041996: jsr $28758.l
pub fn i_041996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4199C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4199C);
    cpu.pc = 0x28758;
    16
}
/// $04199C: bcc.w $419a6
pub fn i_04199c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x419A6; return 10; }
    8
}
/// $0419A0: lea.l $419be(pc), a1
pub fn i_0419a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x419BE;
    16
}
/// $0419A4: move.l a1, (a6)
pub fn i_0419a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0419A6: movea.l #$ffffffff, a0
pub fn i_0419a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419AC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0419AC: jsr $5dd56.l
pub fn i_0419ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x419B2);
    cpu.pc = 0x5DD56;
    16
}
/// $0419B2: bcc.w $419bc
pub fn i_0419b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x419BC; return 10; }
    8
}
// ===== SetTaskHandler_0419b6  ($0419B6, 8 B, kernel/task_handlers.c)
/// $0419B6: lea.l $40ef2(pc), a1
pub fn i_0419b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $0419BA: move.l a1, (a6)
pub fn i_0419ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0419BC: rts
pub fn i_0419bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SquadChild_GlideDeath_0419BE  ($0419BE, 14 B, enemies/squad/children_handlers_041408.s)
/// $0419BE: move.w #$1022, d0
pub fn i_0419be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419C2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0419C2: jsr $2352.l
pub fn i_0419c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x419C8);
    cpu.pc = 0x2352;
    16
}
/// $0419C8: bra.w $415a2
pub fn i_0419c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419CC;
    cpu.ir = 0x6000;
    cpu.pc = 0x415A2;
    10
}
// ===== TrioChild_HandlerA_0419CC  ($0419CC, 48 B, enemies/squad/children_handlers_041408.s)
/// $0419CC: move.w #$d000, d0
pub fn i_0419cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419D0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0419D0: jsr $28134.l
pub fn i_0419d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x419D6);
    cpu.pc = 0x28134;
    16
}
/// $0419D6: andi.w #$ffe3, $38(a6)
pub fn i_0419d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419DC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0419DC: ori.w #$1c, $38(a6)
pub fn i_0419dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419E2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0419E2: move.w #$b3, d1
pub fn i_0419e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419E6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0419E6: jsr $236e.l
pub fn i_0419e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x419EC);
    cpu.pc = 0x236E;
    16
}
/// $0419EC: lea.l $287708.l, a0
pub fn i_0419ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x287708;
    20
}
/// $0419F2: jsr $28cd4.l
pub fn i_0419f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x419F8);
    cpu.pc = 0x28CD4;
    16
}
/// $0419F8: bra.w $415f2
pub fn i_0419f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x419FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x415F2;
    10
}
// ===== TrioChild_HandlerB_0419FC  ($0419FC, 176 B, enemies/squad/children_handlers_041408.s)
/// $0419FC: bset.b #$4, $6b(a6)
pub fn i_0419fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A02;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $041A02: clr.w $34(a6)
pub fn i_041a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A06;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $041A06: move.w #$200, $36(a6)
pub fn i_041a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A0C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041A0C: move.w #$d000, d0
pub fn i_041a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A10;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041A10: jsr $28134.l
pub fn i_041a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41A16);
    cpu.pc = 0x28134;
    16
}
/// $041A16: andi.w #$ffe3, $38(a6)
pub fn i_041a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A1C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $041A1C: ori.w #$14, $38(a6)
pub fn i_041a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A22;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $041A22: move.w #$9c, d1
pub fn i_041a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A26;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041A26: jsr $236e.l
pub fn i_041a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41A2C);
    cpu.pc = 0x236E;
    16
}
/// $041A2C: lea.l $2877aa.l, a0
pub fn i_041a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2877AA;
    20
}
/// $041A32: jsr $28cd4.l
pub fn i_041a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41A38);
    cpu.pc = 0x28CD4;
    16
}
/// $041A38: move.w $9a(a6), d0
pub fn i_041a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A3C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041A3C: andi.w #$3, d0
pub fn i_041a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A40;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $041A40: lea.l $28632c.l, a0
pub fn i_041a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28632C;
    20
}
/// $041A46: lsl.w #$2, d0
pub fn i_041a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A48;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $041A48: move.l (a0, d0.w), $90(a6)
pub fn i_041a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A4E;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $041A4E: clr.w $94(a6)
pub fn i_041a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A52;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $041A52: clr.b $96(a6)
pub fn i_041a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A56;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x96);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $041A56: lea.l $41a5c(pc), a1
pub fn i_041a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41A5C;
    16
}
/// $041A5A: move.l a1, (a6)
pub fn i_041a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041A5C: jsr $78f8a.l
pub fn i_041a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41A62);
    cpu.pc = 0x78F8A;
    16
}
/// $041A62: bcc.w $41a6c
pub fn i_041a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41A6C; return 10; }
    8
}
/// $041A66: lea.l $41ab4(pc), a1
pub fn i_041a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41AB4;
    16
}
/// $041A6A: move.l a1, (a6)
pub fn i_041a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041A6C: jsr $42282(pc)
pub fn i_041a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41A70);
    cpu.pc = 0x42282;
    16
}
/// $041A70: jsr $422ca(pc)
pub fn i_041a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41A74);
    cpu.pc = 0x422CA;
    16
}
/// $041A74: jsr $28d70.l
pub fn i_041a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41A7A);
    cpu.pc = 0x28D70;
    16
}
/// $041A7A: movea.l $c(a6), a0
pub fn i_041a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A7E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $041A7E: movea.l $c(a0), a0
pub fn i_041a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A82;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $041A82: addq.b #$1, $92(a0)
pub fn i_041a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A86;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x92);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $041A86: btst.b #$0, $13(a0)
pub fn i_041a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A8C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $041A8C: beq.w $41a96
pub fn i_041a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x41A96; return 10; }
    8
}
/// $041A90: lea.l $415a2(pc), a1
pub fn i_041a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x415A2;
    16
}
/// $041A94: move.l a1, (a6)
pub fn i_041a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041A96: movea.l #$ffffffff, a0
pub fn i_041a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41A9C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $041A9C: lea.l $28610a.l, a0
pub fn i_041a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AA2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28610A;
    20
}
/// $041AA2: jsr $5dd56.l
pub fn i_041aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41AA8);
    cpu.pc = 0x5DD56;
    16
}
/// $041AA8: bcc.w $41ab2
pub fn i_041aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41AB2; return 10; }
    8
}
// ===== SetTaskHandler_041aac  ($041AAC, 8 B, kernel/task_handlers.c)
/// $041AAC: lea.l $40ef2(pc), a1
pub fn i_041aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $041AB0: move.l a1, (a6)
pub fn i_041ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041AB2: rts
pub fn i_041ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TrioChild_OrbitTracker_041AB4  ($041AB4, 260 B, enemies/squad/children_handlers_041408.s)
/// $041AB4: clr.w $70(a6)
pub fn i_041ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AB8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $041AB8: move.w #$80, $76(a6)
pub fn i_041ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41ABE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041ABE: move.b #$ff, $32(a6)
pub fn i_041abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AC4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041AC4: move.b #$ff, $33(a6)
pub fn i_041ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41ACA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041ACA: lea.l $2bc078.l, a0
pub fn i_041aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AD0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC078;
    20
}
/// $041AD0: jsr $799de.l
pub fn i_041ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41AD6);
    cpu.pc = 0x799DE;
    16
}
/// $041AD6: move.w d0, $66(a6)
pub fn i_041ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41ADA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041ADA: lea.l $2bc1fe.l, a0
pub fn i_041ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AE0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC1FE;
    20
}
/// $041AE0: movea.l $c(a6), a1
pub fn i_041ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AE4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $041AE4: cmpi.b #$ff, $7c(a1)
pub fn i_041ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AEA;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $041AEA: bne.w $41af4
pub fn i_041aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AEE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x41AF4; return 10; }
    8
}
/// $041AEE: lea.l $2bc280.l, a0
pub fn i_041aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC280;
    20
}
/// $041AF4: jsr $799de.l
pub fn i_041af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41AFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41AFA);
    cpu.pc = 0x799DE;
    16
}
/// $041AFA: lea.l $2bc302.l, a0
pub fn i_041afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B00;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BC302;
    20
}
/// $041B00: lsl.w #$2, d0
pub fn i_041b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B02;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $041B02: move.w (a0, d0.w), $36(a6)
pub fn i_041b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B08;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad4, v3 as u16);
    22
}
/// $041B08: move.w $2(a0, d0.w), $72(a6)
pub fn i_041b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B0E;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    22
}
/// $041B0E: jsr $5e1ea.l
pub fn i_041b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41B14);
    cpu.pc = 0x5E1EA;
    16
}
/// $041B14: move.l a0, $9c(a6)
pub fn i_041b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B18;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $041B18: move.w #$0, $88(a6)
pub fn i_041b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041B1E: lea.l $287b1c.l, a0
pub fn i_041b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x287B1C;
    20
}
/// $041B24: jsr $28cd4.l
pub fn i_041b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41B2A);
    cpu.pc = 0x28CD4;
    16
}
/// $041B2A: lea.l $41b30(pc), a1
pub fn i_041b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41B30;
    16
}
/// $041B2E: move.l a1, (a6)
pub fn i_041b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041B30: addq.w #$1, $88(a6)
pub fn i_041b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B34;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $041B34: cmpi.w #$b4, $88(a6)
pub fn i_041b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B3A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xB4, Size::Word, false);
    16
}
/// $041B3A: bgt.w $41b68
pub fn i_041b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B3E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x41B68; return 10; }
    8
}
/// $041B3E: subq.w #$1, $70(a6)
pub fn i_041b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B42;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $041B42: cmpi.w #$0, $70(a6)
pub fn i_041b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B48;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $041B48: bgt.w $41b68
pub fn i_041b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B4C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x41B68; return 10; }
    8
}
/// $041B4C: move.w $72(a6), $70(a6)
pub fn i_041b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B52;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $041B52: movea.l $9c(a6), a0
pub fn i_041b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B56;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $041B56: jsr $5e070.l
pub fn i_041b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41B5C);
    cpu.pc = 0x5E070;
    16
}
/// $041B5C: move.w d0, $78(a6)
pub fn i_041b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B60;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041B60: jsr $41f48(pc)
pub fn i_041b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41B64);
    cpu.pc = 0x41F48;
    16
}
/// $041B64: jsr $420a4(pc)
pub fn i_041b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41B68);
    cpu.pc = 0x420A4;
    16
}
/// $041B68: jsr $27cee.l
pub fn i_041b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41B6E);
    cpu.pc = 0x27CEE;
    16
}
/// $041B6E: bcc.w $41b78
pub fn i_041b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41B78; return 10; }
    8
}
/// $041B72: lea.l $41bb8(pc), a1
pub fn i_041b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41BB8;
    16
}
/// $041B76: move.l a1, (a6)
pub fn i_041b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041B78: move.w $76(a6), d0
pub fn i_041b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B7C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041B7C: lsr.w #$3, d0
pub fn i_041b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B7E;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $041B7E: move.w d0, $34(a6)
pub fn i_041b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B82;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041B82: jsr $28d70.l
pub fn i_041b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41B88);
    cpu.pc = 0x28D70;
    16
}
/// $041B88: jsr $283d8.l
pub fn i_041b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41B8E);
    cpu.pc = 0x283D8;
    16
}
/// $041B8E: btst.b #$1, $13(a6)
pub fn i_041b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B94;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $041B94: beq.w $41b9e
pub fn i_041b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x41B9E; return 10; }
    8
}
/// $041B98: lea.l $415a2(pc), a1
pub fn i_041b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x415A2;
    16
}
/// $041B9C: move.l a1, (a6)
pub fn i_041b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41B9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041B9E: bclr.b #$3, $13(a6)
pub fn i_041b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BA4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $041BA4: jsr $28758.l
pub fn i_041ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41BAA);
    cpu.pc = 0x28758;
    16
}
/// $041BAA: bcc.w $41bb4
pub fn i_041baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41BB4; return 10; }
    8
}
/// $041BAE: lea.l $41bb8(pc), a1
pub fn i_041bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41BB8;
    16
}
/// $041BB2: move.l a1, (a6)
pub fn i_041bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041BB4: bra.w $41a7a
pub fn i_041bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BB8;
    cpu.ir = 0x6000;
    cpu.pc = 0x41A7A;
    10
}
// ===== TrioChild_OrbitDeath_041BB8  ($041BB8, 14 B, enemies/squad/children_handlers_041408.s)
/// $041BB8: move.w #$1022, d0
pub fn i_041bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BBC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041BBC: jsr $2352.l
pub fn i_041bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41BC2);
    cpu.pc = 0x2352;
    16
}
/// $041BC2: bra.w $415a2
pub fn i_041bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BC6;
    cpu.ir = 0x6000;
    cpu.pc = 0x415A2;
    10
}
// ===== SquadChild_FinalPose_041BC6  ($041BC6, 76 B, enemies/squad/children_handlers_041408.s)
/// $041BC6: move.w #$98, d1
pub fn i_041bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BCA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x98;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041BCA: jsr $236e.l
pub fn i_041bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41BD0);
    cpu.pc = 0x236E;
    16
}
/// $041BD0: lea.l $288320.l, a0
pub fn i_041bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BD6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x288320;
    20
}
/// $041BD6: jsr $28cd4.l
pub fn i_041bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41BDC);
    cpu.pc = 0x28CD4;
    16
}
/// $041BDC: lea.l $41be2(pc), a1
pub fn i_041bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x41BE2;
    16
}
/// $041BE0: move.l a1, (a6)
pub fn i_041be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041BE2: jsr $27d50.l
pub fn i_041be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41BE8);
    cpu.pc = 0x27D50;
    16
}
/// $041BE8: bcc.w $41bf2
pub fn i_041be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41BF2; return 10; }
    8
}
/// $041BEC: lea.l $415a2(pc), a1
pub fn i_041bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x415A2;
    16
}
/// $041BF0: move.l a1, (a6)
pub fn i_041bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041BF2: jsr $28d70.l
pub fn i_041bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41BF8);
    cpu.pc = 0x28D70;
    16
}
/// $041BF8: bcc.w $41c02
pub fn i_041bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41BFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41C02; return 10; }
    8
}
/// $041BFC: lea.l $415a2(pc), a1
pub fn i_041bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x415A2;
    16
}
/// $041C00: move.l a1, (a6)
pub fn i_041c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041C02: movea.l #$ffffffff, a0
pub fn i_041c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C08;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $041C08: jsr $5dd56.l
pub fn i_041c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41C0E);
    cpu.pc = 0x5DD56;
    16
}
/// $041C0E: bcc.w $41c18
pub fn i_041c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x41C18; return 10; }
    8
}
// ===== SetTaskHandler_041c12  ($041C12, 8 B, kernel/task_handlers.c)
/// $041C12: lea.l $40ef2(pc), a1
pub fn i_041c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40EF2;
    16
}
/// $041C16: move.l a1, (a6)
pub fn i_041c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $041C18: rts
pub fn i_041c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_ComputeTargetPos_041C1A  ($041C1A, 56 B, enemies/squad/wave_motion_041c1a.s)
/// $041C1A: lea.l $286124.l, a0
pub fn i_041c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C20;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x286124;
    20
}
/// $041C20: move.b $89(a6), d3
pub fn i_041c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C24;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $041C24: andi.w #$f, d3
pub fn i_041c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C28;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $041C28: lsl.w #$2, d3
pub fn i_041c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C2A;
    cpu.ir = 0xE54B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $041C2A: move.w (a0, d3.w), d0
pub fn i_041c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C2E;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $041C2E: move.w $2(a0, d3.w), d1
pub fn i_041c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C32;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $041C32: subi.w #$280, d1
pub fn i_041c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C36;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x280, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $041C36: jsr $440d0.l
pub fn i_041c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41C3C);
    cpu.pc = 0x440D0;
    16
}
/// $041C3C: cmpi.w #$110, d0
pub fn i_041c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C40;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x110, Size::Word, false);
    8
}
/// $041C40: ble.w $41c48
pub fn i_041c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C44;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x41C48; return 10; }
    8
}
/// $041C44: move.w #$110, d0
pub fn i_041c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C48;
    cpu.ir = 0x303C;
    let v1: u32 = 0x110;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041C48: move.w d0, $8a(a6)
pub fn i_041c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C4C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041C4C: move.w d1, $8c(a6)
pub fn i_041c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C50;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $041C50: rts
pub fn i_041c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C52;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_InitFormationSlot_041C52  ($041C52, 68 B, enemies/squad/wave_motion_041c1a.s)
/// $041C52: jsr $5e9b6.l
pub fn i_041c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41C58);
    cpu.pc = 0x5E9B6;
    16
}
/// $041C58: andi.b #$3, d0
pub fn i_041c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C5C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $041C5C: addi.b #$0, d0
pub fn i_041c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C60;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x0, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $041C60: move.b d0, $89(a6)
pub fn i_041c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C64;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041C64: jsr $41c1a(pc)
pub fn i_041c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41C68);
    cpu.pc = 0x41C1A;
    16
}
/// $041C68: jsr $5e9b6.l
pub fn i_041c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41C6E);
    cpu.pc = 0x5E9B6;
    16
}
/// $041C6E: lea.l $286154.l, a0
pub fn i_041c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x286154;
    20
}
/// $041C74: andi.w #$3, d0
pub fn i_041c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C78;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $041C78: lsl.w #$1, d0
pub fn i_041c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C7A;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $041C7A: move.w (a0, d0.w), $36(a6)
pub fn i_041c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C80;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad4, v3 as u16);
    22
}
/// $041C80: move.w $8a(a6), d0
pub fn i_041c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C84;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041C84: move.w $8c(a6), d1
pub fn i_041c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C88;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041C88: sub.w $22(a6), d0
pub fn i_041c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C8C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $041C8C: sub.w $24(a6), d1
pub fn i_041c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C90;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $041C90: jsr $5e018.l
pub fn i_041c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41C96);
    cpu.pc = 0x5E018;
    16
}
// ===== SetTaskB_041c96  ($041C96, 6 B, kernel/task_setters_b.c)
/// $041C96: move.b d0, $76(a6)
pub fn i_041c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C9A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041C9A: rts
pub fn i_041c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41C9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_PickSwoopState_041C9C  ($041C9C, 62 B, enemies/squad/wave_motion_041c1a.s)
/// $041C9C: jsr $5e1ea.l
pub fn i_041c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41CA2);
    cpu.pc = 0x5E1EA;
    16
}
/// $041CA2: movem.l a0, -(a7)
pub fn i_041ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CA6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $041CA6: move.b #$4, $89(a6)
pub fn i_041ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CAC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041CAC: jsr $41c1a(pc)
pub fn i_041cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CB0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41CB0);
    cpu.pc = 0x41C1A;
    16
}
/// $041CB0: movem.l (a7)+, a0
pub fn i_041cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CB4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $041CB4: move.w #$8, d0
pub fn i_041cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CB8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041CB8: move.w $8c(a6), d1
pub fn i_041cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CBC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $041CBC: cmp.w $24(a0), d1
pub fn i_041cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CC0;
    cpu.ir = 0xB268;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $041CC0: blt.w $41cc8
pub fn i_041cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CC4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x41CC8; return 10; }
    8
}
/// $041CC4: move.w #$a, d0
pub fn i_041cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CC8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $041CC8: move.b d0, $89(a6)
pub fn i_041cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CCC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $041CCC: jsr $5e9b6.l
pub fn i_041ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x41CD2);
    cpu.pc = 0x5E9B6;
    16
}
/// $041CD2: andi.b #$1, d0
pub fn i_041cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CD6;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $041CD6: add.b d0, $89(a6)
pub fn i_041cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CDA;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x89);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
// ===== JsrPcThunk_041cda  ($041CDA, 6 B, thunks/jsr_pc_thunks.c)
/// $041CDA: jsr $41c1a(pc)
pub fn i_041cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x41CDE);
    cpu.pc = 0x41C1A;
    16
}
/// $041CDE: rts
pub fn i_041cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x41CE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x040000, i_040000),
    (0x040006, i_040006),
    (0x04000A, i_04000a),
    (0x040010, i_040010),
    (0x040014, i_040014),
    (0x040018, i_040018),
    (0x04001C, i_04001c),
    (0x040020, i_040020),
    (0x040026, i_040026),
    (0x04002A, i_04002a),
    (0x04002E, i_04002e),
    (0x040032, i_040032),
    (0x040034, i_040034),
    (0x04003A, i_04003a),
    (0x04003E, i_04003e),
    (0x040042, i_040042),
    (0x040046, i_040046),
    (0x04004A, i_04004a),
    (0x04004C, i_04004c),
    (0x040052, i_040052),
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
    (0x04008C, i_04008c),
    (0x040090, i_040090),
    (0x040096, i_040096),
    (0x04009A, i_04009a),
    (0x04009C, i_04009c),
    (0x0400A0, i_0400a0),
    (0x0400A4, i_0400a4),
    (0x0400A8, i_0400a8),
    (0x0400AE, i_0400ae),
    (0x0400B4, i_0400b4),
    (0x0400B8, i_0400b8),
    (0x0400BC, i_0400bc),
    (0x0400BE, i_0400be),
    (0x0400C2, i_0400c2),
    (0x0400C6, i_0400c6),
    (0x0400CC, i_0400cc),
    (0x0400D0, i_0400d0),
    (0x0400D6, i_0400d6),
    (0x0400DA, i_0400da),
    (0x0400DC, i_0400dc),
    (0x0400E0, i_0400e0),
    (0x0400E6, i_0400e6),
    (0x0400E8, i_0400e8),
    (0x0400EC, i_0400ec),
    (0x0400F0, i_0400f0),
    (0x0400F2, i_0400f2),
    (0x0400F6, i_0400f6),
    (0x0400FA, i_0400fa),
    (0x0400FE, i_0400fe),
    (0x040102, i_040102),
    (0x040106, i_040106),
    (0x040108, i_040108),
    (0x04010E, i_04010e),
    (0x040112, i_040112),
    (0x040116, i_040116),
    (0x04011C, i_04011c),
    (0x040120, i_040120),
    (0x040124, i_040124),
    (0x04012A, i_04012a),
    (0x04012E, i_04012e),
    (0x040130, i_040130),
    (0x040136, i_040136),
    (0x04013C, i_04013c),
    (0x040140, i_040140),
    (0x040144, i_040144),
    (0x040146, i_040146),
    (0x04014A, i_04014a),
    (0x04014E, i_04014e),
    (0x040154, i_040154),
    (0x040158, i_040158),
    (0x04015E, i_04015e),
    (0x040162, i_040162),
    (0x040168, i_040168),
    (0x04016E, i_04016e),
    (0x040174, i_040174),
    (0x040178, i_040178),
    (0x04017A, i_04017a),
    (0x04017E, i_04017e),
    (0x040182, i_040182),
    (0x040184, i_040184),
    (0x040186, i_040186),
    (0x040188, i_040188),
    (0x04018A, i_04018a),
    (0x04018C, i_04018c),
    (0x040190, i_040190),
    (0x040194, i_040194),
    (0x04019A, i_04019a),
    (0x04019E, i_04019e),
    (0x0401A2, i_0401a2),
    (0x0401A4, i_0401a4),
    (0x0401A8, i_0401a8),
    (0x0401AC, i_0401ac),
    (0x0401B2, i_0401b2),
    (0x0401B6, i_0401b6),
    (0x0401BA, i_0401ba),
    (0x0401BE, i_0401be),
    (0x0401C0, i_0401c0),
    (0x0401C4, i_0401c4),
    (0x0401CA, i_0401ca),
    (0x0401D0, i_0401d0),
    (0x0401D4, i_0401d4),
    (0x0401D8, i_0401d8),
    (0x0401DA, i_0401da),
    (0x0401DE, i_0401de),
    (0x0401E2, i_0401e2),
    (0x0401E8, i_0401e8),
    (0x0401EC, i_0401ec),
    (0x0401F2, i_0401f2),
    (0x0401F6, i_0401f6),
    (0x0401F8, i_0401f8),
    (0x0401FC, i_0401fc),
    (0x040202, i_040202),
    (0x040204, i_040204),
    (0x04020A, i_04020a),
    (0x04020E, i_04020e),
    (0x040212, i_040212),
    (0x040214, i_040214),
    (0x040216, i_040216),
    (0x04021A, i_04021a),
    (0x04021E, i_04021e),
    (0x040220, i_040220),
    (0x040226, i_040226),
    (0x04022A, i_04022a),
    (0x04022E, i_04022e),
    (0x040230, i_040230),
    (0x040234, i_040234),
    (0x040238, i_040238),
    (0x04023E, i_04023e),
    (0x040242, i_040242),
    (0x040246, i_040246),
    (0x04024A, i_04024a),
    (0x04024E, i_04024e),
    (0x040250, i_040250),
    (0x040254, i_040254),
    (0x04025A, i_04025a),
    (0x04025E, i_04025e),
    (0x040262, i_040262),
    (0x040264, i_040264),
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
    (0x040392, i_040392),
    (0x040398, i_040398),
    (0x04039C, i_04039c),
    (0x04039E, i_04039e),
    (0x0403A2, i_0403a2),
    (0x0403A6, i_0403a6),
    (0x0403AC, i_0403ac),
    (0x0403B0, i_0403b0),
    (0x0403B6, i_0403b6),
    (0x0403BA, i_0403ba),
    (0x0403BE, i_0403be),
    (0x0403C4, i_0403c4),
    (0x0403C8, i_0403c8),
    (0x0403CC, i_0403cc),
    (0x0403D0, i_0403d0),
    (0x0403D4, i_0403d4),
    (0x0403D8, i_0403d8),
    (0x0403DC, i_0403dc),
    (0x0403DE, i_0403de),
    (0x0403E2, i_0403e2),
    (0x0403E4, i_0403e4),
    (0x0403E8, i_0403e8),
    (0x0403EE, i_0403ee),
    (0x0403F4, i_0403f4),
    (0x0403FA, i_0403fa),
    (0x0403FE, i_0403fe),
    (0x040402, i_040402),
    (0x040408, i_040408),
    (0x04040E, i_04040e),
    (0x040414, i_040414),
    (0x04041A, i_04041a),
    (0x040420, i_040420),
    (0x040424, i_040424),
    (0x04042A, i_04042a),
    (0x040430, i_040430),
    (0x040436, i_040436),
    (0x04043A, i_04043a),
    (0x04043E, i_04043e),
    (0x040444, i_040444),
    (0x04044A, i_04044a),
    (0x040450, i_040450),
    (0x040456, i_040456),
    (0x04045C, i_04045c),
    (0x040462, i_040462),
    (0x040468, i_040468),
    (0x04046E, i_04046e),
    (0x040474, i_040474),
    (0x04047A, i_04047a),
    (0x040480, i_040480),
    (0x040484, i_040484),
    (0x040486, i_040486),
    (0x04048A, i_04048a),
    (0x040490, i_040490),
    (0x040494, i_040494),
    (0x040498, i_040498),
    (0x04049A, i_04049a),
    (0x04049C, i_04049c),
    (0x0404A0, i_0404a0),
    (0x0404A6, i_0404a6),
    (0x0404AC, i_0404ac),
    (0x0404B2, i_0404b2),
    (0x0404B8, i_0404b8),
    (0x0404BE, i_0404be),
    (0x0404C4, i_0404c4),
    (0x0404CA, i_0404ca),
    (0x0404CE, i_0404ce),
    (0x0404D0, i_0404d0),
    (0x0404D4, i_0404d4),
    (0x0404D8, i_0404d8),
    (0x0404DC, i_0404dc),
    (0x0404E0, i_0404e0),
    (0x0404E2, i_0404e2),
    (0x0404E8, i_0404e8),
    (0x0404EC, i_0404ec),
    (0x0404EE, i_0404ee),
    (0x0404F4, i_0404f4),
    (0x0404F8, i_0404f8),
    (0x0404FA, i_0404fa),
    (0x0404FE, i_0404fe),
    (0x040500, i_040500),
    (0x040504, i_040504),
    (0x040508, i_040508),
    (0x04050C, i_04050c),
    (0x04050E, i_04050e),
    (0x040512, i_040512),
    (0x040516, i_040516),
    (0x04051A, i_04051a),
    (0x04051E, i_04051e),
    (0x040520, i_040520),
    (0x040526, i_040526),
    (0x04052A, i_04052a),
    (0x04052C, i_04052c),
    (0x040532, i_040532),
    (0x040536, i_040536),
    (0x04053A, i_04053a),
    (0x040540, i_040540),
    (0x040544, i_040544),
    (0x040546, i_040546),
    (0x04054C, i_04054c),
    (0x040550, i_040550),
    (0x040554, i_040554),
    (0x040558, i_040558),
    (0x04055C, i_04055c),
    (0x040562, i_040562),
    (0x040568, i_040568),
    (0x04056C, i_04056c),
    (0x040572, i_040572),
    (0x040576, i_040576),
    (0x04057A, i_04057a),
    (0x040580, i_040580),
    (0x040584, i_040584),
    (0x04058A, i_04058a),
    (0x040590, i_040590),
    (0x040596, i_040596),
    (0x04059A, i_04059a),
    (0x0405A0, i_0405a0),
    (0x0405A8, i_0405a8),
    (0x0405AC, i_0405ac),
    (0x0405AE, i_0405ae),
    (0x0405B2, i_0405b2),
    (0x0405B8, i_0405b8),
    (0x0405BE, i_0405be),
    (0x0405C2, i_0405c2),
    (0x0405C8, i_0405c8),
    (0x0405CE, i_0405ce),
    (0x0405D2, i_0405d2),
    (0x0405D6, i_0405d6),
    (0x0405DA, i_0405da),
    (0x0405DE, i_0405de),
    (0x0405E0, i_0405e0),
    (0x0405E4, i_0405e4),
    (0x0405E8, i_0405e8),
    (0x0405EC, i_0405ec),
    (0x0405F0, i_0405f0),
    (0x0405F6, i_0405f6),
    (0x0405FA, i_0405fa),
    (0x0405FE, i_0405fe),
    (0x040604, i_040604),
    (0x040608, i_040608),
    (0x04060E, i_04060e),
    (0x040612, i_040612),
    (0x040616, i_040616),
    (0x04061A, i_04061a),
    (0x04061E, i_04061e),
    (0x040622, i_040622),
    (0x040628, i_040628),
    (0x04062E, i_04062e),
    (0x040632, i_040632),
    (0x040636, i_040636),
    (0x04063C, i_04063c),
    (0x0406B4, i_0406b4),
    (0x0406B8, i_0406b8),
    (0x0406BA, i_0406ba),
    (0x0406BE, i_0406be),
    (0x0406C0, i_0406c0),
    (0x0406C2, i_0406c2),
    (0x0406C4, i_0406c4),
    (0x0406C6, i_0406c6),
    (0x0406CA, i_0406ca),
    (0x0406CE, i_0406ce),
    (0x0406D2, i_0406d2),
    (0x0406D6, i_0406d6),
    (0x0406DA, i_0406da),
    (0x0406E0, i_0406e0),
    (0x0406E4, i_0406e4),
    (0x0406E8, i_0406e8),
    (0x0406EA, i_0406ea),
    (0x0406F0, i_0406f0),
    (0x0406F6, i_0406f6),
    (0x0406FA, i_0406fa),
    (0x0406FE, i_0406fe),
    (0x040704, i_040704),
    (0x04070A, i_04070a),
    (0x04070E, i_04070e),
    (0x040714, i_040714),
    (0x04071A, i_04071a),
    (0x04071E, i_04071e),
    (0x040724, i_040724),
    (0x04072A, i_04072a),
    (0x04072E, i_04072e),
    (0x040734, i_040734),
    (0x04073A, i_04073a),
    (0x04073E, i_04073e),
    (0x040744, i_040744),
    (0x04074A, i_04074a),
    (0x04074E, i_04074e),
    (0x040754, i_040754),
    (0x04075A, i_04075a),
    (0x04075E, i_04075e),
    (0x040764, i_040764),
    (0x040768, i_040768),
    (0x04076C, i_04076c),
    (0x040770, i_040770),
    (0x040776, i_040776),
    (0x04077C, i_04077c),
    (0x040780, i_040780),
    (0x040782, i_040782),
    (0x040786, i_040786),
    (0x04078C, i_04078c),
    (0x040790, i_040790),
    (0x040796, i_040796),
    (0x04079A, i_04079a),
    (0x0407A0, i_0407a0),
    (0x0407A2, i_0407a2),
    (0x0407A6, i_0407a6),
    (0x0407A8, i_0407a8),
    (0x0407AE, i_0407ae),
    (0x0407B2, i_0407b2),
    (0x0407B6, i_0407b6),
    (0x0407BC, i_0407bc),
    (0x0407C2, i_0407c2),
    (0x0407C8, i_0407c8),
    (0x0407CE, i_0407ce),
    (0x0407D2, i_0407d2),
    (0x0407D4, i_0407d4),
    (0x0407D8, i_0407d8),
    (0x0407DC, i_0407dc),
    (0x0407E0, i_0407e0),
    (0x0407E4, i_0407e4),
    (0x0407E8, i_0407e8),
    (0x0407EC, i_0407ec),
    (0x0407F0, i_0407f0),
    (0x0407F4, i_0407f4),
    (0x0407F6, i_0407f6),
    (0x0407FC, i_0407fc),
    (0x040802, i_040802),
    (0x040806, i_040806),
    (0x04083E, i_04083e),
    (0x040844, i_040844),
    (0x04084A, i_04084a),
    (0x040850, i_040850),
    (0x040854, i_040854),
    (0x040856, i_040856),
    (0x04085A, i_04085a),
    (0x04085E, i_04085e),
    (0x040862, i_040862),
    (0x040866, i_040866),
    (0x04086A, i_04086a),
    (0x04086E, i_04086e),
    (0x040872, i_040872),
    (0x040874, i_040874),
    (0x04087A, i_04087a),
    (0x04087E, i_04087e),
    (0x040882, i_040882),
    (0x040886, i_040886),
    (0x04088C, i_04088c),
    (0x040892, i_040892),
    (0x040896, i_040896),
    (0x04089C, i_04089c),
    (0x0408A2, i_0408a2),
    (0x0408A8, i_0408a8),
    (0x0408AA, i_0408aa),
    (0x0408B0, i_0408b0),
    (0x0408B2, i_0408b2),
    (0x0408B6, i_0408b6),
    (0x0408BA, i_0408ba),
    (0x0408BC, i_0408bc),
    (0x0408C0, i_0408c0),
    (0x0408C4, i_0408c4),
    (0x0408CA, i_0408ca),
    (0x0408CE, i_0408ce),
    (0x0408D0, i_0408d0),
    (0x0408D4, i_0408d4),
    (0x0408DA, i_0408da),
    (0x0408DE, i_0408de),
    (0x0408E4, i_0408e4),
    (0x0408E8, i_0408e8),
    (0x0408EE, i_0408ee),
    (0x0408F4, i_0408f4),
    (0x0408F8, i_0408f8),
    (0x0408FE, i_0408fe),
    (0x040902, i_040902),
    (0x040908, i_040908),
    (0x04090E, i_04090e),
    (0x040912, i_040912),
    (0x040918, i_040918),
    (0x04091C, i_04091c),
    (0x040920, i_040920),
    (0x040922, i_040922),
    (0x040926, i_040926),
    (0x04092C, i_04092c),
    (0x040932, i_040932),
    (0x040938, i_040938),
    (0x04093E, i_04093e),
    (0x040944, i_040944),
    (0x040948, i_040948),
    (0x04094A, i_04094a),
    (0x040950, i_040950),
    (0x040954, i_040954),
    (0x040958, i_040958),
    (0x04095C, i_04095c),
    (0x04095E, i_04095e),
    (0x040962, i_040962),
    (0x040966, i_040966),
    (0x04096C, i_04096c),
    (0x040972, i_040972),
    (0x040976, i_040976),
    (0x04097A, i_04097a),
    (0x040980, i_040980),
    (0x040984, i_040984),
    (0x04098A, i_04098a),
    (0x040990, i_040990),
    (0x040994, i_040994),
    (0x04099A, i_04099a),
    (0x0409A0, i_0409a0),
    (0x0409A4, i_0409a4),
    (0x0409FC, i_0409fc),
    (0x040A00, i_040a00),
    (0x040A06, i_040a06),
    (0x040A0A, i_040a0a),
    (0x040A0C, i_040a0c),
    (0x040A12, i_040a12),
    (0x040A16, i_040a16),
    (0x040A1A, i_040a1a),
    (0x040A1C, i_040a1c),
    (0x040A22, i_040a22),
    (0x040A26, i_040a26),
    (0x040A2A, i_040a2a),
    (0x040A2E, i_040a2e),
    (0x040A34, i_040a34),
    (0x040A66, i_040a66),
    (0x040A6C, i_040a6c),
    (0x040A72, i_040a72),
    (0x040A76, i_040a76),
    (0x040A7C, i_040a7c),
    (0x040A82, i_040a82),
    (0x040A86, i_040a86),
    (0x040A8C, i_040a8c),
    (0x040A92, i_040a92),
    (0x040A96, i_040a96),
    (0x040A9C, i_040a9c),
    (0x040AA2, i_040aa2),
    (0x040AA6, i_040aa6),
    (0x040AAC, i_040aac),
    (0x040AB0, i_040ab0),
    (0x040AB4, i_040ab4),
    (0x040ABA, i_040aba),
    (0x040AC0, i_040ac0),
    (0x040AC4, i_040ac4),
    (0x040AC6, i_040ac6),
    (0x040ACC, i_040acc),
    (0x040AD0, i_040ad0),
    (0x040AD4, i_040ad4),
    (0x040ADA, i_040ada),
    (0x040ADE, i_040ade),
    (0x040AE2, i_040ae2),
    (0x040AE4, i_040ae4),
    (0x040AE8, i_040ae8),
    (0x040AEE, i_040aee),
    (0x040AF2, i_040af2),
    (0x040AF4, i_040af4),
    (0x040AFA, i_040afa),
    (0x040AFE, i_040afe),
    (0x040B04, i_040b04),
    (0x040B08, i_040b08),
    (0x040B0C, i_040b0c),
    (0x040B0E, i_040b0e),
    (0x040B32, i_040b32),
    (0x040B38, i_040b38),
    (0x040B3C, i_040b3c),
    (0x040B40, i_040b40),
    (0x040B44, i_040b44),
    (0x040B4A, i_040b4a),
    (0x040B4E, i_040b4e),
    (0x040B50, i_040b50),
    (0x040B56, i_040b56),
    (0x040B5A, i_040b5a),
    (0x040B60, i_040b60),
    (0x040B64, i_040b64),
    (0x040B66, i_040b66),
    (0x040B68, i_040b68),
    (0x040B6C, i_040b6c),
    (0x040B72, i_040b72),
    (0x040B78, i_040b78),
    (0x040B7E, i_040b7e),
    (0x040B84, i_040b84),
    (0x040B8A, i_040b8a),
    (0x040B8E, i_040b8e),
    (0x040B94, i_040b94),
    (0x040B98, i_040b98),
    (0x040B9E, i_040b9e),
    (0x040BA2, i_040ba2),
    (0x040BA8, i_040ba8),
    (0x040BAC, i_040bac),
    (0x040BB0, i_040bb0),
    (0x040BB2, i_040bb2),
    (0x040BB6, i_040bb6),
    (0x040BBC, i_040bbc),
    (0x040BC2, i_040bc2),
    (0x040BC4, i_040bc4),
    (0x040BCA, i_040bca),
    (0x040BD0, i_040bd0),
    (0x040BD6, i_040bd6),
    (0x040BDC, i_040bdc),
    (0x040BE2, i_040be2),
    (0x040BE4, i_040be4),
    (0x040BE6, i_040be6),
    (0x040BEC, i_040bec),
    (0x040BF4, i_040bf4),
    (0x040BFC, i_040bfc),
    (0x040C04, i_040c04),
    (0x040C0A, i_040c0a),
    (0x040C0E, i_040c0e),
    (0x040C14, i_040c14),
    (0x040C1A, i_040c1a),
    (0x040C1E, i_040c1e),
    (0x040C24, i_040c24),
    (0x040C2A, i_040c2a),
    (0x040C30, i_040c30),
    (0x040C36, i_040c36),
    (0x040C3C, i_040c3c),
    (0x040C42, i_040c42),
    (0x040C48, i_040c48),
    (0x040C4E, i_040c4e),
    (0x040C54, i_040c54),
    (0x040C5A, i_040c5a),
    (0x040C60, i_040c60),
    (0x040C66, i_040c66),
    (0x040C6C, i_040c6c),
    (0x040C72, i_040c72),
    (0x040C78, i_040c78),
    (0x040C7E, i_040c7e),
    (0x040C84, i_040c84),
    (0x040C8A, i_040c8a),
    (0x040C90, i_040c90),
    (0x040C96, i_040c96),
    (0x040C9A, i_040c9a),
    (0x040C9C, i_040c9c),
    (0x040CA2, i_040ca2),
    (0x040CA8, i_040ca8),
    (0x040CAE, i_040cae),
    (0x040CB2, i_040cb2),
    (0x040CB6, i_040cb6),
    (0x040CBC, i_040cbc),
    (0x040CC0, i_040cc0),
    (0x040CC2, i_040cc2),
    (0x040CC8, i_040cc8),
    (0x040CCC, i_040ccc),
    (0x040CD0, i_040cd0),
    (0x040CD2, i_040cd2),
    (0x040CD6, i_040cd6),
    (0x040CDA, i_040cda),
    (0x040CE0, i_040ce0),
    (0x040CE4, i_040ce4),
    (0x040CEA, i_040cea),
    (0x040CF0, i_040cf0),
    (0x040CF4, i_040cf4),
    (0x040CFA, i_040cfa),
    (0x040D00, i_040d00),
    (0x040D06, i_040d06),
    (0x040D0C, i_040d0c),
    (0x040D10, i_040d10),
    (0x040D14, i_040d14),
    (0x040D16, i_040d16),
    (0x040D18, i_040d18),
    (0x040D1E, i_040d1e),
    (0x040D24, i_040d24),
    (0x040D2A, i_040d2a),
    (0x040D30, i_040d30),
    (0x040D34, i_040d34),
    (0x040D3A, i_040d3a),
    (0x040D42, i_040d42),
    (0x040D4A, i_040d4a),
    (0x040D50, i_040d50),
    (0x040D56, i_040d56),
    (0x040D5C, i_040d5c),
    (0x040D62, i_040d62),
    (0x040D68, i_040d68),
    (0x040D6C, i_040d6c),
    (0x040D70, i_040d70),
    (0x040D76, i_040d76),
    (0x040D7A, i_040d7a),
    (0x040D80, i_040d80),
    (0x040D86, i_040d86),
    (0x040D8C, i_040d8c),
    (0x040D90, i_040d90),
    (0x040D96, i_040d96),
    (0x040D9C, i_040d9c),
    (0x040DA2, i_040da2),
    (0x040DA8, i_040da8),
    (0x040DAE, i_040dae),
    (0x040DB4, i_040db4),
    (0x040DBA, i_040dba),
    (0x040DC0, i_040dc0),
    (0x040DC6, i_040dc6),
    (0x040DCC, i_040dcc),
    (0x040DD2, i_040dd2),
    (0x040DD6, i_040dd6),
    (0x040DDC, i_040ddc),
    (0x040DE2, i_040de2),
    (0x040DEA, i_040dea),
    (0x040DEE, i_040dee),
    (0x040DF2, i_040df2),
    (0x040DF6, i_040df6),
    (0x040DF8, i_040df8),
    (0x040DFE, i_040dfe),
    (0x040E02, i_040e02),
    (0x040E06, i_040e06),
    (0x040E08, i_040e08),
    (0x040E0C, i_040e0c),
    (0x040E0E, i_040e0e),
    (0x040E12, i_040e12),
    (0x040E18, i_040e18),
    (0x040E1E, i_040e1e),
    (0x040E24, i_040e24),
    (0x040E28, i_040e28),
    (0x040E2A, i_040e2a),
    (0x040E30, i_040e30),
    (0x040E34, i_040e34),
    (0x040E38, i_040e38),
    (0x040E3E, i_040e3e),
    (0x040E42, i_040e42),
    (0x040E48, i_040e48),
    (0x040E4C, i_040e4c),
    (0x040E50, i_040e50),
    (0x040E52, i_040e52),
    (0x040E54, i_040e54),
    (0x040E5A, i_040e5a),
    (0x040E5E, i_040e5e),
    (0x040E64, i_040e64),
    (0x040E68, i_040e68),
    (0x040E6A, i_040e6a),
    (0x040E70, i_040e70),
    (0x040E74, i_040e74),
    (0x040E78, i_040e78),
    (0x040E7E, i_040e7e),
    (0x040E82, i_040e82),
    (0x040E86, i_040e86),
    (0x040E8C, i_040e8c),
    (0x040E90, i_040e90),
    (0x040E92, i_040e92),
    (0x040E98, i_040e98),
    (0x040E9C, i_040e9c),
    (0x040EA2, i_040ea2),
    (0x040EA8, i_040ea8),
    (0x040EAE, i_040eae),
    (0x040EB2, i_040eb2),
    (0x040EB6, i_040eb6),
    (0x040EB8, i_040eb8),
    (0x040EBA, i_040eba),
    (0x040EC0, i_040ec0),
    (0x040EC6, i_040ec6),
    (0x040ECC, i_040ecc),
    (0x040ED2, i_040ed2),
    (0x040ED8, i_040ed8),
    (0x040EDE, i_040ede),
    (0x040EE4, i_040ee4),
    (0x040EEA, i_040eea),
    (0x040EF0, i_040ef0),
    (0x040EF2, i_040ef2),
    (0x040EF8, i_040ef8),
    (0x040F00, i_040f00),
    (0x040F04, i_040f04),
    (0x040F08, i_040f08),
    (0x040F0A, i_040f0a),
    (0x040F10, i_040f10),
    (0x040F14, i_040f14),
    (0x040F1A, i_040f1a),
    (0x040F20, i_040f20),
    (0x040F26, i_040f26),
    (0x040F2C, i_040f2c),
    (0x040F32, i_040f32),
    (0x040F36, i_040f36),
    (0x040F3A, i_040f3a),
    (0x040F40, i_040f40),
    (0x040F42, i_040f42),
    (0x040F46, i_040f46),
    (0x040F4C, i_040f4c),
    (0x040F50, i_040f50),
    (0x040F56, i_040f56),
    (0x040F5A, i_040f5a),
    (0x040F5C, i_040f5c),
    (0x040F60, i_040f60),
    (0x040F64, i_040f64),
    (0x040F6A, i_040f6a),
    (0x040F6E, i_040f6e),
    (0x040F74, i_040f74),
    (0x040F7A, i_040f7a),
    (0x040F7E, i_040f7e),
    (0x040F82, i_040f82),
    (0x040F86, i_040f86),
    (0x040F8A, i_040f8a),
    (0x040F8E, i_040f8e),
    (0x040F94, i_040f94),
    (0x040F96, i_040f96),
    (0x040F9A, i_040f9a),
    (0x040FA0, i_040fa0),
    (0x040FA4, i_040fa4),
    (0x040FAA, i_040faa),
    (0x040FAE, i_040fae),
    (0x040FB0, i_040fb0),
    (0x040FB4, i_040fb4),
    (0x040FB8, i_040fb8),
    (0x040FBE, i_040fbe),
    (0x040FC2, i_040fc2),
    (0x040FC8, i_040fc8),
    (0x040FCE, i_040fce),
    (0x040FD2, i_040fd2),
    (0x040FD6, i_040fd6),
    (0x040FDC, i_040fdc),
    (0x040FE0, i_040fe0),
    (0x040FE6, i_040fe6),
    (0x040FEA, i_040fea),
    (0x0410BA, i_0410ba),
    (0x0410C0, i_0410c0),
    (0x0410C4, i_0410c4),
    (0x0410CA, i_0410ca),
    (0x0410D0, i_0410d0),
    (0x0410D6, i_0410d6),
    (0x0410DA, i_0410da),
    (0x0410DC, i_0410dc),
    (0x0410E0, i_0410e0),
    (0x0410E6, i_0410e6),
    (0x0410EA, i_0410ea),
    (0x0410EE, i_0410ee),
    (0x0410F4, i_0410f4),
    (0x0410FA, i_0410fa),
    (0x041100, i_041100),
    (0x041106, i_041106),
    (0x04110A, i_04110a),
    (0x04110E, i_04110e),
    (0x041112, i_041112),
    (0x041118, i_041118),
    (0x04111C, i_04111c),
    (0x04111E, i_04111e),
    (0x041122, i_041122),
    (0x041126, i_041126),
    (0x04112C, i_04112c),
    (0x041130, i_041130),
    (0x041136, i_041136),
    (0x04113A, i_04113a),
    (0x04113E, i_04113e),
    (0x041142, i_041142),
    (0x041146, i_041146),
    (0x04114A, i_04114a),
    (0x04114E, i_04114e),
    (0x041152, i_041152),
    (0x041158, i_041158),
    (0x04115C, i_04115c),
    (0x041160, i_041160),
    (0x041164, i_041164),
    (0x041168, i_041168),
    (0x04116C, i_04116c),
    (0x041170, i_041170),
    (0x041174, i_041174),
    (0x041176, i_041176),
    (0x04117A, i_04117a),
    (0x04117E, i_04117e),
    (0x041182, i_041182),
    (0x041188, i_041188),
    (0x04118E, i_04118e),
    (0x041192, i_041192),
    (0x041196, i_041196),
    (0x04119C, i_04119c),
    (0x0411A0, i_0411a0),
    (0x0411A4, i_0411a4),
    (0x0411AA, i_0411aa),
    (0x0411B0, i_0411b0),
    (0x0411B6, i_0411b6),
    (0x0411BA, i_0411ba),
    (0x0411BC, i_0411bc),
    (0x0411C0, i_0411c0),
    (0x0411C6, i_0411c6),
    (0x0411CA, i_0411ca),
    (0x0411D0, i_0411d0),
    (0x0411D4, i_0411d4),
    (0x0411D8, i_0411d8),
    (0x0411DC, i_0411dc),
    (0x0411E2, i_0411e2),
    (0x0411E6, i_0411e6),
    (0x0411EC, i_0411ec),
    (0x0411F0, i_0411f0),
    (0x0411F4, i_0411f4),
    (0x0411F8, i_0411f8),
    (0x0411FE, i_0411fe),
    (0x041202, i_041202),
    (0x04120A, i_04120a),
    (0x04120E, i_04120e),
    (0x041210, i_041210),
    (0x041214, i_041214),
    (0x041218, i_041218),
    (0x04121C, i_04121c),
    (0x041222, i_041222),
    (0x041224, i_041224),
    (0x041228, i_041228),
    (0x04122E, i_04122e),
    (0x041232, i_041232),
    (0x041238, i_041238),
    (0x04123C, i_04123c),
    (0x04123E, i_04123e),
    (0x041242, i_041242),
    (0x041248, i_041248),
    (0x04124E, i_04124e),
    (0x041252, i_041252),
    (0x041258, i_041258),
    (0x04125C, i_04125c),
    (0x041262, i_041262),
    (0x041266, i_041266),
    (0x04126C, i_04126c),
    (0x041270, i_041270),
    (0x041274, i_041274),
    (0x041276, i_041276),
    (0x04127A, i_04127a),
    (0x04127E, i_04127e),
    (0x041284, i_041284),
    (0x04128A, i_04128a),
    (0x041290, i_041290),
    (0x041294, i_041294),
    (0x041296, i_041296),
    (0x04129A, i_04129a),
    (0x0412A0, i_0412a0),
    (0x0412A4, i_0412a4),
    (0x0412AA, i_0412aa),
    (0x0412AE, i_0412ae),
    (0x0412B2, i_0412b2),
    (0x0412B4, i_0412b4),
    (0x0412B8, i_0412b8),
    (0x0412BC, i_0412bc),
    (0x0412C2, i_0412c2),
    (0x0412C8, i_0412c8),
    (0x0412CE, i_0412ce),
    (0x0412D2, i_0412d2),
    (0x0412D4, i_0412d4),
    (0x0412D8, i_0412d8),
    (0x0412DE, i_0412de),
    (0x0412E2, i_0412e2),
    (0x0412E8, i_0412e8),
    (0x0412EE, i_0412ee),
    (0x0412F4, i_0412f4),
    (0x0412F8, i_0412f8),
    (0x0412FC, i_0412fc),
    (0x041302, i_041302),
    (0x041306, i_041306),
    (0x04130A, i_04130a),
    (0x041310, i_041310),
    (0x041314, i_041314),
    (0x041316, i_041316),
    (0x04131C, i_04131c),
    (0x041320, i_041320),
    (0x041324, i_041324),
    (0x041326, i_041326),
    (0x041328, i_041328),
    (0x04132E, i_04132e),
    (0x041336, i_041336),
    (0x04133A, i_04133a),
    (0x04133E, i_04133e),
    (0x041344, i_041344),
    (0x04134A, i_04134a),
    (0x041350, i_041350),
    (0x041354, i_041354),
    (0x041356, i_041356),
    (0x04135A, i_04135a),
    (0x041360, i_041360),
    (0x041364, i_041364),
    (0x04136A, i_04136a),
    (0x04136E, i_04136e),
    (0x041372, i_041372),
    (0x041376, i_041376),
    (0x04137C, i_04137c),
    (0x041380, i_041380),
    (0x041386, i_041386),
    (0x04138A, i_04138a),
    (0x04138E, i_04138e),
    (0x041390, i_041390),
    (0x041394, i_041394),
    (0x04139C, i_04139c),
    (0x0413A0, i_0413a0),
    (0x0413A2, i_0413a2),
    (0x0413A6, i_0413a6),
    (0x0413AA, i_0413aa),
    (0x0413AE, i_0413ae),
    (0x0413B4, i_0413b4),
    (0x0413B6, i_0413b6),
    (0x0413BA, i_0413ba),
    (0x0413C0, i_0413c0),
    (0x0413C4, i_0413c4),
    (0x0413CA, i_0413ca),
    (0x0413CE, i_0413ce),
    (0x0413D0, i_0413d0),
    (0x0413D4, i_0413d4),
    (0x0413DA, i_0413da),
    (0x0413DE, i_0413de),
    (0x0413E4, i_0413e4),
    (0x0413E8, i_0413e8),
    (0x0413EE, i_0413ee),
    (0x0413F2, i_0413f2),
    (0x0413F8, i_0413f8),
    (0x0413FC, i_0413fc),
    (0x041400, i_041400),
    (0x041402, i_041402),
    (0x041406, i_041406),
    (0x041408, i_041408),
    (0x04140C, i_04140c),
    (0x041412, i_041412),
    (0x041418, i_041418),
    (0x04141E, i_04141e),
    (0x041424, i_041424),
    (0x04142A, i_04142a),
    (0x041430, i_041430),
    (0x041436, i_041436),
    (0x04143A, i_04143a),
    (0x04143E, i_04143e),
    (0x041444, i_041444),
    (0x04144A, i_04144a),
    (0x041450, i_041450),
    (0x041454, i_041454),
    (0x041456, i_041456),
    (0x04145C, i_04145c),
    (0x041460, i_041460),
    (0x041464, i_041464),
    (0x041466, i_041466),
    (0x04146C, i_04146c),
    (0x041472, i_041472),
    (0x041476, i_041476),
    (0x04147A, i_04147a),
    (0x04147E, i_04147e),
    (0x041482, i_041482),
    (0x041486, i_041486),
    (0x041488, i_041488),
    (0x04148A, i_04148a),
    (0x041490, i_041490),
    (0x041496, i_041496),
    (0x04149C, i_04149c),
    (0x0414A2, i_0414a2),
    (0x0414A8, i_0414a8),
    (0x0414AE, i_0414ae),
    (0x0414B4, i_0414b4),
    (0x0414BA, i_0414ba),
    (0x0414C0, i_0414c0),
    (0x0414C6, i_0414c6),
    (0x0414CC, i_0414cc),
    (0x0414D2, i_0414d2),
    (0x0414D8, i_0414d8),
    (0x0414DC, i_0414dc),
    (0x0414DE, i_0414de),
    (0x0414E4, i_0414e4),
    (0x0414E8, i_0414e8),
    (0x0414EE, i_0414ee),
    (0x0414F6, i_0414f6),
    (0x0414FC, i_0414fc),
    (0x041502, i_041502),
    (0x041506, i_041506),
    (0x041508, i_041508),
    (0x04150E, i_04150e),
    (0x041514, i_041514),
    (0x04151A, i_04151a),
    (0x04151E, i_04151e),
    (0x041524, i_041524),
    (0x041528, i_041528),
    (0x04152E, i_04152e),
    (0x041534, i_041534),
    (0x04153A, i_04153a),
    (0x041540, i_041540),
    (0x041546, i_041546),
    (0x04154C, i_04154c),
    (0x041552, i_041552),
    (0x041558, i_041558),
    (0x04155A, i_04155a),
    (0x041560, i_041560),
    (0x041564, i_041564),
    (0x041566, i_041566),
    (0x04156C, i_04156c),
    (0x041572, i_041572),
    (0x041576, i_041576),
    (0x04157A, i_04157a),
    (0x04157C, i_04157c),
    (0x04157E, i_04157e),
    (0x041586, i_041586),
    (0x04158A, i_04158a),
    (0x041590, i_041590),
    (0x041596, i_041596),
    (0x04159C, i_04159c),
    (0x0415A2, i_0415a2),
    (0x0415A6, i_0415a6),
    (0x0415AC, i_0415ac),
    (0x0415B2, i_0415b2),
    (0x0415B8, i_0415b8),
    (0x0415C0, i_0415c0),
    (0x0415C6, i_0415c6),
    (0x0415CA, i_0415ca),
    (0x0415D0, i_0415d0),
    (0x0415D6, i_0415d6),
    (0x0415DC, i_0415dc),
    (0x0415E0, i_0415e0),
    (0x0415E6, i_0415e6),
    (0x0415EC, i_0415ec),
    (0x0415F2, i_0415f2),
    (0x0415F6, i_0415f6),
    (0x0415F8, i_0415f8),
    (0x0415FE, i_0415fe),
    (0x041604, i_041604),
    (0x041608, i_041608),
    (0x04160C, i_04160c),
    (0x04160E, i_04160e),
    (0x041614, i_041614),
    (0x04161A, i_04161a),
    (0x04161E, i_04161e),
    (0x041622, i_041622),
    (0x041624, i_041624),
    (0x041626, i_041626),
    (0x04162C, i_04162c),
    (0x041630, i_041630),
    (0x041634, i_041634),
    (0x04163A, i_04163a),
    (0x04163E, i_04163e),
    (0x041644, i_041644),
    (0x041648, i_041648),
    (0x04164E, i_04164e),
    (0x041654, i_041654),
    (0x041658, i_041658),
    (0x04165E, i_04165e),
    (0x041664, i_041664),
    (0x041668, i_041668),
    (0x04166E, i_04166e),
    (0x041672, i_041672),
    (0x041674, i_041674),
    (0x041678, i_041678),
    (0x04167C, i_04167c),
    (0x041684, i_041684),
    (0x04168C, i_04168c),
    (0x041692, i_041692),
    (0x041696, i_041696),
    (0x04169C, i_04169c),
    (0x0416A2, i_0416a2),
    (0x0416A6, i_0416a6),
    (0x0416AC, i_0416ac),
    (0x0416AE, i_0416ae),
    (0x0416B2, i_0416b2),
    (0x0416B8, i_0416b8),
    (0x0416BC, i_0416bc),
    (0x0416C2, i_0416c2),
    (0x0416C6, i_0416c6),
    (0x0416CC, i_0416cc),
    (0x0416D2, i_0416d2),
    (0x0416D8, i_0416d8),
    (0x0416DC, i_0416dc),
    (0x0416DE, i_0416de),
    (0x0416E2, i_0416e2),
    (0x0416E6, i_0416e6),
    (0x0416EC, i_0416ec),
    (0x0416F2, i_0416f2),
    (0x0416F6, i_0416f6),
    (0x0416FA, i_0416fa),
    (0x0416FC, i_0416fc),
    (0x041700, i_041700),
    (0x041706, i_041706),
    (0x04170A, i_04170a),
    (0x04170E, i_04170e),
    (0x041710, i_041710),
    (0x041716, i_041716),
    (0x04171A, i_04171a),
    (0x04171E, i_04171e),
    (0x041724, i_041724),
    (0x04172A, i_04172a),
    (0x04172E, i_04172e),
    (0x041732, i_041732),
    (0x041734, i_041734),
    (0x04173A, i_04173a),
    (0x041740, i_041740),
    (0x041744, i_041744),
    (0x041748, i_041748),
    (0x04174A, i_04174a),
    (0x041750, i_041750),
    (0x041756, i_041756),
    (0x04175A, i_04175a),
    (0x04175E, i_04175e),
    (0x041760, i_041760),
    (0x041762, i_041762),
    (0x041766, i_041766),
    (0x04176C, i_04176c),
    (0x041770, i_041770),
    (0x041774, i_041774),
    (0x04177C, i_04177c),
    (0x041782, i_041782),
    (0x041788, i_041788),
    (0x041790, i_041790),
    (0x041794, i_041794),
    (0x04179A, i_04179a),
    (0x0417A0, i_0417a0),
    (0x0417A6, i_0417a6),
    (0x0417AC, i_0417ac),
    (0x0417B2, i_0417b2),
    (0x0417B8, i_0417b8),
    (0x0417BE, i_0417be),
    (0x0417C4, i_0417c4),
    (0x0417C8, i_0417c8),
    (0x0417CA, i_0417ca),
    (0x0417CE, i_0417ce),
    (0x0417D2, i_0417d2),
    (0x0417D6, i_0417d6),
    (0x0417DA, i_0417da),
    (0x0417E0, i_0417e0),
    (0x0417E4, i_0417e4),
    (0x0417EA, i_0417ea),
    (0x0417F0, i_0417f0),
    (0x0417F4, i_0417f4),
    (0x0417F8, i_0417f8),
    (0x0417FE, i_0417fe),
    (0x041804, i_041804),
    (0x04180A, i_04180a),
    (0x04180E, i_04180e),
    (0x041814, i_041814),
    (0x041818, i_041818),
    (0x04181A, i_04181a),
    (0x04181C, i_04181c),
    (0x041822, i_041822),
    (0x041828, i_041828),
    (0x04182C, i_04182c),
    (0x041830, i_041830),
    (0x041832, i_041832),
    (0x041838, i_041838),
    (0x04183E, i_04183e),
    (0x041844, i_041844),
    (0x041848, i_041848),
    (0x04184C, i_04184c),
    (0x04184E, i_04184e),
    (0x041850, i_041850),
    (0x041856, i_041856),
    (0x04185A, i_04185a),
    (0x04185C, i_04185c),
    (0x041862, i_041862),
    (0x041866, i_041866),
    (0x04186A, i_04186a),
    (0x04186E, i_04186e),
    (0x041870, i_041870),
    (0x041872, i_041872),
    (0x041874, i_041874),
    (0x041876, i_041876),
    (0x041878, i_041878),
    (0x04187A, i_04187a),
    (0x04187E, i_04187e),
    (0x041882, i_041882),
    (0x041884, i_041884),
    (0x041888, i_041888),
    (0x04188C, i_04188c),
    (0x041890, i_041890),
    (0x041894, i_041894),
    (0x04189A, i_04189a),
    (0x04189E, i_04189e),
    (0x0418A4, i_0418a4),
    (0x0418AA, i_0418aa),
    (0x0418B0, i_0418b0),
    (0x0418B6, i_0418b6),
    (0x0418BC, i_0418bc),
    (0x0418C0, i_0418c0),
    (0x0418C2, i_0418c2),
    (0x0418C8, i_0418c8),
    (0x0418CC, i_0418cc),
    (0x0418D0, i_0418d0),
    (0x0418D2, i_0418d2),
    (0x0418D6, i_0418d6),
    (0x0418DC, i_0418dc),
    (0x0418E0, i_0418e0),
    (0x0418E6, i_0418e6),
    (0x0418EA, i_0418ea),
    (0x0418EE, i_0418ee),
    (0x0418F0, i_0418f0),
    (0x0418F4, i_0418f4),
    (0x0418F8, i_0418f8),
    (0x0418FC, i_0418fc),
    (0x041902, i_041902),
    (0x041908, i_041908),
    (0x04190E, i_04190e),
    (0x041912, i_041912),
    (0x041916, i_041916),
    (0x041918, i_041918),
    (0x04191A, i_04191a),
    (0x041920, i_041920),
    (0x041924, i_041924),
    (0x04192A, i_04192a),
    (0x041930, i_041930),
    (0x041936, i_041936),
    (0x04193C, i_04193c),
    (0x041942, i_041942),
    (0x041948, i_041948),
    (0x04194E, i_04194e),
    (0x041952, i_041952),
    (0x041958, i_041958),
    (0x04195E, i_04195e),
    (0x041962, i_041962),
    (0x041964, i_041964),
    (0x04196A, i_04196a),
    (0x04196E, i_04196e),
    (0x041972, i_041972),
    (0x041974, i_041974),
    (0x04197A, i_04197a),
    (0x041980, i_041980),
    (0x041986, i_041986),
    (0x04198A, i_04198a),
    (0x04198E, i_04198e),
    (0x041990, i_041990),
    (0x041996, i_041996),
    (0x04199C, i_04199c),
    (0x0419A0, i_0419a0),
    (0x0419A4, i_0419a4),
    (0x0419A6, i_0419a6),
    (0x0419AC, i_0419ac),
    (0x0419B2, i_0419b2),
    (0x0419B6, i_0419b6),
    (0x0419BA, i_0419ba),
    (0x0419BC, i_0419bc),
    (0x0419BE, i_0419be),
    (0x0419C2, i_0419c2),
    (0x0419C8, i_0419c8),
    (0x0419CC, i_0419cc),
    (0x0419D0, i_0419d0),
    (0x0419D6, i_0419d6),
    (0x0419DC, i_0419dc),
    (0x0419E2, i_0419e2),
    (0x0419E6, i_0419e6),
    (0x0419EC, i_0419ec),
    (0x0419F2, i_0419f2),
    (0x0419F8, i_0419f8),
    (0x0419FC, i_0419fc),
    (0x041A02, i_041a02),
    (0x041A06, i_041a06),
    (0x041A0C, i_041a0c),
    (0x041A10, i_041a10),
    (0x041A16, i_041a16),
    (0x041A1C, i_041a1c),
    (0x041A22, i_041a22),
    (0x041A26, i_041a26),
    (0x041A2C, i_041a2c),
    (0x041A32, i_041a32),
    (0x041A38, i_041a38),
    (0x041A3C, i_041a3c),
    (0x041A40, i_041a40),
    (0x041A46, i_041a46),
    (0x041A48, i_041a48),
    (0x041A4E, i_041a4e),
    (0x041A52, i_041a52),
    (0x041A56, i_041a56),
    (0x041A5A, i_041a5a),
    (0x041A5C, i_041a5c),
    (0x041A62, i_041a62),
    (0x041A66, i_041a66),
    (0x041A6A, i_041a6a),
    (0x041A6C, i_041a6c),
    (0x041A70, i_041a70),
    (0x041A74, i_041a74),
    (0x041A7A, i_041a7a),
    (0x041A7E, i_041a7e),
    (0x041A82, i_041a82),
    (0x041A86, i_041a86),
    (0x041A8C, i_041a8c),
    (0x041A90, i_041a90),
    (0x041A94, i_041a94),
    (0x041A96, i_041a96),
    (0x041A9C, i_041a9c),
    (0x041AA2, i_041aa2),
    (0x041AA8, i_041aa8),
    (0x041AAC, i_041aac),
    (0x041AB0, i_041ab0),
    (0x041AB2, i_041ab2),
    (0x041AB4, i_041ab4),
    (0x041AB8, i_041ab8),
    (0x041ABE, i_041abe),
    (0x041AC4, i_041ac4),
    (0x041ACA, i_041aca),
    (0x041AD0, i_041ad0),
    (0x041AD6, i_041ad6),
    (0x041ADA, i_041ada),
    (0x041AE0, i_041ae0),
    (0x041AE4, i_041ae4),
    (0x041AEA, i_041aea),
    (0x041AEE, i_041aee),
    (0x041AF4, i_041af4),
    (0x041AFA, i_041afa),
    (0x041B00, i_041b00),
    (0x041B02, i_041b02),
    (0x041B08, i_041b08),
    (0x041B0E, i_041b0e),
    (0x041B14, i_041b14),
    (0x041B18, i_041b18),
    (0x041B1E, i_041b1e),
    (0x041B24, i_041b24),
    (0x041B2A, i_041b2a),
    (0x041B2E, i_041b2e),
    (0x041B30, i_041b30),
    (0x041B34, i_041b34),
    (0x041B3A, i_041b3a),
    (0x041B3E, i_041b3e),
    (0x041B42, i_041b42),
    (0x041B48, i_041b48),
    (0x041B4C, i_041b4c),
    (0x041B52, i_041b52),
    (0x041B56, i_041b56),
    (0x041B5C, i_041b5c),
    (0x041B60, i_041b60),
    (0x041B64, i_041b64),
    (0x041B68, i_041b68),
    (0x041B6E, i_041b6e),
    (0x041B72, i_041b72),
    (0x041B76, i_041b76),
    (0x041B78, i_041b78),
    (0x041B7C, i_041b7c),
    (0x041B7E, i_041b7e),
    (0x041B82, i_041b82),
    (0x041B88, i_041b88),
    (0x041B8E, i_041b8e),
    (0x041B94, i_041b94),
    (0x041B98, i_041b98),
    (0x041B9C, i_041b9c),
    (0x041B9E, i_041b9e),
    (0x041BA4, i_041ba4),
    (0x041BAA, i_041baa),
    (0x041BAE, i_041bae),
    (0x041BB2, i_041bb2),
    (0x041BB4, i_041bb4),
    (0x041BB8, i_041bb8),
    (0x041BBC, i_041bbc),
    (0x041BC2, i_041bc2),
    (0x041BC6, i_041bc6),
    (0x041BCA, i_041bca),
    (0x041BD0, i_041bd0),
    (0x041BD6, i_041bd6),
    (0x041BDC, i_041bdc),
    (0x041BE0, i_041be0),
    (0x041BE2, i_041be2),
    (0x041BE8, i_041be8),
    (0x041BEC, i_041bec),
    (0x041BF0, i_041bf0),
    (0x041BF2, i_041bf2),
    (0x041BF8, i_041bf8),
    (0x041BFC, i_041bfc),
    (0x041C00, i_041c00),
    (0x041C02, i_041c02),
    (0x041C08, i_041c08),
    (0x041C0E, i_041c0e),
    (0x041C12, i_041c12),
    (0x041C16, i_041c16),
    (0x041C18, i_041c18),
    (0x041C1A, i_041c1a),
    (0x041C20, i_041c20),
    (0x041C24, i_041c24),
    (0x041C28, i_041c28),
    (0x041C2A, i_041c2a),
    (0x041C2E, i_041c2e),
    (0x041C32, i_041c32),
    (0x041C36, i_041c36),
    (0x041C3C, i_041c3c),
    (0x041C40, i_041c40),
    (0x041C44, i_041c44),
    (0x041C48, i_041c48),
    (0x041C4C, i_041c4c),
    (0x041C50, i_041c50),
    (0x041C52, i_041c52),
    (0x041C58, i_041c58),
    (0x041C5C, i_041c5c),
    (0x041C60, i_041c60),
    (0x041C64, i_041c64),
    (0x041C68, i_041c68),
    (0x041C6E, i_041c6e),
    (0x041C74, i_041c74),
    (0x041C78, i_041c78),
    (0x041C7A, i_041c7a),
    (0x041C80, i_041c80),
    (0x041C84, i_041c84),
    (0x041C88, i_041c88),
    (0x041C8C, i_041c8c),
    (0x041C90, i_041c90),
    (0x041C96, i_041c96),
    (0x041C9A, i_041c9a),
    (0x041C9C, i_041c9c),
    (0x041CA2, i_041ca2),
    (0x041CA6, i_041ca6),
    (0x041CAC, i_041cac),
    (0x041CB0, i_041cb0),
    (0x041CB4, i_041cb4),
    (0x041CB8, i_041cb8),
    (0x041CBC, i_041cbc),
    (0x041CC0, i_041cc0),
    (0x041CC4, i_041cc4),
    (0x041CC8, i_041cc8),
    (0x041CCC, i_041ccc),
    (0x041CD2, i_041cd2),
    (0x041CD6, i_041cd6),
    (0x041CDA, i_041cda),
    (0x041CDE, i_041cde),
];
