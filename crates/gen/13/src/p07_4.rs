// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07_4) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $07845E: lea.l $2df21a.l, a0
pub fn i_07845e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78464;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078464: move.l a0, $70(a6)
pub fn i_078464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78468;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078468: move.w #$9e, d1
pub fn i_078468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7846C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07846C: bra.w $78778
pub fn i_07846c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78470;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var14_07848e  ($07848E, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $07848E: lea.l $2de100.l, a0
pub fn i_07848e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78494;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE100;
    20
}
/// $078494: jsr $28cd4.l
pub fn i_078494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7849A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7849A);
    cpu.pc = 0x28CD4;
    16
}
/// $07849A: lea.l $2df224.l, a0
pub fn i_07849a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $0784A0: move.l a0, $70(a6)
pub fn i_0784a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784A4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0784A4: move.w #$9e, d1
pub fn i_0784a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784A8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0784A8: bra.w $78778
pub fn i_0784a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var15_0784ca  ($0784CA, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $0784CA: lea.l $2de1ec.l, a0
pub fn i_0784ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE1EC;
    20
}
/// $0784D0: jsr $28cd4.l
pub fn i_0784d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x784D6);
    cpu.pc = 0x28CD4;
    16
}
/// $0784D6: lea.l $2df21a.l, a0
pub fn i_0784d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0784DC: move.l a0, $70(a6)
pub fn i_0784dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784E0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0784E0: move.w #$9e, d1
pub fn i_0784e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784E4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0784E4: bra.w $78778
pub fn i_0784e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x784E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var16_078506  ($078506, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $078506: lea.l $2de100.l, a0
pub fn i_078506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7850C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE100;
    20
}
/// $07850C: jsr $28cd4.l
pub fn i_07850c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78512);
    cpu.pc = 0x28CD4;
    16
}
/// $078512: lea.l $2df224.l, a0
pub fn i_078512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78518;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078518: move.l a0, $70(a6)
pub fn i_078518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7851C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07851C: move.w #$ad, d1
pub fn i_07851c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78520;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078520: bra.w $78778
pub fn i_078520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78524;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var17_078542  ($078542, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078542: lea.l $2de1ec.l, a0
pub fn i_078542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78548;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE1EC;
    20
}
/// $078548: jsr $28cd4.l
pub fn i_078548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7854E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7854E);
    cpu.pc = 0x28CD4;
    16
}
/// $07854E: lea.l $2df21a.l, a0
pub fn i_07854e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78554;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078554: move.l a0, $70(a6)
pub fn i_078554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78558;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078558: move.w #$ad, d1
pub fn i_078558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7855C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07855C: bra.w $78778
pub fn i_07855c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78560;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var18_078560  ($078560, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078560: lea.l $2de262.l, a0
pub fn i_078560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78566;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE262;
    20
}
/// $078566: jsr $28cd4.l
pub fn i_078566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7856C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7856C);
    cpu.pc = 0x28CD4;
    16
}
/// $07856C: lea.l $2df21a.l, a0
pub fn i_07856c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78572;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078572: move.l a0, $70(a6)
pub fn i_078572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78576;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078576: move.w #$a, d1
pub fn i_078576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7857A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07857A: bra.w $78778
pub fn i_07857a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7857E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var19_07857e  ($07857E, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $07857E: lea.l $2de262.l, a0
pub fn i_07857e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78584;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE262;
    20
}
/// $078584: jsr $28cd4.l
pub fn i_078584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7858A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7858A);
    cpu.pc = 0x28CD4;
    16
}
/// $07858A: lea.l $2df21a.l, a0
pub fn i_07858a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78590;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078590: move.l a0, $70(a6)
pub fn i_078590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78594;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078594: move.w #$72, d1
pub fn i_078594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78598;
    cpu.ir = 0x323C;
    let v1: u32 = 0x72;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078598: bra.w $78778
pub fn i_078598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7859C;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var20_07859c  ($07859C, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $07859C: lea.l $2de2d8.l, a0
pub fn i_07859c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE2D8;
    20
}
/// $0785A2: jsr $28cd4.l
pub fn i_0785a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x785A8);
    cpu.pc = 0x28CD4;
    16
}
/// $0785A8: lea.l $2df21a.l, a0
pub fn i_0785a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0785AE: move.l a0, $70(a6)
pub fn i_0785ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785B2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0785B2: move.w #$a, d1
pub fn i_0785b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0785B6: bra.w $78778
pub fn i_0785b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var21_0785ba  ($0785BA, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0785BA: lea.l $2de2d8.l, a0
pub fn i_0785ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE2D8;
    20
}
/// $0785C0: jsr $28cd4.l
pub fn i_0785c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x785C6);
    cpu.pc = 0x28CD4;
    16
}
/// $0785C6: lea.l $2df21a.l, a0
pub fn i_0785c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0785CC: move.l a0, $70(a6)
pub fn i_0785cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785D0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0785D0: move.w #$72, d1
pub fn i_0785d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x72;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0785D4: bra.w $78778
pub fn i_0785d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var22_0785d8  ($0785D8, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0785D8: lea.l $2de2d8.l, a0
pub fn i_0785d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE2D8;
    20
}
/// $0785DE: jsr $28cd4.l
pub fn i_0785de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x785E4);
    cpu.pc = 0x28CD4;
    16
}
/// $0785E4: lea.l $2df21a.l, a0
pub fn i_0785e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0785EA: move.l a0, $70(a6)
pub fn i_0785ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785EE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0785EE: move.w #$88, d1
pub fn i_0785ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785F2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0785F2: bra.w $78778
pub fn i_0785f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var23_0785f6  ($0785F6, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0785F6: lea.l $2de34e.l, a0
pub fn i_0785f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x785FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE34E;
    20
}
/// $0785FC: jsr $28cd4.l
pub fn i_0785fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78602);
    cpu.pc = 0x28CD4;
    16
}
/// $078602: lea.l $2df224.l, a0
pub fn i_078602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78608;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078608: move.l a0, $70(a6)
pub fn i_078608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7860C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07860C: move.w #$a, d1
pub fn i_07860c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78610;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078610: bra.w $78778
pub fn i_078610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78614;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var24_078614  ($078614, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078614: lea.l $2de34e.l, a0
pub fn i_078614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7861A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE34E;
    20
}
/// $07861A: jsr $28cd4.l
pub fn i_07861a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78620);
    cpu.pc = 0x28CD4;
    16
}
/// $078620: lea.l $2df224.l, a0
pub fn i_078620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78626;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078626: move.l a0, $70(a6)
pub fn i_078626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7862A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07862A: move.w #$72, d1
pub fn i_07862a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7862E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x72;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07862E: bra.w $78778
pub fn i_07862e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78632;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var25_078632  ($078632, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078632: lea.l $2de34e.l, a0
pub fn i_078632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78638;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE34E;
    20
}
/// $078638: jsr $28cd4.l
pub fn i_078638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7863E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7863E);
    cpu.pc = 0x28CD4;
    16
}
/// $07863E: lea.l $2df224.l, a0
pub fn i_07863e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78644;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078644: move.l a0, $70(a6)
pub fn i_078644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78648;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078648: move.w #$77, d1
pub fn i_078648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7864C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07864C: bra.w $78778
pub fn i_07864c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78650;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var26_078650  ($078650, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078650: lea.l $2de3c4.l, a0
pub fn i_078650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78656;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE3C4;
    20
}
/// $078656: jsr $28cd4.l
pub fn i_078656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7865C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7865C);
    cpu.pc = 0x28CD4;
    16
}
/// $07865C: lea.l $2df21a.l, a0
pub fn i_07865c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78662;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078662: move.l a0, $70(a6)
pub fn i_078662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78666;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078666: move.w #$a, d1
pub fn i_078666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7866A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07866A: bra.w $78778
pub fn i_07866a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7866E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var27_07866e  ($07866E, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $07866E: lea.l $2de3c4.l, a0
pub fn i_07866e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78674;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE3C4;
    20
}
/// $078674: jsr $28cd4.l
pub fn i_078674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7867A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7867A);
    cpu.pc = 0x28CD4;
    16
}
/// $07867A: lea.l $2df21a.l, a0
pub fn i_07867a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78680;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078680: move.l a0, $70(a6)
pub fn i_078680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78684;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078684: move.w #$72, d1
pub fn i_078684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78688;
    cpu.ir = 0x323C;
    let v1: u32 = 0x72;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078688: bra.w $78778
pub fn i_078688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7868C;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var28_07868c  ($07868C, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $07868C: lea.l $2de43a.l, a0
pub fn i_07868c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78692;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $078692: jsr $28cd4.l
pub fn i_078692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78698);
    cpu.pc = 0x28CD4;
    16
}
/// $078698: lea.l $2df21a.l, a0
pub fn i_078698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7869E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $07869E: move.l a0, $70(a6)
pub fn i_07869e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786A2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0786A2: move.w #$a, d1
pub fn i_0786a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0786A6: bra.w $78778
pub fn i_0786a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var29_0786aa  ($0786AA, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0786AA: lea.l $2de43a.l, a0
pub fn i_0786aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $0786B0: jsr $28cd4.l
pub fn i_0786b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x786B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0786B6: lea.l $2df21a.l, a0
pub fn i_0786b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0786BC: move.l a0, $70(a6)
pub fn i_0786bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786C0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0786C0: move.w #$72, d1
pub fn i_0786c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786C4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x72;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0786C4: bra.w $78778
pub fn i_0786c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var30_0786c8  ($0786C8, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0786C8: lea.l $2de43a.l, a0
pub fn i_0786c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $0786CE: jsr $28cd4.l
pub fn i_0786ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x786D4);
    cpu.pc = 0x28CD4;
    16
}
/// $0786D4: lea.l $2df21a.l, a0
pub fn i_0786d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0786DA: move.l a0, $70(a6)
pub fn i_0786da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786DE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0786DE: move.w #$77, d1
pub fn i_0786de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0786E2: bra.w $78778
pub fn i_0786e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var31_0786e6  ($0786E6, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $0786E6: lea.l $2de43a.l, a0
pub fn i_0786e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $0786EC: jsr $28cd4.l
pub fn i_0786ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x786F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0786F2: lea.l $2df21a.l, a0
pub fn i_0786f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786F8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $0786F8: move.l a0, $70(a6)
pub fn i_0786f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x786FC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0786FC: move.w #$88, d1
pub fn i_0786fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78700;
    cpu.ir = 0x323C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078700: bra.w $78778
pub fn i_078700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78704;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Explosion_Var32_078704  ($078704, 60 B, props/fx_explosions_platforms_crew_076000.s)
/// $078704: lea.l $2de43a.l, a0
pub fn i_078704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7870A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE43A;
    20
}
/// $07870A: jsr $28cd4.l
pub fn i_07870a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78710);
    cpu.pc = 0x28CD4;
    16
}
/// $078710: lea.l $2df21a.l, a0
pub fn i_078710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78716;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078716: move.l a0, $70(a6)
pub fn i_078716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7871A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07871A: move.w #$28, d1
pub fn i_07871a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7871E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07871E: bra.w $78778
pub fn i_07871e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78722;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Debris_ArcA_078740  ($078740, 30 B, props/fx_explosions_platforms_crew_076000.s)
/// $078740: lea.l $2de5f4.l, a0
pub fn i_078740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78746;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE5F4;
    20
}
/// $078746: jsr $28cd4.l
pub fn i_078746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7874C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7874C);
    cpu.pc = 0x28CD4;
    16
}
/// $07874C: lea.l $2df21a.l, a0
pub fn i_07874c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78752;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF21A;
    20
}
/// $078752: move.l a0, $70(a6)
pub fn i_078752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78756;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078756: move.w #$b, d1
pub fn i_078756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7875A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07875A: bra.w $78778
pub fn i_07875a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7875E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78778;
    10
}
// ===== Debris_ArcB_07875e  ($07875E, 226 B, props/fx_explosions_platforms_crew_076000.s)
/// $07875E: lea.l $2de66a.l, a0
pub fn i_07875e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78764;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE66A;
    20
}
/// $078764: jsr $28cd4.l
pub fn i_078764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7876A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7876A);
    cpu.pc = 0x28CD4;
    16
}
/// $07876A: lea.l $2df224.l, a0
pub fn i_07876a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78770;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF224;
    20
}
/// $078770: move.l a0, $70(a6)
pub fn i_078770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78774;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078774: move.w #$b, d1
pub fn i_078774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78778;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078778: jsr $236e.l
pub fn i_078778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7877E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7877E);
    cpu.pc = 0x236E;
    16
}
/// $07877E: move.b #$ff, $32(a6)
pub fn i_07877e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78784;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $078784: move.b #$ff, $33(a6)
pub fn i_078784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7878A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07878A: jsr $77e00(pc)
pub fn i_07878a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7878E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7878E);
    cpu.pc = 0x77E00;
    16
}
/// $07878E: move.w #$0, d0
pub fn i_07878e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78792;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078792: jsr $5dca4.l
pub fn i_078792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78798);
    cpu.pc = 0x5DCA4;
    16
}
/// $078798: move.w d0, $28(a6)
pub fn i_078798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7879C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07879C: move.w #$7f8, $2a(a6)
pub fn i_07879c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7F8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0787A2: move.w #$ffbc, $2e(a6)
pub fn i_0787a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFBC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0787A8: move.w #$0, $2c(a6)
pub fn i_0787a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0787AE: move.w #$3ff, d0
pub fn i_0787ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787B2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0787B2: jsr $5e9e4.l
pub fn i_0787b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x787B8);
    cpu.pc = 0x5E9E4;
    16
}
/// $0787B8: btst.b #$0, d0
pub fn i_0787b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787BC;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0787BC: beq.w $787c2
pub fn i_0787bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x787C2; return 10; }
    8
}
/// $0787C0: neg.w d0
pub fn i_0787c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787C2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0787C2: move.w d0, $28(a6)
pub fn i_0787c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787C6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0787C6: move.w #$7ff, d0
pub fn i_0787c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787CA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0787CA: jsr $5e9e4.l
pub fn i_0787ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x787D0);
    cpu.pc = 0x5E9E4;
    16
}
/// $0787D0: sub.w d0, $2a(a6)
pub fn i_0787d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787D4;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0787D4: andi.b #$1, d0
pub fn i_0787d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787D8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0787D8: or.b d0, $3a(a6)
pub fn i_0787d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787DC;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0787DC: movea.l $70(a6), a0
pub fn i_0787dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787E0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0787E0: jsr $5dd56.l
pub fn i_0787e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x787E6);
    cpu.pc = 0x5DD56;
    16
}
/// $0787E6: bcs.w $78838
pub fn i_0787e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787EA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78838; return 10; }
    8
}
/// $0787EA: lea.l $ffff.w, a0
pub fn i_0787ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787EE;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0787EE: move.l a0, $48(a6)
pub fn i_0787ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787F2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0787F2: lea.l $787f8(pc), a1
pub fn i_0787f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x787F8;
    16
}
/// $0787F6: move.l a1, (a6)
pub fn i_0787f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0787F8: cmpi.w #$fb00, $2a(a6)
pub fn i_0787f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x787FE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFB00, Size::Word, false);
    16
}
/// $0787FE: bgt.w $78808
pub fn i_0787fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78802;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x78808; return 10; }
    8
}
/// $078802: move.w #$fb01, $2a(a6)
pub fn i_078802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78808;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFB01;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078808: jsr $27cee.l
pub fn i_078808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7880E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7880E);
    cpu.pc = 0x27CEE;
    16
}
/// $07880E: jsr $5e804.l
pub fn i_07880e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78814);
    cpu.pc = 0x5E804;
    16
}
/// $078814: bcc.w $78820
pub fn i_078814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78818;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78820; return 10; }
    8
}
/// $078818: andi.b #$ee, ccr
pub fn i_078818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7881C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07881C: bra.w $78826
pub fn i_07881c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78820;
    cpu.ir = 0x6000;
    cpu.pc = 0x78826;
    10
}
/// $078820: jsr $28d70.l
pub fn i_078820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78826);
    cpu.pc = 0x28D70;
    16
}
/// $078826: bcs.w $7820a
pub fn i_078826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7882A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7820A; return 10; }
    8
}
/// $07882A: movea.l $70(a6), a0
pub fn i_07882a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7882E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07882E: jsr $5dd56.l
pub fn i_07882e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78834;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78834);
    cpu.pc = 0x5DD56;
    16
}
/// $078834: bcc.w $7883e
pub fn i_078834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78838;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7883E; return 10; }
    8
}
/// $078838: jmp $518.l
pub fn i_078838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7883E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07883E: rts
pub fn i_07883e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78840;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Smoke_Small_078840  ($078840, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $078840: move.w #$c, d1
pub fn i_078840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78844;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078844: jsr $236e.l
pub fn i_078844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7884A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7884A);
    cpu.pc = 0x236E;
    16
}
/// $07884A: lea.l $2ded26.l, a0
pub fn i_07884a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78850;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DED26;
    20
}
/// $078850: jsr $28cd4.l
pub fn i_078850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78856;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78856);
    cpu.pc = 0x28CD4;
    16
}
/// $078856: lea.l $2df238.l, a0
pub fn i_078856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7885C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF238;
    20
}
/// $07885C: move.l a0, $70(a6)
pub fn i_07885c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78860;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078860: bra.w $789ae
pub fn i_078860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78864;
    cpu.ir = 0x6000;
    cpu.pc = 0x789AE;
    10
}
// ===== Smoke_Pair40_078864  ($078864, 44 B, props/fx_explosions_platforms_crew_076000.s)
/// $078864: lea.l $78908(pc), a1
pub fn i_078864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78868;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78908;
    16
}
/// $078868: jsr $4ae.l
pub fn i_078868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7886E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7886E);
    cpu.pc = 0x4AE;
    16
}
/// $07886E: jsr $5dd02.l
pub fn i_07886e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78874;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78874);
    cpu.pc = 0x5DD02;
    16
}
/// $078874: addi.w #$20, $22(a0)
pub fn i_078874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7887A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07887A: lea.l $78908(pc), a1
pub fn i_07887a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7887E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78908;
    16
}
/// $07887E: jsr $4ae.l
pub fn i_07887e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78884);
    cpu.pc = 0x4AE;
    16
}
/// $078884: jsr $5dd02.l
pub fn i_078884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7888A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7888A);
    cpu.pc = 0x5DD02;
    16
}
/// $07888A: addi.w #$ffe0, $22(a0)
pub fn i_07888a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78890;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== Smoke_Pair20_078890  ($078890, 48 B, props/fx_explosions_platforms_crew_076000.s)
/// $078890: lea.l $78908(pc), a1
pub fn i_078890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78894;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78908;
    16
}
/// $078894: jsr $4ae.l
pub fn i_078894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7889A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7889A);
    cpu.pc = 0x4AE;
    16
}
/// $07889A: jsr $5dd02.l
pub fn i_07889a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788A0);
    cpu.pc = 0x5DD02;
    16
}
/// $0788A0: addi.w #$10, $22(a0)
pub fn i_0788a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788A6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0788A6: lea.l $78908(pc), a1
pub fn i_0788a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78908;
    16
}
/// $0788AA: jsr $4ae.l
pub fn i_0788aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788B0);
    cpu.pc = 0x4AE;
    16
}
/// $0788B0: jsr $5dd02.l
pub fn i_0788b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788B6);
    cpu.pc = 0x5DD02;
    16
}
/// $0788B6: addi.w #$fff0, $22(a0)
pub fn i_0788b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788BC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0788BC: bra.w $78908
pub fn i_0788bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x78908;
    10
}
// ===== Smoke_Std_0788c0  ($0788C0, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $0788C0: move.w #$c, d1
pub fn i_0788c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788C4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0788C4: jsr $236e.l
pub fn i_0788c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788CA);
    cpu.pc = 0x236E;
    16
}
/// $0788CA: lea.l $2de7c2.l, a0
pub fn i_0788ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE7C2;
    20
}
/// $0788D0: jsr $28cd4.l
pub fn i_0788d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788D6);
    cpu.pc = 0x28CD4;
    16
}
/// $0788D6: lea.l $2df238.l, a0
pub fn i_0788d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF238;
    20
}
/// $0788DC: move.l a0, $70(a6)
pub fn i_0788dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788E0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0788E0: bra.w $78998
pub fn i_0788e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x78998;
    10
}
// ===== Smoke_StdB_0788e4  ($0788E4, 36 B, props/fx_explosions_platforms_crew_076000.s)
/// $0788E4: move.w #$c, d1
pub fn i_0788e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788E8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0788E8: jsr $236e.l
pub fn i_0788e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788EE);
    cpu.pc = 0x236E;
    16
}
/// $0788EE: lea.l $2de7c2.l, a0
pub fn i_0788ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE7C2;
    20
}
/// $0788F4: jsr $28cd4.l
pub fn i_0788f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x788FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x788FA);
    cpu.pc = 0x28CD4;
    16
}
/// $0788FA: lea.l $2df238.l, a0
pub fn i_0788fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78900;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF238;
    20
}
/// $078900: move.l a0, $70(a6)
pub fn i_078900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78904;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078904: bra.w $789ae
pub fn i_078904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78908;
    cpu.ir = 0x6000;
    cpu.pc = 0x789AE;
    10
}
// ===== Smoke_Big_078908  ($078908, 232 B, props/fx_explosions_platforms_crew_076000.s)
/// $078908: move.w #$139, d1
pub fn i_078908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7890C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x139;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07890C: jsr $236e.l
pub fn i_07890c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78912);
    cpu.pc = 0x236E;
    16
}
/// $078912: lea.l $2de7c2.l, a0
pub fn i_078912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78918;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE7C2;
    20
}
/// $078918: jsr $28cd4.l
pub fn i_078918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7891E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7891E);
    cpu.pc = 0x28CD4;
    16
}
/// $07891E: lea.l $2df238.l, a0
pub fn i_07891e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78924;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF238;
    20
}
/// $078924: move.l a0, $70(a6)
pub fn i_078924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78928;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078928: bra.w $78998
pub fn i_078928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7892C;
    cpu.ir = 0x6000;
    cpu.pc = 0x78998;
    10
}
/// $078998: move.b #$ff, $32(a6)
pub fn i_078998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7899E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07899E: move.b #$ff, $33(a6)
pub fn i_07899e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789A4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0789A4: jsr $77e00(pc)
pub fn i_0789a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x789A8);
    cpu.pc = 0x77E00;
    16
}
/// $0789A8: bset.b #$6, $12(a6)
pub fn i_0789a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789AE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0789AE: movea.l $70(a6), a0
pub fn i_0789ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789B2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0789B2: jsr $5dd56.l
pub fn i_0789b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x789B8);
    cpu.pc = 0x5DD56;
    16
}
/// $0789B8: bcs.w $789e8
pub fn i_0789b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789BC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x789E8; return 10; }
    8
}
/// $0789BC: lea.l $ffff.w, a0
pub fn i_0789bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789C0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0789C0: move.l a0, $48(a6)
pub fn i_0789c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789C4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0789C4: lea.l $789ca(pc), a1
pub fn i_0789c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x789CA;
    16
}
/// $0789C8: move.l a1, (a6)
pub fn i_0789c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0789CA: jsr $44048.l
pub fn i_0789ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x789D0);
    cpu.pc = 0x44048;
    16
}
/// $0789D0: jsr $28d70.l
pub fn i_0789d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x789D6);
    cpu.pc = 0x28D70;
    16
}
/// $0789D6: bcs.w $789e8
pub fn i_0789d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789DA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x789E8; return 10; }
    8
}
/// $0789DA: movea.l $70(a6), a0
pub fn i_0789da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789DE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0789DE: jsr $5dd56.l
pub fn i_0789de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x789E4);
    cpu.pc = 0x5DD56;
    16
}
/// $0789E4: bcc.w $789ee
pub fn i_0789e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x789EE; return 10; }
    8
}
/// $0789E8: jmp $518.l
pub fn i_0789e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789EE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0789EE: rts
pub fn i_0789ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Smoke_Column_0789f0  ($0789F0, 120 B, props/fx_explosions_platforms_crew_076000.s)
/// $0789F0: lea.l $2deea2.l, a0
pub fn i_0789f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DEEA2;
    20
}
/// $0789F6: jsr $28cd4.l
pub fn i_0789f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x789FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x789FC);
    cpu.pc = 0x28CD4;
    16
}
/// $0789FC: lea.l $2df24c.l, a0
pub fn i_0789fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF24C;
    20
}
/// $078A02: move.l a0, $70(a6)
pub fn i_078a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A06;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078A06: bra.w $78a0a
pub fn i_078a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A0A;
    cpu.ir = 0x6000;
    cpu.pc = 0x78A0A;
    10
}
/// $078A0A: move.w #$10, d1
pub fn i_078a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A0E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078A0E: jsr $236e.l
pub fn i_078a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78A14);
    cpu.pc = 0x236E;
    16
}
/// $078A14: bset.b #$6, $12(a6)
pub fn i_078a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A1A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $078A1A: move.b #$ff, $32(a6)
pub fn i_078a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A20;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $078A20: move.b #$ff, $33(a6)
pub fn i_078a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A26;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $078A26: movea.l $70(a6), a0
pub fn i_078a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A2A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $078A2A: jsr $5dd56.l
pub fn i_078a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78A30);
    cpu.pc = 0x5DD56;
    16
}
/// $078A30: bcs.w $78a60
pub fn i_078a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A34;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78A60; return 10; }
    8
}
/// $078A34: lea.l $ffff.w, a0
pub fn i_078a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A38;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $078A38: move.l a0, $48(a6)
pub fn i_078a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A3C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078A3C: lea.l $78a42(pc), a1
pub fn i_078a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78A42;
    16
}
/// $078A40: move.l a1, (a6)
pub fn i_078a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078A42: jsr $44048.l
pub fn i_078a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78A48);
    cpu.pc = 0x44048;
    16
}
/// $078A48: jsr $28d70.l
pub fn i_078a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78A4E);
    cpu.pc = 0x28D70;
    16
}
/// $078A4E: bcs.w $78a60
pub fn i_078a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A52;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78A60; return 10; }
    8
}
/// $078A52: movea.l $70(a6), a0
pub fn i_078a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A56;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $078A56: jsr $5dd56.l
pub fn i_078a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78A5C);
    cpu.pc = 0x5DD56;
    16
}
/// $078A5C: bcc.w $78a66
pub fn i_078a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78A66; return 10; }
    8
}
/// $078A60: jmp $518.l
pub fn i_078a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A66;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $078A66: rts
pub fn i_078a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_RetGate_078a68  ($078A68, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $078A68: movea.l $8(a6), a1
pub fn i_078a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A6C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $078A6C: move.b $10(a6), d0
pub fn i_078a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A70;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $078A70: cmp.b $10(a1), d0
pub fn i_078a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A74;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $078A74: bcs.w $78a7e
pub fn i_078a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A78;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78A7E; return 10; }
    8
}
// ===== ClearXN_078a78  ($078A78, 6 B, thunks/ccr_helpers.c)
/// $078A78: andi.b #$ee, ccr
pub fn i_078a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A7C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $078A7C: rts
pub fn i_078a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_078a7e  ($078A7E, 6 B, thunks/ccr_helpers.c)
/// $078A7E: ori.b #$11, ccr
pub fn i_078a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A82;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $078A82: rts
pub fn i_078a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78A84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_FallPath_078be0  ($078BE0, 152 B, props/fx_explosions_platforms_crew_076000.s)
/// $078BE0: ?
pub fn i_078be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78BE4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x18);
    20
}
/// $078BE4: ?
pub fn i_078be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78BEA;
    cpu.ir = 0x3B;
    let ad2 = 0x78C22_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let r4 = (v3 | 0xF4) & 0xFF;
    cpu.sr.set_nz(r4, Size::Byte); cpu.sr.0 &= !0x3;
    18
}
/// $078BEA: dc.w $ffef
pub fn i_078bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78BEC;
    cpu.ir = 0xFFEF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $078C64: move.w $22(a6), d0
