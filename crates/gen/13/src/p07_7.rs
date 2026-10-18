// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07_7) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $07E0C2: move.w $66(a6), $80(a6)
pub fn i_07e0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0C8;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E0C8: lea.l $2e26b0.l, a0
pub fn i_07e0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E26B0;
    20
}
/// $07E0CE: move.l a0, $48(a6)
pub fn i_07e0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0D2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07E0D2: lea.l $7e30e(pc), a1
pub fn i_07e0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E30E;
    16
}
/// $07E0D6: jsr $4ae.l
pub fn i_07e0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E0DC);
    cpu.pc = 0x4AE;
    16
}
/// $07E0DC: jsr $5dd02.l
pub fn i_07e0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E0E2);
    cpu.pc = 0x5DD02;
    16
}
/// $07E0E2: lea.l $7e40c(pc), a1
pub fn i_07e0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E40C;
    16
}
/// $07E0E6: jsr $4ae.l
pub fn i_07e0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E0EC);
    cpu.pc = 0x4AE;
    16
}
/// $07E0EC: jsr $5dd02.l
pub fn i_07e0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E0F2);
    cpu.pc = 0x5DD02;
    16
}
/// $07E0F2: lea.l $7e504(pc), a1
pub fn i_07e0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E504;
    16
}
/// $07E0F6: jsr $4ae.l
pub fn i_07e0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E0FC);
    cpu.pc = 0x4AE;
    16
}
/// $07E0FC: jsr $5dd02.l
pub fn i_07e0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E102;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E102);
    cpu.pc = 0x5DD02;
    16
}
/// $07E102: lea.l $7e8f2(pc), a1
pub fn i_07e102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E106;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E8F2;
    16
}
/// $07E106: jsr $4ae.l
pub fn i_07e106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E10C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E10C);
    cpu.pc = 0x4AE;
    16
}
/// $07E10C: jsr $5dd02.l
pub fn i_07e10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E112;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E112);
    cpu.pc = 0x5DD02;
    16
}
/// $07E112: lea.l $7e854(pc), a1
pub fn i_07e112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E116;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E854;
    16
}
/// $07E116: jsr $4ae.l
pub fn i_07e116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E11C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E11C);
    cpu.pc = 0x4AE;
    16
}
/// $07E11C: jsr $5dd02.l
pub fn i_07e11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E122;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E122);
    cpu.pc = 0x5DD02;
    16
}
/// $07E122: lea.l $7e86a(pc), a1
pub fn i_07e122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E126;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E86A;
    16
}
/// $07E126: jsr $4ae.l
pub fn i_07e126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E12C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E12C);
    cpu.pc = 0x4AE;
    16
}
/// $07E12C: jsr $5dd02.l
pub fn i_07e12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E132;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E132);
    cpu.pc = 0x5DD02;
    16
}
/// $07E132: lea.l $7e138(pc), a1
pub fn i_07e132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E136;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E138;
    16
}
/// $07E136: move.l a1, (a6)
pub fn i_07e136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E138;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E138: jsr $7fee8(pc)
pub fn i_07e138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E13C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E13C);
    cpu.pc = 0x7FEE8;
    16
}
/// $07E13C: jsr $27cee.l
pub fn i_07e13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E142;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E142);
    cpu.pc = 0x27CEE;
    16
}
/// $07E142: move.w $22(a6), d0
pub fn i_07e142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E146;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E146: move.w $24(a6), d1
pub fn i_07e146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E14A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E14A: addi.w #$ffa8, d0
pub fn i_07e14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E14E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFFA8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E14E: addi.w #$20, d1
pub fn i_07e14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E152;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E152: move.w #$30, d2
pub fn i_07e152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E156;
    cpu.ir = 0x343C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E156: jsr $99812.l
pub fn i_07e156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E15C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E15C);
    cpu.pc = 0x99812;
    16
}
/// $07E15C: move.w $22(a6), d0
pub fn i_07e15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E160;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E160: move.w $24(a6), d1
pub fn i_07e160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E164;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E164: addi.w #$38, d0
pub fn i_07e164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E168;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x38, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E168: addi.w #$20, d1
pub fn i_07e168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E16C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E16C: move.w #$30, d2
pub fn i_07e16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E170;
    cpu.ir = 0x343C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E170: jsr $99812.l
pub fn i_07e170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E176;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E176);
    cpu.pc = 0x99812;
    16
}
/// $07E176: move.w $22(a6), d0
pub fn i_07e176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E17A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E17A: move.w $24(a6), d1
pub fn i_07e17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E17E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E17E: addi.w #$ffd0, d0
pub fn i_07e17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E182;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFFD0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E182: addq.w #$8, d1
pub fn i_07e182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E184;
    cpu.ir = 0x5041;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07E184: move.w #$70, d2
pub fn i_07e184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E188;
    cpu.ir = 0x343C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E188: jsr $99812.l
pub fn i_07e188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E18E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E18E);
    cpu.pc = 0x99812;
    16
}
/// $07E18E: addq.w #$1, $72(a6)
pub fn i_07e18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E192;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07E192: move.w $72(a6), d0
pub fn i_07e192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E196;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E196: cmp.w $70(a6), d0
pub fn i_07e196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E19A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07E19A: blt.w $7e1d4
pub fn i_07e19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E19E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7E1D4; return 10; }
    8
}
/// $07E19E: jsr $5e9b6.l
pub fn i_07e19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E1A4);
    cpu.pc = 0x5E9B6;
    16
}
/// $07E1A4: andi.w #$3, d0
pub fn i_07e1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1A8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E1A8: add.w d0, d0
pub fn i_07e1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1AA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07E1AA: lea.l $2e1d4a.l, a0
pub fn i_07e1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E1D4A;
    20
}
/// $07E1B0: move.w (a0, d0.w), d0
pub fn i_07e1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1B4;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07E1B4: move.w d0, $72(a6)
pub fn i_07e1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1B8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E1B8: lea.l $7eac2(pc), a1
pub fn i_07e1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EAC2;
    16
}
/// $07E1BC: jsr $4ae.l
pub fn i_07e1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E1C2);
    cpu.pc = 0x4AE;
    16
}
/// $07E1C2: jsr $5dd02.l
pub fn i_07e1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E1C8);
    cpu.pc = 0x5DD02;
    16
}
/// $07E1C8: addi.w #$38, $22(a0)
pub fn i_07e1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1CE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07E1CE: addi.w #$48, $24(a0)
pub fn i_07e1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1D4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07E1D4: jsr $2870a.l
pub fn i_07e1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E1DA);
    cpu.pc = 0x2870A;
    16
}
/// $07E1DA: bcc.w $7e1f0
pub fn i_07e1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E1F0; return 10; }
    8
}
/// $07E1DE: lea.l $5e766.l, a0
pub fn i_07e1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07E1E4: jsr $5e770.l
pub fn i_07e1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E1EA);
    cpu.pc = 0x5E770;
    16
}
/// $07E1EA: bclr.b #$3, $13(a6)
pub fn i_07e1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1F0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07E1F0: jsr $7feb4(pc)
pub fn i_07e1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E1F4);
    cpu.pc = 0x7FEB4;
    16
}
/// $07E1F4: jsr $28758.l
pub fn i_07e1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E1FA);
    cpu.pc = 0x28758;
    16
}
/// $07E1FA: bcc.w $7e254
pub fn i_07e1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E1FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E254; return 10; }
    8
}
/// $07E1FE: move.b #$3, $20(a6)
pub fn i_07e1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E204;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07E204: subq.w #$1, $7a(a6)
pub fn i_07e204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E208;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07E208: bne.w $7e254
pub fn i_07e208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E20C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E254; return 10; }
    8
}
/// $07E20C: move.b #$1, $21(a6)
pub fn i_07e20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E212;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07E212: move.w #$0, $76(a6)
pub fn i_07e212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E218;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E218: lea.l $ffff.w, a0
pub fn i_07e218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E21C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $07E21C: move.l a0, $48(a6)
pub fn i_07e21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E220;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07E220: lea.l $7f674(pc), a1
pub fn i_07e220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E224;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F674;
    16
}
/// $07E224: jsr $4ae.l
pub fn i_07e224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E22A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E22A);
    cpu.pc = 0x4AE;
    16
}
/// $07E22A: jsr $5dd02.l
pub fn i_07e22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E230;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E230);
    cpu.pc = 0x5DD02;
    16
}
/// $07E230: lea.l $7e602(pc), a1
pub fn i_07e230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E234;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E602;
    16
}
/// $07E234: jsr $4ae.l
pub fn i_07e234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E23A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E23A);
    cpu.pc = 0x4AE;
    16
}
/// $07E23A: jsr $5dd02.l
pub fn i_07e23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E240);
    cpu.pc = 0x5DD02;
    16
}
/// $07E240: move.b $9a(a6), d0
pub fn i_07e240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E244;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07E244: move.w #$9b, d1
pub fn i_07e244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E248;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E248: jsr $9a7aa.l
pub fn i_07e248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E24E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E24E);
    cpu.pc = 0x9A7AA;
    16
}
/// $07E24E: lea.l $7e264(pc), a1
pub fn i_07e24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E252;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E264;
    16
}
/// $07E252: move.l a1, (a6)
pub fn i_07e252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E254;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E254: jsr $7fdc4(pc)
pub fn i_07e254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E258;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E258);
    cpu.pc = 0x7FDC4;
    16
}
/// $07E258: bcc.w $7e262
pub fn i_07e258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E25C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E262; return 10; }
    8
}
// ===== SetTaskHandler_07e25c  ($07E25C, 8 B, kernel/task_handlers.c)
/// $07E25C: lea.l $7fdb6(pc), a1
pub fn i_07e25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E260;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDB6;
    16
}
/// $07E260: move.l a1, (a6)
pub fn i_07e260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E262;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E262: rts
pub fn i_07e262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E264;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Phase4_07e264  ($07E264, 46 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E264: jsr $2783a.l
pub fn i_07e264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E26A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E26A);
    cpu.pc = 0x2783A;
    16
}
/// $07E26A: cmpi.b #$4, $20(a6)
pub fn i_07e26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E270;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $07E270: bne.w $7e286
pub fn i_07e270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E274;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E286; return 10; }
    8
}
/// $07E274: bclr.b #$1, $12(a6)
pub fn i_07e274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E27A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07E27A: move.b #$2, $21(a6)
pub fn i_07e27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E280;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07E280: lea.l $7e29a(pc), a1
pub fn i_07e280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E284;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E29A;
    16
}
/// $07E284: move.l a1, (a6)
pub fn i_07e284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E286;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E286: jsr $7feb4(pc)
pub fn i_07e286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E28A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E28A);
    cpu.pc = 0x7FEB4;
    16
}
/// $07E28A: jsr $7fdc4(pc)
pub fn i_07e28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E28E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E28E);
    cpu.pc = 0x7FDC4;
    16
}
/// $07E28E: bcc.w $7e298
pub fn i_07e28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E292;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E298; return 10; }
    8
}
// ===== SetTaskHandler_07e292  ($07E292, 8 B, kernel/task_handlers.c)
/// $07E292: lea.l $7fdb6(pc), a1
pub fn i_07e292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E296;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDB6;
    16
}
/// $07E296: move.l a1, (a6)
pub fn i_07e296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E298;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E298: rts
pub fn i_07e298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E29A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Phase5_07e29a  ($07E29A, 48 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E29A: cmpi.b #$5, $20(a6)
pub fn i_07e29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2A0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $07E2A0: bne.w $7e2b8
pub fn i_07e2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E2B8; return 10; }
    8
}
/// $07E2A4: move.b #$3, $21(a6)
pub fn i_07e2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2AA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07E2AA: lea.l $ffff.w, a0
pub fn i_07e2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2AE;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $07E2AE: move.l a0, $48(a6)
pub fn i_07e2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2B2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07E2B2: lea.l $7e2d2(pc), a1
pub fn i_07e2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E2D2;
    16
}
/// $07E2B6: move.l a1, (a6)
pub fn i_07e2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E2B8: jsr $2783a.l
pub fn i_07e2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E2BE);
    cpu.pc = 0x2783A;
    16
}
/// $07E2BE: jsr $7feb4(pc)
pub fn i_07e2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E2C2);
    cpu.pc = 0x7FEB4;
    16
}
/// $07E2C2: jsr $7fdc4(pc)
pub fn i_07e2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E2C6);
    cpu.pc = 0x7FDC4;
    16
}
/// $07E2C6: bcc.w $7e2d0
pub fn i_07e2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E2D0; return 10; }
    8
}
// ===== SetTaskHandler_07e2ca  ($07E2CA, 8 B, kernel/task_handlers.c)
/// $07E2CA: lea.l $7fdb6(pc), a1
pub fn i_07e2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDB6;
    16
}
/// $07E2CE: move.l a1, (a6)
pub fn i_07e2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E2D0: rts
pub fn i_07e2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_WaitChildrenGone_07e2d2  ($07E2D2, 24 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E2D2: cmpi.b #$ff, $21(a6)
pub fn i_07e2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2D8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07E2D8: bne.w $7e2e2
pub fn i_07e2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E2E2; return 10; }
    8
}
/// $07E2DC: lea.l $7fdbc(pc), a1
pub fn i_07e2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07E2E0: move.l a1, (a6)
pub fn i_07e2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E2E2: jsr $7fdc4(pc)
pub fn i_07e2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E2E6);
    cpu.pc = 0x7FDC4;
    16
}
/// $07E2E6: bcc.w $7e2f0
pub fn i_07e2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E2F0; return 10; }
    8
}
// ===== SetTaskHandler_07e2ea  ($07E2EA, 8 B, kernel/task_handlers.c)
/// $07E2EA: lea.l $7fdb6(pc), a1
pub fn i_07e2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDB6;
    16
}
/// $07E2EE: move.l a1, (a6)
pub fn i_07e2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E2F0: rts
pub fn i_07e2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Child_FreeWhenDone_07e2f2  ($07E2F2, 28 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E2F2: clr.b $10e39a.l
pub fn i_07e2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2F8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $07E2F8: movea.l $c(a6), a0
pub fn i_07e2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E2FC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E2FC: cmpi.b #$2, $21(a0)
pub fn i_07e2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E302;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $07E302: bne.w $7e30c
pub fn i_07e302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E306;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E30C; return 10; }
    8
}
/// $07E306: jmp $518.l
pub fn i_07e306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E30C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07E30C: rts
pub fn i_07e30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E30E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Hull_Init_07e30e  ($07E30E, 44 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E30E: move.w #$24, d1
pub fn i_07e30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E312;
    cpu.ir = 0x323C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E312: jsr $236e.l
pub fn i_07e312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E318;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E318);
    cpu.pc = 0x236E;
    16
}
/// $07E318: clr.b $20(a6)
pub fn i_07e318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E31C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07E31C: clr.b $21(a6)
pub fn i_07e31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E320;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07E320: movea.l $c(a6), a0
pub fn i_07e320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E324;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E324: move.w $66(a0), d0
pub fn i_07e324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E328;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E328: divu.w #$3, d0
pub fn i_07e328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E32C;
    cpu.ir = 0x80FC;
    let v1: u32 = 0x3;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $07E32C: move.w d0, $66(a6)
pub fn i_07e32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E330;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E330: move.w d0, $72(a6)
pub fn i_07e330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E334;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E334: move.w #$ffff, $70(a6)
pub fn i_07e334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E33A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Carrier_Hull_Rank_07e33a  ($07E33A, 14 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E33A: bclr.b #$3, $13(a6)
pub fn i_07e33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E340;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07E340: jsr $7fffc(pc)
pub fn i_07e340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E344;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E344);
    cpu.pc = 0x7FFFC;
    16
}
/// $07E344: cmp.w $70(a6), d0
pub fn i_07e344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E348;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
// ===== Carrier_Hull_RankChanged_07e348  ($07E348, 188 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E348: beq.w $7e3ac
pub fn i_07e348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E34C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7E3AC; return 10; }
    8
}
/// $07E34C: move.w d0, $70(a6)
pub fn i_07e34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E350;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E350: lea.l $2e1d52.l, a0
pub fn i_07e350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E356;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E1D52;
    20
}
/// $07E356: add.w d0, d0
pub fn i_07e356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E358;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07E358: add.w d0, d0
pub fn i_07e358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E35A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07E35A: movea.l (a0, d0.w), a0
pub fn i_07e35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E35E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07E35E: jsr $28cd4.l
pub fn i_07e35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E364;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E364);
    cpu.pc = 0x28CD4;
    16
}
/// $07E364: lea.l $77f6a.l, a1
pub fn i_07e364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E36A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $07E36A: jsr $4ae.l
pub fn i_07e36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E370;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E370);
    cpu.pc = 0x4AE;
    16
}
/// $07E370: jsr $5dd02.l
pub fn i_07e370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E376;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E376);
    cpu.pc = 0x5DD02;
    16
}
/// $07E376: addi.w #$28, $38(a0)
pub fn i_07e376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E37C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07E37C: subi.w #$48, $22(a0)
pub fn i_07e37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E382;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07E382: move.w $54(a6), $22(a0)
pub fn i_07e382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E388;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E388: move.w $56(a6), $24(a0)
pub fn i_07e388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E38E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E38E: lea.l $2e2b0c.l, a1
pub fn i_07e38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E394;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E2B0C;
    20
}
/// $07E394: jsr $77c7e.l
pub fn i_07e394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E39A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E39A);
    cpu.pc = 0x77C7E;
    16
}
/// $07E39A: move.w $54(a6), $22(a0)
pub fn i_07e39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3A0;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E3A0: move.w $56(a6), $24(a0)
pub fn i_07e3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3A6;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E3A6: addi.w #$28, $38(a0)
pub fn i_07e3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3AC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07E3AC: lea.l $7e3b2(pc), a1
pub fn i_07e3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E3B2;
    16
}
/// $07E3B0: move.l a1, (a6)
pub fn i_07e3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E3B2: movea.l $c(a6), a0
pub fn i_07e3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3B6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E3B6: move.w $22(a0), $22(a6)
pub fn i_07e3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3BC;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E3BC: move.w $24(a0), $24(a6)
pub fn i_07e3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3C2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E3C2: jsr $28d70.l
pub fn i_07e3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E3C8);
    cpu.pc = 0x28D70;
    16
}
/// $07E3C8: jsr $2870a.l
pub fn i_07e3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E3CE);
    cpu.pc = 0x2870A;
    16
}
/// $07E3CE: bcc.w $7e3d8
pub fn i_07e3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E3D8; return 10; }
    8
}
/// $07E3D2: lea.l $7e33a(pc), a1
pub fn i_07e3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E33A;
    16
}
/// $07E3D6: move.l a1, (a6)
pub fn i_07e3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E3D8: movea.l $c(a6), a0
pub fn i_07e3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3DC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E3DC: cmpi.b #$3, $20(a0)
pub fn i_07e3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3E2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $07E3E2: bne.w $7e3ec
pub fn i_07e3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E3EC; return 10; }
    8
}
/// $07E3E6: lea.l $7fc38(pc), a1
pub fn i_07e3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FC38;
    16
}
/// $07E3EA: move.l a1, (a6)
pub fn i_07e3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E3EC: jsr $28758.l
pub fn i_07e3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E3F2);
    cpu.pc = 0x28758;
    16
}
/// $07E3F2: bcc.w $7e3fc
pub fn i_07e3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E3FC; return 10; }
    8
}
/// $07E3F6: lea.l $7fc1a(pc), a1
pub fn i_07e3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FC1A;
    16
}
/// $07E3FA: move.l a1, (a6)
pub fn i_07e3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E3FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E3FC: jsr $7fde0(pc)
pub fn i_07e3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E400;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E400);
    cpu.pc = 0x7FDE0;
    16
}
/// $07E400: bcc.w $7e40a
pub fn i_07e400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E404;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E40A; return 10; }
    8
}
// ===== SetTaskHandler_07e404  ($07E404, 8 B, kernel/task_handlers.c)
/// $07E404: lea.l $7fdbc(pc), a1
pub fn i_07e404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E408;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07E408: move.l a1, (a6)
pub fn i_07e408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E40A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E40A: rts
pub fn i_07e40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E40C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_HullB_Init_07e40c  ($07E40C, 44 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E40C: move.w #$24, d1
pub fn i_07e40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E410;
    cpu.ir = 0x323C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E410: jsr $236e.l
pub fn i_07e410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E416;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E416);
    cpu.pc = 0x236E;
    16
}
/// $07E416: clr.b $20(a6)
pub fn i_07e416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E41A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07E41A: clr.b $21(a6)
pub fn i_07e41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E41E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07E41E: movea.l $c(a6), a0
pub fn i_07e41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E422;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E422: move.w $66(a0), d0
pub fn i_07e422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E426;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E426: divu.w #$3, d0
pub fn i_07e426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E42A;
    cpu.ir = 0x80FC;
    let v1: u32 = 0x3;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $07E42A: move.w d0, $66(a6)
