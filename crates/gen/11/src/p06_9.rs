// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_9) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $06F7B2: move.l a1, (a6)
pub fn i_06f7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F7B4: bra.w $6fac2
pub fn i_06f7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_BarrageEnd_06f7b8  ($06F7B8, 56 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F7B8: move.b #$4, $8c(a6)
pub fn i_06f7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7BE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F7BE: move.w #$1e, $70(a6)
pub fn i_06f7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F7C4: lea.l $6f7ca(pc), a1
pub fn i_06f7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F7CA;
    16
}
/// $06F7C8: move.l a1, (a6)
pub fn i_06f7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F7CA: jsr $28998.l
pub fn i_06f7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F7D0);
    cpu.pc = 0x28998;
    16
}
/// $06F7D0: jsr $70ad8(pc)
pub fn i_06f7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F7D4);
    cpu.pc = 0x70AD8;
    16
}
/// $06F7D4: jsr $70ab0(pc)
pub fn i_06f7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F7D8);
    cpu.pc = 0x70AB0;
    16
}
/// $06F7D8: subq.w #$1, $70(a6)
pub fn i_06f7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7DC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F7DC: cmpi.w #$0, $70(a6)
pub fn i_06f7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7E2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06F7E2: bgt.w $6f7ec
pub fn i_06f7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7E6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F7EC; return 10; }
    8
}
/// $06F7E6: lea.l $6f396(pc), a1
pub fn i_06f7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F396;
    16
}
/// $06F7EA: move.l a1, (a6)
pub fn i_06f7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F7EC: bra.w $6fac2
pub fn i_06f7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_AttackB_06f7f0  ($06F7F0, 82 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F7F0: jsr $70cfe(pc)
pub fn i_06f7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F7F4);
    cpu.pc = 0x70CFE;
    16
}
/// $06F7F4: move.b #$0, $8c(a6)
pub fn i_06f7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7FA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F7FA: move.w #$258, $94(a6)
pub fn i_06f7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F800;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F800: move.w #$1075, d0
pub fn i_06f800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F804;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1075;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F804: jsr $2352.l
pub fn i_06f804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F80A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F80A);
    cpu.pc = 0x2352;
    16
}
/// $06F80A: lea.l $2d5a78.l, a0
pub fn i_06f80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F810;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D5A78;
    20
}
/// $06F810: jsr $28cd4.l
pub fn i_06f810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F816;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F816);
    cpu.pc = 0x28CD4;
    16
}
/// $06F816: lea.l $6f81c(pc), a1
pub fn i_06f816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F81A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F81C;
    16
}
/// $06F81A: move.l a1, (a6)
pub fn i_06f81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F81C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F81C: jsr $28998.l
pub fn i_06f81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F822);
    cpu.pc = 0x28998;
    16
}
/// $06F822: jsr $70ad8(pc)
pub fn i_06f822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F826;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F826);
    cpu.pc = 0x70AD8;
    16
}
/// $06F826: bcc.w $6f830
pub fn i_06f826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F82A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F830; return 10; }
    8
}
/// $06F82A: move.b #$4, $8c(a6)
pub fn i_06f82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F830;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F830: jsr $70ab0(pc)
pub fn i_06f830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F834;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F834);
    cpu.pc = 0x70AB0;
    16
}
/// $06F834: bcc.w $6f83e
pub fn i_06f834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F838;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F83E; return 10; }
    8
}
/// $06F838: lea.l $6f842(pc), a1
pub fn i_06f838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F83C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F842;
    16
}
/// $06F83C: move.l a1, (a6)
pub fn i_06f83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F83E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F83E: bra.w $6fac2
pub fn i_06f83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F842;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_AttackBLoop_06f842  ($06F842, 126 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F842: lea.l $2b9d28.l, a0
pub fn i_06f842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F848;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9D28;
    20
}
/// $06F848: jsr $799de.l
pub fn i_06f848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F84E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F84E);
    cpu.pc = 0x799DE;
    16
}
/// $06F84E: move.b d0, $72(a6)
pub fn i_06f84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F852;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F852: move.w #$2d, $70(a6)
pub fn i_06f852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F858;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F858: lea.l $6f85e(pc), a1
pub fn i_06f858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F85C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F85E;
    16
}
/// $06F85C: move.l a1, (a6)
pub fn i_06f85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F85E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F85E: jsr $28998.l
pub fn i_06f85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F864;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F864);
    cpu.pc = 0x28998;
    16
}
/// $06F864: jsr $70ad8(pc)
pub fn i_06f864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F868;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F868);
    cpu.pc = 0x70AD8;
    16
}
/// $06F868: bcc.w $6f872
pub fn i_06f868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F86C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F872; return 10; }
    8
}
/// $06F86C: move.b #$4, $8c(a6)
pub fn i_06f86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F872;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F872: jsr $70ab0(pc)
pub fn i_06f872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F876;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F876);
    cpu.pc = 0x70AB0;
    16
}
/// $06F876: cmpi.b #$1, $8a(a6)
pub fn i_06f876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F87C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06F87C: bne.w $6f88e
pub fn i_06f87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F880;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6F88E; return 10; }
    8
}
/// $06F880: cmpi.b #$0, $72(a6)
pub fn i_06f880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F886;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06F886: ble.w $6f88e
pub fn i_06f886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F88A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6F88E; return 10; }
    8
}
/// $06F88A: jsr $70e5a(pc)
pub fn i_06f88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F88E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F88E);
    cpu.pc = 0x70E5A;
    16
}
/// $06F88E: cmpi.b #$0, $72(a6)
pub fn i_06f88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F894;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06F894: bgt.w $6f8ac
pub fn i_06f894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F898;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F8AC; return 10; }
    8
}
/// $06F898: subq.w #$1, $70(a6)
pub fn i_06f898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F89C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F89C: cmpi.w #$0, $70(a6)
pub fn i_06f89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8A2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06F8A2: bgt.w $6f8ac
pub fn i_06f8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8A6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F8AC; return 10; }
    8
}
/// $06F8A6: lea.l $6f8c0(pc), a1
pub fn i_06f8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F8C0;
    16
}
/// $06F8AA: move.l a1, (a6)
pub fn i_06f8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F8AC: btst.b #$0, $13(a6)
pub fn i_06f8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8B2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06F8B2: beq.w $6f8bc
pub fn i_06f8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F8BC; return 10; }
    8
}
/// $06F8B6: lea.l $6f8c0(pc), a1
pub fn i_06f8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F8C0;
    16
}
/// $06F8BA: move.l a1, (a6)
pub fn i_06f8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F8BC: bra.w $6fac2
pub fn i_06f8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_AttackBEnd_06f8c0  ($06F8C0, 66 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F8C0: move.w #$1075, d0
pub fn i_06f8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8C4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1075;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F8C4: jsr $2352.l
pub fn i_06f8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F8CA);
    cpu.pc = 0x2352;
    16
}
/// $06F8CA: lea.l $2d5c00.l, a0
pub fn i_06f8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D5C00;
    20
}
/// $06F8D0: jsr $28cd4.l
pub fn i_06f8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F8D6);
    cpu.pc = 0x28CD4;
    16
}
/// $06F8D6: lea.l $6f8dc(pc), a1
pub fn i_06f8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F8DC;
    16
}
/// $06F8DA: move.l a1, (a6)
pub fn i_06f8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F8DC: jsr $28998.l
pub fn i_06f8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F8E2);
    cpu.pc = 0x28998;
    16
}
/// $06F8E2: jsr $70ad8(pc)
pub fn i_06f8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F8E6);
    cpu.pc = 0x70AD8;
    16
}
/// $06F8E6: bcc.w $6f8f0
pub fn i_06f8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F8F0; return 10; }
    8
}
/// $06F8EA: move.b #$4, $8c(a6)
pub fn i_06f8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8F0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F8F0: jsr $70ab0(pc)
pub fn i_06f8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F8F4);
    cpu.pc = 0x70AB0;
    16
}
/// $06F8F4: bcc.w $6f8fe
pub fn i_06f8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F8FE; return 10; }
    8
}
/// $06F8F8: lea.l $6f396(pc), a1
pub fn i_06f8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F396;
    16
}
/// $06F8FC: move.l a1, (a6)
pub fn i_06f8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F8FE: bra.w $6fac2
pub fn i_06f8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F902;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_AttackC_06f902  ($06F902, 82 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F902: jsr $70cbe(pc)
pub fn i_06f902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F906;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F906);
    cpu.pc = 0x70CBE;
    16
}
/// $06F906: move.b #$0, $8c(a6)
pub fn i_06f906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F90C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F90C: move.w #$258, $94(a6)
pub fn i_06f90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F912;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F912: move.w #$1075, d0
pub fn i_06f912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F916;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1075;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F916: jsr $2352.l
pub fn i_06f916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F91C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F91C);
    cpu.pc = 0x2352;
    16
}
/// $06F91C: lea.l $2d5cf2.l, a0
pub fn i_06f91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F922;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D5CF2;
    20
}
/// $06F922: jsr $28cd4.l
pub fn i_06f922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F928;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F928);
    cpu.pc = 0x28CD4;
    16
}
/// $06F928: lea.l $6f92e(pc), a1
pub fn i_06f928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F92C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F92E;
    16
}
/// $06F92C: move.l a1, (a6)
pub fn i_06f92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F92E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F92E: jsr $28998.l
pub fn i_06f92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F934;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F934);
    cpu.pc = 0x28998;
    16
}
/// $06F934: jsr $70ad8(pc)
pub fn i_06f934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F938;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F938);
    cpu.pc = 0x70AD8;
    16
}
/// $06F938: bcc.w $6f942
pub fn i_06f938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F93C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F942; return 10; }
    8
}
/// $06F93C: move.b #$4, $8c(a6)
pub fn i_06f93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F942;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F942: jsr $70ab0(pc)
pub fn i_06f942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F946;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F946);
    cpu.pc = 0x70AB0;
    16
}
/// $06F946: bcc.w $6f950
pub fn i_06f946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F94A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F950; return 10; }
    8
}
/// $06F94A: lea.l $6f954(pc), a1
pub fn i_06f94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F94E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F954;
    16
}
/// $06F94E: move.l a1, (a6)
pub fn i_06f94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F950;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F950: bra.w $6fac2
pub fn i_06f950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F954;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_AttackCLoop_06f954  ($06F954, 118 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F954: move.b #$7f, $72(a6)
pub fn i_06f954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F95A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F95A: move.w #$2d, $70(a6)
pub fn i_06f95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F960;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F960: move.b #$0, $8c(a6)
pub fn i_06f960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F966;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F966: move.w #$258, $94(a6)
pub fn i_06f966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F96C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F96C: lea.l $6f972(pc), a1
pub fn i_06f96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F970;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F972;
    16
}
/// $06F970: move.l a1, (a6)
pub fn i_06f970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F972;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F972: jsr $28998.l
pub fn i_06f972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F978;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F978);
    cpu.pc = 0x28998;
    16
}
/// $06F978: jsr $70ad8(pc)
pub fn i_06f978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F97C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F97C);
    cpu.pc = 0x70AD8;
    16
}
/// $06F97C: bcc.w $6f990
pub fn i_06f97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F980;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F990; return 10; }
    8
}
/// $06F980: move.b #$0, $8c(a6)
pub fn i_06f980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F986;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F986: move.w #$258, $94(a6)
pub fn i_06f986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F98C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F98C: jsr $70cbe(pc)
pub fn i_06f98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F990;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F990);
    cpu.pc = 0x70CBE;
    16
}
/// $06F990: jsr $70ab0(pc)
pub fn i_06f990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F994;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F994);
    cpu.pc = 0x70AB0;
    16
}
/// $06F994: cmpi.b #$2, $73(a6)
pub fn i_06f994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F99A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $06F99A: blt.w $6f9aa
pub fn i_06f99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F99E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6F9AA; return 10; }
    8
}
/// $06F99E: move.l #$2d50c6, $48(a6)
pub fn i_06f99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9A6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D50C6;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06F9A6: bra.w $6f9c6
pub fn i_06f9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6F9C6;
    10
}
/// $06F9AA: tst.b $72(a6)
pub fn i_06f9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9AE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06F9AE: bne.w $6f9c6
pub fn i_06f9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6F9C6; return 10; }
    8
}
/// $06F9B2: subq.w #$1, $70(a6)
pub fn i_06f9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9B6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F9B6: cmpi.w #$0, $70(a6)
pub fn i_06f9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9BC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06F9BC: bgt.w $6f9c6
pub fn i_06f9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9C0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F9C6; return 10; }
    8
}
/// $06F9C0: lea.l $6f8c0(pc), a1
pub fn i_06f9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F8C0;
    16
}
/// $06F9C4: move.l a1, (a6)
pub fn i_06f9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F9C6: bra.w $6fae4
pub fn i_06f9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAE4;
    10
}
// ===== M5Boss_Die_06f9ca  ($06F9CA, 142 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F9CA: clr.b $106ed3.l
pub fn i_06f9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9D0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED3) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED3, r2 as u8);
    16
}
/// $06F9D0: move.w #$200, $28(a6)
pub fn i_06f9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F9D6: move.w #$ff80, $2a(a6)
pub fn i_06f9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9DC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F9DC: move.b #$3, $73(a6)
pub fn i_06f9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9E2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F9E2: lea.l $2d544c.l, a1
pub fn i_06f9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9E8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D544C;
    20
}
/// $06F9E8: jsr $77c7e.l
pub fn i_06f9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F9EE);
    cpu.pc = 0x77C7E;
    16
}
/// $06F9EE: move.w #$c000, $38(a0)
pub fn i_06f9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9F4;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F9F4: lea.l $2d60f6.l, a0
pub fn i_06f9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D60F6;
    20
}
/// $06F9FA: jsr $28cd4.l
pub fn i_06f9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FA00);
    cpu.pc = 0x28CD4;
    16
}
/// $06FA00: lea.l $6fa06(pc), a1
pub fn i_06fa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FA06;
    16
}
/// $06FA04: move.l a1, (a6)
pub fn i_06fa04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FA06: bset.b #$6, $13(a6)
pub fn i_06fa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA0C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FA0C: jsr $27cee.l
pub fn i_06fa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FA12);
    cpu.pc = 0x27CEE;
    16
}
/// $06FA12: bcc.w $6fa1a
pub fn i_06fa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA16;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FA1A; return 10; }
    8
}
/// $06FA16: clr.w $2a(a6)
pub fn i_06fa16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA1A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06FA1A: cmpi.w #$160, $22(a6)
pub fn i_06fa1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA20;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x160, Size::Word, false);
    16
}
/// $06FA20: blt.w $6fa30
pub fn i_06fa20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA24;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6FA30; return 10; }
    8
}
/// $06FA24: jsr $267e2.l
pub fn i_06fa24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FA2A);
    cpu.pc = 0x267E2;
    16
}
/// $06FA2A: lea.l $6fa60(pc), a1
pub fn i_06fa2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FA60;
    16
}
/// $06FA2E: move.l a1, (a6)
pub fn i_06fa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FA30: cmpi.w #$140, $24(a6)
pub fn i_06fa30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA36;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $06FA36: bgt.w $6fa40
pub fn i_06fa36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA3A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6FA40; return 10; }
    8
}
/// $06FA3A: move.w #$40, $2a(a6)
pub fn i_06fa3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA40;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FA40: cmpi.w #$170, $24(a6)
pub fn i_06fa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA46;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x170, Size::Word, false);
    16
}
/// $06FA46: blt.w $6fa50
pub fn i_06fa46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA4A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6FA50; return 10; }
    8
}
/// $06FA4A: move.w #$ffc0, $2a(a6)
pub fn i_06fa4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FA50: jsr $70ab0(pc)
pub fn i_06fa50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FA54);
    cpu.pc = 0x70AB0;
    16
}
/// $06FA54: bcc.w $6fa5e
pub fn i_06fa54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FA5E; return 10; }
    8
}
// ===== SetTaskHandler_06fa58  ($06FA58, 8 B, kernel/task_handlers.c)
/// $06FA58: lea.l $6fa60(pc), a1
pub fn i_06fa58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FA60;
    16
}
/// $06FA5C: move.l a1, (a6)
pub fn i_06fa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FA5E: rts
pub fn i_06fa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Explode_06fa60  ($06FA60, 84 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FA60: move.w $22(a6), $5c(a6)
pub fn i_06fa60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA66;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06FA66: move.w #$120, $22(a6)
pub fn i_06fa66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA6C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FA6C: move.w #$1033, d0
pub fn i_06fa6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FA70: jsr $2352.l
pub fn i_06fa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FA76);
    cpu.pc = 0x2352;
    16
}
/// $06FA76: move.b #$4, $10a2d0.l
pub fn i_06fa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA7E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D0, v1 as u8);
    16
}
/// $06FA7E: move.b #$0, $10a2d1.l
pub fn i_06fa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA86;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $06FA86: lea.l $77fd6.l, a1
pub fn i_06fa86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA8C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06FA8C: jsr $4ae.l
pub fn i_06fa8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FA92);
    cpu.pc = 0x4AE;
    16
}
/// $06FA92: jsr $5dd02.l
pub fn i_06fa92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FA98);
    cpu.pc = 0x5DD02;
    16
}
/// $06FA98: addq.w #$1, $38(a0)
pub fn i_06fa98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA9C;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06FA9C: jsr $628ea.l
pub fn i_06fa9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FAA2);
    cpu.pc = 0x628EA;
    16
}
/// $06FAA2: jsr $628ea.l
pub fn i_06faa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FAA8);
    cpu.pc = 0x628EA;
    16
}
/// $06FAA8: move.w $5c(a6), $22(a6)
pub fn i_06faa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAAE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06FAAE: lea.l $6faba(pc), a1
pub fn i_06faae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FABA;
    16
}
/// $06FAB2: move.l a1, (a6)
pub fn i_06fab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_06fab4  ($06FAB4, 6 B, thunks/jsr_pc_thunks.c)
/// $06FAB4: jsr $70ab0(pc)
pub fn i_06fab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FAB8);
    cpu.pc = 0x70AB0;
    16
}
/// $06FAB8: rts
pub fn i_06fab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FABA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_06faba  ($06FABA, 8 B, kernel/jmp_to_scheduler.c)
/// $06FABA: jmp $518.l
pub fn i_06faba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAC0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== M5Boss_HitTest_06fac2  ($06FAC2, 34 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FAC2: jsr $70c4e(pc)
pub fn i_06fac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FAC6);
    cpu.pc = 0x70C4E;
    16
}
/// $06FAC6: jsr $28758.l
pub fn i_06fac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FACC);
    cpu.pc = 0x28758;
    16
}
/// $06FACC: bcc.w $6fae2
pub fn i_06facc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAD0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FAE2; return 10; }
    8
}
/// $06FAD0: bclr.b #$0, $13(a6)
pub fn i_06fad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAD6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FAD6: move.w #$1, $66(a6)
pub fn i_06fad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FADC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FADC: move.b #$2, $73(a6)
pub fn i_06fadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAE2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06FAE2: rts
pub fn i_06fae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_HitTestB_06fae4  ($06FAE4, 14 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FAE4: jsr $70c4e(pc)
pub fn i_06fae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FAE8);
    cpu.pc = 0x70C4E;
    16
}
/// $06FAE8: jsr $28758.l
pub fn i_06fae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FAEE);
    cpu.pc = 0x28758;
    16
}
/// $06FAEE: bcc.w $6faf8
pub fn i_06faee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FAF8; return 10; }
    8
}
// ===== SetTaskHandler_06faf2  ($06FAF2, 8 B, kernel/task_handlers.c)
/// $06FAF2: lea.l $6f9ca(pc), a1
pub fn i_06faf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F9CA;
    16
}
/// $06FAF6: move.l a1, (a6)
pub fn i_06faf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FAF8: rts
pub fn i_06faf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_06fafa  ($06FAFA, 8 B, kernel/jmp_to_scheduler.c)
/// $06FAFA: jmp $518.l
pub fn i_06fafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB00;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Jsr5B6ThenJmpScheduler_06fb02  ($06FB02, 14 B, kernel/jsr5b6_jmp518.c)
/// $06FB02: jsr $5b6.l
pub fn i_06fb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FB08);
    cpu.pc = 0x5B6;
    16
}
/// $06FB08: jmp $518.l
pub fn i_06fb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB0E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== M5Boss_Part_06fb10  ($06FB10, 30 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FB10: move.w #$59, d1
pub fn i_06fb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB14;
    cpu.ir = 0x323C;
    let v1: u32 = 0x59;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FB14: jsr $236e.l
pub fn i_06fb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FB1A);
    cpu.pc = 0x236E;
    16
}
/// $06FB1A: move.w #$b, $1c(a6)
pub fn i_06fb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB20;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FB20: jsr $138fe.l
pub fn i_06fb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FB26);
    cpu.pc = 0x138FE;
    16
}
/// $06FB26: clr.b $73(a6)
pub fn i_06fb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB2A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06FB2A: clr.b $9b(a6)
pub fn i_06fb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB2E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== M5Boss_PartFollow_06fb2e  ($06FB2E, 106 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FB2E: move.b $98(a6), d0
pub fn i_06fb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB32;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FB32: andi.w #$3, d0
pub fn i_06fb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB36;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FB36: lsl.w #$2, d0
pub fn i_06fb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB38;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06FB38: move.b $73(a6), d1
pub fn i_06fb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB3C;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FB3C: andi.w #$3, d1
pub fn i_06fb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB40;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FB40: add.w d1, d0
pub fn i_06fb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB42;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06FB42: movea.l #$2d5372, a0
pub fn i_06fb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB48;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D5372;
    cpu.a[0] = v1;
    4
}
/// $06FB48: lsl.w #$2, d0
pub fn i_06fb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB4A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06FB4A: movea.l (a0, d0.w), a0
pub fn i_06fb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB4E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06FB4E: cmpa.l #$ffffffff, a0
pub fn i_06fb4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB54;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06FB54: beq.w $6fb5e
pub fn i_06fb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB58;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FB5E; return 10; }
    8
}
/// $06FB58: jsr $28cd4.l
pub fn i_06fb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FB5E);
    cpu.pc = 0x28CD4;
    16
}
/// $06FB5E: lea.l $6fb64(pc), a1
pub fn i_06fb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB64;
    16
}
/// $06FB62: move.l a1, (a6)
pub fn i_06fb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FB64: jsr $70d5a(pc)
pub fn i_06fb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FB68);
    cpu.pc = 0x70D5A;
    16
}
/// $06FB68: jsr $28d70.l
pub fn i_06fb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FB6E);
    cpu.pc = 0x28D70;
    16
}
/// $06FB6E: movea.l $c(a6), a0
pub fn i_06fb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB72;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06FB72: move.b $9b(a0), $9b(a6)
pub fn i_06fb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB78;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06FB78: move.b $73(a0), d0
pub fn i_06fb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB7C;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FB7C: cmp.b $73(a6), d0
pub fn i_06fb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB80;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06FB80: beq.w $6fb8e
pub fn i_06fb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB84;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FB8E; return 10; }
    8
}
/// $06FB84: move.b d0, $73(a6)
pub fn i_06fb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB88;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06FB88: lea.l $6fb2e(pc), a1
pub fn i_06fb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB2E;
    16
}
/// $06FB8C: move.l a1, (a6)
pub fn i_06fb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FB8E: jsr $5e45a.l
pub fn i_06fb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FB94);
    cpu.pc = 0x5E45A;
    16
}
/// $06FB94: bcc.w $6fb9e
pub fn i_06fb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB98;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FB9E; return 10; }
    8
}
// ===== SetTaskHandler_06fb98  ($06FB98, 8 B, kernel/task_handlers.c)
/// $06FB98: lea.l $6fb02(pc), a1
pub fn i_06fb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $06FB9C: move.l a1, (a6)
pub fn i_06fb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FB9E: rts
pub fn i_06fb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Rotor_06fba0  ($06FBA0, 168 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FBA0: move.w #$59, d1
pub fn i_06fba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBA4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x59;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FBA4: jsr $236e.l
pub fn i_06fba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FBAA);
    cpu.pc = 0x236E;
    16
}
/// $06FBAA: move.w #$b, $1c(a6)
pub fn i_06fbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBB0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FBB0: jsr $138fe.l
pub fn i_06fbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FBB6);
    cpu.pc = 0x138FE;
    16
}
/// $06FBB6: move.w #$3c, $70(a6)
pub fn i_06fbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBBC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FBBC: lea.l $2d61a8.l, a0
pub fn i_06fbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBC2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D61A8;
    20
}
/// $06FBC2: jsr $28cd4.l
pub fn i_06fbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FBC8);
    cpu.pc = 0x28CD4;
    16
}
/// $06FBC8: lea.l $6fbce(pc), a1
pub fn i_06fbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FBCE;
    16
}
/// $06FBCC: move.l a1, (a6)
pub fn i_06fbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FBCE: jsr $70d5a(pc)
pub fn i_06fbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FBD2);
    cpu.pc = 0x70D5A;
    16
}
/// $06FBD2: jsr $28d70.l
pub fn i_06fbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FBD8);
    cpu.pc = 0x28D70;
    16
}
/// $06FBD8: bcc.w $6fbe8
pub fn i_06fbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FBE8; return 10; }
    8
}
/// $06FBDC: lea.l $2d61a8.l, a0
pub fn i_06fbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBE2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D61A8;
    20
}
/// $06FBE2: jsr $28cd4.l
pub fn i_06fbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FBE8);
    cpu.pc = 0x28CD4;
    16
}
/// $06FBE8: movea.l $c(a6), a0
pub fn i_06fbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBEC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06FBEC: cmpi.b #$2, $8b(a0)
pub fn i_06fbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBF2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $06FBF2: bne.w $6fbfc
pub fn i_06fbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBF6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6FBFC; return 10; }
    8
}
/// $06FBF6: lea.l $6fc50(pc), a1
pub fn i_06fbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FC50;
    16
}
/// $06FBFA: move.l a1, (a6)
pub fn i_06fbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FBFC: cmpi.b #$3, $8b(a0)
pub fn i_06fbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC02;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $06FC02: bne.w $6fc0c
pub fn i_06fc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6FC0C; return 10; }
    8
}
/// $06FC06: lea.l $6fca2(pc), a1
pub fn i_06fc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FCA2;
    16
}
/// $06FC0A: move.l a1, (a6)
pub fn i_06fc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FC0C: movea.l $c(a6), a0
pub fn i_06fc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC10;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06FC10: cmpi.b #$0, $8b(a0)
pub fn i_06fc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC16;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06FC16: beq.w $6fc3e
pub fn i_06fc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC1A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FC3E; return 10; }
    8
}
/// $06FC1A: subq.w #$1, $70(a6)
pub fn i_06fc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC1E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06FC1E: cmpi.w #$0, $70(a6)
pub fn i_06fc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC24;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06FC24: bgt.w $6fc3e
pub fn i_06fc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC28;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6FC3E; return 10; }
    8
}
/// $06FC28: lea.l $708c0(pc), a1
pub fn i_06fc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x708C0;
    16
}
/// $06FC2C: jsr $4ae.l
pub fn i_06fc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FC32);
    cpu.pc = 0x4AE;
    16
}
/// $06FC32: jsr $5dd02.l
pub fn i_06fc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FC38);
    cpu.pc = 0x5DD02;
    16
}
/// $06FC38: move.w #$3c, $70(a6)
pub fn i_06fc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC3E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FC3E: jsr $5e45a.l
pub fn i_06fc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FC44);
    cpu.pc = 0x5E45A;
    16
}
/// $06FC44: bcc.w $6fc4e
pub fn i_06fc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC48;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FC4E; return 10; }
    8
}
// ===== SetTaskHandler_06fc48  ($06FC48, 8 B, kernel/task_handlers.c)
/// $06FC48: lea.l $6fb02(pc), a1
pub fn i_06fc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $06FC4C: move.l a1, (a6)
pub fn i_06fc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FC4E: rts
pub fn i_06fc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_RotorA_06fc50  ($06FC50, 82 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FC50: lea.l $2d638a.l, a0
pub fn i_06fc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC56;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D638A;
    20
}
/// $06FC56: jsr $28cd4.l
pub fn i_06fc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FC5C);
    cpu.pc = 0x28CD4;
    16
}
/// $06FC5C: lea.l $6fc62(pc), a1
pub fn i_06fc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FC62;
    16
}
/// $06FC60: move.l a1, (a6)
pub fn i_06fc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FC62: jsr $70d5a(pc)
pub fn i_06fc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FC66);
    cpu.pc = 0x70D5A;
    16
}
/// $06FC66: jsr $28d70.l
pub fn i_06fc66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FC6C);
    cpu.pc = 0x28D70;
    16
}
/// $06FC6C: bcc.w $6fc76
pub fn i_06fc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FC76; return 10; }
    8
}
/// $06FC70: lea.l $6fc50(pc), a1
pub fn i_06fc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FC50;
    16
}
/// $06FC74: move.l a1, (a6)
pub fn i_06fc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FC76: movea.l $c(a6), a0
pub fn i_06fc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC7A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06FC7A: cmpi.b #$1, $8b(a0)
pub fn i_06fc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC80;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06FC80: bne.w $6fc8a
pub fn i_06fc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6FC8A; return 10; }
    8
}
/// $06FC84: lea.l $6fbb6(pc), a1
pub fn i_06fc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FBB6;
    16
}
/// $06FC88: move.l a1, (a6)
pub fn i_06fc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FC8A: cmpi.b #$3, $8b(a0)
pub fn i_06fc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC90;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $06FC90: bne.w $6fc9a
pub fn i_06fc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC94;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6FC9A; return 10; }
    8
}
/// $06FC94: lea.l $6fca2(pc), a1
pub fn i_06fc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FCA2;
    16
}
/// $06FC98: move.l a1, (a6)
pub fn i_06fc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FC9A: subq.w #$3, $70(a6)
pub fn i_06fc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC9E;
    cpu.ir = 0x576E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06FC9E: bra.w $6fc0c
