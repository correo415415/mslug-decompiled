// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p03_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $030000: rts
pub fn i_030000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30002;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
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
// ===== Chain3_InitAlt_030696  ($030696, 106 B, player/tables_fx_030696.s)
/// $030696: bset.b #$3, $5b(a6)
pub fn i_030696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3069C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03069C: lea.l $3010c(pc), a1
pub fn i_03069c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3010C;
    16
}
/// $0306A0: jsr $4ae.l
pub fn i_0306a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x306A6);
    cpu.pc = 0x4AE;
    16
}
/// $0306A6: jsr $5dd02.l
pub fn i_0306a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x306AC);
    cpu.pc = 0x5DD02;
    16
}
/// $0306AC: move.l a0, $7c(a6)
pub fn i_0306ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306B0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0306B0: move.w $82(a6), $24(a0)
pub fn i_0306b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306B6;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0306B6: move.b #$1, $98(a0)
pub fn i_0306b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306BC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0306BC: lea.l $30068(pc), a1
pub fn i_0306bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30068;
    16
}
/// $0306C0: jsr $4ae.l
pub fn i_0306c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x306C6);
    cpu.pc = 0x4AE;
    16
}
/// $0306C6: jsr $5dd02.l
pub fn i_0306c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x306CC);
    cpu.pc = 0x5DD02;
    16
}
/// $0306CC: move.l a0, $74(a6)
pub fn i_0306cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306D0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0306D0: move.w $82(a6), $24(a0)
pub fn i_0306d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306D6;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0306D6: move.b #$1, $98(a0)
pub fn i_0306d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306DC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0306DC: lea.l $300ba(pc), a1
pub fn i_0306dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x300BA;
    16
}
/// $0306E0: jsr $4ae.l
pub fn i_0306e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x306E6);
    cpu.pc = 0x4AE;
    16
}
/// $0306E6: jsr $5dd02.l
pub fn i_0306e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x306EC);
    cpu.pc = 0x5DD02;
    16
}
/// $0306EC: move.l a0, $78(a6)
pub fn i_0306ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306F0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0306F0: move.w $82(a6), $24(a0)
pub fn i_0306f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306F6;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0306F6: move.b #$1, $98(a0)
pub fn i_0306f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x306FC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0306FC: bra.w $30664
pub fn i_0306fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30700;
    cpu.ir = 0x6000;
    cpu.pc = 0x30664;
    10
}
// ===== Chain3_Nop_030700  ($030700, 2 B, player/tables_fx_030696.s)
/// $030700: rts
pub fn i_030700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30702;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00030702  ($030702, 2 B, thunks/trivial_rts.c)
/// $030702: rts
pub fn i_030702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30704;
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
// ===== Chain3_LinkCmpField82_03073e  ($03073E, 32 B, player/tables_fx_030696.s)
/// $03073E: jsr $30704(pc)
pub fn i_03073e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30742;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x30742);
    cpu.pc = 0x30704;
    16
}
/// $030742: bne.w $30764
pub fn i_030742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30746;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x30764; return 10; }
    8
}
/// $030746: move.w $82(a1), d1
pub fn i_030746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3074A;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03074A: move.w #$1, d0
pub fn i_03074a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3074E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03074E: cmp.w $82(a2), d1
pub fn i_03074e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30752;
    cpu.ir = 0xB26A;
    let ad2 = cpu.a[2].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $030752: bge.w $3075e
pub fn i_030752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30756;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x3075E; return 10; }
    8
}
/// $030756: move.w $82(a2), d1
pub fn i_030756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3075A;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03075A: move.w #$ffff, d0
pub fn i_03075a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3075E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== SetXN_03075e  ($03075E, 6 B, thunks/ccr_helpers.c)
/// $03075E: ori.b #$11, ccr
pub fn i_03075e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30762;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $030762: rts
pub fn i_030762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30764;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_030764  ($030764, 6 B, thunks/ccr_helpers.c)
/// $030764: andi.b #$ee, ccr
pub fn i_030764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30768;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $030768: rts
pub fn i_030768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3076A;
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
// ===== Entity_CmpDepthWithLink8_0307e8  ($0307E8, 16 B, player/tables_fx_030696.s)
/// $0307E8: movea.l $8(a6), a1
pub fn i_0307e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307EC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0307EC: move.b $10(a6), d0
pub fn i_0307ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307F0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0307F0: cmp.b $10(a1), d0
pub fn i_0307f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307F4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0307F4: bcs.w $307fe
pub fn i_0307f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307F8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x307FE; return 10; }
    8
}
// ===== ClearXN_0307f8  ($0307F8, 6 B, thunks/ccr_helpers.c)
/// $0307F8: andi.b #$ee, ccr
pub fn i_0307f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307FC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0307FC: rts
pub fn i_0307fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x307FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0307fe  ($0307FE, 6 B, thunks/ccr_helpers.c)
/// $0307FE: ori.b #$11, ccr
pub fn i_0307fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30802;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $030802: rts
pub fn i_030802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30804;
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
// ===== PlayerGrenade_Spawn_0308c2  ($0308C2, 120 B, player/tables_fx_030696.s)
/// $0308C2: move.w #$2, d1
pub fn i_0308c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308C6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0308C6: move.w d1, $9e(a6)
pub fn i_0308c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308CA;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0308CA: jsr $236e.l
pub fn i_0308ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x308D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x308D0);
    cpu.pc = 0x236E;
    16
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
// ===== EnemyShot_Straight_030c14  ($030C14, 92 B, player/tables_fx_030696.s)
/// $030C14: move.b $98(a6), $70(a6)
pub fn i_030c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C1A;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad4, v3 as u8);
    20
}
/// $030C1A: move.w #$1d6, d1
pub fn i_030c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C1E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030C1E: jsr $236e.l
pub fn i_030c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30C24);
    cpu.pc = 0x236E;
    16
}
/// $030C24: move.w #$d080, $38(a6)
pub fn i_030c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C2A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD080;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030C2A: move.b $70(a6), d0
pub fn i_030c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C2E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $030C2E: andi.w #$ff, d0
pub fn i_030c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C32;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030C32: lsl.w #$3, d0
pub fn i_030c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C34;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $030C34: move.w #$14, d1
pub fn i_030c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C38;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030C38: jsr $13c0e.l
pub fn i_030c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30C3E);
    cpu.pc = 0x13C0E;
    16
}
/// $030C3E: add.w d1, $22(a6)
pub fn i_030c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C42;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $030C42: add.w d2, $24(a6)
pub fn i_030c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C46;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $030C46: lea.l $29d7d2.l, a0
pub fn i_030c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C4C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29D7D2;
    20
}
/// $030C4C: jsr $28cd4.l
pub fn i_030c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30C52);
    cpu.pc = 0x28CD4;
    16
}
/// $030C52: lea.l $30c58(pc), a1
pub fn i_030c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30C58;
    16
}
/// $030C56: move.l a1, (a6)
pub fn i_030c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030C58: jsr $2783a.l
pub fn i_030c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30C5E);
    cpu.pc = 0x2783A;
    16
}
/// $030C5E: jsr $28d70.l
pub fn i_030c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30C64);
    cpu.pc = 0x28D70;
    16
}
/// $030C64: bcc.w $30c6e
pub fn i_030c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C68;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x30C6E; return 10; }
    8
}
/// $030C68: jmp $518.l
pub fn i_030c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C6E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $030C6E: rts
pub fn i_030c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EnemyShot_Bounce_030c70  ($030C70, 140 B, player/tables_fx_030696.s)
/// $030C70: move.w #$6, d1
pub fn i_030c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C74;
    cpu.ir = 0x323C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030C74: jsr $236e.l
pub fn i_030c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30C7A);
    cpu.pc = 0x236E;
    16
}
/// $030C7A: move.w #$400, $2a(a6)
pub fn i_030c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030C80: move.w #$ff00, $2e(a6)
pub fn i_030c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C86;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030C86: jsr $5e9b6.l
pub fn i_030c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30C8C);
    cpu.pc = 0x5E9B6;
    16
}
/// $030C8C: move.w d0, d1
pub fn i_030c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C8E;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030C8E: andi.w #$140, d1
pub fn i_030c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C92;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x140) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030C92: addi.w #$80, d1
pub fn i_030c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C96;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030C96: neg.w d1
pub fn i_030c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C98;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $030C98: asl.w #$1, d1
pub fn i_030c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C9A;
    cpu.ir = 0xE341;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $030C9A: move.w d1, $28(a6)
pub fn i_030c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30C9E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030C9E: andi.w #$3, d0
pub fn i_030c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CA2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $030CA2: movea.l #$29d844, a0
pub fn i_030ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CA8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x29D844;
    cpu.a[0] = v1;
    4
}
/// $030CA8: lsl.w #$2, d0
pub fn i_030ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CAA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $030CAA: movea.l (a0, d0.w), a0
pub fn i_030caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CAE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $030CAE: cmpa.l #$ffffffff, a0
pub fn i_030cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CB4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $030CB4: beq.w $30cbe
pub fn i_030cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CB8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x30CBE; return 10; }
    8
}
/// $030CB8: jsr $28cd4.l
pub fn i_030cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30CBE);
    cpu.pc = 0x28CD4;
    16
}
/// $030CBE: lea.l $30cc4(pc), a1
pub fn i_030cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30CC4;
    16
}
/// $030CC2: move.l a1, (a6)
pub fn i_030cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030CC4: move.w $2a(a6), d0
pub fn i_030cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CC8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030CC8: move.w #$400, d1
pub fn i_030cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CCC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030CCC: jsr $267f4.l
pub fn i_030ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30CD2);
    cpu.pc = 0x267F4;
    16
}
/// $030CD2: move.w d0, $2a(a6)
pub fn i_030cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CD6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030CD6: jsr $27d50.l
pub fn i_030cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30CDC);
    cpu.pc = 0x27D50;
    16
}
/// $030CDC: bcc.w $30ce6
pub fn i_030cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x30CE6; return 10; }
    8
}
/// $030CE0: lea.l $30d04(pc), a1
pub fn i_030ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30D04;
    16
}
/// $030CE4: move.l a1, (a6)
pub fn i_030ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030CE6: jsr $28d70.l
pub fn i_030ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30CEC);
    cpu.pc = 0x28D70;
    16
}
/// $030CEC: movea.l #$ffffffff, a0
pub fn i_030cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CF2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $030CF2: jsr $5dd56.l
pub fn i_030cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30CF8);
    cpu.pc = 0x5DD56;
    16
}
/// $030CF8: bcc.w $30d02
pub fn i_030cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30CFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x30D02; return 10; }
    8
}
// ===== SetTaskHandler_030cfc  ($030CFC, 8 B, kernel/task_handlers.c)
/// $030CFC: lea.l $30d74(pc), a1
pub fn i_030cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30D74;
    16
}
/// $030D00: move.l a1, (a6)
pub fn i_030d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030D02: rts
pub fn i_030d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EnemyShot_BounceB_030d04  ($030D04, 80 B, player/tables_fx_030696.s)
/// $030D04: move.w #$4c4, $2a(a6)
pub fn i_030d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4C4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030D0A: move.w #$ff86, $2e(a6)
pub fn i_030d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D10;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF86;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030D10: move.b #$fe, $59(a6)
pub fn i_030d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D16;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $030D16: lea.l $30d1c(pc), a1
pub fn i_030d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30D1C;
    16
}
/// $030D1A: move.l a1, (a6)
pub fn i_030d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D1C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030D1C: move.w $2a(a6), d0
pub fn i_030d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D20;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $030D20: move.w #$400, d1
pub fn i_030d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D24;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $030D24: jsr $267f4.l
pub fn i_030d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30D2A);
    cpu.pc = 0x267F4;
    16
}
/// $030D2A: move.w d0, $2a(a6)
pub fn i_030d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D2E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $030D2E: jsr $27d50.l
pub fn i_030d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30D34);
    cpu.pc = 0x27D50;
    16
}
/// $030D34: bcc.w $30d3e
pub fn i_030d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D38;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x30D3E; return 10; }
    8
}
/// $030D38: lea.l $30d74(pc), a1
pub fn i_030d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30D74;
    16
}
/// $030D3C: move.l a1, (a6)
pub fn i_030d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030D3E: jsr $28d70.l
pub fn i_030d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30D44);
    cpu.pc = 0x28D70;
    16
}
/// $030D44: movea.l #$ffffffff, a0
pub fn i_030d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D4A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $030D4A: jsr $5dd56.l
pub fn i_030d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30D50);
    cpu.pc = 0x5DD56;
    16
}
/// $030D50: bcc.w $30d5a
pub fn i_030d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D54;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x30D5A; return 10; }
    8
}
// ===== SetTaskHandler_030d54  ($030D54, 8 B, kernel/task_handlers.c)
/// $030D54: lea.l $30d74(pc), a1
pub fn i_030d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30D74;
    16
}
/// $030D58: move.l a1, (a6)
pub fn i_030d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030D5A: rts
pub fn i_030d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EnemyShot_ExplodeAir_030d5c  ($030D5C, 16 B, player/tables_fx_030696.s)
/// $030D5C: lea.l $31cca(pc), a1
pub fn i_030d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31CCA;
    16
}
/// $030D60: jsr $6fe.l
pub fn i_030d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30D66);
    cpu.pc = 0x6FE;
    16
}
/// $030D66: jsr $5dd02.l
pub fn i_030d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x30D6C);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_030d6c  ($030D6C, 8 B, kernel/task_handlers.c)
/// $030D6C: lea.l $30d74(pc), a1
pub fn i_030d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x30D74;
    16
}
/// $030D70: move.l a1, (a6)
pub fn i_030d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $030D72: rts
pub fn i_030d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_030d74  ($030D74, 8 B, kernel/jmp_to_scheduler.c)
/// $030D74: jmp $518.l
pub fn i_030d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x30D7A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== VehicleLaunch_Init_0311c0  ($0311C0, 980 B, player/tables_fx_030696.s)
/// $0311C0: lea.l $30e20(pc), a0
pub fn i_0311c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311C4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x30E20;
    16
}
/// $0311C4: move.l a0, $4c(a6)
pub fn i_0311c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311C8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0311C8: jsr $283ca.l
pub fn i_0311c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x311CE);
    cpu.pc = 0x283CA;
    16
}
/// $0311CE: movea.l $c(a6), a0
pub fn i_0311ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311D2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0311D2: move.w $94(a0), d3
pub fn i_0311d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311D6;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0311D6: mulu.w #$a, d3
pub fn i_0311d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311DA;
    cpu.ir = 0xC6FC;
    let v1: u32 = 0xA;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[3] & 0xFFFF);
    cpu.d[3] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0311DA: lea.l $3115c(pc), a1