pub fn i_078c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C68;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $078C68: addi.w #$40, d0
pub fn i_078c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C6C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $078C6C: bpl.w $78c76
pub fn i_078c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C70;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x78C76; return 10; }
    8
}
/// $078C70: jmp $518.l
pub fn i_078c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C76;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $078C76: rts
pub fn i_078c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Tmpl24_078c78  ($078C78, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $078C78: move.w #$1, $86(a6)
pub fn i_078c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C7E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078C7E: bset.b #$0, $3a(a6)
pub fn i_078c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C84;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $078C84: bra.w $78ca8
pub fn i_078c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C88;
    cpu.ir = 0x6000;
    cpu.pc = 0x78CA8;
    10
}
// ===== Breakable_Tmpl26_078c88  ($078C88, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $078C88: move.w #$0, $86(a6)
pub fn i_078c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C8E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078C8E: bset.b #$0, $3a(a6)
pub fn i_078c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C94;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $078C94: bra.w $78ca8
pub fn i_078c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C98;
    cpu.ir = 0x6000;
    cpu.pc = 0x78CA8;
    10
}
// ===== Breakable_Tmpl23_078c98  ($078C98, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $078C98: move.w #$1, $86(a6)
pub fn i_078c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78C9E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078C9E: bra.w $78ca8
pub fn i_078c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x78CA8;
    10
}
// ===== Breakable_Tmpl25_078ca2  ($078CA2, 82 B, props/fx_explosions_platforms_crew_076000.s)
/// $078CA2: move.w #$0, $86(a6)
pub fn i_078ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CA8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078CA8: bclr.b #$1, $12(a6)
pub fn i_078ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CAE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $078CAE: move.w #$1e, d1
pub fn i_078cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CB2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078CB2: jsr $236e.l
pub fn i_078cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78CB8);
    cpu.pc = 0x236E;
    16
}
/// $078CB8: lea.l $78b90(pc), a0
pub fn i_078cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CBC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x78B90;
    16
}
/// $078CBC: jsr $28cd4.l
pub fn i_078cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78CC2);
    cpu.pc = 0x28CD4;
    16
}
/// $078CC2: lea.l $78cc8(pc), a1
pub fn i_078cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78CC8;
    16
}
/// $078CC6: move.l a1, (a6)
pub fn i_078cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078CC8: jsr $2783a.l
pub fn i_078cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78CCE);
    cpu.pc = 0x2783A;
    16
}
/// $078CCE: jsr $28d70.l
pub fn i_078cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78CD4);
    cpu.pc = 0x28D70;
    16
}
/// $078CD4: moveq #$0, d0
pub fn i_078cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CD6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $078CD6: move.b $98(a6), d0
pub fn i_078cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CDA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $078CDA: lsl.w #$3, d0
pub fn i_078cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CDC;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $078CDC: neg.w d0
pub fn i_078cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CDE;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $078CDE: addi.w #$140, d0
pub fn i_078cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CE2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x140, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $078CE2: cmp.w $22(a6), d0
pub fn i_078ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CE6;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $078CE6: bcs.w $78cf0
pub fn i_078ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CEA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78CF0; return 10; }
    8
}
/// $078CEA: lea.l $78cf4(pc), a1
pub fn i_078cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78CF4;
    16
}
/// $078CEE: move.l a1, (a6)
pub fn i_078cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078CF0: bra.w $78c64
pub fn i_078cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x78C64;
    10
}
// ===== Breakable_Destroyed_078cf4  ($078CF4, 58 B, props/fx_explosions_platforms_crew_076000.s)
/// $078CF4: lea.l $78a84(pc), a0
pub fn i_078cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CF8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x78A84;
    16
}
/// $078CF8: jsr $28cd4.l
pub fn i_078cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78CFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78CFE);
    cpu.pc = 0x28CD4;
    16
}
/// $078CFE: lea.l $78d58(pc), a1
pub fn i_078cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78D58;
    16
}
/// $078D02: jsr $4ae.l
pub fn i_078d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D08);
    cpu.pc = 0x4AE;
    16
}
/// $078D08: move.w $86(a6), $86(a0)
pub fn i_078d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D0E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x86);
    bus.write16(ad4, v3 as u16);
    20
}
/// $078D0E: lea.l $78d14(pc), a1
pub fn i_078d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78D14;
    16
}
/// $078D12: move.l a1, (a6)
pub fn i_078d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078D14: jsr $2783a.l
pub fn i_078d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D1A);
    cpu.pc = 0x2783A;
    16
}
/// $078D1A: jsr $28d70.l
pub fn i_078d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D20);
    cpu.pc = 0x28D70;
    16
}
/// $078D20: bcc.w $78d2a
pub fn i_078d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78D2A; return 10; }
    8
}
/// $078D24: lea.l $78d2e(pc), a1
pub fn i_078d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78D2E;
    16
}
/// $078D28: move.l a1, (a6)
pub fn i_078d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D2A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078D2A: bra.w $78c64
pub fn i_078d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78C64;
    10
}
// ===== Breakable_FadeOut_078d2e  ($078D2E, 42 B, props/fx_explosions_platforms_crew_076000.s)
/// $078D2E: move.b #$1e, $59(a6)
pub fn i_078d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D34;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $078D34: lea.l $78d3a(pc), a1
pub fn i_078d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78D3A;
    16
}
/// $078D38: move.l a1, (a6)
pub fn i_078d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078D3A: jsr $2783a.l
pub fn i_078d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D40);
    cpu.pc = 0x2783A;
    16
}
/// $078D40: jsr $28d70.l
pub fn i_078d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D46);
    cpu.pc = 0x28D70;
    16
}
/// $078D46: tst.b $59(a6)
pub fn i_078d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D4A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $078D4A: bne.w $78d54
pub fn i_078d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x78D54; return 10; }
    8
}
/// $078D4E: jmp $518.l
pub fn i_078d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D54;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $078D54: bra.w $78c64
pub fn i_078d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D58;
    cpu.ir = 0x6000;
    cpu.pc = 0x78C64;
    10
}
// ===== Breakable_Wreck_Fall_078d58  ($078D58, 150 B, props/fx_explosions_platforms_crew_076000.s)
/// $078D58: move.w #$8000, d0
pub fn i_078d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D5C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078D5C: jsr $28134.l
pub fn i_078d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D62);
    cpu.pc = 0x28134;
    16
}
/// $078D62: andi.w #$ffe3, $38(a6)
pub fn i_078d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D68;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $078D68: ori.w #$18, $38(a6)
pub fn i_078d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D6E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $078D6E: bset.b #$1, $12(a6)
pub fn i_078d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D74;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $078D74: move.w #$e, d1
pub fn i_078d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D78;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078D78: jsr $236e.l
pub fn i_078d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D7E);
    cpu.pc = 0x236E;
    16
}
/// $078D7E: lea.l $78b9c(pc), a0
pub fn i_078d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D82;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x78B9C;
    16
}
/// $078D82: jsr $28cd4.l
pub fn i_078d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78D88);
    cpu.pc = 0x28CD4;
    16
}
/// $078D88: movea.l $c(a6), a0
pub fn i_078d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D8C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $078D8C: lea.l $98(a0), a1
pub fn i_078d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D90;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    cpu.a[1] = ad1;
    16
}
/// $078D90: lea.l $98(a6), a2
pub fn i_078d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D94;
    cpu.ir = 0x45EE;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    cpu.a[2] = ad1;
    16
}
/// $078D94: move.b (a1)+, (a2)+
pub fn i_078d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D96;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078D96: move.b (a1)+, (a2)+
pub fn i_078d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D98;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078D98: move.b (a1)+, (a2)+
pub fn i_078d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D9A;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078D9A: move.b (a1)+, (a2)+
pub fn i_078d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D9C;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078D9C: move.b (a1)+, (a2)+
pub fn i_078d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78D9E;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078D9E: move.b (a1)+, (a2)+
pub fn i_078d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DA0;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078DA0: move.b (a1)+, (a2)+
pub fn i_078da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DA2;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078DA2: move.b (a1)+, (a2)+
pub fn i_078da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DA4;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078DA4: move.w $3a(a0), $3a(a6)
pub fn i_078da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DAA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $078DAA: lea.l $78be0(pc), a0
pub fn i_078daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DAE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x78BE0;
    16
}
/// $078DAE: move.l a0, $70(a6)
pub fn i_078dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DB2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078DB2: lea.l $78db8(pc), a1
pub fn i_078db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78DB8;
    16
}
/// $078DB6: move.l a1, (a6)
pub fn i_078db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078DB8: movea.l $70(a6), a0
pub fn i_078db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DBC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $078DBC: move.w (a0)+, d0
pub fn i_078dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DBE;
    cpu.ir = 0x3018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $078DBE: move.w (a0)+, d1
pub fn i_078dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DC0;
    cpu.ir = 0x3218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $078DC0: btst.b #$0, $3a(a6)
pub fn i_078dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DC6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $078DC6: beq.w $78dcc
pub fn i_078dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DCA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x78DCC; return 10; }
    8
}
/// $078DCA: neg.w d0
pub fn i_078dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DCC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $078DCC: movea.l $c(a6), a1
pub fn i_078dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DD0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $078DD0: add.w $22(a1), d0
pub fn i_078dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DD4;
    cpu.ir = 0xD069;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $078DD4: add.w $24(a1), d1
pub fn i_078dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DD8;
    cpu.ir = 0xD269;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $078DD8: move.w d0, $22(a6)
pub fn i_078dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DDC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078DDC: move.w d1, $24(a6)
pub fn i_078ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DE0;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078DE0: move.l a0, $70(a6)
pub fn i_078de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DE4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $078DE4: cmpa.l #$78c64, a0
pub fn i_078de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DEA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x78C64;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $078DEA: bcc.w $78df6
pub fn i_078dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78DF6; return 10; }
    8
}
// ===== JsrAbsThunk_078dee  ($078DEE, 8 B, thunks/jsr_abs_thunks.c)
/// $078DEE: jsr $28d70.l
pub fn i_078dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78DF4);
    cpu.pc = 0x28D70;
    16
}
/// $078DF4: rts
pub fn i_078df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Wreck_SpawnSoldier_078df6  ($078DF6, 86 B, props/fx_explosions_platforms_crew_076000.s)
/// $078DF6: jsr $28d70.l
pub fn i_078df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78DFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78DFC);
    cpu.pc = 0x28D70;
    16
}
/// $078DFC: move.w $86(a6), d0
pub fn i_078dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E00;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $078E00: cmpi.w #$1, d0
pub fn i_078e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E04;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $078E04: bne.w $78e1e
pub fn i_078e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E08;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x78E1E; return 10; }
    8
}
/// $078E08: lea.l $57880.l, a1
pub fn i_078e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E0E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x57880;
    20
}
/// $078E0E: jsr $4498e.l
pub fn i_078e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78E14);
    cpu.pc = 0x4498E;
    16
}
/// $078E14: jsr $5dd02.l
pub fn i_078e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78E1A);
    cpu.pc = 0x5DD02;
    16
}
/// $078E1A: bra.w $78e30
pub fn i_078e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E1E;
    cpu.ir = 0x6000;
    cpu.pc = 0x78E30;
    10
}
/// $078E1E: lea.l $5783e.l, a1
pub fn i_078e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E24;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x5783E;
    20
}
/// $078E24: jsr $4498e.l
pub fn i_078e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78E2A);
    cpu.pc = 0x4498E;
    16
}
/// $078E2A: jsr $5dd02.l
pub fn i_078e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78E30);
    cpu.pc = 0x5DD02;
    16
}
/// $078E30: lea.l $99(a6), a1
pub fn i_078e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E34;
    cpu.ir = 0x43EE;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    cpu.a[1] = ad1;
    16
}
/// $078E34: lea.l $99(a0), a2
pub fn i_078e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E38;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x99);
    cpu.a[2] = ad1;
    16
}
/// $078E38: move.b (a1)+, (a2)+
pub fn i_078e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E3A;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E3A: move.b (a1)+, (a2)+
pub fn i_078e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E3C;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E3C: move.b (a1)+, (a2)+
pub fn i_078e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E3E;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E3E: move.b (a1)+, (a2)+
pub fn i_078e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E40;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E40: move.b (a1)+, (a2)+
pub fn i_078e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E42;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E42: move.b (a1)+, (a2)+
pub fn i_078e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E44;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E44: move.b (a1)+, (a2)+
pub fn i_078e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E46;
    cpu.ir = 0x14D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2]; cpu.a[2] = ad4.wrapping_add(1);
    bus.write8(ad4, v3 as u8);
    12
}
/// $078E46: jmp $518.l
pub fn i_078e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78E4C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Breakable_Cb_SpawnChip_078ede  ($078EDE, 54 B, props/fx_explosions_platforms_crew_076000.s)
/// $078EDE: lea.l $78f2c(pc), a1
pub fn i_078ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78EE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78F2C;
    16
}
/// $078EE2: jsr $4ae.l
pub fn i_078ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78EE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78EE8);
    cpu.pc = 0x4AE;
    16
}
/// $078EE8: move.b $3a(a6), $3a(a0)
pub fn i_078ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78EEE;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $078EEE: move.w $74(a6), d0
pub fn i_078eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78EF2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $078EF2: move.w $78(a6), d1
pub fn i_078ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78EF6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $078EF6: btst.b #$0, $3a(a6)
pub fn i_078ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78EFC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $078EFC: beq.w $78f02
pub fn i_078efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x78F02; return 10; }
    8
}
/// $078F00: neg.w d0
pub fn i_078f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F02;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $078F02: add.w $22(a6), d0
pub fn i_078f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F06;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $078F06: add.w $24(a6), d1
pub fn i_078f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F0A;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $078F0A: move.w d0, $22(a0)
pub fn i_078f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F0E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078F0E: move.w d1, $24(a0)
pub fn i_078f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F12;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078F12: rts
pub fn i_078f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Cb_SpawnChipFlip_078f14  ($078F14, 24 B, props/fx_explosions_platforms_crew_076000.s)
/// $078F14: lea.l $78f2c(pc), a1
pub fn i_078f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78F2C;
    16
}
/// $078F18: jsr $4ae.l
pub fn i_078f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78F1E);
    cpu.pc = 0x4AE;
    16
}
/// $078F1E: move.b $3a(a6), d0
pub fn i_078f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F22;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $078F22: bchg.b #$0, d0
pub fn i_078f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F26;
    cpu.ir = 0x840;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 ^ mk;
    cpu.d[0] = r2;
    8
}
/// $078F26: move.b d0, $3a(a0)
pub fn i_078f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F2A;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $078F2A: bra.b $78eee
pub fn i_078f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F2C;
    cpu.ir = 0x60C2;
    cpu.pc = 0x78EEE;
    10
}
// ===== Breakable_Chip_Run_078f2c  ($078F2C, 66 B, props/fx_explosions_platforms_crew_076000.s)
/// $078F2C: move.w #$1e, d1
pub fn i_078f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F30;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $078F30: jsr $236e.l
pub fn i_078f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78F36);
    cpu.pc = 0x236E;
    16
}
/// $078F36: move.w #$fffc, $2e(a6)
pub fn i_078f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F3C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $078F3C: lea.l $78e4c(pc), a0
pub fn i_078f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F40;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x78E4C;
    16
}
/// $078F40: jsr $28cd4.l
pub fn i_078f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78F46);
    cpu.pc = 0x28CD4;
    16
}
/// $078F46: lea.l $78f4c(pc), a1
pub fn i_078f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78F4C;
    16
}
/// $078F4A: move.l a1, (a6)
pub fn i_078f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $078F4C: jsr $27d50.l
pub fn i_078f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78F52);
    cpu.pc = 0x27D50;
    16
}
/// $078F52: bcc.w $78f5c
pub fn i_078f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78F5C; return 10; }
    8
}
/// $078F56: jmp $518.l
pub fn i_078f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F5C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $078F5C: jsr $28d70.l
pub fn i_078f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78F62);
    cpu.pc = 0x28D70;
    16
}
/// $078F62: bcc.w $78f6c
pub fn i_078f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x78F6C; return 10; }
    8
}
/// $078F66: jmp $518.l
pub fn i_078f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F6C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $078F6C: rts
pub fn i_078f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_078f7e  ($078F7E, 6 B, thunks/ccr_helpers.c)
/// $078F7E: andi.b #$ee, ccr
pub fn i_078f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F82;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $078F82: rts
pub fn i_078f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_078f84  ($078F84, 6 B, thunks/ccr_helpers.c)
/// $078F84: ori.b #$11, ccr
pub fn i_078f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F88;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $078F88: rts
pub fn i_078f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PathScript_StepCtx_078f8a  ($078F8A, 6 B, props/fx_explosions_platforms_crew_076000.s)
/// $078F8A: jsr $2783a.l
pub fn i_078f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x78F90);
    cpu.pc = 0x2783A;
    16
}
// ===== PathScript_Step_078f90  ($078F90, 48 B, props/fx_explosions_platforms_crew_076000.s)
/// $078F90: move.w $36(a6), d5
pub fn i_078f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F94;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $078F94: movea.l $90(a6), a0
pub fn i_078f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F98;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $078F98: lea.l $78fc0(pc), a1
pub fn i_078f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x78FC0;
    16
}
/// $078F9C: move.b (a0), d0
pub fn i_078f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78F9E;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $078F9E: cmpi.b #$b, d0
pub fn i_078f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FA2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $078FA2: bcs.w $78fb2
pub fn i_078fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FA6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x78FB2; return 10; }
    8
}
/// $078FA6: nop
pub fn i_078fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FA8;
    cpu.ir = 0x4E71;
    4
}
/// $078FA8: nop
pub fn i_078fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FAA;
    cpu.ir = 0x4E71;
    4
}
/// $078FAA: cmpi.b #$b, d0
pub fn i_078faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FAE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $078FAE: nop
pub fn i_078fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FB0;
    cpu.ir = 0x4E71;
    4
}
/// $078FB0: trap #$f
pub fn i_078fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FB2;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $078FB2: andi.l #$f, d0
pub fn i_078fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FB8;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $078FB8: lsl.l #$2, d0
pub fn i_078fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FBA;
    cpu.ir = 0xE588;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 1, true);
    cpu.d[0] = r;
    10
}
/// $078FBA: movea.l (a1, d0.w), a2
pub fn i_078fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FBE;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $078FBE: jmp (a2)
pub fn i_078fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x78FC0;
    cpu.ir = 0x4ED2;
    cpu.jump_to(cpu.a[2]);
    8
}
// ===== SetXN_07900e  ($07900E, 6 B, thunks/ccr_helpers.c)
/// $07900E: ori.b #$11, ccr
pub fn i_07900e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79012;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $079012: rts
pub fn i_079012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79014;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_079100  ($079100, 6 B, thunks/ccr_helpers.c)
/// $079100: andi.b #$ee, ccr
pub fn i_079100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79104;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $079104: rts
pub fn i_079104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79106;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_079174  ($079174, 6 B, thunks/ccr_helpers.c)
/// $079174: andi.b #$ee, ccr
pub fn i_079174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79178;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $079178: rts
pub fn i_079178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7917A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskW_07924a  ($07924A, 6 B, kernel/task_setters_w.c)
/// $07924A: move.w d0, $22(a6)
pub fn i_07924a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7924E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07924E: rts
pub fn i_07924e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79250;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_RetGate_079250  ($079250, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079250: movea.l $8(a6), a1
pub fn i_079250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79254;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079254: move.b $10(a6), d0
pub fn i_079254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79258;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079258: cmp.b $10(a1), d0
pub fn i_079258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7925C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07925C: bcs.w $79266
pub fn i_07925c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79260;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x79266; return 10; }
    8
}
// ===== ClearXN_079260  ($079260, 6 B, thunks/ccr_helpers.c)
/// $079260: andi.b #$ee, ccr
pub fn i_079260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79264;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $079264: rts
pub fn i_079264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79266;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_079266  ($079266, 6 B, thunks/ccr_helpers.c)
/// $079266: ori.b #$11, ccr
pub fn i_079266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7926A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07926A: rts
pub fn i_07926a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7926C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_RecordBest_07926c  ($07926C, 42 B, props/fx_explosions_platforms_crew_076000.s)
/// $07926C: move.b $98(a6), d0
pub fn i_07926c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79270;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079270: andi.w #$ff, d0
pub fn i_079270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79274;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079274: cmpi.w #$63, d0
pub fn i_079274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79278;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x63, Size::Word, false);
    8
}
/// $079278: ble.w $79280
pub fn i_079278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7927C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x79280; return 10; }
    8
}
/// $07927C: move.w #$63, d0
pub fn i_07927c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79280;
    cpu.ir = 0x303C;
    let v1: u32 = 0x63;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079280: cmp.w $106e92.l, d0
pub fn i_079280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79286;
    cpu.ir = 0xB079;
    let v2 = bus.read16(0x106E92) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    16
}
/// $079286: ble.w $79290
pub fn i_079286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7928A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x79290; return 10; }
    8
}
/// $07928A: move.w d0, $106e92.l
pub fn i_07928a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79290;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106E92, v1 as u16);
    16
}
/// $079290: jmp $518.l
pub fn i_079290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79296;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== AutoDemo_Rts_079296  ($079296, 2 B, props/fx_explosions_platforms_crew_076000.s)
/// $079296: rts
pub fn i_079296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79298;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_Tmpl_Driver_079298  ($079298, 142 B, props/fx_explosions_platforms_crew_076000.s)
/// $079298: clr.b d4
pub fn i_079298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7929A;
    cpu.ir = 0x4204;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $07929A: move.b $10e2e4.l, $10e2e3.l
pub fn i_07929a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792A4;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2E4) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v2 as u8);
    28
}
/// $0792A4: move.b $10e2ea.l, $10e2e9.l
pub fn i_0792a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792AE;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2EA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v2 as u8);
    28
}
/// $0792AE: move.b d4, $10e2e3.l
pub fn i_0792ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792B4;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v1 as u8);
    16
}
/// $0792B4: move.b d4, $10e2e9.l
pub fn i_0792b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792BA;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v1 as u8);
    16
}
/// $0792BA: move.b d4, $10e2e4.l
pub fn i_0792ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792C0;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, v1 as u8);
    16
}
/// $0792C0: move.b d4, $10e2ea.l
pub fn i_0792c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792C6;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, v1 as u8);
    16
}
/// $0792C6: move.b d4, $10e2e5.l
pub fn i_0792c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792CC;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E5, v1 as u8);
    16
}
/// $0792CC: move.b d4, $10e2eb.l
pub fn i_0792cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792D2;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EB, v1 as u8);
    16
}
/// $0792D2: move.b d4, $10e2e6.l
pub fn i_0792d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792D8;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E6, v1 as u8);
    16
}
/// $0792D8: move.b d4, $10e2ec.l
pub fn i_0792d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792DE;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EC, v1 as u8);
    16
}
/// $0792DE: clr.b $10e39c.l
pub fn i_0792de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792E4;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39C, r2 as u8);
    16
}
/// $0792E4: clr.b $10e39d.l
pub fn i_0792e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792EA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39D) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39D, r2 as u8);
    16
}
/// $0792EA: lea.l $792f0(pc), a1
pub fn i_0792ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x792F0;
    16
}
/// $0792EE: move.l a1, (a6)
pub fn i_0792ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0792F0: jsr $7962c(pc)
pub fn i_0792f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x792F4);
    cpu.pc = 0x7962C;
    16
}
/// $0792F4: tst.b $10e39c.l
pub fn i_0792f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792FA;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E39C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $0792FA: beq.w $79324
pub fn i_0792fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x792FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79324; return 10; }
    8
}
/// $0792FE: clr.w d0
pub fn i_0792fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79300;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079300: jsr $5e3a2.l
pub fn i_079300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79306;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79306);
    cpu.pc = 0x5E3A2;
    16
}
/// $079306: bcc.w $79310
pub fn i_079306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7930A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79310; return 10; }
    8
}
/// $07930A: move.b #$2, $45(a0)
pub fn i_07930a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79310;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079310: move.w #$1, d0
pub fn i_079310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79314;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079314: jsr $5e3a2.l
pub fn i_079314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7931A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7931A);
    cpu.pc = 0x5E3A2;
    16
}
/// $07931A: bcc.w $79324
pub fn i_07931a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7931E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79324; return 10; }
    8
}
/// $07931E: move.b #$2, $45(a0)
pub fn i_07931e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79324;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079324: rts
pub fn i_079324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79326;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_Stop_079326  ($079326, 20 B, props/fx_explosions_platforms_crew_076000.s)
/// $079326: clr.b $10e39d.l
pub fn i_079326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7932C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39D) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39D, r2 as u8);
    16
}
/// $07932C: clr.b $10e39c.l
pub fn i_07932c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79332;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39C, r2 as u8);
    16
}
/// $079332: jmp $518.l
pub fn i_079332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79338;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== SetTaskHandler_07943c  ($07943C, 8 B, kernel/task_handlers.c)
/// $07943C: lea.l $79326(pc), a1
pub fn i_07943c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79440;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79326;
    16
}
/// $079440: move.l a1, (a6)
pub fn i_079440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79442;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079442: rts
pub fn i_079442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79444;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0007962A  ($07962A, 2 B, thunks/trivial_rts.c)
/// $07962A: rts
pub fn i_07962a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7962C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_StartLevelScript_07962c  ($07962C, 184 B, props/fx_explosions_platforms_crew_076000.s)
/// $07962C: move.b $10e39c.l, d0
pub fn i_07962c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79632;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E39C) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $079632: cmpi.b #$0, d0
pub fn i_079632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79636;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $079636: beq.w $796ea
pub fn i_079636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7963A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x796EA; return 10; }
    8
}
/// $07963A: cmpi.b #$5, d0
pub fn i_07963a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7963E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07963E: bcs.w $7964e
pub fn i_07963e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79642;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7964E; return 10; }
    8
}
/// $079642: nop
pub fn i_079642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79644;
    cpu.ir = 0x4E71;
    4
}
/// $079644: nop
pub fn i_079644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79646;
    cpu.ir = 0x4E71;
    4
}
/// $079646: cmpi.b #$5, d0
pub fn i_079646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7964A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $07964A: nop
pub fn i_07964a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7964C;
    cpu.ir = 0x4E71;
    4
}
/// $07964C: trap #$f
pub fn i_07964c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7964E;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $07964E: cmp.b $10e39d.l, d0
pub fn i_07964e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79654;
    cpu.ir = 0xB039;
    let v2 = bus.read8(0x10E39D) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    16
}
/// $079654: beq.w $796f0
pub fn i_079654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79658;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x796F0; return 10; }
    8
}
/// $079658: move.b d0, $10e39d.l
pub fn i_079658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7965E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39D, v1 as u8);
    16
}
/// $07965E: move.b $10e200.l, d1
pub fn i_07965e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79664;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x10E200) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $079664: move.b d1, $10e2e2.l
pub fn i_079664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7966A;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E2, v1 as u8);
    16
}
/// $07966A: move.b d1, $10e2e8.l
pub fn i_07966a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79670;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E8, v1 as u8);
    16
}
/// $079670: clr.b d4
pub fn i_079670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79672;
    cpu.ir = 0x4204;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $079672: move.b $10e2e4.l, $10e2e3.l
