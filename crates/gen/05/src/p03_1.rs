// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p03_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $031EA4: addi.w #$ffe0, $22(a0)
pub fn i_031ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EAA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031EAA: addi.w #$fff8, $24(a0)
pub fn i_031eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EB0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031EB0: bra.b $31e74
pub fn i_031eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EB2;
    cpu.ir = 0x60C2;
    cpu.pc = 0x31E74;
    10
}
/// $031EB2: lea.l $31e38(pc), a1
pub fn i_031eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31E38;
    16
}
/// $031EB6: jsr $4ae.l
pub fn i_031eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31EBC);
    cpu.pc = 0x4AE;
    16
}
/// $031EBC: jsr $5dd02.l
pub fn i_031ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31EC2);
    cpu.pc = 0x5DD02;
    16
}
/// $031EC2: addi.w #$20, $22(a0)
pub fn i_031ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EC8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031EC8: addi.w #$fff8, $24(a0)
pub fn i_031ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31ECE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031ECE: eori.b #$1, $3a(a0)
pub fn i_031ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31ED4;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $031ED4: bra.b $31e74
pub fn i_031ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31ED6;
    cpu.ir = 0x609E;
    cpu.pc = 0x31E74;
    10
}
/// $031ED6: lea.l $31e38(pc), a1
pub fn i_031ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31E38;
    16
}
/// $031EDA: jsr $4ae.l
pub fn i_031eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31EE0);
    cpu.pc = 0x4AE;
    16
}
/// $031EE0: jsr $5dd02.l
pub fn i_031ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31EE6);
    cpu.pc = 0x5DD02;
    16
}
/// $031EE6: addi.w #$ffe0, $22(a0)
pub fn i_031ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EEC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031EEC: addq.w #$8, $24(a0)
pub fn i_031eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EF0;
    cpu.ir = 0x5068;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031EF0: bra.b $31e74
pub fn i_031ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EF2;
    cpu.ir = 0x6082;
    cpu.pc = 0x31E74;
    10
}
/// $031EF2: lea.l $31e38(pc), a1
pub fn i_031ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31E38;
    16
}
/// $031EF6: jsr $4ae.l
pub fn i_031ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31EFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31EFC);
    cpu.pc = 0x4AE;
    16
}
/// $031EFC: jsr $5dd02.l
pub fn i_031efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31F02);
    cpu.pc = 0x5DD02;
    16
}
/// $031F02: addi.w #$20, $22(a0)
pub fn i_031f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F08;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031F08: addq.w #$8, $24(a0)
pub fn i_031f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F0C;
    cpu.ir = 0x5068;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $031F0C: eori.b #$1, $3a(a0)
pub fn i_031f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F12;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $031F12: bra.w $31e74
pub fn i_031f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F16;
    cpu.ir = 0x6000;
    cpu.pc = 0x31E74;
    10
}
/// $031F16: lea.l $31e38(pc), a1
pub fn i_031f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31E38;
    16
}
/// $031F1A: jsr $4ae.l
pub fn i_031f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31F20);
    cpu.pc = 0x4AE;
    16
}
/// $031F20: jsr $5dd02.l
pub fn i_031f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31F26);
    cpu.pc = 0x5DD02;
    16
}
/// $031F26: addi.w #$ffed, $22(a0)
pub fn i_031f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F2C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFED, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031F2C: addi.w #$fff4, $24(a0)
pub fn i_031f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F32;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031F32: bra.w $31e74
pub fn i_031f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F36;
    cpu.ir = 0x6000;
    cpu.pc = 0x31E74;
    10
}
/// $031F36: lea.l $31e38(pc), a1
pub fn i_031f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31E38;
    16
}
/// $031F3A: jsr $4ae.l
pub fn i_031f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31F40);
    cpu.pc = 0x4AE;
    16
}
/// $031F40: jsr $5dd02.l
pub fn i_031f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31F46);
    cpu.pc = 0x5DD02;
    16
}
/// $031F46: addi.w #$13, $22(a0)
pub fn i_031f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F4C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x13, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031F4C: addi.w #$fff4, $24(a0)
pub fn i_031f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F52;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031F52: eori.b #$1, $3a(a0)
pub fn i_031f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F58;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $031F58: bra.w $31e74
pub fn i_031f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x31E74;
    10
}
/// $031F5C: lea.l $31e38(pc), a1
pub fn i_031f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31E38;
    16
}
/// $031F60: jsr $4ae.l
pub fn i_031f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31F66);
    cpu.pc = 0x4AE;
    16
}
/// $031F66: jsr $5dd02.l
pub fn i_031f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31F6C);
    cpu.pc = 0x5DD02;
    16
}
/// $031F6C: addi.w #$ffed, $22(a0)
pub fn i_031f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F72;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFED, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031F72: addi.w #$c, $24(a0)
pub fn i_031f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F78;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031F78: bra.w $31e74
pub fn i_031f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x31E74;
    10
}
/// $031F7C: lea.l $31e38(pc), a1
pub fn i_031f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31E38;
    16
}
/// $031F80: jsr $4ae.l
pub fn i_031f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31F86);
    cpu.pc = 0x4AE;
    16
}
/// $031F86: jsr $5dd02.l
pub fn i_031f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31F8C);
    cpu.pc = 0x5DD02;
    16
}
/// $031F8C: addi.w #$13, $22(a0)
pub fn i_031f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F92;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x13, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031F92: addi.w #$c, $24(a0)
pub fn i_031f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F98;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $031F98: eori.b #$1, $3a(a0)
pub fn i_031f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31F9E;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $031F9E: bra.w $31e74
pub fn i_031f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x31E74;
    10
}
// ===== Fx_SmokeAnim_031fa2  ($031FA2, 32 B, player/tables_fx_030696.s)
/// $031FA2: move.w #$4, d1
pub fn i_031fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FA6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $031FA6: jsr $236e.l
pub fn i_031fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31FAC);
    cpu.pc = 0x236E;
    16
}
/// $031FAC: lea.l $2dd944.l, a0
pub fn i_031fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FB2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD944;
    20
}
/// $031FB2: jsr $28cd4.l
pub fn i_031fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x31FB8);
    cpu.pc = 0x28CD4;
    16
}
/// $031FB8: lea.l $31a9c(pc), a1
pub fn i_031fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x31A9C;
    16
}
/// $031FBC: move.l a1, (a6)
pub fn i_031fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $031FBE: bra.w $31a9c
pub fn i_031fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x31FC2;
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
// ===== Popcount4_0323b4  ($0323B4, 24 B, player/tables_fx_030696.s)
/// $0323B4: moveq #$0, d1
pub fn i_0323b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323B6;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0323B6: moveq #$0, d2
pub fn i_0323b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323B8;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0323B8: lsl.b #$1, d0
pub fn i_0323b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323BA;
    cpu.ir = 0xE308;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $0323BA: addx.b d2, d1
pub fn i_0323ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323BC;
    cpu.ir = 0xD302;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let h2 = cpu.d[2] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $0323BC: lsl.b #$1, d0
pub fn i_0323bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323BE;
    cpu.ir = 0xE308;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $0323BE: addx.b d2, d1
pub fn i_0323be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323C0;
    cpu.ir = 0xD302;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let h2 = cpu.d[2] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $0323C0: lsl.b #$1, d0
pub fn i_0323c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323C2;
    cpu.ir = 0xE308;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $0323C2: addx.b d2, d1
pub fn i_0323c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323C4;
    cpu.ir = 0xD302;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let h2 = cpu.d[2] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $0323C4: lsl.b #$1, d0
pub fn i_0323c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323C6;
    cpu.ir = 0xE308;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $0323C6: addx.b d2, d1
pub fn i_0323c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323C8;
    cpu.ir = 0xD302;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let h2 = cpu.d[2] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $0323C8: move.b d1, d0
pub fn i_0323c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323CA;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0323CA: rts
pub fn i_0323ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpDepthWithLink8B_0323e8  ($0323E8, 18 B, player/tables_fx_030696.s)
/// $0323E8: move.b d0, d3
pub fn i_0323e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323EA;
    cpu.ir = 0x1600;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0323EA: movea.l $8(a6), a1
pub fn i_0323ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323EE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0323EE: move.b $10(a6), d0
pub fn i_0323ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323F2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0323F2: cmp.b $10(a1), d0
pub fn i_0323f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323F6;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0323F6: bcs.w $32400
pub fn i_0323f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323FA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x32400; return 10; }
    8
}
// ===== ClearXN_0323fa  ($0323FA, 6 B, thunks/ccr_helpers.c)
/// $0323FA: andi.b #$ee, ccr
pub fn i_0323fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x323FE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0323FE: rts
pub fn i_0323fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32400;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_032400  ($032400, 6 B, thunks/ccr_helpers.c)
/// $032400: ori.b #$11, ccr
pub fn i_032400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32404;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $032404: rts
pub fn i_032404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32406;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
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
// ===== Entity_ClearCollisionCb_0329f8  ($0329F8, 10 B, player/tables_fx_030696.s)
/// $0329F8: move.l #$ffffffff, $48(a6)
pub fn i_0329f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A00;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $032A00: rts
pub fn i_032a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32A02;
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
// ===== Player_IncCounterAt84_032B36  ($032B36, 34 B, player/inc_counter_at84_032b36.s)
/// $032B36: cmpi.b #$1, $6d(a6)
pub fn i_032b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B3C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $032B3C: bne.w $32b4a
pub fn i_032b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32B4A; return 10; }
    8
}
/// $032B40: lea.l $100440.l, a1
pub fn i_032b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B46;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $032B46: bra.w $32b50
pub fn i_032b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B4A;
    cpu.ir = 0x6000;
    cpu.pc = 0x32B50;
    10
}
/// $032B4A: lea.l $1004e0.l, a1
pub fn i_032b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B50;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $032B50: addi.b #$1, $84(a1)
pub fn i_032b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B56;
    cpu.ir = 0x629;
    let ad1 = cpu.a[1].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $032B56: rts
pub fn i_032b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32B58;
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
// ===== Player_RefillAmmoClamp999_032bc6  ($032BC6, 76 B, player/core_032aa8.s)
/// $032BC6: cmpi.b #$0, d0
pub fn i_032bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BCA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $032BCA: bne.w $32bd8
pub fn i_032bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BCE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32BD8; return 10; }
    8
}
/// $032BCE: lea.l $100440.l, a1
pub fn i_032bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BD4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $032BD4: bra.w $32bde
pub fn i_032bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BD8;
    cpu.ir = 0x6000;
    cpu.pc = 0x32BDE;
    10
}
/// $032BD8: lea.l $1004e0.l, a1
pub fn i_032bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BDE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $032BDE: move.b $71(a1), d1
pub fn i_032bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BE2;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032BE2: cmpi.b #$0, d1
pub fn i_032be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BE6;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $032BE6: beq.w $32c10
pub fn i_032be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BEA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32C10; return 10; }
    8
}
/// $032BEA: andi.w #$ff, d1
pub fn i_032bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BEE;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $032BEE: lea.l $329d4(pc), a2
pub fn i_032bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BF2;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x329D4;
    16
}
/// $032BF2: lsl.w #$2, d1
pub fn i_032bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BF4;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $032BF4: move.w $82(a1), d0
pub fn i_032bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BF8;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $032BF8: add.w $2(a2, d1.w), d0
pub fn i_032bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32BFC;
    cpu.ir = 0xD072;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $032BFC: cmpi.w #$3e7, d0
pub fn i_032bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C00;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3E7, Size::Word, false);
    8
}
/// $032C00: ble.w $32c08
pub fn i_032c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C04;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x32C08; return 10; }
    8
}
/// $032C04: move.w #$3e7, d0
pub fn i_032c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C08;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3E7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $032C08: move.w d0, $82(a1)
pub fn i_032c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C0C;
    cpu.ir = 0x3340;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032C0C: bra.w $32c10
