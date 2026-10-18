// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_4) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $087FD0: jsr $5dd22.l
pub fn i_087fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FD6);
    cpu.pc = 0x5DD22;
    16
}
/// $087FD6: lea.l $88028(pc), a1
pub fn i_087fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88028;
    16
}
/// $087FDA: jsr $4ae.l
pub fn i_087fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FE0);
    cpu.pc = 0x4AE;
    16
}
/// $087FE0: jsr $5dd22.l
pub fn i_087fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FE6);
    cpu.pc = 0x5DD22;
    16
}
/// $087FE6: lea.l $88042(pc), a1
pub fn i_087fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88042;
    16
}
/// $087FEA: jsr $4ae.l
pub fn i_087fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FF0);
    cpu.pc = 0x4AE;
    16
}
/// $087FF0: jsr $5dd22.l
pub fn i_087ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x87FF6);
    cpu.pc = 0x5DD22;
    16
}
/// $087FF6: lea.l $8805c(pc), a1
pub fn i_087ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x87FFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8805C;
    16
}
/// $087FFA: jsr $4ae.l
pub fn i_087ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88000;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88000);
    cpu.pc = 0x4AE;
    16
}
/// $088000: jsr $5dd22.l
pub fn i_088000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88006;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88006);
    cpu.pc = 0x5DD22;
    16
}
/// $088006: jmp $518.l
pub fn i_088006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8800C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08800c  ($08800C, 2 B, enemies/bosses/fort_scene4_0865be.s)
/// $08800C: rts
pub fn i_08800c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8800E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Quad_A_08800e  ($08800E, 26 B, enemies/bosses/fort_scene4_0865be.s)
/// $08800E: lea.l $2ec30e.l, a0
pub fn i_08800e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88014;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC30E;
    20
}
/// $088014: jsr $28cd4.l
pub fn i_088014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8801A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8801A);
    cpu.pc = 0x28CD4;
    16
}
/// $08801A: lea.l $2ed19a.l, a1
pub fn i_08801a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88020;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED19A;
    20
}
/// $088020: move.l a1, $88(a6)
pub fn i_088020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88024;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088024: bra.w $88076
pub fn i_088024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88028;
    cpu.ir = 0x6000;
    cpu.pc = 0x88076;
    10
}
// ===== Prop_Quad_B_088028  ($088028, 26 B, enemies/bosses/fort_scene4_0865be.s)
/// $088028: lea.l $2ec324.l, a0
pub fn i_088028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8802E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC324;
    20
}
/// $08802E: jsr $28cd4.l
pub fn i_08802e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88034;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88034);
    cpu.pc = 0x28CD4;
    16
}
/// $088034: lea.l $2ed1ac.l, a1
pub fn i_088034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8803A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED1AC;
    20
}
/// $08803A: move.l a1, $88(a6)
pub fn i_08803a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8803E;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08803E: bra.w $88076
pub fn i_08803e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88042;
    cpu.ir = 0x6000;
    cpu.pc = 0x88076;
    10
}
// ===== Prop_Quad_C_088042  ($088042, 26 B, enemies/bosses/fort_scene4_0865be.s)
/// $088042: lea.l $2ec33a.l, a0
pub fn i_088042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88048;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC33A;
    20
}
/// $088048: jsr $28cd4.l
pub fn i_088048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8804E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8804E);
    cpu.pc = 0x28CD4;
    16
}
/// $08804E: lea.l $2ed1be.l, a1
pub fn i_08804e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88054;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED1BE;
    20
}
/// $088054: move.l a1, $88(a6)
pub fn i_088054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88058;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088058: bra.w $88076
pub fn i_088058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8805C;
    cpu.ir = 0x6000;
    cpu.pc = 0x88076;
    10
}
// ===== Prop_Quad_D_08805c  ($08805C, 184 B, enemies/bosses/fort_scene4_0865be.s)
/// $08805C: lea.l $2ec350.l, a0
pub fn i_08805c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88062;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC350;
    20
}
/// $088062: jsr $28cd4.l
pub fn i_088062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88068;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88068);
    cpu.pc = 0x28CD4;
    16
}
/// $088068: lea.l $2ed1d0.l, a1
pub fn i_088068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8806E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED1D0;
    20
}
/// $08806E: move.l a1, $88(a6)
pub fn i_08806e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88072;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088072: bra.w $88076
pub fn i_088072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88076;
    cpu.ir = 0x6000;
    cpu.pc = 0x88076;
    10
}
/// $088076: move.w #$e4, d1
pub fn i_088076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8807A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08807A: jsr $236e.l
pub fn i_08807a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88080;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88080);
    cpu.pc = 0x236E;
    16
}
/// $088080: move.b #$ff, $32(a6)
pub fn i_088080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88086;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $088086: move.b #$ff, $33(a6)
pub fn i_088086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8808C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08808C: move.w #$40, $70(a6)
pub fn i_08808c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88092;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088092: move.w #$a, $66(a6)
pub fn i_088092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88098;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088098: move.w #$8000, $38(a6)
pub fn i_088098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8809E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08809E: jsr $267e2.l
pub fn i_08809e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880A4);
    cpu.pc = 0x267E2;
    16
}
/// $0880A4: jsr $27cee.l
pub fn i_0880a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880AA);
    cpu.pc = 0x27CEE;
    16
}
/// $0880AA: lea.l $880b0(pc), a1
pub fn i_0880aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x880B0;
    16
}
/// $0880AE: move.l a1, (a6)
pub fn i_0880ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0880B0: jsr $2783a.l
pub fn i_0880b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880B6);
    cpu.pc = 0x2783A;
    16
}
/// $0880B6: jsr $28d70.l
pub fn i_0880b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880BC);
    cpu.pc = 0x28D70;
    16
}
/// $0880BC: jsr $2870a.l
pub fn i_0880bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880C2);
    cpu.pc = 0x2870A;
    16
}
/// $0880C2: bcc.w $880d8
pub fn i_0880c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x880D8; return 10; }
    8
}
/// $0880C6: lea.l $5e766.l, a0
pub fn i_0880c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0880CC: jsr $5e770.l
pub fn i_0880cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880D2);
    cpu.pc = 0x5E770;
    16
}
/// $0880D2: bclr.b #$3, $13(a6)
pub fn i_0880d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880D8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0880D8: jsr $28758.l
pub fn i_0880d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880DE);
    cpu.pc = 0x28758;
    16
}
/// $0880DE: bcc.w $88102
pub fn i_0880de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88102; return 10; }
    8
}
/// $0880E2: move.w #$102e, d0
pub fn i_0880e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880E6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0880E6: jsr $2352.l
pub fn i_0880e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880EC);
    cpu.pc = 0x2352;
    16
}
/// $0880EC: lea.l $ffff.w, a0
pub fn i_0880ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880F0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0880F0: move.l a0, $48(a6)
pub fn i_0880f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880F4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0880F4: movea.l $88(a6), a1
pub fn i_0880f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880F8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0880F8: jsr $77c7e.l
pub fn i_0880f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x880FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x880FE);
    cpu.pc = 0x77C7E;
    16
}
/// $0880FE: bra.w $8810c
pub fn i_0880fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88102;
    cpu.ir = 0x6000;
    cpu.pc = 0x8810C;
    10
}
/// $088102: jsr $4fa70.l
pub fn i_088102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88108;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88108);
    cpu.pc = 0x4FA70;
    16
}
/// $088108: bcc.w $88112
pub fn i_088108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8810C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88112; return 10; }
    8
}
/// $08810C: jmp $518.l
pub fn i_08810c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88112;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $088112: rts
pub fn i_088112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88114;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Init_088114  ($088114, 148 B, enemies/bosses/fort_scene4_0865be.s)
/// $088114: movea.l $3c(a6), a1
pub fn i_088114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88118;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $088118: jsr $2942a.l
pub fn i_088118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8811E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8811E);
    cpu.pc = 0x2942A;
    16
}
/// $08811E: move.w #$e6, d1
pub fn i_08811e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88122;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $088122: jsr $236e.l
pub fn i_088122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88128;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88128);
    cpu.pc = 0x236E;
    16
}
/// $088128: move.b #$ff, $32(a6)
pub fn i_088128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8812E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08812E: move.b #$ff, $33(a6)
pub fn i_08812e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88134;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $088134: move.w #$40, $70(a6)
pub fn i_088134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8813A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08813A: move.w #$32, $66(a6)
pub fn i_08813a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88140;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088140: move.w #$8000, $38(a6)
pub fn i_088140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88146;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088146: jsr $267e2.l
pub fn i_088146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8814C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8814C);
    cpu.pc = 0x267E2;
    16
}
/// $08814C: jsr $27cee.l
pub fn i_08814c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88152;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88152);
    cpu.pc = 0x27CEE;
    16
}
/// $088152: lea.l $88158(pc), a1
pub fn i_088152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88156;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88158;
    16
}
/// $088156: move.l a1, (a6)
pub fn i_088156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88158;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $088158: jsr $2783a.l
pub fn i_088158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8815E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8815E);
    cpu.pc = 0x2783A;
    16
}
/// $08815E: jsr $2870a.l
pub fn i_08815e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88164;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88164);
    cpu.pc = 0x2870A;
    16
}
/// $088164: bcc.w $8817a
pub fn i_088164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88168;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8817A; return 10; }
    8
}
/// $088168: lea.l $5e766.l, a0
pub fn i_088168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8816E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08816E: jsr $5e770.l
pub fn i_08816e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88174);
    cpu.pc = 0x5E770;
    16
}
/// $088174: bclr.b #$3, $13(a6)
pub fn i_088174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8817A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08817A: move.w #$32, $66(a6)
pub fn i_08817a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88180;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088180: bclr.b #$0, $13(a6)
pub fn i_088180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88186;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $088186: cmpi.w #$50, $22(a6)
pub fn i_088186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8818C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x50, Size::Word, false);
    16
}
/// $08818C: bgt.w $88196
pub fn i_08818c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88190;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x88196; return 10; }
    8
}
/// $088190: lea.l $881a8(pc), a1
pub fn i_088190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88194;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x881A8;
    16
}
/// $088194: move.l a1, (a6)
pub fn i_088194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88196;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $088196: jsr $4fa70.l
pub fn i_088196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8819C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8819C);
    cpu.pc = 0x4FA70;
    16
}
/// $08819C: bcc.w $881a6
pub fn i_08819c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x881A6; return 10; }
    8
}
/// $0881A0: jmp $518.l
pub fn i_0881a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881A6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0881A6: rts
pub fn i_0881a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Arm_0881a8  ($0881A8, 98 B, enemies/bosses/fort_scene4_0865be.s)
/// $0881A8: lea.l $2ec366.l, a0
pub fn i_0881a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC366;
    20
}
/// $0881AE: jsr $28cd4.l
pub fn i_0881ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x881B4);
    cpu.pc = 0x28CD4;
    16
}
/// $0881B4: lea.l $881ba(pc), a1
pub fn i_0881b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x881BA;
    16
}
/// $0881B8: move.l a1, (a6)
pub fn i_0881b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0881BA: jsr $2783a.l
pub fn i_0881ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x881C0);
    cpu.pc = 0x2783A;
    16
}
/// $0881C0: jsr $28d70.l
pub fn i_0881c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x881C6);
    cpu.pc = 0x28D70;
    16
}
/// $0881C6: bcc.w $881d0
pub fn i_0881c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x881D0; return 10; }
    8
}
/// $0881CA: lea.l $8820a(pc), a1
pub fn i_0881ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8820A;
    16
}
/// $0881CE: move.l a1, (a6)
pub fn i_0881ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0881D0: jsr $2870a.l
pub fn i_0881d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x881D6);
    cpu.pc = 0x2870A;
    16
}
/// $0881D6: bcc.w $881ec
pub fn i_0881d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x881EC; return 10; }
    8
}
/// $0881DA: lea.l $5e766.l, a0
pub fn i_0881da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0881E0: jsr $5e770.l
pub fn i_0881e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x881E6);
    cpu.pc = 0x5E770;
    16
}
/// $0881E6: bclr.b #$3, $13(a6)
pub fn i_0881e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881EC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0881EC: move.w #$32, $66(a6)
pub fn i_0881ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0881F2: bclr.b #$0, $13(a6)
pub fn i_0881f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881F8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0881F8: jsr $4fa70.l
pub fn i_0881f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x881FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x881FE);
    cpu.pc = 0x4FA70;
    16
}
/// $0881FE: bcc.w $88208
pub fn i_0881fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88202;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88208; return 10; }
    8
}
/// $088202: jmp $518.l
pub fn i_088202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88208;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $088208: rts
pub fn i_088208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8820A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Stage1_08820a  ($08820A, 102 B, enemies/bosses/fort_scene4_0865be.s)
/// $08820A: lea.l $ee062.l, a1
pub fn i_08820a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88210;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE062;
    20
}
/// $088210: move.w #$82, d0
pub fn i_088210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88214;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $088214: move.b #$0, $82(a6)
pub fn i_088214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8821A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08821A: jsr $4429e.l
pub fn i_08821a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88220;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88220);
    cpu.pc = 0x4429E;
    16
}
/// $088220: lea.l $88226(pc), a1
pub fn i_088220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88224;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88226;
    16
}
/// $088224: move.l a1, (a6)
pub fn i_088224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88226;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $088226: jsr $2783a.l
pub fn i_088226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8822C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8822C);
    cpu.pc = 0x2783A;
    16
}
/// $08822C: jsr $28d70.l
pub fn i_08822c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88232;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88232);
    cpu.pc = 0x28D70;
    16
}
/// $088232: jsr $2870a.l
pub fn i_088232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88238;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88238);
    cpu.pc = 0x2870A;
    16
}
/// $088238: bcc.w $8824e
pub fn i_088238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8823C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8824E; return 10; }
    8
}
/// $08823C: lea.l $5e766.l, a0
pub fn i_08823c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88242;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $088242: jsr $5e770.l
pub fn i_088242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88248;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88248);
    cpu.pc = 0x5E770;
    16
}
/// $088248: bclr.b #$3, $13(a6)
pub fn i_088248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8824E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08824E: cmpi.w #$28, $66(a6)
pub fn i_08824e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88254;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x28, Size::Word, false);
    16
}
/// $088254: bgt.w $8825e
pub fn i_088254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88258;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8825E; return 10; }
    8
}
/// $088258: lea.l $88270(pc), a1
pub fn i_088258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8825C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88270;
    16
}
/// $08825C: move.l a1, (a6)
pub fn i_08825c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8825E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08825E: jsr $4fa70.l
pub fn i_08825e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88264;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88264);
    cpu.pc = 0x4FA70;
    16
}
/// $088264: bcc.w $8826e
pub fn i_088264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88268;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8826E; return 10; }
    8
}
/// $088268: jmp $518.l
pub fn i_088268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8826E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08826E: rts
pub fn i_08826e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88270;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Stage2_088270  ($088270, 92 B, enemies/bosses/fort_scene4_0865be.s)
/// $088270: lea.l $2ec3de.l, a0
pub fn i_088270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88276;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC3DE;
    20
}
/// $088276: jsr $28cd4.l
pub fn i_088276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8827C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8827C);
    cpu.pc = 0x28CD4;
    16
}
/// $08827C: lea.l $88282(pc), a1
pub fn i_08827c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88280;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88282;
    16
}
/// $088280: move.l a1, (a6)
pub fn i_088280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88282;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $088282: jsr $2783a.l
pub fn i_088282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88288;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88288);
    cpu.pc = 0x2783A;
    16
}
/// $088288: jsr $28d70.l
pub fn i_088288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8828E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8828E);
    cpu.pc = 0x28D70;
    16
}
/// $08828E: jsr $2870a.l
pub fn i_08828e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88294;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88294);
    cpu.pc = 0x2870A;
    16
}
/// $088294: bcc.w $882aa
pub fn i_088294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88298;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x882AA; return 10; }
    8
}
/// $088298: lea.l $5e766.l, a0
pub fn i_088298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8829E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08829E: jsr $5e770.l
pub fn i_08829e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x882A4);
    cpu.pc = 0x5E770;
    16
}
/// $0882A4: bclr.b #$3, $13(a6)
pub fn i_0882a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882AA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0882AA: cmpi.w #$1e, $66(a6)
pub fn i_0882aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Word, false);
    16
}
/// $0882B0: bgt.w $882ba
pub fn i_0882b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882B4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x882BA; return 10; }
    8
}
/// $0882B4: lea.l $882cc(pc), a1
pub fn i_0882b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x882CC;
    16
}
/// $0882B8: move.l a1, (a6)
pub fn i_0882b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0882BA: jsr $4fa70.l
pub fn i_0882ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x882C0);
    cpu.pc = 0x4FA70;
    16
}
/// $0882C0: bcc.w $882ca
pub fn i_0882c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x882CA; return 10; }
    8
}
/// $0882C4: jmp $518.l
pub fn i_0882c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882CA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0882CA: rts
pub fn i_0882ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Stage3_0882cc  ($0882CC, 92 B, enemies/bosses/fort_scene4_0865be.s)
/// $0882CC: lea.l $2ec3ee.l, a0
pub fn i_0882cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC3EE;
    20
}
/// $0882D2: jsr $28cd4.l
pub fn i_0882d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x882D8);
    cpu.pc = 0x28CD4;
    16
}
/// $0882D8: lea.l $882de(pc), a1
pub fn i_0882d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x882DE;
    16
}
/// $0882DC: move.l a1, (a6)
pub fn i_0882dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0882DE: jsr $2783a.l
pub fn i_0882de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x882E4);
    cpu.pc = 0x2783A;
    16
}
/// $0882E4: jsr $28d70.l
pub fn i_0882e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x882EA);
    cpu.pc = 0x28D70;
    16
}
/// $0882EA: jsr $2870a.l
pub fn i_0882ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x882F0);
    cpu.pc = 0x2870A;
    16
}
/// $0882F0: bcc.w $88306
pub fn i_0882f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88306; return 10; }
    8
}
/// $0882F4: lea.l $5e766.l, a0
pub fn i_0882f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x882FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0882FA: jsr $5e770.l
pub fn i_0882fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88300;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88300);
    cpu.pc = 0x5E770;
    16
}
/// $088300: bclr.b #$3, $13(a6)
pub fn i_088300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88306;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $088306: cmpi.w #$14, $66(a6)
pub fn i_088306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8830C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Word, false);
    16
}
/// $08830C: bgt.w $88316
pub fn i_08830c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88310;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x88316; return 10; }
    8
}
/// $088310: lea.l $88328(pc), a1
pub fn i_088310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88314;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88328;
    16
}
/// $088314: move.l a1, (a6)
pub fn i_088314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88316;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $088316: jsr $4fa70.l
pub fn i_088316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8831C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8831C);
    cpu.pc = 0x4FA70;
    16
}
/// $08831C: bcc.w $88326
pub fn i_08831c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88320;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88326; return 10; }
    8
}
/// $088320: jmp $518.l
pub fn i_088320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88326;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $088326: rts
pub fn i_088326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88328;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Stage4_088328  ($088328, 104 B, enemies/bosses/fort_scene4_0865be.s)
/// $088328: lea.l $2ec3fe.l, a0
pub fn i_088328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8832E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC3FE;
    20
}
/// $08832E: jsr $28cd4.l
pub fn i_08832e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88334;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88334);
    cpu.pc = 0x28CD4;
    16
}
/// $088334: lea.l $8833a(pc), a1
pub fn i_088334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88338;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8833A;
    16
}
/// $088338: move.l a1, (a6)
pub fn i_088338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8833A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08833A: jsr $2783a.l
pub fn i_08833a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88340);
    cpu.pc = 0x2783A;
    16
}
/// $088340: jsr $28d70.l
pub fn i_088340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88346);
    cpu.pc = 0x28D70;
    16
}
/// $088346: jsr $2870a.l
pub fn i_088346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8834C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8834C);
    cpu.pc = 0x2870A;
    16
}
/// $08834C: bcc.w $88362
pub fn i_08834c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88350;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88362; return 10; }
    8
}
/// $088350: lea.l $5e766.l, a0
pub fn i_088350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88356;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $088356: jsr $5e770.l
pub fn i_088356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8835C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8835C);
    cpu.pc = 0x5E770;
    16
}
/// $08835C: bclr.b #$3, $13(a6)
pub fn i_08835c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88362;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $088362: cmpi.w #$a, $66(a6)
pub fn i_088362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88368;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA, Size::Word, false);
    16
}
/// $088368: bgt.w $8837e
pub fn i_088368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8836C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8837E; return 10; }
    8
}
/// $08836C: lea.l $2ed1e2.l, a1
pub fn i_08836c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88372;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED1E2;
    20
}
/// $088372: jsr $77c7e.l
pub fn i_088372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88378;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88378);
    cpu.pc = 0x77C7E;
    16
}
/// $088378: lea.l $88390(pc), a1
pub fn i_088378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8837C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88390;
    16
}
/// $08837C: move.l a1, (a6)
pub fn i_08837c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8837E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08837E: jsr $4fa70.l
pub fn i_08837e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88384);
    cpu.pc = 0x4FA70;
    16
}
/// $088384: bcc.w $8838e
pub fn i_088384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88388;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8838E; return 10; }
    8
}
/// $088388: jmp $518.l
pub fn i_088388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8838E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08838E: rts
pub fn i_08838e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88390;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Barricade_Stage5_088390  ($088390, 92 B, enemies/bosses/fort_scene4_0865be.s)
/// $088390: lea.l $2ec40e.l, a0
pub fn i_088390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88396;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC40E;
    20
}
/// $088396: jsr $28cd4.l
pub fn i_088396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8839C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8839C);
    cpu.pc = 0x28CD4;
    16
}
/// $08839C: lea.l $883a2(pc), a1
pub fn i_08839c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x883A2;
    16
}
/// $0883A0: move.l a1, (a6)
pub fn i_0883a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0883A2: jsr $86552(pc)
pub fn i_0883a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x883A6);
    cpu.pc = 0x86552;
    16
}
/// $0883A6: bcs.b $8837e
pub fn i_0883a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883A8;
    cpu.ir = 0x65D6;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8837E; return 10; }
    8
}
/// $0883A8: jsr $2783a.l
pub fn i_0883a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x883AE);
    cpu.pc = 0x2783A;
    16
}
/// $0883AE: jsr $28d70.l
pub fn i_0883ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x883B4);
    cpu.pc = 0x28D70;
    16
}
/// $0883B4: jsr $2870a.l
pub fn i_0883b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x883BA);
    cpu.pc = 0x2870A;
    16
}
/// $0883BA: bcc.w $883d0
pub fn i_0883ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x883D0; return 10; }
    8
}
/// $0883BE: lea.l $5e766.l, a0
pub fn i_0883be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0883C4: jsr $5e770.l
pub fn i_0883c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x883CA);
    cpu.pc = 0x5E770;
    16
}
/// $0883CA: bclr.b #$3, $13(a6)
pub fn i_0883ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883D0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0883D0: jsr $28758.l
pub fn i_0883d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x883D6);
    cpu.pc = 0x28758;
    16
}
/// $0883D6: bra.w $883e4
pub fn i_0883d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883DA;
    cpu.ir = 0x6000;
    cpu.pc = 0x883E4;
    10
}
/// $0883E4: jmp $518.l
pub fn i_0883e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883EA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Heli_RotorAnim_0883ec  ($0883EC, 70 B, enemies/bosses/fort_scene4_0865be.s)
/// $0883EC: move.b $106f28.l, d0
pub fn i_0883ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883F2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0883F2: cmpi.w #$0, $72(a6)
pub fn i_0883f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883F8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0883F8: ble.w $8841c
pub fn i_0883f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x883FC;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8841C; return 10; }
    8
}
/// $0883FC: andi.b #$3, d0
pub fn i_0883fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88400;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $088400: bne.w $88436
pub fn i_088400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88404;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88436; return 10; }
    8
}
/// $088404: move.w $18(a6), d0
pub fn i_088404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88408;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $088408: move.w $16(a6), $18(a6)
pub fn i_088408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8840E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x18);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08840E: move.w d0, $16(a6)
pub fn i_08840e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88412;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088412: move.w d0, $14(a6)
pub fn i_088412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88416;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088416: subq.w #$1, $72(a6)
pub fn i_088416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8841A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08841A: rts
pub fn i_08841a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8841C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08841C: andi.b #$f, d0
pub fn i_08841c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88420;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $088420: bne.w $88436
pub fn i_088420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88424;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88436; return 10; }
    8
}
/// $088424: move.w $18(a6), d0
pub fn i_088424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88428;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $088428: move.w $16(a6), $18(a6)
pub fn i_088428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8842E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x18);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08842E: move.w d0, $16(a6)
pub fn i_08842e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88432;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskW_088432  ($088432, 6 B, kernel/task_setters_w.c)
/// $088432: move.w d0, $14(a6)
pub fn i_088432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88436;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088436: rts
pub fn i_088436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88438;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_HitboxPulseTable_088438  ($088438, 50 B, enemies/bosses/fort_scene4_0865be.s)
/// $088438: addq.w #$1, $72(a6)
pub fn i_088438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8843C;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08843C: move.w $72(a6), d0
pub fn i_08843c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88440;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $088440: andi.b #$3, d0
pub fn i_088440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88444;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $088444: bne.w $88460
pub fn i_088444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88448;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88460; return 10; }
    8
}
/// $088448: clr.l d0
pub fn i_088448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8844A;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08844A: move.b $21(a6), d0
pub fn i_08844a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8844E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08844E: asl.l #$2, d0
pub fn i_08844e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88450;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $088450: lea.l $2ec712.l, a0
pub fn i_088450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88456;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EC712;
    20
}
/// $088456: movea.l (a0, d0.w), a0
pub fn i_088456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8845A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08845A: move.l a0, $48(a6)
pub fn i_08845a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8845E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08845E: rts
pub fn i_08845e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88460;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $088460: lea.l $ffff.w, a0
pub fn i_088460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88464;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $088464: move.l a0, $48(a6)
pub fn i_088464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88468;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088468: rts
pub fn i_088468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8846A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_HitboxPulseSaved_08846a  ($08846A, 34 B, enemies/bosses/fort_scene4_0865be.s)
/// $08846A: addq.w #$1, $72(a6)
pub fn i_08846a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8846E;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08846E: move.w $72(a6), d0
pub fn i_08846e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88472;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $088472: andi.b #$3, d0
pub fn i_088472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88476;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $088476: bne.w $88482
pub fn i_088476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8847A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88482; return 10; }
    8
}
/// $08847A: move.l $84(a6), $48(a6)
pub fn i_08847a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88480;
    cpu.ir = 0x2D6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $088480: rts
pub fn i_088480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88482;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $088482: lea.l $ffff.w, a0
pub fn i_088482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88486;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $088486: move.l a0, $48(a6)
pub fn i_088486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8848A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08848A: rts
pub fn i_08848a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8848C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_PropagateDamageToParent_08848c  ($08848C, 26 B, enemies/bosses/fort_scene4_0865be.s)
/// $08848C: move.w $80(a6), d0
pub fn i_08848c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88490;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $088490: move.w $66(a6), d1
pub fn i_088490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88494;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $088494: sub.w d1, d0
pub fn i_088494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88496;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $088496: movea.l $c(a6), a0
pub fn i_088496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8849A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08849A: sub.w d0, $66(a0)
pub fn i_08849a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8849E;
    cpu.ir = 0x9168;
    let ad1 = cpu.a[0].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08849E: move.w $66(a6), $80(a6)
