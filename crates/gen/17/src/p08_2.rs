// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $083448: jsr $267e2.l
pub fn i_083448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8344E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8344E);
    cpu.pc = 0x267E2;
    16
}
/// $08344E: move.w #$80, d0
pub fn i_08344e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83452;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083452: cmpi.w #$a0, $22(a6)
pub fn i_083452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83458;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $083458: blt.w $83464
pub fn i_083458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8345C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x83464; return 10; }
    8
}
/// $08345C: neg.w d0
pub fn i_08345c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8345E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08345E: bset.b #$0, $3a(a6)
pub fn i_08345e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83464;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $083464: move.w d0, $28(a6)
pub fn i_083464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83468;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083468: lea.l $2e6cdc.l, a0
pub fn i_083468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8346E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6CDC;
    20
}
/// $08346E: jsr $28cd4.l
pub fn i_08346e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83474;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83474);
    cpu.pc = 0x28CD4;
    16
}
/// $083474: lea.l $ffff.w, a0
pub fn i_083474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83478;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $083478: move.l a0, $48(a6)
pub fn i_083478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8347C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08347C: lea.l $83482(pc), a1
pub fn i_08347c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83480;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83482;
    16
}
/// $083480: move.l a1, (a6)
pub fn i_083480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83482;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083482: jsr $86050(pc)
pub fn i_083482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83486;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83486);
    cpu.pc = 0x86050;
    16
}
/// $083486: jsr $28d70.l
pub fn i_083486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8348C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8348C);
    cpu.pc = 0x28D70;
    16
}
/// $08348C: movea.l #$ffffffff, a0
pub fn i_08348c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83492;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $083492: jsr $5dd5c.l
pub fn i_083492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83498);
    cpu.pc = 0x5DD5C;
    16
}
/// $083498: bcc.w $834a2
pub fn i_083498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8349C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x834A2; return 10; }
    8
}
/// $08349C: jmp $518.l
pub fn i_08349c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0834A2: rts
pub fn i_0834a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0834a4  ($0834A4, 170 B, enemies/bosses/miniboss_module_083262.s)
/// $0834A4: movea.l $3c(a6), a1
pub fn i_0834a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834A8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0834A8: jsr $2942a.l
pub fn i_0834a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x834AE);
    cpu.pc = 0x2942A;
    16
}
/// $0834AE: move.w #$a7, d1
pub fn i_0834ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834B2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0834B2: jsr $236e.l
pub fn i_0834b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x834B8);
    cpu.pc = 0x236E;
    16
}
/// $0834B8: move.w #$30, $70(a6)
pub fn i_0834b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0834BE: move.b #$ff, $32(a6)
pub fn i_0834be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0834C4: move.b #$ff, $33(a6)
pub fn i_0834c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0834CA: move.b #$0, $3a(a6)
pub fn i_0834ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834D0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0834D0: move.w #$1e, $66(a6)
pub fn i_0834d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0834D6: lea.l $2e6c00.l, a0
pub fn i_0834d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6C00;
    20
}
/// $0834DC: jsr $28cd4.l
pub fn i_0834dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x834E2);
    cpu.pc = 0x28CD4;
    16
}
/// $0834E2: lea.l $834e8(pc), a1
pub fn i_0834e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x834E8;
    16
}
/// $0834E6: move.l a1, (a6)
pub fn i_0834e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0834E8: clr.b $10e39a.l
pub fn i_0834e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834EE;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0834EE: jsr $2783a.l
pub fn i_0834ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x834F4);
    cpu.pc = 0x2783A;
    16
}
/// $0834F4: jsr $28d70.l
pub fn i_0834f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x834FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x834FA);
    cpu.pc = 0x28D70;
    16
}
/// $0834FA: jsr $2870a.l
pub fn i_0834fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83500;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83500);
    cpu.pc = 0x2870A;
    16
}
/// $083500: bcc.w $83516
pub fn i_083500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83504;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83516; return 10; }
    8
}
/// $083504: lea.l $5e766.l, a0
pub fn i_083504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8350A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08350A: jsr $5e770.l
pub fn i_08350a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83510);
    cpu.pc = 0x5E770;
    16
}
/// $083510: bclr.b #$3, $13(a6)
pub fn i_083510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83516;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $083516: jsr $28758.l
pub fn i_083516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8351C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8351C);
    cpu.pc = 0x28758;
    16
}
/// $08351C: bcc.w $8353c
pub fn i_08351c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83520;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8353C; return 10; }
    8
}
/// $083520: lea.l $2eac8c.l, a1
pub fn i_083520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83526;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EAC8C;
    20
}
/// $083526: jsr $43fac.l
pub fn i_083526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8352C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8352C);
    cpu.pc = 0x43FAC;
    16
}
/// $08352C: move.w #$1037, d0
pub fn i_08352c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83530;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1037;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083530: jsr $2352.l
pub fn i_083530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83536;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83536);
    cpu.pc = 0x2352;
    16
}
/// $083536: lea.l $8354e(pc), a1
pub fn i_083536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8353A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8354E;
    16
}
/// $08353A: move.l a1, (a6)
pub fn i_08353a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8353C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08353C: jsr $4fa70.l
pub fn i_08353c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83542;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83542);
    cpu.pc = 0x4FA70;
    16
}
/// $083542: bcc.w $8354c
pub fn i_083542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83546;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8354C; return 10; }
    8
}
/// $083546: jmp $518.l
pub fn i_083546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8354C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08354C: rts
pub fn i_08354c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8354E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08354e  ($08354E, 72 B, enemies/bosses/miniboss_module_083262.s)
/// $08354E: move.l #$100, d0
pub fn i_08354e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83554;
    cpu.ir = 0x203C;
    let v1: u32 = 0x100;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $083554: jsr $51a28.l
pub fn i_083554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8355A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8355A);
    cpu.pc = 0x51A28;
    16
}
/// $08355A: lea.l $ffff.w, a0
pub fn i_08355a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8355E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08355E: move.l a0, $48(a6)
pub fn i_08355e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83562;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $083562: lea.l $2e6c16.l, a0
pub fn i_083562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83568;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6C16;
    20
}
/// $083568: jsr $28cd4.l
pub fn i_083568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8356E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8356E);
    cpu.pc = 0x28CD4;
    16
}
/// $08356E: lea.l $83574(pc), a1
pub fn i_08356e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83572;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83574;
    16
}
/// $083572: move.l a1, (a6)
pub fn i_083572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83574;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083574: jsr $2783a.l
pub fn i_083574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8357A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8357A);
    cpu.pc = 0x2783A;
    16
}
/// $08357A: jsr $28d70.l
pub fn i_08357a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83580;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83580);
    cpu.pc = 0x28D70;
    16
}
/// $083580: bcs.w $8358e
pub fn i_083580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83584;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8358E; return 10; }
    8
}
/// $083584: jsr $4fa70.l
pub fn i_083584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8358A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8358A);
    cpu.pc = 0x4FA70;
    16
}
/// $08358A: bcc.w $83594
pub fn i_08358a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8358E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83594; return 10; }
    8
}
/// $08358E: jmp $518.l
pub fn i_08358e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83594;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083594: rts
pub fn i_083594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83596;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083596  ($083596, 92 B, enemies/bosses/miniboss_module_083262.s)
/// $083596: movea.l $3c(a6), a1
pub fn i_083596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8359A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08359A: jsr $2942a.l
pub fn i_08359a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835A0);
    cpu.pc = 0x2942A;
    16
}
/// $0835A0: move.w #$ab, d1
pub fn i_0835a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835A4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0835A4: jsr $236e.l
pub fn i_0835a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835AA);
    cpu.pc = 0x236E;
    16
}
/// $0835AA: move.w #$30, $70(a6)
pub fn i_0835aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0835B0: move.b #$ff, $32(a6)
pub fn i_0835b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835B6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0835B6: move.b #$ff, $33(a6)
pub fn i_0835b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835BC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0835BC: move.b #$0, $3a(a6)
pub fn i_0835bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835C2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0835C2: lea.l $2e6d28.l, a0
pub fn i_0835c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6D28;
    20
}
/// $0835C8: jsr $28cd4.l
pub fn i_0835c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835CE);
    cpu.pc = 0x28CD4;
    16
}
/// $0835CE: lea.l $835d4(pc), a1
pub fn i_0835ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x835D4;
    16
}
/// $0835D2: move.l a1, (a6)
pub fn i_0835d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0835D4: jsr $2783a.l
pub fn i_0835d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835DA);
    cpu.pc = 0x2783A;
    16
}
/// $0835DA: jsr $28d70.l
pub fn i_0835da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835E0);
    cpu.pc = 0x28D70;
    16
}
/// $0835E0: jsr $4fa70.l
pub fn i_0835e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835E6);
    cpu.pc = 0x4FA70;
    16
}
/// $0835E6: bcc.w $835f0
pub fn i_0835e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x835F0; return 10; }
    8
}
/// $0835EA: jmp $518.l
pub fn i_0835ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835F0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0835F0: rts
pub fn i_0835f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0835f2  ($0835F2, 32 B, enemies/bosses/miniboss_module_083262.s)
/// $0835F2: movea.l $3c(a6), a1
pub fn i_0835f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835F6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0835F6: jsr $2942a.l
pub fn i_0835f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x835FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x835FC);
    cpu.pc = 0x2942A;
    16
}
/// $0835FC: move.b #$0, $21(a6)
pub fn i_0835fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83602;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083602: lea.l $2e6d38.l, a0
pub fn i_083602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83608;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6D38;
    20
}
/// $083608: jsr $28cd4.l
pub fn i_083608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8360E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8360E);
    cpu.pc = 0x28CD4;
    16
}
/// $08360E: bra.w $8364e
pub fn i_08360e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83612;
    cpu.ir = 0x6000;
    cpu.pc = 0x8364E;
    10
}
// ===== TaskHandler_083612  ($083612, 32 B, enemies/bosses/miniboss_module_083262.s)
/// $083612: movea.l $3c(a6), a1
pub fn i_083612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83616;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083616: jsr $2942a.l
pub fn i_083616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8361C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8361C);
    cpu.pc = 0x2942A;
    16
}
/// $08361C: move.b #$1, $21(a6)
pub fn i_08361c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83622;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083622: lea.l $2e6da4.l, a0
pub fn i_083622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83628;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6DA4;
    20
}
/// $083628: jsr $28cd4.l
pub fn i_083628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8362E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8362E);
    cpu.pc = 0x28CD4;
    16
}
/// $08362E: bra.w $8364e
pub fn i_08362e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83632;
    cpu.ir = 0x6000;
    cpu.pc = 0x8364E;
    10
}
// ===== TaskHandler_083632  ($083632, 226 B, enemies/bosses/miniboss_module_083262.s)
/// $083632: movea.l $3c(a6), a1
pub fn i_083632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83636;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083636: jsr $2942a.l
pub fn i_083636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8363C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8363C);
    cpu.pc = 0x2942A;
    16
}
/// $08363C: move.b #$2, $21(a6)
pub fn i_08363c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83642;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083642: lea.l $2e6da4.l, a0
pub fn i_083642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83648;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6DA4;
    20
}
/// $083648: jsr $28cd4.l
pub fn i_083648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8364E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8364E);
    cpu.pc = 0x28CD4;
    16
}
/// $08364E: move.w #$aa, d1
pub fn i_08364e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83652;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083652: jsr $236e.l
pub fn i_083652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83658;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83658);
    cpu.pc = 0x236E;
    16
}
/// $083658: move.b #$0, $3a(a6)
pub fn i_083658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8365E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08365E: move.w $70(a6), d0
pub fn i_08365e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83662;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $083662: or.b d0, $3a(a6)
pub fn i_083662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83666;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $083666: asr.w #$8, d0
pub fn i_083666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83668;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $083668: move.b d0, $20(a6)
pub fn i_083668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8366C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08366C: move.w #$30, $70(a6)
pub fn i_08366c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83672;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083672: move.b #$ff, $32(a6)
pub fn i_083672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83678;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083678: move.b #$ff, $33(a6)
pub fn i_083678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8367E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08367E: move.w #$3c, $66(a6)
pub fn i_08367e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83684;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083684: move.w #$8000, $38(a6)
pub fn i_083684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8368A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08368A: jsr $267e2.l
pub fn i_08368a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83690;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83690);
    cpu.pc = 0x267E2;
    16
}
/// $083690: jsr $27cee.l
pub fn i_083690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83696;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83696);
    cpu.pc = 0x27CEE;
    16
}
/// $083696: btst.b #$0, $3a(a6)
pub fn i_083696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8369C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08369C: beq.w $836a6
pub fn i_08369c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x836A6; return 10; }
    8
}
/// $0836A0: addi.w #$20, $22(a6)
pub fn i_0836a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836A6;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0836A6: cmpi.b #$2, $21(a6)
pub fn i_0836a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836AC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $0836AC: bne.w $836c0
pub fn i_0836ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836B0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x836C0; return 10; }
    8
}
/// $0836B0: lea.l $2e9698.l, a0
pub fn i_0836b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9698;
    20
}
/// $0836B6: move.l a0, $4c(a6)
pub fn i_0836b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836BA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0836BA: jsr $283ca.l
pub fn i_0836ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x836C0);
    cpu.pc = 0x283CA;
    16
}
/// $0836C0: lea.l $836c6(pc), a1
pub fn i_0836c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x836C6;
    16
}
/// $0836C4: move.l a1, (a6)
pub fn i_0836c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0836C6: jsr $2783a.l
pub fn i_0836c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x836CC);
    cpu.pc = 0x2783A;
    16
}
/// $0836CC: jsr $28d70.l
pub fn i_0836cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x836D2);
    cpu.pc = 0x28D70;
    16
}
/// $0836D2: jsr $863be(pc)
pub fn i_0836d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x836D6);
    cpu.pc = 0x863BE;
    16
}
/// $0836D6: jsr $2870a.l
pub fn i_0836d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x836DC);
    cpu.pc = 0x2870A;
    16
}
/// $0836DC: bcc.w $836f2
pub fn i_0836dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x836F2; return 10; }
    8
}
/// $0836E0: lea.l $5e766.l, a0
pub fn i_0836e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836E6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0836E6: jsr $5e770.l
pub fn i_0836e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x836EC);
    cpu.pc = 0x5E770;
    16
}
/// $0836EC: bclr.b #$3, $13(a6)
pub fn i_0836ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836F2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0836F2: cmpi.w #$28, $66(a6)
pub fn i_0836f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836F8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x28, Size::Word, false);
    16
}
/// $0836F8: bgt.w $83702
pub fn i_0836f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x836FC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83702; return 10; }
    8
}
/// $0836FC: lea.l $83714(pc), a1
pub fn i_0836fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83700;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83714;
    16
}
/// $083700: move.l a1, (a6)
pub fn i_083700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83702;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083702: jsr $4fa70.l
pub fn i_083702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83708);
    cpu.pc = 0x4FA70;
    16
}
/// $083708: bcc.w $83712
pub fn i_083708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8370C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83712; return 10; }
    8
}
/// $08370C: jmp $518.l
pub fn i_08370c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83712;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083712: rts
pub fn i_083712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83714;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083714  ($083714, 118 B, enemies/bosses/miniboss_module_083262.s)
/// $083714: lea.l $2e6d4e.l, a0
pub fn i_083714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8371A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6D4E;
    20
}
/// $08371A: jsr $28cd4.l
pub fn i_08371a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83720;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83720);
    cpu.pc = 0x28CD4;
    16
}
/// $083720: cmpi.b #$0, $21(a6)
pub fn i_083720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83726;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $083726: beq.w $83736
pub fn i_083726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8372A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x83736; return 10; }
    8
}
/// $08372A: lea.l $2e6dba.l, a0
pub fn i_08372a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83730;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6DBA;
    20
}
/// $083730: jsr $28cd4.l
pub fn i_083730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83736;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83736);
    cpu.pc = 0x28CD4;
    16
}
/// $083736: lea.l $8373c(pc), a1
pub fn i_083736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8373A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8373C;
    16
}
/// $08373A: move.l a1, (a6)
pub fn i_08373a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8373C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08373C: jsr $2783a.l
pub fn i_08373c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83742;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83742);
    cpu.pc = 0x2783A;
    16
}
/// $083742: jsr $28d70.l
pub fn i_083742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83748;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83748);
    cpu.pc = 0x28D70;
    16
}
/// $083748: jsr $863be(pc)
pub fn i_083748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8374C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8374C);
    cpu.pc = 0x863BE;
    16
}
/// $08374C: jsr $2870a.l
pub fn i_08374c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83752;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83752);
    cpu.pc = 0x2870A;
    16
}
/// $083752: bcc.w $83768
pub fn i_083752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83756;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83768; return 10; }
    8
}
/// $083756: lea.l $5e766.l, a0
pub fn i_083756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8375C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08375C: jsr $5e770.l
pub fn i_08375c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83762;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83762);
    cpu.pc = 0x5E770;
    16
}
/// $083762: bclr.b #$3, $13(a6)
pub fn i_083762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83768;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $083768: cmpi.w #$14, $66(a6)
pub fn i_083768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8376E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Word, false);
    16
}
/// $08376E: bgt.w $83778
pub fn i_08376e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83772;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83778; return 10; }
    8
}
/// $083772: lea.l $8378a(pc), a1
pub fn i_083772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83776;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8378A;
    16
}
/// $083776: move.l a1, (a6)
pub fn i_083776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83778;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083778: jsr $4fa70.l
pub fn i_083778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8377E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8377E);
    cpu.pc = 0x4FA70;
    16
}
/// $08377E: bcc.w $83788
pub fn i_08377e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83782;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83788; return 10; }
    8
}
/// $083782: jmp $518.l
pub fn i_083782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83788;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083788: rts
pub fn i_083788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8378A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08378a  ($08378A, 118 B, enemies/bosses/miniboss_module_083262.s)
/// $08378A: lea.l $2e6d6a.l, a0
pub fn i_08378a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83790;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6D6A;
    20
}
/// $083790: jsr $28cd4.l
pub fn i_083790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83796;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83796);
    cpu.pc = 0x28CD4;
    16
}
/// $083796: cmpi.b #$0, $21(a6)
pub fn i_083796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8379C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08379C: beq.w $837ac
pub fn i_08379c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837A0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x837AC; return 10; }
    8
}
/// $0837A0: lea.l $2e6dd6.l, a0
pub fn i_0837a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6DD6;
    20
}
/// $0837A6: jsr $28cd4.l
pub fn i_0837a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837AC);
    cpu.pc = 0x28CD4;
    16
}
/// $0837AC: lea.l $837b2(pc), a1
pub fn i_0837ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837B0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x837B2;
    16
}
/// $0837B0: move.l a1, (a6)
pub fn i_0837b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837B2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0837B2: jsr $2783a.l
pub fn i_0837b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837B8);
    cpu.pc = 0x2783A;
    16
}
/// $0837B8: jsr $28d70.l
pub fn i_0837b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837BE);
    cpu.pc = 0x28D70;
    16
}
/// $0837BE: jsr $863be(pc)
pub fn i_0837be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x837C2);
    cpu.pc = 0x863BE;
    16
}
/// $0837C2: jsr $2870a.l
pub fn i_0837c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837C8);
    cpu.pc = 0x2870A;
    16
}
/// $0837C8: bcc.w $837de
pub fn i_0837c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x837DE; return 10; }
    8
}
/// $0837CC: lea.l $5e766.l, a0
pub fn i_0837cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0837D2: jsr $5e770.l
pub fn i_0837d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837D8);
    cpu.pc = 0x5E770;
    16
}
/// $0837D8: bclr.b #$3, $13(a6)
pub fn i_0837d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837DE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0837DE: jsr $28758.l
pub fn i_0837de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837E4);
    cpu.pc = 0x28758;
    16
}
/// $0837E4: bcc.w $837ee
pub fn i_0837e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x837EE; return 10; }
    8
}
/// $0837E8: lea.l $83800(pc), a1
pub fn i_0837e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83800;
    16
}
/// $0837EC: move.l a1, (a6)
pub fn i_0837ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0837EE: jsr $4fa70.l
pub fn i_0837ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x837F4);
    cpu.pc = 0x4FA70;
    16
}
/// $0837F4: bcc.w $837fe
pub fn i_0837f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x837FE; return 10; }
    8
}
/// $0837F8: jmp $518.l
pub fn i_0837f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x837FE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0837FE: rts
pub fn i_0837fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83800;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083800  ($083800, 294 B, enemies/bosses/miniboss_module_083262.s)
/// $083800: jsr $2783a.l
pub fn i_083800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83806);
    cpu.pc = 0x2783A;
    16
}
/// $083806: move.l #$2000, d0
pub fn i_083806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8380C;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08380C: jsr $51a28.l
pub fn i_08380c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83812;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83812);
    cpu.pc = 0x51A28;
    16
}
/// $083812: jsr $85fb0(pc)
pub fn i_083812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83816;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83816);
    cpu.pc = 0x85FB0;
    16
}
/// $083816: lea.l $ffff.w, a0
pub fn i_083816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8381A;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08381A: move.l a0, $4c(a6)
pub fn i_08381a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8381E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08381E: jsr $283ca.l
pub fn i_08381e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83824;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83824);
    cpu.pc = 0x283CA;
    16
}
/// $083824: lea.l $ffff.w, a0
pub fn i_083824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83828;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $083828: move.l a0, $48(a6)
pub fn i_083828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8382C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08382C: cmpi.w #$170, $24(a6)
pub fn i_08382c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83832;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x170, Size::Word, false);
    16
}
/// $083832: bgt.w $83852
pub fn i_083832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83836;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83852; return 10; }
    8
}
/// $083836: lea.l $2e9a1e.l, a1
pub fn i_083836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8383C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9A1E;
    20
}
/// $08383C: cmpi.b #$0, $21(a6)
pub fn i_08383c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83842;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $083842: beq.w $8384c
pub fn i_083842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83846;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8384C; return 10; }
    8
}
/// $083846: lea.l $2e9a30.l, a1
pub fn i_083846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8384C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9A30;
    20
}
/// $08384C: jsr $77c7e.l
pub fn i_08384c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83852);
    cpu.pc = 0x77C7E;
    16
}
/// $083852: lea.l $2e6d86.l, a0
pub fn i_083852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83858;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6D86;
    20
}
/// $083858: jsr $28cd4.l
pub fn i_083858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8385E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8385E);
    cpu.pc = 0x28CD4;
    16
}
/// $08385E: lea.l $2eaca0.l, a1
pub fn i_08385e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83864;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EACA0;
    20
}
/// $083864: cmpi.b #$0, $21(a6)
pub fn i_083864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8386A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08386A: beq.w $83880
pub fn i_08386a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8386E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x83880; return 10; }
    8
}
/// $08386E: lea.l $2e6df2.l, a0
pub fn i_08386e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83874;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6DF2;
    20
}
/// $083874: jsr $28cd4.l
pub fn i_083874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8387A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8387A);
    cpu.pc = 0x28CD4;
    16
}
/// $08387A: lea.l $2eacac.l, a1
pub fn i_08387a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83880;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EACAC;
    20
}
/// $083880: move.w $22(a6), d0
pub fn i_083880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83884;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $083884: movem.w d0, -(a7)
pub fn i_083884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83888;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $083888: btst.b #$0, $3a(a6)
pub fn i_083888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8388E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08388E: beq.w $83898
pub fn i_08388e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83892;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x83898; return 10; }
    8
}
/// $083892: subi.w #$18, $22(a6)
pub fn i_083892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83898;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $083898: cmpi.b #$1, $20(a6)
pub fn i_083898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8389E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08389E: beq.w $838a8
pub fn i_08389e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x838A8; return 10; }
    8
}
/// $0838A2: jsr $43fac.l
pub fn i_0838a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x838A8);
    cpu.pc = 0x43FAC;
    16
}
/// $0838A8: movem.w (a7)+, d0
pub fn i_0838a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838AC;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $0838AC: move.w d0, $22(a6)
pub fn i_0838ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838B0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0838B0: jsr $267e2.l
pub fn i_0838b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x838B6);
    cpu.pc = 0x267E2;
    16
}
/// $0838B6: move.w #$fffd, $2e(a6)
pub fn i_0838b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0838BC: move.w #$40, d0
pub fn i_0838bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838C0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0838C0: btst.b #$0, $3a(a6)
pub fn i_0838c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838C6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0838C6: beq.w $838cc
pub fn i_0838c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838CA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x838CC; return 10; }
    8
}
/// $0838CA: neg.w d0
pub fn i_0838ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838CC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0838CC: move.w d0, $28(a6)
pub fn i_0838cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838D0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0838D0: move.w #$5a, $72(a6)
pub fn i_0838d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0838D6: move.b #$f0, d0
pub fn i_0838d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838DA;
    cpu.ir = 0x103C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0838DA: jsr $5e722.l