pub fn i_0311da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3115C;
    16
}
/// $0311DE: lea.l $29fa8a.l, a0
pub fn i_0311de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29FA8A;
    20
}
/// $0311E4: jsr $28cd4.l
pub fn i_0311e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x311EA);
    cpu.pc = 0x28CD4;
    16
}
/// $0311EA: move.w $4(a1, d3.w), d1
pub fn i_0311ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311EE;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0311EE: add.w d1, $22(a6)
pub fn i_0311ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311F2;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0311F2: move.w $6(a1, d3.w), d2
pub fn i_0311f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311F6;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0311F6: add.w d2, $24(a6)
pub fn i_0311f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311FA;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0311FA: move.w $8(a1, d3.w), d0
pub fn i_0311fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x311FE;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x8).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0311FE: move.w d0, $34(a6)
pub fn i_0311fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31202;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031202: lsr.w #$3, d0
pub fn i_031202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31204;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $031204: move.w d0, $30(a6)
pub fn i_031204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31208;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031208: move.w #$4400, $72(a6)
pub fn i_031208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3120E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03120E: move.w #$600, $36(a6)
pub fn i_03120e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31214;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031214: move.w #$700, d1
pub fn i_031214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31218;
    cpu.ir = 0x323C;
    let v1: u32 = 0x700;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031218: move.w $34(a6), d0
pub fn i_031218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3121C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03121C: jsr $13c0e.l
pub fn i_03121c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31222;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31222);
    cpu.pc = 0x13C0E;
    16
}
/// $031222: move.w d1, $28(a6)
pub fn i_031222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31226;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031226: move.w d2, $2a(a6)
pub fn i_031226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3122A;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03122A: bra.w $314f0
pub fn i_03122a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3122E;
    cpu.ir = 0x6000;
    cpu.pc = 0x314F0;
    10
}
/// $0312B2: lea.l $30e20(pc), a0
pub fn i_0312b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312B6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x30E20;
    16
}
/// $0312B6: move.l a0, $4c(a6)
pub fn i_0312b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312BA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0312BA: jsr $283ca.l
pub fn i_0312ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x312C0);
    cpu.pc = 0x283CA;
    16
}
/// $0312C0: lea.l $29fa8a.l, a0
pub fn i_0312c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29FA8A;
    20
}
/// $0312C6: jsr $28cd4.l
pub fn i_0312c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x312CC);
    cpu.pc = 0x28CD4;
    16
}
/// $0312CC: move.b #$ff, $20(a6)
pub fn i_0312cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312D2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0312D2: move.l #$31b78, $74(a6)
pub fn i_0312d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312DA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x31B78;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0312DA: move.w #$600, $36(a6)
pub fn i_0312da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0312E0: move.w #$1000, $72(a6)
pub fn i_0312e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0312E6: movea.l $c(a6), a0
pub fn i_0312e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312EA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0312EA: moveq #$0, d0
pub fn i_0312ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312EC;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0312EC: move.w $80(a0), d0
pub fn i_0312ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312F0;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0312F0: bgt.w $312fc
pub fn i_0312f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312F4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x312FC; return 10; }
    8
}
/// $0312F4: addi.w #$10, d0
pub fn i_0312f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312F8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0312F8: bra.w $31300
pub fn i_0312f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x312FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x31300;
    10
}
/// $0312FC: addi.w #$8, d0
pub fn i_0312fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31300;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031300: move.b $106f2a.l, d1
pub fn i_031300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31306;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106F2A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $031306: cmpi.b #$3, d1
pub fn i_031306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3130A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $03130A: bne.w $31312
pub fn i_03130a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3130E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x31312; return 10; }
    8
}
/// $03130E: addi.w #$0, d0
pub fn i_03130e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31312;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031312: andi.w #$ff, d0
pub fn i_031312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31316;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031316: move.w d0, $34(a6)
pub fn i_031316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3131A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03131A: move.b $106f2a.l, d1
pub fn i_03131a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31320;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106F2A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $031320: cmpi.b #$2, d1
pub fn i_031320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31324;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $031324: bne.w $313a6
pub fn i_031324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31328;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x313A6; return 10; }
    8
}
/// $031328: bra.w $313a6
pub fn i_031328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3132C;
    cpu.ir = 0x6000;
    cpu.pc = 0x313A6;
    10
}
/// $0313A6: cmpi.b #$5, d1
pub fn i_0313a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313AA;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $0313AA: bne.w $31428
pub fn i_0313aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x31428; return 10; }
    8
}
/// $0313AE: movem.l a6, -(a7)
pub fn i_0313ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313B2;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0313B2: movea.l $c(a6), a6
pub fn i_0313b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313B6;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $0313B6: jsr $5cd90.l
pub fn i_0313b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x313BC);
    cpu.pc = 0x5CD90;
    16
}
/// $0313BC: movem.l (a7)+, a6
pub fn i_0313bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313C0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0313C0: bcc.w $31428
pub fn i_0313c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x31428; return 10; }
    8
}
/// $0313C4: jsr $323b4(pc)
pub fn i_0313c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x313C8);
    cpu.pc = 0x323B4;
    16
}
/// $0313C8: cmpi.b #$4, d0
pub fn i_0313c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313CC;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $0313CC: bne.w $313e4
pub fn i_0313cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x313E4; return 10; }
    8
}
/// $0313D0: move.w #$c00, $36(a6)
pub fn i_0313d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0313D6: move.w #$7400, $72(a6)
pub fn i_0313d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313DC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0313DC: move.w #$fff8, d2
pub fn i_0313dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313E0;
    cpu.ir = 0x343C;
    let v1: u32 = 0xFFF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0313E0: bra.w $31416
pub fn i_0313e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x31416;
    10
}
/// $0313E4: cmpi.b #$3, d0
pub fn i_0313e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313E8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $0313E8: bne.w $313fe
pub fn i_0313e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313EC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x313FE; return 10; }
    8
}
/// $0313EC: move.w #$c00, $36(a6)
pub fn i_0313ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0313F2: move.w #$7400, $72(a6)
pub fn i_0313f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0313F8: clr.w d2
pub fn i_0313f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313FA;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0313FA: bra.w $31416
pub fn i_0313fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x313FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x31416;
    10
}
/// $0313FE: cmpi.b #$2, d0
pub fn i_0313fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31402;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $031402: bne.w $31424
pub fn i_031402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31406;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x31424; return 10; }
    8
}
/// $031406: move.w #$c00, $36(a6)
pub fn i_031406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3140C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03140C: move.w #$7400, $72(a6)
pub fn i_03140c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31412;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031412: move.w #$8, d2
pub fn i_031412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31416;
    cpu.ir = 0x343C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031416: move.w $34(a6), d0
pub fn i_031416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3141A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03141A: sub.w d2, d0
pub fn i_03141a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3141C;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $03141C: andi.w #$f8, d0
pub fn i_03141c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31420;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031420: move.w d0, $34(a6)
pub fn i_031420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31424;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031424: bra.w $314ac
pub fn i_031424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31428;
    cpu.ir = 0x6000;
    cpu.pc = 0x314AC;
    10
}
/// $031428: cmpi.b #$3, d1
pub fn i_031428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3142C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $03142C: bne.w $314ac
pub fn i_03142c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31430;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x314AC; return 10; }
    8
}
/// $031430: movem.l a6, -(a7)
pub fn i_031430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31434;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $031434: movea.l $c(a6), a6
pub fn i_031434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31438;
    cpu.ir = 0x2C6E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $031438: jsr $5cd90.l
pub fn i_031438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3143E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3143E);
    cpu.pc = 0x5CD90;
    16
}
/// $03143E: movem.l (a7)+, a6
pub fn i_03143e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31442;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $031442: bcc.w $314ac
pub fn i_031442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31446;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x314AC; return 10; }
    8
}
/// $031446: jsr $323b4(pc)
pub fn i_031446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3144A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3144A);
    cpu.pc = 0x323B4;
    16
}
/// $03144A: bra.w $31452
pub fn i_03144a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3144E;
    cpu.ir = 0x6000;
    cpu.pc = 0x31452;
    10
}
/// $031452: movem.l a6, -(a7)
pub fn i_031452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31456;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $031456: movea.l $c(a6), a1
pub fn i_031456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3145A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03145A: movea.l a1, a6
pub fn i_03145a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3145C;
    cpu.ir = 0x2C49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $03145C: jsr $2a328.l
pub fn i_03145c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31462;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31462);
    cpu.pc = 0x2A328;
    16
}
/// $031462: movem.l (a7)+, a6
pub fn i_031462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31466;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $031466: bcc.w $314a8
pub fn i_031466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3146A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x314A8; return 10; }
    8
}
/// $03146A: move.w #$c00, $36(a6)
pub fn i_03146a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31470;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031470: move.w #$26aa, $72(a6)
pub fn i_031470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31476;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x26AA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031476: move.w $34(a6), d0
pub fn i_031476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3147A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03147A: subq.w #$8, d0
pub fn i_03147a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3147C;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03147C: andi.w #$f8, d0
pub fn i_03147c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31480;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031480: move.w d0, $34(a6)
pub fn i_031480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31484;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031484: move.b $106f2a.l, d1
pub fn i_031484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3148A;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106F2A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $03148A: cmpi.b #$3, d1
pub fn i_03148a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3148E;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $03148E: bne.w $31496
pub fn i_03148e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31492;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x31496; return 10; }
    8
}
/// $031492: clr.b $20(a6)
pub fn i_031492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31496;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $031496: move.b $106f2a.l, d1
pub fn i_031496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3149C;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106F2A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $03149C: cmpi.b #$2, d1
pub fn i_03149c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314A0;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $0314A0: bne.w $314a8
pub fn i_0314a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x314A8; return 10; }
    8
}
/// $0314A4: clr.b $20(a6)
pub fn i_0314a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314A8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0314A8: bra.w $314ac
pub fn i_0314a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x314AC;
    10
}
/// $0314AC: movea.l $c(a6), a1
pub fn i_0314ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314B0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0314B0: movea.l $70(a1), a1
pub fn i_0314b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314B4;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0314B4: movea.l (a1), a1
pub fn i_0314b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314B6;
    cpu.ir = 0x2251;
    let v2 = bus.read32(cpu.a[1]);
    cpu.a[1] = v2;
    12
}
/// $0314B6: move.w $8(a1), d1
pub fn i_0314b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314BA;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0314BA: move.w $a(a1), d2
pub fn i_0314ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314BE;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0314BE: add.w d1, $22(a6)
pub fn i_0314be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314C2;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0314C2: addi.w #$0, $22(a6)
pub fn i_0314c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314C8;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0314C8: add.w d2, $24(a6)
pub fn i_0314c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314CC;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0314CC: addq.w #$8, $24(a6)
pub fn i_0314cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314D0;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0314D0: move.w $34(a6), d1
pub fn i_0314d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314D4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0314D4: lsr.w #$3, d1
pub fn i_0314d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314D6;
    cpu.ir = 0xE649;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0314D6: move.w d1, $30(a6)
pub fn i_0314d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314DA;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0314DA: move.w #$700, d1
pub fn i_0314da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314DE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x700;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0314DE: move.w $34(a6), d0
pub fn i_0314de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314E2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0314E2: jsr $13c0e.l
pub fn i_0314e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x314E8);
    cpu.pc = 0x13C0E;
    16
}
/// $0314E8: move.w d1, $28(a6)
pub fn i_0314e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314EC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0314EC: move.w d2, $2a(a6)
pub fn i_0314ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314F0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0314F0: move.w #$1084, d0
pub fn i_0314f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314F4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1084;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0314F4: jsr $2352.l
pub fn i_0314f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x314FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x314FA);
    cpu.pc = 0x2352;
    16
}
/// $0314FA: move.w #$d000, $38(a6)
pub fn i_0314fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31500;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031500: lea.l $77ab6.l, a1
pub fn i_031500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31506;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77AB6;
    20
}
/// $031506: jsr $4ae.l
pub fn i_031506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3150C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3150C);
    cpu.pc = 0x4AE;
    16
}
/// $03150C: jsr $5dd02.l
pub fn i_03150c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31512);
    cpu.pc = 0x5DD02;
    16
}
/// $031512: move.w #$58, d1
pub fn i_031512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31516;
    cpu.ir = 0x323C;
    let v1: u32 = 0x58;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031516: jsr $236e.l