pub fn i_08849e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884A4;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0884A4: rts
pub fn i_0884a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_SpawnChildren_V0_0884a6  ($0884A6, 270 B, enemies/bosses/fort_scene4_0865be.s)
/// $0884A6: lea.l $868a6(pc), a1
pub fn i_0884a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $0884AA: jsr $4ae.l
pub fn i_0884aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x884B0);
    cpu.pc = 0x4AE;
    16
}
/// $0884B0: jsr $5dd22.l
pub fn i_0884b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x884B6);
    cpu.pc = 0x5DD22;
    16
}
/// $0884B6: addi.w #$28, $22(a0)
pub fn i_0884b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884BC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0884BC: addi.w #$50, $24(a0)
pub fn i_0884bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884C2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0884C2: move.w #$60, $38(a0)
pub fn i_0884c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884C8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0884C8: lea.l $2ed738.l, a1
pub fn i_0884c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884CE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED738;
    20
}
/// $0884CE: move.l a1, $74(a0)
pub fn i_0884ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884D2;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0884D2: move.w #$0, $72(a0)
pub fn i_0884d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884D8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0884D8: lea.l $ee066.l, a1
pub fn i_0884d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884DE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE066;
    20
}
/// $0884DE: move.l a1, $7c(a0)
pub fn i_0884de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884E2;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0884E2: lea.l $8686a(pc), a1
pub fn i_0884e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8686A;
    16
}
/// $0884E6: jsr $4ae.l
pub fn i_0884e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x884EC);
    cpu.pc = 0x4AE;
    16
}
/// $0884EC: jsr $5dd22.l
pub fn i_0884ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x884F2);
    cpu.pc = 0x5DD22;
    16
}
/// $0884F2: addi.w #$68, $22(a0)
pub fn i_0884f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884F8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x68, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0884F8: addi.w #$50, $24(a0)
pub fn i_0884f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x884FE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0884FE: move.w #$60, $38(a0)
pub fn i_0884fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88504;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088504: lea.l $2ed74c.l, a1
pub fn i_088504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8850A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED74C;
    20
}
/// $08850A: move.l a1, $74(a0)
pub fn i_08850a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8850E;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08850E: move.w #$1, $72(a0)
pub fn i_08850e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88514;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088514: lea.l $ee07c.l, a1
pub fn i_088514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8851A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE07C;
    20
}
/// $08851A: move.l a1, $7c(a0)
pub fn i_08851a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8851E;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08851E: lea.l $868a6(pc), a1
pub fn i_08851e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88522;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $088522: jsr $4ae.l
pub fn i_088522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88528;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88528);
    cpu.pc = 0x4AE;
    16
}
/// $088528: jsr $5dd22.l
pub fn i_088528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8852E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8852E);
    cpu.pc = 0x5DD22;
    16
}
/// $08852E: addi.w #$a8, $22(a0)
pub fn i_08852e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88534;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088534: addi.w #$50, $24(a0)
pub fn i_088534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8853A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08853A: move.w #$60, $38(a0)
pub fn i_08853a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88540;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088540: lea.l $2ed760.l, a1
pub fn i_088540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88546;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED760;
    20
}
/// $088546: move.l a1, $74(a0)
pub fn i_088546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8854A;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08854A: move.w #$2, $72(a0)
pub fn i_08854a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88550;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088550: lea.l $ee092.l, a1
pub fn i_088550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88556;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE092;
    20
}
/// $088556: move.l a1, $7c(a0)
pub fn i_088556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8855A;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08855A: lea.l $86cd0(pc), a1
pub fn i_08855a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8855E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86CD0;
    16
}
/// $08855E: jsr $4ae.l
pub fn i_08855e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88564);
    cpu.pc = 0x4AE;
    16
}
/// $088564: jsr $5dd22.l
pub fn i_088564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8856A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8856A);
    cpu.pc = 0x5DD22;
    16
}
/// $08856A: addi.w #$30, $22(a0)
pub fn i_08856a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88570;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088570: addi.w #$8, $24(a0)
pub fn i_088570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88576;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088576: lea.l $2ed774.l, a1
pub fn i_088576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8857C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED774;
    20
}
/// $08857C: move.l a1, $74(a0)
pub fn i_08857c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88580;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088580: move.w #$3, $72(a0)
pub fn i_088580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88586;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088586: lea.l $86cd0(pc), a1
pub fn i_088586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8858A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86CD0;
    16
}
/// $08858A: jsr $4ae.l
pub fn i_08858a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88590);
    cpu.pc = 0x4AE;
    16
}
/// $088590: jsr $5dd22.l
pub fn i_088590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88596);
    cpu.pc = 0x5DD22;
    16
}
/// $088596: addi.w #$a8, $22(a0)
pub fn i_088596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8859C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08859C: addi.w #$8, $24(a0)
pub fn i_08859c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885A2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0885A2: lea.l $2ed788.l, a1
pub fn i_0885a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885A8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED788;
    20
}
/// $0885A8: move.l a1, $74(a0)
pub fn i_0885a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885AC;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0885AC: move.w #$4, $72(a0)
pub fn i_0885ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885B2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0885B2: rts
pub fn i_0885b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_SpawnChildren_V1_0885b4  ($0885B4, 270 B, enemies/bosses/fort_scene4_0865be.s)
/// $0885B4: lea.l $868a6(pc), a1
pub fn i_0885b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $0885B8: jsr $4ae.l
pub fn i_0885b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x885BE);
    cpu.pc = 0x4AE;
    16
}
/// $0885BE: jsr $5dd22.l
pub fn i_0885be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x885C4);
    cpu.pc = 0x5DD22;
    16
}
/// $0885C4: addi.w #$28, $22(a0)
pub fn i_0885c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885CA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0885CA: addi.w #$40, $24(a0)
pub fn i_0885ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885D0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0885D0: move.w #$70, $38(a0)
pub fn i_0885d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885D6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0885D6: lea.l $2ed79c.l, a1
pub fn i_0885d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885DC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED79C;
    20
}
/// $0885DC: move.l a1, $74(a0)
pub fn i_0885dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885E0;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0885E0: move.w #$0, $72(a0)
pub fn i_0885e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885E6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0885E6: lea.l $ee0a8.l, a1
pub fn i_0885e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885EC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE0A8;
    20
}
/// $0885EC: move.l a1, $7c(a0)
pub fn i_0885ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885F0;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0885F0: lea.l $8686a(pc), a1
pub fn i_0885f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8686A;
    16
}
/// $0885F4: jsr $4ae.l
pub fn i_0885f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x885FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x885FA);
    cpu.pc = 0x4AE;
    16
}
/// $0885FA: jsr $5dd22.l
pub fn i_0885fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88600;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88600);
    cpu.pc = 0x5DD22;
    16
}
/// $088600: addi.w #$68, $22(a0)
pub fn i_088600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88606;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x68, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088606: addi.w #$40, $24(a0)
pub fn i_088606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8860C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08860C: move.w #$70, $38(a0)
pub fn i_08860c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88612;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088612: lea.l $2ed7b0.l, a1
pub fn i_088612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88618;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED7B0;
    20
}
/// $088618: move.l a1, $74(a0)
pub fn i_088618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8861C;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08861C: move.w #$1, $72(a0)
pub fn i_08861c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88622;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088622: lea.l $ee0be.l, a1
pub fn i_088622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88628;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE0BE;
    20
}
/// $088628: move.l a1, $7c(a0)
pub fn i_088628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8862C;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08862C: lea.l $868a6(pc), a1
pub fn i_08862c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88630;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $088630: jsr $4ae.l
pub fn i_088630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88636;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88636);
    cpu.pc = 0x4AE;
    16
}
/// $088636: jsr $5dd22.l
pub fn i_088636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8863C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8863C);
    cpu.pc = 0x5DD22;
    16
}
/// $08863C: addi.w #$a8, $22(a0)
pub fn i_08863c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88642;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088642: addi.w #$40, $24(a0)
pub fn i_088642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88648;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088648: move.w #$70, $38(a0)
pub fn i_088648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8864E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08864E: lea.l $2ed7c4.l, a1
pub fn i_08864e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88654;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED7C4;
    20
}
/// $088654: move.l a1, $74(a0)
pub fn i_088654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88658;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088658: move.w #$2, $72(a0)
pub fn i_088658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8865E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08865E: lea.l $ee0d4.l, a1
pub fn i_08865e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88664;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE0D4;
    20
}
/// $088664: move.l a1, $7c(a0)
pub fn i_088664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88668;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088668: lea.l $86ce4(pc), a1
pub fn i_088668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8866C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86CE4;
    16
}
/// $08866C: jsr $4ae.l
pub fn i_08866c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88672;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88672);
    cpu.pc = 0x4AE;
    16
}
/// $088672: jsr $5dd22.l
pub fn i_088672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88678;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88678);
    cpu.pc = 0x5DD22;
    16
}
/// $088678: addi.w #$28, $22(a0)
pub fn i_088678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8867E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08867E: addi.w #$0, $24(a0)
pub fn i_08867e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88684;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088684: lea.l $2ed7d8.l, a1
pub fn i_088684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8868A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED7D8;
    20
}
/// $08868A: move.l a1, $74(a0)
pub fn i_08868a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8868E;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08868E: move.w #$3, $72(a0)
pub fn i_08868e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88694;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088694: lea.l $86ce4(pc), a1
pub fn i_088694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88698;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86CE4;
    16
}
/// $088698: jsr $4ae.l
pub fn i_088698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8869E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8869E);
    cpu.pc = 0x4AE;
    16
}
/// $08869E: jsr $5dd22.l
pub fn i_08869e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x886A4);
    cpu.pc = 0x5DD22;
    16
}
/// $0886A4: addi.w #$a0, $22(a0)
pub fn i_0886a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886AA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0886AA: addi.w #$0, $24(a0)
pub fn i_0886aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886B0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0886B0: lea.l $2ed7ec.l, a1
pub fn i_0886b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886B6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED7EC;
    20
}
/// $0886B6: move.l a1, $74(a0)
pub fn i_0886b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886BA;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0886BA: move.w #$4, $72(a0)
pub fn i_0886ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0886C0: rts
pub fn i_0886c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_SpawnChildren_V2_0886c2  ($0886C2, 270 B, enemies/bosses/fort_scene4_0865be.s)
/// $0886C2: lea.l $868ba(pc), a1
pub fn i_0886c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868BA;
    16
}
/// $0886C6: jsr $4ae.l
pub fn i_0886c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x886CC);
    cpu.pc = 0x4AE;
    16
}
/// $0886CC: jsr $5dd22.l
pub fn i_0886cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x886D2);
    cpu.pc = 0x5DD22;
    16
}
/// $0886D2: addi.w #$28, $22(a0)
pub fn i_0886d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886D8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0886D8: addi.w #$50, $24(a0)
pub fn i_0886d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886DE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0886DE: move.w #$60, $38(a0)
pub fn i_0886de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886E4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0886E4: lea.l $2ed800.l, a1
pub fn i_0886e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886EA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED800;
    20
}
/// $0886EA: move.l a1, $74(a0)
pub fn i_0886ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886EE;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0886EE: move.w #$0, $72(a0)
pub fn i_0886ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886F4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0886F4: lea.l $ee0ea.l, a1
pub fn i_0886f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886FA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE0EA;
    20
}
/// $0886FA: move.l a1, $7c(a0)
pub fn i_0886fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x886FE;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0886FE: lea.l $868ba(pc), a1
pub fn i_0886fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88702;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868BA;
    16
}
/// $088702: jsr $4ae.l
pub fn i_088702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88708);
    cpu.pc = 0x4AE;
    16
}
/// $088708: jsr $5dd22.l
pub fn i_088708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8870E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8870E);
    cpu.pc = 0x5DD22;
    16
}
/// $08870E: addi.w #$68, $22(a0)
pub fn i_08870e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88714;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x68, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088714: addi.w #$50, $24(a0)
pub fn i_088714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8871A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08871A: move.w #$60, $38(a0)
pub fn i_08871a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88720;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088720: lea.l $2ed814.l, a1
pub fn i_088720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88726;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED814;
    20
}
/// $088726: move.l a1, $74(a0)
pub fn i_088726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8872A;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08872A: move.w #$1, $72(a0)
pub fn i_08872a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88730;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088730: lea.l $ee100.l, a1
pub fn i_088730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88736;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE100;
    20
}
/// $088736: move.l a1, $7c(a0)
pub fn i_088736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8873A;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08873A: lea.l $868ba(pc), a1
pub fn i_08873a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8873E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868BA;
    16
}
/// $08873E: jsr $4ae.l
pub fn i_08873e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88744;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88744);
    cpu.pc = 0x4AE;
    16
}
/// $088744: jsr $5dd22.l
pub fn i_088744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8874A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8874A);
    cpu.pc = 0x5DD22;
    16
}
/// $08874A: addi.w #$a8, $22(a0)
pub fn i_08874a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88750;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088750: addi.w #$50, $24(a0)
pub fn i_088750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88756;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088756: move.w #$60, $38(a0)
pub fn i_088756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8875C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08875C: lea.l $2ed828.l, a1
pub fn i_08875c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88762;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED828;
    20
}
/// $088762: move.l a1, $74(a0)
pub fn i_088762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88766;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088766: move.w #$2, $72(a0)
pub fn i_088766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8876C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08876C: lea.l $ee116.l, a1
pub fn i_08876c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88772;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE116;
    20
}
/// $088772: move.l a1, $7c(a0)
pub fn i_088772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88776;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088776: lea.l $86cf8(pc), a1
pub fn i_088776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8877A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86CF8;
    16
}
/// $08877A: jsr $4ae.l
pub fn i_08877a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88780;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88780);
    cpu.pc = 0x4AE;
    16
}
/// $088780: jsr $5dd22.l
pub fn i_088780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88786;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88786);
    cpu.pc = 0x5DD22;
    16
}
/// $088786: addi.w #$28, $22(a0)
pub fn i_088786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8878C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08878C: addi.w #$8, $24(a0)
pub fn i_08878c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88792;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088792: lea.l $2ed83c.l, a1
pub fn i_088792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88798;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED83C;
    20
}
/// $088798: move.l a1, $74(a0)
pub fn i_088798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8879C;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08879C: move.w #$3, $72(a0)
pub fn i_08879c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887A2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0887A2: lea.l $86cf8(pc), a1
pub fn i_0887a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86CF8;
    16
}
/// $0887A6: jsr $4ae.l
pub fn i_0887a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x887AC);
    cpu.pc = 0x4AE;
    16
}
/// $0887AC: jsr $5dd22.l
pub fn i_0887ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x887B2);
    cpu.pc = 0x5DD22;
    16
}
/// $0887B2: addi.w #$a0, $22(a0)
pub fn i_0887b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887B8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0887B8: addi.w #$8, $24(a0)
pub fn i_0887b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887BE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0887BE: lea.l $2ed850.l, a1
pub fn i_0887be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887C4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED850;
    20
}
/// $0887C4: move.l a1, $74(a0)
pub fn i_0887c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887C8;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0887C8: move.w #$4, $72(a0)
pub fn i_0887c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887CE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0887CE: rts
pub fn i_0887ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_SpawnChildren_V3_0887d0  ($0887D0, 330 B, enemies/bosses/fort_scene4_0865be.s)
/// $0887D0: lea.l $86892(pc), a1
pub fn i_0887d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86892;
    16
}
/// $0887D4: jsr $4ae.l
pub fn i_0887d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x887DA);
    cpu.pc = 0x4AE;
    16
}
/// $0887DA: jsr $5dd22.l
pub fn i_0887da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x887E0);
    cpu.pc = 0x5DD22;
    16
}
/// $0887E0: addi.w #$18, $22(a0)
pub fn i_0887e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887E6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0887E6: addi.w #$40, $24(a0)
pub fn i_0887e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887EC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0887EC: move.w #$70, $38(a0)
pub fn i_0887ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887F2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0887F2: lea.l $2ed88c.l, a1
pub fn i_0887f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887F8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED88C;
    20
}
/// $0887F8: move.l a1, $74(a0)
pub fn i_0887f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x887FC;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0887FC: move.w #$0, $72(a0)
pub fn i_0887fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88802;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088802: lea.l $ee12c.l, a1
pub fn i_088802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88808;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE12C;
    20
}
/// $088808: move.l a1, $7c(a0)
pub fn i_088808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8880C;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08880C: lea.l $868a6(pc), a1
pub fn i_08880c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88810;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $088810: jsr $4ae.l
pub fn i_088810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88816;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88816);
    cpu.pc = 0x4AE;
    16
}
/// $088816: jsr $5dd22.l
pub fn i_088816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8881C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8881C);
    cpu.pc = 0x5DD22;
    16
}
/// $08881C: addi.w #$48, $22(a0)
pub fn i_08881c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88822;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088822: addi.w #$40, $24(a0)
pub fn i_088822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88828;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088828: move.w #$70, $38(a0)
pub fn i_088828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8882E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08882E: lea.l $2ed8a0.l, a1
pub fn i_08882e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88834;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED8A0;
    20
}
/// $088834: move.l a1, $74(a0)
pub fn i_088834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88838;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088838: move.w #$1, $72(a0)
pub fn i_088838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8883E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08883E: lea.l $ee130.l, a1
pub fn i_08883e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88844;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE130;
    20
}
/// $088844: move.l a1, $7c(a0)
pub fn i_088844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88848;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088848: lea.l $868a6(pc), a1
pub fn i_088848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8884C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $08884C: jsr $4ae.l
pub fn i_08884c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88852);
    cpu.pc = 0x4AE;
    16
}
/// $088852: jsr $5dd22.l
pub fn i_088852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88858;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88858);
    cpu.pc = 0x5DD22;
    16
}
/// $088858: addi.w #$78, $22(a0)
pub fn i_088858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8885E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x78, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08885E: addi.w #$40, $24(a0)
pub fn i_08885e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88864;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088864: move.w #$70, $38(a0)
pub fn i_088864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8886A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08886A: lea.l $2ed8b4.l, a1
pub fn i_08886a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88870;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED8B4;
    20
}
/// $088870: move.l a1, $74(a0)
pub fn i_088870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88874;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088874: move.w #$2, $72(a0)
pub fn i_088874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8887A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08887A: lea.l $ee146.l, a1
pub fn i_08887a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88880;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE146;
    20
}
/// $088880: move.l a1, $7c(a0)
pub fn i_088880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88884;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088884: lea.l $868a6(pc), a1
pub fn i_088884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88888;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868A6;
    16
}
/// $088888: jsr $4ae.l
pub fn i_088888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8888E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8888E);
    cpu.pc = 0x4AE;
    16
}
/// $08888E: jsr $5dd22.l
pub fn i_08888e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88894;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88894);
    cpu.pc = 0x5DD22;
    16
}
/// $088894: addi.w #$a8, $22(a0)
pub fn i_088894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8889A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08889A: addi.w #$40, $24(a0)
pub fn i_08889a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888A0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0888A0: move.w #$70, $38(a0)
pub fn i_0888a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888A6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0888A6: lea.l $2ed8c8.l, a1
pub fn i_0888a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888AC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED8C8;
    20
}
/// $0888AC: move.l a1, $74(a0)
pub fn i_0888ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888B0;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0888B0: move.w #$3, $72(a0)
pub fn i_0888b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888B6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0888B6: lea.l $ee15c.l, a1
pub fn i_0888b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888BC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE15C;
    20
}
/// $0888BC: move.l a1, $7c(a0)
pub fn i_0888bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888C0;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0888C0: lea.l $86d0c(pc), a1
pub fn i_0888c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86D0C;
    16
}
/// $0888C4: jsr $4ae.l
pub fn i_0888c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x888CA);
    cpu.pc = 0x4AE;
    16
}
/// $0888CA: jsr $5dd22.l
pub fn i_0888ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x888D0);
    cpu.pc = 0x5DD22;
    16
}
/// $0888D0: addi.w #$28, $22(a0)
pub fn i_0888d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888D6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0888D6: addi.w #$8, $24(a0)
pub fn i_0888d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888DC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0888DC: lea.l $2ed8dc.l, a1
pub fn i_0888dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888E2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED8DC;
    20
}
/// $0888E2: move.l a1, $74(a0)
pub fn i_0888e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888E6;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0888E6: move.w #$4, $72(a0)
pub fn i_0888e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888EC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0888EC: lea.l $86d0c(pc), a1
pub fn i_0888ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86D0C;
    16
}
/// $0888F0: jsr $4ae.l
pub fn i_0888f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x888F6);
    cpu.pc = 0x4AE;
    16
}
/// $0888F6: jsr $5dd22.l
pub fn i_0888f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x888FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x888FC);
    cpu.pc = 0x5DD22;
    16
}
/// $0888FC: addi.w #$a0, $22(a0)
pub fn i_0888fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88902;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088902: addi.w #$8, $24(a0)
pub fn i_088902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88908;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088908: lea.l $2ed8f0.l, a1
pub fn i_088908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8890E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED8F0;
    20
}
/// $08890E: move.l a1, $74(a0)
pub fn i_08890e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88912;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088912: move.w #$5, $72(a0)
pub fn i_088912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88918;
    cpu.ir = 0x317C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088918: rts