pub fn i_032c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C10;
    cpu.ir = 0x6000;
    cpu.pc = 0x32C10;
    10
}
/// $032C10: rts
pub fn i_032c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_GiveAmmo_ToPlayer_032c12  ($032C12, 100 B, player/core_032aa8.s)
/// $032C12: cmpi.b #$0, d0
pub fn i_032c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C16;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $032C16: bne.w $32c24
pub fn i_032c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C1A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32C24; return 10; }
    8
}
/// $032C1A: lea.l $100440.l, a1
pub fn i_032c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C20;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $032C20: bra.w $32c2a
pub fn i_032c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C24;
    cpu.ir = 0x6000;
    cpu.pc = 0x32C2A;
    10
}
/// $032C24: lea.l $1004e0.l, a1
pub fn i_032c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C2A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $032C2A: move.b $71(a1), d1
pub fn i_032c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C2E;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032C2E: cmpi.b #$4, d1
pub fn i_032c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C32;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $032C32: bne.w $32c3e
pub fn i_032c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C36;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32C3E; return 10; }
    8
}
/// $032C36: move.b $99(a6), d2
pub fn i_032c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C3A;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032C3A: bra.w $32c42
pub fn i_032c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x32C42;
    10
}
/// $032C3E: move.b $98(a6), d2
pub fn i_032c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C42;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032C42: cmpi.b #$ff, d2
pub fn i_032c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C46;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $032C46: beq.w $32c7c
pub fn i_032c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C4A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32C7C; return 10; }
    8
}
/// $032C4A: cmpi.b #$0, d1
pub fn i_032c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C4E;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $032C4E: beq.w $32c76
pub fn i_032c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C52;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32C76; return 10; }
    8
}
/// $032C52: move.w $82(a1), d0
pub fn i_032c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C56;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $032C56: moveq #$0, d1
pub fn i_032c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C58;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $032C58: move.b d2, d1
pub fn i_032c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C5A;
    cpu.ir = 0x1202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $032C5A: add.w d1, d0
pub fn i_032c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C5C;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $032C5C: cmpi.w #$3e7, d0
pub fn i_032c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C60;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3E7, Size::Word, false);
    8
}
/// $032C60: ble.w $32c68
pub fn i_032c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C64;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x32C68; return 10; }
    8
}
/// $032C64: move.w #$3e7, d0
pub fn i_032c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C68;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3E7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $032C68: move.w d0, $82(a1)
pub fn i_032c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C6C;
    cpu.ir = 0x3340;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $032C6C: jsr $9b9ba.l
pub fn i_032c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32C72);
    cpu.pc = 0x9B9BA;
    16
}
/// $032C72: bra.w $32c7c
pub fn i_032c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C76;
    cpu.ir = 0x6000;
    cpu.pc = 0x32C7C;
    10
}
// ===== JsrAbsThunk_032c76  ($032C76, 8 B, thunks/jsr_abs_thunks.c)
/// $032C76: jsr $9b9f6.l
pub fn i_032c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32C7C);
    cpu.pc = 0x9B9F6;
    16
}
/// $032C7C: rts
pub fn i_032c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_GiveBombs_ToPlayer_032c7e  ($032C7E, 60 B, player/core_032aa8.s)
/// $032C7E: cmpi.b #$0, d0
pub fn i_032c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C82;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $032C82: bne.w $32c90
pub fn i_032c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C86;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32C90; return 10; }
    8
}
/// $032C86: lea.l $100440.l, a1
pub fn i_032c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C8C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $032C8C: bra.w $32c96
pub fn i_032c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C90;
    cpu.ir = 0x6000;
    cpu.pc = 0x32C96;
    10
}
/// $032C90: lea.l $1004e0.l, a1
pub fn i_032c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C96;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $032C96: cmpi.b #$ff, $98(a6)
pub fn i_032c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32C9C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $032C9C: beq.w $32cb8
pub fn i_032c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CA0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32CB8; return 10; }
    8
}
/// $032CA0: move.b $80(a1), d0
pub fn i_032ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CA4;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032CA4: add.b $98(a6), d0
pub fn i_032ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CA8;
    cpu.ir = 0xD02E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $032CA8: cmpi.w #$63, d0
pub fn i_032ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CAC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x63, Size::Word, false);
    8
}
/// $032CAC: ble.w $32cb4
pub fn i_032cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CB0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x32CB4; return 10; }
    8
}
/// $032CB0: move.w #$63, d0
pub fn i_032cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CB4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x63;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $032CB4: move.b d0, $80(a1)
pub fn i_032cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CB8;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $032CB8: rts
pub fn i_032cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32CBA;
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
// ===== Player_IsSlotRidingReady_032de0  ($032DE0, 28 B, player/core_032aa8.s)
/// $032DE0: cmpa.l #$100440, a0
pub fn i_032de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DE6;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $032DE6: beq.b $32df2
pub fn i_032de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DE8;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32DF2; return 10; }
    8
}
/// $032DE8: cmpa.l #$1004e0, a0
pub fn i_032de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DEE;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $032DEE: bne.w $32e02
pub fn i_032dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DF2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32E02; return 10; }
    8
}
/// $032DF2: btst.b #$6, $8d(a0)
pub fn i_032df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DF8;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $032DF8: beq.w $32e02
pub fn i_032df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32DFC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32E02; return 10; }
    8
}
// ===== SetXN_032dfc  ($032DFC, 6 B, thunks/ccr_helpers.c)
/// $032DFC: ori.b #$11, ccr
pub fn i_032dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E00;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $032E00: rts
pub fn i_032e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_032e02  ($032E02, 6 B, thunks/ccr_helpers.c)
/// $032E02: andi.b #$ee, ccr
pub fn i_032e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E06;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $032E06: rts
pub fn i_032e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_SlotBit1Test_032e08  ($032E08, 18 B, player/core_032aa8.s)
/// $032E08: btst.b #$1, $8d(a0)
pub fn i_032e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E0E;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $032E0E: beq.w $32e1a
pub fn i_032e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E12;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32E1A; return 10; }
    8
}
/// $032E12: ori.b #$11, ccr
pub fn i_032e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E16;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $032E16: bra.w $32e1e
pub fn i_032e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x32E1E;
    10
}
// ===== ClearXN_032e1a  ($032E1A, 6 B, thunks/ccr_helpers.c)
/// $032E1A: andi.b #$ee, ccr
pub fn i_032e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E1E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $032E1E: rts
pub fn i_032e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Consume8B_032e20  ($032E20, 14 B, player/core_032aa8.s)
/// $032E20: tst.b $8b(a6)
pub fn i_032e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E24;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $032E24: beq.w $32e2c
pub fn i_032e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32E2C; return 10; }
    8
}
/// $032E28: clr.b $8b(a6)
pub fn i_032e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E2C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $032E2C: rts
pub fn i_032e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Consume8B_Alt_032e2e  ($032E2E, 14 B, player/core_032aa8.s)
/// $032E2E: tst.b $8b(a6)
pub fn i_032e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E32;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $032E32: beq.w $32e3a
pub fn i_032e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E36;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32E3A; return 10; }
    8
}
/// $032E36: clr.b $8b(a6)
pub fn i_032e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E3A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $032E3A: rts
pub fn i_032e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E3C;
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
// ===== Input_BackwardByFacing_032e5a  ($032E5A, 18 B, player/core_032aa8.s)
/// $032E5A: btst.b #$0, $3a(a6)
pub fn i_032e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E60;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $032E60: bne.w $32e68
pub fn i_032e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E64;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32E68; return 10; }
    8
}
/// $032E64: jmp $32e42(pc)
pub fn i_032e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E68;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x32E42;
    8
}
/// $032E68: jmp $32e3c(pc)
pub fn i_032e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E6C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x32E3C;
    8
}
// ===== Input_JumpThunk_032e6c  ($032E6C, 4 B, player/core_032aa8.s)
/// $032E6C: jmp $32e70(pc)
pub fn i_032e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E70;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x32E70;
    8
}
// ===== Input_JumpOrFire_032e70  ($032E70, 30 B, player/core_032aa8.s)
/// $032E70: jsr $5cf84.l
pub fn i_032e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32E76);
    cpu.pc = 0x5CF84;
    16
}
/// $032E76: bcc.w $32e7c
pub fn i_032e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x32E7C; return 10; }
    8
}
/// $032E7A: rts
pub fn i_032e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $032E7C: jsr $5ceec.l
pub fn i_032e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32E82);
    cpu.pc = 0x5CEEC;
    16
}
/// $032E82: bcc.w $32e8c
pub fn i_032e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E86;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x32E8C; return 10; }
    8
}
/// $032E86: jmp $5cf9c.l
pub fn i_032e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E8C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CF9C;
    8
}
/// $032E8C: rts
pub fn i_032e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32E8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
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
// ===== Player_PlayLifeMusic_032ed0  ($032ED0, 48 B, player/core_032aa8.s)
/// $032ED0: movem.l a0, -(a7)
pub fn i_032ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32ED4;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $032ED4: movea.l $c(a6), a0
pub fn i_032ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32ED8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $032ED8: cmpa.l #$100440, a0
pub fn i_032ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EDE;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $032EDE: bne.w $32ef0
pub fn i_032ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EE2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x32EF0; return 10; }
    8
}
/// $032EE2: move.w #$1123, d0
pub fn i_032ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EE6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1123;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $032EE6: jsr $2352.l
pub fn i_032ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32EEC);
    cpu.pc = 0x2352;
    16
}
/// $032EEC: bra.w $32efa
pub fn i_032eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EF0;
    cpu.ir = 0x6000;
    cpu.pc = 0x32EFA;
    10
}
/// $032EF0: move.w #$1087, d0
pub fn i_032ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EF4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1087;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $032EF4: jsr $2352.l
pub fn i_032ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32EFA);
    cpu.pc = 0x2352;
    16
}
/// $032EFA: movem.l (a7)+, a0
pub fn i_032efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32EFE;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $032EFE: rts
pub fn i_032efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_EffectHit40or80_032f00  ($032F00, 28 B, player/core_032aa8.s)
/// $032F00: jsr $27eba.l
pub fn i_032f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32F06);
    cpu.pc = 0x27EBA;
    16
}
/// $032F06: andi.b #$c0, d0
pub fn i_032f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F0A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $032F0A: cmpi.b #$40, d0
pub fn i_032f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F0E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $032F0E: beq.w $32f1c
pub fn i_032f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F12;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32F1C; return 10; }
    8
}
/// $032F12: cmpi.b #$80, d0
pub fn i_032f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F16;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x80, Size::Byte, false);
    8
}
/// $032F16: beq.w $32f1c
pub fn i_032f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F1A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32F1C; return 10; }
    8
}
/// $032F1A: rts
pub fn i_032f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00032F1C  ($032F1C, 2 B, thunks/trivial_rts.c)
/// $032F1C: rts
pub fn i_032f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_EffectHit40or80_B_032f1e  ($032F1E, 28 B, player/core_032aa8.s)
/// $032F1E: jsr $27eba.l
pub fn i_032f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x32F24);
    cpu.pc = 0x27EBA;
    16
}
/// $032F24: andi.b #$c0, d0
pub fn i_032f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F28;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $032F28: cmpi.b #$40, d0
pub fn i_032f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F2C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $032F2C: beq.w $32f3a
pub fn i_032f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F30;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32F3A; return 10; }
    8
}
/// $032F30: cmpi.b #$80, d0
pub fn i_032f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F34;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x80, Size::Byte, false);
    8
}
/// $032F34: beq.w $32f3a
pub fn i_032f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32F3A; return 10; }
    8
}
/// $032F38: rts
pub fn i_032f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00032F3A  ($032F3A, 2 B, thunks/trivial_rts.c)
/// $032F3A: rts
pub fn i_032f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32F3C;
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
// ===== Player_DecBombs_032fd4  ($032FD4, 14 B, player/core_032aa8.s)
/// $032FD4: moveq #$1, d1
pub fn i_032fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FD6;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $032FD6: move.b $80(a0), d0
pub fn i_032fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FDA;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $032FDA: beq.w $32fe8
pub fn i_032fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FDE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x32FE8; return 10; }
    8
}
/// $032FDE: sub.w d1, $80(a0)
pub fn i_032fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FE2;
    cpu.ir = 0x9368;
    let ad1 = cpu.a[0].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== ClearXN_032fe2  ($032FE2, 6 B, thunks/ccr_helpers.c)