pub fn i_0838da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x838E0);
    cpu.pc = 0x5E722;
    16
}
/// $0838E0: lea.l $838e6(pc), a1
pub fn i_0838e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x838E6;
    16
}
/// $0838E4: move.l a1, (a6)
pub fn i_0838e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0838E6: cmpi.w #$50, $72(a6)
pub fn i_0838e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838EC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x50, Size::Word, false);
    16
}
/// $0838EC: bne.w $838fa
pub fn i_0838ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x838FA; return 10; }
    8
}
/// $0838F0: move.w #$1054, d0
pub fn i_0838f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838F4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1054;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0838F4: jsr $2352.l
pub fn i_0838f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x838FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x838FA);
    cpu.pc = 0x2352;
    16
}
/// $0838FA: jsr $27cee.l
pub fn i_0838fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83900);
    cpu.pc = 0x27CEE;
    16
}
/// $083900: jsr $28d70.l
pub fn i_083900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83906;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83906);
    cpu.pc = 0x28D70;
    16
}
/// $083906: subq.w #$1, $72(a6)
pub fn i_083906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8390A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08390A: bne.w $83914
pub fn i_08390a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8390E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x83914; return 10; }
    8
}
/// $08390E: lea.l $83926(pc), a1
pub fn i_08390e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83912;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83926;
    16
}
/// $083912: move.l a1, (a6)
pub fn i_083912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83914;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083914: jsr $4fa70.l
pub fn i_083914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8391A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8391A);
    cpu.pc = 0x4FA70;
    16
}
/// $08391A: bcc.w $83924
pub fn i_08391a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8391E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83924; return 10; }
    8
}
/// $08391E: jmp $518.l
pub fn i_08391e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83924;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083924: rts
pub fn i_083924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83926;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083926  ($083926, 68 B, enemies/bosses/miniboss_module_083262.s)
/// $083926: lea.l $2e987e.l, a1
pub fn i_083926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8392C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E987E;
    20
}
/// $08392C: jsr $77c7e.l
pub fn i_08392c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83932);
    cpu.pc = 0x77C7E;
    16
}
/// $083932: move.w #$1e, $72(a6)
pub fn i_083932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83938;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083938: jsr $267e2.l
pub fn i_083938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8393E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8393E);
    cpu.pc = 0x267E2;
    16
}
/// $08393E: lea.l $83944(pc), a1
pub fn i_08393e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83942;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83944;
    16
}
/// $083942: move.l a1, (a6)
pub fn i_083942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83944;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083944: jsr $27cee.l
pub fn i_083944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8394A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8394A);
    cpu.pc = 0x27CEE;
    16
}
/// $08394A: jsr $28d70.l
pub fn i_08394a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83950;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83950);
    cpu.pc = 0x28D70;
    16
}
/// $083950: subq.w #$1, $72(a6)
pub fn i_083950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83954;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $083954: beq.w $83962
pub fn i_083954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83958;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x83962; return 10; }
    8
}
/// $083958: jsr $4fa70.l
pub fn i_083958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8395E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8395E);
    cpu.pc = 0x4FA70;
    16
}
/// $08395E: bcc.w $83968
pub fn i_08395e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83962;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83968; return 10; }
    8
}
/// $083962: jmp $518.l
pub fn i_083962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83968;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083968: rts
pub fn i_083968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8396A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08396a  ($08396A, 120 B, enemies/bosses/miniboss_module_083262.s)
/// $08396A: movea.l $3c(a6), a1
pub fn i_08396a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8396E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08396E: jsr $2942a.l
pub fn i_08396e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83974;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83974);
    cpu.pc = 0x2942A;
    16
}
/// $083974: move.w #$a5, d1
pub fn i_083974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83978;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083978: jsr $236e.l
pub fn i_083978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8397E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8397E);
    cpu.pc = 0x236E;
    16
}
/// $08397E: move.w #$10, $70(a6)
pub fn i_08397e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83984;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083984: move.b #$ff, $32(a6)
pub fn i_083984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8398A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08398A: move.b #$ff, $33(a6)
pub fn i_08398a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83990;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083990: move.b #$0, $3a(a6)
pub fn i_083990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83996;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083996: lea.l $2e6e10.l, a0
pub fn i_083996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8399C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6E10;
    20
}
/// $08399C: jsr $28cd4.l
pub fn i_08399c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839A2);
    cpu.pc = 0x28CD4;
    16
}
/// $0839A2: bclr.b #$3, $13(a6)
pub fn i_0839a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839A8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0839A8: lea.l $839ae(pc), a1
pub fn i_0839a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x839AE;
    16
}
/// $0839AC: move.l a1, (a6)
pub fn i_0839ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0839AE: jsr $2783a.l
pub fn i_0839ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839B4);
    cpu.pc = 0x2783A;
    16
}
/// $0839B4: move.b #$1, $44(a6)
pub fn i_0839b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839BA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0839BA: jsr $28d70.l
pub fn i_0839ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839C0);
    cpu.pc = 0x28D70;
    16
}
/// $0839C0: jsr $2870a.l
pub fn i_0839c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839C6);
    cpu.pc = 0x2870A;
    16
}
/// $0839C6: bcc.w $839d0
pub fn i_0839c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x839D0; return 10; }
    8
}
/// $0839CA: lea.l $839e2(pc), a1
pub fn i_0839ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x839E2;
    16
}
/// $0839CE: move.l a1, (a6)
pub fn i_0839ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0839D0: jsr $4fa70.l
pub fn i_0839d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839D6);
    cpu.pc = 0x4FA70;
    16
}
/// $0839D6: bcc.w $839e0
pub fn i_0839d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x839E0; return 10; }
    8
}
/// $0839DA: jmp $518.l
pub fn i_0839da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839E0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0839E0: rts
pub fn i_0839e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0839e2  ($0839E2, 84 B, enemies/bosses/miniboss_module_083262.s)
/// $0839E2: move.w #$10a9, d0
pub fn i_0839e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839E6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0839E6: jsr $2352.l
pub fn i_0839e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839EC);
    cpu.pc = 0x2352;
    16
}
/// $0839EC: move.b #$0, $44(a6)
pub fn i_0839ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839F2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0839F2: move.w #$a, d0
pub fn i_0839f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839F6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0839F6: jsr $5e9e4.l
pub fn i_0839f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x839FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x839FC);
    cpu.pc = 0x5E9E4;
    16
}
/// $0839FC: addi.w #$23, d0
pub fn i_0839fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A00;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x23, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $083A00: move.w d0, $72(a6)
pub fn i_083a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A04;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083A04: lea.l $83a0a(pc), a1
pub fn i_083a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83A0A;
    16
}
/// $083A08: move.l a1, (a6)
pub fn i_083a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083A0A: jsr $2783a.l
pub fn i_083a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A10);
    cpu.pc = 0x2783A;
    16
}
/// $083A10: jsr $28d70.l
pub fn i_083a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A16);
    cpu.pc = 0x28D70;
    16
}
/// $083A16: subq.w #$1, $72(a6)
pub fn i_083a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A1A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $083A1A: bne.w $83a24
pub fn i_083a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A1E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x83A24; return 10; }
    8
}
/// $083A1E: lea.l $839a2(pc), a1
pub fn i_083a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x839A2;
    16
}
/// $083A22: move.l a1, (a6)
pub fn i_083a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083A24: jsr $4fa70.l
pub fn i_083a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A2A);
    cpu.pc = 0x4FA70;
    16
}
/// $083A2A: bcc.w $83a34
pub fn i_083a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A2E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83A34; return 10; }
    8
}
/// $083A2E: jmp $518.l
pub fn i_083a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A34;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083A34: rts
pub fn i_083a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A36;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083a36  ($083A36, 110 B, enemies/bosses/miniboss_module_083262.s)
/// $083A36: movea.l $3c(a6), a1
pub fn i_083a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A3A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083A3A: jsr $2942a.l
pub fn i_083a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A40);
    cpu.pc = 0x2942A;
    16
}
/// $083A40: move.w #$a5, d1
pub fn i_083a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A44;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083A44: jsr $236e.l
pub fn i_083a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A4A);
    cpu.pc = 0x236E;
    16
}
/// $083A4A: move.w #$0, $70(a6)
pub fn i_083a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083A50: move.b #$ff, $32(a6)
pub fn i_083a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A56;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083A56: move.b #$ff, $33(a6)
pub fn i_083a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A5C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083A5C: move.b #$0, $3a(a6)
pub fn i_083a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A62;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083A62: move.w #$8000, $38(a6)
pub fn i_083a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A68;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083A68: jsr $267e2.l
pub fn i_083a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A6E);
    cpu.pc = 0x267E2;
    16
}
/// $083A6E: jsr $27cee.l
pub fn i_083a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A74);
    cpu.pc = 0x27CEE;
    16
}
/// $083A74: lea.l $2e6ebc.l, a0
pub fn i_083a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6EBC;
    20
}
/// $083A7A: jsr $28cd4.l
pub fn i_083a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A80);
    cpu.pc = 0x28CD4;
    16
}
/// $083A80: lea.l $83a86(pc), a1
pub fn i_083a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83A86;
    16
}
/// $083A84: move.l a1, (a6)
pub fn i_083a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083A86: jsr $2783a.l
pub fn i_083a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A8C);
    cpu.pc = 0x2783A;
    16
}
/// $083A8C: jsr $28d70.l
pub fn i_083a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A92);
    cpu.pc = 0x28D70;
    16
}
/// $083A92: jsr $4fa70.l
pub fn i_083a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83A98);
    cpu.pc = 0x4FA70;
    16
}
/// $083A98: bcc.w $83aa2
pub fn i_083a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83A9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83AA2; return 10; }
    8
}
/// $083A9C: jmp $518.l
pub fn i_083a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AA2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083AA2: rts
pub fn i_083aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083aa4  ($083AA4, 110 B, enemies/bosses/miniboss_module_083262.s)
/// $083AA4: movea.l $3c(a6), a1
pub fn i_083aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AA8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083AA8: jsr $2942a.l
pub fn i_083aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83AAE);
    cpu.pc = 0x2942A;
    16
}
/// $083AAE: move.w #$a7, d1
pub fn i_083aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AB2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083AB2: jsr $236e.l
pub fn i_083ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83AB8);
    cpu.pc = 0x236E;
    16
}
/// $083AB8: move.w #$0, $70(a6)
pub fn i_083ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ABE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083ABE: move.b #$ff, $32(a6)
pub fn i_083abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AC4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083AC4: move.b #$ff, $33(a6)
pub fn i_083ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ACA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083ACA: move.b #$0, $3a(a6)
pub fn i_083aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AD0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083AD0: move.w #$8000, $38(a6)
pub fn i_083ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AD6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083AD6: jsr $267e2.l
pub fn i_083ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83ADC);
    cpu.pc = 0x267E2;
    16
}
/// $083ADC: jsr $27cee.l
pub fn i_083adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83AE2);
    cpu.pc = 0x27CEE;
    16
}
/// $083AE2: lea.l $2e6ecc.l, a0
pub fn i_083ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AE8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6ECC;
    20
}
/// $083AE8: jsr $28cd4.l
pub fn i_083ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83AEE);
    cpu.pc = 0x28CD4;
    16
}
/// $083AEE: lea.l $83af4(pc), a1
pub fn i_083aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83AF4;
    16
}
/// $083AF2: move.l a1, (a6)
pub fn i_083af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083AF4: jsr $2783a.l
pub fn i_083af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83AFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83AFA);
    cpu.pc = 0x2783A;
    16
}
/// $083AFA: jsr $28d70.l
pub fn i_083afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B00);
    cpu.pc = 0x28D70;
    16
}
/// $083B00: jsr $4fa70.l
pub fn i_083b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B06);
    cpu.pc = 0x4FA70;
    16
}
/// $083B06: bcc.w $83b10
pub fn i_083b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83B10; return 10; }
    8
}
/// $083B0A: jmp $518.l
pub fn i_083b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B10;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083B10: rts
pub fn i_083b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083b12  ($083B12, 114 B, enemies/bosses/miniboss_module_083262.s)
/// $083B12: movea.l $3c(a6), a1
pub fn i_083b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B16;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083B16: jsr $2942a.l
pub fn i_083b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B1C);
    cpu.pc = 0x2942A;
    16
}
/// $083B1C: move.w #$a7, d1
pub fn i_083b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B20;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083B20: jsr $236e.l
pub fn i_083b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B26);
    cpu.pc = 0x236E;
    16
}
/// $083B26: move.w #$100, $70(a6)
pub fn i_083b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B2C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083B2C: move.b #$ff, $32(a6)
pub fn i_083b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B32;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083B32: move.b #$ff, $33(a6)
pub fn i_083b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B38;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083B38: move.b #$0, $3a(a6)
pub fn i_083b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B3E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083B3E: lea.l $2e6edc.l, a0
pub fn i_083b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6EDC;
    20
}
/// $083B44: jsr $28cd4.l
pub fn i_083b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B4A);
    cpu.pc = 0x28CD4;
    16
}
/// $083B4A: lea.l $83b92(pc), a1
pub fn i_083b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83B92;
    16
}
/// $083B4E: jsr $4ae.l
pub fn i_083b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B54);
    cpu.pc = 0x4AE;
    16
}
/// $083B54: move.w $38(a6), $38(a0)
pub fn i_083b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B5A;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $083B5A: jsr $86076(pc)
pub fn i_083b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83B5E);
    cpu.pc = 0x86076;
    16
}
/// $083B5E: lea.l $83b64(pc), a1
pub fn i_083b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83B64;
    16
}
/// $083B62: move.l a1, (a6)
pub fn i_083b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083B64: jsr $2783a.l
pub fn i_083b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B6A);
    cpu.pc = 0x2783A;
    16
}
/// $083B6A: cmpi.w #$120, $22(a6)
pub fn i_083b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B70;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $083B70: bgt.w $83b7a
pub fn i_083b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B74;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83B7A; return 10; }
    8
}
/// $083B74: jsr $28d70.l
pub fn i_083b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B7A);
    cpu.pc = 0x28D70;
    16
}
/// $083B7A: jsr $4fa70.l
pub fn i_083b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B80);
    cpu.pc = 0x4FA70;
    16
}
/// $083B80: bcc.w $83b90
pub fn i_083b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83B90; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_083b84  ($083B84, 14 B, kernel/jsr5b6_jmp518.c)
/// $083B84: jsr $5b6.l
pub fn i_083b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B8A);
    cpu.pc = 0x5B6;
    16
}
/// $083B8A: jmp $518.l
pub fn i_083b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B90;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083B90: rts
pub fn i_083b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083b92  ($083B92, 72 B, enemies/bosses/miniboss_module_083262.s)
/// $083B92: move.w #$a7, d1
pub fn i_083b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B96;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083B96: jsr $236e.l
pub fn i_083b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83B9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83B9C);
    cpu.pc = 0x236E;
    16
}
/// $083B9C: move.b #$ff, $32(a6)
pub fn i_083b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BA2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083BA2: move.b #$ff, $33(a6)
pub fn i_083ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BA8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083BA8: move.b #$0, $3a(a6)
pub fn i_083ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BAE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083BAE: lea.l $2e6eec.l, a0
pub fn i_083bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BB4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6EEC;
    20
}
/// $083BB4: jsr $28cd4.l
pub fn i_083bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83BBA);
    cpu.pc = 0x28CD4;
    16
}
/// $083BBA: lea.l $83bc0(pc), a1
pub fn i_083bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83BC0;
    16
}
/// $083BBE: move.l a1, (a6)
pub fn i_083bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083BC0: movea.l $c(a6), a0
pub fn i_083bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BC4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $083BC4: move.w $22(a0), $22(a6)
pub fn i_083bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BCA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $083BCA: move.w $24(a0), $24(a6)
pub fn i_083bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BD0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $083BD0: cmpi.w #$ff80, $22(a6)
pub fn i_083bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BD6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $083BD6: blt.w $83be0
pub fn i_083bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BDA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x83BE0; return 10; }
    8
}
// ===== JsrAbsThunk_083bda  ($083BDA, 8 B, thunks/jsr_abs_thunks.c)
/// $083BDA: jsr $28d70.l
pub fn i_083bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83BE0);
    cpu.pc = 0x28D70;
    16
}
/// $083BE0: rts
pub fn i_083be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083be2  ($083BE2, 176 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083BE2: move.w #$166, d1
pub fn i_083be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BE6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x166;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083BE6: jsr $236e.l
pub fn i_083be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83BEC);
    cpu.pc = 0x236E;
    16
}
/// $083BEC: move.w #$c000, $38(a6)
pub fn i_083bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BF2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083BF2: move.b $98(a6), $21(a6)
pub fn i_083bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BF8;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad4, v3 as u8);
    20
}
/// $083BF8: move.b $99(a6), $3a(a6)
pub fn i_083bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83BFE;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $083BFE: bra.w $83c0c
pub fn i_083bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C02;
    cpu.ir = 0x6000;
    cpu.pc = 0x83C0C;
    10
}
/// $083C02: move.w #$aa, d1
pub fn i_083c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C06;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083C06: jsr $236e.l
pub fn i_083c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C0C);
    cpu.pc = 0x236E;
    16
}
/// $083C0C: move.b #$ff, $32(a6)
pub fn i_083c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083C12: move.b #$ff, $33(a6)
pub fn i_083c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C18;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083C18: move.w #$20, $70(a6)
pub fn i_083c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083C1E: lea.l $2e6efc.l, a0
pub fn i_083c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6EFC;
    20
}
/// $083C24: jsr $28cd4.l
pub fn i_083c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C2A);
    cpu.pc = 0x28CD4;
    16
}
/// $083C2A: btst.b #$0, $21(a6)
pub fn i_083c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C30;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $083C30: beq.w $83c40
pub fn i_083c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C34;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x83C40; return 10; }
    8
}
/// $083C34: lea.l $2e6f12.l, a0
pub fn i_083c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F12;
    20
}
/// $083C3A: jsr $28cd4.l
pub fn i_083c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C40);
    cpu.pc = 0x28CD4;
    16
}
/// $083C40: lea.l $83c46(pc), a1
pub fn i_083c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83C46;
    16
}
/// $083C44: move.l a1, (a6)
pub fn i_083c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083C46: jsr $2783a.l
pub fn i_083c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C4C);
    cpu.pc = 0x2783A;
    16
}
/// $083C4C: cmpi.w #$140, $22(a6)
pub fn i_083c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C52;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $083C52: bgt.w $83c5c
pub fn i_083c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C56;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83C5C; return 10; }
    8
}
/// $083C56: jsr $28d70.l
pub fn i_083c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C5C);
    cpu.pc = 0x28D70;
    16
}
/// $083C5C: jsr $2870a.l
pub fn i_083c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C62);
    cpu.pc = 0x2870A;
    16
}
/// $083C62: bcc.w $83c80
pub fn i_083c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83C80; return 10; }
    8
}
/// $083C66: move.w #$10a6, d0
pub fn i_083c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C6A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083C6A: jsr $2352.l
pub fn i_083c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C70);
    cpu.pc = 0x2352;
    16
}
/// $083C70: lea.l $2e9a0c.l, a1
pub fn i_083c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C76;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9A0C;
    20
}
/// $083C76: jsr $77c7e.l
pub fn i_083c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C7C);
    cpu.pc = 0x77C7E;
    16
}
/// $083C7C: bra.w $83c8a
pub fn i_083c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C80;
    cpu.ir = 0x6000;
    cpu.pc = 0x83C8A;
    10
}
/// $083C80: jsr $4fa70.l
pub fn i_083c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C86);
    cpu.pc = 0x4FA70;
    16
}
/// $083C86: bcc.w $83c90
pub fn i_083c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83C90; return 10; }
    8
}
/// $083C8A: jmp $518.l
pub fn i_083c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C90;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083C90: rts
pub fn i_083c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083c92  ($083C92, 152 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083C92: movea.l $3c(a6), a1
pub fn i_083c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C96;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083C96: jsr $2942a.l
pub fn i_083c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83C9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83C9C);
    cpu.pc = 0x2942A;
    16
}
/// $083C9C: move.w #$a6, d1
pub fn i_083c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CA0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083CA0: jsr $236e.l
pub fn i_083ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83CA6);
    cpu.pc = 0x236E;
    16
}
/// $083CA6: move.w #$30, $70(a6)
pub fn i_083ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CAC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083CAC: move.b #$ff, $32(a6)
pub fn i_083cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CB2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083CB2: move.b #$ff, $33(a6)
pub fn i_083cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CB8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083CB8: move.b #$0, $3a(a6)
pub fn i_083cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CBE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083CBE: move.w #$14, $66(a6)
pub fn i_083cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CC4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083CC4: lea.l $2e6f28.l, a0
pub fn i_083cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CCA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F28;
    20
}
/// $083CCA: jsr $28cd4.l
pub fn i_083cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83CD0);
    cpu.pc = 0x28CD4;
    16
}
/// $083CD0: lea.l $83cd6(pc), a1
pub fn i_083cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83CD6;
    16
}
/// $083CD4: move.l a1, (a6)
pub fn i_083cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CD6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083CD6: jsr $2783a.l
pub fn i_083cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83CDC);
    cpu.pc = 0x2783A;
    16
}
/// $083CDC: jsr $28d70.l
pub fn i_083cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83CE2);
    cpu.pc = 0x28D70;
    16
}
/// $083CE2: jsr $2870a.l
pub fn i_083ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83CE8);
    cpu.pc = 0x2870A;
    16
}
/// $083CE8: bcc.w $83cfe
pub fn i_083ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83CFE; return 10; }
    8
}
/// $083CEC: lea.l $5e766.l, a0
pub fn i_083cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $083CF2: jsr $5e770.l
pub fn i_083cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83CF8);
    cpu.pc = 0x5E770;
    16
}
/// $083CF8: bclr.b #$3, $13(a6)
pub fn i_083cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83CFE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $083CFE: jsr $28758.l
pub fn i_083cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D04);
    cpu.pc = 0x28758;
    16
}
/// $083D04: bcc.w $83d18
pub fn i_083d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D08;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83D18; return 10; }
    8
}
/// $083D08: move.w #$102e, d0
pub fn i_083d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D0C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083D0C: jsr $2352.l
pub fn i_083d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D12);
    cpu.pc = 0x2352;
    16
}
/// $083D12: lea.l $83d2a(pc), a1
pub fn i_083d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83D2A;
    16
}
/// $083D16: move.l a1, (a6)
pub fn i_083d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083D18: jsr $4fa70.l
pub fn i_083d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D1E);
    cpu.pc = 0x4FA70;
    16
}
/// $083D1E: bcc.w $83d28
pub fn i_083d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83D28; return 10; }
    8
}
/// $083D22: jmp $518.l
pub fn i_083d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D28;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083D28: rts
pub fn i_083d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083d2a  ($083D2A, 68 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083D2A: lea.l $ffff.w, a0
pub fn i_083d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D2E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $083D2E: move.l a0, $48(a6)
pub fn i_083d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D32;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $083D32: lea.l $2e9a94.l, a1
pub fn i_083d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D38;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9A94;
    20
}
/// $083D38: jsr $77c7e.l
pub fn i_083d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D3E);
    cpu.pc = 0x77C7E;
    16
}
/// $083D3E: lea.l $2e6f3e.l, a0
pub fn i_083d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F3E;
    20
}
/// $083D44: jsr $28cd4.l
pub fn i_083d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D4A);
    cpu.pc = 0x28CD4;
    16
}
/// $083D4A: lea.l $83d50(pc), a1
pub fn i_083d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83D50;
    16
}
/// $083D4E: move.l a1, (a6)
pub fn i_083d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D50;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083D50: jsr $2783a.l
pub fn i_083d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D56);
    cpu.pc = 0x2783A;
    16
}
/// $083D56: jsr $28d70.l
pub fn i_083d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D5C);
    cpu.pc = 0x28D70;
    16
}
/// $083D5C: jsr $4fa70.l
pub fn i_083d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D62);
    cpu.pc = 0x4FA70;
    16
}
/// $083D62: bcc.w $83d6c
pub fn i_083d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83D6C; return 10; }
    8
}
/// $083D66: jmp $518.l
pub fn i_083d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D6C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083D6C: rts
pub fn i_083d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083d6e  ($083D6E, 218 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083D6E: movea.l $3c(a6), a1
pub fn i_083d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D72;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083D72: jsr $2942a.l
pub fn i_083d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D78);
    cpu.pc = 0x2942A;
    16
}
/// $083D78: lea.l $2e6f54.l, a0
pub fn i_083d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F54;
    20
}
/// $083D7E: jsr $28cd4.l
pub fn i_083d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D84);
    cpu.pc = 0x28CD4;
    16
}
/// $083D84: move.b #$0, $21(a6)
pub fn i_083d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D8A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083D8A: bra.w $83daa
pub fn i_083d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x83DAA;
    10
}
/// $083D8E: movea.l $3c(a6), a1
pub fn i_083d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D92;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083D92: jsr $2942a.l
pub fn i_083d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83D98);
    cpu.pc = 0x2942A;
    16
}
/// $083D98: lea.l $2e6f6a.l, a0
pub fn i_083d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83D9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F6A;
    20
}
/// $083D9E: jsr $28cd4.l
pub fn i_083d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DA4);
    cpu.pc = 0x28CD4;
    16
}
/// $083DA4: move.b #$1, $21(a6)
pub fn i_083da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083DAA: move.w #$30, $70(a6)
pub fn i_083daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DB0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083DB0: move.b #$0, $3a(a6)
pub fn i_083db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DB6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083DB6: move.w #$14, $66(a6)
pub fn i_083db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DBC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083DBC: lea.l $83dc2(pc), a1
pub fn i_083dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83DC2;
    16
}
/// $083DC0: move.l a1, (a6)
pub fn i_083dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083DC2: jsr $2783a.l
pub fn i_083dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DC8);
    cpu.pc = 0x2783A;
    16
}
/// $083DC8: jsr $28d70.l
pub fn i_083dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DCE);
    cpu.pc = 0x28D70;
    16
}
/// $083DCE: jsr $2870a.l
pub fn i_083dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DD4);
    cpu.pc = 0x2870A;
    16
}
/// $083DD4: bcc.w $83dea
pub fn i_083dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DD8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83DEA; return 10; }
    8
}
/// $083DD8: lea.l $5e766.l, a0
pub fn i_083dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DDE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $083DDE: jsr $5e770.l
pub fn i_083dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DE4);
    cpu.pc = 0x5E770;
    16
}
/// $083DE4: bclr.b #$3, $13(a6)
pub fn i_083de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DEA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $083DEA: jsr $28758.l
pub fn i_083dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DF0);
    cpu.pc = 0x28758;
    16
}
/// $083DF0: bcc.w $83e36
pub fn i_083df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DF4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83E36; return 10; }
    8
}
/// $083DF4: move.w #$1027, d0
pub fn i_083df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DF8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083DF8: jsr $2352.l
pub fn i_083df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83DFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83DFE);
    cpu.pc = 0x2352;
    16
}
/// $083DFE: cmpi.b #$0, $21(a6)
pub fn i_083dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E04;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $083E04: bne.w $83e1c
pub fn i_083e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E08;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x83E1C; return 10; }
    8
}
/// $083E08: jsr $863e4(pc)
pub fn i_083e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83E0C);
    cpu.pc = 0x863E4;
    16
}
/// $083E0C: lea.l $2e9aa6.l, a1
pub fn i_083e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E12;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9AA6;
    20
}
/// $083E12: jsr $77c7e.l
pub fn i_083e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E18);
    cpu.pc = 0x77C7E;
    16
}
/// $083E18: bra.w $83e40
pub fn i_083e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x83E40;
    10
}
/// $083E1C: jsr $863f2(pc)
pub fn i_083e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83E20);
    cpu.pc = 0x863F2;
    16
}
/// $083E20: subi.w #$10, $24(a6)
pub fn i_083e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E26;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $083E26: lea.l $2e9aa6.l, a1
pub fn i_083e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E2C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9AA6;
    20
}
/// $083E2C: jsr $77c7e.l
pub fn i_083e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E32);
    cpu.pc = 0x77C7E;
    16
}
/// $083E32: bra.w $83e40
pub fn i_083e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E36;
    cpu.ir = 0x6000;
    cpu.pc = 0x83E40;
    10
}
/// $083E36: jsr $4fa70.l
pub fn i_083e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E3C);
    cpu.pc = 0x4FA70;
    16
}
/// $083E3C: bcc.w $83e46
pub fn i_083e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83E46; return 10; }
    8
}
/// $083E40: jmp $518.l
pub fn i_083e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E46;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083E46: rts
pub fn i_083e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083e48  ($083E48, 164 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083E48: movea.l $3c(a6), a1
pub fn i_083e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E4C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083E4C: jsr $2942a.l
pub fn i_083e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E52);
    cpu.pc = 0x2942A;
    16
}
/// $083E52: move.w #$89, d1
pub fn i_083e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E56;
    cpu.ir = 0x323C;
    let v1: u32 = 0x89;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083E56: jsr $236e.l