pub fn i_07e42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E42E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E42E: move.w d0, $72(a6)
pub fn i_07e42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E432;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E432: move.w #$ffff, $70(a6)
pub fn i_07e432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E438;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Carrier_HullB_Run_07e438  ($07E438, 196 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E438: bclr.b #$3, $13(a6)
pub fn i_07e438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E43E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07E43E: jsr $7fffc(pc)
pub fn i_07e43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E442;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E442);
    cpu.pc = 0x7FFFC;
    16
}
/// $07E442: cmp.w $70(a6), d0
pub fn i_07e442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E446;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07E446: beq.w $7e4a4
pub fn i_07e446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E44A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7E4A4; return 10; }
    8
}
/// $07E44A: move.w d0, $70(a6)
pub fn i_07e44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E44E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E44E: lea.l $2e1de4.l, a0
pub fn i_07e44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E454;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E1DE4;
    20
}
/// $07E454: add.w d0, d0
pub fn i_07e454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E456;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07E456: add.w d0, d0
pub fn i_07e456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E458;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07E458: movea.l (a0, d0.w), a0
pub fn i_07e458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E45C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07E45C: jsr $28cd4.l
pub fn i_07e45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E462;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E462);
    cpu.pc = 0x28CD4;
    16
}
/// $07E462: lea.l $77f6a.l, a1
pub fn i_07e462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E468;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $07E468: jsr $4ae.l
pub fn i_07e468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E46E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E46E);
    cpu.pc = 0x4AE;
    16
}
/// $07E46E: jsr $5dd02.l
pub fn i_07e46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E474;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E474);
    cpu.pc = 0x5DD02;
    16
}
/// $07E474: move.w $54(a6), $22(a0)
pub fn i_07e474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E47A;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E47A: move.w $56(a6), $24(a0)
pub fn i_07e47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E480;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E480: addi.w #$28, $38(a0)
pub fn i_07e480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E486;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07E486: lea.l $2e2b0c.l, a1
pub fn i_07e486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E48C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E2B0C;
    20
}
/// $07E48C: jsr $77c7e.l
pub fn i_07e48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E492;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E492);
    cpu.pc = 0x77C7E;
    16
}
/// $07E492: move.w $54(a6), $22(a0)
pub fn i_07e492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E498;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E498: move.w $56(a6), $24(a0)
pub fn i_07e498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E49E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E49E: addi.w #$28, $38(a0)
pub fn i_07e49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4A4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07E4A4: lea.l $7e4aa(pc), a1
pub fn i_07e4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E4AA;
    16
}
/// $07E4A8: move.l a1, (a6)
pub fn i_07e4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E4AA: movea.l $c(a6), a0
pub fn i_07e4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4AE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E4AE: move.w $22(a0), $22(a6)
pub fn i_07e4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4B4;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E4B4: move.w $24(a0), $24(a6)
pub fn i_07e4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4BA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E4BA: jsr $28d70.l
pub fn i_07e4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E4C0);
    cpu.pc = 0x28D70;
    16
}
/// $07E4C0: jsr $2870a.l
pub fn i_07e4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E4C6);
    cpu.pc = 0x2870A;
    16
}
/// $07E4C6: bcc.w $7e4d0
pub fn i_07e4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E4D0; return 10; }
    8
}
/// $07E4CA: lea.l $7e438(pc), a1
pub fn i_07e4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E438;
    16
}
/// $07E4CE: move.l a1, (a6)
pub fn i_07e4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E4D0: movea.l $c(a6), a0
pub fn i_07e4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4D4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E4D4: cmpi.b #$3, $20(a0)
pub fn i_07e4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4DA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $07E4DA: bne.w $7e4e4
pub fn i_07e4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E4E4; return 10; }
    8
}
/// $07E4DE: lea.l $7fc6c(pc), a1
pub fn i_07e4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FC6C;
    16
}
/// $07E4E2: move.l a1, (a6)
pub fn i_07e4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E4E4: jsr $28758.l
pub fn i_07e4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E4EA);
    cpu.pc = 0x28758;
    16
}
/// $07E4EA: bcc.w $7e4f4
pub fn i_07e4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E4F4; return 10; }
    8
}
/// $07E4EE: lea.l $7fc54(pc), a1
pub fn i_07e4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FC54;
    16
}
/// $07E4F2: move.l a1, (a6)
pub fn i_07e4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E4F4: jsr $7fde0(pc)
pub fn i_07e4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E4F8);
    cpu.pc = 0x7FDE0;
    16
}
/// $07E4F8: bcc.w $7e502
pub fn i_07e4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E4FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E502; return 10; }
    8
}
// ===== SetTaskHandler_07e4fc  ($07E4FC, 8 B, kernel/task_handlers.c)
/// $07E4FC: lea.l $7fdbc(pc), a1
pub fn i_07e4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E500;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07E500: move.l a1, (a6)
pub fn i_07e500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E502;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E502: rts
pub fn i_07e502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E504;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_HullC_Init_07e504  ($07E504, 44 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E504: move.w #$24, d1
pub fn i_07e504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E508;
    cpu.ir = 0x323C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E508: jsr $236e.l
pub fn i_07e508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E50E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E50E);
    cpu.pc = 0x236E;
    16
}
/// $07E50E: clr.b $20(a6)
pub fn i_07e50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E512;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07E512: clr.b $21(a6)
pub fn i_07e512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E516;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07E516: movea.l $c(a6), a0
pub fn i_07e516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E51A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E51A: move.w $66(a0), d0
pub fn i_07e51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E51E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E51E: divu.w #$3, d0
pub fn i_07e51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E522;
    cpu.ir = 0x80FC;
    let v1: u32 = 0x3;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $07E522: move.w d0, $66(a6)
pub fn i_07e522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E526;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E526: move.w d0, $72(a6)
pub fn i_07e526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E52A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E52A: move.w #$ffff, $70(a6)
pub fn i_07e52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E530;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Carrier_HullC_Run_07e530  ($07E530, 202 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E530: bclr.b #$3, $13(a6)
pub fn i_07e530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E536;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07E536: jsr $7fffc(pc)
pub fn i_07e536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E53A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E53A);
    cpu.pc = 0x7FFFC;
    16
}
/// $07E53A: cmp.w $70(a6), d0
pub fn i_07e53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E53E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07E53E: beq.w $7e5a2
pub fn i_07e53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E542;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7E5A2; return 10; }
    8
}
/// $07E542: move.w d0, $70(a6)
pub fn i_07e542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E546;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E546: lea.l $2e1e76.l, a0
pub fn i_07e546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E54C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E1E76;
    20
}
/// $07E54C: add.w d0, d0
pub fn i_07e54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E54E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07E54E: add.w d0, d0
pub fn i_07e54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E550;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07E550: movea.l (a0, d0.w), a0
pub fn i_07e550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E554;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07E554: jsr $28cd4.l
pub fn i_07e554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E55A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E55A);
    cpu.pc = 0x28CD4;
    16
}
/// $07E55A: lea.l $77f6a.l, a1
pub fn i_07e55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E560;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $07E560: jsr $4ae.l
pub fn i_07e560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E566;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E566);
    cpu.pc = 0x4AE;
    16
}
/// $07E566: jsr $5dd02.l
pub fn i_07e566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E56C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E56C);
    cpu.pc = 0x5DD02;
    16
}
/// $07E56C: addi.w #$40, $22(a0)
pub fn i_07e56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E572;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07E572: move.w $54(a6), $22(a0)
pub fn i_07e572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E578;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E578: move.w $56(a6), $24(a0)
pub fn i_07e578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E57E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E57E: addi.w #$28, $38(a0)
pub fn i_07e57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E584;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07E584: lea.l $2e2b0c.l, a1
pub fn i_07e584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E58A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E2B0C;
    20
}
/// $07E58A: jsr $77c7e.l
pub fn i_07e58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E590);
    cpu.pc = 0x77C7E;
    16
}
/// $07E590: move.w $54(a6), $22(a0)
pub fn i_07e590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E596;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E596: move.w $56(a6), $24(a0)
pub fn i_07e596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E59C;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E59C: addi.w #$28, $38(a0)
pub fn i_07e59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5A2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07E5A2: lea.l $7e5a8(pc), a1
pub fn i_07e5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E5A8;
    16
}
/// $07E5A6: move.l a1, (a6)
pub fn i_07e5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E5A8: movea.l $c(a6), a0
pub fn i_07e5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5AC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E5AC: move.w $22(a0), $22(a6)
pub fn i_07e5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5B2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E5B2: move.w $24(a0), $24(a6)
pub fn i_07e5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5B8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E5B8: jsr $28d70.l
pub fn i_07e5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E5BE);
    cpu.pc = 0x28D70;
    16
}
/// $07E5BE: jsr $2870a.l
pub fn i_07e5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E5C4);
    cpu.pc = 0x2870A;
    16
}
/// $07E5C4: bcc.w $7e5ce
pub fn i_07e5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E5CE; return 10; }
    8
}
/// $07E5C8: lea.l $7e530(pc), a1
pub fn i_07e5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E530;
    16
}
/// $07E5CC: move.l a1, (a6)
pub fn i_07e5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E5CE: movea.l $c(a6), a0
pub fn i_07e5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5D2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E5D2: cmpi.b #$3, $20(a0)
pub fn i_07e5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5D8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $07E5D8: bne.w $7e5e2
pub fn i_07e5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E5E2; return 10; }
    8
}
/// $07E5DC: lea.l $7fca6(pc), a1
pub fn i_07e5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FCA6;
    16
}
/// $07E5E0: move.l a1, (a6)
pub fn i_07e5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E5E2: jsr $28758.l
pub fn i_07e5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E5E8);
    cpu.pc = 0x28758;
    16
}
/// $07E5E8: bcc.w $7e5f2
pub fn i_07e5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E5F2; return 10; }
    8
}
/// $07E5EC: lea.l $7fc88(pc), a1
pub fn i_07e5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FC88;
    16
}
/// $07E5F0: move.l a1, (a6)
pub fn i_07e5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E5F2: jsr $7fde0(pc)
pub fn i_07e5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E5F6);
    cpu.pc = 0x7FDE0;
    16
}
/// $07E5F6: bcc.w $7e600
pub fn i_07e5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E600; return 10; }
    8
}
// ===== SetTaskHandler_07e5fa  ($07E5FA, 8 B, kernel/task_handlers.c)
/// $07E5FA: lea.l $7fdbc(pc), a1
pub fn i_07e5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E5FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07E5FE: move.l a1, (a6)
pub fn i_07e5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E600;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E600: rts
pub fn i_07e600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E602;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Cockpit_Init_07e602  ($07E602, 106 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E602: move.w #$24, d1
pub fn i_07e602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E606;
    cpu.ir = 0x323C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E606: jsr $236e.l
pub fn i_07e606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E60C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E60C);
    cpu.pc = 0x236E;
    16
}
/// $07E60C: bset.b #$1, $12(a6)
pub fn i_07e60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E612;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07E612: clr.b $20(a6)
pub fn i_07e612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E616;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07E616: clr.b $21(a6)
pub fn i_07e616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E61A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07E61A: move.w #$5000, $38(a6)
pub fn i_07e61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E620;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E620: movea.l $c(a6), a0
pub fn i_07e620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E624;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E624: tst.b $82(a0)
pub fn i_07e624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E628;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07E628: bne.w $7e640
pub fn i_07e628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E62C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E640; return 10; }
    8
}
/// $07E62C: lea.l $2bcbc4.l, a0
pub fn i_07e62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E632;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCBC4;
    20
}
/// $07E632: jsr $799de.l
pub fn i_07e632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E638;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E638);
    cpu.pc = 0x799DE;
    16
}
/// $07E638: move.w d0, $66(a6)
pub fn i_07e638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E63C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E63C: bra.w $7e650
pub fn i_07e63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E640;
    cpu.ir = 0x6000;
    cpu.pc = 0x7E650;
    10
}
/// $07E640: lea.l $2bccc8.l, a0
pub fn i_07e640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E646;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCCC8;
    20
}
/// $07E646: jsr $799de.l
pub fn i_07e646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E64C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E64C);
    cpu.pc = 0x799DE;
    16
}
/// $07E64C: move.w d0, $66(a6)
pub fn i_07e64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E650;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E650: lea.l $2e1f08.l, a0
pub fn i_07e650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E656;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E1F08;
    20
}
/// $07E656: jsr $28cd4.l
pub fn i_07e656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E65C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E65C);
    cpu.pc = 0x28CD4;
    16
}
/// $07E65C: lea.l $7e8b4(pc), a1
pub fn i_07e65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E660;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E8B4;
    16
}
/// $07E660: jsr $4ae.l
pub fn i_07e660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E666;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E666);
    cpu.pc = 0x4AE;
    16
}
/// $07E666: jsr $5dd02.l
pub fn i_07e666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E66C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E66C);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_07e66c  ($07E66C, 8 B, kernel/task_handlers.c)
/// $07E66C: lea.l $7e674(pc), a1
pub fn i_07e66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E670;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E674;
    16
}
/// $07E670: move.l a1, (a6)
pub fn i_07e670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E672;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E672: rts
pub fn i_07e672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E674;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Cockpit_Run_07e674  ($07E674, 148 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E674: movea.l $c(a6), a0
pub fn i_07e674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E678;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E678: move.w $22(a0), $22(a6)
pub fn i_07e678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E67E;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E67E: move.w $24(a0), $24(a6)
pub fn i_07e67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E684;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E684: subq.w #$4, $24(a6)
pub fn i_07e684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E688;
    cpu.ir = 0x596E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07E688: move.w $22(a6), $78(a0)
pub fn i_07e688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E68E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x78);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E68E: move.w $24(a6), $7a(a0)
pub fn i_07e68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E694;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x7A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E694: jsr $28d70.l
pub fn i_07e694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E69A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E69A);
    cpu.pc = 0x28D70;
    16
}
/// $07E69A: move.w $22(a6), d0
pub fn i_07e69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E69E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E69E: move.w $24(a6), d1
pub fn i_07e69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6A2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E6A2: addi.w #$ffd8, d0
pub fn i_07e6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6A6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFFD8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E6A6: addi.w #$30, d1
pub fn i_07e6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6AA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E6AA: move.w #$28, d2
pub fn i_07e6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6AE;
    cpu.ir = 0x343C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E6AE: jsr $99812.l
pub fn i_07e6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E6B4);
    cpu.pc = 0x99812;
    16
}
/// $07E6B4: move.w $22(a6), d0
pub fn i_07e6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6B8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E6B8: move.w $24(a6), d1
pub fn i_07e6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6BC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E6BC: addi.w #$ffc8, d0
pub fn i_07e6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6C0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFFC8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E6C0: move.w #$58, d2
pub fn i_07e6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6C4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x58;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E6C4: jsr $99812.l
pub fn i_07e6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E6CA);
    cpu.pc = 0x99812;
    16
}
/// $07E6CA: move.w $22(a6), d0
pub fn i_07e6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6CE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E6CE: move.w $24(a6), d1
pub fn i_07e6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6D2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E6D2: addi.w #$10, d0
pub fn i_07e6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6D6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E6D6: addi.w #$10, d1
pub fn i_07e6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6DA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E6DA: move.w #$20, d2
pub fn i_07e6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6DE;
    cpu.ir = 0x343C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E6DE: jsr $99812.l