pub fn i_06fc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FC0C;
    10
}
// ===== M5Boss_RotorB_06fca2  ($06FCA2, 88 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FCA2: lea.l $2d638a.l, a0
pub fn i_06fca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D638A;
    20
}
/// $06FCA8: jsr $28cd4.l
pub fn i_06fca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FCAE);
    cpu.pc = 0x28CD4;
    16
}
/// $06FCAE: lea.l $6fcb4(pc), a1
pub fn i_06fcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FCB4;
    16
}
/// $06FCB2: move.l a1, (a6)
pub fn i_06fcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FCB4: jsr $70d5a(pc)
pub fn i_06fcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FCB8);
    cpu.pc = 0x70D5A;
    16
}
/// $06FCB8: jsr $28d70.l
pub fn i_06fcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FCBE);
    cpu.pc = 0x28D70;
    16
}
/// $06FCBE: bcc.w $6fcce
pub fn i_06fcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FCCE; return 10; }
    8
}
/// $06FCC2: lea.l $2d638a.l, a0
pub fn i_06fcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D638A;
    20
}
/// $06FCC8: jsr $28cd4.l
pub fn i_06fcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FCCE);
    cpu.pc = 0x28CD4;
    16
}
/// $06FCCE: movea.l $c(a6), a0
pub fn i_06fcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCD2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06FCD2: cmpi.b #$1, $8b(a0)
pub fn i_06fcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCD8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06FCD8: bne.w $6fce2
pub fn i_06fcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCDC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6FCE2; return 10; }
    8
}
/// $06FCDC: lea.l $6fbb6(pc), a1
pub fn i_06fcdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FBB6;
    16
}
/// $06FCE0: move.l a1, (a6)
pub fn i_06fce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FCE2: cmpi.b #$2, $8b(a0)
pub fn i_06fce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCE8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $06FCE8: bne.w $6fcf2
pub fn i_06fce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6FCF2; return 10; }
    8
}
/// $06FCEC: lea.l $6fc50(pc), a1
pub fn i_06fcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FC50;
    16
}
/// $06FCF0: move.l a1, (a6)
pub fn i_06fcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FCF2: subq.w #$7, $70(a6)
pub fn i_06fcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCF6;
    cpu.ir = 0x5F6E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 7, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06FCF6: bra.w $6fc0c