pub fn i_079672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7967C;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2E4) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v2 as u8);
    28
}
/// $07967C: move.b $10e2ea.l, $10e2e9.l
pub fn i_07967c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79686;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E2EA) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v2 as u8);
    28
}
/// $079686: move.b d4, $10e2e3.l
pub fn i_079686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7968C;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E3, v1 as u8);
    16
}
/// $07968C: move.b d4, $10e2e9.l
pub fn i_07968c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79692;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E9, v1 as u8);
    16
}
/// $079692: move.b d4, $10e2e4.l
pub fn i_079692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79698;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, v1 as u8);
    16
}
/// $079698: move.b d4, $10e2ea.l
pub fn i_079698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7969E;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, v1 as u8);
    16
}
/// $07969E: move.b d4, $10e2e5.l
pub fn i_07969e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796A4;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E5, v1 as u8);
    16
}
/// $0796A4: move.b d4, $10e2eb.l
pub fn i_0796a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796AA;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EB, v1 as u8);
    16
}
/// $0796AA: move.b d4, $10e2e6.l
pub fn i_0796aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796B0;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E6, v1 as u8);
    16
}
/// $0796B0: move.b d4, $10e2ec.l
pub fn i_0796b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796B6;
    cpu.ir = 0x13C4;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EC, v1 as u8);
    16
}
/// $0796B6: lea.l $1001c0.l, a4
pub fn i_0796b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796BC;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x1001C0;
    20
}
/// $0796BC: move.b #$2, $44(a4)
pub fn i_0796bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796C2;
    cpu.ir = 0x197C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[4].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0796C2: andi.w #$ff, d0
pub fn i_0796c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796C6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0796C6: lsl.w #$2, d0
pub fn i_0796c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796C8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0796C8: lea.l $2df442.l, a1
pub fn i_0796c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796CE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF442;
    20
}
/// $0796CE: move.l (a1, d0.w), d0
pub fn i_0796ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796D2;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $0796D2: cmpi.l #$ffffffff, d0
pub fn i_0796d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796D8;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $0796D8: beq.w $796f0
pub fn i_0796d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x796F0; return 10; }
    8
}
/// $0796DC: movea.l d0, a1
pub fn i_0796dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796DE;
    cpu.ir = 0x2240;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $0796DE: jsr $4ae.l
pub fn i_0796de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x796E4);
    cpu.pc = 0x4AE;
    16
}
// ===== SetXN_0796e4  ($0796E4, 6 B, thunks/ccr_helpers.c)
/// $0796E4: ori.b #$11, ccr
pub fn i_0796e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796E8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0796E8: rts
pub fn i_0796e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_ClearLevel_0796ea  ($0796EA, 6 B, props/fx_explosions_platforms_crew_076000.s)
/// $0796EA: clr.b $10e39d.l
pub fn i_0796ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796F0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39D) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39D, r2 as u8);
    16
}
// ===== ClearXN_0796f0  ($0796F0, 6 B, thunks/ccr_helpers.c)
/// $0796F0: andi.b #$ee, ccr
pub fn i_0796f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796F4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0796F4: rts
pub fn i_0796f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_JumpIfBlocked_0796f6  ($0796F6, 216 B, props/fx_explosions_platforms_crew_076000.s)
/// $0796F6: tst.b $80(a6)
pub fn i_0796f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796FA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0796FA: bne.w $79760
pub fn i_0796fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x796FE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79760; return 10; }
    8
}
/// $0796FE: clr.w d0
pub fn i_0796fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79700;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079700: jsr $5e3a2.l
pub fn i_079700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79706;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79706);
    cpu.pc = 0x5E3A2;
    16
}
/// $079706: bcc.w $79760
pub fn i_079706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7970A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79760; return 10; }
    8
}
/// $07970A: movem.l a6, -(a7)
pub fn i_07970a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7970E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $07970E: move.l a0, $5c(a6)
pub fn i_07970e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79712;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $079712: movea.l a0, a6
pub fn i_079712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79714;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $079714: jsr $2a8c0.l
pub fn i_079714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7971A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7971A);
    cpu.pc = 0x2A8C0;
    16
}
/// $07971A: movem.l (a7)+, a6
pub fn i_07971a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7971E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $07971E: bcs.w $79760
pub fn i_07971e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79722;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x79760; return 10; }
    8
}
/// $079722: movea.l $5c(a6), a0
pub fn i_079722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79726;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079726: move.w $22(a0), d1
pub fn i_079726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7972A;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07972A: move.w $24(a0), d2
pub fn i_07972a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7972E;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07972E: addi.w #$18, d1
pub fn i_07972e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79732;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079732: subq.w #$8, d2
pub fn i_079732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79734;
    cpu.ir = 0x5142;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079734: jsr $27db2.l
pub fn i_079734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7973A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7973A);
    cpu.pc = 0x27DB2;
    16
}
/// $07973A: cmpi.b #$40, d7
pub fn i_07973a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7973E;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $07973E: beq.w $79760
pub fn i_07973e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79742;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79760; return 10; }
    8
}
/// $079742: move.b #$1, $80(a6)
pub fn i_079742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79748;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079748: ori.b #$1, $10e2e4.l
pub fn i_079748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79750;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2E4) as u32;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E4, r2 as u8);
    20
}
/// $079750: ori.b #$1, $10e2e5.l
pub fn i_079750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79758;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2E5) as u32;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E5, r2 as u8);
    20
}
/// $079758: ori.b #$1, $10e2e6.l
pub fn i_079758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79760;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2E6) as u32;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2E6, r2 as u8);
    20
}
/// $079760: tst.b $81(a6)
pub fn i_079760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79764;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $079764: bne.w $797cc
pub fn i_079764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79768;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x797CC; return 10; }
    8
}
/// $079768: move.w #$1, d0
pub fn i_079768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7976C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07976C: jsr $5e3a2.l
pub fn i_07976c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79772;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79772);
    cpu.pc = 0x5E3A2;
    16
}
/// $079772: bcc.w $797cc
pub fn i_079772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79776;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x797CC; return 10; }
    8
}
/// $079776: movem.l a6, -(a7)
pub fn i_079776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7977A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $07977A: move.l a0, $5c(a6)
pub fn i_07977a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7977E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07977E: movea.l a0, a6
pub fn i_07977e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79780;
    cpu.ir = 0x2C48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[6] = v1;
    4
}
/// $079780: jsr $2a8c0.l
pub fn i_079780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79786;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79786);
    cpu.pc = 0x2A8C0;
    16
}
/// $079786: movem.l (a7)+, a6
pub fn i_079786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7978A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $07978A: bcs.w $797cc
pub fn i_07978a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7978E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x797CC; return 10; }
    8
}
/// $07978E: movea.l $5c(a6), a0
pub fn i_07978e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79792;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079792: move.w $22(a0), d1
pub fn i_079792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79796;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079796: move.w $24(a0), d2
pub fn i_079796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7979A;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07979A: addi.w #$18, d1
pub fn i_07979a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7979E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x18, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07979E: subq.w #$8, d2
pub fn i_07979e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797A0;
    cpu.ir = 0x5142;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0797A0: jsr $27db2.l
pub fn i_0797a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x797A6);
    cpu.pc = 0x27DB2;
    16
}
/// $0797A6: cmpi.b #$40, d7
pub fn i_0797a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797AA;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x40, Size::Byte, false);
    8
}
/// $0797AA: beq.w $797cc
pub fn i_0797aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x797CC; return 10; }
    8
}
/// $0797AE: move.b #$1, $81(a6)
pub fn i_0797ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x81);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0797B4: ori.b #$1, $10e2ea.l
pub fn i_0797b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797BC;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2EA) as u32;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EA, r2 as u8);
    20
}
/// $0797BC: ori.b #$1, $10e2eb.l
pub fn i_0797bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797C4;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2EB) as u32;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EB, r2 as u8);
    20
}
/// $0797C4: ori.b #$1, $10e2ec.l
pub fn i_0797c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797CC;
    cpu.ir = 0x39;
    let v1 = bus.read8(0x10E2EC) as u32;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E2EC, r2 as u8);
    20
}
/// $0797CC: rts
pub fn i_0797cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_DebugWarpKeys_0797ce  ($0797CE, 90 B, props/fx_explosions_platforms_crew_076000.s)
/// $0797CE: lea.l $10e200.l, a0
pub fn i_0797ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E200;
    20
}
/// $0797D4: move.b $2(a0), d0
pub fn i_0797d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797D8;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0797D8: move.b $3(a0), d1
pub fn i_0797d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797DC;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0797DC: move.b d1, d2
pub fn i_0797dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797DE;
    cpu.ir = 0x1401;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0797DE: andi.b #$80, d0
pub fn i_0797de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797E2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0797E2: andi.b #$40, d1
pub fn i_0797e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797E6;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0797E6: andi.b #$20, d2
pub fn i_0797e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797EA;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x20) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0797EA: tst.b d0
pub fn i_0797ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797EC;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0797EC: beq.w $79826
pub fn i_0797ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79826; return 10; }
    8
}
/// $0797F0: tst.b d1
pub fn i_0797f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797F2;
    cpu.ir = 0x4A01;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0797F2: beq.w $7981a
pub fn i_0797f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7981A; return 10; }
    8
}
/// $0797F6: move.w #$a0, $22(a6)
pub fn i_0797f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x797FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0797FC: move.w #$180, $24(a6)
pub fn i_0797fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79802;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079802: move.l #$2df456, $4c(a6)
pub fn i_079802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7980A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2DF456;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07980A: jsr $283ca.l
pub fn i_07980a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79810;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79810);
    cpu.pc = 0x283CA;
    16
}
/// $079810: jsr $283d8.l
pub fn i_079810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79816;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79816);
    cpu.pc = 0x283D8;
    16
}
/// $079816: bra.w $79826
pub fn i_079816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7981A;
    cpu.ir = 0x6000;
    cpu.pc = 0x79826;
    10
}
/// $07981A: tst.b d2
pub fn i_07981a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7981C;
    cpu.ir = 0x4A02;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $07981C: beq.w $79826
pub fn i_07981c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79820;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79826; return 10; }
    8
}
/// $079820: clr.w $106e92.l
pub fn i_079820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79826;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106E92) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106E92, r2 as u16);
    16
}
/// $079826: rts
pub fn i_079826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79828;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AutoDemo_RetGateB_079828  ($079828, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079828: movea.l $8(a6), a1
pub fn i_079828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7982C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07982C: move.b $10(a6), d0
pub fn i_07982c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79830;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079830: cmp.b $10(a1), d0
pub fn i_079830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79834;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $079834: bcs.w $7983e
pub fn i_079834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79838;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7983E; return 10; }
    8
}
// ===== ClearXN_079838  ($079838, 6 B, thunks/ccr_helpers.c)
/// $079838: andi.b #$ee, ccr
pub fn i_079838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7983C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07983C: rts
pub fn i_07983c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7983E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07983e  ($07983E, 6 B, thunks/ccr_helpers.c)
/// $07983E: ori.b #$11, ccr
pub fn i_07983e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79842;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $079842: rts
pub fn i_079842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79844;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hit_TestPlayersAndWeapons_079844  ($079844, 98 B, props/fx_explosions_platforms_crew_076000.s)
/// $079844: move.l a2, d0
pub fn i_079844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79846;
    cpu.ir = 0x200A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079846: cmpi.l #$ffffffff, d0
pub fn i_079846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7984C;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $07984C: beq.w $798aa
pub fn i_07984c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79850;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x798AA; return 10; }
    8
}
/// $079850: movem.l a2, -(a7)
pub fn i_079850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79854;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $079854: clr.w d0
pub fn i_079854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79856;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079856: jsr $5e3a2.l
pub fn i_079856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7985C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7985C);
    cpu.pc = 0x5E3A2;
    16
}
/// $07985C: movem.l (a7)+, a2
pub fn i_07985c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79860;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $079860: bcc.w $7987c
pub fn i_079860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79864;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7987C; return 10; }
    8
}
/// $079864: movea.l $74(a0), a1
pub fn i_079864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79868;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079868: jsr $798ac(pc)
pub fn i_079868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7986C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7986C);
    cpu.pc = 0x798AC;
    16
}
/// $07986C: movea.l $7c(a0), a1
pub fn i_07986c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79870;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079870: jsr $798ac(pc)
pub fn i_079870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79874;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79874);
    cpu.pc = 0x798AC;
    16
}
/// $079874: movea.l $78(a0), a1
pub fn i_079874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79878;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079878: jsr $798ac(pc)
pub fn i_079878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7987C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7987C);
    cpu.pc = 0x798AC;
    16
}
/// $07987C: movem.l a2, -(a7)
pub fn i_07987c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79880;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $079880: move.w #$1, d0
pub fn i_079880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79884;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079884: jsr $5e3a2.l
pub fn i_079884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7988A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7988A);
    cpu.pc = 0x5E3A2;
    16
}
/// $07988A: movem.l (a7)+, a2
pub fn i_07988a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7988E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $07988E: bcc.w $798aa
pub fn i_07988e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79892;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x798AA; return 10; }
    8
}
/// $079892: movea.l $74(a0), a1
pub fn i_079892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79896;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079896: jsr $798ac(pc)
pub fn i_079896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7989A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7989A);
    cpu.pc = 0x798AC;
    16
}
/// $07989A: movea.l $7c(a0), a1
pub fn i_07989a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7989E;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07989E: jsr $798ac(pc)
pub fn i_07989e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x798A2);
    cpu.pc = 0x798AC;
    16
}
/// $0798A2: movea.l $78(a0), a1
pub fn i_0798a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798A6;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
// ===== JsrPcThunk_0798a6  ($0798A6, 6 B, thunks/jsr_pc_thunks.c)
/// $0798A6: jsr $798ac(pc)
pub fn i_0798a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x798AA);
    cpu.pc = 0x798AC;
    16
}
/// $0798AA: rts
pub fn i_0798aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CheckBoxOverlapWithSelector_0798AC  ($0798AC, 164 B, entity/check_box_overlap_selector_0798ac.s)
/// $0798AC: move.l a1, d0
pub fn i_0798ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798AE;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0798AE: cmpi.l #$ffffffff, d0
pub fn i_0798ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798B4;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $0798B4: beq.w $7994e
pub fn i_0798b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7994E; return 10; }
    8
}
/// $0798B8: move.w (a2), d0
pub fn i_0798b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798BA;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $0798BA: move.w $2(a2), d1
pub fn i_0798ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798BE;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798BE: move.w $22(a6), d2
pub fn i_0798be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798C2;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798C2: add.w d2, d0
pub fn i_0798c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798C4;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0798C4: add.w d2, d1
pub fn i_0798c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798C6;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0798C6: move.w $22(a1), d2
pub fn i_0798c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798CA;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798CA: cmp.w d0, d2
pub fn i_0798ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798CC;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0798CC: blt.w $7994e
pub fn i_0798cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798D0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7994E; return 10; }
    8
}
/// $0798D0: cmp.w d1, d2
pub fn i_0798d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798D2;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0798D2: bgt.w $7994e
pub fn i_0798d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798D6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7994E; return 10; }
    8
}
/// $0798D6: move.w $4(a2), d0
pub fn i_0798d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798DA;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798DA: move.w $6(a2), d1
pub fn i_0798da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798DE;
    cpu.ir = 0x322A;
    let ad2 = cpu.a[2].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798DE: move.w $24(a6), d2
pub fn i_0798de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798E2;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798E2: add.w d2, d0
pub fn i_0798e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798E4;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0798E4: add.w d2, d1
pub fn i_0798e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798E6;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0798E6: move.w $24(a1), d2
pub fn i_0798e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798EA;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0798EA: cmp.w d0, d2
pub fn i_0798ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798EC;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0798EC: blt.w $7994e
pub fn i_0798ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798F0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7994E; return 10; }
    8
}
/// $0798F0: cmp.w d1, d2
pub fn i_0798f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798F2;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0798F2: bgt.w $7994e
pub fn i_0798f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798F6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7994E; return 10; }
    8
}
/// $0798F6: move.w (a2), d0
pub fn i_0798f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798F8;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $0798F8: move.w d0, d1
pub fn i_0798f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798FA;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0798FA: add.w $22(a6), d0
pub fn i_0798fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x798FE;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0798FE: sub.w $22(a1), d0
pub fn i_0798fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79902;
    cpu.ir = 0x9069;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $079902: move.w $2(a2), d2
pub fn i_079902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79906;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079906: sub.w d1, d2
pub fn i_079906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79908;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079908: asr.w #$1, d2
pub fn i_079908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7990A;
    cpu.ir = 0xE242;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07990A: cmp.w d2, d0
pub fn i_07990a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7990C;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $07990C: ble.w $7991e
pub fn i_07990c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79910;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7991E; return 10; }
    8
}
/// $079910: move.w (a2), d1
pub fn i_079910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79912;
    cpu.ir = 0x3212;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $079912: neg.w d1
pub fn i_079912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79914;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079914: add.w $22(a6), d1
pub fn i_079914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79918;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $079918: move.w $22(a1), d0
pub fn i_079918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7991C;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07991C: sub.w d1, d0
pub fn i_07991c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7991E;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07991E: neg.w d0
pub fn i_07991e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79920;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $079920: move.b $8(a2), d3
pub fn i_079920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79924;
    cpu.ir = 0x162A;
    let ad2 = cpu.a[2].wrapping_add(0x8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079924: andi.w #$3, d3
pub fn i_079924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79928;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079928: lsl.w #$2, d3
pub fn i_079928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7992A;
    cpu.ir = 0xE54B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07992A: lea.l $2df4aa.l, a3
pub fn i_07992a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79930;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x2DF4AA;
    20
}
/// $079930: movea.l (a3, d3.w), a3
pub fn i_079930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79934;
    cpu.ir = 0x2673;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    18
}
/// $079934: jsr (a3)
pub fn i_079934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79936;
    cpu.ir = 0x4E93;
    cpu.push32(bus, 0x79936);
    cpu.jump_to(cpu.a[3]);
    16
}
/// $079936: add.w $24(a6), d0
pub fn i_079936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7993A;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07993A: sub.w $24(a1), d0
pub fn i_07993a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7993E;
    cpu.ir = 0x9069;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07993E: cmp.w $6(a2), d0
pub fn i_07993e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79942;
    cpu.ir = 0xB06A;
    let ad2 = cpu.a[2].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $079942: ble.w $7994a
pub fn i_079942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79946;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7994A; return 10; }
    8
}
/// $079946: move.w $6(a2), d0
pub fn i_079946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7994A;
    cpu.ir = 0x302A;
    let ad2 = cpu.a[2].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07994A: move.w d0, $8e(a1)
pub fn i_07994a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7994E;
    cpu.ir = 0x3340;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07994E: rts
pub fn i_07994e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79950;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_00079950  ($079950, 2 B, thunks/trivial_rts.c)
/// $079950: rts
pub fn i_079950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79952;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scale_X2_079952  ($079952, 4 B, props/fx_explosions_platforms_crew_076000.s)
/// $079952: add.w d0, d0
pub fn i_079952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79954;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079954: rts
pub fn i_079954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79956;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scale_X1_5_079956  ($079956, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $079956: move.w d0, d1
pub fn i_079956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79958;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079958: asr.w #$1, d1
pub fn i_079958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7995A;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07995A: add.w d1, d0
pub fn i_07995a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7995C;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07995C: rts
pub fn i_07995c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7995E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hit_TestPlayersDefault_07995e  ($07995E, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $07995E: lea.l $2df4ba.l, a2
pub fn i_07995e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79964;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2DF4BA;
    20
}
/// $079964: jmp $79844(pc)
pub fn i_079964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79968;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x79844;
    8
}
// ===== Rank_DelayByDifficulty_079970  ($079970, 24 B, props/fx_explosions_platforms_crew_076000.s)
/// $079970: lea.l $10fd84.l, a3
pub fn i_079970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79976;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10FD84;
    20
}
/// $079976: moveq #$0, d0
pub fn i_079976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79978;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079978: move.b $8(a3), d0
pub fn i_079978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7997C;
    cpu.ir = 0x102B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07997C: lea.l $79968(pc), a0
pub fn i_07997c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79980;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x79968;
    16
}
/// $079980: lsl.w #$1, d0
pub fn i_079980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79982;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $079982: move.w (a0, d0.w), d0
pub fn i_079982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79986;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $079986: rts
pub fn i_079986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79988;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rank_RetGate_079988  ($079988, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079988: movea.l $8(a6), a1
pub fn i_079988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7998C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07998C: move.b $10(a6), d0
pub fn i_07998c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79990;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079990: cmp.b $10(a1), d0
pub fn i_079990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79994;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $079994: bcs.w $7999e
pub fn i_079994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79998;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7999E; return 10; }
    8
}
// ===== ClearXN_079998  ($079998, 6 B, thunks/ccr_helpers.c)
/// $079998: andi.b #$ee, ccr
pub fn i_079998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7999C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07999C: rts
pub fn i_07999c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7999E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07999e  ($07999E, 6 B, thunks/ccr_helpers.c)
/// $07999E: ori.b #$11, ccr
pub fn i_07999e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799A2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0799A2: rts
pub fn i_0799a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rank_SubIndex_0799a4  ($0799A4, 58 B, props/fx_explosions_platforms_crew_076000.s)
/// $0799A4: move.b $10fd8b.l, d1
pub fn i_0799a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799AA;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x10FD8B) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0799AA: cmpi.b #$1, $10fdaf.l
pub fn i_0799aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799B2;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDAF) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $0799B2: bne.w $799ba
pub fn i_0799b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x799BA; return 10; }
    8
}
/// $0799B6: move.b #$4, d1
pub fn i_0799b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799BA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0799BA: move.l a0, -(a7)
pub fn i_0799ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799BC;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $0799BC: move.w d0, -(a7)
pub fn i_0799bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799BE;
    cpu.ir = 0x3F00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $0799BE: move.b d1, -(a7)
pub fn i_0799be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799C0;
    cpu.ir = 0x1F01;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write8(ad2, v1 as u8);
    10
}
/// $0799C0: jsr $2ac4c.l
pub fn i_0799c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x799C6);
    cpu.pc = 0x2AC4C;
    16
}
/// $0799C6: scs.b d1
pub fn i_0799c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799C8;
    cpu.ir = 0x55C1;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let v2 = cpu.d[1] & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (c1 & 0xFF);
    4
}
/// $0799C8: neg.b d1
pub fn i_0799c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799CA;
    cpu.ir = 0x4401;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.set(StatusRegister::V, v1 == 0x80); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0799CA: add.b (a7)+, d1
pub fn i_0799ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799CC;
    cpu.ir = 0xD21F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    8
}
/// $0799CC: move.w (a7)+, d0
pub fn i_0799cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799CE;
    cpu.ir = 0x301F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $0799CE: movea.l (a7)+, a0
pub fn i_0799ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799D0;
    cpu.ir = 0x205F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $0799D0: cmpi.b #$8, d1
pub fn i_0799d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799D4;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $0799D4: bcs.w $799dc
pub fn i_0799d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799D8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x799DC; return 10; }
    8
}
/// $0799D8: move.b #$7, d1
pub fn i_0799d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799DC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0799DC: rts
pub fn i_0799dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tbl_Decode2D_0799DE  ($0799DE, 48 B, kernel/table_decode_2d_0799de.s)
/// $0799DE: cmpi.w #$2, (a0)+
pub fn i_0799de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799E2;
    cpu.ir = 0xC58;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    12
}
/// $0799E2: beq.w $79a0e
pub fn i_0799e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79A0E; return 10; }
    8
}
/// $0799E6: move.b $11(a6), d0
pub fn i_0799e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799EA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x11);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0799EA: andi.w #$3, d0
pub fn i_0799ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799EE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0799EE: add.w d0, d0
pub fn i_0799ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799F0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0799F0: add.w d0, d0
pub fn i_0799f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799F2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0799F2: move.b $106ed1.l, d1
pub fn i_0799f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799F8;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106ED1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0799F8: andi.w #$2, d1
pub fn i_0799f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799FC;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x2) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0799FC: add.w d1, d0
pub fn i_0799fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x799FE;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0799FE: bsr.b $799a4
pub fn i_0799fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A00;
    cpu.ir = 0x61A4;
    cpu.push32(bus, 0x79A00);
    cpu.pc = 0x799A4;
    18
}
/// $079A00: andi.w #$7, d1
pub fn i_079a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A04;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079A04: lsl.w #$4, d1
pub fn i_079a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A06;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $079A06: add.w d1, d0
pub fn i_079a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A08;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079A08: move.w (a0, d0.w), d0
pub fn i_079a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A0C;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $079A0C: rts
pub fn i_079a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Tbl_DecodeShort_079A0E  ($079A0E, 66 B, props/fx_explosions_platforms_crew_076000.s)
/// $079A0E: move.b $11(a6), d0
pub fn i_079a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A12;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x11);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079A12: andi.w #$3, d0
pub fn i_079a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A16;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079A16: add.w d0, d0
pub fn i_079a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A18;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079A18: move.b $106ed1.l, d1
pub fn i_079a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A1E;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106ED1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $079A1E: subq.b #$1, d1
pub fn i_079a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A20;
    cpu.ir = 0x5301;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $079A20: andi.w #$1, d1
pub fn i_079a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A24;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079A24: add.w d1, d0
pub fn i_079a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A26;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079A26: jsr $799a4(pc)
pub fn i_079a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79A2A);
    cpu.pc = 0x799A4;
    16
}
/// $079A2A: andi.w #$7, d1
pub fn i_079a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A2E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079A2E: lsl.w #$3, d1
pub fn i_079a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A30;
    cpu.ir = 0xE749;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $079A30: add.w d1, d0