pub fn i_07e6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E6E4);
    cpu.pc = 0x99812;
    16
}
/// $07E6E4: jsr $2870a.l
pub fn i_07e6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E6EA);
    cpu.pc = 0x2870A;
    16
}
/// $07E6EA: bcc.w $7e700
pub fn i_07e6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E700; return 10; }
    8
}
/// $07E6EE: bclr.b #$3, $13(a6)
pub fn i_07e6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07E6F4: lea.l $5e766.l, a0
pub fn i_07e6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E6FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07E6FA: jsr $5e770.l
pub fn i_07e6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E700;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E700);
    cpu.pc = 0x5E770;
    16
}
/// $07E700: tst.w $66(a6)
pub fn i_07e700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E704;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07E704: bne.w $7e70e
pub fn i_07e704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E708;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E70E; return 10; }
    8
}
// ===== Carrier_Cockpit_MarkHit_07e708  ($07E708, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E708: bset.b #$0, $13(a6)
pub fn i_07e708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E70E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07E70E: jsr $28758.l
pub fn i_07e70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E714;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E714);
    cpu.pc = 0x28758;
    16
}
/// $07E714: bcc.w $7e71e
pub fn i_07e714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E718;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E71E; return 10; }
    8
}
/// $07E718: lea.l $7e72e(pc), a1
pub fn i_07e718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E71C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E72E;
    16
}
/// $07E71C: move.l a1, (a6)
pub fn i_07e71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E71E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E71E: jsr $7fde0(pc)
pub fn i_07e71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E722;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E722);
    cpu.pc = 0x7FDE0;
    16
}
/// $07E722: bcc.w $7e72c
pub fn i_07e722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E726;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E72C; return 10; }
    8
}
// ===== SetTaskHandler_07e726  ($07E726, 8 B, kernel/task_handlers.c)
/// $07E726: lea.l $7fdb6(pc), a1
pub fn i_07e726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E72A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDB6;
    16
}
/// $07E72A: move.l a1, (a6)
pub fn i_07e72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E72C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E72C: rts
pub fn i_07e72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E72E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Cockpit_Eject_07e72e  ($07E72E, 286 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E72E: movea.l $c(a6), a0
pub fn i_07e72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E732;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E732: move.w $22(a0), $22(a6)
pub fn i_07e732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E738;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E738: move.w $24(a0), $24(a6)
pub fn i_07e738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E73E;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E73E: subq.w #$4, $24(a6)
pub fn i_07e73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E742;
    cpu.ir = 0x596E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07E742: move.b #$4, $20(a0)
pub fn i_07e742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E748;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07E748: jsr $267e2.l
pub fn i_07e748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E74E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E74E);
    cpu.pc = 0x267E2;
    16
}
/// $07E74E: move.w #$ff40, $2a(a6)
pub fn i_07e74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E754;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E754: moveq #$0, d0
pub fn i_07e754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E756;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07E756: move.b d0, $21(a6)
pub fn i_07e756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E75A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07E75A: move.w d0, $66(a6)
pub fn i_07e75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E75E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E75E: move.w d0, $74(a6)
pub fn i_07e75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E762;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E762: move.w d0, $76(a6)
pub fn i_07e762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E766;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E766: jsr $434dc.l
pub fn i_07e766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E76C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E76C);
    cpu.pc = 0x434DC;
    16
}
/// $07E76C: bclr.b #$1, $12(a6)
pub fn i_07e76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E772;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07E772: lea.l $2e1f1e.l, a0
pub fn i_07e772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E778;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E1F1E;
    20
}
/// $07E778: jsr $28cd4.l
pub fn i_07e778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E77E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E77E);
    cpu.pc = 0x28CD4;
    16
}
/// $07E77E: lea.l $77fd6.l, a1
pub fn i_07e77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E784;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $07E784: jsr $4ae.l
pub fn i_07e784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E78A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E78A);
    cpu.pc = 0x4AE;
    16
}
/// $07E78A: jsr $5dd02.l
pub fn i_07e78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E790);
    cpu.pc = 0x5DD02;
    16
}
/// $07E790: lea.l $7e796(pc), a1
pub fn i_07e790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E794;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E796;
    16
}
/// $07E794: move.l a1, (a6)
pub fn i_07e794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E796;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E796: jsr $27cee.l
pub fn i_07e796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E79C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E79C);
    cpu.pc = 0x27CEE;
    16
}
/// $07E79C: movea.l $c(a6), a0
pub fn i_07e79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7A0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E7A0: move.w $22(a6), $78(a0)
pub fn i_07e7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7A6;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x78);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E7A6: move.w $24(a6), $7a(a0)
pub fn i_07e7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7AC;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x7A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E7AC: move.l #$2e23ca, $60(a6)
pub fn i_07e7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7B4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E23CA;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07E7B4: jsr $28998.l
pub fn i_07e7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E7BA);
    cpu.pc = 0x28998;
    16
}
/// $07E7BA: move.l #$2e2416, $60(a6)
pub fn i_07e7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7C2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2416;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07E7C2: jsr $28998.l
pub fn i_07e7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E7C8);
    cpu.pc = 0x28998;
    16
}
/// $07E7C8: move.l #$2e2462, $60(a6)
pub fn i_07e7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7D0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2462;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07E7D0: jsr $28998.l
pub fn i_07e7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E7D6);
    cpu.pc = 0x28998;
    16
}
/// $07E7D6: jsr $28d70.l
pub fn i_07e7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E7DC);
    cpu.pc = 0x28D70;
    16
}
/// $07E7DC: move.w $22(a6), d0
pub fn i_07e7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7E0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E7E0: move.w $24(a6), d1
pub fn i_07e7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7E4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E7E4: addi.w #$ffd8, d0
pub fn i_07e7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7E8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFFD8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E7E8: addi.w #$30, d1
pub fn i_07e7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7EC;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x30, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E7EC: move.w #$28, d2
pub fn i_07e7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7F0;
    cpu.ir = 0x343C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E7F0: jsr $99812.l
pub fn i_07e7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E7F6);
    cpu.pc = 0x99812;
    16
}
/// $07E7F6: move.w $22(a6), d0
pub fn i_07e7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7FA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E7FA: move.w $24(a6), d1
pub fn i_07e7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E7FE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E7FE: addi.w #$ffc8, d0
pub fn i_07e7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E802;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFFC8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E802: move.w #$58, d2
pub fn i_07e802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E806;
    cpu.ir = 0x343C;
    let v1: u32 = 0x58;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E806: jsr $99812.l
pub fn i_07e806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E80C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E80C);
    cpu.pc = 0x99812;
    16
}
/// $07E80C: move.w $22(a6), d0
pub fn i_07e80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E810;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E810: move.w $24(a6), d1
pub fn i_07e810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E814;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E814: addi.w #$10, d0
pub fn i_07e814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E818;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E818: addi.w #$10, d1
pub fn i_07e818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E81C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E81C: move.w #$20, d2
pub fn i_07e81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E820;
    cpu.ir = 0x343C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E820: jsr $99812.l
pub fn i_07e820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E826);
    cpu.pc = 0x99812;
    16
}
/// $07E826: jsr $7feb4(pc)
pub fn i_07e826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E82A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E82A);
    cpu.pc = 0x7FEB4;
    16
}
/// $07E82A: cmpi.w #$d0, $24(a6)
pub fn i_07e82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E830;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD0, Size::Word, false);
    16
}
/// $07E830: bgt.w $7e844
pub fn i_07e830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E834;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7E844; return 10; }
    8
}
/// $07E834: movea.l $c(a6), a0
pub fn i_07e834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E838;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E838: move.b #$5, $20(a0)
pub fn i_07e838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E83E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07E83E: move.w #$fbc0, $2a(a6)
pub fn i_07e83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E844;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFBC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E844: jsr $7fde0(pc)
pub fn i_07e844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E848;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E848);
    cpu.pc = 0x7FDE0;
    16
}
/// $07E848: bcc.w $7e852
pub fn i_07e848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E84C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E852; return 10; }
    8
}
// ===== SetTaskHandler_07e84c  ($07E84C, 8 B, kernel/task_handlers.c)
/// $07E84C: lea.l $7fdb6(pc), a1
pub fn i_07e84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E850;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDB6;
    16
}
/// $07E850: move.l a1, (a6)
pub fn i_07e850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E852;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E852: rts
pub fn i_07e852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E854;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Mark_Left_07e854  ($07E854, 14 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E854: move.l #$2e2332, $60(a6)
pub fn i_07e854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E85C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E2332;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07E85C: move.w #$ffc8, $70(a6)
pub fn i_07e85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E862;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_07e862  ($07E862, 8 B, kernel/task_handlers.c)
/// $07E862: lea.l $7e880(pc), a1
pub fn i_07e862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E866;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E880;
    16
}
/// $07E866: move.l a1, (a6)
pub fn i_07e866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E868;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E868: rts
pub fn i_07e868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E86A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Mark_Right_07e86a  ($07E86A, 14 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E86A: move.l #$2e237e, $60(a6)
pub fn i_07e86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E872;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E237E;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07E872: move.w #$48, $70(a6)
pub fn i_07e872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E878;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_07e878  ($07E878, 8 B, kernel/task_handlers.c)
/// $07E878: lea.l $7e880(pc), a1
pub fn i_07e878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E87C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E880;
    16
}
/// $07E87C: move.l a1, (a6)
pub fn i_07e87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E87E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E87E: rts
pub fn i_07e87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E880;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Mark_Run_07e880  ($07E880, 52 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E880: movea.l $c(a6), a0
pub fn i_07e880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E884;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E884: move.w $22(a0), $22(a6)
pub fn i_07e884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E88A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E88A: move.w $24(a0), $24(a6)
pub fn i_07e88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E890;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E890: move.w $70(a6), d0
pub fn i_07e890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E894;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E894: add.w d0, $22(a6)
pub fn i_07e894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E898;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07E898: jsr $28998.l
pub fn i_07e898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E89E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E89E);
    cpu.pc = 0x28998;
    16
}
/// $07E89E: movea.l $c(a6), a0
pub fn i_07e89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8A2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E8A2: cmpi.b #$1, $21(a0)
pub fn i_07e8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8A8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07E8A8: bne.w $7e8b2
pub fn i_07e8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8AC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E8B2; return 10; }
    8
}
/// $07E8AC: jmp $518.l
pub fn i_07e8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8B2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07E8B2: rts
pub fn i_07e8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Mark_Cockpit_07e8b4  ($07E8B4, 62 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E8B4: move.l #$2e24ae, $60(a6)
pub fn i_07e8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8BC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E24AE;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07E8BC: lea.l $7e8c2(pc), a1
pub fn i_07e8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E8C2;
    16
}
/// $07E8C0: move.l a1, (a6)
pub fn i_07e8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E8C2: movea.l $c(a6), a0
pub fn i_07e8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8C6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E8C6: move.w $22(a0), $22(a6)
pub fn i_07e8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8CC;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E8CC: move.w $24(a0), $24(a6)
pub fn i_07e8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8D2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E8D2: addi.w #$20, $22(a6)
pub fn i_07e8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8D8;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07E8D8: jsr $28998.l
pub fn i_07e8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E8DE);
    cpu.pc = 0x28998;
    16
}
/// $07E8DE: movea.l $c(a6), a0
pub fn i_07e8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8E2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E8E2: tst.b $21(a0)
pub fn i_07e8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8E6;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07E8E6: beq.w $7e8f0
pub fn i_07e8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8EA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7E8F0; return 10; }
    8
}
/// $07E8EA: jmp $518.l
pub fn i_07e8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8F0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07E8F0: rts
pub fn i_07e8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Hatch_Init_07e8f2  ($07E8F2, 140 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E8F2: move.w #$25, d1
pub fn i_07e8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8F6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x25;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07E8F6: jsr $236e.l
pub fn i_07e8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E8FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E8FC);
    cpu.pc = 0x236E;
    16
}
/// $07E8FC: movea.l $c(a6), a0
pub fn i_07e8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E900;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E900: move.w $38(a0), $38(a6)
pub fn i_07e900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E906;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E906: addi.w #$18, $38(a6)
pub fn i_07e906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E90C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07E90C: clr.b $20(a6)
pub fn i_07e90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E910;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07E910: clr.b $21(a6)
pub fn i_07e910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E914;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07E914: clr.w $7c(a6)
pub fn i_07e914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E918;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07E918: lea.l $2e1f42.l, a0
pub fn i_07e918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E91E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E1F42;
    20
}
/// $07E91E: jsr $28cd4.l
pub fn i_07e91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E924;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E924);
    cpu.pc = 0x28CD4;
    16
}
/// $07E924: lea.l $7e92a(pc), a1
pub fn i_07e924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E928;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E92A;
    16
}
/// $07E928: move.l a1, (a6)
pub fn i_07e928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E92A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E92A: movea.l $c(a6), a0
pub fn i_07e92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E92E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E92E: move.w $22(a0), $22(a6)
pub fn i_07e92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E934;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E934: move.w $24(a0), $24(a6)
pub fn i_07e934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E93A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E93A: move.w $76(a0), d0
pub fn i_07e93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E93E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E93E: sub.w d0, $24(a6)
pub fn i_07e93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E942;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07E942: jsr $80026(pc)
pub fn i_07e942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E946;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E946);
    cpu.pc = 0x80026;
    16
}
/// $07E946: move.w $7c(a6), d0
pub fn i_07e946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E94A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E94A: andi.w #$3, d0
pub fn i_07e94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E94E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07E94E: lea.l $2e1f7a.l, a0
pub fn i_07e94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E954;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E1F7A;
    20
}
/// $07E954: asl.w #$2, d0
pub fn i_07e954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E956;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07E956: move.l (a0, d0.w), $5c(a6)
pub fn i_07e956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E95C;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $07E95C: jsr $28d70.l
pub fn i_07e95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E962;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E962);
    cpu.pc = 0x28D70;
    16
}
/// $07E962: movea.l $c(a6), a0
pub fn i_07e962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E966;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E966: cmpi.b #$1, $21(a0)
pub fn i_07e966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E96C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07E96C: bne.w $7e976
pub fn i_07e96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E970;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E976; return 10; }
    8
}
/// $07E970: lea.l $7e986(pc), a1
pub fn i_07e970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E974;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E986;
    16
}
/// $07E974: move.l a1, (a6)
pub fn i_07e974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E976;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E976: jsr $7fde0(pc)
pub fn i_07e976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E97A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E97A);
    cpu.pc = 0x7FDE0;
    16
}
/// $07E97A: bcc.w $7e984
pub fn i_07e97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E97E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E984; return 10; }
    8
}
// ===== SetTaskHandler_07e97e  ($07E97E, 8 B, kernel/task_handlers.c)
/// $07E97E: lea.l $7fdbc(pc), a1
pub fn i_07e97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E982;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07E982: move.l a1, (a6)
pub fn i_07e982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E984;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E984: rts
pub fn i_07e984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E986;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Hatch_Open_07e986  ($07E986, 76 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E986: lea.l $2e1f9c.l, a0
pub fn i_07e986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E98C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E1F9C;
    20
}
/// $07E98C: jsr $28cd4.l
pub fn i_07e98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E992;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E992);
    cpu.pc = 0x28CD4;
    16
}
/// $07E992: lea.l $7e998(pc), a1
pub fn i_07e992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E996;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E998;
    16
}
/// $07E996: move.l a1, (a6)
pub fn i_07e996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E998;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E998: movea.l $c(a6), a0
pub fn i_07e998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E99C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E99C: move.w $22(a0), $22(a6)
pub fn i_07e99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9A2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E9A2: move.w $24(a0), $24(a6)
pub fn i_07e9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9A8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E9A8: move.w $76(a0), d0
pub fn i_07e9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9AC;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07E9AC: sub.w d0, $24(a6)
pub fn i_07e9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9B0;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07E9B0: jsr $28d70.l
pub fn i_07e9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E9B6);
    cpu.pc = 0x28D70;
    16
}
/// $07E9B6: movea.l $c(a6), a0
pub fn i_07e9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9BA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E9BA: cmpi.b #$2, $21(a0)
pub fn i_07e9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9C0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $07E9C0: bne.w $7e9ca
pub fn i_07e9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7E9CA; return 10; }
    8
}
/// $07E9C4: lea.l $7e9da(pc), a1
pub fn i_07e9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E9DA;
    16
}
/// $07E9C8: move.l a1, (a6)
pub fn i_07e9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E9CA: jsr $7fde0(pc)
pub fn i_07e9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7E9CE);
    cpu.pc = 0x7FDE0;
    16
}
/// $07E9CE: bcc.w $7e9d8
pub fn i_07e9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7E9D8; return 10; }
    8
}
// ===== SetTaskHandler_07e9d2  ($07E9D2, 8 B, kernel/task_handlers.c)
/// $07E9D2: lea.l $7fdbc(pc), a1
pub fn i_07e9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07E9D6: move.l a1, (a6)
pub fn i_07e9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E9D8: rts
pub fn i_07e9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Hatch_Open2_07e9da  ($07E9DA, 76 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E9DA: lea.l $2e1ff8.l, a0
pub fn i_07e9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E1FF8;
    20
}
/// $07E9E0: jsr $28cd4.l
pub fn i_07e9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E9E6);
    cpu.pc = 0x28CD4;
    16
}
/// $07E9E6: lea.l $7e9ec(pc), a1
pub fn i_07e9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E9EC;
    16
}
/// $07E9EA: move.l a1, (a6)
pub fn i_07e9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E9EC: movea.l $c(a6), a0
pub fn i_07e9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9F0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07E9F0: move.w $22(a0), $22(a6)
pub fn i_07e9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9F6;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E9F6: move.w $24(a0), $24(a6)
pub fn i_07e9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E9FC;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07E9FC: move.w $76(a0), d0
pub fn i_07e9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA00;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07EA00: sub.w d0, $24(a6)
pub fn i_07ea00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA04;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07EA04: jsr $28d70.l
pub fn i_07ea04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EA0A);
    cpu.pc = 0x28D70;
    16
}
/// $07EA0A: movea.l $c(a6), a0
pub fn i_07ea0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA0E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07EA0E: cmpi.b #$3, $21(a0)
pub fn i_07ea0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA14;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $07EA14: bne.w $7ea1e
pub fn i_07ea14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA18;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7EA1E; return 10; }
    8
}
/// $07EA18: lea.l $7ea2e(pc), a1
pub fn i_07ea18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EA2E;
    16
}
/// $07EA1C: move.l a1, (a6)
pub fn i_07ea1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EA1E: jsr $7fde0(pc)
pub fn i_07ea1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EA22);
    cpu.pc = 0x7FDE0;
    16
}
/// $07EA22: bcc.w $7ea2c
pub fn i_07ea22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EA2C; return 10; }
    8
}
// ===== SetTaskHandler_07ea26  ($07EA26, 8 B, kernel/task_handlers.c)
/// $07EA26: lea.l $7fdbc(pc), a1
pub fn i_07ea26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07EA2A: move.l a1, (a6)
pub fn i_07ea2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EA2C: rts
pub fn i_07ea2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Hatch_Drop_07ea2e  ($07EA2E, 74 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EA2E: lea.l $2e202c.l, a0
pub fn i_07ea2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E202C;
    20
}
/// $07EA34: jsr $28cd4.l
pub fn i_07ea34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EA3A);
    cpu.pc = 0x28CD4;
    16
}
/// $07EA3A: lea.l $7ea40(pc), a1
pub fn i_07ea3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EA40;
    16
}
/// $07EA3E: move.l a1, (a6)
pub fn i_07ea3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EA40: movea.l $c(a6), a0
pub fn i_07ea40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA44;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07EA44: move.w $22(a0), $22(a6)
pub fn i_07ea44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA4A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07EA4A: move.w $24(a0), $24(a6)
pub fn i_07ea4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA50;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07EA50: move.w $76(a0), d0
pub fn i_07ea50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA54;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07EA54: sub.w d0, $24(a6)
pub fn i_07ea54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA58;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07EA58: move.w $78(a6), d0
pub fn i_07ea58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA5C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07EA5C: sub.w d0, $22(a6)
pub fn i_07ea5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA60;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07EA60: jsr $28d70.l
pub fn i_07ea60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EA66);
    cpu.pc = 0x28D70;
    16
}
/// $07EA66: bcc.w $7ea70
pub fn i_07ea66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EA70; return 10; }
    8
}
/// $07EA6A: lea.l $7ea80(pc), a1
pub fn i_07ea6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EA80;
    16
}
/// $07EA6E: move.l a1, (a6)
pub fn i_07ea6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EA70: jsr $7fde0(pc)
pub fn i_07ea70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EA74);
    cpu.pc = 0x7FDE0;
    16
}
/// $07EA74: bcc.w $7ea7e
pub fn i_07ea74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA78;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EA7E; return 10; }
    8
}
// ===== SetTaskHandler_07ea78  ($07EA78, 8 B, kernel/task_handlers.c)
/// $07EA78: lea.l $7fdbc(pc), a1
pub fn i_07ea78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07EA7C: move.l a1, (a6)
pub fn i_07ea7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EA7E: rts
pub fn i_07ea7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Hatch_Fall_07ea80  ($07EA80, 58 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EA80: move.w #$c, d1
pub fn i_07ea80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA84;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07EA84: jsr $236e.l
pub fn i_07ea84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EA8A);
    cpu.pc = 0x236E;
    16
}
/// $07EA8A: movea.l $c(a6), a0
pub fn i_07ea8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA8E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07EA8E: move.b #$ff, $21(a0)
pub fn i_07ea8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA94;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07EA94: subq.w #$4, $24(a6)
pub fn i_07ea94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA98;
    cpu.ir = 0x596E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07EA98: lea.l $2de6e0.l, a0
