// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $053646: bcc.w $53650
pub fn i_053646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5364A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53650; return 10; }
    8
}
/// $05364A: jmp $518.l
pub fn i_05364a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53650;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053650: rts
pub fn i_053650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53652;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GateWreck_053652  ($053652, 90 B, props/destructible_0527ba.s)
/// $053652: move.l #$2000, d0
pub fn i_053652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53658;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $053658: jsr $51a28.l
pub fn i_053658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5365E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5365E);
    cpu.pc = 0x51A28;
    16
}
/// $05365E: cmpi.w #$0, $38(a6)
pub fn i_05365e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53664;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $053664: beq.w $53670
pub fn i_053664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53668;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53670; return 10; }
    8
}
/// $053668: jsr $53f54(pc)
pub fn i_053668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5366C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5366C);
    cpu.pc = 0x53F54;
    16
}
/// $05366C: bra.w $53674
pub fn i_05366c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53670;
    cpu.ir = 0x6000;
    cpu.pc = 0x53674;
    10
}
/// $053670: jsr $53f08(pc)
pub fn i_053670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53674;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x53674);
    cpu.pc = 0x53F08;
    16
}
/// $053674: lea.l $ffff.w, a0
pub fn i_053674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53678;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $053678: move.l a0, $48(a6)
pub fn i_053678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5367C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05367C: lea.l $297658.l, a0
pub fn i_05367c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53682;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297658;
    20
}
/// $053682: jsr $28cd4.l
pub fn i_053682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53688;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53688);
    cpu.pc = 0x28CD4;
    16
}
/// $053688: lea.l $5368e(pc), a1
pub fn i_053688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5368C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5368E;
    16
}
/// $05368C: move.l a1, (a6)
pub fn i_05368c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5368E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05368E: jsr $2783a.l
pub fn i_05368e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53694;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53694);
    cpu.pc = 0x2783A;
    16
}
/// $053694: jsr $28d70.l
pub fn i_053694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5369A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5369A);
    cpu.pc = 0x28D70;
    16
}
/// $05369A: jsr $4fa70.l
pub fn i_05369a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x536A0);
    cpu.pc = 0x4FA70;
    16
}
/// $0536A0: bcc.w $536aa
pub fn i_0536a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536A4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x536AA; return 10; }
    8
}
/// $0536A4: jmp $518.l
pub fn i_0536a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536AA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0536AA: rts
pub fn i_0536aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlink_PhaseA_0536ac  ($0536AC, 94 B, props/destructible_0527ba.s)
/// $0536AC: move.w #$19, d1
pub fn i_0536ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536B0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0536B0: move.w #$c, d2
pub fn i_0536b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536B4;
    cpu.ir = 0x343C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0536B4: move.w #$2, d3
pub fn i_0536b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536B8;
    cpu.ir = 0x363C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0536B8: move.w #$1, d4
pub fn i_0536b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536BC;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0536BC: jsr $2c26.l
pub fn i_0536bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x536C2);
    cpu.pc = 0x2C26;
    16
}
/// $0536C2: move.w #$1b, d1
pub fn i_0536c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536C6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0536C6: move.w #$e, d2
pub fn i_0536c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536CA;
    cpu.ir = 0x343C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0536CA: move.w #$2, d3
pub fn i_0536ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536CE;
    cpu.ir = 0x363C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0536CE: move.w #$1, d4
pub fn i_0536ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536D2;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0536D2: jsr $2c26.l
pub fn i_0536d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x536D8);
    cpu.pc = 0x2C26;
    16
}
/// $0536D8: move.w #$b4, $72(a6)
pub fn i_0536d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536DE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0536DE: lea.l $536e4(pc), a1
pub fn i_0536de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x536E4;
    16
}
/// $0536E2: move.l a1, (a6)
pub fn i_0536e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0536E4: subq.w #$1, $72(a6)
pub fn i_0536e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536E8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0536E8: bne.w $536f2
pub fn i_0536e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536EC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x536F2; return 10; }
    8
}
/// $0536EC: lea.l $5370a(pc), a1
pub fn i_0536ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5370A;
    16
}
/// $0536F0: move.l a1, (a6)
pub fn i_0536f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0536F2: move.l $106f5c.l, d0
pub fn i_0536f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536F8;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0536F8: swap d0
pub fn i_0536f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536FA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0536FA: cmpi.w #$140, d0
pub fn i_0536fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x536FE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x140, Size::Word, false);
    8
}
/// $0536FE: blt.w $53708
pub fn i_0536fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53702;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x53708; return 10; }
    8
}
/// $053702: jmp $518.l
pub fn i_053702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53708;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053708: rts
pub fn i_053708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5370A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlink_PhaseB_05370a  ($05370A, 94 B, props/destructible_0527ba.s)
/// $05370A: move.w #$19, d1
pub fn i_05370a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5370E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05370E: move.w #$d, d2
pub fn i_05370e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53712;
    cpu.ir = 0x343C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053712: move.w #$1, d3
pub fn i_053712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53716;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053716: move.w #$1, d4
pub fn i_053716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5371A;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05371A: jsr $2c26.l
pub fn i_05371a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53720;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53720);
    cpu.pc = 0x2C26;
    16
}
/// $053720: move.w #$1b, d1
pub fn i_053720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53724;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053724: move.w #$f, d2
pub fn i_053724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53728;
    cpu.ir = 0x343C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053728: move.w #$1, d3
pub fn i_053728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5372C;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05372C: move.w #$1, d4
pub fn i_05372c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53730;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053730: jsr $2c26.l
pub fn i_053730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53736;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53736);
    cpu.pc = 0x2C26;
    16
}
/// $053736: move.w #$78, $72(a6)
pub fn i_053736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5373C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05373C: lea.l $53742(pc), a1
pub fn i_05373c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53740;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53742;
    16
}
/// $053740: move.l a1, (a6)
pub fn i_053740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53742;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053742: subq.w #$1, $72(a6)
pub fn i_053742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53746;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $053746: bne.w $53750
pub fn i_053746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5374A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53750; return 10; }
    8
}
/// $05374A: lea.l $536ac(pc), a1
pub fn i_05374a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5374E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x536AC;
    16
}
/// $05374E: move.l a1, (a6)
pub fn i_05374e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53750;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053750: move.l $106f5c.l, d0
pub fn i_053750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53756;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $053756: swap d0
pub fn i_053756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53758;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $053758: cmpi.w #$140, d0
pub fn i_053758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5375C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x140, Size::Word, false);
    8
}
/// $05375C: blt.w $53766
pub fn i_05375c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53760;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x53766; return 10; }
    8
}
/// $053760: jmp $518.l
pub fn i_053760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53766;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053766: rts
pub fn i_053766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53768;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PropDrop_Item_053768  ($053768, 52 B, props/destructible_0527ba.s)
/// $053768: move.w #$a, d1
pub fn i_053768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5376C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05376C: jsr $236e.l
pub fn i_05376c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53772;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53772);
    cpu.pc = 0x236E;
    16
}
/// $053772: jsr $53e9c(pc)
pub fn i_053772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53776;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x53776);
    cpu.pc = 0x53E9C;
    16
}
/// $053776: movea.l $c(a6), a0
pub fn i_053776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5377A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05377A: move.w $54(a0), $22(a6)
pub fn i_05377a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53780;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053780: move.w $56(a0), $24(a6)
pub fn i_053780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53786;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053786: move.w #$3ff, d0
pub fn i_053786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5378A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05378A: jsr $5ea1c.l
pub fn i_05378a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53790);
    cpu.pc = 0x5EA1C;
    16
}
/// $053790: addq.w #$1, d0
pub fn i_053790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53792;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $053792: move.w d0, $36(a6)
pub fn i_053792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53796;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053796: jmp $6dce0.l
pub fn i_053796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5379C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x6DCE0;
    8
}
// ===== PropDebris_ChunkAlt_05379c  ($05379C, 78 B, props/destructible_0527ba.s)
/// $05379C: move.w #$b, d1
pub fn i_05379c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537A0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0537A0: jsr $236e.l
pub fn i_0537a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x537A6);
    cpu.pc = 0x236E;
    16
}
/// $0537A6: bra.w $53804
pub fn i_0537a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x53804;
    10
}
/// $0537AA: move.w #$b, d1
pub fn i_0537aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537AE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0537AE: jsr $236e.l
pub fn i_0537ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x537B4);
    cpu.pc = 0x236E;
    16
}
/// $0537B4: jsr $267e2.l
pub fn i_0537b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x537BA);
    cpu.pc = 0x267E2;
    16
}
/// $0537BA: move.w #$1ff, d0
pub fn i_0537ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537BE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0537BE: jsr $5ea1c.l
pub fn i_0537be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x537C4);
    cpu.pc = 0x5EA1C;
    16
}
/// $0537C4: move.w d0, $2a(a6)
pub fn i_0537c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537C8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0537C8: move.w #$fff0, $2e(a6)
pub fn i_0537c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537CE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0537CE: move.w #$ff, d0
pub fn i_0537ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537D2;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0537D2: jsr $5ea1c.l
pub fn i_0537d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x537D8);
    cpu.pc = 0x5EA1C;
    16
}
/// $0537D8: btst.b #$0, d0
pub fn i_0537d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537DC;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0537DC: beq.w $537e2
pub fn i_0537dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x537E2; return 10; }
    8
}
/// $0537E0: neg.w d0
pub fn i_0537e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537E2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0537E2: move.w d0, $28(a6)
pub fn i_0537e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537E6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0537E6: bra.w $53828
pub fn i_0537e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x53828;
    10
}
// ===== PropDebris_Chunk_0537ea  ($0537EA, 156 B, props/destructible_0527ba.s)
/// $0537EA: move.w #$29, d1
pub fn i_0537ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537EE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x29;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0537EE: jsr $236e.l
pub fn i_0537ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x537F4);
    cpu.pc = 0x236E;
    16
}
/// $0537F4: movea.l $c(a6), a0
pub fn i_0537f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537F8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0537F8: move.w $54(a0), $22(a6)
pub fn i_0537f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x537FE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0537FE: move.w $56(a0), $24(a6)
pub fn i_0537fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53804;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053804: jsr $267e2.l
pub fn i_053804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5380A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5380A);
    cpu.pc = 0x267E2;
    16
}
/// $05380A: move.w #$fff0, $2e(a6)
pub fn i_05380a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53810;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053810: move.w #$f, d0
pub fn i_053810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53814;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053814: jsr $5ea1c.l
pub fn i_053814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5381A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5381A);
    cpu.pc = 0x5EA1C;
    16
}
/// $05381A: btst.b #$0, d0
pub fn i_05381a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5381E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05381E: beq.w $53824
pub fn i_05381e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53822;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53824; return 10; }
    8
}
/// $053822: neg.w d0
pub fn i_053822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53824;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $053824: move.w d0, $28(a6)
pub fn i_053824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53828;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053828: move.w #$28, $72(a6)
pub fn i_053828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5382E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05382E: lea.l $2de4b0.l, a0
pub fn i_05382e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53834;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DE4B0;
    20
}
/// $053834: jsr $28cd4.l
pub fn i_053834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5383A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5383A);
    cpu.pc = 0x28CD4;
    16
}
/// $05383A: lea.l $53840(pc), a1
pub fn i_05383a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5383E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53840;
    16
}
/// $05383E: move.l a1, (a6)
pub fn i_05383e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53840;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053840: jsr $27cee.l
pub fn i_053840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53846);
    cpu.pc = 0x27CEE;
    16
}
/// $053846: cmpi.w #$a, $72(a6)
pub fn i_053846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5384C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA, Size::Word, false);
    16
}
/// $05384C: bne.w $5385a
pub fn i_05384c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53850;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5385A; return 10; }
    8
}
/// $053850: move.b #$14, d0
pub fn i_053850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53854;
    cpu.ir = 0x103C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $053854: jsr $5e722.l
pub fn i_053854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5385A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5385A);
    cpu.pc = 0x5E722;
    16
}
/// $05385A: jsr $28d70.l
pub fn i_05385a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53860;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53860);
    cpu.pc = 0x28D70;
    16
}
/// $053860: movea.l #$ffffffff, a0
pub fn i_053860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53866;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $053866: lea.l $298736.l, a0
pub fn i_053866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5386C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298736;
    20
}
/// $05386C: jsr $5dd56.l
pub fn i_05386c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53872;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53872);
    cpu.pc = 0x5DD56;
    16
}
/// $053872: bcs.w $5387e
pub fn i_053872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53876;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5387E; return 10; }
    8
}
/// $053876: subq.w #$1, $72(a6)
pub fn i_053876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5387A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05387A: bne.w $53884
pub fn i_05387a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5387E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53884; return 10; }
    8
}
/// $05387E: jmp $518.l
pub fn i_05387e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53884;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053884: rts
pub fn i_053884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53886;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PropDebris_FlyingAlt_053886  ($053886, 14 B, props/destructible_0527ba.s)
/// $053886: move.w #$ad, d1
pub fn i_053886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5388A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05388A: jsr $236e.l
pub fn i_05388a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53890;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53890);
    cpu.pc = 0x236E;
    16
}
/// $053890: bra.w $5389e
pub fn i_053890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53894;
    cpu.ir = 0x6000;
    cpu.pc = 0x5389E;
    10
}
// ===== PropDebris_Flying_053894  ($053894, 208 B, props/destructible_0527ba.s)
/// $053894: move.w #$9e, d1
pub fn i_053894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53898;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053898: jsr $236e.l
pub fn i_053898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5389E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5389E);
    cpu.pc = 0x236E;
    16
}
/// $05389E: jsr $267e2.l
pub fn i_05389e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x538A4);
    cpu.pc = 0x267E2;
    16
}
/// $0538A4: movea.l $c(a6), a0
pub fn i_0538a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538A8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0538A8: move.w $54(a0), $22(a6)
pub fn i_0538a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538AE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0538AE: move.w $56(a0), $24(a6)
pub fn i_0538ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538B4;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x56);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0538B4: move.w #$c00, $2a(a6)
pub fn i_0538b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538BA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0538BA: move.w #$100, d0
pub fn i_0538ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538BE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0538BE: move.b $106f28.l, d1
pub fn i_0538be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538C4;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0538C4: btst.b #$0, d1
pub fn i_0538c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538C8;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0538C8: beq.w $538ce
pub fn i_0538c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x538CE; return 10; }
    8
}
/// $0538CC: neg.w d0
pub fn i_0538cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538CE;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0538CE: move.w d0, $28(a6)
pub fn i_0538ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538D2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0538D2: cmpi.b #$4, $58(a0)
pub fn i_0538d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538D8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $0538D8: beq.w $538f4
pub fn i_0538d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x538F4; return 10; }
    8
}
/// $0538DC: movea.l $50(a0), a0
pub fn i_0538dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538E0;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0538E0: move.w $28(a0), d0
pub fn i_0538e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538E4;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0538E4: move.w $2a(a0), d1
pub fn i_0538e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538E8;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0538E8: asl.w #$1, d0
pub fn i_0538e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538EA;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0538EA: asl.w #$1, d1
pub fn i_0538ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538EC;
    cpu.ir = 0xE341;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0538EC: move.w d0, $28(a6)
pub fn i_0538ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538F0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0538F0: move.w d1, $2a(a6)
pub fn i_0538f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538F4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0538F4: move.w #$1ff, d0
pub fn i_0538f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0538F8: jsr $5ea1c.l
pub fn i_0538f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x538FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x538FE);
    cpu.pc = 0x5EA1C;
    16
}
/// $0538FE: ori.w #$100, d0
pub fn i_0538fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53902;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x100) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $053902: andi.w #$1ff, d0
pub fn i_053902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53906;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $053906: sub.w d0, $2e(a6)
pub fn i_053906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5390A;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05390A: jsr $53e9c(pc)
pub fn i_05390a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5390E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5390E);
    cpu.pc = 0x53E9C;
    16
}
/// $05390E: lea.l $53914(pc), a1
pub fn i_05390e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53912;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53914;
    16
}
/// $053912: move.l a1, (a6)
pub fn i_053912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53914;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053914: jsr $27d50.l
pub fn i_053914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5391A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5391A);
    cpu.pc = 0x27D50;
    16
}
/// $05391A: bcc.w $53946
pub fn i_05391a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5391E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53946; return 10; }
    8
}
/// $05391E: move.w $2a(a6), d0
pub fn i_05391e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53922;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $053922: neg.w d0
pub fn i_053922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53924;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $053924: asr.w #$1, d0
pub fn i_053924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53926;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $053926: move.w d0, $2a(a6)
pub fn i_053926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5392A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05392A: lea.l $77e10.l, a1
pub fn i_05392a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53930;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77E10;
    20
}
/// $053930: jsr $4ae.l
pub fn i_053930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53936);
    cpu.pc = 0x4AE;
    16
}
/// $053936: jsr $5dd22.l
pub fn i_053936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5393C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5393C);
    cpu.pc = 0x5DD22;
    16
}
/// $05393C: cmpi.w #$100, $2a(a6)
pub fn i_05393c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53942;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $053942: blt.w $5395c
pub fn i_053942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53946;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5395C; return 10; }
    8
}
/// $053946: jsr $28d70.l
pub fn i_053946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5394C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5394C);
    cpu.pc = 0x28D70;
    16
}
/// $05394C: movea.l #$ffffffff, a0
pub fn i_05394c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53952;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $053952: jsr $5dd56.l
pub fn i_053952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53958;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53958);
    cpu.pc = 0x5DD56;
    16
}
/// $053958: bcc.w $53962
pub fn i_053958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5395C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53962; return 10; }
    8
}
/// $05395C: jmp $518.l
pub fn i_05395c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53962;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053962: rts
pub fn i_053962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53964;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Indestructible_053964  ($053964, 126 B, props/destructible_0527ba.s)
/// $053964: movea.l $3c(a6), a1
pub fn i_053964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53968;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $053968: jsr $2942a.l
pub fn i_053968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5396E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5396E);
    cpu.pc = 0x2942A;
    16
}
/// $05396E: move.w #$1ae, d1
pub fn i_05396e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53972;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053972: jsr $236e.l
pub fn i_053972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53978;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53978);
    cpu.pc = 0x236E;
    16
}
/// $053978: jsr $267e2.l
pub fn i_053978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5397E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5397E);
    cpu.pc = 0x267E2;
    16
}
/// $05397E: move.w #$0, $38(a6)
pub fn i_05397e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53984;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053984: lea.l $297728.l, a0
pub fn i_053984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5398A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297728;
    20
}
/// $05398A: jsr $28cd4.l
pub fn i_05398a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53990);
    cpu.pc = 0x28CD4;
    16
}
/// $053990: move.b #$0, $21(a6)
pub fn i_053990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53996;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053996: lea.l $539f0(pc), a1
pub fn i_053996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5399A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x539F0;
    16
}
/// $05399A: jsr $4ae.l
pub fn i_05399a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x539A0);
    cpu.pc = 0x4AE;
    16
}
/// $0539A0: jsr $5dd22.l
pub fn i_0539a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x539A6);
    cpu.pc = 0x5DD22;
    16
}
/// $0539A6: addi.w #$60, $22(a0)
pub fn i_0539a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539AC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0539AC: lea.l $539b2(pc), a1
pub fn i_0539ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x539B2;
    16
}
/// $0539B0: move.l a1, (a6)
pub fn i_0539b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0539B2: jsr $2783a.l
pub fn i_0539b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x539B8);
    cpu.pc = 0x2783A;
    16
}
/// $0539B8: jsr $28d70.l
pub fn i_0539b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x539BE);
    cpu.pc = 0x28D70;
    16
}
/// $0539BE: move.w #$7fff, $66(a6)
pub fn i_0539be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0539C4: jsr $2870a.l
pub fn i_0539c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x539CA);
    cpu.pc = 0x2870A;
    16
}
/// $0539CA: bcc.w $539d8
pub fn i_0539ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x539D8; return 10; }
    8
}
/// $0539CE: jsr $53d80(pc)
pub fn i_0539ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x539D2);
    cpu.pc = 0x53D80;
    16
}
/// $0539D2: bclr.b #$3, $13(a6)
pub fn i_0539d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539D8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0539D8: jsr $4fa70.l
pub fn i_0539d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x539DE);
    cpu.pc = 0x4FA70;
    16
}
/// $0539DE: bcc.w $539ee
pub fn i_0539de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x539EE; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_0539e2  ($0539E2, 14 B, kernel/jsr5b6_jmp518.c)
/// $0539E2: jsr $5b6.l
pub fn i_0539e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x539E8);
    cpu.pc = 0x5B6;
    16
}
/// $0539E8: jmp $518.l
pub fn i_0539e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539EE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0539EE: rts
pub fn i_0539ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_IndestructibleChild_0539f0  ($0539F0, 76 B, props/helpers_0539f0.s)
/// $0539F0: move.w #$1af, d1
pub fn i_0539f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539F4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0539F4: jsr $236e.l
pub fn i_0539f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x539FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x539FA);
    cpu.pc = 0x236E;
    16
}
/// $0539FA: jsr $267e2.l
pub fn i_0539fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53A00);
    cpu.pc = 0x267E2;
    16
}
/// $053A00: move.w #$f000, $38(a6)
pub fn i_053a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A06;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053A06: lea.l $2977d0.l, a0
pub fn i_053a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A0C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2977D0;
    20
}
/// $053A0C: jsr $28cd4.l
pub fn i_053a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53A12);
    cpu.pc = 0x28CD4;
    16
}
/// $053A12: lea.l $53a18(pc), a1
pub fn i_053a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53A18;
    16
}
/// $053A16: move.l a1, (a6)
pub fn i_053a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053A18: jsr $2783a.l
pub fn i_053a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53A1E);
    cpu.pc = 0x2783A;
    16
}
/// $053A1E: move.w $107fe8.l, d0
pub fn i_053a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A24;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x107FE8) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $053A24: sub.w d0, $22(a6)
pub fn i_053a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A28;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $053A28: move.w $107fea.l, d0
pub fn i_053a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A2E;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x107FEA) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $053A2E: add.w d0, $24(a6)
pub fn i_053a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A32;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $053A32: jsr $28d70.l
pub fn i_053a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53A38);
    cpu.pc = 0x28D70;
    16
}
/// $053A38: bcc.w $53a40
pub fn i_053a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53A40; return 10; }
    8
}
// ===== JsrPcThunk_053a3c  ($053A3C, 6 B, thunks/jsr_pc_thunks.c)
/// $053A3C: jsr $53dca(pc)
pub fn i_053a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x53A40);
    cpu.pc = 0x53DCA;
    16
}
/// $053A40: rts
pub fn i_053a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TrapFlame_053a42  ($053A42, 348 B, props/helpers_0539f0.s)
/// $053A42: move.w #$1e4, d1
pub fn i_053a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A46;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053A46: jsr $236e.l
pub fn i_053a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53A4C);
    cpu.pc = 0x236E;
    16
}
/// $053A4C: jsr $267e2.l
pub fn i_053a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53A52);
    cpu.pc = 0x267E2;
    16
}
/// $053A52: move.w #$4000, $38(a6)
pub fn i_053a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A58;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053A58: jsr $27cee.l
pub fn i_053a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53A5E);
    cpu.pc = 0x27CEE;
    16
}
/// $053A5E: lea.l $2977e0.l, a0
pub fn i_053a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2977E0;
    20
}
/// $053A64: jsr $28cd4.l
pub fn i_053a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53A6A);
    cpu.pc = 0x28CD4;
    16
}
/// $053A6A: move.w #$0, $72(a6)
pub fn i_053a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053A70: move.b #$0, $20(a6)
pub fn i_053a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A76;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053A76: lea.l $53b9e(pc), a1
pub fn i_053a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A7A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53B9E;
    16
}
/// $053A7A: jsr $4ae.l
pub fn i_053a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53A80);
    cpu.pc = 0x4AE;
    16
}
/// $053A80: lea.l $53a86(pc), a1
pub fn i_053a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53A86;
    16
}
/// $053A84: move.l a1, (a6)
pub fn i_053a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053A86: jsr $2783a.l
pub fn i_053a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53A8C);
    cpu.pc = 0x2783A;
    16
}
/// $053A8C: jsr $28d70.l
pub fn i_053a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53A92);
    cpu.pc = 0x28D70;
    16
}
/// $053A92: jsr $5e9b6.l
pub fn i_053a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53A98);
    cpu.pc = 0x5E9B6;
    16
}
/// $053A98: andi.b #$7, d0
pub fn i_053a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53A9C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $053A9C: bne.w $53aaa
pub fn i_053a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AA0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53AAA; return 10; }
    8
}
/// $053AA0: move.w #$1098, d0
pub fn i_053aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AA4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1098;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053AA4: jsr $2352.l
pub fn i_053aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53AAA);
    cpu.pc = 0x2352;
    16
}
/// $053AAA: cmpi.w #$0, $72(a6)
pub fn i_053aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AB0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $053AB0: ble.w $53abc
pub fn i_053ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AB4;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x53ABC; return 10; }
    8
}
/// $053AB4: subq.w #$1, $72(a6)
pub fn i_053ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AB8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $053AB8: bra.w $53b7a
pub fn i_053ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53ABC;
    cpu.ir = 0x6000;
    cpu.pc = 0x53B7A;
    10
}
/// $053ABC: lea.l $298896.l, a0
pub fn i_053abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AC2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298896;
    20
}
/// $053AC2: move.l a0, $4c(a6)
pub fn i_053ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AC6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $053AC6: jsr $283ca.l
pub fn i_053ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53ACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53ACC);
    cpu.pc = 0x283CA;
    16
}
/// $053ACC: jsr $283ca.l
pub fn i_053acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53AD2);
    cpu.pc = 0x283CA;
    16
}
/// $053AD2: jsr $283d8.l
pub fn i_053ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53AD8);
    cpu.pc = 0x283D8;
    16
}
/// $053AD8: btst.b #$1, $13(a6)
pub fn i_053ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53ADE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $053ADE: beq.w $53af4
pub fn i_053ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AE2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53AF4; return 10; }
    8
}
/// $053AE2: bclr.b #$1, $13(a6)
pub fn i_053ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AE8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $053AE8: move.l #$ffffffff, $50(a6)
pub fn i_053ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AF0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $053AF0: bra.w $53b58
pub fn i_053af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x53B58;
    10
}
/// $053AF4: lea.l $2987ee.l, a0
pub fn i_053af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AFA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2987EE;
    20
}
/// $053AFA: move.l a0, $4c(a6)
pub fn i_053afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53AFE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $053AFE: jsr $283ca.l
pub fn i_053afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53B04);
    cpu.pc = 0x283CA;
    16
}
/// $053B04: jsr $283ca.l
pub fn i_053b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53B0A);
    cpu.pc = 0x283CA;
    16
}
/// $053B0A: jsr $283d8.l
pub fn i_053b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53B10);
    cpu.pc = 0x283D8;
    16
}
/// $053B10: btst.b #$1, $13(a6)
pub fn i_053b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B16;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $053B16: beq.w $53b28
pub fn i_053b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B1A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53B28; return 10; }
    8
}
/// $053B1A: lea.l $100440.l, a0
pub fn i_053b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B20;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $053B20: move.l a0, $50(a6)
pub fn i_053b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B24;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $053B24: bra.w $53b58
pub fn i_053b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B28;
    cpu.ir = 0x6000;
    cpu.pc = 0x53B58;
    10
}
/// $053B28: lea.l $298842.l, a0
pub fn i_053b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B2E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298842;
    20
}
/// $053B2E: move.l a0, $4c(a6)
pub fn i_053b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B32;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $053B32: jsr $283ca.l
pub fn i_053b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53B38);
    cpu.pc = 0x283CA;
    16
}
/// $053B38: jsr $283ca.l
pub fn i_053b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53B3E);
    cpu.pc = 0x283CA;
    16
}
/// $053B3E: jsr $283d8.l
pub fn i_053b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53B44);
    cpu.pc = 0x283D8;
    16
}
/// $053B44: btst.b #$1, $13(a6)
pub fn i_053b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B4A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $053B4A: beq.w $53b7a
pub fn i_053b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B4E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53B7A; return 10; }
    8
}
/// $053B4E: lea.l $1004e0.l, a0
pub fn i_053b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $053B54: move.l a0, $50(a6)
pub fn i_053b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B58;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $053B58: bclr.b #$1, $13(a6)
pub fn i_053b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B5E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $053B5E: lea.l $53c44(pc), a1
pub fn i_053b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53C44;
    16
}
/// $053B62: jsr $4ae.l
pub fn i_053b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53B68);
    cpu.pc = 0x4AE;
    16
}
/// $053B68: jsr $5dd02.l
pub fn i_053b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53B6E);
    cpu.pc = 0x5DD02;
    16
}
/// $053B6E: move.l $50(a6), $50(a0)
pub fn i_053b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B74;
    cpu.ir = 0x216E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0].wrapping_add(0x50);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $053B74: move.w #$c, $72(a6)