pub fn i_088918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8891A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_SpawnChildren_V4_08891a  ($08891A, 270 B, enemies/bosses/fort_scene4_0865be.s)
/// $08891A: lea.l $868ba(pc), a1
pub fn i_08891a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8891E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868BA;
    16
}
/// $08891E: jsr $4ae.l
pub fn i_08891e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88924;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88924);
    cpu.pc = 0x4AE;
    16
}
/// $088924: jsr $5dd22.l
pub fn i_088924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8892A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8892A);
    cpu.pc = 0x5DD22;
    16
}
/// $08892A: addi.w #$28, $22(a0)
pub fn i_08892a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88930;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088930: addi.w #$50, $24(a0)
pub fn i_088930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88936;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088936: move.w #$60, $38(a0)
pub fn i_088936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8893C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08893C: lea.l $2ed904.l, a1
pub fn i_08893c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88942;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED904;
    20
}
/// $088942: move.l a1, $74(a0)
pub fn i_088942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88946;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088946: move.w #$0, $72(a0)
pub fn i_088946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8894C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08894C: lea.l $ee172.l, a1
pub fn i_08894c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88952;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE172;
    20
}
/// $088952: move.l a1, $7c(a0)
pub fn i_088952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88956;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088956: lea.l $868ba(pc), a1
pub fn i_088956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8895A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868BA;
    16
}
/// $08895A: jsr $4ae.l
pub fn i_08895a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88960;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88960);
    cpu.pc = 0x4AE;
    16
}
/// $088960: jsr $5dd22.l
pub fn i_088960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88966;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88966);
    cpu.pc = 0x5DD22;
    16
}
/// $088966: addi.w #$68, $22(a0)
pub fn i_088966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8896C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x68, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08896C: addi.w #$50, $24(a0)
pub fn i_08896c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88972;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088972: move.w #$60, $38(a0)
pub fn i_088972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88978;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088978: lea.l $2ed918.l, a1
pub fn i_088978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8897E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED918;
    20
}
/// $08897E: move.l a1, $74(a0)
pub fn i_08897e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88982;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088982: move.w #$1, $72(a0)
pub fn i_088982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88988;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088988: lea.l $ee188.l, a1
pub fn i_088988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8898E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE188;
    20
}
/// $08898E: move.l a1, $7c(a0)
pub fn i_08898e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88992;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088992: lea.l $868ba(pc), a1
pub fn i_088992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88996;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x868BA;
    16
}
/// $088996: jsr $4ae.l
pub fn i_088996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8899C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8899C);
    cpu.pc = 0x4AE;
    16
}
/// $08899C: jsr $5dd22.l
pub fn i_08899c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x889A2);
    cpu.pc = 0x5DD22;
    16
}
/// $0889A2: addi.w #$a8, $22(a0)
pub fn i_0889a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889A8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0889A8: addi.w #$50, $24(a0)
pub fn i_0889a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889AE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0889AE: move.w #$60, $38(a0)
pub fn i_0889ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889B4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0889B4: lea.l $2ed92c.l, a1
pub fn i_0889b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889BA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED92C;
    20
}
/// $0889BA: move.l a1, $74(a0)
pub fn i_0889ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889BE;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0889BE: move.w #$2, $72(a0)
pub fn i_0889be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889C4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0889C4: lea.l $ee19e.l, a1
pub fn i_0889c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889CA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEE19E;
    20
}
/// $0889CA: move.l a1, $7c(a0)
pub fn i_0889ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889CE;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0889CE: lea.l $86d20(pc), a1
pub fn i_0889ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86D20;
    16
}
/// $0889D2: jsr $4ae.l
pub fn i_0889d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x889D8);
    cpu.pc = 0x4AE;
    16
}
/// $0889D8: jsr $5dd22.l
pub fn i_0889d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x889DE);
    cpu.pc = 0x5DD22;
    16
}
/// $0889DE: addi.w #$28, $22(a0)
pub fn i_0889de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889E4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0889E4: addi.w #$0, $24(a0)
pub fn i_0889e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889EA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0889EA: lea.l $2ed940.l, a1
pub fn i_0889ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889F0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED940;
    20
}
/// $0889F0: move.l a1, $74(a0)
pub fn i_0889f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889F4;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0889F4: move.w #$3, $72(a0)
pub fn i_0889f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889FA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0889FA: lea.l $86d20(pc), a1
pub fn i_0889fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x889FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x86D20;
    16
}
/// $0889FE: jsr $4ae.l
pub fn i_0889fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88A04);
    cpu.pc = 0x4AE;
    16
}
/// $088A04: jsr $5dd22.l
pub fn i_088a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88A0A);
    cpu.pc = 0x5DD22;
    16
}
/// $088A0A: addi.w #$98, $22(a0)
pub fn i_088a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A10;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x98, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088A10: addi.w #$0, $24(a0)
pub fn i_088a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A16;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $088A16: lea.l $2ed954.l, a1
pub fn i_088a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A1C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2ED954;
    20
}
/// $088A1C: move.l a1, $74(a0)
pub fn i_088a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A20;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $088A20: move.w #$4, $72(a0)
pub fn i_088a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A26;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088A26: rts
pub fn i_088a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_BlitWreck_088a28  ($088A28, 46 B, enemies/bosses/fort_scene4_0865be.s)
/// $088A28: cmpi.b #$0, $20(a6)
pub fn i_088a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A2E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $088A2E: bne.w $88a64
pub fn i_088a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A32;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88A64; return 10; }
    8
}
/// $088A32: lea.l $2edaa8.l, a2
pub fn i_088a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A38;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDAA8;
    20
}
/// $088A38: jsr $5022a.l
pub fn i_088a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88A3E);
    cpu.pc = 0x5022A;
    16
}
/// $088A3E: lea.l $2edabc.l, a2
pub fn i_088a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A44;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDABC;
    20
}
/// $088A44: jsr $5022a.l
pub fn i_088a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88A4A);
    cpu.pc = 0x5022A;
    16
}
/// $088A4A: lea.l $2edad0.l, a2
pub fn i_088a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A50;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDAD0;
    20
}
/// $088A50: jsr $5022a.l
pub fn i_088a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88A56);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_088a56  ($088A56, 14 B, kernel/state_dispatch_stubs.c)
/// $088A56: lea.l $2edae4.l, a2
pub fn i_088a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A5C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDAE4;
    20
}
/// $088A5C: jsr $5022a.l
pub fn i_088a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88A62);
    cpu.pc = 0x5022A;
    16
}
/// $088A62: rts
pub fn i_088a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_BlitWreck_V1_088a64  ($088A64, 34 B, enemies/bosses/scene5_airship_088a64.s)
/// $088A64: cmpi.b #$1, $20(a6)
pub fn i_088a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A6A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $088A6A: bne.w $88a94
pub fn i_088a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A6E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88A94; return 10; }
    8
}
/// $088A6E: lea.l $2edaf8.l, a2
pub fn i_088a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A74;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDAF8;
    20
}
/// $088A74: jsr $5022a.l
pub fn i_088a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88A7A);
    cpu.pc = 0x5022A;
    16
}
/// $088A7A: lea.l $2edb0c.l, a2
pub fn i_088a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A80;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDB0C;
    20
}
/// $088A80: jsr $5022a.l
pub fn i_088a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88A86);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_088a86  ($088A86, 14 B, kernel/state_dispatch_stubs.c)
/// $088A86: lea.l $2edb20.l, a2
pub fn i_088a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A8C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDB20;
    20
}
/// $088A8C: jsr $5022a.l
pub fn i_088a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88A92);
    cpu.pc = 0x5022A;
    16
}
/// $088A92: rts
pub fn i_088a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_BlitWreck_V2_088a94  ($088A94, 46 B, enemies/bosses/scene5_airship_088a64.s)
/// $088A94: cmpi.b #$2, $20(a6)
pub fn i_088a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A9A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $088A9A: bne.w $88ad0
pub fn i_088a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88A9E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88AD0; return 10; }
    8
}
/// $088A9E: lea.l $2edb34.l, a2
pub fn i_088a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AA4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDB34;
    20
}
/// $088AA4: jsr $5022a.l
pub fn i_088aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88AAA);
    cpu.pc = 0x5022A;
    16
}
/// $088AAA: lea.l $2edb48.l, a2
pub fn i_088aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AB0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDB48;
    20
}
/// $088AB0: jsr $5022a.l
pub fn i_088ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88AB6);
    cpu.pc = 0x5022A;
    16
}
/// $088AB6: lea.l $2edb5c.l, a2
pub fn i_088ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88ABC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDB5C;
    20
}
/// $088ABC: jsr $5022a.l
pub fn i_088abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88AC2);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_088ac2  ($088AC2, 14 B, kernel/state_dispatch_stubs.c)
/// $088AC2: lea.l $2edb70.l, a2
pub fn i_088ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AC8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDB70;
    20
}
/// $088AC8: jsr $5022a.l
pub fn i_088ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88ACE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88ACE);
    cpu.pc = 0x5022A;
    16
}
/// $088ACE: rts
pub fn i_088ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_BlitWreck_V3_088ad0  ($088AD0, 34 B, enemies/bosses/scene5_airship_088a64.s)
/// $088AD0: cmpi.b #$3, $20(a6)
pub fn i_088ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AD6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $088AD6: bne.w $88b00
pub fn i_088ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88ADA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88B00; return 10; }
    8
}
/// $088ADA: lea.l $2edb84.l, a2
pub fn i_088ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AE0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDB84;
    20
}
/// $088AE0: jsr $5022a.l
pub fn i_088ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88AE6);
    cpu.pc = 0x5022A;
    16
}
/// $088AE6: lea.l $2edb98.l, a2
pub fn i_088ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AEC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDB98;
    20
}
/// $088AEC: jsr $5022a.l
pub fn i_088aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88AF2);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_088af2  ($088AF2, 14 B, kernel/state_dispatch_stubs.c)
/// $088AF2: lea.l $2edbac.l, a2
pub fn i_088af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AF8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDBAC;
    20
}
/// $088AF8: jsr $5022a.l
pub fn i_088af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88AFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88AFE);
    cpu.pc = 0x5022A;
    16
}
/// $088AFE: rts
pub fn i_088afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fort_BlitWreck_V4_088b00  ($088B00, 46 B, enemies/bosses/scene5_airship_088a64.s)
/// $088B00: cmpi.b #$4, $20(a6)
pub fn i_088b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B06;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $088B06: bne.w $88b3c
pub fn i_088b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B0A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x88B3C; return 10; }
    8
}
/// $088B0A: lea.l $2edbc0.l, a2
pub fn i_088b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B10;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDBC0;
    20
}
/// $088B10: jsr $5022a.l
pub fn i_088b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88B16);
    cpu.pc = 0x5022A;
    16
}
/// $088B16: lea.l $2edbd4.l, a2
pub fn i_088b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B1C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDBD4;
    20
}
/// $088B1C: jsr $5022a.l
pub fn i_088b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88B22);
    cpu.pc = 0x5022A;
    16
}
/// $088B22: lea.l $2edbe8.l, a2
pub fn i_088b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B28;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDBE8;
    20
}
/// $088B28: jsr $5022a.l
pub fn i_088b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88B2E);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_088b2e  ($088B2E, 14 B, kernel/state_dispatch_stubs.c)
/// $088B2E: lea.l $2edbfc.l, a2
pub fn i_088b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B34;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDBFC;
    20
}
/// $088B34: jsr $5022a.l
pub fn i_088b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88B3A);
    cpu.pc = 0x5022A;
    16
}
/// $088B3A: rts
pub fn i_088b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00088B3C  ($088B3C, 2 B, thunks/trivial_rts.c)
/// $088B3C: rts
pub fn i_088b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B3E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088b3e  ($088B3E, 14 B, kernel/state_dispatch_stubs.c)
/// $088B3E: lea.l $2ed1fc.l, a2
pub fn i_088b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B44;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED1FC;
    20
}
/// $088B44: jsr $5022a.l
pub fn i_088b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88B4A);
    cpu.pc = 0x5022A;
    16
}
/// $088B4A: rts
pub fn i_088b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088b4c  ($088B4C, 14 B, kernel/state_dispatch_stubs.c)
/// $088B4C: lea.l $2ed210.l, a2
pub fn i_088b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B52;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED210;
    20
}
/// $088B52: jsr $5022a.l
pub fn i_088b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88B58);
    cpu.pc = 0x5022A;
    16
}
/// $088B58: rts
pub fn i_088b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088b5a  ($088B5A, 14 B, kernel/state_dispatch_stubs.c)
/// $088B5A: lea.l $2ed224.l, a2
pub fn i_088b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B60;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED224;
    20
}
/// $088B60: jsr $5022a.l
pub fn i_088b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88B66);
    cpu.pc = 0x5022A;
    16
}
/// $088B66: rts
pub fn i_088b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088b68  ($088B68, 14 B, kernel/state_dispatch_stubs.c)
/// $088B68: lea.l $2ed238.l, a2
pub fn i_088b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B6E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED238;
    20
}
/// $088B6E: jsr $5022a.l
pub fn i_088b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88B74);
    cpu.pc = 0x5022A;
    16
}
/// $088B74: rts
pub fn i_088b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088b76  ($088B76, 14 B, kernel/state_dispatch_stubs.c)
/// $088B76: lea.l $2ed24c.l, a2
pub fn i_088b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B7C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED24C;
    20
}
/// $088B7C: jsr $5022a.l
pub fn i_088b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88B82);
    cpu.pc = 0x5022A;
    16
}
/// $088B82: rts
pub fn i_088b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088b84  ($088B84, 14 B, kernel/state_dispatch_stubs.c)
/// $088B84: lea.l $2ed260.l, a2
pub fn i_088b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B8A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED260;
    20
}
/// $088B8A: jsr $5022a.l
pub fn i_088b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88B90);
    cpu.pc = 0x5022A;
    16
}
/// $088B90: rts
pub fn i_088b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088b92  ($088B92, 14 B, kernel/state_dispatch_stubs.c)
/// $088B92: lea.l $2ed274.l, a2
pub fn i_088b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B98;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED274;
    20
}
/// $088B98: jsr $5022a.l
pub fn i_088b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88B9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88B9E);
    cpu.pc = 0x5022A;
    16
}
/// $088B9E: rts
pub fn i_088b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088ba0  ($088BA0, 14 B, kernel/state_dispatch_stubs.c)
/// $088BA0: lea.l $2ed288.l, a2
pub fn i_088ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BA6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED288;
    20
}
/// $088BA6: jsr $5022a.l
pub fn i_088ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88BAC);
    cpu.pc = 0x5022A;
    16
}
/// $088BAC: rts
pub fn i_088bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BAE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088bae  ($088BAE, 14 B, kernel/state_dispatch_stubs.c)
/// $088BAE: lea.l $2ed29c.l, a2
pub fn i_088bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BB4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED29C;
    20
}
/// $088BB4: jsr $5022a.l
pub fn i_088bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88BBA);
    cpu.pc = 0x5022A;
    16
}
/// $088BBA: rts
pub fn i_088bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088bbc  ($088BBC, 14 B, kernel/state_dispatch_stubs.c)
/// $088BBC: lea.l $2ed2b0.l, a2
pub fn i_088bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BC2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED2B0;
    20
}
/// $088BC2: jsr $5022a.l
pub fn i_088bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88BC8);
    cpu.pc = 0x5022A;
    16
}
/// $088BC8: rts
pub fn i_088bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088bca  ($088BCA, 14 B, kernel/state_dispatch_stubs.c)
/// $088BCA: lea.l $2ed2c4.l, a2
pub fn i_088bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BD0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED2C4;
    20
}
/// $088BD0: jsr $5022a.l
pub fn i_088bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88BD6);
    cpu.pc = 0x5022A;
    16
}
/// $088BD6: rts
pub fn i_088bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088bd8  ($088BD8, 14 B, kernel/state_dispatch_stubs.c)
/// $088BD8: lea.l $2ed2d8.l, a2
pub fn i_088bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BDE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED2D8;
    20
}
/// $088BDE: jsr $5022a.l
pub fn i_088bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88BE4);
    cpu.pc = 0x5022A;
    16
}
/// $088BE4: rts
pub fn i_088be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088be6  ($088BE6, 14 B, kernel/state_dispatch_stubs.c)
/// $088BE6: lea.l $2ed2ec.l, a2
pub fn i_088be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BEC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED2EC;
    20
}
/// $088BEC: jsr $5022a.l
pub fn i_088bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88BF2);
    cpu.pc = 0x5022A;
    16
}
/// $088BF2: rts
pub fn i_088bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088bf4  ($088BF4, 14 B, kernel/state_dispatch_stubs.c)
/// $088BF4: lea.l $2ed300.l, a2
pub fn i_088bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88BFA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED300;
    20
}
/// $088BFA: jsr $5022a.l
pub fn i_088bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88C00);
    cpu.pc = 0x5022A;
    16
}
/// $088C00: rts
pub fn i_088c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088c02  ($088C02, 14 B, kernel/state_dispatch_stubs.c)
/// $088C02: lea.l $2ed314.l, a2
pub fn i_088c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C08;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED314;
    20
}
/// $088C08: jsr $5022a.l
pub fn i_088c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88C0E);
    cpu.pc = 0x5022A;
    16
}
/// $088C0E: rts
pub fn i_088c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088c10  ($088C10, 14 B, kernel/state_dispatch_stubs.c)
/// $088C10: lea.l $2ed328.l, a2
pub fn i_088c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C16;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED328;
    20
}
/// $088C16: jsr $5022a.l
pub fn i_088c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88C1C);
    cpu.pc = 0x5022A;
    16
}
/// $088C1C: rts
pub fn i_088c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088c1e  ($088C1E, 14 B, kernel/state_dispatch_stubs.c)
/// $088C1E: lea.l $2ed33c.l, a2
pub fn i_088c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C24;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED33C;
    20
}
/// $088C24: jsr $5022a.l
pub fn i_088c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88C2A);
    cpu.pc = 0x5022A;
    16
}
/// $088C2A: rts
pub fn i_088c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088c2c  ($088C2C, 14 B, kernel/state_dispatch_stubs.c)
/// $088C2C: lea.l $2ed350.l, a2
pub fn i_088c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C32;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED350;
    20
}
/// $088C32: jsr $5022a.l
pub fn i_088c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88C38);
    cpu.pc = 0x5022A;
    16
}
/// $088C38: rts
pub fn i_088c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088c3a  ($088C3A, 14 B, kernel/state_dispatch_stubs.c)
/// $088C3A: lea.l $2ed364.l, a2
pub fn i_088c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C40;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED364;
    20
}
/// $088C40: jsr $5022a.l
pub fn i_088c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88C46);
    cpu.pc = 0x5022A;
    16
}
/// $088C46: rts
pub fn i_088c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088c48  ($088C48, 14 B, kernel/state_dispatch_stubs.c)
/// $088C48: lea.l $2ed378.l, a2
pub fn i_088c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C4E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED378;
    20
}
/// $088C4E: jsr $5022a.l
pub fn i_088c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88C54);
    cpu.pc = 0x5022A;
    16
}
/// $088C54: rts
pub fn i_088c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088c56  ($088C56, 14 B, kernel/state_dispatch_stubs.c)
/// $088C56: lea.l $2ed38c.l, a2
pub fn i_088c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C5C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED38C;
    20
}
/// $088C5C: jsr $5022a.l
pub fn i_088c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88C62);
    cpu.pc = 0x5022A;
    16
}
/// $088C62: rts
pub fn i_088c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088c64  ($088C64, 14 B, kernel/state_dispatch_stubs.c)
/// $088C64: lea.l $2ed3a0.l, a2
pub fn i_088c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C6A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED3A0;
    20
}
/// $088C6A: jsr $5022a.l
pub fn i_088c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88C70);
    cpu.pc = 0x5022A;
    16
}
/// $088C70: rts
pub fn i_088c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088c72  ($088C72, 14 B, kernel/state_dispatch_stubs.c)
/// $088C72: lea.l $2ed3b4.l, a2
pub fn i_088c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C78;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED3B4;
    20
}
/// $088C78: jsr $5022a.l
pub fn i_088c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88C7E);
    cpu.pc = 0x5022A;
    16
}
/// $088C7E: rts
pub fn i_088c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088c80  ($088C80, 14 B, kernel/state_dispatch_stubs.c)
/// $088C80: lea.l $2ed3c8.l, a2
pub fn i_088c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C86;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED3C8;
    20
}
/// $088C86: jsr $5022a.l
pub fn i_088c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88C8C);
    cpu.pc = 0x5022A;
    16
}
/// $088C8C: rts
pub fn i_088c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088c8e  ($088C8E, 14 B, kernel/state_dispatch_stubs.c)
/// $088C8E: lea.l $2ed3dc.l, a2
pub fn i_088c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C94;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED3DC;
    20
}
/// $088C94: jsr $5022a.l
pub fn i_088c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88C9A);
    cpu.pc = 0x5022A;
    16
}
/// $088C9A: rts
pub fn i_088c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88C9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088c9c  ($088C9C, 14 B, kernel/state_dispatch_stubs.c)
/// $088C9C: lea.l $2ed3f0.l, a2
pub fn i_088c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CA2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED3F0;
    20
}
/// $088CA2: jsr $5022a.l
pub fn i_088ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88CA8);
    cpu.pc = 0x5022A;
    16
}
/// $088CA8: rts
pub fn i_088ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088caa  ($088CAA, 14 B, kernel/state_dispatch_stubs.c)
/// $088CAA: lea.l $2ed404.l, a2
pub fn i_088caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CB0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED404;
    20
}
/// $088CB0: jsr $5022a.l
pub fn i_088cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88CB6);
    cpu.pc = 0x5022A;
    16
}
/// $088CB6: rts
pub fn i_088cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088cb8  ($088CB8, 14 B, kernel/state_dispatch_stubs.c)
/// $088CB8: lea.l $2ed418.l, a2
pub fn i_088cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CBE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED418;
    20
}
/// $088CBE: jsr $5022a.l
pub fn i_088cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88CC4);
    cpu.pc = 0x5022A;
    16
}
/// $088CC4: rts
pub fn i_088cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088cc6  ($088CC6, 14 B, kernel/state_dispatch_stubs.c)
/// $088CC6: lea.l $2ed42c.l, a2
pub fn i_088cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CCC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED42C;
    20
}
/// $088CCC: jsr $5022a.l
pub fn i_088ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88CD2);
    cpu.pc = 0x5022A;
    16
}
/// $088CD2: rts
pub fn i_088cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088cd4  ($088CD4, 14 B, kernel/state_dispatch_stubs.c)
/// $088CD4: lea.l $2ed440.l, a2
pub fn i_088cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CDA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED440;
    20
}
/// $088CDA: jsr $5022a.l
pub fn i_088cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88CE0);
    cpu.pc = 0x5022A;
    16
}
/// $088CE0: rts
pub fn i_088ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088ce2  ($088CE2, 14 B, kernel/state_dispatch_stubs.c)
/// $088CE2: lea.l $2ed454.l, a2
pub fn i_088ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CE8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED454;
    20
}
/// $088CE8: jsr $5022a.l
pub fn i_088ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88CEE);
    cpu.pc = 0x5022A;
    16
}
/// $088CEE: rts
pub fn i_088cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088cf0  ($088CF0, 14 B, kernel/state_dispatch_stubs.c)
/// $088CF0: lea.l $2ed468.l, a2
pub fn i_088cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CF6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED468;
    20
}
/// $088CF6: jsr $5022a.l
pub fn i_088cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88CFC);
    cpu.pc = 0x5022A;
    16
}
/// $088CFC: rts
pub fn i_088cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88CFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088cfe  ($088CFE, 14 B, kernel/state_dispatch_stubs.c)
/// $088CFE: lea.l $2ed47c.l, a2
pub fn i_088cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D04;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED47C;
    20
}
/// $088D04: jsr $5022a.l
pub fn i_088d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88D0A);
    cpu.pc = 0x5022A;
    16
}
/// $088D0A: rts
pub fn i_088d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088d0c  ($088D0C, 14 B, kernel/state_dispatch_stubs.c)
/// $088D0C: lea.l $2ed490.l, a2
pub fn i_088d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D12;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED490;
    20
}
/// $088D12: jsr $5022a.l
pub fn i_088d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88D18);
    cpu.pc = 0x5022A;
    16
}
/// $088D18: rts
pub fn i_088d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088d1a  ($088D1A, 14 B, kernel/state_dispatch_stubs.c)
/// $088D1A: lea.l $2ed4a4.l, a2
pub fn i_088d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D20;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED4A4;
    20
}
/// $088D20: jsr $5022a.l
pub fn i_088d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88D26);
    cpu.pc = 0x5022A;
    16
}
/// $088D26: rts
pub fn i_088d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088d28  ($088D28, 14 B, kernel/state_dispatch_stubs.c)
/// $088D28: lea.l $2ed4b8.l, a2
pub fn i_088d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D2E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED4B8;
    20
}
/// $088D2E: jsr $5022a.l
pub fn i_088d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88D34);
    cpu.pc = 0x5022A;
    16
}
/// $088D34: rts
pub fn i_088d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088d36  ($088D36, 14 B, kernel/state_dispatch_stubs.c)
/// $088D36: lea.l $2ed4cc.l, a2
pub fn i_088d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D3C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED4CC;
    20
}
/// $088D3C: jsr $5022a.l
pub fn i_088d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88D42);
    cpu.pc = 0x5022A;
    16
}
/// $088D42: rts
pub fn i_088d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088d44  ($088D44, 14 B, kernel/state_dispatch_stubs.c)
/// $088D44: lea.l $2ed4e0.l, a2
pub fn i_088d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D4A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED4E0;
    20
}
/// $088D4A: jsr $5022a.l
pub fn i_088d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88D50);
    cpu.pc = 0x5022A;
    16
}
/// $088D50: rts
pub fn i_088d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D52;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088d52  ($088D52, 14 B, kernel/state_dispatch_stubs.c)
/// $088D52: lea.l $2ed4f4.l, a2
pub fn i_088d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D58;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED4F4;
    20
}
/// $088D58: jsr $5022a.l
pub fn i_088d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88D5E);
    cpu.pc = 0x5022A;
    16
}
/// $088D5E: rts
pub fn i_088d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088d60  ($088D60, 14 B, kernel/state_dispatch_stubs.c)
/// $088D60: lea.l $2ed508.l, a2
pub fn i_088d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D66;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED508;
    20
}
/// $088D66: jsr $5022a.l
pub fn i_088d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88D6C);
    cpu.pc = 0x5022A;
    16
}
/// $088D6C: rts
pub fn i_088d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088d6e  ($088D6E, 14 B, kernel/state_dispatch_stubs.c)
/// $088D6E: lea.l $2ed51c.l, a2
pub fn i_088d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D74;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED51C;
    20
}
/// $088D74: jsr $5022a.l
pub fn i_088d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88D7A);
    cpu.pc = 0x5022A;
    16
}
/// $088D7A: rts
pub fn i_088d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088d7c  ($088D7C, 14 B, kernel/state_dispatch_stubs.c)
/// $088D7C: lea.l $2ed530.l, a2
pub fn i_088d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D82;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED530;
    20
}
/// $088D82: jsr $5022a.l
pub fn i_088d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88D88);
    cpu.pc = 0x5022A;
    16
}
/// $088D88: rts
pub fn i_088d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088d8a  ($088D8A, 14 B, kernel/state_dispatch_stubs.c)
/// $088D8A: lea.l $2ed544.l, a2
pub fn i_088d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D90;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED544;
    20
}
/// $088D90: jsr $5022a.l
pub fn i_088d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88D96);
    cpu.pc = 0x5022A;
    16
}
/// $088D96: rts
pub fn i_088d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D98;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088d98  ($088D98, 14 B, kernel/state_dispatch_stubs.c)
/// $088D98: lea.l $2ed558.l, a2
pub fn i_088d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88D9E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED558;
    20
}
/// $088D9E: jsr $5022a.l
pub fn i_088d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88DA4);
    cpu.pc = 0x5022A;
    16
}
/// $088DA4: rts
pub fn i_088da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088da6  ($088DA6, 14 B, kernel/state_dispatch_stubs.c)
/// $088DA6: lea.l $2ed56c.l, a2
pub fn i_088da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DAC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED56C;
    20
}
/// $088DAC: jsr $5022a.l
pub fn i_088dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88DB2);
    cpu.pc = 0x5022A;
    16
}
/// $088DB2: rts
pub fn i_088db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088db4  ($088DB4, 14 B, kernel/state_dispatch_stubs.c)
/// $088DB4: lea.l $2ed580.l, a2
pub fn i_088db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DBA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED580;
    20
}
/// $088DBA: jsr $5022a.l
pub fn i_088dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88DC0);
    cpu.pc = 0x5022A;
    16
}
/// $088DC0: rts
pub fn i_088dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088dc2  ($088DC2, 14 B, kernel/state_dispatch_stubs.c)
/// $088DC2: lea.l $2ed594.l, a2
pub fn i_088dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DC8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED594;
    20
}
/// $088DC8: jsr $5022a.l
pub fn i_088dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88DCE);
    cpu.pc = 0x5022A;
    16
}
/// $088DCE: rts
pub fn i_088dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088dd0  ($088DD0, 14 B, kernel/state_dispatch_stubs.c)
/// $088DD0: lea.l $2ed5a8.l, a2
pub fn i_088dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DD6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED5A8;
    20
}
/// $088DD6: jsr $5022a.l
pub fn i_088dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88DDC);
    cpu.pc = 0x5022A;
    16
}
/// $088DDC: rts
pub fn i_088ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DDE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088dde  ($088DDE, 14 B, kernel/state_dispatch_stubs.c)
/// $088DDE: lea.l $2ed5bc.l, a2
pub fn i_088dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DE4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED5BC;
    20
}
/// $088DE4: jsr $5022a.l
pub fn i_088de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88DEA);
    cpu.pc = 0x5022A;
    16
}
/// $088DEA: rts
pub fn i_088dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088dec  ($088DEC, 14 B, kernel/state_dispatch_stubs.c)
/// $088DEC: lea.l $2ed5d0.l, a2
pub fn i_088dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DF2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED5D0;
    20
}
/// $088DF2: jsr $5022a.l
pub fn i_088df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88DF8);
    cpu.pc = 0x5022A;
    16
}
/// $088DF8: rts
pub fn i_088df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88DFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088dfa  ($088DFA, 14 B, kernel/state_dispatch_stubs.c)
/// $088DFA: lea.l $2ed5e4.l, a2
pub fn i_088dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E00;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED5E4;
    20
}
/// $088E00: jsr $5022a.l
pub fn i_088e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88E06);
    cpu.pc = 0x5022A;
    16
}
/// $088E06: rts
pub fn i_088e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088e08  ($088E08, 14 B, kernel/state_dispatch_stubs.c)
/// $088E08: lea.l $2ed5f8.l, a2
pub fn i_088e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E0E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED5F8;
    20
}
/// $088E0E: jsr $5022a.l
pub fn i_088e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88E14);
    cpu.pc = 0x5022A;
    16
}
/// $088E14: rts
pub fn i_088e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088e16  ($088E16, 14 B, kernel/state_dispatch_stubs.c)
/// $088E16: lea.l $2ed60c.l, a2
pub fn i_088e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E1C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED60C;
    20
}
/// $088E1C: jsr $5022a.l
pub fn i_088e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88E22);
    cpu.pc = 0x5022A;
    16
}
/// $088E22: rts
pub fn i_088e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088e24  ($088E24, 14 B, kernel/state_dispatch_stubs.c)
/// $088E24: lea.l $2ed620.l, a2
pub fn i_088e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E2A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED620;
    20
}
/// $088E2A: jsr $5022a.l
pub fn i_088e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88E30);
    cpu.pc = 0x5022A;
    16
}
/// $088E30: rts
pub fn i_088e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088e32  ($088E32, 14 B, kernel/state_dispatch_stubs.c)
/// $088E32: lea.l $2ed634.l, a2
pub fn i_088e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E38;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED634;
    20
}
/// $088E38: jsr $5022a.l
pub fn i_088e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88E3E);
    cpu.pc = 0x5022A;
    16
}
/// $088E3E: rts
pub fn i_088e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088e40  ($088E40, 14 B, kernel/state_dispatch_stubs.c)
/// $088E40: lea.l $2ed648.l, a2
pub fn i_088e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E46;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED648;
    20
}
/// $088E46: jsr $5022a.l
pub fn i_088e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88E4C);
    cpu.pc = 0x5022A;
    16
}
/// $088E4C: rts
pub fn i_088e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088e4e  ($088E4E, 14 B, kernel/state_dispatch_stubs.c)
/// $088E4E: lea.l $2ed65c.l, a2
pub fn i_088e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E54;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED65C;
    20
}
/// $088E54: jsr $5022a.l
pub fn i_088e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88E5A);
    cpu.pc = 0x5022A;
    16
}
/// $088E5A: rts
pub fn i_088e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088e5c  ($088E5C, 14 B, kernel/state_dispatch_stubs.c)
/// $088E5C: lea.l $2ed670.l, a2
pub fn i_088e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E62;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED670;
    20
}
/// $088E62: jsr $5022a.l
pub fn i_088e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88E68);
    cpu.pc = 0x5022A;
    16
}
/// $088E68: rts
pub fn i_088e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088e6a  ($088E6A, 14 B, kernel/state_dispatch_stubs.c)
/// $088E6A: lea.l $2ed684.l, a2
pub fn i_088e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E70;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED684;
    20
}
/// $088E70: jsr $5022a.l
pub fn i_088e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88E76);
    cpu.pc = 0x5022A;
    16
}
/// $088E76: rts
pub fn i_088e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088e78  ($088E78, 14 B, kernel/state_dispatch_stubs.c)
/// $088E78: lea.l $2ed698.l, a2
pub fn i_088e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E7E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED698;
    20
}
/// $088E7E: jsr $5022a.l
pub fn i_088e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88E84);
    cpu.pc = 0x5022A;
    16
}
/// $088E84: rts
pub fn i_088e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088e86  ($088E86, 14 B, kernel/state_dispatch_stubs.c)
/// $088E86: lea.l $2ed6ac.l, a2
pub fn i_088e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E8C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED6AC;
    20
}
/// $088E8C: jsr $5022a.l
pub fn i_088e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88E92);
    cpu.pc = 0x5022A;
    16
}
/// $088E92: rts
pub fn i_088e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088e94  ($088E94, 14 B, kernel/state_dispatch_stubs.c)
/// $088E94: lea.l $2ed6c0.l, a2
pub fn i_088e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88E9A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED6C0;
    20
}
/// $088E9A: jsr $5022a.l
pub fn i_088e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88EA0);
    cpu.pc = 0x5022A;
    16
}
/// $088EA0: rts
pub fn i_088ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088ea2  ($088EA2, 14 B, kernel/state_dispatch_stubs.c)
/// $088EA2: lea.l $2ed6d4.l, a2
pub fn i_088ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EA8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED6D4;
    20
}
/// $088EA8: jsr $5022a.l
pub fn i_088ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88EAE);
    cpu.pc = 0x5022A;
    16
}
/// $088EAE: rts
pub fn i_088eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088eb0  ($088EB0, 14 B, kernel/state_dispatch_stubs.c)
/// $088EB0: lea.l $2ed6e8.l, a2
pub fn i_088eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EB6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED6E8;
    20
}
/// $088EB6: jsr $5022a.l
pub fn i_088eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88EBC);
    cpu.pc = 0x5022A;
    16
}
/// $088EBC: rts
pub fn i_088ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088ebe  ($088EBE, 14 B, kernel/state_dispatch_stubs.c)
/// $088EBE: lea.l $2ed6fc.l, a2
pub fn i_088ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EC4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED6FC;
    20
}
/// $088EC4: jsr $5022a.l
pub fn i_088ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88ECA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88ECA);
    cpu.pc = 0x5022A;
    16
}
/// $088ECA: rts
pub fn i_088eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88ECC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088ecc  ($088ECC, 14 B, kernel/state_dispatch_stubs.c)
/// $088ECC: lea.l $2ed710.l, a2
pub fn i_088ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88ED2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED710;
    20
}
/// $088ED2: jsr $5022a.l
pub fn i_088ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88ED8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88ED8);
    cpu.pc = 0x5022A;
    16
}
/// $088ED8: rts
pub fn i_088ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088eda  ($088EDA, 14 B, kernel/state_dispatch_stubs.c)
/// $088EDA: lea.l $2ed724.l, a2
pub fn i_088eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EE0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2ED724;
    20
}
/// $088EE0: jsr $5022a.l
pub fn i_088ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88EE6);
    cpu.pc = 0x5022A;
    16
}
/// $088EE6: rts
pub fn i_088ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088ee8  ($088EE8, 14 B, kernel/state_dispatch_stubs.c)
/// $088EE8: lea.l $2edc9c.l, a2
pub fn i_088ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EEE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDC9C;
    20
}
/// $088EEE: jsr $5022a.l
pub fn i_088eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88EF4);
    cpu.pc = 0x5022A;
    16
}
/// $088EF4: rts
pub fn i_088ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088ef6  ($088EF6, 14 B, kernel/state_dispatch_stubs.c)
/// $088EF6: lea.l $2edc10.l, a2
pub fn i_088ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88EFC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDC10;
    20
}
/// $088EFC: jsr $5022a.l
pub fn i_088efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88F02);
    cpu.pc = 0x5022A;
    16
}
/// $088F02: rts
pub fn i_088f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088f04  ($088F04, 14 B, kernel/state_dispatch_stubs.c)
/// $088F04: lea.l $2edc24.l, a2
pub fn i_088f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F0A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDC24;
    20
}
/// $088F0A: jsr $5022a.l
pub fn i_088f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88F10);
    cpu.pc = 0x5022A;
    16
}
/// $088F10: rts
pub fn i_088f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088f12  ($088F12, 14 B, kernel/state_dispatch_stubs.c)
/// $088F12: lea.l $2edc38.l, a2
pub fn i_088f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F18;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDC38;
    20
}
/// $088F18: jsr $5022a.l
pub fn i_088f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88F1E);
    cpu.pc = 0x5022A;
    16
}
/// $088F1E: rts
pub fn i_088f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088f20  ($088F20, 14 B, kernel/state_dispatch_stubs.c)
/// $088F20: lea.l $2edc4c.l, a2
pub fn i_088f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F26;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDC4C;
    20
}
/// $088F26: jsr $5022a.l
pub fn i_088f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88F2C);
    cpu.pc = 0x5022A;
    16
}
/// $088F2C: rts
pub fn i_088f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088f2e  ($088F2E, 14 B, kernel/state_dispatch_stubs.c)
/// $088F2E: lea.l $2edc60.l, a2
pub fn i_088f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F34;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDC60;
    20
}
/// $088F34: jsr $5022a.l
pub fn i_088f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88F3A);
    cpu.pc = 0x5022A;
    16
}
/// $088F3A: rts
pub fn i_088f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088f3c  ($088F3C, 14 B, kernel/state_dispatch_stubs.c)
/// $088F3C: lea.l $2edc74.l, a2
pub fn i_088f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F42;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDC74;
    20
}
/// $088F42: jsr $5022a.l
pub fn i_088f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88F48);
    cpu.pc = 0x5022A;
    16
}
/// $088F48: rts
pub fn i_088f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_088f4a  ($088F4A, 14 B, kernel/state_dispatch_stubs.c)
/// $088F4A: lea.l $2edc88.l, a2
pub fn i_088f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F50;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EDC88;
    20
}
/// $088F50: jsr $5022a.l
pub fn i_088f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88F56);
    cpu.pc = 0x5022A;
    16
}
/// $088F56: rts
pub fn i_088f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_088f58  ($088F58, 16 B, enemies/bosses/scene5_airship_088a64.s)
/// $088F58: movea.l $8(a6), a1
pub fn i_088f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F5C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $088F5C: move.b $10(a6), d0
pub fn i_088f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F60;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $088F60: cmp.b $10(a1), d0
pub fn i_088f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F64;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $088F64: bcs.w $88f6e
pub fn i_088f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F68;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x88F6E; return 10; }
    8
}
// ===== ClearXN_088f68  ($088F68, 6 B, thunks/ccr_helpers.c)
/// $088F68: andi.b #$ee, ccr
pub fn i_088f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F6C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $088F6C: rts
pub fn i_088f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_088f6e  ($088F6E, 6 B, thunks/ccr_helpers.c)
/// $088F6E: ori.b #$11, ccr
pub fn i_088f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F72;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $088F72: rts
pub fn i_088f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_PropStatic_A_088f74  ($088F74, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $088F74: movea.l $3c(a6), a1
pub fn i_088f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F78;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $088F78: jsr $2942a.l
pub fn i_088f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88F7E);
    cpu.pc = 0x2942A;
    16
}
/// $088F7E: lea.l $2ede92.l, a0
pub fn i_088f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F84;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EDE92;
    20
}
/// $088F84: jsr $28cd4.l
pub fn i_088f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88F8A);
    cpu.pc = 0x28CD4;
    16
}
/// $088F8A: bra.w $88fa8
pub fn i_088f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x88FA8;
    10
}
// ===== S5_PropStatic_B_088f8e  ($088F8E, 102 B, enemies/bosses/scene5_airship_088a64.s)
/// $088F8E: movea.l $3c(a6), a1
pub fn i_088f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F92;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $088F92: jsr $2942a.l
pub fn i_088f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88F98);
    cpu.pc = 0x2942A;
    16
}
/// $088F98: lea.l $2edea2.l, a0
pub fn i_088f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88F9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EDEA2;
    20
}
/// $088F9E: jsr $28cd4.l
pub fn i_088f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88FA4);
    cpu.pc = 0x28CD4;
    16
}
/// $088FA4: bra.w $88fa8
pub fn i_088fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x88FA8;
    10
}
/// $088FA8: move.w #$ca, d1
pub fn i_088fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FAC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xCA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $088FAC: jsr $236e.l
pub fn i_088fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88FB2);
    cpu.pc = 0x236E;
    16
}
/// $088FB2: move.w #$70, $70(a6)
pub fn i_088fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FB8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088FB8: move.b #$ff, $32(a6)
pub fn i_088fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FBE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $088FBE: move.b #$ff, $33(a6)
pub fn i_088fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FC4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $088FC4: move.b #$0, $3a(a6)
pub fn i_088fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FCA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $088FCA: move.w #$c000, $38(a6)
pub fn i_088fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FD0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $088FD0: lea.l $88fd6(pc), a1
pub fn i_088fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x88FD6;
    16
}
/// $088FD4: move.l a1, (a6)
pub fn i_088fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FD6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $088FD6: jsr $2783a.l
pub fn i_088fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88FDC);
    cpu.pc = 0x2783A;
    16
}
/// $088FDC: jsr $28d70.l
pub fn i_088fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88FE2);
    cpu.pc = 0x28D70;
    16
}
/// $088FE2: jsr $4fa70.l
pub fn i_088fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88FE8);
    cpu.pc = 0x4FA70;
    16
}
/// $088FE8: bcc.w $88ff2
pub fn i_088fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x88FF2; return 10; }
    8
}
/// $088FEC: jmp $518.l
pub fn i_088fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FF2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $088FF2: rts
pub fn i_088ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Bunker_088ff4  ($088FF4, 216 B, enemies/bosses/scene5_airship_088a64.s)
/// $088FF4: move.w #$cd, d1
pub fn i_088ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FF8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xCD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $088FF8: jsr $236e.l
pub fn i_088ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x88FFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x88FFE);
    cpu.pc = 0x236E;
    16
}
/// $088FFE: move.b $98(a6), d0
pub fn i_088ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89002;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $089002: move.b d0, $21(a6)
pub fn i_089002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89006;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089006: move.w #$40, $70(a6)
pub fn i_089006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8900C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08900C: move.b #$ff, $32(a6)
pub fn i_08900c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89012;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089012: move.b #$ff, $33(a6)
pub fn i_089012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89018;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089018: move.b #$0, $3a(a6)
pub fn i_089018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8901E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08901E: move.w #$28, $66(a6)
pub fn i_08901e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89024;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089024: move.w #$0, $38(a6)
pub fn i_089024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8902A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08902A: lea.l $2edeb2.l, a0
pub fn i_08902a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89030;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EDEB2;
    20
}
/// $089030: jsr $28cd4.l
pub fn i_089030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89036;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89036);
    cpu.pc = 0x28CD4;
    16
}
/// $089036: lea.l $2ee7b4.l, a0
pub fn i_089036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8903C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE7B4;
    20
}
/// $08903C: move.l a0, $48(a6)
pub fn i_08903c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89040;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $089040: lea.l $89046(pc), a1
pub fn i_089040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89044;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89046;
    16
}
/// $089044: move.l a1, (a6)
pub fn i_089044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89046;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089046: jsr $2783a.l
pub fn i_089046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8904C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8904C);
    cpu.pc = 0x2783A;
    16
}
/// $08904C: jsr $28d70.l
pub fn i_08904c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89052;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89052);
    cpu.pc = 0x28D70;
    16
}
/// $089052: jsr $2870a.l
pub fn i_089052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89058;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89058);
    cpu.pc = 0x2870A;
    16
}
/// $089058: bcc.w $8907a
pub fn i_089058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8905C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8907A; return 10; }
    8
}
/// $08905C: lea.l $5e766.l, a0
pub fn i_08905c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89062;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $089062: jsr $5e770.l
pub fn i_089062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89068;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89068);
    cpu.pc = 0x5E770;
    16
}
/// $089068: lea.l $2edec2.l, a0
pub fn i_089068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8906E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EDEC2;
    20
}
/// $08906E: jsr $28cd4.l
pub fn i_08906e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89074;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89074);
    cpu.pc = 0x28CD4;
    16
}
/// $089074: bclr.b #$3, $13(a6)
pub fn i_089074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8907A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08907A: jsr $28758.l
pub fn i_08907a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89080;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89080);
    cpu.pc = 0x28758;
    16
}
/// $089080: bcc.w $890ba
pub fn i_089080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89084;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x890BA; return 10; }
    8
}
/// $089084: move.w #$1023, d0
pub fn i_089084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89088;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089088: jsr $2352.l
pub fn i_089088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8908E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8908E);
    cpu.pc = 0x2352;
    16
}
/// $08908E: lea.l $2ef1ac.l, a1
pub fn i_08908e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89094;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF1AC;
    20
}
/// $089094: jsr $77c7e.l
pub fn i_089094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8909A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8909A);
    cpu.pc = 0x77C7E;
    16
}
/// $08909A: jsr $8b90e(pc)
pub fn i_08909a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8909E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8909E);
    cpu.pc = 0x8B90E;
    16
}
/// $08909E: lea.l $2ef626.l, a1
pub fn i_08909e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890A4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF626;
    20
}
/// $0890A4: jsr $43fac.l
pub fn i_0890a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x890AA);
    cpu.pc = 0x43FAC;
    16
}
/// $0890AA: lea.l $2ef298.l, a1
pub fn i_0890aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890B0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF298;
    20
}
/// $0890B0: jsr $77c7e.l
pub fn i_0890b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x890B6);
    cpu.pc = 0x77C7E;
    16
}
/// $0890B6: bra.w $890c4
pub fn i_0890b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x890C4;
    10
}
/// $0890BA: jsr $4fa70.l
pub fn i_0890ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x890C0);
    cpu.pc = 0x4FA70;
    16
}
/// $0890C0: bcc.w $890ca
pub fn i_0890c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x890CA; return 10; }
    8
}
/// $0890C4: jmp $518.l
pub fn i_0890c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890CA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0890CA: rts
pub fn i_0890ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Camp_Spawn_0890cc  ($0890CC, 146 B, enemies/bosses/scene5_airship_088a64.s)
/// $0890CC: movea.l $3c(a6), a1
pub fn i_0890cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890D0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0890D0: jsr $2942a.l
pub fn i_0890d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x890D6);
    cpu.pc = 0x2942A;
    16
}
/// $0890D6: lea.l $89290(pc), a1
pub fn i_0890d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89290;
    16
}
/// $0890DA: jsr $6fe.l
pub fn i_0890da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x890E0);
    cpu.pc = 0x6FE;
    16
}
/// $0890E0: jsr $5dd02.l
pub fn i_0890e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x890E6);
    cpu.pc = 0x5DD02;
    16
}
/// $0890E6: addi.w #$60, $22(a0)
pub fn i_0890e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890EC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0890EC: addi.w #$24, $24(a0)
pub fn i_0890ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890F2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x24, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0890F2: lea.l $89160(pc), a1
pub fn i_0890f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89160;
    16
}
/// $0890F6: jsr $6fe.l
pub fn i_0890f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x890FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x890FC);
    cpu.pc = 0x6FE;
    16
}
/// $0890FC: jsr $5dd02.l
pub fn i_0890fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89102;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89102);
    cpu.pc = 0x5DD02;
    16
}
/// $089102: move.b #$0, $21(a0)
pub fn i_089102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89108;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089108: addi.w #$20, $24(a0)
pub fn i_089108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8910E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08910E: lea.l $89160(pc), a1
pub fn i_08910e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89112;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89160;
    16
}
/// $089112: jsr $6fe.l
pub fn i_089112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89118;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89118);
    cpu.pc = 0x6FE;
    16
}
/// $089118: jsr $5dd02.l
pub fn i_089118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8911E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8911E);
    cpu.pc = 0x5DD02;
    16
}
/// $08911E: move.b #$1, $21(a0)
pub fn i_08911e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89124;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089124: addi.w #$38, $22(a0)
pub fn i_089124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8912A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08912A: addi.w #$20, $24(a0)
pub fn i_08912a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89130;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $089130: lea.l $89160(pc), a1
pub fn i_089130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89134;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89160;
    16
}
/// $089134: jsr $6fe.l
pub fn i_089134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8913A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8913A);
    cpu.pc = 0x6FE;
    16
}
/// $08913A: jsr $5dd02.l
pub fn i_08913a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89140;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89140);
    cpu.pc = 0x5DD02;
    16
}
/// $089140: move.b #$2, $21(a0)
pub fn i_089140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89146;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089146: addi.w #$80, $22(a0)
pub fn i_089146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8914C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08914C: addi.w #$20, $24(a0)
pub fn i_08914c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89152;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $089152: lea.l $89158(pc), a1
pub fn i_089152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89156;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89158;
    16
}
/// $089156: move.l a1, (a6)
pub fn i_089156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89158;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089158: jmp $518.l
pub fn i_089158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8915E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08915e  ($08915E, 2 B, enemies/bosses/scene5_airship_088a64.s)
/// $08915E: rts
pub fn i_08915e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89160;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Tent_089160  ($089160, 218 B, enemies/bosses/scene5_airship_088a64.s)
/// $089160: move.w #$cc, d1
pub fn i_089160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89164;
    cpu.ir = 0x323C;
    let v1: u32 = 0xCC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089164: jsr $236e.l