pub fn i_031516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3151C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3151C);
    cpu.pc = 0x236E;
    16
}
/// $03151C: clr.w $70(a6)
pub fn i_03151c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31520;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $031520: movea.l $c(a6), a0
pub fn i_031520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31524;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $031524: addi.b #$1, $8f(a0)
pub fn i_031524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3152A;
    cpu.ir = 0x628;
    let ad1 = cpu.a[0].wrapping_add(0x8F);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $03152A: andi.w #$ffe3, $38(a6)
pub fn i_03152a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31530;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $031530: ori.w #$0, $38(a6)
pub fn i_031530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31536;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $031536: jsr $283ca.l
pub fn i_031536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3153C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3153C);
    cpu.pc = 0x283CA;
    16
}
/// $03153C: lea.l $31542(pc), a1
pub fn i_03153c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31540;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31542;
    16
}
/// $031540: move.l a1, (a6)
pub fn i_031540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31542;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031542: move.w $72(a6), d0
pub fn i_031542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31546;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031546: move.w $70(a6), d1
pub fn i_031546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3154A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03154A: cmp.w d0, d1
pub fn i_03154a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3154C;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $03154C: bcc.w $3156c
pub fn i_03154c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31550;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3156C; return 10; }
    8
}
/// $031550: move.w $34(a6), d0
pub fn i_031550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31554;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031554: move.w $36(a6), d1
pub fn i_031554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31558;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031558: jsr $13c0e.l
pub fn i_031558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3155E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3155E);
    cpu.pc = 0x13C0E;
    16
}
/// $03155E: move.w d1, $28(a6)
pub fn i_03155e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31562;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031562: move.w d2, $2a(a6)
pub fn i_031562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31566;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031566: lea.l $3156c(pc), a1
pub fn i_031566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3156A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3156C;
    16
}
/// $03156A: move.l a1, (a6)
pub fn i_03156a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3156C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03156C: move.w $28(a6), d0
pub fn i_03156c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31570;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031570: add.w d0, $70(a6)
pub fn i_031570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31574;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031574: move.w $72(a6), d0
pub fn i_031574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31578;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031578: move.w $70(a6), d1
pub fn i_031578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3157C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03157C: cmp.w d0, d1
pub fn i_03157c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3157E;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $03157E: bcs.w $315a2
pub fn i_03157e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31582;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x315A2; return 10; }
    8
}
/// $031582: lea.l $31594(pc), a1
pub fn i_031582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31586;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31594;
    16
}
/// $031586: move.l a1, (a6)
pub fn i_031586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31588;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031588: move.w $36(a6), d0
pub fn i_031588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3158C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03158C: asr.w #$4, d0
pub fn i_03158c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3158E;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $03158E: neg.w d0
pub fn i_03158e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31590;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $031590: move.w d0, $2e(a6)
pub fn i_031590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31594;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== VehicleLaunch_Fall_031594  ($031594, 238 B, player/tables_fx_030696.s)
/// $031594: tst.w $2a(a6)
pub fn i_031594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31598;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $031598: bgt.w $315a2
pub fn i_031598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3159C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x315A2; return 10; }
    8
}
/// $03159C: lea.l $3168a(pc), a1
pub fn i_03159c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3168A;
    16
}
/// $0315A0: move.l a1, (a6)
pub fn i_0315a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0315A2: move.w $28(a6), d0
pub fn i_0315a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315A6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0315A6: move.w $2a(a6), d1
pub fn i_0315a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315AA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0315AA: asr.w #$4, d0
pub fn i_0315aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315AC;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0315AC: asr.w #$4, d1
pub fn i_0315ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315AE;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0315AE: jsr $5e018.l
pub fn i_0315ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x315B4);
    cpu.pc = 0x5E018;
    16
}
/// $0315B4: lsr.w #$3, d0
pub fn i_0315b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315B6;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0315B6: move.w d0, $30(a6)
pub fn i_0315b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315BA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0315BA: cmpi.b #$ff, $20(a6)
pub fn i_0315ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315C0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0315C0: beq.w $3161e
pub fn i_0315c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3161E; return 10; }
    8
}
/// $0315C4: addq.b #$1, $20(a6)
pub fn i_0315c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315C8;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0315C8: move.b $20(a6), d0
pub fn i_0315c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315CC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0315CC: cmpi.b #$3, $106f2a.l
pub fn i_0315cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315D4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $0315D4: bne.w $315e0
pub fn i_0315d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x315E0; return 10; }
    8
}
/// $0315D8: andi.b #$0, d0
pub fn i_0315d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315DC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0315DC: bra.w $315f8
pub fn i_0315dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x315F8;
    10
}
/// $0315E0: cmpi.b #$2, $106f2a.l
pub fn i_0315e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315E8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $0315E8: bne.w $315f4
pub fn i_0315e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315EC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x315F4; return 10; }
    8
}
/// $0315EC: andi.b #$0, d0
pub fn i_0315ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315F0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0315F0: bra.w $315f8
pub fn i_0315f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x315F8;
    10
}
/// $0315F4: andi.b #$b, d0
pub fn i_0315f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315F8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xB) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0315F8: bne.w $3161e
pub fn i_0315f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x315FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3161E; return 10; }
    8
}
/// $0315FC: movea.l $74(a6), a1
pub fn i_0315fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31600;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $031600: jsr $4ae.l
pub fn i_031600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31606;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31606);
    cpu.pc = 0x4AE;
    16
}
/// $031606: jsr $5dd02.l
pub fn i_031606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3160C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3160C);
    cpu.pc = 0x5DD02;
    16
}
/// $03160C: move.w $34(a6), $34(a0)
pub fn i_03160c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31612;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $031612: move.w $30(a6), $30(a0)
pub fn i_031612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31618;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad4, v3 as u16);
    20
}
/// $031618: move.w $38(a6), $38(a0)
pub fn i_031618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3161E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03161E: bset.b #$6, $13(a6)
pub fn i_03161e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31624;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $031624: jsr $27cee.l
pub fn i_031624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3162A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3162A);
    cpu.pc = 0x27CEE;
    16
}
/// $03162A: bcc.w $31634
pub fn i_03162a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3162E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x31634; return 10; }
    8
}
/// $03162E: lea.l $318b4(pc), a1
pub fn i_03162e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31632;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x318B4;
    16
}
/// $031632: move.l a1, (a6)
pub fn i_031632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31634;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031634: jsr $28d70.l
pub fn i_031634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3163A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3163A);
    cpu.pc = 0x28D70;
    16
}
/// $03163A: jsr $283d8.l
pub fn i_03163a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31640;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31640);
    cpu.pc = 0x283D8;
    16
}
/// $031640: btst.b #$5, $5a(a6)
pub fn i_031640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31646;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $031646: beq.w $31650
pub fn i_031646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3164A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x31650; return 10; }
    8
}
/// $03164A: lea.l $317d2(pc), a1
pub fn i_03164a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3164E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x317D2;
    16
}
/// $03164E: move.l a1, (a6)
pub fn i_03164e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31650;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031650: btst.b #$1, $13(a6)
pub fn i_031650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31656;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $031656: beq.w $3166e
pub fn i_031656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3165A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3166E; return 10; }
    8
}
/// $03165A: lea.l $318b4(pc), a1
pub fn i_03165a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3165E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x318B4;
    16
}
/// $03165E: move.l a1, (a6)
pub fn i_03165e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31660;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031660: cmpi.b #$6, d1
pub fn i_031660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31664;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x6, Size::Byte, false);
    8
}
/// $031664: bne.w $3166e
pub fn i_031664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31668;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3166E; return 10; }
    8
}
/// $031668: lea.l $318b8(pc), a1
pub fn i_031668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3166C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x318B8;
    16
}
/// $03166C: move.l a1, (a6)
pub fn i_03166c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3166E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03166E: movea.l #$ffffffff, a0
pub fn i_03166e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31674;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $031674: lea.l $31154(pc), a0
pub fn i_031674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31678;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x31154;
    16
}
/// $031678: jsr $5dd56.l
pub fn i_031678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3167E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3167E);
    cpu.pc = 0x5DD56;
    16
}
/// $03167E: bcc.w $31688
pub fn i_03167e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31682;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x31688; return 10; }
    8
}
// ===== SetTaskHandler_031682  ($031682, 8 B, kernel/task_handlers.c)
/// $031682: lea.l $318d4(pc), a1
pub fn i_031682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31686;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x318D4;
    16
}
/// $031686: move.l a1, (a6)
pub fn i_031686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31688;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031688: rts
pub fn i_031688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3168A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VehicleLaunch_Glide_03168a  ($03168A, 320 B, player/tables_fx_030696.s)
/// $03168A: cmpi.w #$7400, $72(a6)
pub fn i_03168a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31690;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7400, Size::Word, false);
    16
}
/// $031690: bne.w $316ac
pub fn i_031690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31694;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x316AC; return 10; }
    8
}
/// $031694: move.w #$600, $36(a6)
pub fn i_031694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3169A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03169A: move.w $34(a6), d0
pub fn i_03169a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3169E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03169E: move.w $36(a6), d1
pub fn i_03169e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316A2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0316A2: jsr $13c0e.l
pub fn i_0316a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x316A8);
    cpu.pc = 0x13C0E;
    16
}
/// $0316A8: move.w d1, $28(a6)
pub fn i_0316a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316AC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0316AC: move.w #$ffe0, $2c(a6)
pub fn i_0316ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316B2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0316B2: move.w #$feea, $2e(a6)
pub fn i_0316b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFEEA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0316B8: lea.l $316be(pc), a1
pub fn i_0316b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x316BE;
    16
}
/// $0316BC: move.l a1, (a6)
pub fn i_0316bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0316BE: cmpi.b #$ff, $20(a6)
pub fn i_0316be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316C4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0316C4: beq.w $3171c
pub fn i_0316c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3171C; return 10; }
    8
}
/// $0316C8: addq.b #$1, $20(a6)
pub fn i_0316c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316CC;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0316CC: move.b $20(a6), d0
pub fn i_0316cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316D0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0316D0: cmpi.b #$3, $106f2a.l
pub fn i_0316d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316D8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $0316D8: bne.w $316e4
pub fn i_0316d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x316E4; return 10; }
    8
}
/// $0316DC: andi.b #$0, d0
pub fn i_0316dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316E0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0316E0: bra.w $316fc
pub fn i_0316e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x316FC;
    10
}
/// $0316E4: cmpi.b #$2, $106f2a.l
pub fn i_0316e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316EC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    20
}
/// $0316EC: bne.w $316f8
pub fn i_0316ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x316F8; return 10; }
    8
}
/// $0316F0: andi.b #$0, d0
pub fn i_0316f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316F4;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0316F4: bra.w $316fc
pub fn i_0316f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x316FC;
    10
}
/// $0316F8: andi.b #$b, d0
pub fn i_0316f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x316FC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xB) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0316FC: bne.w $3171c
pub fn i_0316fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31700;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3171C; return 10; }
    8
}
/// $031700: movea.l $74(a6), a1
pub fn i_031700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31704;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $031704: jsr $4ae.l
pub fn i_031704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3170A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3170A);
    cpu.pc = 0x4AE;
    16
}
/// $03170A: jsr $5dd02.l
pub fn i_03170a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31710);
    cpu.pc = 0x5DD02;
    16
}
/// $031710: move.w $30(a6), $30(a0)
pub fn i_031710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31716;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x30);
    bus.write16(ad4, v3 as u16);
    20
}
/// $031716: move.w $38(a6), $38(a0)
pub fn i_031716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3171C;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03171C: move.w $2a(a6), d0
pub fn i_03171c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31720;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031720: move.w #$700, d1
pub fn i_031720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31724;
    cpu.ir = 0x323C;
    let v1: u32 = 0x700;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031724: jsr $267f4.l