/// $032FE2: andi.b #$ee, ccr
pub fn i_032fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FE6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $032FE6: rts
pub fn i_032fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_DecBombs_Zero_032fe8  ($032FE8, 2 B, player/core_032aa8.s)
/// $032FE8: clr.b d1
pub fn i_032fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FEA;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
// ===== SetXN_032fea  ($032FEA, 6 B, thunks/ccr_helpers.c)
/// $032FEA: ori.b #$11, ccr
pub fn i_032fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FEE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $032FEE: rts
pub fn i_032fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x32FF0;
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
// ===== Input_JumpByMode_033080  ($033080, 54 B, player/core_032aa8.s)
/// $033080: cmpi.b #$0, $106f2a.l
pub fn i_033080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33088;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $033088: bne.w $33096
pub fn i_033088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3308C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33096; return 10; }
    8
}
/// $03308C: jsr $5cdb4.l
pub fn i_03308c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33092;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33092);
    cpu.pc = 0x5CDB4;
    16
}
/// $033092: bra.w $330b0
pub fn i_033092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33096;
    cpu.ir = 0x6000;
    cpu.pc = 0x330B0;
    10
}
/// $033096: cmpi.b #$4, $106f2a.l
pub fn i_033096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3309E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F2A) as u32;
    sub_with_flags(cpu, v2, 0x4, Size::Byte, false);
    20
}
/// $03309E: bne.w $330ac
pub fn i_03309e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x330AC; return 10; }
    8
}
/// $0330A2: jsr $5cdb4.l
pub fn i_0330a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x330A8);
    cpu.pc = 0x5CDB4;
    16
}
/// $0330A8: bra.w $330b0
pub fn i_0330a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x330B0;
    10
}
/// $0330AC: jsr $32e6c(pc)
pub fn i_0330ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x330B0);
    cpu.pc = 0x32E6C;
    16
}
/// $0330B0: bcs.w $330bc
pub fn i_0330b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330B4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x330BC; return 10; }
    8
}
/// $0330B4: moveq #$0, d0
pub fn i_0330b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330B6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
// ===== ClearXN_0330b6  ($0330B6, 6 B, thunks/ccr_helpers.c)
/// $0330B6: andi.b #$ee, ccr
pub fn i_0330b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330BA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0330BA: rts
pub fn i_0330ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_JumpByMode_Hit_0330bc  ($0330BC, 8 B, player/core_032aa8.s)
/// $0330BC: jsr $5d5b6.l
pub fn i_0330bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x330C2);
    cpu.pc = 0x5D5B6;
    16
}
/// $0330C2: addq.w #$1, d0
pub fn i_0330c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330C4;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_0330c4  ($0330C4, 6 B, thunks/ccr_helpers.c)
/// $0330C4: ori.b #$11, ccr
pub fn i_0330c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330C8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0330C8: rts
pub fn i_0330c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x330CA;
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
/// $033140: bra.w $33148
pub fn i_033140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33144;
    cpu.ir = 0x6000;
    cpu.pc = 0x33148;
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
// ===== Player_ActionSelect_B_0331ae  ($0331AE, 26 B, player/core_032aa8.s)
/// $0331AE: btst.b #$2, $8c(a6)
pub fn i_0331ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331B4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0331B4: beq.w $331bc
pub fn i_0331b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x331BC; return 10; }
    8
}
/// $0331B8: jmp $331c6(pc)
pub fn i_0331b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331BC;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x331C6;
    8
}
/// $0331BC: jsr $5cef8.l
pub fn i_0331bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x331C2);
    cpu.pc = 0x5CEF8;
    16
}
/// $0331C2: bcs.w $331ce
pub fn i_0331c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x331CE; return 10; }
    8
}
/// $0331C6: clr.b d1
pub fn i_0331c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331C8;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
// ===== ClearXN_0331c8  ($0331C8, 6 B, thunks/ccr_helpers.c)
/// $0331C8: andi.b #$ee, ccr
pub fn i_0331c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331CC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0331CC: rts
pub fn i_0331cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x331CE;
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
/// $033240: bra.w $33248
pub fn i_033240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33244;
    cpu.ir = 0x6000;
    cpu.pc = 0x33248;
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
// ===== Stub_000332BA  ($0332BA, 2 B, thunks/trivial_rts.c)
/// $0332BA: rts
pub fn i_0332ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_ThrowGrenade_0332bc  ($0332BC, 96 B, player/core_032aa8.s)
/// $0332BC: movem.l a0, -(a7)
pub fn i_0332bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332C0;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0332C0: cmpa.l $100440.l, a6
pub fn i_0332c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332C6;
    cpu.ir = 0xBDF9;
    let v2 = bus.read32(0x100440);
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    22
}
/// $0332C6: beq.b $332d2
pub fn i_0332c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332C8;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x332D2; return 10; }
    8
}
/// $0332C8: cmpa.l $1004e0.l, a6
pub fn i_0332c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332CE;
    cpu.ir = 0xBDF9;
    let v2 = bus.read32(0x1004E0);
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v2, Size::Long, false);
    22
}
/// $0332CE: bne.w $332d8
pub fn i_0332ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x332D8; return 10; }
    8
}
/// $0332D2: movea.l a6, a0
pub fn i_0332d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332D4;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0332D4: bra.w $332dc
pub fn i_0332d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x332DC;
    10
}
/// $0332D8: movea.l $c(a6), a0
pub fn i_0332d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332DC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0332DC: cmpi.b #$0, $80(a0)
pub fn i_0332dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332E2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0332E2: beq.w $33316
pub fn i_0332e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33316; return 10; }
    8
}
/// $0332E6: subi.b #$1, $80(a0)
pub fn i_0332e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332EC;
    cpu.ir = 0x428;
    let ad1 = cpu.a[0].wrapping_add(0x80);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $0332EC: lea.l $33346(pc), a1
pub fn i_0332ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33346;
    16
}
/// $0332F0: jsr $5eab6.l
pub fn i_0332f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x332F6);
    cpu.pc = 0x5EAB6;
    16
}
/// $0332F6: jsr $5dd02.l
pub fn i_0332f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x332FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x332FC);
    cpu.pc = 0x5DD02;
    16
}
/// $0332FC: move.b $3a(a6), d0
pub fn i_0332fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33300;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $033300: move.b d0, $3a(a0)
pub fn i_033300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33304;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033304: jsr $517fe.l
pub fn i_033304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3330A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3330A);
    cpu.pc = 0x517FE;
    16
}
/// $03330A: subi.w #$10, $22(a0)
pub fn i_03330a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33310;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $033310: bset.b #$2, $8c(a6)
pub fn i_033310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33316;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033316: movem.l (a7)+, a0
pub fn i_033316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3331A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $03331A: rts
pub fn i_03331a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3331C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_ThrowGrenadeDown_03331c  ($03331C, 42 B, player/core_032aa8.s)
/// $03331C: lea.l $33352(pc), a1
pub fn i_03331c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33320;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33352;
    16
}
/// $033320: jsr $6fe.l
pub fn i_033320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33326;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33326);
    cpu.pc = 0x6FE;
    16
}
/// $033326: jsr $5dd02.l
pub fn i_033326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3332C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3332C);
    cpu.pc = 0x5DD02;
    16
}
/// $03332C: move.b $3a(a6), d0
pub fn i_03332c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33330;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $033330: eori.b #$1, d0
pub fn i_033330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33334;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $033334: move.b d0, $3a(a0)
pub fn i_033334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33338;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033338: jsr $517fe.l
pub fn i_033338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3333E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3333E);
    cpu.pc = 0x517FE;
    16
}
/// $03333E: bset.b #$2, $8c(a6)
pub fn i_03333e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33344;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033344: rts
pub fn i_033344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33346;
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
// ===== Player_JmpGrenadeBounce_03334c  ($03334C, 6 B, player/core_032aa8.s)
/// $03334C: jmp $28d9dc.l
pub fn i_03334c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33352;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x28D9DC;
    8
}
// ===== Player_JmpGrenadeDown_033352  ($033352, 6 B, player/core_032aa8.s)
/// $033352: jmp $28d7aa.l
pub fn i_033352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33358;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x28D7AA;
    8
}
// ===== Player_SpawnBloodFx_033358  ($033358, 22 B, player/core_032aa8.s)
/// $033358: tst.b $10fd8f.l
pub fn i_033358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3335E;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $03335E: beq.w $33374
pub fn i_03335e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33362;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33374; return 10; }
    8
}
/// $033362: lea.l $3d842.l, a1
pub fn i_033362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33368;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3D842;
    20
}
/// $033368: jsr $4ae.l
pub fn i_033368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3336E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3336E);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_03336e  ($03336E, 8 B, thunks/jsr_abs_thunks.c)
/// $03336E: jsr $5dd02.l
pub fn i_03336e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33374);
    cpu.pc = 0x5DD02;
    16
}
/// $033374: rts
pub fn i_033374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33376;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_SpawnTask3D8FA_033376  ($033376, 12 B, player/core_032aa8.s)
/// $033376: lea.l $3d8fa.l, a1
pub fn i_033376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3337C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3D8FA;
    20
}
/// $03337C: jsr $4ae.l
pub fn i_03337c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33382;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33382);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_033382  ($033382, 8 B, thunks/jsr_abs_thunks.c)
/// $033382: jsr $5dd02.l
pub fn i_033382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33388;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33388);
    cpu.pc = 0x5DD02;
    16
}
/// $033388: rts
pub fn i_033388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3338A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
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
// ===== EntityReset_Long48_Long60_033638  ($033638, 18 B, player/helpers_033638.s)
/// $033638: move.l #$ffffffff, $48(a6)
pub fn i_033638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33640;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033640: move.l #$ffffffff, $60(a6)
pub fn i_033640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33648;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033648: rts
pub fn i_033648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3364A;
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
// ===== SchedPublish_033638_A_0336B4  ($0336B4, 6 B, player/helpers_033638.s)
/// $0336B4: move.l #$33638, (a6)
pub fn i_0336b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336BA;
    cpu.ir = 0x2CBC;
    let v1: u32 = 0x33638;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Jsr5B6ThenJmpScheduler_0336ba  ($0336BA, 14 B, kernel/jsr5b6_jmp518.c)
/// $0336BA: jsr $5b6.l
pub fn i_0336ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x336C0);
    cpu.pc = 0x5B6;
    16
}
/// $0336C0: jmp $518.l
pub fn i_0336c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336C6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0336C6: rts
pub fn i_0336c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SchedPublish_033638_B_0336C8  ($0336C8, 6 B, player/helpers_033638.s)
/// $0336C8: move.l #$33638, (a6)
pub fn i_0336c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336CE;
    cpu.ir = 0x2CBC;
    let v1: u32 = 0x33638;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Jsr5B6ThenJmpScheduler_0336ce  ($0336CE, 14 B, kernel/jsr5b6_jmp518.c)
/// $0336CE: jsr $5b6.l
pub fn i_0336ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x336D4);
    cpu.pc = 0x5B6;
    16
}
/// $0336D4: jmp $518.l
pub fn i_0336d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0336DA: rts
pub fn i_0336da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_SpawnStart_0336dc  ($0336DC, 102 B, player/core_032aa8.s)
/// $0336DC: move.w #$50, $22(a6)
pub fn i_0336dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336E2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0336E2: move.w #$1e0, $24(a6)
pub fn i_0336e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336E8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0336E8: jsr $32ac8(pc)
pub fn i_0336e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x336EC);
    cpu.pc = 0x32AC8;
    16
}
/// $0336EC: movem.w d2, -(a7)
pub fn i_0336ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336F0;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    cpu.a[7] = a;
    20
}
/// $0336F0: move.w #$177, d1
pub fn i_0336f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336F4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x177;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0336F4: jsr $236e.l
pub fn i_0336f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x336FA);
    cpu.pc = 0x236E;
    16
}
/// $0336FA: move.w #$190, d1
pub fn i_0336fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x336FE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0336FE: jsr $236e.l
pub fn i_0336fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33704;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33704);
    cpu.pc = 0x236E;
    16
}
/// $033704: move.w #$192, d1
pub fn i_033704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33708;
    cpu.ir = 0x323C;
    let v1: u32 = 0x192;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $033708: jsr $236e.l