pub fn i_053b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B7A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053B7A: movea.l #$ffffffff, a0
pub fn i_053b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B80;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $053B80: lea.l $298740.l, a0
pub fn i_053b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B86;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298740;
    20
}
/// $053B86: jsr $5dd5c.l
pub fn i_053b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53B8C);
    cpu.pc = 0x5DD5C;
    16
}
/// $053B8C: bcc.w $53b9c
pub fn i_053b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B90;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53B9C; return 10; }
    8
}
/// $053B90: move.b #$ff, $20(a6)
pub fn i_053b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B96;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053B96: jmp $518.l
pub fn i_053b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B9C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053B9C: rts
pub fn i_053b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53B9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TrapFlameBase_053b9e  ($053B9E, 96 B, props/helpers_0539f0.s)
/// $053B9E: move.w #$1e4, d1
pub fn i_053b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BA2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053BA2: jsr $236e.l
pub fn i_053ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53BA8);
    cpu.pc = 0x236E;
    16
}
/// $053BA8: jsr $267e2.l
pub fn i_053ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53BAE);
    cpu.pc = 0x267E2;
    16
}
/// $053BAE: move.w #$f000, $38(a6)
pub fn i_053bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053BB4: jsr $27cee.l
pub fn i_053bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53BBA);
    cpu.pc = 0x27CEE;
    16
}
/// $053BBA: lea.l $297856.l, a0
pub fn i_053bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BC0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297856;
    20
}
/// $053BC0: jsr $28cd4.l
pub fn i_053bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53BC6);
    cpu.pc = 0x28CD4;
    16
}
/// $053BC6: move.w #$0, $72(a6)
pub fn i_053bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BCC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053BCC: lea.l $53bd2(pc), a1
pub fn i_053bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53BD2;
    16
}
/// $053BD0: move.l a1, (a6)
pub fn i_053bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053BD2: movea.l $c(a6), a0
pub fn i_053bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BD6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053BD6: move.w $22(a0), $22(a6)
pub fn i_053bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BDC;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053BDC: move.w $24(a0), $24(a6)
pub fn i_053bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BE2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053BE2: jsr $28d70.l
pub fn i_053be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53BE8);
    cpu.pc = 0x28D70;
    16
}
/// $053BE8: movea.l $c(a6), a0
pub fn i_053be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BEC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053BEC: cmpi.b #$ff, $20(a0)
pub fn i_053bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BF2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $053BF2: bne.w $53bfc
pub fn i_053bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BF6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53BFC; return 10; }
    8
}
/// $053BF6: jmp $518.l
pub fn i_053bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BFC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053BFC: rts
pub fn i_053bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53BFE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TrapFlameHitFx_053bfe  ($053BFE, 70 B, props/helpers_0539f0.s)
/// $053BFE: move.w #$1e4, d1
pub fn i_053bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C02;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1E4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053C02: jsr $236e.l
pub fn i_053c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C08);
    cpu.pc = 0x236E;
    16
}
/// $053C08: jsr $267e2.l
pub fn i_053c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C0E);
    cpu.pc = 0x267E2;
    16
}
/// $053C0E: move.w #$8000, $38(a6)
pub fn i_053c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C14;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053C14: jsr $27cee.l
pub fn i_053c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C1A);
    cpu.pc = 0x27CEE;
    16
}
/// $053C1A: lea.l $2978cc.l, a0
pub fn i_053c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C20;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2978CC;
    20
}
/// $053C20: jsr $28cd4.l
pub fn i_053c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C26);
    cpu.pc = 0x28CD4;
    16
}
/// $053C26: lea.l $53c2c(pc), a1
pub fn i_053c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53C2C;
    16
}
/// $053C2A: move.l a1, (a6)
pub fn i_053c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053C2C: jsr $2783a.l
pub fn i_053c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C32);
    cpu.pc = 0x2783A;
    16
}
/// $053C32: jsr $28d70.l
pub fn i_053c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C38);
    cpu.pc = 0x28D70;
    16
}
/// $053C38: bcc.w $53c42
pub fn i_053c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C3C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53C42; return 10; }
    8
}
/// $053C3C: jmp $518.l
pub fn i_053c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C42;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053C42: rts
pub fn i_053c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_TrapFlameHitSpawn_053c44  ($053C44, 16 B, props/helpers_0539f0.s)
/// $053C44: lea.l $53bfe(pc), a1
pub fn i_053c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53BFE;
    16
}
/// $053C48: jsr $4ae.l
pub fn i_053c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C4E);
    cpu.pc = 0x4AE;
    16
}
/// $053C4E: jsr $5dd02.l
pub fn i_053c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53C54);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_053c54  ($053C54, 8 B, kernel/task_handlers.c)
/// $053C54: lea.l $53c5c(pc), a1
pub fn i_053c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53C5C;
    16
}
/// $053C58: move.l a1, (a6)
pub fn i_053c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053C5A: rts
pub fn i_053c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_053c5c  ($053C5C, 8 B, kernel/task_handlers.c)
/// $053C5C: lea.l $53c64(pc), a1
pub fn i_053c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53C64;
    16
}
/// $053C60: move.l a1, (a6)
pub fn i_053c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053C62: rts
pub fn i_053c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BurnFollowVictim_053c64  ($053C64, 142 B, props/helpers_0539f0.s)
/// $053C64: movea.l $50(a6), a0
pub fn i_053c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C68;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053C68: move.l a0, d0
pub fn i_053c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C6A;
    cpu.ir = 0x2008;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $053C6A: cmpi.l #$ffffffff, d0
pub fn i_053c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C70;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $053C70: bne.w $53c7c
pub fn i_053c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53C7C; return 10; }
    8
}
/// $053C74: jmp $518.l
pub fn i_053c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C7A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053C7A: rts
pub fn i_053c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $053C7C: move.b $45(a0), d0
pub fn i_053c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C80;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $053C80: cmpi.b #$0, d0
pub fn i_053c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C84;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $053C84: bgt.w $53c9e
pub fn i_053c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C88;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x53C9E; return 10; }
    8
}
/// $053C88: move.w $28(a0), d0
pub fn i_053c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C8C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $053C8C: asl.w #$1, d0
pub fn i_053c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C8E;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $053C8E: move.w d0, $28(a0)
pub fn i_053c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C92;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053C92: move.w #$1000, $2a(a0)
pub fn i_053c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C98;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053C98: move.w #$19, $72(a6)
pub fn i_053c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53C9E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053C9E: lea.l $53ca4(pc), a1
pub fn i_053c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53CA4;
    16
}
/// $053CA2: move.l a1, (a6)
pub fn i_053ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053CA4: movea.l $50(a6), a0
pub fn i_053ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CA8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053CA8: move.w $22(a0), $22(a6)
pub fn i_053ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CAE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053CAE: move.w $24(a0), $24(a6)
pub fn i_053cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CB4;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053CB4: move.w $38(a0), $38(a6)
pub fn i_053cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CBA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053CBA: move.w $72(a6), d0
pub fn i_053cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CBE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $053CBE: andi.w #$3, d0
pub fn i_053cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CC2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $053CC2: bne.w $53ce2
pub fn i_053cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53CE2; return 10; }
    8
}
/// $053CC6: lea.l $53cf2(pc), a1
pub fn i_053cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53CF2;
    16
}
/// $053CCA: jsr $4ae.l
pub fn i_053cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53CD0);
    cpu.pc = 0x4AE;
    16
}
/// $053CD0: jsr $5dd22.l
pub fn i_053cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53CD6);
    cpu.pc = 0x5DD22;
    16
}
/// $053CD6: addi.w #$10, $24(a0)
pub fn i_053cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CDC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $053CDC: move.w $38(a6), $38(a0)
pub fn i_053cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CE2;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053CE2: subq.w #$1, $72(a6)
pub fn i_053ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CE6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $053CE6: bpl.w $53cf0
pub fn i_053ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CEA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x53CF0; return 10; }
    8
}
/// $053CEA: jmp $518.l
pub fn i_053cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CF0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053CF0: rts
pub fn i_053cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BurnSmokePuff_053cf2  ($053CF2, 58 B, props/helpers_0539f0.s)
/// $053CF2: move.w #$2, d1
pub fn i_053cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CF6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053CF6: jsr $236e.l
pub fn i_053cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53CFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53CFC);
    cpu.pc = 0x236E;
    16
}
/// $053CFC: jsr $267e2.l
pub fn i_053cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53D02);
    cpu.pc = 0x267E2;
    16
}
/// $053D02: lea.l $297922.l, a0
pub fn i_053d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297922;
    20
}
/// $053D08: jsr $28cd4.l
pub fn i_053d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53D0E);
    cpu.pc = 0x28CD4;
    16
}
/// $053D0E: lea.l $53d14(pc), a1
pub fn i_053d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53D14;
    16
}
/// $053D12: move.l a1, (a6)
pub fn i_053d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053D14: jsr $2783a.l
pub fn i_053d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53D1A);
    cpu.pc = 0x2783A;
    16
}
/// $053D1A: jsr $28d70.l
pub fn i_053d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53D20);
    cpu.pc = 0x28D70;
    16
}
/// $053D20: bcc.w $53d2a
pub fn i_053d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x53D2A; return 10; }
    8
}
/// $053D24: jmp $518.l
pub fn i_053d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D2A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $053D2A: rts
pub fn i_053d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SpawnBonusOnce_053d2c  ($053D2C, 36 B, props/helpers_0539f0.s)
/// $053D2C: cmpi.b #$0, $21(a6)
pub fn i_053d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D32;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $053D32: bne.w $53d4e
pub fn i_053d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D36;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53D4E; return 10; }
    8
}
/// $053D36: move.b #$ff, $21(a6)
pub fn i_053d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D3C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053D3C: jsr $9ba56.l
pub fn i_053d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53D42);
    cpu.pc = 0x9BA56;
    16
}
/// $053D42: subi.w #$18, $22(a0)
pub fn i_053d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D48;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $053D48: addi.w #$30, $24(a0)
pub fn i_053d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D4E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $053D4E: rts
pub fn i_053d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_ToggleSpriteByTimer_053d50  ($053D50, 48 B, props/helpers_0539f0.s)
/// $053D50: cmpi.w #$0, $72(a6)
pub fn i_053d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D56;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $053D56: ble.w $53d78
pub fn i_053d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D5A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x53D78; return 10; }
    8
}
/// $053D5A: clr.l d0
pub fn i_053d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D5C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $053D5C: move.w $72(a6), d0
pub fn i_053d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D60;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $053D60: btst.b #$0, d0
pub fn i_053d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D64;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $053D64: beq.w $53d70
pub fn i_053d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D68;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53D70; return 10; }
    8
}
/// $053D68: move.w $16(a6), $14(a6)
pub fn i_053d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D6E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053D6E: rts
pub fn i_053d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $053D70: move.w $18(a6), $14(a6)
pub fn i_053d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D76;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053D76: rts
pub fn i_053d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $053D78: move.w $16(a6), $14(a6)
pub fn i_053d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D7E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $053D7E: rts
pub fn i_053d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_PickSpriteByVictimDir_053d80  ($053D80, 66 B, props/helpers_0539f0.s)
/// $053D80: movea.l $50(a6), a0
pub fn i_053d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D84;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053D84: clr.l d0
pub fn i_053d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D86;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $053D86: move.w $28(a0), d0
pub fn i_053d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D8A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $053D8A: cmpi.w #$0, d0
pub fn i_053d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D8E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $053D8E: beq.w $53dc8
pub fn i_053d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53DC8; return 10; }
    8
}
/// $053D92: cmpi.b #$0, $20(a6)
pub fn i_053d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D98;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $053D98: beq.w $53db4
pub fn i_053d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53D9C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53DB4; return 10; }
    8
}
/// $053D9C: btst.b #$f, d0
pub fn i_053d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DA0;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $053DA0: beq.w $53dc8
pub fn i_053da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DA4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53DC8; return 10; }
    8
}
/// $053DA4: lea.l $2976d2.l, a0
pub fn i_053da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2976D2;
    20
}
/// $053DAA: jsr $28cd4.l
pub fn i_053daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53DB0);
    cpu.pc = 0x28CD4;
    16
}
/// $053DB0: bra.w $53dc8
pub fn i_053db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DB4;
    cpu.ir = 0x6000;
    cpu.pc = 0x53DC8;
    10
}
/// $053DB4: btst.b #$f, d0
pub fn i_053db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DB8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $053DB8: bne.w $53dc8
pub fn i_053db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DBC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53DC8; return 10; }
    8
}
/// $053DBC: lea.l $297668.l, a0
pub fn i_053dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DC2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297668;
    20
}
// ===== JsrAbsThunk_053dc2  ($053DC2, 8 B, thunks/jsr_abs_thunks.c)
/// $053DC2: jsr $28cd4.l
pub fn i_053dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53DC8);
    cpu.pc = 0x28CD4;
    16
}
/// $053DC8: rts
pub fn i_053dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SyncSpriteWithParent_053dca  ($053DCA, 66 B, props/helpers_0539f0.s)
/// $053DCA: movea.l $c(a6), a0
pub fn i_053dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DCE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053DCE: move.b $20(a0), d0
pub fn i_053dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DD2;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $053DD2: cmp.b $20(a6), d0
pub fn i_053dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DD6;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $053DD6: beq.w $53e00
pub fn i_053dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DDA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53E00; return 10; }
    8
}
/// $053DDA: cmpi.b #$0, $20(a0)
pub fn i_053dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DE0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $053DE0: bne.w $53df4
pub fn i_053de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53DF4; return 10; }
    8
}
/// $053DE4: lea.l $297798.l, a0
pub fn i_053de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297798;
    20
}
/// $053DEA: jsr $28cd4.l
pub fn i_053dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53DF0);
    cpu.pc = 0x28CD4;
    16
}
/// $053DF0: bra.w $53e00
pub fn i_053df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x53E00;
    10
}
/// $053DF4: lea.l $297750.l, a0
pub fn i_053df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53DFA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297750;
    20
}
/// $053DFA: jsr $28cd4.l
pub fn i_053dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E00);
    cpu.pc = 0x28CD4;
    16
}
/// $053E00: movea.l $c(a6), a0
pub fn i_053e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E04;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $053E04: move.b $20(a0), $20(a6)
pub fn i_053e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E0A;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad4, v3 as u8);
    20
}
/// $053E0A: rts
pub fn i_053e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_PlayBreakMusicByPhase_053e0c  ($053E0C, 100 B, props/helpers_0539f0.s)
/// $053E0C: cmpi.b #$0, $21(a6)
pub fn i_053e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E12;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $053E12: bne.w $53e24
pub fn i_053e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53E24; return 10; }
    8
}
/// $053E16: move.w #$102c, d0
pub fn i_053e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E1A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053E1A: jsr $2352.l
pub fn i_053e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E20);
    cpu.pc = 0x2352;
    16
}
/// $053E20: bra.w $53e76
pub fn i_053e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E24;
    cpu.ir = 0x6000;
    cpu.pc = 0x53E76;
    10
}
/// $053E24: cmpi.b #$1, $21(a6)
pub fn i_053e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E2A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $053E2A: bne.w $53e3c
pub fn i_053e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53E3C; return 10; }
    8
}
/// $053E2E: move.w #$102c, d0
pub fn i_053e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E32;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053E32: jsr $2352.l
pub fn i_053e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E38);
    cpu.pc = 0x2352;
    16
}
/// $053E38: bra.w $53e76
pub fn i_053e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x53E76;
    10
}
/// $053E3C: cmpi.b #$2, $21(a6)
pub fn i_053e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E42;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $053E42: bne.w $53e54
pub fn i_053e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E46;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53E54; return 10; }
    8
}
/// $053E46: move.w #$1023, d0
pub fn i_053e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053E4A: jsr $2352.l
pub fn i_053e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E50);
    cpu.pc = 0x2352;
    16
}
/// $053E50: bra.w $53e76
pub fn i_053e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E54;
    cpu.ir = 0x6000;
    cpu.pc = 0x53E76;
    10
}
/// $053E54: cmpi.b #$3, $21(a6)
pub fn i_053e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E5A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $053E5A: bne.w $53e6c
pub fn i_053e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E5E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x53E6C; return 10; }
    8
}
/// $053E5E: move.w #$1035, d0
pub fn i_053e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E62;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053E62: jsr $2352.l
pub fn i_053e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E68);
    cpu.pc = 0x2352;
    16
}
/// $053E68: bra.w $53e76
pub fn i_053e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x53E76;
    10
}
/// $053E6C: move.w #$1023, d0
pub fn i_053e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_053e70  ($053E70, 8 B, thunks/jsr_abs_thunks.c)
/// $053E70: jsr $2352.l
pub fn i_053e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E76);
    cpu.pc = 0x2352;
    16
}
/// $053E76: rts
pub fn i_053e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_RunDebrisScriptByPhase_053e78  ($053E78, 28 B, props/helpers_0539f0.s)
/// $053E78: lea.l $297f50.l, a0
pub fn i_053e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x297F50;
    20
}
/// $053E7E: move.b $21(a6), d0
pub fn i_053e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E82;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $053E82: andi.w #$ff, d0
pub fn i_053e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E86;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $053E86: asl.w #$2, d0
pub fn i_053e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E88;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $053E88: move.w $38(a6), d1
pub fn i_053e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E8C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $053E8C: asl.w #$4, d1
pub fn i_053e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E8E;
    cpu.ir = 0xE941;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $053E8E: add.w d1, d0
pub fn i_053e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E90;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $053E90: movea.l (a0, d0.w), a2
pub fn i_053e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E94;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_053e94  ($053E94, 8 B, thunks/jsr_abs_thunks.c)
/// $053E94: jsr $5022a.l
pub fn i_053e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53E9A);
    cpu.pc = 0x5022A;
    16
}
/// $053E9A: rts
pub fn i_053e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53E9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_PickRandomItemPtr_053e9c  ($053E9C, 22 B, props/helpers_0539f0.s)
/// $053E9C: move.w #$3, d0
pub fn i_053e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EA0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053EA0: jsr $5ea1c.l
pub fn i_053ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53EA6);
    cpu.pc = 0x5EA1C;
    16
}
/// $053EA6: asl.w #$2, d0
pub fn i_053ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EA8;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $053EA8: lea.l $2980fc.l, a0
pub fn i_053ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EAE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2980FC;
    20
}
/// $053EAE: movea.l (a0, d0.w), a0
pub fn i_053eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EB2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
// ===== JsrAbsThunk_053eb2  ($053EB2, 8 B, thunks/jsr_abs_thunks.c)
/// $053EB2: jsr $28cd4.l
pub fn i_053eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53EB8);
    cpu.pc = 0x28CD4;
    16
}
/// $053EB8: rts
pub fn i_053eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_RunDebrisScriptByPrio_053eba  ($053EBA, 32 B, props/helpers_0539f0.s)
/// $053EBA: lea.l $298074.l, a1
pub fn i_053eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EC0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x298074;
    20
}
/// $053EC0: cmpi.w #$0, $38(a6)
pub fn i_053ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EC6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $053EC6: beq.w $53eda
pub fn i_053ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53ECA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53EDA; return 10; }
    8
}
/// $053ECA: cmpi.w #$6, $38(a6)
pub fn i_053eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53ED0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Word, false);
    16
}
/// $053ED0: beq.w $53eda
pub fn i_053ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53ED4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x53EDA; return 10; }
    8
}
/// $053ED4: lea.l $298062.l, a1
pub fn i_053ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EDA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x298062;
    20
}
// ===== JsrAbsThunk_053eda  ($053EDA, 8 B, thunks/jsr_abs_thunks.c)
/// $053EDA: jsr $77c7e.l
pub fn i_053eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53EE0);
    cpu.pc = 0x77C7E;
    16
}
/// $053EE0: rts
pub fn i_053ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GateDebrisA_053ee2  ($053EE2, 24 B, props/helpers_0539f0.s)
/// $053EE2: lea.l $29860c.l, a2
pub fn i_053ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EE8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29860C;
    20
}
/// $053EE8: jsr $5022a.l
pub fn i_053ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53EEE);
    cpu.pc = 0x5022A;
    16
}
/// $053EEE: lea.l $298620.l, a2
pub fn i_053eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EF4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298620;
    20
}
/// $053EF4: jsr $5022a.l
pub fn i_053ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53EFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53EFA);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_053efa  ($053EFA, 14 B, kernel/state_dispatch_stubs.c)
/// $053EFA: lea.l $298634.l, a2
pub fn i_053efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F00;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298634;
    20
}
/// $053F00: jsr $5022a.l
pub fn i_053f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F06);
    cpu.pc = 0x5022A;
    16
}
/// $053F06: rts
pub fn i_053f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GateDebrisB_053f08  ($053F08, 24 B, props/helpers_0539f0.s)
/// $053F08: lea.l $298648.l, a2
pub fn i_053f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F0E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298648;
    20
}
/// $053F0E: jsr $5022a.l
pub fn i_053f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F14);
    cpu.pc = 0x5022A;
    16
}
/// $053F14: lea.l $29865c.l, a2
pub fn i_053f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F1A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29865C;
    20
}
/// $053F1A: jsr $5022a.l
pub fn i_053f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F20);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_053f20  ($053F20, 14 B, kernel/state_dispatch_stubs.c)
/// $053F20: lea.l $298670.l, a2
pub fn i_053f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F26;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298670;
    20
}
/// $053F26: jsr $5022a.l
pub fn i_053f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F2C);
    cpu.pc = 0x5022A;
    16
}
/// $053F2C: rts
pub fn i_053f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GateDebrisC_053f2e  ($053F2E, 24 B, props/helpers_0539f0.s)
/// $053F2E: lea.l $298684.l, a2
pub fn i_053f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F34;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298684;
    20
}
/// $053F34: jsr $5022a.l
pub fn i_053f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F3A);
    cpu.pc = 0x5022A;
    16
}
/// $053F3A: lea.l $298698.l, a2
pub fn i_053f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F40;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x298698;
    20
}
/// $053F40: jsr $5022a.l
pub fn i_053f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F46);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_053f46  ($053F46, 14 B, kernel/state_dispatch_stubs.c)
/// $053F46: lea.l $2986ac.l, a2
pub fn i_053f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F4C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2986AC;
    20
}
/// $053F4C: jsr $5022a.l
pub fn i_053f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F52);
    cpu.pc = 0x5022A;
    16
}
/// $053F52: rts
pub fn i_053f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_GateDebrisD_053f54  ($053F54, 24 B, props/helpers_0539f0.s)
/// $053F54: lea.l $2986c0.l, a2
pub fn i_053f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F5A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2986C0;
    20
}
/// $053F5A: jsr $5022a.l
pub fn i_053f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F60);
    cpu.pc = 0x5022A;
    16
}
/// $053F60: lea.l $2986d4.l, a2
pub fn i_053f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F66;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2986D4;
    20
}
/// $053F66: jsr $5022a.l
pub fn i_053f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F6C);
    cpu.pc = 0x5022A;
    16
}
// ===== SDS_053f6c  ($053F6C, 14 B, kernel/state_dispatch_stubs.c)
/// $053F6C: lea.l $2986e8.l, a2
pub fn i_053f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F72;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2986E8;
    20
}
/// $053F72: jsr $5022a.l
pub fn i_053f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53F78);
    cpu.pc = 0x5022A;
    16
}
/// $053F78: rts
pub fn i_053f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpField10WithLink8_053f7a  ($053F7A, 16 B, props/helpers_0539f0.s)
/// $053F7A: movea.l $8(a6), a1
pub fn i_053f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F7E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $053F7E: move.b $10(a6), d0
pub fn i_053f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F82;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $053F82: cmp.b $10(a1), d0
pub fn i_053f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F86;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $053F86: bcs.w $53f90
pub fn i_053f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F8A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x53F90; return 10; }
    8
}
// ===== ClearXN_053f8a  ($053F8A, 6 B, thunks/ccr_helpers.c)
/// $053F8A: andi.b #$ee, ccr
pub fn i_053f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F8E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $053F8E: rts
pub fn i_053f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_053f90  ($053F90, 6 B, thunks/ccr_helpers.c)
/// $053F90: ori.b #$11, ccr
pub fn i_053f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F94;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $053F94: rts
pub fn i_053f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Building_053f96  ($053F96, 198 B, props/mission_053f96.s)
/// $053F96: movea.l $3c(a6), a1
pub fn i_053f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53F9A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $053F9A: jsr $2942a.l
pub fn i_053f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53FA0);
    cpu.pc = 0x2942A;
    16
}
/// $053FA0: move.w #$8b, d1
pub fn i_053fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FA4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $053FA4: jsr $236e.l
pub fn i_053fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53FAA);
    cpu.pc = 0x236E;
    16
}
/// $053FAA: move.w #$90, $70(a6)
pub fn i_053faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FB0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053FB0: move.b #$ff, $32(a6)
pub fn i_053fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FB6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053FB6: move.b #$ff, $33(a6)
pub fn i_053fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FBC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053FBC: move.b #$0, $3a(a6)
pub fn i_053fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $053FC2: move.w #$78, $66(a6)
pub fn i_053fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $053FC8: lea.l $298936.l, a0
pub fn i_053fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FCE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298936;
    20
}
/// $053FCE: jsr $28cd4.l
pub fn i_053fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53FD4);
    cpu.pc = 0x28CD4;
    16
}
/// $053FD4: lea.l $53fda(pc), a1
pub fn i_053fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x53FDA;
    16
}
/// $053FD8: move.l a1, (a6)
pub fn i_053fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $053FDA: clr.b $10e39a.l
pub fn i_053fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FE0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $053FE0: jsr $2783a.l
pub fn i_053fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53FE6);
    cpu.pc = 0x2783A;
    16
}
/// $053FE6: jsr $28d70.l
pub fn i_053fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53FEC);
    cpu.pc = 0x28D70;
    16
}
/// $053FEC: cmpi.b #$5, $58(a6)
pub fn i_053fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FF2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Byte, false);
    16
}
/// $053FF2: beq.w $5401c
pub fn i_053ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5401C; return 10; }
    8
}
/// $053FF6: jsr $2870a.l
pub fn i_053ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x53FFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x53FFC);
    cpu.pc = 0x2870A;
    16
}
/// $053FFC: bcc.w $54012
pub fn i_053ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54000;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54012; return 10; }
    8
}
/// $054000: lea.l $5e766.l, a0
pub fn i_054000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54006;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054006: jsr $5e770.l
pub fn i_054006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5400C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5400C);
    cpu.pc = 0x5E770;
    16
}
/// $05400C: bclr.b #$3, $13(a6)
pub fn i_05400c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54012;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054012: cmpi.w #$3c, $66(a6)
pub fn i_054012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54018;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3C, Size::Word, false);
    16
}
/// $054018: bgt.w $5404a
pub fn i_054018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5401C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5404A; return 10; }
    8
}
/// $05401C: move.w #$1023, d0
pub fn i_05401c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54020;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054020: jsr $2352.l
pub fn i_054020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54026);
    cpu.pc = 0x2352;
    16
}
/// $054026: bclr.b #$3, $13(a6)
pub fn i_054026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5402C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05402C: lea.l $29a278.l, a1
pub fn i_05402c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54032;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A278;
    20
}
/// $054032: jsr $77c7e.l
pub fn i_054032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54038;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54038);
    cpu.pc = 0x77C7E;
    16
}
/// $054038: lea.l $29a28a.l, a1
pub fn i_054038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5403E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A28A;
    20
}
/// $05403E: jsr $77c7e.l
pub fn i_05403e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54044;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54044);
    cpu.pc = 0x77C7E;
    16
}
/// $054044: lea.l $5405c(pc), a1
pub fn i_054044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54048;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5405C;
    16
}
/// $054048: move.l a1, (a6)
pub fn i_054048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5404A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05404A: jsr $4fa70.l
pub fn i_05404a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54050;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54050);
    cpu.pc = 0x4FA70;
    16
}
/// $054050: bcc.w $5405a
pub fn i_054050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54054;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5405A; return 10; }
    8
}
/// $054054: jmp $518.l
pub fn i_054054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5405A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05405A: rts
pub fn i_05405a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5405C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BuildingDamaged_05405c  ($05405C, 174 B, props/mission_053f96.s)
/// $05405C: jsr $2783a.l
pub fn i_05405c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54062;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54062);
    cpu.pc = 0x2783A;
    16
}
/// $054062: lea.l $29b29a.l, a1
pub fn i_054062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54068;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B29A;
    20
}
/// $054068: jsr $43fac.l
pub fn i_054068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5406E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5406E);
    cpu.pc = 0x43FAC;
    16
}
/// $05406E: lea.l $298958.l, a0
pub fn i_05406e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54074;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298958;
    20
}
/// $054074: jsr $28cd4.l
pub fn i_054074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5407A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5407A);
    cpu.pc = 0x28CD4;
    16
}
/// $05407A: lea.l $54080(pc), a1
pub fn i_05407a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5407E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54080;
    16
}
/// $05407E: move.l a1, (a6)
pub fn i_05407e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54080;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054080: clr.b $10e39a.l
pub fn i_054080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54086;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $054086: jsr $2783a.l
pub fn i_054086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5408C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5408C);
    cpu.pc = 0x2783A;
    16
}
/// $05408C: jsr $28d70.l
pub fn i_05408c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54092;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54092);
    cpu.pc = 0x28D70;
    16
}
/// $054092: jsr $2870a.l
pub fn i_054092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54098;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54098);
    cpu.pc = 0x2870A;
    16
}
/// $054098: bcc.w $540ae
pub fn i_054098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5409C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x540AE; return 10; }
    8
}
/// $05409C: lea.l $5e766.l, a0
pub fn i_05409c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0540A2: jsr $5e770.l
pub fn i_0540a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540A8);
    cpu.pc = 0x5E770;
    16
}
/// $0540A8: bclr.b #$3, $13(a6)
pub fn i_0540a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540AE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0540AE: jsr $28758.l
pub fn i_0540ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540B4);
    cpu.pc = 0x28758;
    16
}
/// $0540B4: bcc.w $540f8
pub fn i_0540b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540B8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x540F8; return 10; }
    8
}
/// $0540B8: bclr.b #$0, $13(a6)
pub fn i_0540b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540BE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0540BE: move.w #$1030, d0
pub fn i_0540be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540C2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0540C2: jsr $2352.l
pub fn i_0540c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540C8);
    cpu.pc = 0x2352;
    16
}
/// $0540C8: lea.l $29a278.l, a1
pub fn i_0540c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540CE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A278;
    20
}
/// $0540CE: jsr $77c7e.l
pub fn i_0540ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540D4);
    cpu.pc = 0x77C7E;
    16
}
/// $0540D4: lea.l $29a28a.l, a1
pub fn i_0540d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540DA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A28A;
    20
}
/// $0540DA: jsr $77c7e.l
pub fn i_0540da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540E0);
    cpu.pc = 0x77C7E;
    16
}
/// $0540E0: lea.l $29a3ce.l, a1
pub fn i_0540e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A3CE;
    20
}
/// $0540E6: jsr $77c7e.l
pub fn i_0540e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540EC);
    cpu.pc = 0x77C7E;
    16
}
/// $0540EC: move.w #$f000, $38(a0)
pub fn i_0540ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540F2;
    cpu.ir = 0x317C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0540F2: lea.l $5410a(pc), a1
