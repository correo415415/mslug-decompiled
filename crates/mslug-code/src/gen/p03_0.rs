// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p03_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


// ===== Chain3_Init_030002  ($030002, 94 B, slug/vehicle_02dd20.s)
/// $030002: tst.b $98(a6)
pub fn i_030002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30006;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $030006: bne.w $30010
pub fn i_030006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3000A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30010; return 10; }
    8
}
/// $03000A: bset.b #$2, $5b(a6)
pub fn i_03000a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30010;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $030010: bset.b #$1, $6b(a6)
pub fn i_030010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30016;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $030016: bset.b #$6, $12(a6)
pub fn i_030016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3001C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03001C: move.w #$f000, d0
pub fn i_03001c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30020;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030020: jsr $28134.l
pub fn i_030020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30026);
    cpu.pc = 0x28134;
    16
}
/// $030026: andi.w #$ffe3, $38(a6)
pub fn i_030026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3002C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03002C: ori.w #$10, $38(a6)
pub fn i_03002c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30032;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $030032: clr.w $2c(a6)
pub fn i_030032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30036;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $030036: clr.w $2e(a6)
pub fn i_030036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3003A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03003A: clr.w $8e(a6)
pub fn i_03003a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3003E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03003E: clr.b $5c(a6)
pub fn i_03003e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30042;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $030042: jsr $27eba.l
pub fn i_030042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30048;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30048);
    cpu.pc = 0x27EBA;
    16
}
/// $030048: bcc.w $30054
pub fn i_030048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3004C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x30054; return 10; }
    8
}
/// $03004C: clr.b $84(a6)
pub fn i_03004c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30050;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $030050: bra.w $3005a
pub fn i_030050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30054;
    cpu.ir = 0x6000;
    cpu.pc = 0x3005A;
    10
}
/// $030054: move.b #$1, $84(a6)
pub fn i_030054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3005A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03005A: lea.l $29dc90.l, a0
pub fn i_03005a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30060;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29DC90;
    20
}
// ===== JsrAbsThunk_030060  ($030060, 8 B, thunks/jsr_abs_thunks.c)
/// $030060: jsr $28cd4.l
pub fn i_030060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30066;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30066);
    cpu.pc = 0x28CD4;
    16
}
/// $030066: rts
pub fn i_030066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30068;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chain3_TplA_030068  ($030068, 82 B, slug/vehicle_02dd20.s)
/// $030068: move.w #$7, d1
pub fn i_030068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3006C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03006C: jsr $236e.l
pub fn i_03006c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30072;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30072);
    cpu.pc = 0x236E;
    16
}
/// $030072: move.w #$9, d1
pub fn i_030072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30076;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030076: jsr $236e.l
pub fn i_030076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3007C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3007C);
    cpu.pc = 0x236E;
    16
}
/// $03007C: move.w #$8, d1
pub fn i_03007c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30080;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030080: jsr $236e.l
pub fn i_030080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30086;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30086);
    cpu.pc = 0x236E;
    16
}
/// $030086: move.w #$6, d1
pub fn i_030086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3008A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03008A: jsr $236e.l
pub fn i_03008a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30090;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30090);
    cpu.pc = 0x236E;
    16
}
/// $030090: jsr $30002(pc)
pub fn i_030090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30094;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x30094);
    cpu.pc = 0x30002;
    16
}
/// $030094: lea.l $279668.l, a1
pub fn i_030094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3009A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x279668;
    20
}
/// $03009A: move.w $2(a1), d0
pub fn i_03009a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3009E;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03009E: move.w d0, $7c(a6)
pub fn i_03009e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300A2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0300A2: add.w $22(a6), d0
pub fn i_0300a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300A6;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0300A6: move.w d0, $22(a6)
pub fn i_0300a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300AA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0300AA: move.b #$7f, $6a(a6)
pub fn i_0300aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300B0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0300B0: move.w #$0, $7e(a6)
pub fn i_0300b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0300B6: bra.w $301f0
pub fn i_0300b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x301F0;
    10
}
// ===== Chain3_TplB_0300ba  ($0300BA, 82 B, slug/vehicle_02dd20.s)
/// $0300BA: move.w #$7, d1
pub fn i_0300ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300BE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0300BE: jsr $236e.l
pub fn i_0300be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x300C4);
    cpu.pc = 0x236E;
    16
}
/// $0300C4: move.w #$9, d1
pub fn i_0300c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0300C8: jsr $236e.l
pub fn i_0300c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x300CE);
    cpu.pc = 0x236E;
    16
}
/// $0300CE: move.w #$8, d1
pub fn i_0300ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300D2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0300D2: jsr $236e.l
pub fn i_0300d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x300D8);
    cpu.pc = 0x236E;
    16
}
/// $0300D8: move.w #$6, d1
pub fn i_0300d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300DC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0300DC: jsr $236e.l
pub fn i_0300dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x300E2);
    cpu.pc = 0x236E;
    16
}
/// $0300E2: jsr $30002(pc)
pub fn i_0300e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x300E6);
    cpu.pc = 0x30002;
    16
}
/// $0300E6: lea.l $27966e.l, a1
pub fn i_0300e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300EC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x27966E;
    20
}
/// $0300EC: move.w $2(a1), d0
pub fn i_0300ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300F0;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0300F0: move.w d0, $7c(a6)
pub fn i_0300f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300F4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0300F4: add.w $22(a6), d0
pub fn i_0300f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300F8;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0300F8: move.w d0, $22(a6)
pub fn i_0300f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x300FC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0300FC: move.b #$ff, $6a(a6)
pub fn i_0300fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30102;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $030102: move.w #$8000, $7e(a6)
pub fn i_030102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30108;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030108: bra.w $301f0
pub fn i_030108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3010C;
    cpu.ir = 0x6000;
    cpu.pc = 0x301F0;
    10
}
// ===== Chain3_TplC_03010c  ($03010C, 220 B, slug/vehicle_02dd20.s)
/// $03010C: move.w #$7, d1
pub fn i_03010c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30110;
    cpu.ir = 0x323C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030110: jsr $236e.l
pub fn i_030110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30116;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30116);
    cpu.pc = 0x236E;
    16
}
/// $030116: move.w #$9, d1
pub fn i_030116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3011A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03011A: jsr $236e.l
pub fn i_03011a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30120;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30120);
    cpu.pc = 0x236E;
    16
}
/// $030120: move.w #$8, d1
pub fn i_030120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30124;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030124: jsr $236e.l
pub fn i_030124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3012A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3012A);
    cpu.pc = 0x236E;
    16
}
/// $03012A: move.w #$6, d1
pub fn i_03012a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3012E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03012E: jsr $236e.l
pub fn i_03012e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30134;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30134);
    cpu.pc = 0x236E;
    16
}
/// $030134: lea.l $279674.l, a1
pub fn i_030134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3013A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x279674;
    20
}
/// $03013A: move.w $2(a1), d0
pub fn i_03013a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3013E;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03013E: add.w d0, $22(a6)
pub fn i_03013e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30142;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $030142: move.w d0, $7c(a6)
pub fn i_030142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30146;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030146: jsr $30002(pc)
pub fn i_030146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3014A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3014A);
    cpu.pc = 0x30002;
    16
}
/// $03014A: move.w $16(a6), $14(a6)
pub fn i_03014a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30150;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $030150: lea.l $30156(pc), a1
pub fn i_030150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30154;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30156;
    16
}
/// $030154: move.l a1, (a6)
pub fn i_030154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30156;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030156: movea.l $c(a6), a0
pub fn i_030156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3015A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03015A: btst.b #$3, $13(a0)
pub fn i_03015a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30160;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $030160: beq.w $30168
pub fn i_030160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30164;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30168; return 10; }
    8
}
/// $030164: clr.w $8e(a6)
pub fn i_030164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30168;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $030168: move.w $22(a6), $80(a6)
pub fn i_030168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3016E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03016E: move.w $24(a6), $82(a6)
pub fn i_03016e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30174;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $030174: movea.l $c(a6), a2
pub fn i_030174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30178;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $030178: move.w $82(a2), $24(a6)
pub fn i_030178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3017E;
    cpu.ir = 0x3D6A;
    let ad2 = cpu.a[2].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03017E: move.w $22(a2), $22(a6)
pub fn i_03017e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30184;
    cpu.ir = 0x3D6A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $030184: jsr $27eba.l
pub fn i_030184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3018A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3018A);
    cpu.pc = 0x27EBA;
    16
}
/// $03018A: scc.b d0
pub fn i_03018a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3018C;
    cpu.ir = 0x54C0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $03018C: andi.b #$1, d0
pub fn i_03018c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30190;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $030190: move.b d0, $84(a6)
pub fn i_030190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30194;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $030194: move.w $24(a6), $9a(a6)
pub fn i_030194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3019A;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x9A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03019A: move.w $8e(a6), d0
pub fn i_03019a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3019E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03019E: beq.w $301ba
pub fn i_03019e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x301BA; return 10; }
    8
}
/// $0301A2: cmpi.w #$0, $279666.l
pub fn i_0301a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301AA;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x279666) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Word, false);
    20
}
/// $0301AA: bne.w $301b6
pub fn i_0301aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x301B6; return 10; }
    8
}
/// $0301AE: add.w d0, $9a(a6)
pub fn i_0301ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301B2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x9A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0301B2: bra.w $301ba
pub fn i_0301b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x301BA;
    10
}
/// $0301B6: add.w d0, $24(a6)
pub fn i_0301b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301BA;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0301BA: movea.l $70(a6), a1
pub fn i_0301ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301BE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0301BE: movea.l $74(a6), a2
pub fn i_0301be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301C2;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0301C2: move.w $24(a1), $8a(a6)
pub fn i_0301c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301C8;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0301C8: move.w $24(a2), $8c(a6)
pub fn i_0301c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301CE;
    cpu.ir = 0x3D6A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0301CE: jsr $30554(pc)
pub fn i_0301ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x301D2);
    cpu.pc = 0x30554;
    16
}
/// $0301D2: clr.w $8e(a6)
pub fn i_0301d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301D6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0301D6: btst.b #$4, $100001.l
pub fn i_0301d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301DE;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0301DE: beq.w $301ee
pub fn i_0301de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x301EE; return 10; }
    8
}
/// $0301E2: move.w #$f000, $38(a6)
pub fn i_0301e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301E8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_0301e8  ($0301E8, 8 B, thunks/jsr_abs_thunks.c)
/// $0301E8: jsr $28d70.l
pub fn i_0301e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x301EE);
    cpu.pc = 0x28D70;
    16
}
/// $0301EE: rts
pub fn i_0301ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chain3_Follow_0301f0  ($0301F0, 410 B, slug/vehicle_02dd20.s)
/// $0301F0: lea.l $301f6(pc), a1
pub fn i_0301f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x301F6;
    16
}
/// $0301F4: move.l a1, (a6)
pub fn i_0301f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0301F6: movea.l $c(a6), a0
pub fn i_0301f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x301FA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0301FA: btst.b #$3, $13(a0)
pub fn i_0301fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30200;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $030200: beq.w $30208
pub fn i_030200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30204;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30208; return 10; }
    8
}
/// $030204: clr.w $8e(a6)
pub fn i_030204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30208;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $030208: move.w $22(a6), $80(a6)
pub fn i_030208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3020E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03020E: move.w $24(a6), $82(a6)
pub fn i_03020e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30214;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $030214: jsr $2abcc.l
pub fn i_030214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3021A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3021A);
    cpu.pc = 0x2ABCC;
    16
}
/// $03021A: bcs.w $30224
pub fn i_03021a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3021E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x30224; return 10; }
    8
}
/// $03021E: jsr $2783a.l
pub fn i_03021e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30224;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30224);
    cpu.pc = 0x2783A;
    16
}
/// $030224: jsr $30462(pc)
pub fn i_030224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30228;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x30228);
    cpu.pc = 0x30462;
    16
}
/// $030228: tst.b $84(a6)
pub fn i_030228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3022C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03022C: beq.w $30294
pub fn i_03022c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30230;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30294; return 10; }
    8
}
/// $030230: move.w $16(a6), $14(a6)
pub fn i_030230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30236;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $030236: bsr.w $3050c
pub fn i_030236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3023A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3023A);
    cpu.pc = 0x3050C;
    18
}
/// $03023A: jsr $27902.l
pub fn i_03023a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30240);
    cpu.pc = 0x27902;
    16
}
/// $030240: bcc.w $3024a
pub fn i_030240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30244;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3024A; return 10; }
    8
}
/// $030244: bset.b #$4, $5a(a6)
pub fn i_030244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3024A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03024A: jsr $27eba.l
pub fn i_03024a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30250;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30250);
    cpu.pc = 0x27EBA;
    16
}
/// $030250: bcc.w $30258
pub fn i_030250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30254;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x30258; return 10; }
    8
}
/// $030254: clr.b $84(a6)
pub fn i_030254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30258;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $030258: movea.l $c(a6), a0
pub fn i_030258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3025C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03025C: cmpi.w #$ffff, $34(a0)
pub fn i_03025c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30262;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $030262: bne.w $3026a
pub fn i_030262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30266;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3026A; return 10; }
    8
}
/// $030266: clr.b $84(a6)
pub fn i_030266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3026A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03026A: bsr.w $30392
pub fn i_03026a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3026E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3026E);
    cpu.pc = 0x30392;
    18
}
/// $03026E: bcc.w $30280
pub fn i_03026e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30272;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x30280; return 10; }
    8
}
/// $030272: clr.w $2a(a6)
pub fn i_030272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30276;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $030276: clr.b $84(a6)
pub fn i_030276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3027A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03027A: move.b #$ff, $5c(a6)
pub fn i_03027a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30280;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $030280: tst.b $84(a6)
pub fn i_030280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30284;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $030284: bne.w $30290
pub fn i_030284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30288;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30290; return 10; }
    8
}
/// $030288: clr.w $28(a6)
pub fn i_030288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3028C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03028C: bra.w $30294
pub fn i_03028c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30290;
    cpu.ir = 0x6000;
    cpu.pc = 0x30294;
    10
}
/// $030290: bra.w $3034e
pub fn i_030290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30294;
    cpu.ir = 0x6000;
    cpu.pc = 0x3034E;
    10
}
/// $030294: movea.l $c(a6), a0
pub fn i_030294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30298;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $030298: move.w $82(a0), d0
pub fn i_030298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3029C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03029C: move.w $34(a0), d1
pub fn i_03029c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302A0;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0302A0: cmpi.w #$ffff, d1
pub fn i_0302a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302A4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $0302A4: bne.w $302e0
pub fn i_0302a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x302E0; return 10; }
    8
}
/// $0302A8: move.w $18(a6), $14(a6)
pub fn i_0302a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302AE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0302AE: bsr.w $30416
pub fn i_0302ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302B2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x302B2);
    cpu.pc = 0x30416;
    18
}
/// $0302B2: move.w $22(a6), d1
pub fn i_0302b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302B6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0302B6: move.w $24(a6), d2
pub fn i_0302b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302BA;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0302BA: jsr $280c6.l
pub fn i_0302ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x302C0);
    cpu.pc = 0x280C6;
    16
}
/// $0302C0: bcs.w $302dc
pub fn i_0302c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302C4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x302DC; return 10; }
    8
}
/// $0302C4: move.w $22(a6), d1
pub fn i_0302c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302C8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0302C8: move.w $24(a6), d2
pub fn i_0302c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302CC;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0302CC: subq.w #$4, d2
pub fn i_0302cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302CE;
    cpu.ir = 0x5942;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0302CE: jsr $280c6.l
pub fn i_0302ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x302D4);
    cpu.pc = 0x280C6;
    16
}
/// $0302D4: bcs.w $302dc
pub fn i_0302d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302D8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x302DC; return 10; }
    8
}
/// $0302D8: clr.b $5c(a6)
pub fn i_0302d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302DC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0302DC: bra.w $30304
pub fn i_0302dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x30304;
    10
}
/// $0302E0: move.w $1a(a6), $14(a6)
pub fn i_0302e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302E6;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x1A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0302E6: bsr.w $303ee
pub fn i_0302e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302EA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x302EA);
    cpu.pc = 0x303EE;
    18
}
/// $0302EA: sub.w $24(a6), d0
pub fn i_0302ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302EE;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0302EE: blt.w $302f6
pub fn i_0302ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302F2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x302F6; return 10; }
    8
}
/// $0302F2: clr.b $5c(a6)
pub fn i_0302f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302F6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0302F6: lsl.w #$6, d0
pub fn i_0302f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302F8;
    cpu.ir = 0xED48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $0302F8: move.w $2a(a6), d1
pub fn i_0302f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302FC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0302FC: asr.w #$1, d1
pub fn i_0302fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x302FE;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0302FE: sub.w d1, d0
pub fn i_0302fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30300;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $030300: add.w d0, $2a(a6)
pub fn i_030300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30304;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $030304: tst.b $5c(a6)
pub fn i_030304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30308;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $030308: beq.w $30316
pub fn i_030308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3030C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30316; return 10; }
    8
}
/// $03030C: jsr $27d50.l
pub fn i_03030c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30312;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30312);
    cpu.pc = 0x27D50;
    16
}
/// $030312: bra.w $3031c
pub fn i_030312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30316;
    cpu.ir = 0x6000;
    cpu.pc = 0x3031C;
    10
}
/// $030316: jsr $27c2a.l
pub fn i_030316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3031C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3031C);
    cpu.pc = 0x27C2A;
    16
}
/// $03031C: bcc.w $3032e
pub fn i_03031c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30320;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3032E; return 10; }
    8
}
/// $030320: move.b #$1, $84(a6)
pub fn i_030320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30326;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $030326: clr.b $5c(a6)
pub fn i_030326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3032A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03032A: bra.w $3034e
pub fn i_03032a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3032E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3034E;
    10
}
/// $03032E: move.w $2a(a6), d0
pub fn i_03032e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30332;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030332: cmpi.w #$8, d0
pub fn i_030332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30336;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $030336: bcc.w $3034e
pub fn i_030336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3033A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3034E; return 10; }
    8
}
/// $03033A: jsr $27eba.l
pub fn i_03033a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30340);
    cpu.pc = 0x27EBA;
    16
}
/// $030340: bcs.w $3034e
pub fn i_030340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30344;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3034E; return 10; }
    8
}
/// $030344: move.b #$1, $84(a6)
pub fn i_030344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3034A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03034A: clr.b $5c(a6)
pub fn i_03034a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3034E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03034E: move.w $24(a6), $9a(a6)
pub fn i_03034e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30354;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x9A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $030354: move.w $8e(a6), d0
pub fn i_030354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30358;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030358: beq.w $30378
pub fn i_030358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3035C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30378; return 10; }
    8
}
/// $03035C: cmpi.w #$0, $279666.l
pub fn i_03035c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30364;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x279666) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Word, false);
    20
}
/// $030364: bne.w $30370
pub fn i_030364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30368;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30370; return 10; }
    8
}
/// $030368: add.w d0, $9a(a6)
pub fn i_030368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3036C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x9A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03036C: bra.w $30374
pub fn i_03036c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30370;
    cpu.ir = 0x6000;
    cpu.pc = 0x30374;
    10
}
/// $030370: add.w d0, $24(a6)
pub fn i_030370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30374;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $030374: clr.w $8e(a6)
pub fn i_030374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30378;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $030378: btst.b #$4, $100001.l
pub fn i_030378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30380;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $030380: beq.w $30390
pub fn i_030380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30384;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30390; return 10; }
    8
}
/// $030384: move.w #$f000, $38(a6)
pub fn i_030384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3038A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_03038a  ($03038A, 8 B, thunks/jsr_abs_thunks.c)
/// $03038A: jsr $28d70.l
pub fn i_03038a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30390;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30390);
    cpu.pc = 0x28D70;
    16
}
/// $030390: rts
pub fn i_030390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30392;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chain3_YDelta_030392  ($030392, 92 B, slug/vehicle_02dd20.s)
/// $030392: movea.l $c(a6), a0
pub fn i_030392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30396;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $030396: move.w $24(a6), d0
pub fn i_030396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3039A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03039A: sub.w $82(a0), d0
pub fn i_03039a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3039E;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03039E: cmpi.w #$ffff, $34(a0)
pub fn i_03039e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303A4;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $0303A4: bne.w $303d2
pub fn i_0303a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x303D2; return 10; }
    8
}
/// $0303A8: move.w $28(a6), d1
pub fn i_0303a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303AC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0303AC: beq.w $303c6
pub fn i_0303ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x303C6; return 10; }
    8
}
/// $0303B0: move.w $7c(a6), d2
pub fn i_0303b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303B4;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0303B4: eor.w d1, d2
pub fn i_0303b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303B6;
    cpu.ir = 0xB342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0303B6: bpl.w $303c6
pub fn i_0303b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303BA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x303C6; return 10; }
    8
}
/// $0303BA: cmpi.w #$0, d0
pub fn i_0303ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303BE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $0303BE: bge.w $303e8
pub fn i_0303be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303C2;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x303E8; return 10; }
    8
}
/// $0303C2: bra.w $303ce
pub fn i_0303c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x303CE;
    10
}
/// $0303C6: cmpi.w #$18, d0
pub fn i_0303c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303CA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x18, Size::Word, false);
    8
}
/// $0303CA: bge.w $303e8
pub fn i_0303ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303CE;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x303E8; return 10; }
    8
}
/// $0303CE: bra.w $303e2
pub fn i_0303ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x303E2;
    10
}
/// $0303D2: cmpi.w #$12, d0
pub fn i_0303d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303D6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x12, Size::Word, false);
    8
}
/// $0303D6: bgt.w $303e8
pub fn i_0303d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303DA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x303E8; return 10; }
    8
}
/// $0303DA: cmpi.w #$ffee, d0
pub fn i_0303da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303DE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFEE, Size::Word, false);
    8
}
/// $0303DE: blt.w $303e8
pub fn i_0303de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303E2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x303E8; return 10; }
    8
}
/// $0303E2: andi.b #$fe, ccr
pub fn i_0303e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303E6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $0303E6: rts
pub fn i_0303e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0303E8: ori.b #$1, ccr
pub fn i_0303e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303EC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $0303EC: rts
pub fn i_0303ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chain3_Step_0303ee  ($0303EE, 40 B, slug/vehicle_02dd20.s)
/// $0303EE: tst.w d1
pub fn i_0303ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303F0;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0303F0: beq.w $30414
pub fn i_0303f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30414; return 10; }
    8
}
/// $0303F4: move.w $7c(a6), d2
pub fn i_0303f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303F8;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0303F8: tst.w d1
pub fn i_0303f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303FA;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0303FA: bpl.w $30400
pub fn i_0303fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x303FE;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x30400; return 10; }
    8
}
/// $0303FE: neg.w d2
pub fn i_0303fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30400;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030400: subq.w #$1, d2
pub fn i_030400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30402;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030402: cmpi.w #$1000, d1
pub fn i_030402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30406;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1000, Size::Word, false);
    8
}
/// $030406: beq.b $30410
pub fn i_030406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30408;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30410; return 10; }
    8
}
/// $030408: cmpi.w #$f000, d1
pub fn i_030408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3040C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF000, Size::Word, false);
    8
}
/// $03040C: bne.w $30412
pub fn i_03040c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30410;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30412; return 10; }
    8
}
/// $030410: asr.w #$2, d2
pub fn i_030410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30412;
    cpu.ir = 0xE442;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $030412: add.w d2, d0
pub fn i_030412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30414;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $030414: rts
pub fn i_030414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30416;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chain3_VelY_030416  ($030416, 76 B, slug/vehicle_02dd20.s)
/// $030416: movea.l $c(a6), a0
pub fn i_030416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3041A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03041A: btst.b #$3, $5b(a0)
pub fn i_03041a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30420;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $030420: beq.w $3042e
pub fn i_030420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30424;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3042E; return 10; }
    8
}
/// $030424: subi.w #$c0, $2a(a6)
pub fn i_030424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3042A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03042A: bra.w $30460
pub fn i_03042a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3042E;
    cpu.ir = 0x6000;
    cpu.pc = 0x30460;
    10
}
/// $03042E: move.w $2a(a0), $2a(a6)
pub fn i_03042e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30434;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $030434: movea.l $74(a6), a1
pub fn i_030434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30438;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $030438: movea.l $70(a6), a2
pub fn i_030438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3043C;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $03043C: move.w $24(a1), d0
pub fn i_03043c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30440;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030440: sub.w $24(a2), d0
pub fn i_030440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30444;
    cpu.ir = 0x906A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $030444: bpl.w $3044a
pub fn i_030444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30448;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x3044A; return 10; }
    8
}
/// $030448: neg.w d0
pub fn i_030448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3044A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03044A: cmpi.w #$1c, d0
pub fn i_03044a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3044E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1C, Size::Word, false);
    8
}
/// $03044E: bcc.w $30460
pub fn i_03044e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30452;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x30460; return 10; }
    8
}
/// $030452: move.w $82(a0), d0
pub fn i_030452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30456;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030456: sub.w $24(a6), d0
pub fn i_030456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3045A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03045A: lsl.w #$4, d0
pub fn i_03045a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3045C;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $03045C: add.w d0, $2a(a6)
pub fn i_03045c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30460;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $030460: rts
pub fn i_030460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30462;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chain3_VelX_030462  ($030462, 98 B, slug/vehicle_02dd20.s)
/// $030462: movea.l $c(a6), a1
pub fn i_030462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30466;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $030466: move.w $22(a1), d1
pub fn i_030466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3046A;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03046A: add.w $7c(a6), d1
pub fn i_03046a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3046E;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03046E: sub.w $22(a6), d1
pub fn i_03046e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30472;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $030472: asl.w #$8, d1
pub fn i_030472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30474;
    cpu.ir = 0xE141;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $030474: move.w d1, $28(a6)
pub fn i_030474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30478;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030478: cmpi.w #$ffff, $7e(a6)
pub fn i_030478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3047E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $03047E: beq.w $304ae
pub fn i_03047e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30482;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x304AE; return 10; }
    8
}
/// $030482: move.w $28(a1), d1
pub fn i_030482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30486;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030486: andi.w #$8000, d1
pub fn i_030486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3048A;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x8000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03048A: move.w $7e(a6), d2
pub fn i_03048a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3048E;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03048E: eor.w d2, d1
pub fn i_03048e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30490;
    cpu.ir = 0xB541;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030490: bne.w $304ae