pub fn i_033708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3370E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3370E);
    cpu.pc = 0x236E;
    16
}
/// $03370E: move.w #$1c, $1c(a6)
pub fn i_03370e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33714;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033714: jsr $138fe.l
pub fn i_033714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3371A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3371A);
    cpu.pc = 0x138FE;
    16
}
/// $03371A: jsr $32a02(pc)
pub fn i_03371a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3371E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3371E);
    cpu.pc = 0x32A02;
    16
}
/// $03371E: bset.b #$4, $12(a6)
pub fn i_03371e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33724;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033724: ori.w #$0, $38(a6)
pub fn i_033724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3372A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03372A: lea.l $394e6.l, a1
pub fn i_03372a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33730;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x394E6;
    20
}
/// $033730: jsr $4ae.l
pub fn i_033730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33736;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33736);
    cpu.pc = 0x4AE;
    16
}
/// $033736: jsr $5dd02.l
pub fn i_033736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3373C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3373C);
    cpu.pc = 0x5DD02;
    16
}
/// $03373C: jsr $517fe.l
pub fn i_03373c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33742;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33742);
    cpu.pc = 0x517FE;
    16
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
// ===== Player_SpawnLand_03386e  ($03386E, 134 B, player/core_032aa8.s)
/// $03386E: move.w #$0, $7c(a6)
pub fn i_03386e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33874;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033874: move.w #$0, $7e(a6)
pub fn i_033874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3387A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03387A: bset.b #$7, $5b(a6)
pub fn i_03387a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33880;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033880: move.l #$32500, $60(a6)
pub fn i_033880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33888;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033888: clr.w $28(a6)
pub fn i_033888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3388C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03388C: clr.w $2c(a6)
pub fn i_03388c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33890;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033890: lea.l $33896(pc), a1
pub fn i_033890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33894;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33896;
    16
}
/// $033894: move.l a1, (a6)
pub fn i_033894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33896;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033896: jsr $32ff2(pc)
pub fn i_033896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3389A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3389A);
    cpu.pc = 0x32FF2;
    16
}
/// $03389A: jsr $32d32(pc)
pub fn i_03389a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3389E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3389E);
    cpu.pc = 0x32D32;
    16
}
/// $03389E: jsr $27a92.l
pub fn i_03389e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x338A4);
    cpu.pc = 0x27A92;
    16
}
/// $0338A4: jsr $32aa8(pc)
pub fn i_0338a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x338A8);
    cpu.pc = 0x32AA8;
    16
}
/// $0338A8: bcc.w $338e4
pub fn i_0338a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338AC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x338E4; return 10; }
    8
}
/// $0338AC: lea.l $34704(pc), a1
pub fn i_0338ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $0338B0: move.l a1, (a6)
pub fn i_0338b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0338B2: cmpa.l #$100440, a6
pub fn i_0338b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338B8;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0338B8: bne.w $338cc
pub fn i_0338b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338BC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x338CC; return 10; }
    8
}
/// $0338BC: lea.l $32112.l, a1
pub fn i_0338bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338C2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x32112;
    20
}
/// $0338C2: jsr $4ae.l
pub fn i_0338c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x338C8);
    cpu.pc = 0x4AE;
    16
}
/// $0338C8: bra.w $338d8
pub fn i_0338c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338CC;
    cpu.ir = 0x6000;
    cpu.pc = 0x338D8;
    10
}
/// $0338CC: lea.l $32142.l, a1
pub fn i_0338cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338D2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x32142;
    20
}
/// $0338D2: jsr $4ae.l
pub fn i_0338d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x338D8);
    cpu.pc = 0x4AE;
    16
}
/// $0338D8: move.b #$50, $45(a6)
pub fn i_0338d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338DE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0338DE: move.b #$50, $59(a6)
pub fn i_0338de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0338E4: jsr $27eba.l
pub fn i_0338e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x338EA);
    cpu.pc = 0x27EBA;
    16
}
/// $0338EA: bcc.w $338f4
pub fn i_0338ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x338F4; return 10; }
    8
}
/// $0338EE: lea.l $36c8c(pc), a1
pub fn i_0338ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36C8C;
    16
}
/// $0338F2: move.l a1, (a6)
pub fn i_0338f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_0338f4  ($0338F4, 6 B, thunks/jsr_pc_thunks.c)
/// $0338F4: jsr $33522(pc)
pub fn i_0338f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x338F8);
    cpu.pc = 0x33522;
    16
}
/// $0338F8: rts
pub fn i_0338f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x338FA;
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
/// $033C7C: jsr $32d32(pc)
pub fn i_033c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33C80);
    cpu.pc = 0x32D32;
    16
}
/// $033C80: bcc.w $33c8a
pub fn i_033c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33C8A; return 10; }
    8
}
/// $033C84: lea.l $345b8(pc), a1
pub fn i_033c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345B8;
    16
}
/// $033C88: move.l a1, (a6)
pub fn i_033c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033C8A: bra.w $341a4
pub fn i_033c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x341A4;
    10
}
/// $033C8E: move.w #$0, $7c(a6)
pub fn i_033c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C94;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033C94: move.w #$0, $7e(a6)
pub fn i_033c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33C9A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033C9A: bset.b #$2, $8c(a6)
pub fn i_033c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CA0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033CA0: bclr.b #$1, $8c(a6)
pub fn i_033ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CA6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033CA6: bclr.b #$3, $8c(a6)
pub fn i_033ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CAC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033CAC: move.b #$33, $70(a6)
pub fn i_033cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CB2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033CB2: lea.l $279702.l, a0
pub fn i_033cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279702;
    20
}
/// $033CB8: move.l -$4(a0), $74(a6)
pub fn i_033cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CBE;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $033CBE: move.b #$ff, $21(a6)
pub fn i_033cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CC4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033CC4: lea.l $279702.l, a0
pub fn i_033cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CCA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279702;
    20
}
/// $033CCA: jsr $28cd4.l
pub fn i_033cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33CD0);
    cpu.pc = 0x28CD4;
    16
}
/// $033CD0: lea.l $326e0(pc), a0
pub fn i_033cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CD4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $033CD4: move.l a0, $48(a6)
pub fn i_033cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CD8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033CD8: move.l #$32500, $60(a6)
pub fn i_033cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CE0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033CE0: jsr $267e6.l
pub fn i_033ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33CE6);
    cpu.pc = 0x267E6;
    16
}
/// $033CE6: clr.w $28(a6)
pub fn i_033ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CEA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033CEA: lea.l $33cf0(pc), a1
pub fn i_033cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33CF0;
    16
}
/// $033CEE: move.l a1, (a6)
pub fn i_033cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033CF0: jsr $32ff2(pc)
pub fn i_033cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33CF4);
    cpu.pc = 0x32FF2;
    16
}
/// $033CF4: jsr $32d32(pc)
pub fn i_033cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33CF8);
    cpu.pc = 0x32D32;
    16
}
/// $033CF8: jsr $27a92.l
pub fn i_033cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33CFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33CFE);
    cpu.pc = 0x27A92;
    16
}
/// $033CFE: jsr $32aa8(pc)
pub fn i_033cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33D02);
    cpu.pc = 0x32AA8;
    16
}
/// $033D02: bcc.w $33d42
pub fn i_033d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33D42; return 10; }
    8
}
/// $033D06: lea.l $34704(pc), a1
pub fn i_033d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $033D0A: move.l a1, (a6)
pub fn i_033d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033D0C: tst.w $28(a6)
pub fn i_033d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D10;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $033D10: beq.w $33d28
pub fn i_033d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D14;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33D28; return 10; }
    8
}
/// $033D14: ble.w $33d22
pub fn i_033d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D18;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x33D22; return 10; }
    8
}
/// $033D18: lea.l $34b38(pc), a1
pub fn i_033d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34B38;
    16
}
/// $033D1C: move.l a1, (a6)
pub fn i_033d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033D1E: bra.w $33d28
pub fn i_033d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D22;
    cpu.ir = 0x6000;
    cpu.pc = 0x33D28;
    10
}
/// $033D22: lea.l $34d32(pc), a1
pub fn i_033d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34D32;
    16
}
/// $033D26: move.l a1, (a6)
pub fn i_033d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033D28: jsr $5e9b6.l
pub fn i_033d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33D2E);
    cpu.pc = 0x5E9B6;
    16
}
/// $033D2E: andi.w #$7, d0
pub fn i_033d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D32;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $033D32: moveq #$4, d0
pub fn i_033d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D34;
    cpu.ir = 0x7004;
    cpu.d[0] = 0x4;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $033D34: cmp.w $82(a6), d0