pub fn i_0540f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5410A;
    16
}
/// $0540F6: move.l a1, (a6)
pub fn i_0540f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0540F8: jsr $4fa70.l
pub fn i_0540f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x540FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x540FE);
    cpu.pc = 0x4FA70;
    16
}
/// $0540FE: bcc.w $54108
pub fn i_0540fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54102;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54108; return 10; }
    8
}
/// $054102: jmp $518.l
pub fn i_054102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54108;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054108: rts
pub fn i_054108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5410A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_BuildingWreck_05410a  ($05410A, 86 B, props/mission_053f96.s)
/// $05410A: move.l #$2000, d0
pub fn i_05410a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54110;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054110: jsr $51a28.l
pub fn i_054110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54116;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54116);
    cpu.pc = 0x51A28;
    16
}
/// $054116: jsr $2783a.l
pub fn i_054116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5411C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5411C);
    cpu.pc = 0x2783A;
    16
}
/// $05411C: lea.l $29b2b2.l, a1
pub fn i_05411c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54122;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B2B2;
    20
}
/// $054122: jsr $43fac.l
pub fn i_054122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54128;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54128);
    cpu.pc = 0x43FAC;
    16
}
/// $054128: lea.l $ffff.w, a0
pub fn i_054128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5412C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $05412C: move.l a0, $48(a6)
pub fn i_05412c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54130;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $054130: lea.l $29897a.l, a0
pub fn i_054130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54136;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29897A;
    20
}
/// $054136: jsr $28cd4.l
pub fn i_054136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5413C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5413C);
    cpu.pc = 0x28CD4;
    16
}
/// $05413C: lea.l $54142(pc), a1
pub fn i_05413c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54140;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54142;
    16
}
/// $054140: move.l a1, (a6)
pub fn i_054140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54142;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054142: jsr $2783a.l
pub fn i_054142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54148;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54148);
    cpu.pc = 0x2783A;
    16
}
/// $054148: jsr $28d70.l
pub fn i_054148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5414E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5414E);
    cpu.pc = 0x28D70;
    16
}
/// $05414E: jsr $4fa70.l
pub fn i_05414e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54154);
    cpu.pc = 0x4FA70;
    16
}
/// $054154: bcc.w $5415e
pub fn i_054154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54158;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5415E; return 10; }
    8
}
/// $054158: jmp $518.l
pub fn i_054158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5415E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05415E: rts
pub fn i_05415e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54160;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Column_054160  ($054160, 244 B, props/mission_053f96.s)
/// $054160: movea.l $3c(a6), a1
pub fn i_054160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54164;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054164: jsr $2942a.l
pub fn i_054164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5416A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5416A);
    cpu.pc = 0x2942A;
    16
}
/// $05416A: move.w #$8c, d1
pub fn i_05416a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5416E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05416E: jsr $236e.l
pub fn i_05416e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54174);
    cpu.pc = 0x236E;
    16
}
/// $054174: move.w #$40, $70(a6)
pub fn i_054174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5417A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05417A: move.b #$ff, $32(a6)
pub fn i_05417a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54180;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054180: move.b #$ff, $33(a6)
pub fn i_054180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54186;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054186: move.b #$0, $3a(a6)
pub fn i_054186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5418C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05418C: move.w #$3c, $66(a6)
pub fn i_05418c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54192;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054192: move.w #$8000, $38(a6)
pub fn i_054192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54198;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054198: jsr $267e2.l
pub fn i_054198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5419E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5419E);
    cpu.pc = 0x267E2;
    16
}
/// $05419E: jsr $27cee.l
pub fn i_05419e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541A4);
    cpu.pc = 0x27CEE;
    16
}
/// $0541A4: lea.l $29899e.l, a0
pub fn i_0541a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29899E;
    20
}
/// $0541AA: jsr $28cd4.l
pub fn i_0541aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541B0);
    cpu.pc = 0x28CD4;
    16
}
/// $0541B0: lea.l $541b6(pc), a1
pub fn i_0541b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x541B6;
    16
}
/// $0541B4: move.l a1, (a6)
pub fn i_0541b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0541B6: jsr $2783a.l
pub fn i_0541b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541BC);
    cpu.pc = 0x2783A;
    16
}
/// $0541BC: jsr $28108.l
pub fn i_0541bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541C2);
    cpu.pc = 0x28108;
    16
}
/// $0541C2: jsr $28d70.l
pub fn i_0541c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541C8);
    cpu.pc = 0x28D70;
    16
}
/// $0541C8: jsr $2870a.l
pub fn i_0541c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541CE);
    cpu.pc = 0x2870A;
    16
}
/// $0541CE: bcc.w $541e4
pub fn i_0541ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x541E4; return 10; }
    8
}
/// $0541D2: lea.l $5e766.l, a0
pub fn i_0541d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0541D8: jsr $5e770.l
pub fn i_0541d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541DE);
    cpu.pc = 0x5E770;
    16
}
/// $0541DE: bclr.b #$3, $13(a6)
pub fn i_0541de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541E4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0541E4: jsr $28758.l
pub fn i_0541e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x541EA);
    cpu.pc = 0x28758;
    16
}
/// $0541EA: bcc.w $54242
pub fn i_0541ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54242; return 10; }
    8
}
/// $0541EE: bclr.b #$0, $13(a6)
pub fn i_0541ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0541F4: move.l #$3000, d0
pub fn i_0541f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x541FA;
    cpu.ir = 0x203C;
    let v1: u32 = 0x3000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0541FA: jsr $51a28.l
pub fn i_0541fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54200;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54200);
    cpu.pc = 0x51A28;
    16
}
/// $054200: move.w #$1023, d0
pub fn i_054200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54204;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054204: jsr $2352.l
pub fn i_054204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5420A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5420A);
    cpu.pc = 0x2352;
    16
}
/// $05420A: lea.l $29b368.l, a1
pub fn i_05420a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54210;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B368;
    20
}
/// $054210: jsr $43fac.l
pub fn i_054210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54216;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54216);
    cpu.pc = 0x43FAC;
    16
}
/// $054216: lea.l $29a29c.l, a1
pub fn i_054216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5421C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A29C;
    20
}
/// $05421C: jsr $77c7e.l
pub fn i_05421c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54222;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54222);
    cpu.pc = 0x77C7E;
    16
}
/// $054222: lea.l $29a2ae.l, a1
pub fn i_054222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54228;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A2AE;
    20
}
/// $054228: jsr $77c7e.l
pub fn i_054228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5422E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5422E);
    cpu.pc = 0x77C7E;
    16
}
/// $05422E: lea.l $29a3e0.l, a1
pub fn i_05422e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54234;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A3E0;
    20
}
/// $054234: jsr $77c7e.l
pub fn i_054234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5423A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5423A);
    cpu.pc = 0x77C7E;
    16
}
/// $05423A: jsr $55266(pc)
pub fn i_05423a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5423E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5423E);
    cpu.pc = 0x55266;
    16
}
/// $05423E: bra.w $5424c
pub fn i_05423e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54242;
    cpu.ir = 0x6000;
    cpu.pc = 0x5424C;
    10
}
/// $054242: jsr $4fa70.l
pub fn i_054242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54248;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54248);
    cpu.pc = 0x4FA70;
    16
}
/// $054248: bcc.w $54252
pub fn i_054248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5424C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54252; return 10; }
    8
}
/// $05424C: jmp $518.l
pub fn i_05424c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54252;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054252: rts
pub fn i_054252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54254;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_CompoundWall_054254  ($054254, 228 B, props/mission_053f96.s)
/// $054254: move.b #$0, $20(a6)
pub fn i_054254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5425A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05425A: move.w #$118, $66(a6)
pub fn i_05425a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54260;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x118;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054260: lea.l $2989b4.l, a0
pub fn i_054260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54266;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2989B4;
    20
}
/// $054266: jsr $28cd4.l
pub fn i_054266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5426C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5426C);
    cpu.pc = 0x28CD4;
    16
}
/// $05426C: jsr $2783a.l
pub fn i_05426c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54272;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54272);
    cpu.pc = 0x2783A;
    16
}
/// $054272: lea.l $29b2ca.l, a1
pub fn i_054272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54278;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B2CA;
    20
}
/// $054278: jsr $43fac.l
pub fn i_054278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5427E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5427E);
    cpu.pc = 0x43FAC;
    16
}
/// $05427E: bra.w $542b6
pub fn i_05427e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54282;
    cpu.ir = 0x6000;
    cpu.pc = 0x542B6;
    10
}
/// $054282: movea.l $3c(a6), a1
pub fn i_054282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54286;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054286: jsr $2942a.l
pub fn i_054286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5428C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5428C);
    cpu.pc = 0x2942A;
    16
}
/// $05428C: move.b #$1, $20(a6)
pub fn i_05428c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54292;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054292: move.w #$280, $66(a6)
pub fn i_054292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54298;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x280;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054298: lea.l $298a10.l, a0
pub fn i_054298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5429E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298A10;
    20
}
/// $05429E: jsr $28cd4.l
pub fn i_05429e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x542A4);
    cpu.pc = 0x28CD4;
    16
}
/// $0542A4: jsr $2783a.l
pub fn i_0542a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x542AA);
    cpu.pc = 0x2783A;
    16
}
/// $0542AA: lea.l $29b2f2.l, a1
pub fn i_0542aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542B0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B2F2;
    20
}
/// $0542B0: jsr $43fac.l
pub fn i_0542b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x542B6);
    cpu.pc = 0x43FAC;
    16
}
/// $0542B6: move.w #$60, $70(a6)
pub fn i_0542b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0542BC: lea.l $542c2(pc), a1
pub fn i_0542bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x542C2;
    16
}
/// $0542C0: move.l a1, (a6)
pub fn i_0542c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0542C2: clr.b $10e39a.l
pub fn i_0542c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542C8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0542C8: jsr $2783a.l
pub fn i_0542c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x542CE);
    cpu.pc = 0x2783A;
    16
}
/// $0542CE: jsr $28d70.l
pub fn i_0542ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x542D4);
    cpu.pc = 0x28D70;
    16
}
/// $0542D4: jsr $2870a.l
pub fn i_0542d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x542DA);
    cpu.pc = 0x2870A;
    16
}
/// $0542DA: bcc.w $542f0
pub fn i_0542da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x542F0; return 10; }
    8
}
/// $0542DE: lea.l $5e766.l, a0
pub fn i_0542de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0542E4: jsr $5e770.l
pub fn i_0542e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x542EA);
    cpu.pc = 0x5E770;
    16
}
/// $0542EA: bclr.b #$3, $13(a6)
pub fn i_0542ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542F0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0542F0: cmpi.w #$a0, $66(a6)
pub fn i_0542f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542F6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $0542F6: bgt.w $54326
pub fn i_0542f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542FA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x54326; return 10; }
    8
}
/// $0542FA: move.w #$102a, d0
pub fn i_0542fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x542FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0542FE: jsr $2352.l
pub fn i_0542fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54304);
    cpu.pc = 0x2352;
    16
}
/// $054304: lea.l $29a2d2.l, a1
pub fn i_054304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5430A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A2D2;
    20
}
/// $05430A: jsr $77c7e.l
pub fn i_05430a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54310;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54310);
    cpu.pc = 0x77C7E;
    16
}
/// $054310: cmpi.b #$0, $20(a6)
pub fn i_054310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54316;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $054316: beq.w $54320
pub fn i_054316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5431A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54320; return 10; }
    8
}
/// $05431A: subi.w #$30, $22(a0)
pub fn i_05431a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54320;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $054320: lea.l $54338(pc), a1
pub fn i_054320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54324;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54338;
    16
}
/// $054324: move.l a1, (a6)
pub fn i_054324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54326;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054326: jsr $4fa70.l
pub fn i_054326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5432C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5432C);
    cpu.pc = 0x4FA70;
    16
}
/// $05432C: bcc.w $54336
pub fn i_05432c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54330;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54336; return 10; }
    8
}
/// $054330: jmp $518.l
pub fn i_054330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54336;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054336: rts
pub fn i_054336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54338;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_CompoundWallDamaged_054338  ($054338, 192 B, props/mission_053f96.s)
/// $054338: cmpi.b #$0, $20(a6)
pub fn i_054338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5433E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $05433E: bne.w $54352
pub fn i_05433e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54342;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x54352; return 10; }
    8
}
/// $054342: lea.l $2989ca.l, a0
pub fn i_054342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54348;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2989CA;
    20
}
/// $054348: jsr $28cd4.l
pub fn i_054348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5434E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5434E);
    cpu.pc = 0x28CD4;
    16
}
/// $05434E: bra.w $5435e
pub fn i_05434e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54352;
    cpu.ir = 0x6000;
    cpu.pc = 0x5435E;
    10
}
/// $054352: lea.l $298a26.l, a0
pub fn i_054352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54358;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298A26;
    20
}
/// $054358: jsr $28cd4.l
pub fn i_054358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5435E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5435E);
    cpu.pc = 0x28CD4;
    16
}
/// $05435E: lea.l $54364(pc), a1
pub fn i_05435e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54362;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54364;
    16
}
/// $054362: move.l a1, (a6)
pub fn i_054362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54364;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054364: clr.b $10e39a.l
pub fn i_054364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5436A;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $05436A: jsr $2783a.l
pub fn i_05436a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54370;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54370);
    cpu.pc = 0x2783A;
    16
}
/// $054370: jsr $28d70.l
pub fn i_054370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54376;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54376);
    cpu.pc = 0x28D70;
    16
}
/// $054376: jsr $2870a.l
pub fn i_054376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5437C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5437C);
    cpu.pc = 0x2870A;
    16
}
/// $05437C: bcc.w $54392
pub fn i_05437c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54380;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54392; return 10; }
    8
}
/// $054380: lea.l $5e766.l, a0
pub fn i_054380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54386;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054386: jsr $5e770.l
pub fn i_054386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5438C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5438C);
    cpu.pc = 0x5E770;
    16
}
/// $05438C: bclr.b #$3, $13(a6)
pub fn i_05438c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54392;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054392: jsr $28758.l
pub fn i_054392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54398;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54398);
    cpu.pc = 0x28758;
    16
}
/// $054398: bcc.w $543e6
pub fn i_054398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5439C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x543E6; return 10; }
    8
}
/// $05439C: bclr.b #$0, $13(a6)
pub fn i_05439c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543A2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0543A2: move.w #$102b, d0
pub fn i_0543a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0543A6: jsr $2352.l
pub fn i_0543a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x543AC);
    cpu.pc = 0x2352;
    16
}
/// $0543AC: cmpi.b #$0, $20(a6)
pub fn i_0543ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543B2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0543B2: beq.w $543bc
pub fn i_0543b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x543BC; return 10; }
    8
}
/// $0543B6: subi.w #$30, $22(a6)
pub fn i_0543b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543BC;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0543BC: lea.l $29a2c0.l, a1
pub fn i_0543bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543C2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A2C0;
    20
}
/// $0543C2: jsr $77c7e.l
pub fn i_0543c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x543C8);
    cpu.pc = 0x77C7E;
    16
}
/// $0543C8: move.w #$c000, $38(a0)
pub fn i_0543c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543CE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0543CE: lea.l $29a3f2.l, a1
pub fn i_0543ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543D4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A3F2;
    20
}
/// $0543D4: jsr $77c7e.l
pub fn i_0543d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x543DA);
    cpu.pc = 0x77C7E;
    16
}
/// $0543DA: move.w #$c000, $38(a0)
pub fn i_0543da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543E0;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0543E0: lea.l $543f8(pc), a1
pub fn i_0543e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x543F8;
    16
}
/// $0543E4: move.l a1, (a6)
pub fn i_0543e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0543E6: jsr $4fa70.l
pub fn i_0543e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x543EC);
    cpu.pc = 0x4FA70;
    16
}
/// $0543EC: bcc.w $543f6
pub fn i_0543ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x543F6; return 10; }
    8
}
/// $0543F0: jmp $518.l
pub fn i_0543f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543F6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0543F6: rts
pub fn i_0543f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_CompoundWallWreck_0543f8  ($0543F8, 132 B, props/mission_053f96.s)
/// $0543F8: jsr $2783a.l
pub fn i_0543f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x543FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x543FE);
    cpu.pc = 0x2783A;
    16
}
/// $0543FE: cmpi.b #$0, $20(a6)
pub fn i_0543fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54404;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $054404: bne.w $5443a
pub fn i_054404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54408;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5443A; return 10; }
    8
}
/// $054408: move.l #$3000, d0
pub fn i_054408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5440E;
    cpu.ir = 0x203C;
    let v1: u32 = 0x3000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05440E: jsr $51a28.l