pub fn i_083e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E5C);
    cpu.pc = 0x236E;
    16
}
/// $083E5C: lea.l $2e6f80.l, a0
pub fn i_083e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E62;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F80;
    20
}
/// $083E62: jsr $28cd4.l
pub fn i_083e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E68);
    cpu.pc = 0x28CD4;
    16
}
/// $083E68: move.w #$50, $70(a6)
pub fn i_083e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E6E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083E6E: move.b #$0, $3a(a6)
pub fn i_083e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E74;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083E74: move.w #$64, $66(a6)
pub fn i_083e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E7A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083E7A: move.w #$8000, $38(a6)
pub fn i_083e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083E80: jsr $267e2.l
pub fn i_083e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E86);
    cpu.pc = 0x267E2;
    16
}
/// $083E86: jsr $27afc.l
pub fn i_083e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E8C);
    cpu.pc = 0x27AFC;
    16
}
/// $083E8C: lea.l $83e92(pc), a1
pub fn i_083e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83E92;
    16
}
/// $083E90: move.l a1, (a6)
pub fn i_083e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083E92: jsr $2783a.l
pub fn i_083e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E98);
    cpu.pc = 0x2783A;
    16
}
/// $083E98: jsr $28d70.l
pub fn i_083e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83E9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83E9E);
    cpu.pc = 0x28D70;
    16
}
/// $083E9E: jsr $2870a.l
pub fn i_083e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83EA4);
    cpu.pc = 0x2870A;
    16
}
/// $083EA4: bcc.w $83ec0
pub fn i_083ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EA8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83EC0; return 10; }
    8
}
/// $083EA8: lea.l $5e766.l, a0
pub fn i_083ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EAE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $083EAE: jsr $5e770.l
pub fn i_083eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83EB4);
    cpu.pc = 0x5E770;
    16
}
/// $083EB4: bclr.b #$3, $13(a6)
pub fn i_083eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EBA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $083EBA: lea.l $83eec(pc), a1
pub fn i_083eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83EEC;
    16
}
/// $083EBE: move.l a1, (a6)
pub fn i_083ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083EC0: jsr $28758.l
pub fn i_083ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83EC6);
    cpu.pc = 0x28758;
    16
}
/// $083EC6: bcc.w $83eda
pub fn i_083ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ECA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83EDA; return 10; }
    8
}
/// $083ECA: move.w #$1023, d0
pub fn i_083eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ECE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083ECE: jsr $2352.l
pub fn i_083ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83ED4);
    cpu.pc = 0x2352;
    16
}
/// $083ED4: lea.l $83f26(pc), a1
pub fn i_083ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83ED8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83F26;
    16
}
/// $083ED8: move.l a1, (a6)
pub fn i_083ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083EDA: jsr $4fa70.l
pub fn i_083eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83EE0);
    cpu.pc = 0x4FA70;
    16
}
/// $083EE0: bcc.w $83eea
pub fn i_083ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EE4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83EEA; return 10; }
    8
}
/// $083EE4: jmp $518.l
pub fn i_083ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EEA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083EEA: rts
pub fn i_083eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083eec  ($083EEC, 58 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083EEC: lea.l $2e6f96.l, a0
pub fn i_083eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6F96;
    20
}
/// $083EF2: jsr $28cd4.l
pub fn i_083ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83EF8);
    cpu.pc = 0x28CD4;
    16
}
/// $083EF8: lea.l $83efe(pc), a1
pub fn i_083ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83EFE;
    16
}
/// $083EFC: move.l a1, (a6)
pub fn i_083efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83EFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083EFE: jsr $2783a.l
pub fn i_083efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F04);
    cpu.pc = 0x2783A;
    16
}
/// $083F04: jsr $28d70.l
pub fn i_083f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F0A);
    cpu.pc = 0x28D70;
    16
}
/// $083F0A: bcc.w $83f14
pub fn i_083f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F0E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83F14; return 10; }
    8
}
/// $083F0E: lea.l $83e8c(pc), a1
pub fn i_083f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83E8C;
    16
}
/// $083F12: move.l a1, (a6)
pub fn i_083f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083F14: jsr $4fa70.l
pub fn i_083f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F1A);
    cpu.pc = 0x4FA70;
    16
}
/// $083F1A: bcc.w $83f24
pub fn i_083f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F1E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83F24; return 10; }
    8
}
/// $083F1E: jmp $518.l
pub fn i_083f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F24;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083F24: rts
pub fn i_083f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083f26  ($083F26, 130 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083F26: move.l #$2000, d0
pub fn i_083f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F2C;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $083F2C: jsr $51a28.l
pub fn i_083f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F32);
    cpu.pc = 0x51A28;
    16
}
/// $083F32: lea.l $ffff.w, a0
pub fn i_083f32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F36;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $083F36: move.l a0, $48(a6)
pub fn i_083f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F3A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $083F3A: lea.l $2e9890.l, a1
pub fn i_083f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F40;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9890;
    20
}
/// $083F40: jsr $77c7e.l
pub fn i_083f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F46);
    cpu.pc = 0x77C7E;
    16
}
/// $083F46: lea.l $2e9a82.l, a1
pub fn i_083f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F4C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9A82;
    20
}
/// $083F4C: jsr $77c7e.l
pub fn i_083f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F52);
    cpu.pc = 0x77C7E;
    16
}
/// $083F52: lea.l $2e6fba.l, a0
pub fn i_083f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6FBA;
    20
}
/// $083F58: jsr $28cd4.l
pub fn i_083f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F5E);
    cpu.pc = 0x28CD4;
    16
}
/// $083F5E: jsr $267e2.l
pub fn i_083f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F64);
    cpu.pc = 0x267E2;
    16
}
/// $083F64: move.w #$280, $2a(a6)
pub fn i_083f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F6A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x280;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083F6A: move.w #$ffc0, $2e(a6)
pub fn i_083f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083F70: lea.l $83f76(pc), a1
pub fn i_083f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83F76;
    16
}
/// $083F74: move.l a1, (a6)
pub fn i_083f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083F76: jsr $27cee.l
pub fn i_083f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F7C);
    cpu.pc = 0x27CEE;
    16
}
/// $083F7C: bcc.w $83f90
pub fn i_083f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83F90; return 10; }
    8
}
/// $083F80: cmpi.w #$0, $2a(a6)
pub fn i_083f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F86;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $083F86: bgt.w $83f90
pub fn i_083f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F8A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83F90; return 10; }
    8
}
/// $083F8A: jsr $267e2.l
pub fn i_083f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F90);
    cpu.pc = 0x267E2;
    16
}
/// $083F90: jsr $28d70.l
pub fn i_083f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F96);
    cpu.pc = 0x28D70;
    16
}
/// $083F96: jsr $4fa70.l
pub fn i_083f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83F9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83F9C);
    cpu.pc = 0x4FA70;
    16
}
/// $083F9C: bcc.w $83fa6
pub fn i_083f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83FA6; return 10; }
    8
}
/// $083FA0: jmp $518.l
pub fn i_083fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FA6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083FA6: rts
pub fn i_083fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083fa8  ($083FA8, 40 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083FA8: movea.l $3c(a6), a1
pub fn i_083fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FAC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $083FAC: jsr $2942a.l
pub fn i_083fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83FB2);
    cpu.pc = 0x2942A;
    16
}
/// $083FB2: move.w #$f0, $70(a6)
pub fn i_083fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FB8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083FB8: move.b #$0, $21(a6)
pub fn i_083fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FBE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083FBE: jsr $86328(pc)
pub fn i_083fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x83FC2);
    cpu.pc = 0x86328;
    16
}
/// $083FC2: lea.l $83fc8(pc), a1
pub fn i_083fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83FC8;
    16
}
/// $083FC6: move.l a1, (a6)
pub fn i_083fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083FC8: jmp $518.l
pub fn i_083fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FCE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083FCE: rts
pub fn i_083fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083fd0  ($083FD0, 18 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083FD0: jsr $2783a.l
pub fn i_083fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83FD6);
    cpu.pc = 0x2783A;
    16
}
/// $083FD6: cmpi.w #$140, $22(a6)
pub fn i_083fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FDC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $083FDC: ble.w $83fe2
pub fn i_083fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FE0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x83FE2; return 10; }
    8
}
/// $083FE0: rts
pub fn i_083fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083fe2  ($083FE2, 126 B, enemies/bosses/miniboss_finale_083be2.s)
/// $083FE2: move.w #$20, $70(a6)
pub fn i_083fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FE8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083FE8: move.w #$a, $66(a6)
pub fn i_083fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083FEE: lea.l $2e6fca.l, a0
pub fn i_083fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6FCA;
    20
}
/// $083FF4: jsr $28cd4.l
pub fn i_083ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83FFA);
    cpu.pc = 0x28CD4;
    16
}
/// $083FFA: lea.l $84000(pc), a1
pub fn i_083ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83FFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84000;
    16
}
/// $083FFE: move.l a1, (a6)
pub fn i_083ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84000;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084000: jsr $2783a.l
pub fn i_084000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84006;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84006);
    cpu.pc = 0x2783A;
    16
}
/// $084006: jsr $28d70.l
pub fn i_084006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8400C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8400C);
    cpu.pc = 0x28D70;
    16
}
/// $08400C: jsr $2870a.l
pub fn i_08400c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84012;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84012);
    cpu.pc = 0x2870A;
    16
}
/// $084012: bcc.w $84028
pub fn i_084012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84016;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84028; return 10; }
    8
}
/// $084016: lea.l $5e766.l, a0
pub fn i_084016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8401C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08401C: jsr $5e798.l
pub fn i_08401c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84022;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84022);
    cpu.pc = 0x5E798;
    16
}
/// $084022: bclr.b #$3, $13(a6)
pub fn i_084022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84028;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084028: jsr $28758.l
pub fn i_084028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8402E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8402E);
    cpu.pc = 0x28758;
    16
}
/// $08402E: bcc.w $8404e
pub fn i_08402e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84032;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8404E; return 10; }
    8
}
/// $084032: lea.l $2eacb8.l, a1
pub fn i_084032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84038;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EACB8;
    20
}
/// $084038: jsr $43fac.l
pub fn i_084038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8403E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8403E);
    cpu.pc = 0x43FAC;
    16
}
/// $08403E: move.w #$1030, d0
pub fn i_08403e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84042;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084042: jsr $2352.l
pub fn i_084042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84048;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84048);
    cpu.pc = 0x2352;
    16
}
/// $084048: lea.l $84060(pc), a1
pub fn i_084048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8404C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84060;
    16
}
/// $08404C: move.l a1, (a6)
pub fn i_08404c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8404E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08404E: jsr $4fa70.l
pub fn i_08404e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84054;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84054);
    cpu.pc = 0x4FA70;
    16
}
/// $084054: bcc.w $8405e
pub fn i_084054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84058;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8405E; return 10; }
    8
}
/// $084058: jmp $518.l
pub fn i_084058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8405E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08405E: rts
pub fn i_08405e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84060;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084060  ($084060, 236 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084060: move.l #$500, d0
pub fn i_084060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84066;
    cpu.ir = 0x203C;
    let v1: u32 = 0x500;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $084066: jsr $51a28.l
pub fn i_084066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8406C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8406C);
    cpu.pc = 0x51A28;
    16
}
/// $08406C: lea.l $ffff.w, a0
pub fn i_08406c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84070;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084070: move.l a0, $48(a6)
pub fn i_084070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84074;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084074: move.b $21(a6), d0
pub fn i_084074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84078;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084078: movea.l $c(a6), a0
pub fn i_084078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8407C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08407C: bset.b d0, $21(a0)
pub fn i_08407c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84080;
    cpu.ir = 0x1E8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084080: lea.l $2e9aca.l, a1
pub fn i_084080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84086;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9ACA;
    20
}
/// $084086: jsr $77c7e.l
pub fn i_084086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8408C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8408C);
    cpu.pc = 0x77C7E;
    16
}
/// $08408C: lea.l $2eaa60.l, a0
pub fn i_08408c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84092;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAA60;
    20
}
/// $084092: clr.l d0
pub fn i_084092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84094;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $084094: move.b $21(a6), d0
pub fn i_084094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84098;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084098: asl.l #$4, d0
pub fn i_084098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8409A;
    cpu.ir = 0xE980;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, true);
    cpu.d[0] = r;
    14
}
/// $08409A: movea.l (a0, d0.w), a2
pub fn i_08409a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8409E;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $08409E: movem.l a0, -(a7)
pub fn i_08409e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840A2;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0840A2: movem.l d0, -(a7)
pub fn i_0840a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840A6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0840A6: jsr $5022a.l
pub fn i_0840a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x840AC);
    cpu.pc = 0x5022A;
    16
}
/// $0840AC: movem.l (a7)+, d0
pub fn i_0840ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840B0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0840B0: movem.l (a7)+, a0
pub fn i_0840b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840B4;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0840B4: movea.l $8(a0, d0.w), a2
pub fn i_0840b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840B8;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x8).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $0840B8: jsr $5022a.l
pub fn i_0840b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x840BE);
    cpu.pc = 0x5022A;
    16
}
/// $0840BE: move.b #$0, $20(a6)
pub fn i_0840be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0840C4: lea.l $840ca(pc), a1
pub fn i_0840c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x840CA;
    16
}
/// $0840C8: move.l a1, (a6)
pub fn i_0840c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0840CA: btst.b #$4, $20(a6)
pub fn i_0840ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840D0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0840D0: bne.w $84102
pub fn i_0840d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840D4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84102; return 10; }
    8
}
/// $0840D4: move.b $21(a6), d0
pub fn i_0840d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840D8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0840D8: subq.b #$1, d0
pub fn i_0840d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840DA;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0840DA: cmpi.b #$0, d0
pub fn i_0840da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840DE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0840DE: bge.w $840ec
pub fn i_0840de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840E2;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x840EC; return 10; }
    8
}
/// $0840E2: bset.b #$4, $20(a6)
pub fn i_0840e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840E8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0840E8: bra.w $84102
pub fn i_0840e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x84102;
    10
}
/// $0840EC: movea.l $c(a6), a0
pub fn i_0840ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840F0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0840F0: btst.l d0, $21(a0)
pub fn i_0840f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840F4;
    cpu.ir = 0x128;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0840F4: beq.w $84102
pub fn i_0840f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840F8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84102; return 10; }
    8
}
/// $0840F8: jsr $864b6(pc)
pub fn i_0840f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x840FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x840FC);
    cpu.pc = 0x864B6;
    16
}
/// $0840FC: bset.b #$4, $20(a6)
pub fn i_0840fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84102;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084102: btst.b #$0, $20(a6)
pub fn i_084102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84108;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $084108: bne.w $8413a
pub fn i_084108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8410C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8413A; return 10; }
    8
}
/// $08410C: move.b $21(a6), d0
pub fn i_08410c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84110;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084110: addq.b #$1, d0
pub fn i_084110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84112;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $084112: cmpi.b #$7, d0
pub fn i_084112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84116;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x7, Size::Byte, false);
    8
}
/// $084116: ble.w $84124
pub fn i_084116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8411A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x84124; return 10; }
    8
}
/// $08411A: bset.b #$0, $20(a6)
pub fn i_08411a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84120;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084120: bra.w $8413a
pub fn i_084120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84124;
    cpu.ir = 0x6000;
    cpu.pc = 0x8413A;
    10
}
/// $084124: movea.l $c(a6), a0
pub fn i_084124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84128;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084128: btst.l d0, $21(a0)
pub fn i_084128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8412C;
    cpu.ir = 0x128;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08412C: beq.w $8413a
pub fn i_08412c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84130;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8413A; return 10; }
    8
}
/// $084130: jsr $864d0(pc)
pub fn i_084130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84134;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84134);
    cpu.pc = 0x864D0;
    16
}
/// $084134: bset.b #$0, $20(a6)
pub fn i_084134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8413A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08413A: cmpi.b #$11, $21(a6)
pub fn i_08413a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84140;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x11, Size::Byte, false);
    16
}
/// $084140: bne.w $8414a
pub fn i_084140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84144;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8414A; return 10; }
    8
}
/// $084144: jmp $518.l
pub fn i_084144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8414A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08414A: rts
pub fn i_08414a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8414C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08414c  ($08414C, 114 B, enemies/bosses/miniboss_finale_083be2.s)
/// $08414C: movea.l $3c(a6), a1
pub fn i_08414c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84150;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $084150: jsr $2942a.l
pub fn i_084150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84156;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84156);
    cpu.pc = 0x2942A;
    16
}
/// $084156: move.w #$a4, d1
pub fn i_084156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8415A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08415A: jsr $236e.l
pub fn i_08415a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84160;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84160);
    cpu.pc = 0x236E;
    16
}
/// $084160: move.w #$60, $70(a6)
pub fn i_084160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84166;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084166: move.b #$ff, $32(a6)
pub fn i_084166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8416C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08416C: move.b #$ff, $33(a6)
pub fn i_08416c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84172;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084172: move.b #$0, $3a(a6)
pub fn i_084172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84178;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084178: move.w #$1, $66(a6)
pub fn i_084178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8417E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08417E: lea.l $2e6fe0.l, a0
pub fn i_08417e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84184;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6FE0;
    20
}
/// $084184: jsr $28cd4.l
pub fn i_084184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8418A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8418A);
    cpu.pc = 0x28CD4;
    16
}
/// $08418A: lea.l $84190(pc), a1
pub fn i_08418a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8418E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84190;
    16
}
/// $08418E: move.l a1, (a6)
pub fn i_08418e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84190;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084190: jsr $2783a.l
pub fn i_084190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84196;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84196);
    cpu.pc = 0x2783A;
    16
}
/// $084196: jsr $28d70.l
pub fn i_084196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8419C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8419C);
    cpu.pc = 0x28D70;
    16
}
/// $08419C: jsr $2870a.l
pub fn i_08419c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x841A2);
    cpu.pc = 0x2870A;
    16
}
/// $0841A2: bcc.w $841ac
pub fn i_0841a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841A6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x841AC; return 10; }
    8
}
/// $0841A6: lea.l $841be(pc), a1
pub fn i_0841a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x841BE;
    16
}
/// $0841AA: move.l a1, (a6)
pub fn i_0841aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0841AC: jsr $4fa70.l
pub fn i_0841ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x841B2);
    cpu.pc = 0x4FA70;
    16
}
/// $0841B2: bcc.w $841bc
pub fn i_0841b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x841BC; return 10; }
    8
}
/// $0841B6: jmp $518.l
pub fn i_0841b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841BC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0841BC: rts
pub fn i_0841bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0841be  ($0841BE, 56 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0841BE: lea.l $ffff.w, a0
pub fn i_0841be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841C2;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0841C2: move.l a0, $48(a6)
pub fn i_0841c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841C6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0841C6: lea.l $2e6ff6.l, a0
pub fn i_0841c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6FF6;
    20
}
/// $0841CC: jsr $28cd4.l
pub fn i_0841cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x841D2);
    cpu.pc = 0x28CD4;
    16
}
/// $0841D2: lea.l $841d8(pc), a1
pub fn i_0841d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x841D8;
    16
}
/// $0841D6: move.l a1, (a6)
pub fn i_0841d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0841D8: jsr $2783a.l
pub fn i_0841d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x841DE);
    cpu.pc = 0x2783A;
    16
}
/// $0841DE: jsr $28d70.l
pub fn i_0841de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x841E4);
    cpu.pc = 0x28D70;
    16
}
/// $0841E4: jsr $4fa70.l
pub fn i_0841e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x841EA);
    cpu.pc = 0x4FA70;
    16
}
/// $0841EA: bcc.w $841f4
pub fn i_0841ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x841F4; return 10; }
    8
}
/// $0841EE: jmp $518.l
pub fn i_0841ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841F4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0841F4: rts
pub fn i_0841f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0841f6  ($0841F6, 86 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0841F6: movea.l $3c(a6), a1
pub fn i_0841f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x841FA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0841FA: jsr $2942a.l
pub fn i_0841fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84200;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84200);
    cpu.pc = 0x2942A;
    16
}
/// $084200: move.w #$40, $70(a6)
pub fn i_084200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84206;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084206: move.w #$1, $66(a6)
pub fn i_084206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8420C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08420C: lea.l $2e7006.l, a0
pub fn i_08420c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84212;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7006;
    20
}
/// $084212: jsr $28cd4.l
pub fn i_084212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84218;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84218);
    cpu.pc = 0x28CD4;
    16
}
/// $084218: lea.l $8421e(pc), a1
pub fn i_084218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8421C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8421E;
    16
}
/// $08421C: move.l a1, (a6)
pub fn i_08421c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8421E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08421E: jsr $2783a.l
pub fn i_08421e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84224;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84224);
    cpu.pc = 0x2783A;
    16
}
/// $084224: jsr $28d70.l
pub fn i_084224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8422A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8422A);
    cpu.pc = 0x28D70;
    16
}
/// $08422A: jsr $2870a.l
pub fn i_08422a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84230;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84230);
    cpu.pc = 0x2870A;
    16
}
/// $084230: bcc.w $8423a
pub fn i_084230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84234;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8423A; return 10; }
    8
}
/// $084234: lea.l $8424c(pc), a1
pub fn i_084234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84238;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8424C;
    16
}
/// $084238: move.l a1, (a6)
pub fn i_084238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8423A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08423A: jsr $4fa70.l
pub fn i_08423a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84240);
    cpu.pc = 0x4FA70;
    16
}
/// $084240: bcc.w $8424a
pub fn i_084240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84244;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8424A; return 10; }
    8
}
/// $084244: jmp $518.l
pub fn i_084244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8424A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08424A: rts
pub fn i_08424a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8424C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08424c  ($08424C, 26 B, enemies/bosses/miniboss_finale_083be2.s)
/// $08424C: lea.l $ffff.w, a0
pub fn i_08424c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84250;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084250: move.l a0, $48(a6)
pub fn i_084250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84254;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084254: jsr $86400(pc)
pub fn i_084254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84258;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84258);
    cpu.pc = 0x86400;
    16
}
/// $084258: lea.l $8425e(pc), a1
pub fn i_084258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8425C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8425E;
    16
}
/// $08425C: move.l a1, (a6)
pub fn i_08425c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8425E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08425E: jmp $518.l
pub fn i_08425e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84264;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084264: rts
pub fn i_084264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84266;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084266  ($084266, 28 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084266: movea.l $3c(a6), a1
pub fn i_084266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8426A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08426A: jsr $2942a.l
pub fn i_08426a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84270);
    cpu.pc = 0x2942A;
    16
}
/// $084270: jsr $86364(pc)
pub fn i_084270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84274;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84274);
    cpu.pc = 0x86364;
    16
}
/// $084274: lea.l $8427a(pc), a1
pub fn i_084274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84278;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8427A;
    16
}
/// $084278: move.l a1, (a6)
pub fn i_084278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8427A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08427A: jmp $518.l
pub fn i_08427a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84280;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084280: rts
pub fn i_084280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84282;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084282  ($084282, 78 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084282: move.w #$0, $70(a6)
pub fn i_084282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84288;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084288: move.w #$1, $66(a6)
pub fn i_084288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8428E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08428E: lea.l $2e701c.l, a0
pub fn i_08428e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84294;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E701C;
    20
}
/// $084294: jsr $28cd4.l
pub fn i_084294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8429A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8429A);
    cpu.pc = 0x28CD4;
    16
}
/// $08429A: lea.l $842a0(pc), a1
pub fn i_08429a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8429E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x842A0;
    16
}
/// $08429E: move.l a1, (a6)
pub fn i_08429e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0842A0: jsr $2783a.l
pub fn i_0842a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842A6);
    cpu.pc = 0x2783A;
    16
}
/// $0842A6: jsr $28d70.l
pub fn i_0842a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842AC);
    cpu.pc = 0x28D70;
    16
}
/// $0842AC: jsr $2870a.l
pub fn i_0842ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842B2);
    cpu.pc = 0x2870A;
    16
}
/// $0842B2: bcc.w $842be
pub fn i_0842b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x842BE; return 10; }
    8
}
/// $0842B6: jsr $864ea(pc)
pub fn i_0842b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x842BA);
    cpu.pc = 0x864EA;
    16
}
/// $0842BA: bra.w $842c8
pub fn i_0842ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x842C8;
    10
}
/// $0842BE: jsr $4fa70.l
pub fn i_0842be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842C4);
    cpu.pc = 0x4FA70;
    16
}
/// $0842C4: bcc.w $842ce
pub fn i_0842c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x842CE; return 10; }
    8
}
/// $0842C8: jmp $518.l
pub fn i_0842c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842CE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0842CE: rts
pub fn i_0842ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0842d0  ($0842D0, 198 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0842D0: movea.l $3c(a6), a1
pub fn i_0842d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842D4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0842D4: jsr $2942a.l
pub fn i_0842d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842DA);
    cpu.pc = 0x2942A;
    16
}
/// $0842DA: move.w #$30, $70(a6)
pub fn i_0842da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0842E0: move.w #$1, $66(a6)
pub fn i_0842e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0842E6: lea.l $2e7032.l, a0
pub fn i_0842e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7032;
    20
}
/// $0842EC: jsr $28cd4.l
pub fn i_0842ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0842F2: lea.l $842f8(pc), a1
pub fn i_0842f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x842F8;
    16
}
/// $0842F6: move.l a1, (a6)
pub fn i_0842f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0842F8: jsr $2783a.l
pub fn i_0842f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x842FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x842FE);
    cpu.pc = 0x2783A;
    16
}
/// $0842FE: jsr $28d70.l
pub fn i_0842fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84304);
    cpu.pc = 0x28D70;
    16
}
/// $084304: jsr $2870a.l
pub fn i_084304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8430A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8430A);
    cpu.pc = 0x2870A;
    16
}
/// $08430A: bcc.w $84320
pub fn i_08430a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8430E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84320; return 10; }
    8
}
/// $08430E: lea.l $5e766.l, a0
pub fn i_08430e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84314;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $084314: jsr $5e770.l
pub fn i_084314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8431A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8431A);
    cpu.pc = 0x5E770;
    16
}
/// $08431A: bclr.b #$3, $13(a6)
pub fn i_08431a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84320;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084320: jsr $28758.l
pub fn i_084320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84326;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84326);
    cpu.pc = 0x28758;
    16
}
/// $084326: bcc.w $84384
pub fn i_084326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8432A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84384; return 10; }
    8
}
/// $08432A: move.l #$1000, d0
pub fn i_08432a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84330;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $084330: jsr $51a28.l
pub fn i_084330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84336);
    cpu.pc = 0x51A28;
    16
}
/// $084336: move.w #$1030, d0
pub fn i_084336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8433A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1030;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08433A: jsr $2352.l
pub fn i_08433a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84340);
    cpu.pc = 0x2352;
    16
}
/// $084340: lea.l $ffff.w, a0
pub fn i_084340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84344;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084344: move.l a0, $48(a6)
pub fn i_084344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84348;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084348: jsr $8640e(pc)
pub fn i_084348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8434C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8434C);
    cpu.pc = 0x8640E;
    16
}
/// $08434C: jsr $8641c(pc)
pub fn i_08434c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84350;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84350);
    cpu.pc = 0x8641C;
    16
}
/// $084350: lea.l $2e9b00.l, a1
pub fn i_084350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84356;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9B00;
    20
}
/// $084356: jsr $77c7e.l
pub fn i_084356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8435C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8435C);
    cpu.pc = 0x77C7E;
    16
}
/// $08435C: lea.l $2e9aca.l, a1
pub fn i_08435c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84362;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9ACA;
    20
}
/// $084362: jsr $77c7e.l
pub fn i_084362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84368;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84368);
    cpu.pc = 0x77C7E;
    16
}
/// $084368: lea.l $2eacb8.l, a1
pub fn i_084368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8436E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EACB8;
    20
}
/// $08436E: jsr $43fac.l
pub fn i_08436e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84374);
    cpu.pc = 0x43FAC;
    16
}
/// $084374: lea.l $2eae76.l, a1
pub fn i_084374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8437A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EAE76;
    20
}
/// $08437A: jsr $43fac.l
pub fn i_08437a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84380;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84380);
    cpu.pc = 0x43FAC;
    16
}
/// $084380: bra.w $8438e
pub fn i_084380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84384;
    cpu.ir = 0x6000;
    cpu.pc = 0x8438E;
    10
}
/// $084384: jsr $4fa70.l
pub fn i_084384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8438A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8438A);
    cpu.pc = 0x4FA70;
    16
}
/// $08438A: bcc.w $84394
pub fn i_08438a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8438E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84394; return 10; }
    8
}
/// $08438E: jmp $518.l
pub fn i_08438e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84394;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084394: rts
pub fn i_084394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84396;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084396  ($084396, 114 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084396: movea.l $3c(a6), a1
pub fn i_084396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8439A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08439A: jsr $2942a.l
pub fn i_08439a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843A0);
    cpu.pc = 0x2942A;
    16
}
/// $0843A0: move.w #$a9, d1
pub fn i_0843a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843A4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0843A4: jsr $236e.l
pub fn i_0843a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843AA);
    cpu.pc = 0x236E;
    16
}
/// $0843AA: move.w #$1, $38(a6)
pub fn i_0843aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0843B0: jsr $860e4(pc)
pub fn i_0843b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x843B4);
    cpu.pc = 0x860E4;
    16
}
/// $0843B4: jsr $86300(pc)
pub fn i_0843b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x843B8);
    cpu.pc = 0x86300;
    16
}
/// $0843B8: lea.l $8495e(pc), a1
pub fn i_0843b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8495E;
    16
}
/// $0843BC: jsr $4ae.l
pub fn i_0843bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843C2);
    cpu.pc = 0x4AE;
    16
}
/// $0843C2: jsr $5dd22.l
pub fn i_0843c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843C8);
    cpu.pc = 0x5DD22;
    16
}
/// $0843C8: move.b #$0, $21(a6)
pub fn i_0843c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843CE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0843CE: lea.l $2e7048.l, a0
pub fn i_0843ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7048;
    20
}
/// $0843D4: jsr $28cd4.l
pub fn i_0843d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843DA);
    cpu.pc = 0x28CD4;
    16
}
/// $0843DA: lea.l $843e0(pc), a1
pub fn i_0843da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x843E0;
    16
}
/// $0843DE: move.l a1, (a6)
pub fn i_0843de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0843E0: clr.b $10e39a.l
pub fn i_0843e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843E6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0843E6: jsr $2783a.l
pub fn i_0843e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843EC);
    cpu.pc = 0x2783A;
    16
}
/// $0843EC: jsr $28d70.l
pub fn i_0843ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x843F2);
    cpu.pc = 0x28D70;
    16
}
/// $0843F2: move.w #$7fff, $66(a6)
pub fn i_0843f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0843F8: cmpi.b #$ff, $21(a6)
pub fn i_0843f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x843FE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0843FE: bne.w $8440e
pub fn i_0843fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84402;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8440E; return 10; }
    8
}
/// $084402: move.b #$ff, $74(a6)
pub fn i_084402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84408;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetTaskHandler_084408  ($084408, 8 B, kernel/task_handlers.c)
/// $084408: lea.l $84410(pc), a1
pub fn i_084408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8440C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84410;
    16
}
/// $08440C: move.l a1, (a6)
pub fn i_08440c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8440E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08440E: rts
pub fn i_08440e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84410;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084410  ($084410, 168 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084410: move.b #$3, $10e39c.l
pub fn i_084410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84418;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39C, v1 as u8);
    16
}
/// $084418: lea.l $845b8(pc), a1
pub fn i_084418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8441C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x845B8;
    16
}
/// $08441C: jsr $4ae.l
pub fn i_08441c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84422);
    cpu.pc = 0x4AE;
    16
}
/// $084422: move.w #$a, $72(a0)
pub fn i_084422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84428;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084428: lea.l $ffff.w, a0
pub fn i_084428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8442C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08442C: move.l a0, $48(a6)
pub fn i_08442c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84430;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084430: move.w #$1033, d0
pub fn i_084430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84434;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084434: jsr $2352.l
pub fn i_084434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8443A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8443A);
    cpu.pc = 0x2352;
    16
}
/// $08443A: move.w #$27, d1
pub fn i_08443a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8443E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x27;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08443E: move.w #$129, d2
pub fn i_08443e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84442;
    cpu.ir = 0x343C;
    let v1: u32 = 0x129;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084442: move.w #$2, d3