pub fn i_033d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D38;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $033D38: bne.w $33d42
pub fn i_033d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D3C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33D42; return 10; }
    8
}
/// $033D3C: lea.l $33afc(pc), a1
pub fn i_033d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33AFC;
    16
}
/// $033D40: move.l a1, (a6)
pub fn i_033d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033D42: btst.b #$2, $8c(a6)
pub fn i_033d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D48;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $033D48: bne.w $33d52
pub fn i_033d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D4C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33D52; return 10; }
    8
}
/// $033D4C: move.b #$1, $85(a6)
pub fn i_033d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033D52: jsr $32d32(pc)
pub fn i_033d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33D56);
    cpu.pc = 0x32D32;
    16
}
/// $033D56: bcc.w $33d60
pub fn i_033d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D5A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33D60; return 10; }
    8
}
/// $033D5A: lea.l $345b8(pc), a1
pub fn i_033d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345B8;
    16
}
/// $033D5E: move.l a1, (a6)
pub fn i_033d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033D60: bra.w $341a4
pub fn i_033d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D64;
    cpu.ir = 0x6000;
    cpu.pc = 0x341A4;
    10
}
// ===== Player_Idle_033d64  ($033D64, 1368 B, player/core_032aa8.s)
/// $033D64: move.w #$0, $7c(a6)
pub fn i_033d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D6A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033D6A: move.w #$0, $7e(a6)
pub fn i_033d6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033D70: bset.b #$2, $8c(a6)
pub fn i_033d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D76;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033D76: bclr.b #$1, $8c(a6)
pub fn i_033d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D7C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033D7C: bclr.b #$3, $8c(a6)
pub fn i_033d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D82;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033D82: cmpi.b #$13, $70(a6)
pub fn i_033d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D88;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x13, Size::Byte, false);
    16
}
/// $033D88: bcs.w $33db6
pub fn i_033d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D8C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x33DB6; return 10; }
    8
}
/// $033D8C: cmpi.b #$24, $70(a6)
pub fn i_033d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D92;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $033D92: bcc.w $33db6
pub fn i_033d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33DB6; return 10; }
    8
}
/// $033D96: cmpi.b #$0, $71(a6)
pub fn i_033d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33D9C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $033D9C: beq.w $33db6
pub fn i_033d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DA0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33DB6; return 10; }
    8
}
/// $033DA0: move.b #$21, $70(a6)
pub fn i_033da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DA6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x21;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033DA6: lea.l $279828.l, a0
pub fn i_033da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DAC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279828;
    20
}
/// $033DAC: move.l -$4(a0), $74(a6)
pub fn i_033dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DB2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $033DB2: bra.w $33dda
pub fn i_033db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x33DDA;
    10
}
/// $033DB6: move.b #$21, $70(a6)
pub fn i_033db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DBC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x21;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033DBC: lea.l $279828.l, a0
pub fn i_033dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DC2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279828;
    20
}
/// $033DC2: move.l -$4(a0), $74(a6)
pub fn i_033dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DC8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $033DC8: move.b #$ff, $21(a6)
pub fn i_033dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DCE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033DCE: lea.l $279828.l, a0
pub fn i_033dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279828;
    20
}
/// $033DD4: jsr $28cd4.l
pub fn i_033dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33DDA);
    cpu.pc = 0x28CD4;
    16
}
/// $033DDA: lea.l $326e0(pc), a0
pub fn i_033dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DDE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $033DDE: move.l a0, $48(a6)
pub fn i_033dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DE2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033DE2: cmpi.b #$0, $71(a6)
pub fn i_033de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DE8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $033DE8: beq.w $33e04
pub fn i_033de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DEC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33E04; return 10; }
    8
}
/// $033DEC: lea.l $ffff.w, a0
pub fn i_033dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DF0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $033DF0: move.l a0, $48(a6)
pub fn i_033df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DF4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033DF4: lea.l $38be4(pc), a1
pub fn i_033df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38BE4;
    16
}
/// $033DF8: jsr $4ae.l
pub fn i_033df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33DFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33DFE);
    cpu.pc = 0x4AE;
    16
}
/// $033DFE: jsr $5dd02.l
pub fn i_033dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33E04);
    cpu.pc = 0x5DD02;
    16
}
/// $033E04: move.b #$0, $71(a6)
pub fn i_033e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E0A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033E0A: move.w #$a, $82(a6)
pub fn i_033e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E10;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033E10: bset.b #$7, $5b(a6)
pub fn i_033e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E16;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033E16: move.l #$32500, $60(a6)
pub fn i_033e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E1E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033E1E: jsr $267e6.l
pub fn i_033e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33E24);
    cpu.pc = 0x267E6;
    16
}
/// $033E24: clr.w $28(a6)
pub fn i_033e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E28;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033E28: lea.l $33e2e(pc), a1
pub fn i_033e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33E2E;
    16
}
/// $033E2C: move.l a1, (a6)
pub fn i_033e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033E2E: btst.b #$4, $8c(a6)
pub fn i_033e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E34;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $033E34: beq.w $33e46
pub fn i_033e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33E46; return 10; }
    8
}
/// $033E38: jsr $32d32(pc)
pub fn i_033e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33E3C);
    cpu.pc = 0x32D32;
    16
}
/// $033E3C: lea.l $345b8(pc), a1
pub fn i_033e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345B8;
    16
}
/// $033E40: move.l a1, (a6)
pub fn i_033e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033E42: bra.w $345b8
pub fn i_033e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E46;
    cpu.ir = 0x6000;
    cpu.pc = 0x345B8;
    10
}
/// $033E46: jsr $32ff2(pc)
pub fn i_033e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33E4A);
    cpu.pc = 0x32FF2;
    16
}
/// $033E4A: jsr $27a92.l
pub fn i_033e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33E50);
    cpu.pc = 0x27A92;
    16
}
/// $033E50: jsr $32aa8(pc)
pub fn i_033e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33E54);
    cpu.pc = 0x32AA8;
    16
}
/// $033E54: bcc.w $33e7a
pub fn i_033e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33E7A; return 10; }
    8
}
/// $033E58: lea.l $34704(pc), a1
pub fn i_033e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $033E5C: move.l a1, (a6)
pub fn i_033e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033E5E: tst.w $28(a6)
pub fn i_033e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E62;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $033E62: beq.w $33e7a
pub fn i_033e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E66;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33E7A; return 10; }
    8
}
/// $033E66: ble.w $33e74
pub fn i_033e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E6A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x33E74; return 10; }
    8
}
/// $033E6A: lea.l $34b38(pc), a1
pub fn i_033e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34B38;
    16
}
/// $033E6E: move.l a1, (a6)
pub fn i_033e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033E70: bra.w $33e7a
pub fn i_033e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E74;
    cpu.ir = 0x6000;
    cpu.pc = 0x33E7A;
    10
}
/// $033E74: lea.l $34d32(pc), a1
pub fn i_033e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34D32;
    16
}
/// $033E78: move.l a1, (a6)
pub fn i_033e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033E7A: btst.b #$2, $8c(a6)
pub fn i_033e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E80;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $033E80: bne.w $33e88
pub fn i_033e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33E88; return 10; }
    8
}
/// $033E84: bra.w $341a4
pub fn i_033e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E88;
    cpu.ir = 0x6000;
    cpu.pc = 0x341A4;
    10
}
/// $033E88: bra.w $341a4
pub fn i_033e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E8C;
    cpu.ir = 0x6000;
    cpu.pc = 0x341A4;
    10
}
/// $033E8C: cmpa.l #$100440, a6
pub fn i_033e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E92;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $033E92: bne.w $33ea6
pub fn i_033e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33EA6; return 10; }
    8
}
/// $033E96: lea.l $32112.l, a1
pub fn i_033e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33E9C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x32112;
    20
}
/// $033E9C: jsr $4ae.l
pub fn i_033e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33EA2);
    cpu.pc = 0x4AE;
    16
}
/// $033EA2: bra.w $33eb2
pub fn i_033ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x33EB2;
    10
}
/// $033EA6: lea.l $32142.l, a1
pub fn i_033ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EAC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x32142;
    20
}
/// $033EAC: jsr $4ae.l
pub fn i_033eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33EB2);
    cpu.pc = 0x4AE;
    16
}
/// $033EB2: move.b #$50, $45(a6)
pub fn i_033eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EB8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033EB8: move.b #$50, $59(a6)
pub fn i_033eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EBE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033EBE: jmp $33ede(pc)
pub fn i_033ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EC2;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x33EDE;
    8
}
/// $033EC2: cmpi.b #$0, $106ed3.l
pub fn i_033ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33ECA;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ED3) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $033ECA: beq.w $33ed4
pub fn i_033eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33ECE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33ED4; return 10; }
    8
}
/// $033ECE: move.b #$1e, $45(a6)
pub fn i_033ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33ED4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033ED4: move.b #$1e, $59(a6)
pub fn i_033ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033EDA: jmp $33ede(pc)
pub fn i_033eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EDE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x33EDE;
    8
}
/// $033EDE: bclr.b #$0, $8c(a6)
pub fn i_033ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EE4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033EE4: cmpi.w #$2, $72(a6)
pub fn i_033ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EEA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $033EEA: bne.w $33ef4
pub fn i_033eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EEE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33EF4; return 10; }
    8
}
/// $033EEE: move.w #$0, $72(a6)
pub fn i_033eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EF4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033EF4: move.w #$0, $7c(a6)
pub fn i_033ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33EFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033EFA: move.w #$0, $7e(a6)
pub fn i_033efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F00;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $033F00: bclr.b #$2, $8c(a6)
pub fn i_033f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F06;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033F06: bclr.b #$1, $8c(a6)
pub fn i_033f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F0C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033F0C: bclr.b #$3, $8c(a6)
pub fn i_033f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F12;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033F12: move.b #$1, $70(a6)
pub fn i_033f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F18;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033F18: lea.l $279a96.l, a0
pub fn i_033f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F1E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A96;
    20
}
/// $033F1E: move.l -$4(a0), $74(a6)
pub fn i_033f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F24;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $033F24: move.b #$ff, $21(a6)
pub fn i_033f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F2A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033F2A: lea.l $279a96.l, a0
pub fn i_033f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A96;
    20
}
/// $033F30: jsr $28cd4.l
pub fn i_033f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33F36);
    cpu.pc = 0x28CD4;
    16
}
/// $033F36: bset.b #$7, $5b(a6)
pub fn i_033f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F3C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $033F3C: jsr $32e90(pc)
pub fn i_033f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33F40);
    cpu.pc = 0x32E90;
    16
}
/// $033F40: bcc.w $33f48
pub fn i_033f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F44;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33F48; return 10; }
    8
}
/// $033F44: jmp $3827a(pc)
pub fn i_033f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F48;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3827A;
    8
}
/// $033F48: cmpi.w #$0, $82(a6)
pub fn i_033f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F4E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $033F4E: bne.w $33f5c
pub fn i_033f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33F5C; return 10; }
    8
}
/// $033F52: lea.l $33d64(pc), a1
pub fn i_033f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $033F56: move.l a1, (a6)
pub fn i_033f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033F58: bra.w $33d64
pub fn i_033f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x33D64;
    10
}
/// $033F5C: cmpi.b #$0, $85(a6)
pub fn i_033f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F62;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $033F62: bne.w $33f7a
pub fn i_033f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F66;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33F7A; return 10; }
    8
}
/// $033F66: cmpi.b #$1, $71(a6)
pub fn i_033f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F6C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $033F6C: bne.w $33f7a
pub fn i_033f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F70;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x33F7A; return 10; }
    8
}
/// $033F70: lea.l $33b9a(pc), a1
pub fn i_033f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33B9A;
    16
}
/// $033F74: move.l a1, (a6)
pub fn i_033f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033F76: bra.w $33b9a
pub fn i_033f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F7A;
    cpu.ir = 0x6000;
    cpu.pc = 0x33B9A;
    10
}
/// $033F7A: btst.b #$0, $88(a6)
pub fn i_033f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F80;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $033F80: beq.w $33f8e
pub fn i_033f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F84;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33F8E; return 10; }
    8
}
/// $033F84: lea.l $33a5e(pc), a1
pub fn i_033f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $033F88: move.l a1, (a6)
pub fn i_033f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033F8A: bra.w $33a5e
pub fn i_033f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x33A5E;
    10
}
/// $033F8E: move.l #$32500, $60(a6)
pub fn i_033f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F96;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033F96: jsr $267e6.l
pub fn i_033f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33F9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33F9C);
    cpu.pc = 0x267E6;
    16
}
/// $033F9C: clr.w $28(a6)
pub fn i_033f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FA0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $033FA0: lea.l $326e0(pc), a0
pub fn i_033fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FA4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $033FA4: move.l a0, $48(a6)
pub fn i_033fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FA8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $033FA8: lea.l $33fae(pc), a1
pub fn i_033fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33FAE;
    16
}
/// $033FAC: move.l a1, (a6)
pub fn i_033fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033FAE: jsr $32ff2(pc)
pub fn i_033fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33FB2);
    cpu.pc = 0x32FF2;
    16
}
/// $033FB2: jsr $32d32(pc)
pub fn i_033fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FB6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33FB6);
    cpu.pc = 0x32D32;
    16
}
/// $033FB6: bcc.w $33fc0
pub fn i_033fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x33FC0; return 10; }
    8
}
/// $033FBA: lea.l $345b8(pc), a1
pub fn i_033fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345B8;
    16
}
/// $033FBE: move.l a1, (a6)
pub fn i_033fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033FC0: jsr $27a92.l
pub fn i_033fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x33FC6);
    cpu.pc = 0x27A92;
    16
}
/// $033FC6: jsr $32aa8(pc)
pub fn i_033fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x33FCA);
    cpu.pc = 0x32AA8;
    16
}
/// $033FCA: bcc.w $34042
pub fn i_033fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34042; return 10; }
    8
}
/// $033FCE: lea.l $34704(pc), a1
pub fn i_033fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $033FD2: move.l a1, (a6)
pub fn i_033fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033FD4: move.b #$1, $78(a6)
pub fn i_033fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033FDA: lsl.b #$4, d0
pub fn i_033fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FDC;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $033FDC: or.b $78(a6), d0
pub fn i_033fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FE0;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $033FE0: move.b d0, $79(a6)
pub fn i_033fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FE4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $033FE4: btst.b #$0, $88(a6)
pub fn i_033fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FEA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $033FEA: beq.w $33ff4
pub fn i_033fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x33FF4; return 10; }
    8
}
/// $033FEE: lea.l $33a5e(pc), a1
pub fn i_033fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $033FF2: move.l a1, (a6)
pub fn i_033ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $033FF4: cmpi.b #$0, $85(a6)
pub fn i_033ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FFA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $033FFA: bne.w $3400e
pub fn i_033ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x33FFE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3400E; return 10; }
    8
}
/// $033FFE: cmpi.b #$1, $71(a6)
pub fn i_033ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34004;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $034004: bne.w $3400e
pub fn i_034004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34008;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3400E; return 10; }
    8
}
/// $034008: lea.l $33b9a(pc), a1
pub fn i_034008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3400C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33B9A;
    16
}
/// $03400C: move.l a1, (a6)
pub fn i_03400c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3400E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03400E: jsr $5e9b6.l
pub fn i_03400e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34014);
    cpu.pc = 0x5E9B6;
    16
}
/// $034014: andi.w #$7, d0
pub fn i_034014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34018;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $034018: moveq #$4, d0
pub fn i_034018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3401A;
    cpu.ir = 0x7004;
    cpu.d[0] = 0x4;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03401A: cmpi.b #$1, $70(a6)