pub fn i_05440e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54414;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54414);
    cpu.pc = 0x51A28;
    16
}
/// $054414: movea.l $c(a6), a0
pub fn i_054414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54418;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $054418: move.b #$ff, $20(a0)
pub fn i_054418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5441E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05441E: lea.l $2989f4.l, a0
pub fn i_05441e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54424;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2989F4;
    20
}
/// $054424: jsr $28cd4.l
pub fn i_054424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5442A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5442A);
    cpu.pc = 0x28CD4;
    16
}
/// $05442A: lea.l $29b2de.l, a1
pub fn i_05442a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54430;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B2DE;
    20
}
/// $054430: jsr $43fac.l
pub fn i_054430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54436);
    cpu.pc = 0x43FAC;
    16
}
/// $054436: bra.w $54464
pub fn i_054436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5443A;
    cpu.ir = 0x6000;
    cpu.pc = 0x54464;
    10
}
/// $05443A: move.l #$1000, d0
pub fn i_05443a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54440;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054440: jsr $51a28.l
pub fn i_054440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54446;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54446);
    cpu.pc = 0x51A28;
    16
}
/// $054446: addi.w #$30, $22(a6)
pub fn i_054446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5444C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05444C: lea.l $298a3c.l, a0
pub fn i_05444c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54452;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298A3C;
    20
}
/// $054452: jsr $28cd4.l
pub fn i_054452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54458;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54458);
    cpu.pc = 0x28CD4;
    16
}
/// $054458: lea.l $29b306.l, a1
pub fn i_054458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5445E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B306;
    20
}
/// $05445E: jsr $43fac.l
pub fn i_05445e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54464;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54464);
    cpu.pc = 0x43FAC;
    16
}
/// $054464: lea.l $5446a(pc), a1
pub fn i_054464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54468;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5446A;
    16
}
/// $054468: move.l a1, (a6)
pub fn i_054468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5446A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05446A: jsr $2783a.l
pub fn i_05446a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54470;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54470);
    cpu.pc = 0x2783A;
    16
}
/// $054470: jsr $28d70.l
pub fn i_054470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54476;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54476);
    cpu.pc = 0x28D70;
    16
}
/// $054476: jmp $518.l
pub fn i_054476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5447C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Prop_Nop_05447c  ($05447C, 2 B, props/mission_053f96.s)
/// $05447C: rts
pub fn i_05447c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5447E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Compound_Spawn_05447e  ($05447E, 76 B, props/mission_053f96.s)
/// $05447E: movea.l $3c(a6), a1
pub fn i_05447e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54482;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054482: jsr $2942a.l
pub fn i_054482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54488;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54488);
    cpu.pc = 0x2942A;
    16
}
/// $054488: move.w #$60, $70(a6)
pub fn i_054488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5448E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05448E: lea.l $54254(pc), a1
pub fn i_05448e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54492;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54254;
    16
}
/// $054492: jsr $4ae.l
pub fn i_054492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54498);
    cpu.pc = 0x4AE;
    16
}
/// $054498: jsr $5dd22.l
pub fn i_054498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5449E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5449E);
    cpu.pc = 0x5DD22;
    16
}
/// $05449E: lea.l $545c0(pc), a1
pub fn i_05449e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x545C0;
    16
}
/// $0544A2: jsr $4ae.l
pub fn i_0544a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x544A8);
    cpu.pc = 0x4AE;
    16
}
/// $0544A8: jsr $5dd22.l
pub fn i_0544a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x544AE);
    cpu.pc = 0x5DD22;
    16
}
/// $0544AE: lea.l $54584(pc), a1
pub fn i_0544ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54584;
    16
}
/// $0544B2: jsr $4ae.l
pub fn i_0544b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x544B8);
    cpu.pc = 0x4AE;
    16
}
/// $0544B8: jsr $5dd22.l
pub fn i_0544b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x544BE);
    cpu.pc = 0x5DD22;
    16
}
/// $0544BE: move.b #$0, $20(a6)
pub fn i_0544be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0544C4: move.b #$0, $80(a6)
pub fn i_0544c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Prop_Compound_PhaseA_0544ca  ($0544CA, 78 B, props/mission_053f96.s)
/// $0544CA: move.b #$0, $21(a6)
pub fn i_0544ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544D0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0544D0: lea.l $544d6(pc), a1
pub fn i_0544d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x544D6;
    16
}
/// $0544D4: move.l a1, (a6)
pub fn i_0544d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0544D6: jsr $2783a.l
pub fn i_0544d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x544DC);
    cpu.pc = 0x2783A;
    16
}
/// $0544DC: cmpi.b #$1, $21(a6)
pub fn i_0544dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544E2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0544E2: bne.w $544ec
pub fn i_0544e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x544EC; return 10; }
    8
}
/// $0544E6: lea.l $54518(pc), a1
pub fn i_0544e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54518;
    16
}
/// $0544EA: move.l a1, (a6)
pub fn i_0544ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0544EC: cmpi.b #$ff, $80(a6)
pub fn i_0544ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544F2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0544F2: bne.w $544fc
pub fn i_0544f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x544FC; return 10; }
    8
}
/// $0544F6: lea.l $54566(pc), a1
pub fn i_0544f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54566;
    16
}
/// $0544FA: move.l a1, (a6)
pub fn i_0544fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x544FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0544FC: jsr $6f0.l
pub fn i_0544fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54502);
    cpu.pc = 0x6F0;
    16
}
/// $054502: bcs.w $54516
pub fn i_054502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54506;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x54516; return 10; }
    8
}
/// $054506: jsr $4fa70.l
pub fn i_054506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5450C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5450C);
    cpu.pc = 0x4FA70;
    16
}
/// $05450C: bcc.w $54516
pub fn i_05450c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54510;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54516; return 10; }
    8
}
/// $054510: jmp $518.l
pub fn i_054510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54516;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054516: rts
pub fn i_054516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54518;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Compound_PhaseB_054518  ($054518, 78 B, props/mission_053f96.s)
/// $054518: move.b #$ff, $21(a6)
pub fn i_054518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5451E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05451E: lea.l $54524(pc), a1
pub fn i_05451e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54522;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54524;
    16
}
/// $054522: move.l a1, (a6)
pub fn i_054522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54524;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054524: jsr $2783a.l
pub fn i_054524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5452A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5452A);
    cpu.pc = 0x2783A;
    16
}
/// $05452A: cmpi.b #$2, $21(a6)
pub fn i_05452a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54530;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $054530: bne.w $5453a
pub fn i_054530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54534;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5453A; return 10; }
    8
}
/// $054534: lea.l $544ca(pc), a1
pub fn i_054534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54538;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x544CA;
    16
}
/// $054538: move.l a1, (a6)
pub fn i_054538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5453A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05453A: cmpi.b #$ff, $80(a6)
pub fn i_05453a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54540;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $054540: bne.w $5454a
pub fn i_054540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54544;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5454A; return 10; }
    8
}
/// $054544: lea.l $54566(pc), a1
pub fn i_054544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54548;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54566;
    16
}
/// $054548: move.l a1, (a6)
pub fn i_054548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5454A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05454A: jsr $6f0.l
pub fn i_05454a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54550);
    cpu.pc = 0x6F0;
    16
}
/// $054550: bcs.w $54564
pub fn i_054550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54554;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x54564; return 10; }
    8
}
/// $054554: jsr $4fa70.l
pub fn i_054554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5455A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5455A);
    cpu.pc = 0x4FA70;
    16
}
/// $05455A: bcc.w $54564
pub fn i_05455a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5455E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54564; return 10; }
    8
}
/// $05455E: jmp $518.l
pub fn i_05455e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54564;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054564: rts
pub fn i_054564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54566;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Compound_Done_054566  ($054566, 30 B, props/mission_053f96.s)
/// $054566: move.b #$ff, $21(a6)
pub fn i_054566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5456C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05456C: lea.l $54572(pc), a1
pub fn i_05456c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54570;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54572;
    16
}
/// $054570: move.l a1, (a6)
pub fn i_054570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54572;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054572: jsr $6f0.l
pub fn i_054572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54578;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54578);
    cpu.pc = 0x6F0;
    16
}
/// $054578: bcs.w $54582
pub fn i_054578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5457C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x54582; return 10; }
    8
}
/// $05457C: jmp $518.l
pub fn i_05457c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54582;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054582: rts
pub fn i_054582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54584;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Compound_TriggerSpawn_054584  ($054584, 60 B, props/mission_053f96.s)
/// $054584: lea.l $5458a(pc), a1
pub fn i_054584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54588;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5458A;
    16
}
/// $054588: move.l a1, (a6)
pub fn i_054588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5458A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05458A: jsr $2783a.l
pub fn i_05458a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54590);
    cpu.pc = 0x2783A;
    16
}
/// $054590: cmpi.w #$e0, $22(a6)
pub fn i_054590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54596;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xE0, Size::Word, false);
    16
}
/// $054596: bgt.w $545be
pub fn i_054596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5459A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x545BE; return 10; }
    8
}
/// $05459A: lea.l $79eb8.l, a1
pub fn i_05459a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545A0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x79EB8;
    20
}
/// $0545A0: jsr $4ae.l
pub fn i_0545a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x545A6);
    cpu.pc = 0x4AE;
    16
}
/// $0545A6: jsr $5dd22.l
pub fn i_0545a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x545AC);
    cpu.pc = 0x5DD22;
    16
}
/// $0545AC: addi.w #$b0, $22(a0)
pub fn i_0545ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545B2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xB0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0545B2: addi.w #$40, $24(a0)
pub fn i_0545b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545B8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0545B8: jmp $518.l
pub fn i_0545b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545BE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0545BE: rts
pub fn i_0545be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_NeonSign_0545c0  ($0545C0, 250 B, props/mission_053f96.s)
/// $0545C0: jsr $2783a.l
pub fn i_0545c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x545C6);
    cpu.pc = 0x2783A;
    16
}
/// $0545C6: cmpi.w #$120, $22(a6)
pub fn i_0545c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545CC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $0545CC: ble.w $545d2
pub fn i_0545cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545D0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x545D2; return 10; }
    8
}
/// $0545D0: rts
pub fn i_0545d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0545D2: move.w #$8d, d1
pub fn i_0545d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545D6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0545D6: jsr $236e.l
pub fn i_0545d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x545DC);
    cpu.pc = 0x236E;
    16
}
/// $0545DC: move.w #$40, $70(a6)
pub fn i_0545dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545E2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0545E2: addi.w #$a0, $22(a6)
pub fn i_0545e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545E8;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0545E8: addi.w #$60, $24(a6)
pub fn i_0545e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545EE;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0545EE: move.b #$ff, $32(a6)
pub fn i_0545ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0545F4: move.b #$ff, $33(a6)
pub fn i_0545f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545FA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0545FA: move.w #$8000, d0
pub fn i_0545fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x545FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0545FE: move.w #$b0, d1
pub fn i_0545fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54602;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054602: jsr $2813c.l
pub fn i_054602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54608;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54608);
    cpu.pc = 0x2813C;
    16
}
/// $054608: move.b #$0, $3a(a6)
pub fn i_054608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5460E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05460E: move.w #$3c, $66(a6)
pub fn i_05460e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54614;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054614: lea.l $29908c.l, a0
pub fn i_054614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5461A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29908C;
    20
}
/// $05461A: jsr $28cd4.l
pub fn i_05461a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54620;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54620);
    cpu.pc = 0x28CD4;
    16
}
/// $054620: move.w #$0, $72(a6)
pub fn i_054620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54626;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054626: move.b #$0, $74(a6)
pub fn i_054626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5462C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05462C: lea.l $54632(pc), a1
pub fn i_05462c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54630;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54632;
    16
}
/// $054630: move.l a1, (a6)
pub fn i_054630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54632;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054632: jsr $2783a.l
pub fn i_054632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54638;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54638);
    cpu.pc = 0x2783A;
    16
}
/// $054638: jsr $28108.l
pub fn i_054638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5463E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5463E);
    cpu.pc = 0x28108;
    16
}
/// $05463E: jsr $5509c(pc)
pub fn i_05463e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54642;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x54642);
    cpu.pc = 0x5509C;
    16
}
/// $054642: jsr $28d70.l
pub fn i_054642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54648;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54648);
    cpu.pc = 0x28D70;
    16
}
/// $054648: jsr $2870a.l
pub fn i_054648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5464E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5464E);
    cpu.pc = 0x2870A;
    16
}
/// $05464E: bcc.w $54664
pub fn i_05464e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54652;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54664; return 10; }
    8
}
/// $054652: lea.l $5e766.l, a0
pub fn i_054652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54658;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054658: jsr $5e770.l
pub fn i_054658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5465E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5465E);
    cpu.pc = 0x5E770;
    16
}
/// $05465E: bclr.b #$3, $13(a6)
pub fn i_05465e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54664;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054664: jsr $28758.l
pub fn i_054664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5466A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5466A);
    cpu.pc = 0x28758;
    16
}
/// $05466A: bcc.w $546a8
pub fn i_05466a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5466E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x546A8; return 10; }
    8
}
/// $05466E: bclr.b #$0, $13(a6)
pub fn i_05466e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54674;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054674: move.w #$102a, d0
pub fn i_054674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54678;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054678: jsr $2352.l
pub fn i_054678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5467E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5467E);
    cpu.pc = 0x2352;
    16
}
/// $05467E: lea.l $29a2e4.l, a1
pub fn i_05467e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54684;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A2E4;
    20
}
/// $054684: jsr $77c7e.l
pub fn i_054684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5468A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5468A);
    cpu.pc = 0x77C7E;
    16
}
/// $05468A: lea.l $29a2f6.l, a1
pub fn i_05468a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54690;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A2F6;
    20
}
/// $054690: jsr $77c7e.l
pub fn i_054690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54696;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54696);
    cpu.pc = 0x77C7E;
    16
}
/// $054696: lea.l $29a404.l, a1
pub fn i_054696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5469C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A404;
    20
}
/// $05469C: jsr $77c7e.l
pub fn i_05469c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546A2);
    cpu.pc = 0x77C7E;
    16
}
/// $0546A2: lea.l $546ba(pc), a1
pub fn i_0546a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x546BA;
    16
}
/// $0546A6: move.l a1, (a6)
pub fn i_0546a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0546A8: jsr $4fa70.l
pub fn i_0546a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546AE);
    cpu.pc = 0x4FA70;
    16
}
/// $0546AE: bcc.w $546b8
pub fn i_0546ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x546B8; return 10; }
    8
}
/// $0546B2: jmp $518.l
pub fn i_0546b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546B8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0546B8: rts
pub fn i_0546b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_NeonSignWreck_0546ba  ($0546BA, 58 B, props/mission_053f96.s)
/// $0546BA: lea.l $2990a2.l, a0
pub fn i_0546ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2990A2;
    20
}
/// $0546C0: jsr $28cd4.l
pub fn i_0546c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546C6);
    cpu.pc = 0x28CD4;
    16
}
/// $0546C6: lea.l $546cc(pc), a1
pub fn i_0546c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x546CC;
    16
}
/// $0546CA: move.l a1, (a6)
pub fn i_0546ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0546CC: jsr $2783a.l
pub fn i_0546cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546D2);
    cpu.pc = 0x2783A;
    16
}
/// $0546D2: jsr $28108.l
pub fn i_0546d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546D8);
    cpu.pc = 0x28108;
    16
}
/// $0546D8: jsr $5509c(pc)
pub fn i_0546d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x546DC);
    cpu.pc = 0x5509C;
    16
}
/// $0546DC: jsr $28d70.l
pub fn i_0546dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546E2);
    cpu.pc = 0x28D70;
    16
}
/// $0546E2: jsr $4fa70.l
pub fn i_0546e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546E8);
    cpu.pc = 0x4FA70;
    16
}
/// $0546E8: bcc.w $546f2
pub fn i_0546e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546EC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x546F2; return 10; }
    8
}
/// $0546EC: jmp $518.l
pub fn i_0546ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546F2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0546F2: rts
pub fn i_0546f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Stall_0546f4  ($0546F4, 214 B, props/mission_053f96.s)
/// $0546F4: movea.l $3c(a6), a1
pub fn i_0546f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546F8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0546F8: jsr $2942a.l
pub fn i_0546f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x546FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x546FE);
    cpu.pc = 0x2942A;
    16
}
/// $0546FE: move.w #$91, d1
pub fn i_0546fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54702;
    cpu.ir = 0x323C;
    let v1: u32 = 0x91;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054702: jsr $236e.l
pub fn i_054702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54708);
    cpu.pc = 0x236E;
    16
}
/// $054708: move.w #$a0, $70(a6)
pub fn i_054708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5470E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05470E: move.b #$ff, $32(a6)
pub fn i_05470e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54714;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054714: move.b #$ff, $33(a6)
pub fn i_054714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5471A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05471A: move.b #$0, $3a(a6)
pub fn i_05471a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54720;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054720: move.w #$3c, $66(a6)
pub fn i_054720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54726;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054726: move.w #$8000, d0
pub fn i_054726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5472A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05472A: move.w #$a0, d1
pub fn i_05472a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5472E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05472E: jsr $2813c.l
pub fn i_05472e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54734;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54734);
    cpu.pc = 0x2813C;
    16
}
/// $054734: lea.l $298aa8.l, a0
pub fn i_054734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5473A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298AA8;
    20
}
/// $05473A: jsr $28cd4.l
pub fn i_05473a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54740;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54740);
    cpu.pc = 0x28CD4;
    16
}
/// $054740: lea.l $54746(pc), a1
pub fn i_054740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54744;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54746;
    16
}
/// $054744: move.l a1, (a6)
pub fn i_054744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54746;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054746: jsr $2783a.l
pub fn i_054746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5474C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5474C);
    cpu.pc = 0x2783A;
    16
}
/// $05474C: jsr $28108.l
pub fn i_05474c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54752;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54752);
    cpu.pc = 0x28108;
    16
}
/// $054752: jsr $28d70.l
pub fn i_054752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54758;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54758);
    cpu.pc = 0x28D70;
    16
}
/// $054758: jsr $2870a.l
pub fn i_054758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5475E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5475E);
    cpu.pc = 0x2870A;
    16
}
/// $05475E: bcc.w $54774
pub fn i_05475e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54762;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54774; return 10; }
    8
}
/// $054762: lea.l $5e766.l, a0
pub fn i_054762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54768;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054768: jsr $5e770.l
pub fn i_054768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5476E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5476E);
    cpu.pc = 0x5E770;
    16
}
/// $05476E: bclr.b #$3, $13(a6)
pub fn i_05476e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54774;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054774: jsr $28758.l
pub fn i_054774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5477A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5477A);
    cpu.pc = 0x28758;
    16
}
/// $05477A: bcc.w $547b8
pub fn i_05477a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5477E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x547B8; return 10; }
    8
}
/// $05477E: bclr.b #$0, $13(a6)
pub fn i_05477e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54784;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054784: move.w #$102c, d0
pub fn i_054784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54788;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054788: jsr $2352.l
pub fn i_054788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5478E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5478E);
    cpu.pc = 0x2352;
    16
}
/// $05478E: lea.l $29a32c.l, a1
pub fn i_05478e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54794;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A32C;
    20
}
/// $054794: jsr $77c7e.l
pub fn i_054794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5479A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5479A);
    cpu.pc = 0x77C7E;
    16
}
/// $05479A: lea.l $29a33e.l, a1
pub fn i_05479a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547A0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A33E;
    20
}
/// $0547A0: jsr $77c7e.l
pub fn i_0547a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x547A6);
    cpu.pc = 0x77C7E;
    16
}
/// $0547A6: lea.l $29a428.l, a1
pub fn i_0547a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547AC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A428;
    20
}
/// $0547AC: jsr $77c7e.l
pub fn i_0547ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x547B2);
    cpu.pc = 0x77C7E;
    16
}
/// $0547B2: lea.l $547ca(pc), a1
pub fn i_0547b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x547CA;
    16
}
/// $0547B6: move.l a1, (a6)
pub fn i_0547b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0547B8: jsr $4fa70.l
pub fn i_0547b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x547BE);
    cpu.pc = 0x4FA70;
    16
}
/// $0547BE: bcc.w $547c8
pub fn i_0547be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x547C8; return 10; }
    8
}
/// $0547C2: jmp $518.l
pub fn i_0547c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547C8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0547C8: rts
pub fn i_0547c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_StallWreck_0547ca  ($0547CA, 80 B, props/mission_053f96.s)
/// $0547CA: jsr $2783a.l
pub fn i_0547ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x547D0);
    cpu.pc = 0x2783A;
    16
}
/// $0547D0: lea.l $298b24.l, a0
pub fn i_0547d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B24;
    20
}
/// $0547D6: jsr $28cd4.l
pub fn i_0547d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x547DC);
    cpu.pc = 0x28CD4;
    16
}
/// $0547DC: move.l $106f50.l, d0
pub fn i_0547dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547E2;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0547E2: swap d0
pub fn i_0547e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547E4;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0547E4: cmpi.w #$390, d0
pub fn i_0547e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547E8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x390, Size::Word, false);
    8
}
/// $0547E8: bge.w $547fc
pub fn i_0547e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547EC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x547FC; return 10; }
    8
}
/// $0547EC: jsr $552c8(pc)
pub fn i_0547ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x547F0);
    cpu.pc = 0x552C8;
    16
}
/// $0547F0: lea.l $29b3b2.l, a1
pub fn i_0547f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B3B2;
    20
}
/// $0547F6: jsr $43fac.l
pub fn i_0547f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x547FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x547FC);
    cpu.pc = 0x43FAC;
    16
}
/// $0547FC: lea.l $54802(pc), a1
pub fn i_0547fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54800;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54802;
    16
}
/// $054800: move.l a1, (a6)
pub fn i_054800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54802;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054802: jsr $2783a.l
pub fn i_054802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54808;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54808);
    cpu.pc = 0x2783A;
    16
}
/// $054808: jsr $28108.l
pub fn i_054808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5480E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5480E);
    cpu.pc = 0x28108;
    16
}
/// $05480E: jsr $28d70.l
pub fn i_05480e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54814;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54814);
    cpu.pc = 0x28D70;
    16
}
/// $054814: jmp $518.l
pub fn i_054814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5481A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Prop_Nop_05481a  ($05481A, 2 B, props/mission_053f96.s)
/// $05481A: rts
pub fn i_05481a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5481C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Fragile_05481c  ($05481C, 172 B, props/mission_053f96.s)
/// $05481C: movea.l $3c(a6), a1
pub fn i_05481c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54820;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054820: jsr $2942a.l
pub fn i_054820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54826);
    cpu.pc = 0x2942A;
    16
}
/// $054826: move.w #$92, d1
pub fn i_054826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5482A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x92;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05482A: jsr $236e.l
pub fn i_05482a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54830;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54830);
    cpu.pc = 0x236E;
    16
}
/// $054830: move.w #$80, $70(a6)
pub fn i_054830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54836;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054836: move.b #$ff, $32(a6)
pub fn i_054836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5483C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05483C: move.b #$ff, $33(a6)
pub fn i_05483c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54842;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054842: move.b #$0, $3a(a6)
pub fn i_054842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54848;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054848: move.w #$1, $66(a6)
pub fn i_054848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5484E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05484E: lea.l $298b34.l, a0
pub fn i_05484e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54854;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B34;
    20
}
/// $054854: jsr $28cd4.l
pub fn i_054854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5485A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5485A);
    cpu.pc = 0x28CD4;
    16
}
/// $05485A: lea.l $54860(pc), a1
pub fn i_05485a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5485E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54860;
    16
}
/// $05485E: move.l a1, (a6)
pub fn i_05485e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54860;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054860: jsr $2783a.l
pub fn i_054860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54866;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54866);
    cpu.pc = 0x2783A;
    16
}
/// $054866: jsr $28d70.l
pub fn i_054866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5486C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5486C);
    cpu.pc = 0x28D70;
    16
}
/// $05486C: jsr $2870a.l
pub fn i_05486c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54872;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54872);
    cpu.pc = 0x2870A;
    16
}
/// $054872: bcc.w $54888
pub fn i_054872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54876;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54888; return 10; }
    8
}
/// $054876: lea.l $5e766.l, a0
pub fn i_054876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5487C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $05487C: jsr $5e770.l
pub fn i_05487c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54882);
    cpu.pc = 0x5E770;
    16
}
/// $054882: bclr.b #$3, $13(a6)
pub fn i_054882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54888;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054888: jsr $28758.l
pub fn i_054888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5488E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5488E);
    cpu.pc = 0x28758;
    16
}
/// $05488E: bcc.w $548b6
pub fn i_05488e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54892;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x548B6; return 10; }
    8
}
/// $054892: bclr.b #$0, $13(a6)
pub fn i_054892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54898;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054898: lea.l $29a350.l, a1
pub fn i_054898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5489E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A350;
    20
}
/// $05489E: jsr $77c7e.l
pub fn i_05489e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548A4);
    cpu.pc = 0x77C7E;
    16
}
/// $0548A4: lea.l $29a43a.l, a1
pub fn i_0548a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548AA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A43A;
    20
}
/// $0548AA: jsr $77c7e.l
pub fn i_0548aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548B0);
    cpu.pc = 0x77C7E;
    16
}
/// $0548B0: lea.l $548c8(pc), a1
pub fn i_0548b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x548C8;
    16
}
/// $0548B4: move.l a1, (a6)
pub fn i_0548b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0548B6: jsr $4fa70.l
pub fn i_0548b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548BC);
    cpu.pc = 0x4FA70;
    16
}
/// $0548BC: bcc.w $548c6
pub fn i_0548bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548C0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x548C6; return 10; }
    8
}
/// $0548C0: jmp $518.l
pub fn i_0548c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548C6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0548C6: rts
pub fn i_0548c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_FragileBreaking_0548c8  ($0548C8, 68 B, props/mission_053f96.s)
/// $0548C8: move.w #$1054, d0
pub fn i_0548c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548CC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1054;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0548CC: jsr $2352.l
pub fn i_0548cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548D2);
    cpu.pc = 0x2352;
    16
}
/// $0548D2: lea.l $298b4a.l, a0
pub fn i_0548d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B4A;
    20
}
/// $0548D8: jsr $28cd4.l
pub fn i_0548d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548DE);
    cpu.pc = 0x28CD4;
    16
}
/// $0548DE: lea.l $548e4(pc), a1
pub fn i_0548de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x548E4;
    16
}
/// $0548E2: move.l a1, (a6)
pub fn i_0548e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0548E4: jsr $2783a.l
pub fn i_0548e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548EA);
    cpu.pc = 0x2783A;
    16
}
/// $0548EA: jsr $28d70.l
pub fn i_0548ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x548F0);
    cpu.pc = 0x28D70;
    16
}
/// $0548F0: bcc.w $548fa
pub fn i_0548f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x548FA; return 10; }
    8
}
/// $0548F4: lea.l $5490c(pc), a1
pub fn i_0548f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5490C;
    16
}
/// $0548F8: move.l a1, (a6)
pub fn i_0548f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x548FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0548FA: jsr $4fa70.l
pub fn i_0548fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54900);
    cpu.pc = 0x4FA70;
    16
}
/// $054900: bcc.w $5490a
pub fn i_054900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54904;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5490A; return 10; }
    8
}
/// $054904: jmp $518.l
pub fn i_054904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5490A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05490A: rts
pub fn i_05490a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5490C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_FragileWreck_05490c  ($05490C, 78 B, props/mission_053f96.s)
/// $05490C: jsr $2783a.l
pub fn i_05490c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54912);
    cpu.pc = 0x2783A;
    16
}
/// $054912: move.l #$1000, d0
pub fn i_054912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54918;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054918: jsr $51a28.l
pub fn i_054918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5491E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5491E);
    cpu.pc = 0x51A28;
    16
}
/// $05491E: lea.l $29b380.l, a1
pub fn i_05491e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54924;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B380;
    20
}
/// $054924: jsr $43fac.l
pub fn i_054924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5492A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5492A);
    cpu.pc = 0x43FAC;
    16
}
/// $05492A: lea.l $298b78.l, a0
pub fn i_05492a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54930;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B78;
    20
}
/// $054930: jsr $28cd4.l
pub fn i_054930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54936);
    cpu.pc = 0x28CD4;
    16
}
/// $054936: lea.l $5493c(pc), a1
pub fn i_054936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5493A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5493C;
    16
}
/// $05493A: move.l a1, (a6)
pub fn i_05493a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5493C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05493C: jsr $2783a.l
pub fn i_05493c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54942;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54942);
    cpu.pc = 0x2783A;
    16
}
/// $054942: jsr $28d70.l
pub fn i_054942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54948;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54948);
    cpu.pc = 0x28D70;
    16
}
/// $054948: jsr $4fa70.l
pub fn i_054948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5494E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5494E);
    cpu.pc = 0x4FA70;
    16
}
/// $05494E: bcc.w $54958
pub fn i_05494e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54952;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54958; return 10; }
    8
}
/// $054952: jmp $518.l
pub fn i_054952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54958;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054958: rts
pub fn i_054958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5495A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Breakable_05495a  ($05495A, 192 B, props/mission_053f96.s)
/// $05495A: movea.l $3c(a6), a1
pub fn i_05495a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5495E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05495E: jsr $2942a.l
pub fn i_05495e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54964);
    cpu.pc = 0x2942A;
    16
}
/// $054964: move.w #$93, d1
pub fn i_054964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54968;
    cpu.ir = 0x323C;
    let v1: u32 = 0x93;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054968: jsr $236e.l