pub fn i_06fcf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FC0C;
    10
}
// ===== M5Boss_Bomb_06fcfa  ($06FCFA, 238 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FCFA: move.w #$10a3, d0
pub fn i_06fcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCFE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FCFE: jsr $2352.l
pub fn i_06fcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FD04);
    cpu.pc = 0x2352;
    16
}
/// $06FD04: bset.b #$4, $6b(a6)
pub fn i_06fd04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD0A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FD0A: lea.l $2b9c24.l, a0
pub fn i_06fd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9C24;
    20
}
/// $06FD10: jsr $799de.l
pub fn i_06fd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FD16);
    cpu.pc = 0x799DE;
    16
}
/// $06FD16: move.w d0, $66(a6)
pub fn i_06fd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD1A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FD1A: move.w d0, d1
pub fn i_06fd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD1C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FD1C: lsr.w #$2, d1
pub fn i_06fd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD1E;
    cpu.ir = 0xE449;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06FD1E: move.w d1, $7c(a6)
pub fn i_06fd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD22;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FD22: sub.w d1, d0
pub fn i_06fd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD24;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06FD24: move.w d0, $7a(a6)
pub fn i_06fd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD28;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FD28: move.w #$5a, d1
pub fn i_06fd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD2C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FD2C: jsr $236e.l
pub fn i_06fd2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FD32);
    cpu.pc = 0x236E;
    16
}
/// $06FD32: move.w #$b, $1c(a6)
pub fn i_06fd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD38;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FD38: jsr $138fe.l
pub fn i_06fd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FD3E);
    cpu.pc = 0x138FE;
    16
}
/// $06FD3E: clr.b $84(a6)
pub fn i_06fd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD42;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06FD42: clr.b $85(a6)
pub fn i_06fd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD46;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x85);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06FD46: clr.b $89(a6)
pub fn i_06fd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD4A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x89);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06FD4A: move.b $87(a6), d0
pub fn i_06fd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD4E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FD4E: andi.w #$1, d0
pub fn i_06fd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD52;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FD52: tst.b $9b(a6)
pub fn i_06fd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD56;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06FD56: beq.w $6fd5c
pub fn i_06fd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD5A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FD5C; return 10; }
    8
}
/// $06FD5A: addq.w #$2, d0
pub fn i_06fd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD5C;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06FD5C: movea.l #$2d53c2, a0
pub fn i_06fd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD62;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D53C2;
    cpu.a[0] = v1;
    4
}
/// $06FD62: lsl.w #$2, d0
pub fn i_06fd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD64;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06FD64: movea.l (a0, d0.w), a0
pub fn i_06fd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD68;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06FD68: cmpa.l #$ffffffff, a0
pub fn i_06fd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD6E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06FD6E: beq.w $6fd78
pub fn i_06fd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD72;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FD78; return 10; }
    8
}
/// $06FD72: jsr $28cd4.l
pub fn i_06fd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FD78);
    cpu.pc = 0x28CD4;
    16
}
/// $06FD78: lea.l $6fd7e(pc), a1
pub fn i_06fd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FD7E;
    16
}
/// $06FD7C: move.l a1, (a6)
pub fn i_06fd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FD7E: movea.l $c(a6), a0
pub fn i_06fd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD82;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06FD82: move.w $22(a0), d1
pub fn i_06fd82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD86;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FD86: move.w $24(a0), d2
pub fn i_06fd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD8A;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FD8A: move.b $84(a6), d3
pub fn i_06fd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD8E;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FD8E: move.b $85(a6), d4
pub fn i_06fd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD92;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FD92: ext.w d3
pub fn i_06fd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD94;
    cpu.ir = 0x4883;
    let v = cpu.d[3] as i8 as i16 as u16 as u32; cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06FD94: ext.w d4