pub fn i_089164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8916A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8916A);
    cpu.pc = 0x236E;
    16
}
/// $08916A: move.w #$40, $70(a6)
pub fn i_08916a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89170;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089170: move.b #$ff, $32(a6)
pub fn i_089170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89176;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089176: move.b #$ff, $33(a6)
pub fn i_089176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8917C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08917C: move.b #$0, $3a(a6)
pub fn i_08917c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89182;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089182: move.w #$0, $38(a6)
pub fn i_089182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89188;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089188: move.w #$14, $66(a6)
pub fn i_089188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8918E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08918E: clr.l d0
pub fn i_08918e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89190;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $089190: move.b $21(a6), d0
pub fn i_089190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89194;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $089194: asl.l #$1, d0
pub fn i_089194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89196;
    cpu.ir = 0xE380;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, true);
    cpu.d[0] = r;
    8
}
/// $089196: movea.l #$2edf76, a0
pub fn i_089196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8919C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EDF76;
    cpu.a[0] = v1;
    4
}
/// $08919C: lsl.w #$2, d0
pub fn i_08919c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8919E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08919E: movea.l (a0, d0.w), a0
pub fn i_08919e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891A2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0891A2: cmpa.l #$ffffffff, a0
pub fn i_0891a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891A8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0891A8: beq.w $891b2
pub fn i_0891a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891AC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x891B2; return 10; }
    8
}
/// $0891AC: jsr $28cd4.l
pub fn i_0891ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x891B2);
    cpu.pc = 0x28CD4;
    16
}
/// $0891B2: lea.l $891b8(pc), a1
pub fn i_0891b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x891B8;
    16
}
/// $0891B6: move.l a1, (a6)
pub fn i_0891b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0891B8: jsr $2783a.l
pub fn i_0891b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x891BE);
    cpu.pc = 0x2783A;
    16
}
/// $0891BE: jsr $28d70.l
pub fn i_0891be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x891C4);
    cpu.pc = 0x28D70;
    16
}
/// $0891C4: jsr $2870a.l
pub fn i_0891c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x891CA);
    cpu.pc = 0x2870A;
    16
}
/// $0891CA: bcc.w $891e0
pub fn i_0891ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x891E0; return 10; }
    8
}
/// $0891CE: lea.l $5e766.l, a0
pub fn i_0891ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0891D4: jsr $5e770.l
pub fn i_0891d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x891DA);
    cpu.pc = 0x5E770;
    16
}
/// $0891DA: bclr.b #$3, $13(a6)
pub fn i_0891da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891E0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0891E0: jsr $28758.l
pub fn i_0891e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x891E6);
    cpu.pc = 0x28758;
    16
}
/// $0891E6: bcc.w $89228
pub fn i_0891e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89228; return 10; }
    8
}
/// $0891EA: move.w #$1030, d0
pub fn i_0891ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0891EE: jsr $2352.l
pub fn i_0891ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x891F4);
    cpu.pc = 0x2352;
    16
}
/// $0891F4: lea.l $2ef1d0.l, a1
pub fn i_0891f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x891FA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF1D0;
    20
}
/// $0891FA: jsr $77c7e.l
pub fn i_0891fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89200;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89200);
    cpu.pc = 0x77C7E;
    16
}
/// $089200: lea.l $631d0.l, a1
pub fn i_089200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89206;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x631D0;
    20
}
/// $089206: jsr $4ae.l
pub fn i_089206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8920C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8920C);
    cpu.pc = 0x4AE;
    16
}
/// $08920C: jsr $5dd22.l
pub fn i_08920c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89212;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89212);
    cpu.pc = 0x5DD22;
    16
}
/// $089212: move.w $54(a6), $22(a0)
pub fn i_089212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89218;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $089218: move.w $56(a6), $24(a0)
pub fn i_089218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8921E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08921E: jsr $8b8f4(pc)
pub fn i_08921e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89222;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x89222);
    cpu.pc = 0x8B8F4;
    16
}
/// $089222: lea.l $8923a(pc), a1
pub fn i_089222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89226;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8923A;
    16
}
/// $089226: move.l a1, (a6)
pub fn i_089226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89228;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089228: jsr $4fa70.l
pub fn i_089228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8922E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8922E);
    cpu.pc = 0x4FA70;
    16
}
/// $08922E: bcc.w $89238
pub fn i_08922e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89232;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89238; return 10; }
    8
}
/// $089232: jmp $518.l
pub fn i_089232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89238;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $089238: rts
pub fn i_089238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8923A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Tent_Ruin_08923a  ($08923A, 86 B, enemies/bosses/scene5_airship_088a64.s)
/// $08923A: move.l #$500, d0
pub fn i_08923a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89240;
    cpu.ir = 0x203C;
    let v1: u32 = 0x500;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $089240: jsr $51a28.l
pub fn i_089240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89246;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89246);
    cpu.pc = 0x51A28;
    16
}
/// $089246: clr.l d0
pub fn i_089246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89248;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $089248: move.b $21(a6), d0
pub fn i_089248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8924C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08924C: asl.l #$1, d0
pub fn i_08924c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8924E;
    cpu.ir = 0xE380;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, true);
    cpu.d[0] = r;
    8
}
/// $08924E: addq.l #$1, d0
pub fn i_08924e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89250;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $089250: movea.l #$2edf76, a0
pub fn i_089250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89256;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EDF76;
    cpu.a[0] = v1;
    4
}
/// $089256: lsl.w #$2, d0
pub fn i_089256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89258;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $089258: movea.l (a0, d0.w), a0
pub fn i_089258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8925C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08925C: cmpa.l #$ffffffff, a0
pub fn i_08925c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89262;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $089262: beq.w $8926c
pub fn i_089262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89266;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8926C; return 10; }
    8
}
/// $089266: jsr $28cd4.l
pub fn i_089266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8926C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8926C);
    cpu.pc = 0x28CD4;
    16
}
/// $08926C: lea.l $89272(pc), a1
pub fn i_08926c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89270;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89272;
    16
}
/// $089270: move.l a1, (a6)
pub fn i_089270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89272;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089272: jsr $2783a.l
pub fn i_089272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89278;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89278);
    cpu.pc = 0x2783A;
    16
}
/// $089278: jsr $28d70.l
pub fn i_089278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8927E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8927E);
    cpu.pc = 0x28D70;
    16
}
/// $08927E: jsr $4fa70.l
pub fn i_08927e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89284;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89284);
    cpu.pc = 0x4FA70;
    16
}
/// $089284: bcc.w $8928e
pub fn i_089284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89288;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8928E; return 10; }
    8
}
/// $089288: jmp $518.l
pub fn i_089288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8928E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08928E: rts
pub fn i_08928e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89290;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Depot_089290  ($089290, 228 B, enemies/bosses/scene5_airship_088a64.s)
/// $089290: move.w #$cb, d1
pub fn i_089290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89294;
    cpu.ir = 0x323C;
    let v1: u32 = 0xCB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089294: jsr $236e.l
pub fn i_089294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8929A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8929A);
    cpu.pc = 0x236E;
    16
}
/// $08929A: move.w #$30, $70(a6)
pub fn i_08929a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892A0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0892A0: move.b #$ff, $32(a6)
pub fn i_0892a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892A6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0892A6: move.b #$ff, $33(a6)
pub fn i_0892a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892AC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0892AC: move.b #$0, $3a(a6)
pub fn i_0892ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892B2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0892B2: move.w #$64, $66(a6)
pub fn i_0892b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0892B8: move.w #$0, $38(a6)
pub fn i_0892b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0892BE: lea.l $2ee2c6.l, a0
pub fn i_0892be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE2C6;
    20
}
/// $0892C4: jsr $28cd4.l
pub fn i_0892c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x892CA);
    cpu.pc = 0x28CD4;
    16
}
/// $0892CA: lea.l $892d0(pc), a1
pub fn i_0892ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x892D0;
    16
}
/// $0892CE: move.l a1, (a6)
pub fn i_0892ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0892D0: jsr $2783a.l
pub fn i_0892d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x892D6);
    cpu.pc = 0x2783A;
    16
}
/// $0892D6: jsr $28d70.l
pub fn i_0892d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x892DC);
    cpu.pc = 0x28D70;
    16
}
/// $0892DC: jsr $2870a.l
pub fn i_0892dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x892E2);
    cpu.pc = 0x2870A;
    16
}
/// $0892E2: bcc.w $892f8
pub fn i_0892e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x892F8; return 10; }
    8
}
/// $0892E6: lea.l $5e766.l, a0
pub fn i_0892e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0892EC: jsr $5e770.l
pub fn i_0892ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x892F2);
    cpu.pc = 0x5E770;
    16
}
/// $0892F2: bclr.b #$3, $13(a6)
pub fn i_0892f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892F8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0892F8: jsr $28758.l
pub fn i_0892f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x892FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x892FE);
    cpu.pc = 0x28758;
    16
}
/// $0892FE: bcc.w $89362
pub fn i_0892fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89302;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89362; return 10; }
    8
}
/// $089302: move.l #$500, d0
pub fn i_089302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89308;
    cpu.ir = 0x203C;
    let v1: u32 = 0x500;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $089308: jsr $51a28.l