pub fn i_054968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5496E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5496E);
    cpu.pc = 0x236E;
    16
}
/// $05496E: move.w #$50, $70(a6)
pub fn i_05496e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54974;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054974: move.b #$ff, $32(a6)
pub fn i_054974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5497A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05497A: move.b #$ff, $33(a6)
pub fn i_05497a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54980;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054980: move.b #$0, $3a(a6)
pub fn i_054980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54986;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054986: move.w #$1, $66(a6)
pub fn i_054986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5498C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05498C: lea.l $298b88.l, a0
pub fn i_05498c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54992;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B88;
    20
}
/// $054992: jsr $28cd4.l
pub fn i_054992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54998;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54998);
    cpu.pc = 0x28CD4;
    16
}
/// $054998: lea.l $5499e(pc), a1
pub fn i_054998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5499C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5499E;
    16
}
/// $05499C: move.l a1, (a6)
pub fn i_05499c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5499E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05499E: jsr $2783a.l
pub fn i_05499e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549A4);
    cpu.pc = 0x2783A;
    16
}
/// $0549A4: jsr $28d70.l
pub fn i_0549a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549AA);
    cpu.pc = 0x28D70;
    16
}
/// $0549AA: jsr $2870a.l
pub fn i_0549aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549B0);
    cpu.pc = 0x2870A;
    16
}
/// $0549B0: bcc.w $549ba
pub fn i_0549b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x549BA; return 10; }
    8
}
/// $0549B4: bclr.b #$3, $13(a6)
pub fn i_0549b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549BA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0549BA: jsr $28758.l
pub fn i_0549ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549C0);
    cpu.pc = 0x28758;
    16
}
/// $0549C0: bcc.w $54a08
pub fn i_0549c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549C4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54A08; return 10; }
    8
}
/// $0549C4: bclr.b #$0, $13(a6)
pub fn i_0549c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549CA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0549CA: move.w #$102e, d0
pub fn i_0549ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549CE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0549CE: jsr $2352.l
pub fn i_0549ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549D4);
    cpu.pc = 0x2352;
    16
}
/// $0549D4: lea.l $29b358.l, a1
pub fn i_0549d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549DA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B358;
    20
}
/// $0549DA: jsr $43fac.l
pub fn i_0549da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549E0);
    cpu.pc = 0x43FAC;
    16
}
/// $0549E0: lea.l $29a362.l, a1
pub fn i_0549e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A362;
    20
}
/// $0549E6: jsr $77c7e.l
pub fn i_0549e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549EC);
    cpu.pc = 0x77C7E;
    16
}
/// $0549EC: lea.l $29a374.l, a1
pub fn i_0549ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549F2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A374;
    20
}
/// $0549F2: jsr $77c7e.l
pub fn i_0549f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x549F8);
    cpu.pc = 0x77C7E;
    16
}
/// $0549F8: lea.l $29a44c.l, a1
pub fn i_0549f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x549FE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A44C;
    20
}
/// $0549FE: jsr $77c7e.l
pub fn i_0549fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A04);
    cpu.pc = 0x77C7E;
    16
}
/// $054A04: bra.w $54a12
pub fn i_054a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A08;
    cpu.ir = 0x6000;
    cpu.pc = 0x54A12;
    10
}
/// $054A08: jsr $4fa70.l
pub fn i_054a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A0E);
    cpu.pc = 0x4FA70;
    16
}
/// $054A0E: bcc.w $54a18
pub fn i_054a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54A18; return 10; }
    8
}
/// $054A12: jmp $518.l
pub fn i_054a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A18;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054A18: rts
pub fn i_054a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_HitStages_054a1a  ($054A1A, 258 B, props/mission_053f96.s)
/// $054A1A: movea.l $3c(a6), a1
pub fn i_054a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A1E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054A1E: jsr $2942a.l
pub fn i_054a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A24);
    cpu.pc = 0x2942A;
    16
}
/// $054A24: move.w #$7a, d1
pub fn i_054a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A28;
    cpu.ir = 0x323C;
    let v1: u32 = 0x7A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054A28: jsr $236e.l
pub fn i_054a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A2E);
    cpu.pc = 0x236E;
    16
}
/// $054A2E: move.w #$40, $70(a6)
pub fn i_054a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A34;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054A34: move.b #$ff, $32(a6)
pub fn i_054a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A3A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054A3A: move.b #$ff, $33(a6)
pub fn i_054a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A40;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054A40: move.b #$0, $3a(a6)
pub fn i_054a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A46;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054A46: move.w #$46, $66(a6)
pub fn i_054a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A4C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x46;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054A4C: move.b #$6, $44(a6)
pub fn i_054a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054A52: lea.l $298b9e.l, a0
pub fn i_054a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B9E;
    20
}
/// $054A58: jsr $28cd4.l
pub fn i_054a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A5E);
    cpu.pc = 0x28CD4;
    16
}
/// $054A5E: lea.l $54a64(pc), a1
pub fn i_054a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54A64;
    16
}
/// $054A62: move.l a1, (a6)
pub fn i_054a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054A64: jsr $2783a.l
pub fn i_054a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A6A);
    cpu.pc = 0x2783A;
    16
}
/// $054A6A: jsr $28d70.l
pub fn i_054a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A70);
    cpu.pc = 0x28D70;
    16
}
/// $054A70: bcc.w $54a80
pub fn i_054a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A74;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54A80; return 10; }
    8
}
/// $054A74: lea.l $298b9e.l, a0
pub fn i_054a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298B9E;
    20
}
/// $054A7A: jsr $28cd4.l
pub fn i_054a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A80);
    cpu.pc = 0x28CD4;
    16
}
/// $054A80: jsr $2870a.l
pub fn i_054a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A86);
    cpu.pc = 0x2870A;
    16
}
/// $054A86: bcc.w $54abc
pub fn i_054a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54ABC; return 10; }
    8
}
/// $054A8A: lea.l $5e766.l, a0
pub fn i_054a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A90;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054A90: jsr $5e770.l
pub fn i_054a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54A96);
    cpu.pc = 0x5E770;
    16
}
/// $054A96: bclr.b #$3, $13(a6)
pub fn i_054a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54A9C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054A9C: move.w $72(a6), d0
pub fn i_054a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AA0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $054AA0: movea.l #$298c46, a0
pub fn i_054aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AA6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x298C46;
    cpu.a[0] = v1;
    4
}
/// $054AA6: lsl.w #$2, d0
pub fn i_054aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AA8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $054AA8: movea.l (a0, d0.w), a0
pub fn i_054aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AAC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $054AAC: cmpa.l #$ffffffff, a0
pub fn i_054aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AB2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $054AB2: beq.w $54abc
pub fn i_054ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AB6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54ABC; return 10; }
    8
}
/// $054AB6: jsr $28cd4.l
pub fn i_054ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ABC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54ABC);
    cpu.pc = 0x28CD4;
    16
}
/// $054ABC: jsr $28758.l
pub fn i_054abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54AC2);
    cpu.pc = 0x28758;
    16
}
/// $054AC2: bcc.w $54b0a
pub fn i_054ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54B0A; return 10; }
    8
}
/// $054AC6: move.l #$300, d0
pub fn i_054ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ACC;
    cpu.ir = 0x203C;
    let v1: u32 = 0x300;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054ACC: jsr $51a28.l
pub fn i_054acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54AD2);
    cpu.pc = 0x51A28;
    16
}
/// $054AD2: bclr.b #$0, $13(a6)
pub fn i_054ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AD8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054AD8: move.w #$1023, d0
pub fn i_054ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ADC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054ADC: jsr $2352.l
pub fn i_054adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54AE2);
    cpu.pc = 0x2352;
    16
}
/// $054AE2: lea.l $29a386.l, a1
pub fn i_054ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AE8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A386;
    20
}
/// $054AE8: jsr $77c7e.l
pub fn i_054ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54AEE);
    cpu.pc = 0x77C7E;
    16
}
/// $054AEE: lea.l $29a398.l, a1
pub fn i_054aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AF4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A398;
    20
}
/// $054AF4: jsr $77c7e.l
pub fn i_054af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54AFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54AFA);
    cpu.pc = 0x77C7E;
    16
}
/// $054AFA: lea.l $29a45e.l, a1
pub fn i_054afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B00;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A45E;
    20
}
/// $054B00: jsr $77c7e.l
pub fn i_054b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B06);
    cpu.pc = 0x77C7E;
    16
}
/// $054B06: bra.w $54b14
pub fn i_054b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B0A;
    cpu.ir = 0x6000;
    cpu.pc = 0x54B14;
    10
}
/// $054B0A: jsr $4fa70.l
pub fn i_054b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B10);
    cpu.pc = 0x4FA70;
    16
}
/// $054B10: bcc.w $54b1a
pub fn i_054b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54B1A; return 10; }
    8
}
/// $054B14: jmp $518.l
pub fn i_054b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B1A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054B1A: rts
pub fn i_054b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Small_054b1c  ($054B1C, 194 B, props/mission_053f96.s)
/// $054B1C: movea.l $3c(a6), a1
pub fn i_054b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B20;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054B20: jsr $2942a.l
pub fn i_054b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B26);
    cpu.pc = 0x2942A;
    16
}
/// $054B26: move.w #$94, d1
pub fn i_054b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B2A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x94;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054B2A: jsr $236e.l
pub fn i_054b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B30);
    cpu.pc = 0x236E;
    16
}
/// $054B30: move.w #$50, $70(a6)
pub fn i_054b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B36;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054B36: move.b #$ff, $32(a6)
pub fn i_054b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B3C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054B3C: move.b #$ff, $33(a6)
pub fn i_054b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B42;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054B42: move.b #$0, $3a(a6)
pub fn i_054b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B48;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054B48: move.w #$28, $66(a6)
pub fn i_054b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054B4E: lea.l $299050.l, a0
pub fn i_054b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x299050;
    20
}
/// $054B54: jsr $28cd4.l
pub fn i_054b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B5A);
    cpu.pc = 0x28CD4;
    16
}
/// $054B5A: lea.l $54b60(pc), a1
pub fn i_054b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54B60;
    16
}
/// $054B5E: move.l a1, (a6)
pub fn i_054b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054B60: jsr $2783a.l
pub fn i_054b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B66);
    cpu.pc = 0x2783A;
    16
}
/// $054B66: jsr $28d70.l
pub fn i_054b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B6C);
    cpu.pc = 0x28D70;
    16
}
/// $054B6C: jsr $2870a.l
pub fn i_054b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B72);
    cpu.pc = 0x2870A;
    16
}
/// $054B72: bcc.w $54b88
pub fn i_054b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54B88; return 10; }
    8
}
/// $054B76: lea.l $5e766.l, a0
pub fn i_054b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054B7C: jsr $5e770.l
pub fn i_054b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B82);
    cpu.pc = 0x5E770;
    16
}
/// $054B82: bclr.b #$3, $13(a6)
pub fn i_054b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B88;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054B88: jsr $28758.l
pub fn i_054b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54B8E);
    cpu.pc = 0x28758;
    16
}
/// $054B8E: bcc.w $54bcc
pub fn i_054b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54BCC; return 10; }
    8
}
/// $054B92: bclr.b #$0, $13(a6)
pub fn i_054b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B98;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054B98: move.w #$102e, d0
pub fn i_054b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54B9C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054B9C: jsr $2352.l
pub fn i_054b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BA2);
    cpu.pc = 0x2352;
    16
}
/// $054BA2: lea.l $29a3aa.l, a1
pub fn i_054ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BA8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A3AA;
    20
}
/// $054BA8: jsr $77c7e.l
pub fn i_054ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BAE);
    cpu.pc = 0x77C7E;
    16
}
/// $054BAE: lea.l $29a3bc.l, a1
pub fn i_054bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BB4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A3BC;
    20
}
/// $054BB4: jsr $77c7e.l
pub fn i_054bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BBA);
    cpu.pc = 0x77C7E;
    16
}
/// $054BBA: lea.l $29a470.l, a1
pub fn i_054bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BC0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A470;
    20
}
/// $054BC0: jsr $77c7e.l
pub fn i_054bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BC6);
    cpu.pc = 0x77C7E;
    16
}
/// $054BC6: lea.l $54bde(pc), a1
pub fn i_054bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54BDE;
    16
}
/// $054BCA: move.l a1, (a6)
pub fn i_054bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054BCC: jsr $4fa70.l
pub fn i_054bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BD2);
    cpu.pc = 0x4FA70;
    16
}
/// $054BD2: bcc.w $54bdc
pub fn i_054bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BD6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54BDC; return 10; }
    8
}
/// $054BD6: jmp $518.l
pub fn i_054bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BDC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054BDC: rts
pub fn i_054bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BDE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_SmallWreck_054bde  ($054BDE, 48 B, props/mission_053f96.s)
/// $054BDE: lea.l $299066.l, a0
pub fn i_054bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x299066;
    20
}
/// $054BE4: jsr $28cd4.l
pub fn i_054be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BEA);
    cpu.pc = 0x28CD4;
    16
}
/// $054BEA: lea.l $54bf0(pc), a1
pub fn i_054bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54BF0;
    16
}
/// $054BEE: move.l a1, (a6)
pub fn i_054bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054BF0: jsr $2783a.l
pub fn i_054bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BF6);
    cpu.pc = 0x2783A;
    16
}
/// $054BF6: jsr $28d70.l
pub fn i_054bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54BFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54BFC);
    cpu.pc = 0x28D70;
    16
}
/// $054BFC: jsr $4fa70.l
pub fn i_054bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C02);
    cpu.pc = 0x4FA70;
    16
}
/// $054C02: bcc.w $54c0c
pub fn i_054c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54C0C; return 10; }
    8
}
/// $054C06: jmp $518.l
pub fn i_054c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C0C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054C0C: rts
pub fn i_054c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_ScaledHP_054c0e  ($054C0E, 228 B, props/mission_053f96.s)
/// $054C0E: movea.l $3c(a6), a1
pub fn i_054c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C12;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054C12: jsr $2942a.l
pub fn i_054c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C18);
    cpu.pc = 0x2942A;
    16
}
/// $054C18: jsr $2783a.l
pub fn i_054c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C1E);
    cpu.pc = 0x2783A;
    16
}
/// $054C1E: lea.l $29b31a.l, a1
pub fn i_054c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C24;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B31A;
    20
}
/// $054C24: jsr $43fac.l
pub fn i_054c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C2A);
    cpu.pc = 0x43FAC;
    16
}
/// $054C2A: move.w #$60, $70(a6)
pub fn i_054c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C30;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054C30: lea.l $2c0196.l, a0
pub fn i_054c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C36;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0196;
    20
}
/// $054C36: jsr $799de.l
pub fn i_054c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C3C);
    cpu.pc = 0x799DE;
    16
}
/// $054C3C: move.w d0, $66(a6)
pub fn i_054c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C40;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054C40: lea.l $298a52.l, a0
pub fn i_054c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x298A52;
    20
}
/// $054C46: jsr $28cd4.l
pub fn i_054c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C4C);
    cpu.pc = 0x28CD4;
    16
}
/// $054C4C: lea.l $54c52(pc), a1
pub fn i_054c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54C52;
    16
}
/// $054C50: move.l a1, (a6)
pub fn i_054c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054C52: clr.b $10e39a.l
pub fn i_054c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C58;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $054C58: jsr $2783a.l
pub fn i_054c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C5E);
    cpu.pc = 0x2783A;
    16
}
/// $054C5E: jsr $28d70.l
pub fn i_054c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C64);
    cpu.pc = 0x28D70;
    16
}
/// $054C64: jsr $2870a.l
pub fn i_054c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C6A);
    cpu.pc = 0x2870A;
    16
}
/// $054C6A: bcc.w $54c80
pub fn i_054c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54C80; return 10; }
    8
}
/// $054C6E: lea.l $5e766.l, a0
pub fn i_054c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054C74: jsr $5e770.l
pub fn i_054c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C7A);
    cpu.pc = 0x5E770;
    16
}
/// $054C7A: bclr.b #$3, $13(a6)
pub fn i_054c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C80;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054C80: jsr $28758.l
pub fn i_054c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C86);
    cpu.pc = 0x28758;
    16
}
/// $054C86: bcc.w $54ce0
pub fn i_054c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54CE0; return 10; }
    8
}
/// $054C8A: move.l #$1000, d0
pub fn i_054c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C90;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054C90: jsr $51a28.l
pub fn i_054c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54C96);
    cpu.pc = 0x51A28;
    16
}
/// $054C96: move.w #$102b, d0
pub fn i_054c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54C9A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054C9A: jsr $2352.l
pub fn i_054c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CA0);
    cpu.pc = 0x2352;
    16
}
/// $054CA0: lea.l $29b32e.l, a1
pub fn i_054ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CA6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29B32E;
    20
}
/// $054CA6: jsr $43fac.l
pub fn i_054ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CAC);
    cpu.pc = 0x43FAC;
    16
}
/// $054CAC: lea.l $29a606.l, a2
pub fn i_054cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CB2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A606;
    20
}
/// $054CB2: jsr $5022a.l
pub fn i_054cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CB8);
    cpu.pc = 0x5022A;
    16
}
/// $054CB8: lea.l $29a308.l, a1
pub fn i_054cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CBE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A308;
    20
}
/// $054CBE: jsr $77c7e.l
pub fn i_054cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CC4);
    cpu.pc = 0x77C7E;
    16
}
/// $054CC4: lea.l $29a308.l, a1
pub fn i_054cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CCA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A308;
    20
}
/// $054CCA: jsr $77c7e.l
pub fn i_054cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CD0);
    cpu.pc = 0x77C7E;
    16
}
/// $054CD0: lea.l $29a416.l, a1
pub fn i_054cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CD6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29A416;
    20
}
/// $054CD6: jsr $77c7e.l
pub fn i_054cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CDC);
    cpu.pc = 0x77C7E;
    16
}
/// $054CDC: bra.w $54cea
pub fn i_054cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x54CEA;
    10
}
/// $054CE0: jsr $4fa70.l
pub fn i_054ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CE6);
    cpu.pc = 0x4FA70;
    16
}
/// $054CE6: bcc.w $54cf0
pub fn i_054ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54CF0; return 10; }
    8
}
/// $054CEA: jmp $518.l
pub fn i_054cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CF0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054CF0: rts
pub fn i_054cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_MultiStage_054cf2  ($054CF2, 382 B, props/mission_053f96.s)
/// $054CF2: movea.l $3c(a6), a1
pub fn i_054cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CF6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054CF6: jsr $2942a.l
pub fn i_054cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54CFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54CFC);
    cpu.pc = 0x2942A;
    16
}
/// $054CFC: move.w #$0, $72(a6)
pub fn i_054cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D02;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054D02: lea.l $2990b8.l, a0
pub fn i_054d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2990B8;
    20
}
/// $054D08: jsr $28cd4.l
pub fn i_054d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54D0E);
    cpu.pc = 0x28CD4;
    16
}
/// $054D0E: bra.w $54d8e
pub fn i_054d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D12;
    cpu.ir = 0x6000;
    cpu.pc = 0x54D8E;
    10
}
/// $054D12: movea.l $3c(a6), a1
pub fn i_054d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D16;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054D16: jsr $2942a.l
pub fn i_054d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54D1C);
    cpu.pc = 0x2942A;
    16
}
/// $054D1C: move.w #$1, $72(a6)
pub fn i_054d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D22;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054D22: lea.l $2990b8.l, a0
pub fn i_054d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2990B8;
    20
}
/// $054D28: jsr $28cd4.l
pub fn i_054d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54D2E);
    cpu.pc = 0x28CD4;
    16
}
/// $054D2E: bra.w $54d8e
pub fn i_054d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D32;
    cpu.ir = 0x6000;
    cpu.pc = 0x54D8E;
    10
}
/// $054D32: movea.l $3c(a6), a1
pub fn i_054d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D36;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054D36: jsr $2942a.l
pub fn i_054d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54D3C);
    cpu.pc = 0x2942A;
    16
}
/// $054D3C: move.w #$2, $72(a6)
pub fn i_054d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D42;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054D42: lea.l $2990b8.l, a0
pub fn i_054d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2990B8;
    20
}
/// $054D48: jsr $28cd4.l
pub fn i_054d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54D4E);
    cpu.pc = 0x28CD4;
    16
}
/// $054D4E: bra.w $54d8e
pub fn i_054d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D52;
    cpu.ir = 0x6000;
    cpu.pc = 0x54D8E;
    10
}
/// $054D52: movea.l $3c(a6), a1
pub fn i_054d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D56;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054D56: jsr $2942a.l
pub fn i_054d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54D5C);
    cpu.pc = 0x2942A;
    16
}
/// $054D5C: move.w #$3, $72(a6)
pub fn i_054d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D62;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054D62: lea.l $2990b8.l, a0
pub fn i_054d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D68;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2990B8;
    20
}
/// $054D68: jsr $28cd4.l
pub fn i_054d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54D6E);
    cpu.pc = 0x28CD4;
    16
}
/// $054D6E: bra.w $54d8e
pub fn i_054d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D72;
    cpu.ir = 0x6000;
    cpu.pc = 0x54D8E;
    10
}
/// $054D72: movea.l $3c(a6), a1
pub fn i_054d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D76;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $054D76: jsr $2942a.l
pub fn i_054d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54D7C);
    cpu.pc = 0x2942A;
    16
}
/// $054D7C: move.w #$4, $72(a6)
pub fn i_054d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D82;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054D82: lea.l $299d6e.l, a0
pub fn i_054d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D88;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x299D6E;
    20
}
/// $054D88: jsr $28cd4.l
pub fn i_054d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54D8E);
    cpu.pc = 0x28CD4;
    16
}
/// $054D8E: move.w #$95, d1
pub fn i_054d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D92;
    cpu.ir = 0x323C;
    let v1: u32 = 0x95;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054D92: jsr $236e.l
pub fn i_054d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54D98);
    cpu.pc = 0x236E;
    16
}
/// $054D98: cmpi.w #$4, $72(a6)
pub fn i_054d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54D9E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $054D9E: beq.w $54dc2
pub fn i_054d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DA2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54DC2; return 10; }
    8
}
/// $054DA2: lea.l $6293e.l, a1
pub fn i_054da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DA8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x6293E;
    20
}
/// $054DA8: jsr $4ae.l
pub fn i_054da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54DAE);
    cpu.pc = 0x4AE;
    16
}
/// $054DAE: jsr $5dd22.l
pub fn i_054dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54DB4);
    cpu.pc = 0x5DD22;
    16
}
/// $054DB4: move.w $72(a6), d0
pub fn i_054db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DB8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $054DB8: move.b d0, $62(a0)
pub fn i_054db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DBC;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x62);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054DBC: addi.w #$40, $22(a0)
pub fn i_054dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DC2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $054DC2: move.w #$80, $70(a6)
pub fn i_054dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054DC8: move.b #$ff, $32(a6)
pub fn i_054dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DCE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054DCE: move.b #$ff, $33(a6)
pub fn i_054dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DD4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054DD4: move.b #$0, $3a(a6)
pub fn i_054dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054DDA: lea.l $2c0114.l, a0
pub fn i_054dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DE0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0114;
    20
}
/// $054DE0: jsr $799de.l
pub fn i_054de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54DE6);
    cpu.pc = 0x799DE;
    16
}
/// $054DE6: move.w d0, $66(a6)
pub fn i_054de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DEA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054DEA: addi.w #$140, $66(a6)
pub fn i_054dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DF0;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x140, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $054DF0: move.b #$0, $20(a6)
pub fn i_054df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DF6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $054DF6: move.w $72(a6), d0
pub fn i_054df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54DFA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $054DFA: cmpi.w #$4, $72(a6)
pub fn i_054dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E00;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $054E00: bne.w $54e0c
pub fn i_054e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E04;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x54E0C; return 10; }
    8
}
/// $054E04: lea.l $ffff.w, a0
pub fn i_054e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E08;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $054E08: move.l a0, $48(a6)
pub fn i_054e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E0C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $054E0C: lea.l $54e12(pc), a1
pub fn i_054e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54E12;
    16
}
/// $054E10: move.l a1, (a6)
pub fn i_054e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054E12: jsr $2783a.l
pub fn i_054e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E18);
    cpu.pc = 0x2783A;
    16
}
/// $054E18: jsr $28d70.l
pub fn i_054e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E1E);
    cpu.pc = 0x28D70;
    16
}
/// $054E1E: cmpi.w #$4, $72(a6)
pub fn i_054e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E24;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $054E24: beq.w $54e5e
pub fn i_054e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54E5E; return 10; }
    8
}
/// $054E28: jsr $2870a.l
pub fn i_054e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E2E);
    cpu.pc = 0x2870A;
    16
}
/// $054E2E: bcc.w $54e44
pub fn i_054e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54E44; return 10; }
    8
}
/// $054E32: lea.l $5e766.l, a0
pub fn i_054e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E38;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054E38: jsr $5e770.l
pub fn i_054e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E3E);
    cpu.pc = 0x5E770;
    16
}
/// $054E3E: bclr.b #$3, $13(a6)
pub fn i_054e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E44;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054E44: cmpi.w #$d4, $66(a6)
pub fn i_054e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E4A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD4, Size::Word, false);
    16
}
/// $054E4A: bgt.w $54e5e
pub fn i_054e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E4E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x54E5E; return 10; }
    8
}
/// $054E4E: move.w #$1027, d0
pub fn i_054e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E52;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054E52: jsr $2352.l
pub fn i_054e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E58);
    cpu.pc = 0x2352;
    16
}
/// $054E58: lea.l $54e70(pc), a1
pub fn i_054e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54E70;
    16
}
/// $054E5C: move.l a1, (a6)
pub fn i_054e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054E5E: jsr $4fa70.l
pub fn i_054e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E64);
    cpu.pc = 0x4FA70;
    16
}
/// $054E64: bcc.w $54e6e
pub fn i_054e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E68;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54E6E; return 10; }
    8
}
/// $054E68: jmp $518.l
pub fn i_054e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E6E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054E6E: rts
pub fn i_054e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_MultiStageDamaged_054e70  ($054E70, 122 B, props/mission_053f96.s)
/// $054E70: move.w $72(a6), d0
pub fn i_054e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E74;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $054E74: movea.l #$2990ce, a0
pub fn i_054e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E7A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2990CE;
    cpu.a[0] = v1;
    4
}
/// $054E7A: lsl.w #$2, d0
pub fn i_054e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E7C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $054E7C: movea.l (a0, d0.w), a0
pub fn i_054e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E80;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $054E80: cmpa.l #$ffffffff, a0
pub fn i_054e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E86;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $054E86: beq.w $54e90
pub fn i_054e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E8A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54E90; return 10; }
    8
}
/// $054E8A: jsr $28cd4.l
pub fn i_054e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E90);
    cpu.pc = 0x28CD4;
    16
}
/// $054E90: lea.l $54e96(pc), a1
pub fn i_054e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54E96;
    16
}
/// $054E94: move.l a1, (a6)
pub fn i_054e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054E96: jsr $2783a.l
pub fn i_054e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54E9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54E9C);
    cpu.pc = 0x2783A;
    16
}
/// $054E9C: jsr $28d70.l
pub fn i_054e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54EA2);
    cpu.pc = 0x28D70;
    16
}
/// $054EA2: jsr $2870a.l
pub fn i_054ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54EA8);
    cpu.pc = 0x2870A;
    16
}
/// $054EA8: bcc.w $54ebe
pub fn i_054ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54EBE; return 10; }
    8
}
/// $054EAC: lea.l $5e766.l, a0
pub fn i_054eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EB2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054EB2: jsr $5e770.l
pub fn i_054eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54EB8);
    cpu.pc = 0x5E770;
    16
}
/// $054EB8: bclr.b #$3, $13(a6)
pub fn i_054eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EBE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054EBE: cmpi.w #$6a, $66(a6)
pub fn i_054ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EC4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6A, Size::Word, false);
    16
}
/// $054EC4: bgt.w $54ed8
pub fn i_054ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EC8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x54ED8; return 10; }
    8
}
/// $054EC8: move.w #$1027, d0
pub fn i_054ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ECC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054ECC: jsr $2352.l
pub fn i_054ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ED2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54ED2);
    cpu.pc = 0x2352;
    16
}
/// $054ED2: lea.l $54eea(pc), a1
pub fn i_054ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ED6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54EEA;
    16
}
/// $054ED6: move.l a1, (a6)
pub fn i_054ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54ED8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054ED8: jsr $4fa70.l
pub fn i_054ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54EDE);
    cpu.pc = 0x4FA70;
    16
}
/// $054EDE: bcc.w $54ee8
pub fn i_054ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54EE8; return 10; }
    8
}
/// $054EE2: jmp $518.l
pub fn i_054ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EE8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054EE8: rts
pub fn i_054ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_MultiStageCritical_054eea  ($054EEA, 128 B, props/mission_053f96.s)
/// $054EEA: move.w $72(a6), d0
pub fn i_054eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EEE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $054EEE: movea.l #$2990de, a0
pub fn i_054eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EF4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2990DE;
    cpu.a[0] = v1;
    4
}
/// $054EF4: lsl.w #$2, d0
pub fn i_054ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EF6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $054EF6: movea.l (a0, d0.w), a0
pub fn i_054ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54EFA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $054EFA: cmpa.l #$ffffffff, a0
pub fn i_054efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F00;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $054F00: beq.w $54f0a
pub fn i_054f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54F0A; return 10; }
    8
}
/// $054F04: jsr $28cd4.l
pub fn i_054f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F0A);
    cpu.pc = 0x28CD4;
    16
}
/// $054F0A: lea.l $54f10(pc), a1
pub fn i_054f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54F10;
    16
}
/// $054F0E: move.l a1, (a6)
pub fn i_054f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054F10: jsr $2783a.l
pub fn i_054f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F16);
    cpu.pc = 0x2783A;
    16
}
/// $054F16: jsr $28d70.l
pub fn i_054f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F1C);
    cpu.pc = 0x28D70;
    16
}
/// $054F1C: jsr $2870a.l
pub fn i_054f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F22);
    cpu.pc = 0x2870A;
    16
}
/// $054F22: bcc.w $54f38
pub fn i_054f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54F38; return 10; }
    8
}
/// $054F26: lea.l $5e766.l, a0
pub fn i_054f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F2C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $054F2C: jsr $5e770.l
pub fn i_054f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F32);
    cpu.pc = 0x5E770;
    16
}
/// $054F32: bclr.b #$3, $13(a6)
pub fn i_054f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F38;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054F38: jsr $28758.l
pub fn i_054f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F3E);
    cpu.pc = 0x28758;
    16
}
/// $054F3E: bcc.w $54f58
pub fn i_054f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54F58; return 10; }
    8
}
/// $054F42: bclr.b #$0, $13(a6)
pub fn i_054f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F48;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $054F48: move.w #$1035, d0
pub fn i_054f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F4C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054F4C: jsr $2352.l
pub fn i_054f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F52);
    cpu.pc = 0x2352;
    16
}
/// $054F52: lea.l $54f6a(pc), a1
pub fn i_054f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54F6A;
    16
}
/// $054F56: move.l a1, (a6)
pub fn i_054f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054F58: jsr $4fa70.l
pub fn i_054f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F5E);
    cpu.pc = 0x4FA70;
    16
}
/// $054F5E: bcc.w $54f68
pub fn i_054f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F62;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54F68; return 10; }
    8
}
/// $054F62: jmp $518.l
pub fn i_054f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F68;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054F68: rts
pub fn i_054f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_MultiStageWreck_054f6a  ($054F6A, 80 B, props/mission_053f96.s)
/// $054F6A: move.l #$4000, d0
pub fn i_054f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F70;
    cpu.ir = 0x203C;
    let v1: u32 = 0x4000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054F70: jsr $51a28.l