pub fn i_07ea98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EA9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE6E0;
    20
}
/// $07EA9E: jsr $28cd4.l
pub fn i_07ea9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EAA4);
    cpu.pc = 0x28CD4;
    16
}
/// $07EAA4: lea.l $7eaaa(pc), a1
pub fn i_07eaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EAAA;
    16
}
/// $07EAA8: move.l a1, (a6)
pub fn i_07eaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EAAA: jsr $2783a.l
pub fn i_07eaaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EAB0);
    cpu.pc = 0x2783A;
    16
}
/// $07EAB0: jsr $28d70.l
pub fn i_07eab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EAB6);
    cpu.pc = 0x28D70;
    16
}
/// $07EAB6: bcc.w $7eac0
pub fn i_07eab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EABA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EAC0; return 10; }
    8
}
// ===== SetTaskHandler_07eaba  ($07EABA, 8 B, kernel/task_handlers.c)
/// $07EABA: lea.l $7fdbc(pc), a1
pub fn i_07eaba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EABE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07EABE: move.l a1, (a6)
pub fn i_07eabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EAC0: rts
pub fn i_07eac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Cannon_Init_07eac2  ($07EAC2, 96 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EAC2: move.w #$4, d1
pub fn i_07eac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAC6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07EAC6: jsr $236e.l
pub fn i_07eac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EACC);
    cpu.pc = 0x236E;
    16
}
/// $07EACC: movea.l $c(a6), a0
pub fn i_07eacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAD0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07EAD0: move.w $38(a0), $38(a6)
pub fn i_07ead0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAD6;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07EAD6: addi.w #$30, $38(a6)
pub fn i_07ead6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EADC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07EADC: clr.b $20(a6)
pub fn i_07eadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAE0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07EAE0: clr.b $21(a6)
pub fn i_07eae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAE4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07EAE4: jsr $267e2.l
pub fn i_07eae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EAEA);
    cpu.pc = 0x267E2;
    16
}
/// $07EAEA: lea.l $2e20aa.l, a0
pub fn i_07eaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAF0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E20AA;
    20
}
/// $07EAF0: jsr $28cd4.l
pub fn i_07eaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EAF6);
    cpu.pc = 0x28CD4;
    16
}
/// $07EAF6: lea.l $7eafc(pc), a1
pub fn i_07eaf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EAFC;
    16
}
/// $07EAFA: move.l a1, (a6)
pub fn i_07eafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EAFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EAFC: jsr $2783a.l
pub fn i_07eafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EB02);
    cpu.pc = 0x2783A;
    16
}
/// $07EB02: jsr $28d70.l
pub fn i_07eb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EB08);
    cpu.pc = 0x28D70;
    16
}
/// $07EB08: bcc.w $7eb12
pub fn i_07eb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB0C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EB12; return 10; }
    8
}
/// $07EB0C: lea.l $7eb2a(pc), a1
pub fn i_07eb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EB2A;
    16
}
/// $07EB10: move.l a1, (a6)
pub fn i_07eb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EB12: movea.l #$ffffffff, a0
pub fn i_07eb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB18;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07EB18: jsr $5dd56.l
pub fn i_07eb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EB1E);
    cpu.pc = 0x5DD56;
    16
}
/// $07EB1E: bcc.w $7eb28
pub fn i_07eb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EB28; return 10; }
    8
}
// ===== SetTaskHandler_07eb22  ($07EB22, 8 B, kernel/task_handlers.c)
/// $07EB22: lea.l $7fdbc(pc), a1
pub fn i_07eb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07EB26: move.l a1, (a6)
pub fn i_07eb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EB28: rts
pub fn i_07eb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Cannon_Fall_07eb2a  ($07EB2A, 68 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EB2A: jsr $267e2.l
pub fn i_07eb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EB30);
    cpu.pc = 0x267E2;
    16
}
/// $07EB30: move.w #$100, $2a(a6)
pub fn i_07eb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB36;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07EB36: lea.l $2e20e4.l, a0
pub fn i_07eb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB3C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E20E4;
    20
}
/// $07EB3C: jsr $28cd4.l
pub fn i_07eb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EB42);
    cpu.pc = 0x28CD4;
    16
}
/// $07EB42: lea.l $7eb48(pc), a1
pub fn i_07eb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EB48;
    16
}
/// $07EB46: move.l a1, (a6)
pub fn i_07eb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EB48: jsr $27cee.l
pub fn i_07eb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EB4E);
    cpu.pc = 0x27CEE;
    16
}
/// $07EB4E: jsr $28d70.l
pub fn i_07eb4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EB54);
    cpu.pc = 0x28D70;
    16
}
/// $07EB54: bcc.w $7eb5e
pub fn i_07eb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EB5E; return 10; }
    8
}
/// $07EB58: lea.l $7fdbc(pc), a1
pub fn i_07eb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07EB5C: move.l a1, (a6)
pub fn i_07eb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EB5E: movea.l #$ffffffff, a0
pub fn i_07eb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB64;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07EB64: jsr $5dd56.l
pub fn i_07eb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EB6A);
    cpu.pc = 0x5DD56;
    16
}
/// $07EB6A: bcc.w $7eb74
pub fn i_07eb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EB74; return 10; }
    8
}
// ===== SetTaskHandler_07eb6e  ($07EB6E, 8 B, kernel/task_handlers.c)
/// $07EB6E: lea.l $7fdbc(pc), a1
pub fn i_07eb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07EB72: move.l a1, (a6)
pub fn i_07eb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EB74: rts
pub fn i_07eb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Spawner_Init_07eb76  ($07EB76, 104 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EB76: clr.b $20(a6)
pub fn i_07eb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB7A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07EB7A: bra.w $7ebc4
pub fn i_07eb7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EB7E;
    cpu.ir = 0x6000;
    cpu.pc = 0x7EBC4;
    10
}
/// $07EBC4: moveq #$0, d0
pub fn i_07ebc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBC6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07EBC6: move.b d0, $21(a6)
pub fn i_07ebc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBCA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07EBCA: move.b d0, $70(a6)
pub fn i_07ebca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBCE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07EBCE: move.b d0, $72(a6)
pub fn i_07ebce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBD2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07EBD2: move.b d0, $73(a6)
pub fn i_07ebd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBD6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07EBD6: move.b d0, $74(a6)
pub fn i_07ebd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBDA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07EBDA: move.w d0, $76(a6)
pub fn i_07ebda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBDE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_07ebde  ($07EBDE, 8 B, kernel/task_handlers.c)
/// $07EBDE: lea.l $7ebe6(pc), a1
pub fn i_07ebde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EBE6;
    16
}
/// $07EBE2: move.l a1, (a6)
pub fn i_07ebe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EBE4: rts
pub fn i_07ebe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Spawner_Run_07ebe6  ($07EBE6, 226 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EBE6: jsr $7ed00(pc)
pub fn i_07ebe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EBEA);
    cpu.pc = 0x7ED00;
    16
}
/// $07EBEA: btst.b #$0, $70(a6)
pub fn i_07ebea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBF0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07EBF0: bne.w $7ec26
pub fn i_07ebf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBF4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7EC26; return 10; }
    8
}
/// $07EBF4: addq.b #$1, $72(a6)
pub fn i_07ebf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBF8;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $07EBF8: move.b $72(a6), d0
pub fn i_07ebf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EBFC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07EBFC: cmp.b $92(a6), d0
pub fn i_07ebfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC00;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07EC00: bcs.w $7ec26
pub fn i_07ec00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC04;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7EC26; return 10; }
    8
}
/// $07EC04: move.b #$0, $72(a6)
pub fn i_07ec04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC0A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07EC0A: lea.l $7ed7c(pc), a1
pub fn i_07ec0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7ED7C;
    16
}
/// $07EC0E: jsr $4ae.l
pub fn i_07ec0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EC14);
    cpu.pc = 0x4AE;
    16
}
/// $07EC14: jsr $5dd02.l
pub fn i_07ec14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EC1A);
    cpu.pc = 0x5DD02;
    16
}
/// $07EC1A: move.w #$ffb8, $70(a0)
pub fn i_07ec1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC20;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFB8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07EC20: move.w #$0, $72(a0)
pub fn i_07ec20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC26;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07EC26: btst.b #$1, $70(a6)
pub fn i_07ec26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC2C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07EC2C: bne.w $7ec60
pub fn i_07ec2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC30;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7EC60; return 10; }
    8
}
/// $07EC30: addq.b #$1, $73(a6)
pub fn i_07ec30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC34;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $07EC34: move.b $73(a6), d0
pub fn i_07ec34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC38;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07EC38: cmp.b $92(a6), d0
pub fn i_07ec38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC3C;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07EC3C: bcs.w $7ec60
pub fn i_07ec3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC40;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7EC60; return 10; }
    8
}
/// $07EC40: clr.b $73(a6)
pub fn i_07ec40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC44;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07EC44: lea.l $7ed7c(pc), a1
pub fn i_07ec44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7ED7C;
    16
}
/// $07EC48: jsr $4ae.l
pub fn i_07ec48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EC4E);
    cpu.pc = 0x4AE;
    16
}
/// $07EC4E: jsr $5dd02.l
pub fn i_07ec4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EC54);
    cpu.pc = 0x5DD02;
    16
}
/// $07EC54: move.w #$28, $70(a0)
pub fn i_07ec54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC5A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07EC5A: move.w #$1, $72(a0)
pub fn i_07ec5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC60;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07EC60: btst.b #$2, $70(a6)
pub fn i_07ec60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC66;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07EC66: bne.w $7ec9a
pub fn i_07ec66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC6A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7EC9A; return 10; }
    8
}
/// $07EC6A: addq.b #$1, $74(a6)
pub fn i_07ec6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC6E;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $07EC6E: move.b $74(a6), d0
pub fn i_07ec6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC72;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07EC72: cmp.b $92(a6), d0
pub fn i_07ec72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC76;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07EC76: bcs.w $7ec9a
pub fn i_07ec76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC7A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7EC9A; return 10; }
    8
}
/// $07EC7A: clr.b $74(a6)
pub fn i_07ec7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC7E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07EC7E: lea.l $7ed7c(pc), a1
pub fn i_07ec7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7ED7C;
    16
}
/// $07EC82: jsr $4ae.l
pub fn i_07ec82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EC88);
    cpu.pc = 0x4AE;
    16
}
/// $07EC88: jsr $5dd02.l
pub fn i_07ec88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EC8E);
    cpu.pc = 0x5DD02;
    16
}
/// $07EC8E: move.w #$48, $70(a0)
pub fn i_07ec8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC94;
    cpu.ir = 0x317C;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07EC94: move.w #$2, $72(a0)