pub fn i_084442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84446;
    cpu.ir = 0x363C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084446: move.w #$1, d4
pub fn i_084446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8444A;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08444A: jsr $2c26.l
pub fn i_08444a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84450;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84450);
    cpu.pc = 0x2C26;
    16
}
/// $084450: lea.l $86586.l, a1
pub fn i_084450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84456;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x86586;
    20
}
/// $084456: jsr $4ae.l
pub fn i_084456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8445C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8445C);
    cpu.pc = 0x4AE;
    16
}
/// $08445C: lea.l $2ea790.l, a2
pub fn i_08445c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84462;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA790;
    20
}
/// $084462: jsr $5022a.l
pub fn i_084462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84468);
    cpu.pc = 0x5022A;
    16
}
/// $084468: jsr $86196(pc)
pub fn i_084468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8446C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8446C);
    cpu.pc = 0x86196;
    16
}
/// $08446C: lea.l $2e98d8.l, a1
pub fn i_08446c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84472;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E98D8;
    20
}
/// $084472: jsr $77c7e.l
pub fn i_084472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84478;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84478);
    cpu.pc = 0x77C7E;
    16
}
/// $084478: lea.l $2e98ea.l, a1
pub fn i_084478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8447E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E98EA;
    20
}
/// $08447E: jsr $77c7e.l
pub fn i_08447e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84484;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84484);
    cpu.pc = 0x77C7E;
    16
}
/// $084484: move.w #$c000, $38(a0)
pub fn i_084484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8448A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08448A: jsr $8610c(pc)
pub fn i_08448a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8448E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8448E);
    cpu.pc = 0x8610C;
    16
}
/// $08448E: move.b #$0, $21(a6)
pub fn i_08448e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84494;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084494: lea.l $8449a(pc), a1
pub fn i_084494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84498;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8449A;
    16
}
/// $084498: move.l a1, (a6)
pub fn i_084498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8449A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08449A: clr.b $10e39a.l
pub fn i_08449a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844A0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0844A0: jsr $2783a.l
pub fn i_0844a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x844A6);
    cpu.pc = 0x2783A;
    16
}
/// $0844A6: cmpi.b #$9, $21(a6)
pub fn i_0844a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844AC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x9, Size::Byte, false);
    16
}
/// $0844AC: blt.w $844be
pub fn i_0844ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844B0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x844BE; return 10; }
    8
}
/// $0844B0: move.b #$2, $10e39a.l
pub fn i_0844b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844B8;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39A, v1 as u8);
    16
}
// ===== SetTaskHandler_0844b8  ($0844B8, 8 B, kernel/task_handlers.c)
/// $0844B8: lea.l $844c0(pc), a1
pub fn i_0844b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x844C0;
    16
}
/// $0844BC: move.l a1, (a6)
pub fn i_0844bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0844BE: rts
pub fn i_0844be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0844c0  ($0844C0, 68 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0844C0: jsr $2783a.l
pub fn i_0844c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x844C6);
    cpu.pc = 0x2783A;
    16
}
/// $0844C6: lea.l $2eacd4.l, a1
pub fn i_0844c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844CC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EACD4;
    20
}
/// $0844CC: jsr $43fac.l
pub fn i_0844cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x844D2);
    cpu.pc = 0x43FAC;
    16
}
/// $0844D2: lea.l $2e7486.l, a0
pub fn i_0844d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7486;
    20
}
/// $0844D8: jsr $28cd4.l
pub fn i_0844d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x844DE);
    cpu.pc = 0x28CD4;
    16
}
/// $0844DE: lea.l $844e4(pc), a1
pub fn i_0844de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x844E4;
    16
}
/// $0844E2: move.l a1, (a6)
pub fn i_0844e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0844E4: clr.b $10e39a.l
pub fn i_0844e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844EA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $0844EA: jsr $2783a.l
pub fn i_0844ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x844F0);
    cpu.pc = 0x2783A;
    16
}
/// $0844F0: jsr $28d70.l
pub fn i_0844f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x844F6);
    cpu.pc = 0x28D70;
    16
}
/// $0844F6: bcc.w $8450a
pub fn i_0844f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x844FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8450A; return 10; }
    8
}
/// $0844FA: jsr $6f0.l
pub fn i_0844fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84500;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84500);
    cpu.pc = 0x6F0;
    16
}
/// $084500: bcs.w $8450a
pub fn i_084500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84504;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8450A; return 10; }
    8
}
// ===== SetTaskHandler_084504  ($084504, 8 B, kernel/task_handlers.c)
/// $084504: lea.l $8450c(pc), a1
pub fn i_084504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84508;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8450C;
    16
}
/// $084508: move.l a1, (a6)
pub fn i_084508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8450A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08450A: rts
pub fn i_08450a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8450C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08450c  ($08450C, 20 B, enemies/bosses/miniboss_finale_083be2.s)
/// $08450C: clr.b $10e39c.l
pub fn i_08450c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84512;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39C, r2 as u8);
    16
}
/// $084512: jsr $320d4.l
pub fn i_084512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84518;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84518);
    cpu.pc = 0x320D4;
    16
}
/// $084518: jmp $518.l
pub fn i_084518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8451E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08451E: rts
pub fn i_08451e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84520;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084520  ($084520, 152 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084520: move.w #$100, $70(a6)
pub fn i_084520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84526;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084526: lea.l $2e9256.l, a0
pub fn i_084526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8452C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9256;
    20
}
/// $08452C: move.l a0, $48(a6)
pub fn i_08452c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84530;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084530: lea.l $84536(pc), a1
pub fn i_084530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84534;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84536;
    16
}
/// $084534: move.l a1, (a6)
pub fn i_084534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84536;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084536: jsr $2783a.l
pub fn i_084536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8453C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8453C);
    cpu.pc = 0x2783A;
    16
}
/// $08453C: move.w #$7fff, $66(a6)
pub fn i_08453c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84542;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084542: jsr $2870a.l
pub fn i_084542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84548;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84548);
    cpu.pc = 0x2870A;
    16
}
/// $084548: bcc.w $84596
pub fn i_084548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8454C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84596; return 10; }
    8
}
/// $08454C: lea.l $5e766.l, a0
pub fn i_08454c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84552;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $084552: jsr $5e770.l
pub fn i_084552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84558;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84558);
    cpu.pc = 0x5E770;
    16
}
/// $084558: bclr.b #$3, $13(a6)
pub fn i_084558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8455E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08455E: move.b $106f28.l, d0
pub fn i_08455e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84564;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $084564: cmpi.b #$1, $58(a6)
pub fn i_084564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8456A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08456A: beq.w $8457c
pub fn i_08456a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8456E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8457C; return 10; }
    8
}
/// $08456E: cmpi.b #$2, $58(a6)
pub fn i_08456e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84574;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $084574: beq.w $8457c
pub fn i_084574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84578;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8457C; return 10; }
    8
}
/// $084578: bra.w $84596
pub fn i_084578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8457C;
    cpu.ir = 0x6000;
    cpu.pc = 0x84596;
    10
}
/// $08457C: btst.b #$0, d0
pub fn i_08457c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84580;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $084580: bne.w $84596
pub fn i_084580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84584;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84596; return 10; }
    8
}
/// $084584: lea.l $53886.l, a1
pub fn i_084584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8458A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x53886;
    20
}
/// $08458A: jsr $4ae.l
pub fn i_08458a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84590);
    cpu.pc = 0x4AE;
    16
}
/// $084590: jsr $5dd22.l
pub fn i_084590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84596;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84596);
    cpu.pc = 0x5DD22;
    16
}
/// $084596: jsr $28758.l
pub fn i_084596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8459C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8459C);
    cpu.pc = 0x28758;
    16
}
/// $08459C: bcc.w $845a6
pub fn i_08459c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x845A6; return 10; }
    8
}
/// $0845A0: bclr.b #$0, $13(a6)
pub fn i_0845a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845A6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0845A6: jsr $4fa70.l
pub fn i_0845a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x845AC);
    cpu.pc = 0x4FA70;
    16
}
/// $0845AC: bcc.w $845b6
pub fn i_0845ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x845B6; return 10; }
    8
}
/// $0845B0: jmp $518.l
pub fn i_0845b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845B6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0845B6: rts
pub fn i_0845b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0845b8  ($0845B8, 44 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0845B8: move.b #$1, $10a2d1.l
pub fn i_0845b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845C0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $0845C0: lea.l $845c6(pc), a1
pub fn i_0845c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x845C6;
    16
}
/// $0845C4: move.l a1, (a6)
pub fn i_0845c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0845C6: subq.w #$1, $72(a6)
pub fn i_0845c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845CA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0845CA: beq.w $845e4
pub fn i_0845ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x845E4; return 10; }
    8
}
/// $0845CE: move.w $72(a6), d0
pub fn i_0845ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845D2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0845D2: andi.w #$1, d0
pub fn i_0845d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845D6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0845D6: beq.w $845e2
pub fn i_0845d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x845E2; return 10; }
    8
}
/// $0845DA: move.b #$1, $10a2d1.l
pub fn i_0845da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845E2;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $0845E2: rts
pub fn i_0845e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_0845e4  ($0845E4, 8 B, kernel/jmp_to_scheduler.c)
/// $0845E4: jmp $518.l
pub fn i_0845e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845EA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0845EA: rts
pub fn i_0845ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0845ec  ($0845EC, 76 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0845EC: lea.l $845f2(pc), a1
pub fn i_0845ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x845F2;
    16
}
/// $0845F0: move.l a1, (a6)
pub fn i_0845f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0845F2: movea.l $c(a6), a0
pub fn i_0845f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845F6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0845F6: btst.b #$3, $13(a0)
pub fn i_0845f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x845FC;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0845FC: beq.w $84622
pub fn i_0845fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84600;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84622; return 10; }
    8
}
/// $084600: move.b $20(a0), d0
pub fn i_084600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84604;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084604: cmp.b $21(a6), d0
pub fn i_084604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84608;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $084608: bne.w $84622
pub fn i_084608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8460C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84622; return 10; }
    8
}
/// $08460C: bclr.b #$3, $13(a0)
pub fn i_08460c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84612;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084612: move.b $58(a0), $58(a6)
pub fn i_084612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84618;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x58);
    bus.write8(ad4, v3 as u8);
    20
}
/// $084618: jsr $8651e(pc)
pub fn i_084618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8461C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8461C);
    cpu.pc = 0x8651E;
    16
}
/// $08461C: lea.l $84638(pc), a1
pub fn i_08461c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84620;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84638;
    16
}
/// $084620: move.l a1, (a6)
pub fn i_084620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84622;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084622: movea.l $c(a6), a0
pub fn i_084622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84626;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084626: cmpi.b #$ff, $21(a0)
pub fn i_084626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8462C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08462C: bne.w $84636
pub fn i_08462c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84630;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84636; return 10; }
    8
}
/// $084630: jmp $518.l
pub fn i_084630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84636;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084636: rts
pub fn i_084636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84638;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084638  ($084638, 98 B, enemies/bosses/miniboss_finale_083be2.s)
/// $084638: move.l #$1000, d0
pub fn i_084638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8463E;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08463E: jsr $51a28.l
pub fn i_08463e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84644;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84644);
    cpu.pc = 0x51A28;
    16
}
/// $084644: move.w #$108d, d0
pub fn i_084644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84648;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084648: jsr $2352.l
pub fn i_084648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8464E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8464E);
    cpu.pc = 0x2352;
    16
}
/// $08464E: lea.l $84654(pc), a1
pub fn i_08464e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84652;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84654;
    16
}
/// $084652: move.l a1, (a6)
pub fn i_084652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84654;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084654: movea.l $c(a6), a0
pub fn i_084654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84658;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084658: btst.b #$3, $13(a0)
pub fn i_084658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8465E;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08465E: beq.w $84684
pub fn i_08465e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84662;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84684; return 10; }
    8
}
/// $084662: move.b $20(a0), d0
pub fn i_084662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84666;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084666: cmp.b $21(a6), d0
pub fn i_084666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8466A;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08466A: bne.w $84684
pub fn i_08466a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8466E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84684; return 10; }
    8
}
/// $08466E: bclr.b #$3, $13(a0)
pub fn i_08466e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84674;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084674: move.b $58(a0), $58(a6)
pub fn i_084674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8467A;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x58);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08467A: jsr $86538(pc)
pub fn i_08467a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8467E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8467E);
    cpu.pc = 0x86538;
    16
}
/// $08467E: lea.l $8469a(pc), a1
pub fn i_08467e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84682;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8469A;
    16
}
/// $084682: move.l a1, (a6)
pub fn i_084682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84684;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084684: movea.l $c(a6), a0
pub fn i_084684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84688;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084688: cmpi.b #$ff, $21(a0)
pub fn i_084688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8468E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08468E: bne.w $84698
pub fn i_08468e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84692;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84698; return 10; }
    8
}
/// $084692: jmp $518.l
pub fn i_084692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84698;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084698: rts
pub fn i_084698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8469A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08469a  ($08469A, 70 B, enemies/bosses/miniboss_finale_083be2.s)
/// $08469A: move.w #$108d, d0
pub fn i_08469a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8469E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08469E: jsr $2352.l
pub fn i_08469e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x846A4);
    cpu.pc = 0x2352;
    16
}
/// $0846A4: lea.l $846aa(pc), a1
pub fn i_0846a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x846AA;
    16
}
/// $0846A8: move.l a1, (a6)
pub fn i_0846a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0846AA: movea.l $c(a6), a0
pub fn i_0846aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846AE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0846AE: btst.b #$3, $13(a0)
pub fn i_0846ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846B4;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0846B4: beq.w $846ca
pub fn i_0846b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x846CA; return 10; }
    8
}
/// $0846B8: move.b $20(a0), d0
pub fn i_0846b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846BC;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0846BC: cmp.b $21(a6), d0
pub fn i_0846bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846C0;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0846C0: bne.w $846ca
pub fn i_0846c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x846CA; return 10; }
    8
}
/// $0846C4: bclr.b #$3, $13(a0)
pub fn i_0846c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846CA;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0846CA: movea.l $c(a6), a0
pub fn i_0846ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846CE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0846CE: cmpi.b #$ff, $21(a0)
pub fn i_0846ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846D4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0846D4: bne.w $846de
pub fn i_0846d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x846DE; return 10; }
    8
}
/// $0846D8: jmp $518.l
pub fn i_0846d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846DE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0846DE: rts
pub fn i_0846de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846E0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0846e0  ($0846E0, 328 B, enemies/bosses/miniboss_finale_083be2.s)
/// $0846E0: move.w #$a9, d1
pub fn i_0846e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846E4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0846E4: jsr $236e.l
pub fn i_0846e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x846EA);
    cpu.pc = 0x236E;
    16
}
/// $0846EA: move.b #$ff, $32(a6)
pub fn i_0846ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846F0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0846F0: move.b #$ff, $33(a6)
pub fn i_0846f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846F6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0846F6: move.b #$0, $3a(a6)
pub fn i_0846f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x846FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0846FC: addi.w #$80, $24(a6)
pub fn i_0846fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84702;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084702: addi.w #$29, $22(a6)
pub fn i_084702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84708;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x29, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084708: cmpi.b #$0, $21(a6)
pub fn i_084708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8470E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08470E: beq.w $84720
pub fn i_08470e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84712;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84720; return 10; }
    8
}
/// $084712: move.b $21(a6), d0
pub fn i_084712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84716;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084716: addi.w #$40, $22(a6)
pub fn i_084716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8471C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08471C: subq.b #$1, d0
pub fn i_08471c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8471E;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08471E: bne.b $84716
pub fn i_08471e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84720;
    cpu.ir = 0x66F6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84716; return 10; }
    8
}
/// $084720: move.w #$14, $66(a6)
pub fn i_084720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84726;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084726: clr.l d0
pub fn i_084726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84728;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $084728: move.b $21(a6), d0
pub fn i_084728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8472C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08472C: movea.l #$2e7556, a0
pub fn i_08472c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84732;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7556;
    cpu.a[0] = v1;
    4
}
/// $084732: lsl.w #$2, d0
pub fn i_084732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84734;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $084734: movea.l (a0, d0.w), a0
pub fn i_084734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84738;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $084738: cmpa.l #$ffffffff, a0
pub fn i_084738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8473E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08473E: beq.w $84748
pub fn i_08473e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84742;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84748; return 10; }
    8
}
/// $084742: jsr $28cd4.l
pub fn i_084742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84748;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84748);
    cpu.pc = 0x28CD4;
    16
}
/// $084748: jsr $5e9b6.l
pub fn i_084748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8474E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8474E);
    cpu.pc = 0x5E9B6;
    16
}
/// $08474E: andi.l #$f, d0
pub fn i_08474e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84754;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $084754: asl.l #$2, d0
pub fn i_084754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84756;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $084756: lea.l $2eb02c.l, a0
pub fn i_084756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8475C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EB02C;
    20
}
/// $08475C: movea.l (a0, d0.w), a1
pub fn i_08475c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84760;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $084760: jsr $4ae.l
pub fn i_084760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84766;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84766);
    cpu.pc = 0x4AE;
    16
}
/// $084766: jsr $5dd22.l
pub fn i_084766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8476C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8476C);
    cpu.pc = 0x5DD22;
    16
}
/// $08476C: addi.w #$4, $22(a0)
pub fn i_08476c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84772;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084772: addi.w #$1, $24(a0)
pub fn i_084772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84778;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084778: andi.b #$1, d0
pub fn i_084778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8477C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08477C: add.b d0, $98(a0)
pub fn i_08477c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84780;
    cpu.ir = 0xD128;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $084780: addi.b #$7f, $99(a0)
pub fn i_084780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84786;
    cpu.ir = 0x628;
    let ad1 = cpu.a[0].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x7F, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $084786: move.b #$1, $11(a0)