pub fn i_054f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F76);
    cpu.pc = 0x51A28;
    16
}
/// $054F76: move.w $72(a6), d0
pub fn i_054f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F7A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $054F7A: movea.l #$2990ee, a0
pub fn i_054f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F80;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2990EE;
    cpu.a[0] = v1;
    4
}
/// $054F80: lsl.w #$2, d0
pub fn i_054f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F82;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $054F82: movea.l (a0, d0.w), a0
pub fn i_054f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F86;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $054F86: cmpa.l #$ffffffff, a0
pub fn i_054f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F8C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $054F8C: beq.w $54f96
pub fn i_054f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x54F96; return 10; }
    8
}
/// $054F90: jsr $28cd4.l
pub fn i_054f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54F96);
    cpu.pc = 0x28CD4;
    16
}
/// $054F96: lea.l $54f9c(pc), a1
pub fn i_054f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54F9C;
    16
}
/// $054F9A: move.l a1, (a6)
pub fn i_054f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54F9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054F9C: jsr $2783a.l
pub fn i_054f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54FA2);
    cpu.pc = 0x2783A;
    16
}
/// $054FA2: jsr $28d70.l
pub fn i_054fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54FA8);
    cpu.pc = 0x28D70;
    16
}
/// $054FA8: jsr $4fa70.l
pub fn i_054fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54FAE);
    cpu.pc = 0x4FA70;
    16
}
/// $054FAE: bcc.w $54fb8
pub fn i_054fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x54FB8; return 10; }
    8
}
/// $054FB2: jmp $518.l
pub fn i_054fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FB8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $054FB8: rts
pub fn i_054fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlink2_PhaseA_054fba  ($054FBA, 72 B, props/mission_053f96.s)
/// $054FBA: move.w #$48, d1
pub fn i_054fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FBE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054FBE: move.w #$79, d2
pub fn i_054fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FC2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x79;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054FC2: move.w #$2, d3
pub fn i_054fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FC6;
    cpu.ir = 0x363C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054FC6: move.w #$1, d4
pub fn i_054fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FCA;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $054FCA: jsr $2c26.l
pub fn i_054fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x54FD0);
    cpu.pc = 0x2C26;
    16
}
/// $054FD0: move.w #$40, $72(a6)
pub fn i_054fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FD6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $054FD6: lea.l $54fdc(pc), a1
pub fn i_054fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54FDC;
    16
}
/// $054FDA: move.l a1, (a6)
pub fn i_054fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054FDC: subq.w #$1, $72(a6)
pub fn i_054fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FE0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $054FE0: bne.w $54fea
pub fn i_054fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x54FEA; return 10; }
    8
}
/// $054FE4: lea.l $55002(pc), a1
pub fn i_054fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x55002;
    16
}
/// $054FE8: move.l a1, (a6)
pub fn i_054fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $054FEA: move.l $106f5c.l, d0
pub fn i_054fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FF0;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $054FF0: swap d0
pub fn i_054ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FF2;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $054FF2: cmpi.w #$300, d0
pub fn i_054ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FF6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x300, Size::Word, false);
    8
}
/// $054FF6: blt.w $55000
pub fn i_054ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x54FFA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x55000; return 10; }
    8
}
/// $054FFA: jmp $518.l
pub fn i_054ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55000;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $055000: rts
pub fn i_055000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55002;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixBlink2_PhaseB_055002  ($055002, 72 B, props/mission_053f96.s)
/// $055002: move.w #$48, d1
pub fn i_055002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55006;
    cpu.ir = 0x323C;
    let v1: u32 = 0x48;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055006: move.w #$78, d2
pub fn i_055006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5500A;
    cpu.ir = 0x343C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05500A: move.w #$1, d3
pub fn i_05500a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5500E;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05500E: move.w #$1, d4
pub fn i_05500e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55012;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055012: jsr $2c26.l
pub fn i_055012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55018;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55018);
    cpu.pc = 0x2C26;
    16
}
/// $055018: move.w #$20, $72(a6)
pub fn i_055018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5501E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05501E: lea.l $55024(pc), a1
pub fn i_05501e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55022;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x55024;
    16
}
/// $055022: move.l a1, (a6)
pub fn i_055022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55024;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $055024: subq.w #$1, $72(a6)
pub fn i_055024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55028;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $055028: bne.w $55032
pub fn i_055028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5502C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55032; return 10; }
    8
}
/// $05502C: lea.l $54fba(pc), a1
pub fn i_05502c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55030;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x54FBA;
    16
}
/// $055030: move.l a1, (a6)
pub fn i_055030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55032;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $055032: move.l $106f5c.l, d0
pub fn i_055032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55038;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $055038: swap d0
pub fn i_055038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5503A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05503A: cmpi.w #$300, d0
pub fn i_05503a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5503E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x300, Size::Word, false);
    8
}
/// $05503E: blt.w $55048
pub fn i_05503e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55042;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x55048; return 10; }
    8
}
/// $055042: jmp $518.l
pub fn i_055042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55048;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $055048: rts
pub fn i_055048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5504A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Prop_Blocker_05504a  ($05504A, 82 B, props/mission_053f96.s)
/// $05504A: movea.l $3c(a6), a1
pub fn i_05504a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5504E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05504E: jsr $2942a.l
pub fn i_05504e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55054;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55054);
    cpu.pc = 0x2942A;
    16
}
/// $055054: move.w #$e, d1
pub fn i_055054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55058;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055058: jsr $236e.l
pub fn i_055058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5505E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5505E);
    cpu.pc = 0x236E;
    16
}
/// $05505E: bset.b #$6, $12(a6)
pub fn i_05505e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55064;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $055064: move.w #$f000, $38(a6)
pub fn i_055064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5506A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05506A: move.w #$60, $70(a6)
pub fn i_05506a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55070;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $055070: move.l #$29b3c0, $60(a6)
pub fn i_055070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55078;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x29B3C0;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $055078: lea.l $5507e(pc), a1
pub fn i_055078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5507C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5507E;
    16
}
/// $05507C: move.l a1, (a6)
pub fn i_05507c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5507E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05507E: jsr $28998.l
pub fn i_05507e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55084;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55084);
    cpu.pc = 0x28998;
    16
}
/// $055084: jsr $2783a.l
pub fn i_055084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5508A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5508A);
    cpu.pc = 0x2783A;
    16
}
/// $05508A: jsr $4fa70.l
pub fn i_05508a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55090;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55090);
    cpu.pc = 0x4FA70;
    16
}
/// $055090: bcc.w $5509a
pub fn i_055090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55094;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5509A; return 10; }
    8
}
/// $055094: jmp $518.l
pub fn i_055094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5509A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05509A: rts
pub fn i_05509a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5509C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_FixUpdate_05509c  ($05509C, 34 B, props/mission_053f96.s)
/// $05509C: cmpi.b #$0, $74(a6)
pub fn i_05509c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550A2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0550A2: bne.w $55146
pub fn i_0550a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55146; return 10; }
    8
}
/// $0550A6: move.b $1081b1.l, d0
pub fn i_0550a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550AC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x1081B1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0550AC: cmpi.b #$0, d0
pub fn i_0550ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550B0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0550B0: beq.w $550c4
pub fn i_0550b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x550C4; return 10; }
    8
}
/// $0550B4: jsr $551d0(pc)
pub fn i_0550b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x550B8);
    cpu.pc = 0x551D0;
    16
}
/// $0550B8: move.b $1081b1.l, d0
pub fn i_0550b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550BE;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x1081B1) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
// ===== SetTaskB_0550be  ($0550BE, 6 B, kernel/task_setters_b.c)
/// $0550BE: move.b d0, $74(a6)
pub fn i_0550be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550C2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0550C2: rts
pub fn i_0550c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_FixByParentPhase_0550c4  ($0550C4, 68 B, props/mission_053f96.s)
/// $0550C4: movea.l $c(a6), a0
pub fn i_0550c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550C8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0550C8: cmpi.b #$7f, $80(a0)
pub fn i_0550c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550CE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7F, Size::Byte, false);
    16
}
/// $0550CE: beq.w $550e6
pub fn i_0550ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550D2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x550E6; return 10; }
    8
}
/// $0550D2: cmpi.b #$ff, $80(a0)
pub fn i_0550d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550D8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0550D8: bne.w $550e8
pub fn i_0550d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x550E8; return 10; }
    8
}
/// $0550DC: jsr $551d0(pc)
pub fn i_0550dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550E0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x550E0);
    cpu.pc = 0x551D0;
    16
}
/// $0550E0: move.b #$7f, $80(a0)
pub fn i_0550e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550E6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0550E6: rts
pub fn i_0550e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0550E8: movea.l $c(a6), a0
pub fn i_0550e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550EC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0550EC: cmpi.b #$ff, $21(a0)
pub fn i_0550ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550F2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0550F2: beq.w $55146
pub fn i_0550f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x55146; return 10; }
    8
}
/// $0550F6: addq.w #$1, $72(a6)
pub fn i_0550f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550FA;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0550FA: movea.l $c(a6), a0
pub fn i_0550fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x550FE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0550FE: cmpi.b #$1, $21(a0)
pub fn i_0550fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55104;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $055104: bne.w $5510e
pub fn i_055104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55108;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5510E; return 10; }
    8
}
// ===== JsrPcThunk_055108  ($055108, 6 B, thunks/jsr_pc_thunks.c)
/// $055108: jsr $55214(pc)
pub fn i_055108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5510C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5510C);
    cpu.pc = 0x55214;
    16
}
/// $05510C: rts
pub fn i_05510c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5510E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_FixPhase2_05510e  ($05510E, 14 B, props/mission_053f96.s)
/// $05510E: movea.l $c(a6), a0
pub fn i_05510e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55112;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $055112: cmpi.b #$2, $21(a0)
pub fn i_055112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55118;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $055118: bne.w $55122
pub fn i_055118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5511C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55122; return 10; }
    8
}
// ===== JsrPcThunk_05511c  ($05511C, 6 B, thunks/jsr_pc_thunks.c)
/// $05511C: jsr $55148(pc)
pub fn i_05511c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55120;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x55120);
    cpu.pc = 0x55148;
    16
}
/// $055120: rts
pub fn i_055120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55122;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_FixBlink_055122  ($055122, 32 B, props/mission_053f96.s)
/// $055122: move.w $72(a6), d0
pub fn i_055122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55126;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055126: andi.w #$3, d0
pub fn i_055126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5512A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05512A: bne.w $55146
pub fn i_05512a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5512E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55146; return 10; }
    8
}
/// $05512E: move.w $72(a6), d0
pub fn i_05512e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55132;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055132: btst.b #$2, d0
pub fn i_055132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55136;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $055136: bne.w $55142
pub fn i_055136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5513A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x55142; return 10; }
    8
}
/// $05513A: jsr $5518c(pc)
pub fn i_05513a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5513E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5513E);
    cpu.pc = 0x5518C;
    16
}
/// $05513E: bra.w $55146
pub fn i_05513e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55142;
    cpu.ir = 0x6000;
    cpu.pc = 0x55146;
    10
}
// ===== JsrPcThunk_055142  ($055142, 6 B, thunks/jsr_pc_thunks.c)
/// $055142: jsr $55148(pc)
pub fn i_055142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55146;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x55146);
    cpu.pc = 0x55148;
    16
}
/// $055146: rts
pub fn i_055146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55148;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_TilesOffA_055148  ($055148, 60 B, props/mission_053f96.s)
/// $055148: move.w #$1a, d1
pub fn i_055148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5514C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05514C: move.w #$4b, d2
pub fn i_05514c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55150;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055150: move.w #$ffff, d3
pub fn i_055150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55154;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055154: move.w #$1, d4
pub fn i_055154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55158;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055158: jsr $2c26.l
pub fn i_055158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5515E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5515E);
    cpu.pc = 0x2C26;
    16
}
/// $05515E: move.w #$1d, d1
pub fn i_05515e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55162;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055162: move.w #$4e, d2
pub fn i_055162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55166;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055166: move.w #$ffff, d3
pub fn i_055166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5516A;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05516A: move.w #$1, d4
pub fn i_05516a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5516E;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05516E: jsr $2c26.l
pub fn i_05516e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55174);
    cpu.pc = 0x2C26;
    16
}
/// $055174: move.w $14(a6), d1
pub fn i_055174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55178;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055178: move.w #$8e, d2
pub fn i_055178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5517C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x8E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05517C: move.w #$ffff, d3
pub fn i_05517c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55180;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055180: move.w #$1, d4
pub fn i_055180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55184;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_055184  ($055184, 8 B, thunks/jsr_abs_thunks.c)
/// $055184: jsr $2c30.l
pub fn i_055184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5518A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5518A);
    cpu.pc = 0x2C30;
    16
}
/// $05518A: rts
pub fn i_05518a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5518C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_TilesOnA_05518c  ($05518C, 60 B, props/mission_053f96.s)
/// $05518C: move.w #$1a, d1
pub fn i_05518c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55190;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055190: move.w #$4a, d2
pub fn i_055190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55194;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055194: move.w #$ffff, d3
pub fn i_055194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55198;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055198: move.w #$1, d4
pub fn i_055198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5519C;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05519C: jsr $2c26.l
pub fn i_05519c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x551A2);
    cpu.pc = 0x2C26;
    16
}
/// $0551A2: move.w #$1d, d1
pub fn i_0551a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551A6: move.w #$4d, d2
pub fn i_0551a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551AA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551AA: move.w #$ffff, d3
pub fn i_0551aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551AE;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551AE: move.w #$1, d4
pub fn i_0551ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551B2;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551B2: jsr $2c26.l
pub fn i_0551b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x551B8);
    cpu.pc = 0x2C26;
    16
}
/// $0551B8: move.w $14(a6), d1
pub fn i_0551b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551BC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0551BC: move.w #$8d, d2
pub fn i_0551bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551C0;
    cpu.ir = 0x343C;
    let v1: u32 = 0x8D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551C0: move.w #$ffff, d3
pub fn i_0551c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551C4;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551C4: move.w #$1, d4
pub fn i_0551c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551C8;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_0551c8  ($0551C8, 8 B, thunks/jsr_abs_thunks.c)
/// $0551C8: jsr $2c30.l
pub fn i_0551c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x551CE);
    cpu.pc = 0x2C30;
    16
}
/// $0551CE: rts
pub fn i_0551ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_TilesDark_0551d0  ($0551D0, 60 B, props/mission_053f96.s)
/// $0551D0: move.w #$1a, d1
pub fn i_0551d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551D4: move.w #$bd, d2
pub fn i_0551d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551D8;
    cpu.ir = 0x343C;
    let v1: u32 = 0xBD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551D8: move.w #$1, d3
pub fn i_0551d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551DC;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551DC: move.w #$2, d4
pub fn i_0551dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551E0;
    cpu.ir = 0x383C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551E0: jsr $2c26.l
pub fn i_0551e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x551E6);
    cpu.pc = 0x2C26;
    16
}
/// $0551E6: move.w #$1d, d1
pub fn i_0551e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551EA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551EA: move.w #$be, d2
pub fn i_0551ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551EE;
    cpu.ir = 0x343C;
    let v1: u32 = 0xBE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551EE: move.w #$1, d3
pub fn i_0551ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551F2;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551F2: move.w #$2, d4
pub fn i_0551f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551F6;
    cpu.ir = 0x383C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0551F6: jsr $2c26.l
pub fn i_0551f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x551FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x551FC);
    cpu.pc = 0x2C26;
    16
}
/// $0551FC: move.w $14(a6), d1
pub fn i_0551fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55200;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055200: move.w #$123, d2
pub fn i_055200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55204;
    cpu.ir = 0x343C;
    let v1: u32 = 0x123;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055204: move.w #$1, d3
pub fn i_055204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55208;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055208: move.w #$2, d4
pub fn i_055208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5520C;
    cpu.ir = 0x383C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_05520c  ($05520C, 8 B, thunks/jsr_abs_thunks.c)
/// $05520C: jsr $2c30.l
pub fn i_05520c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55212;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55212);
    cpu.pc = 0x2C30;
    16
}
/// $055212: rts
pub fn i_055212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55214;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== NeonSign_TilesOnB_055214  ($055214, 60 B, props/mission_053f96.s)
/// $055214: move.w #$1a, d1
pub fn i_055214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55218;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055218: move.w #$4c, d2
pub fn i_055218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5521C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05521C: move.w #$1, d3
pub fn i_05521c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55220;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055220: move.w #$1, d4
pub fn i_055220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55224;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055224: jsr $2c26.l
pub fn i_055224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5522A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5522A);
    cpu.pc = 0x2C26;
    16
}
/// $05522A: move.w #$1d, d1
pub fn i_05522a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5522E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05522E: move.w #$4f, d2
pub fn i_05522e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55232;
    cpu.ir = 0x343C;
    let v1: u32 = 0x4F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055232: move.w #$1, d3
pub fn i_055232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55236;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055236: move.w #$1, d4
pub fn i_055236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5523A;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05523A: jsr $2c26.l
pub fn i_05523a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55240);
    cpu.pc = 0x2C26;
    16
}
/// $055240: move.w $14(a6), d1
pub fn i_055240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55244;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $055244: move.w #$8f, d2
pub fn i_055244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55248;
    cpu.ir = 0x343C;
    let v1: u32 = 0x8F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $055248: move.w #$1, d3
