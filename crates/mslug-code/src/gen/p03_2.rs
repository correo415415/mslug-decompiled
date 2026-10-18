// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p03_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $034642: jsr $32d32(pc)
pub fn i_034642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34646;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34646);
    cpu.pc = 0x32D32;
    16
}
/// $034646: jsr $27a92.l
pub fn i_034646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3464C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3464C);
    cpu.pc = 0x27A92;
    16
}
/// $03464C: jsr $32aa8(pc)
pub fn i_03464c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34650;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34650);
    cpu.pc = 0x32AA8;
    16
}
/// $034650: bcc.w $3465a
pub fn i_034650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34654;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3465A; return 10; }
    8
}
/// $034654: lea.l $34704(pc), a1
pub fn i_034654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34658;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $034658: move.l a1, (a6)
pub fn i_034658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3465A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03465A: cmpi.b #$4, $78(a6)
pub fn i_03465a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34660;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $034660: bne.w $346ac
pub fn i_034660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34664;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x346AC; return 10; }
    8
}
/// $034664: cmpi.l #$39e54, $74(a6)
pub fn i_034664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3466C;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x39E54, Size::Long, false);
    20
}
/// $03466C: bne.w $346a8
pub fn i_03466c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34670;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x346A8; return 10; }
    8
}
/// $034670: move.w #$1, $72(a6)
pub fn i_034670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34676;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034676: jsr $2abcc.l
pub fn i_034676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3467C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3467C);
    cpu.pc = 0x2ABCC;
    16
}
/// $03467C: bcs.w $34696
pub fn i_03467c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34680;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x34696; return 10; }
    8
}
/// $034680: move.b #$10, $70(a6)
pub fn i_034680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34686;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034686: lea.l $2796c4.l, a0
pub fn i_034686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3468C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2796C4;
    20
}
/// $03468C: move.l -$4(a0), $74(a6)
pub fn i_03468c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34692;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034692: bra.w $346a8
pub fn i_034692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34696;
    cpu.ir = 0x6000;
    cpu.pc = 0x346A8;
    10
}
/// $034696: move.b #$10, $70(a6)
pub fn i_034696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3469C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03469C: lea.l $27991c.l, a0
pub fn i_03469c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27991C;
    20
}
/// $0346A2: move.l -$4(a0), $74(a6)
pub fn i_0346a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346A8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0346A8: bra.w $34700
pub fn i_0346a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x34700;
    10
}
/// $0346AC: cmpi.l #$39e54, $74(a6)
pub fn i_0346ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346B4;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x39E54, Size::Long, false);
    20
}
/// $0346B4: bne.w $346f0
pub fn i_0346b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x346F0; return 10; }
    8
}
/// $0346B8: move.w #$0, $72(a6)
pub fn i_0346b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0346BE: jsr $2abcc.l
pub fn i_0346be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x346C4);
    cpu.pc = 0x2ABCC;
    16
}
/// $0346C4: bcs.w $346de
pub fn i_0346c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346C8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x346DE; return 10; }
    8
}
/// $0346C8: move.b #$10, $70(a6)
pub fn i_0346c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346CE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0346CE: lea.l $2796a4.l, a0
pub fn i_0346ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2796A4;
    20
}
/// $0346D4: move.l -$4(a0), $74(a6)
pub fn i_0346d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346DA;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0346DA: bra.w $346f0
pub fn i_0346da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x346F0;
    10
}
/// $0346DE: move.b #$10, $70(a6)
pub fn i_0346de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0346E4: lea.l $279926.l, a0
pub fn i_0346e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0346EA: move.l -$4(a0), $74(a6)
pub fn i_0346ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346F0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0346F0: btst.b #$0, $88(a6)
pub fn i_0346f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346F6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0346F6: beq.w $34700
pub fn i_0346f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346FA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x34700; return 10; }
    8
}
/// $0346FA: lea.l $33a5e(pc), a1
pub fn i_0346fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x346FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $0346FE: move.l a1, (a6)
pub fn i_0346fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34700;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034700: bra.w $34a00
pub fn i_034700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34704;
    cpu.ir = 0x6000;
    cpu.pc = 0x34A00;
    10
}
// ===== Player_Stand_034704  ($034704, 1076 B, player/states_0342c4.s)
/// $034704: cmpi.b #$13, $70(a6)
pub fn i_034704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3470A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x13, Size::Byte, false);
    16
}
/// $03470A: bcs.w $34760
pub fn i_03470a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3470E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x34760; return 10; }
    8
}
/// $03470E: cmpi.b #$24, $70(a6)
pub fn i_03470e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34714;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $034714: bcc.w $34760
pub fn i_034714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34718;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34760; return 10; }
    8
}
/// $034718: jsr $2abcc.l
pub fn i_034718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3471E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3471E);
    cpu.pc = 0x2ABCC;
    16
}
/// $03471E: bcs.w $3473e
pub fn i_03471e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34722;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3473E; return 10; }
    8
}
/// $034722: move.w #$0, $7c(a6)
pub fn i_034722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34728;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034728: move.w #$0, $7e(a6)
pub fn i_034728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3472E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03472E: lea.l $2796a4.l, a0
pub fn i_03472e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34734;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2796A4;
    20
}
/// $034734: jsr $28cd4.l
pub fn i_034734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3473A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3473A);
    cpu.pc = 0x28CD4;
    16
}
/// $03473A: bra.w $34756
pub fn i_03473a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3473E;
    cpu.ir = 0x6000;
    cpu.pc = 0x34756;
    10
}
/// $03473E: move.w #$0, $7c(a6)
pub fn i_03473e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34744;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034744: move.w #$0, $7e(a6)
pub fn i_034744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3474A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03474A: lea.l $279926.l, a0
pub fn i_03474a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34750;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $034750: jsr $28cd4.l
pub fn i_034750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34756;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34756);
    cpu.pc = 0x28CD4;
    16
}
/// $034756: bclr.b #$5, $69(a6)
pub fn i_034756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3475C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03475C: bra.w $347e0
pub fn i_03475c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34760;
    cpu.ir = 0x6000;
    cpu.pc = 0x347E0;
    10
}
/// $034760: bclr.b #$2, $8c(a6)
pub fn i_034760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34766;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034766: bclr.b #$1, $8c(a6)
pub fn i_034766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3476C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03476C: bclr.b #$3, $8c(a6)
pub fn i_03476c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34772;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034772: jsr $2abcc.l
pub fn i_034772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34778;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34778);
    cpu.pc = 0x2ABCC;
    16
}
/// $034778: bcs.w $347b0
pub fn i_034778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3477C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x347B0; return 10; }
    8
}
/// $03477C: move.w #$0, $7c(a6)
pub fn i_03477c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34782;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034782: move.w #$0, $7e(a6)
pub fn i_034782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34788;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034788: move.b #$10, $70(a6)
pub fn i_034788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3478E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03478E: lea.l $2796a4.l, a0
pub fn i_03478e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34794;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2796A4;
    20
}
/// $034794: move.l -$4(a0), $74(a6)
pub fn i_034794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3479A;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03479A: move.b #$ff, $21(a6)
pub fn i_03479a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347A0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0347A0: lea.l $2796a4.l, a0
pub fn i_0347a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2796A4;
    20
}
/// $0347A6: jsr $28cd4.l
pub fn i_0347a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x347AC);
    cpu.pc = 0x28CD4;
    16
}
/// $0347AC: bra.w $347e0
pub fn i_0347ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x347E0;
    10
}
/// $0347B0: move.w #$0, $7c(a6)
pub fn i_0347b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0347B6: move.w #$0, $7e(a6)
pub fn i_0347b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0347BC: move.b #$10, $70(a6)
pub fn i_0347bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347C2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0347C2: lea.l $279926.l, a0
pub fn i_0347c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0347C8: move.l -$4(a0), $74(a6)
pub fn i_0347c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347CE;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0347CE: move.b #$ff, $21(a6)
pub fn i_0347ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347D4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0347D4: lea.l $279926.l, a0
pub fn i_0347d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0347DA: jsr $28cd4.l
pub fn i_0347da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x347E0);
    cpu.pc = 0x28CD4;
    16
}
/// $0347E0: bset.b #$7, $5b(a6)
pub fn i_0347e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347E6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0347E6: move.l #$32500, $60(a6)
pub fn i_0347e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347EE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0347EE: jsr $267e6.l
pub fn i_0347ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x347F4);
    cpu.pc = 0x267E6;
    16
}
/// $0347F4: clr.w $28(a6)
pub fn i_0347f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347F8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0347F8: lea.l $326e0(pc), a0
pub fn i_0347f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x347FC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $0347FC: move.l a0, $48(a6)
pub fn i_0347fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34800;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $034800: lea.l $34806(pc), a1
pub fn i_034800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34804;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34806;
    16
}
/// $034804: move.l a1, (a6)
pub fn i_034804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34806;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034806: jsr $32ff2(pc)
pub fn i_034806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3480A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3480A);
    cpu.pc = 0x32FF2;
    16
}
/// $03480A: jsr $32d32(pc)
pub fn i_03480a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3480E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3480E);
    cpu.pc = 0x32D32;
    16
}
/// $03480E: bcc.w $34818
pub fn i_03480e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34812;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34818; return 10; }
    8
}
/// $034812: lea.l $345b8(pc), a1
pub fn i_034812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34816;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345B8;
    16
}
/// $034816: move.l a1, (a6)
pub fn i_034816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34818;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034818: jsr $27a92.l
pub fn i_034818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3481E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3481E);
    cpu.pc = 0x27A92;
    16
}
/// $03481E: jsr $32aa8(pc)
pub fn i_03481e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34822;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34822);
    cpu.pc = 0x32AA8;
    16
}
/// $034822: bcc.w $34944
pub fn i_034822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34826;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34944; return 10; }
    8
}
/// $034826: move.w #$0, $7c(a6)
pub fn i_034826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3482C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03482C: move.w #$0, $7e(a6)
pub fn i_03482c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34832;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034832: bclr.b #$2, $8c(a6)
pub fn i_034832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34838;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034838: bclr.b #$1, $8c(a6)
pub fn i_034838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3483E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03483E: bclr.b #$3, $8c(a6)
pub fn i_03483e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34844;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034844: cmpi.b #$4, $78(a6)
pub fn i_034844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3484A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $03484A: bne.w $34890
pub fn i_03484a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3484E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34890; return 10; }
    8
}
/// $03484E: move.w #$1, $72(a6)
pub fn i_03484e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34854;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034854: jsr $2abcc.l
pub fn i_034854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3485A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3485A);
    cpu.pc = 0x2ABCC;
    16
}
/// $03485A: bcs.w $34874
pub fn i_03485a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3485E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x34874; return 10; }
    8
}
/// $03485E: move.b #$10, $70(a6)
pub fn i_03485e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34864;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034864: lea.l $2796c4.l, a0
pub fn i_034864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3486A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2796C4;
    20
}
/// $03486A: move.l -$4(a0), $74(a6)
pub fn i_03486a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34870;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034870: bra.w $34886
pub fn i_034870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34874;
    cpu.ir = 0x6000;
    cpu.pc = 0x34886;
    10
}
/// $034874: move.b #$10, $70(a6)
pub fn i_034874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3487A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03487A: lea.l $27991c.l, a0
pub fn i_03487a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34880;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27991C;
    20
}
/// $034880: move.l -$4(a0), $74(a6)
pub fn i_034880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34886;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034886: bset.b #$5, $69(a6)
pub fn i_034886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3488C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03488C: bra.w $348f0
pub fn i_03488c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34890;
    cpu.ir = 0x6000;
    cpu.pc = 0x348F0;
    10
}
/// $034890: move.w #$0, $72(a6)
pub fn i_034890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34896;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034896: jsr $2abcc.l
pub fn i_034896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3489C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3489C);
    cpu.pc = 0x2ABCC;
    16
}
/// $03489C: bcs.w $348c8
pub fn i_03489c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348A0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x348C8; return 10; }
    8
}
/// $0348A0: move.b #$10, $70(a6)
pub fn i_0348a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348A6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0348A6: lea.l $2796a4.l, a0
pub fn i_0348a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2796A4;
    20
}
/// $0348AC: move.l -$4(a0), $74(a6)
pub fn i_0348ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348B2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0348B2: move.b #$ff, $21(a6)
pub fn i_0348b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348B8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0348B8: lea.l $2796a4.l, a0
pub fn i_0348b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2796A4;
    20
}
/// $0348BE: jsr $28cd4.l
pub fn i_0348be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x348C4);
    cpu.pc = 0x28CD4;
    16
}
/// $0348C4: bra.w $348ec
pub fn i_0348c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x348EC;
    10
}
/// $0348C8: move.b #$10, $70(a6)
pub fn i_0348c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348CE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0348CE: lea.l $279926.l, a0
pub fn i_0348ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0348D4: move.l -$4(a0), $74(a6)
pub fn i_0348d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348DA;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0348DA: move.b #$ff, $21(a6)
pub fn i_0348da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348E0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0348E0: lea.l $279926.l, a0
pub fn i_0348e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348E6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0348E6: jsr $28cd4.l
pub fn i_0348e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x348EC);
    cpu.pc = 0x28CD4;
    16
}
/// $0348EC: bra.w $348f0
pub fn i_0348ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x348F0;
    10
}
/// $0348F0: btst.b #$0, $88(a6)
pub fn i_0348f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348F6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0348F6: beq.w $34900
pub fn i_0348f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348FA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x34900; return 10; }
    8
}
/// $0348FA: lea.l $33a5e(pc), a1
pub fn i_0348fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x348FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $0348FE: move.l a1, (a6)
pub fn i_0348fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34900;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034900: cmpi.b #$0, $85(a6)
pub fn i_034900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34906;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $034906: bne.w $3491a
pub fn i_034906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3490A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3491A; return 10; }
    8
}
/// $03490A: cmpi.b #$1, $71(a6)
pub fn i_03490a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34910;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $034910: bne.w $3491a
pub fn i_034910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34914;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3491A; return 10; }
    8
}
/// $034914: lea.l $33b9a(pc), a1
pub fn i_034914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34918;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33B9A;
    16
}
/// $034918: move.l a1, (a6)
pub fn i_034918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3491A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03491A: jsr $5e9b6.l
pub fn i_03491a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34920;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34920);
    cpu.pc = 0x5E9B6;
    16
}
/// $034920: andi.w #$7, d0
pub fn i_034920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34924;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $034924: moveq #$4, d0
pub fn i_034924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34926;
    cpu.ir = 0x7004;
    cpu.d[0] = 0x4;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $034926: cmp.w $82(a6), d0