pub fn i_084786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8478C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08478C: move.w #$8000, $38(a6)
pub fn i_08478c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84792;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084792: jsr $267e2.l
pub fn i_084792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84798);
    cpu.pc = 0x267E2;
    16
}
/// $084798: jsr $27cee.l
pub fn i_084798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8479E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8479E);
    cpu.pc = 0x27CEE;
    16
}
/// $08479E: lea.l $847a4(pc), a1
pub fn i_08479e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x847A4;
    16
}
/// $0847A2: move.l a1, (a6)
pub fn i_0847a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0847A4: jsr $2783a.l
pub fn i_0847a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x847AA);
    cpu.pc = 0x2783A;
    16
}
/// $0847AA: cmpi.w #$150, $22(a6)
pub fn i_0847aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $0847B0: bgt.w $84834
pub fn i_0847b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847B4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x84834; return 10; }
    8
}
/// $0847B4: jsr $28d70.l
pub fn i_0847b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x847BA);
    cpu.pc = 0x28D70;
    16
}
/// $0847BA: movea.l $c(a6), a0
pub fn i_0847ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847BE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0847BE: cmpi.b #$ff, $21(a0)
pub fn i_0847be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847C4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0847C4: beq.w $84828
pub fn i_0847c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84828; return 10; }
    8
}
/// $0847C8: cmpi.l #$ffffffff, $3c(a6)
pub fn i_0847c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847D0;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $0847D0: beq.w $8481e
pub fn i_0847d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8481E; return 10; }
    8
}
/// $0847D4: jsr $2870a.l
pub fn i_0847d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x847DA);
    cpu.pc = 0x2870A;
    16
}
/// $0847DA: bcc.w $847f0
pub fn i_0847da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x847F0; return 10; }
    8
}
/// $0847DE: lea.l $5e766.l, a0
pub fn i_0847de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0847E4: jsr $5e770.l
pub fn i_0847e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x847EA);
    cpu.pc = 0x5E770;
    16
}
/// $0847EA: bclr.b #$3, $13(a6)
pub fn i_0847ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847F0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0847F0: jsr $28758.l
pub fn i_0847f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x847F6);
    cpu.pc = 0x28758;
    16
}
/// $0847F6: bcc.w $8481e
pub fn i_0847f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8481E; return 10; }
    8
}
/// $0847FA: move.w #$102e, d0
pub fn i_0847fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0847FE: jsr $2352.l
pub fn i_0847fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84804);
    cpu.pc = 0x2352;
    16
}
/// $084804: lea.l $ffff.w, a0
pub fn i_084804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84808;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084808: jsr $28cd4.l
pub fn i_084808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8480E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8480E);
    cpu.pc = 0x28CD4;
    16
}
/// $08480E: lea.l $2e9adc.l, a1
pub fn i_08480e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84814;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9ADC;
    20
}
/// $084814: jsr $77c7e.l
pub fn i_084814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8481A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8481A);
    cpu.pc = 0x77C7E;
    16
}
/// $08481A: bra.w $84834
pub fn i_08481a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8481E;
    cpu.ir = 0x6000;
    cpu.pc = 0x84834;
    10
}
/// $08481E: jsr $4fa70.l
pub fn i_08481e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84824;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84824);
    cpu.pc = 0x4FA70;
    16
}
/// $084824: bcc.w $84834
pub fn i_084824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84828;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84834; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_084828  ($084828, 14 B, kernel/jsr5b6_jmp518.c)
/// $084828: jsr $5b6.l
pub fn i_084828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8482E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8482E);
    cpu.pc = 0x5B6;
    16
}
/// $08482E: jmp $518.l
pub fn i_08482e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84834;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084834: rts
pub fn i_084834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84836;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084836  ($084836, 92 B, enemies/squad/rescue_squad_084836.s)
/// $084836: move.w #$a9, d1
pub fn i_084836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8483A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08483A: jsr $236e.l
pub fn i_08483a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84840;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84840);
    cpu.pc = 0x236E;
    16
}
/// $084840: move.b #$ff, $32(a6)
pub fn i_084840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84846;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084846: move.b #$ff, $33(a6)
pub fn i_084846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8484C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08484C: clr.l d0
pub fn i_08484c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8484E;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08484E: move.b $21(a6), d0
pub fn i_08484e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84852;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084852: asl.l #$1, d0
pub fn i_084852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84854;
    cpu.ir = 0xE380;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, true);
    cpu.d[0] = r;
    8
}
/// $084854: movea.l #$2e77ca, a0
pub fn i_084854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8485A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E77CA;
    cpu.a[0] = v1;
    4
}
/// $08485A: lsl.w #$2, d0
pub fn i_08485a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8485C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08485C: movea.l (a0, d0.w), a0
pub fn i_08485c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84860;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $084860: cmpa.l #$ffffffff, a0
pub fn i_084860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84866;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $084866: beq.w $84870
pub fn i_084866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8486A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84870; return 10; }
    8
}
/// $08486A: jsr $28cd4.l
pub fn i_08486a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84870;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84870);
    cpu.pc = 0x28CD4;
    16
}
/// $084870: lea.l $84876(pc), a1
pub fn i_084870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84874;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84876;
    16
}
/// $084874: move.l a1, (a6)
pub fn i_084874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84876;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084876: jsr $2783a.l
pub fn i_084876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8487C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8487C);
    cpu.pc = 0x2783A;
    16
}
/// $08487C: jsr $28d70.l
pub fn i_08487c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84882);
    cpu.pc = 0x28D70;
    16
}
/// $084882: movea.l $c(a6), a0
pub fn i_084882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84886;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084886: move.b $21(a6), d0
pub fn i_084886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8488A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08488A: cmp.b $21(a0), d0
pub fn i_08488a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8488E;
    cpu.ir = 0xB028;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08488E: bne.w $84898
pub fn i_08488e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84892;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84898; return 10; }
    8
}
// ===== SetTaskHandler_084892  ($084892, 8 B, kernel/task_handlers.c)
/// $084892: lea.l $8489a(pc), a1
pub fn i_084892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84896;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8489A;
    16
}
/// $084896: move.l a1, (a6)
pub fn i_084896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84898;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084898: rts
pub fn i_084898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8489A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08489a  ($08489A, 60 B, enemies/squad/rescue_squad_084836.s)
/// $08489A: clr.l d0
pub fn i_08489a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8489C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08489C: move.b $21(a6), d0
pub fn i_08489c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848A0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0848A0: asl.l #$1, d0
pub fn i_0848a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848A2;
    cpu.ir = 0xE380;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, true);
    cpu.d[0] = r;
    8
}
/// $0848A2: addq.l #$1, d0
pub fn i_0848a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848A4;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $0848A4: movea.l #$2e77ca, a0
pub fn i_0848a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848AA;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E77CA;
    cpu.a[0] = v1;
    4
}
/// $0848AA: lsl.w #$2, d0
pub fn i_0848aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848AC;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0848AC: movea.l (a0, d0.w), a0
pub fn i_0848ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848B0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0848B0: cmpa.l #$ffffffff, a0
pub fn i_0848b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848B6;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0848B6: beq.w $848c0
pub fn i_0848b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x848C0; return 10; }
    8
}
/// $0848BA: jsr $28cd4.l
pub fn i_0848ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x848C0);
    cpu.pc = 0x28CD4;
    16
}
/// $0848C0: lea.l $848c6(pc), a1
pub fn i_0848c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x848C6;
    16
}
/// $0848C4: move.l a1, (a6)
pub fn i_0848c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0848C6: jsr $2783a.l
pub fn i_0848c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x848CC);
    cpu.pc = 0x2783A;
    16
}
/// $0848CC: jsr $28d70.l
pub fn i_0848cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x848D2);
    cpu.pc = 0x28D70;
    16
}
/// $0848D2: bcc.w $848dc
pub fn i_0848d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x848DC; return 10; }
    8
}
// ===== SetTaskHandler_0848d6  ($0848D6, 8 B, kernel/task_handlers.c)
/// $0848D6: lea.l $848de(pc), a1
pub fn i_0848d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x848DE;
    16
}
/// $0848DA: move.l a1, (a6)
pub fn i_0848da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0848DC: rts
pub fn i_0848dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0848de  ($0848DE, 76 B, enemies/squad/rescue_squad_084836.s)
/// $0848DE: movea.l $c(a6), a0
pub fn i_0848de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848E2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0848E2: addq.b #$1, $21(a0)
pub fn i_0848e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848E6;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0848E6: jsr $267e2.l
pub fn i_0848e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x848EC);
    cpu.pc = 0x267E2;
    16
}
/// $0848EC: move.w #$fff0, $2e(a6)
pub fn i_0848ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0848F2: lea.l $848f8(pc), a1
pub fn i_0848f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x848F8;
    16
}
/// $0848F6: move.l a1, (a6)
pub fn i_0848f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0848F8: jsr $863a0(pc)
pub fn i_0848f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x848FC);
    cpu.pc = 0x863A0;
    16
}
/// $0848FC: jsr $27cee.l
pub fn i_0848fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84902;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84902);
    cpu.pc = 0x27CEE;
    16
}
/// $084902: bcc.w $8492a
pub fn i_084902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84906;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8492A; return 10; }
    8
}
/// $084906: jsr $434ce.l
pub fn i_084906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8490C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8490C);
    cpu.pc = 0x434CE;
    16
}
/// $08490C: lea.l $2e993c.l, a1
pub fn i_08490c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84912;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E993C;
    20
}
/// $084912: jsr $77c7e.l
pub fn i_084912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84918;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84918);
    cpu.pc = 0x77C7E;
    16
}
/// $084918: bset.b #$6, $12(a0)
pub fn i_084918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8491E;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08491E: subi.w #$40, $24(a0)
pub fn i_08491e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84924;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084924: lea.l $84932(pc), a1
pub fn i_084924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84928;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84932;
    16
}
/// $084928: move.l a1, (a6)
pub fn i_084928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8492A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_08492a  ($08492A, 8 B, thunks/jsr_abs_thunks.c)
/// $08492A: jsr $28d70.l
pub fn i_08492a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84930);
    cpu.pc = 0x28D70;
    16
}
/// $084930: rts
pub fn i_084930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84932;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084932  ($084932, 44 B, enemies/squad/rescue_squad_084836.s)
/// $084932: jsr $8601c(pc)
pub fn i_084932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84936;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84936);
    cpu.pc = 0x8601C;
    16
}
/// $084936: lea.l $8493c(pc), a1
pub fn i_084936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8493A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8493C;
    16
}
/// $08493A: move.l a1, (a6)
pub fn i_08493a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8493C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08493C: jsr $863a0(pc)
pub fn i_08493c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84940;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84940);
    cpu.pc = 0x863A0;
    16
}
/// $084940: jsr $27cee.l
pub fn i_084940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84946;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84946);
    cpu.pc = 0x27CEE;
    16
}
/// $084946: jsr $28d70.l
pub fn i_084946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8494C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8494C);
    cpu.pc = 0x28D70;
    16
}
/// $08494C: cmpi.w #$40, $24(a6)
pub fn i_08494c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84952;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Word, false);
    16
}
/// $084952: bgt.w $8495c
pub fn i_084952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84956;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8495C; return 10; }
    8
}
/// $084956: jmp $518.l
pub fn i_084956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8495C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08495C: rts
pub fn i_08495c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8495E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_0008495E  ($08495E, 92 B, enemies/squad/rescue_squad_084836.s)
/// $08495E: move.w #$85, d1
pub fn i_08495e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84962;
    cpu.ir = 0x323C;
    let v1: u32 = 0x85;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084962: jsr $236e.l
pub fn i_084962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84968;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84968);
    cpu.pc = 0x236E;
    16
}
/// $084968: move.b #$ff, $32(a6)
pub fn i_084968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8496E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08496E: move.b #$ff, $33(a6)
pub fn i_08496e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84974;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084974: move.b #$0, $3a(a6)
pub fn i_084974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8497A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08497A: lea.l $2c0218.l, a0
pub fn i_08497a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84980;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0218;
    20
}
/// $084980: jsr $799de.l
pub fn i_084980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84986;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84986);
    cpu.pc = 0x799DE;
    16
}
/// $084986: move.w d0, $66(a6)
pub fn i_084986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8498A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08498A: move.w #$0, $38(a6)
pub fn i_08498a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84990;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084990: addi.w #$98, $22(a6)
pub fn i_084990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84996;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x98, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084996: subi.w #$8, $24(a6)
pub fn i_084996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8499C;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08499C: move.b #$0, $20(a6)
pub fn i_08499c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849A2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0849A2: lea.l $849a8(pc), a1
pub fn i_0849a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x849A8;
    16
}
/// $0849A6: move.l a1, (a6)
pub fn i_0849a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0849A8: jsr $2783a.l
pub fn i_0849a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x849AE);
    cpu.pc = 0x2783A;
    16
}
/// $0849AE: cmpi.w #$150, $22(a6)
pub fn i_0849ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849B4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $0849B4: ble.w $849ba
pub fn i_0849b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849B8;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x849BA; return 10; }
    8
}
/// $0849B8: rts
pub fn i_0849b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0849ba  ($0849BA, 240 B, enemies/squad/rescue_squad_084836.s)
/// $0849BA: lea.l $2e7566.l, a0
pub fn i_0849ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7566;
    20
}
/// $0849C0: jsr $28cd4.l
pub fn i_0849c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x849C6);
    cpu.pc = 0x28CD4;
    16
}
/// $0849C6: lea.l $ffff.w, a0
pub fn i_0849c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849CA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0849CA: move.l a0, $48(a6)
pub fn i_0849ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849CE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0849CE: lea.l $84aaa(pc), a1
pub fn i_0849ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84AAA;
    16
}
/// $0849D2: jsr $4ae.l
pub fn i_0849d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x849D8);
    cpu.pc = 0x4AE;
    16
}
/// $0849D8: jsr $5dd22.l
pub fn i_0849d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x849DE);
    cpu.pc = 0x5DD22;
    16
}
/// $0849DE: lea.l $6aa14.l, a1
pub fn i_0849de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x6AA14;
    20
}
/// $0849E4: jsr $4ae.l
pub fn i_0849e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x849EA);
    cpu.pc = 0x4AE;
    16
}
/// $0849EA: jsr $5dd22.l
pub fn i_0849ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x849F0);
    cpu.pc = 0x5DD22;
    16
}
/// $0849F0: addi.w #$65, $22(a0)
pub fn i_0849f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849F6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x65, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0849F6: move.b #$1, $3a(a0)
pub fn i_0849f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849FC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0849FC: move.b #$12, $98(a0)
pub fn i_0849fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A02;
    cpu.ir = 0x117C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084A02: move.l #$2eb06c, $60(a6)