pub fn i_03401a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34020;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $034020: bne.w $34032
pub fn i_034020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34024;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34032; return 10; }
    8
}
/// $034024: cmp.w $82(a6), d0
pub fn i_034024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34028;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $034028: bne.w $34032
pub fn i_034028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3402C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34032; return 10; }
    8
}
/// $03402C: lea.l $33afc(pc), a1
pub fn i_03402c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34030;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33AFC;
    16
}
/// $034030: move.l a1, (a6)
pub fn i_034030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34032;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034032: cmpi.w #$0, $82(a6)
pub fn i_034032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34038;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $034038: bne.w $34042
pub fn i_034038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3403C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34042; return 10; }
    8
}
/// $03403C: lea.l $33d64(pc), a1
pub fn i_03403c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34040;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $034040: move.l a1, (a6)
pub fn i_034040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34042;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034042: jmp $341a4(pc)
pub fn i_034042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34046;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x341A4;
    8
}
/// $034046: cmpi.b #$13, $70(a6)
pub fn i_034046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3404C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x13, Size::Byte, false);
    16
}
/// $03404C: bcs.w $3405e
pub fn i_03404c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34050;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3405E; return 10; }
    8
}
/// $034050: cmpi.b #$24, $70(a6)
pub fn i_034050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34056;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $034056: bcc.w $3405e
pub fn i_034056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3405A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3405E; return 10; }
    8
}
/// $03405A: jmp $34704(pc)
pub fn i_03405a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3405E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x34704;
    8
}
/// $03405E: cmpi.w #$1, $72(a6)
pub fn i_03405e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34064;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $034064: bne.w $3406c
pub fn i_034064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34068;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3406C; return 10; }
    8
}
/// $034068: jmp $34704(pc)
pub fn i_034068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3406C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x34704;
    8
}
/// $03406C: move.w #$0, $7c(a6)
pub fn i_03406c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34072;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034072: move.w #$0, $7e(a6)
pub fn i_034072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34078;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034078: move.b #$5, $70(a6)
pub fn i_034078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3407E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03407E: lea.l $279a96.l, a0
pub fn i_03407e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34084;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A96;
    20
}
/// $034084: move.l -$4(a0), $74(a6)
pub fn i_034084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3408A;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03408A: move.b #$ff, $21(a6)
pub fn i_03408a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34090;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034090: lea.l $279a96.l, a0
pub fn i_034090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34096;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A96;
    20
}
/// $034096: jsr $28cd4.l
pub fn i_034096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3409C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3409C);
    cpu.pc = 0x28CD4;
    16
}
/// $03409C: bclr.b #$2, $8c(a6)
pub fn i_03409c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340A2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0340A2: bclr.b #$1, $8c(a6)
pub fn i_0340a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340A8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0340A8: bclr.b #$3, $8c(a6)
pub fn i_0340a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340AE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0340AE: bset.b #$7, $5b(a6)
pub fn i_0340ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340B4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0340B4: move.l #$32500, $60(a6)
pub fn i_0340b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340BC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0340BC: jsr $267e6.l
pub fn i_0340bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x340C2);
    cpu.pc = 0x267E6;
    16
}
/// $0340C2: clr.w $28(a6)
pub fn i_0340c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340C6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0340C6: lea.l $326e0(pc), a0
pub fn i_0340c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340CA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $0340CA: move.l a0, $48(a6)
pub fn i_0340ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340CE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0340CE: cmpi.w #$0, $82(a6)
pub fn i_0340ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340D4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0340D4: bne.w $340e2
pub fn i_0340d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x340E2; return 10; }
    8
}
/// $0340D8: lea.l $33d64(pc), a1
pub fn i_0340d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $0340DC: move.l a1, (a6)
pub fn i_0340dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0340DE: bra.w $33d64
pub fn i_0340de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x33D64;
    10
}
/// $0340E2: cmpi.b #$0, $85(a6)
pub fn i_0340e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340E8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0340E8: bne.w $34100
pub fn i_0340e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340EC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34100; return 10; }
    8
}
/// $0340EC: cmpi.b #$1, $71(a6)
pub fn i_0340ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340F2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0340F2: bne.w $34100
pub fn i_0340f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34100; return 10; }
    8
}
/// $0340F6: lea.l $33b9a(pc), a1
pub fn i_0340f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33B9A;
    16
}
/// $0340FA: move.l a1, (a6)
pub fn i_0340fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x340FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0340FC: bra.w $33b9a
pub fn i_0340fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34100;
    cpu.ir = 0x6000;
    cpu.pc = 0x33B9A;
    10
}
/// $034100: btst.b #$0, $88(a6)
pub fn i_034100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34106;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $034106: beq.w $34114
pub fn i_034106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3410A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x34114; return 10; }
    8
}
/// $03410A: lea.l $33a5e(pc), a1
pub fn i_03410a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3410E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $03410E: move.l a1, (a6)
pub fn i_03410e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34110;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034110: bra.w $33a5e
pub fn i_034110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34114;
    cpu.ir = 0x6000;
    cpu.pc = 0x33A5E;
    10
}
/// $034114: lea.l $3411a(pc), a1
pub fn i_034114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34118;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3411A;
    16
}
/// $034118: move.l a1, (a6)
pub fn i_034118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3411A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03411A: jsr $32ff2(pc)
pub fn i_03411a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3411E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3411E);
    cpu.pc = 0x32FF2;
    16
}
/// $03411E: jsr $32d32(pc)
pub fn i_03411e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34122;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34122);
    cpu.pc = 0x32D32;
    16
}
/// $034122: jsr $27a92.l
pub fn i_034122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34128;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34128);
    cpu.pc = 0x27A92;
    16
}
/// $034128: jsr $32aa8(pc)
pub fn i_034128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3412C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3412C);
    cpu.pc = 0x32AA8;
    16
}
/// $03412C: bcc.w $341a4
pub fn i_03412c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34130;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x341A4; return 10; }
    8
}
/// $034130: lea.l $34704(pc), a1
pub fn i_034130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34134;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $034134: move.l a1, (a6)
pub fn i_034134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34136;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034136: move.b #$1, $78(a6)
pub fn i_034136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3413C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03413C: lsl.b #$4, d0
pub fn i_03413c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3413E;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $03413E: or.b $78(a6), d0
pub fn i_03413e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34142;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $034142: move.b d0, $79(a6)
pub fn i_034142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34146;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034146: btst.b #$0, $88(a6)
pub fn i_034146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3414C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03414C: beq.w $34156
pub fn i_03414c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34150;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x34156; return 10; }
    8
}
/// $034150: lea.l $33a5e(pc), a1
pub fn i_034150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34154;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $034154: move.l a1, (a6)
pub fn i_034154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34156;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034156: cmpi.b #$0, $85(a6)
pub fn i_034156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3415C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03415C: bne.w $34170
pub fn i_03415c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34160;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34170; return 10; }
    8
}
/// $034160: cmpi.b #$1, $71(a6)
pub fn i_034160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34166;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $034166: bne.w $34170
pub fn i_034166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3416A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34170; return 10; }
    8
}
/// $03416A: lea.l $33b9a(pc), a1
pub fn i_03416a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3416E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33B9A;
    16
}
/// $03416E: move.l a1, (a6)
pub fn i_03416e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34170;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034170: jsr $5e9b6.l
pub fn i_034170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34176;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34176);
    cpu.pc = 0x5E9B6;
    16
}
/// $034176: andi.w #$7, d0
pub fn i_034176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3417A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03417A: moveq #$4, d0
pub fn i_03417a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3417C;
    cpu.ir = 0x7004;
    cpu.d[0] = 0x4;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03417C: cmpi.b #$1, $70(a6)