pub fn i_031724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3172A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3172A);
    cpu.pc = 0x267F4;
    16
}
/// $03172A: move.w d0, $2a(a6)
pub fn i_03172a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3172E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03172E: tst.w $28(a6)
pub fn i_03172e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31732;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $031732: bgt.w $3173e
pub fn i_031732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31736;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3173E; return 10; }
    8
}
/// $031736: clr.w $28(a6)
pub fn i_031736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3173A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03173A: clr.w $2c(a6)
pub fn i_03173a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3173E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03173E: bset.b #$6, $13(a6)
pub fn i_03173e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31744;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $031744: jsr $27d50.l
pub fn i_031744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3174A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3174A);
    cpu.pc = 0x27D50;
    16
}
/// $03174A: bcc.w $31764
pub fn i_03174a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3174E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x31764; return 10; }
    8
}
/// $03174E: lea.l $318b4(pc), a1
pub fn i_03174e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31752;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x318B4;
    16
}
/// $031752: move.l a1, (a6)
pub fn i_031752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31754;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031754: jsr $27eba.l
pub fn i_031754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3175A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3175A);
    cpu.pc = 0x27EBA;
    16
}
/// $03175A: bcs.w $31764
pub fn i_03175a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3175E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x31764; return 10; }
    8
}
/// $03175E: lea.l $317d2(pc), a1
pub fn i_03175e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31762;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x317D2;
    16
}
/// $031762: move.l a1, (a6)
pub fn i_031762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31764;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031764: move.w $28(a6), d0
pub fn i_031764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31768;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031768: move.w $2a(a6), d1
pub fn i_031768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3176C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03176C: asr.w #$4, d0
pub fn i_03176c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3176E;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $03176E: asr.w #$4, d1
pub fn i_03176e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31770;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $031770: jsr $5e018.l
pub fn i_031770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31776;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31776);
    cpu.pc = 0x5E018;
    16
}
/// $031776: lsr.w #$3, d0
pub fn i_031776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31778;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $031778: move.w d0, $30(a6)
pub fn i_031778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3177C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03177C: jsr $28d70.l
pub fn i_03177c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31782);
    cpu.pc = 0x28D70;
    16
}
/// $031782: jsr $283d8.l
pub fn i_031782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31788);
    cpu.pc = 0x283D8;
    16
}
/// $031788: btst.b #$5, $5a(a6)
pub fn i_031788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3178E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03178E: beq.w $31798
pub fn i_03178e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31792;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x31798; return 10; }
    8
}
/// $031792: lea.l $317d2(pc), a1
pub fn i_031792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31796;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x317D2;
    16
}
/// $031796: move.l a1, (a6)
pub fn i_031796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31798;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031798: btst.b #$1, $13(a6)
pub fn i_031798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3179E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03179E: beq.w $317b6
pub fn i_03179e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x317B6; return 10; }
    8
}
/// $0317A2: lea.l $318b4(pc), a1
pub fn i_0317a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x318B4;
    16
}
/// $0317A6: move.l a1, (a6)
pub fn i_0317a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0317A8: cmpi.b #$6, d1
pub fn i_0317a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317AC;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x6, Size::Byte, false);
    8
}
/// $0317AC: bne.w $317b6
pub fn i_0317ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317B0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x317B6; return 10; }
    8
}
/// $0317B0: lea.l $318b8(pc), a1
pub fn i_0317b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x318B8;
    16
}
/// $0317B4: move.l a1, (a6)
pub fn i_0317b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0317B6: movea.l #$ffffffff, a0
pub fn i_0317b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317BC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0317BC: lea.l $31154(pc), a0
pub fn i_0317bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317C0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x31154;
    16
}
/// $0317C0: jsr $5dd56.l
pub fn i_0317c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x317C6);
    cpu.pc = 0x5DD56;
    16
}
/// $0317C6: bcc.w $317d0
pub fn i_0317c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x317D0; return 10; }
    8
}
// ===== SetTaskHandler_0317ca  ($0317CA, 8 B, kernel/task_handlers.c)
/// $0317CA: lea.l $318d4(pc), a1
pub fn i_0317ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x318D4;
    16
}
/// $0317CE: move.l a1, (a6)
pub fn i_0317ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0317D0: rts
pub fn i_0317d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VehicleLaunch_Crash_0317d2  ($0317D2, 76 B, player/tables_fx_030696.s)
/// $0317D2: move.b #$1, $10a2d1.l
pub fn i_0317d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317DA;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $0317DA: move.w #$10f2, d0
pub fn i_0317da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10F2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0317DE: jsr $2352.l
pub fn i_0317de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x317E4);
    cpu.pc = 0x2352;
    16
}
/// $0317E4: move.w #$d000, $38(a6)
pub fn i_0317e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0317EA: jsr $434dc.l
pub fn i_0317ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x317F0);
    cpu.pc = 0x434DC;
    16
}
/// $0317F0: lea.l $ffff.w, a0
pub fn i_0317f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317F4;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0317F4: move.l a0, $4c(a6)
pub fn i_0317f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317F8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0317F8: jsr $283ca.l
pub fn i_0317f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x317FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x317FE);
    cpu.pc = 0x283CA;
    16
}
/// $0317FE: movea.l $c(a6), a0
pub fn i_0317fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31802;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $031802: subi.b #$1, $8f(a0)
pub fn i_031802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31808;
    cpu.ir = 0x428;
    let ad1 = cpu.a[0].wrapping_add(0x8F);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $031808: lea.l $31bd0(pc), a1
pub fn i_031808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3180C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31BD0;
    16
}
/// $03180C: move.l a1, (a6)
pub fn i_03180c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3180E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03180E: lea.l $31826(pc), a1
pub fn i_03180e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31812;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31826;
    16
}
/// $031812: jsr $4ae.l
pub fn i_031812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31818;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31818);
    cpu.pc = 0x4AE;
    16
}
/// $031818: jsr $5dd02.l
pub fn i_031818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3181E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3181E);
    cpu.pc = 0x5DD02;
    16
}
// ===== JsrAbsThunk_03181e  ($03181E, 8 B, thunks/jsr_abs_thunks.c)
/// $03181E: jsr $517fe.l
pub fn i_03181e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31824;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31824);
    cpu.pc = 0x517FE;
    16
}
/// $031824: rts
pub fn i_031824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31826;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VehicleLaunch_CrashA_031826  ($031826, 32 B, player/tables_fx_030696.s)
/// $031826: lea.l $3184e(pc), a1
pub fn i_031826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3182A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3184E;
    16
}
/// $03182A: move.l a1, (a6)
pub fn i_03182a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3182C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03182C: jsr $283ca.l
pub fn i_03182c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31832;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31832);
    cpu.pc = 0x283CA;
    16
}
/// $031832: lea.l $30f68(pc), a0
pub fn i_031832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31836;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x30F68;
    16
}
/// $031836: move.l a0, $4c(a6)
pub fn i_031836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3183A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03183A: jsr $283ca.l
pub fn i_03183a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31840;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31840);
    cpu.pc = 0x283CA;
    16
}
/// $031840: jsr $283d8.l
pub fn i_031840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31846);
    cpu.pc = 0x283D8;
    16
}
// ===== JsrAbsThunk_031846  ($031846, 8 B, thunks/jsr_abs_thunks.c)
/// $031846: jsr $5e4b2.l
pub fn i_031846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3184C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3184C);
    cpu.pc = 0x5E4B2;
    16
}
/// $03184C: rts
pub fn i_03184c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3184E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VehicleLaunch_CrashB_03184e  ($03184E, 32 B, player/tables_fx_030696.s)
/// $03184E: jsr $283ca.l
pub fn i_03184e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31854;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31854);
    cpu.pc = 0x283CA;
    16
}
/// $031854: lea.l $3100c(pc), a0
pub fn i_031854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31858;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3100C;
    16
}
/// $031858: move.l a0, $4c(a6)
pub fn i_031858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3185C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03185C: jsr $283ca.l
pub fn i_03185c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31862;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31862);
    cpu.pc = 0x283CA;
    16
}
/// $031862: jsr $283d8.l
pub fn i_031862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31868;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31868);
    cpu.pc = 0x283D8;
    16
}
/// $031868: lea.l $31876(pc), a1
pub fn i_031868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3186C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31876;
    16
}
/// $03186C: move.l a1, (a6)
pub fn i_03186c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3186E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_03186e  ($03186E, 8 B, thunks/jsr_abs_thunks.c)
/// $03186E: jsr $5e4b2.l
pub fn i_03186e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31874);
    cpu.pc = 0x5E4B2;
    16
}
/// $031874: rts
pub fn i_031874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31876;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VehicleLaunch_CrashC_031876  ($031876, 46 B, player/tables_fx_030696.s)
/// $031876: jsr $283ca.l
pub fn i_031876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3187C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3187C);
    cpu.pc = 0x283CA;
    16
}
/// $03187C: lea.l $310b0(pc), a0
pub fn i_03187c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31880;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x310B0;
    16
}
/// $031880: move.l a0, $4c(a6)
pub fn i_031880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31884;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $031884: jsr $283ca.l
pub fn i_031884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3188A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3188A);
    cpu.pc = 0x283CA;
    16
}
/// $03188A: jsr $283d8.l
pub fn i_03188a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31890;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31890);
    cpu.pc = 0x283D8;
    16
}
/// $031890: jsr $5e4b2.l
pub fn i_031890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31896;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31896);
    cpu.pc = 0x5E4B2;
    16
}
/// $031896: lea.l $ffff.w, a0
pub fn i_031896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3189A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $03189A: move.l a0, $4c(a6)
pub fn i_03189a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3189E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03189E: jsr $283ca.l
pub fn i_03189e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x318A4);
    cpu.pc = 0x283CA;
    16
}
// ===== SetTaskHandler_0318a4  ($0318A4, 8 B, kernel/task_handlers.c)
/// $0318A4: lea.l $318ac(pc), a1
pub fn i_0318a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x318AC;
    16
}
/// $0318A8: move.l a1, (a6)
pub fn i_0318a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0318AA: rts
pub fn i_0318aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_0318ac  ($0318AC, 8 B, kernel/jmp_to_scheduler.c)
/// $0318AC: jmp $518.l
pub fn i_0318ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318B2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== VehicleLaunch_JmpCrash_0318b4  ($0318B4, 4 B, player/tables_fx_030696.s)
/// $0318B4: jmp $317d2(pc)
pub fn i_0318b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318B8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x317D2;
    8
}
// ===== VehicleLaunch_Despawn_0318b8  ($0318B8, 20 B, player/tables_fx_030696.s)
/// $0318B8: jsr $5b6.l
pub fn i_0318b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x318BE);
    cpu.pc = 0x5B6;
    16
}
/// $0318BE: lea.l $ffff.w, a0
pub fn i_0318be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318C2;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0318C2: move.l a0, $4c(a6)
pub fn i_0318c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318C6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0318C6: jsr $283ca.l
pub fn i_0318c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x318CC);
    cpu.pc = 0x283CA;
    16
}
// ===== SetTaskHandler_0318cc  ($0318CC, 8 B, kernel/task_handlers.c)
/// $0318CC: lea.l $318d4(pc), a1
pub fn i_0318cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x318D4;
    16
}
/// $0318D0: move.l a1, (a6)
pub fn i_0318d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0318D2: rts
pub fn i_0318d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VehicleLaunch_ReleaseParent_0318d4  ($0318D4, 16 B, player/tables_fx_030696.s)
/// $0318D4: movea.l $c(a6), a0
pub fn i_0318d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318D8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0318D8: subi.b #$1, $8f(a0)
pub fn i_0318d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318DE;
    cpu.ir = 0x428;
    let ad1 = cpu.a[0].wrapping_add(0x8F);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $0318DE: jmp $518.l
pub fn i_0318de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318E4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== VehicleLaunch_Nop_0318e4  ($0318E4, 2 B, player/tables_fx_030696.s)
/// $0318E4: rts
pub fn i_0318e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VehicleLaunch_InitDrop_0318e6  ($0318E6, 226 B, player/tables_fx_030696.s)
/// $0318E6: lea.l $30e20(pc), a0
pub fn i_0318e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318EA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x30E20;
    16
}
/// $0318EA: move.l a0, $4c(a6)
pub fn i_0318ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318EE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0318EE: jsr $283ca.l
pub fn i_0318ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x318F4);
    cpu.pc = 0x283CA;
    16
}
/// $0318F4: movea.l $c(a6), a0
pub fn i_0318f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318F8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0318F8: move.w $94(a0), d3
pub fn i_0318f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x318FC;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0318FC: mulu.w #$a, d3
pub fn i_0318fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31900;
    cpu.ir = 0xC6FC;
    let v1: u32 = 0xA;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[3] & 0xFFFF);
    cpu.d[3] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $031900: lea.l $3115c(pc), a1
pub fn i_031900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31904;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3115C;
    16
}
/// $031904: lea.l $319d0(pc), a0
pub fn i_031904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31908;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x319D0;
    16
}
/// $031908: jsr $28cd4.l
pub fn i_031908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3190E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3190E);
    cpu.pc = 0x28CD4;
    16
}
/// $03190E: move.w $4(a1, d3.w), d1
pub fn i_03190e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31912;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $031912: add.w d1, $22(a6)
pub fn i_031912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31916;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031916: move.w $6(a1, d3.w), d2
pub fn i_031916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3191A;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $03191A: add.w d2, $24(a6)
pub fn i_03191a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3191E;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03191E: bra.w $3197c
pub fn i_03191e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31922;
    cpu.ir = 0x6000;
    cpu.pc = 0x3197C;
    10
}
/// $03197C: move.w #$1084, d0
pub fn i_03197c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31980;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1084;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031980: jsr $2352.l
pub fn i_031980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31986;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31986);
    cpu.pc = 0x2352;
    16
}
/// $031986: movea.l $c(a6), a0
pub fn i_031986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3198A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03198A: addi.b #$1, $8f(a0)
pub fn i_03198a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31990;
    cpu.ir = 0x628;
    let ad1 = cpu.a[0].wrapping_add(0x8F);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $031990: jsr $283ca.l