pub fn i_089308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8930E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8930E);
    cpu.pc = 0x51A28;
    16
}
/// $08930E: move.w #$1035, d0
pub fn i_08930e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89312;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089312: jsr $2352.l
pub fn i_089312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89318;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89318);
    cpu.pc = 0x2352;
    16
}
/// $089318: lea.l $2ef2ea.l, a1
pub fn i_089318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8931E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF2EA;
    20
}
/// $08931E: jsr $77c7e.l
pub fn i_08931e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89324;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89324);
    cpu.pc = 0x77C7E;
    16
}
/// $089324: move.w #$c000, $38(a0)
pub fn i_089324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8932A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08932A: lea.l $7808a.l, a1
pub fn i_08932a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89330;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $089330: jsr $4ae.l
pub fn i_089330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89336);
    cpu.pc = 0x4AE;
    16
}
/// $089336: jsr $5dd22.l
pub fn i_089336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8933C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8933C);
    cpu.pc = 0x5DD22;
    16
}
/// $08933C: move.w #$c000, $38(a0)
pub fn i_08933c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89342;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089342: lea.l $f0742.l, a1
pub fn i_089342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89348;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xF0742;
    20
}
/// $089348: move.w #$82, d0
pub fn i_089348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8934C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08934C: move.b #$0, $82(a6)
pub fn i_08934c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89352;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089352: jsr $4429e.l
pub fn i_089352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89358;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89358);
    cpu.pc = 0x4429E;
    16
}
/// $089358: lea.l $89374(pc), a1
pub fn i_089358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8935C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89374;
    16
}
/// $08935C: move.l a1, (a6)
pub fn i_08935c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8935E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08935E: bra.w $89372
pub fn i_08935e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89362;
    cpu.ir = 0x6000;
    cpu.pc = 0x89372;
    10
}
/// $089362: jsr $4fa70.l
pub fn i_089362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89368;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89368);
    cpu.pc = 0x4FA70;
    16
}
/// $089368: bcc.w $89372
pub fn i_089368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8936C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89372; return 10; }
    8
}
/// $08936C: jmp $518.l
pub fn i_08936c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89372;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $089372: rts
pub fn i_089372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89374;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Depot_Wreck_089374  ($089374, 28 B, enemies/bosses/scene5_airship_088a64.s)
/// $089374: lea.l $2ef00c.l, a0
pub fn i_089374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8937A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF00C;
    20
}
/// $08937A: move.l a0, $4c(a6)
pub fn i_08937a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8937E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08937E: jsr $283ca.l
pub fn i_08937e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89384);
    cpu.pc = 0x283CA;
    16
}
/// $089384: jsr $283ca.l
pub fn i_089384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8938A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8938A);
    cpu.pc = 0x283CA;
    16
}
/// $08938A: jsr $283d8.l
pub fn i_08938a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89390;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89390);
    cpu.pc = 0x283D8;
    16
}
// ===== SetTaskHandler_089390  ($089390, 8 B, kernel/task_handlers.c)
/// $089390: lea.l $89398(pc), a1
pub fn i_089390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89394;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89398;
    16
}
/// $089394: move.l a1, (a6)
pub fn i_089394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89396;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089396: rts
pub fn i_089396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89398;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Depot_Idle_089398  ($089398, 18 B, enemies/bosses/scene5_airship_088a64.s)
/// $089398: jsr $283ca.l
pub fn i_089398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8939E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8939E);
    cpu.pc = 0x283CA;
    16
}
/// $08939E: jsr $283d8.l
pub fn i_08939e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x893A4);
    cpu.pc = 0x283D8;
    16
}
/// $0893A4: jmp $518.l
pub fn i_0893a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893AA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_0893aa  ($0893AA, 2 B, enemies/bosses/scene5_airship_088a64.s)
/// $0893AA: rts
pub fn i_0893aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_Wait_0893ac  ($0893AC, 336 B, enemies/bosses/scene5_airship_088a64.s)
/// $0893AC: movea.l $3c(a6), a1
pub fn i_0893ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893B0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0893B0: jsr $2942a.l
pub fn i_0893b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x893B6);
    cpu.pc = 0x2942A;
    16
}
/// $0893B6: move.w #$68, $22(a6)
pub fn i_0893b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x68;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0893BC: move.w #$b8, $24(a6)
pub fn i_0893bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893C2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0893C2: movea.l $c(a6), a0
pub fn i_0893c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893C6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0893C6: cmpi.b #$c, $106ece.l
pub fn i_0893c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893CE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $0893CE: bne.w $893de
pub fn i_0893ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x893DE; return 10; }
    8
}
/// $0893D2: move.w #$38, $22(a6)
pub fn i_0893d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0893D8: move.w #$110, $24(a6)
pub fn i_0893d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893DE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x110;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0893DE: lea.l $893e4(pc), a1
pub fn i_0893de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x893E4;
    16
}
/// $0893E2: move.l a1, (a6)
pub fn i_0893e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0893E4: jsr $8b8b0(pc)
pub fn i_0893e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x893E8);
    cpu.pc = 0x8B8B0;
    16
}
/// $0893E8: cmpi.w #$280, $106f54.l
pub fn i_0893e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893F0;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x106F54) as u32;
    sub_with_flags(cpu, v2, 0x280, Size::Word, false);
    20
}
/// $0893F0: bge.w $893f6
pub fn i_0893f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893F4;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x893F6; return 10; }
    8
}
/// $0893F4: rts
pub fn i_0893f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0893F6: move.w #$cf, d1
pub fn i_0893f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x893FA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xCF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0893FA: jsr $236e.l
pub fn i_0893fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89400;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89400);
    cpu.pc = 0x236E;
    16
}
/// $089400: move.w #$a0, $70(a6)
pub fn i_089400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89406;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089406: move.w #$0, $84(a6)
pub fn i_089406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8940C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08940C: move.b #$0, $83(a6)
pub fn i_08940c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89412;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089412: move.b #$ff, $32(a6)
pub fn i_089412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89418;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089418: move.b #$ff, $33(a6)
pub fn i_089418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8941E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08941E: move.b #$0, $3a(a6)
pub fn i_08941e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89424;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089424: move.b #$0, $20(a6)
pub fn i_089424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8942A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08942A: move.w #$0, $38(a6)
pub fn i_08942a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89430;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089430: jsr $267e2.l
pub fn i_089430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89436);
    cpu.pc = 0x267E2;
    16
}
/// $089436: move.w #$a0, $2a(a6)
pub fn i_089436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8943C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08943C: lea.l $2edf8e.l, a0
pub fn i_08943c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89442;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EDF8E;
    20
}
/// $089442: jsr $28cd4.l
pub fn i_089442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89448);
    cpu.pc = 0x28CD4;
    16
}
/// $089448: lea.l $89688(pc), a1
pub fn i_089448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8944C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89688;
    16
}
/// $08944C: jsr $4ae.l
pub fn i_08944c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89452);
    cpu.pc = 0x4AE;
    16
}
/// $089452: cmpi.b #$c, $106ece.l
pub fn i_089452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8945A;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $08945A: beq.w $89488
pub fn i_08945a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8945E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x89488; return 10; }
    8
}
/// $08945E: lea.l $897d2(pc), a1
pub fn i_08945e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89462;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x897D2;
    16
}
/// $089462: jsr $4ae.l
pub fn i_089462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89468);
    cpu.pc = 0x4AE;
    16
}
/// $089468: lea.l $896de(pc), a1
pub fn i_089468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8946C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x896DE;
    16
}
/// $08946C: jsr $4ae.l
pub fn i_08946c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89472;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89472);
    cpu.pc = 0x4AE;
    16
}
/// $089472: move.b #$0, $21(a0)
pub fn i_089472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89478;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089478: lea.l $896de(pc), a1
pub fn i_089478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8947C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x896DE;
    16
}
/// $08947C: jsr $4ae.l
pub fn i_08947c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89482;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89482);
    cpu.pc = 0x4AE;
    16
}
/// $089482: move.b #$1, $21(a0)
pub fn i_089482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89488;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089488: lea.l $9a300.l, a1
pub fn i_089488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8948E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x9A300;
    20
}
/// $08948E: jsr $4ae.l
pub fn i_08948e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89494;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89494);
    cpu.pc = 0x4AE;
    16
}
/// $089494: move.w #$0, $22(a0)
pub fn i_089494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8949A;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08949A: move.w #$40, $24(a0)
pub fn i_08949a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894A0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0894A0: move.l $106f50.l, d0
pub fn i_0894a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894A6;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0894A6: swap d0
pub fn i_0894a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894A8;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0894A8: move.w d0, $7c(a6)
pub fn i_0894a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894AC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0894AC: move.w #$1080, d0
pub fn i_0894ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894B0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1080;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0894B0: jsr $2352.l
pub fn i_0894b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x894B6);
    cpu.pc = 0x2352;
    16
}
/// $0894B6: move.w #$107f, d0
pub fn i_0894b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894BA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x107F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0894BA: jsr $2352.l
pub fn i_0894ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x894C0);
    cpu.pc = 0x2352;
    16
}
/// $0894C0: cmpi.b #$c, $106ece.l
pub fn i_0894c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894C8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $0894C8: beq.w $89504
pub fn i_0894c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x89504; return 10; }
    8
}
/// $0894CC: lea.l $894d2(pc), a1
pub fn i_0894cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x894D2;
    16
}
/// $0894D0: move.l a1, (a6)
pub fn i_0894d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0894D2: clr.b $10e39a.l
pub fn i_0894d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894D8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0894D8: bset.b #$6, $13(a6)
pub fn i_0894d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894DE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0894DE: jsr $27cee.l
pub fn i_0894de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x894E4);
    cpu.pc = 0x27CEE;
    16
}
/// $0894E4: jsr $8b8b0(pc)
pub fn i_0894e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x894E8);
    cpu.pc = 0x8B8B0;
    16
}
/// $0894E8: jsr $8b8e4(pc)
pub fn i_0894e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x894EC);
    cpu.pc = 0x8B8E4;
    16
}
/// $0894EC: jsr $28d70.l
pub fn i_0894ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x894F2);
    cpu.pc = 0x28D70;
    16
}
/// $0894F2: cmpi.w #$118, $24(a6)
pub fn i_0894f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894F8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x118, Size::Word, false);
    16
}
/// $0894F8: blt.w $89502
pub fn i_0894f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x894FC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x89502; return 10; }
    8
}
// ===== SetTaskHandler_0894fc  ($0894FC, 8 B, kernel/task_handlers.c)
/// $0894FC: lea.l $89504(pc), a1
pub fn i_0894fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89500;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89504;
    16
}
/// $089500: move.l a1, (a6)
pub fn i_089500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89502;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089502: rts
pub fn i_089502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89504;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_Landed_089504  ($089504, 192 B, enemies/bosses/scene5_airship_088a64.s)
/// $089504: lea.l $38f14.l, a1
pub fn i_089504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8950A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x38F14;
    20
}
/// $08950A: jsr $4ae.l
pub fn i_08950a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89510);
    cpu.pc = 0x4AE;
    16
}
/// $089510: jsr $5dd02.l
pub fn i_089510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89516;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89516);
    cpu.pc = 0x5DD02;
    16
}
/// $089516: addi.w #$9e, $22(a0)
pub fn i_089516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8951C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x9E, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08951C: addi.w #$40, $24(a0)
pub fn i_08951c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89522;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $089522: move.b #$1, $98(a0)
pub fn i_089522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89528;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089528: cmpi.b #$c, $106ece.l
pub fn i_089528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89530;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $089530: beq.w $89556
pub fn i_089530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89534;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x89556; return 10; }
    8
}
/// $089534: clr.b $10e39c.l
pub fn i_089534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8953A;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39C, r2 as u8);
    16
}
/// $08953A: move.w #$ffff, $106f5e.l
pub fn i_08953a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89542;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F5E, v1 as u16);
    16
}
/// $089542: move.l #$8000, $106f60.l
pub fn i_089542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8954C;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x8000;
    bus.write32(0x106F60, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08954C: move.l #$0, $106f64.l
pub fn i_08954c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89556;
    cpu.ir = 0x23FC;
    let v1: u32 = 0x0;
    bus.write32(0x106F64, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $089556: jsr $267e2.l
pub fn i_089556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8955C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8955C);
    cpu.pc = 0x267E2;
    16
}
/// $08955C: move.w #$40, $34(a6)
pub fn i_08955c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89562;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089562: move.w #$3e, $36(a6)
pub fn i_089562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89568;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089568: move.w #$68, $7e(a6)
pub fn i_089568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8956E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x68;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08956E: move.w #$118, $80(a6)
pub fn i_08956e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89574;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x118;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089574: lea.l $2eee24.l, a0
pub fn i_089574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8957A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EEE24;
    20
}
/// $08957A: move.l a0, $4c(a6)
pub fn i_08957a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8957E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08957E: jsr $283ca.l
pub fn i_08957e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89584;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89584);
    cpu.pc = 0x283CA;
    16
}
/// $089584: move.b #$ff, $20(a6)
pub fn i_089584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8958A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08958A: lea.l $89590(pc), a1
pub fn i_08958a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8958E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89590;
    16
}
/// $08958E: move.l a1, (a6)
pub fn i_08958e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89590;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089590: bset.b #$6, $13(a6)
pub fn i_089590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89596;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $089596: jsr $8b6a8(pc)
pub fn i_089596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8959A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8959A);
    cpu.pc = 0x8B6A8;
    16
}
/// $08959A: jsr $8b8b0(pc)
pub fn i_08959a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8959E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8959E);
    cpu.pc = 0x8B8B0;
    16
}
/// $08959E: jsr $8b8e4(pc)
pub fn i_08959e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x895A2);
    cpu.pc = 0x8B8E4;
    16
}
/// $0895A2: jsr $28d70.l
pub fn i_0895a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x895A8);
    cpu.pc = 0x28D70;
    16
}
/// $0895A8: jsr $283ca.l
pub fn i_0895a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x895AE);
    cpu.pc = 0x283CA;
    16
}
/// $0895AE: jsr $283d8.l
pub fn i_0895ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x895B4);
    cpu.pc = 0x283D8;
    16
}
/// $0895B4: move.l $106f50.l, d0
pub fn i_0895b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895BA;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0895BA: swap d0
pub fn i_0895ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895BC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0895BC: cmpi.w #$680, d0
pub fn i_0895bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895C0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x680, Size::Word, false);
    8
}
/// $0895C0: blt.w $895ca
pub fn i_0895c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895C4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x895CA; return 10; }
    8
}
// ===== SetTaskHandler_0895c4  ($0895C4, 8 B, kernel/task_handlers.c)
/// $0895C4: lea.l $895cc(pc), a1
pub fn i_0895c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x895CC;
    16
}
/// $0895C8: move.l a1, (a6)
pub fn i_0895c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0895CA: rts
pub fn i_0895ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_Hover_0895cc  ($0895CC, 42 B, enemies/bosses/scene5_airship_088a64.s)
/// $0895CC: move.w #$100, $80(a6)
pub fn i_0895cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0895D2: lea.l $895d8(pc), a1
pub fn i_0895d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x895D8;
    16
}
/// $0895D6: move.l a1, (a6)
pub fn i_0895d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0895D8: bset.b #$6, $13(a6)
pub fn i_0895d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895DE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0895DE: jsr $8b6a8(pc)
pub fn i_0895de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x895E2);
    cpu.pc = 0x8B6A8;
    16
}
/// $0895E2: jsr $8b8b0(pc)
pub fn i_0895e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x895E6);
    cpu.pc = 0x8B8B0;
    16
}
/// $0895E6: jsr $8b8e4(pc)
pub fn i_0895e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x895EA);
    cpu.pc = 0x8B8E4;
    16
}
/// $0895EA: jsr $28d70.l
pub fn i_0895ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x895F0);
    cpu.pc = 0x28D70;
    16
}
/// $0895F0: jsr $283ca.l
pub fn i_0895f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x895F6);
    cpu.pc = 0x283CA;
    16
}
// ===== JsrAbsThunk_0895f6  ($0895F6, 8 B, thunks/jsr_abs_thunks.c)
/// $0895F6: jsr $283d8.l
pub fn i_0895f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x895FC);
    cpu.pc = 0x283D8;
    16
}
/// $0895FC: rts
pub fn i_0895fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x895FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_Depart_0895fe  ($0895FE, 124 B, enemies/bosses/scene5_airship_088a64.s)
/// $0895FE: move.w #$1026, d0
pub fn i_0895fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89602;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1026;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089602: jsr $2352.l
pub fn i_089602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89608;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89608);
    cpu.pc = 0x2352;
    16
}
/// $089608: cmpi.b #$c, $106ece.l
pub fn i_089608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89610;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $089610: beq.w $8961a
pub fn i_089610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89614;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8961A; return 10; }
    8
}
/// $089614: clr.w $106f5e.l
pub fn i_089614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8961A;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F5E) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F5E, r2 as u16);
    16
}
/// $08961A: move.w #$160, $36(a6)
pub fn i_08961a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89620;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089620: lea.l $ffff.w, a0
pub fn i_089620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89624;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $089624: move.l a0, $4c(a6)
pub fn i_089624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89628;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $089628: jsr $283ca.l
pub fn i_089628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8962E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8962E);
    cpu.pc = 0x283CA;
    16
}
/// $08962E: lea.l $89634(pc), a1
pub fn i_08962e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89632;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89634;
    16
}
/// $089632: move.l a1, (a6)
pub fn i_089632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89634;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089634: jsr $2783a.l
pub fn i_089634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8963A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8963A);
    cpu.pc = 0x2783A;
    16
}
/// $08963A: jsr $8b718(pc)
pub fn i_08963a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8963E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8963E);
    cpu.pc = 0x8B718;
    16
}
/// $08963E: cmpi.b #$c, $106ece.l
pub fn i_08963e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89646;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $089646: bne.w $89652
pub fn i_089646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8964A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x89652; return 10; }
    8
}
/// $08964A: jsr $8b8b0(pc)
pub fn i_08964a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8964E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8964E);
    cpu.pc = 0x8B8B0;
    16
}
/// $08964E: bra.w $89656
pub fn i_08964e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89652;
    cpu.ir = 0x6000;
    cpu.pc = 0x89656;
    10
}
/// $089652: jsr $8b8ca(pc)
pub fn i_089652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89656;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x89656);
    cpu.pc = 0x8B8CA;
    16
}
/// $089656: jsr $28d70.l
pub fn i_089656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8965C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8965C);
    cpu.pc = 0x28D70;
    16
}
/// $08965C: cmpi.w #$3, $36(a6)
pub fn i_08965c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89662;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $089662: bhi.w $89670
pub fn i_089662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89666;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x89670; return 10; }
    8
}
/// $089666: move.w #$23, d0
pub fn i_089666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8966A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08966A: jsr $2352.l
pub fn i_08966a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89670;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89670);
    cpu.pc = 0x2352;
    16
}
/// $089670: jsr $4fa70.l
pub fn i_089670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89676;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89676);
    cpu.pc = 0x4FA70;
    16
}
/// $089676: bcc.w $89686
pub fn i_089676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8967A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89686; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_08967a  ($08967A, 14 B, kernel/jsr5b6_jmp518.c)
/// $08967A: jsr $5b6.l
pub fn i_08967a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89680;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89680);
    cpu.pc = 0x5B6;
    16
}
/// $089680: jmp $518.l
pub fn i_089680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89686;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $089686: rts
pub fn i_089686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89688;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_Attach_089688  ($089688, 78 B, enemies/bosses/scene5_airship_088a64.s)
/// $089688: move.w #$cf, d1
pub fn i_089688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8968C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xCF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08968C: jsr $236e.l
pub fn i_08968c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89692;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89692);
    cpu.pc = 0x236E;
    16
}
/// $089692: move.b #$ff, $32(a6)
pub fn i_089692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89698;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089698: move.b #$ff, $33(a6)
pub fn i_089698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8969E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08969E: move.b #$0, $3a(a6)
pub fn i_08969e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896A4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0896A4: lea.l $2edf9e.l, a0
pub fn i_0896a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EDF9E;
    20
}
/// $0896AA: jsr $28cd4.l
pub fn i_0896aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x896B0);
    cpu.pc = 0x28CD4;
    16
}
/// $0896B0: lea.l $896b6(pc), a1
pub fn i_0896b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x896B6;
    16
}
/// $0896B4: move.l a1, (a6)
pub fn i_0896b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0896B6: movea.l $c(a6), a0
pub fn i_0896b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896BA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0896BA: move.w $22(a0), $22(a6)
pub fn i_0896ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896C0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0896C0: move.w $24(a0), $24(a6)
pub fn i_0896c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896C6;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0896C6: move.w $38(a0), $38(a6)
pub fn i_0896c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896CC;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0896CC: cmpi.w #$0, $22(a6)
pub fn i_0896cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896D2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0896D2: blt.w $896dc
pub fn i_0896d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896D6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x896DC; return 10; }
    8
}
// ===== JsrAbsThunk_0896d6  ($0896D6, 8 B, thunks/jsr_abs_thunks.c)
/// $0896D6: jsr $28d70.l
pub fn i_0896d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x896DC);
    cpu.pc = 0x28D70;
    16
}
/// $0896DC: rts
pub fn i_0896dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_PlayerTracker_0896de  ($0896DE, 244 B, enemies/bosses/scene5_airship_088a64.s)
/// $0896DE: lea.l $100440.l, a0
pub fn i_0896de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $0896E4: move.l a0, $70(a6)
pub fn i_0896e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896E8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0896E8: move.w #$0, d0
pub fn i_0896e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896EC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0896EC: cmpi.b #$0, $21(a6)
pub fn i_0896ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896F2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0896F2: beq.w $89704
pub fn i_0896f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x89704; return 10; }
    8
}
/// $0896F6: lea.l $1004e0.l, a0
pub fn i_0896f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x896FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $0896FC: move.l a0, $70(a6)
pub fn i_0896fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89700;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $089700: move.w #$1, d0
pub fn i_089700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89704;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089704: jsr $5e3a2.l
pub fn i_089704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8970A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8970A);
    cpu.pc = 0x5E3A2;
    16
}
/// $08970A: bcs.w $89716
pub fn i_08970a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8970E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x89716; return 10; }
    8
}
/// $08970E: jmp $518.l
pub fn i_08970e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89714;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $089716: lea.l $8971c(pc), a1
pub fn i_089716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8971A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8971C;
    16
}
/// $08971A: move.l a1, (a6)
pub fn i_08971a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8971C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08971C: move.l $106f50.l, d0
pub fn i_08971c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89722;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $089722: swap d0
pub fn i_089722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89724;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $089724: cmpi.w #$678, d0
pub fn i_089724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89728;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x678, Size::Word, false);
    8
}
/// $089728: bgt.w $897ca
pub fn i_089728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8972C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x897CA; return 10; }
    8
}
/// $08972C: move.l $106f54.l, d0
pub fn i_08972c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89732;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F54);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $089732: swap d0
pub fn i_089732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89734;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $089734: cmpi.w #$280, d0
pub fn i_089734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89738;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x280, Size::Word, false);
    8
}
/// $089738: blt.w $897d0
pub fn i_089738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8973C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x897D0; return 10; }
    8
}
/// $08973C: movea.l $70(a6), a0
pub fn i_08973c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89740;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $089740: cmpi.w #$120, $24(a0)
pub fn i_089740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89746;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $089746: bgt.w $897d0
pub fn i_089746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8974A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x897D0; return 10; }
    8
}
/// $08974A: move.w #$1053, d0
pub fn i_08974a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8974E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1053;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08974E: jsr $2352.l
pub fn i_08974e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89754;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89754);
    cpu.pc = 0x2352;
    16
}
/// $089754: lea.l $78908.l, a1
pub fn i_089754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8975A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78908;
    20
}
/// $08975A: jsr $4ae.l
pub fn i_08975a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89760;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89760);
    cpu.pc = 0x4AE;
    16
}
/// $089760: movea.l $70(a6), a1
pub fn i_089760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89764;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $089764: move.w $22(a1), $22(a0)
pub fn i_089764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8976A;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08976A: subi.w #$18, $22(a0)
pub fn i_08976a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89770;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $089770: move.w $24(a1), $24(a0)
pub fn i_089770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89776;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $089776: move.w #$c000, $38(a0)
pub fn i_089776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8977C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08977C: lea.l $78908.l, a1
pub fn i_08977c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89782;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78908;
    20
}
/// $089782: jsr $4ae.l
pub fn i_089782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89788);
    cpu.pc = 0x4AE;
    16
}
/// $089788: movea.l $70(a6), a1
pub fn i_089788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8978C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08978C: move.w $22(a1), $22(a0)
pub fn i_08978c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89792;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $089792: addi.w #$18, $22(a0)
pub fn i_089792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89798;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $089798: move.w $24(a1), $24(a0)
pub fn i_089798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8979E;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08979E: move.w #$c000, $38(a0)
pub fn i_08979e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897A4;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0897A4: lea.l $8b1f2(pc), a1
pub fn i_0897a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1F2;
    16
}
/// $0897A8: jsr $4ae.l
pub fn i_0897a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x897AE);
    cpu.pc = 0x4AE;
    16
}
/// $0897AE: movea.l $70(a6), a1
pub fn i_0897ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897B2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0897B2: move.w $22(a1), $22(a0)
pub fn i_0897b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897B8;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0897B8: move.w $24(a1), $24(a0)
pub fn i_0897b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897BE;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0897BE: move.w #$c000, $38(a0)
pub fn i_0897be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897C4;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0897C4: move.w #$0, $72(a0)
pub fn i_0897c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897CA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0897CA: jmp $518.l
pub fn i_0897ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897D0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0897D0: rts
pub fn i_0897d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_Ground_0897d2  ($0897D2, 96 B, enemies/bosses/scene5_airship_088a64.s)
/// $0897D2: move.w #$d0, d1
pub fn i_0897d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897D6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0897D6: jsr $236e.l
pub fn i_0897d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x897DC);
    cpu.pc = 0x236E;
    16
}
/// $0897DC: move.b #$ff, $32(a6)
pub fn i_0897dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897E2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0897E2: move.b #$ff, $33(a6)
pub fn i_0897e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0897E8: move.b #$0, $3a(a6)
pub fn i_0897e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0897EE: move.w #$120, $24(a6)
pub fn i_0897ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897F4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0897F4: bset.b #$6, $12(a6)
pub fn i_0897f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x897FA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0897FA: move.w #$f000, $38(a6)
pub fn i_0897fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89800;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089800: lea.l $2edfae.l, a0
pub fn i_089800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89806;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EDFAE;
    20
}
/// $089806: jsr $28cd4.l
pub fn i_089806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8980C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8980C);
    cpu.pc = 0x28CD4;
    16
}
/// $08980C: lea.l $8983a(pc), a1
pub fn i_08980c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89810;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8983A;
    16
}
/// $089810: jsr $4ae.l
pub fn i_089810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89816;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89816);
    cpu.pc = 0x4AE;
    16
}
/// $089816: lea.l $8981c(pc), a1
pub fn i_089816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8981A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8981C;
    16
}
/// $08981A: move.l a1, (a6)
pub fn i_08981a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8981C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08981C: jsr $2783a.l
pub fn i_08981c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89822);
    cpu.pc = 0x2783A;
    16
}
/// $089822: movea.l $c(a6), a0
pub fn i_089822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89826;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $089826: move.w $22(a0), d0
pub fn i_089826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8982A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08982A: subi.w #$a, d0
pub fn i_08982a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8982E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xA, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08982E: move.w d0, $22(a6)
pub fn i_08982e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89832;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_089832  ($089832, 8 B, thunks/jsr_abs_thunks.c)
/// $089832: jsr $28d70.l
pub fn i_089832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89838);
    cpu.pc = 0x28D70;
    16
}
/// $089838: rts
pub fn i_089838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8983A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_GroundB_08983a  ($08983A, 90 B, enemies/bosses/scene5_airship_088a64.s)
/// $08983A: move.w #$d0, d1
pub fn i_08983a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8983E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08983E: jsr $236e.l
pub fn i_08983e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89844;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89844);
    cpu.pc = 0x236E;
    16
}
/// $089844: move.b #$ff, $32(a6)
pub fn i_089844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8984A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08984A: move.b #$ff, $33(a6)
pub fn i_08984a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89850;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089850: move.b #$0, $3a(a6)
pub fn i_089850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89856;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089856: bset.b #$6, $12(a6)
pub fn i_089856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8985C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08985C: move.w #$f000, $38(a6)
pub fn i_08985c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89862;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089862: lea.l $2edfec.l, a0
pub fn i_089862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89868;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EDFEC;
    20
}
/// $089868: jsr $28cd4.l
pub fn i_089868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8986E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8986E);
    cpu.pc = 0x28CD4;
    16
}
/// $08986E: lea.l $89874(pc), a1
pub fn i_08986e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89872;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89874;
    16
}
/// $089872: move.l a1, (a6)
pub fn i_089872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89874;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089874: movea.l $c(a6), a0
pub fn i_089874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89878;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $089878: move.w $22(a0), $22(a6)
pub fn i_089878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8987E;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08987E: move.w $24(a0), $24(a6)
pub fn i_08987e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89884;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $089884: move.w $38(a0), $38(a6)
pub fn i_089884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8988A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08988A: cmpi.w #$0, $22(a6)
pub fn i_08988a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89890;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $089890: blt.w $8989a
pub fn i_089890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89894;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8989A; return 10; }
    8
}
// ===== JsrAbsThunk_089894  ($089894, 8 B, thunks/jsr_abs_thunks.c)
/// $089894: jsr $28d70.l
pub fn i_089894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8989A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8989A);
    cpu.pc = 0x28D70;
    16
}
/// $08989A: rts
pub fn i_08989a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8989C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_Init_08989c  ($08989C, 56 B, enemies/bosses/scene5_airship_088a64.s)
/// $08989C: move.w #$d1, d1
pub fn i_08989c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898A0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0898A0: jsr $236e.l
pub fn i_0898a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x898A6);
    cpu.pc = 0x236E;
    16
}
/// $0898A6: move.b #$ff, $32(a6)
pub fn i_0898a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898AC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0898AC: move.b #$ff, $33(a6)
pub fn i_0898ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898B2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0898B2: move.b #$0, $3a(a6)
pub fn i_0898b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898B8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0898B8: move.w #$2, $34(a6)
pub fn i_0898b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0898BE: move.w #$1, $90(a6)
pub fn i_0898be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0898C4: move.w #$f, $92(a6)
pub fn i_0898c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898CA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0898CA: jsr $8f3a6.l
pub fn i_0898ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x898D0);
    cpu.pc = 0x8F3A6;
    16
}
/// $0898D0: move.b d0, $95(a6)
pub fn i_0898d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898D4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x95);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Turret8_Aim_0898d4  ($0898D4, 132 B, enemies/bosses/scene5_airship_088a64.s)
/// $0898D4: move.w $34(a6), d0
pub fn i_0898d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898D8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0898D8: andi.w #$7, d0
pub fn i_0898d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898DC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0898DC: movea.l #$2ee318, a0
pub fn i_0898dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898E2;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EE318;
    cpu.a[0] = v1;
    4
}
/// $0898E2: lsl.w #$2, d0
pub fn i_0898e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898E4;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0898E4: movea.l (a0, d0.w), a0
pub fn i_0898e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898E8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0898E8: cmpa.l #$ffffffff, a0
pub fn i_0898e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898EE;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0898EE: beq.w $898f8
pub fn i_0898ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x898F8; return 10; }
    8
}
/// $0898F2: jsr $28cd4.l
pub fn i_0898f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x898F8);
    cpu.pc = 0x28CD4;
    16
}
/// $0898F8: lea.l $898fe(pc), a1
pub fn i_0898f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x898FE;
    16
}
/// $0898FC: move.l a1, (a6)
pub fn i_0898fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x898FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0898FE: jsr $8b59e(pc)
pub fn i_0898fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89902;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x89902);
    cpu.pc = 0x8B59E;
    16
}
/// $089902: move.b #$0, $94(a6)
pub fn i_089902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89908;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089908: jsr $28d70.l
pub fn i_089908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8990E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8990E);
    cpu.pc = 0x28D70;
    16
}
/// $08990E: movea.l $c(a6), a0
pub fn i_08990e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89912;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $089912: cmpi.b #$0, $20(a0)
pub fn i_089912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89918;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $089918: beq.w $8995e
pub fn i_089918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8991C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8995E; return 10; }
    8
}
/// $08991C: move.w $22(a6), d0
pub fn i_08991c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89920;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $089920: cmpi.w #$fff0, d0
pub fn i_089920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89924;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFF0, Size::Word, false);
    8
}
/// $089924: blt.w $8995e
pub fn i_089924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89928;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8995E; return 10; }
    8
}
/// $089928: move.w $24(a6), d1
pub fn i_089928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8992C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08992C: move.b $95(a6), d2
pub fn i_08992c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89930;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x95);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $089930: jsr $8f3be.l
pub fn i_089930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89936);
    cpu.pc = 0x8F3BE;
    16
}
/// $089936: btst.b #$7, d4
pub fn i_089936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8993A;
    cpu.ir = 0x804;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08993A: bne.w $8995e