pub fn i_030490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30494;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x304AE; return 10; }
    8
}
/// $030494: move.b $13(a1), d0
pub fn i_030494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30498;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $030498: andi.b #$30, d0
pub fn i_030498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3049C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $03049C: move.b #$30, d1
pub fn i_03049c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304A0;
    cpu.ir = 0x123C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0304A0: not.b d1
pub fn i_0304a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304A2;
    cpu.ir = 0x4601;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = !v1 & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0304A2: and.b d1, $13(a6)
pub fn i_0304a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304A6;
    cpu.ir = 0xC32E;
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0304A6: or.b d0, $13(a6)
pub fn i_0304a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304AA;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0304AA: bra.w $304c2
pub fn i_0304aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x304C2;
    10
}
/// $0304AE: move.b $5b(a6), d1
pub fn i_0304ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304B2;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0304B2: move.b #$30, d2
pub fn i_0304b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304B6;
    cpu.ir = 0x143C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0304B6: and.b d2, d1
pub fn i_0304b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304B8;
    cpu.ir = 0xC202;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0304B8: neg.b d2
pub fn i_0304b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304BA;
    cpu.ir = 0x4402;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0304BA: and.b d2, $13(a6)
pub fn i_0304ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304BE;
    cpu.ir = 0xC52E;
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0304BE: or.b d1, $13(a6)
pub fn i_0304be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304C2;
    cpu.ir = 0x832E;
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0304C2: rts
pub fn i_0304c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chain3_CheckSyncA_0304c4  ($0304C4, 72 B, slug/vehicle_02dd20.s)
/// $0304C4: movea.l $74(a6), a1
pub fn i_0304c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304C8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0304C8: movea.l $78(a6), a2
pub fn i_0304c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304CC;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0304CC: move.w $24(a1), d1
pub fn i_0304cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304D0;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0304D0: cmp.w $24(a2), d1
pub fn i_0304d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304D4;
    cpu.ir = 0xB26A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0304D4: beq.w $304ee
pub fn i_0304d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x304EE; return 10; }
    8
}
/// $0304D8: movea.l $78(a6), a3
pub fn i_0304d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304DC;
    cpu.ir = 0x266E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $0304DC: move.w $8e(a3), d0
pub fn i_0304dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304E0;
    cpu.ir = 0x302B;
    let ad2 = cpu.a[3].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0304E0: or.w $8e(a1), d0
pub fn i_0304e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304E4;
    cpu.ir = 0x8069;
    let ad2 = cpu.a[1].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0304E4: or.w $8e(a2), d0
pub fn i_0304e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304E8;
    cpu.ir = 0x806A;
    let ad2 = cpu.a[2].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0304E8: tst.w d0
pub fn i_0304e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304EA;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0304EA: beq.w $304fe
pub fn i_0304ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304EE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x304FE; return 10; }
    8
}
/// $0304EE: bclr.b #$4, $13(a6)
pub fn i_0304ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0304F4: bclr.b #$5, $13(a6)
pub fn i_0304f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304FA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0304FA: bra.w $3050a
pub fn i_0304fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x304FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3050A;
    10
}
/// $0304FE: bset.b #$4, $13(a6)
pub fn i_0304fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30504;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $030504: bset.b #$5, $13(a6)
pub fn i_030504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3050A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03050A: rts
pub fn i_03050a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3050C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chain3_CheckSyncB_03050c  ($03050C, 72 B, slug/vehicle_02dd20.s)
/// $03050C: movea.l $70(a6), a1
pub fn i_03050c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30510;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $030510: movea.l $74(a6), a2
pub fn i_030510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30514;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $030514: move.w $82(a1), d1
pub fn i_030514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30518;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030518: cmp.w $82(a2), d1
pub fn i_030518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3051C;
    cpu.ir = 0xB26A;
    let ad2 = cpu.a[2].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $03051C: beq.w $30536
pub fn i_03051c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30520;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30536; return 10; }
    8
}
/// $030520: movea.l $78(a6), a3
pub fn i_030520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30524;
    cpu.ir = 0x266E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $030524: move.w $8e(a3), d0
pub fn i_030524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30528;
    cpu.ir = 0x302B;
    let ad2 = cpu.a[3].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030528: or.w $8e(a1), d0
pub fn i_030528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3052C;
    cpu.ir = 0x8069;
    let ad2 = cpu.a[1].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $03052C: or.w $8e(a2), d0
pub fn i_03052c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30530;
    cpu.ir = 0x806A;
    let ad2 = cpu.a[2].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $030530: tst.w d0
pub fn i_030530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30532;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $030532: beq.w $30546
pub fn i_030532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30536;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30546; return 10; }
    8
}
/// $030536: bclr.b #$4, $5b(a6)
pub fn i_030536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3053C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03053C: bclr.b #$5, $5b(a6)
pub fn i_03053c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30542;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $030542: bra.w $30552
pub fn i_030542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30546;
    cpu.ir = 0x6000;
    cpu.pc = 0x30552;
    10
}
/// $030546: bset.b #$4, $5b(a6)
pub fn i_030546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3054C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03054C: bset.b #$5, $5b(a6)
pub fn i_03054c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30552;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $030552: rts
pub fn i_030552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30554;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chain3_DebugHud_030554  ($030554, 174 B, slug/vehicle_02dd20.s)
/// $030554: btst.b #$4, $100001.l
pub fn i_030554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3055C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $03055C: beq.w $30608
pub fn i_03055c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30560;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30608; return 10; }
    8
}
/// $030560: movea.l #$7412, a1
pub fn i_030560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30566;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7412;
    cpu.a[1] = v1;
    4
}
/// $030566: moveq #$0, d0
pub fn i_030566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30568;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $030568: movea.l $70(a6), a2
pub fn i_030568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3056C;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $03056C: move.w $8e(a2), d0
pub fn i_03056c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30570;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030570: jsr $5d6c2.l
pub fn i_030570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30576;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30576);
    cpu.pc = 0x5D6C2;
    16
}
/// $030576: movea.l #$7413, a1
pub fn i_030576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3057C;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7413;
    cpu.a[1] = v1;
    4
}
/// $03057C: moveq #$0, d0
pub fn i_03057c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3057E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03057E: movea.l $74(a6), a2
pub fn i_03057e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30582;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $030582: move.w $8e(a2), d0
pub fn i_030582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30586;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x8E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030586: jsr $5d6c2.l
pub fn i_030586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3058C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3058C);
    cpu.pc = 0x5D6C2;
    16
}
/// $03058C: movea.l #$7415, a1
pub fn i_03058c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30592;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7415;
    cpu.a[1] = v1;
    4
}
/// $030592: moveq #$0, d0
pub fn i_030592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30594;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $030594: movea.l $70(a6), a2
pub fn i_030594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30598;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $030598: move.w $22(a2), d0
pub fn i_030598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3059C;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03059C: jsr $5d6c2.l
pub fn i_03059c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x305A2);
    cpu.pc = 0x5D6C2;
    16
}
/// $0305A2: movea.l #$7416, a1
pub fn i_0305a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305A8;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7416;
    cpu.a[1] = v1;
    4
}
/// $0305A8: moveq #$0, d0
pub fn i_0305a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305AA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0305AA: move.w $22(a6), d0
pub fn i_0305aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305AE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0305AE: jsr $5d6c2.l
pub fn i_0305ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x305B4);
    cpu.pc = 0x5D6C2;
    16
}
/// $0305B4: movea.l #$7417, a1
pub fn i_0305b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305BA;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7417;
    cpu.a[1] = v1;
    4
}
/// $0305BA: moveq #$0, d0
pub fn i_0305ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305BC;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0305BC: movea.l $74(a6), a2
pub fn i_0305bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305C0;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0305C0: move.w $22(a2), d0
pub fn i_0305c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305C4;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0305C4: jsr $5d6c2.l
pub fn i_0305c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x305CA);
    cpu.pc = 0x5D6C2;
    16
}
/// $0305CA: movea.l #$7419, a1
pub fn i_0305ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305D0;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7419;
    cpu.a[1] = v1;
    4
}
/// $0305D0: moveq #$0, d0
pub fn i_0305d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305D2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0305D2: movea.l $70(a6), a2
pub fn i_0305d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305D6;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0305D6: move.w $24(a2), d0
pub fn i_0305d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305DA;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0305DA: jsr $5d6c2.l
pub fn i_0305da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x305E0);
    cpu.pc = 0x5D6C2;
    16
}
/// $0305E0: movea.l #$741a, a1
pub fn i_0305e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305E6;
    cpu.ir = 0x227C;
    let v1: u32 = 0x741A;
    cpu.a[1] = v1;
    4
}
/// $0305E6: moveq #$0, d0
pub fn i_0305e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305E8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0305E8: move.w $24(a6), d0
pub fn i_0305e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305EC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0305EC: jsr $5d6c2.l
pub fn i_0305ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x305F2);
    cpu.pc = 0x5D6C2;
    16
}
/// $0305F2: movea.l #$741b, a1
pub fn i_0305f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305F8;
    cpu.ir = 0x227C;
    let v1: u32 = 0x741B;
    cpu.a[1] = v1;
    4
}
/// $0305F8: moveq #$0, d0
pub fn i_0305f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305FA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0305FA: movea.l $74(a6), a2
pub fn i_0305fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x305FE;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0305FE: move.w $24(a2), d0
pub fn i_0305fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30602;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
// ===== JsrAbsThunk_030602  ($030602, 8 B, thunks/jsr_abs_thunks.c)
/// $030602: jsr $5d6c2.l
pub fn i_030602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30608;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30608);
    cpu.pc = 0x5D6C2;
    16
}
/// $030608: rts
pub fn i_030608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3060A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_Build3ChainCircular_03060A  ($03060A, 140 B, entity/build3_chain_circular_03060a.s)
/// $03060A: lea.l $3010c(pc), a1
pub fn i_03060a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3060E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3010C;
    16
}
/// $03060E: jsr $4ae.l
pub fn i_03060e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30614;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30614);
    cpu.pc = 0x4AE;
    16
}
/// $030614: jsr $5dd02.l
pub fn i_030614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3061A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3061A);
    cpu.pc = 0x5DD02;
    16
}
/// $03061A: move.l a0, $7c(a6)
pub fn i_03061a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3061E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03061E: move.w $82(a6), $24(a0)
pub fn i_03061e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30624;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $030624: clr.b $98(a0)
pub fn i_030624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30628;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $030628: lea.l $30068(pc), a1
pub fn i_030628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3062C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30068;
    16
}
/// $03062C: jsr $4ae.l
pub fn i_03062c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30632;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30632);
    cpu.pc = 0x4AE;
    16
}
/// $030632: jsr $5dd02.l
pub fn i_030632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30638;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30638);
    cpu.pc = 0x5DD02;
    16
}
/// $030638: move.l a0, $74(a6)
pub fn i_030638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3063C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03063C: move.w $82(a6), $24(a0)
pub fn i_03063c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30642;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $030642: clr.b $98(a0)
pub fn i_030642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30646;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $030646: lea.l $300ba(pc), a1
pub fn i_030646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3064A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x300BA;
    16
}
/// $03064A: jsr $4ae.l
pub fn i_03064a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30650;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30650);
    cpu.pc = 0x4AE;
    16
}
/// $030650: jsr $5dd02.l
pub fn i_030650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30656);
    cpu.pc = 0x5DD02;
    16
}
/// $030656: move.l a0, $78(a6)
pub fn i_030656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3065A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03065A: move.w $82(a6), $24(a0)
pub fn i_03065a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30660;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $030660: clr.b $98(a0)
pub fn i_030660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30664;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $030664: movea.l $74(a6), a2
pub fn i_030664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30668;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $030668: movea.l $78(a6), a3
pub fn i_030668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3066C;
    cpu.ir = 0x266E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $03066C: movea.l $7c(a6), a0
pub fn i_03066c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30670;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $030670: move.l a2, $70(a2)
pub fn i_030670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30674;
    cpu.ir = 0x254A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[2].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $030674: move.l a0, $78(a2)
pub fn i_030674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30678;
    cpu.ir = 0x2548;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[2].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $030678: move.l a3, $74(a2)
pub fn i_030678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3067C;
    cpu.ir = 0x254B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    let ad2 = cpu.a[2].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03067C: move.l a2, $70(a3)
pub fn i_03067c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30680;
    cpu.ir = 0x274A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[3].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $030680: move.l a0, $78(a3)
pub fn i_030680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30684;
    cpu.ir = 0x2748;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $030684: move.l a3, $74(a3)
pub fn i_030684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30688;
    cpu.ir = 0x274B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    let ad2 = cpu.a[3].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $030688: move.l a2, $70(a0)
pub fn i_030688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3068C;
    cpu.ir = 0x214A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03068C: move.l a0, $78(a0)
pub fn i_03068c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30690;
    cpu.ir = 0x2148;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $030690: move.l a3, $74(a0)
pub fn i_030690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30694;
    cpu.ir = 0x214B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $030694: rts
pub fn i_030694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30696;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chain3_PickLink_030704  ($030704, 58 B, player/tables_fx_030696.s)
/// $030704: movea.l $74(a6), a1
pub fn i_030704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30708;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $030708: movea.l $78(a6), a2
pub fn i_030708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3070C;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $03070C: movea.l a1, a0
pub fn i_03070c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3070E;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $03070E: move.w #$1, d0
pub fn i_03070e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30712;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030712: move.b $5a(a1), d1
pub fn i_030712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30716;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $030716: andi.b #$20, d1
pub fn i_030716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3071A;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x20) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $03071A: bne.w $30736
pub fn i_03071a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3071E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30736; return 10; }
    8
}
/// $03071E: movea.l a2, a0
pub fn i_03071e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30720;
    cpu.ir = 0x204A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $030720: move.w #$ffff, d0
pub fn i_030720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30724;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030724: move.b $5a(a2), d1
pub fn i_030724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30728;
    cpu.ir = 0x122A;
    let ad2 = cpu.a[2].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $030728: andi.b #$20, d1
pub fn i_030728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3072C;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x20) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $03072C: bne.w $30736
pub fn i_03072c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30730;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30736; return 10; }
    8
}
/// $030730: movea.l a6, a0
pub fn i_030730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30732;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $030732: sub.w d0, d0
pub fn i_030732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30734;
    cpu.ir = 0x9040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $030734: rts
pub fn i_030734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30736;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $030736: cmpa.l #$ffffffff, a6
pub fn i_030736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3073C;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03073C: rts
pub fn i_03073c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3073E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chain3_LinksYDeltaIsStep_03076a  ($03076A, 126 B, player/tables_fx_030696.s)
/// $03076A: movea.l $74(a6), a1
pub fn i_03076a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3076E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03076E: movea.l $78(a6), a2
pub fn i_03076e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30772;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $030772: move.b $5a(a1), d1
pub fn i_030772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30776;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $030776: or.b $5a(a2), d1
pub fn i_030776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3077A;
    cpu.ir = 0x822A;
    let ad2 = cpu.a[2].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03077A: andi.b #$10, d1
pub fn i_03077a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3077E;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $03077E: bne.w $307e4
pub fn i_03077e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30782;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x307E4; return 10; }
    8
}
/// $030782: move.w $24(a1), d1
pub fn i_030782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30786;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030786: move.w $24(a2), d2
pub fn i_030786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3078A;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03078A: sub.w d2, d1
pub fn i_03078a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3078C;
    cpu.ir = 0x9242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $03078C: subx.w d2, d2
pub fn i_03078c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3078E;
    cpu.ir = 0x9542;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let h2 = cpu.d[2] & 0xFFFF;
    let r = subx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $03078E: add.w d2, d1
pub fn i_03078e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30790;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $030790: eor.w d2, d1
pub fn i_030790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30792;
    cpu.ir = 0xB541;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030792: cmpi.w #$0, d1
pub fn i_030792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30796;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $030796: beq.w $307e4
pub fn i_030796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3079A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x307E4; return 10; }
    8
}
/// $03079A: cmpi.w #$1, d1
pub fn i_03079a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3079E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $03079E: beq.w $307e4
pub fn i_03079e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x307E4; return 10; }
    8
}
/// $0307A2: cmpi.w #$2, d1
pub fn i_0307a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307A6;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $0307A6: beq.w $307e4
pub fn i_0307a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x307E4; return 10; }
    8
}
/// $0307AA: cmpi.w #$8, d1
pub fn i_0307aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307AE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $0307AE: beq.w $307e4
pub fn i_0307ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x307E4; return 10; }
    8
}
/// $0307B2: cmpi.w #$7, d1
pub fn i_0307b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307B6;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7, Size::Word, false);
    8
}
/// $0307B6: beq.w $307e4
pub fn i_0307b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x307E4; return 10; }
    8
}
/// $0307BA: cmpi.w #$9, d1
pub fn i_0307ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307BE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x9, Size::Word, false);
    8
}
/// $0307BE: beq.w $307e4
pub fn i_0307be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307C2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x307E4; return 10; }
    8
}
/// $0307C2: cmpi.w #$20, d1
pub fn i_0307c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307C6;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $0307C6: beq.w $307e4
pub fn i_0307c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307CA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x307E4; return 10; }
    8
}
/// $0307CA: cmpi.w #$1f, d1
pub fn i_0307ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307CE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1F, Size::Word, false);
    8
}
/// $0307CE: beq.w $307e4
pub fn i_0307ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307D2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x307E4; return 10; }
    8
}
/// $0307D2: cmpi.w #$21, d1
pub fn i_0307d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307D6;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x21, Size::Word, false);
    8
}
/// $0307D6: beq.w $307e4
pub fn i_0307d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x307E4; return 10; }
    8
}
/// $0307DA: cmpi.w #$22, d1
pub fn i_0307da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307DE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x22, Size::Word, false);
    8
}
/// $0307DE: beq.w $307e4
pub fn i_0307de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x307E4; return 10; }
    8
}
/// $0307E2: rts
pub fn i_0307e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0307E4: cmp.w d1, d1
pub fn i_0307e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307E6;
    cpu.ir = 0xB241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0307E6: rts
pub fn i_0307e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerGrenade_SpawnFromVehicle_0308b0  ($0308B0, 18 B, player/tables_fx_030696.s)
/// $0308B0: move.w #$1d6, d1
pub fn i_0308b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0308B4: move.w d1, $9e(a6)
pub fn i_0308b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308B8;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0308B8: jsr $236e.l
pub fn i_0308b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x308BE);
    cpu.pc = 0x236E;
    16
}
/// $0308BE: bra.w $308d0
pub fn i_0308be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x308D0;
    10
}
/// $0308D0: move.b $98(a6), $70(a6)
pub fn i_0308d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308D6;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0308D6: move.b $9a(a6), $7a(a6)
pub fn i_0308d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308DC;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0308DC: bset.b #$4, $6b(a6)
pub fn i_0308dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308E2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0308E2: move.w #$d000, $38(a6)
pub fn i_0308e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308E8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0308E8: lea.l $77a96.l, a1
pub fn i_0308e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308EE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77A96;
    20
}
/// $0308EE: jsr $4ae.l
pub fn i_0308ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x308F4);
    cpu.pc = 0x4AE;
    16
}
/// $0308F4: jsr $5dd02.l
pub fn i_0308f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x308FA);
    cpu.pc = 0x5DD02;
    16
}
/// $0308FA: move.b $70(a6), d0
pub fn i_0308fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308FE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0308FE: andi.w #$f8, d0
pub fn i_0308fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30902;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030902: asr.w #$4, d0
pub fn i_030902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30904;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $030904: movea.l $c(a6), a0
pub fn i_030904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30908;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $030908: movea.l $c(a0), a0
pub fn i_030908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3090C;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03090C: movea.l #$29d4d2, a0
pub fn i_03090c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30912;
    cpu.ir = 0x207C;
    let v1: u32 = 0x29D4D2;
    cpu.a[0] = v1;
    4
}
/// $030912: lsl.w #$2, d0
pub fn i_030912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30914;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $030914: movea.l (a0, d0.w), a0
pub fn i_030914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30918;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $030918: cmpa.l #$ffffffff, a0
pub fn i_030918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3091E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03091E: beq.w $30928
pub fn i_03091e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30922;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30928; return 10; }
    8
}
/// $030922: jsr $28cd4.l
pub fn i_030922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30928;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30928);
    cpu.pc = 0x28CD4;
    16
}
/// $030928: lea.l $30804(pc), a0
pub fn i_030928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3092C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x30804;
    16
}
/// $03092C: move.l a0, $4c(a6)
pub fn i_03092c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30930;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $030930: jsr $283ca.l
pub fn i_030930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30936);
    cpu.pc = 0x283CA;
    16
}
/// $030936: jmp $309ae(pc)
pub fn i_030936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3093A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x309AE;
    8
}
// ===== PlayerGrenade_SpawnB_03093a  ($03093A, 628 B, player/tables_fx_030696.s)
/// $03093A: move.b $98(a6), $70(a6)
pub fn i_03093a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30940;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad4, v3 as u8);
    20
}
/// $030940: move.b $9a(a6), $7a(a6)
pub fn i_030940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30946;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $030946: bset.b #$4, $6b(a6)
pub fn i_030946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3094C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03094C: lea.l $30804(pc), a0
pub fn i_03094c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30950;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x30804;
    16
}
/// $030950: move.l a0, $4c(a6)
pub fn i_030950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30954;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $030954: jsr $283ca.l
pub fn i_030954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3095A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3095A);
    cpu.pc = 0x283CA;
    16
}
/// $03095A: move.w #$d000, $38(a6)
pub fn i_03095a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30960;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030960: lea.l $77a96.l, a1
pub fn i_030960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30966;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77A96;
    20
}
/// $030966: jsr $4ae.l
pub fn i_030966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3096C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3096C);
    cpu.pc = 0x4AE;
    16
}
/// $03096C: jsr $5dd02.l
pub fn i_03096c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30972;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30972);
    cpu.pc = 0x5DD02;
    16
}
/// $030972: move.w #$2, d1
pub fn i_030972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30976;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030976: move.w d1, $9e(a6)
pub fn i_030976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3097A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03097A: jsr $236e.l
pub fn i_03097a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30980;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30980);
    cpu.pc = 0x236E;
    16
}
/// $030980: move.b $70(a6), d0
pub fn i_030980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30984;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $030984: andi.w #$f8, d0
pub fn i_030984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30988;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030988: asr.w #$4, d0
pub fn i_030988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3098A;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $03098A: movea.l $c(a6), a0
pub fn i_03098a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3098E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03098E: movea.l $c(a0), a0
pub fn i_03098e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30992;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $030992: movea.l #$29d452, a0
pub fn i_030992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30998;
    cpu.ir = 0x207C;
    let v1: u32 = 0x29D452;
    cpu.a[0] = v1;
    4
}
/// $030998: lsl.w #$2, d0
pub fn i_030998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3099A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $03099A: movea.l (a0, d0.w), a0
pub fn i_03099a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3099E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03099E: cmpa.l #$ffffffff, a0
pub fn i_03099e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309A4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0309A4: beq.w $309ae
pub fn i_0309a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x309AE; return 10; }
    8
}
/// $0309A8: jsr $28cd4.l
pub fn i_0309a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x309AE);
    cpu.pc = 0x28CD4;
    16
}
/// $0309AE: jsr $267e2.l
pub fn i_0309ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x309B4);
    cpu.pc = 0x267E2;
    16
}
/// $0309B4: move.b $70(a6), d0
pub fn i_0309b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309B8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0309B8: andi.w #$f8, d0
pub fn i_0309b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309BC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0309BC: btst.b #$1, $100000.l
pub fn i_0309bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309C4;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((1) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0309C4: beq.w $309f4
pub fn i_0309c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x309F4; return 10; }
    8
}
/// $0309C8: move.w d0, d1
pub fn i_0309c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309CA;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0309CA: moveq #$0, d0
pub fn i_0309ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309CC;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0309CC: move.b $9b(a6), d0
pub fn i_0309cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309D0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0309D0: move.b d0, d2
pub fn i_0309d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309D2;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0309D2: andi.w #$8, d2
pub fn i_0309d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309D6;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0309D6: beq.w $309e0
pub fn i_0309d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x309E0; return 10; }
    8
}
/// $0309DA: not.w d0
pub fn i_0309da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309DC;
    cpu.ir = 0x4640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0309DC: andi.w #$7, d0
pub fn i_0309dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309E0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0309E0: addq.w #$4, d0
pub fn i_0309e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309E2;
    cpu.ir = 0x5840;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0309E2: andi.w #$7, d0
pub fn i_0309e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309E6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0309E6: subq.w #$4, d0
pub fn i_0309e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309E8;
    cpu.ir = 0x5940;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0309E8: lsl.w #$1, d0
pub fn i_0309e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309EA;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0309EA: add.w d1, d0
pub fn i_0309ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309EC;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0309EC: andi.w #$ff, d0
pub fn i_0309ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309F0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0309F0: bra.w $30aa4
pub fn i_0309f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x30AA4;
    10
}
/// $0309F4: cmpi.b #$5, $106f2a.l
pub fn i_0309f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x309FC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x5, Size::Byte, false);
    20
}
/// $0309FC: bne.w $30a2c
pub fn i_0309fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A00;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30A2C; return 10; }
    8
}
/// $030A00: move.w d0, d1
pub fn i_030a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A02;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030A02: moveq #$0, d0
pub fn i_030a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A04;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $030A04: move.b $9b(a6), d0
pub fn i_030a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A08;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $030A08: move.b d0, d2
pub fn i_030a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A0A;
    cpu.ir = 0x1400;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $030A0A: andi.w #$8, d2
pub fn i_030a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A0E;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030A0E: beq.w $30a18
pub fn i_030a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A12;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30A18; return 10; }
    8
}
/// $030A12: not.w d0
pub fn i_030a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A14;
    cpu.ir = 0x4640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030A14: andi.w #$7, d0
pub fn i_030a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A18;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030A18: addq.w #$4, d0
pub fn i_030a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A1A;
    cpu.ir = 0x5840;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030A1A: andi.w #$7, d0
pub fn i_030a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A1E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030A1E: subq.w #$4, d0
pub fn i_030a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A20;
    cpu.ir = 0x5940;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030A20: lsl.w #$1, d0