pub fn i_034926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3492A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $03492A: bne.w $34934
pub fn i_03492a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3492E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34934; return 10; }
    8
}
/// $03492E: lea.l $33afc(pc), a1
pub fn i_03492e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34932;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33AFC;
    16
}
/// $034932: move.l a1, (a6)
pub fn i_034932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34934;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034934: cmpi.w #$0, $82(a6)
pub fn i_034934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3493A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03493A: bne.w $34944
pub fn i_03493a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3493E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34944; return 10; }
    8
}
/// $03493E: lea.l $33d64(pc), a1
pub fn i_03493e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34942;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $034942: move.l a1, (a6)
pub fn i_034942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34944;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034944: cmpi.b #$10, $70(a6)
pub fn i_034944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3494A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Byte, false);
    16
}
/// $03494A: bne.w $34a00
pub fn i_03494a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3494E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34A00; return 10; }
    8
}
/// $03494E: cmpi.b #$4, $78(a6)
pub fn i_03494e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34954;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $034954: bne.w $349ae
pub fn i_034954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34958;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x349AE; return 10; }
    8
}
/// $034958: cmpi.w #$1, $72(a6)
pub fn i_034958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3495E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03495E: beq.w $349aa
pub fn i_03495e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34962;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x349AA; return 10; }
    8
}
/// $034962: move.w #$1, $72(a6)
pub fn i_034962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34968;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034968: jsr $2abcc.l
pub fn i_034968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3496E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3496E);
    cpu.pc = 0x2ABCC;
    16
}
/// $03496E: bcs.w $34988
pub fn i_03496e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34972;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x34988; return 10; }
    8
}
/// $034972: move.b #$10, $70(a6)
pub fn i_034972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34978;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034978: lea.l $2796c4.l, a0
pub fn i_034978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3497E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2796C4;
    20
}
/// $03497E: move.l -$4(a0), $74(a6)
pub fn i_03497e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34984;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034984: bra.w $3499a
pub fn i_034984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34988;
    cpu.ir = 0x6000;
    cpu.pc = 0x3499A;
    10
}
/// $034988: move.b #$10, $70(a6)
pub fn i_034988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3498E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03498E: lea.l $27991c.l, a0
pub fn i_03498e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34994;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27991C;
    20
}
/// $034994: move.l -$4(a0), $74(a6)
pub fn i_034994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3499A;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03499A: cmpi.b #$14, $79(a6)
pub fn i_03499a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349A0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Byte, false);
    16
}
/// $0349A0: bne.w $349aa
pub fn i_0349a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x349AA; return 10; }
    8
}
/// $0349A4: bset.b #$5, $69(a6)
pub fn i_0349a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349AA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0349AA: bra.w $34a00
pub fn i_0349aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x34A00;
    10
}
/// $0349AE: cmpi.w #$0, $72(a6)
pub fn i_0349ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349B4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0349B4: beq.w $34a00
pub fn i_0349b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x34A00; return 10; }
    8
}
/// $0349B8: move.w #$0, $72(a6)
pub fn i_0349b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0349BE: jsr $2abcc.l
pub fn i_0349be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x349C4);
    cpu.pc = 0x2ABCC;
    16
}
/// $0349C4: bcs.w $349de
pub fn i_0349c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349C8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x349DE; return 10; }
    8
}
/// $0349C8: move.b #$10, $70(a6)
pub fn i_0349c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349CE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0349CE: lea.l $2796a4.l, a0
pub fn i_0349ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2796A4;
    20
}
/// $0349D4: move.l -$4(a0), $74(a6)
pub fn i_0349d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349DA;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0349DA: bra.w $349f0
pub fn i_0349da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x349F0;
    10
}
/// $0349DE: move.b #$10, $70(a6)
pub fn i_0349de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0349E4: lea.l $279926.l, a0
pub fn i_0349e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0349EA: move.l -$4(a0), $74(a6)
pub fn i_0349ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349F0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0349F0: cmpi.b #$41, $79(a6)
pub fn i_0349f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349F6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x41, Size::Byte, false);
    16
}
/// $0349F6: bne.w $34a00
pub fn i_0349f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x349FA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34A00; return 10; }
    8
}
/// $0349FA: bset.b #$5, $69(a6)
pub fn i_0349fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A00;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034A00: btst.b #$2, $8c(a6)
pub fn i_034a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A06;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $034A06: bne.w $34a5e
pub fn i_034a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A0A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34A5E; return 10; }
    8
}
/// $034A0A: btst.b #$0, $3a(a6)
pub fn i_034a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A10;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $034A10: bne.w $34a34
pub fn i_034a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A14;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34A34; return 10; }
    8
}
/// $034A14: jsr $32e3c(pc)
pub fn i_034a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34A18);
    cpu.pc = 0x32E3C;
    16
}
/// $034A18: bcc.w $34a22
pub fn i_034a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34A22; return 10; }
    8
}
/// $034A1C: lea.l $34b38(pc), a1
pub fn i_034a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34B38;
    16
}
/// $034A20: move.l a1, (a6)
pub fn i_034a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034A22: jsr $32e42(pc)
pub fn i_034a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34A26);
    cpu.pc = 0x32E42;
    16
}
/// $034A26: bcc.w $34a30
pub fn i_034a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34A30; return 10; }
    8
}
/// $034A2A: lea.l $35bf8(pc), a1
pub fn i_034a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35BF8;
    16
}
/// $034A2E: move.l a1, (a6)
pub fn i_034a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034A30: bra.w $34a50
pub fn i_034a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A34;
    cpu.ir = 0x6000;
    cpu.pc = 0x34A50;
    10
}
/// $034A34: jsr $32e3c(pc)
pub fn i_034a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A38;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34A38);
    cpu.pc = 0x32E3C;
    16
}
/// $034A38: bcc.w $34a42
pub fn i_034a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34A42; return 10; }
    8
}
/// $034A3C: lea.l $35aba(pc), a1
pub fn i_034a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35ABA;
    16
}
/// $034A40: move.l a1, (a6)
pub fn i_034a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034A42: jsr $32e42(pc)
pub fn i_034a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34A46);
    cpu.pc = 0x32E42;
    16
}
/// $034A46: bcc.w $34a50
pub fn i_034a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34A50; return 10; }
    8
}
/// $034A4A: lea.l $34d32(pc), a1
pub fn i_034a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34D32;
    16
}
/// $034A4E: move.l a1, (a6)
pub fn i_034a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A50;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034A50: jsr $32e90(pc)
pub fn i_034a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34A54);
    cpu.pc = 0x32E90;
    16
}
/// $034A54: bcc.w $34a5e
pub fn i_034a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34A5E; return 10; }
    8
}
/// $034A58: lea.l $37c74(pc), a1
pub fn i_034a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C74;
    16
}
/// $034A5C: move.l a1, (a6)
pub fn i_034a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034A5E: cmpi.b #$4, $71(a6)
pub fn i_034a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A64;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $034A64: bne.w $34a6c
pub fn i_034a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A68;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34A6C; return 10; }
    8
}
/// $034A68: bra.w $34a76
pub fn i_034a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x34A76;
    10
}
/// $034A6C: btst.b #$2, $8c(a6)
pub fn i_034a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A72;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $034A72: bne.w $34ada
pub fn i_034a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34ADA; return 10; }
    8
}
/// $034A76: jsr $330d0(pc)
pub fn i_034a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A7A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34A7A);
    cpu.pc = 0x330D0;
    16
}
/// $034A7A: bcc.w $34ada
pub fn i_034a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A7E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34ADA; return 10; }
    8
}
/// $034A7E: btst.b #$3, $8c(a6)
pub fn i_034a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A84;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $034A84: bne.w $34abe
pub fn i_034a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A88;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34ABE; return 10; }
    8
}
/// $034A88: cmpi.b #$ff, d1
pub fn i_034a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A8C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $034A8C: bne.w $34a9a
pub fn i_034a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A90;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34A9A; return 10; }
    8
}
/// $034A90: lea.l $35d34(pc), a1
pub fn i_034a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35D34;
    16
}
/// $034A94: move.l a1, (a6)
pub fn i_034a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034A96: bra.w $34ada
pub fn i_034a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x34ADA;
    10
}
/// $034A9A: cmpi.b #$3, d1
pub fn i_034a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34A9E;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $034A9E: bne.w $34aac
pub fn i_034a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AA2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34AAC; return 10; }
    8
}
/// $034AA2: lea.l $360bc(pc), a1
pub fn i_034aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x360BC;
    16
}
/// $034AA6: move.l a1, (a6)
pub fn i_034aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034AA8: bra.w $34ada
pub fn i_034aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AAC;
    cpu.ir = 0x6000;
    cpu.pc = 0x34ADA;
    10
}
/// $034AAC: cmpi.b #$4, d1
pub fn i_034aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AB0;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $034AB0: bne.w $34abe
pub fn i_034ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AB4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34ABE; return 10; }
    8
}
/// $034AB4: lea.l $3873c(pc), a1
pub fn i_034ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $034AB8: move.l a1, (a6)
pub fn i_034ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34ABA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034ABA: bra.w $34ada
pub fn i_034aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34ABE;
    cpu.ir = 0x6000;
    cpu.pc = 0x34ADA;
    10
}
/// $034ABE: cmpi.b #$1, d1
pub fn i_034abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AC2;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $034AC2: bne.w $34ad0
pub fn i_034ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34AD0; return 10; }
    8
}
/// $034AC6: lea.l $3437e(pc), a1
pub fn i_034ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34ACA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3437E;
    16
}
/// $034ACA: move.l a1, (a6)
pub fn i_034aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34ACC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034ACC: bra.w $34ada
pub fn i_034acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x34ADA;
    10
}
/// $034AD0: lea.l $342c4(pc), a1
pub fn i_034ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x342C4;
    16
}
/// $034AD4: move.l a1, (a6)
pub fn i_034ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AD6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034AD6: bra.w $34ada
pub fn i_034ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34ADA;
    cpu.ir = 0x6000;
    cpu.pc = 0x34ADA;
    10
}
/// $034ADA: jsr $27eba.l
pub fn i_034ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34AE0);
    cpu.pc = 0x27EBA;
    16
}
/// $034AE0: bcc.w $34aea
pub fn i_034ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AE4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34AEA; return 10; }
    8
}
/// $034AE4: lea.l $36d64(pc), a1
pub fn i_034ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36D64;
    16
}
/// $034AE8: move.l a1, (a6)
pub fn i_034ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034AEA: jsr $33034(pc)
pub fn i_034aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34AEE);
    cpu.pc = 0x33034;
    16
}
/// $034AEE: bcc.w $34af8
pub fn i_034aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34AF8; return 10; }
    8
}
/// $034AF2: lea.l $36914(pc), a1
pub fn i_034af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $034AF6: move.l a1, (a6)
pub fn i_034af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034AF8: jsr $33522(pc)
pub fn i_034af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34AFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34AFC);
    cpu.pc = 0x33522;
    16
}
/// $034AFC: cmpi.b #$3, $106ece.l
pub fn i_034afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B04;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $034B04: beq.w $34b1c
pub fn i_034b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x34B1C; return 10; }
    8
}
/// $034B08: movea.l #$ffffffff, a0
pub fn i_034b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B0E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $034B0E: lea.l $324c6(pc), a0
pub fn i_034b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B12;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $034B12: jsr $5dd56.l
pub fn i_034b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34B18);
    cpu.pc = 0x5DD56;
    16
}
/// $034B18: bra.w $34b2c
pub fn i_034b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x34B2C;
    10
}
/// $034B1C: movea.l #$ffffffff, a0
pub fn i_034b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B22;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $034B22: lea.l $324bc(pc), a0
pub fn i_034b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B26;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $034B26: jsr $5dd56.l
pub fn i_034b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34B2C);
    cpu.pc = 0x5DD56;
    16
}
/// $034B2C: bcc.w $34b36
pub fn i_034b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B30;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34B36; return 10; }
    8
}
/// $034B30: lea.l $37b8e(pc), a1
pub fn i_034b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $034B34: move.l a1, (a6)
pub fn i_034b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034B36: rts
pub fn i_034b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_WalkRight_034b38  ($034B38, 506 B, player/states_0342c4.s)
/// $034B38: move.w #$0, $7c(a6)
pub fn i_034b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B3E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034B3E: move.w #$ffff, $7e(a6)
pub fn i_034b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B44;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034B44: cmpi.b #$13, $70(a6)
pub fn i_034b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B4A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x13, Size::Byte, false);
    16
}
/// $034B4A: bcs.w $34b6e
pub fn i_034b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B4E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x34B6E; return 10; }
    8
}
/// $034B4E: cmpi.b #$24, $70(a6)
pub fn i_034b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B54;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $034B54: bcc.w $34b6e
pub fn i_034b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34B6E; return 10; }
    8
}
/// $034B58: lea.l $279882.l, a0
pub fn i_034b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279882;
    20
}
/// $034B5E: jsr $28cd4.l
pub fn i_034b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34B64);
    cpu.pc = 0x28CD4;
    16
}
/// $034B64: bclr.b #$5, $69(a6)
pub fn i_034b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B6A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034B6A: bra.w $34ba4
pub fn i_034b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x34BA4;
    10
}
/// $034B6E: bclr.b #$2, $8c(a6)
pub fn i_034b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B74;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034B74: bclr.b #$1, $8c(a6)
pub fn i_034b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B7A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034B7A: bclr.b #$3, $8c(a6)
pub fn i_034b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B80;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034B80: move.b #$11, $70(a6)
pub fn i_034b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B86;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034B86: lea.l $279882.l, a0
pub fn i_034b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279882;
    20
}
/// $034B8C: move.l -$4(a0), $74(a6)
pub fn i_034b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B92;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034B92: move.b #$ff, $21(a6)
pub fn i_034b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B98;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034B98: lea.l $279882.l, a0
pub fn i_034b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34B9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279882;
    20
}
/// $034B9E: jsr $28cd4.l
pub fn i_034b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34BA4);
    cpu.pc = 0x28CD4;
    16
}
/// $034BA4: move.w #$300, $28(a6)
pub fn i_034ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BAA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034BAA: clr.w $2c(a6)
pub fn i_034baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BAE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $034BAE: move.l #$32500, $60(a6)
pub fn i_034bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BB6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $034BB6: lea.l $326e0(pc), a0
pub fn i_034bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BBA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $034BBA: move.l a0, $48(a6)
pub fn i_034bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BBE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $034BBE: bclr.b #$0, $3a(a6)
pub fn i_034bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BC4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034BC4: lea.l $34bca(pc), a1
pub fn i_034bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34BCA;
    16
}
/// $034BC8: move.l a1, (a6)
pub fn i_034bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034BCA: jsr $32ff2(pc)
pub fn i_034bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BCE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34BCE);
    cpu.pc = 0x32FF2;
    16
}
/// $034BCE: jsr $32d32(pc)
pub fn i_034bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34BD2);
    cpu.pc = 0x32D32;
    16
}
/// $034BD2: bcc.w $34bdc
pub fn i_034bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34BDC; return 10; }
    8
}
/// $034BD6: lea.l $345b8(pc), a1
pub fn i_034bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345B8;
    16
}
/// $034BDA: move.l a1, (a6)
pub fn i_034bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034BDC: move.w #$300, $28(a6)
pub fn i_034bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BE2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034BE2: jsr $27a92.l
pub fn i_034be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34BE8);
    cpu.pc = 0x27A92;
    16
}
/// $034BE8: jsr $32aa8(pc)
pub fn i_034be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34BEC);
    cpu.pc = 0x32AA8;
    16
}
/// $034BEC: bcc.w $34bf6
pub fn i_034bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BF0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34BF6; return 10; }
    8
}
/// $034BF0: lea.l $351d8(pc), a1
pub fn i_034bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x351D8;
    16
}
/// $034BF4: move.l a1, (a6)
pub fn i_034bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034BF6: cmpi.b #$4, $78(a6)
pub fn i_034bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34BFC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $034BFC: bne.w $34c28
pub fn i_034bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C00;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34C28; return 10; }
    8
}
/// $034C00: cmpi.l #$3a058, $74(a6)
pub fn i_034c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C08;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x3A058, Size::Long, false);
    20
}
/// $034C08: bne.w $34c24
pub fn i_034c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C0C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34C24; return 10; }
    8
}
/// $034C0C: move.w #$1, $72(a6)
pub fn i_034c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C12;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034C12: move.b #$11, $70(a6)
pub fn i_034c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C18;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034C18: lea.l $279878.l, a0
pub fn i_034c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C1E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279878;
    20
}
/// $034C1E: move.l -$4(a0), $74(a6)
pub fn i_034c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C24;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034C24: bra.w $34c4c
pub fn i_034c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C28;
    cpu.ir = 0x6000;
    cpu.pc = 0x34C4C;
    10
}
/// $034C28: cmpi.l #$3a058, $74(a6)
pub fn i_034c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C30;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x3A058, Size::Long, false);
    20
}
/// $034C30: bne.w $34c4c
pub fn i_034c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C34;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34C4C; return 10; }
    8
}
/// $034C34: move.w #$0, $72(a6)
pub fn i_034c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C3A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034C3A: move.b #$11, $70(a6)
pub fn i_034c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C40;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034C40: lea.l $279882.l, a0
pub fn i_034c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279882;
    20
}
/// $034C46: move.l -$4(a0), $74(a6)
pub fn i_034c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C4C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034C4C: btst.b #$2, $8c(a6)
pub fn i_034c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C52;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $034C52: bne.w $34ccc
pub fn i_034c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34CCC; return 10; }
    8
}
/// $034C56: jsr $32e3c(pc)
pub fn i_034c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34C5A);
    cpu.pc = 0x32E3C;
    16
}
/// $034C5A: bcs.w $34c64
pub fn i_034c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C5E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x34C64; return 10; }
    8
}
/// $034C5E: lea.l $345fa(pc), a1
pub fn i_034c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345FA;
    16
}
/// $034C62: move.l a1, (a6)
pub fn i_034c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034C64: jsr $32e42(pc)
pub fn i_034c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34C68);
    cpu.pc = 0x32E42;
    16
}
/// $034C68: bcc.w $34c72
pub fn i_034c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C6C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34C72; return 10; }
    8
}
/// $034C6C: lea.l $35bf8(pc), a1
pub fn i_034c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35BF8;
    16
}
/// $034C70: move.l a1, (a6)
pub fn i_034c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034C72: jsr $330d0(pc)
pub fn i_034c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34C76);
    cpu.pc = 0x330D0;
    16
}
/// $034C76: bcc.w $34ccc
pub fn i_034c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34CCC; return 10; }
    8
}
/// $034C7A: cmpi.b #$ff, d1
pub fn i_034c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C7E;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $034C7E: bne.w $34c8c
pub fn i_034c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C82;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34C8C; return 10; }
    8
}
/// $034C82: lea.l $35d34(pc), a1
pub fn i_034c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35D34;
    16
}
/// $034C86: move.l a1, (a6)
pub fn i_034c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034C88: bra.w $34ccc
pub fn i_034c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C8C;
    cpu.ir = 0x6000;
    cpu.pc = 0x34CCC;
    10
}
/// $034C8C: cmpi.b #$3, d1
pub fn i_034c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C90;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $034C90: bne.w $34c9e
pub fn i_034c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C94;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34C9E; return 10; }
    8
}
/// $034C94: lea.l $360bc(pc), a1
pub fn i_034c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x360BC;
    16
}
/// $034C98: move.l a1, (a6)
pub fn i_034c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034C9A: bra.w $34ccc
pub fn i_034c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34C9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x34CCC;
    10
}
/// $034C9E: cmpi.b #$4, d1
pub fn i_034c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CA2;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $034CA2: bne.w $34cb0
pub fn i_034ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34CB0; return 10; }
    8
}
/// $034CA6: lea.l $3873c(pc), a1
pub fn i_034ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CAA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $034CAA: move.l a1, (a6)
pub fn i_034caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CAC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034CAC: bra.w $34ccc
pub fn i_034cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CB0;
    cpu.ir = 0x6000;
    cpu.pc = 0x34CCC;
    10
}
/// $034CB0: cmpi.b #$1, d1
pub fn i_034cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CB4;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $034CB4: bne.w $34cc2
pub fn i_034cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CB8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34CC2; return 10; }
    8
}
/// $034CB8: lea.l $3503e(pc), a1
pub fn i_034cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3503E;
    16
}
/// $034CBC: move.l a1, (a6)
pub fn i_034cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034CBE: bra.w $34ccc
pub fn i_034cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x34CCC;
    10
}
/// $034CC2: lea.l $34eca(pc), a1
pub fn i_034cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34ECA;
    16
}
/// $034CC6: move.l a1, (a6)
pub fn i_034cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034CC8: bra.w $34ccc
pub fn i_034cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CCC;
    cpu.ir = 0x6000;
    cpu.pc = 0x34CCC;
    10
}
/// $034CCC: jsr $32e90(pc)
pub fn i_034ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34CD0);
    cpu.pc = 0x32E90;
    16
}
/// $034CD0: bcc.w $34cda
pub fn i_034cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CD4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34CDA; return 10; }
    8
}
/// $034CD4: lea.l $37c74(pc), a1
pub fn i_034cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C74;
    16
}
/// $034CD8: move.l a1, (a6)
pub fn i_034cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034CDA: cmpi.b #$0, $85(a6)
pub fn i_034cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CE0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $034CE0: bne.w $34cf4
pub fn i_034ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34CF4; return 10; }
    8
}
/// $034CE4: cmpi.b #$1, $71(a6)
pub fn i_034ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CEA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $034CEA: bne.w $34cf4
pub fn i_034cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CEE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34CF4; return 10; }
    8
}
/// $034CEE: lea.l $33b9a(pc), a1
pub fn i_034cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33B9A;
    16
}
/// $034CF2: move.l a1, (a6)
pub fn i_034cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034CF4: cmpi.w #$0, $82(a6)
pub fn i_034cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CFA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $034CFA: bne.w $34d0e
pub fn i_034cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34CFE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34D0E; return 10; }
    8
}
/// $034CFE: cmpi.b #$0, $71(a6)
pub fn i_034cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D04;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $034D04: beq.w $34d0e
pub fn i_034d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x34D0E; return 10; }
    8
}
/// $034D08: lea.l $33d64(pc), a1
pub fn i_034d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $034D0C: move.l a1, (a6)
pub fn i_034d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034D0E: jsr $27eba.l
pub fn i_034d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34D14);
    cpu.pc = 0x27EBA;
    16
}
/// $034D14: bcc.w $34d1e
pub fn i_034d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D18;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34D1E; return 10; }
    8
}
/// $034D18: lea.l $36d64(pc), a1
pub fn i_034d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36D64;
    16
}
/// $034D1C: move.l a1, (a6)
pub fn i_034d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034D1E: jsr $33034(pc)
pub fn i_034d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34D22);
    cpu.pc = 0x33034;
    16
}
/// $034D22: bcc.w $34d2c
pub fn i_034d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34D2C; return 10; }
    8
}
/// $034D26: lea.l $36914(pc), a1
pub fn i_034d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $034D2A: move.l a1, (a6)
pub fn i_034d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034D2C: jsr $33522(pc)
pub fn i_034d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34D30);
    cpu.pc = 0x33522;
    16
}
/// $034D30: rts
pub fn i_034d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_WalkLeft_034d32  ($034D32, 408 B, player/states_0342c4.s)
/// $034D32: move.w #$0, $7c(a6)
pub fn i_034d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D38;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034D38: move.w #$ffff, $7e(a6)
pub fn i_034d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D3E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034D3E: cmpi.b #$13, $70(a6)
pub fn i_034d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D44;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x13, Size::Byte, false);
    16
}
/// $034D44: bcs.w $34d68
pub fn i_034d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D48;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x34D68; return 10; }
    8
}
/// $034D48: cmpi.b #$24, $70(a6)
pub fn i_034d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D4E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $034D4E: bcc.w $34d68
pub fn i_034d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D52;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34D68; return 10; }
    8
}
/// $034D52: lea.l $279882.l, a0
pub fn i_034d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279882;
    20
}
/// $034D58: jsr $28cd4.l
pub fn i_034d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34D5E);
    cpu.pc = 0x28CD4;
    16
}
/// $034D5E: bclr.b #$5, $69(a6)
pub fn i_034d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D64;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034D64: bra.w $34d9e
pub fn i_034d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D68;
    cpu.ir = 0x6000;
    cpu.pc = 0x34D9E;
    10
}
/// $034D68: bclr.b #$2, $8c(a6)
pub fn i_034d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D6E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034D6E: bclr.b #$1, $8c(a6)
pub fn i_034d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D74;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034D74: bclr.b #$3, $8c(a6)
pub fn i_034d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D7A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034D7A: move.b #$11, $70(a6)
pub fn i_034d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D80;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034D80: lea.l $279882.l, a0
pub fn i_034d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D86;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279882;
    20
}
/// $034D86: move.l -$4(a0), $74(a6)
pub fn i_034d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D8C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034D8C: move.b #$ff, $21(a6)
pub fn i_034d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D92;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034D92: lea.l $279882.l, a0
pub fn i_034d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D98;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279882;
    20
}
/// $034D98: jsr $28cd4.l
pub fn i_034d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34D9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34D9E);
    cpu.pc = 0x28CD4;
    16
}
/// $034D9E: move.w #$fd00, $28(a6)
pub fn i_034d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DA4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034DA4: clr.w $2c(a6)
pub fn i_034da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DA8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $034DA8: move.l #$32500, $60(a6)
pub fn i_034da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DB0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $034DB0: lea.l $326e0(pc), a0
pub fn i_034db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DB4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $034DB4: move.l a0, $48(a6)
pub fn i_034db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DB8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $034DB8: bset.b #$0, $3a(a6)
pub fn i_034db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DBE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034DBE: lea.l $34dc4(pc), a1
pub fn i_034dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34DC4;
    16
}
/// $034DC2: move.l a1, (a6)
pub fn i_034dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034DC4: jsr $32ff2(pc)
pub fn i_034dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34DC8);
    cpu.pc = 0x32FF2;
    16
}
/// $034DC8: jsr $32d32(pc)
pub fn i_034dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DCC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34DCC);
    cpu.pc = 0x32D32;
    16
}
/// $034DCC: bcc.w $34dd6
pub fn i_034dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DD0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34DD6; return 10; }
    8
}
/// $034DD0: lea.l $345b8(pc), a1
pub fn i_034dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345B8;
    16
}
/// $034DD4: move.l a1, (a6)
pub fn i_034dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DD6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034DD6: move.w #$fd00, $28(a6)
pub fn i_034dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DDC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034DDC: jsr $27a92.l
pub fn i_034ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34DE2);
    cpu.pc = 0x27A92;
    16
}
/// $034DE2: jsr $32aa8(pc)
pub fn i_034de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34DE6);
    cpu.pc = 0x32AA8;
    16
}
/// $034DE6: bcc.w $34df0
pub fn i_034de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34DF0; return 10; }
    8
}
/// $034DEA: lea.l $354f2(pc), a1
pub fn i_034dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x354F2;
    16
}
/// $034DEE: move.l a1, (a6)
pub fn i_034dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034DF0: cmpi.b #$4, $78(a6)
pub fn i_034df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DF6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $034DF6: bne.w $34e22
pub fn i_034df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34DFA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34E22; return 10; }
    8
}
/// $034DFA: cmpi.l #$3a058, $74(a6)
pub fn i_034dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E02;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x3A058, Size::Long, false);
    20
}
/// $034E02: bne.w $34e1e
pub fn i_034e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34E1E; return 10; }
    8
}
/// $034E06: move.w #$1, $72(a6)
pub fn i_034e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E0C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034E0C: move.b #$11, $70(a6)
pub fn i_034e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034E12: lea.l $279878.l, a0
pub fn i_034e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E18;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279878;
    20
}
/// $034E18: move.l -$4(a0), $74(a6)
pub fn i_034e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E1E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034E1E: bra.w $34e46
pub fn i_034e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E22;
    cpu.ir = 0x6000;
    cpu.pc = 0x34E46;
    10
}
/// $034E22: cmpi.l #$3a058, $74(a6)
pub fn i_034e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E2A;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x3A058, Size::Long, false);
    20
}
/// $034E2A: bne.w $34e46
pub fn i_034e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34E46; return 10; }
    8
}
/// $034E2E: move.w #$0, $72(a6)
pub fn i_034e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E34;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034E34: move.b #$11, $70(a6)
pub fn i_034e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E3A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034E3A: lea.l $279882.l, a0
pub fn i_034e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E40;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279882;
    20
}
/// $034E40: move.l -$4(a0), $74(a6)
pub fn i_034e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E46;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034E46: btst.b #$2, $8c(a6)
pub fn i_034e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E4C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $034E4C: bne.w $34ec6
pub fn i_034e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E50;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34EC6; return 10; }
    8
}
/// $034E50: jsr $32e42(pc)
pub fn i_034e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34E54);
    cpu.pc = 0x32E42;
    16
}
/// $034E54: bcs.w $34e5e
pub fn i_034e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E58;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x34E5E; return 10; }
    8
}
/// $034E58: lea.l $345fa(pc), a1
pub fn i_034e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345FA;
    16
}
/// $034E5C: move.l a1, (a6)
pub fn i_034e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034E5E: jsr $32e3c(pc)
pub fn i_034e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34E62);
    cpu.pc = 0x32E3C;
    16
}
/// $034E62: bcc.w $34e6c
pub fn i_034e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34E6C; return 10; }
    8
}
/// $034E66: lea.l $35aba(pc), a1
pub fn i_034e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35ABA;
    16
}
/// $034E6A: move.l a1, (a6)
pub fn i_034e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034E6C: jsr $330d0(pc)
pub fn i_034e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34E70);
    cpu.pc = 0x330D0;
    16
}
/// $034E70: bcc.w $34ec6
pub fn i_034e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E74;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34EC6; return 10; }
    8
}
/// $034E74: cmpi.b #$ff, d1
pub fn i_034e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E78;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $034E78: bne.w $34e86
pub fn i_034e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E7C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34E86; return 10; }
    8
}
/// $034E7C: lea.l $35d34(pc), a1
pub fn i_034e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35D34;
    16
}
/// $034E80: move.l a1, (a6)
pub fn i_034e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034E82: bra.w $34ec6
pub fn i_034e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E86;
    cpu.ir = 0x6000;
    cpu.pc = 0x34EC6;
    10
}
/// $034E86: cmpi.b #$3, d1
pub fn i_034e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E8A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $034E8A: bne.w $34e98
pub fn i_034e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E8E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34E98; return 10; }
    8
}
/// $034E8E: lea.l $360bc(pc), a1
pub fn i_034e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x360BC;
    16
}
/// $034E92: move.l a1, (a6)
pub fn i_034e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034E94: bra.w $34ec6
pub fn i_034e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E98;
    cpu.ir = 0x6000;
    cpu.pc = 0x34EC6;
    10
}
/// $034E98: cmpi.b #$4, d1
pub fn i_034e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34E9C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $034E9C: bne.w $34eaa
pub fn i_034e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EA0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34EAA; return 10; }
    8
}
/// $034EA0: lea.l $3873c(pc), a1
pub fn i_034ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $034EA4: move.l a1, (a6)
pub fn i_034ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034EA6: bra.w $3548c
pub fn i_034ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3548C;
    10
}
/// $034EAA: cmpi.b #$1, d1
pub fn i_034eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EAE;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $034EAE: bne.w $34ebc
pub fn i_034eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EB2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34EBC; return 10; }
    8
}
/// $034EB2: lea.l $351c4(pc), a1
pub fn i_034eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x351C4;
    16
}
/// $034EB6: move.l a1, (a6)
pub fn i_034eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034EB8: bra.w $34ec6
pub fn i_034eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x34EC6;
    10
}
/// $034EBC: lea.l $351b0(pc), a1
pub fn i_034ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x351B0;
    16
}
/// $034EC0: move.l a1, (a6)
pub fn i_034ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $034EC2: bra.w $34ec6
pub fn i_034ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EC6;
    cpu.ir = 0x6000;
    cpu.pc = 0x34EC6;
    10
}
/// $034EC6: bra.w $34ccc
pub fn i_034ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34ECA;
    cpu.ir = 0x6000;
    cpu.pc = 0x34CCC;
    10
}
// ===== Player_WalkRight_Shoot_034eca  ($034ECA, 20 B, player/states_0342c4.s)
/// $034ECA: move.w #$0, $7c(a6)
pub fn i_034eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34ED0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034ED0: move.w #$ffff, $7e(a6)
pub fn i_034ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34ED6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034ED6: jsr $34ede(pc)
pub fn i_034ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EDA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x34EDA);
    cpu.pc = 0x34EDE;
    16
}
/// $034EDA: bra.w $34ba4
pub fn i_034eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EDE;
    cpu.ir = 0x6000;
    cpu.pc = 0x34BA4;
    10
}
// ===== Player_WalkShootPose_034ede  ($034EDE, 352 B, player/states_0342c4.s)
/// $034EDE: bset.b #$1, $8c(a6)
pub fn i_034ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EE4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034EE4: cmpi.w #$1, $72(a6)
pub fn i_034ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EEA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $034EEA: bne.w $34f3c
pub fn i_034eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EEE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x34F3C; return 10; }
    8
}
/// $034EEE: cmpi.b #$24, $70(a6)
pub fn i_034eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EF4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $034EF4: bcc.w $34f14
pub fn i_034ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EF8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34F14; return 10; }
    8
}
/// $034EF8: bset.b #$5, $69(a6)
pub fn i_034ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34EFE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034EFE: move.b #$22, $70(a6)
pub fn i_034efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F04;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034F04: lea.l $2798f4.l, a0
pub fn i_034f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F0A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798F4;
    20
}
/// $034F0A: move.l -$4(a0), $74(a6)
pub fn i_034f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F10;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034F10: bra.w $34f38
pub fn i_034f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F14;
    cpu.ir = 0x6000;
    cpu.pc = 0x34F38;
    10
}
/// $034F14: move.b #$22, $70(a6)
pub fn i_034f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F1A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034F1A: lea.l $2798f4.l, a0
pub fn i_034f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F20;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798F4;
    20
}
/// $034F20: move.l -$4(a0), $74(a6)
pub fn i_034f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F26;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034F26: move.b #$ff, $21(a6)
pub fn i_034f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F2C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034F2C: lea.l $2798f4.l, a0
pub fn i_034f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798F4;
    20
}
/// $034F32: jsr $28cd4.l
pub fn i_034f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34F38);
    cpu.pc = 0x28CD4;
    16
}
/// $034F38: bra.w $34f86
pub fn i_034f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x34F86;
    10
}
/// $034F3C: cmpi.b #$24, $70(a6)
pub fn i_034f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F42;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $034F42: bcc.w $34f62
pub fn i_034f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F46;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x34F62; return 10; }
    8
}
/// $034F46: bset.b #$5, $69(a6)
pub fn i_034f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F4C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $034F4C: move.b #$22, $70(a6)
pub fn i_034f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034F52: lea.l $2798e0.l, a0
pub fn i_034f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798E0;
    20
}
/// $034F58: move.l -$4(a0), $74(a6)
pub fn i_034f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F5E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034F5E: bra.w $34f86
pub fn i_034f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F62;
    cpu.ir = 0x6000;
    cpu.pc = 0x34F86;
    10
}
/// $034F62: move.b #$22, $70(a6)
pub fn i_034f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F68;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034F68: lea.l $2798e0.l, a0
pub fn i_034f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798E0;
    20
}
/// $034F6E: move.l -$4(a0), $74(a6)
pub fn i_034f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F74;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $034F74: move.b #$ff, $21(a6)
pub fn i_034f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F7A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $034F7A: lea.l $2798e0.l, a0
pub fn i_034f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F80;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798E0;
    20
}
/// $034F80: jsr $28cd4.l
pub fn i_034f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x34F86);
    cpu.pc = 0x28CD4;
    16
}
/// $034F86: move.w #$0, $72(a6)
pub fn i_034f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F8C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $034F8C: rts
pub fn i_034f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x34F8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_WalkRight_ShootUp_03503e  ($03503E, 20 B, player/states_0342c4.s)
/// $03503E: move.w #$0, $7c(a6)
pub fn i_03503e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35044;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035044: move.w #$ffff, $7e(a6)
pub fn i_035044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3504A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03504A: jsr $35052(pc)
pub fn i_03504a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3504E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3504E);
    cpu.pc = 0x35052;
    16
}
/// $03504E: bra.w $34ba4
pub fn i_03504e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35052;
    cpu.ir = 0x6000;
    cpu.pc = 0x34BA4;
    10
}
// ===== Player_WalkShootUpPose_035052  ($035052, 350 B, player/states_0342c4.s)
/// $035052: bset.b #$1, $8c(a6)
pub fn i_035052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35058;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035058: cmpi.w #$0, $72(a6)
pub fn i_035058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3505E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03505E: bne.w $350b0
pub fn i_03505e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35062;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x350B0; return 10; }
    8
}
/// $035062: cmpi.b #$24, $70(a6)
pub fn i_035062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35068;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $035068: bcc.w $35088
pub fn i_035068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3506C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35088; return 10; }
    8
}
/// $03506C: bset.b #$5, $69(a6)
pub fn i_03506c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35072;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035072: move.b #$22, $70(a6)
pub fn i_035072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35078;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035078: lea.l $2798fe.l, a0
pub fn i_035078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3507E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798FE;
    20
}
/// $03507E: move.l -$4(a0), $74(a6)
pub fn i_03507e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35084;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035084: bra.w $350ac
pub fn i_035084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35088;
    cpu.ir = 0x6000;
    cpu.pc = 0x350AC;
    10
}
/// $035088: move.b #$22, $70(a6)
pub fn i_035088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3508E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03508E: lea.l $2798fe.l, a0
pub fn i_03508e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35094;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798FE;
    20
}
/// $035094: move.l -$4(a0), $74(a6)
pub fn i_035094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3509A;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03509A: move.b #$ff, $21(a6)
pub fn i_03509a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350A0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0350A0: lea.l $2798fe.l, a0
pub fn i_0350a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798FE;
    20
}
/// $0350A6: jsr $28cd4.l
pub fn i_0350a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x350AC);
    cpu.pc = 0x28CD4;
    16
}
/// $0350AC: bra.w $350fa
pub fn i_0350ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x350FA;
    10
}
/// $0350B0: cmpi.b #$24, $70(a6)
pub fn i_0350b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350B6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $0350B6: bcc.w $350d6
pub fn i_0350b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x350D6; return 10; }
    8
}
/// $0350BA: bset.b #$5, $69(a6)
pub fn i_0350ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350C0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0350C0: move.b #$22, $70(a6)
pub fn i_0350c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350C6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0350C6: lea.l $2798ea.l, a0
pub fn i_0350c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798EA;
    20
}
/// $0350CC: move.l -$4(a0), $74(a6)
pub fn i_0350cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350D2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0350D2: bra.w $350fa
pub fn i_0350d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x350FA;
    10
}
/// $0350D6: move.b #$22, $70(a6)
pub fn i_0350d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350DC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0350DC: lea.l $2798ea.l, a0
pub fn i_0350dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350E2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798EA;
    20
}
/// $0350E2: move.l -$4(a0), $74(a6)
pub fn i_0350e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350E8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0350E8: move.b #$ff, $21(a6)
pub fn i_0350e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0350EE: lea.l $2798ea.l, a0
pub fn i_0350ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798EA;
    20
}
/// $0350F4: jsr $28cd4.l
pub fn i_0350f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x350FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x350FA);
    cpu.pc = 0x28CD4;
    16
}
/// $0350FA: move.w #$1, $72(a6)
pub fn i_0350fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35100;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035100: rts
pub fn i_035100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35102;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_WalkLeft_Shoot_0351b0  ($0351B0, 20 B, player/states_0342c4.s)
/// $0351B0: move.w #$0, $7c(a6)
pub fn i_0351b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0351B6: move.w #$ffff, $7e(a6)
pub fn i_0351b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0351BC: jsr $34ede(pc)
pub fn i_0351bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x351C0);
    cpu.pc = 0x34EDE;
    16
}
/// $0351C0: bra.w $34d9e
pub fn i_0351c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x34D9E;
    10
}
// ===== Player_WalkLeft_ShootUp_0351c4  ($0351C4, 20 B, player/states_0342c4.s)
/// $0351C4: move.w #$0, $7c(a6)
pub fn i_0351c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351CA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0351CA: move.w #$ffff, $7e(a6)
pub fn i_0351ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0351D0: jsr $35052(pc)
pub fn i_0351d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x351D4);
    cpu.pc = 0x35052;
    16
}
/// $0351D4: bra.w $34d9e
pub fn i_0351d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x34D9E;
    10
}
// ===== Player_WalkLoopRight_0351d8  ($0351D8, 794 B, player/states_0342c4.s)
/// $0351D8: cmpi.b #$13, $70(a6)
pub fn i_0351d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351DE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x13, Size::Byte, false);
    16
}
/// $0351DE: bcs.w $3520e
pub fn i_0351de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351E2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3520E; return 10; }
    8
}
/// $0351E2: cmpi.b #$24, $70(a6)
pub fn i_0351e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351E8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $0351E8: bcc.w $3520e
pub fn i_0351e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3520E; return 10; }
    8
}
/// $0351EC: lea.l $279926.l, a0
pub fn i_0351ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0351F2: jsr $28cd4.l
pub fn i_0351f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x351F8);
    cpu.pc = 0x28CD4;
    16
}
/// $0351F8: move.w #$0, $7c(a6)
pub fn i_0351f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x351FE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0351FE: move.w #$3, $7e(a6)
pub fn i_0351fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35204;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035204: bclr.b #$5, $69(a6)
pub fn i_035204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3520A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03520A: bra.w $35250
pub fn i_03520a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3520E;
    cpu.ir = 0x6000;
    cpu.pc = 0x35250;
    10
}
/// $03520E: move.w #$0, $7c(a6)
pub fn i_03520e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35214;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035214: move.w #$ffff, $7e(a6)
pub fn i_035214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3521A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03521A: bclr.b #$2, $8c(a6)
pub fn i_03521a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35220;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035220: bclr.b #$1, $8c(a6)
pub fn i_035220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35226;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035226: bclr.b #$3, $8c(a6)
pub fn i_035226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3522C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03522C: move.b #$12, $70(a6)
pub fn i_03522c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35232;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035232: lea.l $279926.l, a0
pub fn i_035232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35238;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $035238: move.l -$4(a0), $74(a6)
pub fn i_035238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3523E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03523E: move.b #$ff, $21(a6)
pub fn i_03523e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35244;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035244: lea.l $279926.l, a0
pub fn i_035244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3524A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $03524A: jsr $28cd4.l
pub fn i_03524a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35250;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35250);
    cpu.pc = 0x28CD4;
    16
}
/// $035250: move.w #$300, $28(a6)
pub fn i_035250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35256;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035256: clr.w $2c(a6)
pub fn i_035256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3525A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03525A: move.l #$32500, $60(a6)
pub fn i_03525a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35262;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035262: lea.l $326e0(pc), a0
pub fn i_035262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35266;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $035266: move.l a0, $48(a6)
pub fn i_035266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3526A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03526A: bclr.b #$0, $3a(a6)
pub fn i_03526a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35270;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035270: lea.l $35276(pc), a1
pub fn i_035270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35274;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35276;
    16
}
/// $035274: move.l a1, (a6)
pub fn i_035274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35276;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035276: jsr $32ff2(pc)
pub fn i_035276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3527A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3527A);
    cpu.pc = 0x32FF2;
    16
}
/// $03527A: jsr $32d32(pc)
pub fn i_03527a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3527E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3527E);
    cpu.pc = 0x32D32;
    16
}
/// $03527E: bcc.w $35288
pub fn i_03527e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35282;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35288; return 10; }
    8
}
/// $035282: lea.l $345b8(pc), a1
pub fn i_035282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35286;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345B8;
    16
}
/// $035286: move.l a1, (a6)
pub fn i_035286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35288;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035288: move.w #$300, $28(a6)
pub fn i_035288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3528E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03528E: jsr $27a92.l
pub fn i_03528e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35294;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35294);
    cpu.pc = 0x27A92;
    16
}
/// $035294: jsr $32aa8(pc)
pub fn i_035294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35298;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35298);
    cpu.pc = 0x32AA8;
    16
}
/// $035298: bcc.w $35308
pub fn i_035298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3529C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35308; return 10; }
    8
}
/// $03529C: move.w #$0, $7c(a6)
pub fn i_03529c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0352A2: move.w #$ffff, $7e(a6)
pub fn i_0352a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0352A8: bclr.b #$2, $8c(a6)
pub fn i_0352a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352AE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0352AE: bclr.b #$1, $8c(a6)
pub fn i_0352ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352B4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0352B4: bclr.b #$3, $8c(a6)
pub fn i_0352b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352BA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0352BA: cmpi.b #$13, $70(a6)
pub fn i_0352ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352C0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x13, Size::Byte, false);
    16
}
/// $0352C0: bcs.w $352e4
pub fn i_0352c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352C4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x352E4; return 10; }
    8
}
/// $0352C4: cmpi.b #$24, $70(a6)
pub fn i_0352c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352CA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $0352CA: bcc.w $352e4
pub fn i_0352ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x352E4; return 10; }
    8
}
/// $0352CE: move.b #$12, $70(a6)
pub fn i_0352ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352D4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0352D4: lea.l $279926.l, a0
pub fn i_0352d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0352DA: move.l -$4(a0), $74(a6)
pub fn i_0352da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352E0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0352E0: bra.w $35308
pub fn i_0352e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x35308;
    10
}
/// $0352E4: move.b #$12, $70(a6)
pub fn i_0352e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352EA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0352EA: lea.l $279926.l, a0
pub fn i_0352ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0352F0: move.l -$4(a0), $74(a6)
pub fn i_0352f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352F6;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0352F6: move.b #$ff, $21(a6)
pub fn i_0352f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x352FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0352FC: lea.l $279926.l, a0
pub fn i_0352fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35302;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $035302: jsr $28cd4.l
pub fn i_035302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35308;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35308);
    cpu.pc = 0x28CD4;
    16
}
/// $035308: cmpi.b #$12, $70(a6)
pub fn i_035308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3530E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x12, Size::Byte, false);
    16
}
/// $03530E: bne.w $353c4
pub fn i_03530e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35312;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x353C4; return 10; }
    8
}
/// $035312: cmpi.b #$4, $78(a6)
pub fn i_035312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35318;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $035318: bne.w $35372
pub fn i_035318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3531C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35372; return 10; }
    8
}
/// $03531C: cmpi.w #$1, $72(a6)
pub fn i_03531c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35322;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $035322: beq.w $3536e
pub fn i_035322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35326;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3536E; return 10; }
    8
}
/// $035326: move.w #$1, $72(a6)
pub fn i_035326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3532C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03532C: jsr $2abcc.l
pub fn i_03532c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35332;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35332);
    cpu.pc = 0x2ABCC;
    16
}
/// $035332: bcs.w $3534c
pub fn i_035332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35336;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3534C; return 10; }
    8
}
/// $035336: move.b #$12, $70(a6)
pub fn i_035336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3533C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03533C: lea.l $27991c.l, a0
pub fn i_03533c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35342;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27991C;
    20
}
/// $035342: move.l -$4(a0), $74(a6)
pub fn i_035342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35348;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035348: bra.w $3535e
pub fn i_035348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3534C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3535E;
    10
}
/// $03534C: move.b #$12, $70(a6)
pub fn i_03534c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35352;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035352: lea.l $27991c.l, a0
pub fn i_035352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35358;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27991C;
    20
}
/// $035358: move.l -$4(a0), $74(a6)
pub fn i_035358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3535E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03535E: cmpi.b #$14, $79(a6)
pub fn i_03535e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35364;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Byte, false);
    16
}
/// $035364: bne.w $3536e
pub fn i_035364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35368;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3536E; return 10; }
    8
}
/// $035368: bset.b #$5, $69(a6)
pub fn i_035368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3536E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03536E: bra.w $353c4
pub fn i_03536e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35372;
    cpu.ir = 0x6000;
    cpu.pc = 0x353C4;
    10
}
/// $035372: cmpi.w #$0, $72(a6)
pub fn i_035372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35378;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $035378: beq.w $353c4
pub fn i_035378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3537C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x353C4; return 10; }
    8
}
/// $03537C: move.w #$0, $72(a6)
pub fn i_03537c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35382;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035382: jsr $2abcc.l
pub fn i_035382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35388;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35388);
    cpu.pc = 0x2ABCC;
    16
}
/// $035388: bcs.w $353a2
pub fn i_035388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3538C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x353A2; return 10; }
    8
}
/// $03538C: move.b #$12, $70(a6)
pub fn i_03538c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35392;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035392: lea.l $279926.l, a0
pub fn i_035392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35398;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $035398: move.l -$4(a0), $74(a6)
pub fn i_035398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3539E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03539E: bra.w $353b4
pub fn i_03539e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x353B4;
    10
}
/// $0353A2: move.b #$12, $70(a6)
pub fn i_0353a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353A8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0353A8: lea.l $279926.l, a0
pub fn i_0353a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0353AE: move.l -$4(a0), $74(a6)
pub fn i_0353ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353B4;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0353B4: cmpi.b #$41, $79(a6)
pub fn i_0353b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353BA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x41, Size::Byte, false);
    16
}
/// $0353BA: bne.w $353c4
pub fn i_0353ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353BE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x353C4; return 10; }
    8
}
/// $0353BE: bset.b #$5, $69(a6)
pub fn i_0353be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353C4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0353C4: btst.b #$2, $8c(a6)
pub fn i_0353c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353CA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0353CA: beq.w $35402
pub fn i_0353ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x35402; return 10; }
    8
}
/// $0353CE: jsr $32e3c(pc)
pub fn i_0353ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x353D2);
    cpu.pc = 0x32E3C;
    16
}
/// $0353D2: bcs.w $353dc
pub fn i_0353d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353D6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x353DC; return 10; }
    8
}
/// $0353D6: lea.l $34704(pc), a1
pub fn i_0353d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $0353DA: move.l a1, (a6)
pub fn i_0353da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0353DC: jsr $32e42(pc)
pub fn i_0353dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x353E0);
    cpu.pc = 0x32E42;
    16
}
/// $0353E0: bcc.w $353fe
pub fn i_0353e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x353FE; return 10; }
    8
}
/// $0353E4: btst.b #$2, $8c(a6)
pub fn i_0353e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353EA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0353EA: beq.w $353f8
pub fn i_0353ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353EE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x353F8; return 10; }
    8
}
/// $0353EE: eori.b #$1, $3a(a6)
pub fn i_0353ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353F4;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0353F4: bra.w $353fe
pub fn i_0353f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x353FE;
    10
}
/// $0353F8: lea.l $35bf8(pc), a1
pub fn i_0353f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35BF8;
    16
}
/// $0353FC: move.l a1, (a6)
pub fn i_0353fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x353FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0353FE: bra.w $3548c
pub fn i_0353fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35402;
    cpu.ir = 0x6000;
    cpu.pc = 0x3548C;
    10
}
/// $035402: jsr $32e3c(pc)
pub fn i_035402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35406;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35406);
    cpu.pc = 0x32E3C;
    16
}
/// $035406: bcs.w $35410
pub fn i_035406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3540A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x35410; return 10; }
    8
}
/// $03540A: lea.l $34046(pc), a1
pub fn i_03540a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3540E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34046;
    16
}
/// $03540E: move.l a1, (a6)
pub fn i_03540e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35410;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035410: jsr $32e42(pc)
pub fn i_035410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35414;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35414);
    cpu.pc = 0x32E42;
    16
}
/// $035414: bcc.w $35432
pub fn i_035414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35418;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35432; return 10; }
    8
}
/// $035418: btst.b #$2, $8c(a6)
pub fn i_035418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3541E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03541E: beq.w $3542c
pub fn i_03541e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35422;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3542C; return 10; }
    8
}
/// $035422: eori.b #$1, $3a(a6)
pub fn i_035422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35428;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035428: bra.w $35432
pub fn i_035428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3542C;
    cpu.ir = 0x6000;
    cpu.pc = 0x35432;
    10
}
/// $03542C: lea.l $35bf8(pc), a1
pub fn i_03542c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35430;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35BF8;
    16
}
/// $035430: move.l a1, (a6)
pub fn i_035430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35432;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035432: jsr $330d0(pc)
pub fn i_035432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35436;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35436);
    cpu.pc = 0x330D0;
    16
}
/// $035436: bcc.w $3548c
pub fn i_035436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3543A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3548C; return 10; }
    8
}
/// $03543A: cmpi.b #$ff, d1
pub fn i_03543a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3543E;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $03543E: bne.w $3544c
pub fn i_03543e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35442;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3544C; return 10; }
    8
}
/// $035442: lea.l $35d34(pc), a1
pub fn i_035442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35446;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35D34;
    16
}
/// $035446: move.l a1, (a6)
pub fn i_035446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35448;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035448: bra.w $3548c
pub fn i_035448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3544C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3548C;
    10
}
/// $03544C: cmpi.b #$3, d1
pub fn i_03544c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35450;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $035450: bne.w $3545e
pub fn i_035450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35454;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3545E; return 10; }
    8
}
/// $035454: lea.l $360bc(pc), a1
pub fn i_035454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35458;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x360BC;
    16
}
/// $035458: move.l a1, (a6)
pub fn i_035458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3545A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03545A: bra.w $3548c
pub fn i_03545a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3545E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3548C;
    10
}
/// $03545E: cmpi.b #$4, d1
pub fn i_03545e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35462;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $035462: bne.w $35470
pub fn i_035462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35466;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35470; return 10; }
    8
}
/// $035466: lea.l $3873c(pc), a1
pub fn i_035466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3546A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $03546A: move.l a1, (a6)
pub fn i_03546a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3546C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03546C: bra.w $3548c
pub fn i_03546c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35470;
    cpu.ir = 0x6000;
    cpu.pc = 0x3548C;
    10
}
/// $035470: cmpi.b #$1, d1
pub fn i_035470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35474;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $035474: bne.w $35482
pub fn i_035474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35478;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35482; return 10; }
    8
}
/// $035478: lea.l $357be(pc), a1
pub fn i_035478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3547C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x357BE;
    16
}
/// $03547C: move.l a1, (a6)
pub fn i_03547c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3547E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03547E: bra.w $3548c
pub fn i_03547e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35482;
    cpu.ir = 0x6000;
    cpu.pc = 0x3548C;
    10
}
/// $035482: lea.l $357aa(pc), a1
pub fn i_035482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35486;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x357AA;
    16
}
/// $035486: move.l a1, (a6)
pub fn i_035486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35488;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035488: bra.w $3548c
pub fn i_035488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3548C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3548C;
    10
}
/// $03548C: jsr $32e90(pc)
pub fn i_03548c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35490;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35490);
    cpu.pc = 0x32E90;
    16
}
/// $035490: bcc.w $3549a
pub fn i_035490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35494;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3549A; return 10; }
    8
}
/// $035494: lea.l $37c74(pc), a1
pub fn i_035494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35498;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C74;
    16
}
/// $035498: move.l a1, (a6)
pub fn i_035498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3549A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03549A: cmpi.b #$0, $85(a6)
pub fn i_03549a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354A0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0354A0: bne.w $354b4
pub fn i_0354a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x354B4; return 10; }
    8
}
/// $0354A4: cmpi.b #$1, $71(a6)
pub fn i_0354a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354AA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0354AA: bne.w $354b4
pub fn i_0354aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x354B4; return 10; }
    8
}
/// $0354AE: lea.l $33b9a(pc), a1
pub fn i_0354ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33B9A;
    16
}
/// $0354B2: move.l a1, (a6)
pub fn i_0354b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0354B4: cmpi.w #$0, $82(a6)
pub fn i_0354b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354BA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0354BA: bne.w $354ce
pub fn i_0354ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354BE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x354CE; return 10; }
    8
}
/// $0354BE: cmpi.b #$0, $71(a6)
pub fn i_0354be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354C4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0354C4: beq.w $354ce
pub fn i_0354c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x354CE; return 10; }
    8
}
/// $0354C8: lea.l $33d64(pc), a1
pub fn i_0354c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $0354CC: move.l a1, (a6)
pub fn i_0354cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0354CE: jsr $27eba.l
pub fn i_0354ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x354D4);
    cpu.pc = 0x27EBA;
    16
}
/// $0354D4: bcc.w $354de
pub fn i_0354d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x354DE; return 10; }
    8
}
/// $0354D8: lea.l $36d64(pc), a1
pub fn i_0354d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36D64;
    16
}
/// $0354DC: move.l a1, (a6)
pub fn i_0354dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0354DE: jsr $33034(pc)
pub fn i_0354de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x354E2);
    cpu.pc = 0x33034;
    16
}
/// $0354E2: bcc.w $354ec
pub fn i_0354e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x354EC; return 10; }
    8
}
/// $0354E6: lea.l $36914(pc), a1
pub fn i_0354e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $0354EA: move.l a1, (a6)
pub fn i_0354ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0354EC: jsr $33522(pc)
pub fn i_0354ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x354F0);
    cpu.pc = 0x33522;
    16
}
/// $0354F0: rts
pub fn i_0354f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_WalkLoopLeft_0354f2  ($0354F2, 158 B, player/states_0342c4.s)
/// $0354F2: cmpi.b #$13, $70(a6)
pub fn i_0354f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354F8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x13, Size::Byte, false);
    16
}
/// $0354F8: bcs.w $35528
pub fn i_0354f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x354FC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x35528; return 10; }
    8
}
/// $0354FC: cmpi.b #$24, $70(a6)
pub fn i_0354fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35502;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $035502: bcc.w $35528
pub fn i_035502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35506;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35528; return 10; }
    8
}
/// $035506: lea.l $279926.l, a0
pub fn i_035506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3550C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $03550C: jsr $28cd4.l
pub fn i_03550c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35512);
    cpu.pc = 0x28CD4;
    16
}
/// $035512: move.w #$0, $7c(a6)
pub fn i_035512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35518;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035518: move.w #$3, $7e(a6)
pub fn i_035518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3551E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03551E: bclr.b #$5, $69(a6)
pub fn i_03551e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35524;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035524: bra.w $3556a
pub fn i_035524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35528;
    cpu.ir = 0x6000;
    cpu.pc = 0x3556A;
    10
}
/// $035528: move.w #$0, $7c(a6)
pub fn i_035528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3552E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03552E: move.w #$ffff, $7e(a6)
pub fn i_03552e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35534;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035534: bclr.b #$2, $8c(a6)
pub fn i_035534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3553A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03553A: bclr.b #$1, $8c(a6)
pub fn i_03553a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35540;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035540: bclr.b #$3, $8c(a6)
pub fn i_035540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35546;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035546: move.b #$12, $70(a6)
pub fn i_035546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3554C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03554C: lea.l $279926.l, a0
pub fn i_03554c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35552;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $035552: move.l -$4(a0), $74(a6)
pub fn i_035552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35558;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035558: move.b #$ff, $21(a6)
pub fn i_035558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3555E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03555E: lea.l $279926.l, a0
pub fn i_03555e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35564;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $035564: jsr $28cd4.l
pub fn i_035564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3556A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3556A);
    cpu.pc = 0x28CD4;
    16
}
/// $03556A: move.w #$fd00, $28(a6)
pub fn i_03556a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35570;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035570: clr.w $2c(a6)
pub fn i_035570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35574;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $035574: move.l #$32500, $60(a6)
pub fn i_035574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3557C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03557C: lea.l $326e0(pc), a0
pub fn i_03557c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35580;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $035580: move.l a0, $48(a6)
pub fn i_035580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35584;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035584: lea.l $35590(pc), a1
pub fn i_035584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35588;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35590;
    16
}
/// $035588: move.l a1, (a6)
pub fn i_035588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3558A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03558A: bset.b #$0, $3a(a6)
pub fn i_03558a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35590;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== Player_WalkLoopLeft_Run_035590  ($035590, 538 B, player/states_0342c4.s)
/// $035590: jsr $32ff2(pc)
pub fn i_035590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35594;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35594);
    cpu.pc = 0x32FF2;
    16
}
/// $035594: jsr $32d32(pc)
pub fn i_035594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35598;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35598);
    cpu.pc = 0x32D32;
    16
}
/// $035598: bcc.w $355a2
pub fn i_035598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3559C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x355A2; return 10; }
    8
}
/// $03559C: lea.l $345b8(pc), a1
pub fn i_03559c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345B8;
    16
}
/// $0355A0: move.l a1, (a6)
pub fn i_0355a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0355A2: move.w #$fd00, $28(a6)
pub fn i_0355a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0355A8: jsr $27a92.l
pub fn i_0355a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x355AE);
    cpu.pc = 0x27A92;
    16
}
/// $0355AE: jsr $32aa8(pc)
pub fn i_0355ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x355B2);
    cpu.pc = 0x32AA8;
    16
}
/// $0355B2: bcc.w $35622
pub fn i_0355b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35622; return 10; }
    8
}
/// $0355B6: move.w #$0, $7c(a6)
pub fn i_0355b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0355BC: move.w #$ffff, $7e(a6)
pub fn i_0355bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355C2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0355C2: bclr.b #$2, $8c(a6)
pub fn i_0355c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355C8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0355C8: bclr.b #$1, $8c(a6)
pub fn i_0355c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355CE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0355CE: bclr.b #$3, $8c(a6)
pub fn i_0355ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355D4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0355D4: cmpi.b #$13, $70(a6)
pub fn i_0355d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355DA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x13, Size::Byte, false);
    16
}
/// $0355DA: bcs.w $355fe
pub fn i_0355da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355DE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x355FE; return 10; }
    8
}
/// $0355DE: cmpi.b #$24, $70(a6)
pub fn i_0355de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355E4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $0355E4: bcc.w $355fe
pub fn i_0355e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x355FE; return 10; }
    8
}
/// $0355E8: move.b #$12, $70(a6)
pub fn i_0355e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0355EE: lea.l $279926.l, a0
pub fn i_0355ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0355F4: move.l -$4(a0), $74(a6)
pub fn i_0355f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355FA;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0355FA: bra.w $35622
pub fn i_0355fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x355FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x35622;
    10
}
/// $0355FE: move.b #$12, $70(a6)
pub fn i_0355fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35604;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035604: lea.l $279926.l, a0
pub fn i_035604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3560A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $03560A: move.l -$4(a0), $74(a6)
pub fn i_03560a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35610;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035610: move.b #$ff, $21(a6)
pub fn i_035610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35616;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035616: lea.l $279926.l, a0
pub fn i_035616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3561C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $03561C: jsr $28cd4.l
pub fn i_03561c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35622;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35622);
    cpu.pc = 0x28CD4;
    16
}
/// $035622: cmpi.b #$12, $70(a6)
pub fn i_035622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35628;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x12, Size::Byte, false);
    16
}
/// $035628: bne.w $356de
pub fn i_035628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3562C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x356DE; return 10; }
    8
}
/// $03562C: cmpi.b #$4, $78(a6)
pub fn i_03562c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35632;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $035632: bne.w $3568c
pub fn i_035632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35636;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3568C; return 10; }
    8
}
/// $035636: cmpi.w #$1, $72(a6)
pub fn i_035636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3563C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03563C: beq.w $35688
pub fn i_03563c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35640;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x35688; return 10; }
    8
}
/// $035640: move.w #$1, $72(a6)
pub fn i_035640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35646;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035646: jsr $2abcc.l
pub fn i_035646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3564C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3564C);
    cpu.pc = 0x2ABCC;
    16
}
/// $03564C: bcs.w $35666
pub fn i_03564c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35650;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x35666; return 10; }
    8
}
/// $035650: move.b #$12, $70(a6)
pub fn i_035650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35656;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035656: lea.l $27991c.l, a0
pub fn i_035656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3565C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27991C;
    20
}
/// $03565C: move.l -$4(a0), $74(a6)
pub fn i_03565c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35662;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035662: bra.w $35678
pub fn i_035662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35666;
    cpu.ir = 0x6000;
    cpu.pc = 0x35678;
    10
}
/// $035666: move.b #$12, $70(a6)
pub fn i_035666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3566C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03566C: lea.l $27991c.l, a0
pub fn i_03566c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35672;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27991C;
    20
}
/// $035672: move.l -$4(a0), $74(a6)
pub fn i_035672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35678;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035678: cmpi.b #$14, $79(a6)
pub fn i_035678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3567E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Byte, false);
    16
}
/// $03567E: bne.w $35688
pub fn i_03567e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35682;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35688; return 10; }
    8
}
/// $035682: bset.b #$5, $69(a6)
pub fn i_035682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35688;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035688: bra.w $356de
pub fn i_035688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3568C;
    cpu.ir = 0x6000;
    cpu.pc = 0x356DE;
    10
}
/// $03568C: cmpi.w #$0, $72(a6)
pub fn i_03568c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35692;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $035692: beq.w $356de
pub fn i_035692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35696;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x356DE; return 10; }
    8
}
/// $035696: move.w #$0, $72(a6)
pub fn i_035696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3569C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03569C: jsr $2abcc.l
pub fn i_03569c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x356A2);
    cpu.pc = 0x2ABCC;
    16
}
/// $0356A2: bcs.w $356bc
pub fn i_0356a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356A6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x356BC; return 10; }
    8
}
/// $0356A6: move.b #$12, $70(a6)
pub fn i_0356a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356AC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0356AC: lea.l $279926.l, a0
pub fn i_0356ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0356B2: move.l -$4(a0), $74(a6)
pub fn i_0356b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356B8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0356B8: bra.w $356ce
pub fn i_0356b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x356CE;
    10
}
/// $0356BC: move.b #$12, $70(a6)
pub fn i_0356bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356C2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0356C2: lea.l $279926.l, a0
pub fn i_0356c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279926;
    20
}
/// $0356C8: move.l -$4(a0), $74(a6)
pub fn i_0356c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356CE;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0356CE: cmpi.b #$41, $79(a6)
pub fn i_0356ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356D4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x41, Size::Byte, false);
    16
}
/// $0356D4: bne.w $356de
pub fn i_0356d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x356DE; return 10; }
    8
}
/// $0356D8: bset.b #$5, $69(a6)
pub fn i_0356d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356DE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0356DE: btst.b #$2, $8c(a6)
pub fn i_0356de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356E4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0356E4: beq.w $3571c
pub fn i_0356e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356E8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3571C; return 10; }
    8
}
/// $0356E8: jsr $32e42(pc)
pub fn i_0356e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356EC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x356EC);
    cpu.pc = 0x32E42;
    16
}
/// $0356EC: bcs.w $356f6
pub fn i_0356ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356F0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x356F6; return 10; }
    8
}
/// $0356F0: lea.l $34704(pc), a1
pub fn i_0356f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $0356F4: move.l a1, (a6)
pub fn i_0356f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0356F6: jsr $32e3c(pc)
pub fn i_0356f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x356FA);
    cpu.pc = 0x32E3C;
    16
}
/// $0356FA: bcc.w $35718
pub fn i_0356fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x356FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35718; return 10; }
    8
}
/// $0356FE: btst.b #$2, $8c(a6)
pub fn i_0356fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35704;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $035704: beq.w $35712
pub fn i_035704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35708;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x35712; return 10; }
    8
}
/// $035708: eori.b #$1, $3a(a6)
pub fn i_035708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3570E;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03570E: bra.w $35718
pub fn i_03570e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35712;
    cpu.ir = 0x6000;
    cpu.pc = 0x35718;
    10
}
/// $035712: lea.l $35aba(pc), a1
pub fn i_035712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35716;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35ABA;
    16
}
/// $035716: move.l a1, (a6)
pub fn i_035716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35718;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035718: bra.w $357a6
pub fn i_035718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3571C;
    cpu.ir = 0x6000;
    cpu.pc = 0x357A6;
    10
}
/// $03571C: jsr $32e42(pc)
pub fn i_03571c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35720;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35720);
    cpu.pc = 0x32E42;
    16
}
/// $035720: bcs.w $3572a
pub fn i_035720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35724;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3572A; return 10; }
    8
}
/// $035724: lea.l $34046(pc), a1
pub fn i_035724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35728;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34046;
    16
}
/// $035728: move.l a1, (a6)
pub fn i_035728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3572A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03572A: jsr $32e3c(pc)
pub fn i_03572a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3572E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3572E);
    cpu.pc = 0x32E3C;
    16
}
/// $03572E: bcc.w $3574c
pub fn i_03572e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35732;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3574C; return 10; }
    8
}
/// $035732: btst.b #$2, $8c(a6)
pub fn i_035732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35738;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $035738: beq.w $35746
pub fn i_035738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3573C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x35746; return 10; }
    8
}
/// $03573C: eori.b #$1, $3a(a6)
pub fn i_03573c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35742;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035742: bra.w $3574c
pub fn i_035742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35746;
    cpu.ir = 0x6000;
    cpu.pc = 0x3574C;
    10
}
/// $035746: lea.l $35aba(pc), a1
pub fn i_035746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3574A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35ABA;
    16
}
/// $03574A: move.l a1, (a6)
pub fn i_03574a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3574C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03574C: jsr $330d0(pc)
pub fn i_03574c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35750;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35750);
    cpu.pc = 0x330D0;
    16
}
/// $035750: bcc.w $357a6
pub fn i_035750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35754;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x357A6; return 10; }
    8
}
/// $035754: cmpi.b #$ff, d1
pub fn i_035754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35758;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $035758: bne.w $35766
pub fn i_035758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3575C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35766; return 10; }
    8
}
/// $03575C: lea.l $35d34(pc), a1
pub fn i_03575c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35760;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35D34;
    16
}
/// $035760: move.l a1, (a6)
pub fn i_035760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35762;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035762: bra.w $357a6
pub fn i_035762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35766;
    cpu.ir = 0x6000;
    cpu.pc = 0x357A6;
    10
}
/// $035766: cmpi.b #$3, d1
pub fn i_035766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3576A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $03576A: bne.w $35778
pub fn i_03576a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3576E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35778; return 10; }
    8
}
/// $03576E: lea.l $360bc(pc), a1
pub fn i_03576e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35772;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x360BC;
    16
}
/// $035772: move.l a1, (a6)
pub fn i_035772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35774;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035774: bra.w $357a6
pub fn i_035774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35778;
    cpu.ir = 0x6000;
    cpu.pc = 0x357A6;
    10
}
/// $035778: cmpi.b #$4, d1
pub fn i_035778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3577C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $03577C: bne.w $3578a
pub fn i_03577c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35780;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3578A; return 10; }
    8
}
/// $035780: lea.l $3873c(pc), a1
pub fn i_035780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35784;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $035784: move.l a1, (a6)
pub fn i_035784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35786;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035786: bra.w $357a6
pub fn i_035786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3578A;
    cpu.ir = 0x6000;
    cpu.pc = 0x357A6;
    10
}
/// $03578A: cmpi.b #$1, d1
pub fn i_03578a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3578E;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $03578E: bne.w $3579c
pub fn i_03578e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35792;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3579C; return 10; }
    8
}
/// $035792: lea.l $357e6(pc), a1
pub fn i_035792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35796;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x357E6;
    16
}
/// $035796: move.l a1, (a6)
pub fn i_035796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35798;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035798: bra.w $357a6
pub fn i_035798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3579C;
    cpu.ir = 0x6000;
    cpu.pc = 0x357A6;
    10
}
/// $03579C: lea.l $357d2(pc), a1
pub fn i_03579c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x357D2;
    16
}
/// $0357A0: move.l a1, (a6)
pub fn i_0357a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0357A2: bra.w $357a6
pub fn i_0357a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x357A6;
    10
}
/// $0357A6: bra.w $3548c
pub fn i_0357a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3548C;
    10
}
// ===== Player_WalkLoopRight_Shoot_0357aa  ($0357AA, 20 B, player/states_0342c4.s)
/// $0357AA: move.w #$1, $7c(a6)
pub fn i_0357aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0357B0: move.w #$3, $7e(a6)
pub fn i_0357b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0357B6: jsr $357fa(pc)
pub fn i_0357b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x357BA);
    cpu.pc = 0x357FA;
    16
}
/// $0357BA: bra.w $35250
pub fn i_0357ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x35250;
    10
}
// ===== Player_WalkLoopRight_ShootUp_0357be  ($0357BE, 20 B, player/states_0342c4.s)
/// $0357BE: move.w #$0, $7c(a6)
pub fn i_0357be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0357C4: move.w #$4, $7e(a6)
pub fn i_0357c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357CA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0357CA: jsr $3595a(pc)
pub fn i_0357ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x357CE);
    cpu.pc = 0x3595A;
    16
}
/// $0357CE: bra.w $35250
pub fn i_0357ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x35250;
    10
}
// ===== Player_WalkLoopLeft_Shoot_0357d2  ($0357D2, 20 B, player/states_0342c4.s)
/// $0357D2: move.w #$1, $7c(a6)
pub fn i_0357d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0357D8: move.w #$3, $7e(a6)
pub fn i_0357d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357DE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0357DE: jsr $357fa(pc)
pub fn i_0357de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x357E2);
    cpu.pc = 0x357FA;
    16
}
/// $0357E2: bra.w $3556a
pub fn i_0357e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3556A;
    10
}
// ===== Player_WalkLoopLeft_ShootUp_0357e6  ($0357E6, 20 B, player/states_0342c4.s)
/// $0357E6: move.w #$0, $7c(a6)
pub fn i_0357e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0357EC: move.w #$4, $7e(a6)
pub fn i_0357ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0357F2: jsr $3595a(pc)
pub fn i_0357f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x357F6);
    cpu.pc = 0x3595A;
    16
}
/// $0357F6: bra.w $3556a
pub fn i_0357f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x357FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3556A;
    10
}
// ===== Player_WalkLoopShootPose_0357fa  ($0357FA, 176 B, player/states_0342c4.s)
/// $0357FA: bset.b #$1, $8c(a6)
pub fn i_0357fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35800;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035800: cmpi.w #$1, $72(a6)
pub fn i_035800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35806;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $035806: bne.w $35858
pub fn i_035806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3580A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35858; return 10; }
    8
}
/// $03580A: cmpi.b #$24, $70(a6)
pub fn i_03580a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35810;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $035810: bcc.w $35830
pub fn i_035810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35814;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35830; return 10; }
    8
}
/// $035814: bset.b #$5, $69(a6)
pub fn i_035814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3581A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03581A: move.b #$23, $70(a6)
pub fn i_03581a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35820;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035820: lea.l $2798f4.l, a0
pub fn i_035820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35826;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798F4;
    20
}
/// $035826: move.l -$4(a0), $74(a6)
pub fn i_035826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3582C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03582C: bra.w $35854
pub fn i_03582c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35830;
    cpu.ir = 0x6000;
    cpu.pc = 0x35854;
    10
}
/// $035830: move.b #$23, $70(a6)
pub fn i_035830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35836;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035836: lea.l $2798f4.l, a0
pub fn i_035836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3583C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798F4;
    20
}
/// $03583C: move.l -$4(a0), $74(a6)
pub fn i_03583c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35842;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035842: move.b #$ff, $21(a6)
pub fn i_035842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35848;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035848: lea.l $2798f4.l, a0
pub fn i_035848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3584E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798F4;
    20
}
/// $03584E: jsr $28cd4.l
pub fn i_03584e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35854;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35854);
    cpu.pc = 0x28CD4;
    16
}
/// $035854: bra.w $358a2
pub fn i_035854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35858;
    cpu.ir = 0x6000;
    cpu.pc = 0x358A2;
    10
}
/// $035858: cmpi.b #$24, $70(a6)
pub fn i_035858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3585E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $03585E: bcc.w $3587e
pub fn i_03585e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35862;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3587E; return 10; }
    8
}
/// $035862: bset.b #$5, $69(a6)
pub fn i_035862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35868;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035868: move.b #$23, $70(a6)
pub fn i_035868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3586E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03586E: lea.l $2798e0.l, a0
pub fn i_03586e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35874;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798E0;
    20
}
/// $035874: move.l -$4(a0), $74(a6)
pub fn i_035874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3587A;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03587A: bra.w $358a2
pub fn i_03587a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3587E;
    cpu.ir = 0x6000;
    cpu.pc = 0x358A2;
    10
}
/// $03587E: move.b #$23, $70(a6)
pub fn i_03587e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35884;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035884: lea.l $2798e0.l, a0
pub fn i_035884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3588A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798E0;
    20
}
/// $03588A: move.l -$4(a0), $74(a6)
pub fn i_03588a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35890;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035890: move.b #$ff, $21(a6)
pub fn i_035890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35896;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035896: lea.l $2798e0.l, a0
pub fn i_035896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3589C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798E0;
    20
}
/// $03589C: jsr $28cd4.l
pub fn i_03589c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x358A2);
    cpu.pc = 0x28CD4;
    16
}
/// $0358A2: move.w #$0, $72(a6)
pub fn i_0358a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358A8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0358A8: rts
pub fn i_0358a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_WalkLoopShootPose_Alt_0358aa  ($0358AA, 176 B, player/states_0342c4.s)
/// $0358AA: bset.b #$1, $8c(a6)
pub fn i_0358aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358B0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0358B0: cmpi.w #$1, $72(a6)
pub fn i_0358b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358B6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $0358B6: bne.w $35908
pub fn i_0358b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358BA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35908; return 10; }
    8
}
/// $0358BA: cmpi.b #$24, $70(a6)
pub fn i_0358ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358C0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $0358C0: bcc.w $358e0
pub fn i_0358c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x358E0; return 10; }
    8
}
/// $0358C4: bset.b #$5, $69(a6)
pub fn i_0358c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358CA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0358CA: move.b #$23, $70(a6)
pub fn i_0358ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358D0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0358D0: lea.l $2798f4.l, a0
pub fn i_0358d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798F4;
    20
}
/// $0358D6: move.l -$4(a0), $74(a6)
pub fn i_0358d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358DC;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0358DC: bra.w $35904
pub fn i_0358dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x35904;
    10
}
/// $0358E0: move.b #$23, $70(a6)
pub fn i_0358e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358E6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0358E6: lea.l $2798f4.l, a0
pub fn i_0358e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798F4;
    20
}
/// $0358EC: move.l -$4(a0), $74(a6)
pub fn i_0358ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358F2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0358F2: move.b #$ff, $21(a6)
pub fn i_0358f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358F8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0358F8: lea.l $2798f4.l, a0
pub fn i_0358f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x358FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798F4;
    20
}
/// $0358FE: jsr $28cd4.l
pub fn i_0358fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35904;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35904);
    cpu.pc = 0x28CD4;
    16
}
/// $035904: bra.w $35952
pub fn i_035904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35908;
    cpu.ir = 0x6000;
    cpu.pc = 0x35952;
    10
}
/// $035908: cmpi.b #$24, $70(a6)
pub fn i_035908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3590E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $03590E: bcc.w $3592e
pub fn i_03590e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35912;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3592E; return 10; }
    8
}
/// $035912: bset.b #$5, $69(a6)
pub fn i_035912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35918;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035918: move.b #$23, $70(a6)
pub fn i_035918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3591E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03591E: lea.l $279908.l, a0
pub fn i_03591e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35924;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279908;
    20
}
/// $035924: move.l -$4(a0), $74(a6)
pub fn i_035924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3592A;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03592A: bra.w $35952
pub fn i_03592a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3592E;
    cpu.ir = 0x6000;
    cpu.pc = 0x35952;
    10
}
/// $03592E: move.b #$23, $70(a6)
pub fn i_03592e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35934;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035934: lea.l $279908.l, a0
pub fn i_035934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3593A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279908;
    20
}
/// $03593A: move.l -$4(a0), $74(a6)
pub fn i_03593a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35940;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035940: move.b #$ff, $21(a6)
pub fn i_035940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35946;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035946: lea.l $279908.l, a0
pub fn i_035946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3594C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279908;
    20
}
/// $03594C: jsr $28cd4.l
pub fn i_03594c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35952);
    cpu.pc = 0x28CD4;
    16
}
/// $035952: move.w #$0, $72(a6)
pub fn i_035952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35958;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035958: rts
pub fn i_035958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3595A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_WalkLoopShootUpPose_03595a  ($03595A, 176 B, player/states_0342c4.s)
/// $03595A: bset.b #$1, $8c(a6)
pub fn i_03595a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35960;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035960: cmpi.w #$0, $72(a6)
pub fn i_035960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35966;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $035966: bne.w $359b8
pub fn i_035966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3596A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x359B8; return 10; }
    8
}
/// $03596A: cmpi.b #$24, $70(a6)
pub fn i_03596a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35970;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $035970: bcc.w $35990
pub fn i_035970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35974;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35990; return 10; }
    8
}
/// $035974: bset.b #$5, $69(a6)
pub fn i_035974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3597A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03597A: move.b #$23, $70(a6)
pub fn i_03597a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35980;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035980: lea.l $2798fe.l, a0
pub fn i_035980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35986;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798FE;
    20
}
/// $035986: move.l -$4(a0), $74(a6)
pub fn i_035986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3598C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03598C: bra.w $359b4
pub fn i_03598c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35990;
    cpu.ir = 0x6000;
    cpu.pc = 0x359B4;
    10
}
/// $035990: move.b #$23, $70(a6)
pub fn i_035990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35996;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035996: lea.l $2798fe.l, a0
pub fn i_035996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3599C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798FE;
    20
}
/// $03599C: move.l -$4(a0), $74(a6)
pub fn i_03599c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359A2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0359A2: move.b #$ff, $21(a6)
pub fn i_0359a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359A8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0359A8: lea.l $2798fe.l, a0
pub fn i_0359a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798FE;
    20
}
/// $0359AE: jsr $28cd4.l
pub fn i_0359ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x359B4);
    cpu.pc = 0x28CD4;
    16
}
/// $0359B4: bra.w $35a02
pub fn i_0359b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x35A02;
    10
}
/// $0359B8: cmpi.b #$24, $70(a6)
pub fn i_0359b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359BE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $0359BE: bcc.w $359de
pub fn i_0359be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x359DE; return 10; }
    8
}
/// $0359C2: bset.b #$5, $69(a6)
pub fn i_0359c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359C8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0359C8: move.b #$23, $70(a6)
pub fn i_0359c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359CE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0359CE: lea.l $2798ea.l, a0
pub fn i_0359ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798EA;
    20
}
/// $0359D4: move.l -$4(a0), $74(a6)
pub fn i_0359d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359DA;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0359DA: bra.w $35a02
pub fn i_0359da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x35A02;
    10
}
/// $0359DE: move.b #$23, $70(a6)
pub fn i_0359de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0359E4: lea.l $2798ea.l, a0
pub fn i_0359e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798EA;
    20
}
/// $0359EA: move.l -$4(a0), $74(a6)
pub fn i_0359ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359F0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0359F0: move.b #$ff, $21(a6)
pub fn i_0359f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359F6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0359F6: lea.l $2798ea.l, a0
pub fn i_0359f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x359FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798EA;
    20
}
/// $0359FC: jsr $28cd4.l
pub fn i_0359fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35A02);
    cpu.pc = 0x28CD4;
    16
}
/// $035A02: move.w #$1, $72(a6)
pub fn i_035a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A08;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035A08: rts
pub fn i_035a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A0A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_WalkLoopShootUpPose_Alt_035a0a  ($035A0A, 176 B, player/states_0342c4.s)
/// $035A0A: bset.b #$1, $8c(a6)
pub fn i_035a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A10;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035A10: cmpi.w #$0, $72(a6)
pub fn i_035a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A16;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $035A16: bne.w $35a68
pub fn i_035a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A1A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35A68; return 10; }
    8
}
/// $035A1A: cmpi.b #$24, $70(a6)
pub fn i_035a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A20;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $035A20: bcc.w $35a40
pub fn i_035a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35A40; return 10; }
    8
}
/// $035A24: bset.b #$5, $69(a6)
pub fn i_035a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A2A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035A2A: move.b #$23, $70(a6)
pub fn i_035a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A30;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035A30: lea.l $2798fe.l, a0
pub fn i_035a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A36;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798FE;
    20
}
/// $035A36: move.l -$4(a0), $74(a6)
pub fn i_035a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A3C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035A3C: bra.w $35a64
pub fn i_035a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A40;
    cpu.ir = 0x6000;
    cpu.pc = 0x35A64;
    10
}
/// $035A40: move.b #$23, $70(a6)
pub fn i_035a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035A46: lea.l $2798fe.l, a0
pub fn i_035a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A4C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798FE;
    20
}
/// $035A4C: move.l -$4(a0), $74(a6)
pub fn i_035a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A52;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035A52: move.b #$ff, $21(a6)
pub fn i_035a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A58;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035A58: lea.l $2798fe.l, a0
pub fn i_035a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A5E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2798FE;
    20
}
/// $035A5E: jsr $28cd4.l
pub fn i_035a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35A64);
    cpu.pc = 0x28CD4;
    16
}
/// $035A64: bra.w $35ab2
pub fn i_035a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A68;
    cpu.ir = 0x6000;
    cpu.pc = 0x35AB2;
    10
}
/// $035A68: cmpi.b #$24, $70(a6)
pub fn i_035a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A6E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x24, Size::Byte, false);
    16
}
/// $035A6E: bcc.w $35a8e
pub fn i_035a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35A8E; return 10; }
    8
}
/// $035A72: bset.b #$5, $69(a6)
pub fn i_035a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A78;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035A78: move.b #$23, $70(a6)
pub fn i_035a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A7E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035A7E: lea.l $279912.l, a0
pub fn i_035a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A84;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279912;
    20
}
/// $035A84: move.l -$4(a0), $74(a6)
pub fn i_035a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A8A;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035A8A: bra.w $35ab2
pub fn i_035a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x35AB2;
    10
}
/// $035A8E: move.b #$23, $70(a6)
pub fn i_035a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A94;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035A94: lea.l $279912.l, a0
pub fn i_035a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35A9A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279912;
    20
}
/// $035A9A: move.l -$4(a0), $74(a6)
pub fn i_035a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AA0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035AA0: move.b #$ff, $21(a6)
pub fn i_035aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AA6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035AA6: lea.l $279912.l, a0
pub fn i_035aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AAC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279912;
    20
}
/// $035AAC: jsr $28cd4.l
pub fn i_035aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35AB2);
    cpu.pc = 0x28CD4;
    16
}
/// $035AB2: move.w #$1, $72(a6)
pub fn i_035ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AB8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035AB8: rts
pub fn i_035ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35ABA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_TurnRight_035aba  ($035ABA, 318 B, player/states_0342c4.s)
/// $035ABA: move.w #$0, $7c(a6)
pub fn i_035aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035AC0: move.w #$0, $7e(a6)
pub fn i_035ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AC6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035AC6: bclr.b #$2, $8c(a6)
pub fn i_035ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35ACC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035ACC: bclr.b #$1, $8c(a6)
pub fn i_035acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AD2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035AD2: bclr.b #$3, $8c(a6)
pub fn i_035ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AD8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035AD8: cmpi.w #$1, $72(a6)
pub fn i_035ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35ADE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $035ADE: bne.w $35b0a
pub fn i_035ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AE2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35B0A; return 10; }
    8
}
/// $035AE2: move.b #$31, $70(a6)
pub fn i_035ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AE8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x31;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035AE8: lea.l $279a5a.l, a0
pub fn i_035ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AEE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A5A;
    20
}
/// $035AEE: move.l -$4(a0), $74(a6)
pub fn i_035aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AF4;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035AF4: move.b #$ff, $21(a6)
pub fn i_035af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35AFA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035AFA: lea.l $279a5a.l, a0
pub fn i_035afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B00;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A5A;
    20
}
/// $035B00: jsr $28cd4.l
pub fn i_035b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35B06);
    cpu.pc = 0x28CD4;
    16
}
/// $035B06: bra.w $35b2e
pub fn i_035b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B0A;
    cpu.ir = 0x6000;
    cpu.pc = 0x35B2E;
    10
}
/// $035B0A: move.b #$31, $70(a6)
pub fn i_035b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B10;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x31;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035B10: lea.l $279a3c.l, a0
pub fn i_035b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A3C;
    20
}
/// $035B16: move.l -$4(a0), $74(a6)
pub fn i_035b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B1C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035B1C: move.b #$ff, $21(a6)
pub fn i_035b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B22;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035B22: lea.l $279a3c.l, a0
pub fn i_035b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A3C;
    20
}
/// $035B28: jsr $28cd4.l
pub fn i_035b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35B2E);
    cpu.pc = 0x28CD4;
    16
}
/// $035B2E: move.w #$300, $28(a6)
pub fn i_035b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B34;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035B34: clr.w $2c(a6)
pub fn i_035b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B38;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $035B38: move.l #$32500, $60(a6)
pub fn i_035b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B40;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035B40: lea.l $326e0(pc), a0
pub fn i_035b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B44;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $035B44: move.l a0, $48(a6)
pub fn i_035b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B48;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035B48: bclr.b #$0, $3a(a6)
pub fn i_035b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B4E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035B4E: lea.l $35b54(pc), a1
pub fn i_035b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35B54;
    16
}
/// $035B52: move.l a1, (a6)
pub fn i_035b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035B54: jsr $32ff2(pc)
pub fn i_035b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35B58);
    cpu.pc = 0x32FF2;
    16
}
/// $035B58: jsr $32d32(pc)
pub fn i_035b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35B5C);
    cpu.pc = 0x32D32;
    16
}
/// $035B5C: bcc.w $35b66
pub fn i_035b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35B66; return 10; }
    8
}
/// $035B60: lea.l $345b8(pc), a1
pub fn i_035b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345B8;
    16
}
/// $035B64: move.l a1, (a6)
pub fn i_035b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035B66: move.w #$300, $28(a6)
pub fn i_035b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B6C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035B6C: jsr $27a92.l
pub fn i_035b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35B72);
    cpu.pc = 0x27A92;
    16
}
/// $035B72: jsr $32aa8(pc)
pub fn i_035b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35B76);
    cpu.pc = 0x32AA8;
    16
}
/// $035B76: bcc.w $35b90
pub fn i_035b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35B90; return 10; }
    8
}
/// $035B7A: lea.l $34704(pc), a1
pub fn i_035b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $035B7E: move.l a1, (a6)
pub fn i_035b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035B80: btst.b #$0, $88(a6)
pub fn i_035b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B86;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $035B86: beq.w $35b90
pub fn i_035b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B8A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x35B90; return 10; }
    8
}
/// $035B8A: lea.l $33a5e(pc), a1
pub fn i_035b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $035B8E: move.l a1, (a6)
pub fn i_035b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035B90: btst.b #$2, $8c(a6)
pub fn i_035b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B96;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $035B96: bne.w $35bf4
pub fn i_035b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B9A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35BF4; return 10; }
    8
}
/// $035B9A: jsr $330d0(pc)
pub fn i_035b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35B9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35B9E);
    cpu.pc = 0x330D0;
    16
}
/// $035B9E: bcc.w $35bf4
pub fn i_035b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BA2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35BF4; return 10; }
    8
}
/// $035BA2: cmpi.b #$ff, d1
pub fn i_035ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BA6;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $035BA6: bne.w $35bb4
pub fn i_035ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BAA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35BB4; return 10; }
    8
}
/// $035BAA: lea.l $35d34(pc), a1
pub fn i_035baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35D34;
    16
}
/// $035BAE: move.l a1, (a6)
pub fn i_035bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035BB0: bra.w $35bf4
pub fn i_035bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BB4;
    cpu.ir = 0x6000;
    cpu.pc = 0x35BF4;
    10
}
/// $035BB4: cmpi.b #$3, d1
pub fn i_035bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BB8;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $035BB8: bne.w $35bc6
pub fn i_035bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BBC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35BC6; return 10; }
    8
}
/// $035BBC: lea.l $360bc(pc), a1
pub fn i_035bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x360BC;
    16
}
/// $035BC0: move.l a1, (a6)
pub fn i_035bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035BC2: bra.w $35bf4
pub fn i_035bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BC6;
    cpu.ir = 0x6000;
    cpu.pc = 0x35BF4;
    10
}
/// $035BC6: cmpi.b #$4, d1
pub fn i_035bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BCA;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $035BCA: bne.w $35bd8
pub fn i_035bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BCE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35BD8; return 10; }
    8
}
/// $035BCE: lea.l $3873c(pc), a1
pub fn i_035bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $035BD2: move.l a1, (a6)
pub fn i_035bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035BD4: bra.w $35bf4
pub fn i_035bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BD8;
    cpu.ir = 0x6000;
    cpu.pc = 0x35BF4;
    10
}
/// $035BD8: cmpi.b #$1, d1
pub fn i_035bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BDC;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $035BDC: bne.w $35bea
pub fn i_035bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35BEA; return 10; }
    8
}
/// $035BE0: lea.l $3437e(pc), a1
pub fn i_035be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3437E;
    16
}
/// $035BE4: move.l a1, (a6)
pub fn i_035be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035BE6: bra.w $35bf4
pub fn i_035be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BEA;
    cpu.ir = 0x6000;
    cpu.pc = 0x35BF4;
    10
}
/// $035BEA: lea.l $342c4(pc), a1
pub fn i_035bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x342C4;
    16
}
/// $035BEE: move.l a1, (a6)
pub fn i_035bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035BF0: bra.w $35bf4
pub fn i_035bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x35BF4;
    10
}
/// $035BF4: jmp $3548c(pc)
pub fn i_035bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BF8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3548C;
    8
}
// ===== Player_TurnLeft_035bf8  ($035BF8, 160 B, player/states_0342c4.s)
/// $035BF8: move.w #$0, $7c(a6)
pub fn i_035bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35BFE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035BFE: move.w #$0, $7e(a6)
pub fn i_035bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C04;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035C04: bclr.b #$2, $8c(a6)
pub fn i_035c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C0A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035C0A: bclr.b #$1, $8c(a6)
pub fn i_035c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C10;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035C10: bclr.b #$3, $8c(a6)
pub fn i_035c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C16;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035C16: cmpi.w #$1, $72(a6)
pub fn i_035c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C1C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $035C1C: bne.w $35c4e
pub fn i_035c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C20;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35C4E; return 10; }
    8
}
/// $035C20: jsr $2abcc.l
pub fn i_035c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35C26);
    cpu.pc = 0x2ABCC;
    16
}
/// $035C26: move.b #$31, $70(a6)
pub fn i_035c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C2C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x31;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035C2C: lea.l $279a5a.l, a0
pub fn i_035c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A5A;
    20
}
/// $035C32: move.l -$4(a0), $74(a6)
pub fn i_035c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C38;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035C38: move.b #$ff, $21(a6)
pub fn i_035c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C3E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035C3E: lea.l $279a5a.l, a0
pub fn i_035c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A5A;
    20
}
/// $035C44: jsr $28cd4.l
pub fn i_035c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35C4A);
    cpu.pc = 0x28CD4;
    16
}
/// $035C4A: bra.w $35c72
pub fn i_035c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x35C72;
    10
}
/// $035C4E: move.b #$31, $70(a6)
pub fn i_035c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C54;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x31;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035C54: lea.l $279a3c.l, a0
pub fn i_035c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C5A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A3C;
    20
}
/// $035C5A: move.l -$4(a0), $74(a6)
pub fn i_035c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C60;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035C60: move.b #$ff, $21(a6)
pub fn i_035c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C66;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035C66: lea.l $279a3c.l, a0
pub fn i_035c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C6C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A3C;
    20
}
/// $035C6C: jsr $28cd4.l
pub fn i_035c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35C72);
    cpu.pc = 0x28CD4;
    16
}
/// $035C72: move.w #$fd00, $28(a6)
pub fn i_035c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C78;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035C78: clr.w $2c(a6)
pub fn i_035c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C7C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $035C7C: move.l #$32500, $60(a6)
pub fn i_035c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C84;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035C84: lea.l $326e0(pc), a0
pub fn i_035c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C88;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x326E0;
    16
}
/// $035C88: move.l a0, $48(a6)
pub fn i_035c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C8C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035C8C: lea.l $35c98(pc), a1
pub fn i_035c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35C98;
    16
}
/// $035C90: move.l a1, (a6)
pub fn i_035c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035C92: bset.b #$0, $3a(a6)
pub fn i_035c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C98;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== Player_TurnLeft_Run_035c98  ($035C98, 64 B, player/states_0342c4.s)
/// $035C98: jsr $32ff2(pc)
pub fn i_035c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35C9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35C9C);
    cpu.pc = 0x32FF2;
    16
}
/// $035C9C: jsr $32d32(pc)
pub fn i_035c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35CA0);
    cpu.pc = 0x32D32;
    16
}
/// $035CA0: bcc.w $35caa
pub fn i_035ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CA4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35CAA; return 10; }
    8
}
/// $035CA4: lea.l $345b8(pc), a1
pub fn i_035ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x345B8;
    16
}
/// $035CA8: move.l a1, (a6)
pub fn i_035ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035CAA: move.w #$fd00, $28(a6)
pub fn i_035caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CB0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035CB0: jsr $27a92.l
pub fn i_035cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35CB6);
    cpu.pc = 0x27A92;
    16
}
/// $035CB6: jsr $32aa8(pc)
pub fn i_035cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CBA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35CBA);
    cpu.pc = 0x32AA8;
    16
}
/// $035CBA: bcc.w $35cd4
pub fn i_035cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CBE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35CD4; return 10; }
    8
}
/// $035CBE: lea.l $34704(pc), a1
pub fn i_035cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CC2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $035CC2: move.l a1, (a6)
pub fn i_035cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CC4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035CC4: btst.b #$0, $88(a6)
pub fn i_035cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CCA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $035CCA: beq.w $35cd4
pub fn i_035cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CCE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x35CD4; return 10; }
    8
}
/// $035CCE: lea.l $33a5e(pc), a1
pub fn i_035cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $035CD2: move.l a1, (a6)
pub fn i_035cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035CD4: bra.w $35b90
pub fn i_035cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CD8;
    cpu.ir = 0x6000;
    cpu.pc = 0x35B90;
    10
}
// ===== Player_ActionDispatch_035cd8  ($035CD8, 92 B, player/states_0342c4.s)
/// $035CD8: jsr $330d0(pc)
pub fn i_035cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35CDC);
    cpu.pc = 0x330D0;
    16
}
/// $035CDC: bcc.w $35d32
pub fn i_035cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CE0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35D32; return 10; }
    8
}
/// $035CE0: cmpi.b #$ff, d1
pub fn i_035ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CE4;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $035CE4: bne.w $35cf2
pub fn i_035ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35CF2; return 10; }
    8
}
/// $035CE8: lea.l $35d34(pc), a1
pub fn i_035ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35D34;
    16
}
/// $035CEC: move.l a1, (a6)
pub fn i_035cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035CEE: bra.w $35d32
pub fn i_035cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CF2;
    cpu.ir = 0x6000;
    cpu.pc = 0x35D32;
    10
}
/// $035CF2: cmpi.b #$3, d1
pub fn i_035cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CF6;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $035CF6: bne.w $35d04
pub fn i_035cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CFA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35D04; return 10; }
    8
}
/// $035CFA: lea.l $360bc(pc), a1
pub fn i_035cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35CFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x360BC;
    16
}
/// $035CFE: move.l a1, (a6)
pub fn i_035cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035D00: bra.w $35d32
pub fn i_035d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D04;
    cpu.ir = 0x6000;
    cpu.pc = 0x35D32;
    10
}
/// $035D04: cmpi.b #$4, d1
pub fn i_035d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D08;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $035D08: bne.w $35d16
pub fn i_035d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D0C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35D16; return 10; }
    8
}
/// $035D0C: lea.l $3873c(pc), a1
pub fn i_035d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $035D10: move.l a1, (a6)
pub fn i_035d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035D12: bra.w $35d32
pub fn i_035d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D16;
    cpu.ir = 0x6000;
    cpu.pc = 0x35D32;
    10
}
/// $035D16: cmpi.b #$1, d1
pub fn i_035d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D1A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $035D1A: bne.w $35d28
pub fn i_035d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D1E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35D28; return 10; }
    8
}
/// $035D1E: lea.l $3437e(pc), a1
pub fn i_035d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3437E;
    16
}
/// $035D22: move.l a1, (a6)
pub fn i_035d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035D24: bra.w $35d32
pub fn i_035d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D28;
    cpu.ir = 0x6000;
    cpu.pc = 0x35D32;
    10
}
/// $035D28: lea.l $342c4(pc), a1
pub fn i_035d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x342C4;
    16
}
/// $035D2C: move.l a1, (a6)
pub fn i_035d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035D2E: bra.w $35d32
pub fn i_035d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D32;
    cpu.ir = 0x6000;
    cpu.pc = 0x35D32;
    10
}
/// $035D32: rts
pub fn i_035d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Melee_035d34  ($035D34, 172 B, player/states_0342c4.s)
/// $035D34: move.w #$0, $7c(a6)
pub fn i_035d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D3A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035D3A: move.w #$0, $7e(a6)
pub fn i_035d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D40;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035D40: bclr.b #$2, $8c(a6)
pub fn i_035d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D46;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035D46: bclr.b #$1, $8c(a6)
pub fn i_035d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D4C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035D4C: bclr.b #$3, $8c(a6)
pub fn i_035d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D52;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035D52: lea.l $32638(pc), a0
pub fn i_035d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D56;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32638;
    16
}
/// $035D56: move.l a0, $4c(a6)
pub fn i_035d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D5A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035D5A: jsr $283ca.l
pub fn i_035d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35D60);
    cpu.pc = 0x283CA;
    16
}
/// $035D60: clr.w $28(a6)
pub fn i_035d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D64;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $035D64: clr.w $2c(a6)
pub fn i_035d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D68;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $035D68: clr.w $2a(a6)
pub fn i_035d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D6C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $035D6C: clr.w $2e(a6)
pub fn i_035d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D70;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $035D70: move.l #$32500, $60(a6)
pub fn i_035d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D78;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035D78: jsr $2abcc.l
pub fn i_035d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35D7E);
    cpu.pc = 0x2ABCC;
    16
}
/// $035D7E: bcs.w $35daa
pub fn i_035d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D82;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x35DAA; return 10; }
    8
}
/// $035D82: move.b #$33, $70(a6)
pub fn i_035d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D88;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035D88: lea.l $279d56.l, a0
pub fn i_035d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D8E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D56;
    20
}
/// $035D8E: move.l -$4(a0), $74(a6)
pub fn i_035d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D94;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035D94: move.b #$ff, $21(a6)
pub fn i_035d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35D9A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035D9A: lea.l $279d56.l, a0
pub fn i_035d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D56;
    20
}
/// $035DA0: jsr $28cd4.l
pub fn i_035da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35DA6);
    cpu.pc = 0x28CD4;
    16
}
/// $035DA6: bra.w $35dce
pub fn i_035da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x35DCE;
    10
}
/// $035DAA: move.b #$33, $70(a6)
pub fn i_035daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DB0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035DB0: lea.l $279d56.l, a0
pub fn i_035db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DB6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D56;
    20
}
/// $035DB6: move.l -$4(a0), $74(a6)
pub fn i_035db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DBC;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035DBC: move.b #$ff, $21(a6)
pub fn i_035dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035DC2: lea.l $279d56.l, a0
pub fn i_035dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D56;
    20
}
/// $035DC8: jsr $28cd4.l
pub fn i_035dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35DCE);
    cpu.pc = 0x28CD4;
    16
}
/// $035DCE: lea.l $32788(pc), a0
pub fn i_035dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DD2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32788;
    16
}
/// $035DD2: move.l a0, $48(a6)
pub fn i_035dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DD6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035DD6: lea.l $35de0(pc), a1
pub fn i_035dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35DE0;
    16
}
/// $035DDA: move.l a1, (a6)
pub fn i_035dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035DDC: bra.w $35e54
pub fn i_035ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x35E54;
    10
}
// ===== Player_Melee_Run_035de0  ($035DE0, 200 B, player/states_0342c4.s)
/// $035DE0: clr.w d7
pub fn i_035de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DE2;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $035DE2: jsr $32e3c(pc)
pub fn i_035de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35DE6);
    cpu.pc = 0x32E3C;
    16
}
/// $035DE6: bcc.w $35dee
pub fn i_035de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35DEE; return 10; }
    8
}
/// $035DEA: move.w #$300, d7
pub fn i_035dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DEE;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $035DEE: jsr $32e42(pc)
pub fn i_035dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35DF2);
    cpu.pc = 0x32E42;
    16
}
/// $035DF2: bcc.w $35dfa
pub fn i_035df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35DFA; return 10; }
    8
}
/// $035DF6: move.w #$fd00, d7
pub fn i_035df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DFA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $035DFA: cmp.w $28(a6), d7
pub fn i_035dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35DFE;
    cpu.ir = 0xBE6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $035DFE: beq.w $35e50