pub fn i_084a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A0A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2EB06C;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084A0A: lea.l $84a10(pc), a1
pub fn i_084a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84A10;
    16
}
/// $084A0E: move.l a1, (a6)
pub fn i_084a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084A10: jsr $28998.l
pub fn i_084a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A16);
    cpu.pc = 0x28998;
    16
}
/// $084A16: jsr $2783a.l
pub fn i_084a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A1C);
    cpu.pc = 0x2783A;
    16
}
/// $084A1C: jsr $28d70.l
pub fn i_084a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A22);
    cpu.pc = 0x28D70;
    16
}
/// $084A22: jsr $2870a.l
pub fn i_084a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A28);
    cpu.pc = 0x2870A;
    16
}
/// $084A28: bcc.w $84a3e
pub fn i_084a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A2C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84A3E; return 10; }
    8
}
/// $084A2C: lea.l $5e766.l, a0
pub fn i_084a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $084A32: jsr $5e770.l
pub fn i_084a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A38);
    cpu.pc = 0x5E770;
    16
}
/// $084A38: bclr.b #$3, $13(a6)
pub fn i_084a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A3E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084A3E: move.l $106f50.l, d0
pub fn i_084a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A44;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $084A44: swap d0
pub fn i_084a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A46;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $084A46: cmpi.w #$4f0, d0
pub fn i_084a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A4A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x4F0, Size::Word, false);
    8
}
/// $084A4A: blt.w $84a58
pub fn i_084a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A4E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x84A58; return 10; }
    8
}
/// $084A4E: lea.l $2e915a.l, a0
pub fn i_084a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E915A;
    20
}
/// $084A54: move.l a0, $48(a6)
pub fn i_084a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A58;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084A58: jsr $28758.l
pub fn i_084a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A5E);
    cpu.pc = 0x28758;
    16
}
/// $084A5E: bcc.w $84aa8
pub fn i_084a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A62;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84AA8; return 10; }
    8
}
/// $084A62: move.b #$ff, $20(a6)
pub fn i_084a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A68;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084A68: move.w #$1028, d0
pub fn i_084a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A6C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084A6C: jsr $2352.l
pub fn i_084a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A72);
    cpu.pc = 0x2352;
    16
}
/// $084A72: lea.l $2e98a2.l, a1
pub fn i_084a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A78;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E98A2;
    20
}
/// $084A78: jsr $77c7e.l
pub fn i_084a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A7E);
    cpu.pc = 0x77C7E;
    16
}
/// $084A7E: lea.l $2e98b4.l, a1
pub fn i_084a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A84;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E98B4;
    20
}
/// $084A84: jsr $77c7e.l
pub fn i_084a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A8A);
    cpu.pc = 0x77C7E;
    16
}
/// $084A8A: lea.l $2e9aee.l, a1
pub fn i_084a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A90;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9AEE;
    20
}
/// $084A90: jsr $77c7e.l
pub fn i_084a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A96);
    cpu.pc = 0x77C7E;
    16
}
/// $084A96: lea.l $2eacc4.l, a1
pub fn i_084a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A9C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EACC4;
    20
}
/// $084A9C: jsr $43fac.l
pub fn i_084a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84AA2);
    cpu.pc = 0x43FAC;
    16
}
/// $084AA2: jmp $518.l
pub fn i_084aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AA8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084AA8: rts
pub fn i_084aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084aaa  ($084AAA, 114 B, enemies/squad/rescue_squad_084836.s)
/// $084AAA: move.w #$e, d1
pub fn i_084aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AAE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084AAE: jsr $236e.l
pub fn i_084aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84AB4);
    cpu.pc = 0x236E;
    16
}
/// $084AB4: move.b #$ff, $32(a6)
pub fn i_084ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84ABA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084ABA: move.b #$ff, $33(a6)
pub fn i_084aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AC0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084AC0: move.b #$1, $3a(a6)
pub fn i_084ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AC6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084AC6: addi.w #$17, $22(a6)
pub fn i_084ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84ACC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x17, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084ACC: addi.w #$28, $24(a6)
pub fn i_084acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AD2;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084AD2: move.w #$0, $38(a6)
pub fn i_084ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AD8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084AD8: lea.l $2c10d8.l, a0
pub fn i_084ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84ADE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C10D8;
    20
}
/// $084ADE: jsr $28cd4.l
pub fn i_084ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84AE4);
    cpu.pc = 0x28CD4;
    16
}
/// $084AE4: lea.l $ffff.w, a0
pub fn i_084ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AE8;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084AE8: move.l a0, $48(a6)
pub fn i_084ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AEC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084AEC: lea.l $84af2(pc), a1
pub fn i_084aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84AF2;
    16
}
/// $084AF0: move.l a1, (a6)
pub fn i_084af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084AF2: jsr $2783a.l
pub fn i_084af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84AF8);
    cpu.pc = 0x2783A;
    16
}
/// $084AF8: jsr $28d70.l
pub fn i_084af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84AFE);
    cpu.pc = 0x28D70;
    16
}
/// $084AFE: movea.l $c(a6), a0
pub fn i_084afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B02;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084B02: cmpi.b #$ff, $20(a0)
pub fn i_084b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B08;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $084B08: bne.w $84b22
pub fn i_084b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B0C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84B22; return 10; }
    8
}
/// $084B0C: lea.l $84c5e(pc), a1
pub fn i_084b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84C5E;
    16
}
/// $084B10: jsr $4ae.l
pub fn i_084b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B16);
    cpu.pc = 0x4AE;
    16
}
/// $084B16: jsr $5dd22.l
pub fn i_084b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B1C);
    cpu.pc = 0x5DD22;
    16
}
// ===== SetTaskHandler_084b1c  ($084B1C, 8 B, kernel/task_handlers.c)
/// $084B1C: lea.l $84b24(pc), a1
pub fn i_084b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84B24;
    16
}
/// $084B20: move.l a1, (a6)
pub fn i_084b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084B22: rts
pub fn i_084b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084b24  ($084B24, 54 B, enemies/squad/rescue_squad_084836.s)
/// $084B24: jsr $267e2.l
pub fn i_084b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B2A);
    cpu.pc = 0x267E2;
    16
}
/// $084B2A: move.w #$ffe0, $2e(a6)
pub fn i_084b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B30;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084B30: lea.l $2b5b92.l, a0
pub fn i_084b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B36;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5B92;
    20
}
/// $084B36: jsr $28cd4.l
pub fn i_084b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B3C);
    cpu.pc = 0x28CD4;
    16
}
/// $084B3C: lea.l $ffff.w, a0
pub fn i_084b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B40;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084B40: move.l a0, $48(a6)
pub fn i_084b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B44;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084B44: lea.l $84b4a(pc), a1
pub fn i_084b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84B4A;
    16
}
/// $084B48: move.l a1, (a6)
pub fn i_084b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084B4A: jsr $27cee.l
pub fn i_084b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B50);
    cpu.pc = 0x27CEE;
    16
}
/// $084B50: bcc.w $84b5a
pub fn i_084b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B54;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84B5A; return 10; }
    8
}
/// $084B54: lea.l $84b62(pc), a1
pub fn i_084b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84B62;
    16
}
/// $084B58: move.l a1, (a6)
pub fn i_084b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_084b5a  ($084B5A, 8 B, thunks/jsr_abs_thunks.c)
/// $084B5A: jsr $28d70.l
pub fn i_084b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B60);
    cpu.pc = 0x28D70;
    16
}
/// $084B60: rts
pub fn i_084b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084b62  ($084B62, 48 B, enemies/squad/rescue_squad_084836.s)
/// $084B62: jsr $267e2.l
pub fn i_084b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B68);
    cpu.pc = 0x267E2;
    16
}
/// $084B68: lea.l $2b5c22.l, a0
pub fn i_084b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5C22;
    20
}
/// $084B6E: jsr $28cd4.l
pub fn i_084b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B74);
    cpu.pc = 0x28CD4;
    16
}
/// $084B74: lea.l $ffff.w, a0
pub fn i_084b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B78;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084B78: move.l a0, $48(a6)
pub fn i_084b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B7C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084B7C: lea.l $84b82(pc), a1
pub fn i_084b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84B82;
    16
}
/// $084B80: move.l a1, (a6)
pub fn i_084b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084B82: jsr $2783a.l
pub fn i_084b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B88);
    cpu.pc = 0x2783A;
    16
}
/// $084B88: jsr $28d70.l
pub fn i_084b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B8E);
    cpu.pc = 0x28D70;
    16
}
/// $084B8E: bcc.w $84b98
pub fn i_084b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84B98; return 10; }
    8
}
// ===== SetTaskHandler_084b92  ($084B92, 8 B, kernel/task_handlers.c)
/// $084B92: lea.l $84b9a(pc), a1
pub fn i_084b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84B9A;
    16
}
/// $084B96: move.l a1, (a6)
pub fn i_084b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084B98: rts
pub fn i_084b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084b9a  ($084B9A, 48 B, enemies/squad/rescue_squad_084836.s)
/// $084B9A: jsr $267e2.l
pub fn i_084b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84BA0);
    cpu.pc = 0x267E2;
    16
}
/// $084BA0: lea.l $2b5cfa.l, a0
pub fn i_084ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5CFA;
    20
}
/// $084BA6: jsr $28cd4.l
pub fn i_084ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84BAC);
    cpu.pc = 0x28CD4;
    16
}
/// $084BAC: lea.l $ffff.w, a0
pub fn i_084bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BB0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084BB0: move.l a0, $48(a6)
pub fn i_084bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BB4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084BB4: lea.l $84bba(pc), a1
pub fn i_084bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84BBA;
    16
}
/// $084BB8: move.l a1, (a6)
pub fn i_084bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084BBA: jsr $2783a.l
pub fn i_084bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84BC0);
    cpu.pc = 0x2783A;
    16
}
/// $084BC0: jsr $28d70.l
pub fn i_084bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84BC6);
    cpu.pc = 0x28D70;
    16
}
/// $084BC6: bcc.w $84bd0
pub fn i_084bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BCA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84BD0; return 10; }
    8
}
// ===== SetTaskHandler_084bca  ($084BCA, 8 B, kernel/task_handlers.c)
/// $084BCA: lea.l $84bd2(pc), a1
pub fn i_084bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84BD2;
    16
}
/// $084BCE: move.l a1, (a6)
pub fn i_084bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084BD0: rts
pub fn i_084bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084bd2  ($084BD2, 76 B, enemies/squad/rescue_squad_084836.s)
/// $084BD2: jsr $267e2.l
pub fn i_084bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84BD8);
    cpu.pc = 0x267E2;
    16
}
/// $084BD8: move.w #$fe80, $28(a6)
pub fn i_084bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BDE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084BDE: move.w #$27, $2a(a6)
pub fn i_084bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BE4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x27;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084BE4: lea.l $2b604a.l, a0
pub fn i_084be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B604A;
    20
}
/// $084BEA: jsr $28cd4.l
pub fn i_084bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84BF0);
    cpu.pc = 0x28CD4;
    16
}
/// $084BF0: lea.l $ffff.w, a0
pub fn i_084bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BF4;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084BF4: move.l a0, $48(a6)
pub fn i_084bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BF8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084BF8: lea.l $84bfe(pc), a1
pub fn i_084bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84BFE;
    16
}
/// $084BFC: move.l a1, (a6)
pub fn i_084bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084BFE: jsr $27cee.l
pub fn i_084bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C04);
    cpu.pc = 0x27CEE;
    16
}
/// $084C04: jsr $28d70.l
pub fn i_084c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C0A);
    cpu.pc = 0x28D70;
    16
}
/// $084C0A: cmpi.w #$84, $22(a6)
pub fn i_084c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C10;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x84, Size::Word, false);
    16
}
/// $084C10: bgt.w $84c24
pub fn i_084c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C14;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x84C24; return 10; }
    8
}
/// $084C14: move.w #$1040, d0
pub fn i_084c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C18;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1040;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084C18: jsr $2352.l
pub fn i_084c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C1E);
    cpu.pc = 0x2352;
    16
}
// ===== SetTaskHandler_084c1e  ($084C1E, 8 B, kernel/task_handlers.c)
/// $084C1E: lea.l $84c26(pc), a1
pub fn i_084c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84C26;
    16
}
/// $084C22: move.l a1, (a6)
pub fn i_084c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084C24: rts
pub fn i_084c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084c26  ($084C26, 42 B, enemies/squad/rescue_squad_084836.s)
/// $084C26: lea.l $84c2c(pc), a1
pub fn i_084c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84C2C;
    16
}
/// $084C2A: move.l a1, (a6)
pub fn i_084c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084C2C: jsr $27cee.l
pub fn i_084c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C32);
    cpu.pc = 0x27CEE;
    16
}
/// $084C32: jsr $28d70.l
pub fn i_084c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C38);
    cpu.pc = 0x28D70;
    16
}
/// $084C38: cmpi.w #$64, $22(a6)
pub fn i_084c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C3E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x64, Size::Word, false);
    16
}
/// $084C3E: bgt.w $84c5c
pub fn i_084c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C42;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x84C5C; return 10; }
    8
}
/// $084C42: movea.l $c(a6), a0
pub fn i_084c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C46;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084C46: movea.l $c(a0), a0
pub fn i_084c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C4A;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084C4A: move.b #$ff, $21(a0)
pub fn i_084c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C50;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Jsr5B6ThenJmpScheduler_084c50  ($084C50, 14 B, kernel/jsr5b6_jmp518.c)
/// $084C50: jsr $5b6.l
pub fn i_084c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C56);
    cpu.pc = 0x5B6;
    16
}
/// $084C56: jmp $518.l
pub fn i_084c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C5C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084C5C: rts
pub fn i_084c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084c5e  ($084C5E, 62 B, enemies/squad/rescue_squad_084836.s)
/// $084C5E: move.w #$4, d1
pub fn i_084c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C62;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084C62: jsr $236e.l
pub fn i_084c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C68);
    cpu.pc = 0x236E;
    16
}
/// $084C68: move.b #$ff, $32(a6)
pub fn i_084c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C6E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084C6E: move.b #$ff, $33(a6)
pub fn i_084c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C74;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084C74: lea.l $2e75c0.l, a0
pub fn i_084c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E75C0;
    20
}
/// $084C7A: jsr $28cd4.l
pub fn i_084c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C80);
    cpu.pc = 0x28CD4;
    16
}
/// $084C80: lea.l $84c86(pc), a1
pub fn i_084c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84C86;
    16
}
/// $084C84: move.l a1, (a6)
pub fn i_084c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084C86: movea.l $c(a6), a0
pub fn i_084c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C8A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084C8A: move.w $22(a0), $22(a6)
pub fn i_084c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C90;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $084C90: move.w $24(a0), $24(a6)
pub fn i_084c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C96;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $084C96: addi.w #$1e, $24(a6)
pub fn i_084c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C9C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1E, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== JsrAbsThunk_084c9c  ($084C9C, 8 B, thunks/jsr_abs_thunks.c)
/// $084C9C: jsr $28d70.l
pub fn i_084c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84CA2);
    cpu.pc = 0x28D70;
    16
}
/// $084CA2: rts
pub fn i_084ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084ca4  ($084CA4, 158 B, enemies/squad/rescue_squad_084836.s)
/// $084CA4: movea.l $3c(a6), a1
pub fn i_084ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CA8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $084CA8: jsr $2942a.l
pub fn i_084ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84CAE);
    cpu.pc = 0x2942A;
    16
}
/// $084CAE: move.w #$20, $70(a6)
pub fn i_084cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084CB4: move.w #$a, $66(a6)
pub fn i_084cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CBA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084CBA: move.w $38(a6), d0
pub fn i_084cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CBE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $084CBE: move.b d0, $21(a6)
pub fn i_084cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CC2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084CC2: lea.l $2e9202.l, a0
pub fn i_084cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9202;
    20
}
/// $084CC8: move.l a0, $48(a6)
pub fn i_084cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CCC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084CCC: lea.l $84cd2(pc), a1
pub fn i_084ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84CD2;
    16
}
/// $084CD0: move.l a1, (a6)
pub fn i_084cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084CD2: jsr $2783a.l
pub fn i_084cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84CD8);
    cpu.pc = 0x2783A;
    16
}
/// $084CD8: jsr $2870a.l
pub fn i_084cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84CDE);
    cpu.pc = 0x2870A;
    16
}
/// $084CDE: bcc.w $84cf4
pub fn i_084cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84CF4; return 10; }
    8
}
/// $084CE2: lea.l $5e766.l, a0
pub fn i_084ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CE8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $084CE8: jsr $5e770.l
pub fn i_084ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84CEE);
    cpu.pc = 0x5E770;
    16
}
/// $084CEE: bclr.b #$3, $13(a6)
pub fn i_084cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CF4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084CF4: jsr $28758.l
pub fn i_084cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84CFA);
    cpu.pc = 0x28758;
    16
}
/// $084CFA: bcc.w $84d30
pub fn i_084cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84D30; return 10; }
    8
}
/// $084CFE: move.l #$2000, d0
pub fn i_084cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D04;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $084D04: jsr $51a28.l
pub fn i_084d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D0A);
    cpu.pc = 0x51A28;
    16
}
/// $084D0A: move.w #$1027, d0
pub fn i_084d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D0E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084D0E: jsr $2352.l
pub fn i_084d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D14);
    cpu.pc = 0x2352;
    16
}
/// $084D14: lea.l $ffff.w, a0
pub fn i_084d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D18;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084D18: move.l a0, $48(a6)
pub fn i_084d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D1C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084D1C: jsr $86504(pc)
pub fn i_084d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84D20);
    cpu.pc = 0x86504;
    16
}
/// $084D20: lea.l $2e9ab8.l, a1
pub fn i_084d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D26;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9AB8;
    20
}
/// $084D26: jsr $77c7e.l
pub fn i_084d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D2C);
    cpu.pc = 0x77C7E;
    16
}
/// $084D2C: bra.w $84d3a
pub fn i_084d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D30;
    cpu.ir = 0x6000;
    cpu.pc = 0x84D3A;
    10
}
/// $084D30: jsr $4fa70.l
pub fn i_084d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D36);
    cpu.pc = 0x4FA70;
    16
}
/// $084D36: bcc.w $84d40
pub fn i_084d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84D40; return 10; }
    8
}
/// $084D3A: jmp $518.l
pub fn i_084d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D40;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084D40: rts
pub fn i_084d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084d42  ($084D42, 110 B, enemies/squad/rescue_squad_084836.s)
/// $084D42: movea.l $3c(a6), a1
pub fn i_084d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D46;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $084D46: jsr $2942a.l
pub fn i_084d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D4C);
    cpu.pc = 0x2942A;
    16
}
/// $084D4C: move.w #$a8, d1
pub fn i_084d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D50;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084D50: jsr $236e.l
pub fn i_084d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D56);
    cpu.pc = 0x236E;
    16
}
/// $084D56: move.w #$40, $70(a6)
pub fn i_084d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D5C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084D5C: move.b #$ff, $32(a6)
pub fn i_084d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D62;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084D62: move.b #$ff, $33(a6)
pub fn i_084d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D68;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084D68: move.b #$0, $3a(a6)
pub fn i_084d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D6E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084D6E: lea.l $2e77ba.l, a0
pub fn i_084d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E77BA;
    20
}
/// $084D74: jsr $28cd4.l
pub fn i_084d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D7A);
    cpu.pc = 0x28CD4;
    16
}
/// $084D7A: move.w #$8000, $38(a6)
pub fn i_084d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084D80: jsr $267e2.l
pub fn i_084d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D86);
    cpu.pc = 0x267E2;
    16
}
/// $084D86: jsr $27cee.l
pub fn i_084d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D8C);
    cpu.pc = 0x27CEE;
    16
}
/// $084D8C: lea.l $84d92(pc), a1
pub fn i_084d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84D92;
    16
}
/// $084D90: move.l a1, (a6)
pub fn i_084d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084D92: jsr $2783a.l
pub fn i_084d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D98);
    cpu.pc = 0x2783A;
    16
}
/// $084D98: jsr $28d70.l
pub fn i_084d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D9E);
    cpu.pc = 0x28D70;
    16
}
/// $084D9E: jsr $4fa70.l
pub fn i_084d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84DA4);
    cpu.pc = 0x4FA70;
    16
}
/// $084DA4: bcc.w $84dae
pub fn i_084da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DA8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84DAE; return 10; }
    8
}
/// $084DA8: jmp $518.l
pub fn i_084da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DAE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084DAE: rts
pub fn i_084dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084db0  ($084DB0, 274 B, enemies/squad/rescue_squad_084836.s)
/// $084DB0: move.b #$ff, $32(a6)
pub fn i_084db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DB6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084DB6: move.b #$ff, $33(a6)
pub fn i_084db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DBC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084DBC: move.b #$0, $3a(a6)
pub fn i_084dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084DC2: cmpi.b #$0, $98(a6)
pub fn i_084dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DC8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $084DC8: bne.w $84dee
pub fn i_084dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DCC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84DEE; return 10; }
    8
}
/// $084DCC: lea.l $2e92aa.l, a0
pub fn i_084dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DD2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E92AA;
    20
}
/// $084DD2: move.l a0, $48(a6)
pub fn i_084dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DD6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084DD6: lea.l $2e9c12.l, a0
pub fn i_084dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9C12;
    20
}
/// $084DDC: move.l a0, $70(a6)
pub fn i_084ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DE0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084DE0: lea.l $2e9838.l, a0
pub fn i_084de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DE6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9838;
    20
}
/// $084DE6: move.l a0, $74(a6)
pub fn i_084de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DEA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084DEA: bra.w $84e38
pub fn i_084dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x84E38;
    10
}
/// $084DEE: cmpi.b #$1, $98(a6)
pub fn i_084dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DF4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $084DF4: bne.w $84e1a
pub fn i_084df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DF8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84E1A; return 10; }
    8
}
/// $084DF8: lea.l $2e92fe.l, a0
pub fn i_084df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DFE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E92FE;
    20
}
/// $084DFE: move.l a0, $48(a6)
pub fn i_084dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E02;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084E02: lea.l $2e9c24.l, a0
pub fn i_084e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9C24;
    20
}
/// $084E08: move.l a0, $70(a6)
pub fn i_084e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E0C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084E0C: lea.l $2e9842.l, a0
pub fn i_084e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9842;
    20
}
/// $084E12: move.l a0, $74(a6)
pub fn i_084e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E16;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084E16: bra.w $84e38
pub fn i_084e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x84E38;
    10
}
/// $084E1A: lea.l $2e9352.l, a0
pub fn i_084e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E20;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9352;
    20
}
/// $084E20: move.l a0, $48(a6)
pub fn i_084e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E24;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084E24: lea.l $2e9c36.l, a0
pub fn i_084e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E2A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9C36;
    20
}
/// $084E2A: move.l a0, $70(a6)
pub fn i_084e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E2E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084E2E: lea.l $2e984c.l, a0
pub fn i_084e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E984C;
    20
}
/// $084E34: move.l a0, $74(a6)
pub fn i_084e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E38;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084E38: clr.w d0
pub fn i_084e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E3A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $084E3A: move.b $9a(a6), d0
pub fn i_084e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E3E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084E3E: move.w d0, $66(a6)
pub fn i_084e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E42;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084E42: jsr $267e2.l
pub fn i_084e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E48);
    cpu.pc = 0x267E2;
    16
}
/// $084E48: lea.l $2e6bf0.l, a0
pub fn i_084e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E4E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6BF0;
    20
}
/// $084E4E: jsr $28cd4.l
pub fn i_084e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E54);
    cpu.pc = 0x28CD4;
    16
}
/// $084E54: lea.l $84e5a(pc), a1
pub fn i_084e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84E5A;
    16
}
/// $084E58: move.l a1, (a6)
pub fn i_084e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084E5A: jsr $2783a.l
pub fn i_084e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E60);
    cpu.pc = 0x2783A;
    16
}
/// $084E60: jsr $28d70.l
pub fn i_084e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E66);
    cpu.pc = 0x28D70;
    16
}
/// $084E66: jsr $2870a.l
pub fn i_084e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E6C);
    cpu.pc = 0x2870A;
    16
}
/// $084E6C: bcc.w $84e82
pub fn i_084e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84E82; return 10; }
    8
}
/// $084E70: lea.l $5e766.l, a0
pub fn i_084e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $084E76: jsr $5e770.l
pub fn i_084e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E7C);
    cpu.pc = 0x5E770;
    16
}
/// $084E7C: bclr.b #$3, $13(a6)
pub fn i_084e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E82;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084E82: jsr $28758.l
pub fn i_084e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E88);
    cpu.pc = 0x28758;
    16
}
/// $084E88: bcc.w $84eac
pub fn i_084e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E8C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84EAC; return 10; }
    8
}
/// $084E8C: cmpi.b #$0, $99(a6)
pub fn i_084e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E92;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $084E92: beq.w $84eba
pub fn i_084e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E96;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84EBA; return 10; }
    8
}
/// $084E96: movea.l $70(a6), a1
pub fn i_084e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E9A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $084E9A: jsr $77c7e.l
pub fn i_084e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84EA0);
    cpu.pc = 0x77C7E;
    16
}
/// $084EA0: subi.b #$1, $99(a6)
pub fn i_084ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EA6;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $084EA6: beq.w $84eba
pub fn i_084ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EAA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84EBA; return 10; }
    8
}
/// $084EAA: bra.b $84e96
pub fn i_084eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EAC;
    cpu.ir = 0x60EA;
    cpu.pc = 0x84E96;
    10
}
/// $084EAC: movea.l $74(a6), a0
pub fn i_084eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EB0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084EB0: jsr $5dd5c.l
pub fn i_084eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84EB6);
    cpu.pc = 0x5DD5C;
    16
}
/// $084EB6: bcc.w $84ec0
pub fn i_084eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84EC0; return 10; }
    8
}
/// $084EBA: jmp $518.l
pub fn i_084eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EC0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084EC0: rts
pub fn i_084ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084ec2  ($084EC2, 100 B, enemies/squad/rescue_squad_084836.s)
/// $084EC2: movea.l $3c(a6), a1
pub fn i_084ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EC6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $084EC6: jsr $2942a.l
pub fn i_084ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84ECC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84ECC);
    cpu.pc = 0x2942A;
    16
}
/// $084ECC: lea.l $2e7812.l, a0
pub fn i_084ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84ED2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7812;
    20
}
/// $084ED2: jsr $28cd4.l
pub fn i_084ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84ED8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84ED8);
    cpu.pc = 0x28CD4;
    16
}
/// $084ED8: move.b #$ff, $32(a6)
pub fn i_084ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EDE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084EDE: move.b #$ff, $33(a6)
pub fn i_084ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EE4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084EE4: move.b #$0, $3a(a6)
pub fn i_084ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EEA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084EEA: lea.l $84ef0(pc), a1
pub fn i_084eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84EF0;
    16
}
/// $084EEE: move.l a1, (a6)
pub fn i_084eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084EF0: jsr $2783a.l
pub fn i_084ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84EF6);
    cpu.pc = 0x2783A;
    16
}
/// $084EF6: jsr $28d70.l
pub fn i_084ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84EFC);
    cpu.pc = 0x28D70;
    16
}
/// $084EFC: bclr.b #$3, $13(a6)
pub fn i_084efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F02;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084F02: bclr.b #$0, $13(a6)
pub fn i_084f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F08;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084F08: move.w #$7fff, $66(a6)
pub fn i_084f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F0E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084F0E: lea.l $2e9856.l, a0
pub fn i_084f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F14;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9856;
    20
}
/// $084F14: jsr $5dd5c.l
pub fn i_084f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F1A);
    cpu.pc = 0x5DD5C;
    16
}
/// $084F1A: bcc.w $84f24
pub fn i_084f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F1E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84F24; return 10; }
    8
}
/// $084F1E: jmp $518.l
pub fn i_084f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F24;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084F24: rts
pub fn i_084f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084f26  ($084F26, 40 B, enemies/squad/rescue_squad_084836.s)
/// $084F26: move.w #$1b1, d1
pub fn i_084f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F2A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084F2A: jsr $236e.l
pub fn i_084f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F30);
    cpu.pc = 0x236E;
    16
}
/// $084F30: move.w #$0, $72(a6)
pub fn i_084f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F36;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084F36: move.w #$2000, $38(a6)
pub fn i_084f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F3C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084F3C: jsr $267e2.l
pub fn i_084f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F42);
    cpu.pc = 0x267E2;
    16
}
/// $084F42: jsr $85f44(pc)
pub fn i_084f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84F46);
    cpu.pc = 0x85F44;
    16
}
/// $084F46: jsr $85f60(pc)
pub fn i_084f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84F4A);
    cpu.pc = 0x85F60;
    16
}
/// $084F4A: bcc.w $84f56
pub fn i_084f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84F56; return 10; }
    8
}
// ===== SetTaskHandler_084f4e  ($084F4E, 8 B, kernel/task_handlers.c)
/// $084F4E: lea.l $84fca(pc), a1
pub fn i_084f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84FCA;
    16
}
/// $084F52: move.l a1, (a6)
pub fn i_084f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084F54: rts
pub fn i_084f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_084f56  ($084F56, 8 B, kernel/task_handlers.c)
/// $084F56: lea.l $84f5e(pc), a1
pub fn i_084f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84F5E;
    16
}
/// $084F5A: move.l a1, (a6)
pub fn i_084f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084F5C: rts
pub fn i_084f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084f5e  ($084F5E, 108 B, enemies/squad/rescue_squad_084836.s)
/// $084F5E: move.w #$1074, d0
pub fn i_084f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F62;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1074;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084F62: jsr $2352.l
pub fn i_084f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F68);
    cpu.pc = 0x2352;
    16
}
/// $084F68: lea.l $2e7828.l, a0
pub fn i_084f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7828;
    20
}
/// $084F6E: jsr $28cd4.l
pub fn i_084f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F74);
    cpu.pc = 0x28CD4;
    16
}
/// $084F74: clr.w d0
pub fn i_084f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F76;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $084F76: move.b $9a(a6), d0
pub fn i_084f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F7A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084F7A: asl.w #$3, d0
pub fn i_084f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F7C;
    cpu.ir = 0xE740;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $084F7C: move.w d0, $2a(a6)
pub fn i_084f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F80;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084F80: lea.l $84f86(pc), a1
pub fn i_084f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84F86;
    16
}
/// $084F84: move.l a1, (a6)
pub fn i_084f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084F86: jsr $27cee.l
pub fn i_084f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F8C);
    cpu.pc = 0x27CEE;
    16
}
/// $084F8C: jsr $85ee8(pc)
pub fn i_084f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F90;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84F90);
    cpu.pc = 0x85EE8;
    16
}
/// $084F90: jsr $85f08(pc)
pub fn i_084f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84F94);
    cpu.pc = 0x85F08;
    16
}
/// $084F94: jsr $28d70.l
pub fn i_084f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F9A);
    cpu.pc = 0x28D70;
    16
}
/// $084F9A: jsr $85f44(pc)
pub fn i_084f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84F9E);
    cpu.pc = 0x85F44;
    16
}
/// $084F9E: jsr $85f60(pc)
pub fn i_084f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84FA2);
    cpu.pc = 0x85F60;
    16
}
/// $084FA2: bcc.w $84fac
pub fn i_084fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FA6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84FAC; return 10; }
    8
}
/// $084FA6: lea.l $85062(pc), a1
pub fn i_084fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FAA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85062;
    16
}
/// $084FAA: move.l a1, (a6)
pub fn i_084faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FAC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084FAC: movea.l #$ffffffff, a0
pub fn i_084fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FB2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $084FB2: lea.l $2e9860.l, a0
pub fn i_084fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9860;
    20
}
/// $084FB8: jsr $5dd5c.l
pub fn i_084fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84FBE);
    cpu.pc = 0x5DD5C;
    16
}
/// $084FBE: bcc.w $84fc8
pub fn i_084fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84FC8; return 10; }
    8
}
/// $084FC2: jmp $518.l
pub fn i_084fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FC8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084FC8: rts
pub fn i_084fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084fca  ($084FCA, 152 B, enemies/squad/rescue_squad_084836.s)
/// $084FCA: move.w #$1074, d0
pub fn i_084fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FCE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1074;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084FCE: jsr $2352.l
pub fn i_084fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84FD4);
    cpu.pc = 0x2352;
    16
}
/// $084FD4: lea.l $2e7866.l, a0
pub fn i_084fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FDA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7866;
    20
}
/// $084FDA: jsr $28cd4.l
pub fn i_084fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84FE0);
    cpu.pc = 0x28CD4;
    16
}
/// $084FE0: clr.w d0
pub fn i_084fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FE2;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $084FE2: move.b $9a(a6), d0
pub fn i_084fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FE6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084FE6: asl.w #$3, d0
pub fn i_084fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FE8;
    cpu.ir = 0xE740;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $084FE8: neg.w d0