pub fn i_030a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A22;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $030A22: add.w d1, d0
pub fn i_030a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A24;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $030A24: andi.w #$ff, d0
pub fn i_030a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A28;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030A28: bra.w $30aa4
pub fn i_030a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A2C;
    cpu.ir = 0x6000;
    cpu.pc = 0x30AA4;
    10
}
/// $030A2C: cmpi.b #$1, $106f2a.l
pub fn i_030a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A34;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $030A34: bne.w $30a56
pub fn i_030a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A38;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30A56; return 10; }
    8
}
/// $030A38: move.w d0, d1
pub fn i_030a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A3A;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030A3A: moveq #$0, d0
pub fn i_030a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A3C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $030A3C: move.b $9b(a6), d0
pub fn i_030a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A40;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $030A40: andi.w #$1, d0
pub fn i_030a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A44;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030A44: lsl.w #$1, d0
pub fn i_030a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A46;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $030A46: subq.w #$1, d0
pub fn i_030a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A48;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030A48: add.w d1, d0
pub fn i_030a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A4A;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $030A4A: andi.w #$ff, d0
pub fn i_030a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A4E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030A4E: jsr $30bfe(pc)
pub fn i_030a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x30A52);
    cpu.pc = 0x30BFE;
    16
}
/// $030A52: bra.w $30aa4
pub fn i_030a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A56;
    cpu.ir = 0x6000;
    cpu.pc = 0x30AA4;
    10
}
/// $030A56: cmpi.b #$3, $106f2a.l
pub fn i_030a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A5E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $030A5E: bne.w $30a6a
pub fn i_030a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A62;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30A6A; return 10; }
    8
}
/// $030A62: jsr $30bfe(pc)
pub fn i_030a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x30A66);
    cpu.pc = 0x30BFE;
    16
}
/// $030A66: bra.w $30aa4
pub fn i_030a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A6A;
    cpu.ir = 0x6000;
    cpu.pc = 0x30AA4;
    10
}
/// $030A6A: cmpi.b #$2, $106f2a.l
pub fn i_030a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A72;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $030A72: bne.w $30a7e
pub fn i_030a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30A7E; return 10; }
    8
}
/// $030A76: jsr $30bfe(pc)
pub fn i_030a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A7A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x30A7A);
    cpu.pc = 0x30BFE;
    16
}
/// $030A7A: bra.w $30aa4
pub fn i_030a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A7E;
    cpu.ir = 0x6000;
    cpu.pc = 0x30AA4;
    10
}
/// $030A7E: cmpi.b #$0, $106f2a.l
pub fn i_030a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A86;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $030A86: bne.w $30aa4
pub fn i_030a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A8A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30AA4; return 10; }
    8
}
/// $030A8A: move.w d0, d1
pub fn i_030a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A8C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030A8C: moveq #$0, d0
pub fn i_030a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A8E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $030A8E: move.b $9b(a6), d0
pub fn i_030a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A92;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $030A92: addq.w #$4, d0
pub fn i_030a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A94;
    cpu.ir = 0x5840;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030A94: andi.w #$7, d0
pub fn i_030a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A98;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030A98: subq.w #$4, d0
pub fn i_030a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A9A;
    cpu.ir = 0x5940;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030A9A: add.w d1, d0
pub fn i_030a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30A9C;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $030A9C: andi.w #$ff, d0
pub fn i_030a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AA0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030AA0: bra.w $30aa4
pub fn i_030aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x30AA4;
    10
}
/// $030AA4: movem.w d0, -(a7)
pub fn i_030aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AA8;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $030AA8: move.b $7a(a6), d1
pub fn i_030aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AAC;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $030AAC: andi.b #$1, d1
pub fn i_030aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AB0;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $030AB0: beq.w $30adc
pub fn i_030ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AB4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30ADC; return 10; }
    8
}
/// $030AB4: move.w #$f800, d1
pub fn i_030ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AB8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030AB8: jsr $13c0e.l
pub fn i_030ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30ABE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30ABE);
    cpu.pc = 0x13C0E;
    16
}
/// $030ABE: movea.l $c(a6), a0
pub fn i_030abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AC2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $030AC2: btst.b #$0, $3a(a0)
pub fn i_030ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AC8;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $030AC8: beq.w $30ace
pub fn i_030ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30ACC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30ACE; return 10; }
    8
}
/// $030ACC: neg.w d1
pub fn i_030acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30ACE;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030ACE: move.w d1, $28(a6)
pub fn i_030ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AD2;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030AD2: move.w d2, $2a(a6)
pub fn i_030ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AD6;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030AD6: jsr $27cee.l
pub fn i_030ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30ADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30ADC);
    cpu.pc = 0x27CEE;
    16
}
/// $030ADC: movem.w (a7)+, d0
pub fn i_030adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AE0;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $030AE0: move.w #$800, d1
pub fn i_030ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AE4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030AE4: jsr $13c0e.l
pub fn i_030ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30AEA);
    cpu.pc = 0x13C0E;
    16
}
/// $030AEA: move.w d1, $28(a6)
pub fn i_030aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AEE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030AEE: move.w d2, $2a(a6)
pub fn i_030aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AF2;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030AF2: movea.l $c(a6), a0
pub fn i_030af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AF6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $030AF6: movea.l $c(a0), a0
pub fn i_030af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30AFA;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $030AFA: btst.b #$6, $6b(a0)
pub fn i_030afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B00;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $030B00: beq.w $30b1c
pub fn i_030b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30B1C; return 10; }
    8
}
/// $030B04: movea.l $c(a6), a0
pub fn i_030b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B08;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $030B08: move.b $3a(a0), d1
pub fn i_030b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B0C;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $030B0C: move.b d1, $3a(a6)
pub fn i_030b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B10;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $030B10: andi.b #$1, d1
pub fn i_030b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B14;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $030B14: beq.w $30b1c
pub fn i_030b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B18;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30B1C; return 10; }
    8
}
/// $030B18: neg.w $28(a6)
pub fn i_030b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B1C;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $030B1C: jsr $283ca.l
pub fn i_030b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30B22);
    cpu.pc = 0x283CA;
    16
}
/// $030B22: lea.l $30b28(pc), a1
pub fn i_030b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30B28;
    16
}
/// $030B26: move.l a1, (a6)
pub fn i_030b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030B28: bset.b #$6, $13(a6)
pub fn i_030b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B2E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $030B2E: jsr $27cee.l
pub fn i_030b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30B34);
    cpu.pc = 0x27CEE;
    16
}
/// $030B34: bcs.w $30b42
pub fn i_030b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B38;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x30B42; return 10; }
    8
}
/// $030B38: jsr $27cee.l
pub fn i_030b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30B3E);
    cpu.pc = 0x27CEE;
    16
}
/// $030B3E: bcc.w $30b6e
pub fn i_030b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x30B6E; return 10; }
    8
}
/// $030B42: lea.l $30bb6(pc), a1
pub fn i_030b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30BB6;
    16
}
/// $030B46: move.l a1, (a6)
pub fn i_030b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030B48: cmpi.b #$38, d0
pub fn i_030b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B4C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x38, Size::Byte, false);
    8
}
/// $030B4C: beq.w $30b64
pub fn i_030b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B50;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30B64; return 10; }
    8
}
/// $030B50: andi.b #$c0, d7
pub fn i_030b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B54;
    cpu.ir = 0x207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $030B54: cmpi.b #$0, d7
pub fn i_030b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B58;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $030B58: beq.w $30b6e
pub fn i_030b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B5C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30B6E; return 10; }
    8
}
/// $030B5C: cmpi.b #$c0, d7
pub fn i_030b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B60;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Byte, false);
    8
}
/// $030B60: beq.w $30b6e
pub fn i_030b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B64;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30B6E; return 10; }
    8
}
/// $030B64: lea.l $30bf6(pc), a1
pub fn i_030b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30BF6;
    16
}
/// $030B68: move.l a1, (a6)
pub fn i_030b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030B6A: bra.w $30b6e
pub fn i_030b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x30B6E;
    10
}
/// $030B6E: jsr $28d70.l
pub fn i_030b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30B74);
    cpu.pc = 0x28D70;
    16
}
/// $030B74: subi.b #$1, $76(a6)
pub fn i_030b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B7A;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $030B7A: bne.w $30b84
pub fn i_030b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B7E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30B84; return 10; }
    8
}
/// $030B7E: lea.l $30bf6(pc), a1
pub fn i_030b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30BF6;
    16
}
/// $030B82: move.l a1, (a6)
pub fn i_030b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030B84: jsr $283d8.l
pub fn i_030b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30B8A);
    cpu.pc = 0x283D8;
    16
}
/// $030B8A: btst.b #$1, $13(a6)
pub fn i_030b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B90;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $030B90: beq.w $30b9a
pub fn i_030b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30B9A; return 10; }
    8
}
/// $030B94: lea.l $30bd6(pc), a1
pub fn i_030b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30BD6;
    16
}
/// $030B98: move.l a1, (a6)
pub fn i_030b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30B9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030B9A: movea.l #$ffffffff, a0
pub fn i_030b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BA0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $030BA0: lea.l $308a8(pc), a0
pub fn i_030ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BA4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x308A8;
    16
}
/// $030BA4: jsr $5dd56.l
pub fn i_030ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30BAA);
    cpu.pc = 0x5DD56;
    16
}
/// $030BAA: bcc.w $30bb4
pub fn i_030baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x30BB4; return 10; }
    8
}
// ===== SetTaskHandler_030bae  ($030BAE, 8 B, kernel/task_handlers.c)
/// $030BAE: lea.l $30bf6(pc), a1
pub fn i_030bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30BF6;
    16
}
/// $030BB2: move.l a1, (a6)
pub fn i_030bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030BB4: rts
pub fn i_030bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerGrenade_ExplodeGround_030bb6  ($030BB6, 24 B, player/tables_fx_030696.s)
/// $030BB6: lea.l $31c72(pc), a1
pub fn i_030bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31C72;
    16
}
/// $030BBA: jsr $6fe.l
pub fn i_030bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30BC0);
    cpu.pc = 0x6FE;
    16
}
/// $030BC0: jsr $5dd02.l
pub fn i_030bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30BC6);
    cpu.pc = 0x5DD02;
    16
}
/// $030BC6: move.w $9e(a6), d1
pub fn i_030bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BCA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030BCA: move.w d1, $9e(a0)
pub fn i_030bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BCE;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9E);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_030bce  ($030BCE, 8 B, kernel/task_handlers.c)
/// $030BCE: lea.l $30bf6(pc), a1
pub fn i_030bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30BF6;
    16
}
/// $030BD2: move.l a1, (a6)
pub fn i_030bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030BD4: rts
pub fn i_030bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerGrenade_ExplodeAir_030bd6  ($030BD6, 24 B, player/tables_fx_030696.s)
/// $030BD6: lea.l $31cca(pc), a1
pub fn i_030bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31CCA;
    16
}
/// $030BDA: jsr $6fe.l
pub fn i_030bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30BE0);
    cpu.pc = 0x6FE;
    16
}
/// $030BE0: jsr $5dd02.l
pub fn i_030be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30BE6);
    cpu.pc = 0x5DD02;
    16
}
/// $030BE6: move.w $9e(a6), d1
pub fn i_030be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BEA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030BEA: move.w d1, $9e(a0)
pub fn i_030bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BEE;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9E);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_030bee  ($030BEE, 8 B, kernel/task_handlers.c)
/// $030BEE: lea.l $30bf6(pc), a1
pub fn i_030bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30BF6;
    16
}
/// $030BF2: move.l a1, (a6)
pub fn i_030bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030BF4: rts
pub fn i_030bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_030bf6  ($030BF6, 8 B, kernel/jmp_to_scheduler.c)
/// $030BF6: jmp $518.l
pub fn i_030bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30BFC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== PlayerGrenade_AngleFromSpread_030bfe  ($030BFE, 22 B, player/tables_fx_030696.s)
/// $030BFE: move.w d0, d1
pub fn i_030bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C00;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030C00: moveq #$0, d0
pub fn i_030c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C02;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $030C02: move.b $9b(a6), d0
pub fn i_030c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C06;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $030C06: andi.w #$3, d0
pub fn i_030c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C0A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030C0A: subq.w #$2, d0
pub fn i_030c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C0C;
    cpu.ir = 0x5540;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030C0C: add.w d1, d0
pub fn i_030c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C0E;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $030C0E: andi.w #$ff, d0
pub fn i_030c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C12;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030C12: rts
pub fn i_030c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $031A9C: jsr $2783a.l
pub fn i_031a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31AA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31AA2);
    cpu.pc = 0x2783A;
    16
}
/// $031AA2: jsr $28d70.l
pub fn i_031aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31AA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31AA8);
    cpu.pc = 0x28D70;
    16
}
/// $031AA8: bcc.w $31ab8
pub fn i_031aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31AAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x31AB8; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_031aac  ($031AAC, 14 B, kernel/jsr5b6_jmp518.c)
/// $031AAC: jsr $5b6.l
pub fn i_031aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31AB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31AB2);
    cpu.pc = 0x5B6;
    16
}
/// $031AB2: jmp $518.l
pub fn i_031ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31AB8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $031AB8: rts
pub fn i_031ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31ABA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fx_GroundBurst_031c72  ($031C72, 88 B, player/tables_fx_030696.s)
/// $031C72: move.w $9e(a6), d1
pub fn i_031c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C76;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031C76: cmpi.w #$1d6, d1
pub fn i_031c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C7A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1D6, Size::Word, false);
    8
}
/// $031C7A: bne.w $31c86
pub fn i_031c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C7E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x31C86; return 10; }
    8
}
/// $031C7E: move.w #$1d6, d1
pub fn i_031c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C82;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031C82: bra.w $31c8a
pub fn i_031c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C86;
    cpu.ir = 0x6000;
    cpu.pc = 0x31C8A;
    10
}
/// $031C86: move.w #$4, d1
pub fn i_031c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C8A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031C8A: jsr $236e.l
pub fn i_031c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31C90);
    cpu.pc = 0x236E;
    16
}
/// $031C90: jsr $5e9b6.l
pub fn i_031c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31C96);
    cpu.pc = 0x5E9B6;
    16
}
/// $031C96: andi.w #$f, d0
pub fn i_031c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C9A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031C9A: subq.w #$8, d0
pub fn i_031c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C9C;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $031C9C: add.w d0, $22(a6)
pub fn i_031c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CA0;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031CA0: andi.w #$1, d0
pub fn i_031ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CA4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031CA4: movea.l #$31c6a, a0
pub fn i_031ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CAA;
    cpu.ir = 0x207C;
    let v1: u32 = 0x31C6A;
    cpu.a[0] = v1;
    4
}
/// $031CAA: lsl.w #$2, d0
pub fn i_031caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CAC;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $031CAC: movea.l (a0, d0.w), a0
pub fn i_031cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CB0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $031CB0: cmpa.l #$ffffffff, a0
pub fn i_031cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CB6;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $031CB6: beq.w $31cc0
pub fn i_031cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CBA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x31CC0; return 10; }
    8
}
/// $031CBA: jsr $28cd4.l
pub fn i_031cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31CC0);
    cpu.pc = 0x28CD4;
    16
}
/// $031CC0: lea.l $31a9c(pc), a1
pub fn i_031cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31A9C;
    16
}
/// $031CC4: move.l a1, (a6)
pub fn i_031cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031CC6: bra.w $31a9c
pub fn i_031cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CCA;
    cpu.ir = 0x6000;
    cpu.pc = 0x31A9C;
    10
}
// ===== Fx_AirBurst_031cca  ($031CCA, 84 B, player/tables_fx_030696.s)
/// $031CCA: move.w $9e(a6), d1
pub fn i_031cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CCE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031CCE: cmpi.w #$1d6, d1
pub fn i_031cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CD2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1D6, Size::Word, false);
    8
}
/// $031CD2: bne.w $31cde
pub fn i_031cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CD6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x31CDE; return 10; }
    8
}
/// $031CD6: move.w #$1d6, d1
pub fn i_031cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CDA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031CDA: bra.w $31ce2
pub fn i_031cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CDE;
    cpu.ir = 0x6000;
    cpu.pc = 0x31CE2;
    10
}
/// $031CDE: move.w #$2, d1
pub fn i_031cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CE2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031CE2: jsr $236e.l
pub fn i_031ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31CE8);
    cpu.pc = 0x236E;
    16
}
/// $031CE8: jsr $5e9b6.l
pub fn i_031ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31CEE);
    cpu.pc = 0x5E9B6;
    16
}
/// $031CEE: andi.w #$1f, d0
pub fn i_031cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CF2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031CF2: subq.w #$8, d0
pub fn i_031cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CF4;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $031CF4: add.w d0, $22(a6)
pub fn i_031cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CF8;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031CF8: jsr $5e9b6.l
pub fn i_031cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31CFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31CFE);
    cpu.pc = 0x5E9B6;
    16
}
/// $031CFE: andi.w #$1f, d0
pub fn i_031cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D02;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031D02: subq.w #$8, d0
pub fn i_031d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D04;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $031D04: add.w d0, $24(a6)
pub fn i_031d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D08;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031D08: lea.l $2dd8b6.l, a0
pub fn i_031d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D0E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD8B6;
    20
}
/// $031D0E: jsr $28cd4.l
pub fn i_031d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31D14);
    cpu.pc = 0x28CD4;
    16
}
/// $031D14: lea.l $31a9c(pc), a1
pub fn i_031d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31A9C;
    16
}
/// $031D18: move.l a1, (a6)
pub fn i_031d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031D1A: bra.w $31a9c
pub fn i_031d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D1E;
    cpu.ir = 0x6000;
    cpu.pc = 0x31A9C;
    10
}
// ===== Entity_TestBit2Field8C_031fc2  ($031FC2, 8 B, player/tables_fx_030696.s)
/// $031FC2: btst.b #$2, $8c(a0)
pub fn i_031fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FC8;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $031FC8: rts
pub fn i_031fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerIcon_Pow_031fca  ($031FCA, 158 B, player/tables_fx_030696.s)
/// $031FCA: move.w #$1ad, d1
pub fn i_031fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FCE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031FCE: jsr $236e.l
pub fn i_031fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31FD4);
    cpu.pc = 0x236E;
    16
}
/// $031FD4: cmpi.b #$5, $106ece.l
pub fn i_031fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FDC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x5, Size::Byte, false);
    20
}
/// $031FDC: bne.w $31fee
pub fn i_031fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x31FEE; return 10; }
    8
}
/// $031FE0: lea.l $32286(pc), a0
pub fn i_031fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FE4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32286;
    16
}
/// $031FE4: jsr $28cd4.l
pub fn i_031fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31FEA);
    cpu.pc = 0x28CD4;
    16
}
/// $031FEA: bra.w $31ff8
pub fn i_031fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x31FF8;
    10
}
/// $031FEE: lea.l $32292(pc), a0
pub fn i_031fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FF2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32292;
    16
}
/// $031FF2: jsr $28cd4.l
pub fn i_031ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31FF8);
    cpu.pc = 0x28CD4;
    16
}
/// $031FF8: move.w #$28, $5c(a6)
pub fn i_031ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FFE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031FFE: move.w #$ffff, $38(a6)
pub fn i_031ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32004;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032004: bset.b #$6, $12(a6)
pub fn i_032004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3200A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03200A: lea.l $32010(pc), a1
pub fn i_03200a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3200E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x32010;
    16
}
/// $03200E: move.l a1, (a6)
pub fn i_03200e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32010;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $032010: jsr $5e4ca.l
pub fn i_032010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32016;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32016);
    cpu.pc = 0x5E4CA;
    16
}
/// $032016: move.w d1, $22(a6)
pub fn i_032016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3201A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03201A: add.w $5c(a6), d2
pub fn i_03201a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3201E;
    cpu.ir = 0xD46E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03201E: move.w d2, $24(a6)
pub fn i_03201e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32022;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032022: move.w d0, $38(a6)
pub fn i_032022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32026;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032026: movea.l $c(a6), a0
pub fn i_032026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3202A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03202A: cmpi.b #$0, $106ed3.l
pub fn i_03202a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32032;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ED3) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $032032: beq.w $3205a
pub fn i_032032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32036;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3205A; return 10; }
    8
}
/// $032036: btst.b #$5, $8d(a0)
pub fn i_032036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3203C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03203C: beq.w $3205a
pub fn i_03203c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32040;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3205A; return 10; }
    8
}
/// $032040: movea.l #$ffffffff, a0
pub fn i_032040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32046;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $032046: lea.l $32070(pc), a0
pub fn i_032046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3204A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32070;
    16
}
/// $03204A: jsr $5dd56.l
pub fn i_03204a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32050;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32050);
    cpu.pc = 0x5DD56;
    16
}
/// $032050: bcs.w $3205a
pub fn i_032050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32054;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3205A; return 10; }
    8
}
/// $032054: jsr $28d70.l
pub fn i_032054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3205A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3205A);
    cpu.pc = 0x28D70;
    16
}
/// $03205A: movea.l $c(a6), a0
pub fn i_03205a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3205E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03205E: btst.b #$0, $13(a0)
pub fn i_03205e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32064;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $032064: beq.w $3206e
pub fn i_032064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32068;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3206E; return 10; }
    8
}
// ===== SetTaskHandler_032068  ($032068, 8 B, kernel/task_handlers.c)
/// $032068: lea.l $321bc(pc), a1
pub fn i_032068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3206C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x321BC;
    16
}
/// $03206C: move.l a1, (a6)
pub fn i_03206c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3206E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03206E: rts
pub fn i_03206e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32070;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerIcon_FreeFallP1_032112  ($032112, 48 B, player/tables_fx_030696.s)
/// $032112: cmpi.l #$100440, $c(a6)
pub fn i_032112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3211A;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x100440, Size::Long, false);
    20
}
/// $03211A: beq.w $32124
pub fn i_03211a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3211E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32124; return 10; }
    8
}
/// $03211E: jmp $518.l
pub fn i_03211e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32124;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $032124: move.w #$a2, d1
pub fn i_032124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32128;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $032128: jsr $236e.l
pub fn i_032128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3212E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3212E);
    cpu.pc = 0x236E;
    16
}
/// $03212E: lea.l $321c2(pc), a0
pub fn i_03212e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32132;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x321C2;
    16
}
/// $032132: jsr $28cd4.l
pub fn i_032132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32138;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32138);
    cpu.pc = 0x28CD4;
    16
}
/// $032138: move.w #$30, $5c(a6)
pub fn i_032138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3213E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03213E: bra.w $3216e
pub fn i_03213e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32142;
    cpu.ir = 0x6000;
    cpu.pc = 0x3216E;
    10
}
/// $03216E: move.w #$ffff, $38(a6)
pub fn i_03216e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32174;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032174: bset.b #$6, $12(a6)
pub fn i_032174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3217A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03217A: lea.l $32180(pc), a1
pub fn i_03217a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3217E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x32180;
    16
}
/// $03217E: move.l a1, (a6)
pub fn i_03217e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32180;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $032180: jsr $5e4ca.l
pub fn i_032180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32186;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32186);
    cpu.pc = 0x5E4CA;
    16
}
/// $032186: move.w d1, $22(a6)
pub fn i_032186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3218A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03218A: add.w $5c(a6), d2
pub fn i_03218a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3218E;
    cpu.ir = 0xD46E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03218E: move.w d2, $24(a6)
pub fn i_03218e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32192;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032192: move.w d0, $38(a6)
pub fn i_032192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32196;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032196: jsr $28d70.l
pub fn i_032196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3219C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3219C);
    cpu.pc = 0x28D70;
    16
}
/// $03219C: bcc.w $321a6
pub fn i_03219c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x321A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x321A6; return 10; }
    8
}
/// $0321A0: lea.l $321bc(pc), a1
pub fn i_0321a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x321A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x321BC;
    16
}
/// $0321A4: move.l a1, (a6)
pub fn i_0321a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x321A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0321A6: movea.l $c(a6), a0
pub fn i_0321a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x321AA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0321AA: btst.b #$0, $13(a0)
pub fn i_0321aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x321B0;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0321B0: beq.w $321ba
pub fn i_0321b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x321B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x321BA; return 10; }
    8
}
// ===== SetTaskHandler_0321b4  ($0321B4, 8 B, kernel/task_handlers.c)
/// $0321B4: lea.l $321bc(pc), a1
pub fn i_0321b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x321B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x321BC;
    16
}
/// $0321B8: move.l a1, (a6)
pub fn i_0321b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x321BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0321BA: rts
pub fn i_0321ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x321BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_0321bc  ($0321BC, 6 B, thunks/jmp_abs_thunks.c)
/// $0321BC: jmp $518.l
pub fn i_0321bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x321C2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== JsrPcThunk_032406  ($032406, 6 B, thunks/jsr_pc_thunks.c)
/// $032406: jsr $32ea4(pc)
pub fn i_032406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3240A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3240A);
    cpu.pc = 0x32EA4;
    16
}
/// $03240A: rts
pub fn i_03240a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3240C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_03240c  ($03240C, 6 B, thunks/jsr_pc_thunks.c)
/// $03240C: jsr $32eba(pc)
pub fn i_03240c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32410;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x32410);
    cpu.pc = 0x32EBA;
    16
}
/// $032410: rts
pub fn i_032410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32412;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerEntity_InitAuxState_032A02  ($032A02, 158 B, player/entity_init_aux_state_032a02.s)
/// $032A02: move.w $16(a6), $14(a6)
pub fn i_032a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A08;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $032A08: bset.b #$2, $5b(a6)
pub fn i_032a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A0E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $032A0E: bset.b #$6, $6b(a6)
pub fn i_032a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A14;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $032A14: move.l #$32500, $60(a6)
pub fn i_032a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A1C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $032A1C: move.b #$ff, $32(a6)
pub fn i_032a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A22;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032A22: move.b #$ff, $33(a6)
pub fn i_032a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A28;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032A28: move.w #$8000, $38(a6)
pub fn i_032a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A2E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032A2E: ori.w #$10, $38(a6)
pub fn i_032a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A34;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $032A34: jsr $5e98a.l
pub fn i_032a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32A3A);
    cpu.pc = 0x5E98A;
    16
}
/// $032A3A: move.b #$a, $80(a6)
pub fn i_032a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A40;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032A40: move.w #$0, $82(a6)
pub fn i_032a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A46;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032A46: move.b #$1, $85(a6)
pub fn i_032a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A4C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032A4C: clr.b $8c(a6)
pub fn i_032a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A50;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $032A50: clr.b $87(a6)
pub fn i_032a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A54;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x87);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $032A54: jsr $8f6d2.l
pub fn i_032a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32A5A);
    cpu.pc = 0x8F6D2;
    16
}
/// $032A5A: clr.w $72(a6)
pub fn i_032a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A5E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $032A5E: clr.w $36(a6)
pub fn i_032a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A62;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $032A62: clr.b $91(a6)
pub fn i_032a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A66;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $032A66: clr.b $21(a6)
pub fn i_032a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A6A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $032A6A: move.b #$0, $71(a6)
pub fn i_032a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A70;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032A70: move.b #$0, $70(a6)
pub fn i_032a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A76;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032A76: move.b #$1, $78(a6)
pub fn i_032a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A7C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032A7C: move.w #$0, $7c(a6)
pub fn i_032a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A82;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032A82: move.w #$0, $7e(a6)
pub fn i_032a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A88;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032A88: bclr.b #$3, $13(a6)
pub fn i_032a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A8E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $032A8E: jsr $517aa.l
pub fn i_032a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32A94);
    cpu.pc = 0x517AA;
    16
}
/// $032A94: lea.l $776e2.l, a1
pub fn i_032a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A9A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $032A9A: jsr $4ae.l
pub fn i_032a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32AA0);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_032aa0  ($032AA0, 8 B, thunks/jsr_abs_thunks.c)
/// $032AA0: jsr $5dd02.l
pub fn i_032aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32AA6);
    cpu.pc = 0x5DD02;
    16
}
/// $032AA6: rts
pub fn i_032aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CheckDeathOrState21_032aa8  ($032AA8, 20 B, player/core_032aa8.s)
/// $032AA8: jsr $28d70.l
pub fn i_032aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32AAE);
    cpu.pc = 0x28D70;
    16
}
/// $032AAE: bcs.w $32ac2
pub fn i_032aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AB2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x32AC2; return 10; }
    8
}
/// $032AB2: cmpi.b #$0, $21(a6)
pub fn i_032ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AB8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $032AB8: beq.w $32ac2
pub fn i_032ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32ABC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32AC2; return 10; }
    8
}
// ===== ClearXN_032abc  ($032ABC, 6 B, thunks/ccr_helpers.c)
/// $032ABC: andi.b #$ee, ccr
pub fn i_032abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AC0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $032AC0: rts
pub fn i_032ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_032ac2  ($032AC2, 6 B, thunks/ccr_helpers.c)
/// $032AC2: ori.b #$11, ccr
pub fn i_032ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AC6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $032AC6: rts
pub fn i_032ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_PlaceAtScrollOffset_032ac8  ($032AC8, 34 B, player/core_032aa8.s)
/// $032AC8: jsr $4cac4.l
pub fn i_032ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32ACE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32ACE);
    cpu.pc = 0x4CAC4;
    16
}
/// $032ACE: cmpa.l #$100440, a6
pub fn i_032ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AD4;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $032AD4: bne.w $32ade
pub fn i_032ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AD8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32ADE; return 10; }
    8
}
/// $032AD8: add.w d3, d0
pub fn i_032ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32ADA;
    cpu.ir = 0xD043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $032ADA: bra.w $32ae0