pub fn i_079a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A32;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $079A32: move.b (a0, d0.w), d0
pub fn i_079a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A36;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $079A36: andi.w #$ff, d0
pub fn i_079a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A3A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079A3A: lsl.w #$4, d0
pub fn i_079a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A3C;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $079A3C: lea.l $40(a0, d0.w), a0
pub fn i_079a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A40;
    cpu.ir = 0x41F0;
    let ad1 = cpu.a[0].wrapping_add(0x40).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    cpu.a[0] = ad1;
    18
}
/// $079A40: jsr $5e9b6.l
pub fn i_079a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79A46);
    cpu.pc = 0x5E9B6;
    16
}
/// $079A46: andi.w #$e, d0
pub fn i_079a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A4A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xE) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $079A4A: move.w (a0, d0.w), d0
pub fn i_079a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A4E;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $079A4E: rts
pub fn i_079a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_RetGate_079a50  ($079A50, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079A50: movea.l $8(a6), a1
pub fn i_079a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A54;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079A54: move.b $10(a6), d0
pub fn i_079a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A58;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079A58: cmp.b $10(a1), d0
pub fn i_079a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A5C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $079A5C: bcs.w $79a66
pub fn i_079a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A60;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x79A66; return 10; }
    8
}
// ===== ClearXN_079a60  ($079A60, 6 B, thunks/ccr_helpers.c)
/// $079A60: andi.b #$ee, ccr
pub fn i_079a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A64;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $079A64: rts
pub fn i_079a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_079a66  ($079A66, 6 B, thunks/ccr_helpers.c)
/// $079A66: ori.b #$11, ccr
pub fn i_079a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A6A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $079A6A: rts
pub fn i_079a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Tmpl125_079a6c  ($079A6C, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $079A6C: move.w #$19, d1
pub fn i_079a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A70;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079A70: jsr $236e.l
pub fn i_079a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79A76);
    cpu.pc = 0x236E;
    16
}
/// $079A76: move.w #$1, $66(a6)
pub fn i_079a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A7C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079A7C: jsr $267e2.l
pub fn i_079a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79A82);
    cpu.pc = 0x267E2;
    16
}
// ===== SetTaskHandler_079a82  ($079A82, 8 B, kernel/task_handlers.c)
/// $079A82: lea.l $79a8a(pc), a1
pub fn i_079a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79A8A;
    16
}
/// $079A86: move.l a1, (a6)
pub fn i_079a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079A88: rts
pub fn i_079a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Jump_Run_079a8a  ($079A8A, 82 B, props/fx_explosions_platforms_crew_076000.s)
/// $079A8A: jsr $5e7c0.l
pub fn i_079a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79A90);
    cpu.pc = 0x5E7C0;
    16
}
/// $079A90: move.w #$8000, d0
pub fn i_079a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A94;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079A94: jsr $28134.l
pub fn i_079a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79A9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79A9A);
    cpu.pc = 0x28134;
    16
}
/// $079A9A: andi.w #$ffe3, $38(a6)
pub fn i_079a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AA0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $079AA0: ori.w #$c, $38(a6)
pub fn i_079aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AA6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $079AA6: lea.l $29c0ae.l, a0
pub fn i_079aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AAC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C0AE;
    20
}
/// $079AAC: jsr $28cd4.l
pub fn i_079aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79AB2);
    cpu.pc = 0x28CD4;
    16
}
/// $079AB2: lea.l $79ab8(pc), a1
pub fn i_079ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79AB8;
    16
}
/// $079AB6: move.l a1, (a6)
pub fn i_079ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079AB8: jsr $2783a.l
pub fn i_079ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ABE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79ABE);
    cpu.pc = 0x2783A;
    16
}
/// $079ABE: jsr $28d70.l
pub fn i_079abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79AC4);
    cpu.pc = 0x28D70;
    16
}
/// $079AC4: jsr $2870a.l
pub fn i_079ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ACA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79ACA);
    cpu.pc = 0x2870A;
    16
}
/// $079ACA: bcc.w $79ad4
pub fn i_079aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ACE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79AD4; return 10; }
    8
}
/// $079ACE: lea.l $79ae4(pc), a1
pub fn i_079ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79AE4;
    16
}
/// $079AD2: move.l a1, (a6)
pub fn i_079ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079AD4: jsr $79b4a(pc)
pub fn i_079ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79AD8);
    cpu.pc = 0x79B4A;
    16
}
/// $079AD8: bcc.w $79ae2
pub fn i_079ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ADC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79AE2; return 10; }
    8
}
// ===== SetTaskHandler_079adc  ($079ADC, 8 B, kernel/task_handlers.c)
/// $079ADC: lea.l $79b42(pc), a1
pub fn i_079adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79B42;
    16
}
/// $079AE0: move.l a1, (a6)
pub fn i_079ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079AE2: rts
pub fn i_079ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Land_079ae4  ($079AE4, 54 B, props/fx_explosions_platforms_crew_076000.s)
/// $079AE4: bclr.b #$1, $12(a6)
pub fn i_079ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AEA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $079AEA: lea.l $29c1d4.l, a0
pub fn i_079aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AF0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C1D4;
    20
}
/// $079AF0: jsr $28cd4.l
pub fn i_079af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79AF6);
    cpu.pc = 0x28CD4;
    16
}
/// $079AF6: lea.l $79afc(pc), a1
pub fn i_079af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79AFC;
    16
}
/// $079AFA: move.l a1, (a6)
pub fn i_079afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79AFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079AFC: jsr $2783a.l
pub fn i_079afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B02);
    cpu.pc = 0x2783A;
    16
}
/// $079B02: jsr $28d70.l
pub fn i_079b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B08);
    cpu.pc = 0x28D70;
    16
}
/// $079B08: bcc.w $79b12
pub fn i_079b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B0C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79B12; return 10; }
    8
}
/// $079B0C: lea.l $79b22(pc), a1
pub fn i_079b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79B22;
    16
}
/// $079B10: move.l a1, (a6)
pub fn i_079b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079B12: jsr $79b4a(pc)
pub fn i_079b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79B16);
    cpu.pc = 0x79B4A;
    16
}
/// $079B16: bcc.w $79b20
pub fn i_079b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B1A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79B20; return 10; }
    8
}
// ===== SetTaskHandler_079b1a  ($079B1A, 8 B, kernel/task_handlers.c)
/// $079B1A: lea.l $79b42(pc), a1
pub fn i_079b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79B42;
    16
}
/// $079B1E: move.l a1, (a6)
pub fn i_079b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079B20: rts
pub fn i_079b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Flee_079b22  ($079B22, 32 B, props/fx_explosions_platforms_crew_076000.s)
/// $079B22: move.b #$28, $59(a6)
pub fn i_079b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B28;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079B28: lea.l $79b2e(pc), a1
pub fn i_079b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79B2E;
    16
}
/// $079B2C: move.l a1, (a6)
pub fn i_079b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079B2E: jsr $2783a.l
pub fn i_079b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B34);
    cpu.pc = 0x2783A;
    16
}
/// $079B34: jsr $28d70.l
pub fn i_079b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B3A);
    cpu.pc = 0x28D70;
    16
}
/// $079B3A: tst.b $59(a6)
pub fn i_079b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B3E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $079B3E: bne.w $79b48
pub fn i_079b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B42;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79B48; return 10; }
    8
}
// ===== Crew_Free_079b42  ($079B42, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $079B42: jmp $518.l
pub fn i_079b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B48;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $079B48: rts
pub fn i_079b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_FreeIfOffWorld_079b4a  ($079B4A, 14 B, props/fx_explosions_platforms_crew_076000.s)
/// $079B4A: lea.l $79b64(pc), a0
pub fn i_079b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B4E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x79B64;
    16
}
/// $079B4E: jsr $5dd5c.l
pub fn i_079b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B54);
    cpu.pc = 0x5DD5C;
    16
}
/// $079B54: bcc.w $79b5e
pub fn i_079b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79B5E; return 10; }
    8
}
// ===== SetC_079b58  ($079B58, 6 B, thunks/ccr_helpers.c)
/// $079B58: ori.b #$1, ccr
pub fn i_079b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B5C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $079B5C: rts
pub fn i_079b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_079b5e  ($079B5E, 6 B, thunks/ccr_helpers.c)
/// $079B5E: andi.b #$fe, ccr
pub fn i_079b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B62;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $079B62: rts
pub fn i_079b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_OffWorldBox_079b64  ($079B64, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $079B64: dc.w $fff0
pub fn i_079b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B66;
    cpu.ir = 0xFFF0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Crew_Hatch_079b6e  ($079B6E, 156 B, props/fx_explosions_platforms_crew_076000.s)
/// $079B6E: jsr $5e9b6.l
pub fn i_079b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B74);
    cpu.pc = 0x5E9B6;
    16
}
/// $079B74: btst.b #$0, d0
pub fn i_079b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B78;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $079B78: beq.w $79b82
pub fn i_079b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B7C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79B82; return 10; }
    8
}
/// $079B7C: jmp $79d9a.l
pub fn i_079b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B82;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x79D9A;
    8
}
/// $079B82: movea.l $c(a6), a0
pub fn i_079b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B86;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079B86: move.w $72(a6), d0
pub fn i_079b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B8A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079B8A: bset.b d0, $70(a0)
pub fn i_079b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B8E;
    cpu.ir = 0x1E8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $079B8E: move.w #$19, d1
pub fn i_079b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B92;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079B92: jsr $236e.l
pub fn i_079b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79B98);
    cpu.pc = 0x236E;
    16
}
/// $079B98: addq.w #$3, $38(a6)
pub fn i_079b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79B9C;
    cpu.ir = 0x566E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $079B9C: move.w $38(a6), d0
pub fn i_079b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BA0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079BA0: jsr $28134.l
pub fn i_079ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79BA6);
    cpu.pc = 0x28134;
    16
}
/// $079BA6: move.w #$1, $66(a6)
pub fn i_079ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BAC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079BAC: jsr $267e2.l
pub fn i_079bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79BB2);
    cpu.pc = 0x267E2;
    16
}
/// $079BB2: lea.l $29c0ae.l, a0
pub fn i_079bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C0AE;
    20
}
/// $079BB8: jsr $28cd4.l
pub fn i_079bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79BBE);
    cpu.pc = 0x28CD4;
    16
}
/// $079BBE: lea.l $79bc4(pc), a1
pub fn i_079bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79BC4;
    16
}
/// $079BC2: move.l a1, (a6)
pub fn i_079bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079BC4: jsr $7fdfa.l
pub fn i_079bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79BCA);
    cpu.pc = 0x7FDFA;
    16
}
/// $079BCA: jsr $28d70.l
pub fn i_079bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79BD0);
    cpu.pc = 0x28D70;
    16
}
/// $079BD0: movea.l $c(a6), a0
pub fn i_079bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BD4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079BD4: cmpi.b #$3, $21(a0)
pub fn i_079bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BDA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $079BDA: beq.w $79be8
pub fn i_079bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BDE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79BE8; return 10; }
    8
}
/// $079BDE: cmpi.b #$ff, $20(a0)
pub fn i_079bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BE4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $079BE4: bne.w $79bf0
pub fn i_079be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79BF0; return 10; }
    8
}
/// $079BE8: lea.l $77efe.l, a1
pub fn i_079be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BEE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77EFE;
    20
}
/// $079BEE: move.l a1, (a6)
pub fn i_079bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079BF0: jsr $2870a.l
pub fn i_079bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79BF6);
    cpu.pc = 0x2870A;
    16
}
/// $079BF6: bcc.w $79c00
pub fn i_079bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BFA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79C00; return 10; }
    8
}
/// $079BFA: lea.l $79c12(pc), a1
pub fn i_079bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79BFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79C12;
    16
}
/// $079BFE: move.l a1, (a6)
pub fn i_079bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079C00: jsr $7fde0.l
pub fn i_079c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C06);
    cpu.pc = 0x7FDE0;
    16
}
/// $079C06: bcc.w $79c10
pub fn i_079c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79C10; return 10; }
    8
}
// ===== SetTaskHandler_079c0a  ($079C0A, 8 B, kernel/task_handlers.c)
/// $079C0A: lea.l $79c68(pc), a1
pub fn i_079c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79C68;
    16
}
/// $079C0E: move.l a1, (a6)
pub fn i_079c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079C10: rts
pub fn i_079c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hatch_Land_079c12  ($079C12, 34 B, props/fx_explosions_platforms_crew_076000.s)
/// $079C12: lea.l $29c1d4.l, a0
pub fn i_079c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C18;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C1D4;
    20
}
/// $079C18: jsr $28cd4.l
pub fn i_079c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C1E);
    cpu.pc = 0x28CD4;
    16
}
/// $079C1E: lea.l $79c24(pc), a1
pub fn i_079c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79C24;
    16
}
/// $079C22: move.l a1, (a6)
pub fn i_079c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079C24: jsr $7fdfa.l
pub fn i_079c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C2A);
    cpu.pc = 0x7FDFA;
    16
}
/// $079C2A: jsr $28d70.l
pub fn i_079c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C30);
    cpu.pc = 0x28D70;
    16
}
/// $079C30: bcc.w $79c3a
pub fn i_079c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C34;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79C3A; return 10; }
    8
}
// ===== SetTaskHandler_079c34  ($079C34, 8 B, kernel/task_handlers.c)
/// $079C34: lea.l $79c3c(pc), a1
pub fn i_079c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79C3C;
    16
}
/// $079C38: move.l a1, (a6)
pub fn i_079c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079C3A: rts
pub fn i_079c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hatch_Flee_079c3c  ($079C3C, 44 B, props/fx_explosions_platforms_crew_076000.s)
/// $079C3C: move.b #$28, $59(a6)
pub fn i_079c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C42;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079C42: lea.l $79c48(pc), a1
pub fn i_079c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79C48;
    16
}
/// $079C46: move.l a1, (a6)
pub fn i_079c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079C48: jsr $7fdfa.l
pub fn i_079c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C4E);
    cpu.pc = 0x7FDFA;
    16
}
/// $079C4E: jsr $28d70.l
pub fn i_079c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C54);
    cpu.pc = 0x28D70;
    16
}
/// $079C54: tst.b $59(a6)
pub fn i_079c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C58;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $079C58: bne.w $79c6e
pub fn i_079c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C5C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79C6E; return 10; }
    8
}
/// $079C5C: movea.l $c(a6), a0
pub fn i_079c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C60;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079C60: move.w $72(a6), d0
pub fn i_079c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079C64: bclr.b d0, $70(a0)
pub fn i_079c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C68;
    cpu.ir = 0x1A8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== Crew_Hatch_Free_079c68  ($079C68, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $079C68: jmp $518.l
pub fn i_079c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C6E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $079C6E: rts
pub fn i_079c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_RetGateB_079c70  ($079C70, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079C70: movea.l $8(a6), a1
pub fn i_079c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C74;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079C74: move.b $10(a6), d0
pub fn i_079c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C78;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079C78: cmp.b $10(a1), d0
pub fn i_079c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C7C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $079C7C: bcs.w $79c86
pub fn i_079c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C80;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x79C86; return 10; }
    8
}
// ===== ClearXN_079c80  ($079C80, 6 B, thunks/ccr_helpers.c)
/// $079C80: andi.b #$ee, ccr
pub fn i_079c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C84;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $079C84: rts
pub fn i_079c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_079c86  ($079C86, 6 B, thunks/ccr_helpers.c)
/// $079C86: ori.b #$11, ccr
pub fn i_079c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C8A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $079C8A: rts
pub fn i_079c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Tmpl126_079c8c  ($079C8C, 22 B, props/fx_explosions_platforms_crew_076000.s)
/// $079C8C: move.w #$19, d1
pub fn i_079c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C90;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079C90: jsr $236e.l
pub fn i_079c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79C96);
    cpu.pc = 0x236E;
    16
}
/// $079C96: move.w #$1, $66(a6)
pub fn i_079c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79C9C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079C9C: jsr $267e2.l
pub fn i_079c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CA2);
    cpu.pc = 0x267E2;
    16
}
// ===== SetTaskHandler_079ca2  ($079CA2, 8 B, kernel/task_handlers.c)
/// $079CA2: lea.l $79caa(pc), a1
pub fn i_079ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79CAA;
    16
}
/// $079CA6: move.l a1, (a6)
pub fn i_079ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079CA8: rts
pub fn i_079ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_JumpB_Run_079caa  ($079CAA, 82 B, props/fx_explosions_platforms_crew_076000.s)
/// $079CAA: jsr $5e7c0.l
pub fn i_079caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CB0);
    cpu.pc = 0x5E7C0;
    16
}
/// $079CB0: move.w #$8000, d0
pub fn i_079cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CB4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079CB4: jsr $28134.l
pub fn i_079cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CBA);
    cpu.pc = 0x28134;
    16
}
/// $079CBA: andi.w #$ffe3, $38(a6)
pub fn i_079cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CC0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $079CC0: ori.w #$c, $38(a6)
pub fn i_079cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CC6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $079CC6: lea.l $29c244.l, a0
pub fn i_079cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CCC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C244;
    20
}
/// $079CCC: jsr $28cd4.l
pub fn i_079ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CD2);
    cpu.pc = 0x28CD4;
    16
}
/// $079CD2: lea.l $79cd8(pc), a1
pub fn i_079cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79CD8;
    16
}
/// $079CD6: move.l a1, (a6)
pub fn i_079cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079CD8: jsr $2783a.l
pub fn i_079cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CDE);
    cpu.pc = 0x2783A;
    16
}
/// $079CDE: jsr $28d70.l
pub fn i_079cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CE4);
    cpu.pc = 0x28D70;
    16
}
/// $079CE4: jsr $2870a.l
pub fn i_079ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79CEA);
    cpu.pc = 0x2870A;
    16
}
/// $079CEA: bcc.w $79cf4
pub fn i_079cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79CF4; return 10; }
    8
}
/// $079CEE: lea.l $79d04(pc), a1
pub fn i_079cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79D04;
    16
}
/// $079CF2: move.l a1, (a6)
pub fn i_079cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079CF4: jsr $79d7e(pc)
pub fn i_079cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79CF8);
    cpu.pc = 0x79D7E;
    16
}
/// $079CF8: bcc.w $79d02
pub fn i_079cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79CFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79D02; return 10; }
    8
}
// ===== SetTaskHandler_079cfc  ($079CFC, 8 B, kernel/task_handlers.c)
/// $079CFC: lea.l $79d76(pc), a1
pub fn i_079cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79D76;
    16
}
/// $079D00: move.l a1, (a6)
pub fn i_079d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079D02: rts
pub fn i_079d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_LandB_079d04  ($079D04, 74 B, props/fx_explosions_platforms_crew_076000.s)
/// $079D04: tst.b $10fd8f.l
pub fn i_079d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D0A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $079D0A: bne.w $79d18
pub fn i_079d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D0E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79D18; return 10; }
    8
}
/// $079D0E: move.w #$1d5, d1
pub fn i_079d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D12;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079D12: jsr $236e.l
pub fn i_079d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D18);
    cpu.pc = 0x236E;
    16
}
/// $079D18: bclr.b #$1, $12(a6)
pub fn i_079d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D1E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $079D1E: lea.l $29c28c.l, a0
pub fn i_079d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C28C;
    20
}
/// $079D24: jsr $28cd4.l
pub fn i_079d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D2A);
    cpu.pc = 0x28CD4;
    16
}
/// $079D2A: lea.l $79d30(pc), a1
pub fn i_079d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79D30;
    16
}
/// $079D2E: move.l a1, (a6)
pub fn i_079d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079D30: jsr $2783a.l
pub fn i_079d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D36);
    cpu.pc = 0x2783A;
    16
}
/// $079D36: jsr $28d70.l
pub fn i_079d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D3C);
    cpu.pc = 0x28D70;
    16
}
/// $079D3C: bcc.w $79d46
pub fn i_079d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79D46; return 10; }
    8
}
/// $079D40: lea.l $79d56(pc), a1
pub fn i_079d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79D56;
    16
}
/// $079D44: move.l a1, (a6)
pub fn i_079d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079D46: jsr $79d7e(pc)
pub fn i_079d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x79D4A);
    cpu.pc = 0x79D7E;
    16
}
/// $079D4A: bcc.w $79d54
pub fn i_079d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79D54; return 10; }
    8
}
// ===== SetTaskHandler_079d4e  ($079D4E, 8 B, kernel/task_handlers.c)
/// $079D4E: lea.l $79d76(pc), a1
pub fn i_079d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79D76;
    16
}
/// $079D52: move.l a1, (a6)
pub fn i_079d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079D54: rts
pub fn i_079d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_FleeB_079d56  ($079D56, 32 B, props/fx_explosions_platforms_crew_076000.s)
/// $079D56: move.b #$28, $59(a6)
pub fn i_079d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079D5C: lea.l $79d62(pc), a1
pub fn i_079d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79D62;
    16
}
/// $079D60: move.l a1, (a6)
pub fn i_079d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079D62: jsr $2783a.l
pub fn i_079d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D68);
    cpu.pc = 0x2783A;
    16
}
/// $079D68: jsr $28d70.l
pub fn i_079d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D6E);
    cpu.pc = 0x28D70;
    16
}
/// $079D6E: tst.b $59(a6)
pub fn i_079d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D72;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $079D72: bne.w $79d7c
pub fn i_079d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79D7C; return 10; }
    8
}
// ===== Crew_FreeB_079d76  ($079D76, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $079D76: jmp $518.l
pub fn i_079d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D7C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $079D7C: rts
pub fn i_079d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_FreeIfOffWorldB_079d7e  ($079D7E, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079D7E: lea.l $2df4e2.l, a0
pub fn i_079d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D84;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF4E2;
    20
}
/// $079D84: jsr $5dd5c.l
pub fn i_079d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79D8A);
    cpu.pc = 0x5DD5C;
    16
}
/// $079D8A: bcc.w $79d94
pub fn i_079d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79D94; return 10; }
    8
}
// ===== SetC_079d8e  ($079D8E, 6 B, thunks/ccr_helpers.c)
/// $079D8E: ori.b #$1, ccr
pub fn i_079d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D92;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $079D92: rts
pub fn i_079d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_079d94  ($079D94, 6 B, thunks/ccr_helpers.c)
/// $079D94: andi.b #$fe, ccr
pub fn i_079d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D98;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $079D98: rts
pub fn i_079d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_HatchB_079d9a  ($079D9A, 136 B, props/fx_explosions_platforms_crew_076000.s)
/// $079D9A: movea.l $c(a6), a0
pub fn i_079d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79D9E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079D9E: move.w $72(a6), d0
pub fn i_079d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DA2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079DA2: bset.b d0, $70(a0)
pub fn i_079da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DA6;
    cpu.ir = 0x1E8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $079DA6: move.w #$19, d1
pub fn i_079da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DAA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079DAA: jsr $236e.l
pub fn i_079daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79DB0);
    cpu.pc = 0x236E;
    16
}
/// $079DB0: addq.w #$3, $38(a6)
pub fn i_079db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DB4;
    cpu.ir = 0x566E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $079DB4: move.w $38(a6), d0
pub fn i_079db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DB8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079DB8: jsr $28134.l
pub fn i_079db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79DBE);
    cpu.pc = 0x28134;
    16
}
/// $079DBE: move.w #$1, $66(a6)
pub fn i_079dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DC4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079DC4: jsr $267e2.l
pub fn i_079dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79DCA);
    cpu.pc = 0x267E2;
    16
}
/// $079DCA: lea.l $29c244.l, a0
pub fn i_079dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DD0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C244;
    20
}
/// $079DD0: jsr $28cd4.l
pub fn i_079dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79DD6);
    cpu.pc = 0x28CD4;
    16
}
/// $079DD6: lea.l $79ddc(pc), a1
pub fn i_079dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79DDC;
    16
}
/// $079DDA: move.l a1, (a6)
pub fn i_079dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079DDC: jsr $7fdfa.l
pub fn i_079ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79DE2);
    cpu.pc = 0x7FDFA;
    16
}
/// $079DE2: jsr $28d70.l
pub fn i_079de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79DE8);
    cpu.pc = 0x28D70;
    16
}
/// $079DE8: movea.l $c(a6), a0
pub fn i_079de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DEC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079DEC: cmpi.b #$3, $21(a0)
pub fn i_079dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DF2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $079DF2: beq.w $79e00
pub fn i_079df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79E00; return 10; }
    8
}
/// $079DF6: cmpi.b #$ff, $20(a0)
pub fn i_079df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79DFC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $079DFC: bne.w $79e08
pub fn i_079dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E00;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79E08; return 10; }
    8
}
/// $079E00: lea.l $77efe.l, a1
pub fn i_079e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E06;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77EFE;
    20
}
/// $079E06: move.l a1, (a6)
pub fn i_079e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E08;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079E08: jsr $2870a.l
pub fn i_079e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E0E);
    cpu.pc = 0x2870A;
    16
}
/// $079E0E: bcc.w $79e18
pub fn i_079e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79E18; return 10; }
    8
}
/// $079E12: lea.l $79e2a(pc), a1
pub fn i_079e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79E2A;
    16
}
/// $079E16: move.l a1, (a6)
pub fn i_079e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079E18: jsr $7fde0.l
pub fn i_079e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E1E);
    cpu.pc = 0x7FDE0;
    16
}
/// $079E1E: bcc.w $79e28
pub fn i_079e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79E28; return 10; }
    8
}
// ===== SetTaskHandler_079e22  ($079E22, 8 B, kernel/task_handlers.c)
/// $079E22: lea.l $79e94(pc), a1
pub fn i_079e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79E94;
    16
}
/// $079E26: move.l a1, (a6)
pub fn i_079e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079E28: rts
pub fn i_079e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_HatchB_Land_079e2a  ($079E2A, 54 B, props/fx_explosions_platforms_crew_076000.s)
/// $079E2A: tst.b $10fd8f.l
pub fn i_079e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E30;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $079E30: bne.w $79e3e
pub fn i_079e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E34;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79E3E; return 10; }
    8
}
/// $079E34: move.w #$1d5, d1
pub fn i_079e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E38;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079E38: jsr $236e.l
pub fn i_079e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E3E);
    cpu.pc = 0x236E;
    16
}
/// $079E3E: lea.l $29c28c.l, a0
pub fn i_079e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C28C;
    20
}
/// $079E44: jsr $28cd4.l
pub fn i_079e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E4A);
    cpu.pc = 0x28CD4;
    16
}
/// $079E4A: lea.l $79e50(pc), a1
pub fn i_079e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79E50;
    16
}
/// $079E4E: move.l a1, (a6)
pub fn i_079e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E50;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079E50: jsr $7fdfa.l
pub fn i_079e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E56);
    cpu.pc = 0x7FDFA;
    16
}
/// $079E56: jsr $28d70.l
pub fn i_079e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E5C);
    cpu.pc = 0x28D70;
    16
}
/// $079E5C: bcc.w $79e66
pub fn i_079e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79E66; return 10; }
    8
}
// ===== SetTaskHandler_079e60  ($079E60, 8 B, kernel/task_handlers.c)
/// $079E60: lea.l $79e68(pc), a1
pub fn i_079e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79E68;
    16
}
/// $079E64: move.l a1, (a6)
pub fn i_079e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079E66: rts
pub fn i_079e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_HatchB_Flee_079e68  ($079E68, 44 B, props/fx_explosions_platforms_crew_076000.s)
/// $079E68: move.b #$28, $59(a6)
pub fn i_079e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E6E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079E6E: lea.l $79e74(pc), a1
pub fn i_079e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79E74;
    16
}
/// $079E72: move.l a1, (a6)
pub fn i_079e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079E74: jsr $7fdfa.l
pub fn i_079e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E7A);
    cpu.pc = 0x7FDFA;
    16
}
/// $079E7A: jsr $28d70.l
pub fn i_079e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79E80);
    cpu.pc = 0x28D70;
    16
}
/// $079E80: tst.b $59(a6)
pub fn i_079e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E84;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $079E84: bne.w $79e9a
pub fn i_079e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E88;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79E9A; return 10; }
    8
}
/// $079E88: movea.l $c(a6), a0
pub fn i_079e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E8C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079E8C: move.w $72(a6), d0
pub fn i_079e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E90;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $079E90: bclr.b d0, $70(a0)
pub fn i_079e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E94;
    cpu.ir = 0x1A8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== Crew_HatchB_Free_079e94  ($079E94, 8 B, props/fx_explosions_platforms_crew_076000.s)