pub fn i_035dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E02;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x35E50; return 10; }
    8
}
/// $035E02: cmpi.w #$0, d7
pub fn i_035e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E06;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $035E06: bne.w $35e20
pub fn i_035e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E0A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35E20; return 10; }
    8
}
/// $035E0A: lea.l $279d56.l, a0
pub fn i_035e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D56;
    20
}
/// $035E10: jsr $28cd4.l
pub fn i_035e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35E16);
    cpu.pc = 0x28CD4;
    16
}
/// $035E16: bclr.b #$5, $69(a6)
pub fn i_035e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E1C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035E1C: bra.w $35e50
pub fn i_035e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E20;
    cpu.ir = 0x6000;
    cpu.pc = 0x35E50;
    10
}
/// $035E20: cmp.w $28(a6), d7
pub fn i_035e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E24;
    cpu.ir = 0xBE6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $035E24: ble.w $35e3e
pub fn i_035e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E28;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x35E3E; return 10; }
    8
}
/// $035E28: lea.l $279d76.l, a0
pub fn i_035e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E2E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D76;
    20
}
/// $035E2E: jsr $28cd4.l
pub fn i_035e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35E34);
    cpu.pc = 0x28CD4;
    16
}
/// $035E34: bclr.b #$5, $69(a6)
pub fn i_035e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E3A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035E3A: bra.w $35e50
pub fn i_035e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x35E50;
    10
}
/// $035E3E: lea.l $279e30.l, a0
pub fn i_035e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279E30;
    20
}
/// $035E44: jsr $28cd4.l
pub fn i_035e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35E4A);
    cpu.pc = 0x28CD4;
    16
}
/// $035E4A: bclr.b #$5, $69(a6)
pub fn i_035e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E50;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035E50: move.w d7, $28(a6)
pub fn i_035e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E54;
    cpu.ir = 0x3D47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035E54: jsr $32ff2(pc)