pub fn i_032ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32ADE;
    cpu.ir = 0x6000;
    cpu.pc = 0x32AE0;
    10
}
/// $032ADE: sub.w d3, d0
pub fn i_032ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AE0;
    cpu.ir = 0x9043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $032AE0: move.w d0, $22(a6)
pub fn i_032ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AE4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032AE4: move.w d1, $24(a6)
pub fn i_032ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AE8;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032AE8: rts
pub fn i_032ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_ConsumeFlag81_032aea  ($032AEA, 16 B, player/core_032aa8.s)
/// $032AEA: move.b #$1, d1
pub fn i_032aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AEE;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $032AEE: move.b $81(a6), d0
pub fn i_032aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AF2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032AF2: clr.b $81(a6)
pub fn i_032af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AF6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x81);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $032AF6: cmp.b d0, d1
pub fn i_032af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AF8;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $032AF8: rts
pub fn i_032af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32AFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_IncCounterAt81_032AFA  ($032AFA, 34 B, player/inc_counter_at81_032afa.s)
/// $032AFA: cmpi.b #$1, $6d(a6)
pub fn i_032afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B00;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $032B00: bne.w $32b0e
pub fn i_032b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B04;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32B0E; return 10; }
    8
}
/// $032B04: lea.l $100440.l, a1
pub fn i_032b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B0A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $032B0A: bra.w $32b14
pub fn i_032b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B0E;
    cpu.ir = 0x6000;
    cpu.pc = 0x32B14;
    10
}
/// $032B0E: lea.l $1004e0.l, a1
pub fn i_032b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B14;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $032B14: addi.b #$1, $81(a1)
pub fn i_032b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B1A;
    cpu.ir = 0x629;
    let ad1 = cpu.a[1].wrapping_add(0x81);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $032B1A: rts
pub fn i_032b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_ConsumeFireFlag84_032b1c  ($032B1C, 26 B, player/core_032aa8.s)
/// $032B1C: lea.l $329e8(pc), a1
pub fn i_032b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x329E8;
    16
}
/// $032B20: moveq #$0, d0
pub fn i_032b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B22;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $032B22: move.b $71(a6), d0
pub fn i_032b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B26;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032B26: move.b (a1, d0.w), d1
pub fn i_032b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B2A;
    cpu.ir = 0x1231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $032B2A: move.b $84(a6), d0
pub fn i_032b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B2E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032B2E: clr.b $84(a6)
pub fn i_032b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B32;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $032B32: cmp.b d0, d1
pub fn i_032b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B34;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $032B34: rts
pub fn i_032b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_SetWeaponAndAmmo_032b58  ($032B58, 18 B, player/core_032aa8.s)
/// $032B58: movem.w d1, -(a7)
pub fn i_032b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B5C;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $032B5C: jsr $32b6a(pc)
pub fn i_032b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x32B60);
    cpu.pc = 0x32B6A;
    16
}
/// $032B60: movem.w (a7)+, d1
pub fn i_032b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B64;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $032B64: move.b d1, $71(a1)
pub fn i_032b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B68;
    cpu.ir = 0x1341;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032B68: rts
pub fn i_032b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_SetAmmoForWeapon_032b6a  ($032B6A, 92 B, player/core_032aa8.s)
/// $032B6A: andi.w #$ff, d2
pub fn i_032b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B6E;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $032B6E: cmpi.b #$0, d0
pub fn i_032b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B72;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $032B72: bne.w $32b80
pub fn i_032b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32B80; return 10; }
    8
}
/// $032B76: lea.l $100440.l, a1
pub fn i_032b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B7C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $032B7C: bra.w $32b86
pub fn i_032b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B80;
    cpu.ir = 0x6000;
    cpu.pc = 0x32B86;
    10
}
/// $032B80: lea.l $1004e0.l, a1
pub fn i_032b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B86;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $032B86: andi.w #$ff, d1
pub fn i_032b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B8A;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $032B8A: lea.l $329d4(pc), a2
pub fn i_032b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B8E;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x329D4;
    16
}
/// $032B8E: cmp.b $71(a1), d1
pub fn i_032b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B92;
    cpu.ir = 0xB229;
    let ad2 = cpu.a[1].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $032B92: bne.w $32bb2
pub fn i_032b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32BB2; return 10; }
    8
}
/// $032B96: cmpi.b #$0, d2
pub fn i_032b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B9A;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $032B9A: beq.w $32ba4
pub fn i_032b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B9E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32BA4; return 10; }
    8
}
/// $032B9E: lsl.w #$2, d1
pub fn i_032b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BA0;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $032BA0: move.w $2(a2, d1.w), d2
pub fn i_032ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BA4;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $032BA4: move.w $82(a1), d0
pub fn i_032ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BA8;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $032BA8: add.w d2, d0
pub fn i_032ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BAA;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $032BAA: move.w d0, $82(a1)
pub fn i_032baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BAE;
    cpu.ir = 0x3340;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032BAE: bra.w $32bc4
pub fn i_032bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x32BC4;
    10
}
/// $032BB2: cmpi.b #$0, d2
pub fn i_032bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BB6;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $032BB6: bne.w $32bc0
pub fn i_032bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BBA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32BC0; return 10; }
    8
}
/// $032BBA: lsl.w #$2, d1
pub fn i_032bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BBC;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $032BBC: move.w $2(a2, d1.w), d2
pub fn i_032bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BC0;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $032BC0: move.w d2, $82(a1)
pub fn i_032bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BC4;
    cpu.ir = 0x3342;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032BC4: rts
pub fn i_032bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_StartInvuln5_032cba  ($032CBA, 14 B, player/core_032aa8.s)
/// $032CBA: move.b #$5, $87(a1)
pub fn i_032cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CC0;
    cpu.ir = 0x137C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032CC0: bset.b #$4, $8c(a1)
pub fn i_032cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CC6;
    cpu.ir = 0x8E9;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[1].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $032CC6: rts
pub fn i_032cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_InvulnBlinkStep_032cc8  ($032CC8, 56 B, player/core_032aa8.s)
/// $032CC8: cmpi.b #$0, $87(a6)
pub fn i_032cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CCE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $032CCE: beq.w $32cf8
pub fn i_032cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CD2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32CF8; return 10; }
    8
}
/// $032CD2: subi.b #$1, $87(a6)
pub fn i_032cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CD8;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x87);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $032CD8: moveq #$0, d1
pub fn i_032cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CDA;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $032CDA: move.b $87(a6), d1
pub fn i_032cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CDE;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032CDE: andi.w #$f, d1
pub fn i_032cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CE2;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $032CE2: lsl.w #$1, d1
pub fn i_032ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CE4;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $032CE4: lea.l $329ee(pc), a1
pub fn i_032ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x329EE;
    16
}
/// $032CE8: move.w (a1, d1.w), d0
pub fn i_032ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CEC;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $032CEC: move.w (a6, d0.w), d1
pub fn i_032cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CF0;
    cpu.ir = 0x3236;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $032CF0: move.w d1, $14(a6)
pub fn i_032cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CF4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032CF4: bra.w $32cfe
pub fn i_032cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x32CFE;
    10
}
/// $032CF8: move.w $16(a6), $14(a6)
pub fn i_032cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CFE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $032CFE: rts
pub fn i_032cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_DispatchOpcodeNibble_032D00  ($032D00, 40 B, entity/dispatch_opcode_nibble_032d00.s)
/// $032D00: movea.l $c(a6), a0
pub fn i_032d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D04;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $032D04: move.b $87(a0), $87(a6)
pub fn i_032d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D0A;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad4, v3 as u8);
    20
}
/// $032D0A: moveq #$0, d1
pub fn i_032d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D0C;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $032D0C: move.b $87(a6), d1
pub fn i_032d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D10;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032D10: andi.w #$f, d1
pub fn i_032d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D14;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $032D14: lsl.w #$1, d1
pub fn i_032d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D16;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $032D16: lea.l $329ee(pc), a1
pub fn i_032d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x329EE;
    16
}
/// $032D1A: move.w (a1, d1.w), d0
pub fn i_032d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D1E;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $032D1E: move.w (a6, d0.w), d1
pub fn i_032d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D22;
    cpu.ir = 0x3236;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $032D22: move.w d1, $14(a6)
pub fn i_032d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D26;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032D26: rts
pub fn i_032d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_PlayWeaponMusicIfFlag_032d32  ($032D32, 46 B, player/core_032aa8.s)
/// $032D32: btst.b #$4, $8c(a6)
pub fn i_032d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D38;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $032D38: beq.w $32d66
pub fn i_032d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D3C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32D66; return 10; }
    8
}
/// $032D3C: bclr.b #$4, $8c(a6)
pub fn i_032d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D42;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $032D42: moveq #$0, d0
pub fn i_032d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D44;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $032D44: move.b $71(a6), d0
pub fn i_032d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D48;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032D48: lsl.w #$1, d0
pub fn i_032d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D4A;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $032D4A: lea.l $32d28(pc), a0
pub fn i_032d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D4E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32D28;
    16
}
/// $032D4E: move.w (a0, d0.w), d0
pub fn i_032d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D52;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $032D52: cmpi.w #$ffff, d0
pub fn i_032d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D56;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $032D56: beq.w $32d60
pub fn i_032d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D5A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32D60; return 10; }
    8
}
/// $032D5A: jsr $2352.l
pub fn i_032d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32D60);
    cpu.pc = 0x2352;
    16
}
// ===== SetXN_032d60  ($032D60, 6 B, thunks/ccr_helpers.c)
/// $032D60: ori.b #$11, ccr
pub fn i_032d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D64;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $032D64: rts
pub fn i_032d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_032d66  ($032D66, 6 B, thunks/ccr_helpers.c)
/// $032D66: andi.b #$ee, ccr
pub fn i_032d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D6A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $032D6A: rts
pub fn i_032d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_LinkRidePartner_032d6c  ($032D6C, 104 B, player/core_032aa8.s)
/// $032D6C: cmpi.b #$1, d2
pub fn i_032d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D70;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $032D70: bne.w $32d7e
pub fn i_032d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32D7E; return 10; }
    8
}
/// $032D74: lea.l $100440.l, a0
pub fn i_032d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $032D7A: bra.w $32d94
pub fn i_032d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D7E;
    cpu.ir = 0x6000;
    cpu.pc = 0x32D94;
    10
}
/// $032D7E: cmpi.b #$2, d2
pub fn i_032d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D82;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $032D82: bne.w $32d90
pub fn i_032d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D86;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32D90; return 10; }
    8
}
/// $032D86: lea.l $1004e0.l, a0
pub fn i_032d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $032D8C: bra.w $32d94
pub fn i_032d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D90;
    cpu.ir = 0x6000;
    cpu.pc = 0x32D94;
    10
}
/// $032D90: bra.w $32dda
pub fn i_032d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D94;
    cpu.ir = 0x6000;
    cpu.pc = 0x32DDA;
    10
}
/// $032D94: btst.b #$0, $8c(a0)
pub fn i_032d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D9A;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $032D9A: bne.w $32dda
pub fn i_032d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32D9E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32DDA; return 10; }
    8
}
/// $032D9E: btst.b #$5, $8d(a0)
pub fn i_032d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DA4;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $032DA4: beq.w $32dda
pub fn i_032da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32DDA; return 10; }
    8
}
/// $032DA8: btst.b #$1, $8d(a0)
pub fn i_032da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DAE;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $032DAE: beq.w $32dda
pub fn i_032dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DB2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32DDA; return 10; }
    8
}
/// $032DB2: cmpi.w #$0, $2a(a0)
pub fn i_032db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DB8;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $032DB8: bge.w $32dda
pub fn i_032db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DBC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x32DDA; return 10; }
    8
}
/// $032DBC: move.b d2, $6d(a6)
pub fn i_032dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DC0;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032DC0: bset.b #$0, $8c(a0)
pub fn i_032dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DC6;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $032DC6: move.b $68(a0), d2
pub fn i_032dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DCA;
    cpu.ir = 0x1428;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032DCA: move.b d2, $68(a6)
pub fn i_032dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DCE;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032DCE: jsr $2aca2.l
pub fn i_032dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32DD4);
    cpu.pc = 0x2ACA2;
    16
}
// ===== SetXN_032dd4  ($032DD4, 6 B, thunks/ccr_helpers.c)
/// $032DD4: ori.b #$11, ccr
pub fn i_032dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DD8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $032DD8: rts
pub fn i_032dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_032dda  ($032DDA, 6 B, thunks/ccr_helpers.c)
/// $032DDA: andi.b #$ee, ccr
pub fn i_032dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DDE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $032DDE: rts
pub fn i_032dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_032e3c  ($032E3C, 6 B, thunks/jmp_abs_thunks.c)
/// $032E3C: jmp $5cf04.l
pub fn i_032e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E42;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CF04;
    8
}
// ===== Input_RightThunk_032e42  ($032E42, 6 B, player/core_032aa8.s)
/// $032E42: jmp $5cf10.l
pub fn i_032e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E48;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CF10;
    8
}
// ===== Input_ForwardByFacing_032e48  ($032E48, 18 B, player/core_032aa8.s)
/// $032E48: btst.b #$0, $3a(a6)
pub fn i_032e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E4E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $032E4E: bne.w $32e56
pub fn i_032e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32E56; return 10; }
    8
}
/// $032E52: jmp $32e3c(pc)
pub fn i_032e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E56;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x32E3C;
    8
}
/// $032E56: jmp $32e42(pc)
pub fn i_032e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E5A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x32E42;
    8
}
// ===== Stub_00032E8E  ($032E8E, 2 B, thunks/trivial_rts.c)
/// $032E8E: rts
pub fn i_032e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_DownPressed_032e90  ($032E90, 8 B, player/core_032aa8.s)
/// $032E90: jsr $330ca(pc)
pub fn i_032e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x32E94);
    cpu.pc = 0x330CA;
    16
}
/// $032E94: bcc.w $32e9e
pub fn i_032e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E98;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x32E9E; return 10; }
    8
}
// ===== SetXN_032e98  ($032E98, 6 B, thunks/ccr_helpers.c)
/// $032E98: ori.b #$11, ccr
pub fn i_032e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E9C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $032E9C: rts
pub fn i_032e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_032e9e  ($032E9E, 6 B, thunks/ccr_helpers.c)
/// $032E9E: andi.b #$ee, ccr
pub fn i_032e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EA2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $032EA2: rts
pub fn i_032ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_ClearBoth8D_032ea4  ($032EA4, 22 B, player/core_032aa8.s)
/// $032EA4: lea.l $100440.l, a0
pub fn i_032ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $032EAA: clr.b $8d(a0)
pub fn i_032eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EAE;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $032EAE: lea.l $1004e0.l, a0
pub fn i_032eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EB4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $032EB4: clr.b $8d(a0)
pub fn i_032eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EB8;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $032EB8: rts
pub fn i_032eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_ClearBoth88_032eba  ($032EBA, 22 B, player/core_032aa8.s)
/// $032EBA: lea.l $100440.l, a0
pub fn i_032eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EC0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $032EC0: clr.b $88(a0)
pub fn i_032ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EC4;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $032EC4: lea.l $1004e0.l, a0
pub fn i_032ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32ECA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $032ECA: clr.b $88(a0)
pub fn i_032eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32ECE;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $032ECE: rts
pub fn i_032ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32ED0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_ReadDirNibble_032f3c  ($032F3C, 70 B, player/core_032aa8.s)
/// $032F3C: move.b $78(a6), d0
pub fn i_032f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F40;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032F40: movem.w d0, -(a7)
pub fn i_032f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F44;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $032F44: move.b #$1, $78(a6)
pub fn i_032f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F4A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032F4A: jsr $32e48(pc)
pub fn i_032f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F4E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x32F4E);
    cpu.pc = 0x32E48;
    16
}
/// $032F4E: bcc.w $32f58
pub fn i_032f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F52;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x32F58; return 10; }
    8
}
/// $032F52: move.b #$1, $78(a6)
pub fn i_032f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F58;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032F58: jsr $5cef8.l
pub fn i_032f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32F5E);
    cpu.pc = 0x5CEF8;
    16
}
/// $032F5E: bcc.w $32f68
pub fn i_032f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F62;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x32F68; return 10; }
    8
}
/// $032F62: move.b #$8, $78(a6)
pub fn i_032f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F68;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032F68: jsr $5ceec.l
pub fn i_032f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32F6E);
    cpu.pc = 0x5CEEC;
    16
}
/// $032F6E: bcc.w $32f78
pub fn i_032f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x32F78; return 10; }
    8
}
/// $032F72: move.b #$4, $78(a6)
pub fn i_032f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F78;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032F78: movem.w (a7)+, d0
pub fn i_032f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F7C;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $032F7C: lsl.b #$4, d0
pub fn i_032f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F7E;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $032F7E: or.b $78(a6), d0
pub fn i_032f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F82;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
// ===== SetTaskB_032f82  ($032F82, 6 B, kernel/task_setters_b.c)
/// $032F82: move.b d0, $79(a6)
pub fn i_032f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F86;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032F86: rts
pub fn i_032f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_ReadFireNibble_032f88  ($032F88, 44 B, player/core_032aa8.s)
/// $032F88: move.b $78(a6), d0
pub fn i_032f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F8C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032F8C: movem.w d0, -(a7)
pub fn i_032f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F90;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $032F90: jsr $5ceec.l
pub fn i_032f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32F96);
    cpu.pc = 0x5CEEC;
    16
}
/// $032F96: bcc.w $32fa4
pub fn i_032f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F9A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x32FA4; return 10; }
    8
}
/// $032F9A: move.b #$4, $78(a6)
pub fn i_032f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FA0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032FA0: bra.w $32faa
pub fn i_032fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x32FAA;
    10
}
/// $032FA4: move.b #$1, $78(a6)
pub fn i_032fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032FAA: movem.w (a7)+, d0
pub fn i_032faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FAE;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $032FAE: lsl.b #$4, d0
pub fn i_032fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FB0;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $032FB0: or.b $78(a6), d0
pub fn i_032fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FB4;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
// ===== SetTaskB_032fb4  ($032FB4, 6 B, kernel/task_setters_b.c)
/// $032FB4: move.b d0, $79(a6)
pub fn i_032fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FB8;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032FB8: rts
pub fn i_032fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_GetAmmoOrFFFF_032fba  ($032FBA, 20 B, player/core_032aa8.s)
/// $032FBA: move.w $82(a0), d0
pub fn i_032fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FBE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $032FBE: cmpi.b #$0, $71(a0)
pub fn i_032fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FC4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $032FC4: bne.w $32fcc
pub fn i_032fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FC8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32FCC; return 10; }
    8
}
/// $032FC8: move.w #$ffff, d0
pub fn i_032fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FCC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $032FCC: rts
pub fn i_032fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_GetBombs_032fce  ($032FCE, 6 B, player/core_032aa8.s)
/// $032FCE: move.b $80(a0), d0
pub fn i_032fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FD2;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032FD2: rts
pub fn i_032fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00032FF0  ($032FF0, 2 B, thunks/trivial_rts.c)
/// $032FF0: rts
pub fn i_032ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_FrameCommon_032ff2  ($032FF2, 30 B, player/core_032aa8.s)
/// $032FF2: jsr $32ff0(pc)
pub fn i_032ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x32FF6);
    cpu.pc = 0x32FF0;
    16
}
/// $032FF6: jsr $283ca.l
pub fn i_032ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32FFC);
    cpu.pc = 0x283CA;
    16
}
/// $032FFC: andi.w #$fffc, $38(a6)
pub fn i_032ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33002;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFFC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $033002: jsr $32cc8(pc)
pub fn i_033002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33006;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33006);
    cpu.pc = 0x32CC8;
    16
}
/// $033006: jsr $2a720.l
pub fn i_033006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3300C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3300C);
    cpu.pc = 0x2A720;
    16
}
/// $03300C: jsr $32e8e(pc)
pub fn i_03300c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33010;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33010);
    cpu.pc = 0x32E8E;
    16
}
// ===== JsrPcThunk_033010  ($033010, 6 B, thunks/jsr_pc_thunks.c)
/// $033010: jsr $32f88(pc)
pub fn i_033010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33014;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33014);
    cpu.pc = 0x32F88;
    16
}
/// $033014: rts
pub fn i_033014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33016;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_FrameCommon_NoPrio_033016  ($033016, 24 B, player/core_032aa8.s)
/// $033016: jsr $32ff0(pc)
pub fn i_033016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3301A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3301A);
    cpu.pc = 0x32FF0;
    16
}
/// $03301A: jsr $283ca.l
pub fn i_03301a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33020;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33020);
    cpu.pc = 0x283CA;
    16
}
/// $033020: jsr $32cc8(pc)
pub fn i_033020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33024;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33024);
    cpu.pc = 0x32CC8;
    16
}
/// $033024: jsr $2a720.l
pub fn i_033024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3302A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3302A);
    cpu.pc = 0x2A720;
    16
}
/// $03302A: jsr $32e8e(pc)
pub fn i_03302a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3302E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3302E);
    cpu.pc = 0x32E8E;
    16
}
// ===== JsrPcThunk_03302e  ($03302E, 6 B, thunks/jsr_pc_thunks.c)
/// $03302E: jsr $32f3c(pc)
pub fn i_03302e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33032;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33032);
    cpu.pc = 0x32F3C;
    16
}
/// $033032: rts
pub fn i_033032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33034;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_FireByMode_033034  ($033034, 56 B, player/core_032aa8.s)
/// $033034: cmpi.b #$0, $106f2a.l
pub fn i_033034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3303C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $03303C: bne.w $3304a
pub fn i_03303c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33040;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3304A; return 10; }
    8
}
/// $033040: jsr $5cdb4.l
pub fn i_033040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33046;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33046);
    cpu.pc = 0x5CDB4;
    16
}
/// $033046: bra.w $33066
pub fn i_033046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3304A;
    cpu.ir = 0x6000;
    cpu.pc = 0x33066;
    10
}
/// $03304A: cmpi.b #$4, $106f2a.l
pub fn i_03304a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33052;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x4, Size::Byte, false);
    20
}
/// $033052: bne.w $33060
pub fn i_033052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33056;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33060; return 10; }
    8
}
/// $033056: jsr $5cdb4.l
pub fn i_033056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3305C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3305C);
    cpu.pc = 0x5CDB4;
    16
}
/// $03305C: bra.w $33066
pub fn i_03305c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33060;
    cpu.ir = 0x6000;
    cpu.pc = 0x33066;
    10
}
/// $033060: jsr $5ceec.l
pub fn i_033060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33066;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33066);
    cpu.pc = 0x5CEEC;
    16
}
/// $033066: bcs.w $33072
pub fn i_033066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3306A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x33072; return 10; }
    8
}
/// $03306A: moveq #$0, d0
pub fn i_03306a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3306C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== ClearXN_03306c  ($03306C, 6 B, thunks/ccr_helpers.c)
/// $03306C: andi.b #$ee, ccr
pub fn i_03306c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33070;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $033070: rts
pub fn i_033070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33072;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_FireByMode_Hit_033072  ($033072, 8 B, player/core_032aa8.s)
/// $033072: jsr $5d5b6.l
pub fn i_033072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33078);
    cpu.pc = 0x5D5B6;
    16
}
/// $033078: addq.w #$1, d0
pub fn i_033078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3307A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_03307a  ($03307A, 6 B, thunks/ccr_helpers.c)
/// $03307A: ori.b #$11, ccr
pub fn i_03307a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3307E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $03307E: rts
pub fn i_03307e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33080;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_0330ca  ($0330CA, 6 B, thunks/jmp_abs_thunks.c)
/// $0330CA: jmp $5cef8.l
pub fn i_0330ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330D0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CEF8;
    8
}
// ===== Player_ActionSelect_0330d0  ($0330D0, 150 B, player/core_032aa8.s)
/// $0330D0: btst.b #$2, $8c(a6)
pub fn i_0330d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330D6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0330D6: beq.w $330de
pub fn i_0330d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x330DE; return 10; }
    8
}
/// $0330DA: jmp $33162(pc)
pub fn i_0330da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330DE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x33162;
    8
}
/// $0330DE: jsr $5cef8.l
pub fn i_0330de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x330E4);
    cpu.pc = 0x5CEF8;
    16
}
/// $0330E4: bcs.w $331ce
pub fn i_0330e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330E8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x331CE; return 10; }
    8
}
/// $0330E8: jsr $32b1c(pc)
pub fn i_0330e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x330EC);
    cpu.pc = 0x32B1C;
    16
}
/// $0330EC: bcs.w $33148
pub fn i_0330ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330F0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x33148; return 10; }
    8
}
/// $0330F0: jsr $5cda8.l
pub fn i_0330f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x330F6);
    cpu.pc = 0x5CDA8;
    16
}
/// $0330F6: bcc.w $33148
pub fn i_0330f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33148; return 10; }
    8
}
/// $0330FA: lea.l $325e4(pc), a0
pub fn i_0330fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330FE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x325E4;
    16
}
/// $0330FE: move.l a0, $4c(a6)
pub fn i_0330fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33102;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033102: jsr $283ca.l
pub fn i_033102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33108;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33108);
    cpu.pc = 0x283CA;
    16
}
/// $033108: jsr $283ca.l
pub fn i_033108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3310E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3310E);
    cpu.pc = 0x283CA;
    16
}
/// $03310E: jsr $283d8.l
pub fn i_03310e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33114;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33114);
    cpu.pc = 0x283D8;
    16
}
/// $033114: btst.b #$1, $13(a6)
pub fn i_033114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3311A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03311A: beq.w $33126
pub fn i_03311a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3311E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33126; return 10; }
    8
}
/// $03311E: move.b #$ff, d1
pub fn i_03311e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33122;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $033122: bra.w $3316c
pub fn i_033122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33126;
    cpu.ir = 0x6000;
    cpu.pc = 0x3316C;
    10
}
/// $033126: cmpi.w #$0, $82(a6)
pub fn i_033126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3312C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03312C: bne.b $33138
pub fn i_03312c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3312E;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33138; return 10; }
    8
}
/// $03312E: cmpi.b #$0, $71(a6)
pub fn i_03312e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33134;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $033134: bne.w $33144
pub fn i_033134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33138;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33144; return 10; }
    8
}
/// $033138: jsr $33172(pc)
pub fn i_033138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3313C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3313C);
    cpu.pc = 0x33172;
    16
}
/// $03313C: bra.w $3316c
pub fn i_03313c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33140;
    cpu.ir = 0x6000;
    cpu.pc = 0x3316C;
    10
}
/// $033144: bra.w $33162
pub fn i_033144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33148;
    cpu.ir = 0x6000;
    cpu.pc = 0x33162;
    10
}
/// $033148: jsr $32aea(pc)
pub fn i_033148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3314C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3314C);
    cpu.pc = 0x32AEA;
    16
}
/// $03314C: bcs.w $33162
pub fn i_03314c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33150;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x33162; return 10; }
    8
}
/// $033150: jsr $5cdc0.l
pub fn i_033150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33156;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33156);
    cpu.pc = 0x5CDC0;
    16
}
/// $033156: bcc.w $33162
pub fn i_033156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3315A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33162; return 10; }
    8
}
/// $03315A: move.b #$3, d1
pub fn i_03315a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3315E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03315E: jmp $3316c(pc)
pub fn i_03315e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33162;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3316C;
    8
}
/// $033162: move.b #$0, d1
pub fn i_033162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33166;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== ClearXN_033166  ($033166, 6 B, thunks/ccr_helpers.c)
/// $033166: andi.b #$ee, ccr
pub fn i_033166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3316A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $03316A: rts
pub fn i_03316a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3316C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_03316c  ($03316C, 6 B, thunks/ccr_helpers.c)
/// $03316C: ori.b #$11, ccr
pub fn i_03316c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33170;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $033170: rts
pub fn i_033170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33172;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_DirNibbleToAction_033172  ($033172, 60 B, player/core_032aa8.s)
/// $033172: cmpi.b #$4, $78(a6)
pub fn i_033172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33178;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $033178: bne.w $33184
pub fn i_033178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3317C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33184; return 10; }
    8
}
/// $03317C: move.b #$1, d1
pub fn i_03317c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33180;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $033180: bra.w $331ac
pub fn i_033180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33184;
    cpu.ir = 0x6000;
    cpu.pc = 0x331AC;
    10
}
/// $033184: cmpi.b #$8, $78(a6)
pub fn i_033184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3318A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Byte, false);
    16
}
/// $03318A: bne.w $33196
pub fn i_03318a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3318E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33196; return 10; }
    8
}
/// $03318E: move.b #$4, d1
pub fn i_03318e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33192;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $033192: bra.w $331ac
pub fn i_033192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33196;
    cpu.ir = 0x6000;
    cpu.pc = 0x331AC;
    10
}
/// $033196: cmpi.b #$2, $78(a6)
pub fn i_033196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3319C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $03319C: bne.w $331a8
pub fn i_03319c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x331A8; return 10; }
    8
}
/// $0331A0: move.b #$2, d1
pub fn i_0331a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331A4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0331A4: bra.w $331ac
pub fn i_0331a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x331AC;
    10
}
/// $0331A8: move.b #$0, d1
pub fn i_0331a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331AC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0331AC: rts
pub fn i_0331ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_ActionSelect_B_Fire_0331ce  ($0331CE, 124 B, player/core_032aa8.s)
/// $0331CE: jsr $32aea(pc)
pub fn i_0331ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x331D2);
    cpu.pc = 0x32AEA;
    16
}
/// $0331D2: bcs.w $331e8
pub fn i_0331d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331D6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x331E8; return 10; }
    8
}
/// $0331D6: jsr $5cdc0.l
pub fn i_0331d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x331DC);
    cpu.pc = 0x5CDC0;
    16
}
/// $0331DC: bcc.w $331e8
pub fn i_0331dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x331E8; return 10; }
    8
}
/// $0331E0: move.b #$3, d1
pub fn i_0331e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331E4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0331E4: bra.w $33250
pub fn i_0331e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x33250;
    10
}
/// $0331E8: jsr $32b1c(pc)
pub fn i_0331e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x331EC);
    cpu.pc = 0x32B1C;
    16
}
/// $0331EC: bcs.w $33248
pub fn i_0331ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331F0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x33248; return 10; }
    8
}
/// $0331F0: jsr $5cda8.l
pub fn i_0331f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x331F6);
    cpu.pc = 0x5CDA8;
    16
}
/// $0331F6: bcc.w $33248
pub fn i_0331f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33248; return 10; }
    8
}
/// $0331FA: lea.l $325e4(pc), a0
pub fn i_0331fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331FE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x325E4;
    16
}
/// $0331FE: move.l a0, $4c(a6)
pub fn i_0331fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33202;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033202: jsr $283ca.l
pub fn i_033202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33208;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33208);
    cpu.pc = 0x283CA;
    16
}
/// $033208: jsr $283ca.l
pub fn i_033208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3320E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3320E);
    cpu.pc = 0x283CA;
    16
}
/// $03320E: jsr $283d8.l
pub fn i_03320e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33214;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33214);
    cpu.pc = 0x283D8;
    16
}
/// $033214: btst.b #$1, $13(a6)
pub fn i_033214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3321A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03321A: beq.w $33226
pub fn i_03321a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3321E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33226; return 10; }
    8
}
/// $03321E: move.b #$ff, d1
pub fn i_03321e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33222;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $033222: bra.w $33250
pub fn i_033222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33226;
    cpu.ir = 0x6000;
    cpu.pc = 0x33250;
    10
}
/// $033226: cmpi.w #$0, $82(a6)
pub fn i_033226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3322C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03322C: bne.b $33238
pub fn i_03322c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3322E;
    cpu.ir = 0x660A;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33238; return 10; }
    8
}
/// $03322E: cmpi.b #$0, $71(a6)
pub fn i_03322e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33234;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $033234: bne.w $33244
pub fn i_033234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33238;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33244; return 10; }
    8
}
/// $033238: move.b #$4, d1
pub fn i_033238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3323C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03323C: bra.w $33250
pub fn i_03323c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33240;
    cpu.ir = 0x6000;
    cpu.pc = 0x33250;
    10
}
/// $033244: bra.w $33248
pub fn i_033244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33248;
    cpu.ir = 0x6000;
    cpu.pc = 0x33248;
    10
}
/// $033248: clr.b d1
pub fn i_033248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3324A;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
// ===== ClearXN_03324a  ($03324A, 6 B, thunks/ccr_helpers.c)
/// $03324A: andi.b #$ee, ccr
pub fn i_03324a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3324E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $03324E: rts
pub fn i_03324e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33250;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_033250  ($033250, 6 B, thunks/ccr_helpers.c)
/// $033250: ori.b #$11, ccr
pub fn i_033250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33254;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $033254: rts
pub fn i_033254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33256;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_ThrowGrenadeBack_033256  ($033256, 100 B, player/core_032aa8.s)
/// $033256: movem.l a0, -(a7)
pub fn i_033256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3325A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $03325A: cmpa.l $100440.l, a6
pub fn i_03325a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33260;
    cpu.ir = 0xBDF9;
    let v2 = bus.read32(0x100440);
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    22
}
/// $033260: beq.b $3326c
pub fn i_033260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33262;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3326C; return 10; }
    8
}
/// $033262: cmpa.l $1004e0.l, a6
pub fn i_033262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33268;
    cpu.ir = 0xBDF9;
    let v2 = bus.read32(0x1004E0);
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    22
}
/// $033268: bne.w $33272
pub fn i_033268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3326C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33272; return 10; }
    8
}
/// $03326C: movea.l a6, a0
pub fn i_03326c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3326E;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $03326E: bra.w $33276
pub fn i_03326e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33272;
    cpu.ir = 0x6000;
    cpu.pc = 0x33276;
    10
}
/// $033272: movea.l $c(a6), a0
pub fn i_033272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33276;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $033276: cmpi.b #$0, $80(a0)
pub fn i_033276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3327C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03327C: beq.w $332b4
pub fn i_03327c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33280;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x332B4; return 10; }
    8
}
/// $033280: subi.b #$1, $80(a0)
pub fn i_033280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33286;
    cpu.ir = 0x428;
    let ad1 = cpu.a[0].wrapping_add(0x80);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $033286: lea.l $33346(pc), a1