pub fn i_08993a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8993E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8995E; return 10; }
    8
}
/// $08993E: move.b d4, $21(a6)
pub fn i_08993e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89942;
    cpu.ir = 0x1D44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089942: lea.l $89a30(pc), a1
pub fn i_089942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89946;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89A30;
    16
}
/// $089946: jsr $6fe.l
pub fn i_089946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8994C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8994C);
    cpu.pc = 0x6FE;
    16
}
/// $08994C: jsr $5dd02.l
pub fn i_08994c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89952);
    cpu.pc = 0x5DD02;
    16
}
/// $089952: move.b $21(a6), $21(a0)
pub fn i_089952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89958;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== SetTaskHandler_089958  ($089958, 8 B, kernel/task_handlers.c)
/// $089958: lea.l $89960(pc), a1
pub fn i_089958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8995C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89960;
    16
}
/// $08995C: move.l a1, (a6)
pub fn i_08995c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8995E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08995E: rts
pub fn i_08995e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89960;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_Track_089960  ($089960, 104 B, enemies/bosses/scene5_airship_088a64.s)
/// $089960: lea.l $89966(pc), a1
pub fn i_089960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89964;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89966;
    16
}
/// $089964: move.l a1, (a6)
pub fn i_089964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89966;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089966: jsr $8b59e(pc)
pub fn i_089966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8996A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8996A);
    cpu.pc = 0x8B59E;
    16
}
/// $08996A: move.b #$1, $94(a6)
pub fn i_08996a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89970;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089970: jsr $8b5c8(pc)
pub fn i_089970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89974;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x89974);
    cpu.pc = 0x8B5C8;
    16
}
/// $089974: bcc.w $899a2
pub fn i_089974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89978;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x899A2; return 10; }
    8
}
/// $089978: move.b #$88, $94(a6)
pub fn i_089978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8997E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08997E: move.w $34(a6), d0
pub fn i_08997e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89982;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $089982: andi.w #$7, d0
pub fn i_089982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89986;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $089986: movea.l #$2ee3f0, a0
pub fn i_089986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8998C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EE3F0;
    cpu.a[0] = v1;
    4
}
/// $08998C: lsl.w #$2, d0
pub fn i_08998c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8998E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08998E: movea.l (a0, d0.w), a0
pub fn i_08998e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89992;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $089992: cmpa.l #$ffffffff, a0
pub fn i_089992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89998;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $089998: beq.w $899a2
pub fn i_089998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8999C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x899A2; return 10; }
    8
}
/// $08999C: jsr $28cd4.l
pub fn i_08999c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x899A2);
    cpu.pc = 0x28CD4;
    16
}
/// $0899A2: jsr $28d70.l
pub fn i_0899a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x899A8);
    cpu.pc = 0x28D70;
    16
}
/// $0899A8: movea.l #$100440, a0
pub fn i_0899a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899AE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x100440;
    cpu.a[0] = v1;
    4
}
/// $0899AE: cmpi.b #$0, $21(a6)
pub fn i_0899ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899B4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0899B4: beq.w $899be
pub fn i_0899b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x899BE; return 10; }
    8
}
/// $0899B8: movea.l #$1004e0, a0
pub fn i_0899b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899BE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x1004E0;
    cpu.a[0] = v1;
    4
}
/// $0899BE: btst.b #$0, $13(a0)
pub fn i_0899be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899C4;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0899C4: beq.w $899d0
pub fn i_0899c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x899D0; return 10; }
    8
}
// ===== SetTaskHandler_0899c8  ($0899C8, 8 B, kernel/task_handlers.c)
/// $0899C8: lea.l $89a04(pc), a1
pub fn i_0899c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89A04;
    16
}
/// $0899CC: move.l a1, (a6)
pub fn i_0899cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0899CE: rts
pub fn i_0899ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_Fire_0899d0  ($0899D0, 44 B, enemies/bosses/scene5_airship_088a64.s)
/// $0899D0: jsr $8b626(pc)
pub fn i_0899d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x899D4);
    cpu.pc = 0x8B626;
    16
}
/// $0899D4: move.w $22(a6), d0
pub fn i_0899d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899D8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0899D8: cmpi.w #$fff0, d0
pub fn i_0899d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899DC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFF0, Size::Word, false);
    8
}
/// $0899DC: blt.w $899f6
pub fn i_0899dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899E0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x899F6; return 10; }
    8
}
/// $0899E0: move.w $24(a6), d1
pub fn i_0899e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899E4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0899E4: move.b $95(a6), d2
pub fn i_0899e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899E8;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x95);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0899E8: jsr $8f3be.l
pub fn i_0899e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x899EE);
    cpu.pc = 0x8F3BE;
    16
}
/// $0899EE: btst.b #$7, d4
pub fn i_0899ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899F2;
    cpu.ir = 0x804;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0899F2: beq.w $89a02