pub fn i_035e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35E58);
    cpu.pc = 0x32FF2;
    16
}
/// $035E58: jsr $32d32(pc)
pub fn i_035e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35E5C);
    cpu.pc = 0x32D32;
    16
}
/// $035E5C: jsr $27a92.l
pub fn i_035e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35E62);
    cpu.pc = 0x27A92;
    16
}
/// $035E62: jsr $32aa8(pc)
pub fn i_035e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35E66);
    cpu.pc = 0x32AA8;
    16
}
/// $035E66: bcc.w $35ea0
pub fn i_035e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35EA0; return 10; }
    8
}
/// $035E6A: lea.l $34704(pc), a1
pub fn i_035e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $035E6E: move.l a1, (a6)
pub fn i_035e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035E70: move.b #$1, $78(a6)
pub fn i_035e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E76;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035E76: lsl.b #$4, d0
pub fn i_035e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E78;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $035E78: or.b $78(a6), d0
pub fn i_035e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E7C;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $035E7C: move.b d0, $79(a6)
pub fn i_035e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E80;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035E80: btst.b #$0, $88(a6)
pub fn i_035e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E86;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $035E86: beq.w $35e90
pub fn i_035e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E8A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x35E90; return 10; }
    8
}
/// $035E8A: lea.l $33a5e(pc), a1
pub fn i_035e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $035E8E: move.l a1, (a6)
pub fn i_035e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035E90: cmpi.w #$0, $82(a6)
pub fn i_035e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E96;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $035E96: bne.w $35ea0
pub fn i_035e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E9A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x35EA0; return 10; }
    8
}
/// $035E9A: lea.l $33d64(pc), a1
pub fn i_035e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35E9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $035E9E: move.l a1, (a6)
pub fn i_035e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035EA0: jsr $35cd8(pc)
pub fn i_035ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35EA4);
    cpu.pc = 0x35CD8;
    16
}
/// $035EA4: jmp $34ada(pc)
pub fn i_035ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EA8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x34ADA;
    8
}
// ===== Player_MeleeAlt_035ea8  ($035EA8, 200 B, player/states_0342c4.s)
/// $035EA8: move.w #$0, $7c(a6)
pub fn i_035ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EAE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035EAE: move.w #$0, $7e(a6)
pub fn i_035eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035EB4: bclr.b #$2, $8c(a6)
pub fn i_035eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EBA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035EBA: bclr.b #$1, $8c(a6)
pub fn i_035eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EC0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035EC0: bclr.b #$3, $8c(a6)
pub fn i_035ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EC6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035EC6: lea.l $32638(pc), a0
pub fn i_035ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35ECA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32638;
    16
}
/// $035ECA: move.l a0, $4c(a6)
pub fn i_035eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35ECE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035ECE: jsr $283ca.l
pub fn i_035ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35ED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35ED4);
    cpu.pc = 0x283CA;
    16
}
/// $035ED4: move.l #$32500, $60(a6)
pub fn i_035ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EDC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035EDC: jsr $2abcc.l
pub fn i_035edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35EE2);
    cpu.pc = 0x2ABCC;
    16
}
/// $035EE2: bcs.w $35f0e
pub fn i_035ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EE6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x35F0E; return 10; }
    8
}
/// $035EE6: move.b #$33, $70(a6)
pub fn i_035ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EEC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035EEC: lea.l $279efe.l, a0
pub fn i_035eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035EF2: move.l -$4(a0), $74(a6)
pub fn i_035ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EF8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035EF8: move.b #$ff, $21(a6)
pub fn i_035ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35EFE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035EFE: lea.l $279efe.l, a0
pub fn i_035efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F04;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035F04: jsr $28cd4.l
pub fn i_035f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35F0A);
    cpu.pc = 0x28CD4;
    16
}
/// $035F0A: bra.w $35f32
pub fn i_035f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F0E;
    cpu.ir = 0x6000;
    cpu.pc = 0x35F32;
    10
}
/// $035F0E: move.b #$33, $70(a6)
pub fn i_035f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F14;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035F14: lea.l $279efe.l, a0
pub fn i_035f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F1A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035F1A: move.l -$4(a0), $74(a6)
pub fn i_035f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F20;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035F20: move.b #$ff, $21(a6)
pub fn i_035f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F26;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035F26: lea.l $279efe.l, a0
pub fn i_035f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F2C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035F2C: jsr $28cd4.l
pub fn i_035f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35F32);
    cpu.pc = 0x28CD4;
    16
}
/// $035F32: lea.l $32788(pc), a0
pub fn i_035f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F36;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32788;
    16
}
/// $035F36: move.l a0, $48(a6)
pub fn i_035f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F3A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035F3A: lea.l $35f40(pc), a1
pub fn i_035f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35F40;
    16
}
/// $035F3E: move.l a1, (a6)
pub fn i_035f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035F40: jsr $32ff2(pc)
pub fn i_035f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35F44);
    cpu.pc = 0x32FF2;
    16
}
/// $035F44: jsr $32d32(pc)
pub fn i_035f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35F48);
    cpu.pc = 0x32D32;
    16
}
/// $035F48: jsr $27a92.l
pub fn i_035f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35F4E);
    cpu.pc = 0x27A92;
    16
}
/// $035F4E: jsr $32aa8(pc)
pub fn i_035f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x35F52);
    cpu.pc = 0x32AA8;
    16
}
/// $035F52: bcc.w $35f6c
pub fn i_035f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x35F6C; return 10; }
    8
}
/// $035F56: lea.l $34704(pc), a1
pub fn i_035f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $035F5A: move.l a1, (a6)
pub fn i_035f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $035F5C: move.b #$1, $78(a6)
pub fn i_035f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F62;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035F62: lsl.b #$4, d0
pub fn i_035f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F64;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $035F64: or.b $78(a6), d0
pub fn i_035f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F68;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $035F68: move.b d0, $79(a6)
pub fn i_035f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F6C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035F6C: jmp $34ccc(pc)
pub fn i_035f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F70;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x34CCC;
    8
}
// ===== Player_MeleeAlt_Walk_035f70  ($035F70, 332 B, player/states_0342c4.s)
/// $035F70: move.w #$0, $7c(a6)
pub fn i_035f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F76;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035F76: move.w #$ffff, $7e(a6)
pub fn i_035f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F7C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $035F7C: bclr.b #$2, $8c(a6)
pub fn i_035f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F82;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035F82: bclr.b #$1, $8c(a6)
pub fn i_035f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F88;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035F88: bclr.b #$3, $8c(a6)
pub fn i_035f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F8E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $035F8E: lea.l $32638(pc), a0
pub fn i_035f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F92;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32638;
    16
}
/// $035F92: move.l a0, $4c(a6)
pub fn i_035f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F96;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035F96: jsr $283ca.l
pub fn i_035f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35F9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35F9C);
    cpu.pc = 0x283CA;
    16
}
/// $035F9C: move.l #$32500, $60(a6)
pub fn i_035f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FA4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $035FA4: jsr $2abcc.l
pub fn i_035fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35FAA);
    cpu.pc = 0x2ABCC;
    16
}
/// $035FAA: bcs.w $35fd6
pub fn i_035faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FAE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x35FD6; return 10; }
    8
}
/// $035FAE: move.b #$33, $70(a6)
pub fn i_035fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FB4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035FB4: lea.l $279efe.l, a0
pub fn i_035fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FBA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035FBA: move.l -$4(a0), $74(a6)
pub fn i_035fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FC0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035FC0: move.b #$ff, $21(a6)
pub fn i_035fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FC6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035FC6: lea.l $279efe.l, a0
pub fn i_035fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FCC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035FCC: jsr $28cd4.l
pub fn i_035fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35FD2);
    cpu.pc = 0x28CD4;
    16
}
/// $035FD2: bra.w $35ffa
pub fn i_035fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FD6;
    cpu.ir = 0x6000;
    cpu.pc = 0x35FFA;
    10
}
/// $035FD6: move.b #$33, $70(a6)
pub fn i_035fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FDC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035FDC: lea.l $279efe.l, a0
pub fn i_035fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FE2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035FE2: move.l -$4(a0), $74(a6)
pub fn i_035fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FE8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $035FE8: move.b #$ff, $21(a6)
pub fn i_035fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FEE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $035FEE: lea.l $279efe.l, a0
pub fn i_035fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EFE;
    20
}
/// $035FF4: jsr $28cd4.l
pub fn i_035ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x35FFA);
    cpu.pc = 0x28CD4;
    16
}
/// $035FFA: lea.l $32788(pc), a0
pub fn i_035ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x35FFE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32788;
    16
}
/// $035FFE: move.l a0, $48(a6)
pub fn i_035ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36002;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036002: lea.l $36008(pc), a1
pub fn i_036002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36006;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36008;
    16
}
/// $036006: move.l a1, (a6)
pub fn i_036006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36008;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036008: jsr $32ff2(pc)
pub fn i_036008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3600C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3600C);
    cpu.pc = 0x32FF2;
    16
}
/// $03600C: jsr $32d32(pc)
pub fn i_03600c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36010;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36010);
    cpu.pc = 0x32D32;
    16
}
/// $036010: clr.w d7
pub fn i_036010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36012;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $036012: jsr $32e3c(pc)
pub fn i_036012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36016;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36016);
    cpu.pc = 0x32E3C;
    16
}
/// $036016: bcc.w $36024
pub fn i_036016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3601A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36024; return 10; }
    8
}
/// $03601A: bclr.b #$0, $3a(a6)
pub fn i_03601a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36020;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036020: move.w #$300, d7
pub fn i_036020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36024;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036024: bcc.w $36036
pub fn i_036024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36028;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36036; return 10; }
    8
}
/// $036028: jsr $32e42(pc)
pub fn i_036028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3602C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3602C);
    cpu.pc = 0x32E42;
    16
}
/// $03602C: bset.b #$0, $3a(a6)
pub fn i_03602c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36032;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036032: move.w #$fd00, d7
pub fn i_036032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36036;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $036036: move.w d7, $28(a6)
pub fn i_036036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3603A;
    cpu.ir = 0x3D47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03603A: jsr $27a92.l