pub fn i_033286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3328A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33346;
    16
}
/// $03328A: jsr $5eab6.l
pub fn i_03328a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33290;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33290);
    cpu.pc = 0x5EAB6;
    16
}
/// $033290: jsr $5dd02.l
pub fn i_033290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33296;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33296);
    cpu.pc = 0x5DD02;
    16
}
/// $033296: move.b $3a(a6), d0
pub fn i_033296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3329A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03329A: eori.b #$1, d0
pub fn i_03329a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3329E;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $03329E: move.b d0, $3a(a0)
pub fn i_03329e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332A2;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0332A2: jsr $517fe.l
pub fn i_0332a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x332A8);
    cpu.pc = 0x517FE;
    16
}
/// $0332A8: addi.w #$10, $22(a0)
pub fn i_0332a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332AE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0332AE: bset.b #$2, $8c(a6)
pub fn i_0332ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332B4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0332B4: movem.l (a7)+, a0
pub fn i_0332b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332B8;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0332B8: rts
pub fn i_0332b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpAbsThunk_033346  ($033346, 6 B, thunks/jmp_abs_thunks.c)
/// $033346: jmp $28d876.l
pub fn i_033346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3334C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x28D876;
    8
}
// ===== Probe_Bit3At100001_0334A2  ($0334A2, 36 B, player/route_publish_027eba.s)
/// $0334A2: btst.b #$3, $100001.l
pub fn i_0334a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334AA;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((3) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0334AA: beq.w $334c6
pub fn i_0334aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x334C6; return 10; }
    8
}
/// $0334AE: bclr.b #$3, $13(a6)
pub fn i_0334ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334B4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0334B4: bclr.b #$0, $13(a6)
pub fn i_0334b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334BA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0334BA: move.w #$1, $66(a6)
pub fn i_0334ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0334C0: andi.b #$ee, ccr
pub fn i_0334c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334C4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0334C4: rts
pub fn i_0334c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_DeathGate_0334c6  ($0334C6, 92 B, player/core_032aa8.s)
/// $0334C6: tst.w $106e92.l
pub fn i_0334c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334CC;
    cpu.ir = 0x4A79;
    let v2 = bus.read16(0x106E92) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    16
}
/// $0334CC: bne.w $334da
pub fn i_0334cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x334DA; return 10; }
    8
}
/// $0334D0: bset.b #$0, $13(a6)
pub fn i_0334d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334D6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0334D6: clr.w $66(a6)
pub fn i_0334d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334DA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0334DA: cmpi.b #$1, $106ece.l
pub fn i_0334da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334E2;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0334E2: bne.w $3351c
pub fn i_0334e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3351C; return 10; }
    8
}
/// $0334E6: move.w $22(a6), d1
pub fn i_0334e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334EA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0334EA: move.w $24(a6), d2
pub fn i_0334ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334EE;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0334EE: cmpi.w #$10c, d2
pub fn i_0334ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334F2;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x10C, Size::Word, false);
    8
}
/// $0334F2: bge.w $3351c
pub fn i_0334f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334F6;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x3351C; return 10; }
    8
}
/// $0334F6: jsr $27db2.l
pub fn i_0334f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x334FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x334FC);
    cpu.pc = 0x27DB2;
    16
}
/// $0334FC: cmpi.b #$40, d7
pub fn i_0334fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33500;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $033500: bne.w $3351c
pub fn i_033500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33504;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3351C; return 10; }
    8
}
/// $033504: bset.b #$3, $13(a6)
pub fn i_033504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3350A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03350A: bset.b #$0, $13(a6)
pub fn i_03350a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33510;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033510: clr.w $66(a6)
pub fn i_033510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33514;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033514: move.b #$5, d0
pub fn i_033514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33518;
    cpu.ir = 0x103C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $033518: move.b d0, $58(a6)
pub fn i_033518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3351C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03351C: jmp $28758.l
pub fn i_03351c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33522;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x28758;
    8
}
// ===== PlayerRoute_PublishState_033522  ($033522, 80 B, player/route_publish_027eba.s)
/// $033522: jsr $334a2(pc)
pub fn i_033522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33526;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33526);
    cpu.pc = 0x334A2;
    16
}
/// $033526: bcc.w $33570
pub fn i_033526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3352A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33570; return 10; }
    8
}
/// $03352A: jsr $27eba.l
pub fn i_03352a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33530;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33530);
    cpu.pc = 0x27EBA;
    16
}
/// $033530: bcs.w $3353e
pub fn i_033530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33534;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3353E; return 10; }
    8
}
/// $033534: lea.l $33572(pc), a1
pub fn i_033534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33538;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33572;
    16
}
/// $033538: move.l a1, (a6)
pub fn i_033538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3353A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03353A: bra.w $33544
pub fn i_03353a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3353E;
    cpu.ir = 0x6000;
    cpu.pc = 0x33544;
    10
}
/// $03353E: lea.l $33578(pc), a1
pub fn i_03353e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33542;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33578;
    16
}
/// $033542: move.l a1, (a6)
pub fn i_033542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33544;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033544: cmpi.b #$5, $58(a6)
pub fn i_033544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3354A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $03354A: beq.w $33570
pub fn i_03354a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3354E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33570; return 10; }
    8
}
/// $03354E: cmpa.l #$100440, a6
pub fn i_03354e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33554;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $033554: bne.w $33566
pub fn i_033554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33558;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33566; return 10; }
    8
}
/// $033558: move.w #$1123, d0
pub fn i_033558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3355C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1123;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03355C: jsr $2352.l
pub fn i_03355c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33562;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33562);
    cpu.pc = 0x2352;
    16
}
/// $033562: bra.w $33570
pub fn i_033562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33566;
    cpu.ir = 0x6000;
    cpu.pc = 0x33570;
    10
}
/// $033566: move.w #$1087, d0
pub fn i_033566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3356A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1087;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03356A: jsr $2352.l
pub fn i_03356a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33570;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33570);
    cpu.pc = 0x2352;
    16
}
/// $033570: rts
pub fn i_033570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33572;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerStateDispatch_033572  ($033572, 52 B, player/dispatch_033572.s)
/// $033572: moveq #$0, d7
pub fn i_033572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33574;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $033574: bra.w $3357e
pub fn i_033574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33578;
    cpu.ir = 0x6000;
    cpu.pc = 0x3357E;
    10
}
/// $033578: moveq #$1, d7
pub fn i_033578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3357A;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03357A: bra.w $3357e
pub fn i_03357a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3357E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3357E;
    10
}
/// $03357E: andi.w #$ff, d7
pub fn i_03357e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33582;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $033582: lsl.w #$2, d7
pub fn i_033582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33584;
    cpu.ir = 0xE54F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $033584: lea.l $3349a(pc), a0
pub fn i_033584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33588;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3349A;
    16
}
/// $033588: movea.l (a0, d7.w), a0
pub fn i_033588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3358C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03358C: moveq #$0, d0
pub fn i_03358c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3358E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03358E: move.b $58(a6), d0
pub fn i_03358e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33592;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $033592: cmpi.w #$22, d0
pub fn i_033592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33596;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x22, Size::Word, false);
    8
}
/// $033596: bcs.w $3359e
pub fn i_033596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3359A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3359E; return 10; }
    8
}
/// $03359A: move.w #$f, d0
pub fn i_03359a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3359E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03359E: lsl.w #$2, d0
pub fn i_03359e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335A0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0335A0: movea.l (a0, d0.w), a0
pub fn i_0335a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335A4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0335A4: jmp (a0)
pub fn i_0335a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335A6;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
// ===== PlayerEntitySpawn_0335A6  ($0335A6, 138 B, player/dispatch_033572.s)
/// $0335A6: move.w #$60, $22(a6)
pub fn i_0335a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335AC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0335AC: move.w #$171, $24(a6)
pub fn i_0335ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335B2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x171;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0335B2: move.w #$176, d1
pub fn i_0335b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x176;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0335B6: jsr $236e.l
pub fn i_0335b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x335BC);
    cpu.pc = 0x236E;
    16
}
/// $0335BC: move.w #$190, d1
pub fn i_0335bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0335C0: jsr $236e.l
pub fn i_0335c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x335C6);
    cpu.pc = 0x236E;
    16
}
/// $0335C6: move.w #$191, d1
pub fn i_0335c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335CA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x191;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0335CA: jsr $236e.l
pub fn i_0335ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x335D0);
    cpu.pc = 0x236E;
    16
}
/// $0335D0: move.w #$1b, $1c(a6)
pub fn i_0335d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0335D6: jsr $138fe.l
pub fn i_0335d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x335DC);
    cpu.pc = 0x138FE;
    16
}
/// $0335DC: jsr $32a02(pc)
pub fn i_0335dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x335E0);
    cpu.pc = 0x32A02;
    16
}
/// $0335E0: bclr.b #$4, $12(a6)
pub fn i_0335e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335E6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0335E6: ori.w #$2, $38(a6)
pub fn i_0335e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335EC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0335EC: lea.l $394a8.l, a1
pub fn i_0335ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335F2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x394A8;
    20
}
/// $0335F2: jsr $4ae.l
pub fn i_0335f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x335F8);
    cpu.pc = 0x4AE;
    16
}
/// $0335F8: jsr $5dd02.l
pub fn i_0335f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x335FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x335FE);
    cpu.pc = 0x5DD02;
    16
}
/// $0335FE: jsr $517fe.l
pub fn i_0335fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33604;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33604);
    cpu.pc = 0x517FE;
    16
}
/// $033604: clr.w $28(a6)
pub fn i_033604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33608;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033608: clr.w $2a(a6)
pub fn i_033608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3360C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03360C: clr.w $2c(a6)
pub fn i_03360c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33610;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033610: clr.w $2e(a6)
pub fn i_033610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33614;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033614: clr.b $26(a6)
pub fn i_033614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33618;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $033618: clr.b $27(a6)
pub fn i_033618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3361C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03361C: bset.b #$6, $13(a6)
pub fn i_03361c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33622;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033622: jsr $32ff2(pc)
pub fn i_033622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33626;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33626);
    cpu.pc = 0x32FF2;
    16
}
/// $033626: jsr $27bc8.l
pub fn i_033626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3362C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3362C);
    cpu.pc = 0x27BC8;
    16
}
/// $03362C: jsr $32aa8(pc)
pub fn i_03362c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33630;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33630);
    cpu.pc = 0x32AA8;
    16
}
// ===== SetTaskHandler_033630  ($033630, 8 B, kernel/task_handlers.c)
/// $033630: lea.l $36d64(pc), a1
pub fn i_033630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33634;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36D64;
    16
}
/// $033634: move.l a1, (a6)
pub fn i_033634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33636;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033636: rts
pub fn i_033636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33638;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerEntitySpawn_Alt_03364A  ($03364A, 106 B, player/helpers_033638.s)
/// $03364A: move.w #$78, $22(a6)
pub fn i_03364a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33650;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033650: move.w #$1d8, $24(a6)
pub fn i_033650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33656;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1D8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033656: jsr $32ac8(pc)
pub fn i_033656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3365A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3365A);
    cpu.pc = 0x32AC8;
    16
}
/// $03365A: movem.w d2, -(a7)
pub fn i_03365a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3365E;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    cpu.a[7] = a;
    20
}
/// $03365E: move.w #$176, d1
pub fn i_03365e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33662;
    cpu.ir = 0x323C;
    let v1: u32 = 0x176;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $033662: jsr $236e.l
pub fn i_033662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33668;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33668);
    cpu.pc = 0x236E;
    16
}
/// $033668: move.w #$190, d1
pub fn i_033668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3366C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03366C: jsr $236e.l
pub fn i_03366c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33672;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33672);
    cpu.pc = 0x236E;
    16
}
/// $033672: move.w #$191, d1
pub fn i_033672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33676;
    cpu.ir = 0x323C;
    let v1: u32 = 0x191;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $033676: jsr $236e.l
pub fn i_033676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3367C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3367C);
    cpu.pc = 0x236E;
    16
}
/// $03367C: move.w #$1b, $1c(a6)
pub fn i_03367c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33682;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033682: jsr $138fe.l
pub fn i_033682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33688);
    cpu.pc = 0x138FE;
    16
}
/// $033688: jsr $32a02(pc)
pub fn i_033688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3368C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3368C);
    cpu.pc = 0x32A02;
    16
}
/// $03368C: bclr.b #$4, $12(a6)
pub fn i_03368c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33692;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033692: ori.w #$2, $38(a6)
pub fn i_033692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33698;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $033698: lea.l $394a8.l, a1
pub fn i_033698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3369E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x394A8;
    20
}
/// $03369E: jsr $4ae.l
pub fn i_03369e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x336A4);
    cpu.pc = 0x4AE;
    16
}
/// $0336A4: jsr $5dd02.l
pub fn i_0336a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x336AA);
    cpu.pc = 0x5DD02;
    16
}
/// $0336AA: jsr $517fe.l
pub fn i_0336aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x336B0);
    cpu.pc = 0x517FE;
    16
}
/// $0336B0: bra.w $33742
pub fn i_0336b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x33742;
    10
}
// ===== Player_SpawnByMode_033742  ($033742, 102 B, player/core_032aa8.s)
/// $033742: clr.w $28(a6)
pub fn i_033742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33746;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033746: clr.w $2a(a6)
pub fn i_033746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3374A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03374A: clr.w $2c(a6)
pub fn i_03374a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3374E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03374E: clr.w $2e(a6)
pub fn i_03374e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33752;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033752: clr.b $26(a6)
pub fn i_033752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33756;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $033756: clr.b $27(a6)
pub fn i_033756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3375A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03375A: move.w $38(a6), d1
pub fn i_03375a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3375E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03375E: move.w #$9fe0, d0
pub fn i_03375e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33762;
    cpu.ir = 0x303C;
    let v1: u32 = 0x9FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $033762: andi.w #$ffe3, d1
pub fn i_033762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33766;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $033766: ori.w #$10, d0
pub fn i_033766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3376A;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03376A: or.w d0, d1
pub fn i_03376a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3376C;
    cpu.ir = 0x8240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $03376C: move.w d1, $38(a6)
pub fn i_03376c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33770;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033770: movem.w (a7)+, d2
pub fn i_033770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33774;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $033774: cmpi.w #$2, d2
pub fn i_033774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33778;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $033778: bne.w $33780
pub fn i_033778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3377C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33780; return 10; }
    8
}
/// $03377C: jmp $337a8(pc)
pub fn i_03377c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33780;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x337A8;
    8
}
/// $033780: cmpi.w #$1, d2
pub fn i_033780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33784;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $033784: bne.w $33798
pub fn i_033784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33788;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33798; return 10; }
    8
}
/// $033788: move.b #$7f, $59(a6)
pub fn i_033788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3378E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03378E: move.b #$7f, $45(a6)
pub fn i_03378e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33794;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033794: jmp $33952(pc)
pub fn i_033794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33798;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x33952;
    8
}
/// $033798: move.b #$7f, $59(a6)
pub fn i_033798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3379E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03379E: move.b #$7f, $45(a6)
pub fn i_03379e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337A4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0337A4: jmp $33956(pc)
pub fn i_0337a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337A8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x33956;
    8
}
// ===== Player_SpawnFall_0337a8  ($0337A8, 192 B, player/core_032aa8.s)
/// $0337A8: addq.w #$3, $24(a6)
pub fn i_0337a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337AC;
    cpu.ir = 0x566E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0337AC: move.b #$4, $70(a6)