pub fn i_031990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31996);
    cpu.pc = 0x283CA;
    16
}
/// $031996: lea.l $3199c(pc), a1
pub fn i_031996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3199A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3199C;
    16
}
/// $03199A: move.l a1, (a6)
pub fn i_03199a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3199C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03199C: jsr $2783a.l
pub fn i_03199c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x319A2);
    cpu.pc = 0x2783A;
    16
}
/// $0319A2: jsr $28d70.l
pub fn i_0319a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x319A8);
    cpu.pc = 0x28D70;
    16
}
/// $0319A8: bcc.w $319b2
pub fn i_0319a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x319B2; return 10; }
    8
}
/// $0319AC: lea.l $318d4(pc), a1
pub fn i_0319ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x318D4;
    16
}
/// $0319B0: move.l a1, (a6)
pub fn i_0319b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0319B2: jsr $283d8.l
pub fn i_0319b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x319B8);
    cpu.pc = 0x283D8;
    16
}
/// $0319B8: movea.l #$ffffffff, a0
pub fn i_0319b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319BE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0319BE: jsr $5dd56.l
pub fn i_0319be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x319C4);
    cpu.pc = 0x5DD56;
    16
}
/// $0319C4: bcc.w $319ce
pub fn i_0319c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x319CE; return 10; }
    8
}
// ===== SetTaskHandler_0319c8  ($0319C8, 8 B, kernel/task_handlers.c)
/// $0319C8: lea.l $318d4(pc), a1
pub fn i_0319c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x318D4;
    16
}
/// $0319CC: move.l a1, (a6)
pub fn i_0319cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0319CE: rts
pub fn i_0319ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VehicleDrop_Template_0319d0  ($0319D0, 12 B, player/tables_fx_030696.s)
/// $0319D0: ori.b #$8, d1
pub fn i_0319d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319D4;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0319D4: dc.w $ffff
pub fn i_0319d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319D6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== SlugFx_ExhaustOrDrop_0319f0  ($0319F0, 188 B, player/tables_fx_030696.s)
/// $0319F0: movea.l $c(a6), a0
pub fn i_0319f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319F4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0319F4: btst.b #$2, $8d(a0)
pub fn i_0319f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319FA;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0319FA: beq.w $31a56
pub fn i_0319fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x319FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x31A56; return 10; }
    8
}
/// $0319FE: movea.l $70(a0), a1
pub fn i_0319fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A02;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $031A02: movea.l (a1), a1
pub fn i_031a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A04;
    cpu.ir = 0x2251;
    let v2 = bus.read32(cpu.a[1]);
    cpu.a[1] = v2;
    12
}
/// $031A04: move.w $8(a1), d1
pub fn i_031a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A08;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031A08: move.w $a(a1), d2
pub fn i_031a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A0C;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031A0C: add.w d1, $22(a6)
pub fn i_031a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A10;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031A10: add.w d2, $24(a6)
pub fn i_031a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A14;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031A14: move.w $80(a0), d0
pub fn i_031a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A18;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031A18: cmpi.w #$20, d0
pub fn i_031a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A1C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $031A1C: ble.w $31a24
pub fn i_031a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A20;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x31A24; return 10; }
    8
}
/// $031A20: move.w #$40, d0
pub fn i_031a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A24;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031A24: cmpi.w #$ffe0, d0
pub fn i_031a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A28;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFE0, Size::Word, false);
    8
}
/// $031A28: bge.w $31a30
pub fn i_031a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A2C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x31A30; return 10; }
    8
}
/// $031A2C: move.w #$ffe0, d0
pub fn i_031a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A30;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031A30: andi.w #$ff, d0
pub fn i_031a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A34;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031A34: move.w #$10, d1
pub fn i_031a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A38;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031A38: jsr $13c0e.l
pub fn i_031a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31A3E);
    cpu.pc = 0x13C0E;
    16
}
/// $031A3E: add.w d1, $22(a6)
pub fn i_031a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A42;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031A42: add.w d2, $24(a6)
pub fn i_031a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A46;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031A46: lea.l $29e440.l, a0
pub fn i_031a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A4C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29E440;
    20
}
/// $031A4C: jsr $28cd4.l
pub fn i_031a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31A52);
    cpu.pc = 0x28CD4;
    16
}
/// $031A52: bra.w $31a86
pub fn i_031a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A56;
    cpu.ir = 0x6000;
    cpu.pc = 0x31A86;
    10
}
/// $031A56: movea.l $c(a6), a1
pub fn i_031a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A5A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $031A5A: move.w $94(a1), $34(a6)
pub fn i_031a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A60;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $031A60: move.w $94(a1), $64(a6)
pub fn i_031a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A66;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x64);
    bus.write16(ad4, v3 as u16);
    20
}
/// $031A66: move.w $94(a1), d0
pub fn i_031a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A6A;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031A6A: movea.l #$319dc, a0
pub fn i_031a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A70;
    cpu.ir = 0x207C;
    let v1: u32 = 0x319DC;
    cpu.a[0] = v1;
    4
}
/// $031A70: lsl.w #$2, d0
pub fn i_031a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A72;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $031A72: movea.l (a0, d0.w), a0
pub fn i_031a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A76;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $031A76: cmpa.l #$ffffffff, a0
pub fn i_031a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A7C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $031A7C: beq.w $31a86
pub fn i_031a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x31A86; return 10; }
    8
}
/// $031A80: jsr $28cd4.l
pub fn i_031a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31A86);
    cpu.pc = 0x28CD4;
    16
}
/// $031A86: move.w #$8, d1
pub fn i_031a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A8A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031A8A: jsr $236e.l
pub fn i_031a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31A90);
    cpu.pc = 0x236E;
    16
}
/// $031A90: subi.w #$4, $22(a6)
pub fn i_031a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A96;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031A96: lea.l $31a9c(pc), a1
pub fn i_031a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31A9C;
    16
}
/// $031A9A: move.l a1, (a6)
pub fn i_031a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31A9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
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
// ===== SlugFx_Exhaust_031af6  ($031AF6, 92 B, player/tables_fx_030696.s)
/// $031AF6: movea.l $c(a6), a0
pub fn i_031af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31AFA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $031AFA: movea.l $70(a0), a1
pub fn i_031afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31AFE;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $031AFE: movea.l (a1), a1
pub fn i_031afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B00;
    cpu.ir = 0x2251;
    let v2 = bus.read32(cpu.a[1]);
    cpu.a[1] = v2;
    12
}
/// $031B00: move.w $8(a1), d1
pub fn i_031b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B04;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031B04: move.w $a(a1), d2
pub fn i_031b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B08;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031B08: add.w d1, $22(a6)
pub fn i_031b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B0C;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031B0C: add.w d2, $24(a6)
pub fn i_031b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B10;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031B10: move.w $80(a0), d0
pub fn i_031b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B14;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $031B14: cmpi.w #$20, d0
pub fn i_031b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B18;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $031B18: ble.w $31b20
pub fn i_031b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B1C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x31B20; return 10; }
    8
}
/// $031B1C: move.w #$40, d0
pub fn i_031b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B20;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031B20: cmpi.w #$ffe0, d0
pub fn i_031b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B24;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFE0, Size::Word, false);
    8
}
/// $031B24: bge.w $31b2c
pub fn i_031b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B28;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x31B2C; return 10; }
    8
}
/// $031B28: move.w #$ffe0, d0
pub fn i_031b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B2C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031B2C: andi.w #$ff, d0
pub fn i_031b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B30;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031B30: move.w #$10, d1
pub fn i_031b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B34;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031B34: jsr $13c0e.l
pub fn i_031b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31B3A);
    cpu.pc = 0x13C0E;
    16
}
/// $031B3A: add.w d1, $22(a6)
pub fn i_031b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B3E;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031B3E: add.w d2, $24(a6)
pub fn i_031b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B42;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031B42: lea.l $29e440.l, a0
pub fn i_031b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29E440;
    20
}
/// $031B48: jsr $28cd4.l
pub fn i_031b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31B4E);
    cpu.pc = 0x28CD4;
    16
}
/// $031B4E: bra.w $31a86
pub fn i_031b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B52;
    cpu.ir = 0x6000;
    cpu.pc = 0x31A86;
    10
}
// ===== Fx_Sparkle_031b52  ($031B52, 70 B, player/tables_fx_030696.s)
/// $031B52: move.w #$58, d1
pub fn i_031b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B56;
    cpu.ir = 0x323C;
    let v1: u32 = 0x58;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031B56: jsr $236e.l