pub fn i_0899f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x89A02; return 10; }
    8
}
/// $0899F6: move.b #$ff, $94(a6)
pub fn i_0899f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x899FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetTaskHandler_0899fc  ($0899FC, 8 B, kernel/task_handlers.c)
/// $0899FC: lea.l $898d4(pc), a1
pub fn i_0899fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x898D4;
    16
}
/// $089A00: move.l a1, (a6)
pub fn i_089a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089A02: rts
pub fn i_089a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_Cooldown_089a04  ($089A04, 36 B, enemies/bosses/scene5_airship_088a64.s)
/// $089A04: move.b #$ff, $94(a6)
pub fn i_089a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A0A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089A0A: move.w #$a, $72(a6)
pub fn i_089a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A10;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089A10: lea.l $89a16(pc), a1
pub fn i_089a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89A16;
    16
}
/// $089A14: move.l a1, (a6)
pub fn i_089a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089A16: jsr $8b59e(pc)
pub fn i_089a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x89A1A);
    cpu.pc = 0x8B59E;
    16
}
/// $089A1A: jsr $28d70.l
pub fn i_089a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89A20);
    cpu.pc = 0x28D70;
    16
}
/// $089A20: subq.w #$1, $72(a6)
pub fn i_089a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A24;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $089A24: bne.w $89a2e
pub fn i_089a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A28;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x89A2E; return 10; }
    8
}
// ===== SetTaskHandler_089a28  ($089A28, 8 B, kernel/task_handlers.c)
/// $089A28: lea.l $898d4(pc), a1
pub fn i_089a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x898D4;
    16
}
/// $089A2C: move.l a1, (a6)
pub fn i_089a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089A2E: rts
pub fn i_089a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_Barrel_089a30  ($089A30, 144 B, enemies/bosses/scene5_airship_088a64.s)
/// $089A30: move.b $21(a6), d4
pub fn i_089a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A34;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $089A34: jsr $8f69c.l
pub fn i_089a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89A3A);
    cpu.pc = 0x8F69C;
    16
}
/// $089A3A: jsr $236e.l
pub fn i_089a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89A40);
    cpu.pc = 0x236E;
    16
}
/// $089A40: move.b #$ff, $32(a6)
pub fn i_089a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089A46: move.b #$ff, $33(a6)
pub fn i_089a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A4C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089A4C: move.b #$0, $3a(a6)
pub fn i_089a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089A52: movea.l $c(a6), a0
pub fn i_089a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A56;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $089A56: move.w $34(a0), d0
pub fn i_089a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A5A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $089A5A: andi.w #$7, d0
pub fn i_089a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A5E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $089A5E: movea.l #$2ee4c8, a0
pub fn i_089a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A64;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EE4C8;
    cpu.a[0] = v1;
    4
}
/// $089A64: lsl.w #$2, d0
pub fn i_089a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A66;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $089A66: movea.l (a0, d0.w), a0
pub fn i_089a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A6A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $089A6A: cmpa.l #$ffffffff, a0
pub fn i_089a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A70;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $089A70: beq.w $89a7a
pub fn i_089a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A74;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x89A7A; return 10; }
    8
}
/// $089A74: jsr $28cd4.l
pub fn i_089a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89A7A);
    cpu.pc = 0x28CD4;
    16
}
/// $089A7A: lea.l $89a80(pc), a1
pub fn i_089a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89A80;
    16
}
/// $089A7E: move.l a1, (a6)
pub fn i_089a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089A80: jsr $8b5b6(pc)
pub fn i_089a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A84;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x89A84);
    cpu.pc = 0x8B5B6;
    16
}
/// $089A84: movea.l $c(a6), a0
pub fn i_089a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A88;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $089A88: cmpi.b #$ff, $94(a0)
pub fn i_089a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A8E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x94);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $089A8E: beq.w $89ac8
pub fn i_089a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x89AC8; return 10; }
    8
}
/// $089A92: cmpi.b #$88, $94(a0)
pub fn i_089a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A98;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x94);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $089A98: bne.w $89ac0
pub fn i_089a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89A9C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x89AC0; return 10; }
    8
}
/// $089A9C: move.w $34(a0), d0
pub fn i_089a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AA0;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $089AA0: andi.w #$7, d0
pub fn i_089aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AA4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $089AA4: movea.l #$2ee4c8, a0
pub fn i_089aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AAA;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EE4C8;
    cpu.a[0] = v1;
    4
}
/// $089AAA: lsl.w #$2, d0
pub fn i_089aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AAC;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $089AAC: movea.l (a0, d0.w), a0
pub fn i_089aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AB0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $089AB0: cmpa.l #$ffffffff, a0
pub fn i_089ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AB6;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $089AB6: beq.w $89ac0
pub fn i_089ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89ABA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x89AC0; return 10; }
    8
}
/// $089ABA: jsr $28cd4.l
pub fn i_089aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89AC0);
    cpu.pc = 0x28CD4;
    16
}
// ===== JsrAbsThunk_089ac0  ($089AC0, 8 B, thunks/jsr_abs_thunks.c)
/// $089AC0: jsr $28d70.l
pub fn i_089ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89AC6);
    cpu.pc = 0x28D70;
    16
}
/// $089AC6: rts
pub fn i_089ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_089ac8  ($089AC8, 8 B, kernel/jmp_to_scheduler.c)
/// $089AC8: jmp $518.l
pub fn i_089ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89ACE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Turret8_Casing_089ad0  ($089AD0, 222 B, enemies/bosses/scene5_airship_088a64.s)
/// $089AD0: move.w #$d2, d1
pub fn i_089ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AD4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089AD4: jsr $236e.l
pub fn i_089ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89ADA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89ADA);
    cpu.pc = 0x236E;
    16
}
/// $089ADA: move.b #$ff, $32(a6)
pub fn i_089ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AE0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089AE0: move.b #$ff, $33(a6)
pub fn i_089ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089AE6: move.b #$0, $3a(a6)
pub fn i_089ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AEC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089AEC: move.w #$c000, $38(a6)
pub fn i_089aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AF2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089AF2: lea.l $2ee4e8.l, a0
pub fn i_089af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AF8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE4E8;
    20
}
/// $089AF8: jsr $28cd4.l
pub fn i_089af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89AFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89AFE);
    cpu.pc = 0x28CD4;
    16
}
/// $089AFE: jsr $267e2.l
pub fn i_089afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89B04);
    cpu.pc = 0x267E2;
    16
}
/// $089B04: move.w #$380, $2a(a6)
pub fn i_089b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x380;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089B0A: jsr $5e9b6.l
pub fn i_089b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89B10);
    cpu.pc = 0x5E9B6;
    16
}
/// $089B10: andi.w #$ff, d0
pub fn i_089b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B14;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $089B14: add.w d0, $2a(a6)
pub fn i_089b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B18;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $089B18: move.w #$ffc0, $2e(a6)
pub fn i_089b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089B1E: move.w #$40, $28(a6)
pub fn i_089b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089B24: add.w d0, $28(a6)
pub fn i_089b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B28;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $089B28: lea.l $89b2e(pc), a1
pub fn i_089b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89B2E;
    16
}
/// $089B2C: move.l a1, (a6)
pub fn i_089b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089B2E: jsr $27cee.l
pub fn i_089b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89B34);
    cpu.pc = 0x27CEE;
    16
}
/// $089B34: scs.b $5c(a6)
pub fn i_089b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B38;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $089B38: jsr $28d70.l
pub fn i_089b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89B3E);
    cpu.pc = 0x28D70;
    16
}
/// $089B3E: move.w $2a(a6), d0
pub fn i_089b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B42;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $089B42: bpl.w $89b96
pub fn i_089b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B46;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x89B96; return 10; }
    8
}
/// $089B46: move.b $5c(a6), d0
pub fn i_089b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B4A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $089B4A: beq.w $89b96
pub fn i_089b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B4E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x89B96; return 10; }
    8
}
/// $089B4E: jsr $5e9b6.l
pub fn i_089b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89B54);
    cpu.pc = 0x5E9B6;
    16
}
/// $089B54: andi.b #$f, d0
pub fn i_089b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B58;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $089B58: bne.w $89b66
pub fn i_089b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B5C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x89B66; return 10; }
    8
}
/// $089B5C: neg.w $28(a6)
pub fn i_089b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B60;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $089B60: eori.b #$1, $3a(a6)
pub fn i_089b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B66;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $089B66: move.w $2a(a6), d0
pub fn i_089b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B6A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $089B6A: jsr $8b58e(pc)
pub fn i_089b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B6E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x89B6E);
    cpu.pc = 0x8B58E;
    16
}
/// $089B6E: asr.w #$1, d0
pub fn i_089b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B70;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $089B70: move.w d0, $2a(a6)
pub fn i_089b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B74;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089B74: cmpi.w #$80, d0
pub fn i_089b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B78;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $089B78: bgt.w $89b96
pub fn i_089b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B7C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x89B96; return 10; }
    8
}
/// $089B7C: lea.l $89bae(pc), a1
pub fn i_089b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89BAE;
    16
}
/// $089B80: move.l a1, (a6)
pub fn i_089b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089B82: jsr $5e9b6.l
pub fn i_089b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89B88);
    cpu.pc = 0x5E9B6;
    16
}
/// $089B88: andi.w #$7f, d0
pub fn i_089b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B8C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $089B8C: bne.w $89b96
pub fn i_089b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B90;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x89B96; return 10; }
    8
}
/// $089B90: lea.l $89bd8(pc), a1
pub fn i_089b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89BD8;
    16
}
/// $089B94: move.l a1, (a6)
pub fn i_089b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089B96: lea.l $2ef7b2.l, a0
pub fn i_089b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89B9C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7B2;
    20
}
/// $089B9C: jsr $5dd56.l
pub fn i_089b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89BA2);
    cpu.pc = 0x5DD56;
    16
}
/// $089BA2: bcc.w $89bac
pub fn i_089ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BA6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89BAC; return 10; }
    8
}
/// $089BA6: jmp $518.l
pub fn i_089ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BAC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $089BAC: rts
pub fn i_089bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BAE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_Casing_Rest_089bae  ($089BAE, 42 B, enemies/bosses/scene5_airship_088a64.s)
/// $089BAE: lea.l $89bb4(pc), a1
pub fn i_089bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89BB4;
    16
}
/// $089BB2: move.l a1, (a6)
pub fn i_089bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089BB4: jsr $27cee.l
pub fn i_089bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89BBA);
    cpu.pc = 0x27CEE;
    16
}
/// $089BBA: jsr $28d70.l
pub fn i_089bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89BC0);
    cpu.pc = 0x28D70;
    16
}
/// $089BC0: lea.l $2ef7b2.l, a0
pub fn i_089bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BC6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7B2;
    20
}
/// $089BC6: jsr $5dd56.l
pub fn i_089bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89BCC);
    cpu.pc = 0x5DD56;
    16
}
/// $089BCC: bcc.w $89bd6
pub fn i_089bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BD0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89BD6; return 10; }
    8
}
/// $089BD0: jmp $518.l
pub fn i_089bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BD6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $089BD6: rts
pub fn i_089bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_Casing_Fade_089bd8  ($089BD8, 48 B, enemies/bosses/scene5_airship_088a64.s)
/// $089BD8: lea.l $89bde(pc), a1
pub fn i_089bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89BDE;
    16
}
/// $089BDC: move.l a1, (a6)
pub fn i_089bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BDE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089BDE: jsr $2783a.l
pub fn i_089bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89BE4);
    cpu.pc = 0x2783A;
    16
}
/// $089BE4: move.b #$7f, $44(a6)
pub fn i_089be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BEA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089BEA: jsr $28d70.l
pub fn i_089bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89BF0);
    cpu.pc = 0x28D70;
    16
}
/// $089BF0: lea.l $2ef7b2.l, a0
pub fn i_089bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BF6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7B2;
    20
}
/// $089BF6: jsr $5dd56.l
pub fn i_089bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89BFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89BFC);
    cpu.pc = 0x5DD56;
    16
}
/// $089BFC: bcc.w $89c06
pub fn i_089bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89C06; return 10; }
    8
}
/// $089C00: jmp $518.l
pub fn i_089c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C06;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $089C06: rts
pub fn i_089c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Crate_089c08  ($089C08, 300 B, enemies/bosses/scene5_airship_088a64.s)
/// $089C08: movea.l $3c(a6), a1
pub fn i_089c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C0C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $089C0C: jsr $2942a.l
pub fn i_089c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89C12);
    cpu.pc = 0x2942A;
    16
}
/// $089C12: move.b #$0, $21(a6)
pub fn i_089c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C18;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089C18: bra.w $89c6c
pub fn i_089c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x89C6C;
    10
}
/// $089C6C: move.w #$d6, d1
pub fn i_089c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C70;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089C70: jsr $236e.l
pub fn i_089c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89C76);
    cpu.pc = 0x236E;
    16
}
/// $089C76: move.w #$40, $70(a6)
pub fn i_089c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C7C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089C7C: move.b #$ff, $32(a6)
pub fn i_089c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C82;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089C82: move.b #$ff, $33(a6)
pub fn i_089c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C88;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089C88: move.b #$0, $3a(a6)
pub fn i_089c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C8E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $089C8E: move.w #$14, $66(a6)
pub fn i_089c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C94;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089C94: move.w #$8000, $38(a6)
pub fn i_089c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89C9A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089C9A: jsr $267e2.l
pub fn i_089c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89CA0);
    cpu.pc = 0x267E2;
    16
}
/// $089CA0: jsr $27cee.l
pub fn i_089ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89CA6);
    cpu.pc = 0x27CEE;
    16
}
/// $089CA6: clr.l d0
pub fn i_089ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CA8;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $089CA8: move.b $21(a6), d0
pub fn i_089ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CAC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $089CAC: movea.l #$2ee13c, a0
pub fn i_089cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CB2;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EE13C;
    cpu.a[0] = v1;
    4
}
/// $089CB2: lsl.w #$2, d0
pub fn i_089cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CB4;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $089CB4: movea.l (a0, d0.w), a0
pub fn i_089cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CB8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $089CB8: cmpa.l #$ffffffff, a0
pub fn i_089cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CBE;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $089CBE: beq.w $89cc8
pub fn i_089cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CC2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x89CC8; return 10; }
    8
}
/// $089CC2: jsr $28cd4.l
pub fn i_089cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89CC8);
    cpu.pc = 0x28CD4;
    16
}
/// $089CC8: lea.l $89cce(pc), a1
pub fn i_089cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89CCE;
    16
}
/// $089CCC: move.l a1, (a6)
pub fn i_089ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089CCE: jsr $2783a.l
pub fn i_089cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89CD4);
    cpu.pc = 0x2783A;
    16
}
/// $089CD4: jsr $28d70.l
pub fn i_089cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89CDA);
    cpu.pc = 0x28D70;
    16
}
/// $089CDA: jsr $2870a.l
pub fn i_089cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89CE0);
    cpu.pc = 0x2870A;
    16
}
/// $089CE0: bcc.w $89cf6
pub fn i_089ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CE4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89CF6; return 10; }
    8
}
/// $089CE4: lea.l $5e766.l, a0
pub fn i_089ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $089CEA: jsr $5e770.l
pub fn i_089cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89CF0);
    cpu.pc = 0x5E770;
    16
}
/// $089CF0: bclr.b #$3, $13(a6)
pub fn i_089cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CF6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $089CF6: jsr $28758.l
pub fn i_089cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89CFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89CFC);
    cpu.pc = 0x28758;
    16
}
/// $089CFC: bcc.w $89d22
pub fn i_089cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89D00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89D22; return 10; }
    8
}
/// $089D00: move.w #$102e, d0
pub fn i_089d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89D04;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $089D04: jsr $2352.l
pub fn i_089d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89D0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89D0A);
    cpu.pc = 0x2352;
    16
}
/// $089D0A: lea.l $2ef1d0.l, a1
pub fn i_089d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89D10;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF1D0;
    20
}
/// $089D10: jsr $77c7e.l
pub fn i_089d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89D16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89D16);
    cpu.pc = 0x77C7E;
    16
}
/// $089D16: move.w #$f000, $38(a0)
pub fn i_089d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89D1C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $089D1C: lea.l $89d34(pc), a1
pub fn i_089d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89D20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89D34;
    16
}
/// $089D20: move.l a1, (a6)
pub fn i_089d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89D22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $089D22: jsr $4fa70.l
pub fn i_089d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89D28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x89D28);
    cpu.pc = 0x4FA70;
    16
}
/// $089D28: bcc.w $89d32
pub fn i_089d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89D2C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x89D32; return 10; }
    8
}
/// $089D2C: jmp $518.l
pub fn i_089d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89D32;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $089D32: rts
pub fn i_089d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x89D34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x087FD0, i_087fd0),
    (0x087FD6, i_087fd6),
    (0x087FDA, i_087fda),
    (0x087FE0, i_087fe0),
    (0x087FE6, i_087fe6),
    (0x087FEA, i_087fea),
    (0x087FF0, i_087ff0),
    (0x087FF6, i_087ff6),
    (0x087FFA, i_087ffa),
    (0x088000, i_088000),
    (0x088006, i_088006),
    (0x08800C, i_08800c),
    (0x08800E, i_08800e),
    (0x088014, i_088014),
    (0x08801A, i_08801a),
    (0x088020, i_088020),
    (0x088024, i_088024),
    (0x088028, i_088028),
    (0x08802E, i_08802e),
    (0x088034, i_088034),
    (0x08803A, i_08803a),
    (0x08803E, i_08803e),
    (0x088042, i_088042),
    (0x088048, i_088048),
    (0x08804E, i_08804e),
    (0x088054, i_088054),
    (0x088058, i_088058),
    (0x08805C, i_08805c),
    (0x088062, i_088062),
    (0x088068, i_088068),
    (0x08806E, i_08806e),
    (0x088072, i_088072),
    (0x088076, i_088076),
    (0x08807A, i_08807a),
    (0x088080, i_088080),
    (0x088086, i_088086),
    (0x08808C, i_08808c),
    (0x088092, i_088092),
    (0x088098, i_088098),
    (0x08809E, i_08809e),
    (0x0880A4, i_0880a4),
    (0x0880AA, i_0880aa),
    (0x0880AE, i_0880ae),
    (0x0880B0, i_0880b0),
    (0x0880B6, i_0880b6),
    (0x0880BC, i_0880bc),
    (0x0880C2, i_0880c2),
    (0x0880C6, i_0880c6),
    (0x0880CC, i_0880cc),
    (0x0880D2, i_0880d2),
    (0x0880D8, i_0880d8),
    (0x0880DE, i_0880de),
    (0x0880E2, i_0880e2),
    (0x0880E6, i_0880e6),
    (0x0880EC, i_0880ec),
    (0x0880F0, i_0880f0),
    (0x0880F4, i_0880f4),
    (0x0880F8, i_0880f8),
    (0x0880FE, i_0880fe),
    (0x088102, i_088102),
    (0x088108, i_088108),
    (0x08810C, i_08810c),
    (0x088112, i_088112),
    (0x088114, i_088114),
    (0x088118, i_088118),
    (0x08811E, i_08811e),
    (0x088122, i_088122),
    (0x088128, i_088128),
    (0x08812E, i_08812e),
    (0x088134, i_088134),
    (0x08813A, i_08813a),
    (0x088140, i_088140),
    (0x088146, i_088146),
    (0x08814C, i_08814c),
    (0x088152, i_088152),
    (0x088156, i_088156),
    (0x088158, i_088158),
    (0x08815E, i_08815e),
    (0x088164, i_088164),
    (0x088168, i_088168),
    (0x08816E, i_08816e),
    (0x088174, i_088174),
    (0x08817A, i_08817a),
    (0x088180, i_088180),
    (0x088186, i_088186),
    (0x08818C, i_08818c),
    (0x088190, i_088190),
    (0x088194, i_088194),
    (0x088196, i_088196),
    (0x08819C, i_08819c),
    (0x0881A0, i_0881a0),
    (0x0881A6, i_0881a6),
    (0x0881A8, i_0881a8),
    (0x0881AE, i_0881ae),
    (0x0881B4, i_0881b4),
    (0x0881B8, i_0881b8),
    (0x0881BA, i_0881ba),
    (0x0881C0, i_0881c0),
    (0x0881C6, i_0881c6),
    (0x0881CA, i_0881ca),
    (0x0881CE, i_0881ce),
    (0x0881D0, i_0881d0),
    (0x0881D6, i_0881d6),
    (0x0881DA, i_0881da),
    (0x0881E0, i_0881e0),
    (0x0881E6, i_0881e6),
    (0x0881EC, i_0881ec),
    (0x0881F2, i_0881f2),
    (0x0881F8, i_0881f8),
    (0x0881FE, i_0881fe),
    (0x088202, i_088202),
    (0x088208, i_088208),
    (0x08820A, i_08820a),
    (0x088210, i_088210),
    (0x088214, i_088214),
    (0x08821A, i_08821a),
    (0x088220, i_088220),
    (0x088224, i_088224),
    (0x088226, i_088226),
    (0x08822C, i_08822c),
    (0x088232, i_088232),
    (0x088238, i_088238),
    (0x08823C, i_08823c),
    (0x088242, i_088242),
    (0x088248, i_088248),
    (0x08824E, i_08824e),
    (0x088254, i_088254),
    (0x088258, i_088258),
    (0x08825C, i_08825c),
    (0x08825E, i_08825e),
    (0x088264, i_088264),
    (0x088268, i_088268),
    (0x08826E, i_08826e),
    (0x088270, i_088270),
    (0x088276, i_088276),
    (0x08827C, i_08827c),
    (0x088280, i_088280),
    (0x088282, i_088282),
    (0x088288, i_088288),
    (0x08828E, i_08828e),
    (0x088294, i_088294),
    (0x088298, i_088298),
    (0x08829E, i_08829e),
    (0x0882A4, i_0882a4),
    (0x0882AA, i_0882aa),
    (0x0882B0, i_0882b0),
    (0x0882B4, i_0882b4),
    (0x0882B8, i_0882b8),
    (0x0882BA, i_0882ba),
    (0x0882C0, i_0882c0),
    (0x0882C4, i_0882c4),
    (0x0882CA, i_0882ca),
    (0x0882CC, i_0882cc),
    (0x0882D2, i_0882d2),
    (0x0882D8, i_0882d8),
    (0x0882DC, i_0882dc),
    (0x0882DE, i_0882de),
    (0x0882E4, i_0882e4),
    (0x0882EA, i_0882ea),
    (0x0882F0, i_0882f0),
    (0x0882F4, i_0882f4),
    (0x0882FA, i_0882fa),
    (0x088300, i_088300),
    (0x088306, i_088306),
    (0x08830C, i_08830c),
    (0x088310, i_088310),
    (0x088314, i_088314),
    (0x088316, i_088316),
    (0x08831C, i_08831c),
    (0x088320, i_088320),
    (0x088326, i_088326),
    (0x088328, i_088328),
    (0x08832E, i_08832e),
    (0x088334, i_088334),
    (0x088338, i_088338),
    (0x08833A, i_08833a),
    (0x088340, i_088340),
    (0x088346, i_088346),
    (0x08834C, i_08834c),
    (0x088350, i_088350),
    (0x088356, i_088356),
    (0x08835C, i_08835c),
    (0x088362, i_088362),
    (0x088368, i_088368),
    (0x08836C, i_08836c),
    (0x088372, i_088372),
    (0x088378, i_088378),
    (0x08837C, i_08837c),
    (0x08837E, i_08837e),
    (0x088384, i_088384),
    (0x088388, i_088388),
    (0x08838E, i_08838e),
    (0x088390, i_088390),
    (0x088396, i_088396),
    (0x08839C, i_08839c),
    (0x0883A0, i_0883a0),
    (0x0883A2, i_0883a2),
    (0x0883A6, i_0883a6),
    (0x0883A8, i_0883a8),
    (0x0883AE, i_0883ae),
    (0x0883B4, i_0883b4),
    (0x0883BA, i_0883ba),
    (0x0883BE, i_0883be),
    (0x0883C4, i_0883c4),
    (0x0883CA, i_0883ca),
    (0x0883D0, i_0883d0),
    (0x0883D6, i_0883d6),
    (0x0883E4, i_0883e4),
    (0x0883EC, i_0883ec),
    (0x0883F2, i_0883f2),
    (0x0883F8, i_0883f8),
    (0x0883FC, i_0883fc),
    (0x088400, i_088400),
    (0x088404, i_088404),
    (0x088408, i_088408),
    (0x08840E, i_08840e),
    (0x088412, i_088412),
    (0x088416, i_088416),
    (0x08841A, i_08841a),
    (0x08841C, i_08841c),
    (0x088420, i_088420),
    (0x088424, i_088424),
    (0x088428, i_088428),
    (0x08842E, i_08842e),
    (0x088432, i_088432),
    (0x088436, i_088436),
    (0x088438, i_088438),
    (0x08843C, i_08843c),
    (0x088440, i_088440),
    (0x088444, i_088444),
    (0x088448, i_088448),
    (0x08844A, i_08844a),
    (0x08844E, i_08844e),
    (0x088450, i_088450),
    (0x088456, i_088456),
    (0x08845A, i_08845a),
    (0x08845E, i_08845e),
    (0x088460, i_088460),
    (0x088464, i_088464),
    (0x088468, i_088468),
    (0x08846A, i_08846a),
    (0x08846E, i_08846e),
    (0x088472, i_088472),
    (0x088476, i_088476),
    (0x08847A, i_08847a),
    (0x088480, i_088480),
    (0x088482, i_088482),
    (0x088486, i_088486),
    (0x08848A, i_08848a),
    (0x08848C, i_08848c),
    (0x088490, i_088490),
    (0x088494, i_088494),
    (0x088496, i_088496),
    (0x08849A, i_08849a),
    (0x08849E, i_08849e),
    (0x0884A4, i_0884a4),
    (0x0884A6, i_0884a6),
    (0x0884AA, i_0884aa),
    (0x0884B0, i_0884b0),
    (0x0884B6, i_0884b6),
    (0x0884BC, i_0884bc),
    (0x0884C2, i_0884c2),
    (0x0884C8, i_0884c8),
    (0x0884CE, i_0884ce),
    (0x0884D2, i_0884d2),
    (0x0884D8, i_0884d8),
    (0x0884DE, i_0884de),
    (0x0884E2, i_0884e2),
    (0x0884E6, i_0884e6),
    (0x0884EC, i_0884ec),
    (0x0884F2, i_0884f2),
    (0x0884F8, i_0884f8),
    (0x0884FE, i_0884fe),
    (0x088504, i_088504),
    (0x08850A, i_08850a),
    (0x08850E, i_08850e),
    (0x088514, i_088514),
    (0x08851A, i_08851a),
    (0x08851E, i_08851e),
    (0x088522, i_088522),
    (0x088528, i_088528),
    (0x08852E, i_08852e),
    (0x088534, i_088534),
    (0x08853A, i_08853a),
    (0x088540, i_088540),
    (0x088546, i_088546),
    (0x08854A, i_08854a),
    (0x088550, i_088550),
    (0x088556, i_088556),
    (0x08855A, i_08855a),
    (0x08855E, i_08855e),
    (0x088564, i_088564),
    (0x08856A, i_08856a),
    (0x088570, i_088570),
    (0x088576, i_088576),
    (0x08857C, i_08857c),
    (0x088580, i_088580),
    (0x088586, i_088586),
    (0x08858A, i_08858a),
    (0x088590, i_088590),
    (0x088596, i_088596),
    (0x08859C, i_08859c),
    (0x0885A2, i_0885a2),
    (0x0885A8, i_0885a8),
    (0x0885AC, i_0885ac),
    (0x0885B2, i_0885b2),
    (0x0885B4, i_0885b4),
    (0x0885B8, i_0885b8),
    (0x0885BE, i_0885be),
    (0x0885C4, i_0885c4),
    (0x0885CA, i_0885ca),
    (0x0885D0, i_0885d0),
    (0x0885D6, i_0885d6),
    (0x0885DC, i_0885dc),
    (0x0885E0, i_0885e0),
    (0x0885E6, i_0885e6),
    (0x0885EC, i_0885ec),
    (0x0885F0, i_0885f0),
    (0x0885F4, i_0885f4),
    (0x0885FA, i_0885fa),
    (0x088600, i_088600),
    (0x088606, i_088606),
    (0x08860C, i_08860c),
    (0x088612, i_088612),
    (0x088618, i_088618),
    (0x08861C, i_08861c),
    (0x088622, i_088622),
    (0x088628, i_088628),
    (0x08862C, i_08862c),
    (0x088630, i_088630),
    (0x088636, i_088636),
    (0x08863C, i_08863c),
    (0x088642, i_088642),
    (0x088648, i_088648),
    (0x08864E, i_08864e),
    (0x088654, i_088654),
    (0x088658, i_088658),
    (0x08865E, i_08865e),
    (0x088664, i_088664),
    (0x088668, i_088668),
    (0x08866C, i_08866c),
    (0x088672, i_088672),
    (0x088678, i_088678),
    (0x08867E, i_08867e),
    (0x088684, i_088684),
    (0x08868A, i_08868a),
    (0x08868E, i_08868e),
    (0x088694, i_088694),
    (0x088698, i_088698),
    (0x08869E, i_08869e),
    (0x0886A4, i_0886a4),
    (0x0886AA, i_0886aa),
    (0x0886B0, i_0886b0),
    (0x0886B6, i_0886b6),
    (0x0886BA, i_0886ba),
    (0x0886C0, i_0886c0),
    (0x0886C2, i_0886c2),
    (0x0886C6, i_0886c6),
    (0x0886CC, i_0886cc),
    (0x0886D2, i_0886d2),
    (0x0886D8, i_0886d8),
    (0x0886DE, i_0886de),
    (0x0886E4, i_0886e4),
    (0x0886EA, i_0886ea),
    (0x0886EE, i_0886ee),
    (0x0886F4, i_0886f4),
    (0x0886FA, i_0886fa),
    (0x0886FE, i_0886fe),
    (0x088702, i_088702),
    (0x088708, i_088708),
    (0x08870E, i_08870e),
    (0x088714, i_088714),
    (0x08871A, i_08871a),
    (0x088720, i_088720),
    (0x088726, i_088726),
    (0x08872A, i_08872a),
    (0x088730, i_088730),
    (0x088736, i_088736),
    (0x08873A, i_08873a),
    (0x08873E, i_08873e),
    (0x088744, i_088744),
    (0x08874A, i_08874a),
    (0x088750, i_088750),
    (0x088756, i_088756),
    (0x08875C, i_08875c),
    (0x088762, i_088762),
    (0x088766, i_088766),
    (0x08876C, i_08876c),
    (0x088772, i_088772),
    (0x088776, i_088776),
    (0x08877A, i_08877a),
    (0x088780, i_088780),
    (0x088786, i_088786),
    (0x08878C, i_08878c),
    (0x088792, i_088792),
    (0x088798, i_088798),
    (0x08879C, i_08879c),
    (0x0887A2, i_0887a2),
    (0x0887A6, i_0887a6),
    (0x0887AC, i_0887ac),
    (0x0887B2, i_0887b2),
    (0x0887B8, i_0887b8),
    (0x0887BE, i_0887be),
    (0x0887C4, i_0887c4),
    (0x0887C8, i_0887c8),
    (0x0887CE, i_0887ce),
    (0x0887D0, i_0887d0),
    (0x0887D4, i_0887d4),
    (0x0887DA, i_0887da),
    (0x0887E0, i_0887e0),
    (0x0887E6, i_0887e6),
    (0x0887EC, i_0887ec),
    (0x0887F2, i_0887f2),
    (0x0887F8, i_0887f8),
    (0x0887FC, i_0887fc),
    (0x088802, i_088802),
    (0x088808, i_088808),
    (0x08880C, i_08880c),
    (0x088810, i_088810),
    (0x088816, i_088816),
    (0x08881C, i_08881c),
    (0x088822, i_088822),
    (0x088828, i_088828),
    (0x08882E, i_08882e),
    (0x088834, i_088834),
    (0x088838, i_088838),
    (0x08883E, i_08883e),
    (0x088844, i_088844),
    (0x088848, i_088848),
    (0x08884C, i_08884c),
    (0x088852, i_088852),
    (0x088858, i_088858),
    (0x08885E, i_08885e),
    (0x088864, i_088864),
    (0x08886A, i_08886a),
    (0x088870, i_088870),
    (0x088874, i_088874),
    (0x08887A, i_08887a),
    (0x088880, i_088880),
    (0x088884, i_088884),
    (0x088888, i_088888),
    (0x08888E, i_08888e),
    (0x088894, i_088894),
    (0x08889A, i_08889a),
    (0x0888A0, i_0888a0),
    (0x0888A6, i_0888a6),
    (0x0888AC, i_0888ac),
    (0x0888B0, i_0888b0),
    (0x0888B6, i_0888b6),
    (0x0888BC, i_0888bc),
    (0x0888C0, i_0888c0),
    (0x0888C4, i_0888c4),
    (0x0888CA, i_0888ca),
    (0x0888D0, i_0888d0),
    (0x0888D6, i_0888d6),
    (0x0888DC, i_0888dc),
    (0x0888E2, i_0888e2),
    (0x0888E6, i_0888e6),
    (0x0888EC, i_0888ec),
    (0x0888F0, i_0888f0),
    (0x0888F6, i_0888f6),
    (0x0888FC, i_0888fc),
    (0x088902, i_088902),
    (0x088908, i_088908),
    (0x08890E, i_08890e),
    (0x088912, i_088912),
    (0x088918, i_088918),
    (0x08891A, i_08891a),
    (0x08891E, i_08891e),
    (0x088924, i_088924),
    (0x08892A, i_08892a),
    (0x088930, i_088930),
    (0x088936, i_088936),
    (0x08893C, i_08893c),
    (0x088942, i_088942),
    (0x088946, i_088946),
    (0x08894C, i_08894c),
    (0x088952, i_088952),
    (0x088956, i_088956),
    (0x08895A, i_08895a),
    (0x088960, i_088960),
    (0x088966, i_088966),
    (0x08896C, i_08896c),
    (0x088972, i_088972),
    (0x088978, i_088978),
    (0x08897E, i_08897e),
    (0x088982, i_088982),
    (0x088988, i_088988),
    (0x08898E, i_08898e),
    (0x088992, i_088992),
    (0x088996, i_088996),
    (0x08899C, i_08899c),
    (0x0889A2, i_0889a2),
    (0x0889A8, i_0889a8),
    (0x0889AE, i_0889ae),
    (0x0889B4, i_0889b4),
    (0x0889BA, i_0889ba),
    (0x0889BE, i_0889be),
    (0x0889C4, i_0889c4),
    (0x0889CA, i_0889ca),
    (0x0889CE, i_0889ce),
    (0x0889D2, i_0889d2),
    (0x0889D8, i_0889d8),
    (0x0889DE, i_0889de),
    (0x0889E4, i_0889e4),
    (0x0889EA, i_0889ea),
    (0x0889F0, i_0889f0),
    (0x0889F4, i_0889f4),
    (0x0889FA, i_0889fa),
    (0x0889FE, i_0889fe),
    (0x088A04, i_088a04),
    (0x088A0A, i_088a0a),
    (0x088A10, i_088a10),
    (0x088A16, i_088a16),
    (0x088A1C, i_088a1c),
    (0x088A20, i_088a20),
    (0x088A26, i_088a26),
    (0x088A28, i_088a28),
    (0x088A2E, i_088a2e),
    (0x088A32, i_088a32),
    (0x088A38, i_088a38),
    (0x088A3E, i_088a3e),
    (0x088A44, i_088a44),
    (0x088A4A, i_088a4a),
    (0x088A50, i_088a50),
    (0x088A56, i_088a56),
    (0x088A5C, i_088a5c),
    (0x088A62, i_088a62),
    (0x088A64, i_088a64),
    (0x088A6A, i_088a6a),
    (0x088A6E, i_088a6e),
    (0x088A74, i_088a74),
    (0x088A7A, i_088a7a),
    (0x088A80, i_088a80),
    (0x088A86, i_088a86),
    (0x088A8C, i_088a8c),
    (0x088A92, i_088a92),
    (0x088A94, i_088a94),
    (0x088A9A, i_088a9a),
    (0x088A9E, i_088a9e),
    (0x088AA4, i_088aa4),
    (0x088AAA, i_088aaa),
    (0x088AB0, i_088ab0),
    (0x088AB6, i_088ab6),
    (0x088ABC, i_088abc),
    (0x088AC2, i_088ac2),
    (0x088AC8, i_088ac8),
    (0x088ACE, i_088ace),
    (0x088AD0, i_088ad0),
    (0x088AD6, i_088ad6),
    (0x088ADA, i_088ada),
    (0x088AE0, i_088ae0),
    (0x088AE6, i_088ae6),
    (0x088AEC, i_088aec),
    (0x088AF2, i_088af2),
    (0x088AF8, i_088af8),
    (0x088AFE, i_088afe),
    (0x088B00, i_088b00),
    (0x088B06, i_088b06),
    (0x088B0A, i_088b0a),
    (0x088B10, i_088b10),
    (0x088B16, i_088b16),
    (0x088B1C, i_088b1c),
    (0x088B22, i_088b22),
    (0x088B28, i_088b28),
    (0x088B2E, i_088b2e),
    (0x088B34, i_088b34),
    (0x088B3A, i_088b3a),
    (0x088B3C, i_088b3c),
    (0x088B3E, i_088b3e),
    (0x088B44, i_088b44),
    (0x088B4A, i_088b4a),
    (0x088B4C, i_088b4c),
    (0x088B52, i_088b52),
    (0x088B58, i_088b58),
    (0x088B5A, i_088b5a),
    (0x088B60, i_088b60),
    (0x088B66, i_088b66),
    (0x088B68, i_088b68),
    (0x088B6E, i_088b6e),
    (0x088B74, i_088b74),
    (0x088B76, i_088b76),
    (0x088B7C, i_088b7c),
    (0x088B82, i_088b82),
    (0x088B84, i_088b84),
    (0x088B8A, i_088b8a),
    (0x088B90, i_088b90),
    (0x088B92, i_088b92),
    (0x088B98, i_088b98),
    (0x088B9E, i_088b9e),
    (0x088BA0, i_088ba0),
    (0x088BA6, i_088ba6),
    (0x088BAC, i_088bac),
    (0x088BAE, i_088bae),
    (0x088BB4, i_088bb4),
    (0x088BBA, i_088bba),
    (0x088BBC, i_088bbc),
    (0x088BC2, i_088bc2),
    (0x088BC8, i_088bc8),
    (0x088BCA, i_088bca),
    (0x088BD0, i_088bd0),
    (0x088BD6, i_088bd6),
    (0x088BD8, i_088bd8),
    (0x088BDE, i_088bde),
    (0x088BE4, i_088be4),
    (0x088BE6, i_088be6),
    (0x088BEC, i_088bec),
    (0x088BF2, i_088bf2),
    (0x088BF4, i_088bf4),
    (0x088BFA, i_088bfa),
    (0x088C00, i_088c00),
    (0x088C02, i_088c02),
    (0x088C08, i_088c08),
    (0x088C0E, i_088c0e),
    (0x088C10, i_088c10),
    (0x088C16, i_088c16),
    (0x088C1C, i_088c1c),
    (0x088C1E, i_088c1e),
    (0x088C24, i_088c24),
    (0x088C2A, i_088c2a),
    (0x088C2C, i_088c2c),
    (0x088C32, i_088c32),
    (0x088C38, i_088c38),
    (0x088C3A, i_088c3a),
    (0x088C40, i_088c40),
    (0x088C46, i_088c46),
    (0x088C48, i_088c48),
    (0x088C4E, i_088c4e),
    (0x088C54, i_088c54),
    (0x088C56, i_088c56),
    (0x088C5C, i_088c5c),
    (0x088C62, i_088c62),
    (0x088C64, i_088c64),
    (0x088C6A, i_088c6a),
    (0x088C70, i_088c70),
    (0x088C72, i_088c72),
    (0x088C78, i_088c78),
    (0x088C7E, i_088c7e),
    (0x088C80, i_088c80),
    (0x088C86, i_088c86),
    (0x088C8C, i_088c8c),
    (0x088C8E, i_088c8e),
    (0x088C94, i_088c94),
    (0x088C9A, i_088c9a),
    (0x088C9C, i_088c9c),
    (0x088CA2, i_088ca2),
    (0x088CA8, i_088ca8),
    (0x088CAA, i_088caa),
    (0x088CB0, i_088cb0),
    (0x088CB6, i_088cb6),
    (0x088CB8, i_088cb8),
    (0x088CBE, i_088cbe),
    (0x088CC4, i_088cc4),
    (0x088CC6, i_088cc6),
    (0x088CCC, i_088ccc),
    (0x088CD2, i_088cd2),
    (0x088CD4, i_088cd4),
    (0x088CDA, i_088cda),
    (0x088CE0, i_088ce0),
    (0x088CE2, i_088ce2),
    (0x088CE8, i_088ce8),
    (0x088CEE, i_088cee),
    (0x088CF0, i_088cf0),
    (0x088CF6, i_088cf6),
    (0x088CFC, i_088cfc),
    (0x088CFE, i_088cfe),
    (0x088D04, i_088d04),
    (0x088D0A, i_088d0a),
    (0x088D0C, i_088d0c),
    (0x088D12, i_088d12),
    (0x088D18, i_088d18),
    (0x088D1A, i_088d1a),
    (0x088D20, i_088d20),
    (0x088D26, i_088d26),
    (0x088D28, i_088d28),
    (0x088D2E, i_088d2e),
    (0x088D34, i_088d34),
    (0x088D36, i_088d36),
    (0x088D3C, i_088d3c),
    (0x088D42, i_088d42),
    (0x088D44, i_088d44),
    (0x088D4A, i_088d4a),
    (0x088D50, i_088d50),
    (0x088D52, i_088d52),
    (0x088D58, i_088d58),
    (0x088D5E, i_088d5e),
    (0x088D60, i_088d60),
    (0x088D66, i_088d66),
    (0x088D6C, i_088d6c),
    (0x088D6E, i_088d6e),
    (0x088D74, i_088d74),
    (0x088D7A, i_088d7a),
    (0x088D7C, i_088d7c),
    (0x088D82, i_088d82),
    (0x088D88, i_088d88),
    (0x088D8A, i_088d8a),
    (0x088D90, i_088d90),
    (0x088D96, i_088d96),
    (0x088D98, i_088d98),
    (0x088D9E, i_088d9e),
    (0x088DA4, i_088da4),
    (0x088DA6, i_088da6),
    (0x088DAC, i_088dac),
    (0x088DB2, i_088db2),
    (0x088DB4, i_088db4),
    (0x088DBA, i_088dba),
    (0x088DC0, i_088dc0),
    (0x088DC2, i_088dc2),
    (0x088DC8, i_088dc8),
    (0x088DCE, i_088dce),
    (0x088DD0, i_088dd0),
    (0x088DD6, i_088dd6),
    (0x088DDC, i_088ddc),
    (0x088DDE, i_088dde),
    (0x088DE4, i_088de4),
    (0x088DEA, i_088dea),
    (0x088DEC, i_088dec),
    (0x088DF2, i_088df2),
    (0x088DF8, i_088df8),
    (0x088DFA, i_088dfa),
    (0x088E00, i_088e00),
    (0x088E06, i_088e06),
    (0x088E08, i_088e08),
    (0x088E0E, i_088e0e),
    (0x088E14, i_088e14),
    (0x088E16, i_088e16),
    (0x088E1C, i_088e1c),
    (0x088E22, i_088e22),
    (0x088E24, i_088e24),
    (0x088E2A, i_088e2a),
    (0x088E30, i_088e30),
    (0x088E32, i_088e32),
    (0x088E38, i_088e38),
    (0x088E3E, i_088e3e),
    (0x088E40, i_088e40),
    (0x088E46, i_088e46),
    (0x088E4C, i_088e4c),
    (0x088E4E, i_088e4e),
    (0x088E54, i_088e54),
    (0x088E5A, i_088e5a),
    (0x088E5C, i_088e5c),
    (0x088E62, i_088e62),
    (0x088E68, i_088e68),
    (0x088E6A, i_088e6a),
    (0x088E70, i_088e70),
    (0x088E76, i_088e76),
    (0x088E78, i_088e78),
    (0x088E7E, i_088e7e),
    (0x088E84, i_088e84),
    (0x088E86, i_088e86),
    (0x088E8C, i_088e8c),
    (0x088E92, i_088e92),
    (0x088E94, i_088e94),
    (0x088E9A, i_088e9a),
    (0x088EA0, i_088ea0),
    (0x088EA2, i_088ea2),
    (0x088EA8, i_088ea8),
    (0x088EAE, i_088eae),
    (0x088EB0, i_088eb0),
    (0x088EB6, i_088eb6),
    (0x088EBC, i_088ebc),
    (0x088EBE, i_088ebe),
    (0x088EC4, i_088ec4),
    (0x088ECA, i_088eca),
    (0x088ECC, i_088ecc),
    (0x088ED2, i_088ed2),
    (0x088ED8, i_088ed8),
    (0x088EDA, i_088eda),
    (0x088EE0, i_088ee0),
    (0x088EE6, i_088ee6),
    (0x088EE8, i_088ee8),
    (0x088EEE, i_088eee),
    (0x088EF4, i_088ef4),
    (0x088EF6, i_088ef6),
    (0x088EFC, i_088efc),
    (0x088F02, i_088f02),
    (0x088F04, i_088f04),
    (0x088F0A, i_088f0a),
    (0x088F10, i_088f10),
    (0x088F12, i_088f12),
    (0x088F18, i_088f18),
    (0x088F1E, i_088f1e),
    (0x088F20, i_088f20),
    (0x088F26, i_088f26),
    (0x088F2C, i_088f2c),
    (0x088F2E, i_088f2e),
    (0x088F34, i_088f34),
    (0x088F3A, i_088f3a),
    (0x088F3C, i_088f3c),
    (0x088F42, i_088f42),
    (0x088F48, i_088f48),
    (0x088F4A, i_088f4a),
    (0x088F50, i_088f50),
    (0x088F56, i_088f56),
    (0x088F58, i_088f58),
    (0x088F5C, i_088f5c),
    (0x088F60, i_088f60),
    (0x088F64, i_088f64),
    (0x088F68, i_088f68),
    (0x088F6C, i_088f6c),
    (0x088F6E, i_088f6e),
    (0x088F72, i_088f72),
    (0x088F74, i_088f74),
    (0x088F78, i_088f78),
    (0x088F7E, i_088f7e),
    (0x088F84, i_088f84),
    (0x088F8A, i_088f8a),
    (0x088F8E, i_088f8e),
    (0x088F92, i_088f92),
    (0x088F98, i_088f98),
    (0x088F9E, i_088f9e),
    (0x088FA4, i_088fa4),
    (0x088FA8, i_088fa8),
    (0x088FAC, i_088fac),
    (0x088FB2, i_088fb2),
    (0x088FB8, i_088fb8),
    (0x088FBE, i_088fbe),
    (0x088FC4, i_088fc4),
    (0x088FCA, i_088fca),
    (0x088FD0, i_088fd0),
    (0x088FD4, i_088fd4),
    (0x088FD6, i_088fd6),
    (0x088FDC, i_088fdc),
    (0x088FE2, i_088fe2),
    (0x088FE8, i_088fe8),
    (0x088FEC, i_088fec),
    (0x088FF2, i_088ff2),
    (0x088FF4, i_088ff4),
    (0x088FF8, i_088ff8),
    (0x088FFE, i_088ffe),
    (0x089002, i_089002),
    (0x089006, i_089006),
    (0x08900C, i_08900c),
    (0x089012, i_089012),
    (0x089018, i_089018),
    (0x08901E, i_08901e),
    (0x089024, i_089024),
    (0x08902A, i_08902a),
    (0x089030, i_089030),
    (0x089036, i_089036),
    (0x08903C, i_08903c),
    (0x089040, i_089040),
    (0x089044, i_089044),
    (0x089046, i_089046),
    (0x08904C, i_08904c),
    (0x089052, i_089052),
    (0x089058, i_089058),
    (0x08905C, i_08905c),
    (0x089062, i_089062),
    (0x089068, i_089068),
    (0x08906E, i_08906e),
    (0x089074, i_089074),
    (0x08907A, i_08907a),
    (0x089080, i_089080),
    (0x089084, i_089084),
    (0x089088, i_089088),
    (0x08908E, i_08908e),
    (0x089094, i_089094),
    (0x08909A, i_08909a),
    (0x08909E, i_08909e),
    (0x0890A4, i_0890a4),
    (0x0890AA, i_0890aa),
    (0x0890B0, i_0890b0),
    (0x0890B6, i_0890b6),
    (0x0890BA, i_0890ba),
    (0x0890C0, i_0890c0),
    (0x0890C4, i_0890c4),
    (0x0890CA, i_0890ca),
    (0x0890CC, i_0890cc),
    (0x0890D0, i_0890d0),
    (0x0890D6, i_0890d6),
    (0x0890DA, i_0890da),
    (0x0890E0, i_0890e0),
    (0x0890E6, i_0890e6),
    (0x0890EC, i_0890ec),
    (0x0890F2, i_0890f2),
    (0x0890F6, i_0890f6),
    (0x0890FC, i_0890fc),
    (0x089102, i_089102),
    (0x089108, i_089108),
    (0x08910E, i_08910e),
    (0x089112, i_089112),
    (0x089118, i_089118),
    (0x08911E, i_08911e),
    (0x089124, i_089124),
    (0x08912A, i_08912a),
    (0x089130, i_089130),
    (0x089134, i_089134),
    (0x08913A, i_08913a),
    (0x089140, i_089140),
    (0x089146, i_089146),
    (0x08914C, i_08914c),
    (0x089152, i_089152),
    (0x089156, i_089156),
    (0x089158, i_089158),
    (0x08915E, i_08915e),
    (0x089160, i_089160),
    (0x089164, i_089164),
    (0x08916A, i_08916a),
    (0x089170, i_089170),
    (0x089176, i_089176),
    (0x08917C, i_08917c),
    (0x089182, i_089182),
    (0x089188, i_089188),
    (0x08918E, i_08918e),
    (0x089190, i_089190),
    (0x089194, i_089194),
    (0x089196, i_089196),
    (0x08919C, i_08919c),
    (0x08919E, i_08919e),
    (0x0891A2, i_0891a2),
    (0x0891A8, i_0891a8),
    (0x0891AC, i_0891ac),
    (0x0891B2, i_0891b2),
    (0x0891B6, i_0891b6),
    (0x0891B8, i_0891b8),
    (0x0891BE, i_0891be),
    (0x0891C4, i_0891c4),
    (0x0891CA, i_0891ca),
    (0x0891CE, i_0891ce),
    (0x0891D4, i_0891d4),
    (0x0891DA, i_0891da),
    (0x0891E0, i_0891e0),
    (0x0891E6, i_0891e6),
    (0x0891EA, i_0891ea),
    (0x0891EE, i_0891ee),
    (0x0891F4, i_0891f4),
    (0x0891FA, i_0891fa),
    (0x089200, i_089200),
    (0x089206, i_089206),
    (0x08920C, i_08920c),
    (0x089212, i_089212),
    (0x089218, i_089218),
    (0x08921E, i_08921e),
    (0x089222, i_089222),
    (0x089226, i_089226),
    (0x089228, i_089228),
    (0x08922E, i_08922e),
    (0x089232, i_089232),
    (0x089238, i_089238),
    (0x08923A, i_08923a),
    (0x089240, i_089240),
    (0x089246, i_089246),
    (0x089248, i_089248),
    (0x08924C, i_08924c),
    (0x08924E, i_08924e),
    (0x089250, i_089250),
    (0x089256, i_089256),
    (0x089258, i_089258),
    (0x08925C, i_08925c),
    (0x089262, i_089262),
    (0x089266, i_089266),
    (0x08926C, i_08926c),
    (0x089270, i_089270),
    (0x089272, i_089272),
    (0x089278, i_089278),
    (0x08927E, i_08927e),
    (0x089284, i_089284),
    (0x089288, i_089288),
    (0x08928E, i_08928e),
    (0x089290, i_089290),
    (0x089294, i_089294),
    (0x08929A, i_08929a),
    (0x0892A0, i_0892a0),
    (0x0892A6, i_0892a6),
    (0x0892AC, i_0892ac),
    (0x0892B2, i_0892b2),
    (0x0892B8, i_0892b8),
    (0x0892BE, i_0892be),
    (0x0892C4, i_0892c4),
    (0x0892CA, i_0892ca),
    (0x0892CE, i_0892ce),
    (0x0892D0, i_0892d0),
    (0x0892D6, i_0892d6),
    (0x0892DC, i_0892dc),
    (0x0892E2, i_0892e2),
    (0x0892E6, i_0892e6),
    (0x0892EC, i_0892ec),
    (0x0892F2, i_0892f2),
    (0x0892F8, i_0892f8),
    (0x0892FE, i_0892fe),
    (0x089302, i_089302),
    (0x089308, i_089308),
    (0x08930E, i_08930e),
    (0x089312, i_089312),
    (0x089318, i_089318),
    (0x08931E, i_08931e),
    (0x089324, i_089324),
    (0x08932A, i_08932a),
    (0x089330, i_089330),
    (0x089336, i_089336),
    (0x08933C, i_08933c),
    (0x089342, i_089342),
    (0x089348, i_089348),
    (0x08934C, i_08934c),
    (0x089352, i_089352),
    (0x089358, i_089358),
    (0x08935C, i_08935c),
    (0x08935E, i_08935e),
    (0x089362, i_089362),
    (0x089368, i_089368),
    (0x08936C, i_08936c),
    (0x089372, i_089372),
    (0x089374, i_089374),
    (0x08937A, i_08937a),
    (0x08937E, i_08937e),
    (0x089384, i_089384),
    (0x08938A, i_08938a),
    (0x089390, i_089390),
    (0x089394, i_089394),
    (0x089396, i_089396),
    (0x089398, i_089398),
    (0x08939E, i_08939e),
    (0x0893A4, i_0893a4),
    (0x0893AA, i_0893aa),
    (0x0893AC, i_0893ac),
    (0x0893B0, i_0893b0),
    (0x0893B6, i_0893b6),
    (0x0893BC, i_0893bc),
    (0x0893C2, i_0893c2),
    (0x0893C6, i_0893c6),
    (0x0893CE, i_0893ce),
    (0x0893D2, i_0893d2),
    (0x0893D8, i_0893d8),
    (0x0893DE, i_0893de),
    (0x0893E2, i_0893e2),
    (0x0893E4, i_0893e4),
    (0x0893E8, i_0893e8),
    (0x0893F0, i_0893f0),
    (0x0893F4, i_0893f4),
    (0x0893F6, i_0893f6),
    (0x0893FA, i_0893fa),
    (0x089400, i_089400),
    (0x089406, i_089406),
    (0x08940C, i_08940c),
    (0x089412, i_089412),
    (0x089418, i_089418),
    (0x08941E, i_08941e),
    (0x089424, i_089424),
    (0x08942A, i_08942a),
    (0x089430, i_089430),
    (0x089436, i_089436),
    (0x08943C, i_08943c),
    (0x089442, i_089442),
    (0x089448, i_089448),
    (0x08944C, i_08944c),
    (0x089452, i_089452),
    (0x08945A, i_08945a),
    (0x08945E, i_08945e),
    (0x089462, i_089462),
    (0x089468, i_089468),
    (0x08946C, i_08946c),
    (0x089472, i_089472),
    (0x089478, i_089478),
    (0x08947C, i_08947c),
    (0x089482, i_089482),
    (0x089488, i_089488),
    (0x08948E, i_08948e),
    (0x089494, i_089494),
    (0x08949A, i_08949a),
    (0x0894A0, i_0894a0),
    (0x0894A6, i_0894a6),
    (0x0894A8, i_0894a8),
    (0x0894AC, i_0894ac),
    (0x0894B0, i_0894b0),
    (0x0894B6, i_0894b6),
    (0x0894BA, i_0894ba),
    (0x0894C0, i_0894c0),
    (0x0894C8, i_0894c8),
    (0x0894CC, i_0894cc),
    (0x0894D0, i_0894d0),
    (0x0894D2, i_0894d2),
    (0x0894D8, i_0894d8),
    (0x0894DE, i_0894de),
    (0x0894E4, i_0894e4),
    (0x0894E8, i_0894e8),
    (0x0894EC, i_0894ec),
    (0x0894F2, i_0894f2),
    (0x0894F8, i_0894f8),
    (0x0894FC, i_0894fc),
    (0x089500, i_089500),
    (0x089502, i_089502),
    (0x089504, i_089504),
    (0x08950A, i_08950a),
    (0x089510, i_089510),
    (0x089516, i_089516),
    (0x08951C, i_08951c),
    (0x089522, i_089522),
    (0x089528, i_089528),
    (0x089530, i_089530),
    (0x089534, i_089534),
    (0x08953A, i_08953a),
    (0x089542, i_089542),
    (0x08954C, i_08954c),
    (0x089556, i_089556),
    (0x08955C, i_08955c),
    (0x089562, i_089562),
    (0x089568, i_089568),
    (0x08956E, i_08956e),
    (0x089574, i_089574),
    (0x08957A, i_08957a),
    (0x08957E, i_08957e),
    (0x089584, i_089584),
    (0x08958A, i_08958a),
    (0x08958E, i_08958e),
    (0x089590, i_089590),
    (0x089596, i_089596),
    (0x08959A, i_08959a),
    (0x08959E, i_08959e),
    (0x0895A2, i_0895a2),
    (0x0895A8, i_0895a8),
    (0x0895AE, i_0895ae),
    (0x0895B4, i_0895b4),
    (0x0895BA, i_0895ba),
    (0x0895BC, i_0895bc),
    (0x0895C0, i_0895c0),
    (0x0895C4, i_0895c4),
    (0x0895C8, i_0895c8),
    (0x0895CA, i_0895ca),
    (0x0895CC, i_0895cc),
    (0x0895D2, i_0895d2),
    (0x0895D6, i_0895d6),
    (0x0895D8, i_0895d8),
    (0x0895DE, i_0895de),
    (0x0895E2, i_0895e2),
    (0x0895E6, i_0895e6),
    (0x0895EA, i_0895ea),
    (0x0895F0, i_0895f0),
    (0x0895F6, i_0895f6),
    (0x0895FC, i_0895fc),
    (0x0895FE, i_0895fe),
    (0x089602, i_089602),
    (0x089608, i_089608),
    (0x089610, i_089610),
    (0x089614, i_089614),
    (0x08961A, i_08961a),
    (0x089620, i_089620),
    (0x089624, i_089624),
    (0x089628, i_089628),
    (0x08962E, i_08962e),
    (0x089632, i_089632),
    (0x089634, i_089634),
    (0x08963A, i_08963a),
    (0x08963E, i_08963e),
    (0x089646, i_089646),
    (0x08964A, i_08964a),
    (0x08964E, i_08964e),
    (0x089652, i_089652),
    (0x089656, i_089656),
    (0x08965C, i_08965c),
    (0x089662, i_089662),
    (0x089666, i_089666),
    (0x08966A, i_08966a),
    (0x089670, i_089670),
    (0x089676, i_089676),
    (0x08967A, i_08967a),
    (0x089680, i_089680),
    (0x089686, i_089686),
    (0x089688, i_089688),
    (0x08968C, i_08968c),
    (0x089692, i_089692),
    (0x089698, i_089698),
    (0x08969E, i_08969e),
    (0x0896A4, i_0896a4),
    (0x0896AA, i_0896aa),
    (0x0896B0, i_0896b0),
    (0x0896B4, i_0896b4),
    (0x0896B6, i_0896b6),
    (0x0896BA, i_0896ba),
    (0x0896C0, i_0896c0),
    (0x0896C6, i_0896c6),
    (0x0896CC, i_0896cc),
    (0x0896D2, i_0896d2),
    (0x0896D6, i_0896d6),
    (0x0896DC, i_0896dc),
    (0x0896DE, i_0896de),
    (0x0896E4, i_0896e4),
    (0x0896E8, i_0896e8),
    (0x0896EC, i_0896ec),
    (0x0896F2, i_0896f2),
    (0x0896F6, i_0896f6),
    (0x0896FC, i_0896fc),
    (0x089700, i_089700),
    (0x089704, i_089704),
    (0x08970A, i_08970a),
    (0x08970E, i_08970e),
    (0x089716, i_089716),
    (0x08971A, i_08971a),
    (0x08971C, i_08971c),
    (0x089722, i_089722),
    (0x089724, i_089724),
    (0x089728, i_089728),
    (0x08972C, i_08972c),
    (0x089732, i_089732),
    (0x089734, i_089734),
    (0x089738, i_089738),
    (0x08973C, i_08973c),
    (0x089740, i_089740),
    (0x089746, i_089746),
    (0x08974A, i_08974a),
    (0x08974E, i_08974e),
    (0x089754, i_089754),
    (0x08975A, i_08975a),
    (0x089760, i_089760),
    (0x089764, i_089764),
    (0x08976A, i_08976a),
    (0x089770, i_089770),
    (0x089776, i_089776),
    (0x08977C, i_08977c),
    (0x089782, i_089782),
    (0x089788, i_089788),
    (0x08978C, i_08978c),
    (0x089792, i_089792),
    (0x089798, i_089798),
    (0x08979E, i_08979e),
    (0x0897A4, i_0897a4),
    (0x0897A8, i_0897a8),
    (0x0897AE, i_0897ae),
    (0x0897B2, i_0897b2),
    (0x0897B8, i_0897b8),
    (0x0897BE, i_0897be),
    (0x0897C4, i_0897c4),
    (0x0897CA, i_0897ca),
    (0x0897D0, i_0897d0),
    (0x0897D2, i_0897d2),
    (0x0897D6, i_0897d6),
    (0x0897DC, i_0897dc),
    (0x0897E2, i_0897e2),
    (0x0897E8, i_0897e8),
    (0x0897EE, i_0897ee),
    (0x0897F4, i_0897f4),
    (0x0897FA, i_0897fa),
    (0x089800, i_089800),
    (0x089806, i_089806),
    (0x08980C, i_08980c),
    (0x089810, i_089810),
    (0x089816, i_089816),
    (0x08981A, i_08981a),
    (0x08981C, i_08981c),
    (0x089822, i_089822),
    (0x089826, i_089826),
    (0x08982A, i_08982a),
    (0x08982E, i_08982e),
    (0x089832, i_089832),
    (0x089838, i_089838),
    (0x08983A, i_08983a),
    (0x08983E, i_08983e),
    (0x089844, i_089844),
    (0x08984A, i_08984a),
    (0x089850, i_089850),
    (0x089856, i_089856),
    (0x08985C, i_08985c),
    (0x089862, i_089862),
    (0x089868, i_089868),
    (0x08986E, i_08986e),
    (0x089872, i_089872),
    (0x089874, i_089874),
    (0x089878, i_089878),
    (0x08987E, i_08987e),
    (0x089884, i_089884),
    (0x08988A, i_08988a),
    (0x089890, i_089890),
    (0x089894, i_089894),
    (0x08989A, i_08989a),
    (0x08989C, i_08989c),
    (0x0898A0, i_0898a0),
    (0x0898A6, i_0898a6),
    (0x0898AC, i_0898ac),
    (0x0898B2, i_0898b2),
    (0x0898B8, i_0898b8),
    (0x0898BE, i_0898be),
    (0x0898C4, i_0898c4),
    (0x0898CA, i_0898ca),
    (0x0898D0, i_0898d0),
    (0x0898D4, i_0898d4),
    (0x0898D8, i_0898d8),
    (0x0898DC, i_0898dc),
    (0x0898E2, i_0898e2),
    (0x0898E4, i_0898e4),
    (0x0898E8, i_0898e8),
    (0x0898EE, i_0898ee),
    (0x0898F2, i_0898f2),
    (0x0898F8, i_0898f8),
    (0x0898FC, i_0898fc),
    (0x0898FE, i_0898fe),
    (0x089902, i_089902),
    (0x089908, i_089908),
    (0x08990E, i_08990e),
    (0x089912, i_089912),
    (0x089918, i_089918),
    (0x08991C, i_08991c),
    (0x089920, i_089920),
    (0x089924, i_089924),
    (0x089928, i_089928),
    (0x08992C, i_08992c),
    (0x089930, i_089930),
    (0x089936, i_089936),
    (0x08993A, i_08993a),
    (0x08993E, i_08993e),
    (0x089942, i_089942),
    (0x089946, i_089946),
    (0x08994C, i_08994c),
    (0x089952, i_089952),
    (0x089958, i_089958),
    (0x08995C, i_08995c),
    (0x08995E, i_08995e),
    (0x089960, i_089960),
    (0x089964, i_089964),
    (0x089966, i_089966),
    (0x08996A, i_08996a),
    (0x089970, i_089970),
    (0x089974, i_089974),
    (0x089978, i_089978),
    (0x08997E, i_08997e),
    (0x089982, i_089982),
    (0x089986, i_089986),
    (0x08998C, i_08998c),
    (0x08998E, i_08998e),
    (0x089992, i_089992),
    (0x089998, i_089998),
    (0x08999C, i_08999c),
    (0x0899A2, i_0899a2),
    (0x0899A8, i_0899a8),
    (0x0899AE, i_0899ae),
    (0x0899B4, i_0899b4),
    (0x0899B8, i_0899b8),
    (0x0899BE, i_0899be),
    (0x0899C4, i_0899c4),
    (0x0899C8, i_0899c8),
    (0x0899CC, i_0899cc),
    (0x0899CE, i_0899ce),
    (0x0899D0, i_0899d0),
    (0x0899D4, i_0899d4),
    (0x0899D8, i_0899d8),
    (0x0899DC, i_0899dc),
    (0x0899E0, i_0899e0),
    (0x0899E4, i_0899e4),
    (0x0899E8, i_0899e8),
    (0x0899EE, i_0899ee),
    (0x0899F2, i_0899f2),
    (0x0899F6, i_0899f6),
    (0x0899FC, i_0899fc),
    (0x089A00, i_089a00),
    (0x089A02, i_089a02),
    (0x089A04, i_089a04),
    (0x089A0A, i_089a0a),
    (0x089A10, i_089a10),
    (0x089A14, i_089a14),
    (0x089A16, i_089a16),
    (0x089A1A, i_089a1a),
    (0x089A20, i_089a20),
    (0x089A24, i_089a24),
    (0x089A28, i_089a28),
    (0x089A2C, i_089a2c),
    (0x089A2E, i_089a2e),
    (0x089A30, i_089a30),
    (0x089A34, i_089a34),
    (0x089A3A, i_089a3a),
    (0x089A40, i_089a40),
    (0x089A46, i_089a46),
    (0x089A4C, i_089a4c),
    (0x089A52, i_089a52),
    (0x089A56, i_089a56),
    (0x089A5A, i_089a5a),
    (0x089A5E, i_089a5e),
    (0x089A64, i_089a64),
    (0x089A66, i_089a66),
    (0x089A6A, i_089a6a),
    (0x089A70, i_089a70),
    (0x089A74, i_089a74),
    (0x089A7A, i_089a7a),
    (0x089A7E, i_089a7e),
    (0x089A80, i_089a80),
    (0x089A84, i_089a84),
    (0x089A88, i_089a88),
    (0x089A8E, i_089a8e),
    (0x089A92, i_089a92),
    (0x089A98, i_089a98),
    (0x089A9C, i_089a9c),
    (0x089AA0, i_089aa0),
    (0x089AA4, i_089aa4),
    (0x089AAA, i_089aaa),
    (0x089AAC, i_089aac),
    (0x089AB0, i_089ab0),
    (0x089AB6, i_089ab6),
    (0x089ABA, i_089aba),
    (0x089AC0, i_089ac0),
    (0x089AC6, i_089ac6),
    (0x089AC8, i_089ac8),
    (0x089AD0, i_089ad0),
    (0x089AD4, i_089ad4),
    (0x089ADA, i_089ada),
    (0x089AE0, i_089ae0),
    (0x089AE6, i_089ae6),
    (0x089AEC, i_089aec),
    (0x089AF2, i_089af2),
    (0x089AF8, i_089af8),
    (0x089AFE, i_089afe),
    (0x089B04, i_089b04),
    (0x089B0A, i_089b0a),
    (0x089B10, i_089b10),
    (0x089B14, i_089b14),
    (0x089B18, i_089b18),
    (0x089B1E, i_089b1e),
    (0x089B24, i_089b24),
    (0x089B28, i_089b28),
    (0x089B2C, i_089b2c),
    (0x089B2E, i_089b2e),
    (0x089B34, i_089b34),
    (0x089B38, i_089b38),
    (0x089B3E, i_089b3e),
    (0x089B42, i_089b42),
    (0x089B46, i_089b46),
    (0x089B4A, i_089b4a),
    (0x089B4E, i_089b4e),
    (0x089B54, i_089b54),
    (0x089B58, i_089b58),
    (0x089B5C, i_089b5c),
    (0x089B60, i_089b60),
    (0x089B66, i_089b66),
    (0x089B6A, i_089b6a),
    (0x089B6E, i_089b6e),
    (0x089B70, i_089b70),
    (0x089B74, i_089b74),
    (0x089B78, i_089b78),
    (0x089B7C, i_089b7c),
    (0x089B80, i_089b80),
    (0x089B82, i_089b82),
    (0x089B88, i_089b88),
    (0x089B8C, i_089b8c),
    (0x089B90, i_089b90),
    (0x089B94, i_089b94),
    (0x089B96, i_089b96),
    (0x089B9C, i_089b9c),
    (0x089BA2, i_089ba2),
    (0x089BA6, i_089ba6),
    (0x089BAC, i_089bac),
    (0x089BAE, i_089bae),
    (0x089BB2, i_089bb2),
    (0x089BB4, i_089bb4),
    (0x089BBA, i_089bba),
    (0x089BC0, i_089bc0),
    (0x089BC6, i_089bc6),
    (0x089BCC, i_089bcc),
    (0x089BD0, i_089bd0),
    (0x089BD6, i_089bd6),
    (0x089BD8, i_089bd8),
    (0x089BDC, i_089bdc),
    (0x089BDE, i_089bde),
    (0x089BE4, i_089be4),
    (0x089BEA, i_089bea),
    (0x089BF0, i_089bf0),
    (0x089BF6, i_089bf6),
    (0x089BFC, i_089bfc),
    (0x089C00, i_089c00),
    (0x089C06, i_089c06),
    (0x089C08, i_089c08),
    (0x089C0C, i_089c0c),
    (0x089C12, i_089c12),
    (0x089C18, i_089c18),
    (0x089C6C, i_089c6c),
    (0x089C70, i_089c70),
    (0x089C76, i_089c76),
    (0x089C7C, i_089c7c),
    (0x089C82, i_089c82),
    (0x089C88, i_089c88),
    (0x089C8E, i_089c8e),
    (0x089C94, i_089c94),
    (0x089C9A, i_089c9a),
    (0x089CA0, i_089ca0),
    (0x089CA6, i_089ca6),
    (0x089CA8, i_089ca8),
    (0x089CAC, i_089cac),
    (0x089CB2, i_089cb2),
    (0x089CB4, i_089cb4),
    (0x089CB8, i_089cb8),
    (0x089CBE, i_089cbe),
    (0x089CC2, i_089cc2),
    (0x089CC8, i_089cc8),
    (0x089CCC, i_089ccc),
    (0x089CCE, i_089cce),
    (0x089CD4, i_089cd4),
    (0x089CDA, i_089cda),
    (0x089CE0, i_089ce0),
    (0x089CE4, i_089ce4),
    (0x089CEA, i_089cea),
    (0x089CF0, i_089cf0),
    (0x089CF6, i_089cf6),
    (0x089CFC, i_089cfc),
    (0x089D00, i_089d00),
    (0x089D04, i_089d04),
    (0x089D0A, i_089d0a),
    (0x089D10, i_089d10),
    (0x089D16, i_089d16),
    (0x089D1C, i_089d1c),
    (0x089D20, i_089d20),
    (0x089D22, i_089d22),
    (0x089D28, i_089d28),
    (0x089D2C, i_089d2c),
    (0x089D32, i_089d32),
];