pub fn i_03603a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36040;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36040);
    cpu.pc = 0x27A92;
    16
}
/// $036040: jsr $32aa8(pc)
pub fn i_036040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36044;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36044);
    cpu.pc = 0x32AA8;
    16
}
/// $036044: bcc.w $3605e
pub fn i_036044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36048;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3605E; return 10; }
    8
}
/// $036048: lea.l $34704(pc), a1
pub fn i_036048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3604C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $03604C: move.l a1, (a6)
pub fn i_03604c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3604E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03604E: move.b #$1, $78(a6)
pub fn i_03604e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36054;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036054: lsl.b #$4, d0
pub fn i_036054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36056;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $036056: or.b $78(a6), d0
pub fn i_036056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3605A;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03605A: move.b d0, $79(a6)
pub fn i_03605a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3605E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03605E: jsr $330d0(pc)
pub fn i_03605e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36062;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36062);
    cpu.pc = 0x330D0;
    16
}
/// $036062: bcc.w $360b8
pub fn i_036062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36066;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x360B8; return 10; }
    8
}
/// $036066: cmpi.b #$ff, d1
pub fn i_036066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3606A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $03606A: bne.w $36078
pub fn i_03606a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3606E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36078; return 10; }
    8
}
/// $03606E: lea.l $35d34(pc), a1
pub fn i_03606e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36072;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35D34;
    16
}
/// $036072: move.l a1, (a6)
pub fn i_036072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36074;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036074: bra.w $360b8
pub fn i_036074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36078;
    cpu.ir = 0x6000;
    cpu.pc = 0x360B8;
    10
}
/// $036078: cmpi.b #$3, d1
pub fn i_036078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3607C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $03607C: bne.w $3608a
pub fn i_03607c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36080;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3608A; return 10; }
    8
}
/// $036080: lea.l $360bc(pc), a1
pub fn i_036080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36084;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x360BC;
    16
}
/// $036084: move.l a1, (a6)
pub fn i_036084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36086;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036086: bra.w $360b8
pub fn i_036086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3608A;
    cpu.ir = 0x6000;
    cpu.pc = 0x360B8;
    10
}
/// $03608A: cmpi.b #$4, d1
pub fn i_03608a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3608E;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $03608E: bne.w $3609c
pub fn i_03608e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36092;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3609C; return 10; }
    8
}
/// $036092: lea.l $3873c(pc), a1
pub fn i_036092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36096;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $036096: move.l a1, (a6)
pub fn i_036096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36098;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036098: bra.w $360b8
pub fn i_036098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3609C;
    cpu.ir = 0x6000;
    cpu.pc = 0x360B8;
    10
}
/// $03609C: cmpi.b #$1, d1
pub fn i_03609c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360A0;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $0360A0: bne.w $360ae
pub fn i_0360a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x360AE; return 10; }
    8
}
/// $0360A4: lea.l $357be(pc), a1
pub fn i_0360a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x357BE;
    16
}
/// $0360A8: move.l a1, (a6)
pub fn i_0360a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0360AA: bra.w $360b8
pub fn i_0360aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x360B8;
    10
}
/// $0360AE: lea.l $357aa(pc), a1
pub fn i_0360ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x357AA;
    16
}
/// $0360B2: move.l a1, (a6)
pub fn i_0360b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0360B4: bra.w $360b8
pub fn i_0360b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x360B8;
    10
}
/// $0360B8: jmp $3548c(pc)
pub fn i_0360b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360BC;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3548C;
    8
}
// ===== Player_ThrowGrenade_Stand_0360bc  ($0360BC, 342 B, player/states_0342c4.s)
/// $0360BC: move.w #$0, $7c(a6)
pub fn i_0360bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360C2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0360C2: move.w #$0, $7e(a6)
pub fn i_0360c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0360C8: bclr.b #$2, $8c(a6)
pub fn i_0360c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360CE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0360CE: bclr.b #$1, $8c(a6)
pub fn i_0360ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360D4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0360D4: bclr.b #$3, $8c(a6)
pub fn i_0360d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360DA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0360DA: clr.w $28(a6)
pub fn i_0360da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360DE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0360DE: clr.w $2c(a6)
pub fn i_0360de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360E2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0360E2: clr.w $2a(a6)
pub fn i_0360e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360E6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0360E6: clr.w $2e(a6)
pub fn i_0360e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360EA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0360EA: move.l #$32500, $60(a6)
pub fn i_0360ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360F2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0360F2: jsr $2abcc.l
pub fn i_0360f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x360F8);
    cpu.pc = 0x2ABCC;
    16
}
/// $0360F8: bcs.w $36124
pub fn i_0360f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x360FC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x36124; return 10; }
    8
}
/// $0360FC: move.b #$33, $70(a6)
pub fn i_0360fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36102;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036102: lea.l $279d2e.l, a0
pub fn i_036102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36108;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D2E;
    20
}
/// $036108: move.l -$4(a0), $74(a6)
pub fn i_036108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3610E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03610E: move.b #$ff, $21(a6)
pub fn i_03610e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36114;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036114: lea.l $279d2e.l, a0
pub fn i_036114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3611A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D2E;
    20
}
/// $03611A: jsr $28cd4.l
pub fn i_03611a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36120;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36120);
    cpu.pc = 0x28CD4;
    16
}
/// $036120: bra.w $36148
pub fn i_036120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36124;
    cpu.ir = 0x6000;
    cpu.pc = 0x36148;
    10
}
/// $036124: move.b #$33, $70(a6)
pub fn i_036124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3612A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03612A: lea.l $279d2e.l, a0
pub fn i_03612a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36130;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D2E;
    20
}
/// $036130: move.l -$4(a0), $74(a6)
pub fn i_036130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36136;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036136: move.b #$ff, $21(a6)
pub fn i_036136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3613C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03613C: lea.l $279d2e.l, a0
pub fn i_03613c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36142;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D2E;
    20
}
/// $036142: jsr $28cd4.l
pub fn i_036142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36148;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36148);
    cpu.pc = 0x28CD4;
    16
}
/// $036148: clr.b $3b(a6)
pub fn i_036148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3614C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03614C: lea.l $36152(pc), a1
pub fn i_03614c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36150;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36152;
    16
}
/// $036150: move.l a1, (a6)
pub fn i_036150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36152;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036152: jsr $32ff2(pc)
pub fn i_036152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36156;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36156);
    cpu.pc = 0x32FF2;
    16
}
/// $036156: jsr $32d32(pc)
pub fn i_036156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3615A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3615A);
    cpu.pc = 0x32D32;
    16
}
/// $03615A: jsr $27a92.l
pub fn i_03615a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36160;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36160);
    cpu.pc = 0x27A92;
    16
}
/// $036160: jsr $32aa8(pc)
pub fn i_036160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36164;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36164);
    cpu.pc = 0x32AA8;
    16
}
/// $036164: bcc.w $3619e
pub fn i_036164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36168;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3619E; return 10; }
    8
}
/// $036168: lea.l $34704(pc), a1
pub fn i_036168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3616C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $03616C: move.l a1, (a6)
pub fn i_03616c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3616E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03616E: move.b #$1, $78(a6)
pub fn i_03616e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36174;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036174: lsl.b #$4, d0
pub fn i_036174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36176;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $036176: or.b $78(a6), d0
pub fn i_036176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3617A;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03617A: move.b d0, $79(a6)
pub fn i_03617a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3617E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03617E: btst.b #$0, $88(a6)
pub fn i_03617e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36184;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $036184: beq.w $3618e
pub fn i_036184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36188;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3618E; return 10; }
    8
}
/// $036188: lea.l $33a5e(pc), a1
pub fn i_036188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3618C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $03618C: move.l a1, (a6)
pub fn i_03618c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3618E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03618E: cmpi.w #$0, $82(a6)
pub fn i_03618e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36194;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $036194: bne.w $3619e
pub fn i_036194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36198;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3619E; return 10; }
    8
}
/// $036198: lea.l $33d64(pc), a1
pub fn i_036198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3619C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $03619C: move.l a1, (a6)
pub fn i_03619c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3619E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03619E: btst.b #$2, $8c(a6)
pub fn i_03619e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361A4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0361A4: bne.w $3620a
pub fn i_0361a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3620A; return 10; }
    8
}
/// $0361A8: btst.b #$0, $3a(a6)
pub fn i_0361a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361AE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0361AE: bne.w $361d2
pub fn i_0361ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x361D2; return 10; }
    8
}
/// $0361B2: jsr $32e3c(pc)
pub fn i_0361b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x361B6);
    cpu.pc = 0x32E3C;
    16
}
/// $0361B6: bcc.w $361c0
pub fn i_0361b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x361C0; return 10; }
    8
}
/// $0361BA: lea.l $34b38(pc), a1
pub fn i_0361ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34B38;
    16
}
/// $0361BE: move.l a1, (a6)
pub fn i_0361be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0361C0: jsr $32e42(pc)
pub fn i_0361c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x361C4);
    cpu.pc = 0x32E42;
    16
}
/// $0361C4: bcc.w $361ce
pub fn i_0361c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x361CE; return 10; }
    8
}
/// $0361C8: lea.l $35bf8(pc), a1
pub fn i_0361c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35BF8;
    16
}
/// $0361CC: move.l a1, (a6)
pub fn i_0361cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0361CE: bra.w $361ee
pub fn i_0361ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x361EE;
    10
}
/// $0361D2: jsr $32e3c(pc)
pub fn i_0361d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x361D6);
    cpu.pc = 0x32E3C;
    16
}
/// $0361D6: bcc.w $361e0
pub fn i_0361d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x361E0; return 10; }
    8
}
/// $0361DA: lea.l $35aba(pc), a1
pub fn i_0361da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35ABA;
    16
}
/// $0361DE: move.l a1, (a6)
pub fn i_0361de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0361E0: jsr $32e42(pc)
pub fn i_0361e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x361E4);
    cpu.pc = 0x32E42;
    16
}
/// $0361E4: bcc.w $361ee
pub fn i_0361e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x361EE; return 10; }
    8
}
/// $0361E8: lea.l $34d32(pc), a1
pub fn i_0361e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34D32;
    16
}
/// $0361EC: move.l a1, (a6)
pub fn i_0361ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0361EE: jsr $32e90(pc)
pub fn i_0361ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x361F2);
    cpu.pc = 0x32E90;
    16
}
/// $0361F2: bcc.w $361fc
pub fn i_0361f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x361FC; return 10; }
    8
}
/// $0361F6: lea.l $37c74(pc), a1
pub fn i_0361f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C74;
    16
}
/// $0361FA: move.l a1, (a6)
pub fn i_0361fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x361FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0361FC: jsr $33034(pc)
pub fn i_0361fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36200;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x36200);
    cpu.pc = 0x33034;
    16
}
/// $036200: bcc.w $3620a
pub fn i_036200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36204;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3620A; return 10; }
    8
}
/// $036204: lea.l $36914(pc), a1
pub fn i_036204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36208;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $036208: move.l a1, (a6)
pub fn i_036208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3620A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03620A: jsr $35cd8(pc)
pub fn i_03620a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3620E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3620E);
    cpu.pc = 0x35CD8;
    16
}
/// $03620E: jmp $34ada(pc)
pub fn i_03620e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36212;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x34ADA;
    8
}
// ===== Player_ThrowGrenade_Walk_036212  ($036212, 262 B, player/states_0342c4.s)
/// $036212: move.w #$0, $7c(a6)
pub fn i_036212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36218;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036218: move.w #$0, $7e(a6)
pub fn i_036218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3621E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03621E: bclr.b #$2, $8c(a6)
pub fn i_03621e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36224;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036224: bclr.b #$1, $8c(a6)
pub fn i_036224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3622A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03622A: bclr.b #$3, $8c(a6)
pub fn i_03622a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36230;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036230: move.l #$32500, $60(a6)
pub fn i_036230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36238;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $036238: jsr $2abcc.l
pub fn i_036238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3623E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3623E);
    cpu.pc = 0x2ABCC;
    16
}
/// $03623E: bcs.w $3626a
pub fn i_03623e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36242;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3626A; return 10; }
    8
}
/// $036242: move.b #$33, $70(a6)
pub fn i_036242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36248;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036248: lea.l $279d38.l, a0
pub fn i_036248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3624E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $03624E: move.l -$4(a0), $74(a6)
pub fn i_03624e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36254;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036254: move.b #$ff, $21(a6)
pub fn i_036254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3625A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03625A: lea.l $279d38.l, a0
pub fn i_03625a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36260;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $036260: jsr $28cd4.l
pub fn i_036260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36266;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36266);
    cpu.pc = 0x28CD4;
    16
}
/// $036266: bra.w $3628e
pub fn i_036266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3626A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3628E;
    10
}
/// $03626A: move.b #$33, $70(a6)
pub fn i_03626a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36270;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036270: lea.l $279d38.l, a0
pub fn i_036270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36276;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $036276: move.l -$4(a0), $74(a6)
pub fn i_036276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3627C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03627C: move.b #$ff, $21(a6)
pub fn i_03627c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36282;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036282: lea.l $279d38.l, a0
pub fn i_036282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36288;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $036288: jsr $28cd4.l
pub fn i_036288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3628E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3628E);
    cpu.pc = 0x28CD4;
    16
}
/// $03628E: clr.b $3b(a6)
pub fn i_03628e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36292;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $036292: lea.l $36298(pc), a1
pub fn i_036292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36296;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36298;
    16
}
/// $036296: move.l a1, (a6)
pub fn i_036296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36298;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036298: jsr $32ff2(pc)
pub fn i_036298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3629C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3629C);
    cpu.pc = 0x32FF2;
    16
}
/// $03629C: jsr $32d32(pc)
pub fn i_03629c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x362A0);
    cpu.pc = 0x32D32;
    16
}
/// $0362A0: jsr $27a92.l
pub fn i_0362a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x362A6);
    cpu.pc = 0x27A92;
    16
}
/// $0362A6: jsr $32aa8(pc)
pub fn i_0362a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x362AA);
    cpu.pc = 0x32AA8;
    16
}
/// $0362AA: bcc.w $362c4
pub fn i_0362aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x362C4; return 10; }
    8
}
/// $0362AE: lea.l $34704(pc), a1
pub fn i_0362ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $0362B2: move.l a1, (a6)
pub fn i_0362b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0362B4: move.b #$1, $78(a6)
pub fn i_0362b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362BA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0362BA: lsl.b #$4, d0
pub fn i_0362ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362BC;
    cpu.ir = 0xE908;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $0362BC: or.b $78(a6), d0