pub fn i_055248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5524C;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05524C: move.w #$1, d4
pub fn i_05524c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55250;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_055250  ($055250, 8 B, thunks/jsr_abs_thunks.c)
/// $055250: jsr $2c30.l
pub fn i_055250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55256;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55256);
    cpu.pc = 0x2C30;
    16
}
/// $055256: rts
pub fn i_055256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55258;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055258  ($055258, 14 B, kernel/state_dispatch_stubs.c)
/// $055258: lea.l $29a552.l, a2
pub fn i_055258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5525E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A552;
    20
}
/// $05525E: jsr $5022a.l
pub fn i_05525e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55264;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55264);
    cpu.pc = 0x5022A;
    16
}
/// $055264: rts
pub fn i_055264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55266;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055266  ($055266, 14 B, kernel/state_dispatch_stubs.c)
/// $055266: lea.l $29a566.l, a2
pub fn i_055266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5526C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A566;
    20
}
/// $05526C: jsr $5022a.l
pub fn i_05526c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55272;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55272);
    cpu.pc = 0x5022A;
    16
}
/// $055272: rts
pub fn i_055272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55274;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055274  ($055274, 14 B, kernel/state_dispatch_stubs.c)
/// $055274: lea.l $29a57a.l, a2
pub fn i_055274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5527A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A57A;
    20
}
/// $05527A: jsr $5022a.l
pub fn i_05527a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55280;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x55280);
    cpu.pc = 0x5022A;
    16
}
/// $055280: rts
pub fn i_055280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55282;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055282  ($055282, 14 B, kernel/state_dispatch_stubs.c)
/// $055282: lea.l $29a58e.l, a2
pub fn i_055282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55288;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A58E;
    20
}
/// $055288: jsr $5022a.l
pub fn i_055288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5528E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5528E);
    cpu.pc = 0x5022A;
    16
}
/// $05528E: rts
pub fn i_05528e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55290;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_055290  ($055290, 14 B, kernel/state_dispatch_stubs.c)
/// $055290: lea.l $29a5a2.l, a2
pub fn i_055290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x55296;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A5A2;
    20
}
/// $055296: jsr $5022a.l
pub fn i_055296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5529C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5529C);
    cpu.pc = 0x5022A;
    16
}
/// $05529C: rts
pub fn i_05529c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5529E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_05529e  ($05529E, 14 B, kernel/state_dispatch_stubs.c)
/// $05529E: lea.l $29a5b6.l, a2
pub fn i_05529e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552A4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A5B6;
    20
}
/// $0552A4: jsr $5022a.l
pub fn i_0552a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552AA);
    cpu.pc = 0x5022A;
    16
}
/// $0552AA: rts
pub fn i_0552aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0552ac  ($0552AC, 14 B, kernel/state_dispatch_stubs.c)
/// $0552AC: lea.l $29a5ca.l, a2
pub fn i_0552ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552B2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A5CA;
    20
}
/// $0552B2: jsr $5022a.l
pub fn i_0552b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552B8);
    cpu.pc = 0x5022A;
    16
}
/// $0552B8: rts
pub fn i_0552b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0552ba  ($0552BA, 14 B, kernel/state_dispatch_stubs.c)
/// $0552BA: lea.l $29a5de.l, a2
pub fn i_0552ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552C0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A5DE;
    20
}
/// $0552C0: jsr $5022a.l
pub fn i_0552c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552C6);
    cpu.pc = 0x5022A;
    16
}
/// $0552C6: rts
pub fn i_0552c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0552c8  ($0552C8, 14 B, kernel/state_dispatch_stubs.c)
/// $0552C8: lea.l $29a5f2.l, a2
pub fn i_0552c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552CE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A5F2;
    20
}
/// $0552CE: jsr $5022a.l
pub fn i_0552ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552D4);
    cpu.pc = 0x5022A;
    16
}
/// $0552D4: rts
pub fn i_0552d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_0552d6  ($0552D6, 14 B, kernel/state_dispatch_stubs.c)
/// $0552D6: lea.l $29a61a.l, a2
pub fn i_0552d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552DC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x29A61A;
    20
}
/// $0552DC: jsr $5022a.l
pub fn i_0552dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x552E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x552E2);
    cpu.pc = 0x5022A;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x053646, i_053646),
    (0x05364A, i_05364a),
    (0x053650, i_053650),
    (0x053652, i_053652),
    (0x053658, i_053658),
    (0x05365E, i_05365e),
    (0x053664, i_053664),
    (0x053668, i_053668),
    (0x05366C, i_05366c),
    (0x053670, i_053670),
    (0x053674, i_053674),
    (0x053678, i_053678),
    (0x05367C, i_05367c),
    (0x053682, i_053682),
    (0x053688, i_053688),
    (0x05368C, i_05368c),
    (0x05368E, i_05368e),
    (0x053694, i_053694),
    (0x05369A, i_05369a),
    (0x0536A0, i_0536a0),
    (0x0536A4, i_0536a4),
    (0x0536AA, i_0536aa),
    (0x0536AC, i_0536ac),
    (0x0536B0, i_0536b0),
    (0x0536B4, i_0536b4),
    (0x0536B8, i_0536b8),
    (0x0536BC, i_0536bc),
    (0x0536C2, i_0536c2),
    (0x0536C6, i_0536c6),
    (0x0536CA, i_0536ca),
    (0x0536CE, i_0536ce),
    (0x0536D2, i_0536d2),
    (0x0536D8, i_0536d8),
    (0x0536DE, i_0536de),
    (0x0536E2, i_0536e2),
    (0x0536E4, i_0536e4),
    (0x0536E8, i_0536e8),
    (0x0536EC, i_0536ec),
    (0x0536F0, i_0536f0),
    (0x0536F2, i_0536f2),
    (0x0536F8, i_0536f8),
    (0x0536FA, i_0536fa),
    (0x0536FE, i_0536fe),
    (0x053702, i_053702),
    (0x053708, i_053708),
    (0x05370A, i_05370a),
    (0x05370E, i_05370e),
    (0x053712, i_053712),
    (0x053716, i_053716),
    (0x05371A, i_05371a),
    (0x053720, i_053720),
    (0x053724, i_053724),
    (0x053728, i_053728),
    (0x05372C, i_05372c),
    (0x053730, i_053730),
    (0x053736, i_053736),
    (0x05373C, i_05373c),
    (0x053740, i_053740),
    (0x053742, i_053742),
    (0x053746, i_053746),
    (0x05374A, i_05374a),
    (0x05374E, i_05374e),
    (0x053750, i_053750),
    (0x053756, i_053756),
    (0x053758, i_053758),
    (0x05375C, i_05375c),
    (0x053760, i_053760),
    (0x053766, i_053766),
    (0x053768, i_053768),
    (0x05376C, i_05376c),
    (0x053772, i_053772),
    (0x053776, i_053776),
    (0x05377A, i_05377a),
    (0x053780, i_053780),
    (0x053786, i_053786),
    (0x05378A, i_05378a),
    (0x053790, i_053790),
    (0x053792, i_053792),
    (0x053796, i_053796),
    (0x05379C, i_05379c),
    (0x0537A0, i_0537a0),
    (0x0537A6, i_0537a6),
    (0x0537AA, i_0537aa),
    (0x0537AE, i_0537ae),
    (0x0537B4, i_0537b4),
    (0x0537BA, i_0537ba),
    (0x0537BE, i_0537be),
    (0x0537C4, i_0537c4),
    (0x0537C8, i_0537c8),
    (0x0537CE, i_0537ce),
    (0x0537D2, i_0537d2),
    (0x0537D8, i_0537d8),
    (0x0537DC, i_0537dc),
    (0x0537E0, i_0537e0),
    (0x0537E2, i_0537e2),
    (0x0537E6, i_0537e6),
    (0x0537EA, i_0537ea),
    (0x0537EE, i_0537ee),
    (0x0537F4, i_0537f4),
    (0x0537F8, i_0537f8),
    (0x0537FE, i_0537fe),
    (0x053804, i_053804),
    (0x05380A, i_05380a),
    (0x053810, i_053810),
    (0x053814, i_053814),
    (0x05381A, i_05381a),
    (0x05381E, i_05381e),
    (0x053822, i_053822),
    (0x053824, i_053824),
    (0x053828, i_053828),
    (0x05382E, i_05382e),
    (0x053834, i_053834),
    (0x05383A, i_05383a),
    (0x05383E, i_05383e),
    (0x053840, i_053840),
    (0x053846, i_053846),
    (0x05384C, i_05384c),
    (0x053850, i_053850),
    (0x053854, i_053854),
    (0x05385A, i_05385a),
    (0x053860, i_053860),
    (0x053866, i_053866),
    (0x05386C, i_05386c),
    (0x053872, i_053872),
    (0x053876, i_053876),
    (0x05387A, i_05387a),
    (0x05387E, i_05387e),
    (0x053884, i_053884),
    (0x053886, i_053886),
    (0x05388A, i_05388a),
    (0x053890, i_053890),
    (0x053894, i_053894),
    (0x053898, i_053898),
    (0x05389E, i_05389e),
    (0x0538A4, i_0538a4),
    (0x0538A8, i_0538a8),
    (0x0538AE, i_0538ae),
    (0x0538B4, i_0538b4),
    (0x0538BA, i_0538ba),
    (0x0538BE, i_0538be),
    (0x0538C4, i_0538c4),
    (0x0538C8, i_0538c8),
    (0x0538CC, i_0538cc),
    (0x0538CE, i_0538ce),
    (0x0538D2, i_0538d2),
    (0x0538D8, i_0538d8),
    (0x0538DC, i_0538dc),
    (0x0538E0, i_0538e0),
    (0x0538E4, i_0538e4),
    (0x0538E8, i_0538e8),
    (0x0538EA, i_0538ea),
    (0x0538EC, i_0538ec),
    (0x0538F0, i_0538f0),
    (0x0538F4, i_0538f4),
    (0x0538F8, i_0538f8),
    (0x0538FE, i_0538fe),
    (0x053902, i_053902),
    (0x053906, i_053906),
    (0x05390A, i_05390a),
    (0x05390E, i_05390e),
    (0x053912, i_053912),
    (0x053914, i_053914),
    (0x05391A, i_05391a),
    (0x05391E, i_05391e),
    (0x053922, i_053922),
    (0x053924, i_053924),
    (0x053926, i_053926),
    (0x05392A, i_05392a),
    (0x053930, i_053930),
    (0x053936, i_053936),
    (0x05393C, i_05393c),
    (0x053942, i_053942),
    (0x053946, i_053946),
    (0x05394C, i_05394c),
    (0x053952, i_053952),
    (0x053958, i_053958),
    (0x05395C, i_05395c),
    (0x053962, i_053962),
    (0x053964, i_053964),
    (0x053968, i_053968),
    (0x05396E, i_05396e),
    (0x053972, i_053972),
    (0x053978, i_053978),
    (0x05397E, i_05397e),
    (0x053984, i_053984),
    (0x05398A, i_05398a),
    (0x053990, i_053990),
    (0x053996, i_053996),
    (0x05399A, i_05399a),
    (0x0539A0, i_0539a0),
    (0x0539A6, i_0539a6),
    (0x0539AC, i_0539ac),
    (0x0539B0, i_0539b0),
    (0x0539B2, i_0539b2),
    (0x0539B8, i_0539b8),
    (0x0539BE, i_0539be),
    (0x0539C4, i_0539c4),
    (0x0539CA, i_0539ca),
    (0x0539CE, i_0539ce),
    (0x0539D2, i_0539d2),
    (0x0539D8, i_0539d8),
    (0x0539DE, i_0539de),
    (0x0539E2, i_0539e2),
    (0x0539E8, i_0539e8),
    (0x0539EE, i_0539ee),
    (0x0539F0, i_0539f0),
    (0x0539F4, i_0539f4),
    (0x0539FA, i_0539fa),
    (0x053A00, i_053a00),
    (0x053A06, i_053a06),
    (0x053A0C, i_053a0c),
    (0x053A12, i_053a12),
    (0x053A16, i_053a16),
    (0x053A18, i_053a18),
    (0x053A1E, i_053a1e),
    (0x053A24, i_053a24),
    (0x053A28, i_053a28),
    (0x053A2E, i_053a2e),
    (0x053A32, i_053a32),
    (0x053A38, i_053a38),
    (0x053A3C, i_053a3c),
    (0x053A40, i_053a40),
    (0x053A42, i_053a42),
    (0x053A46, i_053a46),
    (0x053A4C, i_053a4c),
    (0x053A52, i_053a52),
    (0x053A58, i_053a58),
    (0x053A5E, i_053a5e),
    (0x053A64, i_053a64),
    (0x053A6A, i_053a6a),
    (0x053A70, i_053a70),
    (0x053A76, i_053a76),
    (0x053A7A, i_053a7a),
    (0x053A80, i_053a80),
    (0x053A84, i_053a84),
    (0x053A86, i_053a86),
    (0x053A8C, i_053a8c),
    (0x053A92, i_053a92),
    (0x053A98, i_053a98),
    (0x053A9C, i_053a9c),
    (0x053AA0, i_053aa0),
    (0x053AA4, i_053aa4),
    (0x053AAA, i_053aaa),
    (0x053AB0, i_053ab0),
    (0x053AB4, i_053ab4),
    (0x053AB8, i_053ab8),
    (0x053ABC, i_053abc),
    (0x053AC2, i_053ac2),
    (0x053AC6, i_053ac6),
    (0x053ACC, i_053acc),
    (0x053AD2, i_053ad2),
    (0x053AD8, i_053ad8),
    (0x053ADE, i_053ade),
    (0x053AE2, i_053ae2),
    (0x053AE8, i_053ae8),
    (0x053AF0, i_053af0),
    (0x053AF4, i_053af4),
    (0x053AFA, i_053afa),
    (0x053AFE, i_053afe),
    (0x053B04, i_053b04),
    (0x053B0A, i_053b0a),
    (0x053B10, i_053b10),
    (0x053B16, i_053b16),
    (0x053B1A, i_053b1a),
    (0x053B20, i_053b20),
    (0x053B24, i_053b24),
    (0x053B28, i_053b28),
    (0x053B2E, i_053b2e),
    (0x053B32, i_053b32),
    (0x053B38, i_053b38),
    (0x053B3E, i_053b3e),
    (0x053B44, i_053b44),
    (0x053B4A, i_053b4a),
    (0x053B4E, i_053b4e),
    (0x053B54, i_053b54),
    (0x053B58, i_053b58),
    (0x053B5E, i_053b5e),
    (0x053B62, i_053b62),
    (0x053B68, i_053b68),
    (0x053B6E, i_053b6e),
    (0x053B74, i_053b74),
    (0x053B7A, i_053b7a),
    (0x053B80, i_053b80),
    (0x053B86, i_053b86),
    (0x053B8C, i_053b8c),
    (0x053B90, i_053b90),
    (0x053B96, i_053b96),
    (0x053B9C, i_053b9c),
    (0x053B9E, i_053b9e),
    (0x053BA2, i_053ba2),
    (0x053BA8, i_053ba8),
    (0x053BAE, i_053bae),
    (0x053BB4, i_053bb4),
    (0x053BBA, i_053bba),
    (0x053BC0, i_053bc0),
    (0x053BC6, i_053bc6),
    (0x053BCC, i_053bcc),
    (0x053BD0, i_053bd0),
    (0x053BD2, i_053bd2),
    (0x053BD6, i_053bd6),
    (0x053BDC, i_053bdc),
    (0x053BE2, i_053be2),
    (0x053BE8, i_053be8),
    (0x053BEC, i_053bec),
    (0x053BF2, i_053bf2),
    (0x053BF6, i_053bf6),
    (0x053BFC, i_053bfc),
    (0x053BFE, i_053bfe),
    (0x053C02, i_053c02),
    (0x053C08, i_053c08),
    (0x053C0E, i_053c0e),
    (0x053C14, i_053c14),
    (0x053C1A, i_053c1a),
    (0x053C20, i_053c20),
    (0x053C26, i_053c26),
    (0x053C2A, i_053c2a),
    (0x053C2C, i_053c2c),
    (0x053C32, i_053c32),
    (0x053C38, i_053c38),
    (0x053C3C, i_053c3c),
    (0x053C42, i_053c42),
    (0x053C44, i_053c44),
    (0x053C48, i_053c48),
    (0x053C4E, i_053c4e),
    (0x053C54, i_053c54),
    (0x053C58, i_053c58),
    (0x053C5A, i_053c5a),
    (0x053C5C, i_053c5c),
    (0x053C60, i_053c60),
    (0x053C62, i_053c62),
    (0x053C64, i_053c64),
    (0x053C68, i_053c68),
    (0x053C6A, i_053c6a),
    (0x053C70, i_053c70),
    (0x053C74, i_053c74),
    (0x053C7A, i_053c7a),
    (0x053C7C, i_053c7c),
    (0x053C80, i_053c80),
    (0x053C84, i_053c84),
    (0x053C88, i_053c88),
    (0x053C8C, i_053c8c),
    (0x053C8E, i_053c8e),
    (0x053C92, i_053c92),
    (0x053C98, i_053c98),
    (0x053C9E, i_053c9e),
    (0x053CA2, i_053ca2),
    (0x053CA4, i_053ca4),
    (0x053CA8, i_053ca8),
    (0x053CAE, i_053cae),
    (0x053CB4, i_053cb4),
    (0x053CBA, i_053cba),
    (0x053CBE, i_053cbe),
    (0x053CC2, i_053cc2),
    (0x053CC6, i_053cc6),
    (0x053CCA, i_053cca),
    (0x053CD0, i_053cd0),
    (0x053CD6, i_053cd6),
    (0x053CDC, i_053cdc),
    (0x053CE2, i_053ce2),
    (0x053CE6, i_053ce6),
    (0x053CEA, i_053cea),
    (0x053CF0, i_053cf0),
    (0x053CF2, i_053cf2),
    (0x053CF6, i_053cf6),
    (0x053CFC, i_053cfc),
    (0x053D02, i_053d02),
    (0x053D08, i_053d08),
    (0x053D0E, i_053d0e),
    (0x053D12, i_053d12),
    (0x053D14, i_053d14),
    (0x053D1A, i_053d1a),
    (0x053D20, i_053d20),
    (0x053D24, i_053d24),
    (0x053D2A, i_053d2a),
    (0x053D2C, i_053d2c),
    (0x053D32, i_053d32),
    (0x053D36, i_053d36),
    (0x053D3C, i_053d3c),
    (0x053D42, i_053d42),
    (0x053D48, i_053d48),
    (0x053D4E, i_053d4e),
    (0x053D50, i_053d50),
    (0x053D56, i_053d56),
    (0x053D5A, i_053d5a),
    (0x053D5C, i_053d5c),
    (0x053D60, i_053d60),
    (0x053D64, i_053d64),
    (0x053D68, i_053d68),
    (0x053D6E, i_053d6e),
    (0x053D70, i_053d70),
    (0x053D76, i_053d76),
    (0x053D78, i_053d78),
    (0x053D7E, i_053d7e),
    (0x053D80, i_053d80),
    (0x053D84, i_053d84),
    (0x053D86, i_053d86),
    (0x053D8A, i_053d8a),
    (0x053D8E, i_053d8e),
    (0x053D92, i_053d92),
    (0x053D98, i_053d98),
    (0x053D9C, i_053d9c),
    (0x053DA0, i_053da0),
    (0x053DA4, i_053da4),
    (0x053DAA, i_053daa),
    (0x053DB0, i_053db0),
    (0x053DB4, i_053db4),
    (0x053DB8, i_053db8),
    (0x053DBC, i_053dbc),
    (0x053DC2, i_053dc2),
    (0x053DC8, i_053dc8),
    (0x053DCA, i_053dca),
    (0x053DCE, i_053dce),
    (0x053DD2, i_053dd2),
    (0x053DD6, i_053dd6),
    (0x053DDA, i_053dda),
    (0x053DE0, i_053de0),
    (0x053DE4, i_053de4),
    (0x053DEA, i_053dea),
    (0x053DF0, i_053df0),
    (0x053DF4, i_053df4),
    (0x053DFA, i_053dfa),
    (0x053E00, i_053e00),
    (0x053E04, i_053e04),
    (0x053E0A, i_053e0a),
    (0x053E0C, i_053e0c),
    (0x053E12, i_053e12),
    (0x053E16, i_053e16),
    (0x053E1A, i_053e1a),
    (0x053E20, i_053e20),
    (0x053E24, i_053e24),
    (0x053E2A, i_053e2a),
    (0x053E2E, i_053e2e),
    (0x053E32, i_053e32),
    (0x053E38, i_053e38),
    (0x053E3C, i_053e3c),
    (0x053E42, i_053e42),
    (0x053E46, i_053e46),
    (0x053E4A, i_053e4a),
    (0x053E50, i_053e50),
    (0x053E54, i_053e54),
    (0x053E5A, i_053e5a),
    (0x053E5E, i_053e5e),
    (0x053E62, i_053e62),
    (0x053E68, i_053e68),
    (0x053E6C, i_053e6c),
    (0x053E70, i_053e70),
    (0x053E76, i_053e76),
    (0x053E78, i_053e78),
    (0x053E7E, i_053e7e),
    (0x053E82, i_053e82),
    (0x053E86, i_053e86),
    (0x053E88, i_053e88),
    (0x053E8C, i_053e8c),
    (0x053E8E, i_053e8e),
    (0x053E90, i_053e90),
    (0x053E94, i_053e94),
    (0x053E9A, i_053e9a),
    (0x053E9C, i_053e9c),
    (0x053EA0, i_053ea0),
    (0x053EA6, i_053ea6),
    (0x053EA8, i_053ea8),
    (0x053EAE, i_053eae),
    (0x053EB2, i_053eb2),
    (0x053EB8, i_053eb8),
    (0x053EBA, i_053eba),
    (0x053EC0, i_053ec0),
    (0x053EC6, i_053ec6),
    (0x053ECA, i_053eca),
    (0x053ED0, i_053ed0),
    (0x053ED4, i_053ed4),
    (0x053EDA, i_053eda),
    (0x053EE0, i_053ee0),
    (0x053EE2, i_053ee2),
    (0x053EE8, i_053ee8),
    (0x053EEE, i_053eee),
    (0x053EF4, i_053ef4),
    (0x053EFA, i_053efa),
    (0x053F00, i_053f00),
    (0x053F06, i_053f06),
    (0x053F08, i_053f08),
    (0x053F0E, i_053f0e),
    (0x053F14, i_053f14),
    (0x053F1A, i_053f1a),
    (0x053F20, i_053f20),
    (0x053F26, i_053f26),
    (0x053F2C, i_053f2c),
    (0x053F2E, i_053f2e),
    (0x053F34, i_053f34),
    (0x053F3A, i_053f3a),
    (0x053F40, i_053f40),
    (0x053F46, i_053f46),
    (0x053F4C, i_053f4c),
    (0x053F52, i_053f52),
    (0x053F54, i_053f54),
    (0x053F5A, i_053f5a),
    (0x053F60, i_053f60),
    (0x053F66, i_053f66),
    (0x053F6C, i_053f6c),
    (0x053F72, i_053f72),
    (0x053F78, i_053f78),
    (0x053F7A, i_053f7a),
    (0x053F7E, i_053f7e),
    (0x053F82, i_053f82),
    (0x053F86, i_053f86),
    (0x053F8A, i_053f8a),
    (0x053F8E, i_053f8e),
    (0x053F90, i_053f90),
    (0x053F94, i_053f94),
    (0x053F96, i_053f96),
    (0x053F9A, i_053f9a),
    (0x053FA0, i_053fa0),
    (0x053FA4, i_053fa4),
    (0x053FAA, i_053faa),
    (0x053FB0, i_053fb0),
    (0x053FB6, i_053fb6),
    (0x053FBC, i_053fbc),
    (0x053FC2, i_053fc2),
    (0x053FC8, i_053fc8),
    (0x053FCE, i_053fce),
    (0x053FD4, i_053fd4),
    (0x053FD8, i_053fd8),
    (0x053FDA, i_053fda),
    (0x053FE0, i_053fe0),
    (0x053FE6, i_053fe6),
    (0x053FEC, i_053fec),
    (0x053FF2, i_053ff2),
    (0x053FF6, i_053ff6),
    (0x053FFC, i_053ffc),
    (0x054000, i_054000),
    (0x054006, i_054006),
    (0x05400C, i_05400c),
    (0x054012, i_054012),
    (0x054018, i_054018),
    (0x05401C, i_05401c),
    (0x054020, i_054020),
    (0x054026, i_054026),
    (0x05402C, i_05402c),
    (0x054032, i_054032),
    (0x054038, i_054038),
    (0x05403E, i_05403e),
    (0x054044, i_054044),
    (0x054048, i_054048),
    (0x05404A, i_05404a),
    (0x054050, i_054050),
    (0x054054, i_054054),
    (0x05405A, i_05405a),
    (0x05405C, i_05405c),
    (0x054062, i_054062),
    (0x054068, i_054068),
    (0x05406E, i_05406e),
    (0x054074, i_054074),
    (0x05407A, i_05407a),
    (0x05407E, i_05407e),
    (0x054080, i_054080),
    (0x054086, i_054086),
    (0x05408C, i_05408c),
    (0x054092, i_054092),
    (0x054098, i_054098),
    (0x05409C, i_05409c),
    (0x0540A2, i_0540a2),
    (0x0540A8, i_0540a8),
    (0x0540AE, i_0540ae),
    (0x0540B4, i_0540b4),
    (0x0540B8, i_0540b8),
    (0x0540BE, i_0540be),
    (0x0540C2, i_0540c2),
    (0x0540C8, i_0540c8),
    (0x0540CE, i_0540ce),
    (0x0540D4, i_0540d4),
    (0x0540DA, i_0540da),
    (0x0540E0, i_0540e0),
    (0x0540E6, i_0540e6),
    (0x0540EC, i_0540ec),
    (0x0540F2, i_0540f2),
    (0x0540F6, i_0540f6),
    (0x0540F8, i_0540f8),
    (0x0540FE, i_0540fe),
    (0x054102, i_054102),
    (0x054108, i_054108),
    (0x05410A, i_05410a),
    (0x054110, i_054110),
    (0x054116, i_054116),
    (0x05411C, i_05411c),
    (0x054122, i_054122),
    (0x054128, i_054128),
    (0x05412C, i_05412c),
    (0x054130, i_054130),
    (0x054136, i_054136),
    (0x05413C, i_05413c),
    (0x054140, i_054140),
    (0x054142, i_054142),
    (0x054148, i_054148),
    (0x05414E, i_05414e),
    (0x054154, i_054154),
    (0x054158, i_054158),
    (0x05415E, i_05415e),
    (0x054160, i_054160),
    (0x054164, i_054164),
    (0x05416A, i_05416a),
    (0x05416E, i_05416e),
    (0x054174, i_054174),
    (0x05417A, i_05417a),
    (0x054180, i_054180),
    (0x054186, i_054186),
    (0x05418C, i_05418c),
    (0x054192, i_054192),
    (0x054198, i_054198),
    (0x05419E, i_05419e),
    (0x0541A4, i_0541a4),
    (0x0541AA, i_0541aa),
    (0x0541B0, i_0541b0),
    (0x0541B4, i_0541b4),
    (0x0541B6, i_0541b6),
    (0x0541BC, i_0541bc),
    (0x0541C2, i_0541c2),
    (0x0541C8, i_0541c8),
    (0x0541CE, i_0541ce),
    (0x0541D2, i_0541d2),
    (0x0541D8, i_0541d8),
    (0x0541DE, i_0541de),
    (0x0541E4, i_0541e4),
    (0x0541EA, i_0541ea),
    (0x0541EE, i_0541ee),
    (0x0541F4, i_0541f4),
    (0x0541FA, i_0541fa),
    (0x054200, i_054200),
    (0x054204, i_054204),
    (0x05420A, i_05420a),
    (0x054210, i_054210),
    (0x054216, i_054216),
    (0x05421C, i_05421c),
    (0x054222, i_054222),
    (0x054228, i_054228),
    (0x05422E, i_05422e),
    (0x054234, i_054234),
    (0x05423A, i_05423a),
    (0x05423E, i_05423e),
    (0x054242, i_054242),
    (0x054248, i_054248),
    (0x05424C, i_05424c),
    (0x054252, i_054252),
    (0x054254, i_054254),
    (0x05425A, i_05425a),
    (0x054260, i_054260),
    (0x054266, i_054266),
    (0x05426C, i_05426c),
    (0x054272, i_054272),
    (0x054278, i_054278),
    (0x05427E, i_05427e),
    (0x054282, i_054282),
    (0x054286, i_054286),
    (0x05428C, i_05428c),
    (0x054292, i_054292),
    (0x054298, i_054298),
    (0x05429E, i_05429e),
    (0x0542A4, i_0542a4),
    (0x0542AA, i_0542aa),
    (0x0542B0, i_0542b0),
    (0x0542B6, i_0542b6),
    (0x0542BC, i_0542bc),
    (0x0542C0, i_0542c0),
    (0x0542C2, i_0542c2),
    (0x0542C8, i_0542c8),
    (0x0542CE, i_0542ce),
    (0x0542D4, i_0542d4),
    (0x0542DA, i_0542da),
    (0x0542DE, i_0542de),
    (0x0542E4, i_0542e4),
    (0x0542EA, i_0542ea),
    (0x0542F0, i_0542f0),
    (0x0542F6, i_0542f6),
    (0x0542FA, i_0542fa),
    (0x0542FE, i_0542fe),
    (0x054304, i_054304),
    (0x05430A, i_05430a),
    (0x054310, i_054310),
    (0x054316, i_054316),
    (0x05431A, i_05431a),
    (0x054320, i_054320),
    (0x054324, i_054324),
    (0x054326, i_054326),
    (0x05432C, i_05432c),
    (0x054330, i_054330),
    (0x054336, i_054336),
    (0x054338, i_054338),
    (0x05433E, i_05433e),
    (0x054342, i_054342),
    (0x054348, i_054348),
    (0x05434E, i_05434e),
    (0x054352, i_054352),
    (0x054358, i_054358),
    (0x05435E, i_05435e),
    (0x054362, i_054362),
    (0x054364, i_054364),
    (0x05436A, i_05436a),
    (0x054370, i_054370),
    (0x054376, i_054376),
    (0x05437C, i_05437c),
    (0x054380, i_054380),
    (0x054386, i_054386),
    (0x05438C, i_05438c),
    (0x054392, i_054392),
    (0x054398, i_054398),
    (0x05439C, i_05439c),
    (0x0543A2, i_0543a2),
    (0x0543A6, i_0543a6),
    (0x0543AC, i_0543ac),
    (0x0543B2, i_0543b2),
    (0x0543B6, i_0543b6),
    (0x0543BC, i_0543bc),
    (0x0543C2, i_0543c2),
    (0x0543C8, i_0543c8),
    (0x0543CE, i_0543ce),
    (0x0543D4, i_0543d4),
    (0x0543DA, i_0543da),
    (0x0543E0, i_0543e0),
    (0x0543E4, i_0543e4),
    (0x0543E6, i_0543e6),
    (0x0543EC, i_0543ec),
    (0x0543F0, i_0543f0),
    (0x0543F6, i_0543f6),
    (0x0543F8, i_0543f8),
    (0x0543FE, i_0543fe),
    (0x054404, i_054404),
    (0x054408, i_054408),
    (0x05440E, i_05440e),
    (0x054414, i_054414),
    (0x054418, i_054418),
    (0x05441E, i_05441e),
    (0x054424, i_054424),
    (0x05442A, i_05442a),
    (0x054430, i_054430),
    (0x054436, i_054436),
    (0x05443A, i_05443a),
    (0x054440, i_054440),
    (0x054446, i_054446),
    (0x05444C, i_05444c),
    (0x054452, i_054452),
    (0x054458, i_054458),
    (0x05445E, i_05445e),
    (0x054464, i_054464),
    (0x054468, i_054468),
    (0x05446A, i_05446a),
    (0x054470, i_054470),
    (0x054476, i_054476),
    (0x05447C, i_05447c),
    (0x05447E, i_05447e),
    (0x054482, i_054482),
    (0x054488, i_054488),
    (0x05448E, i_05448e),
    (0x054492, i_054492),
    (0x054498, i_054498),
    (0x05449E, i_05449e),
    (0x0544A2, i_0544a2),
    (0x0544A8, i_0544a8),
    (0x0544AE, i_0544ae),
    (0x0544B2, i_0544b2),
    (0x0544B8, i_0544b8),
    (0x0544BE, i_0544be),
    (0x0544C4, i_0544c4),
    (0x0544CA, i_0544ca),
    (0x0544D0, i_0544d0),
    (0x0544D4, i_0544d4),
    (0x0544D6, i_0544d6),
    (0x0544DC, i_0544dc),
    (0x0544E2, i_0544e2),
    (0x0544E6, i_0544e6),
    (0x0544EA, i_0544ea),
    (0x0544EC, i_0544ec),
    (0x0544F2, i_0544f2),
    (0x0544F6, i_0544f6),
    (0x0544FA, i_0544fa),
    (0x0544FC, i_0544fc),
    (0x054502, i_054502),
    (0x054506, i_054506),
    (0x05450C, i_05450c),
    (0x054510, i_054510),
    (0x054516, i_054516),
    (0x054518, i_054518),
    (0x05451E, i_05451e),
    (0x054522, i_054522),
    (0x054524, i_054524),
    (0x05452A, i_05452a),
    (0x054530, i_054530),
    (0x054534, i_054534),
    (0x054538, i_054538),
    (0x05453A, i_05453a),
    (0x054540, i_054540),
    (0x054544, i_054544),
    (0x054548, i_054548),
    (0x05454A, i_05454a),
    (0x054550, i_054550),
    (0x054554, i_054554),
    (0x05455A, i_05455a),
    (0x05455E, i_05455e),
    (0x054564, i_054564),
    (0x054566, i_054566),
    (0x05456C, i_05456c),
    (0x054570, i_054570),
    (0x054572, i_054572),
    (0x054578, i_054578),
    (0x05457C, i_05457c),
    (0x054582, i_054582),
    (0x054584, i_054584),
    (0x054588, i_054588),
    (0x05458A, i_05458a),
    (0x054590, i_054590),
    (0x054596, i_054596),
    (0x05459A, i_05459a),
    (0x0545A0, i_0545a0),
    (0x0545A6, i_0545a6),
    (0x0545AC, i_0545ac),
    (0x0545B2, i_0545b2),
    (0x0545B8, i_0545b8),
    (0x0545BE, i_0545be),
    (0x0545C0, i_0545c0),
    (0x0545C6, i_0545c6),
    (0x0545CC, i_0545cc),
    (0x0545D0, i_0545d0),
    (0x0545D2, i_0545d2),
    (0x0545D6, i_0545d6),
    (0x0545DC, i_0545dc),
    (0x0545E2, i_0545e2),
    (0x0545E8, i_0545e8),
    (0x0545EE, i_0545ee),
    (0x0545F4, i_0545f4),
    (0x0545FA, i_0545fa),
    (0x0545FE, i_0545fe),
    (0x054602, i_054602),
    (0x054608, i_054608),
    (0x05460E, i_05460e),
    (0x054614, i_054614),
    (0x05461A, i_05461a),
    (0x054620, i_054620),
    (0x054626, i_054626),
    (0x05462C, i_05462c),
    (0x054630, i_054630),
    (0x054632, i_054632),
    (0x054638, i_054638),
    (0x05463E, i_05463e),
    (0x054642, i_054642),
    (0x054648, i_054648),
    (0x05464E, i_05464e),
    (0x054652, i_054652),
    (0x054658, i_054658),
    (0x05465E, i_05465e),
    (0x054664, i_054664),
    (0x05466A, i_05466a),
    (0x05466E, i_05466e),
    (0x054674, i_054674),
    (0x054678, i_054678),
    (0x05467E, i_05467e),
    (0x054684, i_054684),
    (0x05468A, i_05468a),
    (0x054690, i_054690),
    (0x054696, i_054696),
    (0x05469C, i_05469c),
    (0x0546A2, i_0546a2),
    (0x0546A6, i_0546a6),
    (0x0546A8, i_0546a8),
    (0x0546AE, i_0546ae),
    (0x0546B2, i_0546b2),
    (0x0546B8, i_0546b8),
    (0x0546BA, i_0546ba),
    (0x0546C0, i_0546c0),
    (0x0546C6, i_0546c6),
    (0x0546CA, i_0546ca),
    (0x0546CC, i_0546cc),
    (0x0546D2, i_0546d2),
    (0x0546D8, i_0546d8),
    (0x0546DC, i_0546dc),
    (0x0546E2, i_0546e2),
    (0x0546E8, i_0546e8),
    (0x0546EC, i_0546ec),
    (0x0546F2, i_0546f2),
    (0x0546F4, i_0546f4),
    (0x0546F8, i_0546f8),
    (0x0546FE, i_0546fe),
    (0x054702, i_054702),
    (0x054708, i_054708),
    (0x05470E, i_05470e),
    (0x054714, i_054714),
    (0x05471A, i_05471a),
    (0x054720, i_054720),
    (0x054726, i_054726),
    (0x05472A, i_05472a),
    (0x05472E, i_05472e),
    (0x054734, i_054734),
    (0x05473A, i_05473a),
    (0x054740, i_054740),
    (0x054744, i_054744),
    (0x054746, i_054746),
    (0x05474C, i_05474c),
    (0x054752, i_054752),
    (0x054758, i_054758),
    (0x05475E, i_05475e),
    (0x054762, i_054762),
    (0x054768, i_054768),
    (0x05476E, i_05476e),
    (0x054774, i_054774),
    (0x05477A, i_05477a),
    (0x05477E, i_05477e),
    (0x054784, i_054784),
    (0x054788, i_054788),
    (0x05478E, i_05478e),
    (0x054794, i_054794),
    (0x05479A, i_05479a),
    (0x0547A0, i_0547a0),
    (0x0547A6, i_0547a6),
    (0x0547AC, i_0547ac),
    (0x0547B2, i_0547b2),
    (0x0547B6, i_0547b6),
    (0x0547B8, i_0547b8),
    (0x0547BE, i_0547be),
    (0x0547C2, i_0547c2),
    (0x0547C8, i_0547c8),
    (0x0547CA, i_0547ca),
    (0x0547D0, i_0547d0),
    (0x0547D6, i_0547d6),
    (0x0547DC, i_0547dc),
    (0x0547E2, i_0547e2),
    (0x0547E4, i_0547e4),
    (0x0547E8, i_0547e8),
    (0x0547EC, i_0547ec),
    (0x0547F0, i_0547f0),
    (0x0547F6, i_0547f6),
    (0x0547FC, i_0547fc),
    (0x054800, i_054800),
    (0x054802, i_054802),
    (0x054808, i_054808),
    (0x05480E, i_05480e),
    (0x054814, i_054814),
    (0x05481A, i_05481a),
    (0x05481C, i_05481c),
    (0x054820, i_054820),
    (0x054826, i_054826),
    (0x05482A, i_05482a),
    (0x054830, i_054830),
    (0x054836, i_054836),
    (0x05483C, i_05483c),
    (0x054842, i_054842),
    (0x054848, i_054848),
    (0x05484E, i_05484e),
    (0x054854, i_054854),
    (0x05485A, i_05485a),
    (0x05485E, i_05485e),
    (0x054860, i_054860),
    (0x054866, i_054866),
    (0x05486C, i_05486c),
    (0x054872, i_054872),
    (0x054876, i_054876),
    (0x05487C, i_05487c),
    (0x054882, i_054882),
    (0x054888, i_054888),
    (0x05488E, i_05488e),
    (0x054892, i_054892),
    (0x054898, i_054898),
    (0x05489E, i_05489e),
    (0x0548A4, i_0548a4),
    (0x0548AA, i_0548aa),
    (0x0548B0, i_0548b0),
    (0x0548B4, i_0548b4),
    (0x0548B6, i_0548b6),
    (0x0548BC, i_0548bc),
    (0x0548C0, i_0548c0),
    (0x0548C6, i_0548c6),
    (0x0548C8, i_0548c8),
    (0x0548CC, i_0548cc),
    (0x0548D2, i_0548d2),
    (0x0548D8, i_0548d8),
    (0x0548DE, i_0548de),
    (0x0548E2, i_0548e2),
    (0x0548E4, i_0548e4),
    (0x0548EA, i_0548ea),
    (0x0548F0, i_0548f0),
    (0x0548F4, i_0548f4),
    (0x0548F8, i_0548f8),
    (0x0548FA, i_0548fa),
    (0x054900, i_054900),
    (0x054904, i_054904),
    (0x05490A, i_05490a),
    (0x05490C, i_05490c),
    (0x054912, i_054912),
    (0x054918, i_054918),
    (0x05491E, i_05491e),
    (0x054924, i_054924),
    (0x05492A, i_05492a),
    (0x054930, i_054930),
    (0x054936, i_054936),
    (0x05493A, i_05493a),
    (0x05493C, i_05493c),
    (0x054942, i_054942),
    (0x054948, i_054948),
    (0x05494E, i_05494e),
    (0x054952, i_054952),
    (0x054958, i_054958),
    (0x05495A, i_05495a),
    (0x05495E, i_05495e),
    (0x054964, i_054964),
    (0x054968, i_054968),
    (0x05496E, i_05496e),
    (0x054974, i_054974),
    (0x05497A, i_05497a),
    (0x054980, i_054980),
    (0x054986, i_054986),
    (0x05498C, i_05498c),
    (0x054992, i_054992),
    (0x054998, i_054998),
    (0x05499C, i_05499c),
    (0x05499E, i_05499e),
    (0x0549A4, i_0549a4),
    (0x0549AA, i_0549aa),
    (0x0549B0, i_0549b0),
    (0x0549B4, i_0549b4),
    (0x0549BA, i_0549ba),
    (0x0549C0, i_0549c0),
    (0x0549C4, i_0549c4),
    (0x0549CA, i_0549ca),
    (0x0549CE, i_0549ce),
    (0x0549D4, i_0549d4),
    (0x0549DA, i_0549da),
    (0x0549E0, i_0549e0),
    (0x0549E6, i_0549e6),
    (0x0549EC, i_0549ec),
    (0x0549F2, i_0549f2),
    (0x0549F8, i_0549f8),
    (0x0549FE, i_0549fe),
    (0x054A04, i_054a04),
    (0x054A08, i_054a08),
    (0x054A0E, i_054a0e),
    (0x054A12, i_054a12),
    (0x054A18, i_054a18),
    (0x054A1A, i_054a1a),
    (0x054A1E, i_054a1e),
    (0x054A24, i_054a24),
    (0x054A28, i_054a28),
    (0x054A2E, i_054a2e),
    (0x054A34, i_054a34),
    (0x054A3A, i_054a3a),
    (0x054A40, i_054a40),
    (0x054A46, i_054a46),
    (0x054A4C, i_054a4c),
    (0x054A52, i_054a52),
    (0x054A58, i_054a58),
    (0x054A5E, i_054a5e),
    (0x054A62, i_054a62),
    (0x054A64, i_054a64),
    (0x054A6A, i_054a6a),
    (0x054A70, i_054a70),
    (0x054A74, i_054a74),
    (0x054A7A, i_054a7a),
    (0x054A80, i_054a80),
    (0x054A86, i_054a86),
    (0x054A8A, i_054a8a),
    (0x054A90, i_054a90),
    (0x054A96, i_054a96),
    (0x054A9C, i_054a9c),
    (0x054AA0, i_054aa0),
    (0x054AA6, i_054aa6),
    (0x054AA8, i_054aa8),
    (0x054AAC, i_054aac),
    (0x054AB2, i_054ab2),
    (0x054AB6, i_054ab6),
    (0x054ABC, i_054abc),
    (0x054AC2, i_054ac2),
    (0x054AC6, i_054ac6),
    (0x054ACC, i_054acc),
    (0x054AD2, i_054ad2),
    (0x054AD8, i_054ad8),
    (0x054ADC, i_054adc),
    (0x054AE2, i_054ae2),
    (0x054AE8, i_054ae8),
    (0x054AEE, i_054aee),
    (0x054AF4, i_054af4),
    (0x054AFA, i_054afa),
    (0x054B00, i_054b00),
    (0x054B06, i_054b06),
    (0x054B0A, i_054b0a),
    (0x054B10, i_054b10),
    (0x054B14, i_054b14),
    (0x054B1A, i_054b1a),
    (0x054B1C, i_054b1c),
    (0x054B20, i_054b20),
    (0x054B26, i_054b26),
    (0x054B2A, i_054b2a),
    (0x054B30, i_054b30),
    (0x054B36, i_054b36),
    (0x054B3C, i_054b3c),
    (0x054B42, i_054b42),
    (0x054B48, i_054b48),
    (0x054B4E, i_054b4e),
    (0x054B54, i_054b54),
    (0x054B5A, i_054b5a),
    (0x054B5E, i_054b5e),
    (0x054B60, i_054b60),
    (0x054B66, i_054b66),
    (0x054B6C, i_054b6c),
    (0x054B72, i_054b72),
    (0x054B76, i_054b76),
    (0x054B7C, i_054b7c),
    (0x054B82, i_054b82),
    (0x054B88, i_054b88),
    (0x054B8E, i_054b8e),
    (0x054B92, i_054b92),
    (0x054B98, i_054b98),
    (0x054B9C, i_054b9c),
    (0x054BA2, i_054ba2),
    (0x054BA8, i_054ba8),
    (0x054BAE, i_054bae),
    (0x054BB4, i_054bb4),
    (0x054BBA, i_054bba),
    (0x054BC0, i_054bc0),
    (0x054BC6, i_054bc6),
    (0x054BCA, i_054bca),
    (0x054BCC, i_054bcc),
    (0x054BD2, i_054bd2),
    (0x054BD6, i_054bd6),
    (0x054BDC, i_054bdc),
    (0x054BDE, i_054bde),
    (0x054BE4, i_054be4),
    (0x054BEA, i_054bea),
    (0x054BEE, i_054bee),
    (0x054BF0, i_054bf0),
    (0x054BF6, i_054bf6),
    (0x054BFC, i_054bfc),
    (0x054C02, i_054c02),
    (0x054C06, i_054c06),
    (0x054C0C, i_054c0c),
    (0x054C0E, i_054c0e),
    (0x054C12, i_054c12),
    (0x054C18, i_054c18),
    (0x054C1E, i_054c1e),
    (0x054C24, i_054c24),
    (0x054C2A, i_054c2a),
    (0x054C30, i_054c30),
    (0x054C36, i_054c36),
    (0x054C3C, i_054c3c),
    (0x054C40, i_054c40),
    (0x054C46, i_054c46),
    (0x054C4C, i_054c4c),
    (0x054C50, i_054c50),
    (0x054C52, i_054c52),
    (0x054C58, i_054c58),
    (0x054C5E, i_054c5e),
    (0x054C64, i_054c64),
    (0x054C6A, i_054c6a),
    (0x054C6E, i_054c6e),
    (0x054C74, i_054c74),
    (0x054C7A, i_054c7a),
    (0x054C80, i_054c80),
    (0x054C86, i_054c86),
    (0x054C8A, i_054c8a),
    (0x054C90, i_054c90),
    (0x054C96, i_054c96),
    (0x054C9A, i_054c9a),
    (0x054CA0, i_054ca0),
    (0x054CA6, i_054ca6),
    (0x054CAC, i_054cac),
    (0x054CB2, i_054cb2),
    (0x054CB8, i_054cb8),
    (0x054CBE, i_054cbe),
    (0x054CC4, i_054cc4),
    (0x054CCA, i_054cca),
    (0x054CD0, i_054cd0),
    (0x054CD6, i_054cd6),
    (0x054CDC, i_054cdc),
    (0x054CE0, i_054ce0),
    (0x054CE6, i_054ce6),
    (0x054CEA, i_054cea),
    (0x054CF0, i_054cf0),
    (0x054CF2, i_054cf2),
    (0x054CF6, i_054cf6),
    (0x054CFC, i_054cfc),
    (0x054D02, i_054d02),
    (0x054D08, i_054d08),
    (0x054D0E, i_054d0e),
    (0x054D12, i_054d12),
    (0x054D16, i_054d16),
    (0x054D1C, i_054d1c),
    (0x054D22, i_054d22),
    (0x054D28, i_054d28),
    (0x054D2E, i_054d2e),
    (0x054D32, i_054d32),
    (0x054D36, i_054d36),
    (0x054D3C, i_054d3c),
    (0x054D42, i_054d42),
    (0x054D48, i_054d48),
    (0x054D4E, i_054d4e),
    (0x054D52, i_054d52),
    (0x054D56, i_054d56),
    (0x054D5C, i_054d5c),
    (0x054D62, i_054d62),
    (0x054D68, i_054d68),
    (0x054D6E, i_054d6e),
    (0x054D72, i_054d72),
    (0x054D76, i_054d76),
    (0x054D7C, i_054d7c),
    (0x054D82, i_054d82),
    (0x054D88, i_054d88),
    (0x054D8E, i_054d8e),
    (0x054D92, i_054d92),
    (0x054D98, i_054d98),
    (0x054D9E, i_054d9e),
    (0x054DA2, i_054da2),
    (0x054DA8, i_054da8),
    (0x054DAE, i_054dae),
    (0x054DB4, i_054db4),
    (0x054DB8, i_054db8),
    (0x054DBC, i_054dbc),
    (0x054DC2, i_054dc2),
    (0x054DC8, i_054dc8),
    (0x054DCE, i_054dce),
    (0x054DD4, i_054dd4),
    (0x054DDA, i_054dda),
    (0x054DE0, i_054de0),
    (0x054DE6, i_054de6),
    (0x054DEA, i_054dea),
    (0x054DF0, i_054df0),
    (0x054DF6, i_054df6),
    (0x054DFA, i_054dfa),
    (0x054E00, i_054e00),
    (0x054E04, i_054e04),
    (0x054E08, i_054e08),
    (0x054E0C, i_054e0c),
    (0x054E10, i_054e10),
    (0x054E12, i_054e12),
    (0x054E18, i_054e18),
    (0x054E1E, i_054e1e),
    (0x054E24, i_054e24),
    (0x054E28, i_054e28),
    (0x054E2E, i_054e2e),
    (0x054E32, i_054e32),
    (0x054E38, i_054e38),
    (0x054E3E, i_054e3e),
    (0x054E44, i_054e44),
    (0x054E4A, i_054e4a),
    (0x054E4E, i_054e4e),
    (0x054E52, i_054e52),
    (0x054E58, i_054e58),
    (0x054E5C, i_054e5c),
    (0x054E5E, i_054e5e),
    (0x054E64, i_054e64),
    (0x054E68, i_054e68),
    (0x054E6E, i_054e6e),
    (0x054E70, i_054e70),
    (0x054E74, i_054e74),
    (0x054E7A, i_054e7a),
    (0x054E7C, i_054e7c),
    (0x054E80, i_054e80),
    (0x054E86, i_054e86),
    (0x054E8A, i_054e8a),
    (0x054E90, i_054e90),
    (0x054E94, i_054e94),
    (0x054E96, i_054e96),
    (0x054E9C, i_054e9c),
    (0x054EA2, i_054ea2),
    (0x054EA8, i_054ea8),
    (0x054EAC, i_054eac),
    (0x054EB2, i_054eb2),
    (0x054EB8, i_054eb8),
    (0x054EBE, i_054ebe),
    (0x054EC4, i_054ec4),
    (0x054EC8, i_054ec8),
    (0x054ECC, i_054ecc),
    (0x054ED2, i_054ed2),
    (0x054ED6, i_054ed6),
    (0x054ED8, i_054ed8),
    (0x054EDE, i_054ede),
    (0x054EE2, i_054ee2),
    (0x054EE8, i_054ee8),
    (0x054EEA, i_054eea),
    (0x054EEE, i_054eee),
    (0x054EF4, i_054ef4),
    (0x054EF6, i_054ef6),
    (0x054EFA, i_054efa),
    (0x054F00, i_054f00),
    (0x054F04, i_054f04),
    (0x054F0A, i_054f0a),
    (0x054F0E, i_054f0e),
    (0x054F10, i_054f10),
    (0x054F16, i_054f16),
    (0x054F1C, i_054f1c),
    (0x054F22, i_054f22),
    (0x054F26, i_054f26),
    (0x054F2C, i_054f2c),
    (0x054F32, i_054f32),
    (0x054F38, i_054f38),
    (0x054F3E, i_054f3e),
    (0x054F42, i_054f42),
    (0x054F48, i_054f48),
    (0x054F4C, i_054f4c),
    (0x054F52, i_054f52),
    (0x054F56, i_054f56),
    (0x054F58, i_054f58),
    (0x054F5E, i_054f5e),
    (0x054F62, i_054f62),
    (0x054F68, i_054f68),
    (0x054F6A, i_054f6a),
    (0x054F70, i_054f70),
    (0x054F76, i_054f76),
    (0x054F7A, i_054f7a),
    (0x054F80, i_054f80),
    (0x054F82, i_054f82),
    (0x054F86, i_054f86),
    (0x054F8C, i_054f8c),
    (0x054F90, i_054f90),
    (0x054F96, i_054f96),
    (0x054F9A, i_054f9a),
    (0x054F9C, i_054f9c),
    (0x054FA2, i_054fa2),
    (0x054FA8, i_054fa8),
    (0x054FAE, i_054fae),
    (0x054FB2, i_054fb2),
    (0x054FB8, i_054fb8),
    (0x054FBA, i_054fba),
    (0x054FBE, i_054fbe),
    (0x054FC2, i_054fc2),
    (0x054FC6, i_054fc6),
    (0x054FCA, i_054fca),
    (0x054FD0, i_054fd0),
    (0x054FD6, i_054fd6),
    (0x054FDA, i_054fda),
    (0x054FDC, i_054fdc),
    (0x054FE0, i_054fe0),
    (0x054FE4, i_054fe4),
    (0x054FE8, i_054fe8),
    (0x054FEA, i_054fea),
    (0x054FF0, i_054ff0),
    (0x054FF2, i_054ff2),
    (0x054FF6, i_054ff6),
    (0x054FFA, i_054ffa),
    (0x055000, i_055000),
    (0x055002, i_055002),
    (0x055006, i_055006),
    (0x05500A, i_05500a),
    (0x05500E, i_05500e),
    (0x055012, i_055012),
    (0x055018, i_055018),
    (0x05501E, i_05501e),
    (0x055022, i_055022),
    (0x055024, i_055024),
    (0x055028, i_055028),
    (0x05502C, i_05502c),
    (0x055030, i_055030),
    (0x055032, i_055032),
    (0x055038, i_055038),
    (0x05503A, i_05503a),
    (0x05503E, i_05503e),
    (0x055042, i_055042),
    (0x055048, i_055048),
    (0x05504A, i_05504a),
    (0x05504E, i_05504e),
    (0x055054, i_055054),
    (0x055058, i_055058),
    (0x05505E, i_05505e),
    (0x055064, i_055064),
    (0x05506A, i_05506a),
    (0x055070, i_055070),
    (0x055078, i_055078),
    (0x05507C, i_05507c),
    (0x05507E, i_05507e),
    (0x055084, i_055084),
    (0x05508A, i_05508a),
    (0x055090, i_055090),
    (0x055094, i_055094),
    (0x05509A, i_05509a),
    (0x05509C, i_05509c),
    (0x0550A2, i_0550a2),
    (0x0550A6, i_0550a6),
    (0x0550AC, i_0550ac),
    (0x0550B0, i_0550b0),
    (0x0550B4, i_0550b4),
    (0x0550B8, i_0550b8),
    (0x0550BE, i_0550be),
    (0x0550C2, i_0550c2),
    (0x0550C4, i_0550c4),
    (0x0550C8, i_0550c8),
    (0x0550CE, i_0550ce),
    (0x0550D2, i_0550d2),
    (0x0550D8, i_0550d8),
    (0x0550DC, i_0550dc),
    (0x0550E0, i_0550e0),
    (0x0550E6, i_0550e6),
    (0x0550E8, i_0550e8),
    (0x0550EC, i_0550ec),
    (0x0550F2, i_0550f2),
    (0x0550F6, i_0550f6),
    (0x0550FA, i_0550fa),
    (0x0550FE, i_0550fe),
    (0x055104, i_055104),
    (0x055108, i_055108),
    (0x05510C, i_05510c),
    (0x05510E, i_05510e),
    (0x055112, i_055112),
    (0x055118, i_055118),
    (0x05511C, i_05511c),
    (0x055120, i_055120),
    (0x055122, i_055122),
    (0x055126, i_055126),
    (0x05512A, i_05512a),
    (0x05512E, i_05512e),
    (0x055132, i_055132),
    (0x055136, i_055136),
    (0x05513A, i_05513a),
    (0x05513E, i_05513e),
    (0x055142, i_055142),
    (0x055146, i_055146),
    (0x055148, i_055148),
    (0x05514C, i_05514c),
    (0x055150, i_055150),
    (0x055154, i_055154),
    (0x055158, i_055158),
    (0x05515E, i_05515e),
    (0x055162, i_055162),
    (0x055166, i_055166),
    (0x05516A, i_05516a),
    (0x05516E, i_05516e),
    (0x055174, i_055174),
    (0x055178, i_055178),
    (0x05517C, i_05517c),
    (0x055180, i_055180),
    (0x055184, i_055184),
    (0x05518A, i_05518a),
    (0x05518C, i_05518c),
    (0x055190, i_055190),
    (0x055194, i_055194),
    (0x055198, i_055198),
    (0x05519C, i_05519c),
    (0x0551A2, i_0551a2),
    (0x0551A6, i_0551a6),
    (0x0551AA, i_0551aa),
    (0x0551AE, i_0551ae),
    (0x0551B2, i_0551b2),
    (0x0551B8, i_0551b8),
    (0x0551BC, i_0551bc),
    (0x0551C0, i_0551c0),
    (0x0551C4, i_0551c4),
    (0x0551C8, i_0551c8),
    (0x0551CE, i_0551ce),
    (0x0551D0, i_0551d0),
    (0x0551D4, i_0551d4),
    (0x0551D8, i_0551d8),
    (0x0551DC, i_0551dc),
    (0x0551E0, i_0551e0),
    (0x0551E6, i_0551e6),
    (0x0551EA, i_0551ea),
    (0x0551EE, i_0551ee),
    (0x0551F2, i_0551f2),
    (0x0551F6, i_0551f6),
    (0x0551FC, i_0551fc),
    (0x055200, i_055200),
    (0x055204, i_055204),
    (0x055208, i_055208),
    (0x05520C, i_05520c),
    (0x055212, i_055212),
    (0x055214, i_055214),
    (0x055218, i_055218),
    (0x05521C, i_05521c),
    (0x055220, i_055220),
    (0x055224, i_055224),
    (0x05522A, i_05522a),
    (0x05522E, i_05522e),
    (0x055232, i_055232),
    (0x055236, i_055236),
    (0x05523A, i_05523a),
    (0x055240, i_055240),
    (0x055244, i_055244),
    (0x055248, i_055248),
    (0x05524C, i_05524c),
    (0x055250, i_055250),
    (0x055256, i_055256),
    (0x055258, i_055258),
    (0x05525E, i_05525e),
    (0x055264, i_055264),
    (0x055266, i_055266),
    (0x05526C, i_05526c),
    (0x055272, i_055272),
    (0x055274, i_055274),
    (0x05527A, i_05527a),
    (0x055280, i_055280),
    (0x055282, i_055282),
    (0x055288, i_055288),
    (0x05528E, i_05528e),
    (0x055290, i_055290),
    (0x055296, i_055296),
    (0x05529C, i_05529c),
    (0x05529E, i_05529e),
    (0x0552A4, i_0552a4),
    (0x0552AA, i_0552aa),
    (0x0552AC, i_0552ac),
    (0x0552B2, i_0552b2),
    (0x0552B8, i_0552b8),
    (0x0552BA, i_0552ba),
    (0x0552C0, i_0552c0),
    (0x0552C6, i_0552c6),
    (0x0552C8, i_0552c8),
    (0x0552CE, i_0552ce),
    (0x0552D4, i_0552d4),
    (0x0552D6, i_0552d6),
    (0x0552DC, i_0552dc),
];