/// $079E94: jmp $518.l
pub fn i_079e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E9A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $079E9A: rts
pub fn i_079e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79E9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_RetGateC_079e9c  ($079E9C, 16 B, props/fx_explosions_platforms_crew_076000.s)
/// $079E9C: movea.l $8(a6), a1
pub fn i_079e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EA0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $079EA0: move.b $10(a6), d0
pub fn i_079ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EA4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $079EA4: cmp.b $10(a1), d0
pub fn i_079ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EA8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $079EA8: bcs.w $79eb2
pub fn i_079ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EAC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x79EB2; return 10; }
    8
}
// ===== ClearXN_079eac  ($079EAC, 6 B, thunks/ccr_helpers.c)
/// $079EAC: andi.b #$ee, ccr
pub fn i_079eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EB0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $079EB0: rts
pub fn i_079eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_079eb2  ($079EB2, 6 B, thunks/ccr_helpers.c)
/// $079EB2: ori.b #$11, ccr
pub fn i_079eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EB6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $079EB6: rts
pub fn i_079eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Tmpl127_079eb8  ($079EB8, 10 B, props/fx_explosions_platforms_crew_076000.s)
/// $079EB8: move.w #$ffff, $70(a6)
pub fn i_079eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EBE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079EBE: bra.w $79ec8
pub fn i_079ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x79EC8;
    10
}
// ===== Crew_Tmpl128_079ec2  ($079EC2, 134 B, props/fx_explosions_platforms_crew_076000.s)
/// $079EC2: move.w #$0, $70(a6)
pub fn i_079ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079EC8: moveq #$0, d0
pub fn i_079ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ECA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079ECA: move.b d0, $20(a6)
pub fn i_079eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ECE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079ECE: move.b d0, $21(a6)
pub fn i_079ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ED2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079ED2: lea.l $79fe8(pc), a1
pub fn i_079ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79ED6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79FE8;
    16
}
/// $079ED6: jsr $4ae.l
pub fn i_079ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79EDC);
    cpu.pc = 0x4AE;
    16
}
/// $079EDC: jsr $5dd02.l
pub fn i_079edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79EE2);
    cpu.pc = 0x5DD02;
    16
}
/// $079EE2: subi.w #$40, $22(a0)
pub fn i_079ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EE8;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $079EE8: lea.l $7a19e(pc), a1
pub fn i_079ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A19E;
    16
}
/// $079EEC: jsr $4ae.l
pub fn i_079eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79EF2);
    cpu.pc = 0x4AE;
    16
}
/// $079EF2: jsr $5dd02.l
pub fn i_079ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79EF8);
    cpu.pc = 0x5DD02;
    16
}
/// $079EF8: subi.w #$20, $22(a0)
pub fn i_079ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79EFE;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $079EFE: lea.l $79f48(pc), a1
pub fn i_079efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79F48;
    16
}
/// $079F02: jsr $4ae.l
pub fn i_079f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F08);
    cpu.pc = 0x4AE;
    16
}
/// $079F08: jsr $5dd02.l
pub fn i_079f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F0E);
    cpu.pc = 0x5DD02;
    16
}
/// $079F0E: bclr.b #$1, $12(a6)
pub fn i_079f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F14;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $079F14: lea.l $79f1a(pc), a1
pub fn i_079f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79F1A;
    16
}
/// $079F18: move.l a1, (a6)
pub fn i_079f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079F1A: tst.w $70(a6)
pub fn i_079f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F1E;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $079F1E: beq.w $79f30
pub fn i_079f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F22;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79F30; return 10; }
    8
}
/// $079F22: movea.l $c(a6), a0
pub fn i_079f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F26;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079F26: movea.l $c(a0), a0
pub fn i_079f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F2A;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079F2A: move.b $20(a0), $20(a6)
pub fn i_079f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F30;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $079F30: cmpi.b #$ff, $21(a6)
pub fn i_079f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F36;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $079F36: bne.w $79f40
pub fn i_079f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79F40; return 10; }
    8
}
/// $079F3A: jmp $518.l
pub fn i_079f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F40;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $079F40: move.w #$0, $72(a6)
pub fn i_079f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F46;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079F46: rts
pub fn i_079f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Gunner_079f48  ($079F48, 26 B, props/fx_explosions_platforms_crew_076000.s)
/// $079F48: move.w #$e, d1
pub fn i_079f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F4C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079F4C: jsr $236e.l
pub fn i_079f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F52);
    cpu.pc = 0x236E;
    16
}
/// $079F52: moveq #$0, d0
pub fn i_079f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F54;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079F54: move.b d0, $20(a6)
pub fn i_079f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F58;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079F58: move.b d0, $21(a6)
pub fn i_079f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F5C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079F5C: move.w #$1, $66(a6)
pub fn i_079f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F62;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_079f62  ($079F62, 8 B, kernel/task_handlers.c)
/// $079F62: lea.l $79f6a(pc), a1
pub fn i_079f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79F6A;
    16
}
/// $079F66: move.l a1, (a6)
pub fn i_079f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F68;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079F68: rts
pub fn i_079f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Gunner_Run_079f6a  ($079F6A, 118 B, props/fx_explosions_platforms_crew_076000.s)
/// $079F6A: jsr $5e7c0.l
pub fn i_079f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F70);
    cpu.pc = 0x5E7C0;
    16
}
/// $079F70: move.w #$8000, d0
pub fn i_079f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F74;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079F74: jsr $28134.l
pub fn i_079f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F7A);
    cpu.pc = 0x28134;
    16
}
/// $079F7A: andi.w #$ffe3, $38(a6)
pub fn i_079f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F80;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $079F80: ori.w #$c, $38(a6)
pub fn i_079f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F86;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $079F86: lea.l $2df4f2.l, a0
pub fn i_079f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF4F2;
    20
}
/// $079F8C: jsr $28cd4.l
pub fn i_079f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F92);
    cpu.pc = 0x28CD4;
    16
}
/// $079F92: lea.l $79f98(pc), a1
pub fn i_079f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x79F98;
    16
}
/// $079F96: move.l a1, (a6)
pub fn i_079f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079F98: jsr $2783a.l
pub fn i_079f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79F9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79F9E);
    cpu.pc = 0x2783A;
    16
}
/// $079F9E: jsr $28d70.l
pub fn i_079f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79FA4);
    cpu.pc = 0x28D70;
    16
}
/// $079FA4: movea.l $c(a6), a0
pub fn i_079fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FA8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $079FA8: cmpi.b #$ff, $20(a0)
pub fn i_079fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FAE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $079FAE: beq.w $79fbc
pub fn i_079fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FB2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x79FBC; return 10; }
    8
}
/// $079FB2: cmpi.b #$ff, $21(a0)
pub fn i_079fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FB8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $079FB8: bne.w $79fc4
pub fn i_079fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FBC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x79FC4; return 10; }
    8
}
/// $079FBC: lea.l $58fc2.l, a1
pub fn i_079fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FC2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58FC2;
    20
}
/// $079FC2: move.l a1, (a6)
pub fn i_079fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079FC4: jsr $49fd0.l
pub fn i_079fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79FCA);
    cpu.pc = 0x49FD0;
    16
}
/// $079FCA: movea.l #$ffffffff, a0
pub fn i_079fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FD0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $079FD0: lea.l $2df594.l, a0
pub fn i_079fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FD6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF594;
    20
}
/// $079FD6: jsr $5dd5c.l
pub fn i_079fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79FDC);
    cpu.pc = 0x5DD5C;
    16
}
/// $079FDC: bcc.w $79fe6
pub fn i_079fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x79FE6; return 10; }
    8
}
// ===== SetTaskHandler_079fe0  ($079FE0, 8 B, kernel/task_handlers.c)
/// $079FE0: lea.l $7a3e6(pc), a1
pub fn i_079fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3E6;
    16
}
/// $079FE4: move.l a1, (a6)
pub fn i_079fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $079FE6: rts
pub fn i_079fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Driver_079fe8  ($079FE8, 26 B, props/fx_explosions_platforms_crew_076000.s)
/// $079FE8: move.w #$e, d1
pub fn i_079fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FEC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $079FEC: jsr $236e.l
pub fn i_079fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x79FF2);
    cpu.pc = 0x236E;
    16
}
/// $079FF2: move.w #$1, $66(a6)
pub fn i_079ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FF8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $079FF8: moveq #$0, d0
pub fn i_079ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FFA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $079FFA: move.b d0, $20(a6)
pub fn i_079ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x79FFE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $079FFE: move.b d0, $21(a6)
pub fn i_079ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A002;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetTaskHandler_07a002  ($07A002, 8 B, kernel/task_handlers.c)
/// $07A002: lea.l $7a00a(pc), a1
pub fn i_07a002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A006;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A00A;
    16
}
/// $07A006: move.l a1, (a6)
pub fn i_07a006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A008;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A008: rts
pub fn i_07a008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A00A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hostage_Idle_07a00a  ($07A00A, 158 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A00A: jsr $5e7c0.l
pub fn i_07a00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A010;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A010);
    cpu.pc = 0x5E7C0;
    16
}
/// $07A010: move.w #$8000, d0
pub fn i_07a010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A014;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A014: jsr $28134.l
pub fn i_07a014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A01A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A01A);
    cpu.pc = 0x28134;
    16
}
/// $07A01A: andi.w #$ffe3, $38(a6)
pub fn i_07a01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A020;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A020: ori.w #$c, $38(a6)
pub fn i_07a020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A026;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A026: addq.w #$1, $38(a6)
pub fn i_07a026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A02A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07A02A: lea.l $29c36a.l, a0
pub fn i_07a02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A030;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C36A;
    20
}
/// $07A030: jsr $28cd4.l
pub fn i_07a030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A036;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A036);
    cpu.pc = 0x28CD4;
    16
}
/// $07A036: lea.l $7a03c(pc), a1
pub fn i_07a036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A03A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A03C;
    16
}
/// $07A03A: move.l a1, (a6)
pub fn i_07a03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A03C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A03C: jsr $2783a.l
pub fn i_07a03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A042;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A042);
    cpu.pc = 0x2783A;
    16
}
/// $07A042: jsr $28d70.l
pub fn i_07a042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A048;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A048);
    cpu.pc = 0x28D70;
    16
}
/// $07A048: movea.l $c(a6), a0
pub fn i_07a048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A04C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A04C: cmpi.w #$2, $72(a0)
pub fn i_07a04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A052;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $07A052: bne.w $7a05c
pub fn i_07a052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A056;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A05C; return 10; }
    8
}
/// $07A056: lea.l $7a0b8(pc), a1
pub fn i_07a056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A05A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A0B8;
    16
}
/// $07A05A: move.l a1, (a6)
pub fn i_07a05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A05C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A05C: movea.l $c(a6), a0
pub fn i_07a05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A060;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A060: cmpi.b #$ff, $20(a0)
pub fn i_07a060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A066;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07A066: beq.w $7a074
pub fn i_07a066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A06A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A074; return 10; }
    8
}
/// $07A06A: cmpi.b #$ff, $21(a0)
pub fn i_07a06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A070;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07A070: bne.w $7a082
pub fn i_07a070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A074;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A082; return 10; }
    8
}
/// $07A074: eori.b #$1, $3a(a6)
pub fn i_07a074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A07A;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07A07A: lea.l $58fc2.l, a1
pub fn i_07a07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A080;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58FC2;
    20
}
/// $07A080: move.l a1, (a6)
pub fn i_07a080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A082;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A082: jsr $49fd0.l
pub fn i_07a082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A088;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A088);
    cpu.pc = 0x49FD0;
    16
}
/// $07A088: btst.b #$3, $13(a6)
pub fn i_07a088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A08E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07A08E: bne.w $7a0b0
pub fn i_07a08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A092;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A0B0; return 10; }
    8
}
/// $07A092: movea.l #$ffffffff, a0
pub fn i_07a092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A098;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A098: lea.l $2df594.l, a0
pub fn i_07a098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A09E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF594;
    20
}
/// $07A09E: jsr $5dd5c.l
pub fn i_07a09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A0A4);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A0A4: bcc.w $7a0ae
pub fn i_07a0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A0AE; return 10; }
    8
}
// ===== SetTaskHandler_07a0a8  ($07A0A8, 8 B, kernel/task_handlers.c)
/// $07A0A8: lea.l $7a3e6(pc), a1
pub fn i_07a0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3E6;
    16
}
/// $07A0AC: move.l a1, (a6)
pub fn i_07a0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A0AE: rts
pub fn i_07a0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hostage_ToggleFacing_07a0b0  ($07A0B0, 122 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A0B0: eori.b #$1, $3a(a6)
pub fn i_07a0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0B6;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07A0B6: rts
pub fn i_07a0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07A0B8: lea.l $29c39e.l, a0
pub fn i_07a0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C39E;
    20
}
/// $07A0BE: jsr $28cd4.l
pub fn i_07a0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A0C4);
    cpu.pc = 0x28CD4;
    16
}
/// $07A0C4: lea.l $7a0ca(pc), a1
pub fn i_07a0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A0CA;
    16
}
/// $07A0C8: move.l a1, (a6)
pub fn i_07a0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A0CA: jsr $2783a.l
pub fn i_07a0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A0D0);
    cpu.pc = 0x2783A;
    16
}
/// $07A0D0: jsr $28d70.l
pub fn i_07a0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A0D6);
    cpu.pc = 0x28D70;
    16
}
/// $07A0D6: bcc.w $7a0e0
pub fn i_07a0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A0E0; return 10; }
    8
}
/// $07A0DA: lea.l $7a02a(pc), a1
pub fn i_07a0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A02A;
    16
}
/// $07A0DE: move.l a1, (a6)
pub fn i_07a0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A0E0: movea.l $c(a6), a0
pub fn i_07a0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0E4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A0E4: cmpi.b #$ff, $20(a0)
pub fn i_07a0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0EA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07A0EA: beq.w $7a0f8
pub fn i_07a0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0EE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A0F8; return 10; }
    8
}
/// $07A0EE: cmpi.b #$ff, $21(a0)
pub fn i_07a0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0F4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07A0F4: bne.w $7a106
pub fn i_07a0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A106; return 10; }
    8
}
/// $07A0F8: eori.b #$1, $3a(a6)
pub fn i_07a0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A0FE;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07A0FE: lea.l $58fc2.l, a1
pub fn i_07a0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A104;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58FC2;
    20
}
/// $07A104: move.l a1, (a6)
pub fn i_07a104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A106;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A106: jsr $49fd0.l
pub fn i_07a106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A10C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A10C);
    cpu.pc = 0x49FD0;
    16
}
/// $07A10C: btst.b #$3, $13(a6)
pub fn i_07a10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A112;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07A112: bne.b $7a0b0
pub fn i_07a112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A114;
    cpu.ir = 0x669C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A0B0; return 10; }
    8
}
/// $07A114: movea.l #$ffffffff, a0
pub fn i_07a114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A11A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A11A: lea.l $2df594.l, a0
pub fn i_07a11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A120;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF594;
    20
}
/// $07A120: jsr $5dd5c.l
pub fn i_07a120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A126;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A126);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A126: bcc.w $7a130
pub fn i_07a126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A12A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A130; return 10; }
    8
}
// ===== SetTaskHandler_07a12a  ($07A12A, 8 B, kernel/task_handlers.c)
/// $07A12A: lea.l $7a3e6(pc), a1
pub fn i_07a12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A12E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3E6;
    16
}
/// $07A12E: move.l a1, (a6)
pub fn i_07a12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A130;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A130: rts
pub fn i_07a130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A132;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hostage_Dying_07a132  ($07A132, 100 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A132: move.w #$8, d1
pub fn i_07a132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A136;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A136: jsr $236e.l
pub fn i_07a136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A13C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A13C);
    cpu.pc = 0x236E;
    16
}
/// $07A13C: lea.l $29c4da.l, a0
pub fn i_07a13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A142;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C4DA;
    20
}
/// $07A142: jsr $28cd4.l
pub fn i_07a142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A148;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A148);
    cpu.pc = 0x28CD4;
    16
}
/// $07A148: lea.l $7a14e(pc), a1
pub fn i_07a148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A14C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A14E;
    16
}
/// $07A14C: move.l a1, (a6)
pub fn i_07a14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A14E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A14E: jsr $2783a.l
pub fn i_07a14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A154);
    cpu.pc = 0x2783A;
    16
}
/// $07A154: jsr $28d70.l
pub fn i_07a154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A15A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A15A);
    cpu.pc = 0x28D70;
    16
}
/// $07A15A: move.b #$1, $44(a6)
pub fn i_07a15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A160;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A160: movea.l $c(a6), a0
pub fn i_07a160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A164;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A164: tst.w $72(a0)
pub fn i_07a164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A168;
    cpu.ir = 0x4A68;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07A168: beq.w $7a170
pub fn i_07a168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A16C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A170; return 10; }
    8
}
/// $07A16C: clr.b $44(a6)
pub fn i_07a16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A170;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x44);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07A170: cmpi.b #$ff, $20(a0)
pub fn i_07a170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A176;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07A176: bne.w $7a180
pub fn i_07a176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A17A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A180; return 10; }
    8
}
/// $07A17A: lea.l $7a36e(pc), a1
pub fn i_07a17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A17E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A36E;
    16
}
/// $07A17E: move.l a1, (a6)
pub fn i_07a17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A180;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A180: movea.l #$ffffffff, a0
pub fn i_07a180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A186;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A186: lea.l $2df594.l, a0
pub fn i_07a186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A18C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF594;
    20
}
/// $07A18C: jsr $5dd5c.l
pub fn i_07a18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A192;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A192);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A192: bcc.w $7a19c
pub fn i_07a192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A196;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A19C; return 10; }
    8
}
// ===== SetTaskHandler_07a196  ($07A196, 8 B, kernel/task_handlers.c)
/// $07A196: lea.l $7a3e6(pc), a1
pub fn i_07a196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A19A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3E6;
    16
}
/// $07A19A: move.l a1, (a6)
pub fn i_07a19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A19C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A19C: rts
pub fn i_07a19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A19E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hostage_Init_07a19e  ($07A19E, 26 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A19E: move.w #$2e, d1
pub fn i_07a19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1A2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A1A2: jsr $236e.l
pub fn i_07a1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A1A8);
    cpu.pc = 0x236E;
    16
}
/// $07A1A8: moveq #$0, d0
pub fn i_07a1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1AA;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07A1AA: move.b d0, $20(a6)
pub fn i_07a1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1AE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A1AE: move.b d0, $21(a6)
pub fn i_07a1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1B2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A1B2: move.w #$1, $66(a6)
pub fn i_07a1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_07a1b8  ($07A1B8, 8 B, kernel/task_handlers.c)
/// $07A1B8: lea.l $7a1c0(pc), a1
pub fn i_07a1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A1C0;
    16
}
/// $07A1BC: move.l a1, (a6)
pub fn i_07a1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A1BE: rts
pub fn i_07a1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Captor_Idle_07a1c0  ($07A1C0, 180 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A1C0: jsr $5e7c0.l
pub fn i_07a1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A1C6);
    cpu.pc = 0x5E7C0;
    16
}
/// $07A1C6: move.w #$8000, d0
pub fn i_07a1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1CA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A1CA: jsr $28134.l
pub fn i_07a1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A1D0);
    cpu.pc = 0x28134;
    16
}
/// $07A1D0: andi.w #$ffe3, $38(a6)
pub fn i_07a1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1D6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A1D6: ori.w #$c, $38(a6)
pub fn i_07a1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1DC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A1DC: addq.w #$1, $38(a6)
pub fn i_07a1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1E0;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07A1E0: lea.l $7a132(pc), a1
pub fn i_07a1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A132;
    16
}
/// $07A1E4: jsr $4ae.l
pub fn i_07a1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A1EA);
    cpu.pc = 0x4AE;
    16
}
/// $07A1EA: jsr $5dd02.l
pub fn i_07a1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A1F0);
    cpu.pc = 0x5DD02;
    16
}
/// $07A1F0: bra.w $7a20e
pub fn i_07a1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A20E;
    10
}
/// $07A1F4: movea.l $c(a6), a0
pub fn i_07a1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1F8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A1F8: tst.w $70(a0)
pub fn i_07a1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A1FC;
    cpu.ir = 0x4A68;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07A1FC: beq.w $7a20e