pub fn i_031b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31B5C);
    cpu.pc = 0x236E;
    16
}
/// $031B5C: lea.l $29fcce.l, a0
pub fn i_031b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B62;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29FCCE;
    20
}
/// $031B62: jsr $28cd4.l
pub fn i_031b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31B68);
    cpu.pc = 0x28CD4;
    16
}
/// $031B68: move.w #$2, $70(a6)
pub fn i_031b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B6E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031B6E: lea.l $31b98(pc), a1
pub fn i_031b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31B98;
    16
}
/// $031B72: move.l a1, (a6)
pub fn i_031b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031B74: bra.w $31b98
pub fn i_031b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B78;
    cpu.ir = 0x6000;
    cpu.pc = 0x31B98;
    10
}
// ===== Fx_Sparkle_Tick_031b98  ($031B98, 42 B, player/tables_fx_030696.s)
/// $031B98: subi.w #$1, $38(a6)
pub fn i_031b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31B9E;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031B9E: jsr $2783a.l
pub fn i_031b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31BA4);
    cpu.pc = 0x2783A;
    16
}
/// $031BA4: jsr $28d70.l
pub fn i_031ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31BAA);
    cpu.pc = 0x28D70;
    16
}
/// $031BAA: tst.b $46(a6)
pub fn i_031baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BAE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $031BAE: bne.w $31bb8
pub fn i_031bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BB2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x31BB8; return 10; }
    8
}
/// $031BB2: subi.b #$1, $46(a6)
pub fn i_031bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BB8;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $031BB8: subi.w #$1, $70(a6)
pub fn i_031bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BBE;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031BBE: bgt.w $31bce
pub fn i_031bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BC2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x31BCE; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_031bc2  ($031BC2, 14 B, kernel/jsr5b6_jmp518.c)
/// $031BC2: jsr $5b6.l
pub fn i_031bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31BC8);
    cpu.pc = 0x5B6;
    16
}
/// $031BC8: jmp $518.l
pub fn i_031bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BCE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $031BCE: rts
pub fn i_031bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fx_SmokePuff_031bd0  ($031BD0, 66 B, player/tables_fx_030696.s)
/// $031BD0: move.w #$178, d1
pub fn i_031bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BD4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031BD4: jsr $236e.l
pub fn i_031bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31BDA);
    cpu.pc = 0x236E;
    16
}
/// $031BDA: lea.l $29e64a.l, a0
pub fn i_031bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BE0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29E64A;
    20
}
/// $031BE0: jsr $28cd4.l
pub fn i_031be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31BE6);
    cpu.pc = 0x28CD4;
    16
}
/// $031BE6: move.w #$2000, $38(a6)
pub fn i_031be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BEC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $031BEC: jsr $5e9b6.l
pub fn i_031bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31BF2);
    cpu.pc = 0x5E9B6;
    16
}
/// $031BF2: andi.w #$f, d0
pub fn i_031bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BF6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031BF6: subq.w #$8, d0
pub fn i_031bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BF8;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $031BF8: add.w d0, $22(a6)
pub fn i_031bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31BFC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031BFC: lea.l $31c02(pc), a1
pub fn i_031bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31C02;
    16
}
/// $031C00: move.l a1, (a6)
pub fn i_031c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031C02: jsr $2783a.l
pub fn i_031c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31C08);
    cpu.pc = 0x2783A;
    16
}
/// $031C08: jsr $28d70.l
pub fn i_031c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31C0E);
    cpu.pc = 0x28D70;
    16
}
/// $031C0E: bcc.w $31c1e
pub fn i_031c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x31C1E; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_031c12  ($031C12, 14 B, kernel/jsr5b6_jmp518.c)
/// $031C12: jsr $5b6.l
pub fn i_031c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31C18);
    cpu.pc = 0x5B6;
    16
}
/// $031C18: jmp $518.l
pub fn i_031c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C1E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $031C1E: rts
pub fn i_031c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fx_DustCloud_031c20  ($031C20, 60 B, player/tables_fx_030696.s)
/// $031C20: move.w #$5, d1
pub fn i_031c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C24;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031C24: jsr $236e.l
pub fn i_031c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31C2A);
    cpu.pc = 0x236E;
    16
}
/// $031C2A: jsr $5e9b6.l
pub fn i_031c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31C30);
    cpu.pc = 0x5E9B6;
    16
}
/// $031C30: andi.w #$f, d0
pub fn i_031c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C34;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031C34: subq.w #$8, d0
pub fn i_031c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C36;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $031C36: add.w d0, $22(a6)
pub fn i_031c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C3A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031C3A: lea.l $29e568.l, a0
pub fn i_031c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C40;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29E568;
    20
}
/// $031C40: jsr $28cd4.l
pub fn i_031c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31C46);
    cpu.pc = 0x28CD4;
    16
}
/// $031C46: lea.l $31c4c(pc), a1
pub fn i_031c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31C4C;
    16
}
/// $031C4A: move.l a1, (a6)
pub fn i_031c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031C4C: jsr $2783a.l
pub fn i_031c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31C52);
    cpu.pc = 0x2783A;
    16
}
/// $031C52: jsr $28d70.l
pub fn i_031c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31C58);
    cpu.pc = 0x28D70;
    16
}
/// $031C58: bcc.w $31c68
pub fn i_031c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x31C68; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_031c5c  ($031C5C, 14 B, kernel/jsr5b6_jmp518.c)
/// $031C5C: jsr $5b6.l
pub fn i_031c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31C62);
    cpu.pc = 0x5B6;
    16
}
/// $031C62: jmp $518.l
pub fn i_031c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C68;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $031C68: rts
pub fn i_031c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31C6A;
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
// ===== Fx_GroundBurstB_031d26  ($031D26, 68 B, player/tables_fx_030696.s)
/// $031D26: move.w #$4, d1
pub fn i_031d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D2A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031D2A: jsr $236e.l
pub fn i_031d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31D30);
    cpu.pc = 0x236E;
    16
}
/// $031D30: jsr $5e9b6.l
pub fn i_031d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31D36);
    cpu.pc = 0x5E9B6;
    16
}
/// $031D36: andi.w #$f, d0
pub fn i_031d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D3A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031D3A: subq.w #$8, d0
pub fn i_031d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D3C;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $031D3C: add.w d0, $22(a6)
pub fn i_031d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D40;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031D40: andi.w #$1, d0
pub fn i_031d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D44;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $031D44: movea.l #$31d1e, a0
pub fn i_031d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D4A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x31D1E;
    cpu.a[0] = v1;
    4
}
/// $031D4A: lsl.w #$2, d0
pub fn i_031d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D4C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $031D4C: movea.l (a0, d0.w), a0
pub fn i_031d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D50;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $031D50: cmpa.l #$ffffffff, a0
pub fn i_031d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D56;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $031D56: beq.w $31d60
pub fn i_031d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D5A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x31D60; return 10; }
    8
}
/// $031D5A: jsr $28cd4.l
pub fn i_031d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31D60);
    cpu.pc = 0x28CD4;
    16
}
/// $031D60: lea.l $31a9c(pc), a1
pub fn i_031d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31A9C;
    16
}
/// $031D64: move.l a1, (a6)
pub fn i_031d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031D66: bra.w $31a9c
pub fn i_031d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D6A;
    cpu.ir = 0x6000;
    cpu.pc = 0x31A9C;
    10
}
// ===== Fx_Spark_031d6a  ($031D6A, 6 B, player/tables_fx_030696.s)
/// $031D6A: jmp $518.l
pub fn i_031d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D70;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Entity_SpawnSparkIfBit1_031d90  ($031D90, 42 B, player/tables_fx_030696.s)
/// $031D90: btst.b #$1, $8c(a6)
pub fn i_031d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D96;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $031D96: beq.w $31da8
pub fn i_031d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31D9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x31DA8; return 10; }
    8
}
/// $031D9A: bclr.b #$1, $8c(a6)
pub fn i_031d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DA0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $031DA0: ori.b #$11, ccr
pub fn i_031da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DA4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $031DA4: bra.w $31dac
pub fn i_031da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x31DAC;
    10
}
/// $031DA8: andi.b #$ee, ccr
pub fn i_031da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DAC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $031DAC: bcc.w $31dc0
pub fn i_031dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DB0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x31DC0; return 10; }
    8
}
/// $031DB0: lea.l $31d6a(pc), a1
pub fn i_031db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31D6A;
    16
}
/// $031DB4: jsr $4ae.l
pub fn i_031db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31DBA);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_031dba  ($031DBA, 8 B, thunks/jsr_abs_thunks.c)
/// $031DBA: jsr $5dd02.l
pub fn i_031dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31DC0);
    cpu.pc = 0x5DD02;
    16
}
/// $031DC0: rts
pub fn i_031dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SpawnSparkLeftIfBit1_031dc2  ($031DC2, 60 B, player/tables_fx_030696.s)
/// $031DC2: btst.b #$1, $8c(a6)
pub fn i_031dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DC8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $031DC8: beq.w $31dda
pub fn i_031dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DCC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x31DDA; return 10; }
    8
}
/// $031DCC: bclr.b #$1, $8c(a6)
pub fn i_031dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DD2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $031DD2: ori.b #$11, ccr
pub fn i_031dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DD6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $031DD6: bra.w $31dde
pub fn i_031dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DDA;
    cpu.ir = 0x6000;
    cpu.pc = 0x31DDE;
    10
}
/// $031DDA: andi.b #$ee, ccr
pub fn i_031dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DDE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $031DDE: bcc.b $31dc0
pub fn i_031dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DE0;
    cpu.ir = 0x64E0;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x31DC0; return 10; }
    8
}
/// $031DE0: lea.l $31d6a(pc), a1
pub fn i_031de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31D6A;
    16
}
/// $031DE4: jsr $4ae.l
pub fn i_031de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31DEA);
    cpu.pc = 0x4AE;
    16
}
/// $031DEA: jsr $5dd02.l
pub fn i_031dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31DF0);
    cpu.pc = 0x5DD02;
    16
}
/// $031DF0: addi.w #$ffff, $22(a0)
pub fn i_031df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DF6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFFF, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031DF6: addi.w #$fffd, $24(a0)
pub fn i_031df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DFC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFFD, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031DFC: bra.b $31dc0
pub fn i_031dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31DFE;
    cpu.ir = 0x60C2;
    cpu.pc = 0x31DC0;
    10
}
// ===== Entity_SpawnSparkRightIfBit1_031dfe  ($031DFE, 58 B, player/tables_fx_030696.s)
/// $031DFE: btst.b #$1, $8c(a6)
pub fn i_031dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E04;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $031E04: beq.w $31e16
pub fn i_031e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x31E16; return 10; }
    8
}
/// $031E08: bclr.b #$1, $8c(a6)
pub fn i_031e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E0E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $031E0E: ori.b #$11, ccr
pub fn i_031e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E12;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $031E12: bra.w $31e1a
pub fn i_031e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E16;
    cpu.ir = 0x6000;
    cpu.pc = 0x31E1A;
    10
}
/// $031E16: andi.b #$ee, ccr
pub fn i_031e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E1A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $031E1A: bcc.b $31dc0
pub fn i_031e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E1C;
    cpu.ir = 0x64A4;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x31DC0; return 10; }
    8
}
/// $031E1C: lea.l $31d6a(pc), a1
pub fn i_031e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31D6A;
    16
}
/// $031E20: jsr $4ae.l
pub fn i_031e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31E26);
    cpu.pc = 0x4AE;
    16
}
/// $031E26: jsr $5dd02.l
pub fn i_031e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31E2C);
    cpu.pc = 0x5DD02;
    16
}
/// $031E2C: addq.w #$1, $22(a0)
pub fn i_031e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E30;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031E30: addi.w #$fffd, $24(a0)
pub fn i_031e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E36;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFFD, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031E36: bra.b $31dc0
pub fn i_031e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E38;
    cpu.ir = 0x6088;
    cpu.pc = 0x31DC0;
    10
}
// ===== Fx_Dust_031e38  ($031E38, 6 B, player/tables_fx_030696.s)
/// $031E38: jmp $518.l
pub fn i_031e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E3E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Fx_SpawnDustPair_031e5e  ($031E5E, 324 B, player/tables_fx_030696.s)
/// $031E5E: lea.l $31e38(pc), a1
pub fn i_031e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31E38;
    16
}
/// $031E62: jsr $4ae.l
pub fn i_031e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31E68);
    cpu.pc = 0x4AE;
    16
}
/// $031E68: jsr $5dd02.l
pub fn i_031e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31E6E);
    cpu.pc = 0x5DD02;
    16
}
/// $031E6E: addi.w #$ffe0, $22(a0)
pub fn i_031e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E74;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031E74: rts
pub fn i_031e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $031E76: lea.l $31e38(pc), a1
pub fn i_031e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E7A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31E38;
    16
}
/// $031E7A: jsr $4ae.l
pub fn i_031e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31E80);
    cpu.pc = 0x4AE;
    16
}
/// $031E80: jsr $5dd02.l
pub fn i_031e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31E86);
    cpu.pc = 0x5DD02;
    16
}
/// $031E86: addi.w #$20, $22(a0)
pub fn i_031e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E8C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031E8C: eori.b #$1, $3a(a0)
pub fn i_031e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E92;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $031E92: bra.b $31e74
pub fn i_031e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31E94;
    cpu.ir = 0x60E0;
    cpu.pc = 0x31E74;
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
// ===== PlayerIcon_Bubble_03207c  ($03207C, 80 B, player/tables_fx_030696.s)
/// $03207C: move.w #$1b0, d1
pub fn i_03207c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32080;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $032080: jsr $236e.l
pub fn i_032080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32086;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32086);
    cpu.pc = 0x236E;
    16
}
/// $032086: lea.l $3232a(pc), a0
pub fn i_032086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3208A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3232A;
    16
}
/// $03208A: jsr $28cd4.l
pub fn i_03208a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32090;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32090);
    cpu.pc = 0x28CD4;
    16
}
/// $032090: move.w #$38, $5c(a6)
pub fn i_032090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32096;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032096: bra.w $3209a
pub fn i_032096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3209A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3209A;
    10
}
/// $03209A: move.w #$ffff, $38(a6)
pub fn i_03209a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320A0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0320A0: bset.b #$6, $12(a6)
pub fn i_0320a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320A6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0320A6: lea.l $320ac(pc), a1
pub fn i_0320a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x320AC;
    16
}
/// $0320AA: move.l a1, (a6)
pub fn i_0320aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0320AC: jsr $5e4ca.l
pub fn i_0320ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x320B2);
    cpu.pc = 0x5E4CA;
    16
}
/// $0320B2: move.w d1, $22(a6)
pub fn i_0320b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320B6;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0320B6: add.w $5c(a6), d2
pub fn i_0320b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320BA;
    cpu.ir = 0xD46E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0320BA: move.w d2, $24(a6)