pub fn i_07ec94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC9A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07EC9A: movea.l $c(a6), a0
pub fn i_07ec9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EC9E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07EC9E: move.w $22(a0), $22(a6)
pub fn i_07ec9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECA4;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07ECA4: move.w $24(a0), $24(a6)
pub fn i_07eca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECAA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07ECAA: cmpi.b #$3, $20(a0)
pub fn i_07ecaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECB0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $07ECB0: bne.w $7ecc0
pub fn i_07ecb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECB4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7ECC0; return 10; }
    8
}
/// $07ECB4: move.b #$ff, $20(a6)
pub fn i_07ecb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECBA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ECBA: lea.l $7ecd0(pc), a1
pub fn i_07ecba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7ECD0;
    16
}
/// $07ECBE: move.l a1, (a6)
pub fn i_07ecbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07ECC0: jsr $7fde0(pc)
pub fn i_07ecc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7ECC4);
    cpu.pc = 0x7FDE0;
    16
}
/// $07ECC4: bcc.w $7ecce
pub fn i_07ecc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7ECCE; return 10; }
    8
}
// ===== SetTaskHandler_07ecc8  ($07ECC8, 8 B, kernel/task_handlers.c)
/// $07ECC8: lea.l $7fdb6(pc), a1
pub fn i_07ecc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDB6;
    16
}
/// $07ECCC: move.l a1, (a6)
pub fn i_07eccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07ECCE: rts
pub fn i_07ecce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Spawner_WaitParent_07ecd0  ($07ECD0, 40 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07ECD0: movea.l $c(a6), a0
pub fn i_07ecd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECD4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07ECD4: move.w $22(a0), $22(a6)
pub fn i_07ecd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECDA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07ECDA: move.w $24(a0), $24(a6)
pub fn i_07ecda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECE0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07ECE0: cmpi.b #$1, $21(a0)
pub fn i_07ece0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECE6;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07ECE6: bne.w $7ecf0
pub fn i_07ece6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7ECF0; return 10; }
    8
}
/// $07ECEA: lea.l $7fdbc(pc), a1
pub fn i_07ecea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07ECEE: move.l a1, (a6)
pub fn i_07ecee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07ECF0: jsr $7fde0(pc)
pub fn i_07ecf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7ECF4);
    cpu.pc = 0x7FDE0;
    16
}
/// $07ECF4: bcc.w $7ecfe
pub fn i_07ecf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECF8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7ECFE; return 10; }
    8
}
// ===== SetTaskHandler_07ecf8  ($07ECF8, 8 B, kernel/task_handlers.c)
/// $07ECF8: lea.l $7fdb6(pc), a1
pub fn i_07ecf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDB6;
    16
}
/// $07ECFC: move.l a1, (a6)
pub fn i_07ecfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ECFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07ECFE: rts
pub fn i_07ecfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Spawner_RateFromTbl_07ed00  ($07ED00, 36 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07ED00: cmpi.w #$f, $76(a6)
pub fn i_07ed00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED06;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF, Size::Word, false);
    16
}
/// $07ED06: bgt.w $7ed2a
pub fn i_07ed06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED0A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7ED2A; return 10; }
    8
}
/// $07ED0A: lea.l $2bcd4a.l, a0
pub fn i_07ed0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCD4A;
    20
}
/// $07ED10: jsr $799de.l
pub fn i_07ed10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ED16);
    cpu.pc = 0x799DE;
    16
}
/// $07ED16: move.b d0, $92(a6)
pub fn i_07ed16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED1A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ED1A: moveq #$0, d0
pub fn i_07ed1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED1C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07ED1C: move.b $92(a6), d0
pub fn i_07ed1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED20;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07ED20: mulu.w #$1e, d0
pub fn i_07ed20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED24;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0x1E;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
// ===== SetTaskB_07ed24  ($07ED24, 6 B, kernel/task_setters_b.c)
/// $07ED24: move.b d0, $92(a6)
pub fn i_07ed24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED28;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ED28: rts
pub fn i_07ed28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Spawner_RateFromTblB_07ed2a  ($07ED2A, 26 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07ED2A: lea.l $2bcdcc.l, a0
pub fn i_07ed2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCDCC;
    20
}
/// $07ED30: jsr $799de.l
pub fn i_07ed30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ED36);
    cpu.pc = 0x799DE;
    16
}
/// $07ED36: move.b d0, $92(a6)
pub fn i_07ed36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED3A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ED3A: moveq #$0, d0
pub fn i_07ed3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED3C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07ED3C: move.b $92(a6), d0
pub fn i_07ed3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED40;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07ED40: mulu.w #$1e, d0
pub fn i_07ed40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED44;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0x1E;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
// ===== SetTaskB_07ed44  ($07ED44, 6 B, kernel/task_setters_b.c)
/// $07ED44: move.b d0, $92(a6)
pub fn i_07ed44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED48;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ED48: rts
pub fn i_07ed48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_HPFromTbl_07ed4a  ($07ED4A, 26 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07ED4A: movea.l $c(a6), a0
pub fn i_07ed4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED4E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07ED4E: cmpi.w #$f, $76(a0)
pub fn i_07ed4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED54;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF, Size::Word, false);
    16
}
/// $07ED54: bgt.w $7ed6a
pub fn i_07ed54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED58;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7ED6A; return 10; }
    8
}
/// $07ED58: lea.l $2bce4e.l, a0
pub fn i_07ed58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCE4E;
    20
}
/// $07ED5E: jsr $799de.l
pub fn i_07ed5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ED64);
    cpu.pc = 0x799DE;
    16
}
// ===== SetTaskW_07ed64  ($07ED64, 6 B, kernel/task_setters_w.c)
/// $07ED64: move.w d0, $90(a6)
pub fn i_07ed64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED68;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ED68: rts
pub fn i_07ed68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_HPFromTblB_07ed6a  ($07ED6A, 12 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07ED6A: lea.l $2bced0.l, a0
pub fn i_07ed6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCED0;
    20
}
/// $07ED70: jsr $799de.l
pub fn i_07ed70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ED76);
    cpu.pc = 0x799DE;
    16
}
// ===== SetTaskW_07ed76  ($07ED76, 6 B, kernel/task_setters_w.c)
/// $07ED76: move.w d0, $90(a6)
pub fn i_07ed76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED7A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ED7A: rts
pub fn i_07ed7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_Init_07ed7c  ($07ED7C, 112 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07ED7C: movea.l $c(a6), a0
pub fn i_07ed7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED80;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07ED80: move.w $72(a6), d0
pub fn i_07ed80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED84;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07ED84: bset.b d0, $70(a0)
pub fn i_07ed84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED88;
    cpu.ir = 0x1E8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07ED88: move.w #$0, $76(a6)
pub fn i_07ed88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED8E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07ED8E: move.w #$1d, d1
pub fn i_07ed8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED92;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07ED92: jsr $236e.l
pub fn i_07ed92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7ED98);
    cpu.pc = 0x236E;
    16
}
/// $07ED98: move.b #$1, $5c(a6)
pub fn i_07ed98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7ED9E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07ED9E: jsr $7ed4a(pc)
pub fn i_07ed9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EDA2);
    cpu.pc = 0x7ED4A;
    16
}
/// $07EDA2: clr.b $20(a6)
pub fn i_07eda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDA6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07EDA6: clr.b $21(a6)
pub fn i_07eda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDAA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07EDAA: move.w #$1, $66(a6)
pub fn i_07edaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDB0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07EDB0: jsr $267e2.l
pub fn i_07edb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EDB6);
    cpu.pc = 0x267E2;
    16
}
/// $07EDB6: lea.l $2e2286.l, a0
pub fn i_07edb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDBC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2286;
    20
}
/// $07EDBC: jsr $28cd4.l
pub fn i_07edbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EDC2);
    cpu.pc = 0x28CD4;
    16
}
/// $07EDC2: lea.l $7edc8(pc), a1
pub fn i_07edc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EDC8;
    16
}
/// $07EDC6: move.l a1, (a6)
pub fn i_07edc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EDC8: jsr $7fdfa(pc)
pub fn i_07edc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDCC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EDCC);
    cpu.pc = 0x7FDFA;
    16
}
/// $07EDCC: jsr $28d70.l
pub fn i_07edcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EDD2);
    cpu.pc = 0x28D70;
    16
}
/// $07EDD2: bcc.w $7eddc
pub fn i_07edd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EDDC; return 10; }
    8
}
/// $07EDD6: lea.l $7edf4(pc), a1
pub fn i_07edd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EDF4;
    16
}
/// $07EDDA: move.l a1, (a6)
pub fn i_07edda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EDDC: jsr $7ef12(pc)
pub fn i_07eddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EDE0);
    cpu.pc = 0x7EF12;
    16
}
/// $07EDE0: jsr $80034(pc)
pub fn i_07ede0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EDE4);
    cpu.pc = 0x80034;
    16
}
/// $07EDE4: jsr $7fde0(pc)
pub fn i_07ede4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EDE8);
    cpu.pc = 0x7FDE0;
    16
}
/// $07EDE8: bcc.w $7edf2
pub fn i_07ede8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EDF2; return 10; }
    8
}
// ===== SetTaskHandler_07edec  ($07EDEC, 8 B, kernel/task_handlers.c)
/// $07EDEC: lea.l $7fdbc(pc), a1
pub fn i_07edec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07EDF0: move.l a1, (a6)
pub fn i_07edf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EDF2: rts
pub fn i_07edf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_Jump_07edf4  ($07EDF4, 64 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EDF4: move.w #$e, d1
pub fn i_07edf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDF8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07EDF8: jsr $236e.l
pub fn i_07edf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EDFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EDFE);
    cpu.pc = 0x236E;
    16
}
/// $07EDFE: lea.l $2e22e8.l, a0
pub fn i_07edfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE04;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E22E8;
    20
}
/// $07EE04: jsr $28cd4.l
pub fn i_07ee04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EE0A);
    cpu.pc = 0x28CD4;
    16
}
/// $07EE0A: lea.l $7ee10(pc), a1
pub fn i_07ee0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EE10;
    16
}
/// $07EE0E: move.l a1, (a6)
pub fn i_07ee0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EE10: jsr $7fdfa(pc)
pub fn i_07ee10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EE14);
    cpu.pc = 0x7FDFA;
    16
}
/// $07EE14: jsr $28d70.l
pub fn i_07ee14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EE1A);
    cpu.pc = 0x28D70;
    16
}
/// $07EE1A: bcc.w $7ee24
pub fn i_07ee1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE1E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EE24; return 10; }
    8
}
/// $07EE1E: lea.l $7ee84(pc), a1
pub fn i_07ee1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EE84;
    16
}
/// $07EE22: move.l a1, (a6)
pub fn i_07ee22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EE24: jsr $7ef12(pc)
pub fn i_07ee24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EE28);
    cpu.pc = 0x7EF12;
    16
}
/// $07EE28: jsr $80034(pc)
pub fn i_07ee28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EE2C);
    cpu.pc = 0x80034;
    16
}
/// $07EE2C: jsr $7fde0(pc)
pub fn i_07ee2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EE30);
    cpu.pc = 0x7FDE0;
    16
}
/// $07EE30: bcc.w $7ee3a
pub fn i_07ee30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE34;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EE3A; return 10; }
    8
}
// ===== SetTaskHandler_07ee34  ($07EE34, 8 B, kernel/task_handlers.c)
/// $07EE34: lea.l $7fdbc(pc), a1
pub fn i_07ee34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07EE38: move.l a1, (a6)
pub fn i_07ee38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EE3A: rts
pub fn i_07ee3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_Land_07ee3c  ($07EE3C, 54 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EE3C: lea.l $29bfb8.l, a0
pub fn i_07ee3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE42;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BFB8;
    20
}
/// $07EE42: jsr $28cd4.l
pub fn i_07ee42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EE48);
    cpu.pc = 0x28CD4;
    16
}
/// $07EE48: lea.l $7ee4e(pc), a1
pub fn i_07ee48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EE4E;
    16
}
/// $07EE4C: move.l a1, (a6)
pub fn i_07ee4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EE4E: jsr $7fdfa(pc)
pub fn i_07ee4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EE52);
    cpu.pc = 0x7FDFA;
    16
}
/// $07EE52: jsr $28d70.l
pub fn i_07ee52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EE58);
    cpu.pc = 0x28D70;
    16
}
/// $07EE58: bcc.w $7ee62
pub fn i_07ee58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EE62; return 10; }
    8
}
/// $07EE5C: lea.l $7ee7a(pc), a1
pub fn i_07ee5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EE7A;
    16
}
/// $07EE60: move.l a1, (a6)
pub fn i_07ee60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EE62: jsr $7ef12(pc)
pub fn i_07ee62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EE66);
    cpu.pc = 0x7EF12;
    16
}
/// $07EE66: jsr $80034(pc)
pub fn i_07ee66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EE6A);
    cpu.pc = 0x80034;
    16
}
/// $07EE6A: jsr $7fde0(pc)
pub fn i_07ee6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE6E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EE6E);
    cpu.pc = 0x7FDE0;
    16
}
/// $07EE6E: bcc.w $7ee78
pub fn i_07ee6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EE78; return 10; }
    8
}
// ===== SetTaskHandler_07ee72  ($07EE72, 8 B, kernel/task_handlers.c)
/// $07EE72: lea.l $7fdbc(pc), a1
pub fn i_07ee72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07EE76: move.l a1, (a6)
pub fn i_07ee76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EE78: rts
pub fn i_07ee78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_Run_07ee7a  ($07EE7A, 144 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EE7A: move.b #$1, $3a(a6)
pub fn i_07ee7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE80;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07EE80: bra.w $7ee8c
pub fn i_07ee80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE84;
    cpu.ir = 0x6000;
    cpu.pc = 0x7EE8C;
    10
}
/// $07EE8C: move.w #$e, d1
pub fn i_07ee8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE90;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07EE90: jsr $236e.l
pub fn i_07ee90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EE96);
    cpu.pc = 0x236E;
    16
}
/// $07EE96: jsr $267e2.l
pub fn i_07ee96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EE9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EE9C);
    cpu.pc = 0x267E2;
    16
}
/// $07EE9C: move.w #$fe00, $28(a6)
pub fn i_07ee9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEA2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07EEA2: btst.b #$0, $3a(a6)
pub fn i_07eea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEA8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07EEA8: beq.w $7eeb0
pub fn i_07eea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7EEB0; return 10; }
    8
}
/// $07EEAC: neg.w $28(a6)
pub fn i_07eeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEB0;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07EEB0: clr.w $74(a6)
pub fn i_07eeb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEB4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07EEB4: lea.l $29b744.l, a0
pub fn i_07eeb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEBA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B744;
    20
}
/// $07EEBA: jsr $28cd4.l
pub fn i_07eeba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EEC0);
    cpu.pc = 0x28CD4;
    16
}
/// $07EEC0: lea.l $7eec6(pc), a1
pub fn i_07eec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EEC6;
    16
}
/// $07EEC4: move.l a1, (a6)
pub fn i_07eec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EEC6: jsr $7fdfa(pc)
pub fn i_07eec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EECA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EECA);
    cpu.pc = 0x7FDFA;
    16
}
/// $07EECA: jsr $28d70.l
pub fn i_07eeca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EED0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EED0);
    cpu.pc = 0x28D70;
    16
}
/// $07EED0: move.w $74(a6), d0
pub fn i_07eed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EED4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07EED4: asr.w #$8, d0
pub fn i_07eed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EED6;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07EED6: cmp.w $70(a6), d0
pub fn i_07eed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEDA;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07EEDA: bne.w $7eeee
pub fn i_07eeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEDE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7EEEE; return 10; }
    8
}
/// $07EEDE: jsr $267e2.l
pub fn i_07eede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EEE4);
    cpu.pc = 0x267E2;
    16
}
/// $07EEE4: lea.l $7ef4a(pc), a1
pub fn i_07eee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EF4A;
    16
}
/// $07EEE8: move.l a1, (a6)
pub fn i_07eee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EEEA: bra.w $7eefa
pub fn i_07eeea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x7EEFA;
    10
}
/// $07EEEE: move.w $74(a6), d0
pub fn i_07eeee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEF2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07EEF2: add.w $28(a6), d0
pub fn i_07eef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEF6;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07EEF6: move.w d0, $74(a6)
pub fn i_07eef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEFA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07EEFA: jsr $7ef12(pc)
pub fn i_07eefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EEFE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EEFE);
    cpu.pc = 0x7EF12;
    16
}
/// $07EEFE: jsr $80034(pc)
pub fn i_07eefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EF02);
    cpu.pc = 0x80034;
    16
}
/// $07EF02: jsr $7fde0(pc)
pub fn i_07ef02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EF06);
    cpu.pc = 0x7FDE0;
    16
}
/// $07EF06: bcc.w $7ef10
pub fn i_07ef06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EF10; return 10; }
    8
}
// ===== SetTaskHandler_07ef0a  ($07EF0A, 8 B, kernel/task_handlers.c)
/// $07EF0A: lea.l $7fdbc(pc), a1
pub fn i_07ef0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07EF0E: move.l a1, (a6)
pub fn i_07ef0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EF10: rts
pub fn i_07ef10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_OnHit_07ef12  ($07EF12, 48 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EF12: jsr $2870a.l
pub fn i_07ef12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EF18);
    cpu.pc = 0x2870A;
    16
}
/// $07EF18: bcc.w $7ef48
pub fn i_07ef18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EF48; return 10; }
    8
}
/// $07EF1C: move.w $72(a6), d0
pub fn i_07ef1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF20;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07EF20: movea.l $c(a6), a0
pub fn i_07ef20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF24;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07EF24: bclr.b d0, $70(a0)
pub fn i_07ef24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF28;
    cpu.ir = 0x1A8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07EF28: addq.w #$1, $76(a0)