pub fn i_03417c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34182;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $034182: bne.w $34194
pub fn i_034182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34186;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34194; return 10; }
    8
}
/// $034186: cmp.w $82(a6), d0
pub fn i_034186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3418A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $03418A: bne.w $34194
pub fn i_03418a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3418E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34194; return 10; }
    8
}
/// $03418E: lea.l $33afc(pc), a1
pub fn i_03418e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34192;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33AFC;
    16
}
/// $034192: move.l a1, (a6)
pub fn i_034192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34194;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034194: cmpi.w #$0, $82(a6)
pub fn i_034194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3419A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03419A: bne.w $341a4
pub fn i_03419a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3419E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x341A4; return 10; }
    8
}
/// $03419E: lea.l $33d64(pc), a1
pub fn i_03419e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $0341A2: move.l a1, (a6)
pub fn i_0341a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0341A4: btst.b #$2, $8c(a6)
pub fn i_0341a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341AA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0341AA: bne.w $3425c
pub fn i_0341aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3425C; return 10; }
    8
}
/// $0341AE: btst.b #$0, $3a(a6)
pub fn i_0341ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341B4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0341B4: bne.w $341d8
pub fn i_0341b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x341D8; return 10; }
    8
}
/// $0341B8: jsr $32e3c(pc)
pub fn i_0341b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x341BC);
    cpu.pc = 0x32E3C;
    16
}
/// $0341BC: bcc.w $341c6
pub fn i_0341bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341C0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x341C6; return 10; }
    8
}
/// $0341C0: lea.l $34b38(pc), a1
pub fn i_0341c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34B38;
    16
}
/// $0341C4: move.l a1, (a6)
pub fn i_0341c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0341C6: jsr $32e42(pc)
pub fn i_0341c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x341CA);
    cpu.pc = 0x32E42;
    16
}
/// $0341CA: bcc.w $341d4
pub fn i_0341ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x341D4; return 10; }
    8
}
/// $0341CE: lea.l $35bf8(pc), a1
pub fn i_0341ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35BF8;
    16
}
/// $0341D2: move.l a1, (a6)
pub fn i_0341d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0341D4: bra.w $341f4
pub fn i_0341d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x341F4;
    10
}
/// $0341D8: jsr $32e3c(pc)
pub fn i_0341d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x341DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x341DC);
    cpu.pc = 0x32E3C;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x031EA4, i_031ea4),
    (0x031EAA, i_031eaa),
    (0x031EB0, i_031eb0),
    (0x031EB2, i_031eb2),
    (0x031EB6, i_031eb6),
    (0x031EBC, i_031ebc),
    (0x031EC2, i_031ec2),
    (0x031EC8, i_031ec8),
    (0x031ECE, i_031ece),
    (0x031ED4, i_031ed4),
    (0x031ED6, i_031ed6),
    (0x031EDA, i_031eda),
    (0x031EE0, i_031ee0),
    (0x031EE6, i_031ee6),
    (0x031EEC, i_031eec),
    (0x031EF0, i_031ef0),
    (0x031EF2, i_031ef2),
    (0x031EF6, i_031ef6),
    (0x031EFC, i_031efc),
    (0x031F02, i_031f02),
    (0x031F08, i_031f08),
    (0x031F0C, i_031f0c),
    (0x031F12, i_031f12),
    (0x031F16, i_031f16),
    (0x031F1A, i_031f1a),
    (0x031F20, i_031f20),
    (0x031F26, i_031f26),
    (0x031F2C, i_031f2c),
    (0x031F32, i_031f32),
    (0x031F36, i_031f36),
    (0x031F3A, i_031f3a),
    (0x031F40, i_031f40),
    (0x031F46, i_031f46),
    (0x031F4C, i_031f4c),
    (0x031F52, i_031f52),
    (0x031F58, i_031f58),
    (0x031F5C, i_031f5c),
    (0x031F60, i_031f60),
    (0x031F66, i_031f66),
    (0x031F6C, i_031f6c),
    (0x031F72, i_031f72),
    (0x031F78, i_031f78),
    (0x031F7C, i_031f7c),
    (0x031F80, i_031f80),
    (0x031F86, i_031f86),
    (0x031F8C, i_031f8c),
    (0x031F92, i_031f92),
    (0x031F98, i_031f98),
    (0x031F9E, i_031f9e),
    (0x031FA2, i_031fa2),
    (0x031FA6, i_031fa6),
    (0x031FAC, i_031fac),
    (0x031FB2, i_031fb2),
    (0x031FB8, i_031fb8),
    (0x031FBC, i_031fbc),
    (0x031FBE, i_031fbe),
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
    (0x0321B4, i_0321b4),
    (0x0321B8, i_0321b8),
    (0x0321BA, i_0321ba),
    (0x0321BC, i_0321bc),
    (0x0323B4, i_0323b4),
    (0x0323B6, i_0323b6),
    (0x0323B8, i_0323b8),
    (0x0323BA, i_0323ba),
    (0x0323BC, i_0323bc),
    (0x0323BE, i_0323be),
    (0x0323C0, i_0323c0),
    (0x0323C2, i_0323c2),
    (0x0323C4, i_0323c4),
    (0x0323C6, i_0323c6),
    (0x0323C8, i_0323c8),
    (0x0323CA, i_0323ca),
    (0x0323E8, i_0323e8),
    (0x0323EA, i_0323ea),
    (0x0323EE, i_0323ee),
    (0x0323F2, i_0323f2),
    (0x0323F6, i_0323f6),
    (0x0323FA, i_0323fa),
    (0x0323FE, i_0323fe),
    (0x032400, i_032400),
    (0x032404, i_032404),
    (0x032406, i_032406),
    (0x03240A, i_03240a),
    (0x03240C, i_03240c),
    (0x032410, i_032410),
    (0x0329F8, i_0329f8),
    (0x032A00, i_032a00),
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
    (0x032B36, i_032b36),
    (0x032B3C, i_032b3c),
    (0x032B40, i_032b40),
    (0x032B46, i_032b46),
    (0x032B4A, i_032b4a),
    (0x032B50, i_032b50),
    (0x032B56, i_032b56),
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
    (0x032BC6, i_032bc6),
    (0x032BCA, i_032bca),
    (0x032BCE, i_032bce),
    (0x032BD4, i_032bd4),
    (0x032BD8, i_032bd8),
    (0x032BDE, i_032bde),
    (0x032BE2, i_032be2),
    (0x032BE6, i_032be6),
    (0x032BEA, i_032bea),
    (0x032BEE, i_032bee),
    (0x032BF2, i_032bf2),
    (0x032BF4, i_032bf4),
    (0x032BF8, i_032bf8),
    (0x032BFC, i_032bfc),
    (0x032C00, i_032c00),
    (0x032C04, i_032c04),
    (0x032C08, i_032c08),
    (0x032C0C, i_032c0c),
    (0x032C10, i_032c10),
    (0x032C12, i_032c12),
    (0x032C16, i_032c16),
    (0x032C1A, i_032c1a),
    (0x032C20, i_032c20),
    (0x032C24, i_032c24),
    (0x032C2A, i_032c2a),
    (0x032C2E, i_032c2e),
    (0x032C32, i_032c32),
    (0x032C36, i_032c36),
    (0x032C3A, i_032c3a),
    (0x032C3E, i_032c3e),
    (0x032C42, i_032c42),
    (0x032C46, i_032c46),
    (0x032C4A, i_032c4a),
    (0x032C4E, i_032c4e),
    (0x032C52, i_032c52),
    (0x032C56, i_032c56),
    (0x032C58, i_032c58),
    (0x032C5A, i_032c5a),
    (0x032C5C, i_032c5c),
    (0x032C60, i_032c60),
    (0x032C64, i_032c64),
    (0x032C68, i_032c68),
    (0x032C6C, i_032c6c),
    (0x032C72, i_032c72),
    (0x032C76, i_032c76),
    (0x032C7C, i_032c7c),
    (0x032C7E, i_032c7e),
    (0x032C82, i_032c82),
    (0x032C86, i_032c86),
    (0x032C8C, i_032c8c),
    (0x032C90, i_032c90),
    (0x032C96, i_032c96),
    (0x032C9C, i_032c9c),
    (0x032CA0, i_032ca0),
    (0x032CA4, i_032ca4),
    (0x032CA8, i_032ca8),
    (0x032CAC, i_032cac),
    (0x032CB0, i_032cb0),
    (0x032CB4, i_032cb4),
    (0x032CB8, i_032cb8),
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
    (0x032DE0, i_032de0),
    (0x032DE6, i_032de6),
    (0x032DE8, i_032de8),
    (0x032DEE, i_032dee),
    (0x032DF2, i_032df2),
    (0x032DF8, i_032df8),
    (0x032DFC, i_032dfc),
    (0x032E00, i_032e00),
    (0x032E02, i_032e02),
    (0x032E06, i_032e06),
    (0x032E08, i_032e08),
    (0x032E0E, i_032e0e),
    (0x032E12, i_032e12),
    (0x032E16, i_032e16),
    (0x032E1A, i_032e1a),
    (0x032E1E, i_032e1e),
    (0x032E20, i_032e20),
    (0x032E24, i_032e24),
    (0x032E28, i_032e28),
    (0x032E2C, i_032e2c),
    (0x032E2E, i_032e2e),
    (0x032E32, i_032e32),
    (0x032E36, i_032e36),
    (0x032E3A, i_032e3a),
    (0x032E3C, i_032e3c),
    (0x032E42, i_032e42),
    (0x032E48, i_032e48),
    (0x032E4E, i_032e4e),
    (0x032E52, i_032e52),
    (0x032E56, i_032e56),
    (0x032E5A, i_032e5a),
    (0x032E60, i_032e60),
    (0x032E64, i_032e64),
    (0x032E68, i_032e68),
    (0x032E6C, i_032e6c),
    (0x032E70, i_032e70),
    (0x032E76, i_032e76),
    (0x032E7A, i_032e7a),
    (0x032E7C, i_032e7c),
    (0x032E82, i_032e82),
    (0x032E86, i_032e86),
    (0x032E8C, i_032e8c),
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
    (0x032ED0, i_032ed0),
    (0x032ED4, i_032ed4),
    (0x032ED8, i_032ed8),
    (0x032EDE, i_032ede),
    (0x032EE2, i_032ee2),
    (0x032EE6, i_032ee6),
    (0x032EEC, i_032eec),
    (0x032EF0, i_032ef0),
    (0x032EF4, i_032ef4),
    (0x032EFA, i_032efa),
    (0x032EFE, i_032efe),
    (0x032F00, i_032f00),
    (0x032F06, i_032f06),
    (0x032F0A, i_032f0a),
    (0x032F0E, i_032f0e),
    (0x032F12, i_032f12),
    (0x032F16, i_032f16),
    (0x032F1A, i_032f1a),
    (0x032F1C, i_032f1c),
    (0x032F1E, i_032f1e),
    (0x032F24, i_032f24),
    (0x032F28, i_032f28),
    (0x032F2C, i_032f2c),
    (0x032F30, i_032f30),
    (0x032F34, i_032f34),
    (0x032F38, i_032f38),
    (0x032F3A, i_032f3a),
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
    (0x032FD4, i_032fd4),
    (0x032FD6, i_032fd6),
    (0x032FDA, i_032fda),
    (0x032FDE, i_032fde),
    (0x032FE2, i_032fe2),
    (0x032FE6, i_032fe6),
    (0x032FE8, i_032fe8),
    (0x032FEA, i_032fea),
    (0x032FEE, i_032fee),
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
    (0x033080, i_033080),
    (0x033088, i_033088),
    (0x03308C, i_03308c),
    (0x033092, i_033092),
    (0x033096, i_033096),
    (0x03309E, i_03309e),
    (0x0330A2, i_0330a2),
    (0x0330A8, i_0330a8),
    (0x0330AC, i_0330ac),
    (0x0330B0, i_0330b0),
    (0x0330B4, i_0330b4),
    (0x0330B6, i_0330b6),
    (0x0330BA, i_0330ba),
    (0x0330BC, i_0330bc),
    (0x0330C2, i_0330c2),
    (0x0330C4, i_0330c4),
    (0x0330C8, i_0330c8),
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
    (0x033140, i_033140),
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
    (0x0331AE, i_0331ae),
    (0x0331B4, i_0331b4),
    (0x0331B8, i_0331b8),
    (0x0331BC, i_0331bc),
    (0x0331C2, i_0331c2),
    (0x0331C6, i_0331c6),
    (0x0331C8, i_0331c8),
    (0x0331CC, i_0331cc),
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
    (0x033240, i_033240),
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
    (0x0332BA, i_0332ba),
    (0x0332BC, i_0332bc),
    (0x0332C0, i_0332c0),
    (0x0332C6, i_0332c6),
    (0x0332C8, i_0332c8),
    (0x0332CE, i_0332ce),
    (0x0332D2, i_0332d2),
    (0x0332D4, i_0332d4),
    (0x0332D8, i_0332d8),
    (0x0332DC, i_0332dc),
    (0x0332E2, i_0332e2),
    (0x0332E6, i_0332e6),
    (0x0332EC, i_0332ec),
    (0x0332F0, i_0332f0),
    (0x0332F6, i_0332f6),
    (0x0332FC, i_0332fc),
    (0x033300, i_033300),
    (0x033304, i_033304),
    (0x03330A, i_03330a),
    (0x033310, i_033310),
    (0x033316, i_033316),
    (0x03331A, i_03331a),
    (0x03331C, i_03331c),
    (0x033320, i_033320),
    (0x033326, i_033326),
    (0x03332C, i_03332c),
    (0x033330, i_033330),
    (0x033334, i_033334),
    (0x033338, i_033338),
    (0x03333E, i_03333e),
    (0x033344, i_033344),
    (0x033346, i_033346),
    (0x03334C, i_03334c),
    (0x033352, i_033352),
    (0x033358, i_033358),
    (0x03335E, i_03335e),
    (0x033362, i_033362),
    (0x033368, i_033368),
    (0x03336E, i_03336e),
    (0x033374, i_033374),
    (0x033376, i_033376),
    (0x03337C, i_03337c),
    (0x033382, i_033382),
    (0x033388, i_033388),
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
    (0x033638, i_033638),
    (0x033640, i_033640),
    (0x033648, i_033648),
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
    (0x0336B4, i_0336b4),
    (0x0336BA, i_0336ba),
    (0x0336C0, i_0336c0),
    (0x0336C6, i_0336c6),
    (0x0336C8, i_0336c8),
    (0x0336CE, i_0336ce),
    (0x0336D4, i_0336d4),
    (0x0336DA, i_0336da),
    (0x0336DC, i_0336dc),
    (0x0336E2, i_0336e2),
    (0x0336E8, i_0336e8),
    (0x0336EC, i_0336ec),
    (0x0336F0, i_0336f0),
    (0x0336F4, i_0336f4),
    (0x0336FA, i_0336fa),
    (0x0336FE, i_0336fe),
    (0x033704, i_033704),
    (0x033708, i_033708),
    (0x03370E, i_03370e),
    (0x033714, i_033714),
    (0x03371A, i_03371a),
    (0x03371E, i_03371e),
    (0x033724, i_033724),
    (0x03372A, i_03372a),
    (0x033730, i_033730),
    (0x033736, i_033736),
    (0x03373C, i_03373c),
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
    (0x03386E, i_03386e),
    (0x033874, i_033874),
    (0x03387A, i_03387a),
    (0x033880, i_033880),
    (0x033888, i_033888),
    (0x03388C, i_03388c),
    (0x033890, i_033890),
    (0x033894, i_033894),
    (0x033896, i_033896),
    (0x03389A, i_03389a),
    (0x03389E, i_03389e),
    (0x0338A4, i_0338a4),
    (0x0338A8, i_0338a8),
    (0x0338AC, i_0338ac),
    (0x0338B0, i_0338b0),
    (0x0338B2, i_0338b2),
    (0x0338B8, i_0338b8),
    (0x0338BC, i_0338bc),
    (0x0338C2, i_0338c2),
    (0x0338C8, i_0338c8),
    (0x0338CC, i_0338cc),
    (0x0338D2, i_0338d2),
    (0x0338D8, i_0338d8),
    (0x0338DE, i_0338de),
    (0x0338E4, i_0338e4),
    (0x0338EA, i_0338ea),
    (0x0338EE, i_0338ee),
    (0x0338F2, i_0338f2),
    (0x0338F4, i_0338f4),
    (0x0338F8, i_0338f8),
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
    (0x033C7C, i_033c7c),
    (0x033C80, i_033c80),
    (0x033C84, i_033c84),
    (0x033C88, i_033c88),
    (0x033C8A, i_033c8a),
    (0x033C8E, i_033c8e),
    (0x033C94, i_033c94),
    (0x033C9A, i_033c9a),
    (0x033CA0, i_033ca0),
    (0x033CA6, i_033ca6),
    (0x033CAC, i_033cac),
    (0x033CB2, i_033cb2),
    (0x033CB8, i_033cb8),
    (0x033CBE, i_033cbe),
    (0x033CC4, i_033cc4),
    (0x033CCA, i_033cca),
    (0x033CD0, i_033cd0),
    (0x033CD4, i_033cd4),
    (0x033CD8, i_033cd8),
    (0x033CE0, i_033ce0),
    (0x033CE6, i_033ce6),
    (0x033CEA, i_033cea),
    (0x033CEE, i_033cee),
    (0x033CF0, i_033cf0),
    (0x033CF4, i_033cf4),
    (0x033CF8, i_033cf8),
    (0x033CFE, i_033cfe),
    (0x033D02, i_033d02),
    (0x033D06, i_033d06),
    (0x033D0A, i_033d0a),
    (0x033D0C, i_033d0c),
    (0x033D10, i_033d10),
    (0x033D14, i_033d14),
    (0x033D18, i_033d18),
    (0x033D1C, i_033d1c),
    (0x033D1E, i_033d1e),
    (0x033D22, i_033d22),
    (0x033D26, i_033d26),
    (0x033D28, i_033d28),
    (0x033D2E, i_033d2e),
    (0x033D32, i_033d32),
    (0x033D34, i_033d34),
    (0x033D38, i_033d38),
    (0x033D3C, i_033d3c),
    (0x033D40, i_033d40),
    (0x033D42, i_033d42),
    (0x033D48, i_033d48),
    (0x033D4C, i_033d4c),
    (0x033D52, i_033d52),
    (0x033D56, i_033d56),
    (0x033D5A, i_033d5a),
    (0x033D5E, i_033d5e),
    (0x033D60, i_033d60),
    (0x033D64, i_033d64),
    (0x033D6A, i_033d6a),
    (0x033D70, i_033d70),
    (0x033D76, i_033d76),
    (0x033D7C, i_033d7c),
    (0x033D82, i_033d82),
    (0x033D88, i_033d88),
    (0x033D8C, i_033d8c),
    (0x033D92, i_033d92),
    (0x033D96, i_033d96),
    (0x033D9C, i_033d9c),
    (0x033DA0, i_033da0),
    (0x033DA6, i_033da6),
    (0x033DAC, i_033dac),
    (0x033DB2, i_033db2),
    (0x033DB6, i_033db6),
    (0x033DBC, i_033dbc),
    (0x033DC2, i_033dc2),
    (0x033DC8, i_033dc8),
    (0x033DCE, i_033dce),
    (0x033DD4, i_033dd4),
    (0x033DDA, i_033dda),
    (0x033DDE, i_033dde),
    (0x033DE2, i_033de2),
    (0x033DE8, i_033de8),
    (0x033DEC, i_033dec),
    (0x033DF0, i_033df0),
    (0x033DF4, i_033df4),
    (0x033DF8, i_033df8),
    (0x033DFE, i_033dfe),
    (0x033E04, i_033e04),
    (0x033E0A, i_033e0a),
    (0x033E10, i_033e10),
    (0x033E16, i_033e16),
    (0x033E1E, i_033e1e),
    (0x033E24, i_033e24),
    (0x033E28, i_033e28),
    (0x033E2C, i_033e2c),
    (0x033E2E, i_033e2e),
    (0x033E34, i_033e34),
    (0x033E38, i_033e38),
    (0x033E3C, i_033e3c),
    (0x033E40, i_033e40),
    (0x033E42, i_033e42),
    (0x033E46, i_033e46),
    (0x033E4A, i_033e4a),
    (0x033E50, i_033e50),
    (0x033E54, i_033e54),
    (0x033E58, i_033e58),
    (0x033E5C, i_033e5c),
    (0x033E5E, i_033e5e),
    (0x033E62, i_033e62),
    (0x033E66, i_033e66),
    (0x033E6A, i_033e6a),
    (0x033E6E, i_033e6e),
    (0x033E70, i_033e70),
    (0x033E74, i_033e74),
    (0x033E78, i_033e78),
    (0x033E7A, i_033e7a),
    (0x033E80, i_033e80),
    (0x033E84, i_033e84),
    (0x033E88, i_033e88),
    (0x033E8C, i_033e8c),
    (0x033E92, i_033e92),
    (0x033E96, i_033e96),
    (0x033E9C, i_033e9c),
    (0x033EA2, i_033ea2),
    (0x033EA6, i_033ea6),
    (0x033EAC, i_033eac),
    (0x033EB2, i_033eb2),
    (0x033EB8, i_033eb8),
    (0x033EBE, i_033ebe),
    (0x033EC2, i_033ec2),
    (0x033ECA, i_033eca),
    (0x033ECE, i_033ece),
    (0x033ED4, i_033ed4),
    (0x033EDA, i_033eda),
    (0x033EDE, i_033ede),
    (0x033EE4, i_033ee4),
    (0x033EEA, i_033eea),
    (0x033EEE, i_033eee),
    (0x033EF4, i_033ef4),
    (0x033EFA, i_033efa),
    (0x033F00, i_033f00),
    (0x033F06, i_033f06),
    (0x033F0C, i_033f0c),
    (0x033F12, i_033f12),
    (0x033F18, i_033f18),
    (0x033F1E, i_033f1e),
    (0x033F24, i_033f24),
    (0x033F2A, i_033f2a),
    (0x033F30, i_033f30),
    (0x033F36, i_033f36),
    (0x033F3C, i_033f3c),
    (0x033F40, i_033f40),
    (0x033F44, i_033f44),
    (0x033F48, i_033f48),
    (0x033F4E, i_033f4e),
    (0x033F52, i_033f52),
    (0x033F56, i_033f56),
    (0x033F58, i_033f58),
    (0x033F5C, i_033f5c),
    (0x033F62, i_033f62),
    (0x033F66, i_033f66),
    (0x033F6C, i_033f6c),
    (0x033F70, i_033f70),
    (0x033F74, i_033f74),
    (0x033F76, i_033f76),
    (0x033F7A, i_033f7a),
    (0x033F80, i_033f80),
    (0x033F84, i_033f84),
    (0x033F88, i_033f88),
    (0x033F8A, i_033f8a),
    (0x033F8E, i_033f8e),
    (0x033F96, i_033f96),
    (0x033F9C, i_033f9c),
    (0x033FA0, i_033fa0),
    (0x033FA4, i_033fa4),
    (0x033FA8, i_033fa8),
    (0x033FAC, i_033fac),
    (0x033FAE, i_033fae),
    (0x033FB2, i_033fb2),
    (0x033FB6, i_033fb6),
    (0x033FBA, i_033fba),
    (0x033FBE, i_033fbe),
    (0x033FC0, i_033fc0),
    (0x033FC6, i_033fc6),
    (0x033FCA, i_033fca),
    (0x033FCE, i_033fce),
    (0x033FD2, i_033fd2),
    (0x033FD4, i_033fd4),
    (0x033FDA, i_033fda),
    (0x033FDC, i_033fdc),
    (0x033FE0, i_033fe0),
    (0x033FE4, i_033fe4),
    (0x033FEA, i_033fea),
    (0x033FEE, i_033fee),
    (0x033FF2, i_033ff2),
    (0x033FF4, i_033ff4),
    (0x033FFA, i_033ffa),
    (0x033FFE, i_033ffe),
    (0x034004, i_034004),
    (0x034008, i_034008),
    (0x03400C, i_03400c),
    (0x03400E, i_03400e),
    (0x034014, i_034014),
    (0x034018, i_034018),
    (0x03401A, i_03401a),
    (0x034020, i_034020),
    (0x034024, i_034024),
    (0x034028, i_034028),
    (0x03402C, i_03402c),
    (0x034030, i_034030),
    (0x034032, i_034032),
    (0x034038, i_034038),
    (0x03403C, i_03403c),
    (0x034040, i_034040),
    (0x034042, i_034042),
    (0x034046, i_034046),
    (0x03404C, i_03404c),
    (0x034050, i_034050),
    (0x034056, i_034056),
    (0x03405A, i_03405a),
    (0x03405E, i_03405e),
    (0x034064, i_034064),
    (0x034068, i_034068),
    (0x03406C, i_03406c),
    (0x034072, i_034072),
    (0x034078, i_034078),
    (0x03407E, i_03407e),
    (0x034084, i_034084),
    (0x03408A, i_03408a),
    (0x034090, i_034090),
    (0x034096, i_034096),
    (0x03409C, i_03409c),
    (0x0340A2, i_0340a2),
    (0x0340A8, i_0340a8),
    (0x0340AE, i_0340ae),
    (0x0340B4, i_0340b4),
    (0x0340BC, i_0340bc),
    (0x0340C2, i_0340c2),
    (0x0340C6, i_0340c6),
    (0x0340CA, i_0340ca),
    (0x0340CE, i_0340ce),
    (0x0340D4, i_0340d4),
    (0x0340D8, i_0340d8),
    (0x0340DC, i_0340dc),
    (0x0340DE, i_0340de),
    (0x0340E2, i_0340e2),
    (0x0340E8, i_0340e8),
    (0x0340EC, i_0340ec),
    (0x0340F2, i_0340f2),
    (0x0340F6, i_0340f6),
    (0x0340FA, i_0340fa),
    (0x0340FC, i_0340fc),
    (0x034100, i_034100),
    (0x034106, i_034106),
    (0x03410A, i_03410a),
    (0x03410E, i_03410e),
    (0x034110, i_034110),
    (0x034114, i_034114),
    (0x034118, i_034118),
    (0x03411A, i_03411a),
    (0x03411E, i_03411e),
    (0x034122, i_034122),
    (0x034128, i_034128),
    (0x03412C, i_03412c),
    (0x034130, i_034130),
    (0x034134, i_034134),
    (0x034136, i_034136),
    (0x03413C, i_03413c),
    (0x03413E, i_03413e),
    (0x034142, i_034142),
    (0x034146, i_034146),
    (0x03414C, i_03414c),
    (0x034150, i_034150),
    (0x034154, i_034154),
    (0x034156, i_034156),
    (0x03415C, i_03415c),
    (0x034160, i_034160),
    (0x034166, i_034166),
    (0x03416A, i_03416a),
    (0x03416E, i_03416e),
    (0x034170, i_034170),
    (0x034176, i_034176),
    (0x03417A, i_03417a),
    (0x03417C, i_03417c),
    (0x034182, i_034182),
    (0x034186, i_034186),
    (0x03418A, i_03418a),
    (0x03418E, i_03418e),
    (0x034192, i_034192),
    (0x034194, i_034194),
    (0x03419A, i_03419a),
    (0x03419E, i_03419e),
    (0x0341A2, i_0341a2),
    (0x0341A4, i_0341a4),
    (0x0341AA, i_0341aa),
    (0x0341AE, i_0341ae),
    (0x0341B4, i_0341b4),
    (0x0341B8, i_0341b8),
    (0x0341BC, i_0341bc),
    (0x0341C0, i_0341c0),
    (0x0341C4, i_0341c4),
    (0x0341C6, i_0341c6),
    (0x0341CA, i_0341ca),
    (0x0341CE, i_0341ce),
    (0x0341D2, i_0341d2),
    (0x0341D4, i_0341d4),
    (0x0341D8, i_0341d8),
];