pub fn i_084fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FEA;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $084FEA: move.w d0, $2a(a6)
pub fn i_084fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FEE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084FEE: move.l $106f5c.l, d0
pub fn i_084fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FF4;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $084FF4: swap d0
pub fn i_084ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FF6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $084FF6: cmpi.w #$108, d0
pub fn i_084ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FFA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x108, Size::Word, false);
    8
}
/// $084FFA: bgt.w $85018
pub fn i_084ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FFE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x85018; return 10; }
    8
}
/// $084FFE: jsr $6f0.l
pub fn i_084ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85004;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85004);
    cpu.pc = 0x6F0;
    16
}
/// $085004: bcs.w $85018
pub fn i_085004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85008;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x85018; return 10; }
    8
}
/// $085008: lea.l $8512c(pc), a1
pub fn i_085008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8500C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8512C;
    16
}
/// $08500C: jsr $4ae.l
pub fn i_08500c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85012;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85012);
    cpu.pc = 0x4AE;
    16
}
/// $085012: jsr $5dd02.l
pub fn i_085012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85018;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85018);
    cpu.pc = 0x5DD02;
    16
}
/// $085018: lea.l $8501e(pc), a1
pub fn i_085018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8501C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8501E;
    16
}
/// $08501C: move.l a1, (a6)
pub fn i_08501c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8501E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08501E: jsr $27cee.l
pub fn i_08501e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85024;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85024);
    cpu.pc = 0x27CEE;
    16
}
/// $085024: jsr $85ee8(pc)
pub fn i_085024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85028;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85028);
    cpu.pc = 0x85EE8;
    16
}
/// $085028: jsr $85f08(pc)
pub fn i_085028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8502C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8502C);
    cpu.pc = 0x85F08;
    16
}
/// $08502C: jsr $28d70.l
pub fn i_08502c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85032;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85032);
    cpu.pc = 0x28D70;
    16
}
/// $085032: jsr $85f44(pc)
pub fn i_085032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85036;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85036);
    cpu.pc = 0x85F44;
    16
}
/// $085036: jsr $85f60(pc)
pub fn i_085036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8503A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8503A);
    cpu.pc = 0x85F60;
    16
}
/// $08503A: bcs.w $85044
pub fn i_08503a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8503E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x85044; return 10; }
    8
}
/// $08503E: lea.l $85062(pc), a1
pub fn i_08503e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85042;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85062;
    16
}
/// $085042: move.l a1, (a6)
pub fn i_085042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85044;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085044: movea.l #$ffffffff, a0
pub fn i_085044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8504A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08504A: lea.l $2e9860.l, a0
pub fn i_08504a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85050;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9860;
    20
}
/// $085050: jsr $5dd5c.l
pub fn i_085050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85056;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85056);
    cpu.pc = 0x5DD5C;
    16
}
/// $085056: bcc.w $85060
pub fn i_085056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8505A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85060; return 10; }
    8
}
/// $08505A: jmp $518.l
pub fn i_08505a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85060;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085060: rts
pub fn i_085060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85062;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_085062  ($085062, 100 B, enemies/squad/rescue_squad_084836.s)
/// $085062: move.w #$1075, d0
pub fn i_085062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85066;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1075;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085066: jsr $2352.l
pub fn i_085066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8506C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8506C);
    cpu.pc = 0x2352;
    16
}
/// $08506C: lea.l $2e78a4.l, a0
pub fn i_08506c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85072;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E78A4;
    20
}
/// $085072: jsr $28cd4.l
pub fn i_085072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85078);
    cpu.pc = 0x28CD4;
    16
}
/// $085078: move.w #$a, $72(a6)
pub fn i_085078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8507E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08507E: lea.l $85084(pc), a1
pub fn i_08507e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85082;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85084;
    16
}
/// $085082: move.l a1, (a6)
pub fn i_085082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85084;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085084: jsr $2783a.l
pub fn i_085084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8508A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8508A);
    cpu.pc = 0x2783A;
    16
}
/// $08508A: jsr $85f08(pc)
pub fn i_08508a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8508E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8508E);
    cpu.pc = 0x85F08;
    16
}
/// $08508E: jsr $28d70.l
pub fn i_08508e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85094;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85094);
    cpu.pc = 0x28D70;
    16
}
/// $085094: jsr $85f44(pc)
pub fn i_085094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85098;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85098);
    cpu.pc = 0x85F44;
    16
}
/// $085098: cmpi.w #$0, $72(a6)
pub fn i_085098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8509E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08509E: bgt.w $850a8
pub fn i_08509e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850A2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x850A8; return 10; }
    8
}
/// $0850A2: lea.l $850c6(pc), a1
pub fn i_0850a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x850C6;
    16
}
/// $0850A6: move.l a1, (a6)
pub fn i_0850a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x083448, i_083448),
    (0x08344E, i_08344e),
    (0x083452, i_083452),
    (0x083458, i_083458),
    (0x08345C, i_08345c),
    (0x08345E, i_08345e),
    (0x083464, i_083464),
    (0x083468, i_083468),
    (0x08346E, i_08346e),
    (0x083474, i_083474),
    (0x083478, i_083478),
    (0x08347C, i_08347c),
    (0x083480, i_083480),
    (0x083482, i_083482),
    (0x083486, i_083486),
    (0x08348C, i_08348c),
    (0x083492, i_083492),
    (0x083498, i_083498),
    (0x08349C, i_08349c),
    (0x0834A2, i_0834a2),
    (0x0834A4, i_0834a4),
    (0x0834A8, i_0834a8),
    (0x0834AE, i_0834ae),
    (0x0834B2, i_0834b2),
    (0x0834B8, i_0834b8),
    (0x0834BE, i_0834be),
    (0x0834C4, i_0834c4),
    (0x0834CA, i_0834ca),
    (0x0834D0, i_0834d0),
    (0x0834D6, i_0834d6),
    (0x0834DC, i_0834dc),
    (0x0834E2, i_0834e2),
    (0x0834E6, i_0834e6),
    (0x0834E8, i_0834e8),
    (0x0834EE, i_0834ee),
    (0x0834F4, i_0834f4),
    (0x0834FA, i_0834fa),
    (0x083500, i_083500),
    (0x083504, i_083504),
    (0x08350A, i_08350a),
    (0x083510, i_083510),
    (0x083516, i_083516),
    (0x08351C, i_08351c),
    (0x083520, i_083520),
    (0x083526, i_083526),
    (0x08352C, i_08352c),
    (0x083530, i_083530),
    (0x083536, i_083536),
    (0x08353A, i_08353a),
    (0x08353C, i_08353c),
    (0x083542, i_083542),
    (0x083546, i_083546),
    (0x08354C, i_08354c),
    (0x08354E, i_08354e),
    (0x083554, i_083554),
    (0x08355A, i_08355a),
    (0x08355E, i_08355e),
    (0x083562, i_083562),
    (0x083568, i_083568),
    (0x08356E, i_08356e),
    (0x083572, i_083572),
    (0x083574, i_083574),
    (0x08357A, i_08357a),
    (0x083580, i_083580),
    (0x083584, i_083584),
    (0x08358A, i_08358a),
    (0x08358E, i_08358e),
    (0x083594, i_083594),
    (0x083596, i_083596),
    (0x08359A, i_08359a),
    (0x0835A0, i_0835a0),
    (0x0835A4, i_0835a4),
    (0x0835AA, i_0835aa),
    (0x0835B0, i_0835b0),
    (0x0835B6, i_0835b6),
    (0x0835BC, i_0835bc),
    (0x0835C2, i_0835c2),
    (0x0835C8, i_0835c8),
    (0x0835CE, i_0835ce),
    (0x0835D2, i_0835d2),
    (0x0835D4, i_0835d4),
    (0x0835DA, i_0835da),
    (0x0835E0, i_0835e0),
    (0x0835E6, i_0835e6),
    (0x0835EA, i_0835ea),
    (0x0835F0, i_0835f0),
    (0x0835F2, i_0835f2),
    (0x0835F6, i_0835f6),
    (0x0835FC, i_0835fc),
    (0x083602, i_083602),
    (0x083608, i_083608),
    (0x08360E, i_08360e),
    (0x083612, i_083612),
    (0x083616, i_083616),
    (0x08361C, i_08361c),
    (0x083622, i_083622),
    (0x083628, i_083628),
    (0x08362E, i_08362e),
    (0x083632, i_083632),
    (0x083636, i_083636),
    (0x08363C, i_08363c),
    (0x083642, i_083642),
    (0x083648, i_083648),
    (0x08364E, i_08364e),
    (0x083652, i_083652),
    (0x083658, i_083658),
    (0x08365E, i_08365e),
    (0x083662, i_083662),
    (0x083666, i_083666),
    (0x083668, i_083668),
    (0x08366C, i_08366c),
    (0x083672, i_083672),
    (0x083678, i_083678),
    (0x08367E, i_08367e),
    (0x083684, i_083684),
    (0x08368A, i_08368a),
    (0x083690, i_083690),
    (0x083696, i_083696),
    (0x08369C, i_08369c),
    (0x0836A0, i_0836a0),
    (0x0836A6, i_0836a6),
    (0x0836AC, i_0836ac),
    (0x0836B0, i_0836b0),
    (0x0836B6, i_0836b6),
    (0x0836BA, i_0836ba),
    (0x0836C0, i_0836c0),
    (0x0836C4, i_0836c4),
    (0x0836C6, i_0836c6),
    (0x0836CC, i_0836cc),
    (0x0836D2, i_0836d2),
    (0x0836D6, i_0836d6),
    (0x0836DC, i_0836dc),
    (0x0836E0, i_0836e0),
    (0x0836E6, i_0836e6),
    (0x0836EC, i_0836ec),
    (0x0836F2, i_0836f2),
    (0x0836F8, i_0836f8),
    (0x0836FC, i_0836fc),
    (0x083700, i_083700),
    (0x083702, i_083702),
    (0x083708, i_083708),
    (0x08370C, i_08370c),
    (0x083712, i_083712),
    (0x083714, i_083714),
    (0x08371A, i_08371a),
    (0x083720, i_083720),
    (0x083726, i_083726),
    (0x08372A, i_08372a),
    (0x083730, i_083730),
    (0x083736, i_083736),
    (0x08373A, i_08373a),
    (0x08373C, i_08373c),
    (0x083742, i_083742),
    (0x083748, i_083748),
    (0x08374C, i_08374c),
    (0x083752, i_083752),
    (0x083756, i_083756),
    (0x08375C, i_08375c),
    (0x083762, i_083762),
    (0x083768, i_083768),
    (0x08376E, i_08376e),
    (0x083772, i_083772),
    (0x083776, i_083776),
    (0x083778, i_083778),
    (0x08377E, i_08377e),
    (0x083782, i_083782),
    (0x083788, i_083788),
    (0x08378A, i_08378a),
    (0x083790, i_083790),
    (0x083796, i_083796),
    (0x08379C, i_08379c),
    (0x0837A0, i_0837a0),
    (0x0837A6, i_0837a6),
    (0x0837AC, i_0837ac),
    (0x0837B0, i_0837b0),
    (0x0837B2, i_0837b2),
    (0x0837B8, i_0837b8),
    (0x0837BE, i_0837be),
    (0x0837C2, i_0837c2),
    (0x0837C8, i_0837c8),
    (0x0837CC, i_0837cc),
    (0x0837D2, i_0837d2),
    (0x0837D8, i_0837d8),
    (0x0837DE, i_0837de),
    (0x0837E4, i_0837e4),
    (0x0837E8, i_0837e8),
    (0x0837EC, i_0837ec),
    (0x0837EE, i_0837ee),
    (0x0837F4, i_0837f4),
    (0x0837F8, i_0837f8),
    (0x0837FE, i_0837fe),
    (0x083800, i_083800),
    (0x083806, i_083806),
    (0x08380C, i_08380c),
    (0x083812, i_083812),
    (0x083816, i_083816),
    (0x08381A, i_08381a),
    (0x08381E, i_08381e),
    (0x083824, i_083824),
    (0x083828, i_083828),
    (0x08382C, i_08382c),
    (0x083832, i_083832),
    (0x083836, i_083836),
    (0x08383C, i_08383c),
    (0x083842, i_083842),
    (0x083846, i_083846),
    (0x08384C, i_08384c),
    (0x083852, i_083852),
    (0x083858, i_083858),
    (0x08385E, i_08385e),
    (0x083864, i_083864),
    (0x08386A, i_08386a),
    (0x08386E, i_08386e),
    (0x083874, i_083874),
    (0x08387A, i_08387a),
    (0x083880, i_083880),
    (0x083884, i_083884),
    (0x083888, i_083888),
    (0x08388E, i_08388e),
    (0x083892, i_083892),
    (0x083898, i_083898),
    (0x08389E, i_08389e),
    (0x0838A2, i_0838a2),
    (0x0838A8, i_0838a8),
    (0x0838AC, i_0838ac),
    (0x0838B0, i_0838b0),
    (0x0838B6, i_0838b6),
    (0x0838BC, i_0838bc),
    (0x0838C0, i_0838c0),
    (0x0838C6, i_0838c6),
    (0x0838CA, i_0838ca),
    (0x0838CC, i_0838cc),
    (0x0838D0, i_0838d0),
    (0x0838D6, i_0838d6),
    (0x0838DA, i_0838da),
    (0x0838E0, i_0838e0),
    (0x0838E4, i_0838e4),
    (0x0838E6, i_0838e6),
    (0x0838EC, i_0838ec),
    (0x0838F0, i_0838f0),
    (0x0838F4, i_0838f4),
    (0x0838FA, i_0838fa),
    (0x083900, i_083900),
    (0x083906, i_083906),
    (0x08390A, i_08390a),
    (0x08390E, i_08390e),
    (0x083912, i_083912),
    (0x083914, i_083914),
    (0x08391A, i_08391a),
    (0x08391E, i_08391e),
    (0x083924, i_083924),
    (0x083926, i_083926),
    (0x08392C, i_08392c),
    (0x083932, i_083932),
    (0x083938, i_083938),
    (0x08393E, i_08393e),
    (0x083942, i_083942),
    (0x083944, i_083944),
    (0x08394A, i_08394a),
    (0x083950, i_083950),
    (0x083954, i_083954),
    (0x083958, i_083958),
    (0x08395E, i_08395e),
    (0x083962, i_083962),
    (0x083968, i_083968),
    (0x08396A, i_08396a),
    (0x08396E, i_08396e),
    (0x083974, i_083974),
    (0x083978, i_083978),
    (0x08397E, i_08397e),
    (0x083984, i_083984),
    (0x08398A, i_08398a),
    (0x083990, i_083990),
    (0x083996, i_083996),
    (0x08399C, i_08399c),
    (0x0839A2, i_0839a2),
    (0x0839A8, i_0839a8),
    (0x0839AC, i_0839ac),
    (0x0839AE, i_0839ae),
    (0x0839B4, i_0839b4),
    (0x0839BA, i_0839ba),
    (0x0839C0, i_0839c0),
    (0x0839C6, i_0839c6),
    (0x0839CA, i_0839ca),
    (0x0839CE, i_0839ce),
    (0x0839D0, i_0839d0),
    (0x0839D6, i_0839d6),
    (0x0839DA, i_0839da),
    (0x0839E0, i_0839e0),
    (0x0839E2, i_0839e2),
    (0x0839E6, i_0839e6),
    (0x0839EC, i_0839ec),
    (0x0839F2, i_0839f2),
    (0x0839F6, i_0839f6),
    (0x0839FC, i_0839fc),
    (0x083A00, i_083a00),
    (0x083A04, i_083a04),
    (0x083A08, i_083a08),
    (0x083A0A, i_083a0a),
    (0x083A10, i_083a10),
    (0x083A16, i_083a16),
    (0x083A1A, i_083a1a),
    (0x083A1E, i_083a1e),
    (0x083A22, i_083a22),
    (0x083A24, i_083a24),
    (0x083A2A, i_083a2a),
    (0x083A2E, i_083a2e),
    (0x083A34, i_083a34),
    (0x083A36, i_083a36),
    (0x083A3A, i_083a3a),
    (0x083A40, i_083a40),
    (0x083A44, i_083a44),
    (0x083A4A, i_083a4a),
    (0x083A50, i_083a50),
    (0x083A56, i_083a56),
    (0x083A5C, i_083a5c),
    (0x083A62, i_083a62),
    (0x083A68, i_083a68),
    (0x083A6E, i_083a6e),
    (0x083A74, i_083a74),
    (0x083A7A, i_083a7a),
    (0x083A80, i_083a80),
    (0x083A84, i_083a84),
    (0x083A86, i_083a86),
    (0x083A8C, i_083a8c),
    (0x083A92, i_083a92),
    (0x083A98, i_083a98),
    (0x083A9C, i_083a9c),
    (0x083AA2, i_083aa2),
    (0x083AA4, i_083aa4),
    (0x083AA8, i_083aa8),
    (0x083AAE, i_083aae),
    (0x083AB2, i_083ab2),
    (0x083AB8, i_083ab8),
    (0x083ABE, i_083abe),
    (0x083AC4, i_083ac4),
    (0x083ACA, i_083aca),
    (0x083AD0, i_083ad0),
    (0x083AD6, i_083ad6),
    (0x083ADC, i_083adc),
    (0x083AE2, i_083ae2),
    (0x083AE8, i_083ae8),
    (0x083AEE, i_083aee),
    (0x083AF2, i_083af2),
    (0x083AF4, i_083af4),
    (0x083AFA, i_083afa),
    (0x083B00, i_083b00),
    (0x083B06, i_083b06),
    (0x083B0A, i_083b0a),
    (0x083B10, i_083b10),
    (0x083B12, i_083b12),
    (0x083B16, i_083b16),
    (0x083B1C, i_083b1c),
    (0x083B20, i_083b20),
    (0x083B26, i_083b26),
    (0x083B2C, i_083b2c),
    (0x083B32, i_083b32),
    (0x083B38, i_083b38),
    (0x083B3E, i_083b3e),
    (0x083B44, i_083b44),
    (0x083B4A, i_083b4a),
    (0x083B4E, i_083b4e),
    (0x083B54, i_083b54),
    (0x083B5A, i_083b5a),
    (0x083B5E, i_083b5e),
    (0x083B62, i_083b62),
    (0x083B64, i_083b64),
    (0x083B6A, i_083b6a),
    (0x083B70, i_083b70),
    (0x083B74, i_083b74),
    (0x083B7A, i_083b7a),
    (0x083B80, i_083b80),
    (0x083B84, i_083b84),
    (0x083B8A, i_083b8a),
    (0x083B90, i_083b90),
    (0x083B92, i_083b92),
    (0x083B96, i_083b96),
    (0x083B9C, i_083b9c),
    (0x083BA2, i_083ba2),
    (0x083BA8, i_083ba8),
    (0x083BAE, i_083bae),
    (0x083BB4, i_083bb4),
    (0x083BBA, i_083bba),
    (0x083BBE, i_083bbe),
    (0x083BC0, i_083bc0),
    (0x083BC4, i_083bc4),
    (0x083BCA, i_083bca),
    (0x083BD0, i_083bd0),
    (0x083BD6, i_083bd6),
    (0x083BDA, i_083bda),
    (0x083BE0, i_083be0),
    (0x083BE2, i_083be2),
    (0x083BE6, i_083be6),
    (0x083BEC, i_083bec),
    (0x083BF2, i_083bf2),
    (0x083BF8, i_083bf8),
    (0x083BFE, i_083bfe),
    (0x083C02, i_083c02),
    (0x083C06, i_083c06),
    (0x083C0C, i_083c0c),
    (0x083C12, i_083c12),
    (0x083C18, i_083c18),
    (0x083C1E, i_083c1e),
    (0x083C24, i_083c24),
    (0x083C2A, i_083c2a),
    (0x083C30, i_083c30),
    (0x083C34, i_083c34),
    (0x083C3A, i_083c3a),
    (0x083C40, i_083c40),
    (0x083C44, i_083c44),
    (0x083C46, i_083c46),
    (0x083C4C, i_083c4c),
    (0x083C52, i_083c52),
    (0x083C56, i_083c56),
    (0x083C5C, i_083c5c),
    (0x083C62, i_083c62),
    (0x083C66, i_083c66),
    (0x083C6A, i_083c6a),
    (0x083C70, i_083c70),
    (0x083C76, i_083c76),
    (0x083C7C, i_083c7c),
    (0x083C80, i_083c80),
    (0x083C86, i_083c86),
    (0x083C8A, i_083c8a),
    (0x083C90, i_083c90),
    (0x083C92, i_083c92),
    (0x083C96, i_083c96),
    (0x083C9C, i_083c9c),
    (0x083CA0, i_083ca0),
    (0x083CA6, i_083ca6),
    (0x083CAC, i_083cac),
    (0x083CB2, i_083cb2),
    (0x083CB8, i_083cb8),
    (0x083CBE, i_083cbe),
    (0x083CC4, i_083cc4),
    (0x083CCA, i_083cca),
    (0x083CD0, i_083cd0),
    (0x083CD4, i_083cd4),
    (0x083CD6, i_083cd6),
    (0x083CDC, i_083cdc),
    (0x083CE2, i_083ce2),
    (0x083CE8, i_083ce8),
    (0x083CEC, i_083cec),
    (0x083CF2, i_083cf2),
    (0x083CF8, i_083cf8),
    (0x083CFE, i_083cfe),
    (0x083D04, i_083d04),
    (0x083D08, i_083d08),
    (0x083D0C, i_083d0c),
    (0x083D12, i_083d12),
    (0x083D16, i_083d16),
    (0x083D18, i_083d18),
    (0x083D1E, i_083d1e),
    (0x083D22, i_083d22),
    (0x083D28, i_083d28),
    (0x083D2A, i_083d2a),
    (0x083D2E, i_083d2e),
    (0x083D32, i_083d32),
    (0x083D38, i_083d38),
    (0x083D3E, i_083d3e),
    (0x083D44, i_083d44),
    (0x083D4A, i_083d4a),
    (0x083D4E, i_083d4e),
    (0x083D50, i_083d50),
    (0x083D56, i_083d56),
    (0x083D5C, i_083d5c),
    (0x083D62, i_083d62),
    (0x083D66, i_083d66),
    (0x083D6C, i_083d6c),
    (0x083D6E, i_083d6e),
    (0x083D72, i_083d72),
    (0x083D78, i_083d78),
    (0x083D7E, i_083d7e),
    (0x083D84, i_083d84),
    (0x083D8A, i_083d8a),
    (0x083D8E, i_083d8e),
    (0x083D92, i_083d92),
    (0x083D98, i_083d98),
    (0x083D9E, i_083d9e),
    (0x083DA4, i_083da4),
    (0x083DAA, i_083daa),
    (0x083DB0, i_083db0),
    (0x083DB6, i_083db6),
    (0x083DBC, i_083dbc),
    (0x083DC0, i_083dc0),
    (0x083DC2, i_083dc2),
    (0x083DC8, i_083dc8),
    (0x083DCE, i_083dce),
    (0x083DD4, i_083dd4),
    (0x083DD8, i_083dd8),
    (0x083DDE, i_083dde),
    (0x083DE4, i_083de4),
    (0x083DEA, i_083dea),
    (0x083DF0, i_083df0),
    (0x083DF4, i_083df4),
    (0x083DF8, i_083df8),
    (0x083DFE, i_083dfe),
    (0x083E04, i_083e04),
    (0x083E08, i_083e08),
    (0x083E0C, i_083e0c),
    (0x083E12, i_083e12),
    (0x083E18, i_083e18),
    (0x083E1C, i_083e1c),
    (0x083E20, i_083e20),
    (0x083E26, i_083e26),
    (0x083E2C, i_083e2c),
    (0x083E32, i_083e32),
    (0x083E36, i_083e36),
    (0x083E3C, i_083e3c),
    (0x083E40, i_083e40),
    (0x083E46, i_083e46),
    (0x083E48, i_083e48),
    (0x083E4C, i_083e4c),
    (0x083E52, i_083e52),
    (0x083E56, i_083e56),
    (0x083E5C, i_083e5c),
    (0x083E62, i_083e62),
    (0x083E68, i_083e68),
    (0x083E6E, i_083e6e),
    (0x083E74, i_083e74),
    (0x083E7A, i_083e7a),
    (0x083E80, i_083e80),
    (0x083E86, i_083e86),
    (0x083E8C, i_083e8c),
    (0x083E90, i_083e90),
    (0x083E92, i_083e92),
    (0x083E98, i_083e98),
    (0x083E9E, i_083e9e),
    (0x083EA4, i_083ea4),
    (0x083EA8, i_083ea8),
    (0x083EAE, i_083eae),
    (0x083EB4, i_083eb4),
    (0x083EBA, i_083eba),
    (0x083EBE, i_083ebe),
    (0x083EC0, i_083ec0),
    (0x083EC6, i_083ec6),
    (0x083ECA, i_083eca),
    (0x083ECE, i_083ece),
    (0x083ED4, i_083ed4),
    (0x083ED8, i_083ed8),
    (0x083EDA, i_083eda),
    (0x083EE0, i_083ee0),
    (0x083EE4, i_083ee4),
    (0x083EEA, i_083eea),
    (0x083EEC, i_083eec),
    (0x083EF2, i_083ef2),
    (0x083EF8, i_083ef8),
    (0x083EFC, i_083efc),
    (0x083EFE, i_083efe),
    (0x083F04, i_083f04),
    (0x083F0A, i_083f0a),
    (0x083F0E, i_083f0e),
    (0x083F12, i_083f12),
    (0x083F14, i_083f14),
    (0x083F1A, i_083f1a),
    (0x083F1E, i_083f1e),
    (0x083F24, i_083f24),
    (0x083F26, i_083f26),
    (0x083F2C, i_083f2c),
    (0x083F32, i_083f32),
    (0x083F36, i_083f36),
    (0x083F3A, i_083f3a),
    (0x083F40, i_083f40),
    (0x083F46, i_083f46),
    (0x083F4C, i_083f4c),
    (0x083F52, i_083f52),
    (0x083F58, i_083f58),
    (0x083F5E, i_083f5e),
    (0x083F64, i_083f64),
    (0x083F6A, i_083f6a),
    (0x083F70, i_083f70),
    (0x083F74, i_083f74),
    (0x083F76, i_083f76),
    (0x083F7C, i_083f7c),
    (0x083F80, i_083f80),
    (0x083F86, i_083f86),
    (0x083F8A, i_083f8a),
    (0x083F90, i_083f90),
    (0x083F96, i_083f96),
    (0x083F9C, i_083f9c),
    (0x083FA0, i_083fa0),
    (0x083FA6, i_083fa6),
    (0x083FA8, i_083fa8),
    (0x083FAC, i_083fac),
    (0x083FB2, i_083fb2),
    (0x083FB8, i_083fb8),
    (0x083FBE, i_083fbe),
    (0x083FC2, i_083fc2),
    (0x083FC6, i_083fc6),
    (0x083FC8, i_083fc8),
    (0x083FCE, i_083fce),
    (0x083FD0, i_083fd0),
    (0x083FD6, i_083fd6),
    (0x083FDC, i_083fdc),
    (0x083FE0, i_083fe0),
    (0x083FE2, i_083fe2),
    (0x083FE8, i_083fe8),
    (0x083FEE, i_083fee),
    (0x083FF4, i_083ff4),
    (0x083FFA, i_083ffa),
    (0x083FFE, i_083ffe),
    (0x084000, i_084000),
    (0x084006, i_084006),
    (0x08400C, i_08400c),
    (0x084012, i_084012),
    (0x084016, i_084016),
    (0x08401C, i_08401c),
    (0x084022, i_084022),
    (0x084028, i_084028),
    (0x08402E, i_08402e),
    (0x084032, i_084032),
    (0x084038, i_084038),
    (0x08403E, i_08403e),
    (0x084042, i_084042),
    (0x084048, i_084048),
    (0x08404C, i_08404c),
    (0x08404E, i_08404e),
    (0x084054, i_084054),
    (0x084058, i_084058),
    (0x08405E, i_08405e),
    (0x084060, i_084060),
    (0x084066, i_084066),
    (0x08406C, i_08406c),
    (0x084070, i_084070),
    (0x084074, i_084074),
    (0x084078, i_084078),
    (0x08407C, i_08407c),
    (0x084080, i_084080),
    (0x084086, i_084086),
    (0x08408C, i_08408c),
    (0x084092, i_084092),
    (0x084094, i_084094),
    (0x084098, i_084098),
    (0x08409A, i_08409a),
    (0x08409E, i_08409e),
    (0x0840A2, i_0840a2),
    (0x0840A6, i_0840a6),
    (0x0840AC, i_0840ac),
    (0x0840B0, i_0840b0),
    (0x0840B4, i_0840b4),
    (0x0840B8, i_0840b8),
    (0x0840BE, i_0840be),
    (0x0840C4, i_0840c4),
    (0x0840C8, i_0840c8),
    (0x0840CA, i_0840ca),
    (0x0840D0, i_0840d0),
    (0x0840D4, i_0840d4),
    (0x0840D8, i_0840d8),
    (0x0840DA, i_0840da),
    (0x0840DE, i_0840de),
    (0x0840E2, i_0840e2),
    (0x0840E8, i_0840e8),
    (0x0840EC, i_0840ec),
    (0x0840F0, i_0840f0),
    (0x0840F4, i_0840f4),
    (0x0840F8, i_0840f8),
    (0x0840FC, i_0840fc),
    (0x084102, i_084102),
    (0x084108, i_084108),
    (0x08410C, i_08410c),
    (0x084110, i_084110),
    (0x084112, i_084112),
    (0x084116, i_084116),
    (0x08411A, i_08411a),
    (0x084120, i_084120),
    (0x084124, i_084124),
    (0x084128, i_084128),
    (0x08412C, i_08412c),
    (0x084130, i_084130),
    (0x084134, i_084134),
    (0x08413A, i_08413a),
    (0x084140, i_084140),
    (0x084144, i_084144),
    (0x08414A, i_08414a),
    (0x08414C, i_08414c),
    (0x084150, i_084150),
    (0x084156, i_084156),
    (0x08415A, i_08415a),
    (0x084160, i_084160),
    (0x084166, i_084166),
    (0x08416C, i_08416c),
    (0x084172, i_084172),
    (0x084178, i_084178),
    (0x08417E, i_08417e),
    (0x084184, i_084184),
    (0x08418A, i_08418a),
    (0x08418E, i_08418e),
    (0x084190, i_084190),
    (0x084196, i_084196),
    (0x08419C, i_08419c),
    (0x0841A2, i_0841a2),
    (0x0841A6, i_0841a6),
    (0x0841AA, i_0841aa),
    (0x0841AC, i_0841ac),
    (0x0841B2, i_0841b2),
    (0x0841B6, i_0841b6),
    (0x0841BC, i_0841bc),
    (0x0841BE, i_0841be),
    (0x0841C2, i_0841c2),
    (0x0841C6, i_0841c6),
    (0x0841CC, i_0841cc),
    (0x0841D2, i_0841d2),
    (0x0841D6, i_0841d6),
    (0x0841D8, i_0841d8),
    (0x0841DE, i_0841de),
    (0x0841E4, i_0841e4),
    (0x0841EA, i_0841ea),
    (0x0841EE, i_0841ee),
    (0x0841F4, i_0841f4),
    (0x0841F6, i_0841f6),
    (0x0841FA, i_0841fa),
    (0x084200, i_084200),
    (0x084206, i_084206),
    (0x08420C, i_08420c),
    (0x084212, i_084212),
    (0x084218, i_084218),
    (0x08421C, i_08421c),
    (0x08421E, i_08421e),
    (0x084224, i_084224),
    (0x08422A, i_08422a),
    (0x084230, i_084230),
    (0x084234, i_084234),
    (0x084238, i_084238),
    (0x08423A, i_08423a),
    (0x084240, i_084240),
    (0x084244, i_084244),
    (0x08424A, i_08424a),
    (0x08424C, i_08424c),
    (0x084250, i_084250),
    (0x084254, i_084254),
    (0x084258, i_084258),
    (0x08425C, i_08425c),
    (0x08425E, i_08425e),
    (0x084264, i_084264),
    (0x084266, i_084266),
    (0x08426A, i_08426a),
    (0x084270, i_084270),
    (0x084274, i_084274),
    (0x084278, i_084278),
    (0x08427A, i_08427a),
    (0x084280, i_084280),
    (0x084282, i_084282),
    (0x084288, i_084288),
    (0x08428E, i_08428e),
    (0x084294, i_084294),
    (0x08429A, i_08429a),
    (0x08429E, i_08429e),
    (0x0842A0, i_0842a0),
    (0x0842A6, i_0842a6),
    (0x0842AC, i_0842ac),
    (0x0842B2, i_0842b2),
    (0x0842B6, i_0842b6),
    (0x0842BA, i_0842ba),
    (0x0842BE, i_0842be),
    (0x0842C4, i_0842c4),
    (0x0842C8, i_0842c8),
    (0x0842CE, i_0842ce),
    (0x0842D0, i_0842d0),
    (0x0842D4, i_0842d4),
    (0x0842DA, i_0842da),
    (0x0842E0, i_0842e0),
    (0x0842E6, i_0842e6),
    (0x0842EC, i_0842ec),
    (0x0842F2, i_0842f2),
    (0x0842F6, i_0842f6),
    (0x0842F8, i_0842f8),
    (0x0842FE, i_0842fe),
    (0x084304, i_084304),
    (0x08430A, i_08430a),
    (0x08430E, i_08430e),
    (0x084314, i_084314),
    (0x08431A, i_08431a),
    (0x084320, i_084320),
    (0x084326, i_084326),
    (0x08432A, i_08432a),
    (0x084330, i_084330),
    (0x084336, i_084336),
    (0x08433A, i_08433a),
    (0x084340, i_084340),
    (0x084344, i_084344),
    (0x084348, i_084348),
    (0x08434C, i_08434c),
    (0x084350, i_084350),
    (0x084356, i_084356),
    (0x08435C, i_08435c),
    (0x084362, i_084362),
    (0x084368, i_084368),
    (0x08436E, i_08436e),
    (0x084374, i_084374),
    (0x08437A, i_08437a),
    (0x084380, i_084380),
    (0x084384, i_084384),
    (0x08438A, i_08438a),
    (0x08438E, i_08438e),
    (0x084394, i_084394),
    (0x084396, i_084396),
    (0x08439A, i_08439a),
    (0x0843A0, i_0843a0),
    (0x0843A4, i_0843a4),
    (0x0843AA, i_0843aa),
    (0x0843B0, i_0843b0),
    (0x0843B4, i_0843b4),
    (0x0843B8, i_0843b8),
    (0x0843BC, i_0843bc),
    (0x0843C2, i_0843c2),
    (0x0843C8, i_0843c8),
    (0x0843CE, i_0843ce),
    (0x0843D4, i_0843d4),
    (0x0843DA, i_0843da),
    (0x0843DE, i_0843de),
    (0x0843E0, i_0843e0),
    (0x0843E6, i_0843e6),
    (0x0843EC, i_0843ec),
    (0x0843F2, i_0843f2),
    (0x0843F8, i_0843f8),
    (0x0843FE, i_0843fe),
    (0x084402, i_084402),
    (0x084408, i_084408),
    (0x08440C, i_08440c),
    (0x08440E, i_08440e),
    (0x084410, i_084410),
    (0x084418, i_084418),
    (0x08441C, i_08441c),
    (0x084422, i_084422),
    (0x084428, i_084428),
    (0x08442C, i_08442c),
    (0x084430, i_084430),
    (0x084434, i_084434),
    (0x08443A, i_08443a),
    (0x08443E, i_08443e),
    (0x084442, i_084442),
    (0x084446, i_084446),
    (0x08444A, i_08444a),
    (0x084450, i_084450),
    (0x084456, i_084456),
    (0x08445C, i_08445c),
    (0x084462, i_084462),
    (0x084468, i_084468),
    (0x08446C, i_08446c),
    (0x084472, i_084472),
    (0x084478, i_084478),
    (0x08447E, i_08447e),
    (0x084484, i_084484),
    (0x08448A, i_08448a),
    (0x08448E, i_08448e),
    (0x084494, i_084494),
    (0x084498, i_084498),
    (0x08449A, i_08449a),
    (0x0844A0, i_0844a0),
    (0x0844A6, i_0844a6),
    (0x0844AC, i_0844ac),
    (0x0844B0, i_0844b0),
    (0x0844B8, i_0844b8),
    (0x0844BC, i_0844bc),
    (0x0844BE, i_0844be),
    (0x0844C0, i_0844c0),
    (0x0844C6, i_0844c6),
    (0x0844CC, i_0844cc),
    (0x0844D2, i_0844d2),
    (0x0844D8, i_0844d8),
    (0x0844DE, i_0844de),
    (0x0844E2, i_0844e2),
    (0x0844E4, i_0844e4),
    (0x0844EA, i_0844ea),
    (0x0844F0, i_0844f0),
    (0x0844F6, i_0844f6),
    (0x0844FA, i_0844fa),
    (0x084500, i_084500),
    (0x084504, i_084504),
    (0x084508, i_084508),
    (0x08450A, i_08450a),
    (0x08450C, i_08450c),
    (0x084512, i_084512),
    (0x084518, i_084518),
    (0x08451E, i_08451e),
    (0x084520, i_084520),
    (0x084526, i_084526),
    (0x08452C, i_08452c),
    (0x084530, i_084530),
    (0x084534, i_084534),
    (0x084536, i_084536),
    (0x08453C, i_08453c),
    (0x084542, i_084542),
    (0x084548, i_084548),
    (0x08454C, i_08454c),
    (0x084552, i_084552),
    (0x084558, i_084558),
    (0x08455E, i_08455e),
    (0x084564, i_084564),
    (0x08456A, i_08456a),
    (0x08456E, i_08456e),
    (0x084574, i_084574),
    (0x084578, i_084578),
    (0x08457C, i_08457c),
    (0x084580, i_084580),
    (0x084584, i_084584),
    (0x08458A, i_08458a),
    (0x084590, i_084590),
    (0x084596, i_084596),
    (0x08459C, i_08459c),
    (0x0845A0, i_0845a0),
    (0x0845A6, i_0845a6),
    (0x0845AC, i_0845ac),
    (0x0845B0, i_0845b0),
    (0x0845B6, i_0845b6),
    (0x0845B8, i_0845b8),
    (0x0845C0, i_0845c0),
    (0x0845C4, i_0845c4),
    (0x0845C6, i_0845c6),
    (0x0845CA, i_0845ca),
    (0x0845CE, i_0845ce),
    (0x0845D2, i_0845d2),
    (0x0845D6, i_0845d6),
    (0x0845DA, i_0845da),
    (0x0845E2, i_0845e2),
    (0x0845E4, i_0845e4),
    (0x0845EA, i_0845ea),
    (0x0845EC, i_0845ec),
    (0x0845F0, i_0845f0),
    (0x0845F2, i_0845f2),
    (0x0845F6, i_0845f6),
    (0x0845FC, i_0845fc),
    (0x084600, i_084600),
    (0x084604, i_084604),
    (0x084608, i_084608),
    (0x08460C, i_08460c),
    (0x084612, i_084612),
    (0x084618, i_084618),
    (0x08461C, i_08461c),
    (0x084620, i_084620),
    (0x084622, i_084622),
    (0x084626, i_084626),
    (0x08462C, i_08462c),
    (0x084630, i_084630),
    (0x084636, i_084636),
    (0x084638, i_084638),
    (0x08463E, i_08463e),
    (0x084644, i_084644),
    (0x084648, i_084648),
    (0x08464E, i_08464e),
    (0x084652, i_084652),
    (0x084654, i_084654),
    (0x084658, i_084658),
    (0x08465E, i_08465e),
    (0x084662, i_084662),
    (0x084666, i_084666),
    (0x08466A, i_08466a),
    (0x08466E, i_08466e),
    (0x084674, i_084674),
    (0x08467A, i_08467a),
    (0x08467E, i_08467e),
    (0x084682, i_084682),
    (0x084684, i_084684),
    (0x084688, i_084688),
    (0x08468E, i_08468e),
    (0x084692, i_084692),
    (0x084698, i_084698),
    (0x08469A, i_08469a),
    (0x08469E, i_08469e),
    (0x0846A4, i_0846a4),
    (0x0846A8, i_0846a8),
    (0x0846AA, i_0846aa),
    (0x0846AE, i_0846ae),
    (0x0846B4, i_0846b4),
    (0x0846B8, i_0846b8),
    (0x0846BC, i_0846bc),
    (0x0846C0, i_0846c0),
    (0x0846C4, i_0846c4),
    (0x0846CA, i_0846ca),
    (0x0846CE, i_0846ce),
    (0x0846D4, i_0846d4),
    (0x0846D8, i_0846d8),
    (0x0846DE, i_0846de),
    (0x0846E0, i_0846e0),
    (0x0846E4, i_0846e4),
    (0x0846EA, i_0846ea),
    (0x0846F0, i_0846f0),
    (0x0846F6, i_0846f6),
    (0x0846FC, i_0846fc),
    (0x084702, i_084702),
    (0x084708, i_084708),
    (0x08470E, i_08470e),
    (0x084712, i_084712),
    (0x084716, i_084716),
    (0x08471C, i_08471c),
    (0x08471E, i_08471e),
    (0x084720, i_084720),
    (0x084726, i_084726),
    (0x084728, i_084728),
    (0x08472C, i_08472c),
    (0x084732, i_084732),
    (0x084734, i_084734),
    (0x084738, i_084738),
    (0x08473E, i_08473e),
    (0x084742, i_084742),
    (0x084748, i_084748),
    (0x08474E, i_08474e),
    (0x084754, i_084754),
    (0x084756, i_084756),
    (0x08475C, i_08475c),
    (0x084760, i_084760),
    (0x084766, i_084766),
    (0x08476C, i_08476c),
    (0x084772, i_084772),
    (0x084778, i_084778),
    (0x08477C, i_08477c),
    (0x084780, i_084780),
    (0x084786, i_084786),
    (0x08478C, i_08478c),
    (0x084792, i_084792),
    (0x084798, i_084798),
    (0x08479E, i_08479e),
    (0x0847A2, i_0847a2),
    (0x0847A4, i_0847a4),
    (0x0847AA, i_0847aa),
    (0x0847B0, i_0847b0),
    (0x0847B4, i_0847b4),
    (0x0847BA, i_0847ba),
    (0x0847BE, i_0847be),
    (0x0847C4, i_0847c4),
    (0x0847C8, i_0847c8),
    (0x0847D0, i_0847d0),
    (0x0847D4, i_0847d4),
    (0x0847DA, i_0847da),
    (0x0847DE, i_0847de),
    (0x0847E4, i_0847e4),
    (0x0847EA, i_0847ea),
    (0x0847F0, i_0847f0),
    (0x0847F6, i_0847f6),
    (0x0847FA, i_0847fa),
    (0x0847FE, i_0847fe),
    (0x084804, i_084804),
    (0x084808, i_084808),
    (0x08480E, i_08480e),
    (0x084814, i_084814),
    (0x08481A, i_08481a),
    (0x08481E, i_08481e),
    (0x084824, i_084824),
    (0x084828, i_084828),
    (0x08482E, i_08482e),
    (0x084834, i_084834),
    (0x084836, i_084836),
    (0x08483A, i_08483a),
    (0x084840, i_084840),
    (0x084846, i_084846),
    (0x08484C, i_08484c),
    (0x08484E, i_08484e),
    (0x084852, i_084852),
    (0x084854, i_084854),
    (0x08485A, i_08485a),
    (0x08485C, i_08485c),
    (0x084860, i_084860),
    (0x084866, i_084866),
    (0x08486A, i_08486a),
    (0x084870, i_084870),
    (0x084874, i_084874),
    (0x084876, i_084876),
    (0x08487C, i_08487c),
    (0x084882, i_084882),
    (0x084886, i_084886),
    (0x08488A, i_08488a),
    (0x08488E, i_08488e),
    (0x084892, i_084892),
    (0x084896, i_084896),
    (0x084898, i_084898),
    (0x08489A, i_08489a),
    (0x08489C, i_08489c),
    (0x0848A0, i_0848a0),
    (0x0848A2, i_0848a2),
    (0x0848A4, i_0848a4),
    (0x0848AA, i_0848aa),
    (0x0848AC, i_0848ac),
    (0x0848B0, i_0848b0),
    (0x0848B6, i_0848b6),
    (0x0848BA, i_0848ba),
    (0x0848C0, i_0848c0),
    (0x0848C4, i_0848c4),
    (0x0848C6, i_0848c6),
    (0x0848CC, i_0848cc),
    (0x0848D2, i_0848d2),
    (0x0848D6, i_0848d6),
    (0x0848DA, i_0848da),
    (0x0848DC, i_0848dc),
    (0x0848DE, i_0848de),
    (0x0848E2, i_0848e2),
    (0x0848E6, i_0848e6),
    (0x0848EC, i_0848ec),
    (0x0848F2, i_0848f2),
    (0x0848F6, i_0848f6),
    (0x0848F8, i_0848f8),
    (0x0848FC, i_0848fc),
    (0x084902, i_084902),
    (0x084906, i_084906),
    (0x08490C, i_08490c),
    (0x084912, i_084912),
    (0x084918, i_084918),
    (0x08491E, i_08491e),
    (0x084924, i_084924),
    (0x084928, i_084928),
    (0x08492A, i_08492a),
    (0x084930, i_084930),
    (0x084932, i_084932),
    (0x084936, i_084936),
    (0x08493A, i_08493a),
    (0x08493C, i_08493c),
    (0x084940, i_084940),
    (0x084946, i_084946),
    (0x08494C, i_08494c),
    (0x084952, i_084952),
    (0x084956, i_084956),
    (0x08495C, i_08495c),
    (0x08495E, i_08495e),
    (0x084962, i_084962),
    (0x084968, i_084968),
    (0x08496E, i_08496e),
    (0x084974, i_084974),
    (0x08497A, i_08497a),
    (0x084980, i_084980),
    (0x084986, i_084986),
    (0x08498A, i_08498a),
    (0x084990, i_084990),
    (0x084996, i_084996),
    (0x08499C, i_08499c),
    (0x0849A2, i_0849a2),
    (0x0849A6, i_0849a6),
    (0x0849A8, i_0849a8),
    (0x0849AE, i_0849ae),
    (0x0849B4, i_0849b4),
    (0x0849B8, i_0849b8),
    (0x0849BA, i_0849ba),
    (0x0849C0, i_0849c0),
    (0x0849C6, i_0849c6),
    (0x0849CA, i_0849ca),
    (0x0849CE, i_0849ce),
    (0x0849D2, i_0849d2),
    (0x0849D8, i_0849d8),
    (0x0849DE, i_0849de),
    (0x0849E4, i_0849e4),
    (0x0849EA, i_0849ea),
    (0x0849F0, i_0849f0),
    (0x0849F6, i_0849f6),
    (0x0849FC, i_0849fc),
    (0x084A02, i_084a02),
    (0x084A0A, i_084a0a),
    (0x084A0E, i_084a0e),
    (0x084A10, i_084a10),
    (0x084A16, i_084a16),
    (0x084A1C, i_084a1c),
    (0x084A22, i_084a22),
    (0x084A28, i_084a28),
    (0x084A2C, i_084a2c),
    (0x084A32, i_084a32),
    (0x084A38, i_084a38),
    (0x084A3E, i_084a3e),
    (0x084A44, i_084a44),
    (0x084A46, i_084a46),
    (0x084A4A, i_084a4a),
    (0x084A4E, i_084a4e),
    (0x084A54, i_084a54),
    (0x084A58, i_084a58),
    (0x084A5E, i_084a5e),
    (0x084A62, i_084a62),
    (0x084A68, i_084a68),
    (0x084A6C, i_084a6c),
    (0x084A72, i_084a72),
    (0x084A78, i_084a78),
    (0x084A7E, i_084a7e),
    (0x084A84, i_084a84),
    (0x084A8A, i_084a8a),
    (0x084A90, i_084a90),
    (0x084A96, i_084a96),
    (0x084A9C, i_084a9c),
    (0x084AA2, i_084aa2),
    (0x084AA8, i_084aa8),
    (0x084AAA, i_084aaa),
    (0x084AAE, i_084aae),
    (0x084AB4, i_084ab4),
    (0x084ABA, i_084aba),
    (0x084AC0, i_084ac0),
    (0x084AC6, i_084ac6),
    (0x084ACC, i_084acc),
    (0x084AD2, i_084ad2),
    (0x084AD8, i_084ad8),
    (0x084ADE, i_084ade),
    (0x084AE4, i_084ae4),
    (0x084AE8, i_084ae8),
    (0x084AEC, i_084aec),
    (0x084AF0, i_084af0),
    (0x084AF2, i_084af2),
    (0x084AF8, i_084af8),
    (0x084AFE, i_084afe),
    (0x084B02, i_084b02),
    (0x084B08, i_084b08),
    (0x084B0C, i_084b0c),
    (0x084B10, i_084b10),
    (0x084B16, i_084b16),
    (0x084B1C, i_084b1c),
    (0x084B20, i_084b20),
    (0x084B22, i_084b22),
    (0x084B24, i_084b24),
    (0x084B2A, i_084b2a),
    (0x084B30, i_084b30),
    (0x084B36, i_084b36),
    (0x084B3C, i_084b3c),
    (0x084B40, i_084b40),
    (0x084B44, i_084b44),
    (0x084B48, i_084b48),
    (0x084B4A, i_084b4a),
    (0x084B50, i_084b50),
    (0x084B54, i_084b54),
    (0x084B58, i_084b58),
    (0x084B5A, i_084b5a),
    (0x084B60, i_084b60),
    (0x084B62, i_084b62),
    (0x084B68, i_084b68),
    (0x084B6E, i_084b6e),
    (0x084B74, i_084b74),
    (0x084B78, i_084b78),
    (0x084B7C, i_084b7c),
    (0x084B80, i_084b80),
    (0x084B82, i_084b82),
    (0x084B88, i_084b88),
    (0x084B8E, i_084b8e),
    (0x084B92, i_084b92),
    (0x084B96, i_084b96),
    (0x084B98, i_084b98),
    (0x084B9A, i_084b9a),
    (0x084BA0, i_084ba0),
    (0x084BA6, i_084ba6),
    (0x084BAC, i_084bac),
    (0x084BB0, i_084bb0),
    (0x084BB4, i_084bb4),
    (0x084BB8, i_084bb8),
    (0x084BBA, i_084bba),
    (0x084BC0, i_084bc0),
    (0x084BC6, i_084bc6),
    (0x084BCA, i_084bca),
    (0x084BCE, i_084bce),
    (0x084BD0, i_084bd0),
    (0x084BD2, i_084bd2),
    (0x084BD8, i_084bd8),
    (0x084BDE, i_084bde),
    (0x084BE4, i_084be4),
    (0x084BEA, i_084bea),
    (0x084BF0, i_084bf0),
    (0x084BF4, i_084bf4),
    (0x084BF8, i_084bf8),
    (0x084BFC, i_084bfc),
    (0x084BFE, i_084bfe),
    (0x084C04, i_084c04),
    (0x084C0A, i_084c0a),
    (0x084C10, i_084c10),
    (0x084C14, i_084c14),
    (0x084C18, i_084c18),
    (0x084C1E, i_084c1e),
    (0x084C22, i_084c22),
    (0x084C24, i_084c24),
    (0x084C26, i_084c26),
    (0x084C2A, i_084c2a),
    (0x084C2C, i_084c2c),
    (0x084C32, i_084c32),
    (0x084C38, i_084c38),
    (0x084C3E, i_084c3e),
    (0x084C42, i_084c42),
    (0x084C46, i_084c46),
    (0x084C4A, i_084c4a),
    (0x084C50, i_084c50),
    (0x084C56, i_084c56),
    (0x084C5C, i_084c5c),
    (0x084C5E, i_084c5e),
    (0x084C62, i_084c62),
    (0x084C68, i_084c68),
    (0x084C6E, i_084c6e),
    (0x084C74, i_084c74),
    (0x084C7A, i_084c7a),
    (0x084C80, i_084c80),
    (0x084C84, i_084c84),
    (0x084C86, i_084c86),
    (0x084C8A, i_084c8a),
    (0x084C90, i_084c90),
    (0x084C96, i_084c96),
    (0x084C9C, i_084c9c),
    (0x084CA2, i_084ca2),
    (0x084CA4, i_084ca4),
    (0x084CA8, i_084ca8),
    (0x084CAE, i_084cae),
    (0x084CB4, i_084cb4),
    (0x084CBA, i_084cba),
    (0x084CBE, i_084cbe),
    (0x084CC2, i_084cc2),
    (0x084CC8, i_084cc8),
    (0x084CCC, i_084ccc),
    (0x084CD0, i_084cd0),
    (0x084CD2, i_084cd2),
    (0x084CD8, i_084cd8),
    (0x084CDE, i_084cde),
    (0x084CE2, i_084ce2),
    (0x084CE8, i_084ce8),
    (0x084CEE, i_084cee),
    (0x084CF4, i_084cf4),
    (0x084CFA, i_084cfa),
    (0x084CFE, i_084cfe),
    (0x084D04, i_084d04),
    (0x084D0A, i_084d0a),
    (0x084D0E, i_084d0e),
    (0x084D14, i_084d14),
    (0x084D18, i_084d18),
    (0x084D1C, i_084d1c),
    (0x084D20, i_084d20),
    (0x084D26, i_084d26),
    (0x084D2C, i_084d2c),
    (0x084D30, i_084d30),
    (0x084D36, i_084d36),
    (0x084D3A, i_084d3a),
    (0x084D40, i_084d40),
    (0x084D42, i_084d42),
    (0x084D46, i_084d46),
    (0x084D4C, i_084d4c),
    (0x084D50, i_084d50),
    (0x084D56, i_084d56),
    (0x084D5C, i_084d5c),
    (0x084D62, i_084d62),
    (0x084D68, i_084d68),
    (0x084D6E, i_084d6e),
    (0x084D74, i_084d74),
    (0x084D7A, i_084d7a),
    (0x084D80, i_084d80),
    (0x084D86, i_084d86),
    (0x084D8C, i_084d8c),
    (0x084D90, i_084d90),
    (0x084D92, i_084d92),
    (0x084D98, i_084d98),
    (0x084D9E, i_084d9e),
    (0x084DA4, i_084da4),
    (0x084DA8, i_084da8),
    (0x084DAE, i_084dae),
    (0x084DB0, i_084db0),
    (0x084DB6, i_084db6),
    (0x084DBC, i_084dbc),
    (0x084DC2, i_084dc2),
    (0x084DC8, i_084dc8),
    (0x084DCC, i_084dcc),
    (0x084DD2, i_084dd2),
    (0x084DD6, i_084dd6),
    (0x084DDC, i_084ddc),
    (0x084DE0, i_084de0),
    (0x084DE6, i_084de6),
    (0x084DEA, i_084dea),
    (0x084DEE, i_084dee),
    (0x084DF4, i_084df4),
    (0x084DF8, i_084df8),
    (0x084DFE, i_084dfe),
    (0x084E02, i_084e02),
    (0x084E08, i_084e08),
    (0x084E0C, i_084e0c),
    (0x084E12, i_084e12),
    (0x084E16, i_084e16),
    (0x084E1A, i_084e1a),
    (0x084E20, i_084e20),
    (0x084E24, i_084e24),
    (0x084E2A, i_084e2a),
    (0x084E2E, i_084e2e),
    (0x084E34, i_084e34),
    (0x084E38, i_084e38),
    (0x084E3A, i_084e3a),
    (0x084E3E, i_084e3e),
    (0x084E42, i_084e42),
    (0x084E48, i_084e48),
    (0x084E4E, i_084e4e),
    (0x084E54, i_084e54),
    (0x084E58, i_084e58),
    (0x084E5A, i_084e5a),
    (0x084E60, i_084e60),
    (0x084E66, i_084e66),
    (0x084E6C, i_084e6c),
    (0x084E70, i_084e70),
    (0x084E76, i_084e76),
    (0x084E7C, i_084e7c),
    (0x084E82, i_084e82),
    (0x084E88, i_084e88),
    (0x084E8C, i_084e8c),
    (0x084E92, i_084e92),
    (0x084E96, i_084e96),
    (0x084E9A, i_084e9a),
    (0x084EA0, i_084ea0),
    (0x084EA6, i_084ea6),
    (0x084EAA, i_084eaa),
    (0x084EAC, i_084eac),
    (0x084EB0, i_084eb0),
    (0x084EB6, i_084eb6),
    (0x084EBA, i_084eba),
    (0x084EC0, i_084ec0),
    (0x084EC2, i_084ec2),
    (0x084EC6, i_084ec6),
    (0x084ECC, i_084ecc),
    (0x084ED2, i_084ed2),
    (0x084ED8, i_084ed8),
    (0x084EDE, i_084ede),
    (0x084EE4, i_084ee4),
    (0x084EEA, i_084eea),
    (0x084EEE, i_084eee),
    (0x084EF0, i_084ef0),
    (0x084EF6, i_084ef6),
    (0x084EFC, i_084efc),
    (0x084F02, i_084f02),
    (0x084F08, i_084f08),
    (0x084F0E, i_084f0e),
    (0x084F14, i_084f14),
    (0x084F1A, i_084f1a),
    (0x084F1E, i_084f1e),
    (0x084F24, i_084f24),
    (0x084F26, i_084f26),
    (0x084F2A, i_084f2a),
    (0x084F30, i_084f30),
    (0x084F36, i_084f36),
    (0x084F3C, i_084f3c),
    (0x084F42, i_084f42),
    (0x084F46, i_084f46),
    (0x084F4A, i_084f4a),
    (0x084F4E, i_084f4e),
    (0x084F52, i_084f52),
    (0x084F54, i_084f54),
    (0x084F56, i_084f56),
    (0x084F5A, i_084f5a),
    (0x084F5C, i_084f5c),
    (0x084F5E, i_084f5e),
    (0x084F62, i_084f62),
    (0x084F68, i_084f68),
    (0x084F6E, i_084f6e),
    (0x084F74, i_084f74),
    (0x084F76, i_084f76),
    (0x084F7A, i_084f7a),
    (0x084F7C, i_084f7c),
    (0x084F80, i_084f80),
    (0x084F84, i_084f84),
    (0x084F86, i_084f86),
    (0x084F8C, i_084f8c),
    (0x084F90, i_084f90),
    (0x084F94, i_084f94),
    (0x084F9A, i_084f9a),
    (0x084F9E, i_084f9e),
    (0x084FA2, i_084fa2),
    (0x084FA6, i_084fa6),
    (0x084FAA, i_084faa),
    (0x084FAC, i_084fac),
    (0x084FB2, i_084fb2),
    (0x084FB8, i_084fb8),
    (0x084FBE, i_084fbe),
    (0x084FC2, i_084fc2),
    (0x084FC8, i_084fc8),
    (0x084FCA, i_084fca),
    (0x084FCE, i_084fce),
    (0x084FD4, i_084fd4),
    (0x084FDA, i_084fda),
    (0x084FE0, i_084fe0),
    (0x084FE2, i_084fe2),
    (0x084FE6, i_084fe6),
    (0x084FE8, i_084fe8),
    (0x084FEA, i_084fea),
    (0x084FEE, i_084fee),
    (0x084FF4, i_084ff4),
    (0x084FF6, i_084ff6),
    (0x084FFA, i_084ffa),
    (0x084FFE, i_084ffe),
    (0x085004, i_085004),
    (0x085008, i_085008),
    (0x08500C, i_08500c),
    (0x085012, i_085012),
    (0x085018, i_085018),
    (0x08501C, i_08501c),
    (0x08501E, i_08501e),
    (0x085024, i_085024),
    (0x085028, i_085028),
    (0x08502C, i_08502c),
    (0x085032, i_085032),
    (0x085036, i_085036),
    (0x08503A, i_08503a),
    (0x08503E, i_08503e),
    (0x085042, i_085042),
    (0x085044, i_085044),
    (0x08504A, i_08504a),
    (0x085050, i_085050),
    (0x085056, i_085056),
    (0x08505A, i_08505a),
    (0x085060, i_085060),
    (0x085062, i_085062),
    (0x085066, i_085066),
    (0x08506C, i_08506c),
    (0x085072, i_085072),
    (0x085078, i_085078),
    (0x08507E, i_08507e),
    (0x085082, i_085082),
    (0x085084, i_085084),
    (0x08508A, i_08508a),
    (0x08508E, i_08508e),
    (0x085094, i_085094),
    (0x085098, i_085098),
    (0x08509E, i_08509e),
    (0x0850A2, i_0850a2),
    (0x0850A6, i_0850a6),
];