pub fn i_0337ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337B2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0337B2: lea.l $279f8a.l, a0
pub fn i_0337b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F8A;
    20
}
/// $0337B8: move.l -$4(a0), $74(a6)
pub fn i_0337b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337BE;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0337BE: move.b #$ff, $21(a6)
pub fn i_0337be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0337C4: lea.l $279f8a.l, a0
pub fn i_0337c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F8A;
    20
}
/// $0337CA: jsr $28cd4.l
pub fn i_0337ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x337D0);
    cpu.pc = 0x28CD4;
    16
}
/// $0337D0: move.w #$fc00, $2a(a6)
pub fn i_0337d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0337D6: lea.l $ffff.w, a0
pub fn i_0337d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337DA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0337DA: move.l a0, $48(a6)
pub fn i_0337da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337DE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0337DE: bset.b #$7, $5b(a6)
pub fn i_0337de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337E4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0337E4: lea.l $337ea(pc), a1
pub fn i_0337e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x337EA;
    16
}
/// $0337E8: move.l a1, (a6)
pub fn i_0337e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0337EA: bset.b #$6, $13(a6)
pub fn i_0337ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337F0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0337F0: bset.b #$1, $8d(a6)
pub fn i_0337f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337F6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0337F6: jsr $32ff2(pc)
pub fn i_0337f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x337FA);
    cpu.pc = 0x32FF2;
    16
}
/// $0337FA: jsr $32d32(pc)
pub fn i_0337fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x337FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x337FE);
    cpu.pc = 0x32D32;
    16
}
/// $0337FE: lea.l $3386e(pc), a1
pub fn i_0337fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33802;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3386E;
    16
}
/// $033802: move.l a1, (a6)
pub fn i_033802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33804;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033804: bclr.b #$1, $8d(a6)
pub fn i_033804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3380A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03380A: jsr $27bc8.l
pub fn i_03380a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33810;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33810);
    cpu.pc = 0x27BC8;
    16
}
/// $033810: bcs.w $3382a
pub fn i_033810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33814;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3382A; return 10; }
    8
}
/// $033814: jsr $27bc8.l
pub fn i_033814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3381A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3381A);
    cpu.pc = 0x27BC8;
    16
}
/// $03381A: bcs.w $3382a
pub fn i_03381a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3381E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3382A; return 10; }
    8
}
/// $03381E: lea.l $338fa(pc), a1
pub fn i_03381e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33822;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x338FA;
    16
}
/// $033822: move.l a1, (a6)
pub fn i_033822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33824;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033824: bset.b #$1, $8d(a6)
pub fn i_033824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3382A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03382A: jsr $32aa8(pc)
pub fn i_03382a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3382E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3382E);
    cpu.pc = 0x32AA8;
    16
}
/// $03382E: cmpi.b #$3, $106ece.l
pub fn i_03382e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33836;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $033836: beq.w $3384e
pub fn i_033836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3383A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3384E; return 10; }
    8
}
/// $03383A: movea.l #$ffffffff, a0
pub fn i_03383a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33840;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $033840: lea.l $324c6(pc), a0
pub fn i_033840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33844;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $033844: jsr $5dd56.l
pub fn i_033844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3384A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3384A);
    cpu.pc = 0x5DD56;
    16
}
/// $03384A: bra.w $3385e
pub fn i_03384a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3384E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3385E;
    10
}
/// $03384E: movea.l #$ffffffff, a0
pub fn i_03384e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33854;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $033854: lea.l $324bc(pc), a0
pub fn i_033854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33858;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $033858: jsr $5dd56.l
pub fn i_033858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3385E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3385E);
    cpu.pc = 0x5DD56;
    16
}
/// $03385E: bcc.w $33868
pub fn i_03385e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33862;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33868; return 10; }
    8
}
/// $033862: lea.l $37b8e(pc), a1
pub fn i_033862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33866;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $033866: move.l a1, (a6)
pub fn i_033866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33868;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_033868  ($033868, 6 B, thunks/jsr_pc_thunks.c)
/// $033868: jsr $334a2(pc)
pub fn i_033868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3386C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3386C);
    cpu.pc = 0x334A2;
    16
}
/// $03386C: rts
pub fn i_03386c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3386E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_SpawnLandB_0338fa  ($0338FA, 82 B, player/core_032aa8.s)
/// $0338FA: move.w #$0, $7c(a6)
pub fn i_0338fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33900;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033900: move.w #$0, $7e(a6)
pub fn i_033900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33906;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033906: bset.b #$7, $5b(a6)
pub fn i_033906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3390C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03390C: move.l #$32500, $60(a6)
pub fn i_03390c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33914;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033914: clr.w $28(a6)
pub fn i_033914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33918;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033918: clr.w $2a(a6)
pub fn i_033918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3391C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03391C: clr.w $2c(a6)
pub fn i_03391c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33920;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033920: clr.w $2e(a6)
pub fn i_033920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33924;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033924: lea.l $3392a(pc), a1
pub fn i_033924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33928;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3392A;
    16
}
/// $033928: move.l a1, (a6)
pub fn i_033928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3392A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03392A: bset.b #$1, $8d(a6)
pub fn i_03392a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33930;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033930: jsr $32ff2(pc)
pub fn i_033930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33934;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33934);
    cpu.pc = 0x32FF2;
    16
}
/// $033934: jsr $32d32(pc)
pub fn i_033934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33938;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33938);
    cpu.pc = 0x32D32;
    16
}
/// $033938: jsr $27bc8.l
pub fn i_033938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3393E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3393E);
    cpu.pc = 0x27BC8;
    16
}
/// $03393E: jsr $32aa8(pc)
pub fn i_03393e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33942;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33942);
    cpu.pc = 0x32AA8;
    16
}
/// $033942: bcc.w $3394c
pub fn i_033942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33946;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3394C; return 10; }
    8
}
/// $033946: lea.l $36c8c(pc), a1
pub fn i_033946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3394A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36C8C;
    16
}
/// $03394A: move.l a1, (a6)
pub fn i_03394a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3394C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_03394c  ($03394C, 6 B, thunks/jsr_pc_thunks.c)
/// $03394C: jsr $33522(pc)
pub fn i_03394c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33950;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33950);
    cpu.pc = 0x33522;
    16
}
/// $033950: rts
pub fn i_033950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33952;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_JmpState36C8C_033952  ($033952, 4 B, player/core_032aa8.s)
/// $033952: jmp $36c8c(pc)
pub fn i_033952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33956;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x36C8C;
    8
}
// ===== Player_SpawnParachute_033956  ($033956, 258 B, player/core_032aa8.s)
/// $033956: lea.l $38cf6(pc), a1
pub fn i_033956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3395A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38CF6;
    16
}
/// $03395A: jsr $4ae.l
pub fn i_03395a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33960;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33960);
    cpu.pc = 0x4AE;
    16
}
/// $033960: jsr $5dd02.l
pub fn i_033960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33966;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33966);
    cpu.pc = 0x5DD02;
    16
}
/// $033966: move.w #$0, $7c(a6)
pub fn i_033966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3396C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03396C: move.w #$0, $7e(a6)
pub fn i_03396c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33972;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033972: move.b #$4, $70(a6)
pub fn i_033972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33978;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033978: lea.l $279b2c.l, a0
pub fn i_033978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3397E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B2C;
    20
}
/// $03397E: move.l -$4(a0), $74(a6)
pub fn i_03397e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33984;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $033984: move.b #$ff, $21(a6)
pub fn i_033984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3398A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03398A: lea.l $279b2c.l, a0
pub fn i_03398a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33990;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279B2C;
    20
}
/// $033990: jsr $28cd4.l
pub fn i_033990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33996);
    cpu.pc = 0x28CD4;
    16
}
/// $033996: move.w #$ff00, $2a(a6)
pub fn i_033996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3399C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03399C: lea.l $ffff.w, a0
pub fn i_03399c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339A0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0339A0: move.l a0, $48(a6)
pub fn i_0339a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339A4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0339A4: lea.l $339aa(pc), a1
pub fn i_0339a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x339AA;
    16
}
/// $0339A8: move.l a1, (a6)
pub fn i_0339a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0339AA: bset.b #$6, $13(a6)
pub fn i_0339aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339B0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0339B0: bset.b #$1, $8d(a6)
pub fn i_0339b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339B6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0339B6: jsr $32ff2(pc)
pub fn i_0339b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x339BA);
    cpu.pc = 0x32FF2;
    16
}
/// $0339BA: jsr $32d32(pc)
pub fn i_0339ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x339BE);
    cpu.pc = 0x32D32;
    16
}
/// $0339BE: move.w $2a(a6), d0
pub fn i_0339be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339C2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0339C2: move.w #$400, d1
pub fn i_0339c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339C6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0339C6: jsr $267f4.l
pub fn i_0339c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x339CC);
    cpu.pc = 0x267F4;
    16
}
/// $0339CC: move.w d0, $2a(a6)
pub fn i_0339cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339D0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0339D0: jsr $27bc8.l
pub fn i_0339d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x339D6);
    cpu.pc = 0x27BC8;
    16
}
/// $0339D6: bcc.w $339e6
pub fn i_0339d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x339E6; return 10; }
    8
}
/// $0339DA: lea.l $33e8c(pc), a1
pub fn i_0339da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33E8C;
    16
}
/// $0339DE: move.l a1, (a6)
pub fn i_0339de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0339E0: bclr.b #$1, $8d(a6)
pub fn i_0339e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339E6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0339E6: move.w $38(a6), d1
pub fn i_0339e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339EA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0339EA: move.w #$9fe0, d0
pub fn i_0339ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x9FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0339EE: andi.w #$ffe3, d1
pub fn i_0339ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339F2;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0339F2: ori.w #$10, d0
pub fn i_0339f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339F6;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0339F6: or.w d0, d1
pub fn i_0339f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339F8;
    cpu.ir = 0x8240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0339F8: move.w d1, $38(a6)
pub fn i_0339f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x339FC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0339FC: jsr $32aa8(pc)
pub fn i_0339fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33A00);
    cpu.pc = 0x32AA8;
    16
}
/// $033A00: jsr $33034(pc)
pub fn i_033a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33A04);
    cpu.pc = 0x33034;
    16
}
/// $033A04: bcc.w $33a0e
pub fn i_033a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A08;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33A0E; return 10; }
    8
}
/// $033A08: lea.l $36c8c(pc), a1
pub fn i_033a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36C8C;
    16
}
/// $033A0C: move.l a1, (a6)
pub fn i_033a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033A0E: cmpi.w #$170, $24(a6)
pub fn i_033a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A14;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x170, Size::Word, false);
    16
}
/// $033A14: bgt.w $33a1e
pub fn i_033a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A18;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x33A1E; return 10; }
    8
}
/// $033A18: lea.l $36c8c(pc), a1
pub fn i_033a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36C8C;
    16
}
/// $033A1C: move.l a1, (a6)
pub fn i_033a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033A1E: cmpi.b #$3, $106ece.l
pub fn i_033a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A26;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $033A26: beq.w $33a3e
pub fn i_033a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A2A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33A3E; return 10; }
    8
}
/// $033A2A: movea.l #$ffffffff, a0
pub fn i_033a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A30;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $033A30: lea.l $324c6(pc), a0
pub fn i_033a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A34;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $033A34: jsr $5dd56.l
pub fn i_033a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33A3A);
    cpu.pc = 0x5DD56;
    16
}
/// $033A3A: bra.w $33a4e
pub fn i_033a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x33A4E;
    10
}
/// $033A3E: movea.l #$ffffffff, a0
pub fn i_033a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A44;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $033A44: lea.l $324bc(pc), a0
pub fn i_033a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A48;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $033A48: jsr $5dd56.l
pub fn i_033a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33A4E);
    cpu.pc = 0x5DD56;
    16
}
/// $033A4E: bcc.w $33a58
pub fn i_033a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A52;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33A58; return 10; }
    8
}
/// $033A52: lea.l $37b8e(pc), a1
pub fn i_033a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $033A56: move.l a1, (a6)
pub fn i_033a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_033a58  ($033A58, 6 B, thunks/jsr_pc_thunks.c)
/// $033A58: jsr $334a2(pc)
pub fn i_033a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33A5C);
    cpu.pc = 0x334A2;
    16
}
/// $033A5C: rts
pub fn i_033a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Crouch_033a5e  ($033A5E, 158 B, player/core_032aa8.s)
/// $033A5E: move.w #$0, $7c(a6)
pub fn i_033a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A64;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033A64: move.w #$0, $7e(a6)
pub fn i_033a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A6A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033A6A: bclr.b #$2, $8c(a6)
pub fn i_033a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A70;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033A70: bclr.b #$1, $8c(a6)
pub fn i_033a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A76;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033A76: bclr.b #$3, $8c(a6)
pub fn i_033a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A7C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033A7C: move.b #$5, $70(a6)
pub fn i_033a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A82;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033A82: lea.l $27973e.l, a0
pub fn i_033a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A88;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27973E;
    20
}
/// $033A88: move.l -$4(a0), $74(a6)
pub fn i_033a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A8E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $033A8E: move.b #$ff, $21(a6)
pub fn i_033a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A94;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033A94: lea.l $27973e.l, a0
pub fn i_033a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33A9A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27973E;
    20
}
/// $033A9A: jsr $28cd4.l
pub fn i_033a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33AA0);
    cpu.pc = 0x28CD4;
    16
}
/// $033AA0: lea.l $326e0(pc), a0
pub fn i_033aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AA4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $033AA4: move.l a0, $48(a6)
pub fn i_033aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AA8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033AA8: move.l #$32500, $60(a6)
pub fn i_033aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AB0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033AB0: jsr $267e6.l
pub fn i_033ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33AB6);
    cpu.pc = 0x267E6;
    16
}
/// $033AB6: clr.w $28(a6)
pub fn i_033ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33ABA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033ABA: lea.l $33ac0(pc), a1
pub fn i_033aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33ABE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33AC0;
    16
}
/// $033ABE: move.l a1, (a6)
pub fn i_033abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033AC0: jsr $32ff2(pc)
pub fn i_033ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33AC4);
    cpu.pc = 0x32FF2;
    16
}
/// $033AC4: jsr $32d32(pc)
pub fn i_033ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33AC8);
    cpu.pc = 0x32D32;
    16
}
/// $033AC8: jsr $27a92.l
pub fn i_033ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33ACE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33ACE);
    cpu.pc = 0x27A92;
    16
}
/// $033ACE: jsr $32aa8(pc)
pub fn i_033ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33AD2);
    cpu.pc = 0x32AA8;
    16
}
/// $033AD2: bcc.w $33af8
pub fn i_033ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33AF8; return 10; }
    8
}
/// $033AD6: lea.l $34704(pc), a1
pub fn i_033ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33ADA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $033ADA: move.l a1, (a6)
pub fn i_033ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33ADC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033ADC: tst.w $28(a6)
pub fn i_033adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AE0;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $033AE0: beq.w $33af8
pub fn i_033ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AE4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33AF8; return 10; }
    8
}
/// $033AE4: ble.w $33af2
pub fn i_033ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AE8;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x33AF2; return 10; }
    8
}
/// $033AE8: lea.l $34b38(pc), a1
pub fn i_033ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34B38;
    16
}
/// $033AEC: move.l a1, (a6)
pub fn i_033aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033AEE: bra.w $33af8
pub fn i_033aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AF2;
    cpu.ir = 0x6000;
    cpu.pc = 0x33AF8;
    10
}
/// $033AF2: lea.l $34d32(pc), a1
pub fn i_033af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34D32;
    16
}
/// $033AF6: move.l a1, (a6)
pub fn i_033af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033AF8: jmp $341a4(pc)
pub fn i_033af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33AFC;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x341A4;
    8
}
// ===== Player_CrouchB_033afc  ($033AFC, 158 B, player/core_032aa8.s)
/// $033AFC: move.w #$0, $7c(a6)
pub fn i_033afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B02;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033B02: move.w #$0, $7e(a6)
pub fn i_033b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B08;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033B08: bclr.b #$2, $8c(a6)
pub fn i_033b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B0E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033B0E: bclr.b #$1, $8c(a6)
pub fn i_033b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B14;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033B14: bclr.b #$3, $8c(a6)
pub fn i_033b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B1A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033B1A: move.b #$5, $70(a6)
pub fn i_033b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B20;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033B20: lea.l $27981e.l, a0
pub fn i_033b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27981E;
    20
}
/// $033B26: move.l -$4(a0), $74(a6)
pub fn i_033b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B2C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $033B2C: move.b #$ff, $21(a6)
pub fn i_033b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B32;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033B32: lea.l $27981e.l, a0
pub fn i_033b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B38;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27981E;
    20
}
/// $033B38: jsr $28cd4.l
pub fn i_033b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33B3E);
    cpu.pc = 0x28CD4;
    16
}
/// $033B3E: lea.l $326e0(pc), a0
pub fn i_033b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B42;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $033B42: move.l a0, $48(a6)
pub fn i_033b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B46;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033B46: move.l #$32500, $60(a6)
pub fn i_033b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B4E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033B4E: jsr $267e6.l
pub fn i_033b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33B54);
    cpu.pc = 0x267E6;
    16
}
/// $033B54: clr.w $28(a6)
pub fn i_033b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B58;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033B58: lea.l $33b5e(pc), a1
pub fn i_033b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33B5E;
    16
}
/// $033B5C: move.l a1, (a6)
pub fn i_033b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033B5E: jsr $32ff2(pc)
pub fn i_033b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33B62);
    cpu.pc = 0x32FF2;
    16
}
/// $033B62: jsr $32d32(pc)
pub fn i_033b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33B66);
    cpu.pc = 0x32D32;
    16
}
/// $033B66: jsr $27a92.l
pub fn i_033b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33B6C);
    cpu.pc = 0x27A92;
    16
}
/// $033B6C: jsr $32aa8(pc)
pub fn i_033b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33B70);
    cpu.pc = 0x32AA8;
    16
}
/// $033B70: bcc.w $33b96
pub fn i_033b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B74;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33B96; return 10; }
    8
}
/// $033B74: lea.l $34704(pc), a1
pub fn i_033b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $033B78: move.l a1, (a6)
pub fn i_033b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033B7A: tst.w $28(a6)
pub fn i_033b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B7E;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $033B7E: beq.w $33b96
pub fn i_033b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33B96; return 10; }
    8
}
/// $033B82: ble.w $33b90
pub fn i_033b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B86;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x33B90; return 10; }
    8
}
/// $033B86: lea.l $34b38(pc), a1
pub fn i_033b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34B38;
    16
}
/// $033B8A: move.l a1, (a6)
pub fn i_033b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033B8C: bra.w $33b96
pub fn i_033b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B90;
    cpu.ir = 0x6000;
    cpu.pc = 0x33B96;
    10
}
/// $033B90: lea.l $34d32(pc), a1
pub fn i_033b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34D32;
    16
}
/// $033B94: move.l a1, (a6)
pub fn i_033b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033B96: bra.w $341a4
pub fn i_033b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33B9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x341A4;
    10
}
// ===== Player_Reload_033b9a  ($033B9A, 458 B, player/core_032aa8.s)
/// $033B9A: cmpi.w #$1, $72(a6)
pub fn i_033b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BA0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $033BA0: bne.w $33ba8
pub fn i_033ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33BA8; return 10; }
    8
}
/// $033BA4: jmp $33c8e(pc)
pub fn i_033ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BA8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x33C8E;
    8
}
/// $033BA8: move.w #$0, $7c(a6)
pub fn i_033ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BAE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033BAE: move.w #$0, $7e(a6)
pub fn i_033bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033BB4: bset.b #$2, $8c(a6)
pub fn i_033bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BBA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033BBA: bclr.b #$1, $8c(a6)
pub fn i_033bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BC0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033BC0: bclr.b #$3, $8c(a6)
pub fn i_033bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BC6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033BC6: move.b #$33, $70(a6)
pub fn i_033bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BCC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033BCC: lea.l $2796f8.l, a0
pub fn i_033bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BD2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2796F8;
    20
}
/// $033BD2: move.l -$4(a0), $74(a6)
pub fn i_033bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BD8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $033BD8: move.b #$ff, $21(a6)
pub fn i_033bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BDE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033BDE: lea.l $2796f8.l, a0
pub fn i_033bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2796F8;
    20
}
/// $033BE4: jsr $28cd4.l
pub fn i_033be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33BEA);
    cpu.pc = 0x28CD4;
    16
}
/// $033BEA: lea.l $326e0(pc), a0
pub fn i_033bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BEE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $033BEE: move.l a0, $48(a6)
pub fn i_033bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BF2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033BF2: move.l #$32500, $60(a6)
pub fn i_033bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33BFA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033BFA: jsr $267e6.l
pub fn i_033bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33C00);
    cpu.pc = 0x267E6;
    16
}
/// $033C00: clr.w $28(a6)
pub fn i_033c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C04;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033C04: lea.l $33c0a(pc), a1
pub fn i_033c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33C0A;
    16
}
/// $033C08: move.l a1, (a6)
pub fn i_033c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033C0A: jsr $32ff2(pc)
pub fn i_033c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33C0E);
    cpu.pc = 0x32FF2;
    16
}
/// $033C0E: jsr $32d32(pc)
pub fn i_033c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33C12);
    cpu.pc = 0x32D32;
    16
}
/// $033C12: jsr $27a92.l
pub fn i_033c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33C18);
    cpu.pc = 0x27A92;
    16
}
/// $033C18: jsr $32aa8(pc)
pub fn i_033c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33C1C);
    cpu.pc = 0x32AA8;
    16
}
/// $033C1C: bcc.w $33c6c
pub fn i_033c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33C6C; return 10; }
    8
}
/// $033C20: lea.l $34704(pc), a1
pub fn i_033c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $033C24: move.l a1, (a6)
pub fn i_033c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033C26: tst.w $28(a6)
pub fn i_033c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C2A;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $033C2A: beq.w $33c42
pub fn i_033c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C2E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33C42; return 10; }
    8
}
/// $033C2E: ble.w $33c3c
pub fn i_033c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C32;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x33C3C; return 10; }
    8
}
/// $033C32: lea.l $34b38(pc), a1
pub fn i_033c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34B38;
    16
}
/// $033C36: move.l a1, (a6)
pub fn i_033c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033C38: bra.w $33c42
pub fn i_033c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x33C42;
    10
}
/// $033C3C: lea.l $34d32(pc), a1
pub fn i_033c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34D32;
    16
}
/// $033C40: move.l a1, (a6)
pub fn i_033c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033C42: jsr $5e9b6.l
pub fn i_033c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33C48);
    cpu.pc = 0x5E9B6;
    16
}
/// $033C48: andi.w #$7, d0
pub fn i_033c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C4C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $033C4C: moveq #$4, d0
pub fn i_033c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C4E;
    cpu.ir = 0x7004;
    cpu.d[0] = 0x4;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $033C4E: cmp.w $82(a6), d0