pub fn i_07ef28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF2C;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07EF2C: move.w #$c000, d0
pub fn i_07ef2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF30;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07EF30: jsr $28134.l
pub fn i_07ef30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EF36);
    cpu.pc = 0x28134;
    16
}
/// $07EF36: andi.w #$ffe3, $38(a6)
pub fn i_07ef36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF3C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07EF3C: ori.w #$10, $38(a6)
pub fn i_07ef3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF42;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== JsrAbsThunk_07ef42  ($07EF42, 8 B, thunks/jsr_abs_thunks.c)
/// $07EF42: jsr $49fd0.l
pub fn i_07ef42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EF48);
    cpu.pc = 0x49FD0;
    16
}
/// $07EF48: rts
pub fn i_07ef48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_Stand_07ef4a  ($07EF4A, 54 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EF4A: lea.l $29b7c8.l, a0
pub fn i_07ef4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF50;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B7C8;
    20
}
/// $07EF50: jsr $28cd4.l
pub fn i_07ef50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EF56);
    cpu.pc = 0x28CD4;
    16
}
/// $07EF56: lea.l $7ef5c(pc), a1
pub fn i_07ef56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EF5C;
    16
}
/// $07EF5A: move.l a1, (a6)
pub fn i_07ef5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EF5C: jsr $7fdfa(pc)
pub fn i_07ef5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EF60);
    cpu.pc = 0x7FDFA;
    16
}
/// $07EF60: jsr $28d70.l
pub fn i_07ef60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EF66);
    cpu.pc = 0x28D70;
    16
}
/// $07EF66: bcc.w $7ef70
pub fn i_07ef66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EF70; return 10; }
    8
}
/// $07EF6A: lea.l $7ef88(pc), a1
pub fn i_07ef6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EF88;
    16
}
/// $07EF6E: move.l a1, (a6)
pub fn i_07ef6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EF70: jsr $7f63c(pc)
pub fn i_07ef70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EF74);
    cpu.pc = 0x7F63C;
    16
}
/// $07EF74: jsr $7ef12(pc)
pub fn i_07ef74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EF78);
    cpu.pc = 0x7EF12;
    16
}
/// $07EF78: jsr $7fde0(pc)
pub fn i_07ef78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EF7C);
    cpu.pc = 0x7FDE0;
    16
}
/// $07EF7C: bcc.w $7ef86
pub fn i_07ef7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EF86; return 10; }
    8
}
// ===== SetTaskHandler_07ef80  ($07EF80, 8 B, kernel/task_handlers.c)
/// $07EF80: lea.l $7fdbc(pc), a1
pub fn i_07ef80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07EF84: move.l a1, (a6)
pub fn i_07ef84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EF86: rts
pub fn i_07ef86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_Idle_07ef88  ($07EF88, 60 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EF88: lea.l $29b816.l, a0
pub fn i_07ef88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF8E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B816;
    20
}
/// $07EF8E: jsr $28cd4.l
pub fn i_07ef8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EF94);
    cpu.pc = 0x28CD4;
    16
}
/// $07EF94: lea.l $7ef9a(pc), a1
pub fn i_07ef94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EF9A;
    16
}
/// $07EF98: move.l a1, (a6)
pub fn i_07ef98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EF9A: jsr $7fdfa(pc)
pub fn i_07ef9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EF9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EF9E);
    cpu.pc = 0x7FDFA;
    16
}
/// $07EF9E: jsr $28d70.l
pub fn i_07ef9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EFA4);
    cpu.pc = 0x28D70;
    16
}
/// $07EFA4: tst.w $90(a6)
pub fn i_07efa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFA8;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07EFA8: bne.w $7efb0
pub fn i_07efa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFAC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7EFB0; return 10; }
    8
}
/// $07EFAC: jsr $7efcc(pc)
pub fn i_07efac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFB0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EFB0);
    cpu.pc = 0x7EFCC;
    16
}
/// $07EFB0: subq.w #$1, $90(a6)
pub fn i_07efb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFB4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07EFB4: jsr $7f63c(pc)
pub fn i_07efb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EFB8);
    cpu.pc = 0x7F63C;
    16
}
/// $07EFB8: jsr $7ef12(pc)
pub fn i_07efb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EFBC);
    cpu.pc = 0x7EF12;
    16
}
/// $07EFBC: jsr $7fde0(pc)
pub fn i_07efbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7EFC0);
    cpu.pc = 0x7FDE0;
    16
}
/// $07EFC0: bcc.w $7efca
pub fn i_07efc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7EFCA; return 10; }
    8
}
// ===== SetTaskHandler_07efc4  ($07EFC4, 8 B, kernel/task_handlers.c)
/// $07EFC4: lea.l $7fdbc(pc), a1
pub fn i_07efc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07EFC8: move.l a1, (a6)
pub fn i_07efc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07EFCA: rts
pub fn i_07efca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFCC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_DecideByPlayer_07efcc  ($07EFCC, 70 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07EFCC: jsr $5e0d4.l
pub fn i_07efcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7EFD2);
    cpu.pc = 0x5E0D4;
    16
}
/// $07EFD2: move.w $22(a0), d0
pub fn i_07efd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFD6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07EFD6: cmp.w $22(a6), d0
pub fn i_07efd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFDA;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07EFDA: ble.w $7eff8
pub fn i_07efda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFDE;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7EFF8; return 10; }
    8
}
/// $07EFDE: sub.w $22(a6), d0
pub fn i_07efde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFE2;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07EFE2: cmpi.w #$30, d0
pub fn i_07efe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFE6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $07EFE6: blt.w $7f1b4
pub fn i_07efe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFEA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7F1B4; return 10; }
    8
}
/// $07EFEA: btst.b #$0, $3a(a6)
pub fn i_07efea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFF0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07EFF0: bne.w $7f012
pub fn i_07eff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFF4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F012; return 10; }
    8
}
/// $07EFF4: bra.w $7f01a
pub fn i_07eff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x7F01A;
    10
}
/// $07EFF8: move.w $22(a6), d0
pub fn i_07eff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7EFFC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07EFFC: sub.w $22(a0), d0
pub fn i_07effc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F000;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07F000: cmpi.w #$30, d0
pub fn i_07f000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F004;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $07F004: blt.w $7f1b4
pub fn i_07f004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F008;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7F1B4; return 10; }
    8
}
/// $07F008: btst.b #$0, $3a(a6)
pub fn i_07f008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F00E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07F00E: bne.w $7f01a
pub fn i_07f00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F012;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F01A; return 10; }
    8
}
// ===== SetTaskHandler_07f012  ($07F012, 8 B, kernel/task_handlers.c)
/// $07F012: lea.l $7f186(pc), a1
pub fn i_07f012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F016;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F186;
    16
}
/// $07F016: move.l a1, (a6)
pub fn i_07f016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F018;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F018: rts
pub fn i_07f018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F01A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_07f01a  ($07F01A, 8 B, kernel/task_handlers.c)
/// $07F01A: lea.l $7f022(pc), a1
pub fn i_07f01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F01E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F022;
    16
}
/// $07F01E: move.l a1, (a6)
pub fn i_07f01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F020;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F020: rts
pub fn i_07f020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F022;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_Aim_07f022  ($07F022, 54 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F022: lea.l $29bf34.l, a0
pub fn i_07f022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F028;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BF34;
    20
}
/// $07F028: jsr $28cd4.l
pub fn i_07f028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F02E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F02E);
    cpu.pc = 0x28CD4;
    16
}
/// $07F02E: lea.l $7f034(pc), a1
pub fn i_07f02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F032;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F034;
    16
}
/// $07F032: move.l a1, (a6)
pub fn i_07f032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F034;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F034: jsr $7fdfa(pc)
pub fn i_07f034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F038;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F038);
    cpu.pc = 0x7FDFA;
    16
}
/// $07F038: jsr $28d70.l
pub fn i_07f038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F03E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F03E);
    cpu.pc = 0x28D70;
    16
}
/// $07F03E: bcc.w $7f048
pub fn i_07f03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F042;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F048; return 10; }
    8
}
/// $07F042: lea.l $7f186(pc), a1
pub fn i_07f042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F046;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F186;
    16
}
/// $07F046: move.l a1, (a6)
pub fn i_07f046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F048;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F048: jsr $7f63c(pc)
pub fn i_07f048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F04C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F04C);
    cpu.pc = 0x7F63C;
    16
}
/// $07F04C: jsr $7ef12(pc)
pub fn i_07f04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F050;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F050);
    cpu.pc = 0x7EF12;
    16
}
/// $07F050: jsr $7fde0(pc)
pub fn i_07f050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F054;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F054);
    cpu.pc = 0x7FDE0;
    16
}
/// $07F054: bcc.w $7f05e
pub fn i_07f054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F058;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F05E; return 10; }
    8
}
// ===== SetTaskHandler_07f058  ($07F058, 8 B, kernel/task_handlers.c)
/// $07F058: lea.l $7fdbc(pc), a1
pub fn i_07f058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F05C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F05C: move.l a1, (a6)
pub fn i_07f05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F05E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F05E: rts
pub fn i_07f05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F060;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_Walk_07f060  ($07F060, 148 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F060: eori.b #$1, $3a(a6)
pub fn i_07f060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F066;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07F066: jsr $267e2.l
pub fn i_07f066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F06C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F06C);
    cpu.pc = 0x267E2;
    16
}
/// $07F06C: move.w #$fe00, $28(a6)
pub fn i_07f06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F072;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F072: btst.b #$0, $3a(a6)
pub fn i_07f072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F078;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07F078: beq.w $7f080
pub fn i_07f078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F07C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7F080; return 10; }
    8
}
/// $07F07C: neg.w $28(a6)
pub fn i_07f07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F080;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07F080: move.w #$0, $76(a6)
pub fn i_07f080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F086;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F086: lea.l $29b744.l, a0
pub fn i_07f086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F08C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B744;
    20
}
/// $07F08C: jsr $28cd4.l
pub fn i_07f08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F092;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F092);
    cpu.pc = 0x28CD4;
    16
}
/// $07F092: lea.l $7f098(pc), a1
pub fn i_07f092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F096;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F098;
    16
}
/// $07F096: move.l a1, (a6)
pub fn i_07f096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F098;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F098: jsr $7fdfa(pc)
pub fn i_07f098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F09C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F09C);
    cpu.pc = 0x7FDFA;
    16
}
/// $07F09C: jsr $28d70.l
pub fn i_07f09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F0A2);
    cpu.pc = 0x28D70;
    16
}
/// $07F0A2: subq.w #$1, $90(a6)
pub fn i_07f0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0A6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07F0A6: beq.w $7f0d8
pub fn i_07f0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7F0D8; return 10; }
    8
}
/// $07F0AA: move.w $74(a6), d0
pub fn i_07f0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0AE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07F0AE: add.w $28(a6), d0
pub fn i_07f0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0B2;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07F0B2: move.w d0, $74(a6)
pub fn i_07f0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0B6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F0B6: move.w $74(a6), d0
pub fn i_07f0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0BA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07F0BA: asr.w #$8, d0
pub fn i_07f0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0BC;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07F0BC: tst.w $28(a6)
pub fn i_07f0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0C0;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07F0C0: bmi.w $7f0d0
pub fn i_07f0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0C4;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x7F0D0; return 10; }
    8
}
/// $07F0C4: cmp.w $70(a6), d0
pub fn i_07f0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0C8;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07F0C8: bge.w $7f0d8
pub fn i_07f0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0CC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x7F0D8; return 10; }
    8
}
/// $07F0CC: bra.w $7f0e4
pub fn i_07f0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x7F0E4;
    10
}
/// $07F0D0: cmp.w $76(a6), d0
pub fn i_07f0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0D4;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07F0D4: bgt.w $7f0e4
pub fn i_07f0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0D8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7F0E4; return 10; }
    8
}
/// $07F0D8: jsr $267e2.l
pub fn i_07f0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F0DE);
    cpu.pc = 0x267E2;
    16
}
/// $07F0DE: lea.l $7f0fc(pc), a1
pub fn i_07f0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F0FC;
    16
}
/// $07F0E2: move.l a1, (a6)
pub fn i_07f0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F0E4: jsr $7ef12(pc)
pub fn i_07f0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F0E8);
    cpu.pc = 0x7EF12;
    16
}
/// $07F0E8: jsr $80034(pc)
pub fn i_07f0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F0EC);
    cpu.pc = 0x80034;
    16
}
/// $07F0EC: jsr $7fde0(pc)
pub fn i_07f0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F0F0);
    cpu.pc = 0x7FDE0;
    16
}
/// $07F0F0: bcc.w $7f0fa
pub fn i_07f0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F0FA; return 10; }
    8
}
// ===== SetTaskHandler_07f0f4  ($07F0F4, 8 B, kernel/task_handlers.c)
/// $07F0F4: lea.l $7fdbc(pc), a1
pub fn i_07f0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F0F8: move.l a1, (a6)
pub fn i_07f0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F0FA: rts
pub fn i_07f0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F0FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_Stop_07f0fc  ($07F0FC, 68 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F0FC: lea.l $29b7c8.l, a0
pub fn i_07f0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F102;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B7C8;
    20
}
/// $07F102: jsr $28cd4.l
pub fn i_07f102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F108;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F108);
    cpu.pc = 0x28CD4;
    16
}
/// $07F108: lea.l $7f10e(pc), a1
pub fn i_07f108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F10C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F10E;
    16
}
/// $07F10C: move.l a1, (a6)
pub fn i_07f10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F10E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F10E: jsr $7fdfa(pc)
pub fn i_07f10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F112;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F112);
    cpu.pc = 0x7FDFA;
    16
}
/// $07F112: jsr $28d70.l
pub fn i_07f112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F118;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F118);
    cpu.pc = 0x28D70;
    16
}
/// $07F118: bcc.w $7f130
pub fn i_07f118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F11C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F130; return 10; }
    8
}
/// $07F11C: lea.l $7f148(pc), a1
pub fn i_07f11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F120;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F148;
    16
}
/// $07F120: move.l a1, (a6)
pub fn i_07f120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F122;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F122: tst.w $90(a6)
pub fn i_07f122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F126;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07F126: bne.w $7f130
pub fn i_07f126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F12A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F130; return 10; }
    8
}
/// $07F12A: lea.l $7ef88(pc), a1
pub fn i_07f12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F12E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EF88;
    16
}
/// $07F12E: move.l a1, (a6)
pub fn i_07f12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F130;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F130: jsr $7ef12(pc)
pub fn i_07f130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F134;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F134);
    cpu.pc = 0x7EF12;
    16
}
/// $07F134: jsr $80034(pc)
pub fn i_07f134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F138;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F138);
    cpu.pc = 0x80034;
    16
}
/// $07F138: jsr $7fde0(pc)
pub fn i_07f138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F13C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F13C);
    cpu.pc = 0x7FDE0;
    16
}
/// $07F13C: bcc.w $7f146
pub fn i_07f13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F140;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F146; return 10; }
    8
}
// ===== SetTaskHandler_07f140  ($07F140, 8 B, kernel/task_handlers.c)
/// $07F140: lea.l $7fdbc(pc), a1
pub fn i_07f140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F144;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F144: move.l a1, (a6)
pub fn i_07f144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F146;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F146: rts
pub fn i_07f146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F148;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_Turn_07f148  ($07F148, 54 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F148: lea.l $29bfb8.l, a0
pub fn i_07f148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F14E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BFB8;
    20
}
/// $07F14E: jsr $28cd4.l
pub fn i_07f14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F154);
    cpu.pc = 0x28CD4;
    16
}
/// $07F154: lea.l $7f15a(pc), a1
pub fn i_07f154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F158;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F15A;
    16
}
/// $07F158: move.l a1, (a6)
pub fn i_07f158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F15A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F15A: jsr $7fdfa(pc)
pub fn i_07f15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F15E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F15E);
    cpu.pc = 0x7FDFA;
    16
}
/// $07F15E: jsr $28d70.l
pub fn i_07f15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F164;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F164);
    cpu.pc = 0x28D70;
    16
}
/// $07F164: bcc.w $7f16e
pub fn i_07f164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F168;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F16E; return 10; }
    8
}
/// $07F168: lea.l $7f060(pc), a1
pub fn i_07f168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F16C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F060;
    16
}
/// $07F16C: move.l a1, (a6)
pub fn i_07f16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F16E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F16E: jsr $7ef12(pc)
pub fn i_07f16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F172;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F172);
    cpu.pc = 0x7EF12;
    16
}
/// $07F172: jsr $80034(pc)
pub fn i_07f172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F176;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F176);
    cpu.pc = 0x80034;
    16
}
/// $07F176: jsr $7fde0(pc)
pub fn i_07f176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F17A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F17A);
    cpu.pc = 0x7FDE0;
    16
}
/// $07F17A: bcc.w $7f184
pub fn i_07f17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F17E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F184; return 10; }
    8
}
// ===== SetTaskHandler_07f17e  ($07F17E, 8 B, kernel/task_handlers.c)
/// $07F17E: lea.l $7fdbc(pc), a1
pub fn i_07f17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F182;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07F182: move.l a1, (a6)
pub fn i_07f182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F184;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F184: rts
pub fn i_07f184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F186;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Trooper_Fire_07f186  ($07F186, 156 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07F186: jsr $5e9b6.l
pub fn i_07f186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F18C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F18C);
    cpu.pc = 0x5E9B6;
    16
}
/// $07F18C: andi.w #$3, d0
pub fn i_07f18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F190;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07F190: move.b d0, $5c(a6)
pub fn i_07f190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F194;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07F194: lea.l $29ba70.l, a0
pub fn i_07f194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F19A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BA70;
    20
}
/// $07F19A: jsr $28cd4.l
pub fn i_07f19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F1A0);
    cpu.pc = 0x28CD4;
    16
}
/// $07F1A0: lea.l $7f1a6(pc), a1
pub fn i_07f1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F1A6;
    16
}
/// $07F1A4: move.l a1, (a6)
pub fn i_07f1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F1A6: jsr $7fdfa(pc)
pub fn i_07f1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F1AA);
    cpu.pc = 0x7FDFA;
    16
}
/// $07F1AA: jsr $28d70.l
pub fn i_07f1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F1B0);
    cpu.pc = 0x28D70;
    16
}
/// $07F1B0: bcc.w $7f212
pub fn i_07f1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7F212; return 10; }
    8
}
/// $07F1B4: jsr $7ed4a(pc)
pub fn i_07f1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F1B8);
    cpu.pc = 0x7ED4A;
    16
}
/// $07F1B8: lea.l $7ef88(pc), a1
pub fn i_07f1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EF88;
    16
}
/// $07F1BC: move.l a1, (a6)
pub fn i_07f1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F1BE: movea.l $c(a6), a0
pub fn i_07f1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1C2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F1C2: movea.l $c(a0), a0
pub fn i_07f1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1C6;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F1C6: cmpi.w #$1f4, $66(a0)
pub fn i_07f1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1CC;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F4, Size::Word, false);
    16
}
/// $07F1CC: bgt.w $7f202
pub fn i_07f1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1D0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7F202; return 10; }
    8
}
/// $07F1D0: jsr $5e9b6.l
pub fn i_07f1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F1D6);
    cpu.pc = 0x5E9B6;
    16
}
/// $07F1D6: andi.w #$f, d0
pub fn i_07f1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1DA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07F1DA: cmpi.w #$9, d0
pub fn i_07f1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1DE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x9, Size::Word, false);
    8
}
/// $07F1DE: blt.w $7f202
pub fn i_07f1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1E2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7F202; return 10; }
    8
}
/// $07F1E2: move.w $72(a6), d0
pub fn i_07f1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1E6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07F1E6: movea.l $c(a6), a0
pub fn i_07f1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1EA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07F1EA: bclr.b d0, $70(a0)
pub fn i_07f1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1EE;
    cpu.ir = 0x1A8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07F1EE: jsr $5e9b6.l