pub fn i_06fd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD96;
    cpu.ir = 0x4884;
    let v = cpu.d[4] as i8 as i16 as u16 as u32; cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06FD96: add.w d3, d1
pub fn i_06fd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD98;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06FD98: add.w d4, d2
pub fn i_06fd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD9A;
    cpu.ir = 0xD444;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06FD9A: addi.w #$2c, d2
pub fn i_06fd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD9E;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2C, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FD9E: move.w d1, $22(a6)
pub fn i_06fd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDA2;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FDA2: move.w d2, $24(a6)
pub fn i_06fda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDA6;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FDA6: move.w $38(a0), d0
pub fn i_06fda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDAA;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FDAA: subq.w #$2, d0
pub fn i_06fdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDAC;
    cpu.ir = 0x5540;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06FDAC: move.w d0, $38(a6)
pub fn i_06fdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDB0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FDB0: jsr $28d70.l
pub fn i_06fdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FDB6);
    cpu.pc = 0x28D70;
    16
}
/// $06FDB6: bcc.w $6fde6
pub fn i_06fdb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FDE6; return 10; }
    8
}
/// $06FDBA: lea.l $6fde8(pc), a1
pub fn i_06fdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FDE8;
    16
}
/// $06FDBE: move.l a1, (a6)
pub fn i_06fdbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FDC0: cmpi.b #$4, $88(a6)
pub fn i_06fdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDC6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $06FDC6: bcs.w $6fdd0
pub fn i_06fdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDCA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6FDD0; return 10; }
    8
}
/// $06FDCA: lea.l $6ffca(pc), a1
pub fn i_06fdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FFCA;
    16
}
/// $06FDCE: move.l a1, (a6)
pub fn i_06fdce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FDD0: move.w #$8000, d0
pub fn i_06fdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDD4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FDD4: jsr $28134.l
pub fn i_06fdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FDDA);
    cpu.pc = 0x28134;
    16
}
/// $06FDDA: andi.w #$ffe3, $38(a6)
pub fn i_06fdda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDE0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06FDE0: ori.w #$1c, $38(a6)
pub fn i_06fde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDE6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06FDE6: rts
pub fn i_06fde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_BombFall_06fde8  ($06FDE8, 256 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FDE8: jsr $267e2.l
pub fn i_06fde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FDEE);
    cpu.pc = 0x267E2;
    16
}
/// $06FDEE: lea.l $2b9ca6.l, a0
pub fn i_06fdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9CA6;
    20
}
/// $06FDF4: jsr $799de.l
pub fn i_06fdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FDFA);
    cpu.pc = 0x799DE;
    16
}
/// $06FDFA: move.w d0, $36(a6)
pub fn i_06fdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDFE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FDFE: tst.b $34(a6)
pub fn i_06fdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE02;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06FE02: beq.w $6fe08
pub fn i_06fe02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE06;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FE08; return 10; }
    8
}
/// $06FE06: neg.w d0
pub fn i_06fe06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE08;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06FE08: move.w d0, $28(a6)
pub fn i_06fe08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE0C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FE0C: move.b $88(a6), d0
pub fn i_06fe0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE10;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FE10: andi.w #$3, d0
pub fn i_06fe10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE14;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FE14: lea.l $2d545e.l, a0
pub fn i_06fe14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE1A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D545E;
    20
}
/// $06FE1A: tst.b $34(a6)
pub fn i_06fe1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE1E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06FE1E: beq.w $6fe28
pub fn i_06fe1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE22;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FE28; return 10; }
    8
}
/// $06FE22: lea.l $2d546e.l, a0
pub fn i_06fe22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D546E;
    20
}
/// $06FE28: lsl.w #$2, d0
pub fn i_06fe28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE2A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06FE2A: move.l (a0, d0.w), $90(a6)
pub fn i_06fe2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE30;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $06FE30: clr.w $94(a6)
pub fn i_06fe30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE34;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06FE34: clr.b $96(a6)
pub fn i_06fe34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE38;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x96);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06FE38: lea.l $2d6d70.l, a0
pub fn i_06fe38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE3E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D6D70;
    20
}
/// $06FE3E: jsr $28cd4.l
pub fn i_06fe3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FE44);
    cpu.pc = 0x28CD4;
    16
}
/// $06FE44: lea.l $6fe4a(pc), a1
pub fn i_06fe44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FE4A;
    16
}
/// $06FE48: move.l a1, (a6)
pub fn i_06fe48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FE4A: jsr $78f90.l
pub fn i_06fe4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FE50);
    cpu.pc = 0x78F90;
    16
}
/// $06FE50: move.b $34(a6), d0
pub fn i_06fe50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE54;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FE54: andi.w #$ff, d0
pub fn i_06fe54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE58;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FE58: lsr.w #$3, d0
pub fn i_06fe58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE5A;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06FE5A: move.w d0, $76(a6)
pub fn i_06fe5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE5E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FE5E: jsr $28d70.l
pub fn i_06fe5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FE64);
    cpu.pc = 0x28D70;
    16
}
/// $06FE64: jsr $283d8.l
pub fn i_06fe64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FE6A);
    cpu.pc = 0x283D8;
    16
}
/// $06FE6A: btst.b #$1, $13(a6)
pub fn i_06fe6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE70;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06FE70: beq.w $6fe7a
pub fn i_06fe70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE74;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FE7A; return 10; }
    8
}
/// $06FE74: lea.l $70074(pc), a1
pub fn i_06fe74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70074;
    16
}
/// $06FE78: move.l a1, (a6)
pub fn i_06fe78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FE7A: jsr $2870a.l
pub fn i_06fe7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FE80);
    cpu.pc = 0x2870A;
    16
}
/// $06FE80: bcc.w $6fec2
pub fn i_06fe80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FEC2; return 10; }
    8
}
/// $06FE84: bclr.b #$3, $13(a6)
pub fn i_06fe84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE8A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FE8A: move.w $7a(a6), d0
pub fn i_06fe8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE8E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FE8E: cmp.w $66(a6), d0
pub fn i_06fe8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE92;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06FE92: bcs.w $6fec2
pub fn i_06fe92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE96;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6FEC2; return 10; }
    8
}
/// $06FE96: sub.w $7c(a6), d0
pub fn i_06fe96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE9A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06FE9A: move.w d0, $7a(a6)
pub fn i_06fe9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE9E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FE9E: jsr $13600.l
pub fn i_06fe9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FEA4);
    cpu.pc = 0x13600;
    16
}
/// $06FEA4: addq.b #$1, $73(a6)
pub fn i_06fea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEA8;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06FEA8: move.b $73(a6), d0
pub fn i_06fea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEAC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FEAC: andi.w #$7, d0
pub fn i_06feac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEB0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FEB0: add.w d0, d0
pub fn i_06feb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEB2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06FEB2: lea.l $2d552e.l, a0
pub fn i_06feb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D552E;
    20
}
/// $06FEB8: move.w (a0, d0.w), d1
pub fn i_06feb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEBC;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06FEBC: jsr $236e.l
pub fn i_06febc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FEC2);
    cpu.pc = 0x236E;
    16
}
/// $06FEC2: jsr $28758.l
pub fn i_06fec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FEC8);
    cpu.pc = 0x28758;
    16
}
/// $06FEC8: bcc.w $6fed2
pub fn i_06fec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FECC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FED2; return 10; }
    8
}
/// $06FECC: lea.l $70074(pc), a1
pub fn i_06fecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FED0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70074;
    16
}
/// $06FED0: move.l a1, (a6)
pub fn i_06fed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FED2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FED2: movea.l #$ffffffff, a0
pub fn i_06fed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FED8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06FED8: lea.l $2d5316.l, a0
pub fn i_06fed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEDE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D5316;
    20
}
/// $06FEDE: jsr $5dd56.l
pub fn i_06fede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FEE4);
    cpu.pc = 0x5DD56;
    16
}
/// $06FEE4: bcc.w $6feee
pub fn i_06fee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEE8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FEEE; return 10; }
    8
}
// ===== SetTaskHandler_06fee8  ($06FEE8, 8 B, kernel/task_handlers.c)
/// $06FEE8: lea.l $700a6(pc), a1
pub fn i_06fee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x700A6;
    16
}
/// $06FEEC: move.l a1, (a6)
pub fn i_06feec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FEEE: rts
pub fn i_06feee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_BombBounce_06fef0  ($06FEF0, 66 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FEF0: jsr $267e2.l
pub fn i_06fef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FEF6);
    cpu.pc = 0x267E2;
    16
}
/// $06FEF6: move.w #$ffa0, $2e(a6)
pub fn i_06fef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEFC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FEFC: lea.l $6ff02(pc), a1
pub fn i_06fefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FF02;
    16
}
/// $06FF00: move.l a1, (a6)
pub fn i_06ff00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF02: bset.b #$6, $13(a6)
pub fn i_06ff02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF08;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FF08: jsr $27d50.l
pub fn i_06ff08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FF0E);
    cpu.pc = 0x27D50;
    16
}
/// $06FF0E: bcc.w $6ff18
pub fn i_06ff0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FF18; return 10; }
    8
}
/// $06FF12: lea.l $70074(pc), a1
pub fn i_06ff12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70074;
    16
}
/// $06FF16: move.l a1, (a6)
pub fn i_06ff16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF18: jsr $28d70.l
pub fn i_06ff18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FF1E);
    cpu.pc = 0x28D70;
    16
}
/// $06FF1E: cmpi.w #$150, $24(a6)
pub fn i_06ff1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF24;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $06FF24: bgt.w $6ff2e
pub fn i_06ff24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF28;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6FF2E; return 10; }
    8
}
/// $06FF28: lea.l $70074(pc), a1
pub fn i_06ff28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70074;
    16
}
/// $06FF2C: move.l a1, (a6)
pub fn i_06ff2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF2E: bra.w $6fe64
pub fn i_06ff2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF32;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FE64;
    10
}
// ===== M5Boss_Missile_06ff32  ($06FF32, 64 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FF32: lea.l $6ff38(pc), a1
pub fn i_06ff32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FF38;
    16
}
/// $06FF36: move.l a1, (a6)
pub fn i_06ff36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF38: bset.b #$6, $13(a6)
pub fn i_06ff38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF3E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FF3E: jsr $27cee.l
pub fn i_06ff3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FF44);
    cpu.pc = 0x27CEE;
    16
}
/// $06FF44: jsr $28d70.l
pub fn i_06ff44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FF4A);
    cpu.pc = 0x28D70;
    16
}
/// $06FF4A: jsr $5e1ea.l
pub fn i_06ff4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FF50);
    cpu.pc = 0x5E1EA;
    16
}
/// $06FF50: move.w $22(a6), d0
pub fn i_06ff50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF54;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FF54: sub.w $22(a0), d0
pub fn i_06ff54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF58;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06FF58: cmpi.w #$40, d0
pub fn i_06ff58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF5C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $06FF5C: bgt.w $6ff6e
pub fn i_06ff5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF60;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6FF6E; return 10; }
    8
}
/// $06FF60: cmpi.w #$ffc0, d0
pub fn i_06ff60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF64;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFC0, Size::Word, false);
    8
}
/// $06FF64: blt.w $6ff6e
pub fn i_06ff64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF68;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6FF6E; return 10; }
    8
}
/// $06FF68: lea.l $6ff72(pc), a1
pub fn i_06ff68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FF72;
    16
}
/// $06FF6C: move.l a1, (a6)
pub fn i_06ff6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF6E: bra.w $6fe64
pub fn i_06ff6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF72;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FE64;
    10
}
// ===== M5Boss_MissileHome_06ff72  ($06FF72, 88 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FF72: move.w #$ff80, $2e(a6)
pub fn i_06ff72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF78;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FF78: lea.l $6ff7e(pc), a1
pub fn i_06ff78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FF7E;
    16
}
/// $06FF7C: move.l a1, (a6)
pub fn i_06ff7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF7E: bset.b #$6, $13(a6)
pub fn i_06ff7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF84;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FF84: jsr $27d50.l
pub fn i_06ff84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FF8A);
    cpu.pc = 0x27D50;
    16
}
/// $06FF8A: bcc.w $6ff98
pub fn i_06ff8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FF98; return 10; }
    8
}
/// $06FF8E: lea.l $70074(pc), a1
pub fn i_06ff8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70074;
    16
}
/// $06FF92: move.l a1, (a6)
pub fn i_06ff92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF94: bra.w $6ffb0
pub fn i_06ff94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF98;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FFB0;
    10
}
/// $06FF98: move.w $28(a6), d0
pub fn i_06ff98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF9C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FF9C: move.w $2a(a6), d1
pub fn i_06ff9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFA0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FFA0: asr.w #$4, d0
pub fn i_06ffa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFA2;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06FFA2: asr.w #$4, d1
pub fn i_06ffa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFA4;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06FFA4: jsr $5e018.l
pub fn i_06ffa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FFAA);
    cpu.pc = 0x5E018;
    16
}
/// $06FFAA: lsr.w #$3, d0
pub fn i_06ffaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFAC;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06FFAC: move.w d0, $76(a6)
pub fn i_06ffac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFB0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FFB0: jsr $28d70.l
pub fn i_06ffb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FFB6);
    cpu.pc = 0x28D70;
    16
}
/// $06FFB6: cmpi.w #$150, $24(a6)
pub fn i_06ffb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFBC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $06FFBC: bgt.w $6ffc6
pub fn i_06ffbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFC0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6FFC6; return 10; }
    8
}
/// $06FFC0: lea.l $70074(pc), a1
pub fn i_06ffc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70074;
    16
}
/// $06FFC4: move.l a1, (a6)
pub fn i_06ffc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FFC6: bra.w $6fe64
pub fn i_06ffc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFCA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FE64;
    10
}
// ===== M5Boss_Shell_06ffca  ($06FFCA, 124 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FFCA: jsr $5e9b6.l
pub fn i_06ffca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FFD0);
    cpu.pc = 0x5E9B6;
    16
}
/// $06FFD0: andi.w #$20, d0
pub fn i_06ffd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFD4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x20) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FFD4: addi.w #$10, d0
pub fn i_06ffd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFD8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FFD8: move.w d0, $82(a6)
pub fn i_06ffd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFDC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FFDC: move.w #$80, d0
pub fn i_06ffdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFE0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FFE0: tst.b $34(a6)
pub fn i_06ffe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFE4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06FFE4: beq.w $6ffec
pub fn i_06ffe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FFEC; return 10; }
    8
}
/// $06FFE8: move.w #$ff80, d0
pub fn i_06ffe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFEC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FFEC: move.w d0, $28(a6)
pub fn i_06ffec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFF0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FFF0: move.b $34(a6), d0
pub fn i_06fff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFF4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FFF4: andi.w #$ff, d0
pub fn i_06fff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFF8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FFF8: lsr.w #$3, d0
pub fn i_06fff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFFA;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06FFFA: move.w d0, $76(a6)
pub fn i_06fffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFFE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FFFE: move.w #$fa00, $2a(a6)
pub fn i_06fffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70004;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFA00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x06F7B2, i_06f7b2),
    (0x06F7B4, i_06f7b4),
    (0x06F7B8, i_06f7b8),
    (0x06F7BE, i_06f7be),
    (0x06F7C4, i_06f7c4),
    (0x06F7C8, i_06f7c8),
    (0x06F7CA, i_06f7ca),
    (0x06F7D0, i_06f7d0),
    (0x06F7D4, i_06f7d4),
    (0x06F7D8, i_06f7d8),
    (0x06F7DC, i_06f7dc),
    (0x06F7E2, i_06f7e2),
    (0x06F7E6, i_06f7e6),
    (0x06F7EA, i_06f7ea),
    (0x06F7EC, i_06f7ec),
    (0x06F7F0, i_06f7f0),
    (0x06F7F4, i_06f7f4),
    (0x06F7FA, i_06f7fa),
    (0x06F800, i_06f800),
    (0x06F804, i_06f804),
    (0x06F80A, i_06f80a),
    (0x06F810, i_06f810),
    (0x06F816, i_06f816),
    (0x06F81A, i_06f81a),
    (0x06F81C, i_06f81c),
    (0x06F822, i_06f822),
    (0x06F826, i_06f826),
    (0x06F82A, i_06f82a),
    (0x06F830, i_06f830),
    (0x06F834, i_06f834),
    (0x06F838, i_06f838),
    (0x06F83C, i_06f83c),
    (0x06F83E, i_06f83e),
    (0x06F842, i_06f842),
    (0x06F848, i_06f848),
    (0x06F84E, i_06f84e),
    (0x06F852, i_06f852),
    (0x06F858, i_06f858),
    (0x06F85C, i_06f85c),
    (0x06F85E, i_06f85e),
    (0x06F864, i_06f864),
    (0x06F868, i_06f868),
    (0x06F86C, i_06f86c),
    (0x06F872, i_06f872),
    (0x06F876, i_06f876),
    (0x06F87C, i_06f87c),
    (0x06F880, i_06f880),
    (0x06F886, i_06f886),
    (0x06F88A, i_06f88a),
    (0x06F88E, i_06f88e),
    (0x06F894, i_06f894),
    (0x06F898, i_06f898),
    (0x06F89C, i_06f89c),
    (0x06F8A2, i_06f8a2),
    (0x06F8A6, i_06f8a6),
    (0x06F8AA, i_06f8aa),
    (0x06F8AC, i_06f8ac),
    (0x06F8B2, i_06f8b2),
    (0x06F8B6, i_06f8b6),
    (0x06F8BA, i_06f8ba),
    (0x06F8BC, i_06f8bc),
    (0x06F8C0, i_06f8c0),
    (0x06F8C4, i_06f8c4),
    (0x06F8CA, i_06f8ca),
    (0x06F8D0, i_06f8d0),
    (0x06F8D6, i_06f8d6),
    (0x06F8DA, i_06f8da),
    (0x06F8DC, i_06f8dc),
    (0x06F8E2, i_06f8e2),
    (0x06F8E6, i_06f8e6),
    (0x06F8EA, i_06f8ea),
    (0x06F8F0, i_06f8f0),
    (0x06F8F4, i_06f8f4),
    (0x06F8F8, i_06f8f8),
    (0x06F8FC, i_06f8fc),
    (0x06F8FE, i_06f8fe),
    (0x06F902, i_06f902),
    (0x06F906, i_06f906),
    (0x06F90C, i_06f90c),
    (0x06F912, i_06f912),
    (0x06F916, i_06f916),
    (0x06F91C, i_06f91c),
    (0x06F922, i_06f922),
    (0x06F928, i_06f928),
    (0x06F92C, i_06f92c),
    (0x06F92E, i_06f92e),
    (0x06F934, i_06f934),
    (0x06F938, i_06f938),
    (0x06F93C, i_06f93c),
    (0x06F942, i_06f942),
    (0x06F946, i_06f946),
    (0x06F94A, i_06f94a),
    (0x06F94E, i_06f94e),
    (0x06F950, i_06f950),
    (0x06F954, i_06f954),
    (0x06F95A, i_06f95a),
    (0x06F960, i_06f960),
    (0x06F966, i_06f966),
    (0x06F96C, i_06f96c),
    (0x06F970, i_06f970),
    (0x06F972, i_06f972),
    (0x06F978, i_06f978),
    (0x06F97C, i_06f97c),
    (0x06F980, i_06f980),
    (0x06F986, i_06f986),
    (0x06F98C, i_06f98c),
    (0x06F990, i_06f990),
    (0x06F994, i_06f994),
    (0x06F99A, i_06f99a),
    (0x06F99E, i_06f99e),
    (0x06F9A6, i_06f9a6),
    (0x06F9AA, i_06f9aa),
    (0x06F9AE, i_06f9ae),
    (0x06F9B2, i_06f9b2),
    (0x06F9B6, i_06f9b6),
    (0x06F9BC, i_06f9bc),
    (0x06F9C0, i_06f9c0),
    (0x06F9C4, i_06f9c4),
    (0x06F9C6, i_06f9c6),
    (0x06F9CA, i_06f9ca),
    (0x06F9D0, i_06f9d0),
    (0x06F9D6, i_06f9d6),
    (0x06F9DC, i_06f9dc),
    (0x06F9E2, i_06f9e2),
    (0x06F9E8, i_06f9e8),
    (0x06F9EE, i_06f9ee),
    (0x06F9F4, i_06f9f4),
    (0x06F9FA, i_06f9fa),
    (0x06FA00, i_06fa00),
    (0x06FA04, i_06fa04),
    (0x06FA06, i_06fa06),
    (0x06FA0C, i_06fa0c),
    (0x06FA12, i_06fa12),
    (0x06FA16, i_06fa16),
    (0x06FA1A, i_06fa1a),
    (0x06FA20, i_06fa20),
    (0x06FA24, i_06fa24),
    (0x06FA2A, i_06fa2a),
    (0x06FA2E, i_06fa2e),
    (0x06FA30, i_06fa30),
    (0x06FA36, i_06fa36),
    (0x06FA3A, i_06fa3a),
    (0x06FA40, i_06fa40),
    (0x06FA46, i_06fa46),
    (0x06FA4A, i_06fa4a),
    (0x06FA50, i_06fa50),
    (0x06FA54, i_06fa54),
    (0x06FA58, i_06fa58),
    (0x06FA5C, i_06fa5c),
    (0x06FA5E, i_06fa5e),
    (0x06FA60, i_06fa60),
    (0x06FA66, i_06fa66),
    (0x06FA6C, i_06fa6c),
    (0x06FA70, i_06fa70),
    (0x06FA76, i_06fa76),
    (0x06FA7E, i_06fa7e),
    (0x06FA86, i_06fa86),
    (0x06FA8C, i_06fa8c),
    (0x06FA92, i_06fa92),
    (0x06FA98, i_06fa98),
    (0x06FA9C, i_06fa9c),
    (0x06FAA2, i_06faa2),
    (0x06FAA8, i_06faa8),
    (0x06FAAE, i_06faae),
    (0x06FAB2, i_06fab2),
    (0x06FAB4, i_06fab4),
    (0x06FAB8, i_06fab8),
    (0x06FABA, i_06faba),
    (0x06FAC2, i_06fac2),
    (0x06FAC6, i_06fac6),
    (0x06FACC, i_06facc),
    (0x06FAD0, i_06fad0),
    (0x06FAD6, i_06fad6),
    (0x06FADC, i_06fadc),
    (0x06FAE2, i_06fae2),
    (0x06FAE4, i_06fae4),
    (0x06FAE8, i_06fae8),
    (0x06FAEE, i_06faee),
    (0x06FAF2, i_06faf2),
    (0x06FAF6, i_06faf6),
    (0x06FAF8, i_06faf8),
    (0x06FAFA, i_06fafa),
    (0x06FB02, i_06fb02),
    (0x06FB08, i_06fb08),
    (0x06FB10, i_06fb10),
    (0x06FB14, i_06fb14),
    (0x06FB1A, i_06fb1a),
    (0x06FB20, i_06fb20),
    (0x06FB26, i_06fb26),
    (0x06FB2A, i_06fb2a),
    (0x06FB2E, i_06fb2e),
    (0x06FB32, i_06fb32),
    (0x06FB36, i_06fb36),
    (0x06FB38, i_06fb38),
    (0x06FB3C, i_06fb3c),
    (0x06FB40, i_06fb40),
    (0x06FB42, i_06fb42),
    (0x06FB48, i_06fb48),
    (0x06FB4A, i_06fb4a),
    (0x06FB4E, i_06fb4e),
    (0x06FB54, i_06fb54),
    (0x06FB58, i_06fb58),
    (0x06FB5E, i_06fb5e),
    (0x06FB62, i_06fb62),
    (0x06FB64, i_06fb64),
    (0x06FB68, i_06fb68),
    (0x06FB6E, i_06fb6e),
    (0x06FB72, i_06fb72),
    (0x06FB78, i_06fb78),
    (0x06FB7C, i_06fb7c),
    (0x06FB80, i_06fb80),
    (0x06FB84, i_06fb84),
    (0x06FB88, i_06fb88),
    (0x06FB8C, i_06fb8c),
    (0x06FB8E, i_06fb8e),
    (0x06FB94, i_06fb94),
    (0x06FB98, i_06fb98),
    (0x06FB9C, i_06fb9c),
    (0x06FB9E, i_06fb9e),
    (0x06FBA0, i_06fba0),
    (0x06FBA4, i_06fba4),
    (0x06FBAA, i_06fbaa),
    (0x06FBB0, i_06fbb0),
    (0x06FBB6, i_06fbb6),
    (0x06FBBC, i_06fbbc),
    (0x06FBC2, i_06fbc2),
    (0x06FBC8, i_06fbc8),
    (0x06FBCC, i_06fbcc),
    (0x06FBCE, i_06fbce),
    (0x06FBD2, i_06fbd2),
    (0x06FBD8, i_06fbd8),
    (0x06FBDC, i_06fbdc),
    (0x06FBE2, i_06fbe2),
    (0x06FBE8, i_06fbe8),
    (0x06FBEC, i_06fbec),
    (0x06FBF2, i_06fbf2),
    (0x06FBF6, i_06fbf6),
    (0x06FBFA, i_06fbfa),
    (0x06FBFC, i_06fbfc),
    (0x06FC02, i_06fc02),
    (0x06FC06, i_06fc06),
    (0x06FC0A, i_06fc0a),
    (0x06FC0C, i_06fc0c),
    (0x06FC10, i_06fc10),
    (0x06FC16, i_06fc16),
    (0x06FC1A, i_06fc1a),
    (0x06FC1E, i_06fc1e),
    (0x06FC24, i_06fc24),
    (0x06FC28, i_06fc28),
    (0x06FC2C, i_06fc2c),
    (0x06FC32, i_06fc32),
    (0x06FC38, i_06fc38),
    (0x06FC3E, i_06fc3e),
    (0x06FC44, i_06fc44),
    (0x06FC48, i_06fc48),
    (0x06FC4C, i_06fc4c),
    (0x06FC4E, i_06fc4e),
    (0x06FC50, i_06fc50),
    (0x06FC56, i_06fc56),
    (0x06FC5C, i_06fc5c),
    (0x06FC60, i_06fc60),
    (0x06FC62, i_06fc62),
    (0x06FC66, i_06fc66),
    (0x06FC6C, i_06fc6c),
    (0x06FC70, i_06fc70),
    (0x06FC74, i_06fc74),
    (0x06FC76, i_06fc76),
    (0x06FC7A, i_06fc7a),
    (0x06FC80, i_06fc80),
    (0x06FC84, i_06fc84),
    (0x06FC88, i_06fc88),
    (0x06FC8A, i_06fc8a),
    (0x06FC90, i_06fc90),
    (0x06FC94, i_06fc94),
    (0x06FC98, i_06fc98),
    (0x06FC9A, i_06fc9a),
    (0x06FC9E, i_06fc9e),
    (0x06FCA2, i_06fca2),
    (0x06FCA8, i_06fca8),
    (0x06FCAE, i_06fcae),
    (0x06FCB2, i_06fcb2),
    (0x06FCB4, i_06fcb4),
    (0x06FCB8, i_06fcb8),
    (0x06FCBE, i_06fcbe),
    (0x06FCC2, i_06fcc2),
    (0x06FCC8, i_06fcc8),
    (0x06FCCE, i_06fcce),
    (0x06FCD2, i_06fcd2),
    (0x06FCD8, i_06fcd8),
    (0x06FCDC, i_06fcdc),
    (0x06FCE0, i_06fce0),
    (0x06FCE2, i_06fce2),
    (0x06FCE8, i_06fce8),
    (0x06FCEC, i_06fcec),
    (0x06FCF0, i_06fcf0),
    (0x06FCF2, i_06fcf2),
    (0x06FCF6, i_06fcf6),
    (0x06FCFA, i_06fcfa),
    (0x06FCFE, i_06fcfe),
    (0x06FD04, i_06fd04),
    (0x06FD0A, i_06fd0a),
    (0x06FD10, i_06fd10),
    (0x06FD16, i_06fd16),
    (0x06FD1A, i_06fd1a),
    (0x06FD1C, i_06fd1c),
    (0x06FD1E, i_06fd1e),
    (0x06FD22, i_06fd22),
    (0x06FD24, i_06fd24),
    (0x06FD28, i_06fd28),
    (0x06FD2C, i_06fd2c),
    (0x06FD32, i_06fd32),
    (0x06FD38, i_06fd38),
    (0x06FD3E, i_06fd3e),
    (0x06FD42, i_06fd42),
    (0x06FD46, i_06fd46),
    (0x06FD4A, i_06fd4a),
    (0x06FD4E, i_06fd4e),
    (0x06FD52, i_06fd52),
    (0x06FD56, i_06fd56),
    (0x06FD5A, i_06fd5a),
    (0x06FD5C, i_06fd5c),
    (0x06FD62, i_06fd62),
    (0x06FD64, i_06fd64),
    (0x06FD68, i_06fd68),
    (0x06FD6E, i_06fd6e),
    (0x06FD72, i_06fd72),
    (0x06FD78, i_06fd78),
    (0x06FD7C, i_06fd7c),
    (0x06FD7E, i_06fd7e),
    (0x06FD82, i_06fd82),
    (0x06FD86, i_06fd86),
    (0x06FD8A, i_06fd8a),
    (0x06FD8E, i_06fd8e),
    (0x06FD92, i_06fd92),
    (0x06FD94, i_06fd94),
    (0x06FD96, i_06fd96),
    (0x06FD98, i_06fd98),
    (0x06FD9A, i_06fd9a),
    (0x06FD9E, i_06fd9e),
    (0x06FDA2, i_06fda2),
    (0x06FDA6, i_06fda6),
    (0x06FDAA, i_06fdaa),
    (0x06FDAC, i_06fdac),
    (0x06FDB0, i_06fdb0),
    (0x06FDB6, i_06fdb6),
    (0x06FDBA, i_06fdba),
    (0x06FDBE, i_06fdbe),
    (0x06FDC0, i_06fdc0),
    (0x06FDC6, i_06fdc6),
    (0x06FDCA, i_06fdca),
    (0x06FDCE, i_06fdce),
    (0x06FDD0, i_06fdd0),
    (0x06FDD4, i_06fdd4),
    (0x06FDDA, i_06fdda),
    (0x06FDE0, i_06fde0),
    (0x06FDE6, i_06fde6),
    (0x06FDE8, i_06fde8),
    (0x06FDEE, i_06fdee),
    (0x06FDF4, i_06fdf4),
    (0x06FDFA, i_06fdfa),
    (0x06FDFE, i_06fdfe),
    (0x06FE02, i_06fe02),
    (0x06FE06, i_06fe06),
    (0x06FE08, i_06fe08),
    (0x06FE0C, i_06fe0c),
    (0x06FE10, i_06fe10),
    (0x06FE14, i_06fe14),
    (0x06FE1A, i_06fe1a),
    (0x06FE1E, i_06fe1e),
    (0x06FE22, i_06fe22),
    (0x06FE28, i_06fe28),
    (0x06FE2A, i_06fe2a),
    (0x06FE30, i_06fe30),
    (0x06FE34, i_06fe34),
    (0x06FE38, i_06fe38),
    (0x06FE3E, i_06fe3e),
    (0x06FE44, i_06fe44),
    (0x06FE48, i_06fe48),
    (0x06FE4A, i_06fe4a),
    (0x06FE50, i_06fe50),
    (0x06FE54, i_06fe54),
    (0x06FE58, i_06fe58),
    (0x06FE5A, i_06fe5a),
    (0x06FE5E, i_06fe5e),
    (0x06FE64, i_06fe64),
    (0x06FE6A, i_06fe6a),
    (0x06FE70, i_06fe70),
    (0x06FE74, i_06fe74),
    (0x06FE78, i_06fe78),
    (0x06FE7A, i_06fe7a),
    (0x06FE80, i_06fe80),
    (0x06FE84, i_06fe84),
    (0x06FE8A, i_06fe8a),
    (0x06FE8E, i_06fe8e),
    (0x06FE92, i_06fe92),
    (0x06FE96, i_06fe96),
    (0x06FE9A, i_06fe9a),
    (0x06FE9E, i_06fe9e),
    (0x06FEA4, i_06fea4),
    (0x06FEA8, i_06fea8),
    (0x06FEAC, i_06feac),
    (0x06FEB0, i_06feb0),
    (0x06FEB2, i_06feb2),
    (0x06FEB8, i_06feb8),
    (0x06FEBC, i_06febc),
    (0x06FEC2, i_06fec2),
    (0x06FEC8, i_06fec8),
    (0x06FECC, i_06fecc),
    (0x06FED0, i_06fed0),
    (0x06FED2, i_06fed2),
    (0x06FED8, i_06fed8),
    (0x06FEDE, i_06fede),
    (0x06FEE4, i_06fee4),
    (0x06FEE8, i_06fee8),
    (0x06FEEC, i_06feec),
    (0x06FEEE, i_06feee),
    (0x06FEF0, i_06fef0),
    (0x06FEF6, i_06fef6),
    (0x06FEFC, i_06fefc),
    (0x06FF00, i_06ff00),
    (0x06FF02, i_06ff02),
    (0x06FF08, i_06ff08),
    (0x06FF0E, i_06ff0e),
    (0x06FF12, i_06ff12),
    (0x06FF16, i_06ff16),
    (0x06FF18, i_06ff18),
    (0x06FF1E, i_06ff1e),
    (0x06FF24, i_06ff24),
    (0x06FF28, i_06ff28),
    (0x06FF2C, i_06ff2c),
    (0x06FF2E, i_06ff2e),
    (0x06FF32, i_06ff32),
    (0x06FF36, i_06ff36),
    (0x06FF38, i_06ff38),
    (0x06FF3E, i_06ff3e),
    (0x06FF44, i_06ff44),
    (0x06FF4A, i_06ff4a),
    (0x06FF50, i_06ff50),
    (0x06FF54, i_06ff54),
    (0x06FF58, i_06ff58),
    (0x06FF5C, i_06ff5c),
    (0x06FF60, i_06ff60),
    (0x06FF64, i_06ff64),
    (0x06FF68, i_06ff68),
    (0x06FF6C, i_06ff6c),
    (0x06FF6E, i_06ff6e),
    (0x06FF72, i_06ff72),
    (0x06FF78, i_06ff78),
    (0x06FF7C, i_06ff7c),
    (0x06FF7E, i_06ff7e),
    (0x06FF84, i_06ff84),
    (0x06FF8A, i_06ff8a),
    (0x06FF8E, i_06ff8e),
    (0x06FF92, i_06ff92),
    (0x06FF94, i_06ff94),
    (0x06FF98, i_06ff98),
    (0x06FF9C, i_06ff9c),
    (0x06FFA0, i_06ffa0),
    (0x06FFA2, i_06ffa2),
    (0x06FFA4, i_06ffa4),
    (0x06FFAA, i_06ffaa),
    (0x06FFAC, i_06ffac),
    (0x06FFB0, i_06ffb0),
    (0x06FFB6, i_06ffb6),
    (0x06FFBC, i_06ffbc),
    (0x06FFC0, i_06ffc0),
    (0x06FFC4, i_06ffc4),
    (0x06FFC6, i_06ffc6),
    (0x06FFCA, i_06ffca),
    (0x06FFD0, i_06ffd0),
    (0x06FFD4, i_06ffd4),
    (0x06FFD8, i_06ffd8),
    (0x06FFDC, i_06ffdc),
    (0x06FFE0, i_06ffe0),
    (0x06FFE4, i_06ffe4),
    (0x06FFE8, i_06ffe8),
    (0x06FFEC, i_06ffec),
    (0x06FFF0, i_06fff0),
    (0x06FFF4, i_06fff4),
    (0x06FFF8, i_06fff8),
    (0x06FFFA, i_06fffa),
    (0x06FFFE, i_06fffe),
];