pub fn i_07a1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A200;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A20E; return 10; }
    8
}
/// $07A200: movea.l $c(a0), a0
pub fn i_07a200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A204;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A204: movea.l $c(a0), a0
pub fn i_07a204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A208;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A208: move.b #$2, $21(a0)
pub fn i_07a208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A20E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A20E: lea.l $29c3c4.l, a0
pub fn i_07a20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A214;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C3C4;
    20
}
/// $07A214: jsr $28cd4.l
pub fn i_07a214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A21A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A21A);
    cpu.pc = 0x28CD4;
    16
}
/// $07A21A: lea.l $7a220(pc), a1
pub fn i_07a21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A21E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A220;
    16
}
/// $07A21E: move.l a1, (a6)
pub fn i_07a21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A220;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A220: jsr $2783a.l
pub fn i_07a220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A226);
    cpu.pc = 0x2783A;
    16
}
/// $07A226: jsr $28d70.l
pub fn i_07a226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A22C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A22C);
    cpu.pc = 0x28D70;
    16
}
/// $07A22C: movea.l $c(a6), a0
pub fn i_07a22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A230;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A230: move.w $72(a0), d0
pub fn i_07a230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A234;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A234: cmpi.w #$1, d0
pub fn i_07a234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A238;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $07A238: beq.w $7a244
pub fn i_07a238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A23C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A244; return 10; }
    8
}
/// $07A23C: cmpi.w #$2, d0
pub fn i_07a23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A240;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $07A240: bne.w $7a24e
pub fn i_07a240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A244;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A24E; return 10; }
    8
}
/// $07A244: move.w d0, $72(a6)
pub fn i_07a244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A248;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A248: lea.l $7a27c(pc), a1
pub fn i_07a248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A24C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A27C;
    16
}
/// $07A24C: move.l a1, (a6)
pub fn i_07a24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A24E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A24E: jsr $2870a.l
pub fn i_07a24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A254;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A254);
    cpu.pc = 0x2870A;
    16
}
/// $07A254: bcc.w $7a25e
pub fn i_07a254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A258;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A25E; return 10; }
    8
}
/// $07A258: lea.l $7a338(pc), a1
pub fn i_07a258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A25C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A338;
    16
}
/// $07A25C: move.l a1, (a6)
pub fn i_07a25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A25E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A25E: movea.l #$ffffffff, a0
pub fn i_07a25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A264;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A264: lea.l $2df59c.l, a0
pub fn i_07a264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A26A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF59C;
    20
}
/// $07A26A: jsr $5dd5c.l
pub fn i_07a26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A270);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A270: bcc.w $7a27a
pub fn i_07a270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A274;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A27A; return 10; }
    8
}
// ===== SetTaskHandler_07a274  ($07A274, 8 B, kernel/task_handlers.c)
/// $07A274: lea.l $7a3ee(pc), a1
pub fn i_07a274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A278;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3EE;
    16
}
/// $07A278: move.l a1, (a6)
pub fn i_07a278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A27A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A27A: rts
pub fn i_07a27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A27C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Captor_Taunt_07a27c  ($07A27C, 130 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A27C: movea.l $c(a6), a0
pub fn i_07a27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A280;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A280: tst.w $70(a0)
pub fn i_07a280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A284;
    cpu.ir = 0x4A68;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07A284: beq.w $7a296
pub fn i_07a284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A288;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A296; return 10; }
    8
}
/// $07A288: movea.l $c(a0), a0
pub fn i_07a288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A28C;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A28C: movea.l $c(a0), a0
pub fn i_07a28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A290;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A290: move.b #$1, $21(a0)
pub fn i_07a290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A296;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A296: cmpi.w #$1, $72(a6)
pub fn i_07a296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A29C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $07A29C: beq.w $7a2b0
pub fn i_07a29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A2B0; return 10; }
    8
}
/// $07A2A0: lea.l $29c3f8.l, a0
pub fn i_07a2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C3F8;
    20
}
/// $07A2A6: jsr $28cd4.l
pub fn i_07a2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A2AC);
    cpu.pc = 0x28CD4;
    16
}
/// $07A2AC: bra.w $7a2bc
pub fn i_07a2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A2BC;
    10
}
/// $07A2B0: lea.l $29c48c.l, a0
pub fn i_07a2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C48C;
    20
}
/// $07A2B6: jsr $28cd4.l
pub fn i_07a2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A2BC);
    cpu.pc = 0x28CD4;
    16
}
/// $07A2BC: lea.l $7a2c2(pc), a1
pub fn i_07a2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A2C2;
    16
}
/// $07A2C0: move.l a1, (a6)
pub fn i_07a2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A2C2: jsr $2783a.l
pub fn i_07a2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A2C8);
    cpu.pc = 0x2783A;
    16
}
/// $07A2C8: jsr $28d70.l
pub fn i_07a2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A2CE);
    cpu.pc = 0x28D70;
    16
}
/// $07A2CE: bcc.w $7a2d8
pub fn i_07a2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A2D8; return 10; }
    8
}
/// $07A2D2: lea.l $7a1f4(pc), a1
pub fn i_07a2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A1F4;
    16
}
/// $07A2D6: move.l a1, (a6)
pub fn i_07a2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A2D8: jsr $2870a.l
pub fn i_07a2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A2DE);
    cpu.pc = 0x2870A;
    16
}
/// $07A2DE: bcc.w $7a2e8
pub fn i_07a2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A2E8; return 10; }
    8
}
/// $07A2E2: lea.l $7a338(pc), a1
pub fn i_07a2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A338;
    16
}
/// $07A2E6: move.l a1, (a6)
pub fn i_07a2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A2E8: movea.l #$ffffffff, a0
pub fn i_07a2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2EE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A2EE: lea.l $2df59c.l, a0
pub fn i_07a2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF59C;
    20
}
/// $07A2F4: jsr $5dd5c.l
pub fn i_07a2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A2FA);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A2FA: bcc.w $7a304
pub fn i_07a2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A2FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A304; return 10; }
    8
}
// ===== SetTaskHandler_07a2fe  ($07A2FE, 8 B, kernel/task_handlers.c)
/// $07A2FE: lea.l $7a3ee(pc), a1
pub fn i_07a2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A302;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3EE;
    16
}
/// $07A302: move.l a1, (a6)
pub fn i_07a302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A304;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A304: rts
pub fn i_07a304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A306;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Captor_ToHut_07a306  ($07A306, 50 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A306: eori.b #$1, $3a(a6)
pub fn i_07a306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A30C;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07A30C: lea.l $29bd40.l, a0
pub fn i_07a30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A312;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BD40;
    20
}
/// $07A312: jsr $28cd4.l
pub fn i_07a312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A318;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A318);
    cpu.pc = 0x28CD4;
    16
}
/// $07A318: lea.l $7a31e(pc), a1
pub fn i_07a318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A31C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A31E;
    16
}
/// $07A31C: move.l a1, (a6)
pub fn i_07a31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A31E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A31E: jsr $2783a.l
pub fn i_07a31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A324;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A324);
    cpu.pc = 0x2783A;
    16
}
/// $07A324: jsr $28d70.l
pub fn i_07a324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A32A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A32A);
    cpu.pc = 0x28D70;
    16
}
/// $07A32A: bcc.w $7a336
pub fn i_07a32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A32E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A336; return 10; }
    8
}
/// $07A32E: lea.l $5fa00.l, a1
pub fn i_07a32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A334;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x5FA00;
    20
}
/// $07A334: move.l a1, (a6)
pub fn i_07a334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A336;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A336: rts
pub fn i_07a336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A338;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Captor_Death_07a338  ($07A338, 54 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A338: move.b #$ff, $20(a6)
pub fn i_07a338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A33E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A33E: movea.l $c(a6), a0
pub fn i_07a33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A342;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A342: move.b #$ff, $21(a0)
pub fn i_07a342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A348;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A348: tst.w $70(a0)
pub fn i_07a348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A34C;
    cpu.ir = 0x4A68;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07A34C: beq.w $7a35e
pub fn i_07a34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A350;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A35E; return 10; }
    8
}
/// $07A350: movea.l $c(a0), a0
pub fn i_07a350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A354;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A354: movea.l $c(a0), a0
pub fn i_07a354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A358;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A358: move.b #$ff, $80(a0)
pub fn i_07a358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A35E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A35E: move.w #$109f, d0
pub fn i_07a35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A362;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A362: jsr $2352.l
pub fn i_07a362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A368;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A368);
    cpu.pc = 0x2352;
    16
}
/// $07A368: jmp $77f6a.l
pub fn i_07a368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A36E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Crew_Hostage_FadeOut_07a36e  ($07A36E, 78 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A36E: clr.b $44(a6)
pub fn i_07a36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A372;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x44);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07A372: move.b #$14, $59(a6)
pub fn i_07a372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A378;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A378: move.w #$14, $74(a6)
pub fn i_07a378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A37E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A37E: lea.l $7a384(pc), a1
pub fn i_07a37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A382;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A384;
    16
}
/// $07A382: move.l a1, (a6)
pub fn i_07a382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A384;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A384: jsr $2783a.l
pub fn i_07a384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A38A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A38A);
    cpu.pc = 0x2783A;
    16
}
/// $07A38A: jsr $28d70.l
pub fn i_07a38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A390;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A390);
    cpu.pc = 0x28D70;
    16
}
/// $07A390: movea.l #$ffffffff, a0
pub fn i_07a390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A396;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A396: lea.l $2df594.l, a0
pub fn i_07a396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A39C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF594;
    20
}
/// $07A39C: jsr $5dd5c.l
pub fn i_07a39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A3A2);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A3A2: bcc.w $7a3ac
pub fn i_07a3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3A6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A3AC; return 10; }
    8
}
/// $07A3A6: lea.l $7a3e6(pc), a1
pub fn i_07a3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3E6;
    16
}
/// $07A3AA: move.l a1, (a6)
pub fn i_07a3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A3AC: subq.w #$1, $74(a6)
pub fn i_07a3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3B0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07A3B0: bne.w $7a3ba
pub fn i_07a3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3B4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7A3BA; return 10; }
    8
}
/// $07A3B4: jmp $518.l
pub fn i_07a3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3BA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07A3BA: rts
pub fn i_07a3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_Hostage_WaitOffWorld_07a3bc  ($07A3BC, 34 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A3BC: jsr $2783a.l
pub fn i_07a3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A3C2);
    cpu.pc = 0x2783A;
    16
}
/// $07A3C2: jsr $28d70.l
pub fn i_07a3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A3C8);
    cpu.pc = 0x28D70;
    16
}
/// $07A3C8: movea.l #$ffffffff, a0
pub fn i_07a3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3CE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07A3CE: lea.l $2df594.l, a0
pub fn i_07a3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF594;
    20
}
/// $07A3D4: jsr $5dd5c.l
pub fn i_07a3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A3DA);
    cpu.pc = 0x5DD5C;
    16
}
/// $07A3DA: bcc.w $7a3e4
pub fn i_07a3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7A3E4; return 10; }
    8
}
// ===== SetTaskHandler_07a3de  ($07A3DE, 8 B, kernel/task_handlers.c)
/// $07A3DE: lea.l $7a3e6(pc), a1
pub fn i_07a3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A3E6;
    16
}
/// $07A3E2: move.l a1, (a6)
pub fn i_07a3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A3E4: rts
pub fn i_07a3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_07a3e6  ($07A3E6, 8 B, kernel/jmp_to_scheduler.c)
/// $07A3E6: jmp $518.l
pub fn i_07a3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3EC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Crew_Captor_MarkParentDead_07a3ee  ($07A3EE, 32 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A3EE: movea.l $c(a6), a0
pub fn i_07a3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3F2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A3F2: move.b #$ff, $21(a0)
pub fn i_07a3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3F8;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A3F8: tst.w $70(a0)
pub fn i_07a3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A3FC;
    cpu.ir = 0x4A68;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07A3FC: beq.w $7a40e
pub fn i_07a3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A400;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A40E; return 10; }
    8
}
/// $07A400: movea.l $c(a0), a0
pub fn i_07a400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A404;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A404: movea.l $c(a0), a0
pub fn i_07a404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A408;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07A408: move.b #$ff, $80(a0)
pub fn i_07a408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A40E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Jsr5B6ThenJmpScheduler_07a40e  ($07A40E, 14 B, kernel/jsr5b6_jmp518.c)
/// $07A40E: jsr $5b6.l
pub fn i_07a40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A414;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A414);
    cpu.pc = 0x5B6;
    16
}
/// $07A414: jmp $518.l
pub fn i_07a414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A41A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Crew_Captor_PickRandCmd_07a41c  ($07A41C, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A41C: jsr $5e9b6.l
pub fn i_07a41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A422);
    cpu.pc = 0x5E9B6;
    16
}
/// $07A422: andi.w #$3, d0
pub fn i_07a422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A426;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07A426: add.w d0, d0
pub fn i_07a426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A428;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A428: lea.l $2df4ea.l, a0
pub fn i_07a428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A42E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF4EA;
    20
}
/// $07A42E: movea.l $c(a6), a1
pub fn i_07a42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A432;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07A432: move.w (a0, d0.w), $72(a1)
pub fn i_07a432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A438;
    cpu.ir = 0x3370;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    22
}
/// $07A438: rts
pub fn i_07a438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A43A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crew_RetGateD_07a43a  ($07A43A, 16 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A43A: movea.l $8(a6), a1
pub fn i_07a43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A43E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07A43E: move.b $10(a6), d0
pub fn i_07a43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A442;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07A442: cmp.b $10(a1), d0
pub fn i_07a442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A446;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07A446: bcs.w $7a450
pub fn i_07a446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A44A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7A450; return 10; }
    8
}
// ===== ClearXN_07a44a  ($07A44A, 6 B, thunks/ccr_helpers.c)
/// $07A44A: andi.b #$ee, ccr
pub fn i_07a44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A44E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07A44E: rts
pub fn i_07a44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A450;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07a450  ($07A450, 6 B, thunks/ccr_helpers.c)
/// $07A450: ori.b #$11, ccr
pub fn i_07a450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A454;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07A454: rts
pub fn i_07a454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A456;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SoundTest_Init_07a456  ($07A456, 160 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A456: movea.l #$7072, a1
pub fn i_07a456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A45C;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7072;
    cpu.a[1] = v1;
    4
}
/// $07A45C: lea.l $7a7fa(pc), a2
pub fn i_07a45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A460;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x7A7FA;
    16
}
/// $07A460: move.w #$2300, d0
pub fn i_07a460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A464;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A464: jsr $5dad8.l
pub fn i_07a464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A46A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A46A);
    cpu.pc = 0x5DAD8;
    16
}
/// $07A46A: movea.l #$72d2, a1
pub fn i_07a46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A470;
    cpu.ir = 0x227C;
    let v1: u32 = 0x72D2;
    cpu.a[1] = v1;
    4
}
/// $07A470: lea.l $7a88e(pc), a2
pub fn i_07a470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A474;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x7A88E;
    16
}
/// $07A474: move.w #$2300, d0
pub fn i_07a474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A478;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A478: jsr $5dad8.l
pub fn i_07a478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A47E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A47E);
    cpu.pc = 0x5DAD8;
    16
}
/// $07A47E: move.w #$20, $70(a6)
pub fn i_07a47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A484;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A484: move.w #$20, $72(a6)
pub fn i_07a484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A48A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A48A: move.w #$20, $74(a6)
pub fn i_07a48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A490;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A490: move.w #$20, $76(a6)
pub fn i_07a490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A496;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A496: move.w #$0, $78(a6)
pub fn i_07a496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A49C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A49C: move.w #$0, $7a(a6)
pub fn i_07a49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4A2: move.w #$0, $7c(a6)
pub fn i_07a4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4A8: move.w #$0, $7e(a6)
pub fn i_07a4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4AE: move.w #$0, $80(a6)
pub fn i_07a4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4B4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4B4: lea.l $433be.l, a1
pub fn i_07a4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4BA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x433BE;
    20
}
/// $07A4BA: jsr $4ae.l
pub fn i_07a4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A4C0);
    cpu.pc = 0x4AE;
    16
}
/// $07A4C0: move.w #$f0, $22(a0)
pub fn i_07a4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4C6;
    cpu.ir = 0x317C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4C6: move.w #$180, $24(a0)
pub fn i_07a4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4CC;
    cpu.ir = 0x317C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4CC: move.b #$ff, $98(a0)
pub fn i_07a4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4D2;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A4D2: lea.l $433be.l, a1
pub fn i_07a4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4D8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x433BE;
    20
}
/// $07A4D8: jsr $4ae.l
pub fn i_07a4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A4DE);
    cpu.pc = 0x4AE;
    16
}
/// $07A4DE: move.w #$50, $22(a0)
pub fn i_07a4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4E4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4E4: move.w #$180, $24(a0)
pub fn i_07a4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4EA;
    cpu.ir = 0x317C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07A4EA: move.b #$0, $98(a0)
pub fn i_07a4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4F0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07A4F0: move.l #$7a4f6, (a6)
pub fn i_07a4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4F6;
    cpu.ir = 0x2CBC;
    let v1: u32 = 0x7A4F6;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== SoundTest_Run_07a4f6  ($07A4F6, 746 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07A4F6: movea.l #$71e8, a1
pub fn i_07a4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A4FC;
    cpu.ir = 0x227C;
    let v1: u32 = 0x71E8;
    cpu.a[1] = v1;
    4
}
/// $07A4FC: lea.l $7a7e8(pc), a2
pub fn i_07a4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A500;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x7A7E8;
    16
}
/// $07A500: move.w #$2300, d0
pub fn i_07a500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A504;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07A504: jsr $5dad8.l
pub fn i_07a504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A50A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A50A);
    cpu.pc = 0x5DAD8;
    16
}
/// $07A50A: move.b $10e203.l, d0
pub fn i_07a50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A510;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E203) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07A510: btst.b #$6, d0
pub fn i_07a510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A514;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A514: beq.w $7a526
pub fn i_07a514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A518;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A526; return 10; }
    8
}
/// $07A518: move.b #$5, d0
pub fn i_07a518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A51C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07A51C: jsr $2152.l
pub fn i_07a51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A522;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A522);
    cpu.pc = 0x2152;
    16
}
/// $07A522: bra.w $7a614
pub fn i_07a522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A526;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A614;
    10
}
/// $07A526: btst.b #$7, d0
pub fn i_07a526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A52A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A52A: beq.w $7a53c
pub fn i_07a52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A52E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A53C; return 10; }
    8
}
/// $07A52E: move.b #$6, d0
pub fn i_07a52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A532;
    cpu.ir = 0x103C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $07A532: jsr $2152.l
pub fn i_07a532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A538;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A538);
    cpu.pc = 0x2152;
    16
}
/// $07A538: bra.w $7a614
pub fn i_07a538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A53C;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A614;
    10
}
/// $07A53C: btst.b #$0, d0
pub fn i_07a53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A540;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A540: beq.w $7a562
pub fn i_07a540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A544;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A562; return 10; }
    8
}
/// $07A544: move.w $80(a6), d1
pub fn i_07a544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A548;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A548: add.w d1, d1
pub fn i_07a548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A54A;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A54A: addq.w #$1, $70(a6, d1.w)
pub fn i_07a54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A54E;
    cpu.ir = 0x5276;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $07A54E: cmpi.w #$ff, $70(a6, d1.w)
pub fn i_07a54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A554;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Word, false);
    18
}
/// $07A554: ble.w $7a5e0
pub fn i_07a554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A558;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7A5E0; return 10; }
    8
}
/// $07A558: move.w #$20, $70(a6, d1.w)
pub fn i_07a558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A55E;
    cpu.ir = 0x3DBC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $07A55E: bra.w $7a5e0
pub fn i_07a55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A562;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A5E0;
    10
}
/// $07A562: btst.b #$1, d0
pub fn i_07a562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A566;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A566: beq.w $7a588
pub fn i_07a566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A56A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A588; return 10; }
    8
}
/// $07A56A: move.w $80(a6), d1
pub fn i_07a56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A56E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A56E: add.w d1, d1
pub fn i_07a56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A570;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A570: subq.w #$1, $70(a6, d1.w)
pub fn i_07a570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A574;
    cpu.ir = 0x5376;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $07A574: cmpi.w #$20, $70(a6, d1.w)
pub fn i_07a574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A57A;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    18
}
/// $07A57A: bge.w $7a5e0
pub fn i_07a57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A57E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x7A5E0; return 10; }
    8
}
/// $07A57E: move.w #$ff, $70(a6, d1.w)
pub fn i_07a57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A584;
    cpu.ir = 0x3DBC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $07A584: bra.w $7a5e0
pub fn i_07a584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A588;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A5E0;
    10
}
/// $07A588: btst.b #$2, d0
pub fn i_07a588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A58C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A58C: beq.w $7a5b6
pub fn i_07a58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A590;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A5B6; return 10; }
    8
}
/// $07A590: move.w $80(a6), d1
pub fn i_07a590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A594;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A594: add.w d1, d1
pub fn i_07a594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A596;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A596: subi.w #$10, $70(a6, d1.w)
pub fn i_07a596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A59C;
    cpu.ir = 0x476;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    18
}
/// $07A59C: cmpi.w #$20, $70(a6, d1.w)
pub fn i_07a59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5A2;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    18
}
/// $07A5A2: bge.w $7a5e0
pub fn i_07a5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5A6;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x7A5E0; return 10; }
    8
}
/// $07A5A6: andi.w #$f, $70(a6, d1.w)
pub fn i_07a5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5AC;
    cpu.ir = 0x276;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    18
}
/// $07A5AC: addi.w #$f0, $70(a6, d1.w)
pub fn i_07a5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5B2;
    cpu.ir = 0x676;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xF0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    18
}
/// $07A5B2: bra.w $7a5e0
pub fn i_07a5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A5E0;
    10
}
/// $07A5B6: btst.b #$3, d0
pub fn i_07a5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5BA;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A5BA: beq.w $7a5e0
pub fn i_07a5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A5E0; return 10; }
    8
}
/// $07A5BE: move.w $80(a6), d1
pub fn i_07a5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5C2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A5C2: add.w d1, d1
pub fn i_07a5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5C4;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A5C4: addi.w #$10, $70(a6, d1.w)
pub fn i_07a5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5CA;
    cpu.ir = 0x676;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    18
}
/// $07A5CA: cmpi.w #$ff, $70(a6, d1.w)
pub fn i_07a5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5D0;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Word, false);
    18
}
/// $07A5D0: ble.w $7a5e0
pub fn i_07a5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5D4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7A5E0; return 10; }
    8
}
/// $07A5D4: andi.w #$f, $70(a6, d1.w)
pub fn i_07a5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5DA;
    cpu.ir = 0x276;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    18
}
/// $07A5DA: addi.w #$20, $70(a6, d1.w)
pub fn i_07a5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5E0;
    cpu.ir = 0x676;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    18
}
/// $07A5E0: btst.b #$5, d0
pub fn i_07a5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5E4;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A5E4: beq.w $7a602
pub fn i_07a5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A602; return 10; }
    8
}
/// $07A5E8: move.w $80(a6), d1
pub fn i_07a5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5EC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A5EC: add.w d1, d1
pub fn i_07a5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5EE;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A5EE: addq.w #$1, $78(a6, d1.w)
pub fn i_07a5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5F2;
    cpu.ir = 0x5276;
    let ad1 = cpu.a[6].wrapping_add(0x78).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $07A5F2: cmpi.w #$3, $78(a6, d1.w)
pub fn i_07a5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5F8;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x78).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    18
}
/// $07A5F8: blt.w $7a602
pub fn i_07a5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A5FC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7A602; return 10; }
    8
}
/// $07A5FC: move.w #$0, $78(a6, d1.w)
pub fn i_07a5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A602;
    cpu.ir = 0x3DBC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $07A602: btst.b #$4, d0
pub fn i_07a602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A606;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A606: beq.w $7a614
pub fn i_07a606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A60A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A614; return 10; }
    8
}
/// $07A60A: addq.w #$1, $80(a6)
pub fn i_07a60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A60E;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07A60E: andi.w #$3, $80(a6)
pub fn i_07a60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A614;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A614: move.b $10e209.l, d0
pub fn i_07a614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A61A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E209) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07A61A: btst.b #$0, d0
pub fn i_07a61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A61E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A61E: beq.w $7a630
pub fn i_07a61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A622;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A630; return 10; }
    8
}
/// $07A622: subq.w #$1, $80(a6)
pub fn i_07a622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A626;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07A626: andi.w #$3, $80(a6)
pub fn i_07a626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A62C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A62C: bra.w $7a6c4
pub fn i_07a62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A630;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A6C4;
    10
}
/// $07A630: btst.b #$1, d0
pub fn i_07a630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A634;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A634: beq.w $7a646
pub fn i_07a634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A638;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A646; return 10; }
    8
}
/// $07A638: addq.w #$1, $80(a6)
pub fn i_07a638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A63C;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07A63C: andi.w #$3, $80(a6)
pub fn i_07a63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A642;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07A642: bra.w $7a6c4
pub fn i_07a642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A646;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A6C4;
    10
}
/// $07A646: move.b $10e20a.l, d0
pub fn i_07a646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A64C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E20A) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07A64C: btst.b #$3, d0
pub fn i_07a64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A650;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A650: beq.w $7a672
pub fn i_07a650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A654;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A672; return 10; }
    8
}
/// $07A654: move.w $80(a6), d1
pub fn i_07a654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A658;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A658: add.w d1, d1
pub fn i_07a658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A65A;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A65A: addq.w #$1, $70(a6, d1.w)
pub fn i_07a65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A65E;
    cpu.ir = 0x5276;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $07A65E: cmpi.w #$ff, $70(a6, d1.w)
pub fn i_07a65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A664;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Word, false);
    18
}
/// $07A664: ble.w $7a6c4
pub fn i_07a664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A668;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7A6C4; return 10; }
    8
}
/// $07A668: move.w #$20, $70(a6, d1.w)
pub fn i_07a668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A66E;
    cpu.ir = 0x3DBC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $07A66E: bra.w $7a6c4
pub fn i_07a66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A672;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A6C4;
    10
}
/// $07A672: btst.b #$2, d0
pub fn i_07a672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A676;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A676: beq.w $7a698
pub fn i_07a676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A67A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A698; return 10; }
    8
}
/// $07A67A: move.w $80(a6), d1
pub fn i_07a67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A67E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A67E: add.w d1, d1
pub fn i_07a67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A680;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A680: subq.w #$1, $70(a6, d1.w)
pub fn i_07a680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A684;
    cpu.ir = 0x5376;
    let ad1 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $07A684: cmpi.w #$20, $70(a6, d1.w)
pub fn i_07a684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A68A;
    cpu.ir = 0xC76;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    18
}
/// $07A68A: bge.w $7a6c4
pub fn i_07a68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A68E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x7A6C4; return 10; }
    8
}
/// $07A68E: move.w #$ff, $70(a6, d1.w)
pub fn i_07a68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A694;
    cpu.ir = 0x3DBC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $07A694: bra.w $7a6c4