pub fn i_07f1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7F1F4);
    cpu.pc = 0x5E9B6;
    16
}
/// $07F1F4: andi.w #$1, d0
pub fn i_07f1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1F8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07F1F8: add.w d0, d0
pub fn i_07f1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1FA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07F1FA: move.w d0, $72(a6)
pub fn i_07f1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F1FE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07F1FE: jmp $7f64a(pc)
pub fn i_07f1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F202;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x7F64A;
    8
}
/// $07F202: cmpi.w #$1, $72(a6)
pub fn i_07f202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F208;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $07F208: bne.w $7f212
pub fn i_07f208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F20C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7F212; return 10; }
    8
}
/// $07F20C: lea.l $7f066(pc), a1
pub fn i_07f20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F210;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F066;
    16
}
/// $07F210: move.l a1, (a6)
pub fn i_07f210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F212;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07F212: jsr $7f63c(pc)
pub fn i_07f212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F216;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F216);
    cpu.pc = 0x7F63C;
    16
}
/// $07F216: jsr $7ef12(pc)
pub fn i_07f216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F21A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F21A);
    cpu.pc = 0x7EF12;
    16
}
/// $07F21A: jsr $7fde0(pc)
pub fn i_07f21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7F21E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7F21E);
    cpu.pc = 0x7FDE0;
    16
}
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

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x07E0C2, i_07e0c2),
    (0x07E0C8, i_07e0c8),
    (0x07E0CE, i_07e0ce),
    (0x07E0D2, i_07e0d2),
    (0x07E0D6, i_07e0d6),
    (0x07E0DC, i_07e0dc),
    (0x07E0E2, i_07e0e2),
    (0x07E0E6, i_07e0e6),
    (0x07E0EC, i_07e0ec),
    (0x07E0F2, i_07e0f2),
    (0x07E0F6, i_07e0f6),
    (0x07E0FC, i_07e0fc),
    (0x07E102, i_07e102),
    (0x07E106, i_07e106),
    (0x07E10C, i_07e10c),
    (0x07E112, i_07e112),
    (0x07E116, i_07e116),
    (0x07E11C, i_07e11c),
    (0x07E122, i_07e122),
    (0x07E126, i_07e126),
    (0x07E12C, i_07e12c),
    (0x07E132, i_07e132),
    (0x07E136, i_07e136),
    (0x07E138, i_07e138),
    (0x07E13C, i_07e13c),
    (0x07E142, i_07e142),
    (0x07E146, i_07e146),
    (0x07E14A, i_07e14a),
    (0x07E14E, i_07e14e),
    (0x07E152, i_07e152),
    (0x07E156, i_07e156),
    (0x07E15C, i_07e15c),
    (0x07E160, i_07e160),
    (0x07E164, i_07e164),
    (0x07E168, i_07e168),
    (0x07E16C, i_07e16c),
    (0x07E170, i_07e170),
    (0x07E176, i_07e176),
    (0x07E17A, i_07e17a),
    (0x07E17E, i_07e17e),
    (0x07E182, i_07e182),
    (0x07E184, i_07e184),
    (0x07E188, i_07e188),
    (0x07E18E, i_07e18e),
    (0x07E192, i_07e192),
    (0x07E196, i_07e196),
    (0x07E19A, i_07e19a),
    (0x07E19E, i_07e19e),
    (0x07E1A4, i_07e1a4),
    (0x07E1A8, i_07e1a8),
    (0x07E1AA, i_07e1aa),
    (0x07E1B0, i_07e1b0),
    (0x07E1B4, i_07e1b4),
    (0x07E1B8, i_07e1b8),
    (0x07E1BC, i_07e1bc),
    (0x07E1C2, i_07e1c2),
    (0x07E1C8, i_07e1c8),
    (0x07E1CE, i_07e1ce),
    (0x07E1D4, i_07e1d4),
    (0x07E1DA, i_07e1da),
    (0x07E1DE, i_07e1de),
    (0x07E1E4, i_07e1e4),
    (0x07E1EA, i_07e1ea),
    (0x07E1F0, i_07e1f0),
    (0x07E1F4, i_07e1f4),
    (0x07E1FA, i_07e1fa),
    (0x07E1FE, i_07e1fe),
    (0x07E204, i_07e204),
    (0x07E208, i_07e208),
    (0x07E20C, i_07e20c),
    (0x07E212, i_07e212),
    (0x07E218, i_07e218),
    (0x07E21C, i_07e21c),
    (0x07E220, i_07e220),
    (0x07E224, i_07e224),
    (0x07E22A, i_07e22a),
    (0x07E230, i_07e230),
    (0x07E234, i_07e234),
    (0x07E23A, i_07e23a),
    (0x07E240, i_07e240),
    (0x07E244, i_07e244),
    (0x07E248, i_07e248),
    (0x07E24E, i_07e24e),
    (0x07E252, i_07e252),
    (0x07E254, i_07e254),
    (0x07E258, i_07e258),
    (0x07E25C, i_07e25c),
    (0x07E260, i_07e260),
    (0x07E262, i_07e262),
    (0x07E264, i_07e264),
    (0x07E26A, i_07e26a),
    (0x07E270, i_07e270),
    (0x07E274, i_07e274),
    (0x07E27A, i_07e27a),
    (0x07E280, i_07e280),
    (0x07E284, i_07e284),
    (0x07E286, i_07e286),
    (0x07E28A, i_07e28a),
    (0x07E28E, i_07e28e),
    (0x07E292, i_07e292),
    (0x07E296, i_07e296),
    (0x07E298, i_07e298),
    (0x07E29A, i_07e29a),
    (0x07E2A0, i_07e2a0),
    (0x07E2A4, i_07e2a4),
    (0x07E2AA, i_07e2aa),
    (0x07E2AE, i_07e2ae),
    (0x07E2B2, i_07e2b2),
    (0x07E2B6, i_07e2b6),
    (0x07E2B8, i_07e2b8),
    (0x07E2BE, i_07e2be),
    (0x07E2C2, i_07e2c2),
    (0x07E2C6, i_07e2c6),
    (0x07E2CA, i_07e2ca),
    (0x07E2CE, i_07e2ce),
    (0x07E2D0, i_07e2d0),
    (0x07E2D2, i_07e2d2),
    (0x07E2D8, i_07e2d8),
    (0x07E2DC, i_07e2dc),
    (0x07E2E0, i_07e2e0),
    (0x07E2E2, i_07e2e2),
    (0x07E2E6, i_07e2e6),
    (0x07E2EA, i_07e2ea),
    (0x07E2EE, i_07e2ee),
    (0x07E2F0, i_07e2f0),
    (0x07E2F2, i_07e2f2),
    (0x07E2F8, i_07e2f8),
    (0x07E2FC, i_07e2fc),
    (0x07E302, i_07e302),
    (0x07E306, i_07e306),
    (0x07E30C, i_07e30c),
    (0x07E30E, i_07e30e),
    (0x07E312, i_07e312),
    (0x07E318, i_07e318),
    (0x07E31C, i_07e31c),
    (0x07E320, i_07e320),
    (0x07E324, i_07e324),
    (0x07E328, i_07e328),
    (0x07E32C, i_07e32c),
    (0x07E330, i_07e330),
    (0x07E334, i_07e334),
    (0x07E33A, i_07e33a),
    (0x07E340, i_07e340),
    (0x07E344, i_07e344),
    (0x07E348, i_07e348),
    (0x07E34C, i_07e34c),
    (0x07E350, i_07e350),
    (0x07E356, i_07e356),
    (0x07E358, i_07e358),
    (0x07E35A, i_07e35a),
    (0x07E35E, i_07e35e),
    (0x07E364, i_07e364),
    (0x07E36A, i_07e36a),
    (0x07E370, i_07e370),
    (0x07E376, i_07e376),
    (0x07E37C, i_07e37c),
    (0x07E382, i_07e382),
    (0x07E388, i_07e388),
    (0x07E38E, i_07e38e),
    (0x07E394, i_07e394),
    (0x07E39A, i_07e39a),
    (0x07E3A0, i_07e3a0),
    (0x07E3A6, i_07e3a6),
    (0x07E3AC, i_07e3ac),
    (0x07E3B0, i_07e3b0),
    (0x07E3B2, i_07e3b2),
    (0x07E3B6, i_07e3b6),
    (0x07E3BC, i_07e3bc),
    (0x07E3C2, i_07e3c2),
    (0x07E3C8, i_07e3c8),
    (0x07E3CE, i_07e3ce),
    (0x07E3D2, i_07e3d2),
    (0x07E3D6, i_07e3d6),
    (0x07E3D8, i_07e3d8),
    (0x07E3DC, i_07e3dc),
    (0x07E3E2, i_07e3e2),
    (0x07E3E6, i_07e3e6),
    (0x07E3EA, i_07e3ea),
    (0x07E3EC, i_07e3ec),
    (0x07E3F2, i_07e3f2),
    (0x07E3F6, i_07e3f6),
    (0x07E3FA, i_07e3fa),
    (0x07E3FC, i_07e3fc),
    (0x07E400, i_07e400),
    (0x07E404, i_07e404),
    (0x07E408, i_07e408),
    (0x07E40A, i_07e40a),
    (0x07E40C, i_07e40c),
    (0x07E410, i_07e410),
    (0x07E416, i_07e416),
    (0x07E41A, i_07e41a),
    (0x07E41E, i_07e41e),
    (0x07E422, i_07e422),
    (0x07E426, i_07e426),
    (0x07E42A, i_07e42a),
    (0x07E42E, i_07e42e),
    (0x07E432, i_07e432),
    (0x07E438, i_07e438),
    (0x07E43E, i_07e43e),
    (0x07E442, i_07e442),
    (0x07E446, i_07e446),
    (0x07E44A, i_07e44a),
    (0x07E44E, i_07e44e),
    (0x07E454, i_07e454),
    (0x07E456, i_07e456),
    (0x07E458, i_07e458),
    (0x07E45C, i_07e45c),
    (0x07E462, i_07e462),
    (0x07E468, i_07e468),
    (0x07E46E, i_07e46e),
    (0x07E474, i_07e474),
    (0x07E47A, i_07e47a),
    (0x07E480, i_07e480),
    (0x07E486, i_07e486),
    (0x07E48C, i_07e48c),
    (0x07E492, i_07e492),
    (0x07E498, i_07e498),
    (0x07E49E, i_07e49e),
    (0x07E4A4, i_07e4a4),
    (0x07E4A8, i_07e4a8),
    (0x07E4AA, i_07e4aa),
    (0x07E4AE, i_07e4ae),
    (0x07E4B4, i_07e4b4),
    (0x07E4BA, i_07e4ba),
    (0x07E4C0, i_07e4c0),
    (0x07E4C6, i_07e4c6),
    (0x07E4CA, i_07e4ca),
    (0x07E4CE, i_07e4ce),
    (0x07E4D0, i_07e4d0),
    (0x07E4D4, i_07e4d4),
    (0x07E4DA, i_07e4da),
    (0x07E4DE, i_07e4de),
    (0x07E4E2, i_07e4e2),
    (0x07E4E4, i_07e4e4),
    (0x07E4EA, i_07e4ea),
    (0x07E4EE, i_07e4ee),
    (0x07E4F2, i_07e4f2),
    (0x07E4F4, i_07e4f4),
    (0x07E4F8, i_07e4f8),
    (0x07E4FC, i_07e4fc),
    (0x07E500, i_07e500),
    (0x07E502, i_07e502),
    (0x07E504, i_07e504),
    (0x07E508, i_07e508),
    (0x07E50E, i_07e50e),
    (0x07E512, i_07e512),
    (0x07E516, i_07e516),
    (0x07E51A, i_07e51a),
    (0x07E51E, i_07e51e),
    (0x07E522, i_07e522),
    (0x07E526, i_07e526),
    (0x07E52A, i_07e52a),
    (0x07E530, i_07e530),
    (0x07E536, i_07e536),
    (0x07E53A, i_07e53a),
    (0x07E53E, i_07e53e),
    (0x07E542, i_07e542),
    (0x07E546, i_07e546),
    (0x07E54C, i_07e54c),
    (0x07E54E, i_07e54e),
    (0x07E550, i_07e550),
    (0x07E554, i_07e554),
    (0x07E55A, i_07e55a),
    (0x07E560, i_07e560),
    (0x07E566, i_07e566),
    (0x07E56C, i_07e56c),
    (0x07E572, i_07e572),
    (0x07E578, i_07e578),
    (0x07E57E, i_07e57e),
    (0x07E584, i_07e584),
    (0x07E58A, i_07e58a),
    (0x07E590, i_07e590),
    (0x07E596, i_07e596),
    (0x07E59C, i_07e59c),
    (0x07E5A2, i_07e5a2),
    (0x07E5A6, i_07e5a6),
    (0x07E5A8, i_07e5a8),
    (0x07E5AC, i_07e5ac),
    (0x07E5B2, i_07e5b2),
    (0x07E5B8, i_07e5b8),
    (0x07E5BE, i_07e5be),
    (0x07E5C4, i_07e5c4),
    (0x07E5C8, i_07e5c8),
    (0x07E5CC, i_07e5cc),
    (0x07E5CE, i_07e5ce),
    (0x07E5D2, i_07e5d2),
    (0x07E5D8, i_07e5d8),
    (0x07E5DC, i_07e5dc),
    (0x07E5E0, i_07e5e0),
    (0x07E5E2, i_07e5e2),
    (0x07E5E8, i_07e5e8),
    (0x07E5EC, i_07e5ec),
    (0x07E5F0, i_07e5f0),
    (0x07E5F2, i_07e5f2),
    (0x07E5F6, i_07e5f6),
    (0x07E5FA, i_07e5fa),
    (0x07E5FE, i_07e5fe),
    (0x07E600, i_07e600),
    (0x07E602, i_07e602),
    (0x07E606, i_07e606),
    (0x07E60C, i_07e60c),
    (0x07E612, i_07e612),
    (0x07E616, i_07e616),
    (0x07E61A, i_07e61a),
    (0x07E620, i_07e620),
    (0x07E624, i_07e624),
    (0x07E628, i_07e628),
    (0x07E62C, i_07e62c),
    (0x07E632, i_07e632),
    (0x07E638, i_07e638),
    (0x07E63C, i_07e63c),
    (0x07E640, i_07e640),
    (0x07E646, i_07e646),
    (0x07E64C, i_07e64c),
    (0x07E650, i_07e650),
    (0x07E656, i_07e656),
    (0x07E65C, i_07e65c),
    (0x07E660, i_07e660),
    (0x07E666, i_07e666),
    (0x07E66C, i_07e66c),
    (0x07E670, i_07e670),
    (0x07E672, i_07e672),
    (0x07E674, i_07e674),
    (0x07E678, i_07e678),
    (0x07E67E, i_07e67e),
    (0x07E684, i_07e684),
    (0x07E688, i_07e688),
    (0x07E68E, i_07e68e),
    (0x07E694, i_07e694),
    (0x07E69A, i_07e69a),
    (0x07E69E, i_07e69e),
    (0x07E6A2, i_07e6a2),
    (0x07E6A6, i_07e6a6),
    (0x07E6AA, i_07e6aa),
    (0x07E6AE, i_07e6ae),
    (0x07E6B4, i_07e6b4),
    (0x07E6B8, i_07e6b8),
    (0x07E6BC, i_07e6bc),
    (0x07E6C0, i_07e6c0),
    (0x07E6C4, i_07e6c4),
    (0x07E6CA, i_07e6ca),
    (0x07E6CE, i_07e6ce),
    (0x07E6D2, i_07e6d2),
    (0x07E6D6, i_07e6d6),
    (0x07E6DA, i_07e6da),
    (0x07E6DE, i_07e6de),
    (0x07E6E4, i_07e6e4),
    (0x07E6EA, i_07e6ea),
    (0x07E6EE, i_07e6ee),
    (0x07E6F4, i_07e6f4),
    (0x07E6FA, i_07e6fa),
    (0x07E700, i_07e700),
    (0x07E704, i_07e704),
    (0x07E708, i_07e708),
    (0x07E70E, i_07e70e),
    (0x07E714, i_07e714),
    (0x07E718, i_07e718),
    (0x07E71C, i_07e71c),
    (0x07E71E, i_07e71e),
    (0x07E722, i_07e722),
    (0x07E726, i_07e726),
    (0x07E72A, i_07e72a),
    (0x07E72C, i_07e72c),
    (0x07E72E, i_07e72e),
    (0x07E732, i_07e732),
    (0x07E738, i_07e738),
    (0x07E73E, i_07e73e),
    (0x07E742, i_07e742),
    (0x07E748, i_07e748),
    (0x07E74E, i_07e74e),
    (0x07E754, i_07e754),
    (0x07E756, i_07e756),
    (0x07E75A, i_07e75a),
    (0x07E75E, i_07e75e),
    (0x07E762, i_07e762),
    (0x07E766, i_07e766),
    (0x07E76C, i_07e76c),
    (0x07E772, i_07e772),
    (0x07E778, i_07e778),
    (0x07E77E, i_07e77e),
    (0x07E784, i_07e784),
    (0x07E78A, i_07e78a),
    (0x07E790, i_07e790),
    (0x07E794, i_07e794),
    (0x07E796, i_07e796),
    (0x07E79C, i_07e79c),
    (0x07E7A0, i_07e7a0),
    (0x07E7A6, i_07e7a6),
    (0x07E7AC, i_07e7ac),
    (0x07E7B4, i_07e7b4),
    (0x07E7BA, i_07e7ba),
    (0x07E7C2, i_07e7c2),
    (0x07E7C8, i_07e7c8),
    (0x07E7D0, i_07e7d0),
    (0x07E7D6, i_07e7d6),
    (0x07E7DC, i_07e7dc),
    (0x07E7E0, i_07e7e0),
    (0x07E7E4, i_07e7e4),
    (0x07E7E8, i_07e7e8),
    (0x07E7EC, i_07e7ec),
    (0x07E7F0, i_07e7f0),
    (0x07E7F6, i_07e7f6),
    (0x07E7FA, i_07e7fa),
    (0x07E7FE, i_07e7fe),
    (0x07E802, i_07e802),
    (0x07E806, i_07e806),
    (0x07E80C, i_07e80c),
    (0x07E810, i_07e810),
    (0x07E814, i_07e814),
    (0x07E818, i_07e818),
    (0x07E81C, i_07e81c),
    (0x07E820, i_07e820),
    (0x07E826, i_07e826),
    (0x07E82A, i_07e82a),
    (0x07E830, i_07e830),
    (0x07E834, i_07e834),
    (0x07E838, i_07e838),
    (0x07E83E, i_07e83e),
    (0x07E844, i_07e844),
    (0x07E848, i_07e848),
    (0x07E84C, i_07e84c),
    (0x07E850, i_07e850),
    (0x07E852, i_07e852),
    (0x07E854, i_07e854),
    (0x07E85C, i_07e85c),
    (0x07E862, i_07e862),
    (0x07E866, i_07e866),
    (0x07E868, i_07e868),
    (0x07E86A, i_07e86a),
    (0x07E872, i_07e872),
    (0x07E878, i_07e878),
    (0x07E87C, i_07e87c),
    (0x07E87E, i_07e87e),
    (0x07E880, i_07e880),
    (0x07E884, i_07e884),
    (0x07E88A, i_07e88a),
    (0x07E890, i_07e890),
    (0x07E894, i_07e894),
    (0x07E898, i_07e898),
    (0x07E89E, i_07e89e),
    (0x07E8A2, i_07e8a2),
    (0x07E8A8, i_07e8a8),
    (0x07E8AC, i_07e8ac),
    (0x07E8B2, i_07e8b2),
    (0x07E8B4, i_07e8b4),
    (0x07E8BC, i_07e8bc),
    (0x07E8C0, i_07e8c0),
    (0x07E8C2, i_07e8c2),
    (0x07E8C6, i_07e8c6),
    (0x07E8CC, i_07e8cc),
    (0x07E8D2, i_07e8d2),
    (0x07E8D8, i_07e8d8),
    (0x07E8DE, i_07e8de),
    (0x07E8E2, i_07e8e2),
    (0x07E8E6, i_07e8e6),
    (0x07E8EA, i_07e8ea),
    (0x07E8F0, i_07e8f0),
    (0x07E8F2, i_07e8f2),
    (0x07E8F6, i_07e8f6),
    (0x07E8FC, i_07e8fc),
    (0x07E900, i_07e900),
    (0x07E906, i_07e906),
    (0x07E90C, i_07e90c),
    (0x07E910, i_07e910),
    (0x07E914, i_07e914),
    (0x07E918, i_07e918),
    (0x07E91E, i_07e91e),
    (0x07E924, i_07e924),
    (0x07E928, i_07e928),
    (0x07E92A, i_07e92a),
    (0x07E92E, i_07e92e),
    (0x07E934, i_07e934),
    (0x07E93A, i_07e93a),
    (0x07E93E, i_07e93e),
    (0x07E942, i_07e942),
    (0x07E946, i_07e946),
    (0x07E94A, i_07e94a),
    (0x07E94E, i_07e94e),
    (0x07E954, i_07e954),
    (0x07E956, i_07e956),
    (0x07E95C, i_07e95c),
    (0x07E962, i_07e962),
    (0x07E966, i_07e966),
    (0x07E96C, i_07e96c),
    (0x07E970, i_07e970),
    (0x07E974, i_07e974),
    (0x07E976, i_07e976),
    (0x07E97A, i_07e97a),
    (0x07E97E, i_07e97e),
    (0x07E982, i_07e982),
    (0x07E984, i_07e984),
    (0x07E986, i_07e986),
    (0x07E98C, i_07e98c),
    (0x07E992, i_07e992),
    (0x07E996, i_07e996),
    (0x07E998, i_07e998),
    (0x07E99C, i_07e99c),
    (0x07E9A2, i_07e9a2),
    (0x07E9A8, i_07e9a8),
    (0x07E9AC, i_07e9ac),
    (0x07E9B0, i_07e9b0),
    (0x07E9B6, i_07e9b6),
    (0x07E9BA, i_07e9ba),
    (0x07E9C0, i_07e9c0),
    (0x07E9C4, i_07e9c4),
    (0x07E9C8, i_07e9c8),
    (0x07E9CA, i_07e9ca),
    (0x07E9CE, i_07e9ce),
    (0x07E9D2, i_07e9d2),
    (0x07E9D6, i_07e9d6),
    (0x07E9D8, i_07e9d8),
    (0x07E9DA, i_07e9da),
    (0x07E9E0, i_07e9e0),
    (0x07E9E6, i_07e9e6),
    (0x07E9EA, i_07e9ea),
    (0x07E9EC, i_07e9ec),
    (0x07E9F0, i_07e9f0),
    (0x07E9F6, i_07e9f6),
    (0x07E9FC, i_07e9fc),
    (0x07EA00, i_07ea00),
    (0x07EA04, i_07ea04),
    (0x07EA0A, i_07ea0a),
    (0x07EA0E, i_07ea0e),
    (0x07EA14, i_07ea14),
    (0x07EA18, i_07ea18),
    (0x07EA1C, i_07ea1c),
    (0x07EA1E, i_07ea1e),
    (0x07EA22, i_07ea22),
    (0x07EA26, i_07ea26),
    (0x07EA2A, i_07ea2a),
    (0x07EA2C, i_07ea2c),
    (0x07EA2E, i_07ea2e),
    (0x07EA34, i_07ea34),
    (0x07EA3A, i_07ea3a),
    (0x07EA3E, i_07ea3e),
    (0x07EA40, i_07ea40),
    (0x07EA44, i_07ea44),
    (0x07EA4A, i_07ea4a),
    (0x07EA50, i_07ea50),
    (0x07EA54, i_07ea54),
    (0x07EA58, i_07ea58),
    (0x07EA5C, i_07ea5c),
    (0x07EA60, i_07ea60),
    (0x07EA66, i_07ea66),
    (0x07EA6A, i_07ea6a),
    (0x07EA6E, i_07ea6e),
    (0x07EA70, i_07ea70),
    (0x07EA74, i_07ea74),
    (0x07EA78, i_07ea78),
    (0x07EA7C, i_07ea7c),
    (0x07EA7E, i_07ea7e),
    (0x07EA80, i_07ea80),
    (0x07EA84, i_07ea84),
    (0x07EA8A, i_07ea8a),
    (0x07EA8E, i_07ea8e),
    (0x07EA94, i_07ea94),
    (0x07EA98, i_07ea98),
    (0x07EA9E, i_07ea9e),
    (0x07EAA4, i_07eaa4),
    (0x07EAA8, i_07eaa8),
    (0x07EAAA, i_07eaaa),
    (0x07EAB0, i_07eab0),
    (0x07EAB6, i_07eab6),
    (0x07EABA, i_07eaba),
    (0x07EABE, i_07eabe),
    (0x07EAC0, i_07eac0),
    (0x07EAC2, i_07eac2),
    (0x07EAC6, i_07eac6),
    (0x07EACC, i_07eacc),
    (0x07EAD0, i_07ead0),
    (0x07EAD6, i_07ead6),
    (0x07EADC, i_07eadc),
    (0x07EAE0, i_07eae0),
    (0x07EAE4, i_07eae4),
    (0x07EAEA, i_07eaea),
    (0x07EAF0, i_07eaf0),
    (0x07EAF6, i_07eaf6),
    (0x07EAFA, i_07eafa),
    (0x07EAFC, i_07eafc),
    (0x07EB02, i_07eb02),
    (0x07EB08, i_07eb08),
    (0x07EB0C, i_07eb0c),
    (0x07EB10, i_07eb10),
    (0x07EB12, i_07eb12),
    (0x07EB18, i_07eb18),
    (0x07EB1E, i_07eb1e),
    (0x07EB22, i_07eb22),
    (0x07EB26, i_07eb26),
    (0x07EB28, i_07eb28),
    (0x07EB2A, i_07eb2a),
    (0x07EB30, i_07eb30),
    (0x07EB36, i_07eb36),
    (0x07EB3C, i_07eb3c),
    (0x07EB42, i_07eb42),
    (0x07EB46, i_07eb46),
    (0x07EB48, i_07eb48),
    (0x07EB4E, i_07eb4e),
    (0x07EB54, i_07eb54),
    (0x07EB58, i_07eb58),
    (0x07EB5C, i_07eb5c),
    (0x07EB5E, i_07eb5e),
    (0x07EB64, i_07eb64),
    (0x07EB6A, i_07eb6a),
    (0x07EB6E, i_07eb6e),
    (0x07EB72, i_07eb72),
    (0x07EB74, i_07eb74),
    (0x07EB76, i_07eb76),
    (0x07EB7A, i_07eb7a),
    (0x07EBC4, i_07ebc4),
    (0x07EBC6, i_07ebc6),
    (0x07EBCA, i_07ebca),
    (0x07EBCE, i_07ebce),
    (0x07EBD2, i_07ebd2),
    (0x07EBD6, i_07ebd6),
    (0x07EBDA, i_07ebda),
    (0x07EBDE, i_07ebde),
    (0x07EBE2, i_07ebe2),
    (0x07EBE4, i_07ebe4),
    (0x07EBE6, i_07ebe6),
    (0x07EBEA, i_07ebea),
    (0x07EBF0, i_07ebf0),
    (0x07EBF4, i_07ebf4),
    (0x07EBF8, i_07ebf8),
    (0x07EBFC, i_07ebfc),
    (0x07EC00, i_07ec00),
    (0x07EC04, i_07ec04),
    (0x07EC0A, i_07ec0a),
    (0x07EC0E, i_07ec0e),
    (0x07EC14, i_07ec14),
    (0x07EC1A, i_07ec1a),
    (0x07EC20, i_07ec20),
    (0x07EC26, i_07ec26),
    (0x07EC2C, i_07ec2c),
    (0x07EC30, i_07ec30),
    (0x07EC34, i_07ec34),
    (0x07EC38, i_07ec38),
    (0x07EC3C, i_07ec3c),
    (0x07EC40, i_07ec40),
    (0x07EC44, i_07ec44),
    (0x07EC48, i_07ec48),
    (0x07EC4E, i_07ec4e),
    (0x07EC54, i_07ec54),
    (0x07EC5A, i_07ec5a),
    (0x07EC60, i_07ec60),
    (0x07EC66, i_07ec66),
    (0x07EC6A, i_07ec6a),
    (0x07EC6E, i_07ec6e),
    (0x07EC72, i_07ec72),
    (0x07EC76, i_07ec76),
    (0x07EC7A, i_07ec7a),
    (0x07EC7E, i_07ec7e),
    (0x07EC82, i_07ec82),
    (0x07EC88, i_07ec88),
    (0x07EC8E, i_07ec8e),
    (0x07EC94, i_07ec94),
    (0x07EC9A, i_07ec9a),
    (0x07EC9E, i_07ec9e),
    (0x07ECA4, i_07eca4),
    (0x07ECAA, i_07ecaa),
    (0x07ECB0, i_07ecb0),
    (0x07ECB4, i_07ecb4),
    (0x07ECBA, i_07ecba),
    (0x07ECBE, i_07ecbe),
    (0x07ECC0, i_07ecc0),
    (0x07ECC4, i_07ecc4),
    (0x07ECC8, i_07ecc8),
    (0x07ECCC, i_07eccc),
    (0x07ECCE, i_07ecce),
    (0x07ECD0, i_07ecd0),
    (0x07ECD4, i_07ecd4),
    (0x07ECDA, i_07ecda),
    (0x07ECE0, i_07ece0),
    (0x07ECE6, i_07ece6),
    (0x07ECEA, i_07ecea),
    (0x07ECEE, i_07ecee),
    (0x07ECF0, i_07ecf0),
    (0x07ECF4, i_07ecf4),
    (0x07ECF8, i_07ecf8),
    (0x07ECFC, i_07ecfc),
    (0x07ECFE, i_07ecfe),
    (0x07ED00, i_07ed00),
    (0x07ED06, i_07ed06),
    (0x07ED0A, i_07ed0a),
    (0x07ED10, i_07ed10),
    (0x07ED16, i_07ed16),
    (0x07ED1A, i_07ed1a),
    (0x07ED1C, i_07ed1c),
    (0x07ED20, i_07ed20),
    (0x07ED24, i_07ed24),
    (0x07ED28, i_07ed28),
    (0x07ED2A, i_07ed2a),
    (0x07ED30, i_07ed30),
    (0x07ED36, i_07ed36),
    (0x07ED3A, i_07ed3a),
    (0x07ED3C, i_07ed3c),
    (0x07ED40, i_07ed40),
    (0x07ED44, i_07ed44),
    (0x07ED48, i_07ed48),
    (0x07ED4A, i_07ed4a),
    (0x07ED4E, i_07ed4e),
    (0x07ED54, i_07ed54),
    (0x07ED58, i_07ed58),
    (0x07ED5E, i_07ed5e),
    (0x07ED64, i_07ed64),
    (0x07ED68, i_07ed68),
    (0x07ED6A, i_07ed6a),
    (0x07ED70, i_07ed70),
    (0x07ED76, i_07ed76),
    (0x07ED7A, i_07ed7a),
    (0x07ED7C, i_07ed7c),
    (0x07ED80, i_07ed80),
    (0x07ED84, i_07ed84),
    (0x07ED88, i_07ed88),
    (0x07ED8E, i_07ed8e),
    (0x07ED92, i_07ed92),
    (0x07ED98, i_07ed98),
    (0x07ED9E, i_07ed9e),
    (0x07EDA2, i_07eda2),
    (0x07EDA6, i_07eda6),
    (0x07EDAA, i_07edaa),
    (0x07EDB0, i_07edb0),
    (0x07EDB6, i_07edb6),
    (0x07EDBC, i_07edbc),
    (0x07EDC2, i_07edc2),
    (0x07EDC6, i_07edc6),
    (0x07EDC8, i_07edc8),
    (0x07EDCC, i_07edcc),
    (0x07EDD2, i_07edd2),
    (0x07EDD6, i_07edd6),
    (0x07EDDA, i_07edda),
    (0x07EDDC, i_07eddc),
    (0x07EDE0, i_07ede0),
    (0x07EDE4, i_07ede4),
    (0x07EDE8, i_07ede8),
    (0x07EDEC, i_07edec),
    (0x07EDF0, i_07edf0),
    (0x07EDF2, i_07edf2),
    (0x07EDF4, i_07edf4),
    (0x07EDF8, i_07edf8),
    (0x07EDFE, i_07edfe),
    (0x07EE04, i_07ee04),
    (0x07EE0A, i_07ee0a),
    (0x07EE0E, i_07ee0e),
    (0x07EE10, i_07ee10),
    (0x07EE14, i_07ee14),
    (0x07EE1A, i_07ee1a),
    (0x07EE1E, i_07ee1e),
    (0x07EE22, i_07ee22),
    (0x07EE24, i_07ee24),
    (0x07EE28, i_07ee28),
    (0x07EE2C, i_07ee2c),
    (0x07EE30, i_07ee30),
    (0x07EE34, i_07ee34),
    (0x07EE38, i_07ee38),
    (0x07EE3A, i_07ee3a),
    (0x07EE3C, i_07ee3c),
    (0x07EE42, i_07ee42),
    (0x07EE48, i_07ee48),
    (0x07EE4C, i_07ee4c),
    (0x07EE4E, i_07ee4e),
    (0x07EE52, i_07ee52),
    (0x07EE58, i_07ee58),
    (0x07EE5C, i_07ee5c),
    (0x07EE60, i_07ee60),
    (0x07EE62, i_07ee62),
    (0x07EE66, i_07ee66),
    (0x07EE6A, i_07ee6a),
    (0x07EE6E, i_07ee6e),
    (0x07EE72, i_07ee72),
    (0x07EE76, i_07ee76),
    (0x07EE78, i_07ee78),
    (0x07EE7A, i_07ee7a),
    (0x07EE80, i_07ee80),
    (0x07EE8C, i_07ee8c),
    (0x07EE90, i_07ee90),
    (0x07EE96, i_07ee96),
    (0x07EE9C, i_07ee9c),
    (0x07EEA2, i_07eea2),
    (0x07EEA8, i_07eea8),
    (0x07EEAC, i_07eeac),
    (0x07EEB0, i_07eeb0),
    (0x07EEB4, i_07eeb4),
    (0x07EEBA, i_07eeba),
    (0x07EEC0, i_07eec0),
    (0x07EEC4, i_07eec4),
    (0x07EEC6, i_07eec6),
    (0x07EECA, i_07eeca),
    (0x07EED0, i_07eed0),
    (0x07EED4, i_07eed4),
    (0x07EED6, i_07eed6),
    (0x07EEDA, i_07eeda),
    (0x07EEDE, i_07eede),
    (0x07EEE4, i_07eee4),
    (0x07EEE8, i_07eee8),
    (0x07EEEA, i_07eeea),
    (0x07EEEE, i_07eeee),
    (0x07EEF2, i_07eef2),
    (0x07EEF6, i_07eef6),
    (0x07EEFA, i_07eefa),
    (0x07EEFE, i_07eefe),
    (0x07EF02, i_07ef02),
    (0x07EF06, i_07ef06),
    (0x07EF0A, i_07ef0a),
    (0x07EF0E, i_07ef0e),
    (0x07EF10, i_07ef10),
    (0x07EF12, i_07ef12),
    (0x07EF18, i_07ef18),
    (0x07EF1C, i_07ef1c),
    (0x07EF20, i_07ef20),
    (0x07EF24, i_07ef24),
    (0x07EF28, i_07ef28),
    (0x07EF2C, i_07ef2c),
    (0x07EF30, i_07ef30),
    (0x07EF36, i_07ef36),
    (0x07EF3C, i_07ef3c),
    (0x07EF42, i_07ef42),
    (0x07EF48, i_07ef48),
    (0x07EF4A, i_07ef4a),
    (0x07EF50, i_07ef50),
    (0x07EF56, i_07ef56),
    (0x07EF5A, i_07ef5a),
    (0x07EF5C, i_07ef5c),
    (0x07EF60, i_07ef60),
    (0x07EF66, i_07ef66),
    (0x07EF6A, i_07ef6a),
    (0x07EF6E, i_07ef6e),
    (0x07EF70, i_07ef70),
    (0x07EF74, i_07ef74),
    (0x07EF78, i_07ef78),
    (0x07EF7C, i_07ef7c),
    (0x07EF80, i_07ef80),
    (0x07EF84, i_07ef84),
    (0x07EF86, i_07ef86),
    (0x07EF88, i_07ef88),
    (0x07EF8E, i_07ef8e),
    (0x07EF94, i_07ef94),
    (0x07EF98, i_07ef98),
    (0x07EF9A, i_07ef9a),
    (0x07EF9E, i_07ef9e),
    (0x07EFA4, i_07efa4),
    (0x07EFA8, i_07efa8),
    (0x07EFAC, i_07efac),
    (0x07EFB0, i_07efb0),
    (0x07EFB4, i_07efb4),
    (0x07EFB8, i_07efb8),
    (0x07EFBC, i_07efbc),
    (0x07EFC0, i_07efc0),
    (0x07EFC4, i_07efc4),
    (0x07EFC8, i_07efc8),
    (0x07EFCA, i_07efca),
    (0x07EFCC, i_07efcc),
    (0x07EFD2, i_07efd2),
    (0x07EFD6, i_07efd6),
    (0x07EFDA, i_07efda),
    (0x07EFDE, i_07efde),
    (0x07EFE2, i_07efe2),
    (0x07EFE6, i_07efe6),
    (0x07EFEA, i_07efea),
    (0x07EFF0, i_07eff0),
    (0x07EFF4, i_07eff4),
    (0x07EFF8, i_07eff8),
    (0x07EFFC, i_07effc),
    (0x07F000, i_07f000),
    (0x07F004, i_07f004),
    (0x07F008, i_07f008),
    (0x07F00E, i_07f00e),
    (0x07F012, i_07f012),
    (0x07F016, i_07f016),
    (0x07F018, i_07f018),
    (0x07F01A, i_07f01a),
    (0x07F01E, i_07f01e),
    (0x07F020, i_07f020),
    (0x07F022, i_07f022),
    (0x07F028, i_07f028),
    (0x07F02E, i_07f02e),
    (0x07F032, i_07f032),
    (0x07F034, i_07f034),
    (0x07F038, i_07f038),
    (0x07F03E, i_07f03e),
    (0x07F042, i_07f042),
    (0x07F046, i_07f046),
    (0x07F048, i_07f048),
    (0x07F04C, i_07f04c),
    (0x07F050, i_07f050),
    (0x07F054, i_07f054),
    (0x07F058, i_07f058),
    (0x07F05C, i_07f05c),
    (0x07F05E, i_07f05e),
    (0x07F060, i_07f060),
    (0x07F066, i_07f066),
    (0x07F06C, i_07f06c),
    (0x07F072, i_07f072),
    (0x07F078, i_07f078),
    (0x07F07C, i_07f07c),
    (0x07F080, i_07f080),
    (0x07F086, i_07f086),
    (0x07F08C, i_07f08c),
    (0x07F092, i_07f092),
    (0x07F096, i_07f096),
    (0x07F098, i_07f098),
    (0x07F09C, i_07f09c),
    (0x07F0A2, i_07f0a2),
    (0x07F0A6, i_07f0a6),
    (0x07F0AA, i_07f0aa),
    (0x07F0AE, i_07f0ae),
    (0x07F0B2, i_07f0b2),
    (0x07F0B6, i_07f0b6),
    (0x07F0BA, i_07f0ba),
    (0x07F0BC, i_07f0bc),
    (0x07F0C0, i_07f0c0),
    (0x07F0C4, i_07f0c4),
    (0x07F0C8, i_07f0c8),
    (0x07F0CC, i_07f0cc),
    (0x07F0D0, i_07f0d0),
    (0x07F0D4, i_07f0d4),
    (0x07F0D8, i_07f0d8),
    (0x07F0DE, i_07f0de),
    (0x07F0E2, i_07f0e2),
    (0x07F0E4, i_07f0e4),
    (0x07F0E8, i_07f0e8),
    (0x07F0EC, i_07f0ec),
    (0x07F0F0, i_07f0f0),
    (0x07F0F4, i_07f0f4),
    (0x07F0F8, i_07f0f8),
    (0x07F0FA, i_07f0fa),
    (0x07F0FC, i_07f0fc),
    (0x07F102, i_07f102),
    (0x07F108, i_07f108),
    (0x07F10C, i_07f10c),
    (0x07F10E, i_07f10e),
    (0x07F112, i_07f112),
    (0x07F118, i_07f118),
    (0x07F11C, i_07f11c),
    (0x07F120, i_07f120),
    (0x07F122, i_07f122),
    (0x07F126, i_07f126),
    (0x07F12A, i_07f12a),
    (0x07F12E, i_07f12e),
    (0x07F130, i_07f130),
    (0x07F134, i_07f134),
    (0x07F138, i_07f138),
    (0x07F13C, i_07f13c),
    (0x07F140, i_07f140),
    (0x07F144, i_07f144),
    (0x07F146, i_07f146),
    (0x07F148, i_07f148),
    (0x07F14E, i_07f14e),
    (0x07F154, i_07f154),
    (0x07F158, i_07f158),
    (0x07F15A, i_07f15a),
    (0x07F15E, i_07f15e),
    (0x07F164, i_07f164),
    (0x07F168, i_07f168),
    (0x07F16C, i_07f16c),
    (0x07F16E, i_07f16e),
    (0x07F172, i_07f172),
    (0x07F176, i_07f176),
    (0x07F17A, i_07f17a),
    (0x07F17E, i_07f17e),
    (0x07F182, i_07f182),
    (0x07F184, i_07f184),
    (0x07F186, i_07f186),
    (0x07F18C, i_07f18c),
    (0x07F190, i_07f190),
    (0x07F194, i_07f194),
    (0x07F19A, i_07f19a),
    (0x07F1A0, i_07f1a0),
    (0x07F1A4, i_07f1a4),
    (0x07F1A6, i_07f1a6),
    (0x07F1AA, i_07f1aa),
    (0x07F1B0, i_07f1b0),
    (0x07F1B4, i_07f1b4),
    (0x07F1B8, i_07f1b8),
    (0x07F1BC, i_07f1bc),
    (0x07F1BE, i_07f1be),
    (0x07F1C2, i_07f1c2),
    (0x07F1C6, i_07f1c6),
    (0x07F1CC, i_07f1cc),
    (0x07F1D0, i_07f1d0),
    (0x07F1D6, i_07f1d6),
    (0x07F1DA, i_07f1da),
    (0x07F1DE, i_07f1de),
    (0x07F1E2, i_07f1e2),
    (0x07F1E6, i_07f1e6),
    (0x07F1EA, i_07f1ea),
    (0x07F1EE, i_07f1ee),
    (0x07F1F4, i_07f1f4),
    (0x07F1F8, i_07f1f8),
    (0x07F1FA, i_07f1fa),
    (0x07F1FE, i_07f1fe),
    (0x07F202, i_07f202),
    (0x07F208, i_07f208),
    (0x07F20C, i_07f20c),
    (0x07F210, i_07f210),
    (0x07F212, i_07f212),
    (0x07F216, i_07f216),
    (0x07F21A, i_07f21a),
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
];