pub fn i_033c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C52;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $033C52: bne.w $33c5c
pub fn i_033c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33C5C; return 10; }
    8
}
/// $033C56: lea.l $33afc(pc), a1
pub fn i_033c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33AFC;
    16
}
/// $033C5A: move.l a1, (a6)
pub fn i_033c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033C5C: cmpi.w #$0, $82(a6)
pub fn i_033c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C62;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $033C62: bne.w $33c6c
pub fn i_033c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C66;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33C6C; return 10; }
    8
}
/// $033C66: lea.l $33d64(pc), a1
pub fn i_033c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $033C6A: move.l a1, (a6)
pub fn i_033c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033C6C: btst.b #$2, $8c(a6)
pub fn i_033c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C72;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $033C72: bne.w $33c7c
pub fn i_033c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33C7C; return 10; }
    8
}
/// $033C76: move.b #$1, $85(a6)
pub fn i_033c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C7C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x030002, i_030002),
    (0x030006, i_030006),
    (0x03000A, i_03000a),
    (0x030010, i_030010),
    (0x030016, i_030016),
    (0x03001C, i_03001c),
    (0x030020, i_030020),
    (0x030026, i_030026),
    (0x03002C, i_03002c),
    (0x030032, i_030032),
    (0x030036, i_030036),
    (0x03003A, i_03003a),
    (0x03003E, i_03003e),
    (0x030042, i_030042),
    (0x030048, i_030048),
    (0x03004C, i_03004c),
    (0x030050, i_030050),
    (0x030054, i_030054),
    (0x03005A, i_03005a),
    (0x030060, i_030060),
    (0x030066, i_030066),
    (0x030068, i_030068),
    (0x03006C, i_03006c),
    (0x030072, i_030072),
    (0x030076, i_030076),
    (0x03007C, i_03007c),
    (0x030080, i_030080),
    (0x030086, i_030086),
    (0x03008A, i_03008a),
    (0x030090, i_030090),
    (0x030094, i_030094),
    (0x03009A, i_03009a),
    (0x03009E, i_03009e),
    (0x0300A2, i_0300a2),
    (0x0300A6, i_0300a6),
    (0x0300AA, i_0300aa),
    (0x0300B0, i_0300b0),
    (0x0300B6, i_0300b6),
    (0x0300BA, i_0300ba),
    (0x0300BE, i_0300be),
    (0x0300C4, i_0300c4),
    (0x0300C8, i_0300c8),
    (0x0300CE, i_0300ce),
    (0x0300D2, i_0300d2),
    (0x0300D8, i_0300d8),
    (0x0300DC, i_0300dc),
    (0x0300E2, i_0300e2),
    (0x0300E6, i_0300e6),
    (0x0300EC, i_0300ec),
    (0x0300F0, i_0300f0),
    (0x0300F4, i_0300f4),
    (0x0300F8, i_0300f8),
    (0x0300FC, i_0300fc),
    (0x030102, i_030102),
    (0x030108, i_030108),
    (0x03010C, i_03010c),
    (0x030110, i_030110),
    (0x030116, i_030116),
    (0x03011A, i_03011a),
    (0x030120, i_030120),
    (0x030124, i_030124),
    (0x03012A, i_03012a),
    (0x03012E, i_03012e),
    (0x030134, i_030134),
    (0x03013A, i_03013a),
    (0x03013E, i_03013e),
    (0x030142, i_030142),
    (0x030146, i_030146),
    (0x03014A, i_03014a),
    (0x030150, i_030150),
    (0x030154, i_030154),
    (0x030156, i_030156),
    (0x03015A, i_03015a),
    (0x030160, i_030160),
    (0x030164, i_030164),
    (0x030168, i_030168),
    (0x03016E, i_03016e),
    (0x030174, i_030174),
    (0x030178, i_030178),
    (0x03017E, i_03017e),
    (0x030184, i_030184),
    (0x03018A, i_03018a),
    (0x03018C, i_03018c),
    (0x030190, i_030190),
    (0x030194, i_030194),
    (0x03019A, i_03019a),
    (0x03019E, i_03019e),
    (0x0301A2, i_0301a2),
    (0x0301AA, i_0301aa),
    (0x0301AE, i_0301ae),
    (0x0301B2, i_0301b2),
    (0x0301B6, i_0301b6),
    (0x0301BA, i_0301ba),
    (0x0301BE, i_0301be),
    (0x0301C2, i_0301c2),
    (0x0301C8, i_0301c8),
    (0x0301CE, i_0301ce),
    (0x0301D2, i_0301d2),
    (0x0301D6, i_0301d6),
    (0x0301DE, i_0301de),
    (0x0301E2, i_0301e2),
    (0x0301E8, i_0301e8),
    (0x0301EE, i_0301ee),
    (0x0301F0, i_0301f0),
    (0x0301F4, i_0301f4),
    (0x0301F6, i_0301f6),
    (0x0301FA, i_0301fa),
    (0x030200, i_030200),
    (0x030204, i_030204),
    (0x030208, i_030208),
    (0x03020E, i_03020e),
    (0x030214, i_030214),
    (0x03021A, i_03021a),
    (0x03021E, i_03021e),
    (0x030224, i_030224),
    (0x030228, i_030228),
    (0x03022C, i_03022c),
    (0x030230, i_030230),
    (0x030236, i_030236),
    (0x03023A, i_03023a),
    (0x030240, i_030240),
    (0x030244, i_030244),
    (0x03024A, i_03024a),
    (0x030250, i_030250),
    (0x030254, i_030254),
    (0x030258, i_030258),
    (0x03025C, i_03025c),
    (0x030262, i_030262),
    (0x030266, i_030266),
    (0x03026A, i_03026a),
    (0x03026E, i_03026e),
    (0x030272, i_030272),
    (0x030276, i_030276),
    (0x03027A, i_03027a),
    (0x030280, i_030280),
    (0x030284, i_030284),
    (0x030288, i_030288),
    (0x03028C, i_03028c),
    (0x030290, i_030290),
    (0x030294, i_030294),
    (0x030298, i_030298),
    (0x03029C, i_03029c),
    (0x0302A0, i_0302a0),
    (0x0302A4, i_0302a4),
    (0x0302A8, i_0302a8),
    (0x0302AE, i_0302ae),
    (0x0302B2, i_0302b2),
    (0x0302B6, i_0302b6),
    (0x0302BA, i_0302ba),
    (0x0302C0, i_0302c0),
    (0x0302C4, i_0302c4),
    (0x0302C8, i_0302c8),
    (0x0302CC, i_0302cc),
    (0x0302CE, i_0302ce),
    (0x0302D4, i_0302d4),
    (0x0302D8, i_0302d8),
    (0x0302DC, i_0302dc),
    (0x0302E0, i_0302e0),
    (0x0302E6, i_0302e6),
    (0x0302EA, i_0302ea),
    (0x0302EE, i_0302ee),
    (0x0302F2, i_0302f2),
    (0x0302F6, i_0302f6),
    (0x0302F8, i_0302f8),
    (0x0302FC, i_0302fc),
    (0x0302FE, i_0302fe),
    (0x030300, i_030300),
    (0x030304, i_030304),
    (0x030308, i_030308),
    (0x03030C, i_03030c),
    (0x030312, i_030312),
    (0x030316, i_030316),
    (0x03031C, i_03031c),
    (0x030320, i_030320),
    (0x030326, i_030326),
    (0x03032A, i_03032a),
    (0x03032E, i_03032e),
    (0x030332, i_030332),
    (0x030336, i_030336),
    (0x03033A, i_03033a),
    (0x030340, i_030340),
    (0x030344, i_030344),
    (0x03034A, i_03034a),
    (0x03034E, i_03034e),
    (0x030354, i_030354),
    (0x030358, i_030358),
    (0x03035C, i_03035c),
    (0x030364, i_030364),
    (0x030368, i_030368),
    (0x03036C, i_03036c),
    (0x030370, i_030370),
    (0x030374, i_030374),
    (0x030378, i_030378),
    (0x030380, i_030380),
    (0x030384, i_030384),
    (0x03038A, i_03038a),
    (0x030390, i_030390),
    (0x030392, i_030392),
    (0x030396, i_030396),
    (0x03039A, i_03039a),
    (0x03039E, i_03039e),
    (0x0303A4, i_0303a4),
    (0x0303A8, i_0303a8),
    (0x0303AC, i_0303ac),
    (0x0303B0, i_0303b0),
    (0x0303B4, i_0303b4),
    (0x0303B6, i_0303b6),
    (0x0303BA, i_0303ba),
    (0x0303BE, i_0303be),
    (0x0303C2, i_0303c2),
    (0x0303C6, i_0303c6),
    (0x0303CA, i_0303ca),
    (0x0303CE, i_0303ce),
    (0x0303D2, i_0303d2),
    (0x0303D6, i_0303d6),
    (0x0303DA, i_0303da),
    (0x0303DE, i_0303de),
    (0x0303E2, i_0303e2),
    (0x0303E6, i_0303e6),
    (0x0303E8, i_0303e8),
    (0x0303EC, i_0303ec),
    (0x0303EE, i_0303ee),
    (0x0303F0, i_0303f0),
    (0x0303F4, i_0303f4),
    (0x0303F8, i_0303f8),
    (0x0303FA, i_0303fa),
    (0x0303FE, i_0303fe),
    (0x030400, i_030400),
    (0x030402, i_030402),
    (0x030406, i_030406),
    (0x030408, i_030408),
    (0x03040C, i_03040c),
    (0x030410, i_030410),
    (0x030412, i_030412),
    (0x030414, i_030414),
    (0x030416, i_030416),
    (0x03041A, i_03041a),
    (0x030420, i_030420),
    (0x030424, i_030424),
    (0x03042A, i_03042a),
    (0x03042E, i_03042e),
    (0x030434, i_030434),
    (0x030438, i_030438),
    (0x03043C, i_03043c),
    (0x030440, i_030440),
    (0x030444, i_030444),
    (0x030448, i_030448),
    (0x03044A, i_03044a),
    (0x03044E, i_03044e),
    (0x030452, i_030452),
    (0x030456, i_030456),
    (0x03045A, i_03045a),
    (0x03045C, i_03045c),
    (0x030460, i_030460),
    (0x030462, i_030462),
    (0x030466, i_030466),
    (0x03046A, i_03046a),
    (0x03046E, i_03046e),
    (0x030472, i_030472),
    (0x030474, i_030474),
    (0x030478, i_030478),
    (0x03047E, i_03047e),
    (0x030482, i_030482),
    (0x030486, i_030486),
    (0x03048A, i_03048a),
    (0x03048E, i_03048e),
    (0x030490, i_030490),
    (0x030494, i_030494),
    (0x030498, i_030498),
    (0x03049C, i_03049c),
    (0x0304A0, i_0304a0),
    (0x0304A2, i_0304a2),
    (0x0304A6, i_0304a6),
    (0x0304AA, i_0304aa),
    (0x0304AE, i_0304ae),
    (0x0304B2, i_0304b2),
    (0x0304B6, i_0304b6),
    (0x0304B8, i_0304b8),
    (0x0304BA, i_0304ba),
    (0x0304BE, i_0304be),
    (0x0304C2, i_0304c2),
    (0x0304C4, i_0304c4),
    (0x0304C8, i_0304c8),
    (0x0304CC, i_0304cc),
    (0x0304D0, i_0304d0),
    (0x0304D4, i_0304d4),
    (0x0304D8, i_0304d8),
    (0x0304DC, i_0304dc),
    (0x0304E0, i_0304e0),
    (0x0304E4, i_0304e4),
    (0x0304E8, i_0304e8),
    (0x0304EA, i_0304ea),
    (0x0304EE, i_0304ee),
    (0x0304F4, i_0304f4),
    (0x0304FA, i_0304fa),
    (0x0304FE, i_0304fe),
    (0x030504, i_030504),
    (0x03050A, i_03050a),
    (0x03050C, i_03050c),
    (0x030510, i_030510),
    (0x030514, i_030514),
    (0x030518, i_030518),
    (0x03051C, i_03051c),
    (0x030520, i_030520),
    (0x030524, i_030524),
    (0x030528, i_030528),
    (0x03052C, i_03052c),
    (0x030530, i_030530),
    (0x030532, i_030532),
    (0x030536, i_030536),
    (0x03053C, i_03053c),
    (0x030542, i_030542),
    (0x030546, i_030546),
    (0x03054C, i_03054c),
    (0x030552, i_030552),
    (0x030554, i_030554),
    (0x03055C, i_03055c),
    (0x030560, i_030560),
    (0x030566, i_030566),
    (0x030568, i_030568),
    (0x03056C, i_03056c),
    (0x030570, i_030570),
    (0x030576, i_030576),
    (0x03057C, i_03057c),
    (0x03057E, i_03057e),
    (0x030582, i_030582),
    (0x030586, i_030586),
    (0x03058C, i_03058c),
    (0x030592, i_030592),
    (0x030594, i_030594),
    (0x030598, i_030598),
    (0x03059C, i_03059c),
    (0x0305A2, i_0305a2),
    (0x0305A8, i_0305a8),
    (0x0305AA, i_0305aa),
    (0x0305AE, i_0305ae),
    (0x0305B4, i_0305b4),
    (0x0305BA, i_0305ba),
    (0x0305BC, i_0305bc),
    (0x0305C0, i_0305c0),
    (0x0305C4, i_0305c4),
    (0x0305CA, i_0305ca),
    (0x0305D0, i_0305d0),
    (0x0305D2, i_0305d2),
    (0x0305D6, i_0305d6),
    (0x0305DA, i_0305da),
    (0x0305E0, i_0305e0),
    (0x0305E6, i_0305e6),
    (0x0305E8, i_0305e8),
    (0x0305EC, i_0305ec),
    (0x0305F2, i_0305f2),
    (0x0305F8, i_0305f8),
    (0x0305FA, i_0305fa),
    (0x0305FE, i_0305fe),
    (0x030602, i_030602),
    (0x030608, i_030608),
    (0x03060A, i_03060a),
    (0x03060E, i_03060e),
    (0x030614, i_030614),
    (0x03061A, i_03061a),
    (0x03061E, i_03061e),
    (0x030624, i_030624),
    (0x030628, i_030628),
    (0x03062C, i_03062c),
    (0x030632, i_030632),
    (0x030638, i_030638),
    (0x03063C, i_03063c),
    (0x030642, i_030642),
    (0x030646, i_030646),
    (0x03064A, i_03064a),
    (0x030650, i_030650),
    (0x030656, i_030656),
    (0x03065A, i_03065a),
    (0x030660, i_030660),
    (0x030664, i_030664),
    (0x030668, i_030668),
    (0x03066C, i_03066c),
    (0x030670, i_030670),
    (0x030674, i_030674),
    (0x030678, i_030678),
    (0x03067C, i_03067c),
    (0x030680, i_030680),
    (0x030684, i_030684),
    (0x030688, i_030688),
    (0x03068C, i_03068c),
    (0x030690, i_030690),
    (0x030694, i_030694),
    (0x030704, i_030704),
    (0x030708, i_030708),
    (0x03070C, i_03070c),
    (0x03070E, i_03070e),
    (0x030712, i_030712),
    (0x030716, i_030716),
    (0x03071A, i_03071a),
    (0x03071E, i_03071e),
    (0x030720, i_030720),
    (0x030724, i_030724),
    (0x030728, i_030728),
    (0x03072C, i_03072c),
    (0x030730, i_030730),
    (0x030732, i_030732),
    (0x030734, i_030734),
    (0x030736, i_030736),
    (0x03073C, i_03073c),
    (0x03076A, i_03076a),
    (0x03076E, i_03076e),
    (0x030772, i_030772),
    (0x030776, i_030776),
    (0x03077A, i_03077a),
    (0x03077E, i_03077e),
    (0x030782, i_030782),
    (0x030786, i_030786),
    (0x03078A, i_03078a),
    (0x03078C, i_03078c),
    (0x03078E, i_03078e),
    (0x030790, i_030790),
    (0x030792, i_030792),
    (0x030796, i_030796),
    (0x03079A, i_03079a),
    (0x03079E, i_03079e),
    (0x0307A2, i_0307a2),
    (0x0307A6, i_0307a6),
    (0x0307AA, i_0307aa),
    (0x0307AE, i_0307ae),
    (0x0307B2, i_0307b2),
    (0x0307B6, i_0307b6),
    (0x0307BA, i_0307ba),
    (0x0307BE, i_0307be),
    (0x0307C2, i_0307c2),
    (0x0307C6, i_0307c6),
    (0x0307CA, i_0307ca),
    (0x0307CE, i_0307ce),
    (0x0307D2, i_0307d2),
    (0x0307D6, i_0307d6),
    (0x0307DA, i_0307da),
    (0x0307DE, i_0307de),
    (0x0307E2, i_0307e2),
    (0x0307E4, i_0307e4),
    (0x0307E6, i_0307e6),
    (0x0308B0, i_0308b0),
    (0x0308B4, i_0308b4),
    (0x0308B8, i_0308b8),
    (0x0308BE, i_0308be),
    (0x0308D0, i_0308d0),
    (0x0308D6, i_0308d6),
    (0x0308DC, i_0308dc),
    (0x0308E2, i_0308e2),
    (0x0308E8, i_0308e8),
    (0x0308EE, i_0308ee),
    (0x0308F4, i_0308f4),
    (0x0308FA, i_0308fa),
    (0x0308FE, i_0308fe),
    (0x030902, i_030902),
    (0x030904, i_030904),
    (0x030908, i_030908),
    (0x03090C, i_03090c),
    (0x030912, i_030912),
    (0x030914, i_030914),
    (0x030918, i_030918),
    (0x03091E, i_03091e),
    (0x030922, i_030922),
    (0x030928, i_030928),
    (0x03092C, i_03092c),
    (0x030930, i_030930),
    (0x030936, i_030936),
    (0x03093A, i_03093a),
    (0x030940, i_030940),
    (0x030946, i_030946),
    (0x03094C, i_03094c),
    (0x030950, i_030950),
    (0x030954, i_030954),
    (0x03095A, i_03095a),
    (0x030960, i_030960),
    (0x030966, i_030966),
    (0x03096C, i_03096c),
    (0x030972, i_030972),
    (0x030976, i_030976),
    (0x03097A, i_03097a),
    (0x030980, i_030980),
    (0x030984, i_030984),
    (0x030988, i_030988),
    (0x03098A, i_03098a),
    (0x03098E, i_03098e),
    (0x030992, i_030992),
    (0x030998, i_030998),
    (0x03099A, i_03099a),
    (0x03099E, i_03099e),
    (0x0309A4, i_0309a4),
    (0x0309A8, i_0309a8),
    (0x0309AE, i_0309ae),
    (0x0309B4, i_0309b4),
    (0x0309B8, i_0309b8),
    (0x0309BC, i_0309bc),
    (0x0309C4, i_0309c4),
    (0x0309C8, i_0309c8),
    (0x0309CA, i_0309ca),
    (0x0309CC, i_0309cc),
    (0x0309D0, i_0309d0),
    (0x0309D2, i_0309d2),
    (0x0309D6, i_0309d6),
    (0x0309DA, i_0309da),
    (0x0309DC, i_0309dc),
    (0x0309E0, i_0309e0),
    (0x0309E2, i_0309e2),
    (0x0309E6, i_0309e6),
    (0x0309E8, i_0309e8),
    (0x0309EA, i_0309ea),
    (0x0309EC, i_0309ec),
    (0x0309F0, i_0309f0),
    (0x0309F4, i_0309f4),
    (0x0309FC, i_0309fc),
    (0x030A00, i_030a00),
    (0x030A02, i_030a02),
    (0x030A04, i_030a04),
    (0x030A08, i_030a08),
    (0x030A0A, i_030a0a),
    (0x030A0E, i_030a0e),
    (0x030A12, i_030a12),
    (0x030A14, i_030a14),
    (0x030A18, i_030a18),
    (0x030A1A, i_030a1a),
    (0x030A1E, i_030a1e),
    (0x030A20, i_030a20),
    (0x030A22, i_030a22),
    (0x030A24, i_030a24),
    (0x030A28, i_030a28),
    (0x030A2C, i_030a2c),
    (0x030A34, i_030a34),
    (0x030A38, i_030a38),
    (0x030A3A, i_030a3a),
    (0x030A3C, i_030a3c),
    (0x030A40, i_030a40),
    (0x030A44, i_030a44),
    (0x030A46, i_030a46),
    (0x030A48, i_030a48),
    (0x030A4A, i_030a4a),
    (0x030A4E, i_030a4e),
    (0x030A52, i_030a52),
    (0x030A56, i_030a56),
    (0x030A5E, i_030a5e),
    (0x030A62, i_030a62),
    (0x030A66, i_030a66),
    (0x030A6A, i_030a6a),
    (0x030A72, i_030a72),
    (0x030A76, i_030a76),
    (0x030A7A, i_030a7a),
    (0x030A7E, i_030a7e),
    (0x030A86, i_030a86),
    (0x030A8A, i_030a8a),
    (0x030A8C, i_030a8c),
    (0x030A8E, i_030a8e),
    (0x030A92, i_030a92),
    (0x030A94, i_030a94),
    (0x030A98, i_030a98),
    (0x030A9A, i_030a9a),
    (0x030A9C, i_030a9c),
    (0x030AA0, i_030aa0),
    (0x030AA4, i_030aa4),
    (0x030AA8, i_030aa8),
    (0x030AAC, i_030aac),
    (0x030AB0, i_030ab0),
    (0x030AB4, i_030ab4),
    (0x030AB8, i_030ab8),
    (0x030ABE, i_030abe),
    (0x030AC2, i_030ac2),
    (0x030AC8, i_030ac8),
    (0x030ACC, i_030acc),
    (0x030ACE, i_030ace),
    (0x030AD2, i_030ad2),
    (0x030AD6, i_030ad6),
    (0x030ADC, i_030adc),
    (0x030AE0, i_030ae0),
    (0x030AE4, i_030ae4),
    (0x030AEA, i_030aea),
    (0x030AEE, i_030aee),
    (0x030AF2, i_030af2),
    (0x030AF6, i_030af6),
    (0x030AFA, i_030afa),
    (0x030B00, i_030b00),
    (0x030B04, i_030b04),
    (0x030B08, i_030b08),
    (0x030B0C, i_030b0c),
    (0x030B10, i_030b10),
    (0x030B14, i_030b14),
    (0x030B18, i_030b18),
    (0x030B1C, i_030b1c),
    (0x030B22, i_030b22),
    (0x030B26, i_030b26),
    (0x030B28, i_030b28),
    (0x030B2E, i_030b2e),
    (0x030B34, i_030b34),
    (0x030B38, i_030b38),
    (0x030B3E, i_030b3e),
    (0x030B42, i_030b42),
    (0x030B46, i_030b46),
    (0x030B48, i_030b48),
    (0x030B4C, i_030b4c),
    (0x030B50, i_030b50),
    (0x030B54, i_030b54),
    (0x030B58, i_030b58),
    (0x030B5C, i_030b5c),
    (0x030B60, i_030b60),
    (0x030B64, i_030b64),
    (0x030B68, i_030b68),
    (0x030B6A, i_030b6a),
    (0x030B6E, i_030b6e),
    (0x030B74, i_030b74),
    (0x030B7A, i_030b7a),
    (0x030B7E, i_030b7e),
    (0x030B82, i_030b82),
    (0x030B84, i_030b84),
    (0x030B8A, i_030b8a),
    (0x030B90, i_030b90),
    (0x030B94, i_030b94),
    (0x030B98, i_030b98),
    (0x030B9A, i_030b9a),
    (0x030BA0, i_030ba0),
    (0x030BA4, i_030ba4),
    (0x030BAA, i_030baa),
    (0x030BAE, i_030bae),
    (0x030BB2, i_030bb2),
    (0x030BB4, i_030bb4),
    (0x030BB6, i_030bb6),
    (0x030BBA, i_030bba),
    (0x030BC0, i_030bc0),
    (0x030BC6, i_030bc6),
    (0x030BCA, i_030bca),
    (0x030BCE, i_030bce),
    (0x030BD2, i_030bd2),
    (0x030BD4, i_030bd4),
    (0x030BD6, i_030bd6),
    (0x030BDA, i_030bda),
    (0x030BE0, i_030be0),
    (0x030BE6, i_030be6),
    (0x030BEA, i_030bea),
    (0x030BEE, i_030bee),
    (0x030BF2, i_030bf2),
    (0x030BF4, i_030bf4),
    (0x030BF6, i_030bf6),
    (0x030BFE, i_030bfe),
    (0x030C00, i_030c00),
    (0x030C02, i_030c02),
    (0x030C06, i_030c06),
    (0x030C0A, i_030c0a),
    (0x030C0C, i_030c0c),
    (0x030C0E, i_030c0e),
    (0x030C12, i_030c12),
    (0x031A9C, i_031a9c),
    (0x031AA2, i_031aa2),
    (0x031AA8, i_031aa8),
    (0x031AAC, i_031aac),
    (0x031AB2, i_031ab2),
    (0x031AB8, i_031ab8),
    (0x031C72, i_031c72),
    (0x031C76, i_031c76),
    (0x031C7A, i_031c7a),
    (0x031C7E, i_031c7e),
    (0x031C82, i_031c82),
    (0x031C86, i_031c86),
    (0x031C8A, i_031c8a),
    (0x031C90, i_031c90),
    (0x031C96, i_031c96),
    (0x031C9A, i_031c9a),
    (0x031C9C, i_031c9c),
    (0x031CA0, i_031ca0),
    (0x031CA4, i_031ca4),
    (0x031CAA, i_031caa),
    (0x031CAC, i_031cac),
    (0x031CB0, i_031cb0),
    (0x031CB6, i_031cb6),
    (0x031CBA, i_031cba),
    (0x031CC0, i_031cc0),
    (0x031CC4, i_031cc4),
    (0x031CC6, i_031cc6),
    (0x031CCA, i_031cca),
    (0x031CCE, i_031cce),
    (0x031CD2, i_031cd2),
    (0x031CD6, i_031cd6),
    (0x031CDA, i_031cda),
    (0x031CDE, i_031cde),
    (0x031CE2, i_031ce2),
    (0x031CE8, i_031ce8),
    (0x031CEE, i_031cee),
    (0x031CF2, i_031cf2),
    (0x031CF4, i_031cf4),
    (0x031CF8, i_031cf8),
    (0x031CFE, i_031cfe),
    (0x031D02, i_031d02),
    (0x031D04, i_031d04),
    (0x031D08, i_031d08),
    (0x031D0E, i_031d0e),
    (0x031D14, i_031d14),
    (0x031D18, i_031d18),
    (0x031D1A, i_031d1a),
    (0x031FC2, i_031fc2),
    (0x031FC8, i_031fc8),
    (0x031FCA, i_031fca),
    (0x031FCE, i_031fce),
    (0x031FD4, i_031fd4),
    (0x031FDC, i_031fdc),
    (0x031FE0, i_031fe0),
    (0x031FE4, i_031fe4),
    (0x031FEA, i_031fea),
    (0x031FEE, i_031fee),
    (0x031FF2, i_031ff2),
    (0x031FF8, i_031ff8),
    (0x031FFE, i_031ffe),
    (0x032004, i_032004),
    (0x03200A, i_03200a),
    (0x03200E, i_03200e),
    (0x032010, i_032010),
    (0x032016, i_032016),
    (0x03201A, i_03201a),
    (0x03201E, i_03201e),
    (0x032022, i_032022),
    (0x032026, i_032026),
    (0x03202A, i_03202a),
    (0x032032, i_032032),
    (0x032036, i_032036),
    (0x03203C, i_03203c),
    (0x032040, i_032040),
    (0x032046, i_032046),
    (0x03204A, i_03204a),
    (0x032050, i_032050),
    (0x032054, i_032054),
    (0x03205A, i_03205a),
    (0x03205E, i_03205e),
    (0x032064, i_032064),
    (0x032068, i_032068),
    (0x03206C, i_03206c),
    (0x03206E, i_03206e),
    (0x032112, i_032112),
    (0x03211A, i_03211a),
    (0x03211E, i_03211e),
    (0x032124, i_032124),
    (0x032128, i_032128),
    (0x03212E, i_03212e),
    (0x032132, i_032132),
    (0x032138, i_032138),
    (0x03213E, i_03213e),
    (0x03216E, i_03216e),
    (0x032174, i_032174),
    (0x03217A, i_03217a),
    (0x03217E, i_03217e),
    (0x032180, i_032180),
    (0x032186, i_032186),
    (0x03218A, i_03218a),
    (0x03218E, i_03218e),
    (0x032192, i_032192),
    (0x032196, i_032196),
    (0x03219C, i_03219c),
    (0x0321A0, i_0321a0),
    (0x0321A4, i_0321a4),
    (0x0321A6, i_0321a6),
    (0x0321AA, i_0321aa),
    (0x0321B0, i_0321b0),
    (0x0321B4, i_0321b4),
    (0x0321B8, i_0321b8),
    (0x0321BA, i_0321ba),
    (0x0321BC, i_0321bc),
    (0x032406, i_032406),
    (0x03240A, i_03240a),
    (0x03240C, i_03240c),
    (0x032410, i_032410),
    (0x032A02, i_032a02),
    (0x032A08, i_032a08),
    (0x032A0E, i_032a0e),
    (0x032A14, i_032a14),
    (0x032A1C, i_032a1c),
    (0x032A22, i_032a22),
    (0x032A28, i_032a28),
    (0x032A2E, i_032a2e),
    (0x032A34, i_032a34),
    (0x032A3A, i_032a3a),
    (0x032A40, i_032a40),
    (0x032A46, i_032a46),
    (0x032A4C, i_032a4c),
    (0x032A50, i_032a50),
    (0x032A54, i_032a54),
    (0x032A5A, i_032a5a),
    (0x032A5E, i_032a5e),
    (0x032A62, i_032a62),
    (0x032A66, i_032a66),
    (0x032A6A, i_032a6a),
    (0x032A70, i_032a70),
    (0x032A76, i_032a76),
    (0x032A7C, i_032a7c),
    (0x032A82, i_032a82),
    (0x032A88, i_032a88),
    (0x032A8E, i_032a8e),
    (0x032A94, i_032a94),
    (0x032A9A, i_032a9a),
    (0x032AA0, i_032aa0),
    (0x032AA6, i_032aa6),
    (0x032AA8, i_032aa8),
    (0x032AAE, i_032aae),
    (0x032AB2, i_032ab2),
    (0x032AB8, i_032ab8),
    (0x032ABC, i_032abc),
    (0x032AC0, i_032ac0),
    (0x032AC2, i_032ac2),
    (0x032AC6, i_032ac6),
    (0x032AC8, i_032ac8),
    (0x032ACE, i_032ace),
    (0x032AD4, i_032ad4),
    (0x032AD8, i_032ad8),
    (0x032ADA, i_032ada),
    (0x032ADE, i_032ade),
    (0x032AE0, i_032ae0),
    (0x032AE4, i_032ae4),
    (0x032AE8, i_032ae8),
    (0x032AEA, i_032aea),
    (0x032AEE, i_032aee),
    (0x032AF2, i_032af2),
    (0x032AF6, i_032af6),
    (0x032AF8, i_032af8),
    (0x032AFA, i_032afa),
    (0x032B00, i_032b00),
    (0x032B04, i_032b04),
    (0x032B0A, i_032b0a),
    (0x032B0E, i_032b0e),
    (0x032B14, i_032b14),
    (0x032B1A, i_032b1a),
    (0x032B1C, i_032b1c),
    (0x032B20, i_032b20),
    (0x032B22, i_032b22),
    (0x032B26, i_032b26),
    (0x032B2A, i_032b2a),
    (0x032B2E, i_032b2e),
    (0x032B32, i_032b32),
    (0x032B34, i_032b34),
    (0x032B58, i_032b58),
    (0x032B5C, i_032b5c),
    (0x032B60, i_032b60),
    (0x032B64, i_032b64),
    (0x032B68, i_032b68),
    (0x032B6A, i_032b6a),
    (0x032B6E, i_032b6e),
    (0x032B72, i_032b72),
    (0x032B76, i_032b76),
    (0x032B7C, i_032b7c),
    (0x032B80, i_032b80),
    (0x032B86, i_032b86),
    (0x032B8A, i_032b8a),
    (0x032B8E, i_032b8e),
    (0x032B92, i_032b92),
    (0x032B96, i_032b96),
    (0x032B9A, i_032b9a),
    (0x032B9E, i_032b9e),
    (0x032BA0, i_032ba0),
    (0x032BA4, i_032ba4),
    (0x032BA8, i_032ba8),
    (0x032BAA, i_032baa),
    (0x032BAE, i_032bae),
    (0x032BB2, i_032bb2),
    (0x032BB6, i_032bb6),
    (0x032BBA, i_032bba),
    (0x032BBC, i_032bbc),
    (0x032BC0, i_032bc0),
    (0x032BC4, i_032bc4),
    (0x032CBA, i_032cba),
    (0x032CC0, i_032cc0),
    (0x032CC6, i_032cc6),
    (0x032CC8, i_032cc8),
    (0x032CCE, i_032cce),
    (0x032CD2, i_032cd2),
    (0x032CD8, i_032cd8),
    (0x032CDA, i_032cda),
    (0x032CDE, i_032cde),
    (0x032CE2, i_032ce2),
    (0x032CE4, i_032ce4),
    (0x032CE8, i_032ce8),
    (0x032CEC, i_032cec),
    (0x032CF0, i_032cf0),
    (0x032CF4, i_032cf4),
    (0x032CF8, i_032cf8),
    (0x032CFE, i_032cfe),
    (0x032D00, i_032d00),
    (0x032D04, i_032d04),
    (0x032D0A, i_032d0a),
    (0x032D0C, i_032d0c),
    (0x032D10, i_032d10),
    (0x032D14, i_032d14),
    (0x032D16, i_032d16),
    (0x032D1A, i_032d1a),
    (0x032D1E, i_032d1e),
    (0x032D22, i_032d22),
    (0x032D26, i_032d26),
    (0x032D32, i_032d32),
    (0x032D38, i_032d38),
    (0x032D3C, i_032d3c),
    (0x032D42, i_032d42),
    (0x032D44, i_032d44),
    (0x032D48, i_032d48),
    (0x032D4A, i_032d4a),
    (0x032D4E, i_032d4e),
    (0x032D52, i_032d52),
    (0x032D56, i_032d56),
    (0x032D5A, i_032d5a),
    (0x032D60, i_032d60),
    (0x032D64, i_032d64),
    (0x032D66, i_032d66),
    (0x032D6A, i_032d6a),
    (0x032D6C, i_032d6c),
    (0x032D70, i_032d70),
    (0x032D74, i_032d74),
    (0x032D7A, i_032d7a),
    (0x032D7E, i_032d7e),
    (0x032D82, i_032d82),
    (0x032D86, i_032d86),
    (0x032D8C, i_032d8c),
    (0x032D90, i_032d90),
    (0x032D94, i_032d94),
    (0x032D9A, i_032d9a),
    (0x032D9E, i_032d9e),
    (0x032DA4, i_032da4),
    (0x032DA8, i_032da8),
    (0x032DAE, i_032dae),
    (0x032DB2, i_032db2),
    (0x032DB8, i_032db8),
    (0x032DBC, i_032dbc),
    (0x032DC0, i_032dc0),
    (0x032DC6, i_032dc6),
    (0x032DCA, i_032dca),
    (0x032DCE, i_032dce),
    (0x032DD4, i_032dd4),
    (0x032DD8, i_032dd8),
    (0x032DDA, i_032dda),
    (0x032DDE, i_032dde),
    (0x032E3C, i_032e3c),
    (0x032E42, i_032e42),
    (0x032E48, i_032e48),
    (0x032E4E, i_032e4e),
    (0x032E52, i_032e52),
    (0x032E56, i_032e56),
    (0x032E8E, i_032e8e),
    (0x032E90, i_032e90),
    (0x032E94, i_032e94),
    (0x032E98, i_032e98),
    (0x032E9C, i_032e9c),
    (0x032E9E, i_032e9e),
    (0x032EA2, i_032ea2),
    (0x032EA4, i_032ea4),
    (0x032EAA, i_032eaa),
    (0x032EAE, i_032eae),
    (0x032EB4, i_032eb4),
    (0x032EB8, i_032eb8),
    (0x032EBA, i_032eba),
    (0x032EC0, i_032ec0),
    (0x032EC4, i_032ec4),
    (0x032ECA, i_032eca),
    (0x032ECE, i_032ece),
    (0x032F3C, i_032f3c),
    (0x032F40, i_032f40),
    (0x032F44, i_032f44),
    (0x032F4A, i_032f4a),
    (0x032F4E, i_032f4e),
    (0x032F52, i_032f52),
    (0x032F58, i_032f58),
    (0x032F5E, i_032f5e),
    (0x032F62, i_032f62),
    (0x032F68, i_032f68),
    (0x032F6E, i_032f6e),
    (0x032F72, i_032f72),
    (0x032F78, i_032f78),
    (0x032F7C, i_032f7c),
    (0x032F7E, i_032f7e),
    (0x032F82, i_032f82),
    (0x032F86, i_032f86),
    (0x032F88, i_032f88),
    (0x032F8C, i_032f8c),
    (0x032F90, i_032f90),
    (0x032F96, i_032f96),
    (0x032F9A, i_032f9a),
    (0x032FA0, i_032fa0),
    (0x032FA4, i_032fa4),
    (0x032FAA, i_032faa),
    (0x032FAE, i_032fae),
    (0x032FB0, i_032fb0),
    (0x032FB4, i_032fb4),
    (0x032FB8, i_032fb8),
    (0x032FBA, i_032fba),
    (0x032FBE, i_032fbe),
    (0x032FC4, i_032fc4),
    (0x032FC8, i_032fc8),
    (0x032FCC, i_032fcc),
    (0x032FCE, i_032fce),
    (0x032FD2, i_032fd2),
    (0x032FF0, i_032ff0),
    (0x032FF2, i_032ff2),
    (0x032FF6, i_032ff6),
    (0x032FFC, i_032ffc),
    (0x033002, i_033002),
    (0x033006, i_033006),
    (0x03300C, i_03300c),
    (0x033010, i_033010),
    (0x033014, i_033014),
    (0x033016, i_033016),
    (0x03301A, i_03301a),
    (0x033020, i_033020),
    (0x033024, i_033024),
    (0x03302A, i_03302a),
    (0x03302E, i_03302e),
    (0x033032, i_033032),
    (0x033034, i_033034),
    (0x03303C, i_03303c),
    (0x033040, i_033040),
    (0x033046, i_033046),
    (0x03304A, i_03304a),
    (0x033052, i_033052),
    (0x033056, i_033056),
    (0x03305C, i_03305c),
    (0x033060, i_033060),
    (0x033066, i_033066),
    (0x03306A, i_03306a),
    (0x03306C, i_03306c),
    (0x033070, i_033070),
    (0x033072, i_033072),
    (0x033078, i_033078),
    (0x03307A, i_03307a),
    (0x03307E, i_03307e),
    (0x0330CA, i_0330ca),
    (0x0330D0, i_0330d0),
    (0x0330D6, i_0330d6),
    (0x0330DA, i_0330da),
    (0x0330DE, i_0330de),
    (0x0330E4, i_0330e4),
    (0x0330E8, i_0330e8),
    (0x0330EC, i_0330ec),
    (0x0330F0, i_0330f0),
    (0x0330F6, i_0330f6),
    (0x0330FA, i_0330fa),
    (0x0330FE, i_0330fe),
    (0x033102, i_033102),
    (0x033108, i_033108),
    (0x03310E, i_03310e),
    (0x033114, i_033114),
    (0x03311A, i_03311a),
    (0x03311E, i_03311e),
    (0x033122, i_033122),
    (0x033126, i_033126),
    (0x03312C, i_03312c),
    (0x03312E, i_03312e),
    (0x033134, i_033134),
    (0x033138, i_033138),
    (0x03313C, i_03313c),
    (0x033144, i_033144),
    (0x033148, i_033148),
    (0x03314C, i_03314c),
    (0x033150, i_033150),
    (0x033156, i_033156),
    (0x03315A, i_03315a),
    (0x03315E, i_03315e),
    (0x033162, i_033162),
    (0x033166, i_033166),
    (0x03316A, i_03316a),
    (0x03316C, i_03316c),
    (0x033170, i_033170),
    (0x033172, i_033172),
    (0x033178, i_033178),
    (0x03317C, i_03317c),
    (0x033180, i_033180),
    (0x033184, i_033184),
    (0x03318A, i_03318a),
    (0x03318E, i_03318e),
    (0x033192, i_033192),
    (0x033196, i_033196),
    (0x03319C, i_03319c),
    (0x0331A0, i_0331a0),
    (0x0331A4, i_0331a4),
    (0x0331A8, i_0331a8),
    (0x0331AC, i_0331ac),
    (0x0331CE, i_0331ce),
    (0x0331D2, i_0331d2),
    (0x0331D6, i_0331d6),
    (0x0331DC, i_0331dc),
    (0x0331E0, i_0331e0),
    (0x0331E4, i_0331e4),
    (0x0331E8, i_0331e8),
    (0x0331EC, i_0331ec),
    (0x0331F0, i_0331f0),
    (0x0331F6, i_0331f6),
    (0x0331FA, i_0331fa),
    (0x0331FE, i_0331fe),
    (0x033202, i_033202),
    (0x033208, i_033208),
    (0x03320E, i_03320e),
    (0x033214, i_033214),
    (0x03321A, i_03321a),
    (0x03321E, i_03321e),
    (0x033222, i_033222),
    (0x033226, i_033226),
    (0x03322C, i_03322c),
    (0x03322E, i_03322e),
    (0x033234, i_033234),
    (0x033238, i_033238),
    (0x03323C, i_03323c),
    (0x033244, i_033244),
    (0x033248, i_033248),
    (0x03324A, i_03324a),
    (0x03324E, i_03324e),
    (0x033250, i_033250),
    (0x033254, i_033254),
    (0x033256, i_033256),
    (0x03325A, i_03325a),
    (0x033260, i_033260),
    (0x033262, i_033262),
    (0x033268, i_033268),
    (0x03326C, i_03326c),
    (0x03326E, i_03326e),
    (0x033272, i_033272),
    (0x033276, i_033276),
    (0x03327C, i_03327c),
    (0x033280, i_033280),
    (0x033286, i_033286),
    (0x03328A, i_03328a),
    (0x033290, i_033290),
    (0x033296, i_033296),
    (0x03329A, i_03329a),
    (0x03329E, i_03329e),
    (0x0332A2, i_0332a2),
    (0x0332A8, i_0332a8),
    (0x0332AE, i_0332ae),
    (0x0332B4, i_0332b4),
    (0x0332B8, i_0332b8),
    (0x033346, i_033346),
    (0x0334A2, i_0334a2),
    (0x0334AA, i_0334aa),
    (0x0334AE, i_0334ae),
    (0x0334B4, i_0334b4),
    (0x0334BA, i_0334ba),
    (0x0334C0, i_0334c0),
    (0x0334C4, i_0334c4),
    (0x0334C6, i_0334c6),
    (0x0334CC, i_0334cc),
    (0x0334D0, i_0334d0),
    (0x0334D6, i_0334d6),
    (0x0334DA, i_0334da),
    (0x0334E2, i_0334e2),
    (0x0334E6, i_0334e6),
    (0x0334EA, i_0334ea),
    (0x0334EE, i_0334ee),
    (0x0334F2, i_0334f2),
    (0x0334F6, i_0334f6),
    (0x0334FC, i_0334fc),
    (0x033500, i_033500),
    (0x033504, i_033504),
    (0x03350A, i_03350a),
    (0x033510, i_033510),
    (0x033514, i_033514),
    (0x033518, i_033518),
    (0x03351C, i_03351c),
    (0x033522, i_033522),
    (0x033526, i_033526),
    (0x03352A, i_03352a),
    (0x033530, i_033530),
    (0x033534, i_033534),
    (0x033538, i_033538),
    (0x03353A, i_03353a),
    (0x03353E, i_03353e),
    (0x033542, i_033542),
    (0x033544, i_033544),
    (0x03354A, i_03354a),
    (0x03354E, i_03354e),
    (0x033554, i_033554),
    (0x033558, i_033558),
    (0x03355C, i_03355c),
    (0x033562, i_033562),
    (0x033566, i_033566),
    (0x03356A, i_03356a),
    (0x033570, i_033570),
    (0x033572, i_033572),
    (0x033574, i_033574),
    (0x033578, i_033578),
    (0x03357A, i_03357a),
    (0x03357E, i_03357e),
    (0x033582, i_033582),
    (0x033584, i_033584),
    (0x033588, i_033588),
    (0x03358C, i_03358c),
    (0x03358E, i_03358e),
    (0x033592, i_033592),
    (0x033596, i_033596),
    (0x03359A, i_03359a),
    (0x03359E, i_03359e),
    (0x0335A0, i_0335a0),
    (0x0335A4, i_0335a4),
    (0x0335A6, i_0335a6),
    (0x0335AC, i_0335ac),
    (0x0335B2, i_0335b2),
    (0x0335B6, i_0335b6),
    (0x0335BC, i_0335bc),
    (0x0335C0, i_0335c0),
    (0x0335C6, i_0335c6),
    (0x0335CA, i_0335ca),
    (0x0335D0, i_0335d0),
    (0x0335D6, i_0335d6),
    (0x0335DC, i_0335dc),
    (0x0335E0, i_0335e0),
    (0x0335E6, i_0335e6),
    (0x0335EC, i_0335ec),
    (0x0335F2, i_0335f2),
    (0x0335F8, i_0335f8),
    (0x0335FE, i_0335fe),
    (0x033604, i_033604),
    (0x033608, i_033608),
    (0x03360C, i_03360c),
    (0x033610, i_033610),
    (0x033614, i_033614),
    (0x033618, i_033618),
    (0x03361C, i_03361c),
    (0x033622, i_033622),
    (0x033626, i_033626),
    (0x03362C, i_03362c),
    (0x033630, i_033630),
    (0x033634, i_033634),
    (0x033636, i_033636),
    (0x03364A, i_03364a),
    (0x033650, i_033650),
    (0x033656, i_033656),
    (0x03365A, i_03365a),
    (0x03365E, i_03365e),
    (0x033662, i_033662),
    (0x033668, i_033668),
    (0x03366C, i_03366c),
    (0x033672, i_033672),
    (0x033676, i_033676),
    (0x03367C, i_03367c),
    (0x033682, i_033682),
    (0x033688, i_033688),
    (0x03368C, i_03368c),
    (0x033692, i_033692),
    (0x033698, i_033698),
    (0x03369E, i_03369e),
    (0x0336A4, i_0336a4),
    (0x0336AA, i_0336aa),
    (0x0336B0, i_0336b0),
    (0x033742, i_033742),
    (0x033746, i_033746),
    (0x03374A, i_03374a),
    (0x03374E, i_03374e),
    (0x033752, i_033752),
    (0x033756, i_033756),
    (0x03375A, i_03375a),
    (0x03375E, i_03375e),
    (0x033762, i_033762),
    (0x033766, i_033766),
    (0x03376A, i_03376a),
    (0x03376C, i_03376c),
    (0x033770, i_033770),
    (0x033774, i_033774),
    (0x033778, i_033778),
    (0x03377C, i_03377c),
    (0x033780, i_033780),
    (0x033784, i_033784),
    (0x033788, i_033788),
    (0x03378E, i_03378e),
    (0x033794, i_033794),
    (0x033798, i_033798),
    (0x03379E, i_03379e),
    (0x0337A4, i_0337a4),
    (0x0337A8, i_0337a8),
    (0x0337AC, i_0337ac),
    (0x0337B2, i_0337b2),
    (0x0337B8, i_0337b8),
    (0x0337BE, i_0337be),
    (0x0337C4, i_0337c4),
    (0x0337CA, i_0337ca),
    (0x0337D0, i_0337d0),
    (0x0337D6, i_0337d6),
    (0x0337DA, i_0337da),
    (0x0337DE, i_0337de),
    (0x0337E4, i_0337e4),
    (0x0337E8, i_0337e8),
    (0x0337EA, i_0337ea),
    (0x0337F0, i_0337f0),
    (0x0337F6, i_0337f6),
    (0x0337FA, i_0337fa),
    (0x0337FE, i_0337fe),
    (0x033802, i_033802),
    (0x033804, i_033804),
    (0x03380A, i_03380a),
    (0x033810, i_033810),
    (0x033814, i_033814),
    (0x03381A, i_03381a),
    (0x03381E, i_03381e),
    (0x033822, i_033822),
    (0x033824, i_033824),
    (0x03382A, i_03382a),
    (0x03382E, i_03382e),
    (0x033836, i_033836),
    (0x03383A, i_03383a),
    (0x033840, i_033840),
    (0x033844, i_033844),
    (0x03384A, i_03384a),
    (0x03384E, i_03384e),
    (0x033854, i_033854),
    (0x033858, i_033858),
    (0x03385E, i_03385e),
    (0x033862, i_033862),
    (0x033866, i_033866),
    (0x033868, i_033868),
    (0x03386C, i_03386c),
    (0x0338FA, i_0338fa),
    (0x033900, i_033900),
    (0x033906, i_033906),
    (0x03390C, i_03390c),
    (0x033914, i_033914),
    (0x033918, i_033918),
    (0x03391C, i_03391c),
    (0x033920, i_033920),
    (0x033924, i_033924),
    (0x033928, i_033928),
    (0x03392A, i_03392a),
    (0x033930, i_033930),
    (0x033934, i_033934),
    (0x033938, i_033938),
    (0x03393E, i_03393e),
    (0x033942, i_033942),
    (0x033946, i_033946),
    (0x03394A, i_03394a),
    (0x03394C, i_03394c),
    (0x033950, i_033950),
    (0x033952, i_033952),
    (0x033956, i_033956),
    (0x03395A, i_03395a),
    (0x033960, i_033960),
    (0x033966, i_033966),
    (0x03396C, i_03396c),
    (0x033972, i_033972),
    (0x033978, i_033978),
    (0x03397E, i_03397e),
    (0x033984, i_033984),
    (0x03398A, i_03398a),
    (0x033990, i_033990),
    (0x033996, i_033996),
    (0x03399C, i_03399c),
    (0x0339A0, i_0339a0),
    (0x0339A4, i_0339a4),
    (0x0339A8, i_0339a8),
    (0x0339AA, i_0339aa),
    (0x0339B0, i_0339b0),
    (0x0339B6, i_0339b6),
    (0x0339BA, i_0339ba),
    (0x0339BE, i_0339be),
    (0x0339C2, i_0339c2),
    (0x0339C6, i_0339c6),
    (0x0339CC, i_0339cc),
    (0x0339D0, i_0339d0),
    (0x0339D6, i_0339d6),
    (0x0339DA, i_0339da),
    (0x0339DE, i_0339de),
    (0x0339E0, i_0339e0),
    (0x0339E6, i_0339e6),
    (0x0339EA, i_0339ea),
    (0x0339EE, i_0339ee),
    (0x0339F2, i_0339f2),
    (0x0339F6, i_0339f6),
    (0x0339F8, i_0339f8),
    (0x0339FC, i_0339fc),
    (0x033A00, i_033a00),
    (0x033A04, i_033a04),
    (0x033A08, i_033a08),
    (0x033A0C, i_033a0c),
    (0x033A0E, i_033a0e),
    (0x033A14, i_033a14),
    (0x033A18, i_033a18),
    (0x033A1C, i_033a1c),
    (0x033A1E, i_033a1e),
    (0x033A26, i_033a26),
    (0x033A2A, i_033a2a),
    (0x033A30, i_033a30),
    (0x033A34, i_033a34),
    (0x033A3A, i_033a3a),
    (0x033A3E, i_033a3e),
    (0x033A44, i_033a44),
    (0x033A48, i_033a48),
    (0x033A4E, i_033a4e),
    (0x033A52, i_033a52),
    (0x033A56, i_033a56),
    (0x033A58, i_033a58),
    (0x033A5C, i_033a5c),
    (0x033A5E, i_033a5e),
    (0x033A64, i_033a64),
    (0x033A6A, i_033a6a),
    (0x033A70, i_033a70),
    (0x033A76, i_033a76),
    (0x033A7C, i_033a7c),
    (0x033A82, i_033a82),
    (0x033A88, i_033a88),
    (0x033A8E, i_033a8e),
    (0x033A94, i_033a94),
    (0x033A9A, i_033a9a),
    (0x033AA0, i_033aa0),
    (0x033AA4, i_033aa4),
    (0x033AA8, i_033aa8),
    (0x033AB0, i_033ab0),
    (0x033AB6, i_033ab6),
    (0x033ABA, i_033aba),
    (0x033ABE, i_033abe),
    (0x033AC0, i_033ac0),
    (0x033AC4, i_033ac4),
    (0x033AC8, i_033ac8),
    (0x033ACE, i_033ace),
    (0x033AD2, i_033ad2),
    (0x033AD6, i_033ad6),
    (0x033ADA, i_033ada),
    (0x033ADC, i_033adc),
    (0x033AE0, i_033ae0),
    (0x033AE4, i_033ae4),
    (0x033AE8, i_033ae8),
    (0x033AEC, i_033aec),
    (0x033AEE, i_033aee),
    (0x033AF2, i_033af2),
    (0x033AF6, i_033af6),
    (0x033AF8, i_033af8),
    (0x033AFC, i_033afc),
    (0x033B02, i_033b02),
    (0x033B08, i_033b08),
    (0x033B0E, i_033b0e),
    (0x033B14, i_033b14),
    (0x033B1A, i_033b1a),
    (0x033B20, i_033b20),
    (0x033B26, i_033b26),
    (0x033B2C, i_033b2c),
    (0x033B32, i_033b32),
    (0x033B38, i_033b38),
    (0x033B3E, i_033b3e),
    (0x033B42, i_033b42),
    (0x033B46, i_033b46),
    (0x033B4E, i_033b4e),
    (0x033B54, i_033b54),
    (0x033B58, i_033b58),
    (0x033B5C, i_033b5c),
    (0x033B5E, i_033b5e),
    (0x033B62, i_033b62),
    (0x033B66, i_033b66),
    (0x033B6C, i_033b6c),
    (0x033B70, i_033b70),
    (0x033B74, i_033b74),
    (0x033B78, i_033b78),
    (0x033B7A, i_033b7a),
    (0x033B7E, i_033b7e),
    (0x033B82, i_033b82),
    (0x033B86, i_033b86),
    (0x033B8A, i_033b8a),
    (0x033B8C, i_033b8c),
    (0x033B90, i_033b90),
    (0x033B94, i_033b94),
    (0x033B96, i_033b96),
    (0x033B9A, i_033b9a),
    (0x033BA0, i_033ba0),
    (0x033BA4, i_033ba4),
    (0x033BA8, i_033ba8),
    (0x033BAE, i_033bae),
    (0x033BB4, i_033bb4),
    (0x033BBA, i_033bba),
    (0x033BC0, i_033bc0),
    (0x033BC6, i_033bc6),
    (0x033BCC, i_033bcc),
    (0x033BD2, i_033bd2),
    (0x033BD8, i_033bd8),
    (0x033BDE, i_033bde),
    (0x033BE4, i_033be4),
    (0x033BEA, i_033bea),
    (0x033BEE, i_033bee),
    (0x033BF2, i_033bf2),
    (0x033BFA, i_033bfa),
    (0x033C00, i_033c00),
    (0x033C04, i_033c04),
    (0x033C08, i_033c08),
    (0x033C0A, i_033c0a),
    (0x033C0E, i_033c0e),
    (0x033C12, i_033c12),
    (0x033C18, i_033c18),
    (0x033C1C, i_033c1c),
    (0x033C20, i_033c20),
    (0x033C24, i_033c24),
    (0x033C26, i_033c26),
    (0x033C2A, i_033c2a),
    (0x033C2E, i_033c2e),
    (0x033C32, i_033c32),
    (0x033C36, i_033c36),
    (0x033C38, i_033c38),
    (0x033C3C, i_033c3c),
    (0x033C40, i_033c40),
    (0x033C42, i_033c42),
    (0x033C48, i_033c48),
    (0x033C4C, i_033c4c),
    (0x033C4E, i_033c4e),
    (0x033C52, i_033c52),
    (0x033C56, i_033c56),
    (0x033C5A, i_033c5a),
    (0x033C5C, i_033c5c),
    (0x033C62, i_033c62),
    (0x033C66, i_033c66),
    (0x033C6A, i_033c6a),
    (0x033C6C, i_033c6c),
    (0x033C72, i_033c72),
    (0x033C76, i_033c76),
];