pub fn i_07a694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A698;
    cpu.ir = 0x6000;
    cpu.pc = 0x7A6C4;
    10
}
/// $07A698: move.b $10e209.l, d0
pub fn i_07a698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A69E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10E209) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $07A69E: btst.b #$4, d0
pub fn i_07a69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6A2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A6A2: beq.w $7a6c4
pub fn i_07a6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A6C4; return 10; }
    8
}
/// $07A6A6: move.l d0, -(a7)
pub fn i_07a6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6A8;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07A6A8: move.w $70(a6), d0
pub fn i_07a6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6AC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A6AC: move.w $78(a6), d1
pub fn i_07a6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6B0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A6B0: lsl.w #$1, d1
pub fn i_07a6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6B2;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A6B2: lea.l $7a7f4(pc), a1
pub fn i_07a6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A6B6: move.w (a1, d1.w), d1
pub fn i_07a6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6BA;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A6BA: add.w d1, d0
pub fn i_07a6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6BC;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A6BC: jsr $219c.l
pub fn i_07a6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A6C2);
    cpu.pc = 0x219C;
    16
}
/// $07A6C2: move.l (a7)+, d0
pub fn i_07a6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6C4;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A6C4: btst.b #$5, d0
pub fn i_07a6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6C8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A6C8: beq.w $7a6ea
pub fn i_07a6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A6EA; return 10; }
    8
}
/// $07A6CC: move.l d0, -(a7)
pub fn i_07a6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6CE;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07A6CE: move.w $72(a6), d0
pub fn i_07a6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6D2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A6D2: move.w $7a(a6), d1
pub fn i_07a6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6D6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A6D6: lsl.w #$1, d1
pub fn i_07a6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6D8;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07A6D8: lea.l $7a7f4(pc), a1
pub fn i_07a6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7A7F4;
    16
}
/// $07A6DC: move.w (a1, d1.w), d1
pub fn i_07a6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6E0;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07A6E0: add.w d1, d0
pub fn i_07a6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6E2;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07A6E2: jsr $219c.l
pub fn i_07a6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7A6E8);
    cpu.pc = 0x219C;
    16
}
/// $07A6E8: move.l (a7)+, d0
pub fn i_07a6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6EA;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07A6EA: btst.b #$6, d0
pub fn i_07a6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6EE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $07A6EE: beq.w $7a710
pub fn i_07a6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7A710; return 10; }
    8
}
/// $07A6F2: move.l d0, -(a7)
pub fn i_07a6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6F4;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $07A6F4: move.w $74(a6), d0
pub fn i_07a6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6F8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07A6F8: move.w $7c(a6), d1
pub fn i_07a6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7A6FC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x07845E, i_07845e),
    (0x078464, i_078464),
    (0x078468, i_078468),
    (0x07846C, i_07846c),
    (0x07848E, i_07848e),
    (0x078494, i_078494),
    (0x07849A, i_07849a),
    (0x0784A0, i_0784a0),
    (0x0784A4, i_0784a4),
    (0x0784A8, i_0784a8),
    (0x0784CA, i_0784ca),
    (0x0784D0, i_0784d0),
    (0x0784D6, i_0784d6),
    (0x0784DC, i_0784dc),
    (0x0784E0, i_0784e0),
    (0x0784E4, i_0784e4),
    (0x078506, i_078506),
    (0x07850C, i_07850c),
    (0x078512, i_078512),
    (0x078518, i_078518),
    (0x07851C, i_07851c),
    (0x078520, i_078520),
    (0x078542, i_078542),
    (0x078548, i_078548),
    (0x07854E, i_07854e),
    (0x078554, i_078554),
    (0x078558, i_078558),
    (0x07855C, i_07855c),
    (0x078560, i_078560),
    (0x078566, i_078566),
    (0x07856C, i_07856c),
    (0x078572, i_078572),
    (0x078576, i_078576),
    (0x07857A, i_07857a),
    (0x07857E, i_07857e),
    (0x078584, i_078584),
    (0x07858A, i_07858a),
    (0x078590, i_078590),
    (0x078594, i_078594),
    (0x078598, i_078598),
    (0x07859C, i_07859c),
    (0x0785A2, i_0785a2),
    (0x0785A8, i_0785a8),
    (0x0785AE, i_0785ae),
    (0x0785B2, i_0785b2),
    (0x0785B6, i_0785b6),
    (0x0785BA, i_0785ba),
    (0x0785C0, i_0785c0),
    (0x0785C6, i_0785c6),
    (0x0785CC, i_0785cc),
    (0x0785D0, i_0785d0),
    (0x0785D4, i_0785d4),
    (0x0785D8, i_0785d8),
    (0x0785DE, i_0785de),
    (0x0785E4, i_0785e4),
    (0x0785EA, i_0785ea),
    (0x0785EE, i_0785ee),
    (0x0785F2, i_0785f2),
    (0x0785F6, i_0785f6),
    (0x0785FC, i_0785fc),
    (0x078602, i_078602),
    (0x078608, i_078608),
    (0x07860C, i_07860c),
    (0x078610, i_078610),
    (0x078614, i_078614),
    (0x07861A, i_07861a),
    (0x078620, i_078620),
    (0x078626, i_078626),
    (0x07862A, i_07862a),
    (0x07862E, i_07862e),
    (0x078632, i_078632),
    (0x078638, i_078638),
    (0x07863E, i_07863e),
    (0x078644, i_078644),
    (0x078648, i_078648),
    (0x07864C, i_07864c),
    (0x078650, i_078650),
    (0x078656, i_078656),
    (0x07865C, i_07865c),
    (0x078662, i_078662),
    (0x078666, i_078666),
    (0x07866A, i_07866a),
    (0x07866E, i_07866e),
    (0x078674, i_078674),
    (0x07867A, i_07867a),
    (0x078680, i_078680),
    (0x078684, i_078684),
    (0x078688, i_078688),
    (0x07868C, i_07868c),
    (0x078692, i_078692),
    (0x078698, i_078698),
    (0x07869E, i_07869e),
    (0x0786A2, i_0786a2),
    (0x0786A6, i_0786a6),
    (0x0786AA, i_0786aa),
    (0x0786B0, i_0786b0),
    (0x0786B6, i_0786b6),
    (0x0786BC, i_0786bc),
    (0x0786C0, i_0786c0),
    (0x0786C4, i_0786c4),
    (0x0786C8, i_0786c8),
    (0x0786CE, i_0786ce),
    (0x0786D4, i_0786d4),
    (0x0786DA, i_0786da),
    (0x0786DE, i_0786de),
    (0x0786E2, i_0786e2),
    (0x0786E6, i_0786e6),
    (0x0786EC, i_0786ec),
    (0x0786F2, i_0786f2),
    (0x0786F8, i_0786f8),
    (0x0786FC, i_0786fc),
    (0x078700, i_078700),
    (0x078704, i_078704),
    (0x07870A, i_07870a),
    (0x078710, i_078710),
    (0x078716, i_078716),
    (0x07871A, i_07871a),
    (0x07871E, i_07871e),
    (0x078740, i_078740),
    (0x078746, i_078746),
    (0x07874C, i_07874c),
    (0x078752, i_078752),
    (0x078756, i_078756),
    (0x07875A, i_07875a),
    (0x07875E, i_07875e),
    (0x078764, i_078764),
    (0x07876A, i_07876a),
    (0x078770, i_078770),
    (0x078774, i_078774),
    (0x078778, i_078778),
    (0x07877E, i_07877e),
    (0x078784, i_078784),
    (0x07878A, i_07878a),
    (0x07878E, i_07878e),
    (0x078792, i_078792),
    (0x078798, i_078798),
    (0x07879C, i_07879c),
    (0x0787A2, i_0787a2),
    (0x0787A8, i_0787a8),
    (0x0787AE, i_0787ae),
    (0x0787B2, i_0787b2),
    (0x0787B8, i_0787b8),
    (0x0787BC, i_0787bc),
    (0x0787C0, i_0787c0),
    (0x0787C2, i_0787c2),
    (0x0787C6, i_0787c6),
    (0x0787CA, i_0787ca),
    (0x0787D0, i_0787d0),
    (0x0787D4, i_0787d4),
    (0x0787D8, i_0787d8),
    (0x0787DC, i_0787dc),
    (0x0787E0, i_0787e0),
    (0x0787E6, i_0787e6),
    (0x0787EA, i_0787ea),
    (0x0787EE, i_0787ee),
    (0x0787F2, i_0787f2),
    (0x0787F6, i_0787f6),
    (0x0787F8, i_0787f8),
    (0x0787FE, i_0787fe),
    (0x078802, i_078802),
    (0x078808, i_078808),
    (0x07880E, i_07880e),
    (0x078814, i_078814),
    (0x078818, i_078818),
    (0x07881C, i_07881c),
    (0x078820, i_078820),
    (0x078826, i_078826),
    (0x07882A, i_07882a),
    (0x07882E, i_07882e),
    (0x078834, i_078834),
    (0x078838, i_078838),
    (0x07883E, i_07883e),
    (0x078840, i_078840),
    (0x078844, i_078844),
    (0x07884A, i_07884a),
    (0x078850, i_078850),
    (0x078856, i_078856),
    (0x07885C, i_07885c),
    (0x078860, i_078860),
    (0x078864, i_078864),
    (0x078868, i_078868),
    (0x07886E, i_07886e),
    (0x078874, i_078874),
    (0x07887A, i_07887a),
    (0x07887E, i_07887e),
    (0x078884, i_078884),
    (0x07888A, i_07888a),
    (0x078890, i_078890),
    (0x078894, i_078894),
    (0x07889A, i_07889a),
    (0x0788A0, i_0788a0),
    (0x0788A6, i_0788a6),
    (0x0788AA, i_0788aa),
    (0x0788B0, i_0788b0),
    (0x0788B6, i_0788b6),
    (0x0788BC, i_0788bc),
    (0x0788C0, i_0788c0),
    (0x0788C4, i_0788c4),
    (0x0788CA, i_0788ca),
    (0x0788D0, i_0788d0),
    (0x0788D6, i_0788d6),
    (0x0788DC, i_0788dc),
    (0x0788E0, i_0788e0),
    (0x0788E4, i_0788e4),
    (0x0788E8, i_0788e8),
    (0x0788EE, i_0788ee),
    (0x0788F4, i_0788f4),
    (0x0788FA, i_0788fa),
    (0x078900, i_078900),
    (0x078904, i_078904),
    (0x078908, i_078908),
    (0x07890C, i_07890c),
    (0x078912, i_078912),
    (0x078918, i_078918),
    (0x07891E, i_07891e),
    (0x078924, i_078924),
    (0x078928, i_078928),
    (0x078998, i_078998),
    (0x07899E, i_07899e),
    (0x0789A4, i_0789a4),
    (0x0789A8, i_0789a8),
    (0x0789AE, i_0789ae),
    (0x0789B2, i_0789b2),
    (0x0789B8, i_0789b8),
    (0x0789BC, i_0789bc),
    (0x0789C0, i_0789c0),
    (0x0789C4, i_0789c4),
    (0x0789C8, i_0789c8),
    (0x0789CA, i_0789ca),
    (0x0789D0, i_0789d0),
    (0x0789D6, i_0789d6),
    (0x0789DA, i_0789da),
    (0x0789DE, i_0789de),
    (0x0789E4, i_0789e4),
    (0x0789E8, i_0789e8),
    (0x0789EE, i_0789ee),
    (0x0789F0, i_0789f0),
    (0x0789F6, i_0789f6),
    (0x0789FC, i_0789fc),
    (0x078A02, i_078a02),
    (0x078A06, i_078a06),
    (0x078A0A, i_078a0a),
    (0x078A0E, i_078a0e),
    (0x078A14, i_078a14),
    (0x078A1A, i_078a1a),
    (0x078A20, i_078a20),
    (0x078A26, i_078a26),
    (0x078A2A, i_078a2a),
    (0x078A30, i_078a30),
    (0x078A34, i_078a34),
    (0x078A38, i_078a38),
    (0x078A3C, i_078a3c),
    (0x078A40, i_078a40),
    (0x078A42, i_078a42),
    (0x078A48, i_078a48),
    (0x078A4E, i_078a4e),
    (0x078A52, i_078a52),
    (0x078A56, i_078a56),
    (0x078A5C, i_078a5c),
    (0x078A60, i_078a60),
    (0x078A66, i_078a66),
    (0x078A68, i_078a68),
    (0x078A6C, i_078a6c),
    (0x078A70, i_078a70),
    (0x078A74, i_078a74),
    (0x078A78, i_078a78),
    (0x078A7C, i_078a7c),
    (0x078A7E, i_078a7e),
    (0x078A82, i_078a82),
    (0x078BE0, i_078be0),
    (0x078BE4, i_078be4),
    (0x078BEA, i_078bea),
    (0x078C64, i_078c64),
    (0x078C68, i_078c68),
    (0x078C6C, i_078c6c),
    (0x078C70, i_078c70),
    (0x078C76, i_078c76),
    (0x078C78, i_078c78),
    (0x078C7E, i_078c7e),
    (0x078C84, i_078c84),
    (0x078C88, i_078c88),
    (0x078C8E, i_078c8e),
    (0x078C94, i_078c94),
    (0x078C98, i_078c98),
    (0x078C9E, i_078c9e),
    (0x078CA2, i_078ca2),
    (0x078CA8, i_078ca8),
    (0x078CAE, i_078cae),
    (0x078CB2, i_078cb2),
    (0x078CB8, i_078cb8),
    (0x078CBC, i_078cbc),
    (0x078CC2, i_078cc2),
    (0x078CC6, i_078cc6),
    (0x078CC8, i_078cc8),
    (0x078CCE, i_078cce),
    (0x078CD4, i_078cd4),
    (0x078CD6, i_078cd6),
    (0x078CDA, i_078cda),
    (0x078CDC, i_078cdc),
    (0x078CDE, i_078cde),
    (0x078CE2, i_078ce2),
    (0x078CE6, i_078ce6),
    (0x078CEA, i_078cea),
    (0x078CEE, i_078cee),
    (0x078CF0, i_078cf0),
    (0x078CF4, i_078cf4),
    (0x078CF8, i_078cf8),
    (0x078CFE, i_078cfe),
    (0x078D02, i_078d02),
    (0x078D08, i_078d08),
    (0x078D0E, i_078d0e),
    (0x078D12, i_078d12),
    (0x078D14, i_078d14),
    (0x078D1A, i_078d1a),
    (0x078D20, i_078d20),
    (0x078D24, i_078d24),
    (0x078D28, i_078d28),
    (0x078D2A, i_078d2a),
    (0x078D2E, i_078d2e),
    (0x078D34, i_078d34),
    (0x078D38, i_078d38),
    (0x078D3A, i_078d3a),
    (0x078D40, i_078d40),
    (0x078D46, i_078d46),
    (0x078D4A, i_078d4a),
    (0x078D4E, i_078d4e),
    (0x078D54, i_078d54),
    (0x078D58, i_078d58),
    (0x078D5C, i_078d5c),
    (0x078D62, i_078d62),
    (0x078D68, i_078d68),
    (0x078D6E, i_078d6e),
    (0x078D74, i_078d74),
    (0x078D78, i_078d78),
    (0x078D7E, i_078d7e),
    (0x078D82, i_078d82),
    (0x078D88, i_078d88),
    (0x078D8C, i_078d8c),
    (0x078D90, i_078d90),
    (0x078D94, i_078d94),
    (0x078D96, i_078d96),
    (0x078D98, i_078d98),
    (0x078D9A, i_078d9a),
    (0x078D9C, i_078d9c),
    (0x078D9E, i_078d9e),
    (0x078DA0, i_078da0),
    (0x078DA2, i_078da2),
    (0x078DA4, i_078da4),
    (0x078DAA, i_078daa),
    (0x078DAE, i_078dae),
    (0x078DB2, i_078db2),
    (0x078DB6, i_078db6),
    (0x078DB8, i_078db8),
    (0x078DBC, i_078dbc),
    (0x078DBE, i_078dbe),
    (0x078DC0, i_078dc0),
    (0x078DC6, i_078dc6),
    (0x078DCA, i_078dca),
    (0x078DCC, i_078dcc),
    (0x078DD0, i_078dd0),
    (0x078DD4, i_078dd4),
    (0x078DD8, i_078dd8),
    (0x078DDC, i_078ddc),
    (0x078DE0, i_078de0),
    (0x078DE4, i_078de4),
    (0x078DEA, i_078dea),
    (0x078DEE, i_078dee),
    (0x078DF4, i_078df4),
    (0x078DF6, i_078df6),
    (0x078DFC, i_078dfc),
    (0x078E00, i_078e00),
    (0x078E04, i_078e04),
    (0x078E08, i_078e08),
    (0x078E0E, i_078e0e),
    (0x078E14, i_078e14),
    (0x078E1A, i_078e1a),
    (0x078E1E, i_078e1e),
    (0x078E24, i_078e24),
    (0x078E2A, i_078e2a),
    (0x078E30, i_078e30),
    (0x078E34, i_078e34),
    (0x078E38, i_078e38),
    (0x078E3A, i_078e3a),
    (0x078E3C, i_078e3c),
    (0x078E3E, i_078e3e),
    (0x078E40, i_078e40),
    (0x078E42, i_078e42),
    (0x078E44, i_078e44),
    (0x078E46, i_078e46),
    (0x078EDE, i_078ede),
    (0x078EE2, i_078ee2),
    (0x078EE8, i_078ee8),
    (0x078EEE, i_078eee),
    (0x078EF2, i_078ef2),
    (0x078EF6, i_078ef6),
    (0x078EFC, i_078efc),
    (0x078F00, i_078f00),
    (0x078F02, i_078f02),
    (0x078F06, i_078f06),
    (0x078F0A, i_078f0a),
    (0x078F0E, i_078f0e),
    (0x078F12, i_078f12),
    (0x078F14, i_078f14),
    (0x078F18, i_078f18),
    (0x078F1E, i_078f1e),
    (0x078F22, i_078f22),
    (0x078F26, i_078f26),
    (0x078F2A, i_078f2a),
    (0x078F2C, i_078f2c),
    (0x078F30, i_078f30),
    (0x078F36, i_078f36),
    (0x078F3C, i_078f3c),
    (0x078F40, i_078f40),
    (0x078F46, i_078f46),
    (0x078F4A, i_078f4a),
    (0x078F4C, i_078f4c),
    (0x078F52, i_078f52),
    (0x078F56, i_078f56),
    (0x078F5C, i_078f5c),
    (0x078F62, i_078f62),
    (0x078F66, i_078f66),
    (0x078F6C, i_078f6c),
    (0x078F7E, i_078f7e),
    (0x078F82, i_078f82),
    (0x078F84, i_078f84),
    (0x078F88, i_078f88),
    (0x078F8A, i_078f8a),
    (0x078F90, i_078f90),
    (0x078F94, i_078f94),
    (0x078F98, i_078f98),
    (0x078F9C, i_078f9c),
    (0x078F9E, i_078f9e),
    (0x078FA2, i_078fa2),
    (0x078FA6, i_078fa6),
    (0x078FA8, i_078fa8),
    (0x078FAA, i_078faa),
    (0x078FAE, i_078fae),
    (0x078FB0, i_078fb0),
    (0x078FB2, i_078fb2),
    (0x078FB8, i_078fb8),
    (0x078FBA, i_078fba),
    (0x078FBE, i_078fbe),
    (0x07900E, i_07900e),
    (0x079012, i_079012),
    (0x079100, i_079100),
    (0x079104, i_079104),
    (0x079174, i_079174),
    (0x079178, i_079178),
    (0x07924A, i_07924a),
    (0x07924E, i_07924e),
    (0x079250, i_079250),
    (0x079254, i_079254),
    (0x079258, i_079258),
    (0x07925C, i_07925c),
    (0x079260, i_079260),
    (0x079264, i_079264),
    (0x079266, i_079266),
    (0x07926A, i_07926a),
    (0x07926C, i_07926c),
    (0x079270, i_079270),
    (0x079274, i_079274),
    (0x079278, i_079278),
    (0x07927C, i_07927c),
    (0x079280, i_079280),
    (0x079286, i_079286),
    (0x07928A, i_07928a),
    (0x079290, i_079290),
    (0x079296, i_079296),
    (0x079298, i_079298),
    (0x07929A, i_07929a),
    (0x0792A4, i_0792a4),
    (0x0792AE, i_0792ae),
    (0x0792B4, i_0792b4),
    (0x0792BA, i_0792ba),
    (0x0792C0, i_0792c0),
    (0x0792C6, i_0792c6),
    (0x0792CC, i_0792cc),
    (0x0792D2, i_0792d2),
    (0x0792D8, i_0792d8),
    (0x0792DE, i_0792de),
    (0x0792E4, i_0792e4),
    (0x0792EA, i_0792ea),
    (0x0792EE, i_0792ee),
    (0x0792F0, i_0792f0),
    (0x0792F4, i_0792f4),
    (0x0792FA, i_0792fa),
    (0x0792FE, i_0792fe),
    (0x079300, i_079300),
    (0x079306, i_079306),
    (0x07930A, i_07930a),
    (0x079310, i_079310),
    (0x079314, i_079314),
    (0x07931A, i_07931a),
    (0x07931E, i_07931e),
    (0x079324, i_079324),
    (0x079326, i_079326),
    (0x07932C, i_07932c),
    (0x079332, i_079332),
    (0x07943C, i_07943c),
    (0x079440, i_079440),
    (0x079442, i_079442),
    (0x07962A, i_07962a),
    (0x07962C, i_07962c),
    (0x079632, i_079632),
    (0x079636, i_079636),
    (0x07963A, i_07963a),
    (0x07963E, i_07963e),
    (0x079642, i_079642),
    (0x079644, i_079644),
    (0x079646, i_079646),
    (0x07964A, i_07964a),
    (0x07964C, i_07964c),
    (0x07964E, i_07964e),
    (0x079654, i_079654),
    (0x079658, i_079658),
    (0x07965E, i_07965e),
    (0x079664, i_079664),
    (0x07966A, i_07966a),
    (0x079670, i_079670),
    (0x079672, i_079672),
    (0x07967C, i_07967c),
    (0x079686, i_079686),
    (0x07968C, i_07968c),
    (0x079692, i_079692),
    (0x079698, i_079698),
    (0x07969E, i_07969e),
    (0x0796A4, i_0796a4),
    (0x0796AA, i_0796aa),
    (0x0796B0, i_0796b0),
    (0x0796B6, i_0796b6),
    (0x0796BC, i_0796bc),
    (0x0796C2, i_0796c2),
    (0x0796C6, i_0796c6),
    (0x0796C8, i_0796c8),
    (0x0796CE, i_0796ce),
    (0x0796D2, i_0796d2),
    (0x0796D8, i_0796d8),
    (0x0796DC, i_0796dc),
    (0x0796DE, i_0796de),
    (0x0796E4, i_0796e4),
    (0x0796E8, i_0796e8),
    (0x0796EA, i_0796ea),
    (0x0796F0, i_0796f0),
    (0x0796F4, i_0796f4),
    (0x0796F6, i_0796f6),
    (0x0796FA, i_0796fa),
    (0x0796FE, i_0796fe),
    (0x079700, i_079700),
    (0x079706, i_079706),
    (0x07970A, i_07970a),
    (0x07970E, i_07970e),
    (0x079712, i_079712),
    (0x079714, i_079714),
    (0x07971A, i_07971a),
    (0x07971E, i_07971e),
    (0x079722, i_079722),
    (0x079726, i_079726),
    (0x07972A, i_07972a),
    (0x07972E, i_07972e),
    (0x079732, i_079732),
    (0x079734, i_079734),
    (0x07973A, i_07973a),
    (0x07973E, i_07973e),
    (0x079742, i_079742),
    (0x079748, i_079748),
    (0x079750, i_079750),
    (0x079758, i_079758),
    (0x079760, i_079760),
    (0x079764, i_079764),
    (0x079768, i_079768),
    (0x07976C, i_07976c),
    (0x079772, i_079772),
    (0x079776, i_079776),
    (0x07977A, i_07977a),
    (0x07977E, i_07977e),
    (0x079780, i_079780),
    (0x079786, i_079786),
    (0x07978A, i_07978a),
    (0x07978E, i_07978e),
    (0x079792, i_079792),
    (0x079796, i_079796),
    (0x07979A, i_07979a),
    (0x07979E, i_07979e),
    (0x0797A0, i_0797a0),
    (0x0797A6, i_0797a6),
    (0x0797AA, i_0797aa),
    (0x0797AE, i_0797ae),
    (0x0797B4, i_0797b4),
    (0x0797BC, i_0797bc),
    (0x0797C4, i_0797c4),
    (0x0797CC, i_0797cc),
    (0x0797CE, i_0797ce),
    (0x0797D4, i_0797d4),
    (0x0797D8, i_0797d8),
    (0x0797DC, i_0797dc),
    (0x0797DE, i_0797de),
    (0x0797E2, i_0797e2),
    (0x0797E6, i_0797e6),
    (0x0797EA, i_0797ea),
    (0x0797EC, i_0797ec),
    (0x0797F0, i_0797f0),
    (0x0797F2, i_0797f2),
    (0x0797F6, i_0797f6),
    (0x0797FC, i_0797fc),
    (0x079802, i_079802),
    (0x07980A, i_07980a),
    (0x079810, i_079810),
    (0x079816, i_079816),
    (0x07981A, i_07981a),
    (0x07981C, i_07981c),
    (0x079820, i_079820),
    (0x079826, i_079826),
    (0x079828, i_079828),
    (0x07982C, i_07982c),
    (0x079830, i_079830),
    (0x079834, i_079834),
    (0x079838, i_079838),
    (0x07983C, i_07983c),
    (0x07983E, i_07983e),
    (0x079842, i_079842),
    (0x079844, i_079844),
    (0x079846, i_079846),
    (0x07984C, i_07984c),
    (0x079850, i_079850),
    (0x079854, i_079854),
    (0x079856, i_079856),
    (0x07985C, i_07985c),
    (0x079860, i_079860),
    (0x079864, i_079864),
    (0x079868, i_079868),
    (0x07986C, i_07986c),
    (0x079870, i_079870),
    (0x079874, i_079874),
    (0x079878, i_079878),
    (0x07987C, i_07987c),
    (0x079880, i_079880),
    (0x079884, i_079884),
    (0x07988A, i_07988a),
    (0x07988E, i_07988e),
    (0x079892, i_079892),
    (0x079896, i_079896),
    (0x07989A, i_07989a),
    (0x07989E, i_07989e),
    (0x0798A2, i_0798a2),
    (0x0798A6, i_0798a6),
    (0x0798AA, i_0798aa),
    (0x0798AC, i_0798ac),
    (0x0798AE, i_0798ae),
    (0x0798B4, i_0798b4),
    (0x0798B8, i_0798b8),
    (0x0798BA, i_0798ba),
    (0x0798BE, i_0798be),
    (0x0798C2, i_0798c2),
    (0x0798C4, i_0798c4),
    (0x0798C6, i_0798c6),
    (0x0798CA, i_0798ca),
    (0x0798CC, i_0798cc),
    (0x0798D0, i_0798d0),
    (0x0798D2, i_0798d2),
    (0x0798D6, i_0798d6),
    (0x0798DA, i_0798da),
    (0x0798DE, i_0798de),
    (0x0798E2, i_0798e2),
    (0x0798E4, i_0798e4),
    (0x0798E6, i_0798e6),
    (0x0798EA, i_0798ea),
    (0x0798EC, i_0798ec),
    (0x0798F0, i_0798f0),
    (0x0798F2, i_0798f2),
    (0x0798F6, i_0798f6),
    (0x0798F8, i_0798f8),
    (0x0798FA, i_0798fa),
    (0x0798FE, i_0798fe),
    (0x079902, i_079902),
    (0x079906, i_079906),
    (0x079908, i_079908),
    (0x07990A, i_07990a),
    (0x07990C, i_07990c),
    (0x079910, i_079910),
    (0x079912, i_079912),
    (0x079914, i_079914),
    (0x079918, i_079918),
    (0x07991C, i_07991c),
    (0x07991E, i_07991e),
    (0x079920, i_079920),
    (0x079924, i_079924),
    (0x079928, i_079928),
    (0x07992A, i_07992a),
    (0x079930, i_079930),
    (0x079934, i_079934),
    (0x079936, i_079936),
    (0x07993A, i_07993a),
    (0x07993E, i_07993e),
    (0x079942, i_079942),
    (0x079946, i_079946),
    (0x07994A, i_07994a),
    (0x07994E, i_07994e),
    (0x079950, i_079950),
    (0x079952, i_079952),
    (0x079954, i_079954),
    (0x079956, i_079956),
    (0x079958, i_079958),
    (0x07995A, i_07995a),
    (0x07995C, i_07995c),
    (0x07995E, i_07995e),
    (0x079964, i_079964),
    (0x079970, i_079970),
    (0x079976, i_079976),
    (0x079978, i_079978),
    (0x07997C, i_07997c),
    (0x079980, i_079980),
    (0x079982, i_079982),
    (0x079986, i_079986),
    (0x079988, i_079988),
    (0x07998C, i_07998c),
    (0x079990, i_079990),
    (0x079994, i_079994),
    (0x079998, i_079998),
    (0x07999C, i_07999c),
    (0x07999E, i_07999e),
    (0x0799A2, i_0799a2),
    (0x0799A4, i_0799a4),
    (0x0799AA, i_0799aa),
    (0x0799B2, i_0799b2),
    (0x0799B6, i_0799b6),
    (0x0799BA, i_0799ba),
    (0x0799BC, i_0799bc),
    (0x0799BE, i_0799be),
    (0x0799C0, i_0799c0),
    (0x0799C6, i_0799c6),
    (0x0799C8, i_0799c8),
    (0x0799CA, i_0799ca),
    (0x0799CC, i_0799cc),
    (0x0799CE, i_0799ce),
    (0x0799D0, i_0799d0),
    (0x0799D4, i_0799d4),
    (0x0799D8, i_0799d8),
    (0x0799DC, i_0799dc),
    (0x0799DE, i_0799de),
    (0x0799E2, i_0799e2),
    (0x0799E6, i_0799e6),
    (0x0799EA, i_0799ea),
    (0x0799EE, i_0799ee),
    (0x0799F0, i_0799f0),
    (0x0799F2, i_0799f2),
    (0x0799F8, i_0799f8),
    (0x0799FC, i_0799fc),
    (0x0799FE, i_0799fe),
    (0x079A00, i_079a00),
    (0x079A04, i_079a04),
    (0x079A06, i_079a06),
    (0x079A08, i_079a08),
    (0x079A0C, i_079a0c),
    (0x079A0E, i_079a0e),
    (0x079A12, i_079a12),
    (0x079A16, i_079a16),
    (0x079A18, i_079a18),
    (0x079A1E, i_079a1e),
    (0x079A20, i_079a20),
    (0x079A24, i_079a24),
    (0x079A26, i_079a26),
    (0x079A2A, i_079a2a),
    (0x079A2E, i_079a2e),
    (0x079A30, i_079a30),
    (0x079A32, i_079a32),
    (0x079A36, i_079a36),
    (0x079A3A, i_079a3a),
    (0x079A3C, i_079a3c),
    (0x079A40, i_079a40),
    (0x079A46, i_079a46),
    (0x079A4A, i_079a4a),
    (0x079A4E, i_079a4e),
    (0x079A50, i_079a50),
    (0x079A54, i_079a54),
    (0x079A58, i_079a58),
    (0x079A5C, i_079a5c),
    (0x079A60, i_079a60),
    (0x079A64, i_079a64),
    (0x079A66, i_079a66),
    (0x079A6A, i_079a6a),
    (0x079A6C, i_079a6c),
    (0x079A70, i_079a70),
    (0x079A76, i_079a76),
    (0x079A7C, i_079a7c),
    (0x079A82, i_079a82),
    (0x079A86, i_079a86),
    (0x079A88, i_079a88),
    (0x079A8A, i_079a8a),
    (0x079A90, i_079a90),
    (0x079A94, i_079a94),
    (0x079A9A, i_079a9a),
    (0x079AA0, i_079aa0),
    (0x079AA6, i_079aa6),
    (0x079AAC, i_079aac),
    (0x079AB2, i_079ab2),
    (0x079AB6, i_079ab6),
    (0x079AB8, i_079ab8),
    (0x079ABE, i_079abe),
    (0x079AC4, i_079ac4),
    (0x079ACA, i_079aca),
    (0x079ACE, i_079ace),
    (0x079AD2, i_079ad2),
    (0x079AD4, i_079ad4),
    (0x079AD8, i_079ad8),
    (0x079ADC, i_079adc),
    (0x079AE0, i_079ae0),
    (0x079AE2, i_079ae2),
    (0x079AE4, i_079ae4),
    (0x079AEA, i_079aea),
    (0x079AF0, i_079af0),
    (0x079AF6, i_079af6),
    (0x079AFA, i_079afa),
    (0x079AFC, i_079afc),
    (0x079B02, i_079b02),
    (0x079B08, i_079b08),
    (0x079B0C, i_079b0c),
    (0x079B10, i_079b10),
    (0x079B12, i_079b12),
    (0x079B16, i_079b16),
    (0x079B1A, i_079b1a),
    (0x079B1E, i_079b1e),
    (0x079B20, i_079b20),
    (0x079B22, i_079b22),
    (0x079B28, i_079b28),
    (0x079B2C, i_079b2c),
    (0x079B2E, i_079b2e),
    (0x079B34, i_079b34),
    (0x079B3A, i_079b3a),
    (0x079B3E, i_079b3e),
    (0x079B42, i_079b42),
    (0x079B48, i_079b48),
    (0x079B4A, i_079b4a),
    (0x079B4E, i_079b4e),
    (0x079B54, i_079b54),
    (0x079B58, i_079b58),
    (0x079B5C, i_079b5c),
    (0x079B5E, i_079b5e),
    (0x079B62, i_079b62),
    (0x079B64, i_079b64),
    (0x079B6E, i_079b6e),
    (0x079B74, i_079b74),
    (0x079B78, i_079b78),
    (0x079B7C, i_079b7c),
    (0x079B82, i_079b82),
    (0x079B86, i_079b86),
    (0x079B8A, i_079b8a),
    (0x079B8E, i_079b8e),
    (0x079B92, i_079b92),
    (0x079B98, i_079b98),
    (0x079B9C, i_079b9c),
    (0x079BA0, i_079ba0),
    (0x079BA6, i_079ba6),
    (0x079BAC, i_079bac),
    (0x079BB2, i_079bb2),
    (0x079BB8, i_079bb8),
    (0x079BBE, i_079bbe),
    (0x079BC2, i_079bc2),
    (0x079BC4, i_079bc4),
    (0x079BCA, i_079bca),
    (0x079BD0, i_079bd0),
    (0x079BD4, i_079bd4),
    (0x079BDA, i_079bda),
    (0x079BDE, i_079bde),
    (0x079BE4, i_079be4),
    (0x079BE8, i_079be8),
    (0x079BEE, i_079bee),
    (0x079BF0, i_079bf0),
    (0x079BF6, i_079bf6),
    (0x079BFA, i_079bfa),
    (0x079BFE, i_079bfe),
    (0x079C00, i_079c00),
    (0x079C06, i_079c06),
    (0x079C0A, i_079c0a),
    (0x079C0E, i_079c0e),
    (0x079C10, i_079c10),
    (0x079C12, i_079c12),
    (0x079C18, i_079c18),
    (0x079C1E, i_079c1e),
    (0x079C22, i_079c22),
    (0x079C24, i_079c24),
    (0x079C2A, i_079c2a),
    (0x079C30, i_079c30),
    (0x079C34, i_079c34),
    (0x079C38, i_079c38),
    (0x079C3A, i_079c3a),
    (0x079C3C, i_079c3c),
    (0x079C42, i_079c42),
    (0x079C46, i_079c46),
    (0x079C48, i_079c48),
    (0x079C4E, i_079c4e),
    (0x079C54, i_079c54),
    (0x079C58, i_079c58),
    (0x079C5C, i_079c5c),
    (0x079C60, i_079c60),
    (0x079C64, i_079c64),
    (0x079C68, i_079c68),
    (0x079C6E, i_079c6e),
    (0x079C70, i_079c70),
    (0x079C74, i_079c74),
    (0x079C78, i_079c78),
    (0x079C7C, i_079c7c),
    (0x079C80, i_079c80),
    (0x079C84, i_079c84),
    (0x079C86, i_079c86),
    (0x079C8A, i_079c8a),
    (0x079C8C, i_079c8c),
    (0x079C90, i_079c90),
    (0x079C96, i_079c96),
    (0x079C9C, i_079c9c),
    (0x079CA2, i_079ca2),
    (0x079CA6, i_079ca6),
    (0x079CA8, i_079ca8),
    (0x079CAA, i_079caa),
    (0x079CB0, i_079cb0),
    (0x079CB4, i_079cb4),
    (0x079CBA, i_079cba),
    (0x079CC0, i_079cc0),
    (0x079CC6, i_079cc6),
    (0x079CCC, i_079ccc),
    (0x079CD2, i_079cd2),
    (0x079CD6, i_079cd6),
    (0x079CD8, i_079cd8),
    (0x079CDE, i_079cde),
    (0x079CE4, i_079ce4),
    (0x079CEA, i_079cea),
    (0x079CEE, i_079cee),
    (0x079CF2, i_079cf2),
    (0x079CF4, i_079cf4),
    (0x079CF8, i_079cf8),
    (0x079CFC, i_079cfc),
    (0x079D00, i_079d00),
    (0x079D02, i_079d02),
    (0x079D04, i_079d04),
    (0x079D0A, i_079d0a),
    (0x079D0E, i_079d0e),
    (0x079D12, i_079d12),
    (0x079D18, i_079d18),
    (0x079D1E, i_079d1e),
    (0x079D24, i_079d24),
    (0x079D2A, i_079d2a),
    (0x079D2E, i_079d2e),
    (0x079D30, i_079d30),
    (0x079D36, i_079d36),
    (0x079D3C, i_079d3c),
    (0x079D40, i_079d40),
    (0x079D44, i_079d44),
    (0x079D46, i_079d46),
    (0x079D4A, i_079d4a),
    (0x079D4E, i_079d4e),
    (0x079D52, i_079d52),
    (0x079D54, i_079d54),
    (0x079D56, i_079d56),
    (0x079D5C, i_079d5c),
    (0x079D60, i_079d60),
    (0x079D62, i_079d62),
    (0x079D68, i_079d68),
    (0x079D6E, i_079d6e),
    (0x079D72, i_079d72),
    (0x079D76, i_079d76),
    (0x079D7C, i_079d7c),
    (0x079D7E, i_079d7e),
    (0x079D84, i_079d84),
    (0x079D8A, i_079d8a),
    (0x079D8E, i_079d8e),
    (0x079D92, i_079d92),
    (0x079D94, i_079d94),
    (0x079D98, i_079d98),
    (0x079D9A, i_079d9a),
    (0x079D9E, i_079d9e),
    (0x079DA2, i_079da2),
    (0x079DA6, i_079da6),
    (0x079DAA, i_079daa),
    (0x079DB0, i_079db0),
    (0x079DB4, i_079db4),
    (0x079DB8, i_079db8),
    (0x079DBE, i_079dbe),
    (0x079DC4, i_079dc4),
    (0x079DCA, i_079dca),
    (0x079DD0, i_079dd0),
    (0x079DD6, i_079dd6),
    (0x079DDA, i_079dda),
    (0x079DDC, i_079ddc),
    (0x079DE2, i_079de2),
    (0x079DE8, i_079de8),
    (0x079DEC, i_079dec),
    (0x079DF2, i_079df2),
    (0x079DF6, i_079df6),
    (0x079DFC, i_079dfc),
    (0x079E00, i_079e00),
    (0x079E06, i_079e06),
    (0x079E08, i_079e08),
    (0x079E0E, i_079e0e),
    (0x079E12, i_079e12),
    (0x079E16, i_079e16),
    (0x079E18, i_079e18),
    (0x079E1E, i_079e1e),
    (0x079E22, i_079e22),
    (0x079E26, i_079e26),
    (0x079E28, i_079e28),
    (0x079E2A, i_079e2a),
    (0x079E30, i_079e30),
    (0x079E34, i_079e34),
    (0x079E38, i_079e38),
    (0x079E3E, i_079e3e),
    (0x079E44, i_079e44),
    (0x079E4A, i_079e4a),
    (0x079E4E, i_079e4e),
    (0x079E50, i_079e50),
    (0x079E56, i_079e56),
    (0x079E5C, i_079e5c),
    (0x079E60, i_079e60),
    (0x079E64, i_079e64),
    (0x079E66, i_079e66),
    (0x079E68, i_079e68),
    (0x079E6E, i_079e6e),
    (0x079E72, i_079e72),
    (0x079E74, i_079e74),
    (0x079E7A, i_079e7a),
    (0x079E80, i_079e80),
    (0x079E84, i_079e84),
    (0x079E88, i_079e88),
    (0x079E8C, i_079e8c),
    (0x079E90, i_079e90),
    (0x079E94, i_079e94),
    (0x079E9A, i_079e9a),
    (0x079E9C, i_079e9c),
    (0x079EA0, i_079ea0),
    (0x079EA4, i_079ea4),
    (0x079EA8, i_079ea8),
    (0x079EAC, i_079eac),
    (0x079EB0, i_079eb0),
    (0x079EB2, i_079eb2),
    (0x079EB6, i_079eb6),
    (0x079EB8, i_079eb8),
    (0x079EBE, i_079ebe),
    (0x079EC2, i_079ec2),
    (0x079EC8, i_079ec8),
    (0x079ECA, i_079eca),
    (0x079ECE, i_079ece),
    (0x079ED2, i_079ed2),
    (0x079ED6, i_079ed6),
    (0x079EDC, i_079edc),
    (0x079EE2, i_079ee2),
    (0x079EE8, i_079ee8),
    (0x079EEC, i_079eec),
    (0x079EF2, i_079ef2),
    (0x079EF8, i_079ef8),
    (0x079EFE, i_079efe),
    (0x079F02, i_079f02),
    (0x079F08, i_079f08),
    (0x079F0E, i_079f0e),
    (0x079F14, i_079f14),
    (0x079F18, i_079f18),
    (0x079F1A, i_079f1a),
    (0x079F1E, i_079f1e),
    (0x079F22, i_079f22),
    (0x079F26, i_079f26),
    (0x079F2A, i_079f2a),
    (0x079F30, i_079f30),
    (0x079F36, i_079f36),
    (0x079F3A, i_079f3a),
    (0x079F40, i_079f40),
    (0x079F46, i_079f46),
    (0x079F48, i_079f48),
    (0x079F4C, i_079f4c),
    (0x079F52, i_079f52),
    (0x079F54, i_079f54),
    (0x079F58, i_079f58),
    (0x079F5C, i_079f5c),
    (0x079F62, i_079f62),
    (0x079F66, i_079f66),
    (0x079F68, i_079f68),
    (0x079F6A, i_079f6a),
    (0x079F70, i_079f70),
    (0x079F74, i_079f74),
    (0x079F7A, i_079f7a),
    (0x079F80, i_079f80),
    (0x079F86, i_079f86),
    (0x079F8C, i_079f8c),
    (0x079F92, i_079f92),
    (0x079F96, i_079f96),
    (0x079F98, i_079f98),
    (0x079F9E, i_079f9e),
    (0x079FA4, i_079fa4),
    (0x079FA8, i_079fa8),
    (0x079FAE, i_079fae),
    (0x079FB2, i_079fb2),
    (0x079FB8, i_079fb8),
    (0x079FBC, i_079fbc),
    (0x079FC2, i_079fc2),
    (0x079FC4, i_079fc4),
    (0x079FCA, i_079fca),
    (0x079FD0, i_079fd0),
    (0x079FD6, i_079fd6),
    (0x079FDC, i_079fdc),
    (0x079FE0, i_079fe0),
    (0x079FE4, i_079fe4),
    (0x079FE6, i_079fe6),
    (0x079FE8, i_079fe8),
    (0x079FEC, i_079fec),
    (0x079FF2, i_079ff2),
    (0x079FF8, i_079ff8),
    (0x079FFA, i_079ffa),
    (0x079FFE, i_079ffe),
    (0x07A002, i_07a002),
    (0x07A006, i_07a006),
    (0x07A008, i_07a008),
    (0x07A00A, i_07a00a),
    (0x07A010, i_07a010),
    (0x07A014, i_07a014),
    (0x07A01A, i_07a01a),
    (0x07A020, i_07a020),
    (0x07A026, i_07a026),
    (0x07A02A, i_07a02a),
    (0x07A030, i_07a030),
    (0x07A036, i_07a036),
    (0x07A03A, i_07a03a),
    (0x07A03C, i_07a03c),
    (0x07A042, i_07a042),
    (0x07A048, i_07a048),
    (0x07A04C, i_07a04c),
    (0x07A052, i_07a052),
    (0x07A056, i_07a056),
    (0x07A05A, i_07a05a),
    (0x07A05C, i_07a05c),
    (0x07A060, i_07a060),
    (0x07A066, i_07a066),
    (0x07A06A, i_07a06a),
    (0x07A070, i_07a070),
    (0x07A074, i_07a074),
    (0x07A07A, i_07a07a),
    (0x07A080, i_07a080),
    (0x07A082, i_07a082),
    (0x07A088, i_07a088),
    (0x07A08E, i_07a08e),
    (0x07A092, i_07a092),
    (0x07A098, i_07a098),
    (0x07A09E, i_07a09e),
    (0x07A0A4, i_07a0a4),
    (0x07A0A8, i_07a0a8),
    (0x07A0AC, i_07a0ac),
    (0x07A0AE, i_07a0ae),
    (0x07A0B0, i_07a0b0),
    (0x07A0B6, i_07a0b6),
    (0x07A0B8, i_07a0b8),
    (0x07A0BE, i_07a0be),
    (0x07A0C4, i_07a0c4),
    (0x07A0C8, i_07a0c8),
    (0x07A0CA, i_07a0ca),
    (0x07A0D0, i_07a0d0),
    (0x07A0D6, i_07a0d6),
    (0x07A0DA, i_07a0da),
    (0x07A0DE, i_07a0de),
    (0x07A0E0, i_07a0e0),
    (0x07A0E4, i_07a0e4),
    (0x07A0EA, i_07a0ea),
    (0x07A0EE, i_07a0ee),
    (0x07A0F4, i_07a0f4),
    (0x07A0F8, i_07a0f8),
    (0x07A0FE, i_07a0fe),
    (0x07A104, i_07a104),
    (0x07A106, i_07a106),
    (0x07A10C, i_07a10c),
    (0x07A112, i_07a112),
    (0x07A114, i_07a114),
    (0x07A11A, i_07a11a),
    (0x07A120, i_07a120),
    (0x07A126, i_07a126),
    (0x07A12A, i_07a12a),
    (0x07A12E, i_07a12e),
    (0x07A130, i_07a130),
    (0x07A132, i_07a132),
    (0x07A136, i_07a136),
    (0x07A13C, i_07a13c),
    (0x07A142, i_07a142),
    (0x07A148, i_07a148),
    (0x07A14C, i_07a14c),
    (0x07A14E, i_07a14e),
    (0x07A154, i_07a154),
    (0x07A15A, i_07a15a),
    (0x07A160, i_07a160),
    (0x07A164, i_07a164),
    (0x07A168, i_07a168),
    (0x07A16C, i_07a16c),
    (0x07A170, i_07a170),
    (0x07A176, i_07a176),
    (0x07A17A, i_07a17a),
    (0x07A17E, i_07a17e),
    (0x07A180, i_07a180),
    (0x07A186, i_07a186),
    (0x07A18C, i_07a18c),
    (0x07A192, i_07a192),
    (0x07A196, i_07a196),
    (0x07A19A, i_07a19a),
    (0x07A19C, i_07a19c),
    (0x07A19E, i_07a19e),
    (0x07A1A2, i_07a1a2),
    (0x07A1A8, i_07a1a8),
    (0x07A1AA, i_07a1aa),
    (0x07A1AE, i_07a1ae),
    (0x07A1B2, i_07a1b2),
    (0x07A1B8, i_07a1b8),
    (0x07A1BC, i_07a1bc),
    (0x07A1BE, i_07a1be),
    (0x07A1C0, i_07a1c0),
    (0x07A1C6, i_07a1c6),
    (0x07A1CA, i_07a1ca),
    (0x07A1D0, i_07a1d0),
    (0x07A1D6, i_07a1d6),
    (0x07A1DC, i_07a1dc),
    (0x07A1E0, i_07a1e0),
    (0x07A1E4, i_07a1e4),
    (0x07A1EA, i_07a1ea),
    (0x07A1F0, i_07a1f0),
    (0x07A1F4, i_07a1f4),
    (0x07A1F8, i_07a1f8),
    (0x07A1FC, i_07a1fc),
    (0x07A200, i_07a200),
    (0x07A204, i_07a204),
    (0x07A208, i_07a208),
    (0x07A20E, i_07a20e),
    (0x07A214, i_07a214),
    (0x07A21A, i_07a21a),
    (0x07A21E, i_07a21e),
    (0x07A220, i_07a220),
    (0x07A226, i_07a226),
    (0x07A22C, i_07a22c),
    (0x07A230, i_07a230),
    (0x07A234, i_07a234),
    (0x07A238, i_07a238),
    (0x07A23C, i_07a23c),
    (0x07A240, i_07a240),
    (0x07A244, i_07a244),
    (0x07A248, i_07a248),
    (0x07A24C, i_07a24c),
    (0x07A24E, i_07a24e),
    (0x07A254, i_07a254),
    (0x07A258, i_07a258),
    (0x07A25C, i_07a25c),
    (0x07A25E, i_07a25e),
    (0x07A264, i_07a264),
    (0x07A26A, i_07a26a),
    (0x07A270, i_07a270),
    (0x07A274, i_07a274),
    (0x07A278, i_07a278),
    (0x07A27A, i_07a27a),
    (0x07A27C, i_07a27c),
    (0x07A280, i_07a280),
    (0x07A284, i_07a284),
    (0x07A288, i_07a288),
    (0x07A28C, i_07a28c),
    (0x07A290, i_07a290),
    (0x07A296, i_07a296),
    (0x07A29C, i_07a29c),
    (0x07A2A0, i_07a2a0),
    (0x07A2A6, i_07a2a6),
    (0x07A2AC, i_07a2ac),
    (0x07A2B0, i_07a2b0),
    (0x07A2B6, i_07a2b6),
    (0x07A2BC, i_07a2bc),
    (0x07A2C0, i_07a2c0),
    (0x07A2C2, i_07a2c2),
    (0x07A2C8, i_07a2c8),
    (0x07A2CE, i_07a2ce),
    (0x07A2D2, i_07a2d2),
    (0x07A2D6, i_07a2d6),
    (0x07A2D8, i_07a2d8),
    (0x07A2DE, i_07a2de),
    (0x07A2E2, i_07a2e2),
    (0x07A2E6, i_07a2e6),
    (0x07A2E8, i_07a2e8),
    (0x07A2EE, i_07a2ee),
    (0x07A2F4, i_07a2f4),
    (0x07A2FA, i_07a2fa),
    (0x07A2FE, i_07a2fe),
    (0x07A302, i_07a302),
    (0x07A304, i_07a304),
    (0x07A306, i_07a306),
    (0x07A30C, i_07a30c),
    (0x07A312, i_07a312),
    (0x07A318, i_07a318),
    (0x07A31C, i_07a31c),
    (0x07A31E, i_07a31e),
    (0x07A324, i_07a324),
    (0x07A32A, i_07a32a),
    (0x07A32E, i_07a32e),
    (0x07A334, i_07a334),
    (0x07A336, i_07a336),
    (0x07A338, i_07a338),
    (0x07A33E, i_07a33e),
    (0x07A342, i_07a342),
    (0x07A348, i_07a348),
    (0x07A34C, i_07a34c),
    (0x07A350, i_07a350),
    (0x07A354, i_07a354),
    (0x07A358, i_07a358),
    (0x07A35E, i_07a35e),
    (0x07A362, i_07a362),
    (0x07A368, i_07a368),
    (0x07A36E, i_07a36e),
    (0x07A372, i_07a372),
    (0x07A378, i_07a378),
    (0x07A37E, i_07a37e),
    (0x07A382, i_07a382),
    (0x07A384, i_07a384),
    (0x07A38A, i_07a38a),
    (0x07A390, i_07a390),
    (0x07A396, i_07a396),
    (0x07A39C, i_07a39c),
    (0x07A3A2, i_07a3a2),
    (0x07A3A6, i_07a3a6),
    (0x07A3AA, i_07a3aa),
    (0x07A3AC, i_07a3ac),
    (0x07A3B0, i_07a3b0),
    (0x07A3B4, i_07a3b4),
    (0x07A3BA, i_07a3ba),
    (0x07A3BC, i_07a3bc),
    (0x07A3C2, i_07a3c2),
    (0x07A3C8, i_07a3c8),
    (0x07A3CE, i_07a3ce),
    (0x07A3D4, i_07a3d4),
    (0x07A3DA, i_07a3da),
    (0x07A3DE, i_07a3de),
    (0x07A3E2, i_07a3e2),
    (0x07A3E4, i_07a3e4),
    (0x07A3E6, i_07a3e6),
    (0x07A3EE, i_07a3ee),
    (0x07A3F2, i_07a3f2),
    (0x07A3F8, i_07a3f8),
    (0x07A3FC, i_07a3fc),
    (0x07A400, i_07a400),
    (0x07A404, i_07a404),
    (0x07A408, i_07a408),
    (0x07A40E, i_07a40e),
    (0x07A414, i_07a414),
    (0x07A41C, i_07a41c),
    (0x07A422, i_07a422),
    (0x07A426, i_07a426),
    (0x07A428, i_07a428),
    (0x07A42E, i_07a42e),
    (0x07A432, i_07a432),
    (0x07A438, i_07a438),
    (0x07A43A, i_07a43a),
    (0x07A43E, i_07a43e),
    (0x07A442, i_07a442),
    (0x07A446, i_07a446),
    (0x07A44A, i_07a44a),
    (0x07A44E, i_07a44e),
    (0x07A450, i_07a450),
    (0x07A454, i_07a454),
    (0x07A456, i_07a456),
    (0x07A45C, i_07a45c),
    (0x07A460, i_07a460),
    (0x07A464, i_07a464),
    (0x07A46A, i_07a46a),
    (0x07A470, i_07a470),
    (0x07A474, i_07a474),
    (0x07A478, i_07a478),
    (0x07A47E, i_07a47e),
    (0x07A484, i_07a484),
    (0x07A48A, i_07a48a),
    (0x07A490, i_07a490),
    (0x07A496, i_07a496),
    (0x07A49C, i_07a49c),
    (0x07A4A2, i_07a4a2),
    (0x07A4A8, i_07a4a8),
    (0x07A4AE, i_07a4ae),
    (0x07A4B4, i_07a4b4),
    (0x07A4BA, i_07a4ba),
    (0x07A4C0, i_07a4c0),
    (0x07A4C6, i_07a4c6),
    (0x07A4CC, i_07a4cc),
    (0x07A4D2, i_07a4d2),
    (0x07A4D8, i_07a4d8),
    (0x07A4DE, i_07a4de),
    (0x07A4E4, i_07a4e4),
    (0x07A4EA, i_07a4ea),
    (0x07A4F0, i_07a4f0),
    (0x07A4F6, i_07a4f6),
    (0x07A4FC, i_07a4fc),
    (0x07A500, i_07a500),
    (0x07A504, i_07a504),
    (0x07A50A, i_07a50a),
    (0x07A510, i_07a510),
    (0x07A514, i_07a514),
    (0x07A518, i_07a518),
    (0x07A51C, i_07a51c),
    (0x07A522, i_07a522),
    (0x07A526, i_07a526),
    (0x07A52A, i_07a52a),
    (0x07A52E, i_07a52e),
    (0x07A532, i_07a532),
    (0x07A538, i_07a538),
    (0x07A53C, i_07a53c),
    (0x07A540, i_07a540),
    (0x07A544, i_07a544),
    (0x07A548, i_07a548),
    (0x07A54A, i_07a54a),
    (0x07A54E, i_07a54e),
    (0x07A554, i_07a554),
    (0x07A558, i_07a558),
    (0x07A55E, i_07a55e),
    (0x07A562, i_07a562),
    (0x07A566, i_07a566),
    (0x07A56A, i_07a56a),
    (0x07A56E, i_07a56e),
    (0x07A570, i_07a570),
    (0x07A574, i_07a574),
    (0x07A57A, i_07a57a),
    (0x07A57E, i_07a57e),
    (0x07A584, i_07a584),
    (0x07A588, i_07a588),
    (0x07A58C, i_07a58c),
    (0x07A590, i_07a590),
    (0x07A594, i_07a594),
    (0x07A596, i_07a596),
    (0x07A59C, i_07a59c),
    (0x07A5A2, i_07a5a2),
    (0x07A5A6, i_07a5a6),
    (0x07A5AC, i_07a5ac),
    (0x07A5B2, i_07a5b2),
    (0x07A5B6, i_07a5b6),
    (0x07A5BA, i_07a5ba),
    (0x07A5BE, i_07a5be),
    (0x07A5C2, i_07a5c2),
    (0x07A5C4, i_07a5c4),
    (0x07A5CA, i_07a5ca),
    (0x07A5D0, i_07a5d0),
    (0x07A5D4, i_07a5d4),
    (0x07A5DA, i_07a5da),
    (0x07A5E0, i_07a5e0),
    (0x07A5E4, i_07a5e4),
    (0x07A5E8, i_07a5e8),
    (0x07A5EC, i_07a5ec),
    (0x07A5EE, i_07a5ee),
    (0x07A5F2, i_07a5f2),
    (0x07A5F8, i_07a5f8),
    (0x07A5FC, i_07a5fc),
    (0x07A602, i_07a602),
    (0x07A606, i_07a606),
    (0x07A60A, i_07a60a),
    (0x07A60E, i_07a60e),
    (0x07A614, i_07a614),
    (0x07A61A, i_07a61a),
    (0x07A61E, i_07a61e),
    (0x07A622, i_07a622),
    (0x07A626, i_07a626),
    (0x07A62C, i_07a62c),
    (0x07A630, i_07a630),
    (0x07A634, i_07a634),
    (0x07A638, i_07a638),
    (0x07A63C, i_07a63c),
    (0x07A642, i_07a642),
    (0x07A646, i_07a646),
    (0x07A64C, i_07a64c),
    (0x07A650, i_07a650),
    (0x07A654, i_07a654),
    (0x07A658, i_07a658),
    (0x07A65A, i_07a65a),
    (0x07A65E, i_07a65e),
    (0x07A664, i_07a664),
    (0x07A668, i_07a668),
    (0x07A66E, i_07a66e),
    (0x07A672, i_07a672),
    (0x07A676, i_07a676),
    (0x07A67A, i_07a67a),
    (0x07A67E, i_07a67e),
    (0x07A680, i_07a680),
    (0x07A684, i_07a684),
    (0x07A68A, i_07a68a),
    (0x07A68E, i_07a68e),
    (0x07A694, i_07a694),
    (0x07A698, i_07a698),
    (0x07A69E, i_07a69e),
    (0x07A6A2, i_07a6a2),
    (0x07A6A6, i_07a6a6),
    (0x07A6A8, i_07a6a8),
    (0x07A6AC, i_07a6ac),
    (0x07A6B0, i_07a6b0),
    (0x07A6B2, i_07a6b2),
    (0x07A6B6, i_07a6b6),
    (0x07A6BA, i_07a6ba),
    (0x07A6BC, i_07a6bc),
    (0x07A6C2, i_07a6c2),
    (0x07A6C4, i_07a6c4),
    (0x07A6C8, i_07a6c8),
    (0x07A6CC, i_07a6cc),
    (0x07A6CE, i_07a6ce),
    (0x07A6D2, i_07a6d2),
    (0x07A6D6, i_07a6d6),
    (0x07A6D8, i_07a6d8),
    (0x07A6DC, i_07a6dc),
    (0x07A6E0, i_07a6e0),
    (0x07A6E2, i_07a6e2),
    (0x07A6E8, i_07a6e8),
    (0x07A6EA, i_07a6ea),
    (0x07A6EE, i_07a6ee),
    (0x07A6F2, i_07a6f2),
    (0x07A6F4, i_07a6f4),
    (0x07A6F8, i_07a6f8),
];