pub fn i_0362bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362C0;
    cpu.ir = 0x802E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0362C0: move.b d0, $79(a6)
pub fn i_0362c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362C4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0362C4: btst.b #$2, $8c(a6)
pub fn i_0362c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362CA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0362CA: bne.w $36314
pub fn i_0362ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x36314; return 10; }
    8
}
/// $0362CE: btst.b #$0, $3a(a6)
pub fn i_0362ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362D4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0362D4: bne.w $362ea
pub fn i_0362d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x362EA; return 10; }
    8
}
/// $0362D8: jsr $32e42(pc)
pub fn i_0362d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x362DC);
    cpu.pc = 0x32E42;
    16
}
/// $0362DC: bcc.w $362e6
pub fn i_0362dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x362E6; return 10; }
    8
}
/// $0362E0: lea.l $35bf8(pc), a1
pub fn i_0362e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35BF8;
    16
}
/// $0362E4: move.l a1, (a6)
pub fn i_0362e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0362E6: bra.w $362f8
pub fn i_0362e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x362F8;
    10
}
/// $0362EA: jsr $32e3c(pc)
pub fn i_0362ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x362EE);
    cpu.pc = 0x32E3C;
    16
}
/// $0362EE: bcc.w $362f8
pub fn i_0362ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x362F8; return 10; }
    8
}
/// $0362F2: lea.l $35aba(pc), a1
pub fn i_0362f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35ABA;
    16
}
/// $0362F6: move.l a1, (a6)
pub fn i_0362f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0362F8: jsr $32e90(pc)
pub fn i_0362f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x362FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x362FC);
    cpu.pc = 0x32E90;
    16
}
/// $0362FC: bcc.w $36306
pub fn i_0362fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36300;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36306; return 10; }
    8
}
/// $036300: lea.l $37c74(pc), a1
pub fn i_036300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36304;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C74;
    16
}
/// $036304: move.l a1, (a6)
pub fn i_036304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36306;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036306: jsr $33034(pc)
pub fn i_036306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3630A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3630A);
    cpu.pc = 0x33034;
    16
}
/// $03630A: bcc.w $36314
pub fn i_03630a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3630E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x36314; return 10; }
    8
}
/// $03630E: lea.l $36914(pc), a1
pub fn i_03630e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36312;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $036312: move.l a1, (a6)
pub fn i_036312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36314;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $036314: jmp $34ccc(pc)
pub fn i_036314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36318;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x34CCC;
    8
}
// ===== Player_ThrowGrenade_WalkLoop_036318  ($036318, 394 B, player/states_0342c4.s)
/// $036318: move.w #$0, $7c(a6)
pub fn i_036318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3631E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03631E: move.w #$ffff, $7e(a6)
pub fn i_03631e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36324;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $036324: bclr.b #$2, $8c(a6)
pub fn i_036324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3632A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03632A: bclr.b #$1, $8c(a6)
pub fn i_03632a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36330;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036330: bclr.b #$3, $8c(a6)
pub fn i_036330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36336;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $036336: move.l #$32500, $60(a6)
pub fn i_036336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3633E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03633E: jsr $2abcc.l
pub fn i_03633e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36344;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36344);
    cpu.pc = 0x2ABCC;
    16
}
/// $036344: bcs.w $36370
pub fn i_036344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36348;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x36370; return 10; }
    8
}
/// $036348: move.b #$33, $70(a6)
pub fn i_036348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3634E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03634E: lea.l $279d38.l, a0
pub fn i_03634e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36354;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $036354: move.l -$4(a0), $74(a6)
pub fn i_036354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3635A;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03635A: move.b #$ff, $21(a6)
pub fn i_03635a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36360;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036360: lea.l $279d38.l, a0
pub fn i_036360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36366;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $036366: jsr $28cd4.l
pub fn i_036366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3636C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3636C);
    cpu.pc = 0x28CD4;
    16
}
/// $03636C: bra.w $36394
pub fn i_03636c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36370;
    cpu.ir = 0x6000;
    cpu.pc = 0x36394;
    10
}
/// $036370: move.b #$33, $70(a6)
pub fn i_036370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36376;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036376: lea.l $279d38.l, a0
pub fn i_036376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3637C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $03637C: move.l -$4(a0), $74(a6)
pub fn i_03637c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36382;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $036382: move.b #$ff, $21(a6)
pub fn i_036382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36388;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $036388: lea.l $279d38.l, a0
pub fn i_036388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3638E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D38;
    20
}
/// $03638E: jsr $28cd4.l
pub fn i_03638e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36394;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x36394);
    cpu.pc = 0x28CD4;
    16
}
/// $036394: clr.b $3b(a6)
pub fn i_036394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x36398;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $036398: lea.l $3639e(pc), a1
pub fn i_036398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3639C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3639E;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x034642, i_034642),
    (0x034646, i_034646),
    (0x03464C, i_03464c),
    (0x034650, i_034650),
    (0x034654, i_034654),
    (0x034658, i_034658),
    (0x03465A, i_03465a),
    (0x034660, i_034660),
    (0x034664, i_034664),
    (0x03466C, i_03466c),
    (0x034670, i_034670),
    (0x034676, i_034676),
    (0x03467C, i_03467c),
    (0x034680, i_034680),
    (0x034686, i_034686),
    (0x03468C, i_03468c),
    (0x034692, i_034692),
    (0x034696, i_034696),
    (0x03469C, i_03469c),
    (0x0346A2, i_0346a2),
    (0x0346A8, i_0346a8),
    (0x0346AC, i_0346ac),
    (0x0346B4, i_0346b4),
    (0x0346B8, i_0346b8),
    (0x0346BE, i_0346be),
    (0x0346C4, i_0346c4),
    (0x0346C8, i_0346c8),
    (0x0346CE, i_0346ce),
    (0x0346D4, i_0346d4),
    (0x0346DA, i_0346da),
    (0x0346DE, i_0346de),
    (0x0346E4, i_0346e4),
    (0x0346EA, i_0346ea),
    (0x0346F0, i_0346f0),
    (0x0346F6, i_0346f6),
    (0x0346FA, i_0346fa),
    (0x0346FE, i_0346fe),
    (0x034700, i_034700),
    (0x034704, i_034704),
    (0x03470A, i_03470a),
    (0x03470E, i_03470e),
    (0x034714, i_034714),
    (0x034718, i_034718),
    (0x03471E, i_03471e),
    (0x034722, i_034722),
    (0x034728, i_034728),
    (0x03472E, i_03472e),
    (0x034734, i_034734),
    (0x03473A, i_03473a),
    (0x03473E, i_03473e),
    (0x034744, i_034744),
    (0x03474A, i_03474a),
    (0x034750, i_034750),
    (0x034756, i_034756),
    (0x03475C, i_03475c),
    (0x034760, i_034760),
    (0x034766, i_034766),
    (0x03476C, i_03476c),
    (0x034772, i_034772),
    (0x034778, i_034778),
    (0x03477C, i_03477c),
    (0x034782, i_034782),
    (0x034788, i_034788),
    (0x03478E, i_03478e),
    (0x034794, i_034794),
    (0x03479A, i_03479a),
    (0x0347A0, i_0347a0),
    (0x0347A6, i_0347a6),
    (0x0347AC, i_0347ac),
    (0x0347B0, i_0347b0),
    (0x0347B6, i_0347b6),
    (0x0347BC, i_0347bc),
    (0x0347C2, i_0347c2),
    (0x0347C8, i_0347c8),
    (0x0347CE, i_0347ce),
    (0x0347D4, i_0347d4),
    (0x0347DA, i_0347da),
    (0x0347E0, i_0347e0),
    (0x0347E6, i_0347e6),
    (0x0347EE, i_0347ee),
    (0x0347F4, i_0347f4),
    (0x0347F8, i_0347f8),
    (0x0347FC, i_0347fc),
    (0x034800, i_034800),
    (0x034804, i_034804),
    (0x034806, i_034806),
    (0x03480A, i_03480a),
    (0x03480E, i_03480e),
    (0x034812, i_034812),
    (0x034816, i_034816),
    (0x034818, i_034818),
    (0x03481E, i_03481e),
    (0x034822, i_034822),
    (0x034826, i_034826),
    (0x03482C, i_03482c),
    (0x034832, i_034832),
    (0x034838, i_034838),
    (0x03483E, i_03483e),
    (0x034844, i_034844),
    (0x03484A, i_03484a),
    (0x03484E, i_03484e),
    (0x034854, i_034854),
    (0x03485A, i_03485a),
    (0x03485E, i_03485e),
    (0x034864, i_034864),
    (0x03486A, i_03486a),
    (0x034870, i_034870),
    (0x034874, i_034874),
    (0x03487A, i_03487a),
    (0x034880, i_034880),
    (0x034886, i_034886),
    (0x03488C, i_03488c),
    (0x034890, i_034890),
    (0x034896, i_034896),
    (0x03489C, i_03489c),
    (0x0348A0, i_0348a0),
    (0x0348A6, i_0348a6),
    (0x0348AC, i_0348ac),
    (0x0348B2, i_0348b2),
    (0x0348B8, i_0348b8),
    (0x0348BE, i_0348be),
    (0x0348C4, i_0348c4),
    (0x0348C8, i_0348c8),
    (0x0348CE, i_0348ce),
    (0x0348D4, i_0348d4),
    (0x0348DA, i_0348da),
    (0x0348E0, i_0348e0),
    (0x0348E6, i_0348e6),
    (0x0348EC, i_0348ec),
    (0x0348F0, i_0348f0),
    (0x0348F6, i_0348f6),
    (0x0348FA, i_0348fa),
    (0x0348FE, i_0348fe),
    (0x034900, i_034900),
    (0x034906, i_034906),
    (0x03490A, i_03490a),
    (0x034910, i_034910),
    (0x034914, i_034914),
    (0x034918, i_034918),
    (0x03491A, i_03491a),
    (0x034920, i_034920),
    (0x034924, i_034924),
    (0x034926, i_034926),
    (0x03492A, i_03492a),
    (0x03492E, i_03492e),
    (0x034932, i_034932),
    (0x034934, i_034934),
    (0x03493A, i_03493a),
    (0x03493E, i_03493e),
    (0x034942, i_034942),
    (0x034944, i_034944),
    (0x03494A, i_03494a),
    (0x03494E, i_03494e),
    (0x034954, i_034954),
    (0x034958, i_034958),
    (0x03495E, i_03495e),
    (0x034962, i_034962),
    (0x034968, i_034968),
    (0x03496E, i_03496e),
    (0x034972, i_034972),
    (0x034978, i_034978),
    (0x03497E, i_03497e),
    (0x034984, i_034984),
    (0x034988, i_034988),
    (0x03498E, i_03498e),
    (0x034994, i_034994),
    (0x03499A, i_03499a),
    (0x0349A0, i_0349a0),
    (0x0349A4, i_0349a4),
    (0x0349AA, i_0349aa),
    (0x0349AE, i_0349ae),
    (0x0349B4, i_0349b4),
    (0x0349B8, i_0349b8),
    (0x0349BE, i_0349be),
    (0x0349C4, i_0349c4),
    (0x0349C8, i_0349c8),
    (0x0349CE, i_0349ce),
    (0x0349D4, i_0349d4),
    (0x0349DA, i_0349da),
    (0x0349DE, i_0349de),
    (0x0349E4, i_0349e4),
    (0x0349EA, i_0349ea),
    (0x0349F0, i_0349f0),
    (0x0349F6, i_0349f6),
    (0x0349FA, i_0349fa),
    (0x034A00, i_034a00),
    (0x034A06, i_034a06),
    (0x034A0A, i_034a0a),
    (0x034A10, i_034a10),
    (0x034A14, i_034a14),
    (0x034A18, i_034a18),
    (0x034A1C, i_034a1c),
    (0x034A20, i_034a20),
    (0x034A22, i_034a22),
    (0x034A26, i_034a26),
    (0x034A2A, i_034a2a),
    (0x034A2E, i_034a2e),
    (0x034A30, i_034a30),
    (0x034A34, i_034a34),
    (0x034A38, i_034a38),
    (0x034A3C, i_034a3c),
    (0x034A40, i_034a40),
    (0x034A42, i_034a42),
    (0x034A46, i_034a46),
    (0x034A4A, i_034a4a),
    (0x034A4E, i_034a4e),
    (0x034A50, i_034a50),
    (0x034A54, i_034a54),
    (0x034A58, i_034a58),
    (0x034A5C, i_034a5c),
    (0x034A5E, i_034a5e),
    (0x034A64, i_034a64),
    (0x034A68, i_034a68),
    (0x034A6C, i_034a6c),
    (0x034A72, i_034a72),
    (0x034A76, i_034a76),
    (0x034A7A, i_034a7a),
    (0x034A7E, i_034a7e),
    (0x034A84, i_034a84),
    (0x034A88, i_034a88),
    (0x034A8C, i_034a8c),
    (0x034A90, i_034a90),
    (0x034A94, i_034a94),
    (0x034A96, i_034a96),
    (0x034A9A, i_034a9a),
    (0x034A9E, i_034a9e),
    (0x034AA2, i_034aa2),
    (0x034AA6, i_034aa6),
    (0x034AA8, i_034aa8),
    (0x034AAC, i_034aac),
    (0x034AB0, i_034ab0),
    (0x034AB4, i_034ab4),
    (0x034AB8, i_034ab8),
    (0x034ABA, i_034aba),
    (0x034ABE, i_034abe),
    (0x034AC2, i_034ac2),
    (0x034AC6, i_034ac6),
    (0x034ACA, i_034aca),
    (0x034ACC, i_034acc),
    (0x034AD0, i_034ad0),
    (0x034AD4, i_034ad4),
    (0x034AD6, i_034ad6),
    (0x034ADA, i_034ada),
    (0x034AE0, i_034ae0),
    (0x034AE4, i_034ae4),
    (0x034AE8, i_034ae8),
    (0x034AEA, i_034aea),
    (0x034AEE, i_034aee),
    (0x034AF2, i_034af2),
    (0x034AF6, i_034af6),
    (0x034AF8, i_034af8),
    (0x034AFC, i_034afc),
    (0x034B04, i_034b04),
    (0x034B08, i_034b08),
    (0x034B0E, i_034b0e),
    (0x034B12, i_034b12),
    (0x034B18, i_034b18),
    (0x034B1C, i_034b1c),
    (0x034B22, i_034b22),
    (0x034B26, i_034b26),
    (0x034B2C, i_034b2c),
    (0x034B30, i_034b30),
    (0x034B34, i_034b34),
    (0x034B36, i_034b36),
    (0x034B38, i_034b38),
    (0x034B3E, i_034b3e),
    (0x034B44, i_034b44),
    (0x034B4A, i_034b4a),
    (0x034B4E, i_034b4e),
    (0x034B54, i_034b54),
    (0x034B58, i_034b58),
    (0x034B5E, i_034b5e),
    (0x034B64, i_034b64),
    (0x034B6A, i_034b6a),
    (0x034B6E, i_034b6e),
    (0x034B74, i_034b74),
    (0x034B7A, i_034b7a),
    (0x034B80, i_034b80),
    (0x034B86, i_034b86),
    (0x034B8C, i_034b8c),
    (0x034B92, i_034b92),
    (0x034B98, i_034b98),
    (0x034B9E, i_034b9e),
    (0x034BA4, i_034ba4),
    (0x034BAA, i_034baa),
    (0x034BAE, i_034bae),
    (0x034BB6, i_034bb6),
    (0x034BBA, i_034bba),
    (0x034BBE, i_034bbe),
    (0x034BC4, i_034bc4),
    (0x034BC8, i_034bc8),
    (0x034BCA, i_034bca),
    (0x034BCE, i_034bce),
    (0x034BD2, i_034bd2),
    (0x034BD6, i_034bd6),
    (0x034BDA, i_034bda),
    (0x034BDC, i_034bdc),
    (0x034BE2, i_034be2),
    (0x034BE8, i_034be8),
    (0x034BEC, i_034bec),
    (0x034BF0, i_034bf0),
    (0x034BF4, i_034bf4),
    (0x034BF6, i_034bf6),
    (0x034BFC, i_034bfc),
    (0x034C00, i_034c00),
    (0x034C08, i_034c08),
    (0x034C0C, i_034c0c),
    (0x034C12, i_034c12),
    (0x034C18, i_034c18),
    (0x034C1E, i_034c1e),
    (0x034C24, i_034c24),
    (0x034C28, i_034c28),
    (0x034C30, i_034c30),
    (0x034C34, i_034c34),
    (0x034C3A, i_034c3a),
    (0x034C40, i_034c40),
    (0x034C46, i_034c46),
    (0x034C4C, i_034c4c),
    (0x034C52, i_034c52),
    (0x034C56, i_034c56),
    (0x034C5A, i_034c5a),
    (0x034C5E, i_034c5e),
    (0x034C62, i_034c62),
    (0x034C64, i_034c64),
    (0x034C68, i_034c68),
    (0x034C6C, i_034c6c),
    (0x034C70, i_034c70),
    (0x034C72, i_034c72),
    (0x034C76, i_034c76),
    (0x034C7A, i_034c7a),
    (0x034C7E, i_034c7e),
    (0x034C82, i_034c82),
    (0x034C86, i_034c86),
    (0x034C88, i_034c88),
    (0x034C8C, i_034c8c),
    (0x034C90, i_034c90),
    (0x034C94, i_034c94),
    (0x034C98, i_034c98),
    (0x034C9A, i_034c9a),
    (0x034C9E, i_034c9e),
    (0x034CA2, i_034ca2),
    (0x034CA6, i_034ca6),
    (0x034CAA, i_034caa),
    (0x034CAC, i_034cac),
    (0x034CB0, i_034cb0),
    (0x034CB4, i_034cb4),
    (0x034CB8, i_034cb8),
    (0x034CBC, i_034cbc),
    (0x034CBE, i_034cbe),
    (0x034CC2, i_034cc2),
    (0x034CC6, i_034cc6),
    (0x034CC8, i_034cc8),
    (0x034CCC, i_034ccc),
    (0x034CD0, i_034cd0),
    (0x034CD4, i_034cd4),
    (0x034CD8, i_034cd8),
    (0x034CDA, i_034cda),
    (0x034CE0, i_034ce0),
    (0x034CE4, i_034ce4),
    (0x034CEA, i_034cea),
    (0x034CEE, i_034cee),
    (0x034CF2, i_034cf2),
    (0x034CF4, i_034cf4),
    (0x034CFA, i_034cfa),
    (0x034CFE, i_034cfe),
    (0x034D04, i_034d04),
    (0x034D08, i_034d08),
    (0x034D0C, i_034d0c),
    (0x034D0E, i_034d0e),
    (0x034D14, i_034d14),
    (0x034D18, i_034d18),
    (0x034D1C, i_034d1c),
    (0x034D1E, i_034d1e),
    (0x034D22, i_034d22),
    (0x034D26, i_034d26),
    (0x034D2A, i_034d2a),
    (0x034D2C, i_034d2c),
    (0x034D30, i_034d30),
    (0x034D32, i_034d32),
    (0x034D38, i_034d38),
    (0x034D3E, i_034d3e),
    (0x034D44, i_034d44),
    (0x034D48, i_034d48),
    (0x034D4E, i_034d4e),
    (0x034D52, i_034d52),
    (0x034D58, i_034d58),
    (0x034D5E, i_034d5e),
    (0x034D64, i_034d64),
    (0x034D68, i_034d68),
    (0x034D6E, i_034d6e),
    (0x034D74, i_034d74),
    (0x034D7A, i_034d7a),
    (0x034D80, i_034d80),
    (0x034D86, i_034d86),
    (0x034D8C, i_034d8c),
    (0x034D92, i_034d92),
    (0x034D98, i_034d98),
    (0x034D9E, i_034d9e),
    (0x034DA4, i_034da4),
    (0x034DA8, i_034da8),
    (0x034DB0, i_034db0),
    (0x034DB4, i_034db4),
    (0x034DB8, i_034db8),
    (0x034DBE, i_034dbe),
    (0x034DC2, i_034dc2),
    (0x034DC4, i_034dc4),
    (0x034DC8, i_034dc8),
    (0x034DCC, i_034dcc),
    (0x034DD0, i_034dd0),
    (0x034DD4, i_034dd4),
    (0x034DD6, i_034dd6),
    (0x034DDC, i_034ddc),
    (0x034DE2, i_034de2),
    (0x034DE6, i_034de6),
    (0x034DEA, i_034dea),
    (0x034DEE, i_034dee),
    (0x034DF0, i_034df0),
    (0x034DF6, i_034df6),
    (0x034DFA, i_034dfa),
    (0x034E02, i_034e02),
    (0x034E06, i_034e06),
    (0x034E0C, i_034e0c),
    (0x034E12, i_034e12),
    (0x034E18, i_034e18),
    (0x034E1E, i_034e1e),
    (0x034E22, i_034e22),
    (0x034E2A, i_034e2a),
    (0x034E2E, i_034e2e),
    (0x034E34, i_034e34),
    (0x034E3A, i_034e3a),
    (0x034E40, i_034e40),
    (0x034E46, i_034e46),
    (0x034E4C, i_034e4c),
    (0x034E50, i_034e50),
    (0x034E54, i_034e54),
    (0x034E58, i_034e58),
    (0x034E5C, i_034e5c),
    (0x034E5E, i_034e5e),
    (0x034E62, i_034e62),
    (0x034E66, i_034e66),
    (0x034E6A, i_034e6a),
    (0x034E6C, i_034e6c),
    (0x034E70, i_034e70),
    (0x034E74, i_034e74),
    (0x034E78, i_034e78),
    (0x034E7C, i_034e7c),
    (0x034E80, i_034e80),
    (0x034E82, i_034e82),
    (0x034E86, i_034e86),
    (0x034E8A, i_034e8a),
    (0x034E8E, i_034e8e),
    (0x034E92, i_034e92),
    (0x034E94, i_034e94),
    (0x034E98, i_034e98),
    (0x034E9C, i_034e9c),
    (0x034EA0, i_034ea0),
    (0x034EA4, i_034ea4),
    (0x034EA6, i_034ea6),
    (0x034EAA, i_034eaa),
    (0x034EAE, i_034eae),
    (0x034EB2, i_034eb2),
    (0x034EB6, i_034eb6),
    (0x034EB8, i_034eb8),
    (0x034EBC, i_034ebc),
    (0x034EC0, i_034ec0),
    (0x034EC2, i_034ec2),
    (0x034EC6, i_034ec6),
    (0x034ECA, i_034eca),
    (0x034ED0, i_034ed0),
    (0x034ED6, i_034ed6),
    (0x034EDA, i_034eda),
    (0x034EDE, i_034ede),
    (0x034EE4, i_034ee4),
    (0x034EEA, i_034eea),
    (0x034EEE, i_034eee),
    (0x034EF4, i_034ef4),
    (0x034EF8, i_034ef8),
    (0x034EFE, i_034efe),
    (0x034F04, i_034f04),
    (0x034F0A, i_034f0a),
    (0x034F10, i_034f10),
    (0x034F14, i_034f14),
    (0x034F1A, i_034f1a),
    (0x034F20, i_034f20),
    (0x034F26, i_034f26),
    (0x034F2C, i_034f2c),
    (0x034F32, i_034f32),
    (0x034F38, i_034f38),
    (0x034F3C, i_034f3c),
    (0x034F42, i_034f42),
    (0x034F46, i_034f46),
    (0x034F4C, i_034f4c),
    (0x034F52, i_034f52),
    (0x034F58, i_034f58),
    (0x034F5E, i_034f5e),
    (0x034F62, i_034f62),
    (0x034F68, i_034f68),
    (0x034F6E, i_034f6e),
    (0x034F74, i_034f74),
    (0x034F7A, i_034f7a),
    (0x034F80, i_034f80),
    (0x034F86, i_034f86),
    (0x034F8C, i_034f8c),
    (0x03503E, i_03503e),
    (0x035044, i_035044),
    (0x03504A, i_03504a),
    (0x03504E, i_03504e),
    (0x035052, i_035052),
    (0x035058, i_035058),
    (0x03505E, i_03505e),
    (0x035062, i_035062),
    (0x035068, i_035068),
    (0x03506C, i_03506c),
    (0x035072, i_035072),
    (0x035078, i_035078),
    (0x03507E, i_03507e),
    (0x035084, i_035084),
    (0x035088, i_035088),
    (0x03508E, i_03508e),
    (0x035094, i_035094),
    (0x03509A, i_03509a),
    (0x0350A0, i_0350a0),
    (0x0350A6, i_0350a6),
    (0x0350AC, i_0350ac),
    (0x0350B0, i_0350b0),
    (0x0350B6, i_0350b6),
    (0x0350BA, i_0350ba),
    (0x0350C0, i_0350c0),
    (0x0350C6, i_0350c6),
    (0x0350CC, i_0350cc),
    (0x0350D2, i_0350d2),
    (0x0350D6, i_0350d6),
    (0x0350DC, i_0350dc),
    (0x0350E2, i_0350e2),
    (0x0350E8, i_0350e8),
    (0x0350EE, i_0350ee),
    (0x0350F4, i_0350f4),
    (0x0350FA, i_0350fa),
    (0x035100, i_035100),
    (0x0351B0, i_0351b0),
    (0x0351B6, i_0351b6),
    (0x0351BC, i_0351bc),
    (0x0351C0, i_0351c0),
    (0x0351C4, i_0351c4),
    (0x0351CA, i_0351ca),
    (0x0351D0, i_0351d0),
    (0x0351D4, i_0351d4),
    (0x0351D8, i_0351d8),
    (0x0351DE, i_0351de),
    (0x0351E2, i_0351e2),
    (0x0351E8, i_0351e8),
    (0x0351EC, i_0351ec),
    (0x0351F2, i_0351f2),
    (0x0351F8, i_0351f8),
    (0x0351FE, i_0351fe),
    (0x035204, i_035204),
    (0x03520A, i_03520a),
    (0x03520E, i_03520e),
    (0x035214, i_035214),
    (0x03521A, i_03521a),
    (0x035220, i_035220),
    (0x035226, i_035226),
    (0x03522C, i_03522c),
    (0x035232, i_035232),
    (0x035238, i_035238),
    (0x03523E, i_03523e),
    (0x035244, i_035244),
    (0x03524A, i_03524a),
    (0x035250, i_035250),
    (0x035256, i_035256),
    (0x03525A, i_03525a),
    (0x035262, i_035262),
    (0x035266, i_035266),
    (0x03526A, i_03526a),
    (0x035270, i_035270),
    (0x035274, i_035274),
    (0x035276, i_035276),
    (0x03527A, i_03527a),
    (0x03527E, i_03527e),
    (0x035282, i_035282),
    (0x035286, i_035286),
    (0x035288, i_035288),
    (0x03528E, i_03528e),
    (0x035294, i_035294),
    (0x035298, i_035298),
    (0x03529C, i_03529c),
    (0x0352A2, i_0352a2),
    (0x0352A8, i_0352a8),
    (0x0352AE, i_0352ae),
    (0x0352B4, i_0352b4),
    (0x0352BA, i_0352ba),
    (0x0352C0, i_0352c0),
    (0x0352C4, i_0352c4),
    (0x0352CA, i_0352ca),
    (0x0352CE, i_0352ce),
    (0x0352D4, i_0352d4),
    (0x0352DA, i_0352da),
    (0x0352E0, i_0352e0),
    (0x0352E4, i_0352e4),
    (0x0352EA, i_0352ea),
    (0x0352F0, i_0352f0),
    (0x0352F6, i_0352f6),
    (0x0352FC, i_0352fc),
    (0x035302, i_035302),
    (0x035308, i_035308),
    (0x03530E, i_03530e),
    (0x035312, i_035312),
    (0x035318, i_035318),
    (0x03531C, i_03531c),
    (0x035322, i_035322),
    (0x035326, i_035326),
    (0x03532C, i_03532c),
    (0x035332, i_035332),
    (0x035336, i_035336),
    (0x03533C, i_03533c),
    (0x035342, i_035342),
    (0x035348, i_035348),
    (0x03534C, i_03534c),
    (0x035352, i_035352),
    (0x035358, i_035358),
    (0x03535E, i_03535e),
    (0x035364, i_035364),
    (0x035368, i_035368),
    (0x03536E, i_03536e),
    (0x035372, i_035372),
    (0x035378, i_035378),
    (0x03537C, i_03537c),
    (0x035382, i_035382),
    (0x035388, i_035388),
    (0x03538C, i_03538c),
    (0x035392, i_035392),
    (0x035398, i_035398),
    (0x03539E, i_03539e),
    (0x0353A2, i_0353a2),
    (0x0353A8, i_0353a8),
    (0x0353AE, i_0353ae),
    (0x0353B4, i_0353b4),
    (0x0353BA, i_0353ba),
    (0x0353BE, i_0353be),
    (0x0353C4, i_0353c4),
    (0x0353CA, i_0353ca),
    (0x0353CE, i_0353ce),
    (0x0353D2, i_0353d2),
    (0x0353D6, i_0353d6),
    (0x0353DA, i_0353da),
    (0x0353DC, i_0353dc),
    (0x0353E0, i_0353e0),
    (0x0353E4, i_0353e4),
    (0x0353EA, i_0353ea),
    (0x0353EE, i_0353ee),
    (0x0353F4, i_0353f4),
    (0x0353F8, i_0353f8),
    (0x0353FC, i_0353fc),
    (0x0353FE, i_0353fe),
    (0x035402, i_035402),
    (0x035406, i_035406),
    (0x03540A, i_03540a),
    (0x03540E, i_03540e),
    (0x035410, i_035410),
    (0x035414, i_035414),
    (0x035418, i_035418),
    (0x03541E, i_03541e),
    (0x035422, i_035422),
    (0x035428, i_035428),
    (0x03542C, i_03542c),
    (0x035430, i_035430),
    (0x035432, i_035432),
    (0x035436, i_035436),
    (0x03543A, i_03543a),
    (0x03543E, i_03543e),
    (0x035442, i_035442),
    (0x035446, i_035446),
    (0x035448, i_035448),
    (0x03544C, i_03544c),
    (0x035450, i_035450),
    (0x035454, i_035454),
    (0x035458, i_035458),
    (0x03545A, i_03545a),
    (0x03545E, i_03545e),
    (0x035462, i_035462),
    (0x035466, i_035466),
    (0x03546A, i_03546a),
    (0x03546C, i_03546c),
    (0x035470, i_035470),
    (0x035474, i_035474),
    (0x035478, i_035478),
    (0x03547C, i_03547c),
    (0x03547E, i_03547e),
    (0x035482, i_035482),
    (0x035486, i_035486),
    (0x035488, i_035488),
    (0x03548C, i_03548c),
    (0x035490, i_035490),
    (0x035494, i_035494),
    (0x035498, i_035498),
    (0x03549A, i_03549a),
    (0x0354A0, i_0354a0),
    (0x0354A4, i_0354a4),
    (0x0354AA, i_0354aa),
    (0x0354AE, i_0354ae),
    (0x0354B2, i_0354b2),
    (0x0354B4, i_0354b4),
    (0x0354BA, i_0354ba),
    (0x0354BE, i_0354be),
    (0x0354C4, i_0354c4),
    (0x0354C8, i_0354c8),
    (0x0354CC, i_0354cc),
    (0x0354CE, i_0354ce),
    (0x0354D4, i_0354d4),
    (0x0354D8, i_0354d8),
    (0x0354DC, i_0354dc),
    (0x0354DE, i_0354de),
    (0x0354E2, i_0354e2),
    (0x0354E6, i_0354e6),
    (0x0354EA, i_0354ea),
    (0x0354EC, i_0354ec),
    (0x0354F0, i_0354f0),
    (0x0354F2, i_0354f2),
    (0x0354F8, i_0354f8),
    (0x0354FC, i_0354fc),
    (0x035502, i_035502),
    (0x035506, i_035506),
    (0x03550C, i_03550c),
    (0x035512, i_035512),
    (0x035518, i_035518),
    (0x03551E, i_03551e),
    (0x035524, i_035524),
    (0x035528, i_035528),
    (0x03552E, i_03552e),
    (0x035534, i_035534),
    (0x03553A, i_03553a),
    (0x035540, i_035540),
    (0x035546, i_035546),
    (0x03554C, i_03554c),
    (0x035552, i_035552),
    (0x035558, i_035558),
    (0x03555E, i_03555e),
    (0x035564, i_035564),
    (0x03556A, i_03556a),
    (0x035570, i_035570),
    (0x035574, i_035574),
    (0x03557C, i_03557c),
    (0x035580, i_035580),
    (0x035584, i_035584),
    (0x035588, i_035588),
    (0x03558A, i_03558a),
    (0x035590, i_035590),
    (0x035594, i_035594),
    (0x035598, i_035598),
    (0x03559C, i_03559c),
    (0x0355A0, i_0355a0),
    (0x0355A2, i_0355a2),
    (0x0355A8, i_0355a8),
    (0x0355AE, i_0355ae),
    (0x0355B2, i_0355b2),
    (0x0355B6, i_0355b6),
    (0x0355BC, i_0355bc),
    (0x0355C2, i_0355c2),
    (0x0355C8, i_0355c8),
    (0x0355CE, i_0355ce),
    (0x0355D4, i_0355d4),
    (0x0355DA, i_0355da),
    (0x0355DE, i_0355de),
    (0x0355E4, i_0355e4),
    (0x0355E8, i_0355e8),
    (0x0355EE, i_0355ee),
    (0x0355F4, i_0355f4),
    (0x0355FA, i_0355fa),
    (0x0355FE, i_0355fe),
    (0x035604, i_035604),
    (0x03560A, i_03560a),
    (0x035610, i_035610),
    (0x035616, i_035616),
    (0x03561C, i_03561c),
    (0x035622, i_035622),
    (0x035628, i_035628),
    (0x03562C, i_03562c),
    (0x035632, i_035632),
    (0x035636, i_035636),
    (0x03563C, i_03563c),
    (0x035640, i_035640),
    (0x035646, i_035646),
    (0x03564C, i_03564c),
    (0x035650, i_035650),
    (0x035656, i_035656),
    (0x03565C, i_03565c),
    (0x035662, i_035662),
    (0x035666, i_035666),
    (0x03566C, i_03566c),
    (0x035672, i_035672),
    (0x035678, i_035678),
    (0x03567E, i_03567e),
    (0x035682, i_035682),
    (0x035688, i_035688),
    (0x03568C, i_03568c),
    (0x035692, i_035692),
    (0x035696, i_035696),
    (0x03569C, i_03569c),
    (0x0356A2, i_0356a2),
    (0x0356A6, i_0356a6),
    (0x0356AC, i_0356ac),
    (0x0356B2, i_0356b2),
    (0x0356B8, i_0356b8),
    (0x0356BC, i_0356bc),
    (0x0356C2, i_0356c2),
    (0x0356C8, i_0356c8),
    (0x0356CE, i_0356ce),
    (0x0356D4, i_0356d4),
    (0x0356D8, i_0356d8),
    (0x0356DE, i_0356de),
    (0x0356E4, i_0356e4),
    (0x0356E8, i_0356e8),
    (0x0356EC, i_0356ec),
    (0x0356F0, i_0356f0),
    (0x0356F4, i_0356f4),
    (0x0356F6, i_0356f6),
    (0x0356FA, i_0356fa),
    (0x0356FE, i_0356fe),
    (0x035704, i_035704),
    (0x035708, i_035708),
    (0x03570E, i_03570e),
    (0x035712, i_035712),
    (0x035716, i_035716),
    (0x035718, i_035718),
    (0x03571C, i_03571c),
    (0x035720, i_035720),
    (0x035724, i_035724),
    (0x035728, i_035728),
    (0x03572A, i_03572a),
    (0x03572E, i_03572e),
    (0x035732, i_035732),
    (0x035738, i_035738),
    (0x03573C, i_03573c),
    (0x035742, i_035742),
    (0x035746, i_035746),
    (0x03574A, i_03574a),
    (0x03574C, i_03574c),
    (0x035750, i_035750),
    (0x035754, i_035754),
    (0x035758, i_035758),
    (0x03575C, i_03575c),
    (0x035760, i_035760),
    (0x035762, i_035762),
    (0x035766, i_035766),
    (0x03576A, i_03576a),
    (0x03576E, i_03576e),
    (0x035772, i_035772),
    (0x035774, i_035774),
    (0x035778, i_035778),
    (0x03577C, i_03577c),
    (0x035780, i_035780),
    (0x035784, i_035784),
    (0x035786, i_035786),
    (0x03578A, i_03578a),
    (0x03578E, i_03578e),
    (0x035792, i_035792),
    (0x035796, i_035796),
    (0x035798, i_035798),
    (0x03579C, i_03579c),
    (0x0357A0, i_0357a0),
    (0x0357A2, i_0357a2),
    (0x0357A6, i_0357a6),
    (0x0357AA, i_0357aa),
    (0x0357B0, i_0357b0),
    (0x0357B6, i_0357b6),
    (0x0357BA, i_0357ba),
    (0x0357BE, i_0357be),
    (0x0357C4, i_0357c4),
    (0x0357CA, i_0357ca),
    (0x0357CE, i_0357ce),
    (0x0357D2, i_0357d2),
    (0x0357D8, i_0357d8),
    (0x0357DE, i_0357de),
    (0x0357E2, i_0357e2),
    (0x0357E6, i_0357e6),
    (0x0357EC, i_0357ec),
    (0x0357F2, i_0357f2),
    (0x0357F6, i_0357f6),
    (0x0357FA, i_0357fa),
    (0x035800, i_035800),
    (0x035806, i_035806),
    (0x03580A, i_03580a),
    (0x035810, i_035810),
    (0x035814, i_035814),
    (0x03581A, i_03581a),
    (0x035820, i_035820),
    (0x035826, i_035826),
    (0x03582C, i_03582c),
    (0x035830, i_035830),
    (0x035836, i_035836),
    (0x03583C, i_03583c),
    (0x035842, i_035842),
    (0x035848, i_035848),
    (0x03584E, i_03584e),
    (0x035854, i_035854),
    (0x035858, i_035858),
    (0x03585E, i_03585e),
    (0x035862, i_035862),
    (0x035868, i_035868),
    (0x03586E, i_03586e),
    (0x035874, i_035874),
    (0x03587A, i_03587a),
    (0x03587E, i_03587e),
    (0x035884, i_035884),
    (0x03588A, i_03588a),
    (0x035890, i_035890),
    (0x035896, i_035896),
    (0x03589C, i_03589c),
    (0x0358A2, i_0358a2),
    (0x0358A8, i_0358a8),
    (0x0358AA, i_0358aa),
    (0x0358B0, i_0358b0),
    (0x0358B6, i_0358b6),
    (0x0358BA, i_0358ba),
    (0x0358C0, i_0358c0),
    (0x0358C4, i_0358c4),
    (0x0358CA, i_0358ca),
    (0x0358D0, i_0358d0),
    (0x0358D6, i_0358d6),
    (0x0358DC, i_0358dc),
    (0x0358E0, i_0358e0),
    (0x0358E6, i_0358e6),
    (0x0358EC, i_0358ec),
    (0x0358F2, i_0358f2),
    (0x0358F8, i_0358f8),
    (0x0358FE, i_0358fe),
    (0x035904, i_035904),
    (0x035908, i_035908),
    (0x03590E, i_03590e),
    (0x035912, i_035912),
    (0x035918, i_035918),
    (0x03591E, i_03591e),
    (0x035924, i_035924),
    (0x03592A, i_03592a),
    (0x03592E, i_03592e),
    (0x035934, i_035934),
    (0x03593A, i_03593a),
    (0x035940, i_035940),
    (0x035946, i_035946),
    (0x03594C, i_03594c),
    (0x035952, i_035952),
    (0x035958, i_035958),
    (0x03595A, i_03595a),
    (0x035960, i_035960),
    (0x035966, i_035966),
    (0x03596A, i_03596a),
    (0x035970, i_035970),
    (0x035974, i_035974),
    (0x03597A, i_03597a),
    (0x035980, i_035980),
    (0x035986, i_035986),
    (0x03598C, i_03598c),
    (0x035990, i_035990),
    (0x035996, i_035996),
    (0x03599C, i_03599c),
    (0x0359A2, i_0359a2),
    (0x0359A8, i_0359a8),
    (0x0359AE, i_0359ae),
    (0x0359B4, i_0359b4),
    (0x0359B8, i_0359b8),
    (0x0359BE, i_0359be),
    (0x0359C2, i_0359c2),
    (0x0359C8, i_0359c8),
    (0x0359CE, i_0359ce),
    (0x0359D4, i_0359d4),
    (0x0359DA, i_0359da),
    (0x0359DE, i_0359de),
    (0x0359E4, i_0359e4),
    (0x0359EA, i_0359ea),
    (0x0359F0, i_0359f0),
    (0x0359F6, i_0359f6),
    (0x0359FC, i_0359fc),
    (0x035A02, i_035a02),
    (0x035A08, i_035a08),
    (0x035A0A, i_035a0a),
    (0x035A10, i_035a10),
    (0x035A16, i_035a16),
    (0x035A1A, i_035a1a),
    (0x035A20, i_035a20),
    (0x035A24, i_035a24),
    (0x035A2A, i_035a2a),
    (0x035A30, i_035a30),
    (0x035A36, i_035a36),
    (0x035A3C, i_035a3c),
    (0x035A40, i_035a40),
    (0x035A46, i_035a46),
    (0x035A4C, i_035a4c),
    (0x035A52, i_035a52),
    (0x035A58, i_035a58),
    (0x035A5E, i_035a5e),
    (0x035A64, i_035a64),
    (0x035A68, i_035a68),
    (0x035A6E, i_035a6e),
    (0x035A72, i_035a72),
    (0x035A78, i_035a78),
    (0x035A7E, i_035a7e),
    (0x035A84, i_035a84),
    (0x035A8A, i_035a8a),
    (0x035A8E, i_035a8e),
    (0x035A94, i_035a94),
    (0x035A9A, i_035a9a),
    (0x035AA0, i_035aa0),
    (0x035AA6, i_035aa6),
    (0x035AAC, i_035aac),
    (0x035AB2, i_035ab2),
    (0x035AB8, i_035ab8),
    (0x035ABA, i_035aba),
    (0x035AC0, i_035ac0),
    (0x035AC6, i_035ac6),
    (0x035ACC, i_035acc),
    (0x035AD2, i_035ad2),
    (0x035AD8, i_035ad8),
    (0x035ADE, i_035ade),
    (0x035AE2, i_035ae2),
    (0x035AE8, i_035ae8),
    (0x035AEE, i_035aee),
    (0x035AF4, i_035af4),
    (0x035AFA, i_035afa),
    (0x035B00, i_035b00),
    (0x035B06, i_035b06),
    (0x035B0A, i_035b0a),
    (0x035B10, i_035b10),
    (0x035B16, i_035b16),
    (0x035B1C, i_035b1c),
    (0x035B22, i_035b22),
    (0x035B28, i_035b28),
    (0x035B2E, i_035b2e),
    (0x035B34, i_035b34),
    (0x035B38, i_035b38),
    (0x035B40, i_035b40),
    (0x035B44, i_035b44),
    (0x035B48, i_035b48),
    (0x035B4E, i_035b4e),
    (0x035B52, i_035b52),
    (0x035B54, i_035b54),
    (0x035B58, i_035b58),
    (0x035B5C, i_035b5c),
    (0x035B60, i_035b60),
    (0x035B64, i_035b64),
    (0x035B66, i_035b66),
    (0x035B6C, i_035b6c),
    (0x035B72, i_035b72),
    (0x035B76, i_035b76),
    (0x035B7A, i_035b7a),
    (0x035B7E, i_035b7e),
    (0x035B80, i_035b80),
    (0x035B86, i_035b86),
    (0x035B8A, i_035b8a),
    (0x035B8E, i_035b8e),
    (0x035B90, i_035b90),
    (0x035B96, i_035b96),
    (0x035B9A, i_035b9a),
    (0x035B9E, i_035b9e),
    (0x035BA2, i_035ba2),
    (0x035BA6, i_035ba6),
    (0x035BAA, i_035baa),
    (0x035BAE, i_035bae),
    (0x035BB0, i_035bb0),
    (0x035BB4, i_035bb4),
    (0x035BB8, i_035bb8),
    (0x035BBC, i_035bbc),
    (0x035BC0, i_035bc0),
    (0x035BC2, i_035bc2),
    (0x035BC6, i_035bc6),
    (0x035BCA, i_035bca),
    (0x035BCE, i_035bce),
    (0x035BD2, i_035bd2),
    (0x035BD4, i_035bd4),
    (0x035BD8, i_035bd8),
    (0x035BDC, i_035bdc),
    (0x035BE0, i_035be0),
    (0x035BE4, i_035be4),
    (0x035BE6, i_035be6),
    (0x035BEA, i_035bea),
    (0x035BEE, i_035bee),
    (0x035BF0, i_035bf0),
    (0x035BF4, i_035bf4),
    (0x035BF8, i_035bf8),
    (0x035BFE, i_035bfe),
    (0x035C04, i_035c04),
    (0x035C0A, i_035c0a),
    (0x035C10, i_035c10),
    (0x035C16, i_035c16),
    (0x035C1C, i_035c1c),
    (0x035C20, i_035c20),
    (0x035C26, i_035c26),
    (0x035C2C, i_035c2c),
    (0x035C32, i_035c32),
    (0x035C38, i_035c38),
    (0x035C3E, i_035c3e),
    (0x035C44, i_035c44),
    (0x035C4A, i_035c4a),
    (0x035C4E, i_035c4e),
    (0x035C54, i_035c54),
    (0x035C5A, i_035c5a),
    (0x035C60, i_035c60),
    (0x035C66, i_035c66),
    (0x035C6C, i_035c6c),
    (0x035C72, i_035c72),
    (0x035C78, i_035c78),
    (0x035C7C, i_035c7c),
    (0x035C84, i_035c84),
    (0x035C88, i_035c88),
    (0x035C8C, i_035c8c),
    (0x035C90, i_035c90),
    (0x035C92, i_035c92),
    (0x035C98, i_035c98),
    (0x035C9C, i_035c9c),
    (0x035CA0, i_035ca0),
    (0x035CA4, i_035ca4),
    (0x035CA8, i_035ca8),
    (0x035CAA, i_035caa),
    (0x035CB0, i_035cb0),
    (0x035CB6, i_035cb6),
    (0x035CBA, i_035cba),
    (0x035CBE, i_035cbe),
    (0x035CC2, i_035cc2),
    (0x035CC4, i_035cc4),
    (0x035CCA, i_035cca),
    (0x035CCE, i_035cce),
    (0x035CD2, i_035cd2),
    (0x035CD4, i_035cd4),
    (0x035CD8, i_035cd8),
    (0x035CDC, i_035cdc),
    (0x035CE0, i_035ce0),
    (0x035CE4, i_035ce4),
    (0x035CE8, i_035ce8),
    (0x035CEC, i_035cec),
    (0x035CEE, i_035cee),
    (0x035CF2, i_035cf2),
    (0x035CF6, i_035cf6),
    (0x035CFA, i_035cfa),
    (0x035CFE, i_035cfe),
    (0x035D00, i_035d00),
    (0x035D04, i_035d04),
    (0x035D08, i_035d08),
    (0x035D0C, i_035d0c),
    (0x035D10, i_035d10),
    (0x035D12, i_035d12),
    (0x035D16, i_035d16),
    (0x035D1A, i_035d1a),
    (0x035D1E, i_035d1e),
    (0x035D22, i_035d22),
    (0x035D24, i_035d24),
    (0x035D28, i_035d28),
    (0x035D2C, i_035d2c),
    (0x035D2E, i_035d2e),
    (0x035D32, i_035d32),
    (0x035D34, i_035d34),
    (0x035D3A, i_035d3a),
    (0x035D40, i_035d40),
    (0x035D46, i_035d46),
    (0x035D4C, i_035d4c),
    (0x035D52, i_035d52),
    (0x035D56, i_035d56),
    (0x035D5A, i_035d5a),
    (0x035D60, i_035d60),
    (0x035D64, i_035d64),
    (0x035D68, i_035d68),
    (0x035D6C, i_035d6c),
    (0x035D70, i_035d70),
    (0x035D78, i_035d78),
    (0x035D7E, i_035d7e),
    (0x035D82, i_035d82),
    (0x035D88, i_035d88),
    (0x035D8E, i_035d8e),
    (0x035D94, i_035d94),
    (0x035D9A, i_035d9a),
    (0x035DA0, i_035da0),
    (0x035DA6, i_035da6),
    (0x035DAA, i_035daa),
    (0x035DB0, i_035db0),
    (0x035DB6, i_035db6),
    (0x035DBC, i_035dbc),
    (0x035DC2, i_035dc2),
    (0x035DC8, i_035dc8),
    (0x035DCE, i_035dce),
    (0x035DD2, i_035dd2),
    (0x035DD6, i_035dd6),
    (0x035DDA, i_035dda),
    (0x035DDC, i_035ddc),
    (0x035DE0, i_035de0),
    (0x035DE2, i_035de2),
    (0x035DE6, i_035de6),
    (0x035DEA, i_035dea),
    (0x035DEE, i_035dee),
    (0x035DF2, i_035df2),
    (0x035DF6, i_035df6),
    (0x035DFA, i_035dfa),
    (0x035DFE, i_035dfe),
    (0x035E02, i_035e02),
    (0x035E06, i_035e06),
    (0x035E0A, i_035e0a),
    (0x035E10, i_035e10),
    (0x035E16, i_035e16),
    (0x035E1C, i_035e1c),
    (0x035E20, i_035e20),
    (0x035E24, i_035e24),
    (0x035E28, i_035e28),
    (0x035E2E, i_035e2e),
    (0x035E34, i_035e34),
    (0x035E3A, i_035e3a),
    (0x035E3E, i_035e3e),
    (0x035E44, i_035e44),
    (0x035E4A, i_035e4a),
    (0x035E50, i_035e50),
    (0x035E54, i_035e54),
    (0x035E58, i_035e58),
    (0x035E5C, i_035e5c),
    (0x035E62, i_035e62),
    (0x035E66, i_035e66),
    (0x035E6A, i_035e6a),
    (0x035E6E, i_035e6e),
    (0x035E70, i_035e70),
    (0x035E76, i_035e76),
    (0x035E78, i_035e78),
    (0x035E7C, i_035e7c),
    (0x035E80, i_035e80),
    (0x035E86, i_035e86),
    (0x035E8A, i_035e8a),
    (0x035E8E, i_035e8e),
    (0x035E90, i_035e90),
    (0x035E96, i_035e96),
    (0x035E9A, i_035e9a),
    (0x035E9E, i_035e9e),
    (0x035EA0, i_035ea0),
    (0x035EA4, i_035ea4),
    (0x035EA8, i_035ea8),
    (0x035EAE, i_035eae),
    (0x035EB4, i_035eb4),
    (0x035EBA, i_035eba),
    (0x035EC0, i_035ec0),
    (0x035EC6, i_035ec6),
    (0x035ECA, i_035eca),
    (0x035ECE, i_035ece),
    (0x035ED4, i_035ed4),
    (0x035EDC, i_035edc),
    (0x035EE2, i_035ee2),
    (0x035EE6, i_035ee6),
    (0x035EEC, i_035eec),
    (0x035EF2, i_035ef2),
    (0x035EF8, i_035ef8),
    (0x035EFE, i_035efe),
    (0x035F04, i_035f04),
    (0x035F0A, i_035f0a),
    (0x035F0E, i_035f0e),
    (0x035F14, i_035f14),
    (0x035F1A, i_035f1a),
    (0x035F20, i_035f20),
    (0x035F26, i_035f26),
    (0x035F2C, i_035f2c),
    (0x035F32, i_035f32),
    (0x035F36, i_035f36),
    (0x035F3A, i_035f3a),
    (0x035F3E, i_035f3e),
    (0x035F40, i_035f40),
    (0x035F44, i_035f44),
    (0x035F48, i_035f48),
    (0x035F4E, i_035f4e),
    (0x035F52, i_035f52),
    (0x035F56, i_035f56),
    (0x035F5A, i_035f5a),
    (0x035F5C, i_035f5c),
    (0x035F62, i_035f62),
    (0x035F64, i_035f64),
    (0x035F68, i_035f68),
    (0x035F6C, i_035f6c),
    (0x035F70, i_035f70),
    (0x035F76, i_035f76),
    (0x035F7C, i_035f7c),
    (0x035F82, i_035f82),
    (0x035F88, i_035f88),
    (0x035F8E, i_035f8e),
    (0x035F92, i_035f92),
    (0x035F96, i_035f96),
    (0x035F9C, i_035f9c),
    (0x035FA4, i_035fa4),
    (0x035FAA, i_035faa),
    (0x035FAE, i_035fae),
    (0x035FB4, i_035fb4),
    (0x035FBA, i_035fba),
    (0x035FC0, i_035fc0),
    (0x035FC6, i_035fc6),
    (0x035FCC, i_035fcc),
    (0x035FD2, i_035fd2),
    (0x035FD6, i_035fd6),
    (0x035FDC, i_035fdc),
    (0x035FE2, i_035fe2),
    (0x035FE8, i_035fe8),
    (0x035FEE, i_035fee),
    (0x035FF4, i_035ff4),
    (0x035FFA, i_035ffa),
    (0x035FFE, i_035ffe),
    (0x036002, i_036002),
    (0x036006, i_036006),
    (0x036008, i_036008),
    (0x03600C, i_03600c),
    (0x036010, i_036010),
    (0x036012, i_036012),
    (0x036016, i_036016),
    (0x03601A, i_03601a),
    (0x036020, i_036020),
    (0x036024, i_036024),
    (0x036028, i_036028),
    (0x03602C, i_03602c),
    (0x036032, i_036032),
    (0x036036, i_036036),
    (0x03603A, i_03603a),
    (0x036040, i_036040),
    (0x036044, i_036044),
    (0x036048, i_036048),
    (0x03604C, i_03604c),
    (0x03604E, i_03604e),
    (0x036054, i_036054),
    (0x036056, i_036056),
    (0x03605A, i_03605a),
    (0x03605E, i_03605e),
    (0x036062, i_036062),
    (0x036066, i_036066),
    (0x03606A, i_03606a),
    (0x03606E, i_03606e),
    (0x036072, i_036072),
    (0x036074, i_036074),
    (0x036078, i_036078),
    (0x03607C, i_03607c),
    (0x036080, i_036080),
    (0x036084, i_036084),
    (0x036086, i_036086),
    (0x03608A, i_03608a),
    (0x03608E, i_03608e),
    (0x036092, i_036092),
    (0x036096, i_036096),
    (0x036098, i_036098),
    (0x03609C, i_03609c),
    (0x0360A0, i_0360a0),
    (0x0360A4, i_0360a4),
    (0x0360A8, i_0360a8),
    (0x0360AA, i_0360aa),
    (0x0360AE, i_0360ae),
    (0x0360B2, i_0360b2),
    (0x0360B4, i_0360b4),
    (0x0360B8, i_0360b8),
    (0x0360BC, i_0360bc),
    (0x0360C2, i_0360c2),
    (0x0360C8, i_0360c8),
    (0x0360CE, i_0360ce),
    (0x0360D4, i_0360d4),
    (0x0360DA, i_0360da),
    (0x0360DE, i_0360de),
    (0x0360E2, i_0360e2),
    (0x0360E6, i_0360e6),
    (0x0360EA, i_0360ea),
    (0x0360F2, i_0360f2),
    (0x0360F8, i_0360f8),
    (0x0360FC, i_0360fc),
    (0x036102, i_036102),
    (0x036108, i_036108),
    (0x03610E, i_03610e),
    (0x036114, i_036114),
    (0x03611A, i_03611a),
    (0x036120, i_036120),
    (0x036124, i_036124),
    (0x03612A, i_03612a),
    (0x036130, i_036130),
    (0x036136, i_036136),
    (0x03613C, i_03613c),
    (0x036142, i_036142),
    (0x036148, i_036148),
    (0x03614C, i_03614c),
    (0x036150, i_036150),
    (0x036152, i_036152),
    (0x036156, i_036156),
    (0x03615A, i_03615a),
    (0x036160, i_036160),
    (0x036164, i_036164),
    (0x036168, i_036168),
    (0x03616C, i_03616c),
    (0x03616E, i_03616e),
    (0x036174, i_036174),
    (0x036176, i_036176),
    (0x03617A, i_03617a),
    (0x03617E, i_03617e),
    (0x036184, i_036184),
    (0x036188, i_036188),
    (0x03618C, i_03618c),
    (0x03618E, i_03618e),
    (0x036194, i_036194),
    (0x036198, i_036198),
    (0x03619C, i_03619c),
    (0x03619E, i_03619e),
    (0x0361A4, i_0361a4),
    (0x0361A8, i_0361a8),
    (0x0361AE, i_0361ae),
    (0x0361B2, i_0361b2),
    (0x0361B6, i_0361b6),
    (0x0361BA, i_0361ba),
    (0x0361BE, i_0361be),
    (0x0361C0, i_0361c0),
    (0x0361C4, i_0361c4),
    (0x0361C8, i_0361c8),
    (0x0361CC, i_0361cc),
    (0x0361CE, i_0361ce),
    (0x0361D2, i_0361d2),
    (0x0361D6, i_0361d6),
    (0x0361DA, i_0361da),
    (0x0361DE, i_0361de),
    (0x0361E0, i_0361e0),
    (0x0361E4, i_0361e4),
    (0x0361E8, i_0361e8),
    (0x0361EC, i_0361ec),
    (0x0361EE, i_0361ee),
    (0x0361F2, i_0361f2),
    (0x0361F6, i_0361f6),
    (0x0361FA, i_0361fa),
    (0x0361FC, i_0361fc),
    (0x036200, i_036200),
    (0x036204, i_036204),
    (0x036208, i_036208),
    (0x03620A, i_03620a),
    (0x03620E, i_03620e),
    (0x036212, i_036212),
    (0x036218, i_036218),
    (0x03621E, i_03621e),
    (0x036224, i_036224),
    (0x03622A, i_03622a),
    (0x036230, i_036230),
    (0x036238, i_036238),
    (0x03623E, i_03623e),
    (0x036242, i_036242),
    (0x036248, i_036248),
    (0x03624E, i_03624e),
    (0x036254, i_036254),
    (0x03625A, i_03625a),
    (0x036260, i_036260),
    (0x036266, i_036266),
    (0x03626A, i_03626a),
    (0x036270, i_036270),
    (0x036276, i_036276),
    (0x03627C, i_03627c),
    (0x036282, i_036282),
    (0x036288, i_036288),
    (0x03628E, i_03628e),
    (0x036292, i_036292),
    (0x036296, i_036296),
    (0x036298, i_036298),
    (0x03629C, i_03629c),
    (0x0362A0, i_0362a0),
    (0x0362A6, i_0362a6),
    (0x0362AA, i_0362aa),
    (0x0362AE, i_0362ae),
    (0x0362B2, i_0362b2),
    (0x0362B4, i_0362b4),
    (0x0362BA, i_0362ba),
    (0x0362BC, i_0362bc),
    (0x0362C0, i_0362c0),
    (0x0362C4, i_0362c4),
    (0x0362CA, i_0362ca),
    (0x0362CE, i_0362ce),
    (0x0362D4, i_0362d4),
    (0x0362D8, i_0362d8),
    (0x0362DC, i_0362dc),
    (0x0362E0, i_0362e0),
    (0x0362E4, i_0362e4),
    (0x0362E6, i_0362e6),
    (0x0362EA, i_0362ea),
    (0x0362EE, i_0362ee),
    (0x0362F2, i_0362f2),
    (0x0362F6, i_0362f6),
    (0x0362F8, i_0362f8),
    (0x0362FC, i_0362fc),
    (0x036300, i_036300),
    (0x036304, i_036304),
    (0x036306, i_036306),
    (0x03630A, i_03630a),
    (0x03630E, i_03630e),
    (0x036312, i_036312),
    (0x036314, i_036314),
    (0x036318, i_036318),
    (0x03631E, i_03631e),
    (0x036324, i_036324),
    (0x03632A, i_03632a),
    (0x036330, i_036330),
    (0x036336, i_036336),
    (0x03633E, i_03633e),
    (0x036344, i_036344),
    (0x036348, i_036348),
    (0x03634E, i_03634e),
    (0x036354, i_036354),
    (0x03635A, i_03635a),
    (0x036360, i_036360),
    (0x036366, i_036366),
    (0x03636C, i_03636c),
    (0x036370, i_036370),
    (0x036376, i_036376),
    (0x03637C, i_03637c),
    (0x036382, i_036382),
    (0x036388, i_036388),
    (0x03638E, i_03638e),
    (0x036394, i_036394),
    (0x036398, i_036398),
];