pub fn i_0320ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320BE;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0320BE: move.w d0, $38(a6)
pub fn i_0320be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320C2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0320C2: jsr $28d70.l
pub fn i_0320c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x320C8);
    cpu.pc = 0x28D70;
    16
}
/// $0320C8: bcc.w $320d2
pub fn i_0320c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x320D2; return 10; }
    8
}
// ===== SetTaskHandler_0320cc  ($0320CC, 8 B, kernel/task_handlers.c)
/// $0320CC: lea.l $321bc(pc), a1
pub fn i_0320cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x321BC;
    16
}
/// $0320D0: move.l a1, (a6)
pub fn i_0320d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0320D2: rts
pub fn i_0320d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerIcon_SpawnFreeFallBoth_0320d4  ($0320D4, 62 B, player/tables_fx_030696.s)
/// $0320D4: movem.l a6, -(a7)
pub fn i_0320d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320D8;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0320D8: move.w #$0, d0
pub fn i_0320d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320DC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0320DC: jsr $5e3a2.l
pub fn i_0320dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x320E2);
    cpu.pc = 0x5E3A2;
    16
}
/// $0320E2: bcc.w $320f2
pub fn i_0320e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x320F2; return 10; }
    8
}
/// $0320E6: movea.l a0, a6
pub fn i_0320e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320E8;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $0320E8: lea.l $32112(pc), a1
pub fn i_0320e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x32112;
    16
}
/// $0320EC: jsr $4ae.l
pub fn i_0320ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x320F2);
    cpu.pc = 0x4AE;
    16
}
/// $0320F2: move.w #$1, d0
pub fn i_0320f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320F6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0320F6: jsr $5e3a2.l
pub fn i_0320f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x320FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x320FC);
    cpu.pc = 0x5E3A2;
    16
}
/// $0320FC: bcc.w $3210c
pub fn i_0320fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32100;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3210C; return 10; }
    8
}
/// $032100: movea.l a0, a6
pub fn i_032100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32102;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $032102: lea.l $32142(pc), a1
pub fn i_032102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32106;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x32142;
    16
}
/// $032106: jsr $4ae.l
pub fn i_032106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3210C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3210C);
    cpu.pc = 0x4AE;
    16
}
/// $03210C: movem.l (a7)+, a6
pub fn i_03210c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32110;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $032110: rts
pub fn i_032110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32112;
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
// ===== PlayerIcon_FreeFallP2_032142  ($032142, 114 B, player/tables_fx_030696.s)
/// $032142: cmpi.l #$1004e0, $c(a6)
pub fn i_032142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3214A;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x1004E0, Size::Long, false);
    20
}
/// $03214A: beq.w $32154
pub fn i_03214a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3214E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32154; return 10; }
    8
}
/// $03214E: jmp $518.l
pub fn i_03214e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32154;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $032154: move.w #$a3, d1
pub fn i_032154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32158;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $032158: jsr $236e.l
pub fn i_032158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3215E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3215E);
    cpu.pc = 0x236E;
    16
}
/// $03215E: lea.l $32224(pc), a0
pub fn i_03215e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32162;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32224;
    16
}
/// $032162: jsr $28cd4.l
pub fn i_032162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32168;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32168);
    cpu.pc = 0x28CD4;
    16
}
/// $032168: move.w #$38, $5c(a6)
pub fn i_032168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3216E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
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

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x030000, i_030000),
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
    (0x030696, i_030696),
    (0x03069C, i_03069c),
    (0x0306A0, i_0306a0),
    (0x0306A6, i_0306a6),
    (0x0306AC, i_0306ac),
    (0x0306B0, i_0306b0),
    (0x0306B6, i_0306b6),
    (0x0306BC, i_0306bc),
    (0x0306C0, i_0306c0),
    (0x0306C6, i_0306c6),
    (0x0306CC, i_0306cc),
    (0x0306D0, i_0306d0),
    (0x0306D6, i_0306d6),
    (0x0306DC, i_0306dc),
    (0x0306E0, i_0306e0),
    (0x0306E6, i_0306e6),
    (0x0306EC, i_0306ec),
    (0x0306F0, i_0306f0),
    (0x0306F6, i_0306f6),
    (0x0306FC, i_0306fc),
    (0x030700, i_030700),
    (0x030702, i_030702),
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
    (0x03073E, i_03073e),
    (0x030742, i_030742),
    (0x030746, i_030746),
    (0x03074A, i_03074a),
    (0x03074E, i_03074e),
    (0x030752, i_030752),
    (0x030756, i_030756),
    (0x03075A, i_03075a),
    (0x03075E, i_03075e),
    (0x030762, i_030762),
    (0x030764, i_030764),
    (0x030768, i_030768),
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
    (0x0307E8, i_0307e8),
    (0x0307EC, i_0307ec),
    (0x0307F0, i_0307f0),
    (0x0307F4, i_0307f4),
    (0x0307F8, i_0307f8),
    (0x0307FC, i_0307fc),
    (0x0307FE, i_0307fe),
    (0x030802, i_030802),
    (0x0308B0, i_0308b0),
    (0x0308B4, i_0308b4),
    (0x0308B8, i_0308b8),
    (0x0308BE, i_0308be),
    (0x0308C2, i_0308c2),
    (0x0308C6, i_0308c6),
    (0x0308CA, i_0308ca),
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
    (0x030C14, i_030c14),
    (0x030C1A, i_030c1a),
    (0x030C1E, i_030c1e),
    (0x030C24, i_030c24),
    (0x030C2A, i_030c2a),
    (0x030C2E, i_030c2e),
    (0x030C32, i_030c32),
    (0x030C34, i_030c34),
    (0x030C38, i_030c38),
    (0x030C3E, i_030c3e),
    (0x030C42, i_030c42),
    (0x030C46, i_030c46),
    (0x030C4C, i_030c4c),
    (0x030C52, i_030c52),
    (0x030C56, i_030c56),
    (0x030C58, i_030c58),
    (0x030C5E, i_030c5e),
    (0x030C64, i_030c64),
    (0x030C68, i_030c68),
    (0x030C6E, i_030c6e),
    (0x030C70, i_030c70),
    (0x030C74, i_030c74),
    (0x030C7A, i_030c7a),
    (0x030C80, i_030c80),
    (0x030C86, i_030c86),
    (0x030C8C, i_030c8c),
    (0x030C8E, i_030c8e),
    (0x030C92, i_030c92),
    (0x030C96, i_030c96),
    (0x030C98, i_030c98),
    (0x030C9A, i_030c9a),
    (0x030C9E, i_030c9e),
    (0x030CA2, i_030ca2),
    (0x030CA8, i_030ca8),
    (0x030CAA, i_030caa),
    (0x030CAE, i_030cae),
    (0x030CB4, i_030cb4),
    (0x030CB8, i_030cb8),
    (0x030CBE, i_030cbe),
    (0x030CC2, i_030cc2),
    (0x030CC4, i_030cc4),
    (0x030CC8, i_030cc8),
    (0x030CCC, i_030ccc),
    (0x030CD2, i_030cd2),
    (0x030CD6, i_030cd6),
    (0x030CDC, i_030cdc),
    (0x030CE0, i_030ce0),
    (0x030CE4, i_030ce4),
    (0x030CE6, i_030ce6),
    (0x030CEC, i_030cec),
    (0x030CF2, i_030cf2),
    (0x030CF8, i_030cf8),
    (0x030CFC, i_030cfc),
    (0x030D00, i_030d00),
    (0x030D02, i_030d02),
    (0x030D04, i_030d04),
    (0x030D0A, i_030d0a),
    (0x030D10, i_030d10),
    (0x030D16, i_030d16),
    (0x030D1A, i_030d1a),
    (0x030D1C, i_030d1c),
    (0x030D20, i_030d20),
    (0x030D24, i_030d24),
    (0x030D2A, i_030d2a),
    (0x030D2E, i_030d2e),
    (0x030D34, i_030d34),
    (0x030D38, i_030d38),
    (0x030D3C, i_030d3c),
    (0x030D3E, i_030d3e),
    (0x030D44, i_030d44),
    (0x030D4A, i_030d4a),
    (0x030D50, i_030d50),
    (0x030D54, i_030d54),
    (0x030D58, i_030d58),
    (0x030D5A, i_030d5a),
    (0x030D5C, i_030d5c),
    (0x030D60, i_030d60),
    (0x030D66, i_030d66),
    (0x030D6C, i_030d6c),
    (0x030D70, i_030d70),
    (0x030D72, i_030d72),
    (0x030D74, i_030d74),
    (0x0311C0, i_0311c0),
    (0x0311C4, i_0311c4),
    (0x0311C8, i_0311c8),
    (0x0311CE, i_0311ce),
    (0x0311D2, i_0311d2),
    (0x0311D6, i_0311d6),
    (0x0311DA, i_0311da),
    (0x0311DE, i_0311de),
    (0x0311E4, i_0311e4),
    (0x0311EA, i_0311ea),
    (0x0311EE, i_0311ee),
    (0x0311F2, i_0311f2),
    (0x0311F6, i_0311f6),
    (0x0311FA, i_0311fa),
    (0x0311FE, i_0311fe),
    (0x031202, i_031202),
    (0x031204, i_031204),
    (0x031208, i_031208),
    (0x03120E, i_03120e),
    (0x031214, i_031214),
    (0x031218, i_031218),
    (0x03121C, i_03121c),
    (0x031222, i_031222),
    (0x031226, i_031226),
    (0x03122A, i_03122a),
    (0x0312B2, i_0312b2),
    (0x0312B6, i_0312b6),
    (0x0312BA, i_0312ba),
    (0x0312C0, i_0312c0),
    (0x0312C6, i_0312c6),
    (0x0312CC, i_0312cc),
    (0x0312D2, i_0312d2),
    (0x0312DA, i_0312da),
    (0x0312E0, i_0312e0),
    (0x0312E6, i_0312e6),
    (0x0312EA, i_0312ea),
    (0x0312EC, i_0312ec),
    (0x0312F0, i_0312f0),
    (0x0312F4, i_0312f4),
    (0x0312F8, i_0312f8),
    (0x0312FC, i_0312fc),
    (0x031300, i_031300),
    (0x031306, i_031306),
    (0x03130A, i_03130a),
    (0x03130E, i_03130e),
    (0x031312, i_031312),
    (0x031316, i_031316),
    (0x03131A, i_03131a),
    (0x031320, i_031320),
    (0x031324, i_031324),
    (0x031328, i_031328),
    (0x0313A6, i_0313a6),
    (0x0313AA, i_0313aa),
    (0x0313AE, i_0313ae),
    (0x0313B2, i_0313b2),
    (0x0313B6, i_0313b6),
    (0x0313BC, i_0313bc),
    (0x0313C0, i_0313c0),
    (0x0313C4, i_0313c4),
    (0x0313C8, i_0313c8),
    (0x0313CC, i_0313cc),
    (0x0313D0, i_0313d0),
    (0x0313D6, i_0313d6),
    (0x0313DC, i_0313dc),
    (0x0313E0, i_0313e0),
    (0x0313E4, i_0313e4),
    (0x0313E8, i_0313e8),
    (0x0313EC, i_0313ec),
    (0x0313F2, i_0313f2),
    (0x0313F8, i_0313f8),
    (0x0313FA, i_0313fa),
    (0x0313FE, i_0313fe),
    (0x031402, i_031402),
    (0x031406, i_031406),
    (0x03140C, i_03140c),
    (0x031412, i_031412),
    (0x031416, i_031416),
    (0x03141A, i_03141a),
    (0x03141C, i_03141c),
    (0x031420, i_031420),
    (0x031424, i_031424),
    (0x031428, i_031428),
    (0x03142C, i_03142c),
    (0x031430, i_031430),
    (0x031434, i_031434),
    (0x031438, i_031438),
    (0x03143E, i_03143e),
    (0x031442, i_031442),
    (0x031446, i_031446),
    (0x03144A, i_03144a),
    (0x031452, i_031452),
    (0x031456, i_031456),
    (0x03145A, i_03145a),
    (0x03145C, i_03145c),
    (0x031462, i_031462),
    (0x031466, i_031466),
    (0x03146A, i_03146a),
    (0x031470, i_031470),
    (0x031476, i_031476),
    (0x03147A, i_03147a),
    (0x03147C, i_03147c),
    (0x031480, i_031480),
    (0x031484, i_031484),
    (0x03148A, i_03148a),
    (0x03148E, i_03148e),
    (0x031492, i_031492),
    (0x031496, i_031496),
    (0x03149C, i_03149c),
    (0x0314A0, i_0314a0),
    (0x0314A4, i_0314a4),
    (0x0314A8, i_0314a8),
    (0x0314AC, i_0314ac),
    (0x0314B0, i_0314b0),
    (0x0314B4, i_0314b4),
    (0x0314B6, i_0314b6),
    (0x0314BA, i_0314ba),
    (0x0314BE, i_0314be),
    (0x0314C2, i_0314c2),
    (0x0314C8, i_0314c8),
    (0x0314CC, i_0314cc),
    (0x0314D0, i_0314d0),
    (0x0314D4, i_0314d4),
    (0x0314D6, i_0314d6),
    (0x0314DA, i_0314da),
    (0x0314DE, i_0314de),
    (0x0314E2, i_0314e2),
    (0x0314E8, i_0314e8),
    (0x0314EC, i_0314ec),
    (0x0314F0, i_0314f0),
    (0x0314F4, i_0314f4),
    (0x0314FA, i_0314fa),
    (0x031500, i_031500),
    (0x031506, i_031506),
    (0x03150C, i_03150c),
    (0x031512, i_031512),
    (0x031516, i_031516),
    (0x03151C, i_03151c),
    (0x031520, i_031520),
    (0x031524, i_031524),
    (0x03152A, i_03152a),
    (0x031530, i_031530),
    (0x031536, i_031536),
    (0x03153C, i_03153c),
    (0x031540, i_031540),
    (0x031542, i_031542),
    (0x031546, i_031546),
    (0x03154A, i_03154a),
    (0x03154C, i_03154c),
    (0x031550, i_031550),
    (0x031554, i_031554),
    (0x031558, i_031558),
    (0x03155E, i_03155e),
    (0x031562, i_031562),
    (0x031566, i_031566),
    (0x03156A, i_03156a),
    (0x03156C, i_03156c),
    (0x031570, i_031570),
    (0x031574, i_031574),
    (0x031578, i_031578),
    (0x03157C, i_03157c),
    (0x03157E, i_03157e),
    (0x031582, i_031582),
    (0x031586, i_031586),
    (0x031588, i_031588),
    (0x03158C, i_03158c),
    (0x03158E, i_03158e),
    (0x031590, i_031590),
    (0x031594, i_031594),
    (0x031598, i_031598),
    (0x03159C, i_03159c),
    (0x0315A0, i_0315a0),
    (0x0315A2, i_0315a2),
    (0x0315A6, i_0315a6),
    (0x0315AA, i_0315aa),
    (0x0315AC, i_0315ac),
    (0x0315AE, i_0315ae),
    (0x0315B4, i_0315b4),
    (0x0315B6, i_0315b6),
    (0x0315BA, i_0315ba),
    (0x0315C0, i_0315c0),
    (0x0315C4, i_0315c4),
    (0x0315C8, i_0315c8),
    (0x0315CC, i_0315cc),
    (0x0315D4, i_0315d4),
    (0x0315D8, i_0315d8),
    (0x0315DC, i_0315dc),
    (0x0315E0, i_0315e0),
    (0x0315E8, i_0315e8),
    (0x0315EC, i_0315ec),
    (0x0315F0, i_0315f0),
    (0x0315F4, i_0315f4),
    (0x0315F8, i_0315f8),
    (0x0315FC, i_0315fc),
    (0x031600, i_031600),
    (0x031606, i_031606),
    (0x03160C, i_03160c),
    (0x031612, i_031612),
    (0x031618, i_031618),
    (0x03161E, i_03161e),
    (0x031624, i_031624),
    (0x03162A, i_03162a),
    (0x03162E, i_03162e),
    (0x031632, i_031632),
    (0x031634, i_031634),
    (0x03163A, i_03163a),
    (0x031640, i_031640),
    (0x031646, i_031646),
    (0x03164A, i_03164a),
    (0x03164E, i_03164e),
    (0x031650, i_031650),
    (0x031656, i_031656),
    (0x03165A, i_03165a),
    (0x03165E, i_03165e),
    (0x031660, i_031660),
    (0x031664, i_031664),
    (0x031668, i_031668),
    (0x03166C, i_03166c),
    (0x03166E, i_03166e),
    (0x031674, i_031674),
    (0x031678, i_031678),
    (0x03167E, i_03167e),
    (0x031682, i_031682),
    (0x031686, i_031686),
    (0x031688, i_031688),
    (0x03168A, i_03168a),
    (0x031690, i_031690),
    (0x031694, i_031694),
    (0x03169A, i_03169a),
    (0x03169E, i_03169e),
    (0x0316A2, i_0316a2),
    (0x0316A8, i_0316a8),
    (0x0316AC, i_0316ac),
    (0x0316B2, i_0316b2),
    (0x0316B8, i_0316b8),
    (0x0316BC, i_0316bc),
    (0x0316BE, i_0316be),
    (0x0316C4, i_0316c4),
    (0x0316C8, i_0316c8),
    (0x0316CC, i_0316cc),
    (0x0316D0, i_0316d0),
    (0x0316D8, i_0316d8),
    (0x0316DC, i_0316dc),
    (0x0316E0, i_0316e0),
    (0x0316E4, i_0316e4),
    (0x0316EC, i_0316ec),
    (0x0316F0, i_0316f0),
    (0x0316F4, i_0316f4),
    (0x0316F8, i_0316f8),
    (0x0316FC, i_0316fc),
    (0x031700, i_031700),
    (0x031704, i_031704),
    (0x03170A, i_03170a),
    (0x031710, i_031710),
    (0x031716, i_031716),
    (0x03171C, i_03171c),
    (0x031720, i_031720),
    (0x031724, i_031724),
    (0x03172A, i_03172a),
    (0x03172E, i_03172e),
    (0x031732, i_031732),
    (0x031736, i_031736),
    (0x03173A, i_03173a),
    (0x03173E, i_03173e),
    (0x031744, i_031744),
    (0x03174A, i_03174a),
    (0x03174E, i_03174e),
    (0x031752, i_031752),
    (0x031754, i_031754),
    (0x03175A, i_03175a),
    (0x03175E, i_03175e),
    (0x031762, i_031762),
    (0x031764, i_031764),
    (0x031768, i_031768),
    (0x03176C, i_03176c),
    (0x03176E, i_03176e),
    (0x031770, i_031770),
    (0x031776, i_031776),
    (0x031778, i_031778),
    (0x03177C, i_03177c),
    (0x031782, i_031782),
    (0x031788, i_031788),
    (0x03178E, i_03178e),
    (0x031792, i_031792),
    (0x031796, i_031796),
    (0x031798, i_031798),
    (0x03179E, i_03179e),
    (0x0317A2, i_0317a2),
    (0x0317A6, i_0317a6),
    (0x0317A8, i_0317a8),
    (0x0317AC, i_0317ac),
    (0x0317B0, i_0317b0),
    (0x0317B4, i_0317b4),
    (0x0317B6, i_0317b6),
    (0x0317BC, i_0317bc),
    (0x0317C0, i_0317c0),
    (0x0317C6, i_0317c6),
    (0x0317CA, i_0317ca),
    (0x0317CE, i_0317ce),
    (0x0317D0, i_0317d0),
    (0x0317D2, i_0317d2),
    (0x0317DA, i_0317da),
    (0x0317DE, i_0317de),
    (0x0317E4, i_0317e4),
    (0x0317EA, i_0317ea),
    (0x0317F0, i_0317f0),
    (0x0317F4, i_0317f4),
    (0x0317F8, i_0317f8),
    (0x0317FE, i_0317fe),
    (0x031802, i_031802),
    (0x031808, i_031808),
    (0x03180C, i_03180c),
    (0x03180E, i_03180e),
    (0x031812, i_031812),
    (0x031818, i_031818),
    (0x03181E, i_03181e),
    (0x031824, i_031824),
    (0x031826, i_031826),
    (0x03182A, i_03182a),
    (0x03182C, i_03182c),
    (0x031832, i_031832),
    (0x031836, i_031836),
    (0x03183A, i_03183a),
    (0x031840, i_031840),
    (0x031846, i_031846),
    (0x03184C, i_03184c),
    (0x03184E, i_03184e),
    (0x031854, i_031854),
    (0x031858, i_031858),
    (0x03185C, i_03185c),
    (0x031862, i_031862),
    (0x031868, i_031868),
    (0x03186C, i_03186c),
    (0x03186E, i_03186e),
    (0x031874, i_031874),
    (0x031876, i_031876),
    (0x03187C, i_03187c),
    (0x031880, i_031880),
    (0x031884, i_031884),
    (0x03188A, i_03188a),
    (0x031890, i_031890),
    (0x031896, i_031896),
    (0x03189A, i_03189a),
    (0x03189E, i_03189e),
    (0x0318A4, i_0318a4),
    (0x0318A8, i_0318a8),
    (0x0318AA, i_0318aa),
    (0x0318AC, i_0318ac),
    (0x0318B4, i_0318b4),
    (0x0318B8, i_0318b8),
    (0x0318BE, i_0318be),
    (0x0318C2, i_0318c2),
    (0x0318C6, i_0318c6),
    (0x0318CC, i_0318cc),
    (0x0318D0, i_0318d0),
    (0x0318D2, i_0318d2),
    (0x0318D4, i_0318d4),
    (0x0318D8, i_0318d8),
    (0x0318DE, i_0318de),
    (0x0318E4, i_0318e4),
    (0x0318E6, i_0318e6),
    (0x0318EA, i_0318ea),
    (0x0318EE, i_0318ee),
    (0x0318F4, i_0318f4),
    (0x0318F8, i_0318f8),
    (0x0318FC, i_0318fc),
    (0x031900, i_031900),
    (0x031904, i_031904),
    (0x031908, i_031908),
    (0x03190E, i_03190e),
    (0x031912, i_031912),
    (0x031916, i_031916),
    (0x03191A, i_03191a),
    (0x03191E, i_03191e),
    (0x03197C, i_03197c),
    (0x031980, i_031980),
    (0x031986, i_031986),
    (0x03198A, i_03198a),
    (0x031990, i_031990),
    (0x031996, i_031996),
    (0x03199A, i_03199a),
    (0x03199C, i_03199c),
    (0x0319A2, i_0319a2),
    (0x0319A8, i_0319a8),
    (0x0319AC, i_0319ac),
    (0x0319B0, i_0319b0),
    (0x0319B2, i_0319b2),
    (0x0319B8, i_0319b8),
    (0x0319BE, i_0319be),
    (0x0319C4, i_0319c4),
    (0x0319C8, i_0319c8),
    (0x0319CC, i_0319cc),
    (0x0319CE, i_0319ce),
    (0x0319D0, i_0319d0),
    (0x0319D4, i_0319d4),
    (0x0319F0, i_0319f0),
    (0x0319F4, i_0319f4),
    (0x0319FA, i_0319fa),
    (0x0319FE, i_0319fe),
    (0x031A02, i_031a02),
    (0x031A04, i_031a04),
    (0x031A08, i_031a08),
    (0x031A0C, i_031a0c),
    (0x031A10, i_031a10),
    (0x031A14, i_031a14),
    (0x031A18, i_031a18),
    (0x031A1C, i_031a1c),
    (0x031A20, i_031a20),
    (0x031A24, i_031a24),
    (0x031A28, i_031a28),
    (0x031A2C, i_031a2c),
    (0x031A30, i_031a30),
    (0x031A34, i_031a34),
    (0x031A38, i_031a38),
    (0x031A3E, i_031a3e),
    (0x031A42, i_031a42),
    (0x031A46, i_031a46),
    (0x031A4C, i_031a4c),
    (0x031A52, i_031a52),
    (0x031A56, i_031a56),
    (0x031A5A, i_031a5a),
    (0x031A60, i_031a60),
    (0x031A66, i_031a66),
    (0x031A6A, i_031a6a),
    (0x031A70, i_031a70),
    (0x031A72, i_031a72),
    (0x031A76, i_031a76),
    (0x031A7C, i_031a7c),
    (0x031A80, i_031a80),
    (0x031A86, i_031a86),
    (0x031A8A, i_031a8a),
    (0x031A90, i_031a90),
    (0x031A96, i_031a96),
    (0x031A9A, i_031a9a),
    (0x031A9C, i_031a9c),
    (0x031AA2, i_031aa2),
    (0x031AA8, i_031aa8),
    (0x031AAC, i_031aac),
    (0x031AB2, i_031ab2),
    (0x031AB8, i_031ab8),
    (0x031AF6, i_031af6),
    (0x031AFA, i_031afa),
    (0x031AFE, i_031afe),
    (0x031B00, i_031b00),
    (0x031B04, i_031b04),
    (0x031B08, i_031b08),
    (0x031B0C, i_031b0c),
    (0x031B10, i_031b10),
    (0x031B14, i_031b14),
    (0x031B18, i_031b18),
    (0x031B1C, i_031b1c),
    (0x031B20, i_031b20),
    (0x031B24, i_031b24),
    (0x031B28, i_031b28),
    (0x031B2C, i_031b2c),
    (0x031B30, i_031b30),
    (0x031B34, i_031b34),
    (0x031B3A, i_031b3a),
    (0x031B3E, i_031b3e),
    (0x031B42, i_031b42),
    (0x031B48, i_031b48),
    (0x031B4E, i_031b4e),
    (0x031B52, i_031b52),
    (0x031B56, i_031b56),
    (0x031B5C, i_031b5c),
    (0x031B62, i_031b62),
    (0x031B68, i_031b68),
    (0x031B6E, i_031b6e),
    (0x031B72, i_031b72),
    (0x031B74, i_031b74),
    (0x031B98, i_031b98),
    (0x031B9E, i_031b9e),
    (0x031BA4, i_031ba4),
    (0x031BAA, i_031baa),
    (0x031BAE, i_031bae),
    (0x031BB2, i_031bb2),
    (0x031BB8, i_031bb8),
    (0x031BBE, i_031bbe),
    (0x031BC2, i_031bc2),
    (0x031BC8, i_031bc8),
    (0x031BCE, i_031bce),
    (0x031BD0, i_031bd0),
    (0x031BD4, i_031bd4),
    (0x031BDA, i_031bda),
    (0x031BE0, i_031be0),
    (0x031BE6, i_031be6),
    (0x031BEC, i_031bec),
    (0x031BF2, i_031bf2),
    (0x031BF6, i_031bf6),
    (0x031BF8, i_031bf8),
    (0x031BFC, i_031bfc),
    (0x031C00, i_031c00),
    (0x031C02, i_031c02),
    (0x031C08, i_031c08),
    (0x031C0E, i_031c0e),
    (0x031C12, i_031c12),
    (0x031C18, i_031c18),
    (0x031C1E, i_031c1e),
    (0x031C20, i_031c20),
    (0x031C24, i_031c24),
    (0x031C2A, i_031c2a),
    (0x031C30, i_031c30),
    (0x031C34, i_031c34),
    (0x031C36, i_031c36),
    (0x031C3A, i_031c3a),
    (0x031C40, i_031c40),
    (0x031C46, i_031c46),
    (0x031C4A, i_031c4a),
    (0x031C4C, i_031c4c),
    (0x031C52, i_031c52),
    (0x031C58, i_031c58),
    (0x031C5C, i_031c5c),
    (0x031C62, i_031c62),
    (0x031C68, i_031c68),
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
    (0x031D26, i_031d26),
    (0x031D2A, i_031d2a),
    (0x031D30, i_031d30),
    (0x031D36, i_031d36),
    (0x031D3A, i_031d3a),
    (0x031D3C, i_031d3c),
    (0x031D40, i_031d40),
    (0x031D44, i_031d44),
    (0x031D4A, i_031d4a),
    (0x031D4C, i_031d4c),
    (0x031D50, i_031d50),
    (0x031D56, i_031d56),
    (0x031D5A, i_031d5a),
    (0x031D60, i_031d60),
    (0x031D64, i_031d64),
    (0x031D66, i_031d66),
    (0x031D6A, i_031d6a),
    (0x031D90, i_031d90),
    (0x031D96, i_031d96),
    (0x031D9A, i_031d9a),
    (0x031DA0, i_031da0),
    (0x031DA4, i_031da4),
    (0x031DA8, i_031da8),
    (0x031DAC, i_031dac),
    (0x031DB0, i_031db0),
    (0x031DB4, i_031db4),
    (0x031DBA, i_031dba),
    (0x031DC0, i_031dc0),
    (0x031DC2, i_031dc2),
    (0x031DC8, i_031dc8),
    (0x031DCC, i_031dcc),
    (0x031DD2, i_031dd2),
    (0x031DD6, i_031dd6),
    (0x031DDA, i_031dda),
    (0x031DDE, i_031dde),
    (0x031DE0, i_031de0),
    (0x031DE4, i_031de4),
    (0x031DEA, i_031dea),
    (0x031DF0, i_031df0),
    (0x031DF6, i_031df6),
    (0x031DFC, i_031dfc),
    (0x031DFE, i_031dfe),
    (0x031E04, i_031e04),
    (0x031E08, i_031e08),
    (0x031E0E, i_031e0e),
    (0x031E12, i_031e12),
    (0x031E16, i_031e16),
    (0x031E1A, i_031e1a),
    (0x031E1C, i_031e1c),
    (0x031E20, i_031e20),
    (0x031E26, i_031e26),
    (0x031E2C, i_031e2c),
    (0x031E30, i_031e30),
    (0x031E36, i_031e36),
    (0x031E38, i_031e38),
    (0x031E5E, i_031e5e),
    (0x031E62, i_031e62),
    (0x031E68, i_031e68),
    (0x031E6E, i_031e6e),
    (0x031E74, i_031e74),
    (0x031E76, i_031e76),
    (0x031E7A, i_031e7a),
    (0x031E80, i_031e80),
    (0x031E86, i_031e86),
    (0x031E8C, i_031e8c),
    (0x031E92, i_031e92),
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
    (0x03207C, i_03207c),
    (0x032080, i_032080),
    (0x032086, i_032086),
    (0x03208A, i_03208a),
    (0x032090, i_032090),
    (0x032096, i_032096),
    (0x03209A, i_03209a),
    (0x0320A0, i_0320a0),
    (0x0320A6, i_0320a6),
    (0x0320AA, i_0320aa),
    (0x0320AC, i_0320ac),
    (0x0320B2, i_0320b2),
    (0x0320B6, i_0320b6),
    (0x0320BA, i_0320ba),
    (0x0320BE, i_0320be),
    (0x0320C2, i_0320c2),
    (0x0320C8, i_0320c8),
    (0x0320CC, i_0320cc),
    (0x0320D0, i_0320d0),
    (0x0320D2, i_0320d2),
    (0x0320D4, i_0320d4),
    (0x0320D8, i_0320d8),
    (0x0320DC, i_0320dc),
    (0x0320E2, i_0320e2),
    (0x0320E6, i_0320e6),
    (0x0320E8, i_0320e8),
    (0x0320EC, i_0320ec),
    (0x0320F2, i_0320f2),
    (0x0320F6, i_0320f6),
    (0x0320FC, i_0320fc),
    (0x032100, i_032100),
    (0x032102, i_032102),
    (0x032106, i_032106),
    (0x03210C, i_03210c),
    (0x032110, i_032110),
    (0x032112, i_032112),
    (0x03211A, i_03211a),
    (0x03211E, i_03211e),
    (0x032124, i_032124),
    (0x032128, i_032128),
    (0x03212E, i_03212e),
    (0x032132, i_032132),
    (0x032138, i_032138),
    (0x03213E, i_03213e),
    (0x032142, i_032142),
    (0x03214A, i_03214a),
    (0x03214E, i_03214e),
    (0x032154, i_032154),
    (0x032158, i_032158),
    (0x03215E, i_03215e),
    (0x032162, i_032162),
    (0x032168, i_032168),
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
];
